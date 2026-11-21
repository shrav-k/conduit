# Docker

For S10, where Docker is the second runtime: the engine runs a server in a container rather than
as a process of its own.

1. **On the Mac**, install Docker Desktop (free for personal use) and start it once, so that
   `docker version` answers in a terminal. OrbStack and Colima also provide `docker`; any of
   them will do, and the coordinator records which.
2. **On the second machine**, install Docker Engine inside Ubuntu-24.04, not Docker Desktop
   on Windows, so that its daemon runs in the distro the owner gave to Conduit. It runs rootless,
   as `conduit-run`, the one run user of ruling 77, point 2.

   No user joins the `docker` group: its members are root inside the distro, which ruling 77
   keeps from `conduit-run`.

   **The owner, with sudo, once**, in Ubuntu-24.04 as the owner's own user:

   ```sh
   curl -fsSL https://get.docker.com | sh
   sudo systemctl disable --now docker.service docker.socket
   sudo rm -f /var/run/docker.sock
   sudo apt-get install -y uidmap dbus-user-session
   grep '^conduit-run:' /etc/subuid /etc/subgid
   ```

   Under WSL the install script says that it detected WSL, recommends Docker Desktop and waits
   20 seconds; the owner lets it go on. It installs Docker's packages, the rootless extras among
   them. The next two commands stop the system-wide daemon, which runs as root, and remove its
   socket. The third installs the tools for the user-ID maps and the user's D-Bus session.
   The `grep` must print one line from each file, each giving `conduit-run` a range of at least
   65536 IDs (the line's third field); these ranges are the user-ID maps. Where a file has no
   such line, the owner adds a range that no other line of either file uses, with
   `sudo usermod --add-subuids FIRST-LAST --add-subgids FIRST-LAST conduit-run`, FIRST and LAST
   being the range's first and last ID.

   **The coordinator, as `conduit-run`, without sudo**, installs the rest, as step 3 of
   [ruling 77's setup](second-machine.md#the-setup) has it:

   ```sh
   dockerd-rootless-setuptool.sh install
   systemctl --user enable docker
   docker context use rootless
   docker info --format '{{.SecurityOptions}}'
   ```

   The setup tool creates and selects the CLI context `rootless`, and the third command makes
   sure of it. The last command's output includes `name=rootless`. A CLI context, not an
   environment variable, names the daemon, because `conduit-run`'s login shell is `/bin/sh`, so its
   command sessions read no startup file ([SSH-R's ruling-77 setup][ssh-r-setup]). Lingering, on for
   `conduit-run` since step 1 of [ruling 77's setup](second-machine.md#the-setup), keeps the daemon
   running with nobody logged in.

   These steps have not yet run on the Windows computer; the coordinator confirms them there at
   S3-O2. Whoever ran a step that fails stops and reports the step and its last line of output.
   They do not add any user to the `docker` group instead.

   Two failures are foreseen, and Docker's rootless mode covers each on
   [its troubleshooting page](https://docs.docker.com/engine/security/rootless/troubleshoot/):
   a D-Bus error from the setup tool or the daemon, and a refusal to create a user namespace.
   Ubuntu 24.04 restricts unprivileged user namespaces; that page says Docker's deb packages,
   which the install script installs, need no AppArmor change. A remedy there that needs sudo
   is the owner's.

   Sources: [Docker's rootless mode](https://docs.docker.com/engine/security/rootless/) and
   [the full review's D51][D51], a decision of the coordinator's, open to the owner's strike.

   [D51]: https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md#the-providers-and-the-second-machine
   [ssh-r-setup]: ../../packages/provider-ssh-runtime/README.md#the-owners-ruling-77-setup
3. **Tell the coordinator** which you installed and its version.

The engine's Docker provider does not exist yet: it comes with the second providers, step 4 of
the [roadmap](../../ROADMAP.md#the-order-of-the-work).
