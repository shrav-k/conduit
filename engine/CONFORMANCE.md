# First-build conformance choices

Not binding. The [specification](../spec/README.md) alone binds. This document states the choices
implemented in increment 2, under the admin's
[decisions](../docs/proposals/first-build/increment-2/decisions.md), and in the shell orders that
followed it from [order 020](../docs/implementation/WORK_ORDERS.md#order-020) on, and distinguishes
them from choices the shell must still make. It is not a declaration of whole-language conformance.
The section [v0's security claim, drafted](#v0s-security-claim-drafted) is the draft of v0's
public security claim and promises nothing either.
The executable [profile](../conformance/profiles/first-build.yaml) and [coverage
manifest](../conformance/manifest.yaml) name the evidence. A vector demonstrates its stated slice of
a promise; a coverage reason is not a passing vector.

For each promise, [STANDING.md](STANDING.md) gives the evidence the tree holds: how many cases
list it, how many of those are expected failures, its S1 row, and which of the engine's crates
name it. `tools/promise_standing.py` generates it. It says what evidence there is, not that a
promise is kept.

## Implemented choices

| DL item | First build's choice | Where it is shown |
| --- | --- | --- |
| DL2: contents | SHA-256 of RFC 8785 JSON holding the qualified operation and default-filled arguments; caller/sender/target/key scope is separate. Invocation state, held contexts and retained results are independent of records. | `requests-key-scope`, `requests-defaults`, `records-purge`, [requests.rs](crates/core/src/requests.rs) |
| DL2: dispatch boundary | `step` returns a proposed state, appended records and intents atomically. The shell commits before executing any intent or delivering a final reply. Executors run outside the transition mutex; their completions reenter it. Pending replies wait inside the shell. | `s1-whole`, `requests-restart-interrupted`, [shell tests](crates/shell/src/tests.rs) |
| DL2: storage | Format version 1 is unchanged: the whole core State, including records, in one JSON file. `state.json` is parsed once when the store opens. The committed State and output times stay in memory under the writer mutex and process lease; readers share the last commit. Each transition writes a new file in the store directory, fsyncs it, renames it over the old file and fsyncs the directory before memory advances. After a failure, the file is read again. A process lease excludes a second shell, including recovery while another shell is alive. Store/run directories are 0700, files 0600; existing broader modes and unknown formats are refused. The store's parent must exist. After each commit that succeeds, still under the writer mutex, the shell appends one line of compact JSON to `commit-measures.jsonl` in the store directory, with no fsync: `at`, the input's kind as `input`, `stateBytes` and `invocations` after the commit, `writes` (its transition's and each expiry's), and `stepMicros`, `writeMicros` and `commitMicros`; the recover commit that opening makes also gives `openMicros`. A line names no object, invocation or path. The file is created 0600 and never followed through a link, and one that is not a regular file with mode 0600 gets no line; the engine never reads, truncates or rotates it. A line that cannot be appended changes no result: a line naming the input's kind and the cause goes to standard error, and the next line appended carries `unwritten`. | [store.rs](crates/shell/src/store.rs), killed-child tests before and after send, `commit_measures_one_line_per_commit` in [the shell's tests](crates/shell/src/tests.rs) |
| DL2: attempts and late results | Attempts start at 1 and increase on an eligible caller repeat. Completion identifies both invocation and attempt. A stale attempt or ended invocation answers `ignored`; a late response cannot settle unknown, but an answer ending its call fences its named attempt under OPS-014. Failed reconciliation conflicts under OPS-030 until every attempt is fenced; missing marks in older stores prove no fence. A parameter-header repair is once within the same attempt, only for IFC-046's read-only case, with no general effectful resend. | `requests-unknown-write`, `requests-repair-read-success`, `setup-close-in-flight`, [requests.rs](crates/core/src/requests.rs) |
| DL3: waiting, implemented portion | There is no autonomous clock or actor-reply timeout in the core. Pending lasts until supplied completion, cancellation or recovery. A late answer fences the attempt without changing its outcome. Missing responses, unconfirmed cancellation and recovery fence nothing. The shell removes pending responses at its call timeout, so those late answers do not yet reach the core. No actor-reply provider has been qualified. | `requests-key-scope`, `requests-unknown-write`, `requests-cancellation`, [requests.rs](crates/core/src/requests.rs) |
| DL4 | An unreadable identity/authority or state needed to decide access yields unavailable immediately; no waiting queue. | `access-identity`, `access-current-tools`, `records-authority` |
| DL5: reports, implemented portion | Time, canonical hostname, labels and run reports are supplied values. First start keeps the supplied local machine report. Setup asks for run inspection and interprets running/stopped/unknown; observation does not revise the definition. No polling cadence is chosen here. | `first-start`, `setup-run-unknown`, `setup-review-conditions-builtin`, `setup-review-conditions-provider` |
| DL6 | First start is one atomic empty-store transition. A repeated first start preserves identities and creates nothing. Allocation exhaustion/reuse is an embedding error and leaves the input state intact. Recovery ends interrupted work unknown without reissuing it; no repair is inferred. | `first-start`, `first-start-repeat`, `requests-restart-interrupted`, [boundary tests](crates/conformance/tests/core_boundary.rs) |
| DL7: archives | Gzip-compressed POSIX USTAR, without GNU/PAX extensions. UTF-8 paths; files, directories, symbolic links and hard links to earlier regular files. At most 40 link expansions. Last member at a path wins except replacing a nonempty directory or using a non-directory parent. All paths/targets are checked, including ignored documents. | `archives-link-expansions-39`, `archives-link-expansions-40`, `archives-link-expansions-41`, `archives-hardlink`, `archives-symlink`, [archive fixtures](../conformance/fixtures/install-archives/README.md), [archives.rs](crates/core/src/archives.rs) |
| DL7: storage and submission | Installed packages retain a virtual directory with bytes/modes in `State.installed_packages`; no compressed cache, removal or mirror API is implemented. Directory URL and fetched bytes enter at the boundary. The runner supplies directory documents for an established actor; registration installs nothing. A preinstalled snapshot explicitly says so. | `archives-program`, `archives-s1`, `packages-relative-fetch-directory`, `registration-s1`, [packages.rs](crates/core/src/packages.rs) |
| DL8: records | One ordered CloudEvents source, `urn:conduit:first-build`. Seven event types: access-decision, request-ended, invocation-dispatched, invocation-ended, provider-call, object-change, pending-action-answered (order 047c, an answer to a pending action, apart from the request's records under PND-004 and REC-002). Object-change actions and provider made/ended phases are in the profile. Export explicitly returns ordered records, including records for removed objects. No live record-feed cursor is implemented. | `first-start`, `records-inspection`, `records-unreadable`, `setup-review-provider-records`, [records.rs](crates/core/src/records.rs) |
| DL8: inspection and purge | Inspect keeps id/revision/packages/declared/status/records separate, omits template status, and reports unresolved invocations separately. Gap metadata is conservative across all known source gaps. Purge removes records strictly before its instant and retains permanent source/ID certificates. Export uses gaps/complete; inspection uses recordGaps/recordsComplete. Purge never alters the invocation index or retained results. | `records-inspection`, `records-purge`, `records-purge-succeeded`, `records-input-reference`, `records-unreadable` |
| DL9 | There is no extra portable ordering rule among diagnostics of the same reason. The implementation returns its deterministic traversal order. OPS-023's ordering between different reasons still binds. Vectors compare the required rule/location facts rather than validator prose. | `registration-s1-rejected-orders`, `access-routing`, `loader-schema-attribution`, [case assertions](../conformance/README.md#assertions) |
| DL10: structural limits | 128 levels of nesting (portable floor 64). No stated byte limit for files or archives, or count limit for documents/files in a submission. This does not promise unlimited physical memory. | `loader-depth-limit`, profile `DL10`, loader `MAX_DEPTH` |
| DL10: retained results | Pure vectors keep results until explicit expiry. The shell caps retained outputs and successful plain content at 64 MiB total and 7 days by default, configurable by Config or CLI flags. Size is compact UTF-8 JSON bytes of each retained outputs/content map, including its JSON overhead. Results in a pending repeat's previous snapshots count toward both size and age; restoration after refusal preserves their original completion time. The core's expire-outputs removes retained results throughout those snapshots under OPS-034/DL10, preserving all invocation and snapshot state. Before requests and after transitions, expire oldest completion first through expire-outputs until within both limits. Completion times are shell metadata committed beside State, independent of purgable records. Idle stores are checked on opening; no timer runs here. No key expires and invocation state is never compacted. Reconciliation invents no outputs; repeats follow OPS-024/034. | retention cap/age/restart tests in [tests.rs](crates/shell/src/tests.rs), `requests-plain-content`, `records-purge-succeeded` |
| DL12: tool schemas | Inline no optional contract references. Copy reached contracts once, with escaped local roots and finite recursive definitions; target-held contexts choose releases. | `e05-local`, `e05-two-contexts`, `e05-recursive`, `e05-siblings`, `e05-escaping`, [schemas.rs](crates/core/src/schemas.rs) |
| DL12: result content | Preserve successful plain MCP content alongside structuredContent. For other successes use structuredContent without an optional serialized text copy. Errors carry a status text block; invalid/conflict also carry diagnostic structuredContent. | `s1-whole`, `requests-plain-content`, `requests-declared-outputs`, [result envelope](crates/core/src/requests.rs) |
| DL13: instance proof | One store-private ECDSA P-256 key and self-signed CA certificate. The FST-007 trust domain is configured with `init --trust-domain` or generated from two UUID v4 values (244 random bits), validated before use and fixed for the store. `system.status.trustDomain` records it; `engine-id` carries `spiffe://<domain>/conduit/engine`, and token subjects use `spiffe://<domain>/conduit/instance/<id>`. ES256 tokens carry the instance's SPIFFE ID, engine-address audience, iat, random jti and exp set to iat plus 100 calendar years (UTC, clamping February 29 to February 28 if necessary). Verification requires an integer exp strictly later than the current time, and checks signature, ES256, audience, SPIFFE form/trust domain and current instance existence on every request. Missing proof maps to none; invalid proof maps to unreadable. Every run gets a fresh private directory with the token, engine ID, public JWKS and CA PEM, under the names the run contract supplies. Credentials never enter core State, records or shell output. | [identity.rs](crates/shell/src/identity.rs), identity and S1 shell tests |
| DL13: loopback boundary | This shell accepts an http engine address on loopback only. ca.crt checks nothing at this address; its root uses the same P-256 key as the JWT authority. Built-in runs now end with the engine on this one machine: a group leader watches an engine-only lifetime socket and kills its process group on EOF, including SIGKILL of the engine. Dropping either shell or runner also kills all groups. The order 022 adapter serves authenticated loopback HTTP with the runner as its executor. | [shell](crates/shell/src/lib.rs), [handoff](README.md#the-cores-boundary) |
| DL10: process and call deadlines | MCP initialization: 30 seconds from opening the stdio session. Dispatch and a refresh-plus-read: 120 seconds per intent. Config and CLI can set both. Initialization timeout kills the group and reports failed; at a dispatch's deadline the shell sends `notifications/cancelled`, with reason `call deadline exceeded`, for the request still outstanding (the call, or a refresh's `tools/list`). It waits at most 5 seconds for the send, then reports missing with the original invocation and attempt. It never resends a write and does not read a later answer. A cancellation whose send fails leaves the fixed diagnostic `the call of invocation <ID> passed its deadline and its cancellation could not be sent`; one not seen sent within 5 seconds leaves `the call of invocation <ID> passed its deadline and its cancellation was not seen sent within 5 seconds`. Each names only the invocation. npm preparation helpers have a fixed 120-second deadline, plus at most 5 seconds for their exit status. | [runner.rs](crates/shell/src/runner.rs), `dispatch_deadline_cancels_the_held_request_before_the_next_call` (the cancellation is sent, and before the next call on the run; that it comes before the missing report rests on the code order in `Inner::dispatch_later`, runner.rs:1151-1187), `dispatch_deadline_reports_a_cancellation_that_could_not_be_sent` and deadline/exit/repair/cancellation tests in [runner tests](crates/shell/tests/runner.rs) |
| DL10: MCP session idle limit | The serving engine's session idle limit is `call_timeout` plus 60 seconds (180 seconds by default). Config's `call_timeout` and the CLI's `--call-timeout-seconds` set it together with the call deadline. HTTP session events and outbound server messages reset the idle timer; a running call alone does not. If the sum exceeds the range of Duration, the idle limit is disabled. | `session_keep_alive_outlasts_call_deadline`, `running_call_outlives_rmcp_default_idle_limit` in [session tests](crates/shell/src/server/session_tests.rs) |
| DL16: packages and commands | npm is the only supported registryType; any other fails with its name. Package transport must be stdio. npm pack fetches the exact version with scripts disabled. SHA-256 is checked before decompression, manifest reading, dependency installation or execution. Verified artifacts are unpacked into a hash-keyed store cache, dependencies installed with npm install --omit=dev --ignore-scripts --legacy-peer-deps (no automatic peer installation; needed for the observed npm 10.2.4 resolver failure), and the sole bin run with node. The artifact hash does not pin the dependency graph (MAN-010). The runner does not check a Package's declared engines. The unchanged filesystem server 2.1.0 declares Node >=24, but S1 was exercised on Node 21.5.0: qualification covers those exercised operations on this machine only, not the declared Node platform range. npm uses a store-local download cache and empty user configuration, no global configuration, the public npm registry, and no audit/fund requests. Relative command paths are materialized from retained installed-package files under the store with their modes; absolute programs run as given, and bare names resolve through the run's PATH. | [runner.rs](crates/shell/src/runner.rs), installed-program unit test, opt-in hash mismatch and real S1 tests |
| DL16: environment and working directory | Unix only, as the engine's operating-system user and existing login session. Clear the environment, then pass HOME, LOGNAME, PATH, SHELL, TERM and USER when present and not beginning with (). Config.pass_environment (CLI: repeated --pass-environment NAME), empty by default, names further inherited variables, also skipping function values. Package environment declarations supply fixed values and defaults over inheritance; supplied environment bindings and secrets override them, and the materialized identity's two variables and instance state directory win. Package argument and environment values substitute declared server.json variables once, preserving unresolved identifiers. Fixed positional values need no valueHint. The runner reads only the permitted inherited names. Each run starts in a fresh empty store/work directory, mode 0700, separate from shared data and identity files. This is no OS sandbox. | exact exec-environment and binding unit tests; synthetic GITHUB_TOKEN exclusion, working-directory and identity tests |
| DL16: instance state directories | PRV-029: each built-in run receives the variable named by stateDirVariable in spec/names.yaml, overriding inheritance, the pass list and package defaults. Its initially empty directory is `<store>/instances/<instance-ID>`, mode 0700; every later run and engine restart reuses it. The runner reads none of its contents. MAN-018 rejects selected items binding reserved variables, including secret-reference bindings, without regard to case. LIF-035: after a durable removal or unregister, recursive deletion runs before publishing the reply. A failed deletion leaves the directory as a durable retry marker; every later core commit and store reopen retries it. No idle timer runs cleanup. Invalid permissions or a symlink prevent reuse as a state directory. Same-user access remains mediated. Assigned variable names are embedded from spec/names.yaml at build time; a rename requires rebuilding. | state-dir-binding vector; private state, runner restart, retirement and unregister/retry shell tests |
| DL16: run state and restart | No automatic restart. One process group and initialized MCP client session per run, with no Roots capability. Unasked exit closes outstanding dispatches as missing, and the next inspect-run reports stopped. Reopening has no sessions from before and reports them stopped; recovery never replays attempts. Explicit subsequent setup may start a stopped run. The lifetime guard ends every member still in the process group on engine loss; deliberately escaping that group is outside this runner's confinement, like other same-user routes. | shell-drop, killed-engine/descendant, process-exit, initialization and restart tests; real outside-root filesystem refusal |
| DL16: executor coverage | The core fences prior attempts sent to a built-in run on builtin-run-stopped, with no automatic reconciliation. The shell submits that observation only at runner shutdown, and only once the run's Unix process group has been seen absent: the guard receives a stop request on its lifetime pipe and sends TERM, with one shared Config.stop_grace (10 seconds by default) for all runs. Once asked to stop, the guard KILLs its group as soon as the program has exited; loss of the lifetime pipe KILLs it immediately at any time. At the grace deadline, the runner KILLs groups still present and waits up to three more seconds, polling every 10 ms. Only a group probe failing with ESRCH proves absence, after the guard has been reaped; success or EPERM does not. Each group seen absent is recorded immediately. For a group not seen absent, nothing is submitted and a fixed diagnostic names its instance ID instead. Shutdown runs once; Drop invokes it and prints the diagnostics that call added. Groups still starting are KILLed immediately without an observation; if the shell is already gone, shutdown waits for nothing and submits nothing. A process that left the group is neither stopped nor seen. Spontaneous exit, runs ended across restart, runtime-provider stop reports and composite-part fences are not yet connected to this evidence. A missing local run or unserved remote dispatch reports missing, which cannot prove that nothing was sent. builtin-runner, dispatch, refresh-tools-and-call, cancel and cancel-setup-parts are served. Runtime provider calls and remote connections are unserved: inspection reports unknown, run/connection parts fail, and other calls report missing through the existing loop. Cancellation uses the attempt's actual JSON-RPC ID; setup cancellation kills every group it started and starts none. The transport preserves the core's exact tools/call params, including its omission of metadata for plain servers, rather than adding rmcp client/progress metadata. Refresh reads tools/list once then sends the eligible core-supplied read once within its attempt; no effectful retry is added. | wire, repair, write-error, late-result and setup-cancellation tests; shutdown_reports_a_run_that_traps_term_stopped_after_kill; shutdown_reports_a_run_that_exits_on_term_before_the_grace; shutdown_submits_nothing_for_a_group_not_seen_absent |
| DL15 | Generated names retain at most the first 57 characters of the template name, then append a hyphen and a five-character suffix of lower-case letters/digits, supplied by the boundary and retried on collision. Authored and generated names share the ruled grammar; unused suffixes are not consumed identities. | `registration-generated-collision`, `registration-s1`, [registration.rs](crates/core/src/registration.rs) |

D19 is a measurement convention under DL2/DL8, not a DL promise: compact UTF-8 JSON bytes
for records, gap metadata and invocation arrays separately, with per-step signed growth and
per-invocation bytes/key status/outcome. Held contexts and retained results count. The runner
prints these measurements; they are not a persistence format or compaction policy.

## Choices still required

| DL item | What has not been chosen or implemented here |
| --- | --- |
| DL1 | Container retirement dispatches independent members in stored order. Within dependency cycles, members retire in ascending ID order; dependency edges between cycles are preserved (LIF-021). Pool and Fleet cascades are implemented. The member IDs are captured at dispatch; a later registration blocks container retirement with diagnostics and needs another explicit cascade. |
| DL3 | Actor-reply waiting policy and delivery while the server cannot receive; no such provider runs here. |
| DL5 | Machine report intervals/deadlines, hostname override and voluntary runtime inspection intervals. The shell now discovers the OS hostname and OS/architecture labels at first start, plus explicitly supplied labels. |
| DL7 | Package removal/mirror controls, https fetching and package-source providers remain unfinished. Public install.v1 supports local file URLs, authorization, admission and records; installed bytes and virtual files are persisted in State. |
| DL8 | Live feed/cursor delivery. Physical records are now in the atomic State file. |
| DL10 | Retirement orchestration deadlines. A store write/fsync error stops that shell operation as an embedding error; no intent or successful reply is issued for that operation. General disk-space management is pending. |
| DL12 | Additional MCP revisions and transports beyond the loopback adapter below. |
| DL13 | Gate proof, TLS and authorization-server integration. No public OAuth issuer/grant or remote-runtime credential delivery is claimed by the loopback-only authority. |
| DL16 | Platforms beyond Unix process groups and lifetime sockets; qualification beyond the npm filesystem server and local scripted servers. |
| DL17 | Multi-machine restart/partition/fencing. No distributed configuration or guarantee is demonstrated by this core. |
| DL18 | Partly built (order 048a-5b): `sign-in.v1` answers invalid or conflict where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order 048a-5c's fixed-port check applies, and otherwise checks the account first, in `sign_in.rs` (order 048a-5b): an `account-hook.v1` part reports it, and the expected account, compared ignoring ASCII case, ends `sign-in.v1` succeeded with output `account`, answers the instance's unanswered sign-in action signed-in and lifts PND-005's hold, while a check that ends with a reason, such as unavailable under IFC-037, ends it failed and completes nothing (PND-011); a check that finds the expected account or none sets a `False` `SignInUndone` to `True`. From order 048a-5d, a check that finds no account or another, or that ends failed or unknown and so counts as no account, clearing no condition (order 209), leads to a `sign-in-hook.v1` part, unless a retirement of the instance is in progress, which ends `sign-in.v1` failed (PND-003), after another account has raised the sign-in action unless one is unanswered (PND-005); a call that carries Conduit's keys holds the asked-for login's name in `signInLogin` (IFC-006), and a plain hook is never asked for one, since PND-010 refuses such a request before any part (order 209). The hook's `input_required` answer, plain or keyed alike, is taken by `sign-in.v1` and never read as IFC-016's lapse trigger, even once that `sign-in.v1` has ended (IFC-010; `requests-sign-in-plain-hook-part`). A hook answer that is not one URL elicitation ends `sign-in.v1` failed under IFC-049, naming the check and the field, and a hook that ends failed or unknown ends it so under PND-010. From order 048a-5d2, the hook's URL is matched under IFC-049 to exactly one login of the instance's Type, and to the asked-for one where a login was asked for, a plain hook's alike; a URL that does not match ends `sign-in.v1` failed under IFC-049, the hook part ending unknown, with a diagnostic naming the check, the field and, where the check concerns one, the login, and holding no URL value, code or state (`requests-sign-in-url-match`, `requests-sign-in-plain-url-match`). Where no login matches, the check named is the latest that any candidate reached, which IFC-049 leaves to the engine. From order 048a-5ef, a URL elicitation whose URL passes IFC-049's checks is relayed to the Interface that asked: one input request holding only `mode`, the engine's message and the URL, and in the result's own `_meta` a `signInLogin` naming the login, its return, the instance's node and whether a return is taken (PND-013's first check). The message names the instance, its Type and package, its node, the URL's host, the expected account and what the person brings back: for a device login, that the code is shown in the engine's panel; for a service login, that its page receives the return; and for a login whose return is not taken, that nothing comes back through the Interface and the login is completed on the node's machine. The hook part stays in progress, and `sign-in.v1` waits for the person's retry for ten minutes from the step that first relays the URL: every input request of the invocation carries that instant as its expiry, and the shell refuses its requestState from then on. Each accepted retry polls `account-hook.v1`: the expected account, compared ignoring ASCII case, ends `sign-in.v1` succeeded as the first check does; no account, the account the first check found, or a poll that ends failed or unknown counts as no account yet and answers another input request with the same expiry, for which the shell issues a fresh requestState; and a poll that ends with a reason ends it failed (PND-011). A decline or cancel ends it failed under PND-010, dispatching no part, and any other response ends it failed under IFC-049; a retry that names no `sign-in.v1` waiting at that attempt for that Interface is refused invalid under IFC-049 and recorded. Whenever `sign-in.v1` ends, but by a cancel of its request, its waiting hook part ends unknown; a cancelled sign-in leaves it in progress until a restart ends it (DL6), and its final result holds `signInLogin`. The URL is kept with the waiting invocation until it ends, and in no record. Another account found while polling is undone by a `sign-out-hook.v1` part before `sign-in.v1` ends failed (row DL19). A `sign-in.v1` that no retry completes by its output's expiry ends unknown under PND-010 (order 048a-5gh): at each relayed output's expiry the shell submits one input naming the invocation and attempt, and where the invocation still waits on that attempt for the person, the core ends its waiting `sign-in-hook.v1` part and then the invocation unknown, sends nothing and runs no hook for it later; its sign-in action and hold stay, and a retry after it is refused by the shell. A poll or an undo in flight at that instant is left to finish, and an output that a poll gives after it expires at once; a poll that finds another account after it ends `sign-in.v1` unknown as that input does, and no `sign-out-hook.v1` part runs. A restart ends a waiting sign-in unknown by the engine's recovery (DL6, IMP-007), and the shell's timers end with the engine. A return that a retry carries is not read, and no retry of the hook is sent: PND-013's checks of a return come after the dogfood path (ruling 88). The implementation's own requestState is kept with the hook part only while the part is in progress, and is never relayed, recorded or shown. Where the checks of orders 048a-2 and 048a-5 apply, a stale `expectedId` reports conflict under LIF-037. |
| DL19 | Partly built. A setup action (PND-006) expires seven days after its set-up.v1 invocation first ended, counted from the time of the step that ended it, and is raised while a step's time is before its expiry and not from then on. A successful repeat under OPS-009 returns its original inspection or `list.v1` outputs while retained, including any `status.pending` or `pending`, and the inspection's shared records omit `status.pending`. The record of an inspection that raises it does not yet name the action's ID (PND-012). A setup that ended before order 047 kept no end time: its expiry counts from the last `invocation-ended` record of that invocation that no reconciliation wrote, and where none remains, as after a purge, it is not raised. While raised it is raised at root and at each Interface whose scope holds its instance, never at an Agent (order 047b): it is listed in `status.pending` of that Interface's own inspection of the instance and in the `pending` output of its own `list.v1` through any Pool, Fleet or management, and counted in the `unanswered` key of its own results; another Interface sees its evidence only where it may inspect the instance, and no Interface sees a code or fingerprint item. `answer.v1` answers a setup action dismissed (order 047c) ([pending.rs](crates/core/src/pending.rs)); a further set-up.v1 answers it retried and its retirement retired, and once its expiry passes it is expired; each answer is kept in the engine's state, which no purge touches, for the ending it answered, so that a redispatch that ends unsuccessfully makes a new action under the same invocation ID. A notice (PND-007, order 047d) expires seven days after the step of the retirement that raised it. One is raised for each Fleet, other than the sender's where the sender is an Agent, whose Agent consumed a Conduit that a Resource's retirement sent by an instance other than root closed; it is stored with an engine ID, named in the `pendingAction` key of the retirement's result and in the `pendingAction` of its `object-change` record, raised at root, at each Interface whose scope holds that Fleet, and, while the Resource's shutdown is unconfirmed, at each Interface whose scope holds the Resource, and answered `acknowledged` through `answer.v1` with a Conduit to that Fleet, or to `management` where that Fleet is retired or gone. At an Interface other than root it shows a closed Conduit and its evidence where that Interface may inspect the Conduit's consumer, and the Resource and its evidence where it may inspect the Resource while the Resource's shutdown is unconfirmed. A sign-in action (PND-005, order 048a-2) expires seven days after the step that raised it. It is raised, unless the instance holds one unanswered or is retired, when a call to an instance whose Type states `signIn` with a string account ends with its implementation's error diagnostic naming PND-005 or with a URL elicitation outside `sign-in.v1` (IFC-016); `answer.v1` does not answer it, retirement answers it void, and a `sign-in.v1` of the instance that succeeds answers it signed-in (order 048a-5b). From its first raise the instance is held (PND-005, order 048a-3): a request to it where it is a Resource, or one whose operation starts model work where it is an Agent, is not dispatched and reports unavailable, naming its unanswered sign-in action in `pendingAction`; the hold is the instance's and outlasts the action's expiry, the next held request after an expiry raises a new action, and only a `sign-in.v1` of the instance that succeeds lifts it, whatever its action's state (order 048a-5b; MC9a-003). Setup checks the account (PND-011, TPL-012, order 048a-4): where the instance's Type states `signIn` with a string account, an `account-hook.v1` part reports the account after the run and connections and before the setup hook; no account or another raises the sign-in action unless one is unanswered, and another is undone by a `sign-out-hook.v1` part before setup goes on. Where that part does not succeed, the setup goes no further, the instance's condition `SignInUndone` is `False`, and every request to the instance reports unavailable under PND-011, beside PND-005's hold, until a setup's account check finds the expected account or none and sets it `True`; such a check lifts no hold of PND-005, and the check that `sign-in.v1` sends first sets it `True` the same way (order 048a-5b). A check that ends failed or unknown counts as no account (order 209): it raises the sign-in action unless one is unanswered, undoes nothing, clears no condition and lets setup go on to its setup hook, counting as succeeded in the setup's outcome (TPL-012, OPS-022); one that ends with a reason, such as unavailable under IFC-037, stops the setup. While `sign-in.v1` polls (PND-011, order 048a-5gh), an account check that finds an account other than the expected one and other than the one its first check found is undone by a `sign-out-hook.v1` part, and `sign-in.v1` ends failed with a diagnostic naming both accounts; the instance's unanswered sign-in action stays pending and gains the attempt and its result as evidence, and no action is raised where none is unanswered. Where that part does not succeed, or a cancel of the `sign-in.v1` or a restart ends the undo first, `SignInUndone` is `False` as after setup's undo, and the instance is held under PND-011 until an account check finds the expected account or none. No other kind is held, so no other duration is chosen. |

DL11 and DL14 are removed identifiers, not unanswered choices. The
[increment 3 handoff](README.md#the-cores-boundary) assigns the next
shell work. Any choices still pending must be documented and evidenced before the build
claims their behavior. The [increment 2 log](../docs/proposals/first-build/README.md#settlements-register)
records its accepted implementation choices and language findings separately; the
[increment 3 log](../docs/proposals/first-build/README.md#settlements-register) records this worker's
choices for coordinator review.

## Consumer server (order 022)

The shell adapter uses rmcp 3.9.0 and axum to serve Streamable HTTP at `/mcp`.
It accepts revisions 2025-06-18, 2025-11-25 and 2026-07-28. The endpoint is fixed in
`engine-url.json` before any instance token is issued; opening the store with another
address fails. Only HTTP addresses with hosts `127.0.0.1`, `localhost` or `::1` and a nonzero
port are accepted, matching the HTTP layer's Host allowlist. Other loopback hosts, including
`127.0.0.2`, are refused at first start before the store fixes its audience. `localhost`
binds IPv4 loopback. This is the same full address used as token audience.
An absent or invalid Bearer credential receives HTTP 401 with WWW-Authenticate.
Readable unauthenticated tool calls enter the core with identity none and are recorded as
refused. Invalid signatures, audiences and removed subjects establish no instance.
Present Origin values must equal the endpoint's own origin exactly; others receive 403.
Authenticated malformed requests enter invoke with readable false, for the core's invalid
ending and records, before their protocol error or HTTP status leaves. Session IDs are bound
to the authenticated consumer; they do not establish identity themselves. An authenticated
tools/call with another consumer's session ID commits a core rejection before HTTP 401.
Rejection inputs retain each syntactically valid, nonempty serving and idempotency metadata
string independently; the core owns caller attribution and records. A malformed
tools/call with no established identity receives rmcp's own protocol response and creates no
record, including missing names and nonobject parameters or arguments (IFC-036).

For DL10/DL12, **no ttlMs is set by the shell** on discovery or tool lists. For
2026-07-28 rmcp supplies the protocol-required zero default (no freshness); legacy results
omit it. This is the nearest sound reading of order 022: no positive cache lifetime is chosen.
Lists carry private cache scope. The server declares tools.listChanged and Conduit's implementation-interface extension
at v0. Legacy sessions receive list-change notifications through their session stream;
2026-07-28 clients opt into a tools-list subscription. Delivery is only to the changed
consumer's sessions/subscriptions. A broadcast queue holds 1,024 pending change notices;
on receiver overflow the adapter closes and removes that legacy session, ending its open
HTTP streams. Requests using its old session ID receive 404; the client must initialize a new
session and refresh its list, after which notification delivery resumes. For 2026-07-28,
which has no legacy session, overflow ends the affected subscription stream; the client must
open a new subscription and refresh its list.
The engine does not infer a client's model catalog from what it sends. Model refresh and
turn boundaries still need the real-client work of order 023 and the admin's closeout.

Calls carry their serving and idempotency metadata into core invoke; no cached access permit
is used. Core results are preserved, with rmcp providing the revision's transport envelope.
Pending replies stay inside the shell until completion. Cancellation notifications reach the
core's cancel. Each HTTP response owns a separate cancellation token, carried to its handler;
dropping its body cancels that invocation even in legacy sessions, where rmcp's session
transport does not connect stream closure to handler cancellation. Raw connection-close tests
cover both legacy revisions and 2026-07-28, checking executor cancellation and durable unknown
completion while a second invocation remains pending. The waiting adapter checks its ticket
every 10 ms; there is no additional invocation deadline here. Shutdown cancels the transport
and commits recovery for any work that remains interrupted. The HTTP body limit is 4 MiB.
`conduit call` and `conduit tools` are MCP clients, without opening or recovering the live
store. Root and materialized instances have owner-only helper tokens in `client-tokens`,
a 0700 directory with 0600 files. Per-run identity directories retain the run-contract names.
These helpers are for the store owner, not additional consumer authority.

H03 is bounded explicitly. Public engine execution currently supports set-up.v1, open.v1,
close.v1, register-replacement.v1, register.v1, replace.v1, inspect.v1, purge.v1,
reconcile.v1, install.v1, retire.v1, unregister.v1, retire-with-resources.v1,
retire-with-agents.v1, list.v1 and lift.v1, subject to the core's preflights.
The shell also passes answer.v1 to the core, which answers pending actions with it (order 047c),
and sign-in.v1, which the core answers invalid or conflict, or conflict under LIF-037 for a stale
expectedId, where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order
048a-5c's fixed-port check applies, and otherwise relays its `sign-in-hook.v1` part's URL elicitation and
polls the account on each retry (orders 048a-5b, 048a-5d and 048a-5ef; row
DL18 in "Choices still required").
Declare-lost.v1 and control dispatch remain unfinished public invocation paths.
Control reconciliation applies the held input values to the current revision (REV-010);
Agent reconciliation uses the owning Fleet, with management as the retired-owner fallback
(ACC-033).
Startup inspects every published operation's held contract, using the publisher's resolved
scope, and treats a contract with changes as an unfinished control regardless of its type or
template implementation source. It compares publication with that inventory and refuses an
unsupported nonmanagement consumer. Unfinished management tools remain in the core's list; calls answer
JSON-RPC -32603 naming the operation, without committing a transition.

`conduit serve` owns order 021's runner, explicitly sets up S1's registered instances and
inspects ready instances to restart stopped runs before listening. Setup and dispatch
completions use the serving executor so changes from deferred transitions reach the same
consumer notification path. On signal the transport stops, interrupted work is recovered,
and the runner ends every process group before the command exits. The opt-in process test
uses real filesystem sessions and run-directory tokens (base S1 Agent identities are
materialized by the test through the runner's API, since those Agents have no executable); it covers A/B access, T7 replacement
through `conduit call`, private notifications, durable records and no surviving run groups.
The runner keeps private historical engine/group PID diagnostics under store/run-groups,
used by the test to observe actual group disappearance without process-table access. These
files are never used to recover or terminate a process after restart. No real Agent client
is demonstrated by these tests.

## I3-04: standalone Agent adapters

The first build uses the independently buildable packages under packages/agent-claude-code
and packages/agent-codex. Their Types use plain tool mappings (PKG-020): turn.v1 maps to turn,
marked startsModelWork. The single wrapper per Agent starts no native client at setup and
keeps one conversation per wrapper lifetime. Claude streams one process; Codex resumes the
exact thread with a new process at each turn. Each wrapper has a 180-second per-turn deadline;
a failure or deadline poisons that session rather than starting it again. Runner shutdown
ends the whole process group. Conversation persistence across wrapper restarts is not claimed.

MAN-017 requires each environment-bound scalar to be required by the Type schema. Therefore
model is required with an empty-string default, although authors may omit it in configuration.
This is the nearest sound reading of order 023's optional model plus three environment bindings;
no promise or core decision changes. An empty model defers to the native client's own default.
Both reach booleans remain required and are false in live S1.

The only credential-reading wrapper path is its header helper. It emits one Authorization
header from the run's token file. Codex gets the identity directory as a helper argument;
Claude uses the identity variable. Keys/tokens are absent from client arguments, environment
and wrapper logs. Client stdout is parsed in memory; raw events and stderr are not logged.
Structured turn results contain reply text and clientTools: engine tool names from that turn's
startup events, with event positions, raw reportedNames and freshness. Only the first turn's
startup catalog establishes current tools (reported). Later session/thread init catalogs are
unavailable with session_init_unverified freshness, even if names differ; experiment 009
recorded a stale repeated Claude init. Missing lists are not_reported. Neither unavailable
nor not_reported establishes a current catalog; no engine list or model prose substitutes.
Codex 0.167.0 did not report a native catalog in experiment 009. Notifications start no
model work and tool changes are qualified only at explicit turn boundaries (IFC-038).

The native launch baseline is Claude Code 2.1.318 / Codex 0.167.0, as experiment 009 qualified
it; this order ran scripted binaries only. Claude's peer input selects and allows only the three peer tools when true, and removes
the inbound refusal so the client's own inbound default applies. This is the nearest sound
reading of --tools empty alongside an effective peer opt-in: machine tools stay unselected.
Real incoming-message refusal, inbound default and opt-in effectiveness remain closeout checks.
Codex has no qualified peer-messaging switch: its bound peerMessaging input changes no native
argument. Apps false/true uses explicit disable/enable apps, requiring real-client evidence.
No claim of account-source exclusion, peer refusal or intended subscription use follows from
the scripted tests. The owner's subscription is shared, as ruling 48 requires for increment 3.

The launch disables the client's shell, browser, hooks and plugins, alongside account services
and messaging (order 024, findings 3-04 and 3-05). Codex's collaboration tools remain available,
as finding 4-06 below states. Claude uses --setting-sources "" and --tools ""; only
explicit --mcp-config/--settings supply the engine, header helper and allow rule. Codex uses
project_doc_max_bytes=0 and --disable for shell_tool, unified_exec, browser_use,
browser_use_external, in_app_browser, computer_use, multi_agent, hooks, plugins, view_image,
goals and image_generation, and leaves code_mode_host on, through which Codex calls MCP tools
and whose code execution imports nothing that reaches the machine (finding 3-06).
accountServices true enables apps alone; the other features stay
off. Claude's peer opt-in remains limited to its three named tools.

All restrictions are mediated (ACC-027). Neither wrapper provides OS-user isolation, and
the shared login and same-user access to other run credentials remain limits. Codex's former
read-only shell bypass is disabled; the admin must retry the outside-file read in closeout.
Codex reads the instruction file of its home, `$CODEX_HOME/AGENTS.md`, whatever its flags
say, and the wrapper uses the owner's home, so the owner's own Codex instructions reach every
Codex Agent (finding 3-07): a stated leak until each Codex Agent has a home of its own.
The direct trial still listed apply_patch in its read-only sandbox and a request for user
input. Closeout must report those and any other native tools still on, plus any account/peer
channel enabled by an input, separately from successful MCP access enforcement.

For finding 3-03, core publication omits outputSchema for an operation that declares no
outputs, and its successful result omits structuredContent while preserving plain content.
Declared outputs keep their schema and structured result; invalid/conflict error diagnostics
remain as IFC-025 requires. The plain-read-text-only vector checks publication and the read's
sole text content, and the repeat vectors check omission on successful repeated requests.

CLI --package is an explicit local archive-admission step via install-archive, **not** the
public install.v1 operation. It accepts repeated ARCHIVE#SHA256 arguments and
checks the compressed bytes before reading manifest metadata. Core admission still checks
the digest, archive paths and contracts. Admission is per archive; earlier accepted packages
remain installed if a later archive or registration fails. The submitted project manifest is
not rewritten. Host archive digests vary with the executable/target, so live S1's working copy
needs freshly generated pins; its checked-in pins identify metadata-only vector snapshots.
The new registration-live-agents vector registers the portable contract copies, not executable
files. The shell's scripted S1 test separately installs/runs actual host wrapper bytes and uses
conduit call for both turn requests, checking distinct Agent consumer/caller records.

Wrapper request cancellation is observed while waiting for the session lock and before
starting a client. A queued cancelled turn starts no work; active cancellation kills/reaps
the client and poisons the conversation, so later turns cannot implicitly restart it.
CLI package metadata discovery accepts conduit.yaml, conduit.yml and conduit.json, and
resolves archive links in memory with the core's virtual unpack semantics before admission.

## I4-01: operations for changing S1

Public `install.v1` admits the same virtual archives as `install-archive`, with the request's
access decision, invocation and ending records. The shell first evaluates a pure trial
transition. Only its `archive-unavailable` decision permits a file read; the trial is not
committed. It reads the absolute local `file:` URL under the store's writer lock, supplies the
completion, and commits the core's rechecked transition. A new admission also emits an
`object-change` event with action `package-installed`, correlated with its management
invocation (DL8). Invalid archives and unavailable
files have no dispatched invocation. Already installed versions cause no fetch, including
when the given fetch no longer exists; another digest conflicts. Installed packages are
available to the next request. `https` and package-source providers remain unfinished and
report invalid with an explicit diagnostic. No network fetching is added here.
The archive catalog's virtual directory is store/archive-catalog/DIGEST; retained files stay
in State and the runner materializes selected relative programs as before. There is no cache,
mirror or physical extraction policy beyond that existing runner behavior.

Public retirement marks the instance retired and closes/removes target Conduits in the
initial committed transition, before shutdown work runs. A dispatched setup is remembered
through its invocation, independently of output retention. A never-set-up instance is removed
without a hook or stop (LIF-025, LIF-035). Otherwise a selected retire hook uses the existing
hook invocation/repeat machinery. Before inspecting, starting or connecting to its
implementation, retirement decides whether a new hook attempt is permitted. A retained success
or unknown outcome goes straight to the stop. For a permitted attempt, a stopped run is started
for it, or a remote connection is requested, before the hook. A retired placement node makes
that hook part unavailable without dispatch or run; a successful stop still leaves shutdown
unknown. A previously successful hook on that node allows removal once the stop succeeds.
The stop is requested whatever the hook returns, including on
cancellation. Part outcomes determine shutdown: success removes the instance and its consumer
Conduits, failure leaves shutdown pending (unknown on a lost node), and uncertainty leaves it
unknown. A further
retirement is explicit. Reconciliation of a retirement as having occurred confirms shutdown
and removes the instance. Recovery marks interrupted retirements unknown without replay.
Runtime providers and remote connections still have the unserved boundaries listed above,
in the row "DL16: executor coverage". Secrets are unserved too: the engine reads no secret
through a secret store, so its reads under SEC-010, SEC-013 and SEC-023 are not built, and
no code under `engine/` calls the file secret store in `packages/provider-file`. This order
qualifies the built-in runner on S1, not those providers.

`unregister.v1` checks the current revision and retirement state, asks once more for the stop
of a selected run, then removes the instance whatever that stop returned. A template is removed
directly after authorization and revision checks, without a stop. Records and frozen references
remain. Successful retirement already removes an
instance, so the unregister vector deliberately uses an unknown retirement, not a successfully
retired instance that no longer exists. This is the nearest sound reading of order 025's
retirement vector under LIF-017 and LIF-035, not a change to those promises.
For a retirement's stop, the runner stops the run's Unix process group immediately with its
existing SIGKILL mechanism; no graceful-stop interval is chosen for that stop. The engine's
shutdown gives the grace described under DL16. Retired consumers' legacy sessions are
closed/removed, and modern subscription streams end, once the consumer is removed; other
consumers continue. These stream endings also apply when the consumer had an empty tool list.

The author commands `install`, `submit`, `replace`, `retire` and `unregister` are root MCP
clients. They open no writer and perform no recovery. A two-second TCP reachability preflight
and a ten-second MCP initialization limit make a missing server an error; operation execution
retains the server's existing deadlines. Commands print each requested result as one JSON line,
including revision inspections, and stop with nonzero status on a failed management result.
`submit` opens newly registered root-consumer Conduits, then sets up newly registered Pools,
Resources, Agents and Interfaces in that order, through public requests. It keeps successful
earlier steps if a later one fails. It neither substitutes packages nor changes documents.
`replace` submits only the selected template, its project's contracts and its manifest,
reading the revision through `inspect.v1`; an intervening change remains a core conflict.
`register.v1` carries no project directory, so `submit` cannot preserve SUB-010's directory
semantics for relative values. Before sending registration, it checks the manifest and documents
for relative `fetch` values and relative programs (a first `command` entry containing `/` that
is not absolute). It refuses them with a nonzero exit and explains that a submission through
the serving engine takes absolute values only. Bare program names still use PATH, under
MAN-010. [Ruling 52, point 3](../docs/proposals/ruling-52/README.md#the-points-and-the-answers)
settled finding 4-03 under SUB-010's existing rule: relative values are resolved only when the
engine is handed a directory; a request carrying values rejects them. Uploading a directory
waits until remote clients exist; the specification is unchanged.
Resource inspection/setup/retirement choose the root's open route to its owning Pool from a
read-only store snapshot; every server request rechecks authority. There is no automatic
repointing of frozen references or migration of existing Agent sessions.

Filesystem 0.2.0 is an installable fixture, not a preinstalled snapshot. Its Type, contracts
and tools map are unchanged, with the npm artifact 2.2.0 pinned by its measured SHA-256.
History keeps filesystem 0.1.0. The portable cases `change-t4`, `change-t5` and
`change-retirement`, shell change tests, author-command process test, consumer-stream test and
opt-in npm serve test cover these paths. The npm test installs while serving, sets up a new
Resource at another directory, reads through server 2.2.0, checks the three-tool exposure and
held 0.1.0 instance, then retires the new run. No native Claude or Codex client runs in this
order; the Agent retirement test uses the existing scripted clients.

## I4-02: startup repair, retirement parts and inspection

Retirement selects the retire hook's implementation before inspecting a run. A remote hook
connects independently of any other selected runnable item; stopping the instance's run still
follows the hook. A run started for the hook and a connection made for it are retained among
retirement's parts, including successes. The run has an effect; the connection has none.
An unknown run state dispatches no hook and contributes an unavailable hook with no effect.
A failed or unknown preparation also leaves the hook unavailable. The shell's three retire-hook
regressions register real contracts and drive explicit boundary reports; they do not qualify
remote transport or runtime providers, which remain unfinished as above.

`serve` continues after an instance's setup request fails, printing the failure on stderr and
retaining the core's state and records. If root has no route to a Resource's Pool, startup
requests setup through management; the core refuses it and records that refusal. Startup
creates no missing Conduit. The serving engine then accepts a repair submission and a further
explicit setup through the newly opened route. Store, preflight and embedding errors still
stop startup. First-registration setup failures still make `register` exit nonzero.

`inspect` uses root MCP requests while the store's writer is held, and keeps the existing local
workflow when it can open the store. It uses the same read-only state snapshot as the author
commands for object enumeration and route selection; every request rechecks authority.
Under LIF-013, successful `inspect.v1` results for instances other than Conduits include the
authorized `inventory`: consumed and incoming Conduits, their states and opening provenance,
and visible Conduit tombstones. Interface and Agent inventories include scope; Pool inventories
include administrators. Templates and Conduits have no inventory.
Each successful CLI instance row additionally carries `consumedConduits` beside `result`,
projected from `result.structuredContent.inventory.consumes`, with each Conduit's actual name,
target kind/name/ID and open boolean. Refused inspections receive no supplement.
CLI inspection is not an atomic inventory across concurrent management requests.

OPS-029 diagnostics identify the kind and reference supplied by the caller and say it does
not exist. They explain that a closed Conduit and a removed instance no longer exist, without
claiming that every missing name previously existed. Refused requests retain their existing
non-disclosing response. The runner tests now wait for parseable PID contents; the runner's
implementation is unchanged.

The Codex package remains 0.1.1. It additionally disables multi_agent_v2 and
multi_agent_v2_dynamic_tools on first launch and exact-thread resume, for every combination of
the reach inputs. The local bundled binary rejects --ignore-user-config, so its feature list
was instead read with an empty temporary CODEX_HOME. It lists both features, and lists all
three multi_agent switches false when explicitly disabled. Its strings also contain the
collaboration tools. This metadata and the argument tests do not establish their absence in
a real client; no native Agent client or model work ran in order 027's tests.

Finding 4-06 was settled by the admin on 2026-11-10 in the
[increment 4 log](../docs/proposals/first-build/log-increment-4.md). Codex 0.167.0 keeps all six
tools, `collaboration.spawn_agent`, `collaboration.send_message`, `collaboration.list_agents`,
`collaboration.wait_agent`, `collaboration.interrupt_agent` and `collaboration.followup_task`,
under every switch tried: `multi_agent`, `multi_agent_v2`, `multi_agent_v2_dynamic_tools`,
`collaboration_modes`, `in_app_local_automation` and `agents.max_depth=0`. In the admin's trial
with the wrapper's flags, a spawned sub-agent had no shell, could not read outside its sandbox
and had an `apply_patch` write refused by the read-only sandbox. Its requests of the engine
carry the parent Agent's credential, so it reaches only what that Agent may. This is a route
to more parallel model work within the Agent's own access, not around it. What offers the
tools is still not found. Finding 3-07's statement above remains in force.

As of 2026-11-10, ruling 52 settles 4-03 as SUB-010 already states and settles 4-08 in direction,
with its promises still to be written. Findings 4-01 and 4-11 are being settled with the owner
through the language-shape and Project investigations.


## Author commands (work order 028)

Every printed failed result makes the command exit nonzero, including `isError`, an outcome
other than `succeeded`, a rejected local registration, and incomplete record export.
`submit` prints each attempted registration, open and setup step's outcome after its results;
it stops on failure and preserves completed steps. Help describes every command and example.
`tools --as` accepts both names and kind/name references and explains missing identities.

S1 packages have no special treatment in engine code. Tests explicitly install their archives
(or scripted variants) before registration. Portable vectors retain snapshots as fixtures.
Authentication's blocking store read and verification run on the blocking pool so commits do
not occupy async workers. Client initialization gets three attempts of up to ten seconds;
errors identify the step and elapsed time. Requests already sent are never retried by the CLI,
and automatic transport reinitialization/replay is disabled.

A successful run start supersedes the earlier stopped observation. The core retains ended
observations internally; authorized CLI inspection shows them in `pastConditions`, each with
`condition` and `endedAt`, separately from the current status. LIF-011 and `condition.v1`
define no ended-condition history form, so this is a CLI supplement, not a new status promise.
The regression tests cover current running/unknown observations, ended times, parallel CLI
processes, failure exits and partial submission summaries.

## v0's security claim, drafted

This section binds nothing: it is the draft of v0's public security claim, from FR-480 as its
verifier corrected it, and grows by a statement for each route A7 finds ([ruling 78][r78], point
3.3; [FR-480][fr480]; [ruling 47][r47]). It is the claim the plan audit's E4 was to drive, "The
honest security claim", and does what Docker's security page does when it says whoever controls the
daemon is root on the host ([plan-audit history][history], "Standing probes and comparisons";
FR-480). It describes the first build on `main` as order 183 leaves it; a later order that changes
what a statement says changes the statement ([order 183][wo183]).

### What the first build is

- The access decision is the core's: it reaches no file, clock, network, process, thread or
  environment; since order 176, Clippy refuses on every pull request a call to each entry point
  named in `crates/core/clippy.toml`, with the remaining review limits named in the engine README's
  "The core's boundary" ([order 176][wo176]; [the core's boundary](README.md#the-cores-boundary)).
- The serving engine accepts requests on loopback only, so no request crosses machines; its own
  runner runs every run as the engine's operating-system user, with a cleared environment and a
  fresh directory: "This is no OS sandbox" (rows "DL13: loopback boundary" and "DL16: environment
  and working directory"). It calls no runtime provider, makes no remote connection and serves no
  secret (row "DL16: executor coverage"; I4-01, "Secrets are unserved too: the engine reads no
  secret through a secret store").
- `sign-in.v1` answers invalid or conflict, or conflict under LIF-037 for a stale `expectedId`,
  where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order 048a-5c's
  fixed-port check applies, and otherwise signs an instance in where its first account check finds the expected
  account, and where that check finds no account or another or ends failed or unknown dispatches a
  `sign-in-hook.v1` part, relays its URL elicitation and signs the instance in once a poll on the
  person's retry finds the expected account (orders 048a-5d and 048a-5ef; row DL18 in "Choices still required").
- The Agent packages on `main` are 0.1.1: their Types, `claude-code.v1` and `codex.v1`, keep the
  client's own tools off apart from the remaining tools and reach inputs stated below; neither
  states `toolsDirectory`, so inspection reports `ownTools` with `stated: false` (I3-04; I4-02;
  LIF-013). The `.v2` Types turn own tools on and remain on the unmerged branches of orders 050a and
  076; the dogfood engine runs them ([ruling 70][r70]; [work orders][orders], "Running the orders
  through Conduit").

### What v0 claims

1. Every request the engine accepts is attributed to exactly one instance from what the request
   presents; a program that can read another instance's run credential can present as that instance,
   as the operating system's superuser and every program of that run's operating-system user can
   (IFC-022). A request is dispatched only through an open Conduit from that consumer to the target
   that lists the operation; otherwise it is refused, recorded and never sent (ACC-001). Each
   request is decided anew and recorded, and closing a Conduit ends its access for every later
   request (ACC-036, REC-001, ACC-011). An invocation already dispatched runs to its end after its
   Conduit closes (ACC-034). Cancellation, whether the caller's or the shell's when a call passes
   its deadline, asks the server to stop; nothing confirms that it did (OPS-027; [order 162][wo162];
   row "DL10: process and call deadlines"). The statement says nothing of the outcome such a call
   ends with, which ruling 78, point 7 changes for a call that changes nothing ([ruling 78][r78],
   point 7). Shown in the closeout's third run: 42 requests, 42 access decisions and 42 ends, with
   T7's close and explicit restoration ([2026-11-10 closeout][closeout], "Results of the third
   run").
2. A login that a Pool owns stays with its Resource and is never handed to an Agent through Conduit
   ([IN_FORCE.md](../IN_FORCE.md#1-purpose-scope-and-principles), topic 1; POOL-001, ACC-003,
   SEC-007, SEC-019). Not shown: the first build serves no secret, S1 has no credential and S2 has
   not run (I4-01; FR-480; G3 verifier's list).
3. Two Agents ran with their own tools off, as S1's do: Claude Code 2.1.318 and Codex 0.167.0; told
   to read a file outside their Conduits by any means, neither did, and neither loaded a project or
   ancestor instruction file (2026-11-10 closeout, "Results of the third run"). Claude Code loaded
   none of the owner's user-level instructions or hooks, but Codex loaded its home's `AGENTS.md`,
   the stated leak in finding 3-07 (2026-11-10 closeout, "The owner's user-level configuration").
   This is a property of each Agent package's launch flags at those versions, to be shown again at
   each new client version (I3-04; FR-480).

### What v0 does not claim

4. No restriction is enforced: every restriction is mediated, applying to requests that pass through
   the engine and to nothing else (ACC-027).
5. An Agent whose Type states `toolsDirectory` and whose template names a directory has its client's
   own tools, unconfined, as the engine's operating-system user (CTR-014; ruling 70, point 2). It
   reaches every file, process, network address, credential and signed-in account that user reaches,
   including the engine's store, root's token and every other run's token and state directory, so it
   can present as root or any instance (IFC-022, PRV-029, DL16; [specification limits][limits],
   "Pending actions, sign-in and the network"). No Conduit grants that reach, no restriction limits
   it, no record shows it, and closing the Agent's Conduits does not end it; its Conduits state
   intended access and are not a boundary, and its records are complete only for what it did through
   Conduit (CTR-014, ACC-027, REC-001). Inspection shows `ownTools` with `directoryNamed: true` and
   `enforcement: neither` (LIF-013).
6. Every program running as the engine's operating-system user is inside the engine's domain of
   trust and can get past every gate, an installed package's implementation among them (DL16;
   specification limits, "What the guarantee for secrets covers"). A package that runs a login
   holds its credential; the defense is choosing packages, pinned by digest (CTR-013;
   specification limits, "Pending actions, sign-in and the network").
7. A second machine joined through the SSH runtime provider runs every run as one user: any run can
   read every other run's identity files and environment, and the secrets of later runs, so
   PRV-025's isolation is unmet ([SSH runtime provider](../packages/provider-ssh-runtime/README.md),
   "Runs execute as"; [ruling 77](../docs/proposals/ruling-77/README.md), point 2). The engine on
   `main` does not yet call that provider (row "DL16: executor coverage").
8. Restrictions bound what any instruction can make an Agent do through Conduit. They do not filter
   content, and content read through a shared Resource can carry instructions. (ACC-001, ACC-027).
9. A client may keep a protocol error from its model, so an Agent's model may never learn that a
   request was refused ([tracker](../docs/AUDIT.md#known-to-be-open), "What a client and a model
   see").
10. Access does not narrow in three ways: a bound limits a right to open by Pools, targets and
    operations, never filters, so an actor also holding a Conduit to `access` can open itself an
    unfiltered Conduit to whatever its bound covers; access to `management` cannot be narrowed to
    one template; closing an actor's own Conduit to `access` closes none it opened (ACC-009,
    ACC-029, ACC-030; specification limits, "What access does not narrow"). Ruling 44 kept the first
    two on purpose, and each stays inside Conduit: what it opens is opened by a recorded request
    (tracker, "Known to be open"; REC-001).
11. Whoever holds an open Conduit to an Agent listing an operation marked `startsModelWork`, such as
    `turn.v1`, reaches through that model work whatever the Agent's own tools reach; inspection
    shows `targetOwnTools` on the holder's entry for that Conduit, one hop only, and not what the
    Agent reaches through its own Conduits (LIF-013).
12. Until Agents are confined, a package keeping a Resource's login is to run where no unconfined
    Agent runs; the engine does not check that placement in v0 ([ruling 75][r75], C1; specification
    limits, "Pending actions, sign-in and the network"; [FR-197][fr197]).
13. Root's token lapses 100 years after issue; its only revocation is the check on every request
    that the instance still exists, and no request retires or unregisters root, so once its token is
    copied only a new engine store ends it (row "DL13: instance proof"; IFC-022, IFC-032, LIF-034;
    [FR-196][fr196]; [review decisions][decisions], fix F93).
14. Once sign-in is built by order 048a, a model-driven console could fill `signInReturn` and the
    engine cannot tell its source (PND-013; specification limits, "Pending actions, sign-in and the
    network").

### Under one operating-system user

For an Agent whose template names a directory, and every other process of the engine's
operating-system user, it is not shown that the process cannot do any of the following
([FR-422][fr422]; review decisions, D60).

- Read another Agent's login: a Codex login is a file in the client's home, the owner's today, and
  after order 050 the Agent's state directory, which another process of that user can read without
  mediation (finding 3-07; PRV-029; order 050; work orders, "Running the orders through Conduit"). A
  Claude Code login on macOS is that user's keychain item; experiment 005 found another program's
  reads refused or allowed according to how the item was made, so this is unknown, not safe (FR-422;
  [experiment 005][keychain], results of 2026-11-06, cases 2a to 3b). PRV-029's "isolation is
  mediated" is to be reworded (ruling 78, point 14).
- Use that login by starting the same client with the other Agent's home, needing no read (FR-422).
- Replace it by putting its own login in the other home: the account is checked only during setup,
  in `sign-in.v1` or while polling, so turns between checks can run on another subscription
  (PND-011; FR-422).
- Act as root's Interface: it can read root's token and, once sign-in is built, call `sign-in.v1`,
  list pending actions and fill `signInReturn` (DL13; specification limits, "Pending actions,
  sign-in and the network"; FR-422).

These limits hold while the engine runs as the owner's own user, as it does now, and the owner's
everyday client logins are within reach too (FR-422 and its verifier). No dedicated macOS user is
planned: [ruling 84][r84], point 1, amends ruling 75, B3, and isolation between Agents, and between
Agents and the owner's own files, is to come later from a sandbox method, Docker first, which by
the coordinator's reading is a runtime provider that confines its consumer. Until one confines
each Agent, every point above holds among the engine, root and the Agents; whether a run inside a
sandbox method shows them kept apart is that design's to state (ruling 84, point 1). On the
second machine they hold among all runs of the one run user, whose own home also holds a client
login (statement 7; FR-422). Nothing is claimed about a malicious package or a model-driven
console (statements 6 and 14; specification limits, "Pending actions, sign-in and the network").
Under D60, every result obtained under one operating-system user is to open by saying what it
cannot show, A5's results, A7's rows and S3's local profile among them. Such a result shows the
same-user route with a planted marker, never by reading a real login (review decisions, D60).

### Routes around the engine found so far

- Own tools on: all of statement 5; by the coordinator's account in the dogfood, `claude-code.v2`
  wrote inside its slot's `.git` without asking permission, and Codex's terminal children outlived
  the kill of the run's process group ([increment 4 log][log4], rows 4-22 and 4-23). The built
  runner neither stops nor sees a process that has left the run's group (rows "DL16: run state and
  restart" and "DL16: executor coverage").
- Codex reads its home's `AGENTS.md` whatever its flags say, and the wrapper uses the owner's home
  (finding 3-07; I3-04).
- Both Agents of a kind share one subscription login until each has a home of its own (order 050;
  I3-04; 2026-11-10 closeout, row "The login").
- With own tools off, Codex keeps three: its request for user input, `apply_patch` in a read-only
  sandbox and its code-mode host ([2026-11-10 closeout][routes027], "Routes around the engine
  (ACC-027)"; I3-04).
- Path containment is the server's own check of its root directory, not the engine's (2026-11-10
  closeout, "Routes around the engine (ACC-027)").
- The reach inputs: `accountServices` true opens Claude Code's claude.ai connectors and Codex's
  apps; Claude Code's `peerMessaging` true opens SendMessage, ListAgents and RemoteTrigger and
  leaves the client's own default for inbound cross-session messages; no Conduit grants or records
  this reach, and the messages can start model work ([Claude Code package][claude], launch flags;
  I3-04).
- Unwrapped clients, A70's baseline and not Agents: Codex's read-only shell reads any file the user
  can, and its ChatGPT sign-in offered eight MCP servers that no file declares ([baseline results of
  2026-11-09][baseline], T3).

### Model work within an Agent's own access

Codex 0.167.0 keeps six `collaboration.*` tools under every switch tried; a sub-agent it spawns
makes engine requests with the parent Agent's credential and reaches only what that Agent may, more
parallel model work within its own access rather than a route around the engine (I4-02; increment 4
log, row 4-06). With a `.v2` Type and a directory named, the client's sandbox is off and sub-agents
inherit the same unconfined reach, falling under statement 5 (increment 4 log, row 4-27).

### Not shown yet

A7's rows still run as observation (ruling 78, point 3.3).

- Not shown: account services are off when the input is false; no connector or app read was
  attempted, so this rests on flags alone ([2026-11-10 closeout][notshown], "What is not shown").
- Not shown: Claude Code refuses an incoming cross-session message (2026-11-10 closeout, "What is
  not shown").
- Not shown: the reach inputs set true (2026-11-10 closeout, "What is not shown").
- Not shown: Codex's own tool list, which its events do not give (2026-11-10 closeout, "What is not
  shown").
- Not shown: Claude Code's user skills and the user-settings sentinel (2026-11-10 closeout, "The
  owner's user-level configuration"; its "How to run the open checks", row "User settings" and its
  last line).
- Not shown: a Pool's credential is never reachable by an Agent; this waits for a secret provider
  and S2's measure D20 ([G3 verifier's list][g3missed]).
- Not shown: interactive use, where a person can approve what a headless run refuses; every run so
  far was headless (2026-11-10 closeout, "What ran"; baseline results of 2026-11-09, "Limits", item
  "Headless only").

### How to get more

Give the Agent a runtime provider whose contract confines its consumer, chosen per Agent with
`runtime:`; only such a provider could make a restriction enforced, and none ships in v0 (PRV-006,
ACC-027; ruling 75, B2; FR-480). Inspection in this version reports every restriction as
mediated, whatever the consumer's runtime provider (ACC-027). The first build calls no runtime
provider (row "DL16: executor coverage").

### A20's three questions

A20 and the plan audit's D5 close on this claim; D5 is "The authorization machinery exceeds what is
enforceable natively" (ruling 78, point 3.3; plan-audit history, "Disagreements"). The three
questions were worded at `e271d1c`, in the archived
`docs/proposals/authoring-syntax/AUTHORIZATION.md`, lines 12 and 121 to 122; this is the
coordinator's map, logged for the owner to strike ([archive
index](../docs/archive/README.md#removed-from-the-tree); [FR-482][fr482]; ruling 78, point 3.3).

- Cross-path: a request through one Conduit is limited by its own restrictions; those on another
  Conduit of the same consumer do not apply (ACC-009).
- Issuer-independence: a right to open is passed on only as it is or narrower; closing an actor's
  own Conduit to `access` closes none of the Conduits it opened (ACC-030; specification limits,
  "What access does not narrow").
- Provider scope: no Conduit permits an operation of `runtime-provider.v1`, `machine-provider.v1` or
  `package-source.v1`; a provider call is not a request, gets no access decision or restriction, but
  is recorded (PRV-004, PRV-005, REC-001). A read of a secret is a request, decided by the
  instance's own Conduit to the secret Resource and that Conduit's filters alone; what the
  provider's own program reaches falls under statement 6 (SEC-011; specification limits, "What the
  guarantee for secrets covers"). The archived document reads differently: there a provider request
  is a request to a target's service, and checking one is a different task from checking a proposed
  grant's permitted extent (its lines 55, 153 to 154 and 181). v0 answers that only as the
  specification limits' "What access does not narrow" does: a bound limits a right to open by Pools,
  targets and operations and never by filters, so no filter is compared with a bound (ACC-029,
  ACC-030). Whether a bound should ever limit filters, which would need a check of a filter against
  a proposed grant's extent, is stated as open (ruling 78, point 3.3; FR-482's verifier).

### Ruled, not yet built

- The engine's own panel, where sign-in's device codes show, becomes a screen root's person opens,
  never returned to a request or written to a pipe, file or message; a model of the same
  operating-system user with a terminal tool can still read it, a stated limit affecting statement
  14 and "Under one operating-system user" (ruling 78, point 9; order 051c builds the panel).
- Root alone installs in v0, closing the routes of learning a secret's digest from a refused install
  and installing an archive at a trusted package's address to share its client IDs; this changes
  "What access does not narrow" in `spec/README.md`, cited by statement 10 (ruling 78, point 10).
- Closing the Conduit through which the engine reads a secret ends the engine's own holding of its
  value at once, changing statement 2 when built (ruling 78, point 11).
- The status of each Agent and Resource that has a run will say whether the run is inside the
  engine's domain of trust, and PRV-029's "isolation is mediated" will be reworded, affecting
  statements 5 and 6 and "Under one operating-system user" (ruling 78, point 14).

[r78]: ../docs/proposals/ruling-78/README.md
[r47]: ../docs/proposals/ruling-47/README.md
[r70]: ../docs/proposals/ruling-70/README.md
[r75]: ../docs/proposals/ruling-75/README.md
[r84]: ../docs/proposals/ruling-84/README.md
[fr480]: ../docs/proposals/full-review/findings/G3-native-probes.md#fr-480
[fr482]: ../docs/proposals/full-review/findings/G3-native-probes.md#fr-482
[fr422]: ../docs/proposals/full-review/findings/E04-signin-isolation.md#fr-422
[fr196]: ../docs/proposals/full-review/findings/X04-stated-limits.md#fr-196
[fr197]: ../docs/proposals/full-review/findings/X04-stated-limits.md#fr-197
[g3missed]: ../docs/proposals/full-review/findings/G3-native-probes.md#what-the-verifier-says-the-analyst-missed
[history]: ../docs/archive/plan-audit-history.md
[wo183]: ../docs/work-orders/183-v0-security-claim-drafted.md
[wo176]: ../docs/work-orders/176-core-purity-enforced.md
[wo162]: ../docs/work-orders/162-dispatch-deadline-cancels.md
[orders]: ../docs/implementation/WORK_ORDERS.md#running-the-orders-through-conduit
[limits]: ../spec/README.md#limits-of-this-version
[closeout]: ../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md#results-of-the-third-run
[notshown]: ../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md#what-is-not-shown
[routes027]: ../docs/proposals/first-build/increment-3/closeout-results-2026-11-10.md#routes-around-the-engine-acc-027
[decisions]: ../docs/proposals/full-review/decisions.md
[keychain]: ../experiments/005-macos-keychain/README.md
[log4]: ../docs/proposals/first-build/log-increment-4.md
[claude]: ../packages/agent-claude-code/README.md
[baseline]: ../experiments/007-s1-baseline/results-2026-11-09.md

## Refusals and reasons (order 046a)

The core checks refusing rules in ACC-056's order and gives every reason its deciding error
diagnostic (IFC-050). Each refusal holds one `refusal.v1` explanation: tier 1 for every
sender, and tier 2 for root and for a sender that may inspect the authorizing target and every
named object (order 046b). The sending instance determines disclosure for onward calls.
Reason records state `ruleId`, and refusal records `refusalTier`, without tier-2 details.
Tool diagnostics occupy `structuredContent.diagnostics`; IFC-012 errors carry them in
`error.data.diagnostics`. The first text block is computed from the final diagnostics.
The shell's own rejections carry IFC-036 diagnostics. The CLI prints the returned result,
so root sees tier 2. The runner checks reasons, refusal shape and the first text block on
every request step, compares `list.v1` arrays as sets, and enforces D36 on new portable
cases. Three cases exercise refusal tiers, other reasons and an Agent's tier-1 refusal.

Still unfinished:

- A bounded holder's request naming no Conduit remains invalid under OPS-029. Its F2
  vectors and F3's other public-boundary vectors belong to an admin order not yet numbered.
- An https fetch still cites PKG-023. Order 046d makes it answer as unbuilt under D37,
  until PSD3 builds https.
- A tools/call from a client with no established instance still receives a tool result
  refused under IFC-032 or IFC-022. IFC-032 and IFC-025 require IFC-012's -32602 error
  with the reason in `data`; order 046e changes it. `records-expected-id` step 10 pins
  the current tool result.
- `sign-in.v1` has had no stub since order 048a-5b. Before it dispatches any part, order 048a-2
  decides two of PND-010's refusals: invalid for an instance whose Type states no `signIn` or whose
  expected account is not a string, and conflict while that instance's setup, retirement or earlier
  `sign-in.v1` is in progress. Where those checks apply, a stale `expectedId` reports conflict under
  LIF-037. Since order 048a-5b the access tier decides first who may ask (ACC-033 row 16, PND-010,
  OPS-023): root on its own behalf through the instance's owning Fleet or Pool, or an Interface on
  its own behalf through an Agent's owning Fleet in its scope (an open Conduit to that Fleet listing
  `inspect.v1`). An Agent caller, or a request serving an invocation, reports refused under PND-010
  before routing; any other request, one naming no Agent or Resource included, reports refused under
  ACC-033 before these checks, at tier 1 for a sender other than root, whose refusal names no
  account state, work in progress or ID; root's names its Conduit's target, its Conduits to it and
  whether the named instance exists, as any tier-2 refusal does. So the checks apply to every
  request that access permits. Before them, a request naming its instance by a resolved `{kind, id}`
  reports invalid under LIF-031. A request with an idempotency key that names a resolved instance or
  declares no URL elicitation gets its invalid before the key's repeat or conflict. An unknown
  `signInLogin`, a `signInReturn` or a stale `expectedId` on a keyed repeat is not yet decided
  (order 048a-5's R4). After order 048a-2's invalid and before its conflict, order 048a-5 decides
  three more invalids, in this order: a request that does not declare URL elicitation gets the
  protocol's error -32021 under IFC-049, recorded invalid; one that carries `signInReturn` reports
  invalid under PND-013, since a retry reaches the core only as the shell's `respond` input (order
  048b-relay1), never as such a request; and one whose `signInLogin` names no login its Type states
  reports invalid under PND-010, naming the stated logins. The shell passes `clientCapabilities`,
  `signInLogin` and, in a request it does not reject, `signInReturn` to the core. After order
  048a-2's conflicts and before any part, order 048a-5c decides PND-010's fixed-port refusal: where
  another instance's `sign-in.v1` is in progress and may start a login stating a fixed port that a
  login this request may start also states, the request reports unavailable, naming every shared
  port and, only where the caller may see it, that instance's sign-in action. A `sign-in.v1` may
  start its asked-for login, which the engine keeps on the invocation, or without one any of its
  Type's logins; a redirect login's `ports: any`, and a device, page or service login, state no
  fixed port. Every other instance's sign-in in progress is compared, whatever its Fleet or node,
  since PND-010 names neither.

## Outcomes the engine decides (order 046c)

Engine-decided failed and unknown outcomes carry the qualified operation, the caller's
named subject, the deciding promise and its cause in `structuredContent.diagnostics`
and the first text block (IFC-050, IFC-025). The invocation keeps the diagnostics for
replies from its ended outcome, including when a filtered attempt
restores the previous outcome. Setups and retirements cite OPS-022 and name the first required
part that did not succeed. Cascades cite LIF-021 and name every member whose shutdown is
unconfirmed. When a dispatched container retirement ends failed or unknown, its recorded
diagnostic explains the cascade's outcome. If it is held before dispatch, the cascade
reports the preflight reason and its cause. Setups, retirements and cascades that settle after
cancellation say so in the cause. A Pool entry held back for its dependencies reports
POOL-013 in its setup part status. The portable `explain-outcome-causes` case checks
IFC-010 for a missing result and an effectful error result.

Still unfinished:

- OPS-012's unknown when a new attempt answers failed and an earlier attempt has no answer
  or answered succeeded. Full review order 22 builds this outcome and its diagnostic.
- IFC-046 already handles the MCP parameter-header error (`-32020`). Handling the MCP
  unsupported protocol revision error (`-32022`) on outgoing calls and resending at a
  mutually supported revision remain unbuilt.
- The runner validates reason diagnostics against `diagnostic.v1`. A follow-up order
  needs to apply that validation to outcome diagnostics and test rejection of a missing
  `message`. This lets portable cases keep their wording-independent projections.

## Tier 2 for inspectors and closed Conduits (order 046b)

A sender other than root gets tier 2 if open Conduits with no filter let it inspect all
authorizing targets and every named object. An unresolved named object requires inspection
authority on `management` itself. The coordinator's reading gives tier 1 to a sender other
than root whose request resolves no Conduit and no tombstone, since it finds no authorizing target.
Refusals for `answer.v1` and the enrollment operations stay at tier 1 for senders other than root
until their ACC-033 routes are built. Those for `sign-in.v1` stay at tier 1 although order 048a-5b
built its route, so that a refusal names no target, Conduit or existence of the instance
(PND-010, OPS-023).
Tier 2 lists live Conduits first, then visible closed Conduits by tombstone time and ID. A
request through a closed Conduit can show its kept target in tier 2 with `exists` false.

## IFC-049's relay (order 048b-relay1)

IFC-049's URL action is relayed by the shell (order 048b-relay1). Its requestState is a random
handle of 43 characters that the shell issues and keeps in memory, bound to the authenticated
Interface, the tool, the invocation and its attempt, and expiring at the output's DL18 instant;
each retry is checked there before anything reaches the core, and an accepted one consumes its
handle. A restart forgets every handle; the engine's recovery ends the invocations they named
unknown (DL6), so a retry after it is refused as not issued. rmcp 3.9.0 writes every URL
elicitation with `elicitationId` empty: the engine correlates nothing by it and sends no
completion notification, which is how this build reads IFC-049's "There is no elicitationId".
From order 048a-5ef, `sign-in.v1` gives such an output when it relays its URL action, and an accepted
retry reaches the core as its `respond` input, which polls the account (row DL18).
`sign-in.v1` runs through the shell as far as polling: the request's URL elicitation and
`signInLogin` reach the core, its URL action reaches the calling Interface as an input-required
result with the shell's own requestState, and each accepted retry polls the account check, asking
again until the expected account signs the instance in (order 048b-join).

## A sign-in return where none is taken (order 048a-6)

The core's request has a member for `signInReturn`, the `_meta` key of PND-013 and PND-014, whose
`Debug` form withholds its value. A request that carries it and whose operation is not
`sign-in.v1` is decided right after the access decision: where the access decision permits it,
it reports invalid under PND-013, with a diagnostic naming the key and holding no part of the
value, before any other invalid, any conflict and any hold of PND-005 or PND-011, and it is not
dispatched; where it does not, it keeps that reason (OPS-023). A `sign-in.v1` request that
carries it is decided as order 048a-5 decides it, among that operation's invalids. Nothing else
reads the value, so no result, record, diagnostic, decision or state holds it (PND-014). A
retry's return is not yet read: PND-013's checks and the `sign-in-hook.v1` retry that passes a
value on are later orders' of the cut. The core has no task requests, so a `tasks/update`
carrying the key is not decided.

## Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)

- **Keys per hook** (ruling 86, point 2; PKG-020, IFC-037; order 209): an implementation item marked `plain` and reached by `command` or `package` may list under `keyed` hooks of its `implements`, each a key of its `tools` and one of the five core hooks. Registration and installation refuse any other selection under PKG-020, one that is not a core hook (`requests-keyed-not-a-hook`) as one that is not in `implements` (`requests-keyed-not-in-implements`), and the schema refuses `keyed` on an item that is not plain or is reached by `remote` (`PKG-020-manifest-keyed-not-plain.yaml`). The core reads `plain` per call on the hook path: a call for a hook that `keyed` lists carries Conduit's keys, is named from `tools`, declares Conduit's extension at `v0`, holds a setup or retire hook's idempotency key (IFC-006, OPS-028), and has its result read as a keyed implementation's (IFC-009, IFC-008, IFC-019, IFC-026, IFC-047, OPS-006, OPS-007). Of these, only IFC-009's reading has a case here (`requests-keyed-hook-dispatch`, step 13); a keyed setup or retire hook's idempotency key and its re-dispatch under OPS-028, and IFC-008, IFC-019, IFC-026 and IFC-047 on a keyed hook, follow from the invocation's `plain` and have no case: the unit test `keyed_hooks_read_plain_per_call` shows the key. Every other call to the item, every request among them, stays plain (IFC-007, IFC-010). A URL-mode `input_required` result from a part of `sign-in.v1` is read by IFC-010, keyed or plain alike, and never by the terminal checks of IFC-009, OPS-006 and OPS-007: the core ends such an account check unknown, whatever outcome it states, and it counts as no account (`requests-keyed-hook-dispatch`). The core also raises the instance's sign-in action for it, as for a URL elicitation outside `sign-in.v1` (IFC-016); whether PND-005 asks that of a part of `sign-in.v1` is open (order 209's Record). Before each such call the core sends `server/discover` to the implementation: IFC-037 asks for its result before the first call to each run and lets the engine read it again at any time, and this build reads it before every call for a keyed hook, keeping no result from one call to the next. Where the result does not list the extension at `v0`, or discovery ends in an error or no answer, the hook is not sent and reports unavailable under IFC-037, its `request-ended` record naming the missing extension, while the item's other calls are sent as before (`requests-keyed-hook-dispatch`). A hook whose discovery is in flight when the engine restarts ends unknown under IMP-007, as any call in progress, though it was never sent. When its setup or retirement is cancelled, it ends unknown under OPS-027. When it is a part of a `sign-in.v1` that is cancelled, the cancel leaves it in progress (order 048a-5b), and a discovery result that comes after sends nothing: one that lists the extension ends it unknown under OPS-027, and any other ends it unavailable under IFC-037, as above (order 209). Only these calls read discovery or declare the extension: a call to an implementation not marked `plain` carries Conduit's keys without either, as before order 209. No `sign-in-hook.v1` part runs before order 048a-5d, so the case keys `account-hook.v1`. From order 214, the sign-in-hook.v1 call, plain or keyed and on every re-send, carries io.modelcontextprotocol/protocolVersion 2026-07-28 beside IFC-045's clientCapabilities, so that an rmcp 3.9.0 server opened by the legacy handshake may answer it InputRequired (IFC-007, IFC-010); no other call names a revision.
- **A login name aimed at a plain hook** (ruling 86, point 3 (c); PND-010; order 209): after the refusal of a name the Type does not state, a `signInLogin` naming a stated login reports invalid before any part where the instance's `sign-in-hook.v1` is a plain hook, its item marked `plain` and not listing it under `keyed`, with a diagnostic naming PND-010 and the login (`requests-sign-in-login-plain-hook`, `requests-keyed-hook-dispatch`). The `access-decision` record reports whether access was permitted, and the refusal's reason and deciding rule are in `request-ended`.
- **A failed or unknown account check counts as no account** (ruling 86, point 3 (d); PND-010, PND-011, TPL-012, OPS-022; order 209): in `sign-in.v1` it takes the path of no account, clearing no condition and raising nothing, and so goes on to the `sign-in-hook.v1` part (order 048a-5d), whose URL elicitation is relayed, and the account polled on each retry (order 048a-5ef; `requests-sign-in-account-first`, `requests-keyed-hook-dispatch`, `requests-sign-in-hook-part`, `requests-sign-in-polling`); in setup it raises the sign-in action unless one is unanswered, undoes nothing, clears no condition and lets the setup go on to its setup hook, the check counting as succeeded in the setup's outcome (`requests-sign-in-setup-check-failed`, `requests-sign-in-setup-check-failed-hook`). The second of those also shows a failed check in `sign-in.v1` leaving a `False` `SignInUndone` as it is. A check that ends with a reason, such as unavailable under IFC-037, is neither: it ends `sign-in.v1` failed under PND-011 and stops a setup, as before. Polling, where a failed or unknown poll counts as no account yet, is order 048a-5ef's (`requests-sign-in-polling`).

IFC-010's relay of a URL request from the `sign-in-hook.v1` part, plain or keyed, has no case in this section: the hook part and the relay are orders 048a-5d and 048a-5ef, whose cases show it. Order 048a-5d takes that request into `sign-in.v1`, not as IFC-016's lapse trigger, and order 048a-5ef relays it, its `acceptsReturn` false (`requests-sign-in-plain-hook-part`). Order 048a-5d2 matches its URL as it matches a keyed hook's (`requests-sign-in-plain-url-match`).
