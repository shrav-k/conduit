# Ruling 41: log of the docs editor

Date: 2026-11-06. This is the account of how the docs editor applied section 5, "Docs", of the
[work order](PLAN.md#5-docs) for [ruling 40](../ruling-40/README.md) and
[ruling 41](README.md) to [spec/definitions.md](../../../spec/definitions.md),
[spec/README.md](../../../spec/README.md) and [docs/OVERVIEW.md](../../OVERVIEW.md), after the
seven editing groups, the [integration](log-integration.md) of the promise files and the
[core-and-schemas pass](log-core.md). No other file was changed. The rulings are the owner's;
the wording is the docs editor's, and each choice below stands unless the coordinator or the
owner strikes it. Names are those of the review and the work order, such as `secret-reveal.v1`
and `reveal-secret.v1`: the vocabulary audit that ruling 41 orders proposes others before pull
request 7 is finished.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/check_schemas.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0`. The one error before this pass, the removed key `withheld` written with its prefix in `spec/definitions.md`, is gone |
| `git diff --check` | Clean |
| Every promise ID cited in the three files exists | Yes, apart from the GRA IDs that `spec/README.md` names on purpose, under "Promise files that rulings removed" |
| Every item of the implementation-defined list cited exists | Yes: DL14 is cited nowhere any longer |
| The two documents that `docs/OVERVIEW.md` quotes match the example files | Yes, byte for byte, with the head comment of `directory-read` as it now stands |

## spec/definitions.md

The head paragraph now says that the owner accepted the file as it stood at ruling 39, that
rulings 40 and 41 changed it, and that the terms that came and went are those of section 1 of
the work order; it names the new sections, this log, and the names that are not final. The
history that followed is kept as the record of the rows before ruling 41, with one sentence
that says which of the rows it names are gone.

Sections. "Graphs" is gone. "Providers" and "Secrets" are new. "Placement" is written around
machines and labels. The other sections keep their names and order, so the two anchors that
other files link to (`#objects-and-documents`, `#how-a-request-ends`) still resolve.

Rows: 179 before, 205 after.

| Change | Rows |
| --- | --- |
| Added, from section 1 of the work order | provider, provider capability, runtime provider, run, `runtime`, machine, machines provider, label, label selector, package-source provider, provider call, secret Resource, secret reference, the engine's read of a secret (the work order's "read"), setup hook, retire hook and hook, part, remove, Conduit's extension |
| Added, for terms the promises use that the work order's table does not list | stand for; unreachable, gone; narrow; implementation item; item that is run; connection; the engine's own runner, machine and fetching; user; store; locator; trust domain (of a secret); user information; the contracts of a submitted project (the work order's "unpublished package" row, renamed as it says) |
| Rewritten | Every row that section 1 lists as changed, and every other row that named a removed promise, a removed key or a word that goes: among them object, project, published, name, type, schema, instance, nested instantiation, nested instance, input, argument, standard argument, destination, optional, resolving, resolve, resolved form, diagnostic, register, replace, install, exists, ID, qualified reference, `#name`, reference schema, location, location type, placement, shared-host requirement, entry, depends on, first-start object, `system`, `management`, `access`, root Interface, declared Conduit, close, caller, onward request, chain, the instance that sent a request, permitted, access decision, bound, requirement, applied requirement, filter, restriction, enforced, `supports`, operation, output, management verb, setup operation, effect, request, dispatch, invocation, the engine's ID, operation identity, repeat, attempt, in progress, the invocation's state, result, reconcile, cancel, model work, implementation, plain, Conduit's keys, tool, tool name, handle, hold, retired, set up, retire, shutdown, declare lost, unregister, inspect, status, condition, observation, origin, record, purge, definition, configuration, control, revision, address, manifest, `fetch`, export, core package, selection, digest, runtime profile; and the paragraphs of "How a request ends" |
| Moved to "Words the promises do not use", each with what replaced it | graph, setup graph, retire graph; step; value binding, value reference, `source`, output of a graph; runtime as a template or a type declared it, `runtimes`, `restoresRuntimes`, restore; hold (a handle) and the request for no actor; policy and its two operations; composite, component; constituent, composition; `obliges`, `narrows`, `appliesTo`; re-export; rename; facts, location fact, host, `set-facts.v1`, `location-settings.v1`; default location and its operation, capability and type; requirement on placement; retained result, retrieval tool; withheld, `inProgress`, `interfaceVersion`; isolation plugin; unpublished package, `<digest>#<name>`; secret store |

Where this file differs from the work order's table, and why:

1. **secret store.** The work order defines it as a provider whose type offers
   `secret-reveal.v1`, which is also its definition of secret Resource: two terms for one
   object. The promises use "secret Resource" for the object and "store" for the system
   outside the engine that its locator names (log of G7, C16; integration, PRV-001 and
   PRV-002). So "secret Resource", "store" and "locator" have rows, and "secret store" is
   listed under the words the promises do not use, with what the core package's descriptions
   mean by it.
2. **read.** The promises say "the engine's read of a secret", and "a read" for short (log of
   G3); the row has that name and says so.
3. **label.** NAM-001 uses the word in its DNS sense, one part of a host name, beside the new
   sense of a machine's label. The row gives the new sense and names that one other use, so
   that a reader is not misled; the vocabulary audit may rename one.
4. **depends on.** The work order keeps it as "only an entry on the entries it refers to". The
   row moved from the section on graphs to "Ownership", beside entry, and says also that an
   instance depends on its location through its placement (PLC-015).
5. **output.** The work order's table removes "output (of a graph)" and keeps an operation's
   outputs. The old row mixed the two; the new row keeps the second only, in the `outputSchema`
   form.
6. **resolving.** Its row still says that resolving runs nothing. It now adds that placing an
   instance that is given no placement is not resolving (PLC-002, PRV-015). PLC-007, PLC-013
   and POOL-016 are still of the layer `language` while they read labels that a provider
   reported; that is question R14 of the integration log, which this pass did not settle.
7. **hook and request.** A hook is not a request in the sense of the row for request (three
   kinds), as the work order and the log of G3 have it, and the row says so. IFC-006 still
   speaks of "the request" that is a setup hook or a retire hook, meaning the protocol's
   request: a word for the vocabulary audit.
8. **user** and **trust domain** are new rows for words the promises now use (PRV-001,
   PRV-003, PRV-012, NAM-012; PRV-011).

## spec/README.md

- The paragraph on standing. The owner accepted the specification as it stood at ruling 39.
  The new text says that rulings 40 and 41 then changed it; that at ruling 41 the owner
  accepted every decision of the principles review and the reduced secret type, and then the
  planner's choices with P2 changed; and that the specification as it stands is the rewrite
  that applied them. It does not say that the owner accepted every word of the rewrite, as
  ruling 39 did of the specification before it: the editors' choices beyond the work order
  are, by their logs and the integration log, kept for the coordinator to confirm or strike,
  and the questions R1 to R28 are open. The coordinator can strengthen the sentence once the
  coordinator has confirmed those choices.
- The opening says the secret type is ruled, with its promises in `promises/secrets.yaml`,
  and that the coined names are not final.
- "What this is" gains the paragraph of ruling 40, points 1 and 2, and decision 31: what
  Conduit owns, what is a provider that the user picks, what stays each engine's (records, the
  engine's state, the mechanism of identity) and where isolation is picked. The table of
  layers says that Conduit's keys travel as an extension, and a paragraph after it says that
  the provider contracts are not a fourth layer and where their promises fall. The paragraph on
  what is outside the layers names the provider's contract beside the runtime profile.
- Contents, recounted from the files: objects 24, schemas 21, instantiation 23, templates 14,
  contracts 10, naming 28, packages 21, manifest 16, submissions 10, pools 15, placement 19,
  conduits 22, first-start 5, providers 18, secrets 20, lifecycle 34, access 29, operations 29,
  revisions 14, observation 5, records 6, implicit 5, implementation 38; 426 promises, 196
  `language`, 190 `convention` and 40 `interface`; implementation-defined 14 items; core 102
  exports and one implementation (operations 23, capabilities 7, types 5, requirements 2,
  references 12, results 5, templates 7); schemas 41 in 8 files (document 1, envelope 5,
  templates 7, instances 4, contracts 6, schema 1, manifest 4, values 13); examples `valid` 7
  files in 3 directories, `invalid` 52, `invalid-by-rule` 20. These are the counts the work
  order and the core-and-schemas log give.
- The row of `promises/graphs.yaml` is gone from the table. A new subsection, "Promise files
  that rulings removed", lists the file with the IDs it held and where their content went, as
  the owner's answer on P2 asks; the row for `id` in "The form of a promise" points to it.
- A section "What the guarantee for secrets covers" states the limit of section 4 of the
  review, as a statement and not a promise. The head comment of `promises/secrets.yaml`
  points to it by that title. It says "the machine that the engine runs on" where the review
  says "the engine's host", since "host" is a word that goes.
- "Existing standards first" no longer names graphs, and names the standards the provider
  contracts follow. The reading order adds `providers.yaml` and `secrets.yaml` after
  `first-start.yaml`, as a step of their own, and drops `graphs.yaml`. The Implementation level
  names SEC-006 and SEC-019 among the promises that bind an implementation.

## docs/OVERVIEW.md

Its note under the title stays, and now says it was read against the specification as ruling
41 left it. Rewritten as section 5 of the work order lists: "What Conduit is" gains one
paragraph on connection and access; "Documents, templates and instances" gains `runtime` and
loses rename; "Pools, ownership and placement" is written around machines, labels and the
machines provider; "Conduits and access" loses the retrieval tool, policies, composite
templates and the isolation plugin, and says how access is narrowed and that only `root` opens
a Conduit through which a secret is read; "One lifecycle" has the parts of a setup, the hooks,
the run, removal and a provider's restart in place of graphs, runtimes, restoring and the stop
for no actor; "Operations, outcomes and reasons" has a repeat in progress report conflict,
controls without clamping, a busy instance, cancellation and records as CloudEvents;
"Packages, contracts and the core package" has the three ways of reaching an item, a project's
address, `fetch` and no re-export. Two sections are new: "Providers", with a table of the four
kinds and what serves where the user picks none, and "Secrets", with the five stores of section
4 of the review. The worked example's quoted template now carries the example file's current
head comment, and its bullets describe the shared-host requirement by the hostname label,
decided once. The page grows from 372 lines to about 470, nearly all of it the two new
sections.

## For the coordinator

1. **`spec/promises/graphs.yaml` is still in the tree** (integration, R1). `spec/README.md`
   now lists it as removed, with its IDs, as the owner's answer on P2 asks, so nothing stands
   in the way of deleting it. This pass did not delete it: its task did not say so. Deleting
   it breaks three links that `tools/check_docs.py` checks, all in records:
   `docs/proposals/graphs-and-schemas/README.md` (line 4) and
   `docs/proposals/secrets/README.md` (the rows of GRA-020 and GRA-024). They can point to the
   file's last version at a commit, as ruling 39 did for the files it removed.
2. The standing paragraph of `spec/README.md` is worded as above, and can be made stronger once
   the editors' choices are confirmed.
3. `spec/core/README.md` and the comments of `spec/core/` still say "secret store" for a
   secret Resource's kind of provider. That is consistent with the row under the words the
   promises do not use, and is for the vocabulary audit rather than an error.

## For the vocabulary audit

Beside every name that the work order coined (P1) and the three names of the review, these
words were added to the definitions or are used in more than one sense, and want a look:

- "secret store", "store", "secret Resource" and "locator" (point 1 above).
- "label", in its DNS sense in NAM-001 and its machine sense everywhere else.
- "request", for a request in Conduit's sense and for the protocol's request, as IFC-006 uses
  it of a hook.
- `revision`, the key of Conduit's keys that holds an instance's revision, beside the
  `revision` setting of Conduit's extension (log of G5, item 7). The row for revision names
  both.
- "user", for the person who picks providers.
- "stand for", "claim" (a scheme), "item that is run", "part", "remove", "hook" and
  "provider call".
