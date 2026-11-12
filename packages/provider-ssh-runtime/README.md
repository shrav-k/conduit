# SSH runtime provider

Work orders 137 and 140 build `example.org/conduit/provider-ssh-runtime` 0.1.0. Its only Type,
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

The first start durably makes a random 64-bit part in the provider's state directory. Units
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
0600 local `stderr/<marker>-<generation>.log` file and persists until the operator removes it;
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
all machines are off. One owner.lock excludes a second provider in the same state directory.
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

Decision D's external stop-drainer that outlives provider retirement is **not built**. A stop
still owed on retirement is carried out only if the provider starts again in the same state
directory. Until then RuntimeMaxSec is the only end of that run, and the run directory of a
run that never started stays on the remote. Failed staging and start attach the same named,
value-free reason written on stderr and in 0600 diagnostics.log through `Unavailable::because`.
The kit includes it after `staging failed:` or `start did not complete:`, with its existing
stop suffix for start, after checking for sensitive values and bounding it to one line.
Startup refusals stay on stderr; the kit discards startup reasons.
A remaining kit limitation: on an unreachable machine, its unknown-state repeated-run check
returns `run may still be running` before the owed-stop admission check, so that answer
does not name the held stop. It still starts nothing and keeps the debt. The admission check
reached after staging does name the owed instance and machine, as the race test confirms.

## Startup configuration

The provider uses CONDUIT_STATE_DIR as provided by the engine, a private absolute directory
(mode 0700, no symlink). Configuration is read once, from the item's environment. Missing or
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
| privateKey | PRIVATE_KEY | Absolute key path inside this state directory; regular file, mode 0600; no default |
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
with % doubled, and cannot start with ~. State paths with quotes, backslashes or newlines are
refused. Control sockets stay below 80 bytes in a 0700 directory inside state where it fits,
otherwise a private temporary directory (removed with its owner). Stale sockets are removed
before connecting. Keep this directory private from other local principals.

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
| (a) Own operations/custom code | Observed in source and fake tests: src/backend.rs implements RuntimeBackend, durable markers, generation reservations and cleanup, run admission, contact, sweep and start/stop/state; src/ssh.rs owns subprocesses/master/session and SSH argv; src/codec.rs encodes shell/stdin/environment; src/config.rs validates startup; src/storage.rs atomically commits the ledger; src/main.rs serves; src/bin/package.rs archives; src/lib.rs exports the interface. This is an implementation inventory, not a counted S10 result. |
| (b) Delegation | Observed mapping: kit owns MCP/admission/gate/lifecycle/owed stops (shared work counted once with local-process and future Docker); OpenSSH owns authentication, host pinning and transport; systemd owns exec, cgroup stop, lifetime and stop-post cleanup. Real OpenSSH/systemd behavior is untested here. |
| (c) Persistent state | Observed: provider OS user owns local owner.lock, part, known_hosts, markers, permanent generation reservations, ledger.json, diagnostics.log, stderr logs and operator key; remote shared user owns part/runs and persistent part/state. Inventory, atomic ledger and owed-stop restart tests cover local recovery. Retirement has no external drainer; restart in the same state directory is required. Remote files, modes and staging are tested by a fake executing scripts in a temporary directory. |
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
