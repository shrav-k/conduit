# Ruling 42: the questions after the rewrite, and the names

Status: **ruled by the owner on 2026-11-06** (plan-audit ruling 42). This is the record of a
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

The owner struck none of the 43 decisions of the coordinator. The 23 editorial fixes of the
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

The ruling was applied to `spec/` on 2026-11-06. The editors' logs are beside this record:
[machines](log-machines.md), [secrets](log-secrets.md), [conventions](log-conventions.md),
[language](log-language.md), [core](log-core.md), [docs](log-docs.md), the renames
([nodes](log-rename-a.md), [the rest](log-rename-b.md)), the [editorial fixes](log-editorial.md)
and the [integration](log-integration.md), whose section "Verification" answers 34 findings of
three verifiers. The specification then held 440 promises.

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
