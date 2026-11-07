# Ruling 42: log of the docs editor

Date: 2026-11-06. This is the account of the docs editor's pass after the five groups and the
[integration](log-integration.md) applied [ruling 42](README.md). The rulings are the owner's
and the decisions the coordinator's. The edits below are the docs editor's, and each stands
unless the coordinator or the owner strikes it. Nothing was committed.

The files of this pass: `spec/definitions.md`, `spec/README.md`, `spec/schema-profile.md` and
`docs/OVERVIEW.md`. The integration had already brought the first two up to the decisions and
the answers (its fixes 8 and 9). This pass read them against the promises as they now stand,
completed them, and brought the other two up to the ruling. As in every group, the names N1 to
N20 and the editorial fixes E1 to E23 of the
[vocabulary audit](../vocabulary-audit/README.md) were not applied, so the text keeps the names
in force today; and D5 and D20 keep what the text says for them: for D5 the gap stated in
`spec/README.md`, and for D20 the guard of row 17.

## What changed

| File | Change | Row or answer behind it |
| --- | --- | --- |
| `spec/definitions.md`, head paragraph | Says that ruling 42 accepted the names and the editorial fixes of the vocabulary audit, which wait to be applied; that `identity`, `serving` and `unreconciled` are working names too, as the [open questions](../ruling-41/open-decisions.md) call them; and which rows this pass changed and added. The sentence "The name is a working name" moved there from the row for `identity` | The record; rows 12 and 32; D17 |
| `spec/definitions.md`, new row `serving` | The key by which an onward request names the invocation it serves, and the member of a record's `data` that holds it: for a hook, a read or a part, the setup or retirement it belongs to, and for a read made again, the setup that made the connection; none for a provider call | Row 12 (IFC-020, REC-002, ACC-019) |
| `spec/definitions.md`, new row `unreconciled` | The member of the status that lists the invocations of unknown outcome, not yet reconciled, that were dispatched to an instance or name it; read from the engine's own state, so no purge shortens it | Row 32 (LIF-013, OPS-014, REC-004) |
| `spec/definitions.md`, new row "the list set on the engine's own runner" | The further variables of the engine's environment that the runner passes to every process it starts, set by the user as DL16 leaves to each engine, as systemd's `PassEnvironment=` | D8 (PRV-023, DL16, SEC-020) |
| `spec/definitions.md`, new row "fail over, failover" under the words the promises do not use | The record says "fails over"; the promises say that the engine restarts, on its machine or on another (the integration's fix 6) | D12 (SEC-023, IFC-041) |
| `spec/definitions.md`, row `identity` | The engine's own runner is given it without a call and delivers it in the same way | D17 with the integration's fix 3 (PRV-003) |
| `spec/definitions.md`, row location | A location is made for a machine reported reachable or unreachable, and a machine reported again after its location was retired gets a new one | Row 13 (PLC-026, PLC-024) |
| `spec/definitions.md`, row argument | An argument of a Package is named by its `name`, or by its `valueHint` where it is positional | Row 35 (MAN-010) |
| `spec/definitions.md`, row implementation item | The place a binding points to admits neither an object nor an array, unless `secret-reference.v1` governs it | Row 22 (MAN-017) |
| `spec/definitions.md`, row setup hook, retire hook, hook | The restrictions a hook carries hold no filter in this version; a retirement connects to a `remote` item for its retire hook where no connection lasts | Rows 34 and 9 (ACC-018, TPL-018) |
| `spec/definitions.md`, row `runtime` | The input of a Pool's template that supplies an entry's `runtime` states no `default` | Row 27 (POOL-009, PLC-005) |
| `spec/definitions.md`, row revision | The engine's own retirements and closings take effect at once | Row 11 (REV-012) |
| `spec/definitions.md`, row later work | Names the one exception, the read that SEC-023 makes again | D12 (ACC-019) |
| `spec/definitions.md`, paragraph under "How a request ends" | One line rewrapped to 96 characters | None |
| `spec/README.md`, status | Ruling 42 also accepted the editorial fixes of the vocabulary audit; the sentence on what has been applied names ruling 42 apart from the names and the editorial fixes. The link to the open questions no longer spans two lines, so `check_docs.py` checks it (see below) | The record |
| `spec/README.md`, Contents | The rows for `providers.yaml` and `secrets.yaml` name what the decisions added: what a run is given to reach the engine, a stop that cannot be carried out now, what the engine's own runner passes; what else a secret Resource's type may offer, and the read made again. The counts the integration wrote agree with the validation: 439 promises, 23 in each of the two files, 35 in `lifecycle.yaml`, 31 in `operations.yaml`, 15 items, 26 files under `invalid-by-rule`, 23 Operation contracts, 102 exports | D8, D10, D11, D12, D17; rows 14 and 31 |
| `spec/README.md`, provider contracts | SEC-022 among the `language` examples, PRV-022 among the `convention` ones | D10, D17 |
| `spec/README.md`, What the guarantee for secrets covers | The opening says that the limits follow from the decisions and the answers that ruling 42 accepted, since rows 8 and 23 are the coordinator's. The bullets the integration wrote were read against SEC-014, SEC-020, SEC-023, PRV-021, PRV-022, PRV-023, PLC-027 and LIF-036 and stand. The D5 gap stays, as the owner did not rule it | Rows 8 and 23; D8, D11, D12, D17 |
| `spec/schema-profile.md`, head | Names ruling 42 and what it changed on this page | The record |
| `spec/schema-profile.md`, row Implementation items | An argument named by `name` or `valueHint`, and a binding of one with a fixed `value` rejected; a bound place admits no object or array; a boolean as `true` or `false`; null gives nothing | Rows 22, 24 and 35 (MAN-010, MAN-017, PRV-007, IFC-041) |
| `spec/schema-profile.md`, row Canonical form of a value | A number that an item binds is given in the form RFC 8785 writes it | Row 22 (PRV-007, IFC-041) |
| `docs/OVERVIEW.md`, head | Last read as ruling 42 left it, before the names were applied | The record |
| `docs/OVERVIEW.md`, Pools, ownership and placement | `retire-with-resources.v1` holds back only what the order puts after a Resource that cannot be retired; an instance with a program to run and no runtime provider is placed only where `local`'s hostname label matches, through the machines provider; unavailable where that provider is not ready or cannot place now; an omitted machine counts as unreachable; replacing the machines provider retires no location | R5; rows 1, 13 and 28 |
| `docs/OVERVIEW.md`, Conduits and access | A request through a closed Conduit reports refused | D6 |
| `docs/OVERVIEW.md`, One lifecycle | Unregistering asks for the stop once more, and the provider's contract states what it does with a stop it cannot carry out then | D11 |
| `docs/OVERVIEW.md`, Operations, outcomes and reasons | OPS-034 beside OPS-032; `unreconciled`; the read made again as the one exception to ACC-019 | Rows 31 and 32; D12 |
| `docs/OVERVIEW.md`, Providers | What the engine's own runner passes to a program; only it runs a program at a relative path; a provider call declares Conduit's extension and no `plain` server is a runtime, machines or package-source provider; `identity` | D8, row 17, row 15, D17 |
| `docs/OVERVIEW.md`, Secrets | A secret Resource's type offers nothing else; a secret goes to a variable, an argument or a header; a rotated value arrives only in a later run; the read made again after a restart | D10, rows 23 and 8, D12 |

## What this pass checked

- Every row of `spec/definitions.md` that the integration changed (39) and the new row for
  `identity`, against the promises it cites. They agree with the promises as they stand; the
  rows above were changed where a decision had changed a promise that a row also states.
- Which rows name a term that no promise uses. Only "location type" does: the promises say "the
  type `location.v1`", and `spec/core/types.yaml` still says "the location type". The row is
  kept, because E10 of the vocabulary audit removes it together with the words in
  `spec/core/types.yaml`, which this pass does not edit; the stage that applies E10 removes
  both. No term that the decisions removed from the promises is left with a row.
- The counts of `spec/README.md`, with the validation command and a count of the core package's
  and the schemas' contracts and of the example files.
- That no prose line of this pass is longer than 96 characters, and that no link of these four
  files has its text split across two lines.

## Choices beyond the record

Each is this pass's, and the coordinator or the owner can strike it.

1. **Rows for `serving` and `unreconciled`.** Both were described inside other rows (Conduit's
   keys, record, status, inspect). A row of their own says what each is in one place, as the
   rows for `supports`, `fetch` and `deduplicates` do for other keys.
2. **A row for the list set on the engine's own runner.** The promises name it by a phrase and
   not a coined word, so the row takes the phrase, and coins nothing.
3. **"Fail over" among the words the promises do not use.** The ruling's record and D12 use it;
   the integration replaced it in ACC-019 and IFC-041.
4. **The schema profile's server.json and RFC 8785 rows.** Rows 22 and 35 adopt server.json's
   `valueHint` and RFC 8785's form of a number, which the profile, as the list of standards
   relied on, now names.
5. **The overview's new sentences** follow the promises in ordinary words and bind nothing.

## For other jobs

- `tools/check_docs.py` does not check a link whose text spans two lines: its patterns exclude a
  newline inside the brackets. This pass found two such links in its files, one to a file that
  did not yet exist, and put each on one line. The checker is not this pass's to edit.
- The stage of the vocabulary audit: N1 to N20 and E1 to E23 over these four files too, among
  them E2, E3, E4, E6, E10, E11, E14 to E17, E19, E20 and E22 on `spec/definitions.md`, and E20
  on the overview's table of families.
- The points that the [integration](log-integration.md#needs-a-decision) left for a decision
  are unchanged, and none of them is written into these files.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `439 promises ok` |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0` |
| `python3 tools/check_schemas.py` | `Errors: 0` |
| `git diff --check` | Clean |
