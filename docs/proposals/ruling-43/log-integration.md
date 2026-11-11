# Ruling 43: log of the integration

Date: 2026-11-06. This is the account of the pass that made the files of the four groups that
applied [ruling 43](README.md) agree: the [providers group](log-providers.md), the
[interface group](log-interface.md), the [secrets group](log-secrets.md) and the
[core group](log-core.md). The ruling is the owner's: it accepts the
[proposal on work across machines](../multi-machine/README.md) with D5's second part as option b
and Q1 tightened. This pass checked every citation between the groups' files, every new ID, the
coined names and every row of the proposal's [section 9, "In v0"](../multi-machine/README.md#in-v0)
but the row for A64 and A65, which is the tracker's; it wrote what no group owned,
`spec/definitions.md` and `spec/README.md`; and it fixed what disagreed. The section
[In v1, for A65](../multi-machine/README.md#in-v1-for-a65) is not applied: it is for the tracker.
The choices listed under [Choices of this pass](#choices-of-this-pass) are this pass's, and each
stands unless the coordinator or the owner strikes it. No record was edited but this log, and
nothing was committed.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) | `447 promises ok` |
| Every promise has exactly the seven fields, its text fields folded with `>-`; a YAML line over 96 characters under `spec/` | Yes; none |
| An ID cited under `spec/`, outside the comments that name removed IDs, that does not exist; a DL item cited that does not exist | None. PRV-025, PRV-026, IFC-042, DL17, SEC-024 and MAN-018 exist, and so do the groups' own PRV-027 and IFC-043 |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0`; `CONDUIT_URL` x13, `CONDUIT_IDENTITY_DIR` x10, `` `ca.crt` `` x8, `` `token` `` x8, `` `tls.crt` `` x7, `` `tls.key` `` x6 |
| `python3 tools/check_schemas.py` | `Errors: 0`; 24 rules from 23 promises, 28 sets under `examples/invalid-by-rule/` each rejected by its rule, 55 invalid documents each rejected |
| `git diff --check` | Clean |

## Counts

447 promises, seven more than after ruling 42 (PRV-025, PRV-026, PRV-027, IFC-042, IFC-043,
SEC-024, MAN-018): 201 of the layer `language`, 205 `convention` and 41 `interface`; by level,
399 `must`, 44 `must not`, 3 `may` and 1 `should`. None was removed or moved between files, and
no ID was reused. The implementation-defined list holds 16 items (DL17 added; DL14 not reused).
The groups changed 36 promises and added the seven; this pass changed three of the 36 again
(IFC-017, IFC-022, PRV-022) and the new PRV-025, and added none.

| File | Promises | Changed by the groups | Added |
| --- | --- | --- | --- |
| `promises/providers.yaml` | 27 | PRV-002, PRV-003, PRV-007, PRV-008, PRV-009, PRV-011, PRV-012, PRV-014, PRV-018, PRV-020, PRV-021, PRV-022, PRV-023 | PRV-025, PRV-026, PRV-027 |
| `promises/placement.yaml` | 21 | PLC-023, PLC-026 | None |
| `promises/secrets.yaml` | 24 | SEC-006, SEC-010, SEC-011, SEC-012, SEC-013, SEC-014, SEC-017, SEC-023 | SEC-024 |
| `promises/manifest.yaml` | 17 | MAN-010 | MAN-018 |
| `promises/packages.yaml` | 21 | PKG-020 | None |
| `conventions/*.yaml` | 125 | ACC-019, ACC-027, LIF-036, OBS-004, OPS-027, REC-002 | None |
| `interface/implementation.yaml` | 40 | IFC-017, IFC-022, IFC-030, IFC-032, IFC-041 | IFC-042, IFC-043 |
| `implementation-defined.yaml` | 16 items | DL5, DL13, DL16 | DL17 |

## What this pass changed

| File | What | Why |
| --- | --- | --- |
| `spec/definitions.md` | New rows: decision point, enforcement point, gate, trust bundle. Changed rows: engine (several parts under DL17, a promise binds the whole; the only decision point and enforcement point in this version), caller, mediated, enforced, request, implementation item, connection, plain, the engine's own runner, machine provider and fetching, the list set on the engine's own runner, runtime provider, run, `identity` (the fixed names; a credential named by ID, lasting as long as its instance; the channel), machine (`gone` only where known), the engine's read of a secret, domain of trust, names a version assigns, later work, fail over. The head paragraph names ruling 43, and says that the ruling kept `identity` and that the terms and the names of `identity` were coined | Section 9, rows "ACC-027, definitions", "IFC-022, PRV-018" and "PRV-011, PRV-022"; section 1.2 (the engine as several parts); every group asked for the rows of gate and enforcement point, and no group owned the file. Every term that the new promises use is now defined |
| `spec/README.md` | The status paragraph names ruling 43. "What this is" gives the two forms of proof, IFC-042, the fixed names and the gate. The paragraph on later work names PRV-026's reads beside SEC-023's. The contents table and the totals: 447 promises (201, 205, 41), 17 in `manifest.yaml`, 27 in `providers.yaml`, 24 in `secrets.yaml`, 40 in the interface file, 16 items, 55 and 28 example files, and what `names.yaml` holds. The guarantee for secrets: the run's machine and its gate and where DL13's keys are held join the domain of trust; Q5 stated plainly; X1's exception stated plainly; a credential that lasts as long as its instance; SEC-023's references and PRV-026's reads; SEC-024's limit; the gate's exceptions; PRV-021's stop kept through the provider's retirement; the D5 gap removed | Section 9, row "`spec/README.md`"; the ruling's Q5; X1 as the work order settles it |
| IFC-017 (`then`, observable) | The engine's connection keeps the request and its result confidential and unaltered as far as the side that checks the proof; TLS or the provider's channel carries it, so a value that a secret Resource's server on another machine returns arrives unread | Point 4 of section 3, "The gate, in substance", which no group had written: PRV-025 left the engine's side to IFC-017, which said who proves what but not this. The secrets group's log relied on it for SEC-023's read across machines |
| IFC-022 (observable), DL13 | "Always accepts" and "in every case" became "from every consumer on another machine", as IFC-042's `when` and its observable have it | The three said different things of the same rule; IFC-042 is the one the coordinator assigned |
| PRV-025 (`then`) | Root apart, the excepted user or group and the contract's statement are "for a run" | A server reached by `remote` is its own gate and has no runtime provider; the sentence read as though it had one |
| PRV-022 (observable); the comment on the names in `spec/names.yaml` | Kubernetes' own token and root certificate, and the keys of a TLS Secret, are cited without writing their names in backticks | `tools/set_names.py --identity` rewrites a file name in backticks, so a change of the names would have made the citation false (the core group's open item 3), as MAN-018 already avoids |
| DL16 | "The servers of providers" became "the servers of the runtime, machine and package-source providers" | PRV-026 covers those three alone (its observable says a secret Resource is not among them); DL16 read as covering a secret Resource (the providers group's open item 1) |
| DL17 | "No document changes when the engine is made able to move" now follows SEC-024's rejection, as "no other document needs to change" | The two sentences contradicted each other: SEC-024, under the ruling's option b, rejects on such an engine what it accepts on another |
| `spec/core/README.md` | The three kinds of provider that PRV-026 runs again; the variables of `identity` win over the runner's list beside what the item binds (PRV-023); DL13 called the engine's mechanism of proof; a line on this pass | As DL16; PRV-023 as the providers group wrote it |
| Head comments | A sentence for this pass in `providers.yaml`, `interface/implementation.yaml`, `implementation-defined.yaml` and `names.yaml` | The rule for every changed file |

## Where each row of section 9 landed

| Row of [section 9, "In v0"](../multi-machine/README.md#in-v0) | Where |
| --- | --- |
| New, in `providers.yaml`: the gate | PRV-025; the engine's side in IFC-017 and IFC-041; `run.v1`, `runtime-provider.v1`, `secret-provider.v1`; the row gate |
| IFC-017 | IFC-017, with point 4 of section 3 added here |
| IFC-041, SEC-013 | IFC-041, SEC-013, and SEC-024 as the ruling revised it to option b; IFC-043 for the collision with `Authorization` |
| `inspect-run.v1`, PRV-009 | The optional output, named `url` as `run.v1`'s is since N5 of ruling 42 (the groups' choice); PRV-009, IFC-017 |
| PRV-011, PRV-022 | PRV-011, PRV-022, PRV-007; `run.v1`; the rows `identity` and trust bundle |
| IFC-022, PRV-018 | IFC-022, PRV-018, IFC-032 |
| DL13 | DL13 |
| PRV-012, MAN-010 | PRV-012, MAN-010, `run.v1` |
| PRV-014 | PRV-014; the row machine |
| PRV-021, PRV-020 | PRV-021, PRV-020, LIF-036, DL17, `stop.v1` |
| OPS-027 | OPS-027 |
| ACC-027, definitions | ACC-027; the rows decision point, enforcement point, mediated, enforced and engine (this pass) |
| New item of the implementation-defined list | DL17 |
| New, in `providers.yaml`; SEC-011 | PRV-026; SEC-011 and the promises that the secrets group names beside it; `get-secret.v1` |
| `spec/README.md` | This pass |
| A64, A65 in the audit | Not applied: the tracker's |

Every recommendation of the ruling's list landed: D5, A (PRV-025, IFC-017, IFC-041, PKG-020);
D5's second part, b (SEC-024, IFC-041, DL17); D20, A (MAN-010, PRV-007's guard, the example
project); Q1 as tightened (DL13, PRV-022, IFC-022, IFC-042, `spec/names.yaml`); Q2 (DL17, DL5,
PLC-026); Q3 (PRV-026, SEC-011); Q4 is the tracker's (A65); Q5 (IFC-022, PRV-003, DL16,
`spec/README.md`); and the name `identity`, kept (`spec/definitions.md`).

## The items that waited on this ruling

From the [log of ruling 42's integration](../ruling-42/log-integration.md):

| Item | Settled as | Where |
| --- | --- | --- |
| X1, item 2 of [Still needs a decision](../ruling-42/log-integration.md#still-needs-a-decision): runs under one operating-system user can read each other's credential, against IFC-022's must | Under Q5, the exception is stated plainly: no consumer can present as another, except a program that can read another run's credential, as root and every program of that run's user can | IFC-022 (`then`, observable) and DL16, the interface group's; `spec/README.md`, two points of the guarantee for secrets, this pass |
| X3, and item 2 of [Needs a decision](../ruling-42/log-integration.md#needs-a-decision): a variable named in `identity` and bound by the item | The names are fixed under Q1, so a template whose item binds one is rejected | MAN-018, the secrets group's; PRV-023 for the list set on the engine's own runner, the providers group's; `run.v1`'s `identity`, closed, and the rule `identity-name`, the core group's |
| X7, item 4 of Still needs a decision: the engine restarting on another machine | The engine keeps its hostname across a move (Q2) | DL5 and DL17, the interface group's; PLC-026, the providers group's |
| X9: no call brings a refreshed credential | In this version a credential lasts as long as its instance, and renewal is v1's (proposal, 1.4) | DL13; PRV-022's observable; `run.v1` |
| X10, item 5 of Still needs a decision: `stop.v1` and `inspect-run.v1` give no machine | They take the same `machine` input as `run.v1`, so that a provider that keeps no state can act | PRV-027 (the providers group's own ID), PRV-008, PRV-009, LIF-036; both schemas |
| X11, item 6 of Still needs a decision: which references SEC-023 reads | Those the setup read, kept with the connection without values; the other headers keep the values the setup took | SEC-023, the secrets group's; IFC-041, the interface group's; `get-secret.v1` |
| Item 5 of Needs a decision: a stop that a provider holds when it is retired | Kept where neither a restart of the engine, on its machine or another, nor the provider's retirement loses it, as the proposal's row "PRV-021, PRV-020" has it | PRV-021, PRV-020, DL17, `stop.v1`; `spec/README.md`, this pass |
| Item 5, the rest | Whether SEC-023 stops trying a read that keeps failing: unchanged, SEC-012 reads again before each request. MAN-017's check of a place whose schema states none of `type`, `enum` and `const`: not this ruling's, and still open | SEC-012 |

The questions [for the research on work across machines](../ruling-42/log-integration.md#for-the-research-on-work-across-machines):

| Question | Answer in the text |
| --- | --- |
| D5: who may reach a run's `url`, a store's among them; a secret header to `http` | PRV-025, IFC-017, IFC-041, SEC-006; SEC-024 |
| D20: a packaged program under another runtime provider; the example's Pool template `workspace` | Row 17's guard stays (PRV-007, MAN-010); a server for another machine is a pinned `package` item; the core group took the `runtime` input out of `workspace` |
| How a refreshed credential reaches a run | It does not in this version: a credential lasts as long as its instance (DL13, PRV-022) |
| Which engine address works from another machine; a change of the run's network during a run | The first: IFC-042, and DL17, under which the address survives a move. The second is not answered: PRV-022 still lets a provider give the address as the run sees it, and nothing changes it during a run |
| What stops a run left on a machine that comes back | PRV-021 and PRV-020; and the run reaches nothing through the engine once its instance is removed (IFC-032), and its gate admits only an enforcement point (PRV-025) |
| An onward request of an earlier hook attempt on a machine cut off | Stays a limit of this version (OPS-028); local enforcement is A65's |
| SEC-023's read after a restart on another machine | SEC-024 keeps a secret header off `http` on an engine that can move, and IFC-017 keeps a read from a store's server confidential and unaltered |

The [verification](#verification) refined three of these: X3, whose names MAN-018 now compares
without regard to case, with each variable of `identity` given in place of a Package's default
(C9); the stop that a provider holds when it is retired, which PRV-021's observable now says
only something that runs without the provider's server can carry out (C7, MX-07); and SEC-023's
read after a restart on another machine, which SEC-013's guard keeps off `http` whenever the
template was registered (C1, MX-05). The change of the run's network during a run stays open
as [N4](#still-needs-a-decision).

## Coined names

The names of what `identity` carries agree in every file: `spec/names.yaml` holds them,
`tools/set_names.py --check` counts each and refuses any other `CONDUIT_*` variable under
`spec/`, and `tools/check_schemas.py` checks that `run.v1` agrees. Three groups chose them
independently and the fourth adopted them; the precedents are in the
[providers group's log](log-providers.md#coined-names) and the
[interface group's log](log-interface.md#coined-names). For the owner to strike; one run of
`python3 tools/set_names.py --identity <key>=<value> --write` changes any of them everywhere.

| Name | What it is | Precedent |
| --- | --- | --- |
| `CONDUIT_URL` | The variable that holds the engine's address | Kubernetes gives a Pod the API server's address in a variable; `url` is the name in force for a Streamable HTTP URL |
| `CONDUIT_IDENTITY_DIR` | The variable that names the directory of the files | systemd's `CREDENTIALS_DIRECTORY`, Nomad's `NOMAD_SECRETS_DIR`: runs that share a machine cannot share one fixed path |
| `ca.crt` | The trust bundle | The root certificate that Kubernetes mounts in a Pod |
| `token` | A token in the form of a JWT-SVID | The token that Kubernetes mounts in a Pod |
| `tls.crt`, `tls.key` | A certificate in the form of an X509-SVID, and its key | The keys of a Kubernetes TLS Secret |

The terms, also coined, are defined in `spec/definitions.md` by these precedents:

| Term | Precedent |
| --- | --- |
| decision point | NIST's policy decision point, as section 1.2 of the proposal splits it. No promise uses it; the row names the role that an enforcement point relies on |
| enforcement point | NIST's policy enforcement point |
| gate | The proposal's own word (1.3, 3) for what the kubelet's `--client-ca-file` does for its own server, moved before every server that the engine does not run itself |
| trust bundle | SPIFFE's trust bundle, the set of roots against which a credential is checked. This pass left it the file `ca.crt` alone, X.509 roots; the [verification](#coined-names-1) added `jwks.json`, the keys that check a token, as a SPIFFE bundle carries both |

"Fail over" stays among the words the promises do not use: the row now says what the ruling's
"an engine that can fail over" is in the promises, an engine that can restart on another
machine or run as several parts (DL17), as the groups wrote it.

## Choices of this pass

The [verification](#verification) reversed choice 3 (IFC-042 now names the engine's address at
an `https` URL, whichever machine a request comes from), choice 4 (PRV-026 now covers a secret
Resource) and the second half of choice 5 (DL17 now says what becomes of a template registered
before the engine was configured to move).

1. **Decision point is defined although no promise uses it**, since section 9 asks for both
   terms and the row of enforcement point needs it.
2. **IFC-017 carries point 4 of the gate in substance.** No group had written "between the
   engine and the side that checks the proof, the request and its result stay confidential and
   unaltered", and the secrets group's log relied on it.
3. **IFC-042's scope governs.** IFC-022 and DL13 said that the engine's address accepts the
   token form always; IFC-042, as assigned, says from a consumer on another machine, and its
   observable leaves the engine's own machine to DL13. This pass made the three agree with
   IFC-042.
4. **PRV-026 is left as written** (the three providers that provider calls reach), and DL16 and
   the core README were narrowed to match it, pending [item 1](#needs-a-decision).
5. **DL17 keeps SEC-024's rejection first**: "no other document needs to change". What becomes
   of a template registered before the engine is made able to move is [item 2](#needs-a-decision).
6. **`spec/README.md` states what ruling 43 leaves outside the guarantee for secrets**: a secret
   header over `http` on loopback, where an engine cannot move, reaches whatever listens there,
   a process of another user among them; and a gate admits root and the provider's own user or
   group on the run's machine.
7. **The citation of Kubernetes in PRV-022 and in `spec/names.yaml`** no longer writes the file
   names, which settles the core group's open item 3.

The groups' own choices beyond the record were checked against each other and against the
ruling, and none contradicts another: the providers group's choices 1 to 9, the interface
group's 1 to 9, the secrets group's 1 to 10 and the core group's 1 to 10, each in its log. Each
still stands unless the coordinator or the owner strikes it; the ones that change behavior are
in the next section.

## Needs a decision

Gathered from the four logs and this pass, for the coordinator or the owner. The
[verification](#verification) below settled items 1, 2, 3 and 5 as the ruling's words and the
records it accepts decide them, and stated item 4; what remains open is under
[Still needs a decision](#still-needs-a-decision).

1. **A secret Resource on the engine's own runner after the engine restarts** (the providers
   group's [item 1](log-providers.md#needs-a-decision), the secrets group's
   [item 1](log-secrets.md#needs-a-decision)). Q3 speaks of provider calls, which reach the
   runtime, machine and package-source providers, so PRV-026 leaves out a secret Resource,
   which `spec/definitions.md` counts among the providers. Where its process ends with the
   engine, every read through it, SEC-023's again and PRV-026's among them, fails until an
   actor sets it up again. Options: (a) as written; (b) PRV-026 also runs again, before a read
   through it, a secret Resource whose run on the engine's own runner ended with the engine.
   Both groups recommend (b). Note: an Agent on that runner also ends with the engine and is
   run again only by a further setup (PRV-007), so (b) makes a secret Resource the one
   instance the engine runs again of its own accord.
2. **A template registered before the engine is made able to move** (the secrets group's
   [item 2](log-secrets.md#needs-a-decision)). SEC-024 rejects it on such an engine, and DL17
   now says only that no other document needs to change. Options: (a) DL17 lets the engine be
   made able to move only once no template that exists is one SEC-024 would then reject;
   (b) such a template's connection reports unavailable on that engine; (c) each engine's.
   The secrets group recommends (a).
3. **How a gate checks the engine's token** (the core group's
   [item 1](log-core.md#open-for-the-coordinator-or-the-owner)). Under the token form toward a
   run, the gate checks a JWT-SVID with what `identity` holds, and `ca.crt` holds X.509 roots
   alone. DL13 says where the keys that check a credential are published, but a gate written
   once does not know that place. Options: (a) a fifth file of `identity`, the keys that sign
   tokens as a JWK Set, as a SPIFFE bundle carries its JWT authorities and spiffe-helper writes
   them, its name fixed with the others; (b) the engine signs tokens with a key whose
   certificate chains to `ca.crt` and the token carries it in the JWS header `x5c`, whereas a
   JWT-SVID is checked against the signing keys of its trust domain's bundle; (c) as now. This
   pass recommends (a), SPIFFE's own form.
4. **A local server that takes Conduit's keys at `http://localhost`** (the interface group's
   [item 1](log-interface.md#needs-a-decision)). Under D5, A, it is never connected to;
   option b of the ruling helps only a `plain` server with a secret header. The interface group
   recommends keeping it so: such a server runs over stdio on the engine's own runner or is
   used as `plain`.
5. **A `remote` item that is not `plain`, at an `http` URL** (the interface group's choice 2,
   the core group's [item 2](log-core.md#open-for-the-coordinator-or-the-owner)). It is never
   connected to, and its part reports unavailable at each setup; nothing rejects it when the
   manifest is read. Options: (a) as written; (b) `implementation-item.v1` and a promise reject
   it, as IFC-043's item is rejected. `spec/README.md` has an engine reject a document it cannot
   honor, which leans to (b).
6. **IFC-043, or mutual TLS** (the interface group's [item 2](log-interface.md#needs-a-decision),
   the core group's choice 4): an item that is not `plain` may not name `Authorization`. If it
   is struck, so are the schema's rule and its example.
7. **"Not from the run it came from" on one machine** (the interface group's
   [item 3](log-interface.md#needs-a-decision)): read strictly, it rules out a check of a local
   peer's operating-system credentials. Applied as worded.
8. **Choices that change what is rejected**: SEC-024 also rejects `http` to any host that is
   not a loopback address, on every engine, and is of the layer `language` although it reads
   DL17 (the secrets group's choices 1 and 4); MAN-018 rejects the template, not the item (its
   choice 5); PRV-027 and IFC-043, IDs added by the groups in their own files.
9. **The coined names and terms** above, for the owner to strike.

## For other jobs

- `AGENTS.md`, "Checks": `spec/names.yaml` now also holds the names of what `identity` carries,
  which `python3 tools/set_names.py --identity` changes (the core group's open item 3).
- `docs/AUDIT.md`, the tracker: A64 and A65 as section 9's last row and the section
  [In v1, for A65](../multi-machine/README.md#in-v1-for-a65) have them; and the items above.
- The fifth round of readers that the ruling's "prepare for the codex audit" orders.

## Verification

Date: 2026-11-07. Three readers read the result of this pass, and found 43 things: C1 to C11,
F1 to F11, and MX-01 to MX-21. Each was checked against the text. Where it held and the fix
follows from ruling 43, from the records it accepts (the
[proposal](../multi-machine/README.md) and its research) or from wording, it was fixed;
otherwise it is listed under [Still needs a decision](#still-needs-a-decision), with options
and a recommendation. Several readers found the same thing, and the rows say so. Every choice
below is this verification's, and the coordinator or the owner can strike it. No record was
edited but this log, and nothing was committed.

**40 fixed, 3 open in part** (F11, MX-10 and MX-15), and one part of C1 left for the owner to
add or not. Two IDs were added, each logged here: PRV-028 in `providers.yaml` and IFC-044 in
`interface/implementation.yaml`, the next free IDs of their prefixes; none was reused.

| Finding | Holds | What was done |
| --- | --- | --- |
| C1 (unsafe): a template registered before the engine can move sends a secret over `http` after a failover | Yes | Settled by the ruling's own sentence, a header that carries a secret "is sent only to" `https` or to loopback on an engine that cannot fail over, which binds the send and not only the registration. SEC-013's `then` sends the value only where SEC-024 lets it go on the engine as it is configured then, and otherwise reads nothing, makes no connection and has the part report unavailable. SEC-013's and SEC-023's observables no longer claim what only registration gave; DL17 says such a template sends the header nowhere until it is replaced. This is option (b) of [item 2](#needs-a-decision); adding option (a) on top is [N2](#still-needs-a-decision) |
| C2: PRV-025 states no duty for the server's side to prove who it is | Yes | New PRV-028 (`convention`, `must`), point 4 of [the gate in substance](../multi-machine/README.md#the-gate-in-substance): what answers at a run's `url` proves by TLS, before it is sent anything, with the certificate that `identity` holds in `tls.crt`, which names the instance's ID, whether it is the run's gate or the provider's own end. IFC-017's `then` cites PRV-028 in place of PRV-025, and PRV-025's observable cites it. The proposal already has the gate serve that certificate (1.4, section 6 on D17, the row "PRV-011, PRV-022") |
| C3: PRV-026 leaves out a secret Resource on the engine's own runner | Yes | The ruling's Q3 says "a provider", and `spec/definitions.md` counts a secret Resource among the providers. PRV-026's `when` and `then` also cover a read through a secret Resource, not retired, whose run on the engine's own runner ended with the engine: it is run again before the read, its own secrets read as for any provider. The observable says that an Agent's run is not run again so. DL16, PRV-002's observable, the rows provider and secret Resource, `spec/README.md` and `spec/core/README.md` follow. This is option (b) of [item 1](#needs-a-decision), which both groups recommended |
| C4: a gate cannot check the engine's token with what `identity` holds | Yes | Q1 fixes "the trust bundle file" so that a gate written once works under any engine, and a SPIFFE trust bundle carries its JWT authorities beside its X.509 ones. `identity` gains `jwks.json`, the JWK Set of the keys that check a token that the mechanism signs; with `ca.crt` it is the trust bundle. `spec/names.yaml` (`jwtBundleFile`), `tools/set_names.py`, `tools/check_schemas.py`, `run.v1`, PRV-022, DL13 and the rows `identity` and trust bundle follow. This is option (a) of [item 3](#needs-a-decision) |
| C5: DL13 lets an engine offer mutual TLS alone | Yes | DL13 now gives the token form, which the engine's address at an `https` URL always accepts (IFC-042), and mutual TLS, which it may accept beside it. "One or both" stays only for what the engine presents to a gate and to a `remote` item, and what a run presents to it, as Q1 has it |
| C6: a `remote` item that is not plain at an `http` URL is never connected to, yet not rejected | Yes | `spec/README.md` has an engine reject a document it cannot honor, and no engine can connect to such an item: IFC-041 connects to it only at `https`, and the engine checks who answers before it sends, which plain `http` cannot prove. New IFC-044 (`language`, `must`) rejects it wherever it is read; `implementation-item.v1` states it (`remote.url` begins with `https://` in the branch for an item that is not plain), with the example `invalid/IFC-044-manifest-remote-item-not-plain-at-http.yaml`; IFC-041 cites it. [Item 4](#needs-a-decision), that only a plain local server benefits from the ruling's revision b, follows from D5, A, as above, and is stated in IFC-044, SEC-024, `spec/README.md` and `docs/OVERVIEW.md`. This is option (b) of [item 5](#needs-a-decision) |
| C7: a stop held by a provider that no longer runs is never carried out | Yes | PRV-021's observable says plainly that the provider's own server runs only while the engine runs it, may not be run again after a restart where no call needs it (PRV-026), and never after its retirement, so a stop that only that server carries out waits or is never carried out; a contract that keeps its stops through both, as PRV-021's `then` already requires, has something that runs without that server carry them out, such as the provider's own part on the run's machine, as the proposal's [D11](../multi-machine/README.md#d11-a-run-left-on-a-machine-that-comes-back) has it. PRV-020's observable and the last point of the guarantee in `spec/README.md` follow |
| C8: IFC-042's `when`, a consumer on another machine, is not something the engine can observe | Yes | IFC-042's `when` is now "a consumer sends a request to the engine's address at an `https` URL". Its observable says the engine establishes nothing from the machine a request came from (PRV-018), so it accepts the token whichever machine the request comes from, and that on its own machine DL13 may also give a run an address that is not `https` and other means. IFC-022, DL13, `spec/README.md` and `spec/core/README.md` follow. This reverses choice 3 above |
| C9: on Windows an item binding `conduit_url` collides with `CONDUIT_URL`; a Package's default is not covered | Yes | MAN-018 compares names without regard to case, and so does the rule `identity-name` of `tools/check_schemas.py`. PRV-022's `then` has the provider deliver each variable of `identity` in place of any value that the variable would take from elsewhere, such as a default that the item's Package declares; MAN-018's observable says so |
| C10: `run.v1` leaves `identity.files` open | Yes | `identity.files` is closed to the six names, every one required but `tls.crt` and `tls.key`, and `tools/check_schemas.py` checks the names, the required list and the closure against `spec/names.yaml`. The reason in the core group's [choice 1](log-core.md#choices-beyond-the-record), that the keys are paths, did not hold: PRV-022 keys the files by their paths in the directory, which are their names |
| C11: `spec/core/README.md` still calls `identity` a working name | Yes | Its ruling 43 paragraph now says that the ruling kept `identity`, and that `serving` and `unreconciled` are still working names |
| F1: under DL13 a run on another machine may get no `token` | Yes | As C5. PRV-007's `then` has the engine give every run every file of PRV-022 but `tls.crt` and `tls.key`, `token` among them, so a harness on another machine can send it as a configured `Authorization` header; PRV-022, `run.v1`, the row `identity` and `spec/core/README.md` follow |
| F2: as C3 | Yes | As C3 |
| F3: nothing backs "any client that supports the protocol's authorization can connect" | Yes | DL13 now states which grants of the protocol's authorization its authorization server supports for a consumer and how it decides which instance a token names, and that it issues a token whose audience is anything but the engine's address to the engine alone. IFC-042's observable says a consumer that follows the protocol's authorization gets its token so, and a run finds its token in `identity` |
| F4: being the provider's own channel cannot be verified at a URL | Yes | As C2: whatever answers at the `url`, the provider's own end included, presents the run's certificate (PRV-028), and a plain `http` listener on loopback is said to prove nothing. IFC-017's observable and PRV-025's observable on `docker run -i` say so |
| F5: as C4 | Yes | As C4 |
| F6: the path through an authorization server that the user runs is not DL13's proof, and its client credentials have no home | Yes | The proposal names that path, so it stays. IFC-041's `then`: the token is either a JWT-SVID that DL13's mechanism signs, or one that the authorization server named in the item's metadata issues to the engine under the extension for client credentials, the engine authenticating with its own credential of DL13; the observable names RFC 7523's JWT assertion and RFC 8705's mutual TLS, so the engine holds no client secret outside every Conduit. PRV-025's `then` says "by the mechanism of DL13 or, for a server reached by `remote`, a token obtained with it under IFC-041", and DL13 states how the engine authenticates. Same as MX-13 |
| F7: `secret-provider.v1` says a plain store reached by `remote` is sent no locator | Yes | Its description now says that such a store is reached as before, takes none of Conduit's keys, and learns its locator only through a header that its item binds (SEC-006, PKG-020). The conclusion of the core group's [choice 10](log-core.md#choices-beyond-the-record), that such a store serves one fixed secret at most, is struck |
| F8: the engine's address must present a certificate from DL13's root | Yes | The proposal (1.4) treats the engine's address as a receiver at a stable `https` URL, as a `remote` server is, and Q1 as tightened wants an off-the-shelf client to connect. DL13 now states the roots from which the certificate at the engine's address is issued, the mechanism's own or others such as a public certification authority's, and `ca.crt` holds them beside DL13's; PRV-022 and the row trust bundle follow |
| F9: as C6; the SEC-024 example shows an item that is not plain | Yes | As C6. The example's item `status-board` is now plain, with `tools`, and its comment says that only a plain item stands at `http`. Without that, IFC-044's schema rule would have rejected its manifest |
| F10: "can fail over" read as what the product can do | Yes | The proposal speaks of turning failover on. DL17 reads "whether the engine, as configured, can …, and how it is so configured"; SEC-024's `when`, IFC-041, IFC-044, `register.v1` and the row "fail over, failover" say "as configured" |
| F11: `ca.crt` is given unconditionally, which contradicts DL13 and adds friction on one machine | In part | The contradiction is gone: DL13 now states only where it gives `tls.crt` and `tls.key`, and every other file is given to every run (PRV-007). The friction is stated plainly: on the engine's own machine `CONDUIT_URL` may be an `http` URL on loopback, where `ca.crt` checks nothing, so a run there needs no certificate configured (PRV-022, DL13, IFC-042). Giving `ca.crt` only where it checks something conflicts with C10 and MX-21 and with the proposal's "`identity` also carries the trust bundle": [N1](#still-needs-a-decision) |
| MX-01 (unsafe): a gate cannot tell the engine's credential from an instance's | Yes | The ruling has a gate admit "only proof that an enforcement point sent it", and Q1 a gate written once. `identity` gains `engine-id`, the engine's ID in plain text. PRV-025's `then`: the proof is a credential that names an enforcement point; its observable: a run's gate finds that ID in `engine-id`, and an instance's credential names the instance and does not pass. IFC-041: the operator of a `remote` server configures it to admit only a credential that names the engine's ID, which the conformance document states. DL13: a credential that names the engine's ID, a certificate that names the host of the engine's address, and a token whose audience is anything but the engine's address are issued to the engine alone. PRV-022: a run checks who answers at `CONDUIT_URL` by TLS against `ca.crt` and the host of that URL. The row ID defines the engine's ID |
| MX-02 (unsafe): runs may run as the user that PRV-025 excepts | Yes | The ruling says "nothing else on the server's machine reaches the server"; the exception is for the provider's means. PRV-025's `then`: the excepted user or group is the one the provider uses for its own means, which none of its runs runs as or in; the observable says why; `spec/README.md` and the row gate follow. This is the finding's first option |
| MX-03 (unsafe): a program that takes the provider's port on loopback receives the engine's proof | Yes | As C2 and F4: the engine sends nothing until the run's certificate is presented at the `url` (PRV-028), which a program that takes the port cannot present unless it can read the run's key, which only a program inside the domain of trust can |
| MX-04 (unsafe): after a move, the processes on the machine the engine left go on, and a second run starts | Yes | One run serves one instance (TPL-019), and every choice under the implementation-defined list keeps every promise. DL17 now says so: before the runner runs anything on another machine, every process it started on the machine the engine left has ended, as Kubernetes has a node that stopped answering shut down before it is marked out of service and its Pods are made elsewhere. That covers the servers of providers too |
| MX-05 (unsafe): as C1 | Yes | As C1 |
| MX-06 (unsafe): an actor may reconcile as failed an invocation whose work still runs on a machine cut off | Yes | The proposal's S5 states the inference and the guidance. OPS-014's observable: what an actor checks of an invocation dispatched to an instance on a machine cut off may be out of date until the machine is back and the run is inspected, and the effect may land after the actor recorded that it did not occur. PRV-025's observable: the gate's bound counts only where the server answers with a stream. OPS-027's wording of the bound follows MX-18 |
| MX-07: as C7 | Yes | As C7. Of its options: (c), stated plainly, with the contract's duty that PRV-021 already sets; (a) and (b), which would add an operation or have the engine run a provider it does not need, were not taken |
| MX-08: as C8 | Yes | As C8. The `when` names the address at an `https` URL rather than every address: the token form is the protocol's authorization, whose resource RFC 9728 makes an `https` URL, and on its own machine DL13 may give a run an `http` address |
| MX-09: several parts are not reconciled with the engine's own runner, the providers' servers and connections | Yes | DL17: what the promises say of the engine's own runner, of the servers of providers and of connections holds of the whole: the runner's processes run on one machine at a time, and each server of a provider and each connection to a `remote` item is held by one part at a time. The mechanism stays the engine's, as Q2 has it |
| MX-10: a part that stops leaves its invocations in progress; a paused part lands effects late | In part | DL17: an invocation in progress on a part that stops ends unknown, as under IMP-007, although the engine as a whole goes on, since IMP-007's reason holds of the part; and a paused part may land an effect late, which only a short-lived credential of the engine bounds. Whether that credential must be short-lived on such an engine changes DL13's "may": [N3](#still-needs-a-decision) |
| MX-11: a provider whose secret Resource is on a cut-off node is never run again | Yes | PRV-026's observable states the trap and the guidance, as the proposal's S2 gives guidance for a store: the secret Resources that a provider reads belong on `local`, since where one holds the machine provider's secret, every machine but `local`'s stays omitted. Rejecting such a Conduit was not taken: whether a secret Resource is on `local` may be known only once the machine provider places it |
| MX-12: as C3 | Yes | As C3 |
| MX-13: as F6 | Yes | As F6, in the form this finding gives: the engine authenticates with its own credential of DL13 |
| MX-14: `exp` and `notAfter` are unstated | Yes | DL13: the time at which a run's credential lapses is set so far ahead that it does not lapse while the instance exists, and the engine's check on every request is the only revocation. `spec/README.md` states the limit, and the row credential follows |
| MX-15: an address given as the run sees it breaks TLS, the token's audience and RFC 9728 | Open | Ruling 42 accepted, for D17, that the provider may give the address as the run sees it, and narrowing it changes behavior: [N4](#still-needs-a-decision) |
| MX-16: a gate for several runs can let one server replay the engine's token to another | Yes | PRV-025's `then` already forbids a form that a receiver can present to another. Its observable says that a token-checking proxy meets it only where it admits a token on the route to the server of the instance that the audience names, and does not pass the token on |
| MX-17: LIF-036 says no secret is read, against PRV-026 | Yes | LIF-036: no run of the instance is asked for and no secret is read for it; after a restart the runtime provider may first be run again, with reads of its own, under PRV-026 |
| MX-18: the gate at the provider's own end ends with the engine; the bound measures the wrong silence | Yes | PRV-025's `then` measures the bound from when the connection that carries requests from the enforcement point to the server goes silent. Its observable says the provider's own end ends with the engine and takes the run's channel with it, so its contract, in stating its gate, states what then becomes of the run. `runtime-provider.v1` and OPS-027 follow |
| MX-19: `localhost`, and URL variables | Yes | `localhost` follows from the ruling's own example, "a local server reached at `http://localhost`", and is the secrets group's [choice 3](log-secrets.md#choices-beyond-the-record). SEC-024 now says the URL is compared as the Remote writes it, so a host written as a variable of the Remote is no loopback host; `tools/check_schemas.py` rejects such an `http` URL, where it skipped it |
| MX-20: the README's point on a store's server overstates a `remote` server | Yes | The point now says that for a server reached by `remote`, its own gate, it holds as far as its operator has configured it to admit only the engine's proof (IFC-041) |
| MX-21: as C10; PRV-022 still lets a file be replaced in place | Yes | As C10. PRV-022's leave to replace a file's content in place moved from its `then` to its observable, as what this version does not do; the row `identity` no longer grants it |

### New IDs

| ID | File | Layer, level | What it says |
| --- | --- | --- | --- |
| PRV-028 | `promises/providers.yaml` | `convention`, `must` | What answers at a run's `url` proves by TLS, with the run's certificate, that it serves the instance before it is sent anything (C2, F4, MX-03) |
| IFC-044 | `interface/implementation.yaml` | `language`, `must` | An item reached by `remote` and not marked `plain` whose Remote's `url` is not an `https` URL is rejected (C6, F9) |

### Coined names

Two more files of `identity`, held in `spec/names.yaml` with the others, which
`tools/set_names.py --check` counts and `tools/check_schemas.py` checks against `run.v1`. For the
owner to strike; `python3 tools/set_names.py --identity <key>=<value> --write` changes either.

| Name | Key in `names.yaml` | What it is | Precedent |
| --- | --- | --- | --- |
| `jwks.json` | `jwtBundleFile` | The JWK Set of the keys that check a token that the mechanism signs; with `ca.crt`, the trust bundle | A JWK Set is RFC 7517's form, in which a SPIFFE bundle carries its JWT authorities, OpenID Connect publishes keys at its `jwks_uri` and the Kubernetes API server serves its service account keys at `/openid/v1/jwks` |
| `engine-id` | `engineIdFile` | The engine's ID, in plain text, which a gate admits | Kubernetes writes a Pod's namespace, in plain text, in the file `namespace` beside its token and root certificate; ghostunnel's `--allow-uri` takes the ID that a gate admits |

The term "the engine's ID", the ID by which a credential of DL13 names the engine, is defined
in the row ID of `spec/definitions.md`; DL13 already named it as the ID of the engine's role.

### Choices of the verification

Each changes or fixes behavior beyond the wording, follows from the ruling or the records as
the rows say, and stands unless the coordinator or the owner strikes it.

1. **The certificate is the one proof of who answers at a run's `url`** (PRV-028). The
   proposal's "or by being the provider's channel" stays as the way the request goes on from the
   provider's end, which presents the certificate.
2. **IFC-044** rejects a `remote` item that is not plain at an `http` URL.
3. **Every file of `identity` but `tls.crt` and `tls.key` goes to every run**, `token` and the
   trust bundle among them, and the schema requires them. See [N1](#still-needs-a-decision).
4. **Two files added to `identity`**, `jwks.json` and `engine-id`, under coined names.
5. **The engine's address may present a certificate from other roots**, such as a public
   certification authority's (F8).
6. **No run runs as the user or group that a gate excepts** (MX-02).
7. **MAN-018 compares names without regard to case** (C9).
8. **The engine authenticates to an authorization server that the user runs with its own
   credential** of DL13, by RFC 7523 or RFC 8705 (F6, MX-13). Changed in round 5 (M66, dated
   2026-11-07): IFC-041 drops RFC 8705, since the protocol's extension for client credentials
   does not allow mutual TLS, so the engine authenticates by a JWT assertion under RFC 7523
   alone, and DL13 states only its key and client identifier.
9. **DL17's consequences**: one machine for the runner at a time, one part for each server of
   a provider and each connection, the machine left shut down before the runner runs elsewhere,
   and the invocations of a part that stops ended unknown (MX-04, MX-09, MX-10).
10. **IFC-042 names the address at an `https` URL** (C8, MX-08). Changed in round 5 (M53,
    dated 2026-11-07): IFC-042 also accepts a run's `token` at the `http` loopback address that
    DL13 gives a run, under the condition that DL13 states (M47).
11. **SEC-013's guard** (C1, MX-05): an engine configured to move sends a secret header to no
    `http` URL, whenever the template was registered.
12. **A provider's own secret Resources belong on `local`**, stated as guidance (MX-11).
13. Added in round 5 (M109), which found it missing here: **PRV-022 no longer gives leave to
    replace a file of `identity` in place** (MX-21). In this version a credential lasts as long
    as its instance (DL13), as the proposal has it, so the leave that ruling 42's D17 gave is
    left, in PRV-022's observable, for renewal on the run's machine in v1 (A65).

### Still needs a decision

Added in round 5 (M104): N1 to N4, and items 6 to 8 of Needs a decision, were settled by the
coordinator in the record's [After the ruling](README.md#after-the-ruling), and item 9, the
coined names, is listed there for the owner to strike.

Open, for the coordinator or the owner. Items 6 to 9 of [Needs a decision](#needs-a-decision)
above also still stand.

1. **N1. Which runs get the trust bundle and `engine-id`** (F11). Options: (a) every run, as now,
   which C10 and MX-21 asked for and which the proposal's "`identity` also carries the trust
   bundle" says; (b) only where `CONDUIT_URL` is an `https` URL or the run's server stands
   behind a gate, so that an engine with only its own runner and an `http` loopback address
   keeps no X.509 root. Recommendation: (a). A harness or a gate written once relies on finding
   them; the cost is a root that the engine makes once, which no user configures.
2. **N2. A template registered before the engine is configured to move** (C1, MX-05, the
   secrets group's [item 2](log-secrets.md#needs-a-decision)). The guard of SEC-013 now keeps
   the secret off `http` on such an engine. Options: (a) DL17 also has the engine refuse to be
   configured so while a template exists that SEC-024 would then reject; (b) the guard alone,
   as now. Recommendation: (b). Either keeps the secret off another machine's loopback; (b)
   needs no check of every template at a change of configuration, and the record of each part
   says why it reported unavailable.
3. **N3. The engine's own credential on an engine of several parts** (MX-10). Options: (a) it
   is short-lived, with its lifetime in the conformance document, which bounds how late a part
   that paused can land an effect; (b) "may be short-lived", as now. Recommendation: (a), which
   the proposal says costs nothing (1.4), and which Q2's "no gate is asked to check a term"
   leaves as the only bound.
4. **N4. The engine's address as the run sees it** (MX-15). Options: (a) an address that a
   provider gives as the run sees it keeps the host of the engine's address, by a host entry or
   a tunnel that keeps the server name and `Host`, as `docker run --add-host` can; (b) the
   certificate at that address names its host, and only a configured `Authorization` header is
   promised there, since a client that follows the protocol's authorization checks the
   resource against the URL it used; (c) as now. Recommendation: (a), which keeps TLS, the
   token's audience and RFC 9728 as they are.

### Counts

449 promises, two more than after this pass (PRV-028, IFC-044): 202 of the layer `language`,
206 `convention` and 41 `interface`; by level, 401 `must`, 44 `must not`, 3 `may` and 1
`should`. None was removed or moved between files, and no ID was reused. The
implementation-defined list holds 16 items. `promises/providers.yaml` holds 28 promises and
`interface/implementation.yaml` 41; `examples/invalid/` holds 56 files and
`examples/invalid-by-rule/` 28.

The verification changed 18 promises, each keeping its ID, and three items: PRV-002, PRV-007,
PRV-020, PRV-021, PRV-022, PRV-025 and PRV-026; IFC-017, IFC-022, IFC-041 and IFC-042; SEC-013,
SEC-023 and SEC-024; MAN-018; LIF-036; OPS-014 and OPS-027; and DL13, DL16 and DL17. It also
changed `spec/names.yaml`, `spec/definitions.md`, `spec/README.md`, `spec/core/README.md`,
`spec/core/operations.yaml` (`run.v1`, `register.v1`), `spec/core/capabilities.yaml`
(`runtime-provider.v1`, `secret-provider.v1`), `spec/schemas/manifest.yaml`
(`implementation-item.v1`), `spec/schemas/README.md`, the example for SEC-024, the new example
for IFC-044, `tools/set_names.py`, `tools/check_schemas.py` and `docs/OVERVIEW.md`. Every
changed file under `spec/` has a line for it in its head comment, or, for a Markdown file, a
sentence in its paragraph on ruling 43.

### Checks

Run from the repository root at the end of the verification.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) | `449 promises ok` |
| A YAML line over 96 characters under `spec/` | None |
| `python3 tools/check_docs.py` | `Errors: 0`; 159 Markdown files and 2247 local links |
| `python3 tools/set_names.py --check` | `Errors: 0`; `CONDUIT_URL` x14, `CONDUIT_IDENTITY_DIR` x10, `` `ca.crt` `` x11, `` `jwks.json` `` x12, `` `engine-id` `` x12, `` `token` `` x9, `` `tls.crt` `` x14, `` `tls.key` `` x12 |
| `python3 tools/check_schemas.py` | `Errors: 0`; 56 invalid documents each rejected, IFC-044's among them; 24 rules from 23 promises, 28 sets under `examples/invalid-by-rule/` each rejected by its rule; `run.v1`'s `identity` checked against `spec/names.yaml`, its files closed |
| `git diff --check` | Clean |
