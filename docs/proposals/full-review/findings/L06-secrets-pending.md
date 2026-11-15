# L06-secrets-pending: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The three files are of very different quality. first-start.yaml is small, clear and rightly layered; its one real hole is what its objects hold after the engine's built-in core package changes (F12). secrets.yaml follows the right paradigm (Kubernetes secretKeyRef, the typed stores of External Secrets Operator and Dapr, no store in the engine) and one read path, but its observables are essays that carry further obligations, 22 of its 24 promises have no vector, and it spends much machinery on keeping "every read has a caller" for reads made after the caller's setup has ended. Its weakest point is the lifetime of a value once read: closing the Conduit does not stop the engine's own use on a lasting connection (F4), a run-based instance takes a new value only by being destroyed (F11), and the engine's own runner cannot restart a run under the letter of SEC-018 (F8). pending.yaml is the weakest part of the slice. It is compressed, most promises hold several obligations, and it rests on an undefined surface, "the engine's own panel" (F3). Two of its wire paths are missing (F5, F6), and its hold on work is ambiguous at expiry and absent for Resources (F1, F2). I would cut or move three things: the carried return of PND-013, about seventy lines of an OAuth client's checks in the convention layer, where a device code or the person's own SSH forward already serves; the list-item and operation-input places of secret-reference.v1 that nothing reads (F17); and SEC-024's document-only half, which belongs to the language layer (F9). Situations a real deployment meets that I did not raise as findings: a value from a store that cannot be sent as a header (a final newline, CR or LF); pending-action.v1 has no `state` member although PND-008 has tools list one; and a cycle of store credentials is found only at setup.

**What the reader read.** Commit 2eb8f258 (2026-11-12), read only; nothing was built or run. Read in full, line by line: spec/promises/secrets.yaml (635 lines), spec/promises/pending.yaml (358), spec/promises/first-start.yaml (114), spec/definitions.md (467), spec/README.md (678), docs/guides/full-review.md (229), and AGENTS.md "What holds". Cited texts opened and read: in spec/interface/implementation.yaml IFC-012, IFC-016, IFC-036, IFC-041, IFC-049, IFC-050, and the end of IFC-025; in spec/conventions ACC-003, ACC-011, ACC-019, ACC-032, ACC-033, ACC-039, ACC-056, OPS-022, OPS-023, OPS-033, REC-001 to REC-003, REV-012, REV-014, LIF-013, LIF-035; in spec/promises CTR-013, TPL-012, TPL-017 to TPL-019, MAN-010, MAN-017, OBJ-021, NAM-009, NAM-020, NAM-021, CON-008, FLEET-005, PRV-003, PRV-007, PRV-010, PRV-011, PRV-026 to PRV-030 (PRV-030 in part), ENR-003 to ENR-006, ENR-008 to ENR-012; implementation-defined items DL5, DL6, DL16 to DL19; core contracts get-secret.v1, secret-provider.v1, secret-reference.v1, answer.v1, sign-in.v1, sign-in-hook.v1, account-hook.v1, sign-out-hook.v1, evidence-item.v1, pending-action.v1, sign-in-login.v1, and parts of object.v1 and refusal.v1. Checked for novelty: docs/AUDIT.md "Known to be open" (lines 174-245), the preservation tests, layer decisions and item A67; docs/guides/cold-readers.md lines 39-173; the brief's "Queued and not yet written"; IN_FORCE.md topics 9 to 12 (lines 160-256); and, by search only, G5.md, the increment 4 log and the review list. Not read: the other promise and convention files beyond the promises listed; cited promises I did not open, among them ACC-005, POOL-015, PRV-014, PRV-022, PRV-025, PLC-026, IFC-006, IFC-007, OPS-014 and DL13; IN_FORCE.md topics 1 to 8 and 13 to 16; the ruling records; engine/, packages/, conformance/ and experiments/. No protocol or RFC text was fetched: the statements about RFC 8628, RFC 7009, RFC 8252 and Kubernetes behavior are from memory.


## FR-062

**Whether a sign-in action's expiry lifts the hold on model work is undecided, and one reading lets an Agent work under an unchecked account**

L06/F1 · ambiguity · unsafe · verifier: confirmed, high · decides: coordinator · new

**Promises.** PND-005, PND-003, PND-002, PND-010, PND-011

**Where.** spec/promises/pending.yaml:148

**Quote.** "Until answered, operations that start model work on that instance are not dispatched and report unavailable with the action ID in io.github.shrav-k.conduit/pendingAction. [...] After expiry a new sign-in is raised on the next held request or setup, never by expiry alone."

**Situation.** PND-003 (pending.yaml:60) says that once the expiry passes "the answer is expired", and PND-006 (pending.yaml:307) counts an expiry as answered. Reading A: expiry is an answer, so "until answered" ends and the next model-work request is dispatched. Reading B: the hold outlasts expiry, which is what "the next held request" assumes, though no state is named that keeps it. Steps to harm under A: (1) setup's account-hook reports no account for Agent a1, expected account work@x; an action is raised and work is held. (2) Root calls sign-in.v1 and the URL is relayed. The person finishes the browser login as personal@x after the client stops retrying or after DL18, when "no hook runs later" (pending.yaml:187). (3) The action expires under DL19. (4) A turn is dispatched. The client is signed in as personal@x, no trigger of PND-005 fires, and the Agent works under the wrong account with nothing raised.

**Counterargument.** The account is checked only at setup and at sign-in anyway, and a person who signs in to the wrong account outside Conduit is equally unchecked, so expiry adds one more path to an exposure the design already accepts.

**Least change.** In PND-005, replace "Until answered" with "Until an account-hook.v1 part of sign-in.v1 or of a setup reports the expected account", and add: "Expiry lifts nothing: the next such request is not dispatched, and raises a new action."

**Precedent.** PND-011's own condition SignInUndone, which holds work until an account check clears it; a Kubernetes readiness gate, where a condition and not an event gates traffic.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior: it takes the reading that the promise's own last sentence assumes (ruling 37; ruling 46 point 4). Logged for the owner to strike because it is unsafe.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/pending.yaml:148-150 and 155-156. Reading A rests on pending.yaml:60-61 "Once it passes the answer is expired, with no actor, and nothing is dispatched" and pending.yaml:307-308 "A dismissal or expiry for that invocation stays answered". Reading B rests on "the next held request" (pending.yaml:156). No text in spec/ names a state that outlasts the expiry: spec/definitions.md:274 and spec/README.md:410-412 say only that a held request ends. Harm path checked: pending.yaml:187-188 "DL18 ending without completion ends unknown and no hook runs later"; pending.yaml:284-286 makes the account check the one defense ("It alone stops that attack"). So under A a login finished late is never checked and expiry frees the work. Not recorded: experiments/012-model-checking/G5.md:163-172 leaves the PND promises to MC9; no known-open list, limit or ruling has it. The non-binding plan meant reading B (docs/proposals/spec-batch-1/README.md:279 "A sign-in is raised again at the next held request or set-up").

**Verifier's corrections.** Class, unsafe mark and decider stand. Narrow the least change: the reviewer's wording also lets a setup's account check lift the hold, while PND-003 has sign-in.v1 alone answer the action (pending.yaml:52-53). Either say only that expiry lifts nothing and the hold ends when the instance's sign-in is answered signed-in, or log the setup case as added behavior under ruling 71 point 4.


## FR-063

**The wrong-account hold covers only model work, so a Resource that keeps a shared login goes on serving under the wrong account**

L06/F2 · gap · unsafe · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-192.

**Promises.** PND-011, PND-005, PND-010, CTR-013

**Where.** spec/promises/pending.yaml:216

**Quote.** "If sign-out does not succeed, the engine reports SignInUndone=False with its cause and holds back all model work."

**Situation.** Model work is "a turn of an Agent's model" (spec/definitions.md:225), so a Resource has none. CTR-013 lets a Resource's Type state signIn, and PND-010 lets root sign one in. Steps: (1) Resource github, expected account org-bot, is ready and its token lapses. (2) Root calls sign-in.v1 and picks a personal account in the browser. (3) Polling finds a different account and the sign-out-hook.v1 part fails. (4) SignInUndone=False holds "all model work", which for a Resource is nothing. (5) Every Agent with a Conduit to github has its requests dispatched and they act as the personal account. Also, where sign-in.v1's first check finds another account, "that initial mismatch does not undo the account" (pending.yaml:183), and the Resource serves under it for the whole wait.

**Counterargument.** A first setup that fails its undo leaves the Resource registered and not ready, so requests report unavailable. The hole needs a Resource that was already ready, and the limits already say a package holds its login's credential outside Conduit.

**Least change.** In PND-011: "holds back every request to that instance other than set-up.v1, retire.v1 and sign-in.v1 and their parts", in place of "holds back all model work"; the same for PND-005's hold where the instance is a Resource.

**Precedent.** The existing reason unavailable for a target that is not ready; Kubernetes takes an unready endpoint out of its Service.

**Who decides.** coordinator. A small finding that adds a feature, by extending an existing hold to the other family the text already lets sign in (ruling 71 point 4). Logged for the owner to strike because it is unsafe.

**Verifier.** confirmed, high. Quote verbatim at spec/promises/pending.yaml:216-217. spec/definitions.md:225 "A turn of an Agent's model". PND-005's hold is of "operations that start model work on that instance" (pending.yaml:148-149). A Resource may state signIn: spec/promises/contracts.yaml:311-312 "A Resource needing browser login is implemented by a package that completes it"; pending.yaml:166-167 "Resources remain root's". SignInUndone appears nowhere in spec/ but pending.yaml:216, so nothing holds a Resource's requests. The stated limit covers model work only (spec/README.md:411-412), while the ruled words cover Resources (IN_FORCE.md:189 "The account that each Agent's home or Resource is for is declared in configuration. A sign-in to another account is undone"). Not in any known-open list.

**Verifier's corrections.** Stands, with a narrow path: it needs a wrong sign-in and a failed sign-out-hook on a Resource that was already ready. The fix stops a ready Resource for every consumer, so log it for the owner to strike. See extra 2: "all model work" is itself open between the instance and the engine.


## FR-064

**"The engine's own panel" is the only place for codes, fingerprints and secret-bearing join actions, and no promise says what it is or who may see it**

L06/F3 · gap · unsafe · verifier: weakened, medium · decides: owner · new

**Promises.** PND-004, PND-008, IFC-049, ENR-003, ENR-004, ENR-005

**Where.** spec/promises/pending.yaml:76

**Quote.** "The engine alone renders evidence at its panel. Values of kind code or fingerprint appear in no elicitation message, tool result, _meta, notification or record of a request; they are shown only in the engine's own panel and recorded with the answer."

**Situation.** The definitions give only "control panel", an Interface (spec/definitions.md:267). The values are barred from every message of the implementation interface, so the panel cannot be an Interface's client reading tool results: it is a second surface between a person and the engine. No promise, definition or implementation-defined item says what it is, how a person reaches it, how the viewer proves to be root, or how "root's own panel" (pending.yaml:343) differs from another. Two builders diverge: terminal output, a loopback page, a desktop window. An engine that serves the panel on a port to whoever connects keeps every promise and shows "every command and secret-bearing action" (pending.yaml:85) to a stranger. Enrollment and the device login cannot be built from spec/ alone.

**Counterargument.** In v0 the person's Interface runs on the engine's machine, and the limits already put the engine's operating-system user inside the domain of trust, so a local rendering under that user may have been meant as each engine's own choice.

**Least change.** Add a row "the engine's own panel" to definitions.md and one interface promise: the panel is the engine's own rendering to a person, outside the implementation interface; it shows an action's withheld evidence only to what has proved by the mechanism of DL13 to be an Interface at which the action is raised, root alone for PND-008's kinds; how it is reached is a new item of the implementation-defined list.

**Precedent.** RFC 8628: the device shows the user code on its own display. kubeadm token create --print-join-command and tailscale up print the join secret on the operator's own terminal. POSIX's conformance document for the implementation-defined part.

**Who decides.** owner. It fixes what every engine must offer a person and who may see secret-bearing evidence: behavior and scope. A coordinator may add the definition and the implementation-defined item alone under ruling 46 point 4.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/pending.yaml:76-79. spec/definitions.md:267 defines only "control panel", an Interface; no row defines the engine's panel, and spec/implementation-defined.yaml has no item on it (no match for "panel"). The audience is partly stated: pending.yaml:343-344 "Only root's own panel shows their machine evidence, code, fingerprint or join action"; pending.yaml:90-91 "The panel can display the full recorded evidence to a person permitted to inspect the answer"; spec/promises/enrollment.yaml:73-74. The owner ruled the place: docs/proposals/ruling-56/README.md:23 "The engine alone, at the root Interface's control panel". The surface exists only in a plan for an unwritten build order: docs/proposals/spec-batch-1/README.md:643 "over a root-only channel that no MCP tool exposes". Not in any known-open list or limit.

**Verifier's corrections.** The gap is real and new: an undefined term (form rule 4, spec/README.md:300) for a person-facing surface outside the wire, with no promise or implementation-defined item on what it is or how its viewer is established. Also missing: where a scoped Interface's person sees a device code, since PND-010 lets one sign in (pending.yaml:166-167) and codes are barred from every message. Smaller than stated: "no promise says who may see it" is too strong, and the unsafe mark is not sustained. An engine that shows the panel to whoever connects does not keep "Only root's own panel shows", and a leaked join action or user code still meets ENR-006's fingerprint comparison and PND-011's account check. A statement of how the panel's viewer is established would settle the mark. Decider: the definition row and the implementation-defined item are the coordinator's; where a scoped Interface's person sees a code is the owner's.


## FR-065

**Closing a secret's Conduit, or retiring the secret Resource, does not stop the engine's own use of the value on a lasting connection until the engine happens to restart**

L06/F4 · contradiction · unsafe · verifier: weakened, medium · decides: owner · new

**Promises.** SEC-013, SEC-023, ACC-011, CON-008

**Where.** spec/promises/secrets.yaml:357

**Quote.** "the connection that lasts is kept, and closing the reading Conduit recalls nothing, as SEC-014 has it for a run."

**Situation.** ACC-011 (spec/conventions/access.yaml:187) says of a closed Conduit: "All the access it carried ends. None of it remains in force." For a run the value has left the engine, and no recall is possible. For a remote item the engine itself holds the value, and SEC-013 has it go on sending the header on every HTTP request of the connection. Steps: (1) Root closes the get-secret Conduit of instance r, or retires the secret Resource, to cut r off. (2) Inspection shows r with no Conduit to the secret. (3) Every later request to r still goes out with the secret, for days. (4) At the next engine restart SEC-023's read is refused and r turns unavailable. Revocation takes effect at an unrelated event, and inspection does not show the difference.

**Counterargument.** The text is deliberate and symmetric with a run: a read is a completed act, closing recalls no copy a reader made, and the limits say that closing the Conduit stops the read again of SEC-023.

**Least change.** In SEC-013, add to the events that end the holding: "or the Conduit that the value was read through is closed; the connection then counts as one whose values the engine no longer holds, and SEC-023 applies". The next request reads again, is refused, and reports unavailable.

**Precedent.** RFC 7009 token revocation, and Vault's lease revocation, after which the agent that holds the secret stops presenting it.

**Who decides.** owner. It changes a promised behavior. It reuses SEC-023's existing path and adds no mechanism.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/secrets.yaml:356-358. The text decides the case and squares it with ACC-011: secrets.yaml:384-385 "Closing the Conduit ends every later read through it, under ACC-011, and leaves the run with the value, as closing recalls no copy that a reader made"; spec/conventions/access.yaml:189 "After the close every request through the Conduit reports refused"; secrets.yaml:339-341 names the only events that end the holding. The principle is a recorded decision: docs/proposals/secrets/README.md:127-129 "Closing the Conduit stops later reads and cannot recall a copy; to revoke a credential, rotate it in the store" (accepted by ruling 41). The limits say closing stops only the read again (spec/README.md:561-564).

**Verifier's corrections.** Not a contradiction, and not unsafe by the brief's test: the engine does what SEC-013 says. What survives is an owner's design question, with a situation the secrets record lacks. Its reason, that a copy cannot be recalled, does not hold where the engine itself holds and sends the value. A further setup whose read is refused keeps the old connection (secrets.yaml:356-357), so after a close only the instance's retirement or an engine restart ends the use, and the close takes effect at the restart (secrets.yaml:585-587). Class: a stated behavior to put to the owner under preservation test 2 (docs/AUDIT.md:300). A fix with no new behavior is a bullet in the limits saying that closing does not end the engine's use on a lasting connection. Decider stays the owner.


## FR-066

**A device login's user code has no path from the sign-in hook to the engine**

L06/F5 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PND-005, PND-004, IFC-049, CTR-013

**Where.** spec/promises/pending.yaml:145

**Quote.** "Evidence names the expected account, the login started by name and return, device user code where relevant, triggering request and attempts with their results, subject to PND-004."

**Situation.** The program that the hook starts obtains the user code from the service. sign-in-hook.v1 declares no outputSchema (spec/core/operations.yaml:1412-1429). Its URL elicitation holds only mode, message and url. For device the URL is "authorization itself, with no query, code or client", and "No text from the hook is relayed" (spec/interface/implementation.yaml:1371). So no member carries the code to the engine, yet IFC-049 has the message say "the code is in the engine's panel". One builder parses the hook's message; another adds a private key; a third cannot offer device logins. The limits name the device code as the fallback for a person on another machine (spec/README.md:430).

**Counterargument.** Order 050 still owes the Agent Types' client-specific logins and could define the member in a Type's contract. But evidence is the engine's to record and show, so the member belongs to the core operation.

**Least change.** In IFC-049 and sign-in-hook.v1: "For a device login the hook's input request holds the user code in `_meta` under `io.github.shrav-k.conduit/signInCode`; the engine keeps it as one evidence item of kind code and relays it nowhere." Add the name to names.yaml.

**Precedent.** RFC 8628 section 3.2: the device authorization response gives user_code beside verification_uri.

**Who decides.** coordinator. A small finding that adds one wire member (ruling 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/pending.yaml:145-147. sign-in-hook.v1 has no outputSchema (spec/core/operations.yaml:1412-1429). spec/interface/implementation.yaml:1369-1371 "for device it says the code is in the engine's panel ... No text from the hook is relayed"; 1401-1402 "For device it is authorization itself, with no query, code or client"; 1416 "never in its input request, which holds only mode, message and url". Evidence must come from what "an implementation or provider reported through a call the engine made" (pending.yaml:74-76), and spec/names.yaml states no key for a code. Not recorded: IN_FORCE.md:195 leaves only the Agent Types' client-specific logins to order 050, not a member of the core operation.

**Verifier's corrections.** None of substance. Medium, not high: device is an option beside the browser login in v0 (IN_FORCE.md:191; spec/README.md:429-430).


## FR-067

**PND-005's third trigger, a contract that "reports a missing or lapsed login", has no form an engine can observe**

L06/F6 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PND-005, IFC-016

**Where.** spec/promises/pending.yaml:143

**Quote.** "or its contract reports a missing or lapsed login."

**Situation.** A contract is a document and reports nothing. IFC-016 raises PND-005 only on a URL elicitation outside sign-in.v1, and no key, condition or diagnostic is defined by which an implementation says that its login has lapsed. The usual case: an Agent's token expires mid-day and its client, run without a terminal, ends the turn with an error text and no elicitation. The call ends failed, no action is raised, callers keep failing, and no Interface is told to sign in. One builder matches error text; another never raises. The ruled words "A lapsed login raises PND-005" (IN_FORCE.md:189) are then unmet.

**Counterargument.** The next setup's account check would find no account and raise the action, so the gap closes at the next setup. But nothing prompts a setup.

**Least change.** Replace the clause with: "or its implementation, where it carries Conduit's keys, ends a call failed with an error diagnostic whose ruleId is PND-005". A plain implementation reports it only by URL elicitation.

**Precedent.** The protocol's own URL elicitation, the existing way a server says a login is needed; diagnostic.v1's ruleId, already the form of every deciding rule; HTTP 401.

**Who decides.** coordinator. A small finding that adds a wire form for a trigger already ruled (ruling 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/pending.yaml:143. The only wire trigger is URL elicitation: spec/interface/implementation.yaml:429-431 "A URL elicitation at any other point ends the call unknown, raises PND-005 only for a Type stating signIn". "lapsed" appears in spec/ only at pending.yaml:143 and definitions.md:274, and no key, condition or diagnostic carries the report. The ruled words stand unmet (IN_FORCE.md:189 "A lapsed login raises PND-005"). The draft read "an instance's implementation reports, as its Type's contract states" (docs/proposals/spec-batch-1/areas/pending-actions.md:53, non-binding); the promise lost the subject and never gained a form. Not in any known-open list.

**Verifier's corrections.** None. Decider stands (ruling 71 point 4). Order 050 may state each client's way, but the engine's side of the form belongs to the core, as the reviewer says.


## FR-068

**sign-in.v1 that starts from another account cannot tell that account from a new wrong sign-in**

L06/F7 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** PND-010, PND-011

**Where.** spec/promises/pending.yaml:182

**Quote.** "If that check succeeds with no account or another account, the engine invokes sign-in-hook.v1 as a part and relays its URL elicitation under IFC-049; that initial mismatch does not undo the account or end sign-in.v1. [...] a different account is undone by a sign-out-hook.v1 part, sign-in.v1 ends failed naming both accounts"

**Situation.** The instance is signed in as A' and A is expected. The first check sees A' and goes on. The first poll comes with the client's retry on accept, which is consent and not completion, and account-hook still reports A'. Reading A: that is "a different account" during polling, so it is undone and sign-in.v1 ends failed before the person could finish. Every sign-in from a wrong account then fails once, and the sentence on the initial mismatch means nothing. Reading B: the account seen at the first check counts as "no account yet" until it changes. Then a person who signs in again as A' is never detected, and at DL18 the wrong account stays in place, not undone.

**Counterargument.** Under reading A a second sign-in.v1 succeeds from a clean state, so the cost is one failed attempt.

**Least change.** Replace "that initial mismatch does not undo the account or end sign-in.v1" with: "another account found by that check is undone by a sign-out-hook.v1 part before sign-in-hook.v1 is dispatched, as setup undoes one under PND-011; where the undo does not succeed, sign-in.v1 ends failed".

**Precedent.** PND-011's own rule for setup, which undoes a different account before it proceeds.

**Who decides.** coordinator. An ambiguity; the fix reuses setup's existing rule (ruling 37; ruling 46 point 4).

**Verifier.** confirmed, medium. Quotes verbatim at spec/promises/pending.yaml:181-184 and 210-211. The first poll comes at consent: spec/interface/implementation.yaml:1383-1385 "accept means consent, not completion: retries poll under PND-010". pending.yaml:184-185 polls "until an account appears", and an account is already present. The area was touched before: order 038a's verifier found "sign-in.v1 gave two paths for an account found before the person signs in" (docs/work-orders/038a-spec-pending-and-sign-in.md:81), and the sentence on the initial mismatch was the fix. The case at the first poll is what that fix left, and is not recorded.

**Verifier's corrections.** The least change reverses an explicit sentence and removes a recovery. PND-011 has a later account check within sign-in.v1 clear SignInUndone (pending.yaml:217-218), which needs sign-in.v1 to go on from a wrong account whose undo failed. An alternative keeps it: during polling the account that the first check reported counts as no account yet, any other account that is not the expected one is undone, and DL18 ends unknown. Either choice changes stated behavior, so log it for the owner to strike.


## FR-069

**On the engine's own runner, a restart with the run's values cannot hold together with the rule that the process is the one place the value is kept**

L06/F8 · contradiction · verifier: weakened, low · decides: coordinator · new

**Promises.** SEC-018, SEC-014, SEC-017, PRV-010, PRV-003

**Where.** spec/promises/secrets.yaml:458

**Quote.** "The engine's own runner is part of the engine, so this binds it too: the process it starts is the one place where it keeps the value, which is what PRV-011 has a runtime provider's contract state."

**Situation.** SEC-014 (secrets.yaml:373-375): "the engine reads nothing more for it, a restart that its runtime provider makes under PRV-010 reuses the value that the provider holds". PRV-003 has the engine's own runner keep every promise made of a runtime provider, and DL16 gives it a restart policy. A local server given an API key as a variable crashes. To restart it the runner needs the value. The process is gone and was the one place. A copy in the engine's memory is another place, which SEC-018 forbids. A new read is forbidden by SEC-014 and has no caller under SEC-011. Builder A keeps the values in memory for the life of the run. Builder B restarts no run that was given a secret. This is the default runner of the first user, a single developer on one machine.

**Counterargument.** PRV-010 is a may, so never restarting keeps every promise, and DL16 lets the engine say so. The text is coherent if the runner's restart policy is read as never for such runs.

**Least change.** Add to SEC-018's then: "or, on the engine's own runner, that runner's memory for as long as the run lasts, from which alone it restarts the run under PRV-010", as SEC-013 already lets the engine hold a connection's values; or say that the runner restarts no run that was given a secret.

**Precedent.** systemd keeps Environment= and LoadCredential= with the unit across Restart=; Docker keeps the value in the container's configuration, as PRV-011 cites.

**Who decides.** coordinator. A contradiction whose fix chooses between two readings and adds no behavior (ruling 37; ruling 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/secrets.yaml:458-460; secrets.yaml:373-375 as cited. PRV-010 has the level may: spec/promises/providers.yaml:332-333 "Its runtime provider may restart it under its own restart policy, with the values it was given for that run". DL16 leaves the policy to the engine: spec/implementation-defined.yaml:182 "the runner's restart policy, under PRV-010". Where the engine may hold a value the text says so, for a connection only (secrets.yaml:339 "holds it for as long as the connection lasts"). Not recorded in the known-open lists, G5 or the conformance document.

**Verifier's corrections.** Class: ambiguity, not contradiction. PRV-010 is a may, so a runner that restarts no run given a secret keeps every promise. The open point is whether SEC-018's "the one place" forbids the runner's memory for the life of the run, on which two builders of the default runner diverge. Decider (coordinator) and the two offered fixes stand.


## FR-070

**SEC-024 puts a rule that the documents alone answer in the convention layer, so the Language level accepts a secret bound for a non-loopback http URL**

L06/F9 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** SEC-024, SEC-021

**Where.** spec/promises/secrets.yaml:625

**Quote.** "Whether the engine can restart on another machine or run as several parts is how it is configured, as its conformance document states, so the same engine configured otherwise accepts the template, and the rule is of the layer `convention`: the Language level, which has no conformance document, does not apply it."

**Situation.** SEC-024 holds two rules. One depends on configuration: a loopback host on an engine that can restart elsewhere. The other does not: a header bound to a secret, or an item selected for get-secret.v1, at a URL that is neither https nor http on loopback, which every engine rejects however it is configured. LD1's test is "Language if it can be answered by reading the declarations alone" (docs/AUDIT.md:406). A Language-level engine, or an independent build under LD4, resolves a template that sends a token to http://vault.example.com into objects, and every Engine-level engine rejects it. So what a set of documents means differs by level, and an author's offline check passes a template that no engine will take.

**Counterargument.** No Engine-level engine ever sends the secret, so nothing leaks. Keeping one promise for one concern avoids a second ID.

**Least change.** Split it. A new `language` promise: a template that selects a remote item which binds a header to a place that secret-reference.v1 governs, or that it selects for get-secret.v1, at a URL that is neither https nor http at a loopback host, is rejected, checked as SEC-021's is. SEC-024 keeps only the loopback rule for an engine that can restart on another machine.

**Precedent.** LD1's test; SEC-021, the same registration-time check, already of the layer language.

**Who decides.** coordinator. A defect of layer; Engine-level behavior is unchanged (ruling 37; ruling 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/secrets.yaml:625-628. The half that needs no configuration is at secrets.yaml:604-606. LD1's test is at docs/AUDIT.md:406. The layer was settled before: docs/proposals/reaudit/round-5/DECISIONS.md:57 "SEC-024 takes the layer `convention`, as PRV-002 and PRV-013 already reject documents as conventions. Widening what resolving reads would change the Language level." PRV-002 is another rejection from the documents alone that sits in convention (spec/promises/providers.yaml:51-53).

**Verifier's corrections.** Mark known-new-situation, not new: round 5's M05 moved the whole promise to convention, and the new point is that one half needs no configuration. Smaller than stated: Engine-level behavior is unchanged, and the spec keeps other document-only rejections in convention. The split is still sound, also fixes two obligations in one promise, and is the coordinator's.


## FR-071

**Three kinds of document or request that can never be honored are accepted and fail only later, at setup or at the next read**

L06/F10 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** SEC-009, SEC-010, SEC-013, SEC-021, SEC-008, PND-009, PND-010, TPL-019

**Where.** spec/promises/pending.yaml:104

**Quote.** "A Conduit whose consumer is not an Interface cannot be opened listing answer.v1; that open reports refused."

**Situation.** spec/README.md:324 says an engine "given a document it cannot honor rejects the document and says why". (a) A template of an Interface or a Pool may select a remote item (TPL-019, spec/promises/templates.yaml:356-358) whose headers bind a secret. The read needs the instance's own Conduit listing get-secret.v1, which SEC-009 (secrets.yaml:219-222) never instantiates for an Interface, and a Pool is never a consumer. The template registers, and every setup's connection reports unavailable for ever. (b) A Conduit that lists answer.v1 or sign-in.v1 with an Agent or Resource consumer is instantiated and can never be opened (pending.yaml:104, 169); as a declared Conduit it fails every setup of its consumer. (c) Root opens a second Conduit listing get-secret.v1 between the same two ends. SEC-008 permits it, and from then "the read reports refused" (secrets.yaml:251), so the instance's next setup or restart fails with nothing said at the opening.

**Counterargument.** Each failure is reported when it happens, with a deciding rule, and plan and apply would show the Conduits. Late refusal is a consistent convention for access.

**Least change.** Add to SEC-021's when: "or the template is of an Interface or a Pool and the item's headers binds a place that secret-reference.v1 governs". Restate PND-009 and PND-010's clause as SEC-009 does: "is not instantiated". Add to SEC-008: "a request to open one while its consumer has another open Conduit to that secret Resource that lists the operation reports conflict".

**Precedent.** SEC-009 and TPL-019, which reject at registration what could never be set up; Kubernetes admission validation.

**Who decides.** coordinator. Small findings that add rejections, each answerable from the documents or from the Conduits as they stand (ruling 71 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/pending.yaml:104-105. (a) holds: spec/promises/templates.yaml:356-358 lets an Interface or a Pool select `remote` items; secrets.yaml:198-199 and 219-222 allow a reading Conduit only for a Resource or an Agent; definitions.md:63 a Pool "is never a consumer"; no promise rejects the template. (b) holds as described (pending.yaml:104-105, 169-170), and matches an existing pattern: secrets.yaml:207-208 "ACC-032 refuses the opening of a Conduit that lists an operation reserved to the root Interface in the same way". (c) holds: secrets.yaml:250-251 "or of more than one, the read reports refused", and no promise bars a second open Conduit between the same ends. spec/README.md:324-326 forbids accepting a document and ignoring part of it "unsaid".

**Verifier's corrections.** Only (a) is a document that no engine can honor and that registers with no diagnostic: keep it as a small defect. (b) and (c) are not defects. The text can be followed, each failure carries a reason and a deciding rule (ruling 52 is met), and (b) follows ACC-032's late refusal. They are optional earlier checks, small features under ruling 71 point 4. For (b), "is not instantiated" would be a rule of the layer language, as SEC-009 is.


## FR-072

**Since batch 1, rotating a revoked credential by replacing the instance also destroys an Agent's home and login and needs root and a lift**

L06/F11 · needless-complexity · verifier: weakened, low · decides: owner · known-new-situation

**Promises.** SEC-014, PRV-029, LIF-035, LIF-041, ACC-057

**Where.** spec/promises/secrets.yaml:379

**Quote.** "No operation ends the run of an instance that is ready, so through Conduit alone a rotated value reaches a running instance only in another instance made in its place"

**Situation.** Known as a stated limit and as A67, whose restart verb waits on round 6's D09 measure of rotation by replacement (docs/AUDIT.md:475). What those records lack: a leaked key is revoked in its store, which is the ordinary response to a leak. The Agent's run holds a dead value. Replacing the Agent now also removes its state directory "with its contents" (spec/promises/providers.yaml:974), which holds its home and its subscription login, so the person signs in again. It needs root, since only root registers Agents and opens get-secret Conduits. It needs a lift of the tombstones of the Agent and of each of its Conduits. A remote item rotates by a further setup, since "A further setup connects again and reads again" (secrets.yaml:355-356), while a run does not. The file cites Kubernetes secretKeyRef, but there a new Pod keeps its Deployment, volumes and Service; here the instance is both.

**Counterargument.** The owner chose to measure explicit replacement first, and nothing implicit restarting work is a purpose of the language. An operator can also end the process outside Conduit and send a further setup.

**Least change.** Bring A67's verb to the owner now with these costs counted: one operation that stops and asks again for the run of a ready instance, with reads of its own, keeping its ID, Conduits and state directory. Failing that, have D09 count the lost home and login, root's part and the lifts.

**Precedent.** kubectl rollout restart; systemctl restart; docker compose up --force-recreate.

**Who decides.** owner. It adds a feature (action item A67; the brief on stated limits).

**Verifier.** weakened, low. Quote verbatim at spec/promises/secrets.yaml:379-381. Already recorded: rotation by replacement (spec/README.md:539-541); the lifts (README.md:365-367, 392-394); root alone registering Agents (README.md:376-378); A67 open, with "Round 6's D09 measures a credential's rotation by explicit replacement first, in S2 and S10" (docs/AUDIT.md:475); D09 already counts "person interventions" and "recreated relationships" (experiments/013-s2-shared-login/README.md:145). Not recorded: the state directory lost on replacement, spec/promises/providers.yaml:975 "Removal or unregistration of the instance removes it with its contents".

**Verifier's corrections.** Known in all but one part: the one new situation is the lost state directory when the replaced instance is an Agent. Class: a missing feature (A67), the owner's, not needless complexity. The lost login is overstated for an API-key Agent, whose account is null (contracts.yaml:302-303), though its home is still lost. No spec change follows: have D09 count the lost state directory where S2 or S10 replaces an Agent.


## FR-073

**No promise decides what the first-start objects and root's three Conduits hold after the engine is replaced by one with another version of the core package**

L06/F12 · gap · verifier: confirmed, low · decides: owner · new

Same fault, or overlapping: FR-081, FR-130.

**Promises.** FST-002, FST-005, FLEET-005, NAM-009

**Where.** spec/promises/first-start.yaml:41

**Quote.** "They are the declared Conduits `management`, `access` and `system` of the template `root`, instantiated with it under CON-002, from `root-management`, `root-access` and `root-pool`. [...] The `system` Conduit serves as root's default to that Pool, with the root-pool operation list of FLEET-005."

**Situation.** Every FST promise has the when "started for the first time". FST-005 makes each first-start object "an instance of a template that the core package exports, like any other instance", so it keeps what it resolved and holds the core package's version (NAM-009). The core package is built into the engine, and batch 1 itself added sign-in.v1 and answer.v1 to root-pool's list. A store first started before that change and run by a later engine: root's system Conduit lacks answer.v1, so root cannot answer a machine-removal with kept (PND-008, PND-009). No request closes a first-start Conduit (ACC-024), and its local name is taken. Builder A keeps what was resolved. Builder B remakes the Conduits at start. Builder C refuses the store. The only text on later change is "Later capability additions never widen these lists" (spec/promises/fleets.yaml:96).

**Counterargument.** A conformance claim names one version, v0 is a draft, and DL6 leaves an interrupted or repeated first start to each engine. Moving a store between versions may be out of scope on purpose, but no limit says so.

**Least change.** Ask the owner. The smallest text is a stated limit: "This version promises nothing of a store made under another version of the core package." The alternative is one convention promise: at each start the engine brings the first-start objects and root's three Conduits to its own core templates, and records each change.

**Precedent.** The Kubernetes API server reconciles its default RBAC roles and bindings at every start (the annotation rbac.authorization.kubernetes.io/autoupdate).

**Who decides.** owner. It decides behavior and scope on upgrade.

**Verifier.** confirmed, low. Quote verbatim at spec/promises/first-start.yaml:41-44. Every FST when is first start (first-start.yaml:16, 35, 99). first-start.yaml:67 "like any other instance". spec/promises/naming.yaml:145-146: a template keeps the core package's "address and the version alone". spec/promises/fleets.yaml:96-97 "Later capability additions never widen these lists". DL6 covers only an interrupted or repeated first start (spec/implementation-defined.yaml:50-55). spec/README.md:312-313 has a claim name one version. No limit, known-open item or ruling speaks of a store under another core version.

**Verifier's corrections.** Wider than stated: every registered template keeps the core's version under NAM-009, so the gap is what any store holds under an engine with another core, not only the first-start objects. Divergence is less likely than stated, since FST-005's "like any other instance" leans to keeping what was resolved; the hole is then that ACC-024 leaves no way to change root's three Conduits. A stated limit is the least change, and its scope is the owner's.


## FR-074

**PND-002 and PND-007 give two different sets of Interfaces at which a notice is raised**

L06/F13 · ambiguity · verifier: confirmed, low · decides: coordinator · new

**Promises.** PND-002, PND-007, PND-009

**Where.** spec/promises/pending.yaml:31

**Quote.** "It is raised at root and every Interface whose scope contains a subject: the Pool or Fleet itself, the owner of an instance, or an owner of either end of a Conduit. [...] It is raised at root and Interfaces whose scope contains that Fleet."

**Situation.** A notice's subjects are "the Resource and that Fleet's closed Conduits" (pending.yaml:321). The Fleet is not a subject. By PND-002 the notice is raised wherever a scope holds the Resource's Pool or either end's owner, so also at Interfaces scoped to the Pool, which is usually the side that sent the retirement. By PND-007's observable it is raised only where a scope holds the Fleet. Under the first reading a Pool-scoped Interface lists the notice and carries a nonzero unanswered count on every result until expiry, yet cannot answer it, since PND-009 authorizes the answer through the Fleet. Under the second it never sees it.

**Counterargument.** An observable is not the obligation, so PND-002 may be read to govern and PND-007's sentence to be an incomplete example.

**Least change.** In PND-007 make that Fleet the notice's one subject and move the Resource and the closed Conduits into its evidence. PND-002's rule and PND-009's authorization then need no exception.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (ruling 37; ruling 46 point 4).

**Verifier.** confirmed, low. Quotes verbatim at spec/promises/pending.yaml:31-32 and 326. Subjects are "the Resource and that Fleet's closed Conduits" (pending.yaml:321), so PND-002 reaches Interfaces scoped to the Resource's Pool, and PND-007's observable does not. Answering goes through the Fleet (pending.yaml:113-114). spec/README.md:407-408 accepts seeing without answering, but does not say which set holds. Not recorded.

**Verifier's corrections.** None. The coordinator's, as stated.


## FR-075

**A sign-in.v1 request without the URL capability has no stated ending and no stated moment at which it is checked**

L06/F14 · ambiguity · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-040.

**Promises.** PND-010, IFC-049, IFC-036, REC-001

**Where.** spec/promises/pending.yaml:189

**Quote.** "Missing URL capability uses -32021 with requiredCapabilities, not invalid."

**Situation.** Every request ends in one of seven ways and is recorded with how it ended (REC-001). IFC-036 has the engine's other -32021 answer report invalid. Here "not invalid" may mean that the code is not the JSON-RPC invalid-parameters error, or that the reason is not invalid, and no other reason is named. The moment is open too. Reading A: before any part, so a client without the capability cannot use sign-in.v1 even to confirm a login that is already right. Reading B: only when sign-in-hook.v1 returns a URL, by when the hook has begun a login and may hold a fixed port, and the invocation's outcome is then undecided. Records and the effect on the instance differ between builders.

**Counterargument.** IFC-049's "the request must declare" suggests the check is made up front, and a caller sees the same protocol error either way.

**Least change.** "Before any part is dispatched, a sign-in.v1 request that does not declare URL elicitation is answered with -32021 and reports invalid, as IFC-036 has a request that does not declare Conduit's extension report."

**Precedent.** IFC-036.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (ruling 37; ruling 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/pending.yaml:189. spec/interface/implementation.yaml:1359-1361 "For sign-in.v1 the request must declare ... without it the engine answers -32021 with data.requiredCapabilities naming URL elicitation, sending no elicitation". IFC-036's -32021 "reports invalid" (implementation.yaml:990-992). Every request needs an ending: spec/conventions/records.yaml:25-26 "its outcome, or its reason where it has no outcome". The first check can succeed with no URL at all (pending.yaml:179-181). The source words are in the order (docs/work-orders/038a-spec-pending-and-sign-in.md:45-46, non-binding). Not recorded.

**Verifier's corrections.** The unnamed reason is the firm half. The moment is the weaker half: IFC-049's "the request must declare" favors a check before any part. The least change's "reports invalid" reverses the text's "not invalid" unless that phrase is read as the wire code; say which when logging it.


## FR-076

**PND-013 holds the duties of three parties in one convention promise, and those of the Interface's client fall under no conformance level**

L06/F15 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** PND-013, PND-001, PND-004, PND-010

**Where.** spec/promises/pending.yaml:287

**Quote.** "An Interface carries a return only from a loopback listener it opened for that one login, or what the person typed, never model output. It opens a listener only where it knows it does not run on the machine of the node signInLogin names"

**Situation.** The form asks for one checkable obligation per promise. PND-013 is some seventy lines: the engine's five ordered checks and its rebuilding of the URL (convention); what the hook's implementation must do with the value (the Implementation level, as SEC-006 and SEC-019 are, and wire keys of the interface layer); and what the person's client must do. The three levels are Language, Engine and Implementation (spec/README.md:318-322). A consumer's client is none of them, so the listener rules bind nobody who can claim conformance, while the promise's safety argument rests on "never model output". PND-001's when, "Something waits on a person", names no situation a test can set up, and its then has five obligations. No promise of the file has a vector. A builder cannot tell which sentences bind an engine.

**Counterargument.** The limits already say that the engine cannot tell where a return came from and that a model-driven console could fill it, so the client's duties are known to be unenforced. Keeping the flow in one place helps a reader follow it.

**Least change.** Split PND-013: the engine's checks as a convention promise; the hook's delivery as an interface promise that binds the implementation; the client's duties moved to the limits as statements, or a conformance level named for a client. Give PND-001 the when "The engine raises a pending action under PND-005 to PND-008".

**Precedent.** RFC 8252 states the loopback-redirect duties as the native app's own; POSIX keeps separate conformance classes for an implementation and an application.

**Who decides.** coordinator. A defect of form whose fix moves text and adds no behavior (ruling 37; ruling 46 point 4). A new conformance level for clients would be the owner's.

**Verifier.** weakened, low. Quote verbatim at spec/promises/pending.yaml:287-289; PND-013 runs from 226 to 294. Form rule: spec/README.md:297 "One obligation per promise". The levels name no consumer's client (README.md:314-322). Already recorded: the client's unenforced duties (README.md:402-405; README.md:420 "A model-driven console could fill signInReturn; the engine cannot tell its source") and the missing vectors (README.md:219 "no vectors yet"; docs/guides/cold-readers.md:66-67).

**Verifier's corrections.** Two parts are known: no vectors, and client duties that nothing enforces. What is new is form only: three parties' duties in one promise, no conformance level for a consumer's client, and PND-001's when. No behavior diverges. Moving the client's duties to the limits is the coordinator's; a new conformance level is the owner's, as the reviewer says.


## FR-077

**SEC-001 does not say whether additionalProperties, patternProperties and prefixItems are among the applicators under which secret-reference.v1 is rejected**

L06/F16 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-082, FR-125.

**Promises.** SEC-001, OBJ-021, MAN-017

**Where.** spec/promises/secrets.yaml:50

**Quote.** "and a schema that puts it under any other applicator, or beside any other branch, is rejected."

**Situation.** The profile's applicators include prefixItems, patternProperties and additionalProperties (spec/schema-profile.md:70-71). The observable lists as rejected only allOf, not, if, then, else, dependentSchemas and contains. A Type's schema {type: object, additionalProperties: {$ref: secret-reference.v1}}, a map of named secrets: reading A rejects it as "any other applicator"; reading B accepts it, because for each member that subschema is "the schema of the place itself". The package installs on one engine and is rejected by another. No literal can slip through either way, since the schema accepts only reference maps.

**Counterargument.** OBJ-021 names only the place itself, items, and anyOf or oneOf, and SEC-001 says "only", so a careful reader may settle on rejection.

**Least change.** Name them in the observable: "So is a schema that puts it under additionalProperties, patternProperties or prefixItems." This agrees with MAN-017, under which no item's pointer can lead to such a place.

**Precedent.** JSON Schema 2020-12's applicator vocabulary, which the profile lists.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (ruling 37; ruling 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/secrets.yaml:50-51. The observable names as rejected only "`allOf`, `not`, `if`, `then`, `else`, `dependentSchemas` or `contains`" (secrets.yaml:65-66). The profile's applicators also hold `prefixItems`, `patternProperties`, `additionalProperties` and `propertyNames` (spec/schema-profile.md:70-72). OBJ-021 has the same open wording for every reference schema (spec/promises/objects.yaml:370-373). No promise or first-build settlement decides it; findings 1-04 and 1-17 are on patterns only.

**Verifier's corrections.** The fix's support is wrong for additionalProperties. MAN-017 lets a pointer lead to a place that `required` names and `additionalProperties` describes (spec/promises/manifest.yaml:385-386 rejects only one that "neither `properties` nor `additionalProperties` describes"), so rejecting that form would remove a usable one. The choice is open, and OBJ-021 needs the same sentence. `propertyNames` is a fourth unnamed applicator. Still the coordinator's.


## FR-078

**A secret reference is accepted in places from which nothing can ever read it, with no diagnostic**

L06/F17 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** SEC-001, SEC-010, SEC-013, MAN-017, SEC-007

**Where.** spec/promises/secrets.yaml:47

**Quote.** "`secret-reference.v1` governs a place only as the schema of the place itself, as the `items` of a list that holds the value, or as a branch of `anyOf` or `oneOf` whose every other branch is `{type: null}`"

**Situation.** The engine reads a secret only for a place that a pointer of the item's environment, packageArguments or headers leads to (SEC-010, SEC-013). MAN-017 has that place required by the Type's schema and rejects a pointer to a list (spec/promises/manifest.yaml:383-387), so no pointer reaches an item of a list. SEC-001 also admits the schema in an Operation's inputSchema and a Requirement's input (secrets.yaml:63-64), where an implementation receives {kind: Resource, id} and can read nothing, since SEC-007 refuses every instance's request. An author writes tokens as a list of secret references, or a configuration place that no selected item binds. The document registers, the references resolve, and no value is ever delivered or reported missing.

**Counterargument.** A reference that is stored and sent is not ignored: a Conduit-aware implementation may use the ID as a name. OBJ-021 treats every reference schema alike, and an exception costs a sentence.

**Least change.** Drop "as the `items` of a list that holds the value" from SEC-001, so that a secret reference stands only where MAN-017 lets a pointer lead. Or keep it and return a diagnostic that does not reject when a template's selected items bind no pointer to a place that secret-reference.v1 governs.

**Precedent.** Kubernetes admits secretKeyRef only at the fixed places from which a value is delivered (env valueFrom, envFrom, volumes).

**Who decides.** owner. The first fix removes a feature. The diagnostic alone is a coordinator's small finding under ruling 71 point 4.

**Verifier.** weakened, low. Quote verbatim at spec/promises/secrets.yaml:47-48. The half on operation inputs is refuted: a control's input carries a secret reference into configuration, where it is read (secrets.yaml:578-580 "a place in the configuration that a control has changed since is read at the connection of a later setup"; secrets.yaml:67-68 on `arguments`). MAN-017 does reject a pointer to a list (spec/promises/manifest.yaml:383-384). A reference that no item binds is not ignored unsaid: it is resolved, stored and sent as configuration (spec/promises/objects.yaml:376-379).

**Verifier's corrections.** What remains is small: list items and Requirement inputs that nothing reads, which follow from OBJ-021's one rule for every reference schema; and no diagnostic where a selected item binds no pointer to a place that secret-reference.v1 governs. Not needless complexity of any weight. The diagnostic that does not reject is an optional small feature under ruling 71 point 4; dropping the list form is the owner's and gains little.


## FR-079

**"Anything touching a secret remains root's" names no test, and can be read against SEC-011**

L06/F18 · ambiguity · verifier: confirmed, low · decides: coordinator · new

**Promises.** PND-003, PND-009, SEC-011

**Where.** spec/promises/pending.yaml:55

**Quote.** "Enrollment, machine and machine-removal are answered under PND-008 at root alone; anything touching a secret remains root's."

**Situation.** PND-009 adds "Root alone answers an action whose answer itself touches a secret" (pending.yaml:116). No documented kind has such an answer: a Resource's sign-in is already root's under PND-010, and the enrollment kinds under PND-008. Reading A: the clause decides nothing today. Reading B: a builder makes root-only any answer for an instance that consumes a get-secret Conduit, such as a scoped Interface's sign-in of such an Agent, or its further setup, which answers a setup action retried and makes the engine read secrets with that Interface as caller. Reading B contradicts SEC-011, under which "who set the instance up does not change what it reads" (secrets.yaml:286).

**Counterargument.** PND-009 already says that a setup action touches a secret only through its setup operation, which points to reading A, and the clause may be kept for kinds that batch 2 adds.

**Least change.** Delete both clauses, or replace them with the cases they cover: "the sign-in of a Resource (PND-010) and the enrollment kinds (PND-008) are root's".

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (ruling 37; ruling 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/pending.yaml:54-56; second clause at pending.yaml:115-116. The words are the owner's: docs/proposals/ruling-66/README.md:30-31 "enrollment of machines and anything touching a secret stay root's"; IN_FORCE.md:186. Against reading B: spec/promises/secrets.yaml:286-287 "who set the instance up does not change what it reads"; pending.yaml:52 "decided under ordinary access using the Conduits the actor holds"; ruling 66 point 2 has manage cover setting up (ruling-66/README.md:23-24). Not recorded as open.

**Verifier's corrections.** Reading B is nearer the text than stated: PND-009 says a setup action touches a secret "through its setup operation" (pending.yaml:115), and a further set-up.v1 answers retried (pending.yaml:306), so the two sentences can be read to make that answer root's. The clause carries ruling 66 point 3's words, so deleting it is not the coordinator's; restating it as the cases it covers is.


## Further points the verifier noticed

Not verified by a second reader.

- **pending-action.v1 has no `state` member, though two promises have tools list one** (spec/core/results.yaml:573-611; spec/promises/pending.yaml:353; spec/promises/enrollment.yaml:78). The schema's members are id, kind, subject, evidence, expiry and answer, with additionalProperties false (results.yaml:611). PND-008 says "Tools list only ID, kind, subject, expiry and state", and ENR-003 says the same. A builder either breaks the schema or leaves the state out. The reviewer named this in the assessment and did not raise it. A defect, the coordinator's: add the member, or say that state is read from whether `answer` is present.
- **"holds back all model work" does not say whose** (spec/promises/pending.yaml:216-217). PND-005 holds work "on that instance" (pending.yaml:148-149). PND-011 says "all model work" with no instance named, and the limit repeats it (spec/README.md:411-412). One builder holds the instance's work; another, reading "all", holds every Agent's. F2 depends on the answer: for a Resource the first reading holds nothing. An ambiguity, the coordinator's, to settle together with F2.

## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only. Nothing was built or run.

Read in full: spec/promises/pending.yaml, secrets.yaml and first-start.yaml. Read in part: the cited promises (IFC-016, IFC-036, IFC-045, IFC-049, ACC-011, ACC-019, ACC-024, OPS-023, REC-001, IMP-001 to IMP-008, TPL-012, TPL-019, MAN-017, OBJ-021, CTR-013, FLEET-005, NAM-009, LIF-040, LIF-041, PRV-002, PRV-003, PRV-010, PRV-011, PRV-013, PRV-029, ENR-003 to ENR-005), the sign-in operations and pending-action.v1 in spec/core, DL6 and DL13 to DL19, and spec/README.md from the form of a promise through the limits. For novelty I searched docs/AUDIT.md "Known to be open", cold-readers.md, IN_FORCE.md topics 9 and 12, the full-review brief's queued list, the first-build logs and review list, G5.md, the ledger, and the records of rulings 56, 66 and 71, round 5's decisions and the secrets proposal. Plans and work orders were used only to test novelty and intent, never as what holds.

Every quote is verbatim at its cited line. Outcome: 9 confirmed (F1, F2, F5, F6, F7, F12, F13, F14, F16, F18 is the tenth), 8 weakened (F3, F4, F8, F9, F10, F11, F15, F17), none refuted outright. Correct count: 10 confirmed, 8 weakened.

The two unsafe marks that hold are F1 and F2. F3's and F4's do not hold as stated: F3 because the promises do name root as the panel's audience and later checks bound the harm; F4 because SEC-013 and SEC-014 decide the case expressly and a ruled record (the secrets proposal under ruling 41) holds the principle. Both still carry a real question, F4's for the owner.

Three of the reviewer's least changes need care before use: F7's removes the recovery from a failed undo, F16's rests on a wrong reading of MAN-017 for additionalProperties, and F1's also lifts the hold on a setup's check, which is added behavior.

Not checked: whether the engine or any vector already takes one of the readings; the cited RFC sections; the Model Context Protocol's own text on -32021.
