# Ruling 73: browser logins piped through to the person's Interface

Status: in force; amended by 75 (point 3; the routes across machines) (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach) (spec pending, with the sign-in orders of batch 1). This record says why.

[Ruling 49](../ruling-49/README.md) made sign-in part of the language. Every sign-in prompt reaches
a person's Interface as a URL elicitation, never an Agent, and the account signed in to is checked.
Its [proposal](../sign-in/README.md) left one case to the person. Many browser logins redirect back
to the machine the program runs on, such as Codex's callback on `localhost:1455` or Claude Code's
own. When the person's browser is elsewhere, the proposal had the person forward the callback by
hand, "as each vendor documents for SSH". It also set aside Claude Code's pasted code, because that
code would pass through the engine.

After the coordinator explained that Claude Code signs in through a browser page on the engine's
machine, the owner: "We should be able to pipe the browser login through to the interface! Many
resources may require browser login, let's discuss this further".

The coordinator described three ways a browser login finishes:

- the service handles its own return, as a hosted service or a remote MCP server does with URL
  elicitation;
- a device code;
- a redirect back to the program's own machine, which needs a way back.

It recommended three points:

1. The Interface carries the redirect back. It listens on the same port on the person's machine,
   opens the URL in the person's browser, catches the one redirect, passes it through the engine
   to the waiting program, and stops listening. VS Code Remote and GitHub Codespaces work this way.
   This replaces forwarding by hand.
2. Where nothing can listen, as on a phone, the person pastes the final URL into the Interface as
   a fallback. This reverses "never a pasted code", whose reason no longer holds once the redirect
   passes through the engine anyway.
3. Both apply in v0, to Agents' clients and to Resources, still delivered only to the root
   Interface, as ruling 49 point 2 decided.

The owner: "I accept your recommendations 1-3".

## The decisions

1. **The Interface carries a browser login's redirect back.** A pending sign-in whose URL
   redirects to the machine its program runs on completes through the person's Interface. The
   Interface:
   - listens on that port on the person's machine;
   - opens the URL in the person's browser;
   - passes the one redirect through the engine to the program that waits for it;
   - stops listening.

   The code it carries is never recorded and never reaches a model. The check of the account
   follows, as ruling 49 decided.
2. **A pasted URL is the fallback.** Where the Interface cannot listen, the person pastes the final
   URL from the browser into the Interface, and it takes the same path. A device code is used
   where the service offers one.
3. **In v0, for Agents and for Resources.** Any implementation that needs a browser login uses
   this path: an Agent's client, a Resource's server, or the engine logging in to a Resource. The
   resulting credential stays with whatever completed the login, bound to the account the
   configuration names. Sign-ins still go to the root Interface alone in v0.

The research the coordinator started on the clients' flows, the standards and the risks shapes how
the specification writes this. The risks include a malicious Resource's URL, a hijacked port and a
login into the wrong account. A finding that would change one of these decisions goes back to the
owner.

## What it amends

Ruling 49's "What follows": Claude Code no longer signs in only by a browser on the engine's
machine, and a pasted URL is allowed as the fallback. A remote person no longer forwards the
callback by hand.
