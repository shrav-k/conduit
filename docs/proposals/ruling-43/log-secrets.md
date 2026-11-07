# Ruling 43: log of the secrets group

Date: 2026-11-06. This is the account of how the editor of the secrets group applied
[ruling 43](README.md) to its files. The ruling accepts the
[proposal on work across machines](../multi-machine/README.md) with two revisions: D5's second
part is option b, and Q1 is tightened so that the engine's own address always accepts the token
form. Applied here: what [section 9, "In v0"](../multi-machine/README.md#in-v0) of the proposal
says for these files, with the detail of sections 1.2 to 1.5, 3, 4 and 5, and the items of the
[integration log of ruling 42](../ruling-42/log-integration.md) that waited on this ruling where
they touch these files. The section "In v1, for A65" is the tracker's, and is not applied. The
ruling is the owner's. The wording is the editor's, and stands unless the coordinator or the
owner strikes it. Nothing was committed.

Files: [`spec/promises/secrets.yaml`](../../../spec/promises/secrets.yaml),
[`spec/promises/manifest.yaml`](../../../spec/promises/manifest.yaml),
[`spec/promises/templates.yaml`](../../../spec/promises/templates.yaml),
[`spec/promises/packages.yaml`](../../../spec/promises/packages.yaml) and every file under
[`spec/conventions/`](../../../spec/conventions/). No other file was touched but this log.

| File | Before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| `secrets.yaml` | 23 | 8 | 1 (SEC-024) | 0 | 24 |
| `manifest.yaml` | 16 | 1 | 1 (MAN-018) | 0 | 17 |
| `templates.yaml` | 14 | 0 | 0 | 0 | 14 |
| `packages.yaml` | 21 | 1 | 0 | 0 | 21 |
| `conventions/access.yaml` | 29 | 2 | 0 | 0 | 29 |
| `conventions/lifecycle.yaml` | 35 | 1 | 0 | 0 | 35 |
| `conventions/observation.yaml` | 5 | 1 | 0 | 0 | 5 |
| `conventions/operations.yaml` | 31 | 1 | 0 | 0 | 31 |
| `conventions/records.yaml` | 6 | 1 | 0 | 0 | 6 |
| `conventions/implicit.yaml`, `conventions/revisions.yaml` | 19 | 0 | 0 | 0 | 19 |
| The group | 199 | 16 | 2 | 0 | 201 |

The two new IDs are those assigned in advance. No other ID was added, none was reused, and no
promise moved between files. Each changed file's head comment has a line for ruling 43 that names
what changed and points to this log; `templates.yaml`, `implicit.yaml` and `revisions.yaml` did
not change and have none.

## What changed

| # | What | Promises | Why, with the proposal's section |
| --- | --- | --- | --- |
| 1 | A template whose `remote` item binds a header to a place that `secret-reference.v1` governs is rejected unless the Remote's URL is `https`, or is `http` with the host `localhost`, an IPv4 address in `127.0.0.0/8` or `[::1]` on an engine that, as its conformance document states under DL17, can neither restart on another machine nor run as several parts. The check is made at registration, as SEC-021's is | SEC-024, new | D5's second part, option b, as the ruling revised it (3, "D5, second part"; the ruling's list). See [choices 1 to 4](#choices-beyond-the-record) |
| 2 | The observable says that the item's URL is one to which SEC-024 lets a secret header go | SEC-013 | Section 9, row "IFC-041, SEC-013", as revised to b |
| 3 | A read again after a restart reads the secret Resources that the setup which made the connection read for its headers, kept with the connection as state that holds no value, so a restart changes nothing that a connection carries; a place changed since by a control is read at a later setup's connection. The observable also cites SEC-024: a read again after a restart on another machine never sends a secret over `http` | SEC-023, `then` and observable | X11 of the verification of ruling 42, as the coordinator settles it; item 6 of "Still needs a decision" recommended the setup's references. D12 and 6, "D12" |
| 4 | Only an enforcement point, apart from what PRV-025 excepts, reaches the server of a secret Resource that a runtime provider other than the engine's own runner runs, or one reached by `remote` that carries Conduit's keys, so nothing else can send it a locator; a store reached by `remote` that cannot check the proof is used as plain. What a plain implementation learns its locator from now names `headers` too | SEC-006, observable | D5, A (3, "The gate, in substance"; 2, S2; 8, row "Secret provider"). The `headers` is a wording fix: PKG-020 already names it |
| 5 | The reads before a run also precede the run of a provider that the engine asks its own runner for again once it has restarted; a value reaches another machine only over the channel of PRV-011 | SEC-010, `when` and observable | Q3, b (5, Q3; 9, row "New, in `providers.yaml`; SEC-011"); 9, row "PRV-011, PRV-022" |
| 6 | Such a read belongs to the setup that last asked for that provider's run, has its caller, and is decided by the Conduit and its filters as they stand: the second exception to SEC-011, beside SEC-023 | SEC-011, `then` and observable | Q3, b, which the ruling calls a second exception to SEC-011; the ruling's table "Earlier rulings this changes", the row for ruling 41 (decision 4c) |
| 7 | Where such a read fails, the provider is not run and the provider call that needed it is not dispatched; the engine reads again before the next such call | SEC-012, observable | Q3, b; PRV-026's observable says the same |
| 8 | A provider's run that ended with the engine is a run that has ended: the run that PRV-026 asks for has reads of its own; nothing kept across a restart holds a value, and the engine reads again under PRV-026 | SEC-014, SEC-017, observables | Q3, b |
| 9 | The engine dispatches nothing for an ended invocation, apart from SEC-023's read and now the read for a provider's run made again under PRV-026 | ACC-019, `then` and observable | Q3, b; the ruling's table "Earlier rulings this changes", the row for ruling 41 (decision 4c) |
| 10 | The record of such a read names, as the invocation it serves, the setup that last asked for that run, and has its caller | REC-002, observable | Q3, b |
| 11 | The engine runs a provider again after a restart only for a provider call that it must make, and not because of a difference, so OBS-004 holds | OBS-004, observable | 5, Q3: "OBS-004's rule against acting on a difference alone is kept" |
| 12 | A restriction is enforced where the consumer reaches targets only through an enforcement point; it is mediated in that it applies to the requests that pass through one, which in this version is the engine alone; the gate keeps other callers from a target's server, but not an unconfined consumer from what the target's implementation reaches | ACC-027, `then` and observable | 1.2; 9, row "ACC-027, definitions" |
| 13 | A connection that the engine sees drop cancels the invocation, unknown for the engine; whether and when the implementation learns of it is its transport's and its gate's, whose contract states how soon it closes a stream once the engine's side is silent; a silent cut may reach it late or never | OPS-027, observable | 2, S5; 9, row "OPS-027" |
| 14 | The stop once more on unregistration gives `machine`, as the status of the retired node holds it, under PRV-027, so a provider that keeps no state of its runs can act; a stop that the provider owes outlives the engine's restart and the provider's retirement, under PRV-021 | LIF-036, `then` and observable | X10 of the verification of ruling 42 and item 5 of its "Still needs a decision"; 6, "D11"; 9, row "PRV-021, PRV-020" |
| 15 | A template whose item's `environment` binds `CONDUIT_URL` or `CONDUIT_IDENTITY_DIR`, the variables that `identity` carries under PRV-022, is rejected at registration | MAN-018, new | Q1, a, tightened: the names are fixed once; X3, and item 2 of "Needs a decision", of ruling 42's integration log. See [choices 5 and 6](#choices-beyond-the-record) |
| 16 | A server meant for another machine is published as a `package` item, since a relative program runs only on the engine's own runner; another runtime provider runs a `stdio` package only by bridging it behind its gate, and otherwise ends the run failed; pointers to MAN-018 and SEC-024 | MAN-010, observable | D20, A (4, "Recommendation: A"; 4, "A gap D20 did not raise"); 9, row "PRV-012, MAN-010" |
| 17 | A plain server reached by `remote` stands behind no gate and is reached as before; a `remote` server that cannot check the proof is used as plain | PKG-020, observable | The ruling's D5, A: "A server that is `plain` receives none of Conduit's keys and is reached as before"; 3, "Recommendation: A", cost |
| 18 | A line for ruling 43 in the head comment of each changed file | `secrets.yaml`, `manifest.yaml`, `packages.yaml`, `access.yaml`, `lifecycle.yaml`, `observation.yaml`, `operations.yaml`, `records.yaml` | The rule for every changed file |

`templates.yaml` needed no change: TPL-012 connects to `remote` items under IFC-041, which now
carries the engine's side of the gate, and TPL-018's connection anew after a restart is unchanged.
SEC-024 is a rejection of a template, as SEC-021 is, and stays in `secrets.yaml` with the rules on
secrets. D20's row 17 guard stays where it was, in PRV-007 and MAN-010, as section 4 recommends.

## The items of ruling 42's integration log

| Item | Outcome | Where |
| --- | --- | --- |
| "Needs a decision" 2, and X3: a variable named both in `identity` and in what the item binds | Settled: the names are fixed under Q1, so a template whose item binds one is rejected | MAN-018. The names are in PRV-022 and `spec/names.yaml`, the providers and core groups'; PRV-023 covers the list set on the engine's own runner |
| "Needs a decision" 5, what remained of it | Whether SEC-023 stops trying a read that keeps failing: settled without a change, as the text has it. SEC-012 has the engine read again before each request on the connection, until a later setup's connection takes its place or the retirement ends, and PRV-026's read for a provider follows the same rule before each call. MAN-017's check of a place whose schema states none of `type`, `enum` and `const` is not this ruling's, and stays open. The stop a provider holds when it is retired is PRV-021, the providers group's | SEC-012 (row 7 above) |
| X1: runs under one operating-system user can read each other's credential | Not in these files: IFC-022 and `spec/README.md` state Q5's exception | None here |
| X7: the engine keeps its hostname across a move | Not in these files: DL5 and DL17. SEC-024 relies on DL17's statement of whether the engine can restart on another machine | SEC-024 |
| X9: a credential lasts as long as its instance, and renewal is v1's | Not in these files: PRV-022 and DL13. No promise here relies on a refreshed credential | None here |
| X10: the machine in `stop.v1` and `inspect-run.v1` | Settled: the stop of LIF-036 gives `machine` from the retired node's status, under PRV-027 | LIF-036 |
| X11: which references SEC-023 reads | Settled: those the setup read, kept without values; IFC-041 keeps the values of the other headers as the setup took them (the interface group's) | SEC-023 |
| "For the research": D5, who may reach a run's `url` | The gate, PRV-025 (the providers group's); a secret Resource's server among them | SEC-006 |
| "For the research": D5, a secret header to `http` | SEC-024, option b | SEC-024, SEC-013, SEC-023 |
| "For the research": D20 | A, as the ruling has it: no new rule. The valid example's Pool template `workspace`, which routed a `runtime` input to an entry whose program is a relative path, is the core group's, which has taken that input out | MAN-010 (observable) |
| "For the research": a refreshed credential; the engine's address from another machine | PRV-022, IFC-042, DL13, DL17: not in these files | None here |
| "For the research": a run left on a machine that comes back | PRV-021, PRV-025, PRV-027 (the providers group's); LIF-036 here | LIF-036 |
| "For the research": an onward request of an earlier hook attempt on a machine cut off is decided for the latest caller | Stays a limit of this version, as OPS-028 states; local enforcement is v1's, recorded in A65 by the ruling's Q4 | None |
| "For the research": SEC-023's read crossing to the engine's new machine | Covered by the gate, under which the request and its result cross confidential and unaltered (PRV-025), and by SEC-024, under which no secret header goes to `http` on an engine that can move | SEC-006, SEC-023, SEC-024 |

## Names

No name was coined in these files.

- **The names of `identity`.** MAN-018 writes `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` as
  [`spec/names.yaml`](../../../spec/names.yaml) and PRV-022 fix them; the providers group's
  [log](log-providers.md#coined-names) and the core group's log record them as coined. MAN-018
  names only the two variables, since an item binds no file. Its observable describes the
  Kubernetes precedent without writing the file names `token` and `ca.crt` in backticks, so
  that `tools/set_names.py` does not count them as names of `identity`.
- **Terms.** "Enforcement point" and "gate" are used as the proposal (1.2, 1.3 and 3) and PRV-025
  use them: an enforcement point is what a request passes through on its way to a server, and
  which passes it on only once a decision point has permitted it, as NIST's policy enforcement
  point; a gate is what stands before a server that the engine does not run itself and admits
  only a request that carries an enforcement point's proof. "Decision point" is not used in
  these files. Neither term has a row in `spec/definitions.md` yet, and SEC-006, ACC-027,
  MAN-010, OPS-027 and PKG-020 use them: see [For other groups](#for-other-groups).
- **The loopback hosts of SEC-024** are a choice, not a name:
  [choice 3](#choices-beyond-the-record).

## Choices beyond the record

Each is this editor's, and the coordinator or the owner can strike it.

1. **SEC-024 rejects every other `http` URL at registration.** The ruling says that a secret
   header "is sent only to" `https` or an `http` loopback URL on an engine that cannot fail over,
   and that an engine that can fail over rejects the loopback item at registration. It does not
   say what becomes of an `http` URL to any other host on an engine that cannot fail over. Here
   it is rejected at registration too: the URL and whether the place is a secret's are known
   then, as SEC-021's check is, and a setup could never connect. The other reading is to accept
   the template and have the connection's part end failed at each setup.
2. **"Can fail over" read as DL17 reads it**: an engine that, as its conformance document states
   under DL17, can restart on another machine or run as several parts. `spec/definitions.md`
   lists "fail over" among the words the promises do not use, and DL17 and IFC-041 use the same
   words, as the providers group's choice 7 does.
3. **The loopback hosts.** `localhost`, an IPv4 address in `127.0.0.0/8`, and `[::1]`, compared
   without regard to case, as RFC 3986 compares a scheme and a host; no other name, even one that
   resolves to a loopback address. The Model Context Protocol's security best practices name
   `localhost`, `127.0.0.1` and `::1`; the whole of `127.0.0.0/8` is the IPv4 loopback block,
   and the ruling names `http://localhost`. IFC-041 says "a loopback address" and cites SEC-024
   for it.
4. **SEC-024 is of the layer `language`**, although whether a template is rejected depends on the
   engine's statement under DL17, as the ruling has it ("An engine that can fail over rejects
   such an item when the document is registered").
5. **MAN-018 rejects the template**, as the coordinator's settlement of X3, the integration
   log's option (c) and PRV-022's observable put it, and not the item when its manifest is read.
   Both reject the same documents in the end; the template's check is made where MAN-017's and
   SEC-021's are.
6. **MAN-018 covers `environment` alone**, since `identity` carries variables and files, and an
   item binds no file; an argument or a header is not covered.
7. **SEC-023 keeps which secret Resource the setup read for each header**, as state that holds
   no value, as X11's recommendation has it. A control that has changed the place since is read at
   a later setup's connection, as it is where the engine still holds the values.
8. **ACC-027's observable says why a restriction behind a gate is still only mediated**: the gate
   keeps other callers from a target's server, but a consumer that no runtime provider confines
   may still reach by its own means what the target's implementation reaches. Section 8 of the
   proposal leaves "enforced" to v1, where a provider confines a run.
9. **SEC-006's observable names `headers`** among what a plain implementation learns its locator
   from, as PKG-020 already does, for a plain store reached by `remote`.
10. **SEC-012, SEC-014, OBS-004 and REC-002** each gained a sentence for PRV-026, which the
    proposal's section 9 does not list, so that each says of the read for a provider's run what
    it already said of SEC-023's read, or, for OBS-004, why running a provider again is not
    acting on a difference.

## Needs a decision

1. **A secret Resource on the engine's own runner after the engine restarts.** The providers
   group's [open item 1](log-providers.md#needs-a-decision) states it; it touches SEC-010 and
   SEC-023 here. PRV-026 runs again only the three providers that provider calls reach, while the
   ruling's Q3 says "a provider", which `spec/definitions.md` defines to include a secret
   Resource. Where its process ends with the engine, as DL16 leaves to each engine, D12's read
   again under SEC-023, and every read through it, fails until an actor sets the secret Resource
   up again. This editor agrees with that log's recommendation, (b): PRV-026 also runs again,
   before a read through it, a secret Resource whose run on the engine's own runner ended with
   the engine. SEC-010, SEC-011 and SEC-023 would need no change of substance beyond naming that
   case.
2. **Templates registered before an engine is made able to move.** SEC-024 rejects, on an engine
   that can restart on another machine, a template that sends a secret header to `http`
   loopback, and DL17 says that no document changes when the engine is made able to move. A
   template registered while the engine could not move would then be one that it now rejects.
   Options: (a) DL17 states that an engine is made able to move only once no template that exists
   is one SEC-024 would then reject, so that the operator replaces such templates first; (b) on
   such an engine the connection's part reports unavailable for such a template; (c) leave it to
   each engine. Recommendation: (a), which keeps the secret off another machine's loopback and
   keeps the check at one place. It is DL17's, the interface group's file.
3. **MAN-017's check of a place whose schema states none of `type`, `enum` and `const`**, what
   remains of item 5 of ruling 42's list: unchanged, and not this ruling's.

## For other groups

- `spec/definitions.md`: rows for enforcement point and gate (row 12 and the terms above); the
  rows mediated and enforced, which follow ACC-027; the rows request, later work, caller and the
  engine's read of a secret, which name SEC-023's read as the one made after its invocation has
  ended and now need PRV-026's beside it (ACC-019, SEC-011, REC-002); the row connection, under
  which a read again is of the secret Resources the setup read (SEC-023); and the row fail over,
  which DL17 and SEC-024 now describe as a restart on another machine or several parts.
- `tools/check_schemas.py` and `spec/schemas/examples/invalid-by-rule/`: the core group has
  added a rule and an example each for SEC-024 and MAN-018, which read them as written here: an
  `http` URL to a host that is not a loopback address is rejected on every engine.
- `spec/README.md`: the counts, 24 promises in `secrets.yaml` and 17 in `manifest.yaml`; the
  limit on who may reach a store's server, which SEC-006 and PRV-025 now close; and a secret
  header to an `http` URL, which SEC-024 now bounds.
- DL17: open item 2 above.

## Checks

Run from the repository root at the end of this pass, with the other groups' files as they stood
then.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `447 promises ok` |
| Every text field folded with `>-`; a YAML line over 96 characters in these files | Yes; none |
| `python3 tools/check_docs.py` | `Errors: 2`, neither in these files: `spec/core/README.md` and `spec/schemas/README.md` link `docs/proposals/ruling-43/log-core.md`, which the core group had not yet written. This log's links resolve |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_schemas.py` | `Errors: 0`; 24 rules from 23 promises, 28 sets under `examples/invalid-by-rule/` each rejected by its rule, those of SEC-024 and MAN-018 among them |
| `git diff --check` | Clean |
