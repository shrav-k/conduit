# Ruling 42: log of the integration

Date: 2026-11-06. This is the account of the integrator's pass after five editors applied
[ruling 42](README.md) to disjoint files: the [machines group](log-machines.md), the
[secrets group](log-secrets.md), the [conventions group](log-conventions.md), the
[language group](log-language.md) and the [core group](log-core.md). The rulings are the
owner's and the decisions the coordinator's. The edits below are the integrator's, and each
stands unless the coordinator or the owner strikes it.

What this pass did: it read every promise that ruling 42 changed or added (95 of them) against
each other, against the [open questions after the rewrite](../ruling-41/open-decisions.md) and
against the ruling's record; checked that every promise ID and every item of the
implementation-defined list cited anywhere under `spec/` and in `tools/check_schemas.py` exists
and says what the citing sentence relies on; checked the seven IDs assigned in advance; and
checked that every row of the table "Settled by the coordinator" and every answer of the owner
landed somewhere. No group had `spec/definitions.md` or `spec/README.md`, and the rows of the
table that name them had not landed, so this pass applied them.

Not applied here, as in every group: the names N1 to N20 and the editorial fixes E1 to E23 of
the [vocabulary audit](../vocabulary-audit/README.md), which a later stage applies, so the text
keeps the names in force today (location, operation identity, origin, `secret-reveal.v1`,
`reveal-secret.v1`); and D5 and D20, which the owner did not rule. For D5 the gap stays stated in
`spec/README.md`, and for D20 row 17's guard stays in PRV-007, MAN-010 and `run.v1`. Nothing was
committed.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `439 promises ok` |
| Every text field folded with `>-`; a YAML line over 96 characters under `spec/` | Yes; none |
| A promise ID or a DL item cited under `spec/` or in `tools/check_schemas.py` that does not exist | None outside head comments. Head comments name removed IDs, each as removed, and DL14 as not reused |
| The seven IDs assigned in advance (OPS-034, LIF-036, SEC-022, SEC-023, PRV-021, PRV-022, PRV-023) | Each exists once, in the file it was assigned to, and every sentence that cites it agrees with it after the fixes below |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_schemas.py` | `Errors: 0`; 22 rules from 21 promises, 26 sets under `examples/invalid-by-rule/` each rejected by its rule |
| `git diff --check` | Clean |

## Counts

439 promises, 7 more than before ruling 42 (OPS-034, LIF-036, SEC-022, SEC-023, PRV-021,
PRV-022, PRV-023): 198 of the layer `language`, 201 `convention` and 40 `interface`; by level,
392 `must`, 43 `must not`, 3 `may` and 1 `should`. POOL-016 moved from `language` to
`convention` (row 14). None was removed or moved between files. The implementation-defined list
holds 15 items, with DL16 new. This pass added no ID.

## What this pass fixed

| # | File and place | What was wrong | Fix |
| --- | --- | --- | --- |
| 1 | PRV-001 and IFC-037, observables | Both said that no plain implementation implements an operation of "a provider capability". PRV-001 counts four provider capabilities, `secret-reveal.v1` among them, while PKG-020, SEC-006 and the checker's rule `plain-provider` allow a plain store | Both now name `runtime-provider.v1`, `machine-provider.v1` and `package-source.v1`, which only provider calls reach; PRV-001 adds that a plain store may implement `reveal-secret.v1` |
| 2 | PRV-021 (`then`, observable), LIF-036 (observable), the descriptions of `stop.v1` and `unregister.v1`, `spec/core/README.md` (Providers) | They had every runtime provider carry out a stop it cannot carry out now once it can. The owner's answer D11, as the record states it, is that "the runtime provider's contract states what it does with a stop it cannot carry out now, such as carry it out once it reaches the machine" | PRV-021's `then`: its contract states what it does with that stop, such as carry it out once it can reach the machine, whether or not the instance still exists, and it does what it states. The other four places say the same. See [choice 1](#choices-beyond-the-record) |
| 3 | DL13; PRV-003 (observable) | DL13 kept its sentence "on one machine, over stdio, it is what the engine's own runner gives the process", beside a new one giving `identity` to every run (the language group's open item). Nothing said how the engine's own runner, which no provider call reaches, is given `identity` (the language and core groups' open item) | DL13: `identity` is given to every run, on the engine's own runner as under any runtime provider, in place of the stdio sentence. PRV-003: the engine gives its own runner, without a call, what `run.v1` would give, `secrets` and `identity` among it, and the runner delivers them as PRV-011 and PRV-022 have a provider deliver them |
| 4 | IFC-017 (observable); DL16 | Both said that what a process of the engine's own runner receives beyond PRV-023's variables is DL16, leaving out what `identity` holds (PRV-022) | Both name PRV-022 for `identity` |
| 5 | PRV-015 (observable) | Where the type or an item names `kubernetes.io/hostname` with another value for an instance on the engine's own runner, PLC-007 says that no location matches, but PRV-015 still had the engine ask `place.v1` with "the label selector of PLC-007", which then names one label twice and is not a `label-selector.v1` (the machines group's open item 6) | The provider is not asked, and the instance is not instantiated, as where the provider names none. See [choice 2](#choices-beyond-the-record) |
| 6 | ACC-019 and IFC-041, observables | "Fails over" and "failover", which no row of the definitions defines | "After the engine restarts, on its machine or on another", the words of SEC-023 |
| 7 | IFC-041 (`then`) | It sent the secret "read under SEC-013" only, though after a restart the value is the one read again under SEC-023 | "Read under SEC-013, or again under SEC-023" |
| 8 | `spec/definitions.md` | No group had it. The rows that the table and the answers name had not changed, and the row for a repeat still sent the case of outputs no longer kept to OPS-032 (the conventions group's open item) | The rows listed under [Definitions](#definitions) follow the promises; a row for `identity` is added, since PRV-007, PRV-022 and IFC-022 use the word beside "operation identity" |
| 9 | `spec/README.md` | No group had it. The counts were those before ruling 42; the row for `language` lacked row 14's sentence; the section on secrets still listed rotation, the other capabilities of a store's Type and the run left on a machine reported gone as open, and had none of the costs of D8, D12 and D17 | Status: ruling 42, with D5 and D20 left for research and the names accepted but not yet applied. Row for `language`: resolving reads the lifecycle state and status of what exists (row 14). "What this is": `identity` (D17) and the one request made after an invocation has ended (SEC-023). Counts: 439 promises, 23 in `providers.yaml` and in `secrets.yaml`, 35 in `lifecycle.yaml`, 31 in `operations.yaml`, 15 items, 26 files under `invalid-by-rule`. Secrets: rows 8 and 23 and the answers D8, D11, D12 and D17 are stated limits; D10's line is gone; D5's line stays, pointing to D5 |
| 10 | `spec/schemas/README.md` | Its final newline was lost in the core group's edit | Restored |
| 11 | Head comments of `providers.yaml`, `implementation.yaml`, `access.yaml`, `lifecycle.yaml`, `implementation-defined.yaml` and `core/operations.yaml`; the status of `spec/core/README.md` | | Each names this pass, this log and what it changed |

### Definitions

The rows changed, and the row or answer behind each: resolving (row 14); project and qualified
reference (row 10); install (row 36); stand for, machine and machines provider (rows 13 and
28); placement (rows 1 and 28); unreachable, gone (row 13); depends on (R5); close (D6); caller
and request (D12); setup hook, retire hook, hook (row 20); the engine's ID for an invocation and
repeat (rows 31 and 32); reconcile, inspect, status and purge (row 32); cancel (row 33); how a
request ends: conflict, unavailable and the paragraph below the table (rows 28 and 31);
connection (row 9 and D12); plain (row 15); Conduit's extension and Conduit's keys (rows 15
and 29); provider and secret Resource (D10); the engine's own runner, machine and fetching (rows
1 and 17, D8 and D17); runtime provider (D11, D17, row 27); `runtime` (rows 1 and 27); run (rows
16, 22, 23 and 24, D11 and D17); label selector (rows 1 and 26); provider call (rows 12 and
15); the engine's read of a secret (row 9 and D12); set up (rows 11, 33 and 40); remove and
unregister (D11); record (rows 12 and 18). New: `identity` (D17). The head paragraph says that
ruling 42 accepted the names of the vocabulary audit and that this file keeps the earlier names
until they are applied.

## Where each decision landed

Every row of the table "Settled by the coordinator" but row 5, and every answer of the owner,
landed. Rows 30 and 41 to 43 list no edit and needed none.

| Row or answer | Where |
| --- | --- |
| 1 (D1) | PLC-007, PRV-003, PRV-006, PRV-015, PRV-019, POOL-009, `place.v1`; definitions (this pass). `docs/OVERVIEW.md` is outside `spec/`: [for other jobs](#for-other-jobs) |
| 2 (D2) | PRV-020, PRV-008 |
| 3 (D3) | TPL-009, TPL-018, DL10, `retire.v1`, `retire-hook.v1` |
| 4 (D4) | PLC-027 |
| 6 (D7) | ACC-029, `bound.v1` |
| 7 (D8 part) | Superseded by D8 |
| 8 (D9) | SEC-014; `spec/README.md` (this pass) |
| 9 (D12 part) | SEC-011, SEC-012, SEC-013, TPL-009, TPL-012, TPL-018, ACC-039, CON-008, IFC-041, `retire.v1`, `retire-hook.v1`; definitions (this pass) |
| 10 (R27) | SUB-008, MAN-015, ACC-029, `bound.v1`; definitions (this pass) |
| 11 (D13) | REV-012, REV-014, TPL-012, PLC-017, PLC-024, PLC-026, PRV-007, `set-up.v1` |
| 12 (D14) | REC-002, SEC-011, SEC-015, PRV-005, IFC-020, `record.v1`; definitions (this pass) |
| 13 (D15) | PLC-023, PLC-026, PRV-013, PRV-014, FST-001, DL5, `list-machines.v1`, `location.v1`; definitions (this pass) |
| 14 (D16) | POOL-016; `spec/README.md` (this pass) |
| 15 (D18) | IFC-037, PRV-001, PRV-005, PKG-020, rule `plain-provider` and its example; definitions (this pass) |
| 16 (D19) | DL5, PRV-003, PRV-007, PRV-009, IFC-017, `run.v1`; definitions (this pass) |
| 17 (D20 part) | PRV-007, MAN-010, `run.v1`, kept for D20 |
| 18 (R3) | REC-002, CON-008, `record.v1` |
| 19 (R5 part) | LIF-021, `retire-with-resources.v1` |
| 20 (R6) | TPL-017, TPL-018, the two hooks; definitions (this pass) |
| 21 (R7) | OPS-028, OPS-008, IMP-007 |
| 22, 23, 24 (R9, R10, R11) | MAN-017, PRV-007, PRV-011, SEC-010, IFC-041, `run.v1`, rule `item-place-required` and two examples; `spec/README.md` for row 23 (this pass) |
| 25 (R13) | SEC-004 |
| 26 (R16) | PLC-025, rule `label-conflict` and its example |
| 27 (R17) | PLC-005, POOL-009, PRV-006, PRV-020, LIF-009, `register.v1`, `register-replacement.v1` |
| 28 (R19) | PRV-015, POOL-016, LIF-009; definitions (this pass) |
| 29 (R20) | IFC-020, IFC-023, IFC-029, IFC-036, IFC-038; definitions (this pass) |
| 31 (R22) | OPS-032, OPS-034, OPS-005, OPS-008, OPS-009, OPS-024, IFC-016, IFC-025, REC-004, ACC-012, REV-014, DL10; definitions (this pass) |
| 32 (R23) | LIF-013, OPS-014, REC-004, `inspect.v1`; definitions (this pass) |
| 33 (R25) | OPS-027, TPL-009, TPL-012, LIF-021; definitions (this pass) |
| 34 (R26) | ACC-018 |
| 35, 36 (R27) | MAN-010, `implementation-item.v1`; PKG-029, SUB-008, MAN-016 |
| 37, 38, 39, 40, 44 | PRV-004; PRV-005; REV-014; TPL-012; CTR-012 |
| D6 (row 5) | ACC-011, `close.v1`; definitions (this pass) |
| D8 | PRV-023, PRV-003, PRV-007, SEC-020, IFC-017, DL16; definitions and `spec/README.md` (this pass) |
| D10 | SEC-022, SEC-005, PRV-001, `secret-reveal.v1`, rule `secret-only` and its example; definitions and `spec/README.md` (this pass) |
| D11 | LIF-036, PRV-021, LIF-017, PLC-017, PLC-027, PRV-005, PRV-008, PRV-020, TPL-012, `stop.v1`, `unregister.v1`; definitions and `spec/README.md` (this pass) |
| D12 | SEC-023, SEC-007, SEC-008, SEC-011, SEC-012, SEC-013, SEC-017, SEC-018, SEC-019, ACC-019, REC-002, IFC-041, `reveal-secret.v1`; definitions and `spec/README.md` (this pass) |
| D17 | PRV-022, PRV-007, PRV-018, IFC-022, IFC-030, REC-002, DL13, `run.v1`; definitions and `spec/README.md` (this pass) |
| R5 | LIF-021, `retire-with-resources.v1`, `spec/core/README.md`; definitions (this pass) |

The notes "Settled together" hold: PRV-007, PRV-006, PRV-015, POOL-016, PRV-005, REC-002, REV-014,
TPL-012, LIF-021, DL5 and `run.v1` each carry their rows as one change; "no later part begins"
is written once, in TPL-012; REV-014's exception names OPS-034; PRV-008 no longer says that
there is always a provider to ask.

## Choices beyond the record

Each is this pass's, and the coordinator or the owner can strike it.

1. **PRV-021 in D11's words.** The groups wrote an obligation on every runtime provider to carry
   out a stop it cannot carry out now. This pass put back the record's words: the contract
   states what the provider does with it, carrying it out being the example. The stronger form
   is the other option: every runtime provider must stop such a run once it can, as the kubelet
   and Nomad's clients do. It would close the case of a run left on a machine that comes back
   whatever provider the user picks, and is a candidate for the research on work across
   machines.
2. **PRV-015, a selector that names one label twice.** The provider is not asked. The outcome
   is the one PLC-007 already gives, that the instance is not instantiated; the other reading,
   to ask `place.v1` with the engine's label alone, would let a provider name the engine's
   machine for an instance whose type asks for another.
3. **DL13 drops its stdio sentence.** What the engine's own runner gives a process to present
   is now what the mechanism puts in `identity`, as for any run. An engine whose mechanism on
   one machine needs no cryptographic proof still states its choice there, since IFC-022 asks
   for one only where a request crosses machines.
4. **Definitions and `spec/README.md`.** This pass wrote the rows and the limits that the table
   and the answers name for these two files, in the words of the promises. The limit for D11
   is stated as a limit, because under D11's words it rests on each provider's contract.

## Needs a decision

1. **A retire hook on a `remote` item after the engine restarts.** Where a connection made by a
   setup lasts and the engine has lost its values, TPL-018 makes no new connection, since one
   lasts; SEC-023 then reads again, for the setup's caller, through the instance's own Conduit,
   whose consumer is now retired; ACC-039 refuses that read, because it belongs to no
   retirement; and TPL-009 says that a retirement does nothing else than its hook, its stop and
   the run or connection of TPL-018. So the hook reports unavailable, the retirement does not
   succeed, and the connection ends with it; a further retirement connects anew and succeeds.
   Options: (a) TPL-018 counts a connection whose values were lost as one that does not last,
   so the retirement connects anew with reads that belong to it, as row 9 has it for a
   retirement with no connection; (b) ACC-039 also permits SEC-023's read before a retire hook,
   and TPL-009 names it. Recommendation: (a), which keeps every read of a retirement its own.
2. **A name in both `identity` and what the item binds** (the machines and core groups' open
   items). No promise says which value a run gets where a variable of `identity` has the name of
   one in `environment` or `secrets.environment`. Options: `identity` wins; the item's binding
   wins, as PRV-023 has it for the six variables; or the part ends failed. Recommendation:
   `identity` wins, so that no configuration can send the run's credential to an address it
   chose.
3. **LIF-036 removes the instance whatever came of the stop** (the conventions group's choice
   8, which the core group's `unregister.v1` follows), and **`unregister.v1` keeps
   `deduplicates: true`** (the core group's choice 3). Both read the request to unregister as
   the assertion that the shutdown is settled another way. For the coordinator to confirm.
4. **The ruling's table line for R5.** The record's table "Earlier rulings this changes" says
   that `retire-with-resources.v1` "goes on with every Resource that does not depend on it".
   The answer accepted, and LIF-021 as written, hold back only what the order puts after the
   Resource that could not be retired, which is what that Resource refers to or uses. The table
   line is the record's, which this pass does not edit; the coordinator may correct it.
5. **Smaller points the groups left open**, unchanged here: PRV-023 does not skip a value that
   begins with `()`, as the SDKs do; PRV-021 does not say what becomes of a stop that a provider
   still holds when it is retired after the instance is removed; SEC-013 does not say when the
   connection of an instance that the engine retires itself (PLC-017, PLC-024) ends; whether
   SEC-023 stops trying a read that keeps failing before the next setup; MAN-017's check does
   not catch a place whose schema states none of `type`, `enum` and `const`; and no `then`
   states that no record holds `identity`, which rests on PRV-022's observable, REC-002's and
   the description of `run.v1`.

## For the research on work across machines

The owner asked for more research on how Conduit works across machines, looking past v0. These
are the questions that the text of ruling 42 leaves at that boundary, gathered from the five
groups' logs and this pass.

- D5: who may reach a run's `endpoint`, that of a store's server among them, and whether a
  header that carries a secret may go to an `http` URL. `spec/README.md` states the gap.
- D20: whether a program shipped inside a package can run under another runtime provider, which
  needs the provider to fetch and check the package. Row 17's guard stands meanwhile, and the
  valid example's Pool template `workspace` still routes a `runtime` input to an entry whose
  program is a relative path.
- How a refreshed credential reaches a run: PRV-022 lets the provider replace a file of
  `identity` in place, but no call carries a new credential to a provider during a run, and
  DL13 leaves the rest to each engine.
- Which engine address works from another machine, and what happens when the machine's network
  changes during a run (PRV-022 lets the provider give the address as the run sees it).
- What stops a run left on a machine that comes back: under D11's words the provider's contract
  states it (PRV-021); a provider retired once the instance is removed may still hold a stop.
- An onward request of an earlier hook attempt still running on a machine cut off is decided for
  the latest caller (OPS-028), a limit of this version until local enforcement.
- After the engine restarts on another machine, SEC-023's read crosses from the store's server
  to the engine's new machine, which D5 does not cover.

## For other jobs

Outside `spec/` and `tools/check_schemas.py`, which this pass does not edit:

- `docs/OVERVIEW.md`, row 1: an instance on the engine's own runner is placed where `local`'s
  hostname label matches, through the machines provider.
- `docs/AUDIT.md`, the coordinator's tracker: the candidate features of the record, which no one
  has asked for yet (a forced removal; a verb that restarts a ready instance's run; an address
  owned by its first submitter).
- The stage of the vocabulary audit: N1 to N20 and E1 to E23 over every file above, and the
  working names `identity`, `serving` and `unreconciled`.

## Verification

Date: 2026-11-06. Three readers verified the text once ruling 42, its names and its editorial
fixes had all been applied: one read the decisions and the owner's answers (F1 to F5), one the
names and the editorial fixes (names-01 to names-11), and one walked scenarios across machines
(X1 to X18). This pass checked each finding against the text. Where it holds and the fix follows
from ruling 42, from the records it accepts or from wording, the fix is applied; the rest is
listed under [Still needs a decision](#still-needs-a-decision). Four findings are one defect seen
twice: F1 and X16, F2 and X3, F3 and X4, F4 and X13. D5 and D20 stay unruled: the gap of D5 is
still stated in `spec/README.md`, now more fully (X8), and row 17's guard still stands in
PRV-007, MAN-010 and `run.v1`. This pass edited no record but this log, and nothing was
committed.

### Checks after the verification

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `440 promises ok` |
| Every text field folded with `>-`; a YAML line over 96 characters under `spec/` | Yes; none |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_schemas.py` | `Errors: 0`; 22 rules from 21 promises, 26 sets under `examples/invalid-by-rule/` each rejected by its rule |
| `git diff --check` | Clean |

440 promises, one more than after the integration (PRV-024): 198 of the layer `language`, 202
`convention` and 40 `interface`; by level, 392 `must`, 44 `must not`, 3 `may` and 1 `should`.
None was removed or moved between files. The implementation-defined list still holds 15 items.

### Each finding

| Finding | Outcome | What changed |
| --- | --- | --- |
| F1. No `then` keeps `identity` out of a record | Fixed: D17 says that no record holds what `identity` carries | New PRV-024, `must not`: the engine puts no part of `identity` in a record, an inspection result, an observation, a diagnostic, any result or `_meta`, whatever the runtime provider's type states under `records`. PRV-022 and REC-002 point to it; the description of `record.v1` leaves out what `secrets` and `identity` hold; the row `identity`; the count in `spec/README.md` |
| F2. A variable named both in `identity` and in what the item binds | Needs a decision: [item 1](#still-needs-a-decision) | None |
| F3. A retire hook to a `remote` item after the engine restarts | Fixed, by option (a) of item 1 of [Needs a decision](#needs-a-decision) | TPL-018 (`then`, observable) and SEC-013 (`when`): a retirement connects anew where no connection lasts whose values the engine holds; SEC-013's and SEC-023's observables; the descriptions of `retire.v1` and `retire-hook.v1`; the rows connection and hook. TPL-009, ACC-039 and SEC-011 are unchanged. It follows from row 9, under which such a retirement connects with reads of its own and whose reason is that a connection is the values the engine holds, and from D12, which answers what requests do, a hook not being a request. Option (b) would widen what ACC-039 permits through the Conduit of a retired consumer. The coordinator or the owner may strike this for (b) |
| F4. PRV-023 does not skip a value that begins with `()` | Fixed: the ruling adopts the SDKs' default, which skips it, and D8's option named the skip | PRV-023 (`then`, observable): each of the six only where its value does not begin with `()`; the row the engine's own runner; `spec/README.md`. The list set on the runner passes values as systemd's `PassEnvironment=` does, with no skip |
| F5. The three candidate features are not in the tracker | Open, for the job that owns `docs/AUDIT.md` | None here. As [For other jobs](#for-other-jobs) says: a forced removal (D2, D3), a verb that restarts a ready instance's run (D9), an address owned by its first submitter (D12, R27) |
| names-01. IFC-037 does not cover a hook | Fixed (E6, E7) | IFC-037 (`when`, `then`, observable): a call for a request or a hook, or a provider call; the row Conduit's extension |
| names-02. IFC-007, IFC-019, IFC-026 and two rows still say request where a hook is meant | Fixed (E7) | The `when` of each, and their `then` and observables to match; IFC-026 also says filter where it said restriction (N16); the rows caller and connection |
| names-03. A restriction sent to an implementation | Fixed (N16) | SCH-009, OBJ-021 and LIF-031, observables; the row resolved form |
| names-04. "No Resource represents them" | Fixed (N10) | PRV-003, observable; the row the engine's own runner: they are no Resource |
| names-05. Plain uses of "source" | Fixed (N14) | PLC-007: which of the type, the item or the engine's own runner gives the label; SEC-006: no other store; `spec/README.md`: the fetching of packages |
| names-06. The row node speaks of a Node's policies | Fixed (N20) | The row node: it is not treated as Kubernetes treats a Node |
| names-07. "Each store is its own Type"; "a store's contract" | Fixed (E17, E18) | The description of `secret-provider.v1`: each kind of store; `spec/README.md`: an implementation of `get-secret.v1` |
| names-08. The row argument no longer sets apart a Package's argument | Fixed (N6) | The row argument: a server.json Package's argument and the `arguments` of a call are those standards' own (MAN-010, IFC-001) |
| names-09. "Working names too" | Fixed | The head paragraph of the definitions, which also names this pass |
| names-10. The summary of N7 leaves out `oci` | Fixed (N7) | `spec/README.md`: or, for an `oci` image, by a digest in its `identifier` |
| names-11. REC-005 reads as though a node were a URI-reference | Fixed (N14) | REC-005, observable |
| X1. Runs that share an operating-system user can read each other's credential | Needs a decision: [item 2](#still-needs-a-decision) | None. IFC-022's `then` already has no consumer present as another, and every choice of DL13 and DL16 must keep it, so an engine whose runs share a user and a credential each can read does not conform. What the text does not give is the way, and that is the owner's |
| X2. A process of the engine's user can read the engine's environment | Fixed, wording: the text said "reaches", where only what the runner passes is promised | PRV-023 and SEC-020, observables; the row the list set on the engine's own runner; `spec/README.md`: the runner passes no other variable, a process of the engine's user may still read the engine's environment, and a token belongs in a store. Hardening the engine is the owner's option, [item 7](#still-needs-a-decision) |
| X3. As F2 | Needs a decision: [item 1](#still-needs-a-decision) | None |
| X4. As F3 | Fixed | As F3 |
| X5. LIF-036 removes the instance whatever came of the stop | In part. Fixed: the case of a provider whose discovery result lacks Conduit's extension, which the rule already covers, and the limit. Open: [item 3](#still-needs-a-decision) | LIF-036, observable: the stop not dispatched for want of the extension, and a stop the provider never received is not asked for again; `spec/README.md`, the bullet on a run left on a machine |
| X6. A stop that a provider holds when it is retired | Fixed, as a stated limit: under D11's words the provider's contract states what it does with the stop | PRV-021, observable; `spec/README.md`. A rule that keeps such stops is for the research |
| X7. The engine's own runner across a restart of the engine | In part. Fixed: DL16 has the engine state the runner's restart policy (PRV-010) and what becomes of its processes when the engine stops and starts again. Open: [item 4](#still-needs-a-decision) | DL16 |
| X8. The statement of D5's gap leaves out three exposures | Fixed, as stated limits; D5 is not decided | `spec/README.md`: the bullet on an Agent's credential holds through the engine; the D5 bullet adds that anything that reaches a secret Resource's server can send its own locator; a new bullet states a secret header to an `http` URL, again after each restart, and a loopback URL after a restart on another machine |
| X9. No call brings a refreshed credential | Fixed, wording | PRV-022, observable: no provider call carries one, so a credential is refreshed only by the mechanism's own means and otherwise lasts the run; DL13: how the provider gets the new content. A refresh call is for the research |
| X10. `stop.v1` and `inspect-run.v1` give no machine | Needs a decision: [item 5](#still-needs-a-decision) | None |
| X11. Which references SEC-023 reads | Needs a decision: [item 6](#still-needs-a-decision) | None |
| X12. A connection of an instance whose node is declared lost never ends | Fixed: a declaration retires at once and dispatches nothing, so no request can be sent on the connection again | SEC-013 (`then`, observable): the connection also ends when the instance's node is declared lost; the row connection. A removal or an unregistration comes after a retirement or a declaration, so it needs no words of its own |
| X13. As F4 | Fixed | As F4 |
| X14. PRV-023 against DL16 on Windows | Fixed: D8's option left what other operating systems use to the list's item | PRV-023 (`then`, observable): on an operating system on which a process needs other variables, such as Windows, those that DL16 names; DL16 names that case and the SDKs' Windows variables; SEC-020's observable; the row; `spec/README.md` |
| X15. PRV-018 reads as though a run on the engine's own runner had no `identity` | Fixed | PRV-018, observable |
| X16. As F1 | Fixed | As F1 |
| X17. The declaration of loss asks for no stop | Fixed, wording | PLC-017, observable; `spec/README.md` |
| X18. The record's line for R5 | Open, for the owner of the record, as item 4 of [Needs a decision](#needs-a-decision) says | None. LIF-021, `retire-with-resources.v1` and PRV-020 follow the answer accepted |

Of the earlier [Needs a decision](#needs-a-decision): item 1 is settled by F3's fix, unless struck;
of item 5, the skip of `()`, a stop held when the provider is retired, the end of a connection of
an instance that the engine retires itself, and the `then` for `identity` are settled. Still open
from item 5: whether SEC-023 stops trying a read that keeps failing before the next setup, and
MAN-017's check of a place whose schema states none of `type`, `enum` and `const`.

### Still needs a decision

1. **A variable named both in `identity` and in what the item binds** (F2, X3). The item's value
   cannot win: D17 has `identity` carry the engine's address, and IFC-022 has no consumer present
   as another, which a configuration that sends the run's credential to an address of its own
   choosing would defeat. Options: (a) `identity` wins, stated in PRV-022's `then`, in PRV-023's
   for the engine's own runner and in `run.v1`; (b) the part ends failed before the run, under
   PRV-007; (c) a template whose item binds a name that the engine's mechanism uses is rejected,
   which ties the validity of a document to one engine's choice under DL13. Kubernetes lets a
   container's own variables override those it injects, but there whoever writes the Pod may act
   as it, while here whoever writes an instance's configuration need not be who may act as the
   instance. Recommendation: (a).
2. **Runs that share an operating-system user** (X1). On one machine, a run whose model can run
   commands can read another run's credential file, or its environment. Options: (a) a promise
   that on one machine the proof is bound to the process: stdio, a socket for each run, a check
   of the peer's credentials, or a file that only the run's own user can read, with DL16 running
   each process under a user that cannot read another's; (b) narrow IFC-022 to runs that do not
   share a user, and state the limit in `spec/README.md`. Recommendation: (a); (b) admits that
   one consumer presents as another.
3. **LIF-036 removes the instance whatever came of the stop** (X5, and item 3 above). Options:
   keep it, with the limit now stated, since the request to unregister asserts that the shutdown
   is settled another way, as removing a finalizer does; or remove the instance only once the
   provider has received the stop, a stop that succeeded or ended unknown with the provider's
   answer, and otherwise report unavailable and keep the instance, so that a repeat asks again.
   Recommendation: keep it for v0, and take the stronger form into the research on what stops a
   run left on a machine.
4. **The engine restarting on another machine** (X7). `local` represents the machine whose
   hostname is the engine's (PLC-026), so after a restart elsewhere either the engine keeps its
   hostname, as DL5 allows and as the kubelet's `--hostname-override` does, or `local` represents
   another machine while instances placed with its old hostname label, and their absolute paths
   (PLC-020), name the first. The processes that the engine's own runner left there keep their
   values with no stop to reach them. Where the runner does not start its processes again, which
   DL16 now has each engine state, a secret Resource on that runner must be set up again before
   SEC-023's read can succeed, so in the default configuration D12's read again fails until
   then. Options: require a restart on another machine to keep the engine's hostname; or state
   that such a restart is outside v0, apart from SEC-023. For the research.
5. **The machine in `stop.v1` and `inspect-run.v1`** (X10). Options: give `machine`, the hostname
   and labels of the instance's node, retired or not, as `run.v1` does; or have PRV-021 require
   the provider's contract to state how it finds the machine. For the research.
6. **Which references SEC-023 reads** (X11): those the setup read, kept with the connection as
   state without values, or those the configuration holds now; and the same for the headers of
   IFC-041 that carry no secret. No access is granted either way, since the instance's own
   Conduit decides the read. Recommendation: the setup's, so that a restart changes nothing a
   connection carries, as "reads the secret again" in D12 suggests. For the research, since it is
   about what the engine keeps across a restart.
7. **Hardening the engine's own environment** (X2), optional and the owner's: remove from the
   engine's environment at start the variables that its list does not name, make the engine's
   process unreadable to its user's other processes, or run the processes it starts under
   another user (DL16). The text is true without it.

For other jobs: F5, the candidate features, for the job that owns `docs/AUDIT.md`; and X18, the
record's line for R5, for its owner.

For the [research on work across machines](#for-the-research-on-work-across-machines), beside
the questions above: items 2, 4, 5 and 6; how a refreshed credential reaches a run during it
(X9); a rule for the stops that a runtime provider still holds when it is retired (X6, and
choice 1 of this log); and the three exposures of D5's gap that `spec/README.md` now states, a
secret Resource's server that anything can reach, a secret header to an `http` URL, and a
loopback URL after a restart on another machine (X8).
