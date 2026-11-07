# Ruling 42: log of the editorial fixes

Date: 2026-11-06. This is the account of the editor who applied the 23 editorial fixes, E1 to
E23, of the [vocabulary audit](../vocabulary-audit/README.md#editorial-fixes), which
[ruling 42](README.md) gave the coordinator to apply. They follow the names N1 to N20, which the
[first rename](log-rename-a.md) and the [other renames](log-rename-b.md) applied, so each fix
is written in the names now in force: E10's location type is the node type, E13's "stands for"
is the word that N10 replaced with represent, E17's `reveal-secret.v1` is `get-secret.v1`, and
E3 and E8 speak of an idempotency key and of filters. The fixes are the audit's. The wording
that carries them out is the editor's, and each choice listed under
[Choices beyond the record](#choices-beyond-the-record) stands unless the coordinator or the
owner strikes it. Nothing was committed.

Not applied here: D5 and D20, which the owner did not rule. No fix touches the gap that
`spec/README.md` states for D5, or the guard of row 17 for D20 in PRV-007, MAN-010 and
`run.v1`.

Scope: the files under `spec/`, `docs/OVERVIEW.md` and `tools/check_schemas.py`, and this log.
No other file was changed. Each changed YAML file has a line at the end of its head comment
naming ruling 42's editorial fixes, this log and what changed; the Markdown files say it in
their head paragraphs (`spec/README.md`, `spec/definitions.md`, `spec/core/README.md`,
`spec/schemas/README.md`, `docs/OVERVIEW.md`), and the checker in the comment above `RULES`.

How it was done: the tree was copied aside first. Each fix was made by an exact replacement of
the text it names, after every occurrence of the words it concerns had been found by a search
across line breaks and read where it stands. A changed paragraph of a promise was rewrapped
from the changed line, at 96 characters. The diff against the copy was then read whole.

## Checks

Run from the repository root after the fixes.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | 439 promises ok |
| Every promise has exactly the seven fields, its text fields folded with `>-` | Yes |
| A YAML line over 96 characters under `spec/` | None |
| `python3 tools/check_docs.py` | 0 errors, with this log written |
| `python3 tools/set_names.py --check` | 0 errors |
| `python3 tools/check_schemas.py` | 0 errors. The rule `secret-only` (SEC-022) rejects its example under its new name |
| `git diff --check` | Clean |

No promise was added, removed or moved, and no ID changed. 20 promises in 8 files changed:

| File | Promises changed | Fixes |
| --- | --- | --- |
| `interface/implementation.yaml` | 6: IFC-004, IFC-006, IFC-008, IFC-019, IFC-037, IFC-038 | E4, E5, E7 |
| `promises/secrets.yaml` | 5: SEC-001, SEC-010, SEC-017, SEC-020, SEC-022 | E18, E21 |
| `promises/providers.yaml` | 3: PRV-001, PRV-011, PRV-022 | E9, E18 |
| `promises/templates.yaml` | 2: TPL-008, TPL-017 | E8, E11 |
| `promises/contracts.yaml` | 1: CTR-002 | E3 |
| `promises/schemas.yaml` | 1: SCH-015 | E12 |
| `conventions/records.yaml` | 1: REC-001 | E19 |
| `promises/conduits.yaml` | 1: CON-007 | E19 |

Also changed: `spec/definitions.md` (E2 to E4, E6, E10, E11, E14 to E17, E19, E20, E22); the
core package's README (E1, E13, E17, E23), `templates.yaml` (E23), `types.yaml` (E10, E17),
`capabilities.yaml` and `operations.yaml` (E17); `spec/schemas/README.md` and the example
project's `conduit.yaml` and `contracts.yaml` (E17); the example for SEC-022, renamed (E17);
the checker (E12, E17); `spec/README.md`, which now says that the fixes are applied; and the
overview (E20).

## The fixes

| # | What was done | Note |
| --- | --- | --- |
| E1 | The Names table of the core README has the row "Hook \| Named for the invocation it belongs to, then `-hook` \| `setup-hook.v1`, `retire-hook.v1`", after the row Operation | No Kubernetes precedent is cited, as the fix says |
| E2 | The row reconcile ends: "It is not a Kubernetes controller's reconcile: nothing is driven toward what was declared (OBS-004)." | As written |
| E3 | The row `deduplicates` and CTR-002's observable say that it is not the protocol's `idempotentHint`, which hints that calling the tool again with the same arguments has no further effect; the engine passes that hint into the tool's `annotations` (IFC-033) and never decides a new attempt by it (OPS-012) | The why's "operation identity" is the idempotency key since N15 |
| E4 | IFC-004: "with `revision` in the extension's settings object"; IFC-037 and IFC-038: "with `revision` at `v0` in its settings object". The row Conduit's extension says that the settings object, and the name `revision`, are the protocol's; the row revision says "`revision` in the settings object of Conduit's extension" | IFC-004 speaks of any extension, so it says "the extension's settings object". IFC-038 had the same words as IFC-037 and is changed with it (see the choices). The protocol's [extensions overview](https://modelcontextprotocol.io/extensions/overview) was read again for this: "Each extension specifies the schema of its settings object", and an extension that makes a breaking change "should add or increment a `revision` string in its settings object" |
| E5 | IFC-008's observable: "the record names the revision of Conduit's extension that the call declared" | "Call", where the fix says "request", follows E7 in the same promise |
| E6 | A new row call: "One `tools/call` request of the Model Context Protocol: what a consumer sends the engine, and what the engine sends an implementation for a request, a hook or a provider call (IFC-001, IFC-006, PRV-005)." The row request adds that the protocol calls any JSON-RPC request a request, a call among them, and that in the promises a request is only the asking, which a call carries; the row provider call, that a provider call is carried by a call, which the protocol calls a request | The citations are this editor's |
| E7 | IFC-006: `when` "An engine sends a call for a request or a hook to an implementation that carries Conduit's keys"; `then` "The call declares ...", "when the call is for a setup hook or a retire hook", "Every call that the engine sends for a request or a hook has a caller". IFC-008: `when` "An implementation receives a call that declares ..."; its observable, "before it sends a call". IFC-019's observable: "This holds for the call of a setup hook or a retire hook as for that of any request" | IFC-006 keeps "when the request came through a Conduit": only a request comes through one. IFC-008 now covers a provider call too, which also declares the extension (IFC-037) |
| E8 | TPL-017's observable: "It carries the filters of the invocation of `set-up.v1` that it belongs to" | After N16 the sentence already said filters |
| E9 | PRV-011's `when`: "the `secrets` of a provider call to `run.v1`" | PRV-022, added by ruling 42's answer D17 after the audit, said "in a request for `run.v1`" and is changed the same way (see the choices) |
| E10 | The row node type is removed and listed among the words the promises do not use. `types.yaml` calls `node.v1` "the type of every node", in its description and in its list of promises encoded | After N10 the row was node type, as [the first rename](log-rename-a.md#what-was-kept) left it for this fix. "The type `node.v1`", which the fix proposes, would repeat the name at the head of `node.v1`'s own description, so the description says "The type of every node" |
| E11 | The row setup operation, retire operation is removed and listed among the words the promises do not use. The row set up begins "Carry out the setup of an instance, which is one invocation of `set-up.v1`, requested of the target ACC-033 names with the instance as its input `instance`, and which the engine implements (TPL-008)", and the row retire likewise for a retirement and `retire.v1`, taking in what the removed row said. TPL-008's sentence is removed | The sentence's last clause, "what a setup does is TPL-012, and what a retirement does is TPL-009", is kept as a sentence of its own, since it uses neither term (see the choices) |
| E12 | SCH-015's `then`: "Each of the last two references the schema that is that contract's `spec`" | The docstring of the checker's `bundle`, which paraphrases SCH-015, says the same. The checker's method `stands_for` and its docstring are code and stay |
| E13 | The core README: "One contract for one verb." | As in `operations.yaml` |
| E14 | The row ready adds that it is a lifecycle state, a phase in Kubernetes' terms, and neither the Kubernetes condition `Ready` nor CRI's `RuntimeReady`; that it does not change when the run stops or the machine becomes unreachable, and "not ready" means registered or retired (LIF-010, LIF-011); and that conditions report health (LIF-011, PRV-009) | The citations are this editor's, and each was read against the sentence |
| E15 | The row Resource adds that it is not a resource of the Model Context Protocol, which a result may carry as a resource link or an embedded resource, and that a Resource template is not an MCP resource template | As written |
| E16 | The row connection begins with the protocol's connection of a client to a server, which the engine makes to each implementation item to which it sends requests for an instance, over stdio or Streamable HTTP (IFC-017); the connection to a `remote` item is a part of a setup, and is what the promises mean where they speak of a connection with no more said. The row item that is run: "the engine reaches it at its Remote's URL (IFC-041)" | The second clause of the row connection is this editor's: every promise that says "connection" alone means the one to a `remote` item, as a search showed |
| E17 | "Secret store" became "secret Resource", "the type of a secret Resource" or "an implementation of `get-secret.v1`": twice in the core README's table of files, once in its section on providers; three times in `capabilities.yaml` (the list of promises encoded, a comment and the description of `secret-provider.v1`); twice in `operations.yaml` (its head and the heading of its section); once in `types.yaml`; once in the schemas README's description of the example project; and once each in the example project's `conduit.yaml` and `contracts.yaml`. The row secret store says "Not used." with the three words. The checker's message for SEC-022 says "the type of a secret Resource", and its example, which [the other renames](log-rename-b.md#what-was-kept) left for this fix, is renamed `SEC-022-type-of-a-secret-resource-that-offers-another-capability.yaml` | 12 of the audit's 13 changed. Kept: the core README's paragraph on ruling 41, which records that pass ("the operation of a secret store"). The example file was untracked, so it was renamed without `git mv`; the logs that name its old name (log-core.md, log-rename-b.md) are records and keep it |
| E18 | SEC-010's observable: "the item of the secret Resource that reads that store points to it", "read for that Resource's run through the Resource's own open Conduit", "A chain of such credentials ends at a secret Resource whose item points to no secret, such as one that reads a file or a keychain". SEC-017's: "a secret Resource whose implementation keeps no copy", "Whether a secret Resource's implementation reads its store ... is that implementation's". SEC-020's: "each is reached by a package's implementation of the same contract, one that reads a file among them", and "the user's login session that a secret Resource's implementation needs" | "A store's own credential", "changed in the store" and "a store that a secret Resource locates" stay: there store is the system outside the engine, as the row store has it. SEC-022 ("the store's implementation", twice) and PRV-001 ("a plain store"), written by ruling 42 after the audit, had the same fault and are changed the same way (see the choices) |
| E19 | The row record: "a request, a provider call or another occurrence"; REC-001's observable: "Each such occurrence"; CON-007's: "after it" | IMP-002 ("any of those events"), OPS-018 ("every ordering of those events") and the core README ("which event closed a Conduit") were read and kept: the fix names three places, and in these the word is the plain one, with no record called an event |
| E20 | The rows Agent ("A turn of its model is model work") and model work ("A turn of an Agent's model, started or continued"), and the families table of the overview ("A turn of its model is called model work") | As written |
| E21 | SEC-001's observable: "This rule is narrower than OBJ-021" | As written |
| E22 | The history paragraph of the head of `spec/definitions.md` (51 lines, on the rows as they stood before ruling 41) is replaced by one sentence with a link, and the paragraph moves to [the end of this log](#the-history-of-the-rows-before-ruling-41) | The fix moves it to ruling 41's docs log, which this editor may not edit: no record under `docs/proposals/` but this log is this job's. So it moves here, verbatim but for its links, which are rewritten for this directory. Ruling 41's docs log can take it, and the link in definitions.md follow, if the coordinator wants it there |
| E23 | The Names table of the core README: "except `node`, which FST-005 names"; NAM-020 is dropped from the template `node`'s row of the export table and from its line in `templates.yaml`'s head | After N10 the template was `node`, as [the first rename](log-rename-a.md#what-was-kept) left it for this fix |

No fix failed to apply after the renames or the decisions. Three changed with the names, as the
record foresaw (E10 and E13 with N10, E17 with N1), and E3 and E8 read in the names of N15 and
N16. Three also reach promises that the audit did not name and that had the same words: E4
in IFC-038, and E9 and E18 in PRV-022, SEC-022 and PRV-001, which ruling 42's decisions and
answers wrote after the audit. E22 moves its paragraph to this log in place of ruling 41's
docs log.

### The words the promises do not use

`spec/definitions.md` lists two more rows: node type (E10), and setup operation, retire
operation (E11), each with what the promises say instead. The row secret store was already
there and now says "Not used." (E17).

## Choices beyond the record

1. **IFC-038** says "with `revision` at `v0` in its settings object", as IFC-037 does; E4 names
   only IFC-004 and IFC-037, and IFC-038 had the same words.
2. **PRV-022** says "a provider call to `run.v1`", as E9 has PRV-011 say. Ruling 42's answer
   D17 wrote it after the audit, in the words that E9 corrects.
3. **SEC-022 and PRV-001** say secret Resource where they said store for one, as E18 has
   SEC-010, SEC-017 and SEC-020 say. Ruling 42's answer D10 and its decisions wrote them after
   the audit.
4. **TPL-008** keeps "What a setup does is TPL-012, and what a retirement does is TPL-009", the
   cross-references at the end of the sentence that E11 removes.
5. **IFC-008's observable** says "call" where E5 says "request", since E7 makes its `when` a
   call, which a provider call also is.
6. **The rows set up and retire** take in what the removed row setup operation, retire
   operation said: the target ACC-033 names, the input `instance`, and that the engine
   implements both (TPL-008).
7. **The row connection** says that the connection to a `remote` item is what the promises mean
   by a connection with no more said; **the row call** cites IFC-001, IFC-006 and PRV-005; **the
   row ready** cites LIF-010, LIF-011 and PRV-009.
8. **The description of `node.v1`** begins "The type of every node", not "The type `node.v1`".
9. **The checker's docstring of `bundle`** follows SCH-015's new word.
10. **The example for SEC-022** is renamed, as [the other renames](log-rename-b.md#what-was-kept)
    left it for E17.
11. **E22's destination** is this log, not ruling 41's docs log (see E22 above).
12. **`spec/README.md` and the overview** now say that the editorial fixes are applied, where
    the renames had them say that they were not yet.

## What was kept

| Where | Words | Why |
| --- | --- | --- |
| `spec/core/README.md`, the paragraph on ruling 41 | "the operation of a secret store" | It records ruling 41's pass, in the words of its time, as the renames kept that paragraph |
| `capabilities.yaml` (`secret-provider.v1`), SEC-005, the core README's section on providers | "each store is its own Type", "each kind of store", "its store's own Type" | A kind of store, the system outside the engine, with its own Type: not the secret Resource that E17 and E18 name. The coordinator may want "each kind of store has its own Type" |
| SEC-022, `capabilities.yaml`, the example for SEC-022 | "a store package that lists or writes secrets" | The owner's answer D10, in its own words: a package for a kind of store |
| Head comments of `secrets.yaml` (ruling 41's lines) | "a file store", "a store's package" | Records of ruling 41's pass |
| IMP-002, OPS-018, the core README's list of what is not fixed | "event" | The plain word; E19 names three places, in which a record's subject was called an event |
| OPS-008: "whether or not its request carries an idempotency key" | request | An invocation's request, which is a request, and not a hook as in E8 |
| `tools/check_schemas.py`, the method `stands_for` and its docstring | stands for | Code, and the sense of `$ref`, not of a node |
| Head comments and head paragraphs that record earlier passes | node type, stand for, secret store, `revision` setting, setup operation | Records of their passes; the lines this pass wrote also name the old words, to say what changed |

## The history of the rows before ruling 41

Moved here from the head of [spec/definitions.md](../../../spec/definitions.md) by E22, as it
stood on 2026-11-06 after the names of ruling 42 were applied. It is a record of the rows as
they stood before ruling 41, and binds nothing. Its links are rewritten for this directory;
its words are those of its time.

The rest of this paragraph is the history of the rows as they stood before ruling 41; the rows
it names for isolation plugin, rename, policies, retrieval, graphs, steps, runtimes, composite
Conduits, constituents, `obliges`, re-exports and `<digest>#<name>` are gone. The history of
the vocabulary is in the [vocabulary record](../vocabulary/README.md); where the
[consolidation rulings](../consolidation/README.md#rulings) or the
[re-audit](../reaudit/README.md) changed a word, this file has the current one.
The names of the core package's exports, and the definitions of Interface, Agent, Resource,
Pool, model work, mediated, enforced, project and published, are those of the
[ruling on the core package and the schemas](../core-and-schemas/APPLY.md), which
is plan-audit ruling 37. The
rows for isolation plugin, cancel and a value tied to a location, and what the rows say of an
input's `default`, of `restoresRuntimes` and `records` and of a constituent, follow the
[decisions of the docs audit](../docs-audit/GAPS.md). The rows for the setup and
retire operations, the resolved form of a reference, replace, rename, result, `supports` and
management verb, and what the rows say of a project, of `from`, of `obliges`, of
a control's `changes`, of the scope of an operation identity and of `<digest>#<name>`, follow the
[decisions of the second round of the re-audit](../reaudit/round-2/DECISIONS.md).
So do what the rows say of which target a reconciliation is authorized against, of a policy's
`actors` and `targets`, and of `optional`, which a runtime no longer carries: those are the
decisions on what the fix pass left. The decisions on what
the closing pass left, and on the check against the Model Context Protocol, are behind what
the rows say of the Conduits of a retired consumer, of a request that names what does not
exist, of `#name` written as `from`, of a requirement that does not narrow requests, of
installing, and of `isError`. The
[decisions of the third round of the re-audit](../reaudit/round-3/DECISIONS.md)
are behind the row for `deduplicates`, and what the rows say of the caller of a retrieval, of a
request that holds the resolved form of a reference, of the requests that `management`
authorizes for a retired Pool, of a
bound as a constituent, of the version a policy keeps, of what a purge keeps, and of a request
whose access decision cannot be made. The decisions on what the fix pass of
that round left are behind what the rows say of a graph and `supports`, of until when an
instance holds a runtime, and of the shutdown of an instance one of whose runtimes has a start
of unknown outcome.
The [decisions of the fourth round](../reaudit/round-4/DECISIONS.md) are behind
the row for chain, and what the rows say of a submitted project, of the
manifest that the digest of an unpublished package hashes, of a management verb, of the
shutdown of an instance that holds a runtime, of what a reconciliation states, of a composite
declared Conduit and of a step that names one of its components, of the Conduit of an
invocation that has several
attempts, of the default of a property, of a selection that names nothing, of the version a
policy keeps for a re-export, of what a purge keeps, of the output `revision`, and of a tool
for an operation that the target does not hold.
The decisions of the docs audit and of the second, third and fourth rounds are the
coordinator's, logged in those records, and ruling 39 accepted them.
[Ruling 38](../ruling-38/README.md), of 2026-11-06, made four decisions of its
own, which are the owner's. They are behind the row for later work, which is now among the
words the promises do not use, with everything the other rows
said of it and the row for the tool `start-later-work.v1` gone; and behind what the rows say
of the warning that applying a policy gives, of a directory, which installs nothing when it is
handed over, and of the step of a composite declared Conduit, which makes one request for
`open.v1` for each component.
