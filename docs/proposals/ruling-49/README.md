# Ruling 49: signing in through the engine

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 49). This is the record of a
ruling. Where it and the specification differ, the specification governs.

The owner asked how logins would work when Agents need their own accounts: "when we have to log
in how will it work? We will need a browser login no? I'm worried about piping up prompts to an
interface and how they might be fulfilled for a variety of services and how they may be kept
independent of each other (signing into multiple different Google accounts etc)." The
coordinator researched the clients' login flows and the Model Context Protocol's URL
elicitation and wrote [a proposal](../sign-in/README.md) with three points. The owner's answer:
"1. Agreed 2. Yes for v0, but we should be more granular in the future, and allow for others to
authenticate. Fleshing out interfaces more will likely be a lot of the work for v1. 3. Yes,
check it".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | Whether sign-in prompts become part of the language | **Yes.** The engine passes a URL elicitation, and no other kind, to a person's Interface, never to an Agent. A client's own login reaches the person in the same shape. IFC-007 and IFC-016, which today pass none on, change for it |
| 2 | Which Interface receives them | **The root Interface, for v0.** Later versions are to be more granular and let others than the root sign in; the owner expects fleshing out Interfaces to be much of v1's work |
| 3 | Whether the account expected is checked | **Checked.** The configuration names the account an Agent's home or a Resource is for; after a sign-in the engine asks who signed in, and refuses and undoes a sign-in to any other |

## What follows

- The specification gains the pending sign-in, its shape (a URL elicitation with the instance it
  is for and the account expected), its delivery to the root Interface alone, and the check of
  the account, with vectors. A login that lapses becomes a sign-in pending, and an Agent's turns
  are refused with a reason that names it.
- Each Claude Code and Codex Agent has a home of its own, made and removed with it, with Codex
  storing its login in a file there and the engine reading none of it. This carries out ruling
  48's point 2, b, and settles finding 3-07.
- Codex signs in by its device flow; Claude Code by its own browser callback on the engine's
  machine, never by a pasted code.
- A5 runs on it with the owner's second accounts, including a sign-in to the wrong account,
  which must be refused.
- Interfaces that other people use, and sign-ins they complete, are recorded as v1 work.
