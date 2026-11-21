# Accounts and sign-in

For A5 and for Agents that each have a home of their own ([ruling 49](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-49/README.md)).
Nothing here is done in a terminal the coordinator runs: you do each step yourself.

With Agent packages 0.2.0, each `.v2` Agent keeps its client's login in its own home inside
its state directory. Your everyday Claude Code and Codex logins stay as they are.

## Before the engine can sign an Agent in

Know which email each account uses. The configuration names the account each Agent is for,
and the engine accepts only that account ([ruling 49](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-49/README.md), point 3).
Tell the coordinator the two emails for Claude and the two for ChatGPT, or a label for each if
you would rather keep the emails to yourself.

## When the engine asks you to sign in

Run `conduit pending` to see which Agents are waiting for sign-in. After a restart it shows
any Agent whose account check failed at setup. After each command except `conduit pending`,
while any remain, the engine prints how many pending actions at root are still unanswered.
During the first sign-ins, read that number as the sign-ins you still have to do, not as an
error.

Use `conduit sign-in` or root's Interface only to answer a pending sign-in action.
For a Codex Agent, a failed check proceeds to the sign-in
hook, and Codex's login first signs that home out. A Codex Agent left idle until its access
token lapses may fail its account check and ask you to sign in again. The check blocks token
refresh, though a turn would have refreshed it ([ruling 86](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-86/README.md), point 5).

In 0.2.0, sign in with the client's default login. The engine refuses a login name. The Codex
device code and Claude Code's code-page login wait for later releases.

- **Read the URL before you open it.** It should be on the vendor's own domain: `claude.ai` or
  `claude.com` for Claude, `openai.com` or `chatgpt.com` for Codex.
- **Sign in as the account named.** If your browser is signed in to the other account, use a
  private window or another browser profile. A sign-in as the wrong account is refused and
  undone, and the engine asks again. The account check alone decides who signed in (PND-011).
- **Codex** may open the default browser of the engine user's graphical session on the
  engine's own runner. Close that window, keep that browser free of an OpenAI session and
  open the sign-in link in your own browser. If that browser is on another machine, the
  sign-in completes only through your own SSH forward of the login's loopback port on the
  engine's machine; Conduit carries no return in 0.2.0.
- **Claude Code** completes sign-in only if your browser reaches the login's loopback port
  on the engine's machine. That limit holds until order 050o's managed connection is built.

## When an Agent retires

Each client's logout at retirement revokes that home's token at the vendor, as the
[logout read](../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#the-outcome-under-item-8)
records. If the machine is lost and logout cannot reach it, the login stays until you revoke
it at the vendor. The engine dispatches a failed logout again only after reconciliation
records the outcome as `failed`.

## A5, the test

In A5, your second accounts each sign in to an Agent of their own beside the first for
48 hours. The coordinator records each sign-in, each refresh and the deliberate wrong-account
attempt.
