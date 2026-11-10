# Setup guides for the owner

Not binding. The [specification](../../spec/README.md) alone binds. These guides say what the
owner sets up by hand for the experiments that v0 needs, and what to hand the coordinator
afterwards. Each step that creates a credential says where the credential goes. **Never paste a
password, token or key into a chat with an agent**: put it where the guide says, and tell the
agent only that it is there.

| Guide | For | Needed by |
| --- | --- | --- |
| [Accounts and sign-in](accounts.md) | The second Claude and Codex accounts; Codex's device sign-in | A5, and every Agent with a home of its own (ruling 49) |
| [1Password and Linear](1password-and-linear.md) | A test vault, a service account limited to it, and a Linear workspace whose key the vault holds | S2, A73, S10 |
| [A second machine](second-machine.md) | The Windows computer, with Linux under WSL2, joined to this Mac | S3, A74 |
| [Docker](docker.md) | The second runtime | S10 |

Menu names and commands are as the coordinator understood them on 2026-11-10, and none of these
steps has yet been tried on the owner's machines; where one has moved or fails, say so.

When a guide is done, tell the coordinator which guide and anything that differed from it.
