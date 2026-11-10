# A second machine

For S3, where S1 and S2 run across two machines with a partition and a lost machine, and A74,
its probe without an engine. The second machine is the Windows computer, running Linux under
WSL2: the engine stops the programs it runs by process group, which Linux has and Windows does
not. A native Windows build can come later.

This is a plan, written before any of it has been tried on this computer. Tell the coordinator
where a step differs.

## On the Windows computer

1. **Install WSL2 with Ubuntu.** In PowerShell as administrator: `wsl --install -d Ubuntu`,
   then restart and make a Linux user, such as `conduit`, used only for this work.
2. **Give WSL the computer's own network address.** In `%UserProfile%\.wslconfig` add:

   ```ini
   [wsl2]
   networkingMode=mirrored
   ```

   then run `wsl --shutdown` and start Ubuntu again. Without it, WSL sits behind a private
   network that the Mac cannot reach directly.
3. **Join both machines to one private network.** Tailscale is the simplest: install it on the
   Windows computer and on the Mac, sign both in to the same account, and note the Windows
   computer's Tailscale name. A home network works too, if both stay on it.
4. **In Ubuntu, install what the engine needs**:

   ```sh
   sudo apt update && sudo apt install -y build-essential git openssh-server curl
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   and Node 21.5.0 or later, through `nvm`, as on the Mac.
5. **Let the coordinator in by SSH key, not password.** On the Mac, the coordinator gives you a
   public key made for this; add it to `~/.ssh/authorized_keys` of the Linux user, and start
   SSH with `sudo service ssh start`. Turn off password logins in `/etc/ssh/sshd_config`
   (`PasswordAuthentication no`).

## What to hand the coordinator

The machine's Tailscale name, or address, and the Linux user's name. No password.

## How S3 cuts the network

A partition is made on purpose, from the Linux side, with a firewall rule that drops traffic to
and from the Mac for a time, and a lost machine by stopping WSL with `wsl --shutdown`. Both are
undone by the coordinator's script. Nothing outside the two machines is touched.
