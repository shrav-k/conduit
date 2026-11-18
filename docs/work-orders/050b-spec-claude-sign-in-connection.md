# Work order 050b: the connection Conduit manages for a Claude sign-in, F95, the three stale route texts and Codex's browser-opens limit, in the specification

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since this order adds and changes no case. This
order has network access only because the engine's tests bind loopback ports, which the sandbox
refuses without it; use it for nothing else, and reach no other machine. Run one cargo command at
a time. Never print a whole large file or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `claude`, `codex` or
`client-facts`, and outside your working tree read nothing under your home directory except
Cargo's and rustup's own files; never a login file or a keychain item.

## What and why

[Ruling 80](../proposals/ruling-80/README.md), point 8, amended ruling 75 C3: Conduit manages a
Claude Agent's sign-in. It computes that the Agent must sign in, shows the person the login page
through the person's Interface, and opens, limits and closes the connection, the tunnel or SSH
session, over which the person completes it, by remote desktop or by SSH, pasting the code or
forwarding the port; it never reads, holds or relays the credential that the sign-in returns.
Three texts in `spec/` still say what ruling 75 C3 said, and [IN_FORCE.md](../../IN_FORCE.md),
topic 9 (line 217, which the review's record cites as line 216), owes their rewrite to order 050.

The targeted review of the 0.2.0 sign-in surface then gave that connection its design, as
conditions with tests and no Type or core member, scoped to logins stating `carried: false`
([its record](../proposals/review-signin-2026-11-16/README.md#1-the-open-line-clause-by-clause),
line o2), and [ruling 82](../proposals/ruling-82/README.md#the-decisions), point 6, accepted the
record's lines o1 to o7. Three of them reach `spec/`:

- **o2**, the connection's conditions, written as the record's
  [section 6](../proposals/review-signin-2026-11-16/README.md#050-the-connection-conduit-manages-for-a-claude-sign-in)
  lists them under "050, the connection Conduit manages for a Claude sign-in": item 1, its scope
  and its ends; item 2, when a forward opens and closes, and a port that cannot be bound; item 3,
  that no engine or Interface process holds the code-page login's code; and item 4, the three
  stale texts (the review's verdicts
  [M74 to M76](../proposals/review-signin-2026-11-16/verdicts.md#i-core-contracts-and-spec-text-the-surface-is-fixed-against)).
- **o3**, Codex's browser-opens limit: on the engine's own runner, Codex's browser login may open
  the default browser of the engine user's graphical session, where one exists. It is stated in
  the limits and in the accounts guide, not in a description; the account check stays the only
  guard, and FR-118 is not raised for it.
- **o5**, F95's named spec fix, landed with order 050's rewrite of the limits: the limits and
  IFC-049's observable gain the review's sentence, "The connection Conduit manages for a Claude
  sign-in does not bind the person who completes the login; the expected account (PND-011)
  remains the only binding." ([carriers outside the held
  orders](../proposals/review-signin-2026-11-16/README.md#carriers-outside-the-held-orders);
  verdict M90).

The coordinator split order 050 into orders 050b to 050o. This order, 050b, is the part that lives
in `spec/`: it needs no code, so it lands on `main` on its own, before the packages, and the
orders that build and document the connection (050o, the Interface's build and its tests; 050n,
the packages' READMEs and the accounts guide) cite text in force. The rest of 050 is packages and
code and is not this order's.

**The coordinator's decision on the form.** The conditions are stated in IFC-049's `observable`
and in the limits, not as a new promise. IFC-049 already holds the relay that the conditions start
from, and PND-013's `observable` already states what an Interface's listener does, so an
Interface's part in a sign-in has a place there. The coordinator checked that each condition can
be tested without a promise of its own: order 050o's three tests (the record's P-R3-d, a canary
redirect over the connection appearing in no engine, Interface or provider log, record or
diagnostic, with the forwarded port's sockets belonging to the route's own programs; a scripted
fixed-port sign-in showing the forward absent before the relay, present during the wait and gone
after the end; and a canary page code in no engine or Interface result, record, diagnostic or log)
cite IFC-049. They observe the person's side and the processes there, which no portable case
under `conformance/` can (D36), so they are tests of the shell crate, as 050o plans them, not
cases. F95's sentence is a negative statement that the record gives no test, and it needs none.
So no condition is listed as open, and no promise ID is added.

What does not change, and why:

- **No case and no vector.** The change adds no promise ID, so the rule that a new promise lands
  with a case that can fail ([CONTRIBUTING.md](../../CONTRIBUTING.md#writing-promises), rule 8)
  does not apply. No case cites IFC-049 (`engine/STANDING.md`: "no case, no reason"), and
  `sign-in.v1` is a stub in the first build that answers unavailable (DL18), so no case asserts
  anything these words change. There is no declaration file and no entry in
  `conformance/expected-failures.yaml`.
- **`engine/STANDING.md`.** It is generated from each promise's ID, layer and level, the coverage
  index, the expected failures, the S1 rows and the engine's sources, none of which changes. Run
  `python3 tools/promise_standing.py --write` all the same (decision 6); it must leave the file as
  it is.
- **The model-checking account and the safety catalogue.** IFC-049 is an interface-layer promise
  and has no row in `experiments/012-model-checking/account.yaml`. The safety catalogue's IFC-049
  row quotes "The URL is HTTPS only, contains no credential or personal data and is not
  pre-authenticated", which stays word for word.
- **Other texts.** `spec/core/README.md` line 507, "A plain hook and carried: false accept no
  return.", stays true and stays. PND-013, PND-014 and CTR-013 are cited, not changed. The
  accounts guide's half of o3 is order 050n's. IN_FORCE.md's line 216 stays: its *Spec pending*
  part names the Agent Types' logins, which the packages state. Its line 223 stays too: it names
  the code-page login that `claude-code.v2` states, which is order 050k's.

**Who decided what.** Lines o2, o3 and o5 are the owner's, accepted in ruling 82 point 6, with the
record's words. F95's sentence is the review's must-state text, written word for word, its word
"Claude" included. Everything else is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the split of 050 and this order's share; the form
above; the wording of the conditions and of the three rewritten texts, which says only what the
record's lines say and names no software and no protocol beyond ruling 80 point 8's own route
words, remote desktop and SSH (redirect_uri is already IFC-049's); carrying o3's limits half here
rather than with 050n; and the IN_FORCE.md lines. The coordinator also corrects the plan's place
for the core carried description, which it gave as `spec/core/README.md` near line 507, to
`spec/schemas/contracts.yaml`, per verdict M74; and the plan's line for order 050o that removes
050b's expected-failure entry falls away, since 050b adds none (the coordinator amends the plan's
050o entry when it writes 050o). Report them under `decisions` as the coordinator's.

**Order 048a-1.** It is on main and not yet dispatched, so its change has not landed, and it deletes
F97's sentence from the same IN_FORCE.md bullet (line 228) that holds F95's. Decision 5 handles both
states. If this order merges first, the coordinator amends 048a-1's decision 8 and its two checks
that name F95's sentence (the `grep -c` for "F95, by a named spec fix landed with 050", listed to
print 1, and the one for the *Spec pending* line ending "(82 point 6, o5).") before 048a-1 is
dispatched. Line numbers below are those of `main` at `cbb49fc1`; find each place by the text quoted
with it.

## The decisions it applies

Folded `>-` text keeps its indent and lines under 97 characters. Everything a decision does not
name stays word for word ([CONTRIBUTING.md](../../CONTRIBUTING.md#writing-promises), "Writing
promises", rules 1 and 7).

1. **The core `carried` description** (M74). In `spec/schemas/contracts.yaml`, the Schema
   `sign-in-login.v1`, the `description` of `carried` (lines 420 to 424): replace its second
   line,

   ```text
           False completes only with a browser on the program's machine (CTR-013, PND-013).
   ```

   with these four, at the same eight-space indent:

   ```text
           False carries none: the person completes the login on the program's machine, at it or
           over a connection Conduit manages, by remote desktop or by SSH, pasting the code or
           forwarding the port, and no process running Conduit's code reads, holds or relays the
           return (CTR-013, PND-013, IFC-049).
   ```

   Change nothing else in that file: not its header comment, not the Schema's other members.

2. **IFC-049's message for `carried: false`** (M75). In `spec/interface/implementation.yaml`,
   IFC-049's `then`, replace line 1385,

   ```text
       panel. For carried: false it says to open the URL in a browser on that node's machine.
   ```

   with these three:

   ```text
       panel. For carried: false it says that nothing comes back through the Interface, and that
       the person completes the login on that node's machine, at it or over a connection Conduit
       manages, by remote desktop or by SSH, pasting the code or forwarding the port.
   ```

   The rest of `then` stays word for word.

3. **IFC-049's `observable` gains the conditions and F95's sentence** (o2, items 1 to 3; o5).
   After its last line, line 1437, which ends "receives a sign-in's URL, login metadata or return
   (PND-002, IFC-016).", and before `vectors: []`, add these lines, at the same four-space
   indent:

   ```text
       Where Conduit manages the connection over which a person completes a login stating
       carried: false (spec/README.md), the connection meets these conditions; nothing here
       changes a login stating carried: true. The channel that carries the return ends at each
       end in a program that is not Conduit's: the node's own server for the route, and that
       route's client on the person's side. No process running Conduit's code decrypts,
       terminates or copies it, and no engine, Interface or provider log, record or diagnostic
       holds what passes over it. A forward opens only after the engine relayed this sign-in's
       URL under this rule, for the port its redirect_uri names, to that node's loopback only,
       and closes when sign-in.v1 ends, signed in, failed or unknown at DL18. Where the local
       port cannot be bound, the Interface says so, naming the port, and opens no forward on
       another port. A Claude Agent's paste route, the person's own delivery, on the node, of the
       return or of a page login's code, is held by no engine or Interface process, and no
       result, record, diagnostic or log of either holds a page login's code, as PND-014 holds
       for a carried value. The connection Conduit manages for a Claude sign-in does not bind the
       person who completes the login; the expected account (PND-011) remains the only binding.
   ```

   IFC-049's `id`, `layer`, `level`, `when` and `vectors: []` stay. No other promise in `spec/`
   changes, and no promise ID is added.

4. **The limits, "Pending actions, sign-in and the network"** (M76; o2; o3; o5). In
   `spec/README.md`, replace the paragraph of lines 433 to 440, from "In v0 the person's Interface
   runs on the engine's machine." to "within PND-010's scoped authority (ruling 75 C2).", with
   these three paragraphs, separated by one blank line each:

   ```markdown
   In v0 the person's Interface runs on the engine's machine. From another machine or a phone,
   the person reaches it over SSH and pastes the return, or uses a device code. A phone
   Interface is v1's; root on another machine waits until Agents are confined (ruling 75 C4, C5).
   Claude Code Agent Types state carried: false in v0, because Anthropic's terms forbid
   intermediating Claude.ai credentials: Conduit never reads, holds or relays the credential that
   such a sign-in returns. The person completes the login on the Agent's own machine, at it or
   over a connection that Conduit opens, limits and closes, the tunnel or SSH session, by remote
   desktop or by SSH, pasting the code or forwarding the port (ruling 80 point 8, amending ruling
   75 C3; IFC-049). This is to be revisited in v1 (ruling 75 C3). The Interface receiving a
   sign-in is root in practice in v0, within PND-010's scoped authority (ruling 75 C2).

   That connection serves only logins stating carried: false; nothing changes for carried: true.
   The channel that carries the return ends at each end in a program that is not Conduit's: the
   node's own server for the route, and that route's client on the person's side. No process
   running Conduit's code decrypts, terminates or copies it. A forward opens only after IFC-049
   relayed this sign-in's URL, for the port its redirect_uri names, to that node's loopback only,
   and closes when sign-in.v1 ends; where the local port cannot be bound, the Interface says so,
   naming the port, and opens no forward on another port. A Claude Agent's paste route is the
   person's own delivery, on the node, of the return or of a page login's code, held by no engine
   or Interface process (IFC-049). The connection Conduit manages for a Claude sign-in does not
   bind the person who completes the login; the expected account (PND-011) remains the only
   binding.

   On the engine's own runner, Codex's browser login may open the default browser of the engine
   user's graphical session, where one exists. The hook still relays the printed URL under
   IFC-049, and the account check (PND-011) stays the only guard.
   ```

   The two route sentences go: "An SSH forward of a redirect's port is the person's own and
   carries nothing through Conduit." and "those Agents sign in only with a browser on their own
   machine". Every other sentence of the old paragraph stays, in the order shown. The paragraph
   after it, from "Network membership grants nothing.", stays where it is, one blank line after
   the third new paragraph. Nothing else in `spec/README.md` changes.

5. **IN_FORCE.md, topic 9.** Put no date in any line: `check_rulings.py` refuses one. Keep each
   line's indent.
   - Line 217, the sub-bullet that begins `    - *Spec pending, ruling 80, order 050*: Conduit
     manages a Claude Agent's sign-in.`, becomes exactly this one line, its four-space indent
     kept:

     ```markdown
         - **In spec/**: ruling 80 point 8, in the limits in spec/README.md, "Pending actions, sign-in and the network", IFC-049's message and observable, and the core `carried` description. Conduit manages a Claude Agent's sign-in: it computes that the Agent must sign in, shows the person the login page through the person's Interface, and opens, limits and closes the connection, the tunnel or SSH session, over which the person completes it, by remote desktop or by SSH, pasting the code or forwarding the port. It never reads, holds or relays the credential that the sign-in returns (75 C3). Order 050o builds the connection (80 point 8, amending 75 C3).
     ```

   - Line 225, the bullet that begins `- *Spec pending, order 050*: the connection Conduit manages
     for a Claude sign-in`, becomes exactly this one line:

     ```markdown
     - **In spec/**: the managed connection's conditions, in IFC-049's observable with the limits in spec/README.md, for logins stating `carried: false` only: its channel ends at each end in a program that is not Conduit's; its forward opens only after IFC-049's relay, for the port the redirect_uri names, to that node's loopback, and closes when `sign-in.v1` ends; a port that cannot be bound is reported, naming it, with no forward on another; and a Claude Agent's paste route, the return or a page login's code, is held by no engine or Interface process. They are conditions with tests, with no promise, Type or core member of their own; order 050o builds the connection and its tests (82 point 6, o2; 80 point 8).
     ```

   - Line 226, the bullet that begins `- *Spec pending, order 050*: on the engine's own runner,
     Codex's browser login`, becomes exactly this one line:

     ```markdown
     - **In spec/**: on the engine's own runner, Codex's browser login may open the default browser of the engine user's graphical session, where one exists: a limit in spec/README.md, not in the descriptions. The hook still relays the printed URL under IFC-049, the account check (PND-011) stays the only guard, and FR-118 is not raised for it. The accounts guide states it with order 050n (82 point 6, o3).
     ```

   - Line 228, the bullet that begins `- *Spec pending*:` and holds "F95, by a named spec fix
     landed with 050's rewrite of the limits". If it still holds F97's sentence (`grep -c 'F97, by
     order 048a-1' IN_FORCE.md` prints 1), replace in it exactly the text from "with one loader
     vector. F95, by a named spec fix" to its end, "remains the only binding (82 point 6, o5).",
     with "with one loader vector (82 point 6, o5).", so that the bullet holds F97's sentence
     alone. If order 048a-1 has removed F97's sentence (that `grep` prints 0), the bullet holds
     F95's sentence alone: delete the whole line. Say in your report which state you found.
   - Then add this bullet, one line, at the same level, right after the last of the bullets that
     begin `- **In spec/**: F9` and follow that bullet's place (today only F96's, line 229; F97's
     too where order 048a-1 added it):

     ```markdown
     - **In spec/**: F95: the limits in spec/README.md and IFC-049's observable say that the connection Conduit manages for a Claude sign-in does not bind the person who completes the login, and that the expected account (PND-011) remains the only binding (82 point 6, o5).
     ```

   Change no other line of IN_FORCE.md. Topic 15's *Coordinator's* line, which names F95 among
   the decisions ruling 82 settled, stays.

6. **The standing table.** Once decisions 1 to 4 are done, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks for any change to a promise;
   never edit `engine/STANDING.md` by hand. It must leave the file unchanged. If it changes the
   file, keep the change, report every changed row, and say so as a finding.

## Scope

Change only `spec/schemas/contracts.yaml` (the `carried` description), `spec/interface/implementation.yaml`
(IFC-049's `then` and `observable`), `spec/README.md` (the one paragraph of the limits) and
`IN_FORCE.md` (topic 9, decision 5's lines). Run `tools/promise_standing.py --write`, which must
change nothing. Nothing under `conformance/`, `engine/` (beyond what that tool writes),
`packages/`, `tools/`, `experiments/` or `docs/`; no declaration file. Not
`spec/definitions.md`, `spec/core/`, `spec/promises/`, the accounts guide, the review's record or
the ruling's record. Report anything else that seems to need changing.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_private_paths.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
git diff --check
```

Each of the `tools/` checks ends with a line that ends `Errors: 0`; `git diff --check` prints
nothing. `check_model_account.py` prints the same counts as on your starting tree.

From `engine/`, after the change to `spec/`, as AGENTS.md asks:

```bash
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

Every `test result:` line it prints reads `ok` with `0 failed`. Report the last one. The threads
are 4 because at 24 the runner ran out of memory (order 071's Record).

Each of these prints 1. The `tr` checks read the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
tr -s ' \n' '  ' < spec/README.md | grep -c 'intermediating Claude.ai credentials: Conduit never reads, holds or relays the credential that such a sign-in returns\. The person completes the login on the Agent'"'"'s own machine'
tr -s ' \n' '  ' < spec/README.md | grep -c 'That connection serves only logins stating carried: false; nothing changes for carried: true\.'
tr -s ' \n' '  ' < spec/README.md | grep -c 'and opens no forward on another port\. A Claude Agent'"'"'s paste route is the person'"'"'s own delivery'
tr -s ' \n' '  ' < spec/README.md | grep -c 'The connection Conduit manages for a Claude sign-in does not bind the person who completes the login; the expected account (PND-011) remains the only binding\.'
tr -s ' \n' '  ' < spec/README.md | grep -c 'Codex'"'"'s browser login may open the default browser of the engine user'"'"'s graphical session, where one exists\.'
tr -s ' \n' '  ' < spec/schemas/contracts.yaml | grep -c 'False carries none: the person completes the login on the program'"'"'s machine'
grep -c '^    - \*\*In spec/\*\*: ruling 80 point 8, in the limits in spec/README.md' IN_FORCE.md
grep -c '^- \*\*In spec/\*\*: the managed connection'"'"'s conditions, in IFC-049'"'"'s observable' IN_FORCE.md
grep -c '^- \*\*In spec/\*\*: on the engine'"'"'s own runner, Codex'"'"'s browser login' IN_FORCE.md
grep -c '^- \*\*In spec/\*\*: F95: the limits in spec/README.md and IFC-049'"'"'s observable' IN_FORCE.md
```

Each of these prints 1 too. They match each of decision 5's four new lines whole, so a line that
keeps the opening words and changes the rest fails:

```bash
grep -cxF -e '    - **In spec/**: ruling 80 point 8, in the limits in spec/README.md, "Pending actions, sign-in and the network", IFC-049'"'"'s message and observable, and the core `carried` description. Conduit manages a Claude Agent'"'"'s sign-in: it computes that the Agent must sign in, shows the person the login page through the person'"'"'s Interface, and opens, limits and closes the connection, the tunnel or SSH session, over which the person completes it, by remote desktop or by SSH, pasting the code or forwarding the port. It never reads, holds or relays the credential that the sign-in returns (75 C3). Order 050o builds the connection (80 point 8, amending 75 C3).' IN_FORCE.md
grep -cxF -e '- **In spec/**: the managed connection'"'"'s conditions, in IFC-049'"'"'s observable with the limits in spec/README.md, for logins stating `carried: false` only: its channel ends at each end in a program that is not Conduit'"'"'s; its forward opens only after IFC-049'"'"'s relay, for the port the redirect_uri names, to that node'"'"'s loopback, and closes when `sign-in.v1` ends; a port that cannot be bound is reported, naming it, with no forward on another; and a Claude Agent'"'"'s paste route, the return or a page login'"'"'s code, is held by no engine or Interface process. They are conditions with tests, with no promise, Type or core member of their own; order 050o builds the connection and its tests (82 point 6, o2; 80 point 8).' IN_FORCE.md
grep -cxF -e '- **In spec/**: on the engine'"'"'s own runner, Codex'"'"'s browser login may open the default browser of the engine user'"'"'s graphical session, where one exists: a limit in spec/README.md, not in the descriptions. The hook still relays the printed URL under IFC-049, the account check (PND-011) stays the only guard, and FR-118 is not raised for it. The accounts guide states it with order 050n (82 point 6, o3).' IN_FORCE.md
grep -cxF -e '- **In spec/**: F95: the limits in spec/README.md and IFC-049'"'"'s observable say that the connection Conduit manages for a Claude sign-in does not bind the person who completes the login, and that the expected account (PND-011) remains the only binding (82 point 6, o5).' IN_FORCE.md
```

`grep -c 'F97, by order 048a-1' IN_FORCE.md` prints the value it printed before the change: 1 in
state A, where line 228 kept F97's sentence, and 0 in state B, where order 048a-1 had removed it.
In state A, `grep -c 'with one loader vector (82 point 6, o5)\.$' IN_FORCE.md` prints 1.

This prints `True True True True`. It reads IFC-049 and the `carried` description with their line
breaks as single spaces, and finds that IFC-049's `then` holds decision 2's text and not the old
sentence, that its `observable` ends with decision 3's text, that the `carried` description is
its kept first line followed by decision 1's text, and that every other promise in
`implementation.yaml` equals its version at `HEAD`:

```bash
/usr/bin/python3 -c '
import subprocess, yaml
n = lambda s: " ".join(s.split())
new = {x["id"]: x for x in yaml.safe_load(open("spec/interface/implementation.yaml", encoding="utf-8"))}
old = {x["id"]: x for x in yaml.safe_load(subprocess.run(["git", "show", "HEAD:spec/interface/implementation.yaml"], capture_output=True, text=True, check=True).stdout)}
p = new["IFC-049"]
d2 = ("For carried: false it says that nothing comes back through the Interface, and that the "
      "person completes the login on that node\x27s machine, at it or over a connection Conduit "
      "manages, by remote desktop or by SSH, pasting the code or forwarding the port. No text "
      "from the hook is relayed.")
d3 = ("receives a sign-in\x27s URL, login metadata or return (PND-002, IFC-016). Where Conduit "
      "manages the connection over which a person completes a login stating carried: false "
      "(spec/README.md), the connection meets these conditions; nothing here changes a login "
      "stating carried: true. The channel that carries the return ends at each end in a program "
      "that is not Conduit\x27s: the node\x27s own server for the route, and that route\x27s client "
      "on the person\x27s side. No process running Conduit\x27s code decrypts, terminates or copies "
      "it, and no engine, Interface or provider log, record or diagnostic holds what passes over "
      "it. A forward opens only after the engine relayed this sign-in\x27s URL under this rule, "
      "for the port its redirect_uri names, to that node\x27s loopback only, and closes when "
      "sign-in.v1 ends, signed in, failed or unknown at DL18. Where the local port cannot be "
      "bound, the Interface says so, naming the port, and opens no forward on another port. A "
      "Claude Agent\x27s paste route, the person\x27s own delivery, on the node, of the return or "
      "of a page login\x27s code, is held by no engine or Interface process, and no result, "
      "record, diagnostic or log of either holds a page login\x27s code, as PND-014 holds for a "
      "carried value. The connection Conduit "
      "manages for a Claude sign-in does not bind the person who completes the login; the "
      "expected account (PND-011) remains the only binding.")
docs = list(yaml.safe_load_all(open("spec/schemas/contracts.yaml", encoding="utf-8")))
c = [d for d in docs if d and d.get("metadata", {}).get("name") == "sign-in-login.v1"][0]
d1 = ("Whether the engine may carry the return from the person\x27s Interface to its program. "
      "False carries none: the person completes the login on the program\x27s machine, at it or "
      "over a connection Conduit manages, by remote desktop or by SSH, pasting the code or "
      "forwarding the port, and no process running Conduit\x27s code reads, holds or relays the "
      "return (CTR-013, PND-013, IFC-049).")
others = sorted(new) == sorted(old) and all(new[k] == old[k] for k in new if k != "IFC-049")
print(d2 in n(p["then"]) and "open the URL in a browser" not in n(p["then"]),
      n(p["observable"]).endswith(d3),
      n(c["spec"]["properties"]["carried"]["description"]) == d1,
      others)
'
```

This prints `True True True`: each of the three files in `spec/` equals its version at `HEAD`
outside the span its decision names. `spec/schemas/contracts.yaml` is `HEAD`'s with the old
second line of the `carried` description replaced by decision 1's four lines;
`spec/interface/implementation.yaml` is `HEAD`'s with IFC-049's old `then` line replaced by
decision 2's three and decision 3's fifteen lines added after the line ending "(PND-002,
IFC-016)."; and `spec/README.md` is `HEAD`'s with the old paragraph replaced by decision 4's
twenty-six lines, its three paragraphs and the two blank lines between them:

```bash
/usr/bin/python3 -c '
import subprocess
head = lambda p: subprocess.run(["git", "show", "HEAD:" + p], capture_output=True, text=True, check=True).stdout.splitlines()
now = lambda p: open(p, encoding="utf-8").read().splitlines()
c, C = head("spec/schemas/contracts.yaml"), now("spec/schemas/contracts.yaml")
i, I = head("spec/interface/implementation.yaml"), now("spec/interface/implementation.yaml")
r, R = head("spec/README.md"), now("spec/README.md")
k = c.index("        False completes only with a browser on the program\x27s machine (CTR-013, PND-013).")
a = i.index("    panel. For carried: false it says to open the URL in a browser on that node\x27s machine.")
b = i.index("    receives a sign-in\x27s URL, login metadata or return (PND-002, IFC-016).")
s = r.index("In v0 the person\x27s Interface runs on the engine\x27s machine. From another machine or a phone,")
e = r.index("practice in v0, within PND-010\x27s scoped authority (ruling 75 C2).")
print(C[:k] == c[:k] and C[k + 4:] == c[k + 1:],
      I[:a] == i[:a] and I[a + 3:b + 3] == i[a + 1:b + 1] and I[b + 18:] == i[b + 1:],
      R[:s] == r[:s] and R[s + 26:] == r[e + 1:])
'
```

This prints 0: no sentence in `spec/` says that a `carried: false` login completes only with a
browser on the program's machine, that those Agents sign in only with a browser on their own
machine, that the message says to open the URL in a browser on the node's machine, or that an SSH
forward is the person's own (the record's done-when for its item 4). It reads every file under
`spec/`; `LC_ALL=C` keeps `tr` from stopping at the example that is not UTF-8 on purpose,
`spec/schemas/examples/invalid/SUB-005-not-utf-8.yaml`:

```bash
find spec -type f -exec cat {} + | LC_ALL=C tr -s ' \n' '  ' | LC_ALL=C grep -oE 'completes only with a browser|sign in only with a browser|open the URL in a browser on that node|is the person'"'"'s own and carries nothing' | wc -l
```

Each of these prints 0:

```bash
grep -c 'Spec pending, ruling 80, order 050' IN_FORCE.md
grep -c 'F95, by a named spec fix' IN_FORCE.md
grep -c '^- \*Spec pending, order 050\*: the connection Conduit manages' IN_FORCE.md
grep -c '^- \*Spec pending, order 050\*: on the engine'"'"'s own runner' IN_FORCE.md
```

`git diff --numstat IN_FORCE.md` prints `5`, `4` and `IN_FORCE.md`, separated by tabs, where
line 228 kept F97's sentence, or `4`, `4` where decision 5 deleted it: four lines replaced or
deleted and one added. `git status --short` lists exactly `IN_FORCE.md`, `spec/README.md`,
`spec/interface/implementation.yaml` and `spec/schemas/contracts.yaml`, each modified, and
`engine/STANDING.md` is not among them. Do not stage anything. Report each choice you made
yourself, which state of line 228 you found, and every check's last line.

Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
([ruling 79](../proposals/ruling-79/README.md), point 2), the reader against ruling 82 point 6's
lines o2, o3 and o5 and the record's section 6 above all: that the texts say only what those lines
say, name no software and no protocol beyond ruling 80 point 8's own route words, remote desktop
and SSH, with redirect_uri already IFC-049's, state no condition as a new promise, and keep F95's
sentence word for word. It merges on a pass of `tools/gate.py`; CI runs on the pull request as a
second opinion that the merge does not wait for ([ruling 80](../proposals/ruling-80/README.md)).
It may merge at any time, before or after order 048a-1 (decision 5), and before orders 050n and
050o, which cite it.

## Record

Run on 2026-11-17, from about 11:27 to 12:03 UTC, on the Mac from `main` `5eb71667` (the merge that
added this order), in one round: the worker (1895 s) and a fresh Codex verifier (254 s). The change
is the four files of Scope: decision 1's four lines for the `carried` description in
`spec/schemas/contracts.yaml`; decision 2's three lines for IFC-049's `then` and decision 3's
fifteen lines at the end of its `observable` in `spec/interface/implementation.yaml`; decision 4's
three paragraphs in place of the one paragraph of the limits in `spec/README.md`; and decision 5's
lines in `IN_FORCE.md`. The worker found line 228 in state A, so F97's sentence stays pending, now
ending `with one loader vector (82 point 6, o5).`, and F95's new line follows F96's.
`tools/promise_standing.py --write` changed nothing, and `check_model_account.py` printed the same
counts as on the starting tree (246 rows, 232 safety, 14 not safety, 0 uncatalogued). The worker
made no choice of wording or behaviour of its own; its one choice was to run cargo offline, with no
toolchain installed on demand. It reported, outside this order's Scope, that order 048a-1's decision
8 and its two F95 checks need the coordinator's amendment before that order is dispatched. Its
sandbox denied loopback binds: `cargo test --release` in `engine/` exited 101, 13 HTTP tests failing
with `PermissionDenied`, and `tools/gate.py` failed six test gates on the same denials and skipped
`secrets`, since gitleaks was not installed. The verifier found no must and no should; it confirmed
from the saved logs that those failures were the sandbox's socket denials, and left the engine's
tests and the gate to a run outside the sandbox.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), against ruling 82 point
6's lines o2, o3 and o5 and the record's section 6 above all. Since the order's checks compare with
`HEAD` and the worktree's `HEAD` was the checkpoint, it ran every check of "Done when" in a scratch
clone reset to the base, with the change unstaged. All passed: the eight `tools/` checks (each
ending `Errors: 0`, `check_model_account.py`'s output identical on the changed and the base tree,
`engine/STANDING.md` unchanged by `--write`) and `git diff --check` (no output);
`cargo test --release` in `engine/` with `CONDUIT_VECTOR_THREADS=4`, which the worker's sandbox had
stopped (28 `test result:` lines, all `ok` with `0 failed`); the ten `tr` and `grep` checks and the
four whole-line checks (1 each); `F97, by order 048a-1` (1, as before: state A) and the
loader-vector line (1); the two Python checks (`True True True True` and `True True True`); the scan
of `spec/` for the stale route sentences (0); the four pending-line checks (0 each);
`git diff --numstat IN_FORCE.md` (`5`, `4`); and `git status --short` (the four files of Scope,
modified, nothing staged). It compared decisions 1 to 4 and decision 5's four lines with the order's
text, the order's indent removed: equal byte for byte. It found that the texts say only what o2, o3,
o5 and the record's section 6 items 1 to 4 say; that, beyond text already there, they name no
software or protocol but SSH, remote desktop, a tunnel and redirect_uri; that they add no promise
ID; and that F95's sentence is the same word for word in both places. The added lines held no
private detail, and the checkpoint's author and committer were the work-order identity. It found no
must and no should, and noted that `tools/gate.py`, not a check of "Done when", was still owed a run
outside the sandbox before the merge.

The fixer fetched and rebased the branch onto `origin/main` at `5eb71667` with the work-order
identity; the branch was already on it, so nothing moved. With no finding from the verifier or the
reader, it changed no file but this Record. It then ran, from the worktree's root, every check of
"Done when", with `origin/main` in place of `HEAD`, since the change is now committed: in the two
Python checks, and as `git diff --numstat origin/main -- IN_FORCE.md` and
`git diff --name-status origin/main` for the numstat and the status. The eight `tools/` checks each
ended `Errors: 0`, and `check_model_account.py`'s output was identical to the base tree's, from an
export of `origin/main`; `engine/STANDING.md` was unchanged.
`CONDUIT_VECTOR_THREADS=4 cargo test --release` in `engine/` exited 0 with 28 `test result:` lines,
all `ok` with `0 failed`, the last
`test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`. The
ten `tr` and `grep` checks and the four whole-line checks printed 1 each; `F97, by order 048a-1` and
the loader-vector line 1 each; the Python checks `True True True True` and `True True True`; the
scan of `spec/` 0; and the four pending-line checks 0 each. The numstat printed `5`, `4` and
`IN_FORCE.md`, and the status, before this Record was written, listed exactly the four files of
Scope, each modified. With this Record in place, `python3 tools/check_docs.py` and
`python3 tools/check_private_paths.py` ended `Errors: 0`, and `git diff --check origin/main...HEAD`
and `git diff --check` printed nothing. The gate's run on this order's squash, which this merge
waits on, is the coordinator's at the merge.

At the merge the coordinator applied the second reader's one should, in this Record: the worker's
one choice, running cargo offline. This order merges before order 048a-1 is dispatched, so the
coordinator amends 048a-1's decision 8 and its two checks that name F95's sentence before that
dispatch, as this order's note on 048a-1 says. The gate runs on the squash, and the merge waits on
its PASS.
