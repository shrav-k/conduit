# Work order 140 (SSH-R, finished): the launch fence, MAN-017, the kit's reasons and the setup

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no other machine. Run one
cargo command at a time, and never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never print a key, a token
or a certificate's key, even one that a test made.

## What and why

[Order 137](137-ssh-r-runtime.md) built the SSH runtime provider, `packages/provider-ssh-runtime/`,
in two rounds. Your working tree begins at its last commit, rebased onto `main` after
[order 138](138-a63-rk-backend-reasons.md), which changed the kit. Order 137's decisions all still
apply; read it, then the crate's README. Its final verifier left two musts, and the admin has
since settled the remote's setup. This order finishes it.

## The decisions it applies

1. **Build against the kit as it now is.** Order 138 made `Unavailable` a struct with an optional
   reason and a constant of the same name: it is no longer `Copy`, and it must not be used as a
   pattern. Make the crate build and pass `cargo clippy --all-targets -- -D warnings` against it,
   changing nothing in the kit.
2. **Every place the item binds is required (MAN-017).** Four places that the item's environment
   binds, `systemdRun`, `systemctl`, `loginctl` and `unitPrefix`, are optional in the Type's
   configuration schema (`contracts.yaml`, near line 130; `conduit.yaml`, near lines 21 to 24), so
   MAN-017 refuses a template that selects the item. Make every bound place required, keeping its
   `default` keyword, which the engine fills before it validates (SCH-009), as `maxLifetime`
   already does; update the README's account of them (near lines 130 to 132). Add a package test
   that checks MAN-017's rule for the item: every place its environment binds is required and
   scalar in the schema.
3. **A launch fence that lasts, so that an unacknowledged launch's stop can clear.** Today the run
   directory's path is the same for every staging of an instance, so removing it fences a queued
   `systemd-run` only until the next staging recreates it, and a start that times out before the
   unit is confirmed keeps its stop owed for good, which blocks every new run on that machine.
   - Give each staging its own directory, `runs/<hash>/<generation>/`, with a random generation
     of 16 hex characters that is never reused, and point the unit's environment file, its start
     check and its `WorkingDirectory` at that generation's paths. `ExecStopPost` removes the
     generation's directory.
   - Staging and a stop remove all of `runs/<hash>/`, every generation, before they go on. The
     sweep removes every `runs/<hash>/<generation>/` except the generation of a marker whose run
     started, and every name there that is not a generation's.
   - An owed stop for a launch that was never acknowledged clears once a stop has confirmed the
     unit inactive or absent and removed `runs/<hash>/`: a `systemd-run` arriving later then fails
     at its generation's missing working directory and start check, and runs nothing. No
     acknowledgement is needed: the unit's name is the instance's, so a loaded unit cannot be
     tied to a generation; remove the check that keeps the debt (`src/backend.rs`, near lines 687
     to 692).
   - Tests, with the fake remote: a start that times out; its stop; a late launch for the removed
     generation failing without running the program; the owed stop cleared; a new staging of the
     same instance in a new generation, which the late launch cannot reach; and the sweep removing
     stale generations. The test that asserts the old rule, that the stop stays owed
     (`tests/runtime.rs`, near lines 786 to 834), now asserts that it clears. Keep every other test
     of order 137 passing.
   - The contract and README say how the fence works and what it relies on: systemd refusing to
     start a unit whose working directory is missing, which the owner's test host will confirm.
     Update the text that states the old rule (README, near lines 80 to 87 and 194;
     `contracts.yaml`, near lines 74 to 81).
4. **The kit's reasons.** Order 138 lets a backend attach its own reason to a failed staging or
   start, and only those reach `run.v1`'s answer; a reason at startup is discarded by the kit. Give
   each refusal of `stage` and `start` that the backend now writes to its diagnostics the same
   text as a reason, `Unavailable::because(...)`, naming the setting, variable, program or machine
   and never a value: among them an environment name or value it cannot pass, a program the
   pre-check cannot resolve, the run limit of decision 5, and a connection, first-contact check
   or sweep that fails when staging reaches it (`src/backend.rs`, near lines 425 to 503, where
   `connect` now drops the check's reason). Keep the diagnostics line; startup keeps writing to
   standard error only. Test that `run.v1`'s answer now carries each. Update the text on the kit's
   fixed answers (README, near lines 116 to 118; `contracts.yaml`, near lines 110 to 112).
5. **The remote's setup, as ruling 77's setup now provides it.** Replace the README's "Remote
   setup required from ruling 77" with exactly this list, saying it comes from the owner's
   ruling-77 setup, and with the reason for each:
   - `conduit-run`'s login shell is `/bin/sh`;
   - in sshd's `Match User conduit-run` block: `PermitUserRC no`, `ClientAliveInterval 30`,
     `ClientAliveCountMax 4` and `MaxSessions 64`;
   - lingering is on for `conduit-run`;
   - a root-owned `AuthorizedKeysFile` (`/etc/ssh/conduit-keys/%u`), with `restrict` on the key's
     line;
   - `UsePAM` is on;
   - the lockdown keeps `systemd=true` under `[boot]` in `wsl.conf`;
   - `PermitUserEnvironment` stays at its default, `no`, since it cannot go in a `Match` block.

   One sentence after the list names what the crate also needs there and checks: systemd 254 or
   later, a base directory that `conduit-run` can write, the pinned host key and the installed
   private key.

   **The bounds these set.** The remote `sshd` notices a silent cut within 30 s times 4, 120 s:
   the contract and README say so. **The run limit:** the master opens no session; the
   provider's serialized commands take one at a time, and each instance staging or running takes
   one. Under `MaxSessions 64`, a machine carries at most 62 runs, which leaves one session spare;
   past it, `sshd` refuses a session, and the failed command today drops the master and with it
   every run on that machine. So add a per-machine run limit to the configuration, a required
   place with the default 62, and make `stage` refuse a run past it, with a reason naming the
   machine and the limit, before it opens a session. Test it.
6. **What this order does not do:** change the kit, build decision D's external drainer, test
   over a real `sshd` or `systemd`, or reach any machine.

## Scope

Change only `packages/provider-ssh-runtime/`.

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
goes into the crate. Report each choice you make as your own. Do not stage anything.

## Record

Run on 2026-11-12 on the Windows computer (WSL), stacked on order 137's last commit rebased onto
`main` (cd62eed): the worker (842 s) and a fresh verifier (247 s), which found nothing to change in
one round. Each staging now has its own directory, `runs/<hash>/<generation>/`, with a 16-hex-digit
generation reserved for good, so a late `systemd-run` for a removed generation fails at its missing
working directory and start check; a stop that confirms the unit inactive or absent and removes
`runs/<hash>/` clears the owed stop without an acknowledgement, and the sweep removes stale
generations. Every place the item binds is required, keeping its default (MAN-017, SCH-009), with
a package test. Each staging and start refusal carries its reason to `run.v1`'s answer through the
kit's `Unavailable::because`. The README's setup is exactly the owner's ruling-77 setup as the
admin gave it, with the 120 s bound for a silent cut and a run limit (`runLimit`, default 62 under
`MaxSessions 64`) past which staging refuses before it opens a session.

On the Mac, the lane coordinator found 20 of the runtime tests failing: their fake remote runs
the provider's shell scripts on the test machine, and macOS's `chmod` read `chmod 700 -- PATH` as a
file named `--`, and its `find` has no `-printf`. **Admin-side fix (a small finding, ruling 71):**
`chmod -- MODE PATH`, which both read alike, and the sweep's run-directory listing written as a
POSIX loop over the directory's entries, which also refuses an unreadable `runs` directory, as
`find` did, and does not follow `runs` when it is a symbolic link. A fresh reader checked it,
under dash and macOS's `sh`, and found the scripts unchanged in effect on the Linux remote. Then,
with two test threads: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` clean,
all 33 tests pass, and two archive builds give the same digest on the Mac
(`sha256:94e32f55190e9286c5c2b7af965ae0054971722a96f64bfe89d1a7aa8b5cf051`; no cross-host digest
is claimed).

The worker's own choices: permanent empty reservation files for generations, created atomically,
so that no generation is reused across restarts (one small file per staging stays in the state
directory); the place `runLimit`, from 1 to 62; markers counted conservatively per machine.
