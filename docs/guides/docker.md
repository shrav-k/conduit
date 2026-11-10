# Docker

For S10, where Docker is the second runtime: the engine runs a server in a container rather than
as a process of its own.

1. **On the Mac**, install Docker Desktop (free for personal use) and start it once, so that
   `docker version` answers in a terminal. OrbStack and Colima also provide `docker`; any of
   them will do, and the coordinator records which.
2. **On the second machine**, once it exists, install Docker Engine inside Ubuntu, not Docker
   Desktop on Windows, so that the engine on Linux talks to it directly:

   ```sh
   curl -fsSL https://get.docker.com | sh
   sudo usermod -aG docker "$USER"
   ```

   then log out of Ubuntu and back in.
3. **Tell the coordinator** which you installed and its version.

The engine's Docker provider does not exist yet: it comes with the second providers, step 4 of
the [roadmap](../../ROADMAP.md#the-order-of-the-work).
