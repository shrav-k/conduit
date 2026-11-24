# A90: the usage guard's probe

Pre-registered on 2026-11-23 by work order 310, before the probe was written. No live run has taken
place. Tests use fabricated clients and scratch homes alone.

## The question

C1: does a Codex read change the home's `auth.json` (name, size, inode or times) or leave a lock
file, and what else changes? C2: per read, the milliseconds from the app-server's start to its
`initialize` answer and to the read's answer. C3: which fields the seat's answer fills, by name and
type. C4: two app-servers on one home, each sent the read before either answers: do both answer,
with `auth.json` unchanged? L1: does `/usage` start model work, by the signs of model work under
Claude's read? L2: does it return a `usage_report`, with which fields, and does its reply carry
"Showing last-known usage", and where? L3: what in the home changes?

## The probe

The Unix-only crate is `probe/`, package `usage-guard-probe` 0.1.0, edition 2024, unpublished and
with library doctests disabled. It pins `serde_json = "=1.0.154"` and `libc = "=0.2.191"`, with the
lockfile and toolchain based on experiment 027's refresh probe. Its `usage-probe` binary is the
probe; `fake-codex` and `fake-claude` are test clients only. It builds on macOS and Linux, the two
systems whose process tables it reads; on another Unix it does not build, and off Unix it exits 1
with a fixed message. No live result is recorded here.

- **Commands.** Use `usage-probe codex --home DIR --codex PATH --expect-version V`, or `usage-probe
  claude --home DIR --claude PATH --expect-version V --model NAME`. The home, absolute client path
  and expected version are required; Claude also requires a model. Missing or empty flags, a
  non-directory home and a Codex home without `auth.json` are refused before any client starts.
  `--deadline-seconds N` defaults to 60, accepts 1 to 600 and runs from each client's spawn to its
  answer or result. `--hold-seconds H` defaults to 10 and accepts 0 to 60. A version check runs
  `PATH --version` under that deadline, with both home variables pointing to a new empty scratch
  directory. It recognizes the first `N.N.N` token with any suffix and prints
  `codex.version_matches` or `claude.version_matches`. On mismatch it prints `version_found` with
  only the numeric `N.N.N` core, or `none` when no version token was read, and starts no more
  clients. Suffixes never print.

- **The client.** Its environment is cleared, then given `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM`
  and `USER` where present. The probe reads those variables and `TMPDIR` alone and prints none.
  `CODEX_HOME` or `CLAUDE_CONFIG_DIR` points to the resolved login home; Codex alone also receives
  `CODEX_REFRESH_TOKEN_URL_OVERRIDE` set to `https://refresh.invalid/oauth/token`.

- **Scratch.** The probe writes nowhere but inside one fresh directory of its own, made in the
  temporary root (`TMPDIR`, else `/tmp`) under a new random name, as `mkdtemp` makes one but
  relative to the root's open descriptor, and kept open. Every scratch directory is made inside it,
  relative to its descriptor, and settings are created exclusively relative to their scratch
  directory's descriptor. Before anything is written, and so before any client starts, the probe
  refuses a login home that is itself a mount point (on another device than its parent: it may be a
  remount of a directory elsewhere) or that holds any entry on another device (a mount inside it); a
  temporary root on another device than the home, since a remount of the home or of a directory in
  it (nullfs, bindfs or another FUSE mount, an SMB or NFS loopback) reports a device of its own and
  identity alone cannot rule it out; and, by directory identity, never by spelling, a temporary root
  that is the home, lies inside it or lies above it (where the fresh directory would sit beside the
  home): the home's device and inode against those of the temporary root and each directory above
  it, as `..` finds them from the open root, the root's against the home's and each directory's
  above it, and against every directory under the home (a hard link). So no symlink, case,
  firmlink, hard link or mount lets either path pass. Once the fresh directory is made, the home's
  listing must hold neither its name nor its identity, or it is removed at once and the run
  refused: a filesystem that showed the home's device under inodes of its own is so still caught,
  after one empty directory was made and removed in it. These checks run again before each scratch
  directory, and the path a client is given must still open that very directory. Each refusal
  prints `probe_failed.scratch: true` and `probe_failed.reason:` one of `home_mount_point`,
  `home_holds_mount`, `scratch_other_device`, `scratch_overlap`, `scratch_alias`,
  `scratch_path_moved` or `scratch_failed`. The probe also refuses directory or unresolved symlinks
  anywhere in the login home, so such a link cannot reach the temporary root. Cleanup goes only
  through the descriptors the probe kept, never a path: each entry is removed relative to its open
  parent, never following a link, never across a device boundary (each entry's device against the
  fresh directory's), never an entry the home held at a listing (a home file or directory renamed or
  linked in stays), and a directory only once it is opened without following a link, is the
  directory just seen, and has been emptied the same way; a scratch directory's own entry, and the
  fresh directory's, go only while they are still the directories made. Whatever cleanup leaves
  prints `scratch_left: true` and the command exits 1.

- **Processes.** Each client gets a new empty working directory and its own process group; stderr
  is discarded. While a client runs, the probe looks every 5 ms for its descendants and keeps each
  one it sees, by pid and start time, so a reused pid is never taken for one. A stop sends TERM to
  the group, KILL after two seconds if needed, reaps the client and then reports `group_stopped`
  using signal 0. Then any descendant seen that still runs has left the group (by `setsid` or
  `setpgid`, as a detached helper might): each is killed (on Linux, where the probe is the
  subreaper, reaped too) with any it started meanwhile, it stops all further clients as a
  surviving group does, with `codex.stopped_on_survivor` or `claude.stopped_on_survivor`, and once
  any client has started, `stray_processes: N` prints their count and the command exits 1 when N is
  above 0. SIGINT, SIGTERM and SIGHUP stop all groups and remove scratch directories before
  `interrupted: true` and exit 1. After one, even one that came during a listing, no client starts
  and nothing more is written to a client; the withheld step prints no `probe_failed` line. I/O
  failures print a fixed cause key: `probe_failed.spawn`, `probe_failed.scratch`,
  `probe_failed.settings`, `probe_failed.write` or `probe_failed.listing`, each with `true`; no
  error text, path or client value prints.

- **Codex's read.** The command is `codex app-server --stdio -c cli_auth_credentials_store="file"`.
  It sends `initialize` with clientInfo `usage-guard-probe` 0.1.0 and `experimentalApi: false`, then
  `initialized`, then `account/rateLimits/read` with exactly `{"excludeResetCreditDetails": true}`.
  Every write passes one function that, before writing a byte, refuses any message other than these
  three exactly as given: no other method, id, parameter, capability or field passes. This bars
  `supportsLunaReserve`, `account/read`, thread and turn methods,
  `account/rateLimitResetCredit/consume`, `account/sendAddCreditsNudgeEmail`, and login or logout
  methods. Every incoming line with a method is checked for a sign of model work before it is
  skipped. The signs are listed in one place, `CODEX_WORK_PARTS` and `CODEX_WORK_WORDS` in
  `src/run.rs`: the method, lowercased without `_` and `-`, has a `/`-separated part that starts
  with `turn`, `item` or `task`, or holds `agentmessage`, `tokenusage`, `tokencount` or
  `reasoning`. So every turn, item and agent-message notification, `thread/tokenUsage/updated` and
  the older `codex/event/` forms of them are signs; `account/rateLimits/updated` is not. Three
  independent app-servers run in turn,
  `codex.read.1` through `.3`, followed by `codex.pair`: two app-servers initialized and sent their
  reads before either answer is consumed. A reader per client timestamps each received line; queued
  replies received before their deadline remain eligible when consumed afterward. Each app-server is
  held H seconds after its answer, every line it prints meanwhile read, then stopped; after the stop
  it reads, for up to half a second, what the app-server printed before it. The first sign, before
  an answer, during a hold or in what a stop drains, stops every app-server then running at once and
  sends nothing more: the read prints `model_work: true`, `codex.stopped_on_model_work: true`
  prints, no further read starts and the command exits 1. A line after the answer that is not JSON,
  whose sign could not be read, does the same with `codex.stopped_on_unreadable_line: true`.
  Listings bracket each read and the whole pair. Each read prints `answered`, `error_answer`,
  numeric `error_code` when supplied, `ms_to_initialized`, `ms_to_answer`, `ms_request_to_answer`,
  `group_stopped` and `model_work`. Unobserved times are omitted. Read 1
  prints fields; reads 2 and 3 print `same_fields` for read 1's paths and types. The pair prints
  both sets of times, `both_answered` and `overlapped`: both requests were written before either
  reader stamped an answer. Login changes or a surviving group stop all further clients, print
  `codex.stopped_on_login_change` or `codex.stopped_on_survivor` and exit 1. No usable initialize or
  read answer, including timeout, EOF or malformed protocol data, prints
  `codex.stopped_on_no_answer: true` and exits 1; subsequent reads and the pair are skipped. If this
  occurs in the pair, `both_answered: false` also identifies the incomplete pair.

- **What it prints.** Both streams allow only lines matching `^[A-Za-z0-9_.?\[\] :+-]{1,200}$`. A
  failing line is dropped; `unprintable_lines: true` prints and the command exits 1. Every line is
  one fallible write to its descriptor; a write that fails, such as to a closed pipe or a read-only
  file, panics nothing: no line is tried after it, no client starts, every group is stopped and
  reaped and every scratch directory removed, then `probe_failed.output: true` is tried once on
  standard error and the command exits 1. The panic hook prints only `panicked: true`, with one
  system write whose failure it ignores. A field line has the form `PREFIX.field PATH: TYPE`, then a
  value only where allowed below. No token, header, cookie, account ID, email, amount, plan, model
  name, path, client text or home-file content prints.

  - **PATH** joins object keys with dots. Unknown keys become `?`. Array elements and the data keys
    of `rateLimitsByLimitId` become `[1]`, `[2]` and so on in received order; their data keys never
    print. TYPE is `object`, `array`, `string`, `number`, `boolean` or `null`.
  - **VALUE** includes every boolean. Numbers print only under percentage names `usedPercent`,
    `remainingPercent`, `percent`, `utilization`, or time names `resetsAt`, `resets_at`,
    `overageResetsAt`, `windowDurationMins`. Time strings print only in RFC 3339 shape. Under a
    documented word name, strings print only `word:true` or `word:false`, never the word.
  - **Names, Codex:** `ordinaryUsageAllowed`, `rateLimits`, `rateLimitsByLimitId`,
    `rateLimitResetCredits`, `accountId`, `rateLimitUpsell`, `limitId`, `limitName`,
    `normalModelSlug`, `primary`, `secondary`, `usedPercent`, `windowDurationMins`, `resetsAt`,
    `credits`, `hasCredits`, `unlimited`, `balance`, `individualLimit`, `limit`, `used`,
    `remainingPercent`, `spendControlReached`, `planType`, `rateLimitReachedType`.
  - **Names, Claude:** `message`, `usage_report`, `rate_limits`, `limits`, `kind`, `group`,
    `percent`, `resets_at`, `scope`, `model`, `surface`, `severity`, `is_active`, `extra_usage`,
    `is_enabled`, `monthly_limit`, `used_credits`, `utilization`, `currency`, `rate_limit_info`,
    `status`, `resetsAt`, `errorCode`, `canUserPurchaseCredits`, `hasChargeableSavedPaymentMethod`,
    `overageEnabled`, `rateLimitType`, `overageStatus`, `overageResetsAt`, `overageDisabledReason`,
    `isUsingOverage`, `unifiedWindows`, `five_hour`, `seven_day`.
  - **Word names:** `kind` accepts `session`, `weekly_all`, `weekly_scoped`; `severity` accepts
    `normal`, `warning`, `critical`; `status` accepts `allowed`, `allowed_warning`, `rejected`;
    `rateLimitType` accepts `five_hour`, `seven_day`, `seven_day_opus`, `seven_day_sonnet`,
    `overage`; `rateLimitReachedType` accepts `rate_limit_reached`,
    `workspace_owner_credits_depleted`, `workspace_member_credits_depleted`,
    `workspace_owner_usage_limit_reached`, `workspace_member_usage_limit_reached`.

- **The listing.** The probe opens directories only, never a file in the login home. It records
  relative paths, kinds, sizes, inodes, devices and modification/change times using lstat-style
  descriptor-relative metadata. Directory opens refuse symlinks and compare device/inode with the
  observed entry; replacement causes a safe refusal. An entry that disappears mid-walk is absent. A
  file symlink records only its target's size and times, never the target's path; directory and
  unresolved symlinks are refused. Comparisons stay in memory. The probe prints `home.unchanged`
  and, for Codex alone, `home.login_file_unchanged`: `auth.json` is unchanged and no top-level name
  starting with `auth` or `.auth` was added or removed. Changed top-level entries print
  `home.changed NAME: added`, `removed` or `changed`; deeper names never print. To protect account
  IDs and token-shaped filenames, NAME is printed only for this fixed list: `auth.json`,
  `auth.lock`, `auth.json.lock`, `.auth.lock`, `.claude.json`, `config.toml`, `settings.json`,
  `state`, `logs`, `sessions`, `projects`, `history.jsonl`, `cache`. Every other name becomes `?`,
  even if it would pass the original character and hexadecimal-run checks.

- **Claude's read.** One `claude.read` runs `claude --print --input-format stream-json
  --output-format stream-json --verbose --include-partial-messages --no-session-persistence
  --setting-sources "" --settings S --strict-mcp-config --tools "" --model NAME`. S is the working
  directory's `settings.json`, holding `{"disableAllHooks": true}`. The command omits
  `--disable-slash-commands`. The sole user frame has content `/usage`; the writer refuses any other
  content. It reads through the result, closes stdin and waits up to H seconds for exit, still
  reading every line the client prints; then it stops the group. After any stop it reads, for up to
  half a second, what the client printed before the stop, lines that came after the deadline
  included. Listings bracket the read, without a login-file line.

  - **Signs of model work:** any `stream_event`; a `system` event with subtype `api_retry`; an
    `assistant` event whose `message.usage` has a positive token count; or a result with a positive
    token count in `usage`, an entry in `modelUsage`, positive `total_cost_usd` or positive
    `duration_api_ms`. The first sign, before the result or while waiting for exit, stops the group
    immediately; any sign read prints `model_work: true` and `stopped_on_model_work: true`, and the
    command exits 1. A result-only sign is seen after the work, and one read after a stop was
    printed before it. Otherwise it prints `model_work: false`. A line that is not JSON, or is over
    1 MiB or not UTF-8, prints `stopped_on_unreadable_line: true` and exits 1; read before a stop,
    it stops the group at once. A last line cut short by a stop counts as one.
  - **Printed keys:** `answered` means a result came. `ms_to_report` times the first event with
    `usage_report` at any depth; `ms_to_result` times the result; `ms_api` is its API duration.
    Unobserved times are omitted. It also prints `terminal_reason_absent`, `usage_report_present`,
    `rate_limits_null`, `last_known_note` and `group_stopped`. The last-known check searches string
    values for `last-known`, ignoring case, without printing those strings. `last_known_where` names
    the first matching event as `assistant`, `result`, `system`, `user` or `other`; when no note is
    found it prints `none`. Fields of `usage_report` and each rate-limit event's `rate_limit_info`
    print as What it prints says.

## The sitting

On the owner's machine, the owner present (DEC-135), after order 310 merges: the Codex command on
each Codex team seat's login home; the Claude command on each Claude team seat's login home with a
model the owner names as included in that seat's plan, only on the owner's word, given after being
told that if the installed version's `/usage` does not bypass the query loop, one request reaches
that model before the probe can see it, that on a seat whose included usage is spent and whose
extra usage is on, it may bill credits, and that a request the client makes outside its output
stream, such as a background call, is not seen; stop rule 2 applies after it. Each command runs
outside any tool timeout shorter than its worst case, or in the background: about 5D+4H+18 seconds
for Codex (358 at the defaults) and 2D+H+6 for Claude (136), plus the listings and two seconds for
each client with a stray. Afterwards `ps`
shows that no process of the client's binary remains. The output kept under the crate's `target/`
and quoted in the results only as printed.

## Predictions

These are the coordinator's predictions from the vendors' documentation and Codex's source, not
results: C1, `auth.json` unchanged and no lock file left, as order 208 found for `account/read`,
while the app-server's own state files may change; C2, every read answers within the 30 seconds the
guard's design assumes for the wait (DEC-123); C3, the windows and `ordinaryUsageAllowed` filled,
and no source says whether a team seat fills `credits`, `individualLimit` or `spendControlReached`;
C4, both answer, `auth.json` unchanged; L1, no model work; L2, a `usage_report` whose
`rate_limits.limits` rows no source says a team seat fills; L3, the client writes its own state,
such as `.claude.json`, and may refresh the login.

## What each outcome changes

C1 and C4 as predicted, `codex-login.v2`'s hook may read through a refresh-blocked app-server,
beside the login's other clients; otherwise the outcome goes to the owner (by stop rule 3 where the
login file changed) before that hook is written. C2: the slowest read's `ms_to_answer` sets the
limit on a call's wait (DEC-123), 30 seconds until a read is timed. C3: the fields each seat fills
are those its hook may decide from. L1 and L2: no model work and a `usage_report` let
`account-home.v2` offer `usageRead: when-stale`; otherwise `usageRead` is `never`. The last-known
note: found, the package can tell a last-known report from a fresh one; not found, a `/usage` report
counts as observed 60 minutes before the read. L3: a change to the login goes to the owner before
`usageRead` is offered.

## Stop rules

(1) the probe refuses, stops on a survivor, or a read gets no answer within the deadline: nothing
more runs on that seat until the cause is read; (2) a Claude read shows model work: no Claude read
runs again, and the owner is told; (3) a Codex read changes `auth.json` or leaves a lock file (the
probe then starts no more clients): no Codex read runs again on any seat, and the outcome goes to
the owner as it stands, as ruling 86, point 5, asks of a write under the override; (4) no read is
repeated to change an outcome.

## Not shown

One machine, one morning, the clients as installed. A refresh the Claude client writes outside the
home, such as to a keychain item, is not seen. The signs of model work come from the client's
output stream alone: a request it makes outside that stream, such as a small-model background
call, a quota or title check, an auto-updater or telemetry, is never a sign, so `model_work: false`
does not prove that nothing reached a model or was billed. Whether the Claude client's environment
gains `DISABLE_AUTOUPDATER=1` or `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` waits for the owner's
word, since either changes its exact environment. A descendant that leaves the client's process
group is found only if the probe saw it while a process it already knew of was its parent: one
whose parent exits within the 5 ms between two looks is the init process's child by the next, with
no parent chain back to the client, and macOS has no subreaper to keep it. A mount over a scratch
directory, or a filesystem that misreports devices, is refused or left by the checks above but is
not exercised by the tests, which cannot mount: they rewrite what validation reads instead. If the
probe itself is
killed with SIGKILL, its scratch directories stay, and its clients exit only if they honour the end
of standard input. No turn runs, so whether turns emit `rate_limit_event` at `allowed`, whether
`/usage` rows and the stream's events name matching windows, and whether a read during a turn
leaves its refresh alone stay open: each needs model work, which waits for the owner's word
(DEC-102, DEC-135).
