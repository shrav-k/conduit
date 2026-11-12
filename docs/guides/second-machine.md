# A second machine

For S3, where S1 and S2 run across two machines with a partition and a lost machine, and A74,
its probe without an engine. The second machine is the owner's Windows computer, running
Ubuntu-24.04 under WSL2 ([ruling 72](../proposals/ruling-72/README.md#later-answers),
[ruling 77](../proposals/ruling-77/README.md)): the engine stops the programs it runs by process
group, which Linux has and Windows does not. A native Windows build can come later.

**For testing only** ([ruling 50](../proposals/ruling-50/README.md)): adding a machine is to be
done through Conduit. [Ruling 51](../proposals/ruling-51/README.md) accepts two modes for a
machine the owner controls: managed, enrolled as a node, or peer, running its own engine.
Another party's machine is only ever a peer. Network membership grants no access; root reaches
remote Agents through Conduits. How a person checks a joining machine is
[ruling 56](../proposals/ruling-56/README.md)'s, amended by ruling 68 (point 9). Enrollment is in
`spec/` ([enrollment.yaml](../../spec/promises/enrollment.yaml), order 038b); its core, order
049a1, for the side branch `s3/m2`, is checkpointed unmerged after five rounds on 2026-11-12,
not yet on `main`. Until the computer is a node, this manual setup serves A74 and
S3's preparation, and work-order workers run on it over SSH, as on the Mac
([ruling 76](../proposals/ruling-76/README.md), point 6).

The first setup below ran on 2026-11-11, and workers have run there since. Ruling 77's setup is
the owner's. On 2026-11-12 its steps 1 to 5 were done, but for three parts, which can
run now (at the handover of 2026-11-12, orders 071 and 080 are checkpointed and the computer is
idle): bubblewrap if it is missing (step 3), the restart that applies the lockdown
(step 4), and taking the coordinator's key out of the owner's account (step 2). Tell the
coordinator where a step differs.

## How the computer joins

[Ruling 77](../proposals/ruling-77/README.md) settles it for now
([IN_FORCE.md, topic 10](../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment));
the owner may revisit any point.

- **An SSH provider:** a machine provider, SSH-M, not yet built, and a runtime provider, SSH-R
  ([its README](../../packages/provider-ssh-runtime/README.md); merged by orders 137, 140, 145,
  146, 149 and 150; its key sweep, order 147, is written and not dispatched). Tailscale stays
  the network and a candidate machine provider for later, but is not a Resource for this computer: Windows stays online in Tailscale while Ubuntu is stopped, and an
  SSH probe reaches Ubuntu itself. This departs from ruling 50's words.
- **One run user, `conduit-run`, with no sudo,** for Conduit's runs, the work-order workers and
  the coordinator's key, in place of the owner's own user, which has sudo. Until each Agent has a
  user of its own, PRV-025's isolation is unmet: any run can read and disturb another.
- **The distro locked down:** Windows interop, `appendWindowsPath` and automount off, so nothing
  in Ubuntu-24.04 starts Windows programs, has the Windows path or mounts the Windows drives.
- **The engine's certificate through `tailscale serve`** on the Mac: a Let's Encrypt certificate
  in front of the loopback listener (`--https-listen`) that order 079 built on `s3/m2`. The Mac is
  first renamed in the tailnet to a name that means nothing, since that hostname enters the public
  certificate-transparency logs. Access stays limited to the tailnet, with a token on each request.
- **On demand:** the computer is Conduit's whenever it is on and the owner has not paused it. On
  the owner's "pause" the coordinator checkpoints and stops the work there, and starts nothing
  there until the owner's "resume". The owner then stops the distro with
  `wsl --terminate Ubuntu-24.04`; `wsl --shutdown` would stop every distro, Docker Desktop's too.

## The setup

**First setup (2026-11-11), on the owner's own Linux user:**

1. WSL2 with Ubuntu-24.04, the computer's default distro. Where it is missing, in PowerShell as
   administrator: `wsl --install -d Ubuntu-24.04`, then restart.
2. Give WSL the computer's own network address: in `%UserProfile%\.wslconfig` add

   ```ini
   [wsl2]
   networkingMode=mirrored
   ```

   then run `wsl --shutdown` and start Ubuntu again. Without it, WSL sits behind a private
   network that the Mac cannot reach directly.
3. Join both machines to one tailnet: install Tailscale on the Windows computer and on the Mac,
   and sign both in to the same account.
4. In Ubuntu, install what the engine needs:

   ```sh
   sudo apt update && sudo apt install -y build-essential git openssh-server curl
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   and Node 21.5.0 or later, through `nvm`, as on the Mac.
5. Let the coordinator in by an SSH key made for this, never a password: add the public key it
   gives you to `~/.ssh/authorized_keys` of that user, start SSH with `sudo service ssh start`, and
   turn off password logins in `/etc/ssh/sshd_config` (`PasswordAuthentication no`). Ruling 77's
   step 2 moves the key to `conduit-run`.

**Ruling 77's setup, with the owner's sudo:**

1. Create `conduit-run`: no password, no sudo, the login shell `/bin/sh`, and lingering on, so
   that its user services run with nobody logged in. The owner's home stays unreadable to it.
2. Admit the coordinator's key for `conduit-run` only, from a root-owned key file with
   `restrict`, so that `conduit-run` cannot add keys; then take the key out of the owner's own
   account. sshd's `Match User conduit-run` block and the other settings SSH-R needs are in
   [its setup](../../packages/provider-ssh-runtime/README.md#the-owners-ruling-77-setup).
3. Install the shared system packages, bubblewrap for Codex's sandbox among them. The coordinator
   installs the rest as `conduit-run`, in its own home and without sudo: `nvm` with Node and the
   Codex CLI, `rustup`, a Python environment for the checks, and a clone of the public
   repository with pushes disabled.
4. Write the lockdown into `/etc/wsl.conf`, keeping `systemd=true` under `[boot]`:

   ```ini
   [interop]
   enabled=false
   appendWindowsPath=false

   [automount]
   enabled=false
   ```

   It takes effect when the distro next starts. Desktop shortcuts can hold the pause command
   and the one that starts the distro again.
5. Sign Codex in as `conduit-run`. The owner does it; the coordinator signs nothing in. The
   second subscription accounts are not signed in here by hand: they are used only through
   Conduit's Interface login flow, as its test (ruling 72's later answers).
6. Before the end-to-end run, in the tailnet's admin console: rename the Mac, then turn on HTTPS
   certificates, in that order, because the store fixes the engine's https address on first use.

## What to hand the coordinator

The machine's tailnet name or address and its SSH host key, which the provider pins, and the
result of each step's check. They stay outside the repository. Never a password, a sign-in link
or a device code.

## How S3 cuts the network

S3's pre-registration ([experiment 014](../../experiments/014-s3-two-machines/README.md)) makes a
partition with a firewall rule, set from Linux, that drops traffic to and from the Mac for a
time, and a lost machine with `wsl --shutdown`; a coordinator script undoes both, and nothing
outside the two machines is touched. Under ruling 77 the coordinator can do neither: with interop
off nothing in the distro can run `wsl --shutdown` or set the Windows firewall, and `conduit-run`
has no sudo for a Linux-side rule. So S3's cut, heal and loss need the owner's hand or another
means. This is open for the owner as S3-OF, and S3's runs wait for it
([S3's plan](../../experiments/014-s3-two-machines/plan.md#for-the-owner)):

- **(A), recommended:** at each cut, heal and loss, the run prints one Windows command (a firewall
  rule blocking the Mac's tailnet address, its removal, or `wsl --shutdown`) and continues when
  the owner confirms it ran. It installs and grants nothing.
- **(B):** a Windows-side mechanism set up once, such as an OpenSSH account whose key may run only
  those commands, and only from the Mac's address.
- **(C):** a helper user inside the distro with a sudo rule for one script, which adds back a
  privilege ruling 77 kept out; a power-off from inside may differ from `wsl --shutdown`.
