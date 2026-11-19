# Ruling 82: the answers to the targeted review of 0.2.0's sign-in and home surface

Status: in force; amended by 83 (point 2: `apiKey` is not in `.v2`; the Agent packages 0.2.0 are subscription-only; point 6, o7: the freeze as point 2 is amended; point 1: its clause on `apiKey` falls with point 2); amended by 84 (point 6: A22 is no longer owed before any Claude Agent is a slot; its line that A81 and A22 stay owed as before holds for A81 only); amended by 85 (point 3, as the coordinator read it, narrowed: a dogfood slot on the Mac signs in after 051b-2, not 051b-3; 051b-3 still comes before any sign-in that needs a pasted return) (ruled by the owner on 2026-11-16). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 5, 9, 13 and 15 (spec pending wherever `spec/` must change), and A5's and A78's rows of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes). This record says why.

After the record of [the targeted review of 2026-11-16](../review-signin-2026-11-16/README.md)
merged (#336), the coordinator put its questions to the owner through the session's question tool,
in two dialogs, each question with options and a recommendation. The first dialog put four: the
record's Q3, Q2, Q4 and Q5 ([section
5](../review-signin-2026-11-16/README.md#5-the-owner-questions)). The owner typed answers to the
first two. The coordinator then explained the Codex question, and put four more: the Codex question
as the coordinator reshaped it on the owner's request; the API-key question again; Q1; and the one
accept-or-strike of the record's lines o1 to o7 ([section
1](../review-signin-2026-11-16/README.md#1-the-open-line-clause-by-clause)). The dialogs ran from
04:48 to 04:54 UTC on 2026-11-17, the evening of 2026-11-16 local time. The questions as put, their
options and the owner's answers follow.

## The first dialog

1. The Codex question (the record's Q3). As put: "Codex's login status doesn't say which account is
   signed in. The codex.v2 package may promise sign-in only if its account hook can report a
   confirmed account, and a frozen promise can't be removed later. When should codex.v2 promise
   sign-in?" Its options: "After the account check (Recommended)": "Build 050; codex.v2 promises
   sign-in only after the package's own account hook, in a scratch home you sign into once through
   Conduit (A5 stage 1's first step, you told first), reports the account. Until then Codex's part
   of 0.2.0 waits. On a fail: Codex goes on an API key (per the API-key question) and moving sign-in
   out of the core (FR-118) comes back to you."; "No sign-in for codex.v2": "codex.v2 runs on an API
   key only; Codex sign-in waits for a codex.v3. Nothing waits, but Codex is off the subscription
   login for .v2's life, and this is your 'no' to keeping Codex sign-in in the core (FR-118).";
   and "Promise it now": "As planned before stage 0 found the gap. If the hook then can't report an
   account, codex.v2 breaks a must, and every Codex slot moves to a codex.v3 and re-registers." The
   owner chose no option and typed: "Explain further, and what’s a durable solution for account
   discovery, I don’t want to move to api off of subscription unless it’s intentional".
2. The API key (the record's Q2). As put: "Should the .v2 packages have an API-key option (apiKey)?
   It can only be added in 0.2.0. Two of your decisions collide: you accepted an API key as an
   alternative to sign-in (finding 4-02), and ruling 61 B13 bars an Agent that takes turns from
   reading a secret, which is the spec's only route for a key into a run." Its options: "In,
   unusable for now (Recommended)": "apiKey is in the schema, but no .v2 Agent gets a key until
   batch 2 writes B13's text; a template that sets it fails setup with a clear error until then.
   Forecloses nothing and picks neither decision. If Codex ends up key-only, it waits for batch 2.";
   "In, usable now": "The engine puts the key into the client's environment now, reading B13 as not
   covering that. Works today; an unconfined Agent could read it anyway (ruling 70)."; "Not in .v2":
   "No API key for .v2's life; that route waits for a .v3. Leaves Codex no route if its sign-in
   check fails."; and "In, root-only": "Settable only on an Agent that only root takes turns on,
   checked at registration. Follows B13's own wording, but it's a new engine rule." The owner chose
   no option and typed: "API keys can be included but it should be an explicit setting to us an
   api key not a fallback".
3. The merge order (the record's Q4). As put: "When does 0.2.0 (order 050) merge, relative to the
   sign-in orders? Today's engine checks sign-in's shape but doesn't run it, so a 0.2.0 merged
   before the sign-in orders would have its slots signed in by hand, against ruling 72." Its
   options: "After 048b-join (Recommended)": "Write 050 right away beside 048a-1 and 048a-2; 0.2.0
   merges once 048b-join lands, onto an engine that runs the account check. The recommended Codex
   check needs that sign-in flow."; "As soon as 050 is ready": "Keeps 050 'next' (ruling 80 point
   7). Slots signed in by hand until the sign-in orders land, then rechecked. With the recommended
   Codex option, codex.v2 would have to wait, or ship without sign-in for good."; and "Write 050
   after 048b-join": "Longest path; no drift between 050's hooks and 048a-2's text." The owner
   chose "After 048b-join (Recommended)".
4. Later logins (the record's Q5). As put: "May a later .v2 release add a login method, such as
   Codex's device-code login or Claude's code-page login? The freeze rule (NAM-024) can be read
   either way, and the review's verifiers split on it." Its options: "No: state them now
   (Recommended)": "Strict reading: a later login changes what an unchanged sign-in may start.
   codex.v2 lists its device login now (refused with a clear error until its code path and panel
   exist), and claude-code.v2 lists Claude's code-page login. More client details frozen for .v2's
   life."; and "Yes, add later": "A later release may append a login, as NAM-024's 'a further
   operation' example suggests; the device and code-page logins wait. Fewer frozen details, but
   rests on reading NAM-024's list of allowed additions as open." The owner chose "No: state them
   now (Recommended)".

## The coordinator's explanation

Between the two dialogs the coordinator answered answer 1's request in the session, before the
second dialog was put. In summary, under its headings, with "you" read as the owner:

- **Why the Codex promise matters.** Each Agent slot is configured for one specific account, for
  example the primary or the second. After a sign-in, Conduit asks the package's account hook
  which account signed in, and refuses the slot if it is the wrong one. A package may promise
  sign-in only if its hook can answer that, and once 0.2.0 is frozen the promise cannot be taken
  back. Claude Code passes: its status names the account. Codex does not, so far: `codex login
  status` says only that it is logged in using ChatGPT.
- **A durable way to find the account.** Codex already answers this question for its own IDE
  extension: its `app-server` has an `account/read` method that returns the signed-in ChatGPT
  account's email and plan, taken from the ID token OpenAI issued at sign-in. Codex's account hook
  would ask Codex itself, through that documented interface, inside the Agent's own home, and
  never open the login file. If Codex ever stops answering, the hook fails closed: sign-in reports
  an explicit error rather than guessing. One public bug report shows `account/read` erroring when
  those fields are missing, which is failing closed. The caveat, in the coordinator's words: "I
  checked this against Codex's app-server README and commit mirrors, not against the version we
  pin (0.167.1)." A short source read, like order 195's, would confirm it there, with no client run
  and none of the owner's time.
- **Nothing moves the owner to an API key on its own.** No option switches a Codex Agent from the
  subscription to an API key automatically; with the owner's API-key answer, a key is used only
  where a template sets it explicitly; and if the account check ever fails, Codex Agents do not
  sign in yet, and the decision comes back to the owner. The merge-order answer means the live
  check adds no wait: it runs after 048b-join and before 0.2.0 merges, which is when 0.2.0 merges
  anyway.

The sources it named, as it named them:

- [Codex app-server README](https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md)
- [Auth v2 doc commit (#6353)](https://forge.lthn.ai/core/core-agent-ide/commit/2e81f1900da465f1e65c3edfe212be715d707448)
- [Account struct with email and planType (#5829)](https://forge.lthn.ai/core/core-agent-ide/commit/67a219ffc2c08eb0e7ac4176908964458a1fd2a4)
- [Missing plan info fix (#13072)](https://forge.lthn.ai/core/core-agent-ide/commit/ff5cbfd7d4a3d650d3f7408655d9a2b3052e43d7)
- [Issue #12170 (account/read error)](https://github.com/openai/codex/issues/12170)

None of them was read at Codex 0.167.1. This record cites them as the coordinator's evidence for
`account/read`, not as facts about that version.

## The second dialog

5. The Codex question, reshaped. As put: "How should codex.v2 learn which ChatGPT account signed
   in, and when may it promise sign-in? (A frozen promise can't be removed, and nothing ever
   switches a Codex Agent to an API key on its own.)" Its options: "Ask Codex itself
   (Recommended)": "The account hook calls Codex's own app-server account/read (what its IDE
   extension uses; returns the account's email from OpenAI's ID token) in the Agent's home, never
   opening the login file. A source read of Codex 0.167.1 confirms it (no run, none of your time);
   then one live check after 048b-join: you sign a scratch home in once through Conduit. codex.v2
   promises sign-in only if it passes; on a fail, Codex Agents don't sign in yet and it comes back
   to you."; "Promise it now": "codex.v2 promises sign-in in 0.2.0 with no live check first. If the
   hook then can't report the account, codex.v2 breaks a must, and every Codex slot moves to a
   codex.v3 and re-registers."; and "No sign-in for codex.v2": "codex.v2 never promises sign-in;
   Codex Agents in .v2 run only with an API key you set explicitly, and subscription sign-in waits
   for a codex.v3." The owner chose "Ask Codex itself (Recommended)".
6. The API key's use. As put: "You said API keys can be included, as an explicit setting and never
   a fallback. Ruling 61 B13 bars an Agent that takes turns from reading a secret, and that's the
   spec's only route for a key into a run. When should an explicitly set apiKey actually work?" Its
   options: "Once batch 2 settles B13 (Recommended)": "apiKey is in the .v2 schema now, explicit
   only (the engine never picks it). A template that sets it fails setup with a clear error until
   batch 2 writes B13's text. Nothing needs a key today, since every slot is on subscription."; and
   "Usable now": "The engine puts an explicitly set key into the client's environment now, reading
   B13 as not covering that. Works today; an unconfined Agent could read it anyway (ruling 70)."
   The owner chose "Once batch 2 settles B13 (Recommended)".
7. The descriptions (the record's Q1). As put: "The two .v2 tool descriptions are frozen for .v2's
   life and every Agent reads them. Changes proposed: (i) drop the client version; (ii) add the 'on
   the engine's own runner' / other-runtime sentence to both; (iii) the clause 'ends the
   conversation'; (iv) the retire-hook sentence, naming no credential store; (v) codex.v2 names its
   browser login as the default and lists the device login; (vi) claude-code.v2 lists Claude's
   code-page login. Which?" Its options: "All six, keep the clause (Recommended)": "All six; keep
   'ends the conversation' and strike D55's un-refusing for .v2 (D55 was a logged option to change
   that behavior later)."; "All six, drop the clause": "All six, but drop 'ends the conversation' so
   D55 stays open for .v2; the description then says nothing about a poisoned conversation."; and
   "Minimum only": "The branch descriptions plus only what CTR-013 and D53 require: fewest frozen
   words; the device and code-page logins aren't named." The owner chose "All six, keep the clause
   (Recommended)".
8. The lines o1 to o7 (the record's section 1). As put: "Accept the review's answers on the seven
   open sign-in points? o1: the sign-in hook carries Conduit's keys (the other hooks stay plain).
   o2: Conduit's managed connection for a Claude sign-in, stated as tested conditions: it ends in
   programs that aren't Conduit's, a forward opens only for this sign-in's port and closes when the
   sign-in ends, and it binds no person (the account check does). o3: Codex's login opening a
   browser is a stated limit, not a reason to move sign-in out of the core. o4: a second sign-in of
   one instance is refused as 'conflict'. o5: F95–F97 carried by 048a-1 and two small spec fixes.
   o6: the device login stated now (your answer). o7: the freeze answer follows the Codex
   question." Its options: "Accept all (Recommended)": "These lines go into ruling 82 as written in
   the record's section 1."; and "Strike some": "Say which, in the notes; the rest are accepted."
   The owner chose "Accept all (Recommended)".

All eight answers are this ruling, as ruling 67 has it: each decides a question the record left to
the owner. Answer 1 chose none of Q3's options; it asked for an explanation and a durable route for
learning the account, and said that Codex does not move from the subscription to an API key unless
that is intentional. Answer 5, given after the explanation, decides Q3 as the coordinator reshaped
it on answer 1's request, and is read with answer 1's words. Answer 2 allows `apiKey`, as an
explicit setting and never a fallback; answer 6's chosen option puts it in the `.v2` schema now and
decides when it works. Answers 3, 4 and 7 choose the record's options a of Q4, Q5 and Q1, and
answer 8 accepts the record's lines o1 to o7.
That sorting is the coordinator's.

## The decisions

1. **`codex.v2` learns the account by asking Codex itself, and states `signIn` only after one live
   check** (answers 1 and 5; the record's Q3, as reshaped on the owner's request). `codex.v2`'s
   account hook learns the signed-in ChatGPT account by asking Codex itself, through its
   app-server's `account/read`, in the Agent's own home, and never opens the login file. A source
   read of Codex 0.167.1, which the coordinator's explanation called "the version we pin" and which
   stage 0b's source read used ([its
   results](../../../experiments/027-a5-sign-in/results-2026-11-16-0b-source.md)), confirms that
   route before 050's Codex half is written, with no client run and none of the owner's time. If it
   does not confirm `account/read` at 0.167.1, the coordinator, by its own reading of answer 5,
   brings that back to the owner before 050's Codex half is written. Then one live check, after
   048b-join and before 0.2.0 merges: the owner signs a scratch home in once through Conduit's
   sign-in flow, as A5 stage 1's first step, pre-registered in [experiment 027's
   README](../../../experiments/027-a5-sign-in/README.md), with the owner told first; the
   pre-registration names the Type, not an installed `codex.v2`, under which the scratch home is
   signed in, as the record's Q3 option a has it; the package's account hook must report the
   signed-in account. `codex.v2` states `signIn` only if that check passes, and, as the record's Q3
   option a has it, no archive whose `codex.v2` states `signIn` is installed anywhere before the
   check's outcome is recorded ([section
   6](../review-signin-2026-11-16/README.md#6-must-state-lists), 050 `codex.v2` item 1; that
   answer 5 keeps these two terms of Q3's option a is the coordinator's reading). On a fail,
   `codex.v2` does not state `signIn`, Codex Agents do not sign in yet, and the question returns to
   the owner, with FR-118, as dec-35 and the record's [section 2
   (a)](../review-signin-2026-11-16/README.md#2-the-three-questions-the-owner-left-to-the-review)
   have it. Nothing switches a Codex Agent to an API key: a key is used only where a template sets
   `apiKey` explicitly, under point 2. This replaces the account hook's route that spec batch 1's
   area draft planned ([refusals and identity](../spec-batch-1/areas/refusals-and-identity.md), D2
   at line 187 and O3 at line 370): the status first, and if that names no account, the email claim
   of the ID token in the home's login file. With it goes the route the full review's D80 names,
   "Codex's ID-token fallback is tried only here, by the package"
   ([decisions.md:119](../full-review/decisions.md#the-experiments-documents-and-the-register)): the
   route is now `account/read`, still tried only in stage 1, by the package. The engine-side
   promises are unchanged: CTR-013 (`spec/promises/contracts.yaml:305-310`) still lets a Type state
   `signIn` only where `account-hook.v1` reports an account the service attests, and the account
   that `account/read` reports comes, by the coordinator's sources, from the ID token OpenAI issued
   at sign-in, a kind CTR-013 names; the engine still reads only what the hook reports, and
   PND-011's check is unchanged. The coordinator's evidence for `account/read` is the Codex
   app-server README and the commit mirrors listed above, not yet read at 0.167.1.
2. **`apiKey` is in `.v2`, explicit only, and unusable until batch 2 writes B13** (answers 2 and 6;
   the record's Q2). `.v2` holds `apiKey` in 0.2.0, as an explicit setting only: the engine never
   falls back from a sign-in to a key, and a template chooses the key explicitly. Until batch 2
   writes ruling 61 B13's text, a template that sets `apiKey` fails setup with an explicit error,
   and no `.v2` Agent is given the secret route, SEC-010's `get-secret` Conduit, before then. The
   get-secret term is the record's Q2 option a; that answers 2 and 6 carry it, and the schema shape
   of section 6, is the coordinator's reading, open to the owner's strike. The schema follows the
   record's section 6, 050 `claude-code.v2` item 1, and 048a-1 item 2: `account` required, with no
   default, a string or null under the fixture's `oneOf`, beside `apiKey`. This
   settles, for 0.2.0, the conflict the review recorded between finding 4-02 as the owner accepted
   it on 2026-11-12, "never the default"
   ([increment 4's log](../first-build/log-increment-4.md)), and 61 B13 (the review's decided
   section D item 1), without choosing between them. B13's text stays batch 2's.
3. **050 is written now; 0.2.0 merges after 048b-join** (answer 3; the record's Q4, option a).
   Order 050 is written now, beside 048a-1 and 048a-2; 0.2.0 merges only after 048b-join, onto an
   engine that runs the account check; each slot's sign-in follows 051b
   ([ruling 78](../ruling-78/README.md), point 1, part 2). This settles the edge that
   [order 117](../../work-orders/117-plan-s2-s3.md) left open at lines 91 to 93, where 050 comes
   "After 048b-join" without saying whether in the writing or the merging. It also settles how
   [ruling 80](../ruling-80/README.md) point 7's and [ruling 81](../ruling-81/README.md)'s "next"
   applies to 050: next in the writing, after 048b-join in the merging. The live check of point 1
   runs in that interval, so it adds no wait. That ruling 80 point 7's "next" holds in the writing,
   so that this narrows it and does not amend it, is the coordinator's reading; the dialog had
   described option b as the one that "Keeps 050 'next' (ruling 80 point 7)".
4. **A login that 0.2.0 does not state never joins `.v2`** (answer 4; the record's Q5, option a).
   Under NAM-024 (`spec/promises/naming.yaml:414-439`), a login that 0.2.0 does not state cannot
   join `.v2` in a later release: an addition that changes what an unchanged request may do, a
   further login or a member added to a login the earlier release states, is stated in the first
   release of the address or never; an addition that an unchanged request never exercises, such as a
   new operation with its implementation default, stays one of NAM-024's optional things (the
   skeptic's reading, C01, in the record's o6). So `codex.v2`, where it states `signIn` (point 1),
   states its device login now, which the hook, asked for it, refuses with a diagnostic naming
   PND-010 and the unbuilt path until F83's code path and 051c's panel exist, and never chooses
   unasked; and `claude-code.v2` states Claude's code-page login. NAM-024's text gains words saying
   so. Writing answer 4's reading into NAM-024, so that `spec/` binds it, is the coordinator's
   reading of answer 4, as is its carrier, a named spec fix landed before 050 is written, not
   048a-1. The coordinator's reasons: the record carries words that `spec/` must gain
   outside the held orders by named spec fixes, F96's and F95's ([section
   6](../review-signin-2026-11-16/README.md#carriers-outside-the-held-orders)), and it parks
   NAM-024's words as a separate line (its section 7, line 5); 048a-1's list is CTR-013 in the
   loader and the fixture, each done by a loader vector, and NAM-024's words need no change to the
   loader, since the engine's install check that compares only defaults is parked too (line 4,
   B0-11). 050 states the two logins on these words, so they land first. This answers parked line 5
   for logins and members of a login only; its offers entry stays parked for the owner.
5. **The `.v2` descriptions: all six sentences, the clause kept** (answer 7; the record's Q1,
   option a). The two `.v2` descriptions take the six sentences as the record's Q1 states them:
   (i) no client version; (ii) decision 25's other-runtime sentence in both, with "On the engine's
   own runner"; (iii) the clause "ends the conversation" kept; (iv) D53's retire-hook sentence,
   with the login kept where the client keeps it for that directory and no store named;
   (v) `codex.v2` names the browser login as the unasked default and lists the device login;
   (vi) `claude-code.v2` lists Claude's code-page login. (v) stands only where `codex.v2` states
   `signIn` (point 1). Not in the descriptions: the browser-opens limit and the refusal of a home
   outside the state directory. With (iii), D55's un-refusing of a failed conversation is struck
   for `.v2` ([D55](../full-review/decisions.md#dogfooding-and-the-evidence-kept-in-the-tree):
   "Un-refusing a failed conversation waits for the rerun"), since a frozen clause closes that
   open choice for `.v2`'s life (the record's verdict on M68, with V17's Lens A).
6. **The lines o1 to o7 are accepted** (answer 8; the record's section 1). The owner accepted all
   seven, as the record states them, read with points 1 to 5:
   - o1: the item that implements `sign-in-hook.v1` carries Conduit's keys, for `codex.v2` where
     it states `signIn` and for `claude-code.v2`, which states two logins under point 4; the
     account, sign-out and retire hooks need no keys; PND-005 gains F84's two halves (048a-2); the
     turn item's kind stays 050's choice.
   - o2: the connection Conduit manages for a Claude sign-in (ruling 80 point 8) is conditions
     with tests, no Type or core member, scoped to `carried: false`: the channel that carries the
     return ends at each end in a program that is not Conduit's; a forward opens only after this
     sign-in's URL is relayed, for the port its redirect_uri names, to that node's loopback, and
     closes when `sign-in.v1` ends; the connection binds no person, and the account check does; a
     Claude Agent's paste route is the person's own on-node delivery, held by no engine or
     Interface process.
   - o3: on the engine's own runner, Codex's browser login may open the default browser of the
     engine user's graphical session, where one exists: a stated limit in the limits and the
     accounts guide, not a reason to move sign-in out of the core; the account check stays the
     only guard, and FR-118 is not raised for it.
   - o4: a second `sign-in.v1` of one instance is refused as `conflict` (048a-2).
   - o5: F97 is carried by 048a-1; F96 by a named spec fix before 048a-2 is drafted; F95 by a
     named spec fix landed with 050's rewrite of the limits.
   - o6: `codex.v2`, where it states `signIn`, states its device login now (point 4).
   - o7: 0.2.0's sign-in and home surface can be frozen after the record's listed changes, as
     points 2, 4 and 5 resolve them, and `codex.v2`'s `signIn` once point 1's live check passes.
     A81 and A22 gate nothing.

   IN_FORCE.md topic 9's *Open* line, which listed these seven questions until the record and the
   owner's answers, is replaced by settled lines, each in IN_FORCE.md's form, spec pending where
   `spec/` must change, with its carrier. What stays open stays open in its own line: A81, owed
   before S3, and A22, owed before any Claude Agent is a slot (the record's section 2 (c)); and the
   outcome of point 1's live check.

## What it amends

- **Ruling 81**, point 1: the hold of 048a-1, 048a-2, 048b-join, 050 and 051b ends with these
  answers. They are written as point 3 and "What follows" say. Ruling 81's "What follows" stays as
  written.
- **Ruling 81's "What it does not change"**: A5 now gates `codex.v2`'s `signIn` through stage 1's
  first step (point 1), and nothing else.

The full review's logged decisions that these answers settle: D55, for `.v2`, its un-refusing of a
failed conversation struck (point 5); D80, for Codex's account route, now `account/read` in
stage 1, by the package, and for its "It gates nothing", now except that `codex.v2` states
`signIn` only if stage 1's first step passes (point 1); and F95 to F97, with their carriers
(point 6, o5). The rest of D55 and D80 stays the coordinator's, open to the owner's strike.

## What it does not change

- **CTR-013 and PND-011**: the engine-side promises stand. The engine reads only what
  `account-hook.v1` reports, and the expected account stays the only binding.
- **Ruling 61 B13**: its text stays batch 2's (point 2).
- **Ruling 80, point 7, and ruling 81's line on it**: order 050 stays next in the writing, on the
  path to a Fleet that runs in parallel; point 3 says when it merges.
- **Ruling 80, point 8**: the owner's words stand; o2 is the design that order 050 writes.
- **Ruling 75 C5**: Codex's browser login stays its default, and the device login an option, now
  stated in `codex.v2` where it states `signIn`, and refused until its code path and panel exist
  (point 4).
- **Ruling 78, point 1, part 2**: the pin waits for the six sign-in orders, and the owner signs
  each dogfood Agent in once.
- **FR-118**: it stays set aside. It returns to the owner only if point 1's live check fails.
- **A81 and A22**: they gate no member's freeze, and stay owed as before.
- **The review's record**: it is not edited. Its parked list stays for the owner, except line 5 as
  point 4 answers it for logins, and line 9, which reached the owner as Q2 and which point 2
  answers.

## Facts recorded with it

- The owner's time in the sitting: about 6 minutes, from 04:48 to 04:54 UTC on 2026-11-17 (the
  evening of 2026-11-16 local time). The first dialog was put at 04:48:21 and answered at
  04:51:11; the coordinator's explanation followed; the second dialog was put at 04:52:50 and
  answered at 04:53:50.

## What follows

- The held orders 048a-1, 048a-2, 048b-join, 050 and 051b are written from the record's
  [section 6](../review-signin-2026-11-16/README.md#6-must-state-lists) must-state lists, as
  these answers resolve them: Q1 as point 5, Q2 as point 2, Q3 as point 1, Q4 as point 3 and Q5
  as point 4. 050 is written now, beside 048a-1 and 048a-2; 0.2.0 merges after 048b-join.
- A small order reads Codex 0.167.1's source for `account/read`, with no client run and none of
  the owner's time, as order 195 read Codex's source, and pre-registers stage 1's first step in
  [experiment 027's README](../../../experiments/027-a5-sign-in/README.md), with stage 0's
  guards and the owner told first, before 050's Codex half is written.
- The small order that pre-registers stage 1's first step also replaces the ID-token route and
  "A5 gates nothing" in experiment 027's README (stages list and opening) with point 1's route and
  gate.
- F96's named spec fix comes before 048a-2 is drafted; F95's lands with 050's rewrite of the limits.
- NAM-024's words, as point 4 says, by a named spec fix landed before 050 is written.
- [IN_FORCE.md](../../../IN_FORCE.md): topic 9's *Open* line gives way to the settled lines and
  the two open ones; topic 5 gains NAM-024's line; topic 13 gains the order of point 3; topic 15's
  *Coordinator's* line names D55, D80 and F95 to F97 as settled here, and the full review's
  [decisions.md](../full-review/decisions.md) notes D55, D80 and F95 to F97 as ruled here, and
  D18's hold as ended.
- [The tracker](../../AUDIT.md#waiting-for-the-owner) no longer lists these answers as waiting,
  and lists the review's parked list, which stays for the owner, line 5's offers entry included.
- [The handover](../../implementation/HANDOVER.md), rows 9 and 10, and [spec batch 1's
  plan](../spec-batch-1/README.md#the-work-orders) note that the hold ended, and the plan that
  `account/read` replaces Codex's account route of D2 and O3.
- [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A5's row names stage 1's first step
  as point 1's live check, and A78's row the order of the held orders.
