# K1: URL-mode elicitation at MCP revision 2026-07-28, checked

Not binding. The coordinator's task K1 of [the batch 1 plan](README.md), done on 2026-11-11 by a
research agent that read the published specification and the schema at the `2026-07-28` tag. Order
038a applies it; where it and the plan differ, this check governs, because the plan's drafts assumed
the earlier revision.

Sources: [elicitation](https://modelcontextprotocol.io/specification/2026-07-28/client/elicitation)
(E), [multi round-trip requests](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)
(M), [changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog) (C),
[basic](https://modelcontextprotocol.io/specification/2026-07-28/basic) (B),
[schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts)
(S), [tasks extension](https://github.com/modelcontextprotocol/ext-tasks/blob/v0.2.3/specification/2026-07-28/tasks.md)
(T), [resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources) and
[tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools).

## What differs from the plan

1. **No `elicitationId`, no completion notification.** URL mode is `elicitation/create` with `mode:
   "url"`, `message` and `url` only, carried in an `InputRequiredResult`'s `inputRequests`. C,
   minor change 11: "Remove the `notifications/elicitation/complete` notification and the
   `elicitationId` field … the client learns the outcome of an out-of-band interaction by retrying
   the original request… Servers needing to correlate an elicitation across retries encode their
   own identifier in `requestState`." Error `-32042` is reserved and gone. An `accept` "does not
   mean that the interaction is complete" (E): the first retry usually comes before the person has
   signed in, so the engine treats each retry as a poll, blocking or answering another
   `InputRequiredResult` (M allows several) until the account shows up or the DL18 wait ends, and
   never reads "no account yet" as failed. IFC-049 drops `elicitationId`; the engine's correlation
   goes in its own protected `requestState`.
2. **The capability.** A client declares `"elicitation": {"form": {}, "url": {}}` in
   `_meta.io.modelcontextprotocol/clientCapabilities` on each request; an empty object means form
   only, so URL mode is `{"url": {}}`. A server "MUST NOT send elicitation requests with modes that
   are not supported by the client" (E). A request that needs a capability the client did not
   declare is answered with `MissingRequiredClientCapabilityError` (`-32021`), listing it in
   `data.requiredCapabilities` (B). PND-011's "reports invalid" becomes that error.
3. **`requestState`** is core (M): the client echoes it exactly and never inspects it; the server
   "MUST treat `requestState` as an attacker-controlled input… MUST protect its integrity (e.g. HMAC
   or AEAD)" and "SHOULD include… the authenticated principal… a short expiry… an identifier for
   the originating request"; it "MUST NOT assume that clients will fulfill the `inputRequests` or
   retry". `InputRequiredResult` is allowed only on `tools/call`, `resources/read` and
   `prompts/get`.
4. **Tasks differ.** In the tasks extension a task's `inputRequests` are surfaced by `tasks/get` and
   fulfilled by `tasks/update`, not by retrying; there is no `requestState` (T). IFC-049's "a task's
   inputRequests are handled the same way" is reworded.
5. **Nothing keeps a message or URL from a model.** No text requires it; `message` is "to present to
   the user", and the tasks extension expects requests to go "to the user or model". The one rule is
   that the client opens the URL so that neither it nor the model can inspect the page (E).
   Annotations such as `audience: ["user"]` are hints (resources page). So device user codes and
   fingerprints go only to the engine's own panel, never in an elicitation message or a tool result
   (the plan's rule C5 holds; the area draft's evidence in `structuredContent` does not).

## Security requirements confirmed (E)

The server must not put credentials or personal data in the URL, must not send a pre-authenticated
URL, should use HTTPS (the plan's rule, HTTPS only, is stricter), and must bind an elicitation to
the client and user identity. The client must not pre-fetch or open the URL without consent, must
show the full URL, and should highlight the domain and offer retry and cancel. On phishing, the
server "MUST ensure that the user who started the elicitation request … is the same user who
completes the authorization flow". The engine relays an Agent client's own login URL, so it cannot
prove that directly; the expected-account check, with the undo of a sign-in to any other account
(ruling 49), is how batch 1 meets it. A declined or cancelled answer arrives only if the client
retries, so the DL18 wait is the outcome the engine can count on.

Not checked: the type names in rmcp 3.9.0.
