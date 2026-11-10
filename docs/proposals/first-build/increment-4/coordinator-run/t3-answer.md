# T3: what each Agent can reach (the author's answer)

Through the engine (from `conduit tools --as agent-a` and `--as agent-b`, which list exactly
the operations of each Agent's open Conduits, IFC-012):

- agent-a (Claude Code): the shared directory, read and list (`project`, from `read`); the
  history, read, list and write (`history-a`).
- agent-b (Codex): the shared directory, read, list and write (`project`, from `read-write`);
  the history, read, list and write (`history-b`).

By its own means, around the engine (from the two Agent packages' READMEs and increment 3's
closeout):

- Both: no shell, code execution that reaches the machine, browser, sub-agents, hooks or
  plugins (off by default, ruling 48 and findings 3-04/3-05); no account services or
  messaging (`accountServices` and `peerMessaging` false). Both run as the owner's OS user,
  so any tool that reads files would reach the store and each other's identity.
- agent-a: Claude Code with `--tools ""`: no built-in tools.
- agent-b: Codex keeps `apply_patch` in a read-only sandbox, a request for user input, and its
  code-mode host, whose execution imports nothing that reaches the machine; it reads the
  owner's `~/.codex/AGENTS.md` (finding 3-07, a stated leak).
- Both use the owner's shared subscription login.

Inspection effort: 2 commands for the engine's reach (`tools --as` twice). `management.inspect.v1`
on each Agent (2 more commands, about 37 KB each) shows the declared `project` Conduit but not
the standalone `history-a`/`history-b`, whose consumer is the Agent: inspecting an Agent does
not list the Conduits it consumes. Reach around the engine: 2 package READMEs and the closeout's
routes section read.
