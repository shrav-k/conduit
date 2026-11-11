# Ruling 42: log of the language group

Written by the editor of the language group on 2026-11-06, applying parts (a) and (b) of
[ruling 42](README.md) to `spec/promises/manifest.yaml`, `packages.yaml`, `submissions.yaml`,
`contracts.yaml`, `naming.yaml`, `objects.yaml`, `instantiation.yaml` and `schemas.yaml`,
`spec/interface/implementation.yaml` and `spec/implementation-defined.yaml`. Part (a) is the
coordinator's decisions in the table
[Settled by the coordinator](../ruling-41/open-decisions.md#settled-by-the-coordinator), with
the notes "Settled together" below it, every row but row 5; part (b) is the owner's answers D6,
D8 (option c), D10, D11, D12 (option c), D17 and R5. D5 and D20 are not ruled, and nothing of
them was applied. The names N1 to N20 and the editorial fixes E1 to E23 of the
[vocabulary audit](../vocabulary-audit/README.md) were not applied: another stage applies them.
Every new sentence is written in the names in force today, such as `secret-reveal.v1`,
`reveal-secret.v1`, location and operation identity.

No other file was changed, and nothing was committed. Where an entry of the record agent's list
of edits and the table disagree, the table governs; the entries followed are named below.

## Counts

| File | Promises before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| `interface/implementation.yaml` | 38 | 12 | 0 | 0 | 38 |
| `promises/manifest.yaml` | 16 | 4 | 0 | 0 | 16 |
| `promises/packages.yaml` | 21 | 2 | 0 | 0 | 21 |
| `promises/submissions.yaml` | 10 | 1 | 0 | 0 | 10 |
| `promises/contracts.yaml` | 10 | 1 | 0 | 0 | 10 |
| `promises/naming.yaml`, `objects.yaml`, `instantiation.yaml`, `schemas.yaml` | 96 | 0 | 0 | 0 | 96 |
| All promises of the group | 191 | 20 | 0 | 0 | 191 |
| `implementation-defined.yaml` (items, not promises) | 14 | 3 | 1 | 0 | 15 |

Each of the five promise files that changed, and the implementation-defined list, has a line in
its head comment naming ruling 42 and what changed. No promise moved between files, and no ID
was reused.

## New IDs

| ID | File | Why |
| --- | --- | --- |
| DL16 | `spec/implementation-defined.yaml` | Row 7 with D8: the item "The engine's own runner". DL14 was removed by ruling 41 and is not reused, so the next number is DL16. PRV-023 and IFC-017 cite it. |

No new promise ID was needed in these files. The IDs assigned in advance to other groups are
cited as they were assigned: OPS-034, PRV-022, PRV-023 and SEC-023. Each existed when the checks
below were run.

## Decisions applied

| Row or answer | Promise IDs | What changed |
| --- | --- | --- |
| Row 3 (D3) | DL10 | The time allowed for a call covers a retire hook's, after which its part ends unknown and the stop is asked for under TPL-009. |
| Row 7 (D8, part) with D8 | DL16 (new), IFC-017 | DL16 states what a process that the engine's own runner starts receives beside what its item binds and what PRV-023 passes: its working directory, its user and login session, how the list of further variables is set on the runner, and what other operating systems use in place of the six variables. IFC-017 points to PRV-023 and DL16. |
| Row 9 (D12, part) | IFC-041 | It also covers a connection made in a retirement to serve its retire hook (TPL-018). The observable says that the connection is the engine's holding of the header values, and a failed HTTP request does not end it. |
| Row 10 (D12 R27) | SUB-008, MAN-015 | Two submissions may state one address. Each is told apart by its digest, which its templates keep; on the wire, in records and in a bound an operation is `address#name` whichever submission holds it; a bound covers the operation in every submission at that address. |
| Row 12 (D14) | IFC-020 | The record of an onward request holds the served invocation, under REC-002. |
| Row 13 (D15) | DL5 | Now "Reports of providers": how long the engine waits for the machines provider's report before every machine but `local`'s counts as omitted, and how it learns its own hostname, the operating system's unless configured, as the kubelet's `--hostname-override`. |
| Row 15 (D18, first half) | IFC-037, PKG-020 | IFC-037 covers provider calls to a provider's implementation item, reads discovery before the first request or call to each run and to a `remote` item at each connection, may read it again at any time, declares the extension on each call, and names the missing extension in the record of a request or call that is not dispatched. The engine's own implementations of PRV-003 are not covered. PKG-020: a plain implementation implements no operation of `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`; a plain implementation of `reveal-secret.v1` stays allowed. |
| Row 16 (D19) | DL5, IFC-017 | DL5 says how often the engine asks `inspect-run.v1` of its own accord. IFC-017: only the engine's own runner returns no `endpoint`, and `run.v1` requires it of every runtime provider (PRV-007). |
| Row 17 (D20, part) | MAN-010 | The guard that the ruling keeps for D20: a program given as a relative path is run only by the engine's own runner, under PRV-007. |
| Row 22 (R9) | MAN-017, IFC-041 | A template is rejected whose item points to a place whose schema admits an object or an array, by `type`, `enum` or `const`, unless `secret-reference.v1` governs it. IFC-041 sends a header's value converted as PRV-007 converts a value. |
| Row 24 (R11) | MAN-017, IFC-041 | A template is rejected whose item binds an input marked `isRequired: true` to a place whose schema admits null. IFC-041 leaves out a header whose place holds null. |
| Row 29 (R20, second half) | IFC-038, IFC-036, IFC-029, IFC-020, IFC-023 | The engine's discovery result lists Conduit's extension at `v0` under `capabilities.extensions`. A request that carries any of Conduit's keys and does not declare the extension at a revision the engine lists is answered with -32021, whose `data.requiredCapabilities` names the extension, and reports invalid (IFC-036). A request is read at the revision it declares (IFC-029). An onward request and a request that supplies an operation identity carry Conduit's keys, so they declare the extension (IFC-020, IFC-023). |
| Row 31 (R22) | IFC-016, IFC-025, DL10 | A repeat of an invocation that succeeded and whose outputs are no longer kept reports conflict under OPS-034, and the `ruleId` tells it from OPS-032. DL10: after the outputs are no longer kept, a repeat reports conflict under OPS-034. |
| Row 35 (R27) | MAN-010 | A named argument is keyed by its `name`, a positional one by its `valueHint`; a key that names no declared argument, more than one, or one that states a fixed `value` is rejected; a variable inside an argument's `value` is not bound by the map. |
| Row 36 (R27) | PKG-029, SUB-008, MAN-016 | Installing a package at a submitted project's address reports conflict only while a template or an instance holds a submission there. |
| Row 44 (G6's choice 8) | CTR-012 | The observable gives the corrected reason for `type: object` at both roots. |
| D12 (option c) | IFC-041 | Where the engine has lost the values that a connection held, as on a restart or a failover, it reads the secrets again under SEC-023. |
| D17 | IFC-022, IFC-030, DL13 | A run is given the engine's address and what it presents in the input `identity` of `run.v1` (PRV-007), which its runtime provider delivers under PRV-022. IFC-030 says that a provider call is not a request, so `identity` does not break it. DL13 adds what the mechanism puts in `identity`: the variable for the engine's address, the credential's file and path, and whether and when the file is refreshed in place. |

Rows that touch these files and needed no edit:

- Row 6 (D7). The log records, as the row asks, that the check made before ruling 41 by NAM-007
  on the addresses of a bound is dropped. NAM-007's observable already says that an item of a
  bound's `operations` does not refer to an address in its sense, so `naming.yaml` is unchanged.
- Rows 30, 41, 42 and 43 confirm IFC-008, IFC-009, NAM-008, NAM-021, PKG-017 and OBJ-019 as
  they stand. No edit.
- Row 27 (R17) adds the rejection of a `default` for an input whose destination is an entry's
  `runtime` to PLC-005 and POOL-009. INS-012 still lists `/entries/cache/runtime` as a
  destination, which stays true, and points to POOL-009, so `instantiation.yaml` is unchanged.

Owner answers with nothing in these files: D6 (ACC-011), D10 (SEC-022), D11 (LIF-036, PRV-021)
and R5 (LIF-021 and `retire-with-resources.v1`). IFC-012 and IFC-013 already say that a request
through a closed Conduit reports refused, as D6 decides.

The `run.v1` input for D17 is named `identity`, an object with `environment`, a map of names to
strings, and `files`, a map of paths to contents. The name is coined by the work order of this
stage, for the owner to strike; this group cites it and did not write `run.v1`, which is in
`spec/core/operations.yaml`.

## Choices beyond the record

Each is the editor's, and the owner can strike it.

1. DL16 keeps the user and the login session that row 7 put in the item. The record's text for
   the fixed set (option b of D8) said the item would keep "only the working directory and what
   other operating systems use instead". The owner chose option c, which rules on variables only,
   so the rest of row 7 stands. DL16 also says how the list of further variables is set on the
   runner: no Resource stands for the engine's own runner (PRV-003), so the list is the engine's
   own configuration.
2. IFC-022 and DL13 give `identity` to every run, the engine's own runner included. The record of
   D17 speaks of a run on another machine; PRV-007 and PRV-022, as the providers group wrote
   them, give `identity` to every run and have the engine's own runner deliver it in the same
   way, and these files follow them.
3. IFC-030 gained a sentence saying that a provider call is not a request, so the credential in
   `identity` does not break its rule that nothing a consumer presents reaches an implementation.
   No record entry asked for it.
4. IFC-020 keeps the record entry's note that the protocol's extensions are disabled by default
   and a client enables one only where its developer opts in. Its source is the extensions
   overview of the Model Context Protocol: "Extensions are always disabled by default and require
   explicit opt-in from the developer, at every stage."
5. IFC-036 names `data.requiredCapabilities`, from the record entry. Its source is the base
   protocol page of revision 2026-07-28: the server "MUST return a
   `MissingRequiredClientCapabilityError` (`-32021`) whose `data.requiredCapabilities` lists the
   missing capabilities". The sentence on the tasks extension comes from its rule that a server
   acts on the extension only as each request declares it, "regardless of prior declarations".
6. IFC-038's observable adds that a consumer learns from the discovery result at which revision to
   declare the extension. IFC-038's `capabilities.extensions` follows the protocol's page on
   extension negotiation.
7. PKG-020's rejections now say "when the package is installed or the project is submitted" for
   the whole sentence, not only for the new rule, as the record entry has it for the new rule: a
   submitted project's manifest is read under the same rules.
8. PKG-029's observable leaves out two phrases of the record entry: "like a record", since a
   record is history and does not come to name a later package, and "as decision 11 intends",
   since no promise cites a decision of the review.
9. SUB-008's observable no longer cites MAN-016 for "no package can be installed at its
   address": MAN-016 is the other order, and is now cited for it.
10. CTR-012 and IFC-037 keep the precedents of the record (the protocol's `outputSchema`, which
    may be any schema; the kubelet and the v1 CRI API) in their observables.
11. PKG-023's list of causes for a rejected installation was left as it is: it says "such as",
    and it names neither SEC-005 nor PRV-001, so adding SEC-022 alone would not make it complete.
12. Text that did not change keeps its lines. Changed text is wrapped at 96 columns, and returns
    to the earlier lines where the rest of a field is unchanged.

## Needs a decision, or a later stage

- MAN-017 rejects a place whose schema admits an object or an array "by `type`, `enum` or
  `const`", as row 22 has it. A place whose schema states none of them, such as `{}`, admits
  every value and is not caught. For a reader of round 5, with row 22.
- DL13 now says both "on one machine, over stdio, it is what the engine's own runner gives the
  process it starts" and that every run is given `identity`. Whether the first sentence still
  says anything once `identity` reaches the engine's own runner too is for the integration, with
  how PRV-003 has the engine's own runner receive `identity` without a provider call.
- D5 is not ruled. IFC-022, DL13 and IFC-030 cover requests to the engine only; who can reach a
  run's `endpoint` stays the gap stated in `spec/README.md`.
- IFC-038 says that the engine notifies "each stream on which a request of that instance asked
  to be told". The changelog of revision 2026-07-28 replaces the earlier stream with
  `subscriptions/listen`. Left for the check against the protocol in round 5 (R28); nothing was
  changed here.
- The checker rule for PKG-020 and its invalid example (row 15) are in `tools/` and
  `spec/schemas/`, which another group owns.

## Checks

Run from the repository root after the edits:

- The validation of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation): "439 promises
  ok". The implementation-defined list loads as YAML, with 15 items.
- `python3 tools/check_docs.py`: 1 error, in `spec/core/README.md`, which links to
  `docs/proposals/ruling-42/log-core.md`, the log of another group not yet written. None is in
  these files.
- `python3 tools/set_names.py --check`: "Errors: 0".
- `python3 tools/check_schemas.py`: "Errors: 0".
- `git diff --check`: no output.
- No line of the changed YAML files is over 96 characters, no code span is split across lines in
  the new text, and every ID that the new text cites exists, DL14 apart, which a head comment
  names as not reused.
