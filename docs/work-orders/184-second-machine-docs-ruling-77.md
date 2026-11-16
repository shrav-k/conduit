# Work order 184: the second machine's documents agree with ruling 77

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes three documents. It has no
network access and needs none. It needs no cargo command; if you run one, run one at a time. Reach
no other machine, and run none of the commands this order puts into the guides: they are for the
owner and the coordinator on the Windows computer. Never print a whole large file: read the plan
and the guides in parts, with `grep -n` and `sed -n`. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 (order 42 of [its orders](../proposals/full-review/orders.md))
found four places where the documents about the second machine still disagree with
[ruling 77](../proposals/ruling-77/README.md), the Windows computer's setup, or with answers the
owner has given since:

- **The Docker guide's group step.** Step 2 of `docs/guides/docker.md` (lines 9 to 17) installs
  Docker Engine in Ubuntu and runs `sudo usermod -aG docker "$USER"`, and S3's README sends every
  run to that guide (`experiments/014-s3-two-machines/README.md` lines 190 and 191). Under ruling
  77 the step leaves `conduit-run`, which has no sudo, with a bad choice: no Docker at all, or
  membership of the `docker` group, which is root inside the distro and goes against points 2
  and 4 ([FR-329](../proposals/full-review/findings/P05-second-machine.md#fr-329), as its verifier
  corrected it: the guide does say how Docker is installed, and that is the conflict). The
  coordinator's D51
  ([decisions](../proposals/full-review/decisions.md#the-providers-and-the-second-machine))
  decides that Docker on the second machine runs rootless as the run user. That needs the owner's
  sudo once, for the user-ID maps, which is still to be confirmed on the machine. The guide's step
  is replaced before the owner follows it, and S3's plan states the limit. D50 keeps one run user.
- **MC4-002 still held.** S3's plan marks MC4-002 as an owner hold
  (`experiments/014-s3-two-machines/plan.md` lines 19 and 258). The owner answered it on
  2026-11-12 ([ruling 42's later answer](../proposals/ruling-42/README.md#later-answers);
  `IN_FORCE.md` topic 11, lines 266 and 267). Every runtime provider Conduit ships carries out a
  stop it owes on a machine before it starts any run there, as SSH-R does
  (`packages/provider-ssh-runtime/README.md` line 107), and the overlap is a limit stated in
  `spec/` ([FR-318](../proposals/full-review/findings/P05-second-machine.md#fr-318), its
  verifier's point (c) and corrections). D64 gives the same two lines to the order that adds the
  dated amendments (work order 186, the review's order 35). This order takes them, because its
  row's done-when names them, and 186 leaves them alone.
- **Workers during S3's runs.** The plan's run gate (lines 186 to 191) does not say that no work
  order runs on the second machine during a counted run. Work orders run as the same
  `conduit-run`, and each loss phase stops the distro with them in it
  ([FR-330](../proposals/full-review/findings/P05-second-machine.md#fr-330), point 4, which its
  verifier upholds). Ruling 76, point 6, lets ordinary workers run there over SSH until the
  computer is a node, and keeps Conduit's own runs of work orders (A78) on the Mac until then.
  Neither of those is barred during a counted run.
- **Where the host key is read.** The second-machine guide asks the owner for the SSH host key
  that the provider pins (`docs/guides/second-machine.md` lines 114 to 118), but does not say where
  to read it. A key read over the SSH connection it is meant to check proves nothing. It must be
  read at the Windows computer (FR-318's corrections; ruling 68, R2, asks the same of admitting a
  machine).

FR-318's other corrections correct the analyst's own assessment: "no live process to act on;
leftover run files and a replayed credential remain", and that a Windows host answering in place
of silence is unobserved. No document in this order's scope repeats either claim, so they change
nothing here. FR-329's other points change nothing here either. D50 keeps one run user and
SSH-R's user as it is. The plan already gives V5 (PRV-025) to A63-VR's provider suite (line 180).
A note on what the real runs on the second machine do not show belongs with S3's README's dated
amendments (D64), not here.

The order lands before S3-O2, when the owner would follow the Docker guide. It is paper: it
changes no behavior of Conduit, no text under `spec/` and no vector, so it has no declaration
file. S3's plan binds nothing and changes nothing the owner accepted in S3's README.

## The decisions it applies

1. **The Docker guide's step 2 runs Docker rootless as `conduit-run`** (D51; FR-329). Replace
   step 2 of `docs/guides/docker.md` (lines 9 to 17, from "**On the second machine**" to "then
   log out of Ubuntu and back in.") with a step that says, in this order:
   - Docker Engine goes inside Ubuntu-24.04, not Docker Desktop on Windows, so that its daemon
     runs in the distro the owner gave to Conduit. It runs rootless, as `conduit-run`, the one run
     user of ruling 77, point 2. The words "so that the engine on Linux talks to it directly"
     go: the engine stays on the Mac (S3's answer O3, under ruling 75, D1).
   - This sentence, as written: "No user joins the `docker` group: its members are root inside
     the distro, which ruling 77 keeps from `conduit-run`."
   - **The owner, with sudo, once**, in Ubuntu-24.04 as the owner's own user. Keep the current
     first command, Docker's install script piped to `sh` (line 13), exactly as it is. Follow it,
     in the same `sh` block, with these four lines:

     ```sh
     sudo systemctl disable --now docker.service docker.socket
     sudo rm -f /var/run/docker.sock
     sudo apt-get install -y uidmap dbus-user-session
     grep '^conduit-run:' /etc/subuid /etc/subgid
     ```

     Then say what each command does:
     - Under WSL the install script says that it detected WSL, recommends Docker Desktop and
       waits 20 seconds; the owner lets it go on. It installs Docker's packages, the rootless
       extras among them.
     - The next two commands stop the system-wide daemon, which runs as root, and remove its
       socket.
     - The third installs the tools for the user-ID maps and the user's D-Bus session.
     - The `grep` must print one line from each file, each giving `conduit-run` a range of at
       least 65536 IDs (the line's third field); these ranges are the user-ID maps. Where a file
       has no such line, the owner adds a range that no other line of either file uses, with
       `sudo usermod --add-subuids FIRST-LAST --add-subgids FIRST-LAST conduit-run`, FIRST and
       LAST being the range's first and last ID.
   - **The coordinator, as `conduit-run`, without sudo**, installs the rest, as ruling 77's step 3
     has it, in one `sh` block:

     ```sh
     dockerd-rootless-setuptool.sh install
     systemctl --user enable docker
     docker context use rootless
     docker info --format '{{.SecurityOptions}}'
     ```

     Then say:
     - The setup tool creates and selects the CLI context `rootless`, and the third command makes
       sure of it.
     - The last command's output includes `name=rootless`.
     - A CLI context, not an environment variable, names the daemon, because `conduit-run`'s
       login shell reads no startup file (SSH-R's ruling-77 setup).
     - Lingering, on for `conduit-run` since ruling 77's step 1, keeps the daemon running with
       nobody logged in.
   - This sentence, as written: "These steps have not yet run on the Windows computer; the
     coordinator confirms them there at S3-O2." Then say what happens when a step fails:
     - Whoever ran it stops and reports the step and its last line of output. They do not add
       any user to the `docker` group instead.
     - Name the two failures foreseen, and point to Docker's troubleshooting page for rootless
       mode for each. The first is a D-Bus error from the setup tool or the daemon. The second is
       a refusal to create a user namespace: Ubuntu 24.04 restricts unprivileged user namespaces,
       and the page says that Docker's deb packages, which the install script installs, need no
       AppArmor change.
     - A remedy there that needs sudo is the owner's.
   - Its sources:
     - a link with the text "Docker's rootless mode" to
       https://docs.docker.com/engine/security/rootless/
     - a link with the text "its troubleshooting page" to
       https://docs.docker.com/engine/security/rootless/troubleshoot/
     - a link with the text "the full review's D51" to
       ../proposals/full-review/decisions.md#the-providers-and-the-second-machine, called a
       decision of the coordinator's, open to the owner's strike.

   The coordinator checked these commands and facts on 2026-11-15 against those two pages and the
   install script: the script installs the rootless extras, and pauses 20 seconds under WSL. Write
   them as given; you have no network and need none. Steps 1 and 3, the opening paragraph and the
   closing paragraph do not change. Write no home directory, host name or address: the commands
   name the user, never a path in its home.
2. **The second-machine guide points to it** (D51). In `docs/guides/second-machine.md`, at the end
   of step 3 of ruling 77's setup (lines 91 to 94, ending "a clone of the public repository with
   pushes disabled."), add: "Where a run needs Docker, step 2 of the Docker guide sets it up
   rootless as `conduit-run`, with the owner's sudo once; no user joins the `docker` group." The
   words "the Docker guide" link to docker.md, in the same folder.
3. **The host key is read at the Windows computer** (FR-318's corrections; ruling 68, R2). In the
   same guide's section "What to hand the coordinator" (lines 114 to 118), after its first
   sentence, add, as written:
   "Read the host key at the Windows computer, in Ubuntu-24.04's own terminal there, never over
   SSH: a key read through the connection it is meant to check proves nothing.
   [Ruling 68](../proposals/ruling-68/README.md#the-decisions), R2, asks the same of admitting a
   machine: the fingerprint the person read on it. There, `cat /etc/ssh/ssh_host_ed25519_key.pub`
   prints the key and `ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub` its fingerprint. Where
   the coordinator already holds a key for the machine that was not read there, the owner reads
   the fingerprint there once, and the coordinator compares it with the held key's before the SSH
   provider's first run there and tells the owner of any difference."
   The section's other sentences stay: the key and its fingerprint stay outside the repository.
4. **MC4-002 is not held** (FR-318's corrections; ruling 42's later answer). In
   `experiments/014-s3-two-machines/plan.md`:
   - Line 19 becomes, as written, on one line: "> - MC4-002, answered on 2026-11-12 (ruling 42's
     later answer): not held. Every runtime provider Conduit ships, SSH-R among them, carries out
     a stop it owes on a machine before it starts any run there; the overlap is a stated limit,
     which S3-H observes, with no new rule against it."
   - In line 258, the sentence "MC4-002’s overlap question stays held." (its apostrophe is a
     curly one) becomes, as written:
     "MC4-002 is not held: under
     [ruling 42's later answer](../../docs/proposals/ruling-42/README.md#later-answers) of
     2026-11-12
     ([IN_FORCE.md, topic 11](../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items)),
     every runtime provider Conduit ships, SSH-R among them, carries out a stop it owes on a
     machine before it starts any run there, and a run whose stop did not succeed may run beside a
     later instance, a limit stated in `spec/` (LIF-009, LIF-036, PRV-021)."

   Lines 129 and 130 (S3-L2) and line 175 (S3-H) already say the overlap is observed and not
   fenced, and do not change.
5. **Docker at S3-O2, and its limit** (D51). In the same plan:
   - In the paragraph "S3-O1 / O2" (lines 100 to 109), after the sentence "The owner supplies sudo
     for that setup." (line 106), add, as written: "Docker there runs rootless as `conduit-run`,
     by step 2 of [the Docker guide](../../docs/guides/docker.md): the owner's sudo once, for
     Docker's packages and the user-ID maps, then the coordinator as `conduit-run`; no user joins
     the `docker` group, whose members are root in the distro
     ([the full review's D51](../../docs/proposals/full-review/decisions.md#the-providers-and-the-second-machine))."
   - At the end of "## Limits" (lines 300 to 307), add, as written: "On the second machine Docker
     runs rootless as `conduit-run`, and its socket belongs to that user, so the isolated profile
     keeps runs apart only while no unconfined run and no work-order worker runs as `conduit-run`
     beside it (the full review's D51)."
6. **No work order during a counted run** (FR-330, point 4). At the end of the paragraph
   "S3-R1–4" (lines 186 to 191, ending "including actual outage/stop/effect evidence."), add, as
   written: "From the start of each run's fresh lab to the end of its last phase, the second
   machine carries no work-order worker, neither one the coordinator starts over SSH
   ([ruling 76](../../docs/proposals/ruling-76/README.md#the-decisions), point 6) nor one Conduit
   runs (A78): workers run as `conduit-run` beside the runs and the rootless Docker daemon, and
   each loss phase stops the distro with every process in it."
7. **A revision line.** In the plan's "## Status", after the line that begins "Revised on
   2026-11-11 by work order 121" (line 28), add a paragraph: "Revised on YYYY-MM-DD by work order
   184 (the full review's order 42): Docker rootless as `conduit-run` at S3-O2 and its limit,
   MC4-002 no longer held, and no work-order worker on the second machine during a counted run.",
   with the date you write it.
8. **What stays.** These do not change:
   - Ruling 77's record (FR-330's verifier: it describes the pre-registration as it stood).
   - S3's README, accepted under ruling 75, D1. Its dated notes come with work order 186 (the
     review's order 35) and with ruling 78, point 4's amendments.
   - The guide's section "How S3 cuts the network".
   - In the plan: its checkpoint (lines 30 to 34), its S3-OF item and option texts, and its lines
     that wait on D4, which those orders change.
   - `IN_FORCE.md`, and anything under `spec/`.

   Wrap new prose like the lines around it, at most 100 characters a line where they are wrapped;
   the plan's "In short" lines stay one line each. Rewrap no line you do not change.

## Scope

- `docs/guides/docker.md`: step 2 only (decision 1).
- `docs/guides/second-machine.md`: step 3 of ruling 77's setup, and the section "What to hand the
  coordinator" (decisions 2 and 3).
- `experiments/014-s3-two-machines/plan.md`: lines 19, 28, 100 to 109, 186 to 191, 258 and 300
  to 307, as decisions 4 to 7 say.

Change no other file.

## Done when

From the repository root, each passes:

```sh
grep -n 'usermod -aG docker' docs/guides/docker.md    # prints nothing
git grep -n -e 'Owner hold MC4-002' -e 'overlap question stays held' -- experiments/014-s3-two-machines   # prints nothing
grep -c '^ *dockerd-rootless-setuptool\.sh install$' docs/guides/docker.md    # prints 1
grep -c 'uidmap dbus-user-session' docs/guides/docker.md    # prints 1
tr -s ' \n' '  ' < docs/guides/docker.md | grep -o -F 'No user joins the `docker` group: its members are root inside the distro, which ruling 77 keeps from `conduit-run`.' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < docs/guides/docker.md | grep -o -F 'These steps have not yet run on the Windows computer; the coordinator confirms them there at S3-O2.' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < docs/guides/docker.md | grep -o -F "the full review's D51" | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < docs/guides/second-machine.md | grep -o -F 'sets it up rootless as `conduit-run`' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < docs/guides/second-machine.md | grep -o -F 'never over SSH: a key read through the connection it is meant to check proves nothing' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < docs/guides/second-machine.md | grep -o -F 'ssh-keygen -lf /etc/ssh/ssh_host_ed25519_key.pub' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < experiments/014-s3-two-machines/plan.md | grep -o -F 'carries out a stop it owes on a machine before it starts any run there' | wc -l | tr -d ' '    # prints 2
tr -s ' \n' '  ' < experiments/014-s3-two-machines/plan.md | grep -o -F 'Docker there runs rootless as `conduit-run`' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < experiments/014-s3-two-machines/plan.md | grep -o -F 'keeps runs apart only while no unconfined run and no work-order worker runs as `conduit-run` beside it' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < experiments/014-s3-two-machines/plan.md | grep -o -F 'the second machine carries no work-order worker' | wc -l | tr -d ' '    # prints 1
tr -s ' \n' '  ' < experiments/014-s3-two-machines/plan.md | grep -o -F 'by work order 184' | wc -l | tr -d ' '    # prints 1
python3 tools/check_docs.py              # Errors: 0
python3 tools/check_private_paths.py     # Errors: 0
git diff --check
git status --short                       # lists only the three files in scope
```

On `main`'s text the first two checks fail, because it still holds the step and the hold, and
each check above that prints 1 or 2 prints 0. Show it without changing any file:

```sh
git show HEAD:docs/guides/docker.md | grep -c 'usermod -aG docker'    # prints 1
git show HEAD:experiments/014-s3-two-machines/plan.md | grep -c -e 'Owner hold MC4-002' -e 'overlap question stays held'    # prints 2
git show HEAD:experiments/014-s3-two-machines/plan.md | tr -s ' \n' '  ' | grep -o -F 'the second machine carries no work-order worker' | wc -l | tr -d ' '    # prints 0
```

Do not stage anything. In your report, mark each choice you made yourself as your own, and give
the last line of every check. The admin then shows the pull request's secret scan passing. A
fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `2f33a53e`: the worker (276 s) and a fresh Codex verifier
(235 s), which found nothing to change in one round. Step 2 of the Docker guide now sets Docker
Engine up rootless as `conduit-run`. The owner, with sudo once, runs Docker's install script, stops
and disables the system-wide daemon and removes its socket, installs `uidmap` and
`dbus-user-session`, and checks `conduit-run`'s ranges in `/etc/subuid` and `/etc/subgid`. The
coordinator, as `conduit-run` without sudo, runs `dockerd-rootless-setuptool.sh install`, enables
the user service and selects the context `rootless`. No user joins the `docker` group, and whoever
ran a step that fails stops and reports it. The steps have not yet run on the Windows computer; the
coordinator confirms them there at S3-O2. Step 3 of the second-machine guide's ruling-77 setup
points to that step. Its section "What to hand the coordinator" has the host key read at the Windows
computer, never over SSH, and a key held from elsewhere compared with a fingerprint read there
before the SSH provider's first run. S3's plan no longer holds MC4-002 (ruling 42's later answer).
It states Docker at S3-O2 and its limit, bars any work-order worker from the second machine during a
counted run, and has a revision line. A fresh read-only Opus reader then verified the order (ruling
79, point 2). It ran the order's checks, which gave the expected values, and showed on `main` that
each can fail. `check_docs.py`, `check_private_paths.py`, `check_rulings.py` and `check_roadmap.py`
gave no errors, and `git diff --check` was clean. It read each change against its sources (FR-329,
FR-330, FR-318, D51, D64, ruling 42's later answer and rulings 76, 77 and 78) and found no private
detail and no must. It could not check Docker's pages or the install script, having no network. All
five of its shoulds were applied before merge. One link to Docker's troubleshooting page now covers
both foreseen failures. The reason `conduit-run`'s sessions read no startup file is the one SSH-R's
README gives, the login shell `/bin/sh`, with a link to it. The Docker guide names steps 1 and 3 as
those of ruling 77's setup in the second-machine guide, and links to it; step 1 has the link again,
since "that setup's step 1" would have read as SSH-R's setup. The host-key section names what stays
outside the repository instead of "They". The plan's `[topic-11]` definition moved to its foot,
beside `[D51]`, so the owner's list renders tight again, and the changed lines were rewrapped to at
most 100 characters.

A second fresh Opus reader read the coordinator's fix (ruling 79, point 2) and found no must; its
one wording point on this Record was applied.
