# Work order 207: keys per hook, and three sign-in fixes, in the specification

You are a Codex worker carrying out one work order in this repository. It runs on the Mac or on
the second machine; it needs nothing of Linux. Read [AGENTS.md](../../AGENTS.md), its section
"If you were given a work order" above all, then this order. Your report must match the schema in
`004-i2-01-vectors-and-first-start.schema.json`, the schema the orders share: list in `vectors`
the four cases you add, with the promises each tests, `passes` true for
`requests-keyed-hook-manifest` and false for the other three, each an expected failure until
order 209. This order has network access only because the engine's tests bind loopback ports,
which the sandbox refuses without it; use it for nothing else, add no crate, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, case, fixture or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**What it writes.** Ruling 86, whose record merges before this order starts, has these written
into `spec/` now, ahead of the engine:

- **D, keys per operation** (point 2, as the owner's third answer amends it): an implementation
  item marked `plain` gains a list, `keyed`, of selections from its `implements`, each still a key
  of its `tools`. The engine's calls for those operations carry Conduit's keys and follow every
  rule of an implementation that carries them; every other call to the item stays plain. Ruling
  82's o1, "the item that implements `sign-in-hook.v1` carries Conduit's keys", is read per
  operation: that operation's calls on the item carry the keys. By the coordinator's narrowing
  below, `keyed` lists hooks alone, on an item reached by `command` or `package`;
- **(b) IFC-010's `then`**: a URL request from the `sign-in-hook.v1` part of `sign-in.v1` is
  relayed, as IFC-007, IFC-016 and PND-010 say;
- **(c) a login name aimed at a plain hook**: PND-010 refuses a `signInLogin` aimed at a
  `sign-in-hook.v1` whose operation is not keyed, under D's terms, before any part runs, so that
  no browser opens;
- **(d) a failed account check goes on**: a failed account check before the hook still goes on to
  `sign-in-hook.v1`, and completion still needs a later check that names the account; by the
  coordinator's reading, setup's check before its own hook goes on alike.

This order writes them as promises, in the terms of
[spec/definitions.md](../../spec/definitions.md), with the definitions, the schema, the core
description and the cases that follow, and records that the engine does not keep them yet. Order
209 builds the engine half; this order changes no source.

**What it no longer writes.** Ruling 86 had a point 3 (a), a carrier: IFC-025 would pass the
`content` of a keyed implementation's failed or unknown result as it passes a plain one's. Under D
the Agents' turns stay plain, so the carrier would never be used, and the owner's third answer
writes D instead of it. IFC-025 stays as it is on `main`.

**Where it comes from.** The owner chose, on 2026-11-18 (2026-11-19 in UTC), the option that read:
"0.2.0 ships plain items (default-login sign-in on the Mac works). The carrier spec + engine half
land now (~1 day) so later keyed packages need no new engine; IFC-010 is fixed and a login choice
aimed at a plain hook is refused before any browser opens. A half-day spec check picks D
(per-operation keys) or E (hook arguments) as the route for keying next. o1 amended for 0.2.0." For
`codex.v2`'s account check, whose refresh is blocked so that the check writes nothing (ruling 86,
point 5), the owner chose "(b) Proceed to sign-in (Recommended)", whose option read: "Amend
PND-010 so a failed check before the hook still proceeds to the sign-in hook. Clean, and closes an
existing gap where a rejected refresh token blocks re-sign-in." After the spec check, the owner
chose "D's engine in 0.2.0 (Recommended)", whose option read: "Rule D as the route (o1 read per
operation). Order 207 writes D into the spec instead of the carrier, and 209 builds D's engine half
in 0.2.0, so the later keyed sign-in release is package-only and needs no newer engine, your
stated aim. About the same cost as the carrier (one spec + one engine order, off the chain); the
spec edit is wider (~56 lines in 17 files)." Ruling 86's record holds the three answers and the
options chosen, verbatim.

**Why each change.** Each closes a gap or a contradiction in the text at `c704275f`, which
`7f56262a` leaves unchanged under `spec/`:

- D: `plain` is said of an item, "marked `plain: true` in its implementation item" (the row
  "plain" of spec/definitions.md, PKG-020), and TPL-019 allows a template one item that is run,
  shared by an Agent's turn and every hook. So keying the sign-in hook, as o1 asks, keyed the
  turn, which order 050e kept plain. `implementation-item.v1` holds `additionalProperties: false`,
  so a 0.2.0 engine rejects any new key at install: only a list written and built in 0.2.0 lets
  the later keyed sign-in release be a package alone.
- (b) IFC-010's `then` reads "A result whose `resultType` is `input_required` ... is unknown", with
  no exception, and its `observable` limits the unknown ending to "A URL-mode input request outside
  sign-in.v1". IFC-007's `observable` sends a plain server's URL-mode elicitation to IFC-016 and
  IFC-049; IFC-016 relays "URL-mode elicitation arising as a part of sign-in.v1"; PND-010 has the
  engine invoke `sign-in-hook.v1` "and relays its URL elicitation under IFC-049". Read alone, the
  `then` ends a plain hook's sign-in unknown.
- (c) PND-010 refuses before any part only "a signInLogin naming no login the Type states". Its
  `observable` says that "A plain hook gets no Conduit keys (IFC-007), hence no name; its URL for
  another login fails IFC-049's match": a stated name aimed at a plain hook is dispatched, the hook
  starts a login the person did not ask for, and only IFC-049's match then fails it, after a
  browser may have opened.
- (d) PND-010 goes on to the hook only where the check "succeeds with no account or another
  account", and PND-011 says "A failed or unknown account hook does not complete sign-in", without
  saying whether it goes on. Read strictly, a home whose check fails, as `codex.v2`'s will where an
  expired access token meets the blocked refresh, cannot sign in again through `sign-in.v1`.
  Setup checks before its setup hook (PND-011's `observable`), and TPL-012 begins no part after one
  that did not succeed, so such a home fails every setup too, which OPS-022 ends failed or unknown.

**What it does not take.**

- **The engine half**, order 209 (ruling 86, point 4, as the third answer amends it): D's engine
  half, which is the loader's rules for `keyed` (each selection a key of `tools` and a hook), the
  core's reading of `plain` per call for hook dispatch, the discovery of IFC-037 before the first
  call for a keyed hook to each run, and the keyed hook's dispatch; the refusal of (c); and (d) in
  setup and in `sign-in.v1`. The schema half of D needs no build: the loader reads `manifest.v1`
  from `spec/`, so it accepts `keyed` once this order lands. Order 209 makes this order's three
  expected failures pass and removes them. It builds no keyed `content` copy.
- **Keying the Agent packages** (ruling 86, point 1): their `.v2` items stay plain in 0.2.0, and
  list nothing under `keyed`; a later release keys their sign-in hooks in the packages alone.
- **`codex.v2`'s account check** with refresh blocked (ruling 86, point 5): order 208's probe first
  confirms that the check writes nothing, then order 050m builds it.
- **A case of a keyed hook's call**: its steps need the discovery that order 209 builds, so they
  are order 209's. **The cases of (b) and of (d) in `sign-in.v1`**: the `sign-in-hook.v1` part and
  the relay are orders 048a-5d and 048a-5e, and polling is 048a-5f, whose cases show them.
  `sign-in.v1`'s account check is order 048a-5b's, whose case ends `sign-in.v1` failed, naming
  PND-011, at a failed check (step 24 of `requests-sign-in-account-first`); order 209 changes
  that step (below, "What follows").
- **NAM-024's reading** of a later keying that turns (c)'s refusal of a named login into a started
  login: the spec check left it to the owner, and this order writes nothing of it.
- **MC9a**, which order 201 registered against PND-005, PND-010, PND-011 and PND-013 as they stood
  at `f72c2bc3`, and order 204 runs: this order changes their text, and leaves to the coordinator
  whether MC9a is registered again before it runs (MC9a's rule, D29).

**When this order starts.** After ruling 86's record, with the owner's third answer, has merged:
it starts from `main` at `7f56262a` or later, where that record and order 048a-5b have merged.
Before anything else, from the repository root, run these. Each prints what its comment says; if
one prints anything else, change nothing and report which. Ruling 86's merged record adds four
lines to IN_FORCE.md marked spec pending, order 207, and holds no Coordinator's line for setup,
so decision 17 adds that line after the line for (d); five would mean a record that holds it.

```sh
test -f docs/proposals/ruling-86/README.md && echo present                  # present
grep -ci 'spec pending, order 207' IN_FORCE.md   # 4, or 5 with a Coordinator's line for setup
grep -c 'keyed' spec/schemas/manifest.yaml                                 # 0
ls -d conformance/conventions/requests/keyed-hook-manifest conformance/conventions/requests/keyed-not-a-hook conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/requests/sign-in-setup-check-failed spec/schemas/examples/invalid/PKG-020-manifest-keyed-not-plain.yaml 2>/dev/null | grep -c .   # 0
grep -c 'requests-keyed-not-a-hook\|requests-sign-in-login-plain-hook\|requests-sign-in-setup-check-failed' conformance/expected-failures.yaml   # 0
grep -c '(order 207)' engine/CONFORMANCE.md                                # 0
```

Orders that land beside this one change
`conformance/conventions/requests/index.yaml`, `conformance/expected-failures.yaml`,
`engine/CONFORMANCE.md` and `engine/STANDING.md`; whichever lands second keeps both changes and
writes `engine/STANDING.md` again with the tool. None of them changes `spec/`.

**Who decided what.** Ruling 86 decides D as the route, o1 read per operation, D written here
instead of the carrier, D's engine half in 0.2.0, and (b) to (d), as the owner's words above put
them. Everything else is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- the exact text of each promise change below, and which promises it touches;
- **the narrowing of D**: `keyed` lists hooks alone, the five core hooks `setup-hook.v1`,
  `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` and `sign-out-hook.v1`, and only an item
  reached by `command` or `package` states it, as the spec check's section 6, point 4,
  recommended. So the request path, a request's repeats under OPS-012, IFC-025's results and the
  gate of IFC-041, stays per item, and only hook dispatch reads `plain` per call. Ruling 82's o1
  asks only for a keyed sign-in hook, so nothing it needs is lost;
- for D: the name `keyed`; a definitional sentence in the row "plain" that reads every promise
  speaking of an implementation marked `plain` or one that carries Conduit's keys per call, beside
  the `when` clauses and sentences rewritten below, those the spec check found changing: the rows
  "plain", "implementation item" and "Conduit's extension", a new row "`keyed`", PKG-020,
  `manifest.v1`, IFC-001, IFC-004, IFC-006, IFC-007, IFC-009, IFC-010's `when`, IFC-037, IFC-045,
  IFC-047, PND-010 and PND-013's first check; IFC-008, IFC-019, IFC-026 and OPS-028 changed by
  the definition and named in PKG-020, not reworded; OPS-012, IFC-025, IFC-049, PND-005's halves
  and TPL-019 untouched; IFC-041, IFC-043 and IFC-044, which speak of an item reached by `remote`,
  and of one not marked `plain`, untouched too, since no such item states `keyed`: ruling 86's
  record names IFC-043 and IFC-044 among the promises that change, and they stay unchanged, no
  `remote` item stating `keyed`;
- for D's schema: `keyed` a non-empty array of distinct selections, `false` beside a `plain` that
  is not true, refused beside `remote`; that each selection is a key of `tools` and a hook left to
  PKG-020's rule, as `implementation-item.v1` leaves the core package's operations to a rule today,
  with a row of the rules table of `spec/schemas/README.md` that `check_schemas.py` does not apply
  (`—`); one invalid example, `PKG-020-manifest-keyed-not-plain.yaml`, and its line in the
  loader's fixture of verdicts;
- for IFC-037: where the discovery result of a plain item that states `keyed` lacks the extension,
  only the hooks that `keyed` lists are not dispatched, and its other calls are sent as before;
- for (b): the exception names the `sign-in-hook.v1` part alone, a URL request from any other part
  of `sign-in.v1` staying unknown, and says that such a result "ends nothing" of itself, the words
  IFC-010's `observable` uses for an error after which the call is sent again, and that IFC-016's
  relay of URL-mode elicitation arising as a part of `sign-in.v1` is this one (row 52 of the
  owner's review list);
- for (c): "a plain hook" is a `sign-in-hook.v1` whose implementation is marked `plain` and whose
  item's `keyed` does not list it; the reason invalid, decided with and after the refusal of an
  unknown name, and a diagnostic that says the request may be sent again without `signInLogin`;
- for (d): a failed or unknown check before the hook counts as no account, raises nothing that
  PND-005 does not raise for a contract that reports a missing or lapsed login, and clears no
  condition, and a poll that failed or ended unknown counts as no account yet, so that polling
  goes on (the coordinator's reading of "completion still needing a later check that names the
  account", row 51 of the owner's review list as this order numbers it);
- setup alike (the coordinator's reading, row 50): a setup account check that failed or ended
  unknown counts as no account, raises the sign-in action under PND-005, undoes nothing, clears no
  condition, and counts as succeeded for TPL-012 and, amended to say so, OPS-022, so the setup goes
  on. The other reading, going on with no action raised, was set aside because it would let model
  work run on an account that no check named; this one's cost is a sign-in asked for where only an
  access token had lapsed. It is separable: striking it leaves (d) in `sign-in.v1` whole;
- the four cases, their IDs, fixtures, steps and evidence, their manifests, and showing a keyed
  hook's call, (b) and `sign-in.v1`'s half of (d) in later orders' cases rather than here;
- recording the engine's lag as three expected failures, a section of `engine/CONFORMANCE.md`, the
  regenerated `engine/STANDING.md` and, for the schema half that the loader keeps today, the case
  that passes and the fixture line; marking TPL-012's and OPS-022's model-checking rows "text
  changed, not rerun"; and the four rows of the owner's review list (49 to 52), row 49 for the
  narrowing;
- leaving `requests-sign-in-account-first` as it is, though its step 24, which completes the
  account check failed, asserts the ending that (d) rewrites: CONTRIBUTING.md's rule 8, under
  "Writing promises", lists a case that a change to `spec/` rewrites, and that the build does not
  pass yet, as an expected failure. That case's other steps assert what order 048a-5b built, and
  an entry would excuse all of its 5771 lines until order 209; the step instead keeps what the
  build does, the lag is stated in `engine/CONFORMANCE.md`, beside 048a-5b's sentence there that
  still cites PND-011 for that ending, and order 209 rewrites the step as a declared flip.

Apply them as written and report them under `decisions` as the coordinator's.

## What to read

- `spec/definitions.md`, the rows "plain" (both tables), "implementation item", "Conduit's
  extension", "account check" and "set up".
- `spec/schemas/manifest.yaml`, `implementation-item.v1`; `spec/schemas/README.md`, the table of
  rules no schema can express and the section "Choices the schemas encode".
- `spec/promises/packages.yaml`, PKG-019 and PKG-020.
- `spec/interface/implementation.yaml`: IFC-001, IFC-004, IFC-006, IFC-007, IFC-008, IFC-009,
  IFC-010, IFC-016's `observable` (its sentences from "Only URL-mode elicitation arising"), IFC-019,
  IFC-025, IFC-026, IFC-037, IFC-045, IFC-047, IFC-049's `observable` (from "A sign-in-hook.v1 URL
  matches exactly one login") and IFC-050's `observable`.
- `spec/promises/pending.yaml`: PND-005, PND-010, PND-011 and PND-013's first check.
- `spec/promises/templates.yaml`, TPL-012 and TPL-019; `spec/conventions/operations.yaml`, OPS-012
  and OPS-022; `spec/core/operations.yaml`, `sign-in.v1`'s description.
- `conformance/README.md`, the sections "Expected failures" and "Portable cases";
  `engine/CONFORMANCE.md`, its section "Refusals and reasons (order 046a)";
  `engine/crates/loader/tests/expected/invalid-verdicts.txt`, its header;
  `tools/promise_standing.py`'s docstring; `experiments/012-model-checking/account.yaml`, the row
  of TPL-012.
- The cases `requests-sign-in-setup-account` and `requests-sign-in-refusals-before-parts`, whose
  steps the new cases copy, and `conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json`.

## The decisions it applies

The script after decision 19 applies decisions 1 to 16 exactly; decision 17 is yours by hand.

1. **D in the definitions** (point 2). In `spec/definitions.md`: the short row "plain" becomes
   "plain, `keyed`", whose server "receives none of Conduit's keys and applies no filter, except on
   a call for a hook that its item lists under `keyed`". The row "implementation item" lists
   "`plain` with `tools` and, for an item reached by `command` or `package`, `keyed`". The row
   "plain" says: "It supports no requirement and never counts as deduplicating. A call for a hook
   that its item lists under `keyed` carries Conduit's keys; on every other call it receives none
   of them (IFC-007), and the results of those calls are turned into outcomes by the table of
   IFC-010"; that it learns its configuration also "through the keys of a call for a hook that its
   item lists under `keyed`"; and, after "An implementation that is not plain is one that carries
   Conduit's keys": "and so is a plain one for a call for a hook that its item lists under `keyed`:
   where a promise speaks of a call to, or a result of, an implementation marked `plain` or one
   that carries Conduit's keys, it means the implementation as it serves that call (PKG-020)." A
   new row "`keyed`" follows it: the key, of an item marked `plain: true` and reached by `command`
   or `package`, that lists the hooks whose calls carry Conduit's keys, each a selection in
   `implements`, a key of `tools` and one of the five core hooks; such a call is to an
   implementation that carries Conduit's keys (IFC-037's discovery before the first such call to
   each run, IFC-006, IFC-009, IFC-008, IFC-019, IFC-026, IFC-047); every other call, every request
   and a hook the list does not name, which the promises call a plain hook, is plain (IFC-007,
   IFC-010), so IFC-025 and OPS-012 read a request as before; and a `remote` item states none
   (IFC-041). The row "Conduit's extension" reads discovery before "its first call that carries
   Conduit's keys to each run of an implementation, a call for a hook that a plain item lists under
   `keyed` among them", and "A plain implementation is declared none, except on a call for a hook
   that its item lists under `keyed` (PKG-020)."

2. **D in `manifest.v1`** (point 2, the coordinator's shape). `implementation-item.v1` gains
   `keyed`, an array of `selection.v1` with `minItems: 1` and `uniqueItems: true`; its
   `dependentSchemas` gain `keyed: {not: {required: [remote]}}`; its `else`, for an item not marked
   `plain: true`, gains `keyed: false` under `properties`; and its description says where `keyed`
   is stated and that its rule is PKG-020's, not the schema's. `spec/schemas/README.md` says, in
   "Choices the schemas encode", that an item not marked `plain: true` holds no `keyed`, nor one
   reached by `remote`, and its table of rules gains, after PKG-030's row, a PKG-020 row for "each
   selection in an item's `keyed` is a key of its `tools`" and names one of the five hooks, with
   `—` in the tool's column. The example `invalid/PKG-020-manifest-keyed-not-plain.yaml` is a
   manifest whose item, not marked `plain`, lists its sign-in hook under `keyed`; the schema
   rejects it, and `engine/crates/loader/tests/expected/invalid-verdicts.txt` gains its line,
   `PKG-020-manifest-keyed-not-plain.yaml schema`, before `PKG-020-manifest-plain-with-supports.yaml`,
   as the loader's test asks for each file under `invalid/`.

3. **D in PKG-020** (point 2 and the narrowing). Its `then` gains: "One reached by `command` or
   `package` may state `keyed`, a list of hooks, each a selection in its `implements` and a key of
   its `tools` that names `setup-hook.v1`, `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1`
   or `sign-out-hook.v1`: the engine's calls for a hook that `keyed` lists carry Conduit's keys, and
   its other calls to the implementation do not." Its `observable` rejects "an item that states
   `keyed` and is not marked `plain` or is reached by `remote`, and a selection in `keyed` that is
   not a key of `tools` or names an operation other than those five hooks"; has a plain server sent
   none of Conduit's keys "except on a call for a hook that `keyed` lists", from which it learns its
   configuration through IFC-006's keys; and ends with what a keyed hook's call keeps (IFC-037's
   discovery and extension, IFC-006, IFC-008, IFC-009, IFC-019, IFC-026, IFC-047, and a keyed setup
   or retire hook's idempotency key and new attempts under OPS-028), that every other call is plain
   (IFC-007, IFC-010), that requests, results and repeats stay plain (IFC-025, OPS-012), that a gate
   stands before a `remote` item for all its calls or for none (IFC-041), and that one item that is
   run so serves an Agent's plain turns beside a keyed `sign-in-hook.v1` (TPL-019).

4. **D in the interface** (point 2). Each `when` and sentence that reads `plain` per item reads it
   per call, by the words "other than a call for a hook that its item's `keyed` lists (PKG-020)" or
   their like: IFC-001's `then` ("the name its `tools` map gives under PKG-020, on every call, one
   for a hook that its item's `keyed` lists among them") and `observable` ("A plain implementation
   is sent no such key, except on a call for a hook that its item's `keyed` lists (PKG-020)");
   IFC-004's `then` and `observable` ("A call that IFC-007 governs is declared no extension"); the
   `when` of IFC-006 ("or a call for a hook that the `keyed` of an implementation marked `plain`
   lists (PKG-020)"), IFC-007, IFC-009, IFC-010 and IFC-047; IFC-037's `then` ("Before its first such call to each run of the implementation,
   which for an implementation marked `plain` is its first call for a hook that its item's `keyed`
   lists (PKG-020)") and `observable` (only a call that IFC-007 governs is sent none of Conduit's
   keys, and a missing extension makes only the hooks that `keyed` lists undispatched); and
   IFC-045's `observable`.

5. **D in PND-013's first check** (point 2): "its sign-in-hook.v1 carries Conduit's keys, its
   implementation not marked `plain` or its item's `keyed` listing sign-in-hook.v1 (IFC-007,
   PKG-020), else not-accepted".

6. **IFC-010's `then` relays the hook's URL request** (point 3 (b)). After "and no result at all, is
   unknown." it gains: "An `input_required` result holding a URL-mode input request from the
   sign-in-hook.v1 part of sign-in.v1 ends nothing of itself: it is relayed as IFC-016 and PND-010
   state or, where it fails a check of IFC-049, ends sign-in.v1 failed under IFC-049; one from any
   other part of sign-in.v1 is unknown under this table; for a plain implementation, IFC-016's
   relay of URL-mode elicitation arising as a part of sign-in.v1 is this one." Its `observable` does not change: its last sentences already speak
   of a URL request outside `sign-in.v1`.

7. **PND-010's `then` refuses a name aimed at a plain hook** (point 3 (c), under D). After "with a
   diagnostic naming PND-010 and the stated login names" it gains: "; one naming a stated login
   reports invalid where the instance's sign-in-hook.v1 is a plain hook, its implementation marked
   `plain` and its item's `keyed` not listing sign-in-hook.v1 (PKG-020), with a diagnostic naming
   PND-010 and the login, and saying that a plain hook gets no login name (IFC-007) and that the
   request may be sent without signInLogin." Both invalids come after the Type's own check and
   before the conflict, as "These decisions follow OPS-023's order" has it.

8. **PND-010's `observable`** (points 3 (c) and (d)). The check before the hook reads: "If that
   check succeeds with no account or another account, or fails or ends unknown, the engine invokes
   sign-in-hook.v1 as a part and relays its URL elicitation under IFC-049; that initial mismatch or
   failure does not undo the account or end sign-in.v1. A check there that failed or ended unknown
   counts as no account: it raises nothing that PND-005 does not raise for a contract that reports
   a missing or lapsed login, and clears no condition." Polling reads: "no account yet is not
   failed, and a poll that failed or ended unknown counts as no account yet, so that only a later
   check that reports the expected account completes sign-in.v1." The plain hook's sentence
   becomes: "A plain hook, one whose implementation is marked `plain` and whose item's `keyed` does
   not list sign-in-hook.v1, gets no Conduit keys (IFC-007), hence no name, so a request that asks
   it for a login reports invalid before any part, and no login starts that the person did not ask
   for; without a name its URL still matches one stated login under IFC-049. A sign-in-hook.v1 that
   its item's `keyed` lists gets them, its item's other calls staying plain (PKG-020)." "This
   includes asking for a device code where the account permits it" moves up to follow the sentence
   on the asked-for name, which it qualifies.

9. **PND-011 follows** (point 3 (d)). In its `then`, the check before the hook: "no account,
   another account, or a check that failed or ended unknown proceeds to that hook"; and its last
   sentence becomes: "A failed or unknown account hook completes no sign-in and clears no
   condition, and its uncertainty is explained. It counts as no account before sign-in-hook.v1
   and in setup, and as no account yet during polling; no account yet during polling follows
   PND-010."

10. **Setup's check alike** (point 3 (d), the coordinator's reading). PND-011's `observable` gains,
    after its sentence on setup: "A setup check that failed or ended unknown raises the action under
    PND-005 as no account does, undoes nothing, and lets setup proceed to its setup hook (TPL-012)."
    PND-005's `when` reads "setup's account-hook reports no account or a different account, or fails
    or ends unknown". TPL-012's `then` gains, after "and is not marked optional, has succeeded":
    "; an account check that failed or ended unknown counts here as one that succeeded with no
    account (PND-011)"; its `observable` gains, after "was not dispatched leaves every later part not
    begun,": "an account check that failed or ended unknown apart, which counts as one that succeeded
    with no account,"; and its `observable` reads "No account, a different account, or an account
    check that failed or ended unknown raises the sign-in action under PND-005; setup does not wait
    for the person to sign in, and OPS-022 counts such a check as succeeded." OPS-022, which reads
    the parts of a setup from TPL-012, follows: its `then`'s first sentence reads "Its outcome is
    succeeded only if every part not marked optional succeeded, an account check that failed or
    ended unknown counting as succeeded under TPL-012, failed only if no part had an effect, and
    otherwise unknown.", and its `observable` gains, after "and one that failed and is not
    dispatched again counts as failed.": "An account check that failed or ended unknown counts as
    succeeded only in the setup that dispatched it; a further setup checks the account again under
    TPL-012."

11. **The definitions and `sign-in.v1`'s description follow.** In `spec/definitions.md`, the row
    "account check" sends a failed or unknown check on to the hook, counts it as no account yet
    while polling, has setup raise the action on it and go on once any undo has succeeded, and
    says that it completes nothing and clears no condition; the row "set up" counts such a check
    as one that found no account. In `spec/core/operations.yaml`, `sign-in.v1`'s description sends
    "no account, another account or a failed or unknown check" on to `sign-in-hook`, counts "a
    failed or unknown poll" as none yet, and refuses any name "where the instance's sign-in-hook is
    plain, its item not listing it under keyed, since a plain hook gets no name". In
    `spec/core/README.md`, the sentence on `signInLogin` gains: "except where the instance's
    sign-in-hook is a plain hook, one that its item does not list under `keyed`: it gets no name,
    and the request reports invalid before any part."

12. **TPL-012's and OPS-022's model-checking rows.** In
    `experiments/012-model-checking/account.yaml`, TPL-012's `standing` becomes `text changed, not
    rerun`, with `was: holds, not graded`, and OPS-022's likewise, with `was: 'contradiction:
    MC6-001'`; each keeps its `model`, `read` and `digest`, and each `note` gains one sentence naming
    this order. PND-005, PND-010, PND-011 and PND-013 are `left to MC9` and need no change;
    `check_model_account.py` fails a changed promise whose row claims a run.

13. **Four portable cases.** Each is JSON with two-space indentation and a final newline, starts
    from `{"recipe": "empty"}`, and copies whole the steps it takes from another case, `input`,
    evidence, `expect` and `capture`; a new step's evidence is its time, `2026-11-12T12:00:00Z`, 100
    `ids` `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
    140000 + 100n + k, 16 `revisions` and 16 `suffixes`, where n is the step's number in its case.
    Three use the fixture and documents of `requests-sign-in-lapse-and-conflict` with a manifest of
    their own, built from that case's: its one item marked `plain: true` and given `tools`, each
    implemented selection mapped to its operation's name with `-` written `_`:
    - **`requests-keyed-hook-manifest`**, `promises: [PKG-020, PND-010]`, `layers: [language, convention]`:
      that plain item with `keyed: [github.com/shrav-k/conduit/core#sign-in-hook.v1]`. Steps 1 and
      2 are `requests-sign-in-refusals-before-parts`'s first start and registration, which the
      loader accepts. Step 3 asks `group.sign-in.v1` for `signed`, declaring URL elicitation, with
      `signInLogin` `{"name": "browser"}`, and expects it pending with one dispatch, a `tools/call`
      of `account_hook` holding no `operation` key of Conduit's: a name aimed at a hook that
      `keyed` lists is not refused under (c), and the account check before it, which `keyed` does
      not list, is a plain call (IFC-007). It passes on today's engine once `manifest.v1` holds
      `keyed`, and after order 209, and is D's case of the loader's verdict; on the base's schema
      its step 2 is rejected under SUB-005, `'keyed' was unexpected`.
    - **`requests-keyed-not-a-hook`**, `promises: [PKG-020]`, `layers: [language]`, an expected
      failure until order 209: the plain item with `keyed: ['#ping.v1']`, a request's operation.
      Step 1 is the same first start; step 2 is the same registration, expecting it refused:
      `accepted` false, reason invalid, a diagnostic whose `ruleId` is PKG-020, `stateUnchanged`, no
      external intent, PKG-020 among the decisions' promises, and the records `access-decision` and
      `request-ended`, each with reason invalid and `dispatched` false.
    - **`requests-sign-in-login-plain-hook`**, `promises: [PND-010, IFC-007]`, an expected failure
      until order 209: the plain item with no `keyed`. Steps 1 and 2 are the same first start and
      registration. Steps 3 to 5 ask `group.sign-in.v1` for `signed`, declaring URL elicitation,
      with `signInLogin` `{"name": "browser"}` and `{"name": "device"}`, the Type's two stated
      logins, and `{"name": "phone"}`, which stays invalid beside them, and expect invalid with a
      PND-010 diagnostic, no external intent and the records `access-decision` and `request-ended`
      with reason invalid and `ruleId` PND-010.
    - **`requests-sign-in-setup-check-failed`**, `promises: [PND-011, PND-005, TPL-012, OPS-022]`,
      an expected failure until order 209, on the files of `requests-sign-in-hold`, whose item
      carries Conduit's keys, from `requests-sign-in-setup-account`'s steps. Steps 1 to 5 are its
      steps 1 to 5. Step 6 is its step 6 with the account check's result failed, keyed, and expects
      the setup succeeded, the sign-in action raised (`unanswered` 1) and no external intent: no
      undo. Steps 7 to 16 are its steps 8, 9, 12, 13 to 17, 20 and 21: the action and the hold,
      then a second setup whose undo fails, so that `SignInUndone` is `False`, then a third setup's
      account check. Step 17 is its step 22 with that check's result unknown, keyed, and no external
      intent: the setup succeeds and raises no second action. Step 18 is its step 23 with
      `SignInUndone` still `False`, reason `SignOutFailed`: the unknown check cleared no condition.

    The coordinator ran the script on a scratch copy of `7f56262a`: the first case passed its three
    steps, and each of the other three ran up to its first step that order 209 changes and failed
    there, as "Done when" gives; the later steps of the last two are not yet run, since the engine
    stops before them. If the runner refuses a case, or a copied step gives another reply than its
    own case states, stop and report the error: do not change the expectations to pass it. No
    expectation is generated from the engine's output.

14. **The coverage index and the expected failures.** The four cases' entries go into
    `conformance/conventions/requests/index.yaml` in byte order of ID, each with `id`, `case`,
    `promises` and `safety_rows: []`, as their neighbours. `conformance/expected-failures.yaml`
    gains one entry for each case but `requests-keyed-hook-manifest`, sorted by ID, whose `reason`
    names ruling 86's point, what the core lacks and that order 209 makes the case pass and removes
    the entry.

15. **`engine/CONFORMANCE.md`** gains a last section, "Ahead of the engine: ruling 86 (order 207)",
    which says, for D, what the loader keeps today through `manifest.v1` and what the core lacks,
    naming the cases and the invalid example, and that a keyed hook's call has no case until order
    209's discovery; for (c) and (d), what the core does today and the case of each; and that (b)
    is shown by orders 048a-5d and 048a-5e.

16. **Four rows of the owner's review list.** In
    `docs/proposals/first-build/to-review-with-the-owner.md`, after the last row of the table "From
    the targeted review of the sign-in surface", the next four numbers, 49 to 52 at `7f56262a`:
    D's narrowing to hooks on items that are run; setup's check alike; a check that ended unknown,
    before `sign-in-hook.v1` or while polling, counted as no account (yet), as a failed one is; and
    IFC-010's relay of a URL request from the `sign-in-hook.v1` part alone, each with why it
    matters, what striking it restores, and where.

17. **IN_FORCE.md, by hand.** Ruling 86's merged record adds four lines marked spec pending,
    order 207: points 3 (c) and (d), and the lines for D (point 2) and point 3 (b); it holds no
    Coordinator's line for setup. Replace each of them, whole, with its finished line below, in
    the same place, matching each by the point it cites, add the finished Coordinator's line for
    setup after the line for (d), and change no other line of IN_FORCE.md. Each finished line says
    what the specification now holds and points to it, and drops the sentences that said what it
    said before. If a line differs from its expected before-text below, replace it all the same and
    report it, before and after; report each line you changed or added, before and after.

    The expected before-text, as ruling 86's merged record wrote the lines at `7f56262a`: the line
    for D replaces the carrier's. The Coordinator's line for setup is the record's earlier text,
    which the merged record does not hold. If the record you start from holds it after all, replace
    it as the others; if it holds no Coordinator's line for setup, add its finished line after the
    line for (d) and report that you added it:

    ```text
    - *Spec pending, order 207*: PND-010 refuses, before dispatching any part, a `signInLogin` aimed at a sign-in hook whose operation its item does not list as keyed, which can never receive it, so that no browser opens; in 0.2.0 no item lists one, so every named login is refused. PND-010's observable has such a request reach the plain hook today, its URL then failing IFC-049's match. Before any part, rather than before the hook alone, is the coordinator's reading (86 point 3 (c)).
    - *Spec pending, order 207*: a failed account check before `sign-in-hook.v1` still goes on to that hook, and completion still needs a later check that names the expected account. PND-010's observable and PND-011's `then` name today only a check that succeeds with no account or another account as going on to the hook. A check that ends unknown is not named in the owner's option; whether it goes on alike is left to order 207 to put to the coordinator, and until then it does not (the coordinator's reading) (86 point 3 (d)).
    - *Coordinator's, spec pending, order 207*: by the coordinator's reading, setup's check before its setup hook goes on alike, open to the owner's strike: the owner's option named only the sign-in hook. TPL-012 today begins a part of a setup only once every earlier part not marked optional has succeeded, and PND-011's observable has setup check the account before its setup hook in TPL-012's order; both change. Under OPS-022 as it stands, such a setup does not succeed, the check being a part not marked optional: it ends failed, or unknown where a part had an effect, until a further setup's check names the account (86 point 3 (d)).
    - *Spec pending, order 207*: IFC-010's `then` relays a URL request inside a `sign-in.v1` part, as IFC-007, IFC-016 and PND-010 already say. IFC-010's `then` reads every `input_required` result of a plain implementation as unknown today, and its observable adds only that a URL request outside `sign-in.v1` also raises PND-005; it names none inside `sign-in.v1` (86 point 3 (b)).
    - *Spec pending, order 207*: keys per operation, option D. An implementation item marked `plain` may list hook operations it implements whose calls carry Conduit's keys; a call for a listed operation carries them and is read as a keyed implementation's, and every other call to the item stays plain, so `plain` is read per operation on the hook path and per item for requests. That shape, a list on an item that stays marked `plain`, is the spec check's and the coordinator's, open to the owner's strike; the owner ruled per-operation keying with the turn and the other hooks plain. The term `plain` in spec/definitions.md, the manifest's schema, PKG-020, IFC-037's discovery before the first keyed call to a run, the promises that read `plain` or "carries Conduit's keys", IFC-043 and IFC-044 among them, and PND-010's and PND-013's first check change with it; IFC-025 does not, the turn staying plain, and the carrier of a keyed turn's `content` is dropped. Narrowing the list to hook operations is the coordinator's, open to the owner's strike; the list's name and schema, and whether it is limited to items that a command or a package runs, are order 207's to put to the coordinator. Order 209 builds the engine half in 0.2.0 (86 points 2, 3 (a) and 4, reading 82 point 6, o1, per operation).
    ```

    The finished lines, each for the line that cites its point:

    ```text
    - **In spec/**: PND-010 refuses, before dispatching any part, a `signInLogin` aimed at a plain sign-in hook, one whose implementation is marked `plain` and whose item does not list `sign-in-hook.v1` under `keyed`, which can never receive it, so that no browser opens; in 0.2.0 no item lists one, so every named login is refused. Before any part, rather than before the hook alone, is the coordinator's reading (86 point 3 (c); PND-010, `sign-in.v1`'s description and the case `requests-sign-in-login-plain-hook`; the engine follows in order 209).
    - **In spec/**: a failed account check before `sign-in-hook.v1` still goes on to that hook, and completion still needs a later check that names the expected account. By the coordinator's reading in order 207, open to the owner's strike, a check that ends unknown goes on alike, and a poll that failed or ended unknown counts as no account yet (86 point 3 (d); PND-010, PND-011, spec/definitions.md and `sign-in.v1`'s description; the engine follows in order 209 for the check before the hook, and in orders 048a-5d and 048a-5f for the hook part and polling).
    - *Coordinator's*, **in spec/**: by the coordinator's reading, setup's check before its setup hook goes on alike, open to the owner's strike: the owner's option named only the sign-in hook. A setup check that failed or ended unknown raises the sign-in action, undoes nothing and counts as succeeded (86 point 3 (d); TPL-012, PND-011's observable, PND-005, OPS-022, spec/definitions.md and the case `requests-sign-in-setup-check-failed`; the engine follows in order 209).
    - **In spec/**: keys per operation, option D, are the route for keying, ruling 82's o1 read per operation: an implementation item marked `plain` may list under `keyed` hooks of its `implements`, each a key of its `tools`, whose calls carry Conduit's keys and keep every rule of an implementation that carries them, IFC-037's discovery before the first such call included, while every other call to the item stays plain. By the coordinator's narrowing in order 207, open to the owner's strike, `keyed` lists only the five core hooks, and only on an item reached by `command` or `package`, so requests, their results and the gate read `plain` per item; IFC-043 and IFC-044, which ruling 86 named, stay unchanged, no `remote` item stating `keyed`; the carrier of a keyed turn's `content` is dropped, and IFC-025 is unchanged (86 points 2, 3 (a) and 4, reading 82 point 6, o1, per operation; PKG-020, spec/definitions.md's rows plain and `keyed`, `manifest.v1`, IFC-006, IFC-007, IFC-009, IFC-010, IFC-037, PND-010, PND-013 and the cases `requests-keyed-hook-manifest` and `requests-keyed-not-a-hook`; the engine follows in order 209).
    - **In spec/**: IFC-010's `then` relays a URL request inside a `sign-in.v1` part, as IFC-007, IFC-016 and PND-010 already say. By the coordinator's reading, open to the owner's strike, it names the `sign-in-hook.v1` part alone, IFC-016's relay being this one; a URL request from any other part of `sign-in.v1` stays unknown, and an account check that so ends unknown counts as no account under point 3 (d) (86 point 3 (b); IFC-010; the engine follows in orders 048a-5d and 048a-5e, and in the order that builds IFC-049's URL match, not in order 209).
    ```

18. **`engine/STANDING.md` and the declaration.** From the repository root, after the script and
    decision 17, run `python3 tools/promise_standing.py --write` (or `/usr/bin/python3`), which
    rewrites the rows of IFC-007, OPS-022, PKG-020, PND-005, PND-010, PND-011 and TPL-012: each
    gains its cases and one expected failure (PKG-020 and PND-010 two cases), and keeps the standing
    `case passes`. `docs/work-orders/207-declared.yaml` comes with this order, with four shapes of
    kind `added`, one per case; check that it matches and change it only to make it so.

19. **Nothing else changes.** No source under `engine/`, `packages/`, `experiments/` or `tools/`,
    and no other case, fixture, example or test fixture; not IFC-008, IFC-016, IFC-019, IFC-025,
    IFC-026, IFC-041, IFC-043, IFC-044, IFC-049, OPS-012, TPL-019, the limits in `spec/README.md`,
    `conformance/manifest.yaml`, `docs/work-orders/README.md`, the ledger, ruling 86's record, or
    `experiments/012-model-checking/README.md`.

### The script

From the repository root, write the script below to `engine/target/order-207-write.py` (run
`mkdir -p engine/target` first; Git ignores `target/`) and run
`python3 engine/target/order-207-write.py`. It stops on an assertion, before or part way through
its edits, if a file is not as this order expects; then restore every file it touched with
`git checkout --`, delete every file and directory it created, stop and report the assertion.
Delete the script before you finish.

```python
import copy
import json
from pathlib import Path

REQ = Path("conformance/conventions/requests")
META = "io.github.shrav-k.conduit/"
CORE = "github.com/shrav-k/conduit/core#"


def replace_once(path, old, new):
    text = Path(path).read_text()
    assert text.count(old) == 1, (path, old[:80])
    Path(path).write_text(text.replace(old, new))


def load(path):
    return json.loads(Path(path).read_text())


def write_json(path, value):
    Path(path).write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


# Decision 1: D in the definitions: plain per call, and the new row `keyed` (ruling 86, point 2).
replace_once("spec/definitions.md",
             "| plain | Said of an existing server not written for this specification, which receives "
             "none of Conduit's keys and applies no filter. |",
             "| plain, `keyed` | Said of an existing server not written for this specification, which "
             "receives none of Conduit's keys and applies no filter, except on a call for a hook that its "
             "item lists under `keyed`. |")
replace_once("spec/definitions.md",
             "and `plain` with `tools` (PKG-019, PKG-020).",
             "and `plain` with `tools` and, for an item reached by `command` or `package`, `keyed` "
             "(PKG-019, PKG-020).")
replace_once("spec/definitions.md",
             "marked `plain: true` in its implementation item (PKG-020). It receives none of Conduit's "
             "keys (IFC-007), supports no requirement and never counts as deduplicating, and its results "
             "are turned",
             "marked `plain: true` in its implementation item (PKG-020). It supports no requirement and "
             "never counts as deduplicating. A call for a hook that its item lists under `keyed` carries "
             "Conduit's keys; on every other call it receives none of them (IFC-007), and the results of "
             "those calls are turned")
replace_once("spec/definitions.md",
             "learns its instance's configuration only through what its item's `environment`, "
             "`packageArguments` or `headers` take from it (PKG-020).",
             "learns its instance's configuration only through what its item's `environment`, "
             "`packageArguments` or `headers` take from it, and through the keys of a call for a hook "
             "that its item lists under `keyed` (PKG-020).")
replace_once("spec/definitions.md",
             "An implementation that is not plain is one that carries Conduit's keys.",
             "An implementation that is not plain is one that carries Conduit's keys, and so is a plain "
             "one for a call for a hook that its item lists under `keyed`: where a promise speaks of a "
             "call to, or a result of, an implementation marked `plain` or one that carries Conduit's "
             "keys, it means the implementation as it serves that call (PKG-020).")
replace_once("spec/definitions.md",
             "`interface.v1` and `pool.v1` are the core package's basic types. |\n",
             "`interface.v1` and `pool.v1` are the core package's basic types. |\n"
             "| `keyed` | The key of an implementation item that is marked `plain: true` and reached by "
             "`command` or `package`, which lists the hooks whose calls carry Conduit's keys: selections "
             "in the item's `implements`, each a key of its `tools`, each of them `setup-hook.v1`, "
             "`retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` or `sign-out-hook.v1` (PKG-020). A "
             "call for a hook that it lists is a call to an implementation that carries Conduit's keys: "
             "the engine reads the implementation's result of `server/discover` before the first such "
             "call to each run (IFC-037), the call declares Conduit's extension and holds Conduit's keys "
             "(IFC-006), its result states its outcome (IFC-009), and the implementation keeps for it "
             "every rule of one that carries Conduit's keys (IFC-008, IFC-019, IFC-026, IFC-047), and a "
             "setup or retire hook so listed its idempotency key and new attempts (OPS-028). Every "
             "other call to the item is plain (IFC-007, IFC-010): every request, and a hook that the "
             "list does not name, which the promises call a plain hook. A request's result and its "
             "repeat are therefore read as a plain implementation's (IFC-025, OPS-012). So one item that "
             "is run can serve plain requests, such as an Agent's turns, beside a sign-in hook that "
             "carries Conduit's keys, as TPL-019's one item that is run allows. An item reached by "
             "`remote` states none, so a gate stands before it for all its calls or for none "
             "(IFC-041). |\n")
replace_once("spec/definitions.md",
             "Before its first call to each run of an implementation that carries Conduit's keys, and "
             "to a `remote` item at each connection, the engine reads",
             "Before its first call that carries Conduit's keys to each run of an implementation, a call "
             "for a hook that a plain item lists under `keyed` among them, and to a `remote` item at "
             "each connection, the engine reads")
replace_once("spec/definitions.md",
             "A plain implementation is declared none. ",
             "A plain implementation is declared none, except on a call for a hook that its item lists "
             "under `keyed` (PKG-020). ")

# Decision 2: D in `manifest.v1`, its README and one invalid example (ruling 86, point 2).
replace_once("spec/schemas/manifest.yaml",
             "    binds the Package's `runtimeArguments`. An existing server that knows nothing of this\n"
             "    specification is marked `plain: true`, gives `tools` and states no `supports`; an item that\n"
             "    is not plain gives no `tools`, and, where it is reached by `remote`, stands at an `https`\n",
             "    binds the Package's `runtimeArguments`. An existing server that knows nothing of this\n"
             "    specification is marked `plain: true`, gives `tools` and states no `supports`, and, where it\n"
             "    is reached by `command` or `package`, may list in `keyed` the hooks whose calls carry\n"
             "    Conduit's keys; that each selection in `keyed` is a key of `tools` and one of the core\n"
             "    package's hooks is PKG-020's rule, and not in this schema. An item that is not plain gives\n"
             "    no `tools` and no `keyed`, and, where it is reached by `remote`, stands at an `https`\n")
replace_once("spec/schemas/manifest.yaml",
             "    plain: {type: boolean}\n",
             "    plain: {type: boolean}\n"
             "    keyed:\n"
             "      type: array\n"
             "      items: {$ref: github.com/shrav-k/conduit/core#selection.v1}\n"
             "      minItems: 1\n"
             "      uniqueItems: true\n")
replace_once("spec/schemas/manifest.yaml",
             "    environment:\n      not: {required: [remote]}\n",
             "    environment:\n      not: {required: [remote]}\n"
             "    keyed:\n      not: {required: [remote]}\n")
replace_once("spec/schemas/manifest.yaml",
             "  else:\n    not: {required: [tools]}\n    properties:\n",
             "  else:\n    not: {required: [tools]}\n    properties:\n      keyed: false\n")
replace_once("spec/schemas/README.md",
             "that is not marked `plain: true` holds no `tools` (PKG-020). These remain choices of the\n",
             "that is not marked `plain: true` holds no `tools` and no `keyed`, and one reached by `remote`\n"
             "no `keyed` (PKG-020). These remain choices of the\n")
replace_once("spec/schemas/README.md",
             "| That an item reached by `remote` implements no operation of `runtime-provider.v1`, "
             "`machine-provider.v1` or `package-source.v1` | PKG-030 | As for PKG-020 | Yes, for the "
             "manifests of the valid examples and those in `examples/invalid-by-rule` |\n",
             "| That an item reached by `remote` implements no operation of `runtime-provider.v1`, "
             "`machine-provider.v1` or `package-source.v1` | PKG-030 | As for PKG-020 | Yes, for the "
             "manifests of the valid examples and those in `examples/invalid-by-rule` |\n"
             "| That each selection in an item's `keyed` is a key of its `tools` and names `setup-hook.v1`, "
             "`retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` or `sign-out-hook.v1` | PKG-020 | "
             "The first relates two values of one item, and the second needs the core package's "
             "operations, which `implementation-item.v1` leaves to a rule | — |\n")
EXAMPLE = Path("spec/schemas/examples/invalid/PKG-020-manifest-keyed-not-plain.yaml")
assert not EXAMPLE.exists()
EXAMPLE.write_text(
    "# Example. It must be rejected:\n"
    "# only an item marked `plain: true` and reached by `command` or `package` states `keyed`, the\n"
    "# hooks whose calls carry Conduit's keys (PKG-020); an item that is not plain carries them on\n"
    "# every call. This item is not marked plain and lists its sign-in hook under `keyed`.\n"
    "#\n"
    "# This file carries no change narrative; the reasons are in the records listed in spec/README.md, "
    "'Where the reasons are'.\n"
    "\n"
    "apiVersion: github.com/shrav-k/conduit/v0\n"
    "address: example.org/agent\n"
    "version: 0.2.0\n"
    "implementations:\n"
    "  wrapper:\n"
    "    implements:\n"
    "      - '#turn.v1'\n"
    "      - github.com/shrav-k/conduit/core#sign-in-hook.v1\n"
    "    command: [agent-wrapper]\n"
    "    keyed: [github.com/shrav-k/conduit/core#sign-in-hook.v1]\n")
replace_once("engine/crates/loader/tests/expected/invalid-verdicts.txt",
             "PKG-020-manifest-plain-with-supports.yaml schema\n",
             "PKG-020-manifest-keyed-not-plain.yaml schema\n"
             "PKG-020-manifest-plain-with-supports.yaml schema\n")

# Decision 3: D in PKG-020 (ruling 86, point 2, and the coordinator's narrowing).
replace_once("spec/promises/packages.yaml",
             "    It states no `supports`. A plain implementation is reached by `command`, `package` or\n"
             "    `remote`, and implements no operation of `runtime-provider.v1`, `machine-provider.v1` or\n"
             "    `package-source.v1`.\n",
             "    It states no `supports`. A plain implementation is reached by `command`, `package` or\n"
             "    `remote`, and implements no operation of `runtime-provider.v1`, `machine-provider.v1` or\n"
             "    `package-source.v1`. One reached by `command` or `package` may state `keyed`, a list of\n"
             "    hooks, each a selection in its `implements` and a key of its `tools` that names\n"
             "    `setup-hook.v1`, `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` or\n"
             "    `sign-out-hook.v1`: the engine's calls for a hook that `keyed` lists carry Conduit's keys,\n"
             "    and its other calls to the implementation do not.\n")
replace_once("spec/promises/packages.yaml",
             "    that is not marked `plain` and states `tools`, are each rejected when the package is\n"
             "    installed or the project is submitted. The operations of those capabilities are reached only\n",
             "    that is not marked `plain` and states `tools`, are each rejected when the package is\n"
             "    installed or the project is submitted, and so are an item that states `keyed` and is not\n"
             "    marked `plain` or is reached by `remote`, and a selection in `keyed` that is not a key of\n"
             "    `tools` or names an operation other than those five hooks. The operations of those\n"
             "    capabilities are reached only\n")
replace_once("spec/promises/packages.yaml",
             "    are the operation's inputs under their own names, as IFC-001 states. A plain server reads\n"
             "    none of Conduit's keys, so it learns its instance's configuration only through the values\n"
             "    that its item's `environment`, `packageArguments` or `headers` take from it, under MAN-010.\n",
             "    are the operation's inputs under their own names, as IFC-001 states. A plain server is\n"
             "    sent none of Conduit's keys except on a call for a hook that `keyed` lists, so it learns\n"
             "    its instance's configuration only through the values that its item's `environment`,\n"
             "    `packageArguments` or `headers` take from it, under MAN-010, and on such a call through\n"
             "    IFC-006's keys.\n")
replace_once("spec/promises/packages.yaml",
             "    as before, under PRV-025, and a `remote` server that cannot check an enforcement point's\n"
             "    proof is used as plain, under IFC-041.\n",
             "    as before, under PRV-025, and a `remote` server that cannot check an enforcement point's\n"
             "    proof is used as plain, under IFC-041. For a call for a hook that `keyed` lists, the\n"
             "    implementation is one that carries Conduit's keys: the engine reads its result of\n"
             "    `server/discover` before the first such call to each run and declares Conduit's\n"
             "    extension on the call, under IFC-037, the call holds the keys of IFC-006, and the\n"
             "    implementation keeps IFC-008, IFC-009, IFC-019, IFC-026 and IFC-047 for it, and a keyed\n"
             "    setup or retire hook is given its idempotency key and new attempts under OPS-028. Every\n"
             "    other call to it is plain, under IFC-007 and IFC-010. `keyed` lists hooks alone, so a\n"
             "    request for an operation of a plain implementation, its result and its repeat stay plain,\n"
             "    under IFC-025 and OPS-012; and an item reached by `remote` states none, so a gate stands\n"
             "    before it, under IFC-041, for all its calls or for none. So one item that is run serves an\n"
             "    Agent's plain turns beside a sign-in-hook.v1 that carries Conduit's keys, under TPL-019.\n")

# Decision 4: D in the interface's promises: each reads plain per call (ruling 86, point 2).
IMP = "spec/interface/implementation.yaml"
replace_once(IMP,
             "    `io.github.shrav-k.conduit/operation`, which IFC-006 sends with every request. A plain\n"
             "    implementation is sent no such key, and its `tools` map names a tool for each operation. The\n",
             "    `io.github.shrav-k.conduit/operation`, which IFC-006 sends with every request. A plain\n"
             "    implementation is sent no such key, except on a call for a hook that its item's `keyed`\n"
             "    lists (PKG-020), and its `tools` map names a tool for each operation. The\n")
replace_once(IMP,
             "    name or, for an implementation marked `plain`, the name its `tools` map gives under PKG-020.\n",
             "    name or, for an implementation marked `plain`, the name its `tools` map gives under PKG-020,\n"
             "    on every call, one for a hook that its item's `keyed` lists among them.\n")
replace_once(IMP,
             "    not declare a capability it requires. For a call to an implementation marked `plain`, it is\n",
             "    not declare a capability it requires. For a call to an implementation marked `plain`, other\n"
             "    than one for a hook that its item's `keyed` lists (PKG-020), it is\n")
replace_once(IMP,
             "    sides support. A plain implementation is declared no extension, under IFC-007, so a revision\n"
             "    that has no negotiation of extensions, such as 2025-11-25 and those before it, may serve a\n"
             "    call to one where both sides support it,",
             "    sides support. A call that IFC-007 governs is declared no extension, so a revision\n"
             "    that has no negotiation of extensions, such as 2025-11-25 and those before it, may serve\n"
             "    such a call where both sides support it,")
replace_once(IMP,
             "    An engine sends a call for a request or a hook to an implementation that carries Conduit's\n"
             "    keys.\n",
             "    An engine sends a call for a request or a hook to an implementation that carries Conduit's\n"
             "    keys, or a call for a hook that the `keyed` of an implementation marked `plain` lists\n"
             "    (PKG-020).\n")
replace_once(IMP,
             "    An engine sends a call for a request or a hook to an implementation marked `plain`.\n",
             "    An engine sends a call for a request or a hook to an implementation marked `plain`, other\n"
             "    than a call for a hook that its item's `keyed` lists (PKG-020).\n")
replace_once(IMP,
             "  when: >-\n    An implementation that carries Conduit's keys returns a result.\n",
             "  when: >-\n    An implementation that carries Conduit's keys returns a result, or an implementation marked\n"
             "    `plain` returns one for a call for a hook that its item's `keyed` lists (PKG-020).\n")
replace_once(IMP,
             "    An implementation marked `plain` answers a `tools/call` request.\n",
             "    An implementation marked `plain` answers a `tools/call` request, other than one for a hook\n"
             "    that its item's `keyed` lists (PKG-020).\n")
replace_once(IMP,
             "    Before its first call to each run of the implementation, and to an item reached by `remote`\n"
             "    at each connection that a setup or a retirement makes to it under IFC-041, the engine reads\n",
             "    Before its first such call to each run of the implementation, which for an implementation\n"
             "    marked `plain` is its first call for a hook that its item's `keyed` lists (PKG-020), and to\n"
             "    an item reached by `remote` at each connection that a setup or a retirement makes to it\n"
             "    under IFC-041, the engine reads\n")
replace_once(IMP,
             "    to sending such a call as it would to a plain implementation: only an implementation\n"
             "    marked `plain` is sent none of Conduit's keys, under IFC-007, and the engine declares no\n"
             "    extension of Conduit's to it. What else",
             "    to sending such a call as it would to a plain implementation: only a call that IFC-007\n"
             "    governs, to an implementation marked `plain` for no hook that its item's `keyed` lists, is\n"
             "    sent none of Conduit's keys, and the engine declares no extension of Conduit's on it. Where\n"
             "    the discovery result of an implementation marked `plain` whose item states `keyed` does not\n"
             "    list the extension, only the hooks that `keyed` lists are not dispatched, and its other\n"
             "    calls are sent as before. What else")
replace_once(IMP,
             "    A call to an implementation marked `plain` declares no extension of Conduit's, under\n"
             "    IFC-007 and IFC-037, so it declares only",
             "    A call to an implementation marked `plain`, other than one for a hook that its item's\n"
             "    `keyed` lists (PKG-020), declares no extension of Conduit's, under IFC-007 and IFC-037, so\n"
             "    it declares only")
replace_once(IMP,
             "    An implementation that carries Conduit's keys answers a call that the engine sent it\n"
             "    with the protocol's error -32021,",
             "    An implementation that carries Conduit's keys answers a call that the engine sent it, or one\n"
             "    marked `plain` answers a call for a hook that its item's `keyed` lists (PKG-020), with the\n"
             "    protocol's error -32021,")

# Decision 5: D in PND-013's first check (ruling 86, point 2).
replace_once("spec/promises/pending.yaml",
             "    first, the relayed login's return is redirect or page, it states carried: true, and its\n"
             "    sign-in-hook.v1 implementation carries Conduit's keys (IFC-007), else not-accepted;\n",
             "    first, the relayed login's return is redirect or page, it states carried: true, and its\n"
             "    sign-in-hook.v1 carries Conduit's keys, its implementation not marked `plain` or its item's\n"
             "    `keyed` listing sign-in-hook.v1 (IFC-007, PKG-020), else not-accepted;\n")
# Decision 6: IFC-010's then (point 3 (b)).
replace_once("spec/interface/implementation.yaml",
             "    engine does not recognize, any other error but one after which the engine sends the call once\n"
             "    more under IFC-046, and no result at all, is unknown.\n",
             "    engine does not recognize, any other error but one after which the engine sends the call once\n"
             "    more under IFC-046, and no result at all, is unknown. An `input_required` result holding a\n"
             "    URL-mode input request from the sign-in-hook.v1 part of sign-in.v1 ends nothing of itself:\n"
             "    it is relayed as IFC-016 and PND-010 state or, where it fails a check of IFC-049, ends\n"
             "    sign-in.v1 failed under IFC-049; one from any other part of sign-in.v1 is unknown under\n"
             "    this table; for a plain implementation, IFC-016's relay of URL-mode elicitation arising as\n"
             "    a part of sign-in.v1 is this one.\n")

# Decision 7: PND-010's then refuses a login name aimed at a plain hook (point 3 (c), under D).
replace_once("spec/promises/pending.yaml",
             "    Before dispatching any part, a signInLogin naming no login the Type states reports invalid,\n"
             "    with a diagnostic naming PND-010 and the stated login names. Before dispatching any part,\n"
             "    the request reports unavailable where an in-progress sign-in.v1 invocation may start a\n",
             "    Before dispatching any part, a signInLogin naming no login the Type states reports invalid,\n"
             "    with a diagnostic naming PND-010 and the stated login names; one naming a stated login\n"
             "    reports invalid where the instance's sign-in-hook.v1 is a plain hook, its implementation\n"
             "    marked `plain` and its item's `keyed` not listing sign-in-hook.v1 (PKG-020), with a\n"
             "    diagnostic naming PND-010 and the login, and saying that a plain hook gets no login name\n"
             "    (IFC-007) and that the request may be sent without signInLogin. Before dispatching any\n"
             "    part, the request reports unavailable where an in-progress sign-in.v1 invocation may\n"
             "    start a\n")

# Decision 8: PND-010's observable, a failed check goes on, and a plain hook gets no name
# (points 3 (c) and 3 (d)).
replace_once("spec/promises/pending.yaml",
             "    succeeds with output account and answers any sign-in action signed-in. If that check\n"
             "    succeeds with no account or another account, the engine invokes sign-in-hook.v1 as a\n"
             "    part and relays its URL elicitation under IFC-049; that initial mismatch does not undo\n"
             "    the account or end sign-in.v1. Another account found there raises the instance's sign-in\n"
             "    action under PND-005, unless one is unanswered, and counts as no account yet while\n"
             "    sign-in.v1 polls (PND-011). Each retry polls account-hook.v1 to check the account\n"
             "    after the person's sign-in, under PND-011, until an account appears or DL18 ends; no\n"
             "    account yet is not failed. The same invocation may answer another InputRequiredResult or\n",
             "    succeeds with output account and answers any sign-in action signed-in. If that check\n"
             "    succeeds with no account or another account, or fails or ends unknown, the engine invokes\n"
             "    sign-in-hook.v1 as a part and relays its URL elicitation under IFC-049; that initial\n"
             "    mismatch or failure does not undo the account or end sign-in.v1. A check there that\n"
             "    failed or ended unknown counts as no account: it raises nothing that PND-005 does not\n"
             "    raise for a contract that reports a missing or lapsed login, and clears no condition.\n"
             "    Another account found there raises the instance's sign-in action under PND-005, unless\n"
             "    one is unanswered, and counts as no account yet while sign-in.v1 polls (PND-011). Each\n"
             "    retry polls account-hook.v1 to check the account after the person's sign-in, under\n"
             "    PND-011, until an account appears or DL18 ends; no account yet is not failed, and a poll\n"
             "    that failed or ended unknown counts as no account yet, so that only a later check that\n"
             "    reports the expected account completes sign-in.v1. The same invocation may answer another\n"
             "    InputRequiredResult or\n")
replace_once("spec/promises/pending.yaml",
             "    The engine passes the asked-for name to sign-in-hook.v1 (IFC-006), which starts that login\n"
             "    or fails with a diagnostic naming PND-010 and why it could not. Without a name the hook\n"
             "    chooses among the Type's logins. A plain hook gets no Conduit keys (IFC-007), hence no\n"
             "    name; its URL for another login fails IFC-049's match. This includes asking for a device\n"
             "    code where the account permits it. The return goes only through the Interface that\n",
             "    The engine passes the asked-for name to sign-in-hook.v1 (IFC-006), which starts that login\n"
             "    or fails with a diagnostic naming PND-010 and why it could not. This includes asking for a\n"
             "    device code where the account permits it. Without a name the hook chooses among the Type's\n"
             "    logins. A plain hook, one whose implementation is marked `plain` and whose item's `keyed`\n"
             "    does not list sign-in-hook.v1, gets no Conduit keys (IFC-007), hence no name, so a request\n"
             "    that asks it for a login reports invalid before any part, and no login starts that the\n"
             "    person did not ask for; without a name its URL still matches one stated login under\n"
             "    IFC-049. A sign-in-hook.v1 that its item's `keyed` lists gets them, its item's other calls\n"
             "    staying plain (PKG-020). The return goes only through the Interface that\n")

# Decision 9: PND-011, before the hook, in setup and while polling (point 3 (d)).
replace_once("spec/promises/pending.yaml",
             "    Only the expected account, compared ignoring ASCII case, completes sign-in. The check\n"
             "    before sign-in-hook.v1 follows PND-010: no account or another account proceeds to that\n"
             "    hook. During polling after its URL elicitation, the expected account succeeds with output\n",
             "    Only the expected account, compared ignoring ASCII case, completes sign-in. The check\n"
             "    before sign-in-hook.v1 follows PND-010: no account, another account, or a check that\n"
             "    failed or ended unknown proceeds to that hook. During polling after its URL elicitation,\n"
             "    the expected account succeeds with output\n")
replace_once("spec/promises/pending.yaml",
             "    pending with the attempt as evidence. A failed or unknown\n"
             "    account hook does not complete sign-in, and its uncertainty is explained; no account yet\n"
             "    during polling follows PND-010.\n",
             "    pending with the attempt as evidence. A failed or unknown\n"
             "    account hook completes no sign-in and clears no condition, and its uncertainty is\n"
             "    explained. It counts as no account before sign-in-hook.v1 and in setup, and as no account\n"
             "    yet during polling; no account yet during polling follows PND-010.\n")
replace_once("spec/promises/pending.yaml",
             "    no account or a different account raises the action under PND-005, and a different\n"
             "    account is undone by a sign-out-hook.v1 part before setup proceeds.\n",
             "    no account or a different account raises the action under PND-005, and a different\n"
             "    account is undone by a sign-out-hook.v1 part before setup proceeds. A setup check that\n"
             "    failed or ended unknown raises the action under PND-005 as no account does, undoes\n"
             "    nothing, and lets setup proceed to its setup hook (TPL-012).\n")

# Decision 10: PND-005's when, TPL-012 and OPS-022, setup's check alike (point 3 (d), the coordinator's
# reading).
replace_once("spec/promises/pending.yaml",
             "    An instance whose Type states signIn has a string at its account pointer, and setup's\n"
             "    account-hook reports no account or a different account; or the account-hook.v1 part that\n",
             "    An instance whose Type states signIn has a string at its account pointer, and setup's\n"
             "    account-hook reports no account or a different account, or fails or ends unknown; or the\n"
             "    account-hook.v1 part that\n")
replace_once("spec/promises/templates.yaml",
             "    every part that comes before it in that order, and is not marked optional, has succeeded. No\n"
             "    part is begun once the instance is retired, or once the setup is cancelled under OPS-027.\n",
             "    every part that comes before it in that order, and is not marked optional, has succeeded; an\n"
             "    account check that failed or ended unknown counts here as one that succeeded with no\n"
             "    account (PND-011). No part is begun once the instance is retired, or once the setup is\n"
             "    cancelled under OPS-027.\n")
replace_once("spec/promises/templates.yaml",
             "    succeeded: one that failed, ended unknown, was cancelled or was not dispatched leaves every\n"
             "    later part not begun, and the setup's outcome is as OPS-022 gives it. The run and the\n",
             "    succeeded: one that failed, ended unknown, was cancelled or was not dispatched leaves every\n"
             "    later part not begun, an account check that failed or ended unknown apart, which counts as\n"
             "    one that succeeded with no account, and the setup's outcome is as OPS-022 gives it. The run\n"
             "    and the\n")
replace_once("spec/promises/templates.yaml",
             "    those parts. No account or a different account raises the sign-in action under PND-005;\n"
             "    setup does not wait for the person to sign in. A run is asked for only once the declared\n",
             "    those parts. No account, a different account, or an account check that failed or ended\n"
             "    unknown raises the sign-in action under PND-005; setup does not wait for the person to\n"
             "    sign in, and OPS-022 counts such a check as succeeded. A run is asked for only once the\n"
             "    declared\n")
replace_once("spec/conventions/operations.yaml",
             "    Its outcome is succeeded only if every part not marked optional succeeded, failed only if no\n"
             "    part had an effect, and otherwise unknown.",
             "    Its outcome is succeeded only if every part not marked optional succeeded, an account check\n"
             "    that failed or ended unknown counting as succeeded under TPL-012, failed only if no part had\n"
             "    an effect, and otherwise unknown.")
replace_once("spec/conventions/operations.yaml",
             "    or retirement can succeed without dispatching it again, and one that failed and is not\n"
             "    dispatched again counts as failed.\n",
             "    or retirement can succeed without dispatching it again, and one that failed and is not\n"
             "    dispatched again counts as failed. An account check that failed or ended unknown counts as\n"
             "    succeeded only in the setup that dispatched it; a further setup checks the account again\n"
             "    under TPL-012.\n")

# Decision 11: the definitions and sign-in.v1's description follow.
replace_once("spec/definitions.md",
             "Before sign-in-hook, no account or another account proceeds to that hook, and another account "
             "raises the sign-in action; during polling after its URL elicitation, the account found before "
             "sign-in-hook counts as no account yet, any other account is undone and sign-in fails with the "
             "action pending. Setup raises an action on no account or another account and undoes another "
             "account. A failed undo",
             "Before sign-in-hook, no account, another account or a check that failed or ended unknown "
             "proceeds to that hook, and another account raises the sign-in action; during polling after its "
             "URL elicitation, the account found before sign-in-hook, and a check that failed or ended "
             "unknown, count as no account yet, any other account is undone and sign-in fails with the action "
             "pending. Setup raises an action on no account, another account or a check that failed or ended "
             "unknown, undoes another account, and goes on to its setup hook once any undo has succeeded. A "
             "check that failed or ended "
             "unknown completes nothing and clears no condition. A failed undo")
replace_once("spec/definitions.md",
             "A part is begun only once every earlier part that is not optional has succeeded; the run",
             "A part is begun only once every earlier part that is not optional has succeeded, an account "
             "check that failed or ended unknown counting as one that found no account; the run")
replace_once("spec/core/operations.yaml",
             "    account-hook runs first; the expected account succeeds; no account or another account proceeds\n"
             "    to sign-in-hook, which relays a URL action. Retries poll for the account; a wrong account found\n"
             "    then is undone and the invocation fails with the action still pending. Failed undo holds model\n",
             "    account-hook runs first; the expected account succeeds; no account, another account or a\n"
             "    failed or unknown check proceeds to sign-in-hook, which relays a URL action. Retries poll for\n"
             "    the account, a failed or unknown poll counting as none yet; a wrong account found then is\n"
             "    undone and the invocation fails with the action still pending. Failed undo holds model\n")
replace_once("spec/core/operations.yaml",
             "    The optional request _meta signInLogin asks for one stated login; an unknown name reports\n"
             "    invalid before any part, with the stated names. Without one the hook chooses. Before any\n",
             "    The optional request _meta signInLogin asks for one stated login; an unknown name reports\n"
             "    invalid before any part, with the stated names, and so does any name where the instance's\n"
             "    sign-in-hook is plain, its item not listing it under keyed, since a plain hook gets no\n"
             "    name. Without one the hook chooses. Before any\n")
replace_once("spec/core/README.md",
             "The person may ask for a login with signInLogin; fixed-port logins are serialized (PND-010).\n",
             "The person may ask for a login with signInLogin, except where the instance's sign-in-hook is\n"
             "a plain hook, one that its item does not list under `keyed`: it gets no name, and the request\n"
             "reports invalid before any part. Fixed-port logins are serialized (PND-010).\n")

# Decision 12: TPL-012's and OPS-022's rows in the model-checking account.
replace_once("experiments/012-model-checking/account.yaml",
             "- id: TPL-012\n  class: safety\n  standing: holds, not graded\n",
             "- id: TPL-012\n  class: safety\n  standing: text changed, not rerun\n  was: holds, not graded\n")
replace_once("experiments/012-model-checking/account.yaml",
             " are also absent.'\n- id: TPL-013\n",
             " are also absent. Order 207 (ruling 86, point 3 (d)): a setup account check that failed"
             " or ended unknown counts as one that succeeded with no account; text changed, not rerun.'\n"
             "- id: TPL-013\n")
replace_once("experiments/012-model-checking/account.yaml",
             "- id: OPS-022\n  class: safety\n  standing: 'contradiction: MC6-001'\n",
             "- id: OPS-022\n  class: safety\n  standing: text changed, not rerun\n  was: 'contradiction: MC6-001'\n")
replace_once("experiments/012-model-checking/account.yaml",
             "    finding into the specification, not yet run.'\n- id: OPS-023\n",
             "    finding into the specification, not yet run. Order 207 (ruling 86, point 3 (d)): a setup account"
             " check that failed or ended unknown counts as succeeded; text changed, not rerun.'\n"
             "- id: OPS-023\n")




# Decision 13: four cases: D's manifest accepted; a non-hook in `keyed`, (c) and (d) in setup,
# each an expected failure until order 209.
def evidence(n, base, prefix, letter):
    return {
        "time": "2026-11-12T12:00:00Z",
        "ids": ["00000000-0000-4000-8000-%012d" % (base + 100 * n + k) for k in range(100)],
        "revisions": [f"{prefix}-{n}-{j}" for j in range(16)],
        "suffixes": [f"{letter}{100 * n + j:04d}" for j in range(16)],
    }


def step(source, number):
    return copy.deepcopy(source["steps"][number - 1])


def invoke(tool, arguments, extra=None):
    request = {"identity": {"kind": "instance", "id": "00000000-0000-4000-8000-000000000005"},
               "tool": tool, "arguments": arguments}
    request.update(extra or {})
    return {"kind": "invoke", "request": request}


def with_evidence(value, n, base, prefix, letter):
    value["evidence"] = evidence(n, base, prefix, letter)
    return value


def case(directory, head, steps, files):
    path = REQ / directory
    assert not path.exists(), path
    path.mkdir()
    for name, value in files.items():
        write_json(path / name, value)
    value = dict(head)
    value["steps"] = steps
    ids = [i for s in steps for i in s["input"].get("evidence", {}).get("ids", [])]
    assert len(ids) == len(set(ids)), directory
    write_json(path / "case.yaml", value)


hold = load(REQ / "sign-in-hold/case.yaml")
setup = load(REQ / "sign-in-setup-account/case.yaml")
refusals = load(REQ / "sign-in-refusals-before-parts/case.yaml")
HOLD_FILES = {"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json",
              "documents": "../sign-in-hold/documents.json"}
LAPSE = "../sign-in-lapse-and-conflict/"

plain = load(REQ / "sign-in-lapse-and-conflict/manifest.json")
item = plain["implementations"]["signed-server"]
assert "plain" not in item and "tools" not in item and "keyed" not in item, item
item["plain"] = True
item["tools"] = {ref: ref.split("#")[1].split(".v1")[0].replace("-", "_") for ref in item["implements"]}
assert CORE + "sign-in-hook.v1" in item["tools"] and "#ping.v1" in item["tools"], item


def with_keyed(selection):
    value = copy.deepcopy(plain)
    item = value["implementations"]["signed-server"]
    item["keyed"] = [selection]
    item = {key: item[key] for key in ("command", "implements", "plain", "tools", "keyed")}
    value["implementations"]["signed-server"] = item
    return value


# 13a: requests-keyed-hook-manifest, ruling 86, point 2: a plain item lists its sign-in hook, and
# a stated login name goes on to its account check, which stays plain (point 3 (c) under D).
URL = {"clientCapabilities": {"elicitation": {"url": {}}}}
named = {"input": with_evidence(invoke("group.sign-in.v1", {"instance": {"kind": "Agent", "name": "signed"}},
                                       dict(URL, signInLogin={"name": "browser"})),
                                3, 140000, "keyed-hook", "k"),
         "expect": {"reply": {"pending": True},
                    "external_intents": [{"kind": "dispatch", "attempt": 1,
                                          "call": {"method": "tools/call", "params": {"name": "account_hook"}}}],
                    "absent": ["/external_intents/0/call/params/_meta/io.github.shrav-k.conduit~1operation"]}}
case("keyed-hook-manifest", {
    "id": "requests-keyed-hook-manifest",
    "layers": ["language", "convention"],
    "promises": ["PKG-020", "PND-010"],
    "title": "Register a plain implementation that lists its sign-in hook under keyed, a hook among its implements and a key of its tools, and send a stated login name on to its account check",
    "profile": "first-build",
    "initial": {"recipe": "empty"},
    "files": {"fixture": LAPSE + "fixture.json", "manifest": "manifest.json",
              "documents": LAPSE + "documents.json"}},
     [step(refusals, 1), step(refusals, 2), named], {"manifest.json": with_keyed(CORE + "sign-in-hook.v1")})

# 13b: requests-keyed-not-a-hook, ruling 86, point 2, as the coordinator narrows it.
refused = step(refusals, 2)
refused["expect"] = {"reply": {"accepted": False, "reason": "invalid",
                               "diagnostics": [{"ruleId": "PKG-020"}]},
                     "stateUnchanged": True,
                     "external_intents": [],
                     "required_promises": ["PKG-020"],
                     "record_facts": [{"type": "access-decision", "data": {"reason": "invalid", "dispatched": False}},
                                      {"type": "request-ended", "data": {"reason": "invalid", "dispatched": False}}]}
case("keyed-not-a-hook", {
    "id": "requests-keyed-not-a-hook",
    "layers": ["language"],
    "promises": ["PKG-020"],
    "title": "Refuse to register a plain implementation whose keyed lists an operation that is not a hook, as invalid naming PKG-020",
    "profile": "first-build",
    "initial": {"recipe": "empty"},
    "files": {"fixture": LAPSE + "fixture.json", "manifest": "manifest.json",
              "documents": LAPSE + "documents.json"}},
     [step(refusals, 1), refused], {"manifest.json": with_keyed("#ping.v1")})

# 13c: requests-sign-in-login-plain-hook, point 3 (c) under D.
REFUSED = {"reply": {"isError": True, "_meta": {META + "reason": "invalid"},
                     "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error"}]}},
           "external_intents": [],
           "record_facts": [{"type": "access-decision"},
                            {"type": "request-ended", "data": {"reason": "invalid", "ruleId": "PND-010"}}]}
steps = [step(refusals, 1), step(refusals, 2)]
for n, login in ((3, "browser"), (4, "device"), (5, "phone")):
    steps.append({"input": with_evidence(invoke("group.sign-in.v1", {"instance": {"kind": "Agent", "name": "signed"}},
                                                dict(URL, signInLogin={"name": login})),
                                         n, 140000, "plain-hook", "p"),
                  "expect": copy.deepcopy(REFUSED)})
case("sign-in-login-plain-hook", {
    "id": "requests-sign-in-login-plain-hook",
    "layers": ["convention", "interface"],
    "promises": ["PND-010", "IFC-007"],
    "title": "Refuse a login name aimed at a plain sign-in hook, one its item does not list under keyed, as invalid before any part, beside an unknown name, so that no login the person did not ask for starts",
    "profile": "first-build",
    "initial": {"recipe": "empty"},
    "files": {"fixture": LAPSE + "fixture.json", "manifest": "manifest.json",
              "documents": LAPSE + "documents.json"}},
     steps, {"manifest.json": plain})

# 13d: requests-sign-in-setup-check-failed, point 3 (d) in setup.
steps = [step(setup, n) for n in range(1, 6)]
steps.append(step(setup, 6))
steps[-1]["input"]["completion"]["result"] = {"isError": True, "content": [],
                                              "_meta": {META + "outcome": "failed"}}
steps[-1]["expect"] = {"reply": {"isError": False,
                                 "_meta": {META + "outcome": "succeeded", META + "invocation": "$setup1",
                                           META + "unanswered": 1}},
                       "external_intents": []}
assert steps[-1]["capture"] == {"signout1": "/reply/hook"}
del steps[-1]["capture"]
steps += [step(setup, n) for n in (8, 9, 12, 13, 14, 15, 16, 17, 20, 21)]
steps.append(step(setup, 22))
steps[-1]["input"]["completion"]["result"] = {"isError": True, "content": [],
                                              "_meta": {META + "outcome": "unknown"}}
steps[-1]["expect"]["external_intents"] = []
steps.append(step(setup, 23))
steps[-1]["expect"]["reply"]["structuredContent"]["status"]["conditions"] = [
    {"type": "Running"}, {"type": "SignInUndone", "status": "False", "reason": "SignOutFailed"}]
case("sign-in-setup-check-failed", {
    "id": "requests-sign-in-setup-check-failed",
    "layers": ["convention", "interface"],
    "promises": ["PND-011", "PND-005", "TPL-012", "OPS-022"],
    "title": "Count a setup account check that failed or ended unknown as no account: it raises the sign-in action, undoes nothing, lets the setup succeed and clears no condition",
    "profile": "first-build",
    "initial": {"recipe": "empty"},
    "files": HOLD_FILES}, steps, {})

# Decision 14: the coverage index and the expected failures.
CASES = (("keyed-hook-manifest", "requests-keyed-hook-manifest", ["PKG-020", "PND-010"]),
         ("keyed-not-a-hook", "requests-keyed-not-a-hook", ["PKG-020"]),
         ("sign-in-login-plain-hook", "requests-sign-in-login-plain-hook", ["PND-010", "IFC-007"]),
         ("sign-in-setup-check-failed", "requests-sign-in-setup-check-failed",
          ["PND-011", "PND-005", "TPL-012", "OPS-022"]))
index = load(REQ / "index.yaml")
for directory, cid, promises in CASES:
    assert all(c["id"] != cid for c in index["cases"]), cid
    index["cases"].append({"id": cid, "case": f"conventions/requests/{directory}/case.yaml",
                           "promises": promises, "safety_rows": []})
index["cases"].sort(key=lambda c: c["id"].encode())
write_json(REQ / "index.yaml", index)

expected = load("conformance/expected-failures.yaml")
expected["cases"] += [
    {"id": "requests-keyed-not-a-hook",
     "reason": "PKG-020 under ruling 86, point 2, as the coordinator narrows it to hooks: the core checks no implementation item's keyed, so it registers a plain item whose keyed lists an operation that is not a hook; order 209 refuses it, makes the case pass and removes this entry."},
    {"id": "requests-sign-in-login-plain-hook",
     "reason": "PND-010 under ruling 86, point 3 (c): the core refuses only a signInLogin that names no stated login, so one aimed at a plain sign-in-hook.v1 goes on to the account check; order 209 refuses it before any part, makes the case pass and removes this entry."},
    {"id": "requests-sign-in-setup-check-failed",
     "reason": "PND-011 and TPL-012 under ruling 86, point 3 (d): a setup account check that failed or ended unknown stops the setup, which ends failed or unknown and raises no sign-in action; order 209 counts it as no account, makes the case pass and removes this entry."},
]
expected["cases"].sort(key=lambda c: c["id"].encode())
write_json("conformance/expected-failures.yaml", expected)

# Decision 15: engine/CONFORMANCE.md records what the build lacks until order 209.
conformance = Path("engine/CONFORMANCE.md").read_text()
assert "(order 207)" not in conformance and conformance.endswith("\n")
Path("engine/CONFORMANCE.md").write_text(conformance + """
## Ahead of the engine: ruling 86 (order 207)

Order 207 wrote into `spec/` what this build does not do yet; order 209 builds it. Until it
does, each change is recorded as follows.

- **Keys per operation** (ruling 86, point 2): an implementation item marked `plain` and reached
  by `command` or `package` may list hooks under `keyed`, whose calls carry Conduit's keys while
  every other call to it stays plain (PKG-020, IFC-006, IFC-007, IFC-009, IFC-010, IFC-037,
  PND-013). The loader takes `manifest.v1` from `spec/`, so it accepts such an item
  (`requests-keyed-hook-manifest`) and rejects `keyed` on an item that is not plain
  (`PKG-020-manifest-keyed-not-plain.yaml`). The core checks nothing else of `keyed`: it
  registers one that lists an operation that is not a hook (`requests-keyed-not-a-hook`), and it
  reads `plain` per item, so it sends a hook that `keyed` lists as a plain call, with no
  discovery before it. Those calls have no case here: their steps need the discovery that order
  209 builds.
- PND-010 refuses as invalid, before any part, a `signInLogin` that names a stated login where
  the instance's `sign-in-hook.v1` is a plain hook. The core refuses only a name that no login
  states (`requests-sign-in-login-plain-hook`).
- PND-010, PND-011 and TPL-012 count an account check that failed or ended unknown as no
  account: before `sign-in-hook.v1` it goes on to that hook, while polling it counts as no
  account yet, and in setup it raises the sign-in action and lets the setup go on, clearing no
  condition in any of them. The core stops a setup at such a check, which then ends failed or
  unknown with no sign-in action (`requests-sign-in-setup-check-failed`), and its `sign-in.v1`,
  where the account check is built, ends failed at it, which step 24 of
  `requests-sign-in-account-first`, completing the check failed, asserts since order 048a-5b; order
  209 flips it.

IFC-010's relay of a URL request from a plain `sign-in-hook.v1` part has no case here: the hook
part and the relay are orders 048a-5d and 048a-5e, whose cases show it.
""")

# Decision 16: four rows of the owner's review list.
REVIEW = "docs/proposals/first-build/to-review-with-the-owner.md"
review = Path(REVIEW).read_text()
heading = "## From the targeted review of the sign-in surface\n"
assert review.count(heading) == 1 and "Order 207" not in review
head, table = review.split(heading)
lines = table.split("\n")
rows = [i for i, line in enumerate(lines) if line.startswith("| ") and line.split("|")[1].strip().isdigit()]
last = rows[-1]
assert all(i == rows[0] + k for k, i in enumerate(rows)), "the table's rows are not contiguous"
number = max(int(line.split("|")[1]) for line in review.split("\n")
             if line.startswith("| ") and line.split("|")[1].strip().isdigit()) + 1
ORDER = "[Order 207](../../work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md)"
lines[last + 1:last + 1] = [
    f"| {number} | **`keyed` lists hooks alone, on an item reached by `command` or `package`** (order 207; the coordinator's narrowing of option D, which [ruling 86](../ruling-86/README.md) rules as the route for keying, open to the owner's strike) | Option D lets a plain item carry Conduit's keys on some of its operations. Narrowed to the five core hooks, the request path keeps reading `plain` per item: a request's repeats under OPS-012, IFC-025's results and the gate stay as they are, and only hook dispatch reads `plain` per call. Narrowed to items that are run, no `remote` item stands behind a gate for some calls and not for others; IFC-043 and IFC-044, which ruling 86 named, stay unchanged, no `remote` item stating `keyed`. Ruling 82's o1 asks only for a keyed sign-in hook, so nothing it needs is lost. Striking it lets `keyed` list any operation of the item, and then OPS-012, IFC-025, IFC-041 and the gate must read `plain` per operation too, in `spec/` and in order 209 | {ORDER}; PKG-020, `manifest.v1`, spec/definitions.md |",
    f"| {number + 1} | **A setup account check that failed or ended unknown counts as no account: it raises the sign-in action, undoes nothing, and lets the setup go on and succeed** (order 207; the coordinator's reading of [ruling 86](../ruling-86/README.md), point 3 (d), that setup's check before its hook goes on alike, open to the owner's strike) | Ruling 86 sends a failed check before `sign-in-hook.v1` on to that hook. Setup checks before its own hook, and TPL-012 and OPS-022 stopped it at a failed part, so a home whose access token had lapsed, as the blocked refresh of `codex.v2`'s check makes likely, failed every setup. The reading holds the instance under PND-005 rather than let model work run on an account no check named; its cost is a sign-in asked for where only the access token lapsed. Striking it restores the setup texts of TPL-012, OPS-022, PND-005 and PND-011 and the rows account check and set up of spec/definitions.md, and removes the case `requests-sign-in-setup-check-failed` | {ORDER}; TPL-012, PND-005, PND-011 |",
    f"| {number + 2} | **A check that ended unknown, before sign-in-hook.v1 or while polling, counts as no account (yet), as a failed one does** (order 207; the coordinator's reading of [ruling 86](../ruling-86/README.md), point 3 (d), completion still needing a later check that names the account, open to the owner's strike) | PND-011 said only that a failed check completes nothing. With refresh blocked for `codex.v2`'s check, a poll made before the new login lands reads the old, lapsed login and fails, so ending `sign-in.v1` at the first such poll would end every such sign-in; the owner's option named only a failed check. Striking it restores PND-010's and PND-011's unknown ending, and leaves a failed poll's ending to the polling order, 048a-5f | {ORDER}; PND-010, PND-011, `sign-in.v1` |",
    f"| {number + 3} | **IFC-010 relays a URL request from the sign-in-hook.v1 part alone** (order 207; the coordinator's reading of [ruling 86](../ruling-86/README.md), point 3 (b), which relays a URL request inside a `sign-in.v1` part, open to the owner's strike) | IFC-049 matches only a `sign-in-hook.v1` URL against the Type's stated logins, and PND-010 relays the hook's elicitation alone, so a URL request from the account check has no match to pass and no relay that names it: it stays unknown under IFC-010's table, and an account check that so ends counts as no account under row {number + 2}. Striking it relays a URL request from any part of `sign-in.v1`, which needs IFC-049 to match a URL from the `account-hook.v1` part too | {ORDER}; IFC-010, IFC-016, IFC-049 |",
]
Path(REVIEW).write_text(head + heading + "\n".join(lines))
print(f"review rows {number} to {number + 3}")
print("edited 16 files; wrote 1 example, 4 cases and 3 manifests")
```

The script prints the first and last of the four row numbers it gave, then
`edited 16 files; wrote 1 example, 4 cases and 3 manifests`. It writes curly quotes into the
account row's note as the note already holds them; save it as UTF-8.

## Scope

Change only `spec/definitions.md` (the two rows "plain", the rows "implementation item",
"Conduit's extension", "account check" and "set up", and the new row "`keyed`"),
`spec/schemas/manifest.yaml` (`implementation-item.v1`), `spec/schemas/README.md` (one sentence
and one row), `spec/promises/packages.yaml` (PKG-020's `then` and `observable`),
`spec/interface/implementation.yaml` (IFC-001's `then` and `observable`, IFC-004's `then` and `observable`,
the `when` of IFC-006, IFC-007, IFC-009, IFC-010 and IFC-047, IFC-010's `then`, IFC-037's `then`
and `observable`, IFC-045's `observable`), `spec/promises/pending.yaml` (PND-005's `when`,
PND-010's `then` and `observable`, PND-011's `then` and `observable`, PND-013's `then`),
`spec/promises/templates.yaml` (TPL-012's `then` and `observable`),
`spec/conventions/operations.yaml` (OPS-022's `then` and `observable`),
`spec/core/operations.yaml` (`sign-in.v1`'s description), `spec/core/README.md` (one sentence),
`engine/crates/loader/tests/expected/invalid-verdicts.txt` (one line),
`experiments/012-model-checking/account.yaml` (TPL-012's and OPS-022's rows),
`conformance/conventions/requests/index.yaml`, `conformance/expected-failures.yaml`,
`engine/CONFORMANCE.md` (one section appended), `engine/STANDING.md`, written only by the tool,
`docs/proposals/first-build/to-review-with-the-owner.md` (four rows) and `IN_FORCE.md` (decision
17's lines). Create only `spec/schemas/examples/invalid/PKG-020-manifest-keyed-not-plain.yaml` and
the four case directories of decision 13, with `case.yaml` in each and `manifest.json` in all but
`sign-in-setup-check-failed/`. `docs/work-orders/207-declared.yaml` comes with this order; change
it only as decision 18 says. Anything else that needs changing goes in your report.

## Done when

From the repository root, after the script and decisions 17 and 18, each of these passes.
`/usr/bin/python3` is the Mac's interpreter with PyYAML; where `python3` has PyYAML, it serves as
well. On `7f56262a` the hashes, the counts and the runner's four cases each fail, since none of
this exists there; the schema there rejects the first case's manifest, `'keyed' was unexpected`.

This prints twenty lines, each ending `: OK`, and exits 0 (`sha256sum -c` on Linux):

```bash
shasum -a 256 -c <<'END'
9b1eeb3da56b054b76f03447f0db31fd2dd4ed5e4cec55a03ed4bd270ec3ecf3  conformance/conventions/requests/keyed-hook-manifest/case.yaml
3005f0d8c83f4249f6346db8bd64b2e823136caf214d2d1dd372f6a0134a819d  conformance/conventions/requests/keyed-hook-manifest/manifest.json
5875d330227bc30d4e3aa98162cec61b8bf552eb8b2f645ba651b8d83f33281e  conformance/conventions/requests/keyed-not-a-hook/case.yaml
f1c36c22eaa6956d5c4a468a18fc3ef943b5256138b08fb8ed07367e1c14369d  conformance/conventions/requests/keyed-not-a-hook/manifest.json
57ba64c913425f49cef998d7253c12c323406eafd4e105600c55d358a21b1617  conformance/conventions/requests/sign-in-login-plain-hook/case.yaml
e73638b22bfc5111dfeedcaffc7338a878df1bed625dedcbf03df7d15bbd3aa3  conformance/conventions/requests/sign-in-login-plain-hook/manifest.json
4270391afec76bea89e7a3d7b02fe2a2ef5027e91f656608bcac17f0f1ce0dbd  conformance/conventions/requests/sign-in-setup-check-failed/case.yaml
f4d30ef52905b06454914274d6b8e55bc561a791ef9aac69b44ae0cbe5b456e5  spec/schemas/examples/invalid/PKG-020-manifest-keyed-not-plain.yaml
1e09b16efea4b883889835bada77b5b95dce2524f02ee0d50fd400da0bf78876  spec/definitions.md
6dd97178ee34a8c42169b311aaa974762fab7442b325ab3f783bfa379a98d33f  spec/schemas/manifest.yaml
a8ec53632abc8e11f492924455e0c28ee3c6b28455c616957f785c8b91b91116  spec/schemas/README.md
bd6a0d4a2948b3362fe0022f462e9599d1580fa434da7895647fc6c5ad9f2b5b  spec/promises/packages.yaml
325410518e879da2c4035a5d5727b183b4d9092f24fdf4a3293dbe04abc63ad6  spec/interface/implementation.yaml
0538e40bd36e2ca789a33a46115571ca420e2dd75b500203206cd16b08c569f8  spec/promises/pending.yaml
93611f8ac83a92f2d511275267a0ec6de9c15bf5a4de764e498f2ca2691a4b30  spec/promises/templates.yaml
9e5bae76b197e77ab9d68c9fbc46c198930c68f606b553e71e12f25c1265297b  spec/conventions/operations.yaml
ac3005032874db60d7e28d33c84580659ec4755e9f5d324dc0a1fd806d5a414b  spec/core/operations.yaml
bab15559ec6d8c9e447951a569c46b1b870aa940f15cec43d4d88eaea649b6f9  spec/core/README.md
6070d5bb8693d208bb172d49d9efd645a0f4f58f9fa653820a0af2ae357098a3  experiments/012-model-checking/account.yaml
05361841f52fe68379d6a8e71fda4d7a8f83d83e091c1ff3b116ecbc800b892a  engine/crates/loader/tests/expected/invalid-verdicts.txt
END
```

The last twelve are the files of `7f56262a`, each unchanged since `c704275f`, with this order's
change; if one of them changed on `main` since, report its hash. The first eight must
still match: the cases copy only steps that no order beside this one changes.

Each of these prints 1. The `tr` checks read a file with its line breaks as spaces:

```bash
grep -c '^| .keyed. | The key of an implementation item that is marked .plain: true. and reached by .command. or .package., which lists the hooks whose calls carry Conduit' spec/definitions.md
grep -c '^      keyed: false$' spec/schemas/manifest.yaml
grep -c '^PKG-020-manifest-keyed-not-plain.yaml schema$' engine/crates/loader/tests/expected/invalid-verdicts.txt
tr -s ' \n' '  ' < spec/promises/packages.yaml | grep -c 'One reached by .command. or .package. may state .keyed., a list of hooks, each a selection in its .implements. and a key of its .tools.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c "An engine sends a call for a request or a hook to an implementation marked .plain., other than a call for a hook that its item's .keyed. lists (PKG-020)."
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c "the name its .tools. map gives under PKG-020, on every call, one for a hook that its item's .keyed. lists among them."
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c "Before its first such call to each run of the implementation, which for an implementation marked .plain. is its first call for a hook that its item's .keyed. lists (PKG-020)"
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'An .input_required. result holding a URL-mode input request from the sign-in-hook.v1 part of sign-in.v1 ends nothing of itself: it is relayed as IFC-016 and PND-010 state or, where it fails a check of IFC-049, ends sign-in.v1 failed under IFC-049; one from any other part of sign-in.v1 is unknown under this table; for a plain implementation, IFC-016'"'"'s relay of URL-mode elicitation arising as a part of sign-in.v1 is this one.'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c "reports invalid where the instance's sign-in-hook.v1 is a plain hook, its implementation marked .plain. and its item's .keyed. not listing sign-in-hook.v1 (PKG-020)"
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c "its implementation not marked .plain. or its item's .keyed. listing sign-in-hook.v1 (IFC-007, PKG-020), else not-accepted;"
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'succeeds with no account or another account, or fails or ends unknown, the engine invokes sign-in-hook.v1 as a part'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'a poll that failed or ended unknown counts as no account yet, so that only a later check that reports the expected account completes sign-in.v1.'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'A setup check that failed or ended unknown raises the action under PND-005 as no account does, undoes nothing, and lets setup proceed to its setup hook (TPL-012).'
tr -s ' \n' '  ' < spec/promises/templates.yaml | grep -c 'an account check that failed or ended unknown counts here as one that succeeded with no account (PND-011).'
tr -s ' \n' '  ' < spec/promises/templates.yaml | grep -c 'an account check that failed or ended unknown apart, which counts as one that succeeded with no account,'
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -c 'an account check that failed or ended unknown counting as succeeded under TPL-012'
grep -c '^## Ahead of the engine: ruling 86 (order 207)$' engine/CONFORMANCE.md
grep -A3 '^- id: TPL-012$' experiments/012-model-checking/account.yaml | grep -c '^  was: holds, not graded$'
```

These print `0`, `4`, `1`, `0` and `0`: no line is still marked spec pending, order 207, four
lines say the engine follows in order 209 and one that it follows in orders 048a-5d and 048a-5e,
and none of them still says what the spec says today or that an unknown check does not go on
(decision 17):

```bash
grep -ci 'spec pending, order 207' IN_FORCE.md
grep -c 'the engine follows in order 209' IN_FORCE.md
grep -c 'the engine follows in orders 048a-5d and 048a-5e' IN_FORCE.md
grep 'order 209\|048a-5e' IN_FORCE.md | grep -c ' today\|as it stands'
grep 'order 209\|048a-5e' IN_FORCE.md | grep -c 'until then it does not'
```

This prints exactly these lines, tab-separated, so that no other line of the fourteen files
changed (at `7f56262a`; the index and the expected failures differ where an order beside this one
has landed, and then report them):

```bash
git diff --numstat -- conformance/conventions/requests/index.yaml conformance/expected-failures.yaml engine/crates/loader/tests/expected/invalid-verdicts.txt experiments/012-model-checking/account.yaml spec/
```

```text
37	0	conformance/conventions/requests/index.yaml
12	0	conformance/expected-failures.yaml
1	0	engine/crates/loader/tests/expected/invalid-verdicts.txt
6	4	experiments/012-model-checking/account.yaml
6	3	spec/conventions/operations.yaml
3	1	spec/core/README.md
7	4	spec/core/operations.yaml
7	6	spec/definitions.md
39	20	spec/interface/implementation.yaml
25	6	spec/promises/packages.yaml
40	20	spec/promises/pending.yaml
11	5	spec/promises/templates.yaml
3	1	spec/schemas/README.md
13	2	spec/schemas/manifest.yaml
```

Each of these passes:

```bash
/usr/bin/python3 tools/check_schemas.py | tail -1
/usr/bin/python3 tools/set_names.py --check
/usr/bin/python3 tools/check_rulings.py
/usr/bin/python3 tools/check_docs.py
/usr/bin/python3 tools/check_roadmap.py
/usr/bin/python3 tools/check_model_account.py
/usr/bin/python3 tools/promise_standing.py --check
/usr/bin/python3 -m unittest discover -s tools/tests
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/207-declared.yaml | grep -v '^{'; echo "exit ${PIPESTATUS[0]}"
git diff --check
```

`check_schemas.py`, `set_names.py`, `check_rulings.py`, `check_docs.py`, `check_roadmap.py` and
`check_model_account.py` print `Errors: 0` on their last lines; `check_schemas.py`'s step 4
counts `78 invalid documents in 78 files`, one more than on `7f56262a`, with a line `rejected
PKG-020-manifest-keyed-not-plain.yaml` at `/implementations/wrapper`, and its step 8 still counts
520 promises. `promise_standing.py --check` prints
`engine/STANDING.md is current; promise rows: 520.` and `Errors: 0`. The unit tests end `OK` (129
tests at `7f56262a`; report the count). The vector delta prints
`Declarations read from docs/work-orders/207-declared.yaml: 4 shapes.`,
`Vector delta: 4 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`; run it under bash, whose
`PIPESTATUS` it reads. `git diff --check` prints nothing.

From `engine/`, one command at a time, each passes:

```bash
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-manifest
cargo run --release -p conduit-conformance -- .. requests-keyed-not-a-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-login-plain-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed
cargo run --release -p conduit-conformance -- .. requests-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The first prints `PASS requests-keyed-hook-manifest (3 steps; PKG-020, PND-010)` and
`1 cases passed`.
Each of the next three prints `EXPECTED FAILURE` with its case's ID and the reason decision 14
gives it, then one error line, and ends `0 cases passed; 1 expected failures`. On `7f56262a`
their errors are these, in order; report each as it prints:

```text
  missing ordered record {"data":{"dispatched":false,"reason":"invalid"},"type":"access-decision"}
  missing ordered record {"data":{"reason":"invalid","ruleId":"PND-010"},"type":"request-ended"}
  step 6: /reply/_meta/io.github.shrav-k.conduit/outcome: expected "succeeded", found "unknown"; ...
```

The first of them is step 2's: the core registers the item and records no refusal. The second is
step 3's: since order 048a-5b the core sends the named login on to the account check. The fifth
command lists the existing expected failures and these three, and fails no case: on the
coordinator's scratch copy of `7f56262a` it printed `71 cases passed; 5 expected failures`, and
the runner over every case printed `524 cases passed; 6 expected failures`. On `fde1c615`, before order 048a-5b merged, the
second error read `portable case: step 3 ends with reason unavailable but does not assert it and
its deciding ruleId`, and the two counts were `70 cases passed; 5 expected failures` and
`523 cases passed; 6 expected failures`; they are kept as a record only. The vectors report
`2 passed; 0 failed`, and no line of the full suite reports a failure; its loader tests read the
new example against the fixture line of decision 2. The threads are 4 because at 24 the runner
ran out of memory (order 071's Record).

Next, show that each of the three cases fails on the engine, and is excused only by its entry.
From the repository root, one at a time:

```bash
cp conformance/expected-failures.yaml engine/target/expected-failures.yaml.fixed
git show HEAD:conformance/expected-failures.yaml > conformance/expected-failures.yaml
(cd engine && cargo run --release -p conduit-conformance -- .. requests-keyed-not-a-hook)
(cd engine && cargo run --release -p conduit-conformance -- .. requests-sign-in-login-plain-hook)
(cd engine && cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed)
cp engine/target/expected-failures.yaml.fixed conformance/expected-failures.yaml
```

Each runner command must fail, with the error line it printed above. Then
`git diff --numstat -- conformance/expected-failures.yaml` prints its line above again.

The S1 short-form spike reads `spec/`, so from `experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`, and no `test result`
line reports a failure.

Last, after deleting `engine/target/order-207-write.py` and
`engine/target/expected-failures.yaml.fixed`, `git status --short` prints exactly these
twenty-three lines, in this order:

```text
 M IN_FORCE.md
 M conformance/conventions/requests/index.yaml
 M conformance/expected-failures.yaml
 M docs/proposals/first-build/to-review-with-the-owner.md
 M engine/CONFORMANCE.md
 M engine/STANDING.md
 M engine/crates/loader/tests/expected/invalid-verdicts.txt
 M experiments/012-model-checking/account.yaml
 M spec/conventions/operations.yaml
 M spec/core/README.md
 M spec/core/operations.yaml
 M spec/definitions.md
 M spec/interface/implementation.yaml
 M spec/promises/packages.yaml
 M spec/promises/pending.yaml
 M spec/promises/templates.yaml
 M spec/schemas/README.md
 M spec/schemas/manifest.yaml
?? conformance/conventions/requests/keyed-hook-manifest/
?? conformance/conventions/requests/keyed-not-a-hook/
?? conformance/conventions/requests/sign-in-login-plain-hook/
?? conformance/conventions/requests/sign-in-setup-check-failed/
?? spec/schemas/examples/invalid/PKG-020-manifest-keyed-not-plain.yaml
```

`docs/work-orders/207-declared.yaml` is tracked on the base, so it does not appear. Do not stage
anything. In your report, mark each choice you made yourself as your own, and
give the last line of every check, each exit status and each count above.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then adds the
new files with intent to add and runs `python3 tools/check_private_paths.py` and
`git diff --check`. Before the merge the admin runs the local gate that
[ruling 80](../proposals/ruling-80/README.md), point 10, has each merge wait on, `tools/gate.py`;
CI runs on the pull request as well, and its `docs` job reads `207-declared.yaml` through
`--declared-changed`. A fresh Codex verifier and a fresh read-only Opus reader read the result
before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2). Whether this order is on
the sign-in chain for A77 is the coordinator's to record, under A77's amendment of 2026-11-19.

## What follows

For the coordinator, not the worker:

- **Order 209**, the engine half (ruling 86, point 4, as the third answer amends it), dispatched
  only after this order merges (order 048a-5b merged first, as #387), and refreshed then:
  - D: the loader's rules for `keyed`, each selection a key of `tools` and one of the five core
    hooks, so that `requests-keyed-not-a-hook` passes; the core's `plain` read per call in hook
    dispatch (`dispatch_hook` in `requests.rs` sets it from the item today) and in hook re-dispatch
    (OPS-028), a keyed setup or retire hook given its idempotency key and new attempts, the request
    path keeping its per-item `plain`; IFC-037's discovery before the first call for a keyed hook to
    each run, a missing extension leaving only those hooks undispatched; discovery for a keyed hook
    is read just before that hook's first call, so that `requests-keyed-hook-manifest` step 3 (one
    plain account_hook dispatch) still holds; a build that reads it earlier changes that step as a
    declared change; and the keyed hook's call and result under IFC-006, IFC-008, IFC-009, IFC-019,
    IFC-026 and IFC-047, with cases of its own;
  - (c): refusing, after the unknown-name check, a `signInLogin` naming a stated login where the
    instance's `sign-in-hook.v1` is a plain hook;
  - (d): counting a setup account check that failed or ended unknown as no account (TPL-012,
    PND-005, PND-011); and, in `sign-in.v1` as order 048a-5b built it, sending such a check down
    the no-account path, except that it sets no `False` `SignInUndone` to `True` (PND-011: it
    clears no condition), which step 24 of `requests-sign-in-account-first` or a case of order
    209's own shows, so that until order 048a-5e its interim ending names PND-010, not PND-011:
    step 24 of `requests-sign-in-account-first`, which completes a check with a failed result,
    changes its expected `ruleId` accordingly, a flip that order 209 declares.

  It removes this order's three expected failures. It builds no keyed `content` copy.
- **The held drafts of the chain** follow the new text: 048a-5d dispatches `sign-in-hook.v1` after
  a failed or unknown check too, and its case relays a plain hook's URL request under IFC-010's
  new sentence; 048a-5f counts a failed or unknown poll as no account yet; 050m's `codex.v2` hook
  relies on (d) for an expired access token under ruling 86, point 5.
- **The later keyed sign-in release**: the Agent packages list `sign-in-hook.v1` under `keyed` and
  meet IFC-037, IFC-008, IFC-009, IFC-019, IFC-026 and IFC-047 on that tool alone; NAM-024's
  reading of that change is the owner's.
- **MC9a**: PND-005, PND-010, PND-011 and PND-013 differ from the text order 201 registered;
  whether MC9a is registered again before order 204 runs it is the coordinator's to decide under
  MC9a's own rule.
- **IN_FORCE.md's ruling 86 lines** for points 1, 4 and 5 are not this order's.

## Record

Not yet run.
