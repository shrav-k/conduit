# Work order 137 (SSH-R): the SSH runtime provider, built and unit-tested

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

[Ruling 77](../proposals/ruling-77/README.md) makes SSH-M and SSH-R the Windows computer's machine
and runtime providers: runs there execute as one user, `conduit-run`, with no sudo, in an
Ubuntu-24.04 distro locked down (interop, `appendWindowsPath` and automount off), reached over the
Tailscale network. Read it first. SSH-R is a runtime provider on the runtime-provider kit,
`packages/runtime-provider-kit/` (orders 122, 124, 126, 129 and 130): the kit holds the MCP server,
the gate, the ledger, owed stops and the rule that no run starts on a machine that still owes a
stop (the owner's answer to MC4-002); SSH-R implements the kit's backend for a run on another
machine. Read the kit's README, [PROVIDERS.md](../implementation/PROVIDERS.md), its paragraph
"A63-RK1 / RK2 — What the runtimes share", the SSH rows and decision D, and the local-process
provider, `packages/provider-local-process/` (order 127), the kit's first runtime, which this one
follows wherever this order does not say otherwise.

This order builds SSH-R and tests it with a fake of the remote's commands. A later order tests it
over a loopback `sshd`; its tests against a real user `systemd` and the Windows computer wait for
the owner's ruling-77 setup. Nothing in this order reaches another machine or starts an `sshd`.

## The decisions it applies

1. **Where it lives.** `packages/provider-ssh-runtime/`, a crate with its own `Cargo.toml`,
   committed `Cargo.lock`, `rust-toolchain.toml` at 1.97.1 and a `.gitignore` for `target/` and
   `dist/`, depending on the kit by path and on no other crate of this repository. It is a
   provider: no `scenario-` prefix. The machine provider, SSH-M, is a separate crate (ruling 40).
2. **The package**, in order 127's form: address `example.org/conduit/provider-ssh-runtime`,
   version 0.1.0, the Type `ssh-runtime.v1` offering `runtime-provider.v1` only, its item `provider`
   (`bin/provider-ssh-runtime`) implementing the three operations, not plain, with no `tools`. A
   `package` binary writes the archive. Its configuration reaches the provider through the item's
   environment at its start (required places, no `CONDUIT_` names), and the provider refuses to
   start, naming the place, where one is missing or malformed:
   - the machines it serves, each by hostname, with its SSH host, port and remote user
     (`conduit-run`), and the pinned ed25519 host key of each;
   - the path of the private key it uses, in its own state directory;
   - the remote base directory, an absolute path of the characters `A-Za-z0-9/._-` only (systemd
     does not expand `~`);
   - the run `PATH` on the remote; a run's lifetime bound; the stop timeout; the drain interval
     (the kit's `Options`), suited to SSH;
   - the remote paths of `systemd-run`, `systemctl` and `loginctl` (by default under `/usr/bin/`),
     and the unit-name prefix (by default `conduit-run-`); the later loopback order sets them;
   - the gate's bind address and keepalive.

   **Its own part.** At its first start in a state directory the provider makes a random part,
   kept there, and adds it to every unit name (`<prefix><part>-<hash>`) and every remote path
   (`<base>/<part>/runs/`, `<base>/<part>/state/`), so that two providers sharing the remote user
   never sweep or remove each other's runs.
3. **SSH.**
   - **The program:** `ssh`, found on the provider's `PATH` at startup and run by its absolute path,
     with a cleared environment, so no agent socket or variable of the provider's reaches it.
   - **The options**, all of them, and none from any user or system file: `-F none -T -e none
     -o IdentitiesOnly=yes -o IdentityAgent=none -o IdentityFile=<key> -o User=<remote user>
     -o Port=<port> -o UserKnownHostsFile=<pinned file> -o GlobalKnownHostsFile=/dev/null
     -o HostKeyAlias=<hostname> -o StrictHostKeyChecking=yes -o UpdateHostKeys=no
     -o HostKeyAlgorithms=ssh-ed25519 -o BatchMode=yes -o ConnectTimeout=5
     -o ServerAliveInterval=10 -o ServerAliveCountMax=3 -o LogLevel=ERROR -o ControlPath=<socket>`,
     with `-N -o ControlMaster=yes -o ControlPersist=no` on the master. The pinned file is written
     from the configuration. A path inside an `-o` value is written for `ssh`'s own parser: quoted
     where it holds a space, with `%` written `%%`, and never starting with `~`; test a state
     directory whose path holds a space and a `%`.
   - **The master:** one per machine, a child process the provider owns (no `-f`), its socket in a
     0700 directory with the socket's path under 80 bytes (the master first binds the path plus a
     random suffix, and macOS allows 104 bytes): the state directory where that fits, otherwise a
     private directory the provider makes under the system's temporary directory. A stale socket
     is removed before a master starts; a failed master is retried with a backoff, and while a
     machine's master is known to be down, calls for it answer at once instead of waiting
     `ConnectTimeout` each time.
   - **The remote command:** one string of POSIX single-quoted words, each word quoted (a `'`
     inside one written `'\''`), so the remote user's login shell yields exactly the argument
     vector meant. Each `systemd` tool runs by its configured absolute path, through
     `/usr/bin/env XDG_RUNTIME_DIR=/run/user/<uid>` with the uid learned at first contact.
   - **No value in an argument:** no secret, environment value or file of `identity` (PRV-022) goes
     into any argument, local or remote; the files, the environment file and the pre-check's script
     with the run `PATH` cross the session's standard input (`/bin/sh -s`). Paths, the unit's name
     and the item's own `command` are arguments.
4. **The backend.**
   - **Diagnostics.** The kit answers a backend's `Err` with its own fixed text, which names
     nothing (the kit's `Unavailable` has no payload). So that every refusal is still explicit
     (ruling 52), the backend writes each one's reason, naming the setting, variable or program and
     never a value, as one line to the provider's standard error and to a 0600 diagnostics file in
     its state directory; the tests read it there. Report the kit's fixed text as a finding.
   - **Owner lock and markers.** One provider per state directory, held by an owner lock, as order
     127's `owner.lock`. **Before staging** the backend writes a durable marker in its state
     directory holding the run's record, as order 127's markers do, and removes it after a
     confirmed stop. **Inventory** reads the markers, never the machines, so it is complete while
     a machine is off (the kit refuses to start on an inventory that fails).
   - **The sweep,** at every master connection and reconnection, before any run is admitted there:
     units with the provider's own prefix and part that have no marker are stopped by name, and
     run directories under its part that have no marker, or whose marker's run never started, are
     removed; each is reported on standard error.
   - **Declaration:** `command` items whose program is absolute or a bare name that the run `PATH`
     resolves on the remote (AGT's item, order 078, is one); a relative path with `/`, every
     `package` item and any `package_arguments` end `failed`, as order 127's do; only the
     configured machines.
   - **First contact with a machine** checks, and refuses runs there naming each that fails:
     `id -u` answers; `<systemctl> --user is-system-running` answers `running` or `degraded`;
     `<loginctl> show-user <uid> -p Linger` answers `Linger=yes`; and `<systemd-run> --version`
     names version 254 or later. Never `show-environment`, which prints the environment.
   - **Staging,** under umask 077, at `<base>/<part>/runs/<hex SHA-256 of the instance's ID>/`,
     which is removed first if present: the directory at 0700, every file of `identity` at 0600
     under `identity/`, and an environment file at 0600. The environment holds, in order 127's
     precedence, the run `PATH`, the item's environment, the secrets, the identity's variables,
     then `CONDUIT_IDENTITY_DIR` and the state-directory variable, which names
     `<base>/<part>/state/<the same hash>/` (PRV-029), kept across runs and never removed by the
     provider. Write an encoder for the environment file to the syntax `systemd.exec(5)` gives for
     `EnvironmentFile=`, covering newline, both quotes, backslash, `$`, and leading and trailing
     space, with a round-trip test against a decoder of that syntax; a name that is
     not a letter or `_` followed by letters, digits and `_`, which systemd would drop, or a value
     it cannot encode, ends `failed` naming the variable, never the value. A pre-check resolves a
     bare program with the run `PATH` on the remote, ends `failed` where it cannot, and the unit
     gets the absolute path it found.
     A staged run that is dropped without a start (as when the kit refuses it for an owed stop)
     has its run directory removed over the master, or at the next sweep where the machine cannot
     be reached.
   - **Start:** `<systemd-run> --user --pipe --wait --collect --quiet --service-type=exec
     --expand-environment=no --unit=<unit> -p EnvironmentFile=<file>
     -p WorkingDirectory=<run dir> -p RuntimeMaxSec=<the lifetime the kit supplies>
     -p TimeoutStopSec=<stop timeout> -p 'ExecStopPost=:/bin/rm -rf -- <run dir>'
     -- <absolute program> <args>`, over the machine's master. The session's standard input and
     output are the kit's duplex channel, served by the kit's gate on the engine's machine. The
     session's standard error, the run's with `--pipe`, is drained at once and continuously into
     a private file in the provider's state directory, as order 127 drains the run's. `start`
     returns `Ok` only when `systemctl --user show` confirms the unit active, within a bound;
     otherwise it returns `Err` and the kit stops it. Once the unit is active the environment file
     is removed.
   - **A cancelled start.** Cancelled before `systemd-run` is sent, it launches nothing. Cancelled
     after, it closes the session's standard input and waits, within a bound, until the session
     has exited or `show` sees the unit loaded, and only then returns `Err` (the kit's contract
     for `start`), so that no launch follows the kit's stop.
   - **A session's end:** when a run's SSH session ends for any reason, the backend sends the
     unit's stop at once over the master, as a best effort; the kit's stop and its owed stops stay
     the guarantee.
   - **Stop,** in this order: `<systemctl> --user stop <unit>`, which ends the unit's whole cgroup,
     setsid children included; remove `runs/<hash>/`, so that a `systemd-run` still in flight finds
     no environment file or working directory and fails; then confirm the unit inactive or absent,
     and remove the marker. `unknown` where the machine cannot be reached, and the kit keeps the
     stop owed and carries it out at the next contact (PRV-021).
   - **State:** from `systemctl --user show` of the unit; `unknown` while the machine cannot be
     reached.
   - **State store:** a file in the provider's state directory, written atomically, as order 127's.
5. **Its contract text,** in `contracts.yaml` and the README, states:
   - the gate and its silent-cut bound, with the kit's macOS caveat, since the gate is on the
     engine's machine, and apart from it the SSH session's own bound (`ServerAliveInterval` times
     `ServerAliveCountMax`, after `ConnectTimeout`);
   - the remote user `conduit-run`, with no sudo, shared by every run and by the work-order workers
     (ruling 77), and, in ruling 77's words, that **PRV-025's isolation is unmet until each
     Agent has a user of its own**. Under ruling 52 it names what follows, and that the remote's
     setup closes none of it: any run can read every other run's identity files and environment
     (`/proc/<pid>/environ`); write into another run's standard input and output through
     `/proc/<pid>/fd`; control the shared user manager (stop units,
     `systemctl --user set-environment`); write a drop-in under `~/.config/systemd/user/` for the
     prefix, or a file under `~/.config/environment.d/`, which every later run's unit reads, and so
     read later runs' secrets; and start `systemd-run --user` units of its own, which escape the
     provider's stop and `RuntimeMaxSec`;
   - where secrets and `identity` rest and for how long (PRV-011, PRV-022): the environment file
     from staging until the unit is active; the identity files until the unit stops for any cause
     (`ExecStopPost` removes the run directory) or, for a run that never started, until the next
     removal or sweep; the run's environment for its life;
   - the bound, the stop timeout, the state directory and the `PATH`;
   - the teardown: the stop sent at the session's end, the run's standard input closing when the
     remote `sshd` notices a cut (its `ClientAliveInterval`), `RuntimeMaxSec` as the backstop, and
     the sweep;
   - **the drainer it does not have:** decision D's external stop-drainer, which outlives the
     provider's retirement, is not built by this order. A stop still owed when the provider
     retires is carried out only if the provider starts again in the same state directory; until
     then `RuntimeMaxSec` is the only end of that run, and the run directory of a run that never
     started stays on the remote;
   - the kit's admission and stop-waiting rules, as its README requires of a runtime's contract;
     where owed stops are kept; the owner's rule that a held stop is carried out before any new
     run on that machine starts, and that a start refused for this names the owed stop (the kit
     enforces it); and D15's four fields as
     [S10's README](../../experiments/021-s10-swapped-providers/README.md#measures) defines them.
6. **The remote's setup,** listed in the README as what the owner's ruling-77 setup must provide,
   with the reason for each: `systemd=true` under `[boot]` in `wsl.conf`, kept when the lockdown
   rewrites it; lingering on for `conduit-run`; its login shell `/bin/sh`, which reads no start-up
   file for a command; in `sshd`'s configuration `UsePAM yes` (so that `pam_systemd` starts the
   user's session), `PermitUserRC no`, `PermitUserEnvironment no`, an `AuthorizedKeysFile` that
   `conduit-run` cannot write, `ClientAliveInterval` and `ClientAliveCountMax` with the bound they
   give, and `MaxSessions` above the runs it carries plus two; and no sudo. This order changes no
   machine.
7. **Tests, with a fake remote:** an executor trait that the tests replace, which records each
   command and the bytes sent on standard input and answers as the remote would. Cover: the exact
   command lines, with no value, secret or identity file in any argument; the remote command's
   quoting, with arguments holding spaces, quotes, newlines, `$` and `%`; the `-o` paths; the
   staged files and their modes; the environment file's contents, by synthetic markers, and the
   encoder's round trip and refusals; state parsing from `systemctl show` output; a start
   confirmed only once the unit is active, and one whose unit fails; a start cancelled after the
   send, and a `systemd-run` that arrives after the stop, finding no run directory; an unreachable
   machine giving `unknown` and an owed stop, carried out at the next contact; inventory from the
   markers while every machine is unreachable; the sweep at a reconnection, including a unit and a
   run directory with no marker and a staged run dropped without a start; each first-contact check
   failing, named in the diagnostics; a bare program that the pre-check cannot resolve ending
   `failed`; and a start refused while another instance owes a stop on that machine, naming it.
   What only a real `sshd` and a real user `systemd` show waits for the later orders.
8. **What this order does not do:** reach the Windows computer or any other machine, or start an
   `sshd`; change the kit (report anything it lacks as a finding); build decision D's external
   drainer; or build SSH-M.

## Scope

Change only `packages/provider-ssh-runtime/` (new).

## Done when

In the crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --release` (a skipped test is untested, never a pass), and
`cargo run --release --bin package` twice, with the same digest. From the repository root:
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with `bash`.
Each prints nothing when it passes:

```bash
D=packages/provider-ssh-runtime
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rln -E "PRIVATE KEY|BEGIN CERTIFICATE" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

`target/` and `dist/` stay ignored and unstaged. No real hostname, tailnet name or user's home path
goes into the crate: use synthetic ones. Report each default of the configuration as your own
choice. Do not stage anything.
