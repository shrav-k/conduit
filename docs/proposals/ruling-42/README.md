# Ruling 42: the questions after the rewrite, and the names

Status: in force; amended by 43 (D17); done (D5 and D20: ruling 43) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 3. Vocabulary and the project's names](../../../IN_FORCE.md#3-vocabulary-and-the-projects-names), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PLC-026](../../../spec/promises/placement.yaml), [SEC-023](../../../spec/promises/secrets.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After the rewrite under rulings 40 and 41, the coordinator put two things to the owner:

- Nine questions that the rewrite left open and that no ruling or adopted standard decides,
  from the [open questions after the rewrite](../ruling-41/open-decisions.md#for-the-owner).
  They were put to the owner as questions 1 to 9: D5, D6, D8, D10, D11, D12, D17, D20 and R5,
  in that order. In the same message the coordinator recommended two answers of its own in
  place of the analysts': option c of D8 and option c of D12.
- The twenty names of the [vocabulary audit](../vocabulary-audit/README.md#names-for-the-owner),
  N1 to N20, with two smaller questions that go with N7 and N14.

The 43 questions that the coordinator settled under rulings 37 and 38 were listed with them, in
the same record, for the owner to strike.

The owner's ruling: "I accept every suggestion except for 1 and 8, I'd like some more research
and clarification on how to solve the multi machine issue and be forward looking even if some
functionality doesn't exist in v0".

## What follows

Questions 1 and 8 are D5 and D20. Both are about work across machines, which is what the owner
asks more research on. The coordinator reads "every suggestion" as every recommendation of that
message and of the vocabulary audit.

Accepted as recommended:

- **D6, a request through a closed Conduit.** It reports refused, as ACC-011 says. Decision 17's
  cost line, "a request through it reports invalid rather than refused", is corrected: a request
  that names the closed Conduit itself reports invalid.
- **D8, what the engine's own runner passes to a process.** The variables `HOME`, `LOGNAME`,
  `PATH`, `SHELL`, `TERM` and `USER` of the engine's environment, as the stdio clients of the
  protocol's SDKs pass by default, and the further variables that a list set on the engine's own
  runner names, as systemd's `PassEnvironment=` does. What the item binds is added, and wins.
  Nothing else of the engine's environment reaches the process. This changes section 4 of the
  principles review, line 18 ("what else a run receives is the provider's"), for the engine's own
  runner.
- **D10, other capabilities of a secret Resource's type.** A Type that offers the secret
  capability offers no other, and is rejected when its package is installed if it does. A store
  package that lists or writes secrets does so through another Type, whose Resources are not
  secret Resources.
- **D11, a run left on a machine that comes back.** When an actor unregisters an instance on a
  location declared lost, the engine asks the runtime provider for the stop once more. The
  runtime provider's contract states what it does with a stop it cannot carry out now, such as
  carry it out once it reaches the machine.
- **D12, a remote server whose headers carry a secret, after the engine restarts or fails
  over.** The engine reads the secret again, for the caller of the setup that made the
  connection. The read is decided by the instance's own Conduit and its filters as they stand
  when it is made, so closing that Conduit stops it, and nothing is stored. This changes decision
  4c of the principles review, under which every read belongs to a request that is in progress.
- **D17, how a run on another machine reaches the engine and proves which instance it is.**
  `run.v1` carries variables and files that the mechanism of DL13 fills: the engine's address as
  a variable, and a credential as a file that the runtime provider may refresh in place. The
  provider keeps them as it keeps a run's secrets, and no record holds them.
- **R5, retiring a Pool when one Resource cannot be retired.** The request goes on: only the
  Resources that the order puts after the one that could not be retired wait, every other one is
  retired, the Pool is not, and a repeat finishes. This changes decision 5 of round 3 of the
  re-audit, which ruling 39 accepted.
- **The names N1 to N20**, as the vocabulary audit proposes them. Among them: the capability
  `secret-provider.v1` and its operation `get-secret.v1`; location becomes node; operation
  identity becomes idempotency key; origin becomes source. With N7, an OCI image is pinned by a
  digest in its identifier, as server.json does. With N14, a condition's `source` takes the form
  of a record's, a URI-reference that the engine names.

The owner struck none of the 43 decisions of the coordinator. The 23 [editorial fixes](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/vocabulary-audit/README.md#editorial-fixes) of the
vocabulary audit are the coordinator's to apply.

Not ruled:

- **D5, requests from the engine to a server on another machine,** and **D20, a program shipped
  inside a package, under another runtime provider.** The owner asks for more research on how
  to make Conduit work across machines, looking ahead to what later versions will need even
  where v0 does not carry it. Until then, the text keeps what the coordinator settled for each:
  for D5, the gap is stated in `spec/README.md`; for D20, the guard of row 17 of the record, under
  which no runtime provider other than the engine's own runner is asked to run such a program.

## Earlier rulings this changes

| Ruling | What it said | Now |
| --- | --- | --- |
| 41 (decision 17 of the principles review) | A request through a closed Conduit reports invalid. | Refused (D6). |
| 41 (decision 4c) | Every read of a secret belongs to a request in progress. | Also a read again for the caller of the setup, where a connection's values were lost (D12). |
| 41 (section 4, line 18) | What else a run receives is the provider's. | For the engine's own runner, six variables and a list set on the runner (D8). |
| 39 (decision 5 of round 3) | `retire-with-resources.v1` stops at the first Resource whose retirement does not complete. | It goes on: only the Resources that the order puts after the one it could not retire wait (R5). |
| 17, 18 (V4), 33 | Location; origin; the earlier names that N1 to N20 replace. | The names of the vocabulary audit. |

## After the ruling

The coordinator's decisions on what the logs left open, under rulings 37 and 38:

- A retire hook on a `remote` item after the engine restarts: a connection whose values were
  lost counts as one that does not last, so the retirement connects anew with reads of its own
  (option a of item 1 of the integration log). It keeps every read of a retirement its own.
- A request to unregister removes the instance whatever came of the stop that LIF-036 asks for,
  and `unregister.v1` keeps `deduplicates: true` (item 3). The request is the assertion that the
  shutdown is settled another way, and D11 leaves a stop the provider cannot carry out now to its
  contract.
- The table line for R5 above said "every Resource that does not depend on it"; it is corrected
  to what the answer and LIF-021 say (item 4).

The other open items of the integration log bear on work across machines, and wait for the
owner's answer on the [multi-machine proposal](../multi-machine/README.md).

## Choices that set behavior

Copied from the editor logs before their removal under ruling 58. These are the choices at
the time of each pass, including readings and alternatives; later rulings and the specification
govern. The source labels retain the log and its choice number.

### log-conventions

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-conventions.md). 10 choices.

- log-conventions, 1: **D6, ACC-011.** The analyst's entry for row 5 was not applied as such, as the work order asks. The text applied is D6's option "Refused, as the text says", in the words of [D6](../ruling-41/open-decisions.md#d6-a-request-through-a-closed-conduit). It comes to the same rule as the entry.
- log-conventions, 2: **Row 31, ACC-012.** `close.v1` declares no outputs, so OPS-034 never answers a repeat of it. The analyst's entry would have cited OPS-034 beside OPS-009 and OPS-032; the promise says instead that OPS-034 does not apply to it. The table's row 31 lists ACC-012, and this is how it is changed.
- log-conventions, 3: **Row 34, ACC-018.** `retire-with-resources.v1` is named beside `set-up.v1` and `retire.v1` among the operations that `engine` implements, since the retirements it dispatches carry the hook too; and the request that carries a filter is said to report unavailable, as ACC-015 has it.
- log-conventions, 4: **D12, ACC-019.** The exception is written into `then`, which forbids every request on behalf of an invocation that has ended, and not only into the observable: without it, ACC-019 would forbid the read that SEC-023 makes.
- log-conventions, 5: **D12, REC-002.** A read made again under SEC-023 names, as the invocation it serves, the setup that made the connection. SEC-023 and SEC-011, as the secrets group wrote them, say the same.
- log-conventions, 6: **D17, REC-002.** REC-002 says that the record of a call of `run.v1` holds no part of its `identity`, citing PRV-022, which states the rule, so that the promise that lists what a record holds names it. The sentence on `secrets` cites the description of `run.v1`.
- log-conventions, 7: **R5, LIF-021.** The rule is written as the order's: a Resource's retirement is dispatched only once every Resource that the order puts before it is confirmed shut down. Ruling 42's table says "every Resource that does not depend on it"; the record and R5's option say "only the Resources that the order puts after the one that could not be retired wait". The two are read as one: what waits is what the stuck Resource refers to, names in `runtime` or is the consumer of a Conduit to, and so on in turn, which it may still be using, and within a cycle what the engine's order puts after it. A Resource that neither relation joins to another holds back none, whatever sequence DL1 gives.
- log-conventions, 8: **D11, LIF-036.** Beyond the record: the instance is removed once the stop has ended, or was not dispatched because the runtime provider is not ready, whatever came of it, and the outcome of the request is that of the removal, as the request to unregister asserts that the shutdown is settled in another way (row 4). The core group's description of `unregister.v1` says the same. PLC-023 does not hold the stop back, since a location declared lost stands for no machine (PLC-026). The promise covers only an instance for which an item that is run is selected, as PRV-008 does.
- log-conventions, 9: **Row 33, OPS-027.** "Every retirement still asks for its stop" is read to mean also that a stop already in progress is not cancelled: the cancellation is passed on to a provider call other than a stop. The instance is retired from the moment its retirement was dispatched (LIF-005), so a cancelled stop would gain nothing.
- log-conventions, 10: **Row 32.** The list `unreconciled` is stated in LIF-013's observable, beside the rest of what the status holds, as the analyst's entry put it, and not in its `then`.

### log-core

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-core.md). 12 choices.

- log-core, 1: `identity` is optional, since the engine gives it only where its mechanism gives a run something. The keys of `files` are non-empty strings, and their form, absolute or relative, is DL13's. A file's content is a string: a credential that is not text would need an encoding, which is not fixed.
- log-core, 2: `run.v1`'s `secrets` and `identity` state no `additionalProperties: false`, as the record wrote `secrets` and as `location` is written.
- log-core, 3: `unregister.v1` keeps `deduplicates: true`, although it now asks for a stop: the instance is removed whatever comes of the stop and the outcome is the removal's (LIF-036), and a new attempt only asks for the stop again, which succeeds where nothing runs. The rules comment says so. Its `effect` names no stop. The alternative is to drop the mark, so that a repeat after an unknown outcome reports unknown under OPS-012.
- log-core, 4: The descriptions cite no promise ID, as before, because a description becomes a tool's (IFC-033). The IDs are in the comments and the READMEs.
- log-core, 5: Some contracts changed that the record agent's list for these files did not name, because a row of the table changes what their descriptions state: `register.v1` and `register-replacement.v1` (rows 27 and 28), `place.v1` (row 1), `location.v1` (row 13), `bound.v1` and `install.v1` (rows 6, 10 and 36), `set-up.v1` and `retire.v1` (rows 3, 9, 11 and 33), `retire-hook.v1` (rows 3 and 9), `close.v1` (D6), `unregister.v1` and `stop.v1` (D11), `reveal-secret.v1` (D12) and `secret-reveal.v1` (D10).
- log-core, 6: `retire-with-resources.v1` now names `runtime` among the relations of the order, as LIF-021 does; its description had left it out.
- log-core, 7: The record named the PKG-020 example `PKG-020-plain-provider.yaml`. The rule is on manifests, and the checker reads a file under `examples/invalid-by-rule/` as a manifest only where its name holds `-manifest-`, so it is `PKG-020-manifest-plain-runtime-provider.yaml`.
- log-core, 8: The rules for PKG-020 and SEC-022 are the tool's and not the schemas'. The profile could state each with `if` and `contains`; `type-spec.v1` and `implementation-item.v1` keep out the names of the core package's capabilities and operations, as they do for PRV-001.
- log-core, 9: To show two selected items that name one label with different values, the valid example's `legacy-reader` in `files-package/conduit.yaml` was given `placement: {kubernetes.io/os: windows}`. No valid template selects it, so no valid example breaks.
- log-core, 10: The MAN-017 rule takes both rows 22 and 24, though the record marked row 22's check optional and named no check for row 24. It follows the branches of `anyOf` and `oneOf` at the place, not `allOf` and not a `$ref`, and reads `type`, `enum` and `const` alone, as MAN-017 does. It matches `isRequired` through the server.json input that the binding's key names: a variable or a header by `name`, an argument by `name` or `valueHint`. A `command` item declares no input, so it is not checked for null.
- log-core, 11: No tool rule was added for row 27's `default` of an input whose destination is an entry's `runtime` (PLC-005), nor for row 35's keys of `arguments` (MAN-010): only the comments say that the engine checks them.
- log-core, 12: `spec/core/README.md`'s list of what the package does not settle no longer has the keys of `secrets`, which rows 22 and 23 settle, and has instead what the engine puts in `identity`, which DL13 states.

### log-docs

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-docs.md). 5 choices.

- log-docs, 1: **Rows for `serving` and `unreconciled`.** Both were described inside other rows (Conduit's keys, record, status, inspect). A row of their own says what each is in one place, as the rows for `supports`, `fetch` and `deduplicates` do for other keys.
- log-docs, 2: **A row for the list set on the engine's own runner.** The promises name it by a phrase and not a coined word, so the row takes the phrase, and coins nothing.
- log-docs, 3: **"Fail over" among the words the promises do not use.** The ruling's record and D12 use it; the integration replaced it in ACC-019 and IFC-041.
- log-docs, 4: **The schema profile's server.json and RFC 8785 rows.** Rows 22 and 35 adopt server.json's `valueHint` and RFC 8785's form of a number, which the profile, as the list of standards relied on, now names.
- log-docs, 5: **The overview's new sentences** follow the promises in ordinary words and bind nothing.

### log-editorial

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-editorial.md). 12 choices.

- log-editorial, 1: **IFC-038** says "with `revision` at `v0` in its settings object", as IFC-037 does; E4 names only IFC-004 and IFC-037, and IFC-038 had the same words.
- log-editorial, 2: **PRV-022** says "a provider call to `run.v1`", as E9 has PRV-011 say. Ruling 42's answer D17 wrote it after the audit, in the words that E9 corrects.
- log-editorial, 3: **SEC-022 and PRV-001** say secret Resource where they said store for one, as E18 has SEC-010, SEC-017 and SEC-020 say. Ruling 42's answer D10 and its decisions wrote them after the audit.
- log-editorial, 4: **TPL-008** keeps "What a setup does is TPL-012, and what a retirement does is TPL-009", the cross-references at the end of the sentence that E11 removes.
- log-editorial, 5: **IFC-008's observable** says "call" where E5 says "request", since E7 makes its `when` a call, which a provider call also is.
- log-editorial, 6: **The rows set up and retire** take in what the removed row setup operation, retire operation said: the target ACC-033 names, the input `instance`, and that the engine implements both (TPL-008).
- log-editorial, 7: **The row connection** says that the connection to a `remote` item is what the promises mean by a connection with no more said; **the row call** cites IFC-001, IFC-006 and PRV-005; **the row ready** cites LIF-010, LIF-011 and PRV-009.
- log-editorial, 8: **The description of `node.v1`** begins "The type of every node", not "The type `node.v1`".
- log-editorial, 9: **The checker's docstring of `bundle`** follows SCH-015's new word.
- log-editorial, 10: **The example for SEC-022** is renamed, as [the other renames](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-rename-b.md#what-was-kept) left it for E17.
- log-editorial, 11: **E22's destination** is this log, not ruling 41's docs log (see E22 above).
- log-editorial, 12: **`spec/README.md` and the overview** now say that the editorial fixes are applied, where the renames had them say that they were not yet.

### log-integration

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-integration.md). 5 choices.

- log-integration, 1: **PRV-021 in D11's words.** The groups wrote an obligation on every runtime provider to carry out a stop it cannot carry out now. This pass put back the record's words: the contract states what the provider does with it, carrying it out being the example. The stronger form is the other option: every runtime provider must stop such a run once it can, as the kubelet and Nomad's clients do. It would close the case of a run left on a machine that comes back whatever provider the user picks, and is a candidate for the research on work across machines.
- log-integration, 2: **PRV-015, a selector that names one label twice.** The provider is not asked. The outcome is the one PLC-007 already gives, that the instance is not instantiated; the other reading, to ask `place.v1` with the engine's label alone, would let a provider name the engine's machine for an instance whose type asks for another.
- log-integration, 3: **DL13 drops its stdio sentence.** What the engine's own runner gives a process to present is now what the mechanism puts in `identity`, as for any run. An engine whose mechanism on one machine needs no cryptographic proof still states its choice there, since IFC-022 asks for one only where a request crosses machines.
- log-integration, 4: **Definitions and `spec/README.md`.** This pass wrote the rows and the limits that the table and the answers name for these two files, in the words of the promises. The limit for D11 is stated as a limit, because under D11's words it rests on each provider's contract.
- log-integration, 5: **F3, a retire hook to a `remote` item after the engine restarts.** Options: (a) TPL-018 counts a connection whose values were lost as one that does not last, so the retirement connects anew with reads that belong to it, as row 9 has it for a retirement with no connection; (b) ACC-039 also permits SEC-023's read before a retire hook, and TPL-009 names it. Recommendation: (a), which keeps every read of a retirement its own. Fixed by option (a). Option (b) would widen what ACC-039 permits through the Conduit of a retired consumer. The coordinator or the owner may strike this for (b)

### log-language

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-language.md). 12 choices.

- log-language, 1: DL16 keeps the user and the login session that row 7 put in the item. The record's text for the fixed set (option b of D8) said the item would keep "only the working directory and what other operating systems use instead". The owner chose option c, which rules on variables only, so the rest of row 7 stands. DL16 also says how the list of further variables is set on the runner: no Resource stands for the engine's own runner (PRV-003), so the list is the engine's own configuration.
- log-language, 2: IFC-022 and DL13 give `identity` to every run, the engine's own runner included. The record of D17 speaks of a run on another machine; PRV-007 and PRV-022, as the providers group wrote them, give `identity` to every run and have the engine's own runner deliver it in the same way, and these files follow them.
- log-language, 3: IFC-030 gained a sentence saying that a provider call is not a request, so the credential in `identity` does not break its rule that nothing a consumer presents reaches an implementation. No record entry asked for it.
- log-language, 4: IFC-020 keeps the record entry's note that the protocol's extensions are disabled by default and a client enables one only where its developer opts in. Its source is the extensions overview of the Model Context Protocol: "Extensions are always disabled by default and require explicit opt-in from the developer, at every stage."
- log-language, 5: IFC-036 names `data.requiredCapabilities`, from the record entry. Its source is the base protocol page of revision 2026-07-28: the server "MUST return a `MissingRequiredClientCapabilityError` (`-32021`) whose `data.requiredCapabilities` lists the missing capabilities". The sentence on the tasks extension comes from its rule that a server acts on the extension only as each request declares it, "regardless of prior declarations".
- log-language, 6: IFC-038's observable adds that a consumer learns from the discovery result at which revision to declare the extension. IFC-038's `capabilities.extensions` follows the protocol's page on extension negotiation.
- log-language, 7: PKG-020's rejections now say "when the package is installed or the project is submitted" for the whole sentence, not only for the new rule, as the record entry has it for the new rule: a submitted project's manifest is read under the same rules.
- log-language, 8: PKG-029's observable leaves out two phrases of the record entry: "like a record", since a record is history and does not come to name a later package, and "as decision 11 intends", since no promise cites a decision of the review.
- log-language, 9: SUB-008's observable no longer cites MAN-016 for "no package can be installed at its address": MAN-016 is the other order, and is now cited for it.
- log-language, 10: CTR-012 and IFC-037 keep the precedents of the record (the protocol's `outputSchema`, which may be any schema; the kubelet and the v1 CRI API) in their observables.
- log-language, 11: PKG-023's list of causes for a rejected installation was left as it is: it says "such as", and it names neither SEC-005 nor PRV-001, so adding SEC-022 alone would not make it complete.
- log-language, 12: Text that did not change keeps its lines. Changed text is wrapped at 96 columns, and returns to the earlier lines where the rest of a field is unchanged.

### log-machines

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-machines.md). 15 choices.

- log-machines, 1: PLC-007's `when` now defines the selector, the union of the type's, the selected items' and, for an instance on the engine's own runner, the hostname label. Before, it held only where a type or an item states `placement`. The diagnostic names "the source of the label": the type, the item or the engine's own runner.
- log-machines, 2: PLC-007 says that where the type or an item names `kubernetes.io/hostname` with another value for an instance on the engine's own runner, no location matches. This follows from "hold each label with the value it gives". See [open item 6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-machines.md#needs-a-decision).
- log-machines, 3: PLC-023's `then` says that nothing is retired because a machine is unreachable or omitted, matching its new `when`. PRV-007 and PRV-008 also say "unreachable or omitted".
- log-machines, 4: PLC-017, PLC-027, PRV-005, PRV-008 and PRV-020 cite LIF-036 for D11, in the wording of LIF-036 as the lifecycle group wrote it: the stop is asked before the instance is removed, within the invocation of `unregister.v1`.
- log-machines, 5: PRV-007 keeps row 17's guard in its `then`, as an exception to its one obligation, since the record applies the rows of PRV-007 as one change. Its observable explains the guard: a program at a relative path is a file on the machine the engine runs on.
- log-machines, 6: PRV-007 and PRV-011 say "names" for the keys of `arguments`, `environment` and `secrets`, as the description of `run.v1` that the core group wrote does. Row 35 keys an argument by its `name` or its `valueHint`.
- log-machines, 7: PRV-009: the record's entry asked to say that "the condition always carries the time of the answer". That was not written, since OBS-003 gives `Unknown` with no time where nothing was reported; the time of the answer is already in PRV-009's `then`. Only the pointer to DL5 was added.
- log-machines, 8: PRV-006 gives the reason for rejecting a retired runtime provider as "the shutdown of an instance that named it could never be confirmed, under LIF-024". The record's entry said that LIF-017 would never remove the instance, but LIF-017 does remove one on a location declared lost.
- log-machines, 9: PRV-013: the record's entry said that the locations left behind when a provider reports another form of a hostname are cleared "only by a declaration of loss". The text says that an actor retires one once no instance is placed on it (PLC-011), or the root Interface declares it lost (PLC-017).
- log-machines, 10: PRV-015: "A provider of one machine, and the engine's own machine, place every instance on that machine" became "name that machine or none". Under row 1, a provider of one machine other than the engine's names none for an instance on the engine's own runner.
- log-machines, 11: PRV-018 points to `identity` (D17). The record did not list PRV-018.
- log-machines, 12: PRV-021's `when` reads "a run that it cannot stop now, as where it cannot reach the machine that the run is on", and its observable says that the stop then ends unknown, as `stop.v1`'s description has it where the provider cannot tell whether the run stopped. The precedents are the kubelet and Nomad's `disconnect`, from the analysis of D11.
- log-machines, 13: PRV-022 says that a file is delivered "at that path, as the run sees it", matching `run.v1`'s "under the path at which the run finds each". Its `then` says that the provider "may" replace a file in place, as the ruling says. "No record holds them" is in the observable; the rule for the record is in `run.v1`'s description, written by the core group: "The record of the call holds no value of `secrets` and nothing of `identity`".
- log-machines, 14: PRV-023 says "a list set by the user on the engine's own runner", and cites DL16 for how it is set, for an operating system without the six variables, and for the working directory, user and login session. DL16 is the implementation-defined list's group's new item.
- log-machines, 15: Coined name, for the owner to strike: the input `identity` of `run.v1`, an object with `environment` (names to strings) and `files` (paths to contents). The coordinator gave the name in this stage's work order; PRV-007 and PRV-022 use it, as does `run.v1`.

### log-rename-a

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-rename-a.md). 8 choices.

- log-rename-a, 1: **"The engine's own machine" for the computer.** Where the phrase meant the machine the engine runs on, not the built-in provider, it became "the machine the engine runs on", which N2 keeps for the computer: in `place.v1`'s description, in the section on providers of the core README and twice in `docs/OVERVIEW.md`. Renaming it "the engine's own machine provider" there would have been wrong.
- log-rename-a, 2: **"That machine" in N3.** The subsection says "the machine that `location` names" becomes "that machine". In PRV-019's `then` and in `run.v1`'s effect there is no machine before it to point back to, so they say "the machine given in `machine`"; in PRV-007's observable, which is read apart from its `then`, "the machine of the instance's node".
- log-rename-a, 3: **The description of `labels.v1`.** Written from the subsection's recommendation, without the empty-map sentence (see N8).
- log-rename-a, 4: **A Kubernetes Node.** Five sentences that name Kubernetes' node now write Node, and PLC-025 writes `scheduling.nodeSelector` (see N10). Without it "node" in those sentences would read as Conduit's.
- log-rename-a, 5: **The example project's input.** The Pool template `workspace` in `valid/project/` had an input named `location`, which supplies its entries' placement, and the instance `team` gave it. Both are now `node`, so that the example uses the name in force.
- log-rename-a, 6: **The checker.** The rule name `location-type` became `node-type`, and its variable `location` became `node_type`. The rule's promise, PLC-028, did not change.
- log-rename-a, 7: **The words-not-used rows** (see above) are this editor's wording.
- log-rename-a, 8: **The node row** says that a node "is not a YAML node", as the subsection asks, and that Kubernetes' policies differ, in two cases taken from PLC-023 and PLC-026.

### log-rename-b

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-rename-b.md). 13 choices.

- log-rename-b, 1: **`secrets.packageArguments`.** `run.v1`'s `secrets` holds the values read for the run under `environment` and `arguments`, the names of its two other inputs. N6 renames the input; the member of `secrets` follows it, so that a value in `secrets` stands under the name of the input it would otherwise be in. PRV-007 and the row run say so.
- log-rename-b, 2: **The digest of an `oci` image.** The rule is written as a pattern on `identifier`, `@sha256:[a-f0-9]{64}$`: PKG-004's form of a digest, after the `@` that OCI references use, at the end of the identifier, as go-containerregistry reads the registry's examples. An `oci` Package that also states `fileSha256` is not rejected; nothing gives that hash a meaning.
- log-rename-b, 3: **Where the rule stands.** It is in `implementation-item.v1`, under `dependentSchemas`, and not in `server-package.v1`, which stays a transcription of server.json.
- log-rename-b, 4: **No promise says who checks the hash.** None said who checked the item's `digest` either. MAN-010 says that the item states the hash once, as server.json writes it; server.json has the client that downloads a package check it.
- log-rename-b, 5: **Two new examples** under `examples/invalid/`, one for each way of pinning (see N7).
- log-rename-b, 6: **"No key binds `runtimeArguments`"** is said in MAN-010's observable, the description of `implementation-item.v1`, the profile and the definitions, where the subsection asks only to leave them unbound, so that a reader of the item does not look for them.
- log-rename-b, 7: **"With its dashes"** in MAN-010, which the subsection's wording asks for, for a named argument such as `--root`.
- log-rename-b, 8: **N12 beyond the records.** "The reader's own Conduit" in the core README's export table became "the instance's own Conduit", as "reading instance" did. "The reading Conduit" in SEC-013 stays: it names a Conduit, not an instance.
- log-rename-b, 9: **The condition's `source`.** PRV-009 says "a `source` that names the runtime provider", where it said "the runtime provider as its `origin`", since a `source` is now a URI-reference that the engine names.
- log-rename-b, 10: **The row source** says that it is not `package-source.v1`; **the row domain of trust** that it is not SPIFFE's trust domain; **the row conformance document** that no engine reads it; **the row idempotency key** that the IETF draft defines one so. The first three follow the subsections; the last cites the draft that N15's subsection cites.
- log-rename-b, 11: **PRV-010** cites Kubernetes' `restartPolicy` beside systemd and Compose, from N20's precedents.
- log-rename-b, 12: **The names check** (see above): one pattern of `tools/set_names.py`, a file outside the scope this pass was given, changed so that its check could read the ruled name.
- log-rename-b, 13: **The overview** says that a package is pinned by the SHA-256 of its file in `fileSha256`, or an OCI image by its digest, where it said "pinned by its digest".

### log-secrets

Source: [C0](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-secrets.md). 16 choices.

- log-secrets, 1: SEC-022 rejects such a Type when its project is submitted too, as SEC-005 and PRV-001 do. The ruling names only installing its package.
- log-secrets, 2: SEC-022's observable says that the type may still require a setup hook or a retire hook, which is an operation it requires and not a capability it offers. This is from the text of the option ("Hooks still reach the store's implementation"), which the ruling's summary leaves out.
- log-secrets, 3: SEC-023 reads again when the engine is about to send an HTTP request on the connection, not at the restart. The ruling does not say when. This reads nothing for an instance that no one uses, and decides each read as things stand when it is made. IFC-041, as the interface group wrote it, agrees.
- log-secrets, 4: Where the read of SEC-023 does not succeed, the request that was to be sent reports unavailable and the connection lasts without values, so the next request reads again, and each read is recorded (SEC-012, SEC-023).
- log-secrets, 5: SEC-023 covers a connection that a setup made, as the ruling says. Its observable argues that no other lasts across a restart: a connection made within a retirement ends with it, and an invocation in progress at a restart has the outcome unknown (IMP-007).
- log-secrets, 6: The record of a read under SEC-023 names the setup that made the connection as the invocation it serves, although that setup may have ended. REC-002, as the records group wrote it, agrees.
- log-secrets, 7: SEC-012's `then` adds that the engine sends no HTTP request on a connection for which a read failed, so that it covers SEC-023.
- log-secrets, 8: SEC-017's observable adds that nothing the engine keeps across a restart holds a value read.
- log-secrets, 9: SEC-014 takes row 8's cost line in its own words: retiring the instance that a new one replaces closes the Conduits of which it is an end (CON-008).
- log-secrets, 10: CON-008's observable now says that the engine can read through a retired consumer's Conduit for a connection that its retirement makes, as well as for a run. Row 9 lists no edit to CON-008; this keeps it in step with ACC-039 and TPL-018.
- log-secrets, 11: CON-008 names `declare-lost.v1` where row 18 says "the declaration of loss".
- log-secrets, 12: TPL-009 states the cancelled retirement in its `then`, where the entry put it in the observable, because it says what the retirement does.
- log-secrets, 13: TPL-009 says a hook that did not succeed "keeps the shutdown from being confirmed", not that it leaves it pending, because OPS-022 can make the retirement unknown; and that a further retirement dispatches it again "as OPS-021 and OPS-028 allow", not under OPS-024 alone, because a hook of unknown outcome is OPS-021's.
- log-secrets, 14: TPL-018 says a hook dispatched again "may" run against a new run: where an earlier stop failed, the run may still be reported running, and PRV-007 then asks for none.
- log-secrets, 15: SEC-023's `when` says "as after the engine restarts, on its machine or on another" for the ruling's "restarts or fails over"; failover is not a term of the definitions.
- log-secrets, 16: The head comment of `secrets.yaml` keeps its summary of what `spec/README.md` left open at ruling 41, marked as of that time, and its line for ruling 42 says what changed in that list without quoting `spec/README.md`, which another group edits.
