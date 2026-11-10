# Signing in: how an engine gets a person through a login it cannot do itself

Not binding. The [specification](../../../spec/README.md) alone binds. Research and a proposal by
the coordinator, 2026-11-10. The owner ruled on its three points the same day, in
[ruling 49](../ruling-49/README.md). It answers the owner's question about
adding accounts through Conduit's own flow ([the owner's direction](../owner-direction/2026-11-10.md),
A5): when a login needs a browser, how does the prompt reach a person, how is it completed, and
how are many accounts, such as several Google accounts, kept apart?

## Three kinds of sign-in

| Kind | Example | Who holds the credential | When a person is needed |
| --- | --- | --- | --- |
| 1. A client's own login | Claude Code or Codex as an Agent, on a subscription | The client, in its own home | Once per Agent, and again when the login lapses |
| 2. A service's login held for Agents | S2: a Linear key, kept in 1Password | A secret store, behind a provider | Once, in the store's own interface; never at an Agent's turn |
| 3. Authorization by a third party's OAuth | A Google account that an MCP server reaches for an Agent | The server or a secret store, bound to that account | When the server first needs it, and when the grant lapses |

Only the first and third reach a person while the engine runs. The second is set up in the
store's own application and referenced by the configuration, as the
[secrets promises](../../../spec/promises/secrets.yaml) already have it.

## What the clients offer, read on 2026-11-10

**Claude Code** ([authentication](https://code.claude.com/docs/en/authentication)):

- `claude auth login` opens the browser; if the browser cannot hand the result back, it shows a
  code that is pasted at `Paste code here if prompted`.
- One home per account: `CLAUDE_CONFIG_DIR` gives a directory its own settings, history and
  login, and on macOS "keys the macOS Keychain entry to that directory too, so a session with a
  different `CLAUDE_CONFIG_DIR` reads a different entry."
- `claude auth status` reports the login in use; `/status` shows its organization and email.
- `claude setup-token` prints a one-year token for `CLAUDE_CODE_OAUTH_TOKEN`; the engine would
  then hold a credential, which its terms forbid it to intermediate
  ([experiment 010](../../../experiments/010-subscription-terms/results-2026-11-10.md)). Not
  used.

**Codex** ([authentication](https://learn.chatgpt.com/docs/auth)):

- `codex login` opens the browser and waits on a callback at `localhost:1455`.
- `codex login --device-auth` (beta, enabled in the account's ChatGPT security settings) prints a
  URL and a code for any browser on any device: nothing comes back through the terminal.
- One home per account: `CODEX_HOME`. The credential is kept by `cli_auth_credentials_store`:
  `file` (`auth.json` in the home), `keyring`, `auto` or `ephemeral`. That the keyring entry is
  keyed per home is reported by third parties only; `file` avoids the question.
- `codex login status` reports the method in use. One `auth.json` per runner; it is never copied.

## What the protocol offers: URL elicitation

The Model Context Protocol's `2025-11-25` version added
[URL mode elicitation](https://modelcontextprotocol.io/specification/2025-11-25/client/elicitation):
a server sends a `message`, a `url` and an `elicitationId`, a client shows the URL to the user and
opens it only with consent, and the server may send `notifications/elicitation/complete` when
the user is done. Its rules fit this problem closely:

- secrets and credentials go only by URL, never through the client ("Servers **MUST** use URL
  mode for interactions involving such sensitive information");
- the client "**MUST** show the full URL to the user for examination before consent" and must
  open it so that neither the client nor a model can see what the user enters;
- the server must bind the elicitation to the user who started it, so that someone else cannot
  complete it in their place (its phishing section);
- the third party's tokens stay with the server: they never reach the client.

The specification today forbids the engine to answer an implementation's elicitation or pass it
to a consumer (IFC-016, and IFC-007 for `plain` servers). So kind 3 has no path now.

## Proposal

1. **One shape for every sign-in prompt: a URL elicitation, addressed to a person.** Whether the
   prompt comes from a client's own login (kind 1), which the Agent's package turns into one, or
   from a provider's server (kind 3), the engine records it as a pending sign-in with its
   message, URL, the instance it is for and the account expected, and shows it to the person
   through an Interface, never to an Agent: a model never sees a sign-in URL or what follows
   it. `conduit` lists pending sign-ins and opens one only when the person asks.
2. **The device flow where a client has one, and the browser on the engine's machine where it
   does not.** Codex's `--device-auth` sends nothing back through the engine. Claude Code's
   login completes through its own callback when the browser runs on the same machine; the
   pasted code, its fallback, is an authorization code and would pass through the engine, so it
   is not used: a remote person forwards the callback, as each vendor documents for SSH.
3. **A home per Agent, a store per account.** Each Claude Code or Codex Agent gets its own
   `CLAUDE_CONFIG_DIR` or `CODEX_HOME` in the engine's store, made when the Agent is, removed
   when it is, with Codex set to `file` storage. The engine reads none of it. This also closes
   finding 3-07. A service's accounts are separate secret Resources, one per account.
4. **The expected account is declared, and checked.** The configuration names which account a
   home or a Resource is for, such as an email. After a sign-in the engine asks the client
   (`claude auth status`, `codex login status`) or the provider who signed in, and a sign-in to
   any other account is refused and undone. This, not the browser, is what keeps several Google
   accounts apart: a browser signed in to two of them may complete the flow as the wrong one,
   and the check catches it. Where the third party supports them, the request carries a hint of
   the account (`login_hint`, `prompt=select_account` for Google), and the engine may open the
   URL in a browser profile kept for that account.
5. **A login that lapses is a sign-in pending, not a failure.** An Agent whose login expired
   reports it, its turns are refused with a reason that names the pending sign-in, and the
   person completes it as at first.

## What the owner decides

1. Whether sign-in prompts become part of the language: the engine passes URL elicitations, and
   only those, to a person's Interface, which changes IFC-016 and IFC-007's rule that none is
   passed on. Without it, kind 3 has no path and kind 1 is a command of the engine's alone.
2. Which Interface receives them: the root Interface only (recommended for v0), or the Interface
   that asked for the setup of the instance concerned.
3. Whether the expected account is a field of the configuration that the engine checks
   (recommended), or only recorded.

## What would be tested

A5 runs on it: the owner's second Claude and Codex accounts signed in through the engine's flow,
each to its own Agent, beside the first, for 48 hours, with each sign-in's prompt, the account
check and each refresh recorded. A wrong-account sign-in is attempted on purpose and must be
refused.
