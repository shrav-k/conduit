# Accounts and sign-in

For A5 and for Agents that each have a home of their own ([ruling 49](../proposals/ruling-49/README.md)).
Nothing here is done in a terminal the coordinator runs: you do each step yourself.

State on 2026-11-10: ruling 49's separate homes, sign-in prompts and account checks are
pending specification and implementation work. The first build's
[closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md) used shared
logins. The engine behavior described below is the ruled flow to come, not a flow available
in the current shell.

## Before the engine can sign an Agent in

1. **Choose how to sign Codex in.** Its browser login completes by itself on one machine.
   From another, its return comes back through `conduit` or an SSH forward. A device code
   stays an option where your account has it on. Claude Code needs a browser on its Agent's
   own machine.
2. **Know which email each account uses.** The configuration names the account each Agent is
   for, and the engine refuses a sign-in to any other ([ruling 49](../proposals/ruling-49/README.md),
   point 3). Tell the coordinator the two emails for Claude and the two for ChatGPT, or a label
   for each if you would rather not.
3. **Leave your own logins as they are.** Your everyday `claude` and `codex` keep `~/.claude`
   and `~/.codex`. Each Agent gets its own directory in the engine's store; signing an Agent in
   never signs you out.

## When the engine asks you to sign in

Once the work for ruling 49 is merged, `conduit` lists the sign-ins waiting for you. For each it
shows which Agent it is for, the account expected and the full URL.

- **Read the URL before you open it.** It should be on the vendor's own domain: `claude.ai` or
  `claude.com` for Claude, `openai.com` or `chatgpt.com` for Codex.
- **Sign in as the account named.** If your browser is signed in to the other account, use a
  private window or another browser profile. A sign-in as the wrong account is refused and
  undone, and the engine asks again.
- **Codex** uses its browser login: on one machine it completes by itself; from another,
  bring its return back through `conduit` or an SSH forward. You can choose a device code
  where your account has it on, and enter that code at the page shown. **Claude Code** signs
  in with a browser on the Agent's own machine; Conduit carries no Claude return.

## A5, the test

The coordinator runs it once the flow exists: your second accounts each signed in to an Agent of
their own, beside the first, for 48 hours, recording each sign-in, each refresh and the
deliberate wrong-account attempt.
