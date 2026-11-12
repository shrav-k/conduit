# SSH runtime provider

Work orders 137, 140, 145 and 146 build `example.org/conduit/provider-ssh-runtime` 0.1.0. Its only Type,
`ssh-runtime.v1`, offers `runtime-provider.v1`; the non-plain `provider` item implements
`run.v1`, `stop.v1` and `inspect-run.v1`. It depends by path only on the
[runtime-provider kit](../runtime-provider-kit/README.md). SSH-M is a separate provider.
This is fake-remote unit evidence, not qualification against sshd, user systemd or WSL.
No test starts sshd or reaches another machine.

## Contract

The kit supplies the per-run TLS gate on the engine's machine: it proves the instance URI,
requires the engine's signed token before forwarding bytes and owns the duplex stdio channel.
The gate's silent-cut bound is keepaliveIdle + keepaliveInterval × keepaliveCount seconds
(45 with the suggested 30/5/3 settings). Linux sets TCP_USER_TIMEOUT too. The kit's macOS
caveat applies: outstanding unacknowledged data can exceed this bound when keepalive probes
cannot run. Separately, SSH uses ConnectTimeout=5, ServerAliveInterval=10 and
ServerAliveCountMax=3: its own unresponsive-peer bound is 30 seconds after connecting,
following the connection attempt of up to five seconds. These are different connections.

Runs execute as `conduit-run`, without sudo, shared by every run and the work-order workers
(ruling 77). **PRV-025's isolation is unmet until each Agent has a user of its own.** Any run
can read every other run's identity files and environment (`/proc/<pid>/environ`); write into
another run's standard input and output through `/proc/<pid>/fd`; control the shared user
manager, including stopping units and `systemctl --user set-environment`; write a drop-in
under `~/.config/systemd/user/` for the prefix, or a file under `~/.config/environment.d/`,
which every later run's unit reads, and so read later runs' secrets; and start
`systemd-run --user` units of its own, which escape the provider's stop and RuntimeMaxSec.
The remote's lockdown and setup close none of these openings. Modes 0700/0600 protect files
from other users, not this shared user. The kit gate also needs a separate OS principal
from untrusted runs; SSH keeps those runs on the remote, but does not isolate them from each other.

Only configured machines and `command` items run. A command's program must be absolute or a
bare name resolving to an absolute executable through the effective run PATH on the remote.
Relative paths containing `/`, package items and all package_arguments end failed. Remote
commands contain POSIX single-quoted words. Environment values, secrets and identity file
contents travel only through `/bin/sh -s` stdin, never in local or remote arguments. SSH is
resolved on the provider's PATH once at startup, then invoked by absolute path with an empty
environment and all configuration files disabled. Its ed25519 host keys are pinned.

The first start durably makes a random 64-bit part in the durable instance subdirectory. Units
are `<prefix><part>-<SHA-256(instance-ID)>.service`; remote directories are
`<base>/<part>/runs/<hash>/<generation>/` and `<base>/<part>/state/<hash>/`. Another provider's
part is never swept. Each staging reserves a random 16-character hex generation in the local state,
never reused while that state is retained; reservations persist across provider restarts.
The remote run directory is 0700, identity files and EnvironmentFile are 0600, under
umask 077. Environment precedence is configured PATH, item environment supplied by the engine,
secrets, identity variables, then CONDUIT_IDENTITY_DIR and CONDUIT_STATE_DIR. The last points
to the per-instance remote state directory, kept across starts and never removed (PRV-029).
A supplied PATH binding follows that precedence, so the same effective PATH is used in the
pre-check and the service. No provider environment is inherited.

Secrets and identity (PRV-011, PRV-022) rest in the remote EnvironmentFile from staging until
the unit is confirmed active, when that file is removed; in the run's environment for its
life; and in remote identity files until the unit stops for any cause. ExecStopPost removes
the generation's entire directory. EnvironmentFile uses the optional `-` prefix so its removal
cannot prevent systemd from spawning ExecStopPost. A start-only `ExecStartPre=:/bin/test -f <file>`
still requires that file before the command can launch; WorkingDirectory must also exist.
For a run that never starts these files remain until removal or the next sweep.
The gate holds identity in memory. The provider's state holds no copy of
these values: it holds a private key path, the operator-installed private key, pinned public
host keys, markers, generation reservations, the ledger and diagnostics. Run stderr goes continuously into a private
0600 local `CONDUIT_STATE_DIR/stderr/<marker>-<generation>.log` file, removed with the instance;
a run can itself print credentials there. Diagnostics never include values or subprocess output.

Every run has the lifetime the kit supplies (at most maxLifetime), enforced remotely by
RuntimeMaxSec, and TimeoutStopSec from stopTimeout. systemd stops the whole unit cgroup,
including setsid descendants that remain in it. Runs never restart automatically. Start
returns a channel only after `systemctl --user show` confirms active. Confirmation polls
for ten seconds, including waiting for the machine connection mutex, stalled show commands
and active environment-file removal. Cancellation closes stdin while waiting for that mutex.
Deadline cleanup gets at most five more seconds, including mutex acquisition,
to invalidate the directory, then requests local SSH termination and waits up to one
second for its exit; closing stdin itself is bounded by one second. A cancellation before send launches nothing. After send, stdin
closes and start waits for session exit or a loaded-unit acknowledgement. At the confirmation
deadline it attempts to invalidate the run directory and terminates the owned SSH session;
the kit then stops it, keeping an owed stop if unreachable. A partition cannot provide a
bounded remote acknowledgement; local session exit is distinct from a confirmed remote
stop, and this uncertainty is not reported as stopped. Stop sends systemctl stop, removes the run directory
(to defeat a queued launch), confirms inactive or absent, and only then removes the marker.

Staging and stop remove all of `runs/<hash>/`, including every generation. A stop for an
unacknowledged launch clears once it has removed that parent and confirmed the unit inactive
or absent; no loaded-unit acknowledgement is required. Later staging uses a new generation,
so a queued old launch cannot reach its files. The fence relies on systemd refusing to start
a unit whose WorkingDirectory is missing, together with the generation's start-only file check.
The owner's test host will confirm the systemd behavior; the fake remote only models it.

The backend sends a best-effort stop as soon as an SSH run session ends. After a network cut,
remote sshd closes the run's stdin when its ClientAliveInterval/ClientAliveCountMax notices
the cut, within 30 × 4 = 120 seconds under the owner's setup; RuntimeMaxSec is the backstop
even when SSH cannot deliver a stop. On each master
connection/reconnection, the provider checks id -u, user-manager running/degraded, Linger=yes
and systemd >=254, then sweeps its own unmarked units and orphan/unstarted run directories
before admitting runs. For a started marker it retains only that generation directory and
removes stale generations and every other name under the instance hash. Each sweep removal
is reported on stderr and in diagnostics. A dropped
staged run is removed over a live master; unreachable staging is retained for the next sweep.
A failed master backs off at 2, 4, 8, 16, then 32 seconds; calls during the backoff refuse at
once. Each machine has one owned foreground master, no ssh agent and no shared control socket.

A durable marker precedes staging. Inventory reads markers only and remains complete when
all machines are off. One owner.lock excludes a second provider in the same durable instance subdirectory.
ledger.json atomically retains the kit's records and owed stops. The kit drains at startup,
before well-formed calls and each drainInterval. Recovery stops every marker/ledger run because
old channels cannot survive provider restart; an incomplete inventory prevents serving.
Every held stop is carried out before any new run on its machine starts. A refused start
names the owed instance and machine. Stops owed only on other machines do not block admission.
The kit coordinates admission before staging and immediately before start with each machine's
read-write lock. Starts share its read side; a stop publishes its obligation under the write
side. Stops wait for backend starts already in progress on that machine, not staging, and
never acquire another instance's operation lock. Calls for one instance serialize across
machines. The kit cancels and waits for stage/start/gate opening, closes the gate, then stops;
no launch may follow a confirmed stop. Inspection is unknown while unreachable or stop is owed.

At most one external stop-drainer waits or works in each durable subdirectory. Immediately
after owner.lock and the state-dir check, before the private key, part, known_hosts or kit
recovery can refuse startup, the provider tries drainer.lock without blocking. If free, it
releases it at once and starts the sibling `bin/provider-ssh-runtime-drain`; a failed start
refuses startup, naming it. If held, a drainer is alive or about to be, so none is started.
Throughout initialization, service and shutdown the provider repeats this check once a second,
logging a failed replacement start to diagnostics.log and retrying at the next poll. A drainer
killed while the provider runs, or one that dies before taking its lock, is replaced within
a poll after the kernel releases the lock.

The drainer starts a new session with setsid, /dev/null as stdin and both outputs appended to
mode 0600 drainer.log. Its environment contains only PATH; the durable subdirectory and original
CONDUIT_STATE_DIR bytes are its two arguments. It leaves the engine runner's process group by
design (DL16), and survives that group's kill. Stopping the engine's whole service cgroup or
rebooting also ends the drainer; owed stops then wait for the provider's next start. It starts
no run, serves no call and opens no gate.

Once SshBackend::new succeeds, and before RuntimeServer::new, the provider atomically writes
mode 0600 drainer-config in the subdirectory. It holds only the configuration schema's places:
paths, references and settings, including the private key's path, never secret values (SEC-001).
A provider whose backend refuses startup leaves the previous configuration in place; a failed
write refuses startup, naming drainer-config, after the drainer has started.

The drainer tries drainer.lock without blocking; if held, it exits at once. It holds that lock
for its whole life, including the wait for owner.lock, draining, the retirement watch and yield.
It waits for owner.lock using nonblocking flock and Tokio timers, logging and retrying errors;
only a missing subdirectory or ledger ends that wait. Each time it takes owner.lock it reads
drainer-config afresh and builds its backend from that configuration, so a drainer already
waiting serves with the newest accepted configuration. A missing or invalid drainer-config is
logged by name and ends the drainer. On every exit, including cancellation, the attempt and
backend (and thus SSH masters) drop first, then drainer.lock, then owner.lock. Yield instead
releases only owner.lock, keeping drainer.lock until the same drainer serves again.

Once it owns owner.lock it removes stale yield, reads the ledger afresh and uses the kit's
drain_owed_stops. Missing subdirectory or ledger is logged and ends the drainer; an unreadable
ledger is a failed attempt. Failures wait drainInterval, capped at five minutes, then double
up to that cap.
At maxLifetime + stopTimeout since its latest owner-lock acquisition it cancels any attempt
still in flight and makes one last attempt, then retains any remaining obligation for the
provider's next start. The last attempt retains the backend's SSH command bounds and can be
interrupted by yield. Each attempt rereads the ledger.

A starting provider writes yield on every owner-lock poll and waits at most ten seconds before
initialize. On giving up it removes yield and refuses, naming owner.lock. The drainer checks
yield at least once a second, including during attempts, backoff and the retirement watch;
it drops the in-flight attempt and backend (killing its SSH masters) before releasing owner.lock.
The kit saves each stop's intent before trying it. After yielding it waits eleven seconds,
then tries owner.lock again; a held lock returns it to waiting, even for a brief sweep owner.
No server or second drainer works beside the current owner.

When nothing is owed, including an instance that never ran a run, or after the final attempt,
the drainer watches for retirement for at most one minute. Retirement means symlink_metadata
finds the original CONDUIT_STATE_DIR path absent while its parent still exists; a moved or
unmounted store does not count. This path need not exist at drainer startup and is never
canonicalized. If retired, nothing is owed on a fresh ledger read, and state-dir still holds
those original bytes, it removes the private key and known_hosts from the durable subdirectory.
The ledger and markers stay for the operator. Otherwise it exits retaining key material for
the next provider start. A provider retired while no drainer watched keeps its key until
order 147's sweep, or the operator, removes it.

Failed staging and start attach the same named,
value-free reason written on stderr and in 0600 diagnostics.log through `Unavailable::because`.
The kit includes it after `staging failed:` or `start did not complete:`, with its existing
stop suffix for start, after checking for sensitive values and bounding it to one line.
Startup refusals stay on stderr; the kit discards startup reasons.
A remaining kit limitation: on an unreachable machine, its unknown-state repeated-run check
returns `run may still be running` before the owed-stop admission check, so that answer
does not name the held stop. It still starts nothing and keeps the debt. The admission check
reached after staging does name the owed instance and machine, as the race test confirms.

## Startup configuration

The engine supplies CONDUIT_STATE_DIR (PRV-029), which it removes with the provider instance.
The required `durableDir` / `SSH_RUNTIME_DURABLE_DIR` is outside that lifecycle: under the
admin's decision of 2026-11-12 for PRV-021, it retains owed stops and the identity of the remote
part after provider retirement. The operator creates this directory at mode 0700, owned by the
provider's user, and removes it only after all owed stops and remote material have been cleaned.
It has no default. Configuration is read once, from the item's environment. Missing or
malformed bindings refuse startup and name the binding, without printing its value. Each
config property below maps to `SSH_RUNTIME_<BINDING>` (no new CONDUIT_ configuration names).
The manifest maps those properties to the item environment. Only the three tool paths and
unitPrefix have startup fallbacks for direct launches. Every place bound by the item is required
and scalar in the Type schema (MAN-017), including those four and runLimit; their defaults stay
in the schema and the engine fills them before validation (SCH-009). Other environment bindings
are required even for direct launches.

| Config property | Binding suffix | Meaning / suggested default |
| --- | --- | --- |
| machines | MACHINES | Required JSON string containing a nonempty array of the objects below; no default |
| durableDir | DURABLE_DIR | Absolute operator-owned durable directory, mode 0700, outside the engine store; no default |
| privateKey | PRIVATE_KEY | Absolute key path inside the durable instance subdirectory; regular file, mode 0600; no default |
| base | BASE | Absolute remote base, only A-Za-z0-9/._-, no dot/dotdot components; no default |
| path | PATH | Remote run PATH; no default |
| runLimit | RUN_LIMIT | Per-machine staging/running limit, 1–62, default 62 |
| maxLifetime | MAX_LIFETIME | Positive seconds, suggested 3600 |
| stopTimeout | STOP_TIMEOUT | Positive seconds, suggested 10 |
| drainInterval | DRAIN_INTERVAL | Positive seconds, suggested 5 |
| systemdRun | SYSTEMD_RUN | Absolute remote tool, default /usr/bin/systemd-run |
| systemctl | SYSTEMCTL | Absolute remote tool, default /usr/bin/systemctl |
| loginctl | LOGINCTL | Absolute remote tool, default /usr/bin/loginctl |
| unitPrefix | UNIT_PREFIX | Alphanumeric/hyphens, at most 80 bytes, default conduit-run- |
| bindAddress | BIND_ADDRESS | Specific local IP for the gate, suggested 127.0.0.1 |
| advertisedHost | ADVERTISED_HOST | Gate DNS name or IP, suggested 127.0.0.1 |
| keepaliveIdle | KEEPALIVE_IDLE | Positive seconds, suggested 30 |
| keepaliveInterval | KEEPALIVE_INTERVAL | Positive seconds, suggested 5 |
| keepaliveCount | KEEPALIVE_COUNT | Positive probes, suggested 3 |

Machines have exactly `hostname`, `host`, `port`, `user`, `host_key`; hostname is the engine's
machine name, host is its SSH destination, port is 1–65535, user must be `conduit-run`, and
host_key is `ssh-ed25519 <base64-public-key>` without a comment. No key is generated by this
provider. Numeric settings are whole seconds/counts, at most 2147483647; the gate's full bound
must fit that many milliseconds. Paths in SSH -o values are double-quoted for SSH's parser,
with % doubled, and cannot start with ~. Durable paths with double quotes, backslashes, CR, LF or NUL are
refused. Control sockets always use a private mode 0700 temporary directory (removed with its
owner), below the 80-byte socket-path limit; the hashed durable path is too long. Stale sockets are removed
before connecting. Keep this directory private from other local principals.

## Durable local layout

Each provider instance has one mode 0700 subdirectory under `durableDir`, named by the
lowercase hexadecimal SHA-256 of the exact bytes of its supplied CONDUIT_STATE_DIR value,
without a trailing newline. Do not canonicalize, trim, collapse slashes or remove a trailing
slash before hashing. For example, an operator can pipe `printf '%s' "$CONDUIT_STATE_DIR"`
to `sha256sum` (or `shasum -a 256` on macOS), and use its first field as the subdirectory name.
Create that subdirectory at mode 0700 and install the configured `privateKey` there at mode
0600 before the first start. The provider can also create the subdirectory. The engine gives
no provider instance ID directly; PRV-029's unique, never-reused path identifies it here.

| Place | Contents |
| --- | --- |
| Durable instance subdirectory: `owner.lock` | Exclusive ownership for the server or drainer; different instances have different locks |
| `state-dir` | Exact original CONDUIT_STATE_DIR bytes, without a newline |
| `ledger.json` | Atomically written kit records, including owed stops |
| `markers/` | Durable run inventory, including unstarted material to sweep |
| `generations/` | Generation reservations, never reused while retained |
| `part` | Random remote part identity, retained across restarts |
| `known_hosts` | Pinned public host keys |
| Configured `privateKey` | Operator-installed private key inside this subdirectory |
| `diagnostics.log` | Provider diagnostics, without run output or credential values |
| `drainer.log` | External drainer diagnostics; both stdout and stderr append here |
| `drainer.lock` | Held by one external drainer for its whole life, including waiting and yield |
| `drainer-config` | Atomically published schema configuration, mode 0600; read on every owner-lock acquisition |
| `yield` | Provider's request for the drainer to release owner.lock |
| CONDUIT_STATE_DIR: `stderr/` | Run standard-error files only; may contain run-printed credentials and disappear with instance removal |
| Private temporary socket directory | SSH control sockets, always outside both state directories |

Startup opens or creates the durable subdirectory, takes `owner.lock`, then checks `state-dir`.
For a fresh subdirectory it atomically writes `ledger.json` as `[]` before atomically writing
`state-dir`. An existing valid ledger without `state-dir`, after an interrupted initialization
or an explicit migration, is retained rather than overwritten. Invalid ledgers refuse startup.
A different `state-dir` value, or `state-dir` present without `ledger.json`, refuses startup.
FileStore treats a missing ledger as an error, never an empty set of obligations.

Startup refuses a missing, relative, SSH-option-incompatible, non-directory, symlink,
foreign-owned or non-0700 `durableDir`. It also refuses one inside CONDUIT_STATE_DIR or its
parent, including that parent itself: another instance's removal must not delete durable
state. Containment uses canonical paths (including macOS's `/var` to `/private/var` alias),
while the subdirectory hash always uses the raw supplied value. Refusals name the configuration
place; a private-key refusal names the expected durable instance subdirectory.

There is no automatic migration from orders 137/140. With the provider stopped, create the
computed subdirectory and move every existing `ledger.json`, `markers/`, `generations/`, `part`
and `known_hosts` from CONDUIT_STATE_DIR into it, preserving contents and private modes; move
the private key and diagnostics there too and update `privateKey`. Startup names every old
layout entry it finds and refuses until they are moved. Keep `stderr/` in CONDUIT_STATE_DIR.
Never discard a ledger or change the old part: owed stops and old staged material still need
cleanup. A migrated ledger is retained on initialization; `state-dir` is then written for the
supplied value. Never run two owners during a move.

The backend constructor accepts the durable subdirectory, an optional engine state directory,
and an optional already-held `OwnerLock`. Without an engine state directory it reads, creates
and canonicalizes nothing there; it opens existing durable state for a future drainer even
after the engine state directory and its parent are removed. Only the server makes `stderr/`
and checks containment and the old layout. The external drainer uses this constructor after
taking the owner lock.

## The owner's ruling-77 setup

The following comes from the owner's ruling-77 setup, with the reason for each:

- `conduit-run`'s login shell is `/bin/sh`, so command sessions read no startup file that could
  rewrite arguments or print into the protocol.
- In sshd's `Match User conduit-run` block: `PermitUserRC no`, to prevent user rc commands;
  `ClientAliveInterval 30` and `ClientAliveCountMax 4`, to notice a silent cut within 120 seconds
  and close stdin; and `MaxSessions 64`, to carry the bounded runs and command traffic.
- Lingering is on for `conduit-run`, so its user manager persists without an interactive login.
- A root-owned `AuthorizedKeysFile` (`/etc/ssh/conduit-keys/%u`), with `restrict` on the key's
  line, keeps key admission outside runs' authority and disables forwarding, PTYs and user rc.
- `UsePAM` is on, so pam_systemd starts the user's session and runtime directory.
- The lockdown keeps `systemd=true` under `[boot]` in `wsl.conf`, so the user manager and service
  cgroups exist.
- `PermitUserEnvironment` stays at its default, `no`, since it cannot go in a `Match` block;
  this prevents user environment injection.

The crate also needs and checks systemd 254 or later, a base directory that `conduit-run` can
write, the pinned host key and the installed private key.

The master opens no session, serialized provider commands use one at a time, and each instance
staging or running uses one. With `MaxSessions 64`, runLimit defaults to 62 (and may be lowered),
leaving one session spare alongside a provider command. Stage counts durable markers on the
machine and refuses past the limit before opening any session, naming the machine and runLimit.
This avoids sshd refusing a session: a failed command can otherwise drop the master and every
run on that machine. Unstopped markers count conservatively until their stop clears.

## D15 inventory

These are unit observations, not S10/WSL evidence; see
[S10's four fields](../../experiments/021-s10-swapped-providers/README.md#measures).

| Field | Disposition and evidence |
| --- | --- |
| (a) Own operations/custom code | Observed in source and fake tests: src/backend.rs implements RuntimeBackend, durable markers, generation reservations and cleanup, run admission, contact, sweep and start/stop/state; src/ssh.rs owns subprocesses/master/session and SSH argv; src/codec.rs encodes shell/stdin/environment; src/config.rs validates startup; src/durable.rs owns instance binding and locking; src/storage.rs atomically commits the ledger; src/main.rs serves; src/bin/package.rs archives; src/lib.rs exports the interface. This is an implementation inventory, not a counted S10 result. |
| (b) Delegation | Observed mapping: kit owns MCP/admission/gate/lifecycle/owed stops (shared work counted once with local-process and future Docker); OpenSSH owns authentication, host pinning and transport; systemd owns exec, cgroup stop, lifetime and stop-post cleanup. Real OpenSSH/systemd behavior is untested here. |
| (c) Persistent state | Observed: provider OS user owns durableDir and each hashed subdirectory with owner.lock, state-dir, part, known_hosts, markers, permanent generation reservations, ledger.json, diagnostics.log and operator key; only stderr logs stay in the engine state directory. The operator controls durable-directory removal. Remote shared user owns part/runs and persistent part/state. Inventory, atomic ledger and owed-stop restart tests cover local recovery. The external drainer reads the newest accepted configuration and survives the provider process group; owed stops after a service cgroup stop or reboot wait for reopening the same subdirectory. Remote files, modes and staging are tested by a fake executing scripts in a temporary directory. |
| (d) Operator setup/recovery | Untested on the Windows computer: install keys, pin hosts, configure the table and the remote setup above; route the gate on the engine's machine. Manual recovery after retirement: restart the provider with the same state and configuration to retry held stops, including unacknowledged launches. If the state is lost, an operator must inventory/stop that part's remote units and remove run files; persistent state is deliberately retained. No setup script or independent supervisor is shipped. |

## Build and unit checks

Run one cargo command at a time, from this directory, under the pinned Rust 1.97.1 toolchain:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo build --release --bins
cargo run --release --bin package
cargo run --release --bin package
```

The package binary uses the already built sibling provider executable, avoiding nested cargo
commands, and writes deterministic USTAR/gzip metadata and a SHA-256 digest in ignored dist/.
The two archive digests must agree. target/ is ignored too. Tests require the repository's
schemas, not optional skipped checks. They fake every SSH operation, execute staging scripts
only in local temporary directories and exercise the real kit over an in-memory MCP transport.
A later order must test loopback sshd; real user systemd, network cuts and Windows setup remain
untested. Tests bind no remote port and install no daemon.
