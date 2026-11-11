# Work order 038c (E7 spec): the return of a browser login in the specification

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; list in `vectors` the one case this order adds (step 9). This order
has network access, to fetch crates, for tests that bind a loopback port and to read the standards
it cites; use it for nothing else. Never print a whole large file or log: use `head`, `tail`,
`grep` or a count. Run one cargo command at a time. Run no client's login, and read no client's
home, token or credential.

## What and why

Ruling 73 decided that a browser login whose redirect goes back to the machine its program runs on
completes through the person's Interface, with a pasted final URL as the fallback, for Agents and
Resources in v0. Ruling 75, points C1 to C7, settled what the research behind it found. Neither is
written: [IN_FORCE.md](../../IN_FORCE.md) holds them as *spec pending* at lines 193 to 196. Order
038a wrote sign-in itself: the URL relayed to a person's Interface (IFC-049), the account polled
after it (PND-010, PND-011) and the three hooks (CTR-013). This order writes how a Type states its
logins, which URL the engine relays, and how a login's return comes back. It lands before order
048a, the sign-in core ([the batch 1 plan](../proposals/spec-batch-1/README.md), line 566), so
that 048a builds the stated logins once.

Most of it is off S1's rerun path. S1 runs on one machine: the browser opens beside the program,
whose own listener takes the redirect, and nothing comes back through Conduit. 048a needs the
stated logins, the client-ID rule, the match of the relayed URL, the message the engine writes,
the login the person asks for and one fixed-port login at a time. The return itself, PND-013
below, is built after 048b by an order of its own, before S3.

**A defect to fix.** Order 038a put `signIn` among the properties of `type-spec.v1` (commit
d08fb8b). Order 040 (commit b60112e) moved it, by mistake, into the `$defs` of `contract.v1`
(`spec/schemas/contracts.yaml` lines 107 to 112), where nothing refers to it. `type-spec.v1`
(lines 123 to 193) states `additionalProperties: false` and has no `signIn`, so every Type that
states signIn is rejected today. Step 1 puts it back, with its logins.

**Start after 038b.** Order 038b (enrollment) edits the same places as this order: the row
"prompt" of `spec/definitions.md`, PND-004, IFC-049, the counts in `spec/README.md` and its
Contents rows, `spec/core/README.md` lines 29, 32 and 42 to 44, and the exports in
`spec/core/conduit.yaml`. Two workers who recount apart leave wrong counts once both merge, and no
checker recounts them. So this order starts from a `main` that holds 038b: check that
`spec/promises/enrollment.yaml` exists and that PND-008 is written in `spec/promises/pending.yaml`.
If either is missing, stop and report. Keep 038b's text and add yours, and keep its staged
patches applying (step 10). Order 071 (the coverage index in pieces) may also land before this
one: it splits `conformance/manifest.yaml` and removes the vector count at `spec/README.md` lines
7 and 8. Where your tree has 071, keep its changes. Order 041b changes `conformance/` beside this
order. Where your tree has its cases, step 10's search covers them. If 041b merges after this
order, the admin reruns `cargo test --release` on the merged tree.

Line numbers in this order are those of `main` at 9729f21, where `spec/` is as it was at acc1b6e.
038b moves many of them: where a line has moved, find it by the words quoted.

Size: M. About 300 to 450 lines of specification, one vector case and two handoffs.

Read, in this order:

1. The rulings: [73](../proposals/ruling-73/README.md), its decisions (lines 38 to 61);
   [75](../proposals/ruling-75/README.md), the owner's words (lines 16 to 20) and C1 to C7
   (lines 50 to 75); [49](../proposals/ruling-49/README.md); and
   [66](../proposals/ruling-66/README.md), point 3 (line 28).
2. IN_FORCE.md, topic 9: the pending-actions line (184) and the sign-in lines (188 to 197).
3. What 038a and 038b wrote, on which this order builds:
   - `spec/promises/contracts.yaml`: CTR-004's last sentence (lines 124 and 125) and CTR-013
     (lines 281 to 299);
   - `spec/promises/pending.yaml`: PND-004 (line 63), PND-012 (line 114), PND-005 (line 129),
     PND-010 (line 150), PND-011 (line 176) and 038b's PND-008;
   - `spec/interface/implementation.yaml`: IFC-006 (line 121), IFC-007 (line 157), IFC-016's
     sentences on URL elicitation and task input (from line 425), IFC-045 (line 1240) and
     IFC-049 (line 1348);
   - DL18 (`spec/implementation-defined.yaml` line 241), SEC-020 (`spec/promises/secrets.yaml`
     line 483), PRV-029 (`spec/promises/providers.yaml` line 907), REC-002
     (`spec/conventions/records.yaml` line 36) and the head comment of `spec/core/types.yaml`
     (lines 18 to 20);
   - SUB-008 and SUB-009 (`spec/promises/submissions.yaml`) and PKG-029, on what a submission
     is and when a template or an instance holds one;
   - `spec/core/operations.yaml`: `register.v1` (line 106), `replace.v1` (line 176),
     `inspect.v1` (line 258), `install.v1` (line 573), `sign-in.v1` (line 1290) and the three
     hooks (lines 1323, 1333 and 1351);
   - `spec/schemas/contracts.yaml` lines 1 to 30 and 107 to 193, and `spec/schemas/README.md`
     line 44 and lines 287 to 293;
   - `spec/definitions.md`, "Pending actions and sign-in" (lines 260 to 276), and
     `spec/README.md`, "Pending actions, sign-in and the network" (lines 394 to 424).
4. [The K1 check](../proposals/spec-batch-1/k1-mcp-elicitation.md) of the protocol. Where it
   differs from this order, K1 governs: apply it and report the difference.
5. The standards: RFC 6749 §4.1.2 (the authorization response and its errors), RFC 7636 (PKCE and
   S256), RFC 8252 §7.3 and §8.3 (loopback redirects), RFC 8628 (the device grant, and §5.4 on
   remote phishing), RFC 9207 (`iss`), RFC 9700 §4.4 (mix-up) and §4.5 (code injection), and
   URL-mode elicitation in the Model Context Protocol's revision 2026-07-28.

The decisions below are rulings 73 and 75 and, where marked, the admin's design, drawn from the
coordinator's research on the clients' flows, the standards and the risks. Apply them; do not
reopen them. Write each item in the specification's form and terms
([CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises"; `spec/definitions.md`). In promise
text a Type *states* its logins: the row "declaration" of `spec/definitions.md` (line 411) keeps
that word for a node declared lost and a declared extension. Where a decision here and a ruling's
record differ, the record governs: write the record's meaning and report the difference.

## What to do

1. **The schema of a login** (75 C6). In `spec/schemas/contracts.yaml`:
   - Remove `signIn` from the `$defs` of `contract.v1` (lines 107 to 112). Put it among the
     properties of `type-spec.v1`, holding exactly `account`, a `pointer.v1`, and `logins`, a
     non-empty list of `sign-in-login.v1`, both required. Name `signIn` in `type-spec.v1`'s
     description. In the file's head comment, add CTR-013 to `type-spec.v1`'s entry in the list
     of the promises each schema encodes (lines 4 to 25), add an entry for `sign-in-login.v1`,
     and add CTR-013's cross-document checks (step 2) to what the engine checks (lines 40 to
     51).
   - Add the Schema contract `sign-in-login.v1`. Export it from `spec/core/conduit.yaml`, beside
     `type-spec.v1` (line 131), and give it a row wherever `spec/core/README.md` lists every
     export. A login holds:
     - `name`: a `name.v1`, unique among the Type's logins;
     - `return`, how the login's outcome reaches its program, one of four:
       - `device`: a device code (RFC 8628). The URL relayed is the service's verification
         page, the person types there the code that the engine's panel shows, and nothing comes
         back through Conduit;
       - `redirect`: the browser is sent to a loopback address on the program's machine;
       - `page`: a page of the service shows a code that the person brings back;
       - `service`: the browser is sent to an HTTPS address of the service, which completes the
         login with its program; nothing comes back through Conduit;
     - `authorization`: the HTTPS origin and path of the URL the engine relays, which is the
       verification page for `device` and the authorization endpoint for the others. It holds no
       query, fragment or user information;
     - `clientId`: the OAuth client ID, required for every login. For `device` it is the ID the
       program presents at the device authorization endpoint (RFC 8628 §3.1), which the relayed
       URL does not carry;
     - `redirect`, required for `redirect` only: `host` (`localhost`, `127.0.0.1` or `[::1]`),
       `ports` (a non-empty list of distinct integers from 1024 to 65535, or `any`) and `path`
       (beginning with `/`);
     - `page`, required for `page` and `service` only: the HTTPS URL of the service's page, which
       is the authorization request's `redirect_uri`;
     - `carried`, a boolean, required for `redirect` and `page` only: whether the engine may
       carry the login's return from the person's Interface to its program (step 5). `false`
       means the login completes only with a browser on its program's machine (75 C3);
     - `issuer`, optional for `redirect` only: the authorization server's issuer identifier, for
       the `iss` check of RFC 9207. A `page` login's return is an opaque code that carries no
       `iss`, so it states none.

     No other member is allowed, nor a member that the login's `return` does not take. Every
     value is literal, never a JSON Pointer into configuration: the logins are pinned with the
     package's digest, and configuration is not (75 C6). Write the schema with the profile's
     keywords (`spec/schema-profile.md`, "Schema keywords in the profile").
   - *The admin's design*: the four kinds, `clientId` on every login, and `issuer` for `redirect`
     only. A single kind for "nothing comes back", covering both a device code and a service's
     own HTTPS return, would let a relayed URL carry a `client_id` and a `redirect_uri` that no
     login states and nothing checks. 75 C6 has each login state its client ID and its redirect
     or hosted page. Report them as applied.
   - Rewrite `spec/schemas/README.md` lines 287 to 293 to match: what the schema checks, and what
     CTR-013's cross-document checks add (step 2). Its heading, "Sign-in declarations", uses a
     word that `spec/definitions.md` keeps for other senses: name it for the logins a Type
     states. No link points to its anchor. Add `sign-in-login.v1` to the row of `contracts.yaml`
     in its table of files (line 44).

2. **CTR-013** (75 C1, C3, C6, C7). Rewrite it, and CTR-004's last sentence, so that:
   - **The logins.** signIn holds exactly `account` and `logins`. Beside the three hooks and the
     account's place that CTR-013 already requires, a Type is rejected at install or submission
     where two of its logins have one `name`, or where one URL could pass step 3's checks for two
     of them, as a `redirect` with `ports: any` and another on the same host and path would. A
     relayed URL then matches at most one login.
   - **Pinned and shown** (C6). The logins are part of the Type contract, so the package's digest
     covers them (PKG-004), and the digest of a submitted project's contracts covers its own
     (SUB-008). `install.v1` gains an optional output `logins`: for each Type of the package that
     states signIn, its qualified reference and its logins. `inspect.v1` gains an optional output
     `logins`: the logins of the object's Type, present where that Type states signIn. Write both
     in `spec/core/operations.yaml`. Step 10 says what to do if the engine's tests reject them.
   - **A client ID two Types claim** (C6). The engine compares the client IDs that a Type's
     logins state with those of every other Type it holds at another address: the Types of every
     installed package, and those of every submission that a template or an instance holds
     (SUB-008, PKG-029). Types at one address share client IDs freely: the Types of one package,
     of each of its versions, and of the submissions at one address.
     - Installing a package reports conflict where a login of one of its Types states a client
       ID that such a Type also states, unless the request confirms it. The confirmation is a new
       optional input of `install.v1`, `confirmClientIds`, the list of the client IDs shared.
       Only the root Interface gives it: a request from any other caller that gives it reports
       refused. A confirmation of a client ID that the package shares with no such Type reports
       invalid.
     - Registering a project, or replacing a template with one (SUB-009), reports conflict where
       a login of one of the project's Types states such a client ID. Neither has a
       confirmation.
     - The diagnostic names the client ID and both Types.
     - The record of an `install.v1` request that confirms holds the client IDs it confirmed in
       `data.inputs.confirmClientIds` of `record.v1`, and no other input, although the records
       about `management` keep no inputs (CTR-004, REC-002; `spec/core/types.yaml` lines 18 to
       20). Write this exception in REC-002's observable, cite it from CTR-013, and name it in
       that head comment of `types.yaml`.
     - *The admin's readings*: the ruling names installed Types, and a submitted project's Type
       would otherwise claim another's client ID unchecked, or claim it first so that the
       package installs later with no conflict; "another address" exempts sibling Types in one
       package and the submissions at one address, as well as a package's versions. Report both
       as applied.
   - **The account check stays strict** (C7). A Type states signIn only where its
     `account-hook.v1` reports the account that the service itself attests: one the service
     returned at login, such as in an ID token or a profile, or one it reports when asked, such as
     by OpenID Connect's userinfo or the service's own call. An account that the configuration or
     the person supplies does not count. A Type that cannot learn it states no signIn, so it gets
     no sign-in, and no URL is relayed for it (IFC-016). There is no sign-in without the account
     check: a null account disables both, as CTR-013 already says.
   - **A login is its implementation's** (C1). The engine starts no login and holds no token that
     one yields (SEC-020). A Resource whose service needs a browser login is implemented by a
     package that completes the login and keeps the token in its own state directory (PRV-029).
     C1 amends 73 point 3's "the engine logging in to a Resource".
   - **Not carried** (C3). The engine carries no return for a login stated `carried: false`
     (step 5). In v0 the Claude Code Agent Types state theirs so, because Anthropic's terms forbid
     intermediating Claude.ai credentials. That statement is in the package, order 050's (step
     10).

3. **IFC-049: what the engine relays** (73; 75 C6). Amend it:
   - The URL that `sign-in-hook.v1` gives matches exactly one login that the instance's Type
     states, and the one the person asked for where they asked (step 4). Its origin and path
     equal `authorization`, it holds no user information and no fragment, and no parameter
     appears twice. Also:
     - for `device`, the URL is `authorization` itself, with no query, so it carries no code
       (PND-004) and no client;
     - for `redirect`, `page` and `service`, `client_id` equals `clientId`, `response_type` is
       `code`, and `redirect_uri` is, for `redirect`, `http://`, the stated host, one of the
       stated ports (or any port from 1024 to 65535 for `any`) and the stated path, with no
       query, and for `page` and `service`, the stated `page`;
     - for `redirect` and `page`, also `code_challenge_method` is `S256`, `code_challenge` is
       43 characters of the base64url alphabet (RFC 7636), and `state` is at least 22
       characters, each in RFC 3986's unreserved set.
   - A URL that fails a check is not relayed, and sign-in.v1 ends failed, with a diagnostic that
     names the check and the field and holds no value of the URL. The engine never opens, fetches
     or follows a relayed URL.
   - **The engine writes the message.** It names the instance, its Type and package, its node,
     the URL's host, the expected account, and what the person brings back, if anything; for
     `device`, that the code is in the engine's panel. For a login stated `carried: false` it
     says to open the URL in a browser on that node's machine. No text from the hook is relayed.
     This replaces "The message names the instance, expected account where applicable and what
     follows" (lines 1360 and 1361).
   - **Two `_meta` keys.** Write both in IFC-049: `tools/set_names.py` reads Conduit's keys from
     `spec/interface/implementation.yaml` (its function `meta_keys`, lines 177 to 185), and its
     check reports any key under the prefix that this file does not hold.
     - `io.github.shrav-k.conduit/signInLogin`. In the `_meta` of an Interface's sign-in.v1
       request, optional: `{name}`, the login the person asks for (step 4). In the `_meta` of
       each InputRequiredResult of sign-in.v1, and of its final result: `{name, return, node,
       acceptsReturn}` for the login whose URL is relayed. `node` is the instance's node, named
       as `object.v1` names an object. `acceptsReturn` is true exactly where step 5's first check
       passes. After a retry that carried a return, it also holds `returned` and, where step 5
       gives them, `reason` or `error`. It goes in the result's own `_meta`, not in its input
       request, which holds only `mode`, `message` and `url` (K1).
     - `io.github.shrav-k.conduit/signInReturn`: `{value, via}` in the `_meta` of an Interface's
       retry of sign-in.v1, `via` being `listener` or `paste`; and `{value}` in the engine's
       retry of `sign-in-hook.v1` (IFC-006, step 5). It is never in a result, a notification or a
       record.
   - Accept still means consent, not completion, and the rest of IFC-049 stands.
   - Amend IFC-006: a call of `sign-in-hook.v1` also holds `signInLogin`, `{name}`, where the
     person asked for a login, and on step 5's retry `signInReturn`, `{value}`.
   - *The admin's design*: the names of the keys and their members, `listener` and `paste`, the
     22 characters of `state`, the checks for each kind, and the rule that a URL matches exactly
     one login. Report them as applied.

4. **PND-010** (75 C2, C5; 66 point 3). Amend it:
   - **The login asked for.** A request whose `signInLogin` names no login the Type states
     reports invalid before anything is dispatched, with a diagnostic naming the logins the Type
     states. The engine passes the name to `sign-in-hook.v1`, which starts that login or fails
     with a diagnostic saying why. Without it, the hook chooses among the Type's logins. A hook
     marked `plain` gets no Conduit keys (IFC-007), so it gets no name, and a URL it gives for
     another login fails IFC-049's match. This is how a person asks for a device code, which
     stays an option where the account has it on (C5).
   - **One fixed-port login at a time.** Before dispatching any part, the request reports
     unavailable where a sign-in.v1 invocation in progress may start a login that states a fixed
     port which a login this request may start also states. The diagnostic names the port. It
     names the pending action that the other invocation answers only where the caller may see
     that action (PND-002). A request may start the login it asks for, or, where it asks for
     none, any of its Type's logins. *The admin's reading*: a second client that listens on the
     same fixed port cancels the first, as Codex's login does on port 1455. Report it as applied.
   - **The return** goes to the Interface that received the sign-in (C2). Only that Interface's
     retry, in the session and invocation that its requestState binds (IFC-049), carries a
     return, under PND-013. Polling under PND-011 then goes on, and the account check decides
     (C7).
   - PND-010 already lets a scoped Interface sign in its own Fleets' Agents, and that stands. That
     in v0 the Interface is root in practice (C2) is a limit, written in step 8, not in the
     promise.

5. **PND-013, the return** (73 points 1 and 2; 75 C2, C3, C5). Add it to
   `spec/promises/pending.yaml` after PND-011, with the layer `convention`. PND-008 is 038b's:
   leave it.
   - **When**: an Interface's retry of sign-in.v1 carries `signInReturn`.
   - **The checks, in this order**, each with the reason it reports:
     1. The relayed login's `return` is `redirect` or `page`, the login is stated `carried:
        true`, and the implementation of its `sign-in-hook.v1` carries Conduit's keys
        (IFC-007). Otherwise `not-accepted`.
     2. No return has been passed on for this invocation. Otherwise `already-returned`.
     3. The value is at most 8192 bytes, all printable ASCII. Otherwise `malformed`.
     4. For `redirect`: the value parses once as an absolute URL with no user information, no
        fragment and no parameter twice, otherwise `malformed`. Its scheme is `http`, its host a
        loopback host (`localhost`, `127.0.0.1` and `[::1]` count as one) and its port and path
        those of the relayed `redirect_uri`, otherwise `address-mismatch`. Its `state` equals the
        relayed `state`, otherwise `state-mismatch`. Its `iss` equals the login's `issuer` where
        both are present, otherwise `issuer-mismatch`. For `page`, the value is the code the
        page shows, and it is opaque.

     IFC-049's checks of the retry's requestState come first and are unchanged: a retry they
     reject carries no return anywhere. A retry that declines or cancels ends sign-in.v1 failed
     under PND-010, and a value it carries is not passed on.
   - **Anywhere else.** `signInReturn` on a first request of sign-in.v1, on a request for any
     other operation, or in a `tasks/update`, is not a return. The request reports invalid after
     its access decision and before anything is dispatched (OPS-023), with a diagnostic that
     names PND-013 and the key and holds no value. For a `tasks/update` that is the error IFC-016
     gives a task request that does not proceed, with the reason invalid. A return travels only
     on a `tools/call` retry whose requestState IFC-049 accepts.
   - **Passing it on.** The engine rebuilds a redirect's value as the relayed `redirect_uri` with
     a query of only `code`, `state`, `iss` and `scope`, and on an error `error`,
     `error_description` and `error_uri` (RFC 6749 §4.1.2, RFC 9207). It passes the value on by
     retrying the `sign-in-hook.v1` call that started the login: the retry carries that call's
     own requestState, `accept` in `inputResponses` for its URL elicitation, and the value in
     `signInReturn`. The retry is a part of sign-in.v1, as the call it retries is (PND-011), and
     is recorded as that call is; its record holds neither the value nor `signInReturn`. The
     value so reaches only the package, and the login, that started it. The implementation gives
     it only to the program that waits on that login: for `redirect`, as a request to the
     relayed `redirect_uri`'s port and path at the loopback address of its own machine; for
     `page`, as the program's input. It contacts no other host. The hook succeeds where the
     program took the value, and fails otherwise.
   - **Once.** A value that fails a check is not passed on, and a later value still may be. Once
     the engine has passed a value to the hook, the return is used, whatever the hook reports. A
     return that carries `error` is passed on too.
   - **What the Interface learns.** The result that answers the retry, another
     InputRequiredResult or the final result, carries `signInLogin` with `returned`, one of:
     `delivered`, where the hook reports that the program took the value; `undelivered`, where
     it was passed on and the hook failed, with a diagnostic naming the hook's cause, the return
     being used all the same; or `refused`, with `reason`, the check that failed. For a return
     that carried `error`, `error` names the error code where RFC 6749 registers it, and `other`
     where it does not, never `error_description` or `error_uri`. Polling then goes on under
     PND-011. An undelivered return ends nothing; the person may decline or cancel (PND-010).
   - **Never kept, never shown.** The value appears in no result, `structuredContent`, `_meta` of
     a response, notification, diagnostic, log or record. The engine holds it in memory only
     until the hook's retry ends, and a restart loses it. REC-002 keeps no `_meta` value but the
     ones it names, so say only that the return is none of them.
   - **What each layer stops**, in the observable:
     - `state` stops a blind injection and cross-site request forgery;
     - `iss`, where the login states `issuer`, stops a mix-up of authorization servers (RFC
       9207; RFC 9700 §4.4). Where it states none, Conduit stops no mix-up, and the login's
       program relies on its own defenses;
     - PKCE with S256 stops anyone else from redeeming a code;
     - the stated login stops a look-alike page and, where the URL carries a client ID, a Type
       that claims another Type's client. A `device` URL names no client: the engine checks
       only that the page is the stated one, where the person types the code (RFC 8628 §5.4);
     - the account check stops a code that someone who saw the URL minted for another account
       (RFC 9700 §4.5). It is the only layer that stops that, so no login that returns anything
       is ever without it (C7).
   - **The Interface.** An Interface carries a return only from a listener it opened on a
     loopback address for that one login, or from what the person typed, never from a model's
     output. It opens a listener only where it knows that it does not run on the machine of the
     node that `signInLogin` names: on that machine the program's own listener takes the
     redirect, and a second listener on its port would take the browser's request or keep the
     program from listening. Where it cannot tell, or cannot bind the port, it opens none and
     asks the person to paste. The listener answers only the request whose path, `Host` and
     `state` match, and stops after it or when DL18 ends. The engine cannot tell where a return
     came from; that is a limit (step 8).
   - *The admin's design*: the order of the checks, the reasons and their names, the values of
     `returned`, the 8192 bytes, the parameters kept, passing the value on by retrying the hook,
     invalid for a return anywhere else and no return through `tasks/update`, and the Interface
     that listens only off the node's machine. Report them as applied.

6. **What follows from it.**
   - PND-004: a return is neither evidence nor a prompt. PND-013 alone governs it, and only
     `signInReturn` carries it.
   - PND-005: its evidence names the login started, by its `name` and `return`, in place of
     "method" (line 138). Its attempts list each return: its `via`, its time and its result,
     never its value.
   - PND-012: no record holds a prompt or a return.
   - DL18: the wait also bounds the return.
   - IFC-016 and IFC-045 stay as they are: no form mode, nothing else relayed, and a plain
     server's capabilities as before. Check that nothing you write contradicts them.

7. **The contracts in `spec/core/operations.yaml`.** Write each description so that it names
   what the contract does (IFC-033):
   - `sign-in.v1`: the login asked for, one fixed-port login at a time, the match with a stated
     login, the engine's message, the return, and a return anywhere else;
   - `sign-in-hook.v1`: it starts one of its Type's logins, the one asked for where there is
     one, and gives its URL; on the engine's retry it gives a return only to that login's
     program, as step 5 says;
   - `install.v1`: the output `logins`, the input `confirmClientIds`, and the conflict, refusal
     and invalid of step 2;
   - `inspect.v1`: the output `logins`;
   - `register.v1` and `replace.v1`: the conflict for a submitted project's client ID.

8. **Definitions and READMEs.**
   - `spec/definitions.md`, "Pending actions and sign-in": add the rows "login" (what a Type
     states in `signIn.logins`; CTR-013) and "return" (the value a browser login ends on, the
     final address of a loopback redirect or the code a page shows, carried from the person's
     Interface to the program that waits for it; neither evidence nor a prompt; never kept;
     PND-013). Amend "prompt" (line 269), as 038b left it, "expected account" (line 273) and
     "account check" (line 274) to match.
   - `spec/README.md`: leave the Status paragraph (lines 3 to 11) as it is, because order 071
     edits its lines 7 and 8. Recount, and write, every count you change in the Contents rows
     and in the paragraph that begins "Every area has been written as promises" (lines 250 and
     251). In "Pending actions, sign-in and the network", after the paragraph that ends "A5
     remains real-account evidence to obtain." (line 410), state these limits:
     - an installed package holds the credential of every login it runs, and could take it with
       no return; the defense is the choice of packages, pinned by digest;
     - a model-driven console could fill `signInReturn`, and the engine cannot tell;
     - a `device` login's URL names no client, so the engine checks only its page; a `service`
       login's return reaches its program with no part in Conduit, so the engine checks only
       its URL;
     - on the engine's machine, the root Interface's token (DL13) and every state directory can be
       read by a process of the same operating-system user, an unconfined Agent among them
       (PRV-029, DL16). Until Agents are confined, a package that keeps a Resource's login runs
       where no unconfined Agent runs; the engine does not check this in v0 (75 C1);
     - in v0 the person's Interface runs on the engine's machine. From another machine or a phone
       the person reaches it over SSH and pastes the return, or uses a device code. An SSH
       forward of a redirect's port is the person's own, and carries nothing through Conduit. A
       phone Interface is v1's, and a root Interface on another machine waits until Agents are
       confined (75 C4, C5);
     - in v0 the Claude Code Agent Types state their logins `carried: false`, so a Claude Agent
       signs in only with a browser on its own machine; this is to be revisited in v1 (75 C3);
     - in v0 the Interface that receives a sign-in is root in practice (75 C2).
   - `spec/core/README.md`: the counts at lines 29 and 32, every table you change, and the
     section at lines 447 to 460, which gains the stated logins, the client-ID rule and the
     return. If you stage a patch, lines 42 to 44 also name `038c-staged/` and order 048a.
   - `spec/names.yaml`: no change. It holds the prefix `metaPrefix` and no key after it, and the
     two keys need only IFC-049 (step 3). If you find a reason a name belongs there, report it.

9. **The vector**, live, because the loader reads the schema from `spec/schemas/` and needs no
   change. Add `conformance/language/contracts/sign-in-logins/case.yaml`, with the ID
   `contracts-sign-in-logins`, `layers: [language]`, the promises CTR-013 and SUB-005, the
   profile `first-build`, the initial recipe `empty`, and `load` steps:
   1. accepted: an Agent Type whose signIn states a `redirect` login (`carried: true`, the fixed
      ports 8400 and 8401, an `issuer`) and a `device` login whose `authorization` has another
      path than the `redirect` login's;
   2. accepted: a Resource Type whose signIn states a `page` login, a `service` login whose
      `page` differs from the `page` login's, and a `redirect` login with `ports: any` and
      `carried: false`;
   3. then one step each, rejected under SUB-005 with its keyword location: signIn without
      `logins`; `logins` empty; a `return` that is none of the four; a `redirect` login without
      `clientId`; one without `carried`; one whose host is `example.org`; one whose port is 80;
      an `authorization` that is `http:`; one with a query; a `device` login without `clientId`;
      a `device` login that states `carried`; a `service` login without `page`; a `page` login
      that states `issuer`; a login with a member that step 1 does not list.

   The accepted Types meet every CTR-013 rule, so that 048a's loader keeps accepting them: their
   schema requires `account`, a string or null, they require the three hooks, their logins'
   names differ, no URL could match two of their logins, and the two Types state no client ID in
   common. Use placeholder client IDs and the host `example.org`, never a real service's. Write
   each expectation from the schema first, then run the suite; where the loader says otherwise,
   find out why before you change either.

   Add the case to the coverage index, `conformance/manifest.yaml`, or to the piece that 071's
   README names if your tree has 071, with the same promises and `"safety_rows": []`. List the
   case's path in CTR-013's `vectors`, as SCH-009 lists its own (`spec/promises/schemas.yaml`
   line 161). Write `docs/work-orders/038c-declared.yaml` with exactly the shapes that
   `python3 tools/vector_delta.py` reports for the case: one `added`, and, where it reports them,
   its files as `fixture` with the operation `add`.

10. **Stage what the engine rejects, and hand off.** C25 of the batch 1 plan (line 361): a spec
    order lands only edits that keep the engine's tests green, and stages the rest verbatim.
    - Land live everything that the engine's tests and the checks accept. Stage an edit they
      reject as an exact patch for 048a, made against your finished tree with
      `docs/work-orders/036b-staged/048a.patch` applied, and say why. The likeliest are the new
      input and outputs of `install.v1` and `inspect.v1`, if a vector asserts their tools or
      results whole.
    - No live text names a staged input or output. Where you stage one, stage with it, in the
      same patch, each sentence that names it, in CTR-013, REC-002, PND-010 or elsewhere, and
      report each.
    - Create `docs/work-orders/038c-staged/`, laid out as `036b-staged/` is:
      - `README.md`;
      - `048a.md`, linking `048a.patch` if you staged one. It says what 048a builds of this
        order: CTR-013's cross-document checks; the client-ID rule at install, registration and
        replacement, with its record; `logins` at install and in inspection; the login asked
        for; one fixed-port login at a time; IFC-049's match and the engine's message. It lists
        048a's planned vectors, each with the promises it tests, starting from these:
        login-undeclared, client-id-mismatch, no-s256, short-state, two-logins-match,
        device-url-has-query, service-redirect-mismatch, message-engine-authored,
        login-asked-unknown, login-asked-plain-hook, client-id-claimed-twice,
        sibling-types-share-client-id, client-id-confirmed-recorded,
        client-id-confirmation-not-root, client-id-confirmation-needless,
        submitted-project-claims-client-id, package-after-submission-claims-client-id,
        two-submissions-claim-client-id, replace-claims-client-id, fixed-port-one-at-a-time,
        fixed-port-action-hidden, and user-code-only-in-panel, which replaces the plan's
        user-code-only-in-elicitation (line 573). The scripted signIn fixture Type that 048a
        writes states its logins;
      - `return.md`, for the order that builds PND-013 after 048b and for the packages' and the
        CLI's parts:
        - Its planned vectors, each with its promises: redirect-delivered, page-delivered,
          not-carried, device-not-accepted, service-not-accepted, state-mismatch,
          address-mismatch, issuer-mismatch, malformed, other-session, after-wait, twice,
          refused-not-used, oauth-error-passed-on, undelivered, never-recorded,
          hook-retry-recorded-without-value, return-on-first-request, return-on-other-operation,
          return-under-tasks-update, return-with-decline and injected-other-account-undone.
        - What order 050 states in the Agent Types. For Claude Code, each of its logins with
          `carried: false`. For Codex, its browser login as `redirect` with `carried: true`, and
          its device login as `device`; the Codex hook starts the browser login unless the
          device login is asked for (75 C5). The client IDs, origins, hosts, ports and paths
          are read from the clients by order 050, never guessed.
        - That a probe confirms a carried redirect completes Codex's login (75 C5). It has no
          row in `ROADMAP.md` yet: say so.
        - What the CLI does: the loopback listener of step 5, opened only where the CLI does not
          run on the machine of the node that `signInLogin` names, and a paste where it cannot
          tell or cannot bind the port; a paste read from the terminal with echo off, never from
          an argument, standard input or the environment; `conduit call` refusing to set
          `signInReturn`; the `logins` shown at install and in inspection; and a named flag for
          `confirmClientIds`, counted when used (55 A4), whose name the CLI's order chooses;
      - `vector-impact.md`: every existing expectation that the new text contradicts, by path and
        zero-based step, or "none" with how you searched. Search every case under
        `conformance/`, 041b's among them where your tree has them. Start from the cases that
        assert `install.v1`'s, `inspect.v1`'s, `register.v1`'s or `replace.v1`'s tools or
        results, and any fixture that states signIn;
      - a `.gitattributes` holding `*.patch -whitespace`, if you stage a patch.
    - **The scratch check**, whether or not you stage a patch, because your live edits touch
      files that 038b's patches also change. Copy your working tree, without `engine/target/`,
      to a directory outside the repository. There, running `git apply --check` and then
      `git apply` for each patch in turn, apply `docs/work-orders/036b-staged/048a.patch`, then
      your `038c-staged/048a.patch` if you staged one, then `038b-staged/049a.patch` and
      `038b-staged/049b.patch`. Run `python3 tools/check_schemas.py` and
      `python3 tools/check_docs.py` there. Report each command's last line, then delete the
      copy. Write its location in no file. If one of 038b's patches no longer applies, report
      the hunk and the edit of yours that breaks it, and move your edit where you can; never
      change 038b's patches.
    - In `docs/work-orders/036b-staged/048a.md`, add one line pointing to
      `../038c-staged/048a.md`.

11. **IN_FORCE.md**, topic 9:
    - Lines 193 to 196 become pointers, in that file's form, to the promises and limits that now
      hold them. A part not written says what remains and which order lands it: the Agent Types'
      logins (order 050), and the probe that a carried redirect completes Codex's login (75 C5).
    - Line 192's last sentence, "Codex signs in by its device flow where the account allows it (48
      point 2, 49)", differs from 75 C5, and the owner's words govern (IN_FORCE.md line 5).
      Replace it with C5's words, cited.
    - The sign-in pointer at line 188 ("**In spec/**: sign-in, PND-005, ...") gains PND-013 and
      `sign-in-login.v1`, and the pending-actions pointer at line 184 ("pending actions,
      PND-001 to ...") gains PND-013 in its list.

12. **`docs/guides/accounts.md`.** Lines 14 to 17 (turn on Codex's device sign-in first) and lines
    36 and 37 ("**Codex** shows a code ... you never paste anything back") contradict 75 C3 and
    C5. Rewrite them: Codex signs in by its browser login, which on one machine completes by
    itself and from another comes back through `conduit` or an SSH forward, with the device code
    an option where the account has it on; Claude Code signs in with a browser on the Agent's own
    machine. Keep the guide's voice, and name no command that does not exist yet.

13. **Before you finish, check** for the faults behind batch 1's second rounds:
    - every request that is not dispatched names exactly one of refused, invalid, conflict and
      unavailable, in OPS-023's order: the unknown login asked for, the fixed port in use, the
      shared client ID at install, registration and replacement, its confirmation from a caller
      other than root, a needless confirmation, and a return anywhere but a retry;
    - every refusal and failure has a diagnostic naming its rule and cause (ruling 52; ACC-056;
      IFC-050), and none holds a return, a code, a `state`, a value of a URL, `error_description`
      or `error_uri`;
    - what each caller may see: a return reaches only the Interface that received the sign-in and
      the hook that started its login; no Agent sees a sign-in's URL, login or return (PND-002,
      IFC-016); and no diagnostic names a pending action its caller may not see;
    - every case of each promise: each check of PND-013 and each reason; each value of
      `returned`; each `return`, with `carried` true and false; a return on a first request,
      another operation, a `tasks/update` and a declining retry; a retry with a return after DL18
      and after one was passed on; a URL that matches no login, and one that matches the wrong
      one; a client ID claimed by a package, by a submission, and by each first;
    - every closed list you extend: the `_meta` keys of IFC-006 and IFC-049, PND-005's evidence,
      the inputs and outputs of `install.v1` and `inspect.v1`, what REC-002 keeps, the exports,
      and every count in `spec/README.md` and `spec/core/README.md`, recounted from the files;
    - in promise text, a Type states its logins, and nothing is called a declaration;
    - no trailing white space in Markdown, and every patch exact.

## Scope

Change:

- in `spec/`:
  - `spec/promises/contracts.yaml` (CTR-004, CTR-013) and `spec/promises/pending.yaml` (PND-004,
    PND-005, PND-010, PND-012, and the new PND-013);
  - `spec/interface/implementation.yaml` (IFC-006, IFC-049) and `spec/implementation-defined.yaml`
    (DL18);
  - `spec/conventions/records.yaml` (REC-002's observable) and `spec/core/types.yaml` (its head
    comment, lines 18 to 20);
  - `spec/schemas/contracts.yaml`, `spec/schemas/README.md` and `spec/core/conduit.yaml` (the
    export);
  - `spec/core/operations.yaml`: the six contracts of step 7, live or staged by step 10;
  - `spec/definitions.md`;
  - `spec/README.md`, except lines 3 to 11;
  - `spec/core/README.md`;
- `conformance/language/contracts/sign-in-logins/`, which you create, and its entry in the
  coverage index;
- `docs/work-orders/038c-declared.yaml` and `docs/work-orders/038c-staged/`, which you create, and
  one line in `docs/work-orders/036b-staged/048a.md`;
- `IN_FORCE.md`, lines 184, 188 and 192 to 196;
- `docs/guides/accounts.md`, lines 14 to 17, 36 and 37.

Do not change:

- `engine/`, `packages/` or `tools/`. A checker change that the staged text needs goes into a
  patch;
- `spec/names.yaml`;
- SEC-020, PRV-029, PND-011, IFC-016, IFC-041 and IFC-045, except to cite them; PND-008, or any
  other text 038b wrote, except to add to it where this order says;
- the patches in `docs/work-orders/036b-staged/` and `docs/work-orders/038b-staged/`, or
  `docs/work-orders/README.md`;
- anything for enrollment or for batch 2;
- a record: the rulings' records, the ledger, `docs/AUDIT.md`, `ROADMAP.md`,
  `docs/implementation/` (the safety catalogue among them) and `docs/proposals/`. The batch 1
  plan's lines 47 and 272, which put codes in the elicitation's message against PND-004, and the
  vector name at its line 573, are the admin's to fix: report them.

Report anything else that needs changing.

## Done when

Every item of steps 1 to 12 is written, or staged, or reported absent with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py --declared docs/work-orders/038c-declared.yaml`, with that
  interpreter: exit 0, the one added case, and no undeclared change
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `git diff --check`

In the scratch copy of step 10: `git apply --check` and `git apply` for each patch in turn, then
`python3 tools/check_schemas.py` and `python3 tools/check_docs.py`.

Run these with `bash`. Each prints nothing. The first shows that every promise ID the new files
cite exists in `spec/`; the second and third cover files that `git diff --check` does not see,
because they are not tracked. (The brackets in the third keep this order free of the paths it
looks for.)

```bash
for id in $(grep -rhoE '\b[A-Z]{2,5}-[0-9]{3}\b' docs/work-orders/038c-staged conformance/language/contracts/sign-in-logins | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
grep -rn -E '[[:space:]]+$' docs/work-orders/038c-staged docs/work-orders/038c-declared.yaml conformance/language/contracts/sign-in-logins
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/|\.claude/' docs/work-orders/038c-staged docs/work-orders/038c-declared.yaml conformance/language/contracts/sign-in-logins
```

In `engine/`:

- While you decide what lands live and what is staged, run
  `cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance` as often as you
  need. These crates read `spec/core/` and `spec/schemas/`, and the last runs the new case.
- Once the split is final, run `cargo test --release` once. If it fails because of a live edit,
  stage that edit and run it once more.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers only.
If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load (finding
4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail in your
sandbox: say so, name each, and the admin reruns them.

Report:

- the promises added and amended, by ID;
- the contracts and schemas added and changed, each marked live or staged, with the order that
  lands it, and each sentence staged with them;
- every reading you applied, and each decision marked the admin's, as applied;
- every expectation the new text contradicts, by path and step;
- the rows the safety catalogue needs (75 E1): PND-013's, a convention-layer promise, and any
  row whose quoted text you changed;
- that the probe of 75 C5 needs a row in `ROADMAP.md`;
- if your tree lacks 071, that the vector count at `spec/README.md` line 7 is now stale;
- each check's last line.

Do not `git add` or commit anything.
