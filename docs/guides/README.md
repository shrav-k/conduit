# Setup guides for the owner

Not binding. The [specification](../../spec/README.md) alone binds. These guides say what the
owner sets up by hand for the experiments that v0 needs, and how collaborators read the text
cold. The setup guides say what to hand the coordinator afterwards. Each step that creates a
credential says where the credential goes. **Never paste a
password, token or key into a chat with an agent**: put it where the guide says, and tell the
agent only that it is there.

| Guide | For | Needed by |
| --- | --- | --- |
| [Accounts and sign-in](accounts.md) | The second Claude and Codex accounts; Codex's device sign-in | A5, and every Agent with a home of its own (ruling 49) |
| [1Password and Linear](1password-and-linear.md) | A test vault, a service account limited to it, and a Linear workspace whose key the vault holds | S2, A73, S10 |
| [A second machine](second-machine.md) | Manual test-only setup of Windows with Linux under WSL2, and ruling 77's setup; managed and peer modes are ruled, and enrollment is in `spec/` but not yet built | S3, A74 |
| [Docker](docker.md) | The second runtime | S10 |
| [Cold readers](cold-readers.md) | The owner's collaborators: what to read, what is known to be open and how to report a finding | The Read row after step 3; instructions dated 2026-11-10 |
| [Full review](full-review.md) | The next coordinator: how to review the repository and the progress so far, where to look, and what to hand the owner | The handover of 2026-11-12 |
| [Moving to ConduitLang](move-to-conduitlang.md) | The GitHub organization, npm scope, domains and identifiers of the public name | Ruling 65; ruling 47's namespace |

Menu names and commands in the setup guides are as the coordinator understood them when each
was written, from 2026-11-10; where one has moved or fails, say so. The second-machine guide's
first setup ran on 2026-11-11, and part 1 of ruling 77's setup on 2026-11-12; no other guide's
steps are recorded as tried. The accounts guide
describes the sign-in of [ruling 49](../proposals/ruling-49/README.md), which is in `spec/`
(orders 038a and 038c) but not yet built; the first build still uses shared logins, as its
[closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md) records.
Rulings 50 to 52 accept enrollment's direction and managed and peer modes, and ruling 56, as
ruling 68 point 9 amends it, how a person checks a joining machine. Enrollment is in `spec/`
(order 038b); its core, order 049a1, is not yet merged, so the second-machine guide remains a
manual, test-only setup.
The Interface is to raise all pending actions for a person, sign-ins and enrollment alike,
as a control panel; this is accepted direction, not a flow the current shell supplies.

When a guide is done, tell the coordinator which guide and anything that differed from it.
