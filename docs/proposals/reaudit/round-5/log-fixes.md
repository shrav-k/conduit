# Re-audit, round 5: log of the fix pass

Date: 2026-11-07. The fix pass applied the coordinator's [decisions](DECISIONS.md) on round 5,
which no ruling has accepted and which the owner can strike. Seven passes applied them by
area, each in its own files, and an integration then made the files agree. This log sums up
the seven passes, with a link to each log, and records what the integration changed.

Nothing was committed or staged.

## Result

Counted with a script on the working tree after the integration, against `HEAD`.

| | Before | After |
| --- | --- | --- |
| Promises | 449 | 450 |
| Of the layer `language` | 202 | 202 |
| Of the layer `convention` | 206 | 207 |
| Of the layer `interface` | 41 | 41 |
| Promises in `packages.yaml` | 21 | 22 |
| Items of the implementation-defined list | 16 | 15 |
| Schema contracts under `spec/schemas/` | 41 in 8 files | 41 in 8 files |
| Files under `examples/valid` | 7 in 3 directories | 7 in 3 directories |
| Files under `examples/invalid` | 56 | 56 |
| Files under `examples/invalid-by-rule` | 28 | 30 |
| Rules the checker applies in its step 6 | 24, from 23 promises | 25, from 24 promises |

- Added, 1: PKG-030, of the layer `language` (M25).
- Removed, 1: DL11 of the implementation-defined list (M69). Its ID is not reused.
- Moved between layers, 1: SEC-024, from `language` to `convention` (M05).
- Changed, 91: ACC-019, ACC-033, ACC-037; CON-008, CON-022; CTR-012; IFC-001, IFC-004,
  IFC-007, IFC-012, IFC-013, IFC-015, IFC-016, IFC-017, IFC-019, IFC-022, IFC-025, IFC-026,
  IFC-032, IFC-033, IFC-036, IFC-037, IFC-038, IFC-041, IFC-042, IFC-044; LIF-007, LIF-009,
  LIF-013, LIF-015, LIF-017, LIF-018, LIF-021, LIF-023, LIF-024, LIF-027, LIF-028, LIF-029,
  LIF-033, LIF-035, LIF-036; MAN-010, MAN-017; NAM-009; OBJ-019, OBJ-021, OBJ-024; OPS-007,
  OPS-008, OPS-009, OPS-021, OPS-022, OPS-028, OPS-030, OPS-031, OPS-034; PLC-002, PLC-023;
  PRV-002 to PRV-008, PRV-012, PRV-014, PRV-015, PRV-020, PRV-022, PRV-025 to PRV-028;
  REC-002; REV-007, REV-010, REV-012, REV-014; SEC-001, SEC-006, SEC-010 to SEC-013, SEC-023,
  SEC-024; SUB-008, SUB-010; TPL-018, TPL-019. Each keeps its ID, and none moved file.
- Items of the implementation-defined list changed: DL4, DL10, DL12, DL13, DL16 and DL17.

The decisions give 203 promises of the layer `language` once PKG-030 is in. That figure leaves
out M05, which moves SEC-024 to `convention`, so the recount gives 202, as the log of area F2
foresaw. [spec/README.md](../../../../spec/README.md#contents) gives the recounted figures:
450 promises, 202 `language`, 207 `convention` and 41 `interface`, 22 in `packages.yaml`,
15 items, and 30 files under `examples/invalid-by-rule`.
[INDEPENDENT_BUILDS.md](../../../implementation/INDEPENDENT_BUILDS.md) gives 450 promises, 41
of them in the interface file, which the recount confirms. `README.md` and
[the tracker](../../../AUDIT.md) give no total and link the contents table.

## The seven passes

| Area | Log | Items | What it changed |
| --- | --- | --- | --- |
| F1. Providers, secrets and placement | [log-fix-F1.md](log-fix-F1.md) | M01 to M22, with M23's sentence of SEC-024 and M47's of PRV-022 | `providers.yaml`, `placement.yaml` and `secrets.yaml` |
| F2. Manifests, schemas, templates and submissions | [log-fix-F2.md](log-fix-F2.md) | M23 to M31, with the parts of M03, M06, M17, M41 and M45 in its files | `templates.yaml`, `conduits.yaml`, `objects.yaml`, `naming.yaml`, `packages.yaml` (PKG-030 added), `manifest.yaml`, `submissions.yaml` and `contracts.yaml` |
| F3. Invocations, retirement and reconciliation | [log-fix-F3.md](log-fix-F3.md) | M32 to M44, with the parts of M09, M55, M60 and M69 in its files | `access.yaml`, `lifecycle.yaml`, `operations.yaml`, `records.yaml` and `revisions.yaml` under `spec/conventions/` |
| F4. The implementation interface, the protocol and credentials | [log-fix-F4.md](log-fix-F4.md) | M45 to M77 and M02, with M17's and M90's words in its files; M78 rejected | `spec/interface/implementation.yaml` and `spec/implementation-defined.yaml`, DL11 removed |
| F5. The core package, the examples and the names | [log-fix-F5.md](log-fix-F5.md) | M79 to M86, with the parts of M03, M09, M18, M20, M35, M37 and M41 in its files | `spec/core/`, the schemas README, the example project, `spec/names.yaml` and the two tools |
| F6. The specification's README and definitions | [log-fix-F6.md](log-fix-F6.md) | M87 to M99, with M69, M103, M104 and M107 in its files | `spec/definitions.md`, `spec/README.md` and `docs/OVERVIEW.md` |
| F7. The tracker and the entry documents | [log-fix-F7.md](log-fix-F7.md) | M100 to M111 | `README.md`, `AGENTS.md`, `docs/AUDIT.md`, the indexes, `INDEPENDENT_BUILDS.md`, and a line, a banner or a word in eight records |

Each pass worked on its own files only. The decisions had routed 49 changes to an area that
does not hold their file: 23 to F7, for files of F4, F5 and F6; 15 to F6, for files of F5; 7
to F1, for files of F6 and F2; 3 to F3, for files of F1; and 1 to F4, for a file of F1. Each
pass logged these as not applied. Six of them were made all the same, or replaced: F1 applied
M23's sentence of SEC-024, F4 applied M02's sentence of DL17 with M63's, F6 applied M25's count
in `spec/README.md`, and M47 and M53 replace M90's edits of PRV-022, DL13 and IFC-042, as
[items that change one text](DECISIONS.md#items-that-change-one-text) says. Two others, M02's
parts of the two READMEs, were no separate edit, since M01's text carries them. The
integration made the other 41, below.

## What the integration changed

### Decided changes that no pass could make

| Item | Files | Promises or parts | What changed |
| --- | --- | --- | --- |
| M01, M02 | `spec/README.md` | none | The opening: every server that the engine does not run itself, other than a `plain` server reached by `remote`, stands behind a gate, and a `plain` one takes none of Conduit's keys (IFC-007, IFC-041). "What the guarantee for secrets covers": the bullet on the server of a secret Resource excepts a `plain` implementation of `get-secret.v1` reached by `remote` and says what reaches it; the bullet on SEC-024 covers the value that such an implementation returns and the loopback host on an engine that can move. "There it crosses" became "There each crosses", since the bullet now has two subjects. |
| M01, M02, M23 | `spec/schemas/README.md` | SEC-024 row | First cell as decided, with "goes" and "stands" so that a header does not "stand" at a URL; the tool cell says that the tool checks a header that carries a secret, and drops the host written as a variable. |
| M03 | `spec/implementation-defined.yaml`, `spec/interface/implementation.yaml`, `spec/definitions.md` | DL17, IFC-017, rows run, hook | DL17 and IFC-017 cite PRV-007 alone for one run per instance. The row run says that the run is asked for where the provider reports it `stopped`, and none where it reports `unknown`; its second sentence cites PRV-007 alone, without repeating that condition. The row hook says `stopped`. |
| M03 | `spec/core/operations.yaml`, `spec/core/README.md` | `set-up.v1`, `retire.v1`, `retire-hook.v1` | Not in any list: each description said that the run is asked for where it "is not reported running", which after M03 would ask for one where the provider reports `unknown`. Each now says where the runtime provider reports it `stopped`, as TPL-018 and PRV-007 do. |
| M06 | `spec/definitions.md`, `spec/schemas/README.md` | row item that is run, TPL-019 row | As decided. The README's tool cell says "Yes, for two items": the rule `one-run` was not extended to an Interface or a Pool. |
| M07 | `spec/implementation-defined.yaml` | DL16 | The registry types of a `package` item that the engine's own runner runs, as a runtime provider's contract states them under PRV-012, and that it runs one only over stdio and ends any other failed, under PRV-003. The decided "since" clause is a separate clause. |
| M08, M09 | `spec/definitions.md` | rows provider, request, later work | The provider's server ends when the engine stops or the part that holds it stops (DL17), and a secret Resource's process on the engine's own runner "ends as such a server does", the words F1 used in PRV-002. The two reads of PRV-026 also follow the stop of a part. |
| M09 | `spec/implementation-defined.yaml`, `spec/README.md` | DL17, bullet on lost values, rotation bullet | DL17: a part that stops so counts as a restart for the runs that it held of providers and of secret Resources on the engine's own runner, which end with it and which PRV-026 runs again. The decided "So does every run of the engine's own runner" would have had every run of that runner end unknown, and DECISIONS.md limits M09 to the runs of providers. The rotation bullet's run that PRV-026 makes again also follows the stop of a part. |
| M16 | `spec/README.md` | opening | As decided. |
| M19, M22, M26 | `spec/definitions.md` | rows install, placement, reference schema | As decided. |
| M23 | `spec/schemas/manifest.yaml`, `tools/check_schemas.py` | `server-remote.v1`, `secret_may_go` | As decided. The code does not change. |
| M24 | `spec/schemas/schema.yaml`, `spec/schemas/README.md` | OBJ-019 entry, the row on a schema | As decided. |
| M25 | `spec/schemas/manifest.yaml`, `spec/schemas/README.md`, `tools/check_schemas.py`, `examples/invalid-by-rule/PKG-030-manifest-remote-runtime-provider.yaml` | PKG-030 | The head comment, the README row, "Four rules are on manifests" and the counts of the examples table; the rule `remote-provider`, from the loop of `manifest_rules` over every item; the new example, a hosted runner at an `https` URL that implements the three operations of a runtime provider. |
| M26 | `spec/promises/secrets.yaml` | SEC-001 | As decided. |
| M27 | `spec/promises/providers.yaml` | PRV-007 | As decided. |
| M30 | `tools/check_schemas.py`, `spec/schemas/README.md`, `examples/invalid-by-rule/MAN-017-environment-place-admits-any-value.yaml` | MAN-017 | The function `confined` and its use in `item-place-required`, as decided; the MAN-017 row; the new example, whose type gives `path` the schema `{}`. |
| M32, M35, M42 | `spec/definitions.md` | rows repeat, control, reconcile, attempt | As decided. The row attempt says that a hook whose outcome is unknown is not dispatched again "until it is reconciled", the words of OPS-021 and OPS-028. |
| M41 | `spec/definitions.md` | rows run, retired, remove, unregister | As decided. |
| M91 | `spec/promises/objects.yaml` | OBJ-024 | As decided. |
| M94 | `spec/interface/implementation.yaml` | IFC-037 | As decided. |
| M97 | `spec/interface/implementation.yaml`, `spec/implementation-defined.yaml`, `spec/core/README.md` | IFC-041, DL13 | "The engine's credential ID". The history sentence of the core README that says "the engine's ID" records ruling 43 and keeps its words. |
| M98 | `spec/core/operations.yaml` | `setup-hook.v1`, `retire-hook.v1` | "from its call, under IFC-006". |

### Text that the decisions left behind

These were found by checking each area's changes against the others. Each follows from a
decision; none adds behavior beyond the one choice in the next section.

| Item | Files | Promises or parts | What changed |
| --- | --- | --- | --- |
| M41 | `spec/promises/providers.yaml` | PRV-008, PRV-020, PRV-027 | Each said that a request to unregister an instance "on a node declared lost" asks for the stop once more. Any retired instance can now be unregistered, so each says a retired instance. |
| M41 | `spec/conventions/lifecycle.yaml` | LIF-018 | The instance whose node was declared lost may, "like any retired instance", be removed by a request to unregister it. LIF-018 keeps its statement on the lost-node case, as M41's reason asks. |
| M41 | `spec/promises/placement.yaml`, `spec/conventions/lifecycle.yaml` | PLC-023, LIF-036 | See [the choice of this pass](#the-choice-of-this-pass). |
| M32, M39 | `spec/core/README.md`, `spec/core/operations.yaml`, `spec/implementation-defined.yaml` | section on results, notes on the management verbs, DL10 | Each said that a repeat reports conflict where the engine "no longer keeps" the outputs. Each now also names a filter that the latest attempt was not sent, and DL10 keeps no outputs of an invocation whose outcome a reconciliation set, as OPS-009 says. |
| M34 | `spec/definitions.md` | row `unreconciled` | "each invocation whose outcome is unknown", dropping "not yet reconciled", as LIF-013 now reads: a later attempt that ends unknown is listed again. |
| M36 | `spec/definitions.md` | row shutdown | Confirmed also where a reconciliation has recorded an invocation of `retire.v1` as having occurred, as LIF-024 now reads. |
| M01, M02, M25, M30 | `spec/definitions.md` | row implementation item | The place an item binds is confined to a string, a number, a boolean or null; no item reached by `remote` implements an operation of the three provider capabilities (PKG-030); and SEC-024's rejections include an item selected for `get-secret.v1` and a loopback host on an engine that can move. The log of area F6 named this row as left. |
| M01, M02, M23, M30 | `spec/schema-profile.md` | rows on a `remote` URL and on implementation items | A `url` that holds a variable is rejected and no key binds one; SEC-024 covers an item selected for `get-secret.v1` and a loopback host; a bound place is confined to a string, a number, a boolean or null. |
| M01, M02, M06, M30 | `spec/schemas/templates.yaml` | head comment | What the engine checks, in the same terms as the promises. |
| M01, M02 | `spec/schemas/manifest.yaml` | `implementation-item.v1` | The description says that a `remote` item that carries a secret as the value of `get-secret.v1`, or that stands at a loopback host the engine does not accept, makes the template rejected. |
| none | `spec/README.md` | contents table | `examples/invalid-by-rule` holds 30 files. |

Each changed file has a line in its head comment, or a sentence in its history, that names
this pass and the items: `secrets.yaml`, `providers.yaml`, `placement.yaml`, `objects.yaml`,
`lifecycle.yaml`, `implementation.yaml`, `implementation-defined.yaml`, `core/operations.yaml`,
`core/README.md`, `schemas/README.md`, `schemas/manifest.yaml`, `schemas/schema.yaml`,
`schemas/templates.yaml`, `tools/check_schemas.py`, `definitions.md` and `schema-profile.md`.
The two new examples say what they show in their own head comments.

### The choice of this pass

The log of area F3 found a case that M41 opens and no promise settles. Before M41 only an
instance whose node was declared lost could be unregistered, and such a node represents no
machine, so PLC-023 never held back the stop that LIF-036 asks for. Now a retired instance on
a node that is not lost can be unregistered while the machine provider reports its machine
unreachable or omits it. PLC-023 then held back the stop and reported unavailable, and LIF-036
did not say whether the instance is removed.

This pass settled it under rulings 37 and 38, as following from M41, and the owner can strike
it. PLC-023 no longer holds back the stop that a request to unregister asks for, and LIF-036
says why: the request is the last time the stop is asked for, so it is asked for even while
the machine is reported unreachable or omitted, and what a runtime provider does with a stop
that it cannot carry out now is PRV-021, as for a node declared lost. That keeps what LIF-036
is for, that a run left on a machine that was cut off is asked to stop, and treats both cases
alike.

The options were:

1. As done here: the stop is asked for all the same, and the instance is removed.
2. The stop is not asked for, and the instance is removed all the same, as where the runtime
   provider is not ready. A run left on that machine is then never asked to stop.
3. The request reports unavailable and the instance stays until the machine is reachable, or
   an actor declares its node lost. That keeps a retired instance that the actor asked to
   remove, and declaring the node lost retires every other instance on it.

### Not changed

- `tools/check_schemas.py` checks only SEC-024's branch for a header that carries a secret.
  It does not reject an item selected for `get-secret.v1` at an `http` URL whose host is not
  a loopback address, and the rule `one-run` does not reject an item that is run for an
  Interface or a Pool. The rows of the schemas README say what the tool checks. No decision
  asked for either rule.
- DECISIONS.md keeps its figure of 203 promises of the layer `language`, as a record of what
  was decided. The recount is above.
- The history sentences that record earlier passes keep their words, such as "the engine's
  ID" in the core README's sentence on ruling 43, and the tracker's later amendment of
  ruling 41 that cites TPL-019 beside PRV-007.

## Checks

Run from the repository root on the working tree after the integration:

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
450 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 174 Markdown files and 2362 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x437, apiVersion x257, metaPrefix x104; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
6. Applied 25 rules of this tool, which come from 24 promises, to the core package and 3
example directories. 30 sets of documents in 30 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 53 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```

No line of a YAML file under `spec/` is longer than 96 characters, and every promise and item
of the implementation-defined list that a file under `spec/`, the overview, the tracker or
`INDEPENDENT_BUILDS.md` cites exists, apart from the IDs that the tracker's later amendments
name as removed.

## Verification

Date: 2026-11-07. Two verifiers checked the fixes of this round and the integration, and
reported 29 findings: V1 to V13 from the first, V2-01 to V2-16 from the second. The coordinator
checked each against the text and settled it under rulings 37 and 38, as the decisions of this
round are settled: each fix below follows from a ruling, a record or the wording it cites, and
the owner can strike it. Every finding holds. 28 are fixed; 1, V6, is
[for the owner](#for-the-owner), with options and a recommendation. Nothing was committed or
staged.

Note of 2026-11-07: V6 was settled afterwards by the coordinator, under rulings 37 and 38, as
its option 1, and applied to PLC-007 and PLC-002, as the end of
[for the owner](#for-the-owner) records. All 29 findings of this verification are resolved;
the row for V6 below, and the heading of that section, keep their words as written.

| Finding | Severity | Decision | What changed |
| --- | --- | --- | --- |
| V1 | contradiction | fix | IFC-022's `then`: where only whether the named instance still exists cannot be read, the request establishes neither, and is decided under ACC-037, as IFC-032 says. DL13's "exactly one instance or none" says the same. |
| V2 | contradiction | fix | IFC-046, added: a call that the implementation rejects with -32020, or with the error for a revision it does not support, is sent once more, and the two are one call. IFC-010's `then` and OPS-007's `when` except an error after which the call is sent once more, and IMP-007's observable says that such a call is part of the same dispatch. IFC-001 and IFC-003 cite IFC-046 in place of stating the resend. |
| V3 | misleading | fix | The description of `register.v1` and the core README's section on secrets restate SEC-024 as it now reads: a secret in a header or as the value of `get-secret.v1`, and a loopback host on an engine that can move or run as several parts. |
| V4 | misleading | fix | IFC-016 answers a repeat from the outputs only while every filter that applies to it was sent with the latest attempt, and reports conflict under OPS-034 otherwise. OPS-005, IFC-025, the row "conflict" and the text under "How a request ends" in the definitions name the filter case beside the outputs no longer kept. |
| V5 | misleading | fix | With V2-03: PRV-007 says that no operation settles a run's state, that the part that asks for the run and a retire hook that needs it report unavailable while the provider reports `unknown`, and what an actor can do: read `Running` under PRV-009 and wait, retire the instance and unregister it, or declare its node lost where that node is not `local`. |
| V6 | minor | owner | Holds: PLC-002 places an instance that runs nothing on `local` and checks it under PLC-007, so a type whose `nodeSelector` `local` does not match makes such a template rejected. Neither way follows from the records alone; see [for the owner](#for-the-owner). |
| V7 | minor | fix | PRV-025 drops the condition that a token-checking proxy not pass the token on, and says why a token passed on gives the server nothing: its audience is the instance's ID (DL13), which no other gate, no `remote` item and not the engine's address admits. M51's two proofs to every gate stand. |
| V8 | minor | fix | IFC-036: "where the response carries one". |
| V9 | minor | fix | IFC-045, added, states in its `then` what a call to an implementation declares in `clientCapabilities`, for every call and provider call; IFC-037's `then` no longer states it, and IFC-007, IFC-016 and IFC-037 cite IFC-045. The resend after -32020 is IFC-046's `then` (V2). Reading `tools/list` stays in IFC-001's observable: it is how the engine keeps the protocol's requirement on `x-mcp-header`, which IFC-001's `then` already makes binding. |
| V10 | minor | fix | `spec/schema-profile.md`, `spec/core/README.md` and `spec/schemas/README.md` say that ruling 42 struck none of the coordinator's decisions, as `spec/README.md` does. The head comments of `placement.yaml`, `providers.yaml`, `pools.yaml` and `first-start.yaml` record ruling 42 as it was read then, and keep their words as history. |
| V11 | minor | fix | Items 8 and 10 of the [choices of ruling 43's verification](../../ruling-43/log-integration.md#choices-of-the-verification) have dated lines for M66 and M53. |
| V12 | minor | fix | "Who decides what" in the tracker and the list in `spec/README.md` name this log beside the round's decisions, for the integration's choice on PLC-023 and LIF-036 and the decisions of this verification. |
| V13 | minor | fix | The [re-audit record](../README.md) names round 5, with its counts and links to its decisions and this log. |
| V2-01 | contradiction | fix | CON-006 rejects a shared-host requirement where an end is one for which no item that is run is selected, as it rejects one where an end has no placement: `shared-host.v1` says that the two ends run on one machine, and such an end runs on none. PLC-013, `shared-host.v1` and the row "shared-host requirement" follow. |
| V2-02 | wrong | fix | NAM-024: an input that a later release adds to a Requirement contract states no `default`, so it is sent only where a document writes it, and an implementation built for the earlier release is sent no argument it does not know unless an author wrote one. CON-022, IFC-019 and the row "contract" follow. M45's fail-closed rule stands. |
| V2-03 | misleading | fix | As V5. |
| V2-04 | wrong | fix | PLC-023 does not hold back a read of a secret that PRV-026 makes to run the machine provider again: that provider's report is what gives a machine a state, so holding the read would keep every machine but `local`'s omitted after every restart. A read has no effect, and where it cannot reach its machine it does not succeed and the report counts as not received. PRV-026 and the row "unreachable, gone" follow. The verifier's other option, rejecting a provider whose secret Resource is not on `local`, would add a rule; PRV-026 keeps that as advice. |
| V2-05 | contradiction | fix | `inspect.v1`, the rows "inspect" and "the engine's ID for an invocation", two places in the core README and the overview say "each invocation whose outcome is unknown", as LIF-013 does. The core README's sentence on ruling 42 and the head of `lifecycle.yaml` record that time and keep their words. |
| V2-06 | contradiction | fix | The rows "result", "tool" and `Conduit's keys`, and the core README's section on results, say that -32602 with no tool result is the answer at the baseline revision, and that at a revision at which a tool result answers such a call, it carries the reason (IFC-012, IFC-025). |
| V2-07 | minor | fix | The row "part" defines a part of the engine as a second sense, written "a part of the engine" or "the part of it", and says that a part that reports a reason or ends with an outcome is always a part of a setup or a retirement. |
| V2-08 | misleading | fix | IFC-017 names the stop of a part of the engine, and excepts the run of a secret Resource that PRV-026 makes again and the run that TPL-018 asks for in a retirement. The rows "caller", `serving`, "runtime provider" and "the engine's read of a secret" add the stop of a part. |
| V2-09 | minor | fix | ACC-039, CON-031, LIF-015 and LIF-024 add a request to unregister the consumer, under LIF-017. |
| V2-10 | minor | fix | As V3. |
| V2-11 | minor | fix | IFC-022: the token is accepted over TLS at an `https` URL and without TLS at the `http` loopback URL that DL13 gives a run. DL13 says the same. |
| V2-12 | minor | fix | DL13 leaves to the engine only the key and the client identifier of its JWT assertion under RFC 7523. |
| V2-13 | undetermined | fix | REC-002: the record of a request that ACC-037 decides because whether the instance that its credential names still exists cannot be read holds no caller, and says that this could not be read. IFC-032 cites it. |
| V2-14 | undetermined | fix | IFC-001: where the tool list cannot be read, or does not hold the tool that a call names, nothing is sent, and the request, the hook or the part reports unavailable with a record that says why, as IFC-037 has it for discovery. |
| V2-15 | minor | fix | IFC-010 reads the protocol's -32021 for a client capability that the call did not declare as failed, as it reads -32602, since the protocol has a server answer so where acting would need that capability; OPS-007 excepts it for an implementation that carries Conduit's keys, and IFC-045 says why, as IFC-008 reads the -32021 that names Conduit's extension. |
| V2-16 | minor | fix | IFC-015 says that the engine gives a `remote` item no means to make an onward request, as a consequence of IFC-041, and TPL-017 no longer implies that a hook reached by `remote` makes onward requests. No rule is added on what DL13's authorization server issues, as M54 added no mechanism. |

Note of 2026-11-07: ruling 44 reversed V2-15, on finding 6-1-01 of
[round 6](../round-6/DECISIONS.md#the-six-findings), which an independent check confirmed. The
protocol has a server answer -32021 where "processing" a request needs a capability that it did
not declare, and says nothing of whether the server acted first, so the reason given for V2-15
does not follow from it. The error is now read as any other error, failed for an operation
that declares no effect and unknown for one that declares one, except where Conduit's own
contract binds the server to have done nothing: the -32021 that names Conduit's extension
(IFC-008), and an implementation that carries Conduit's keys, which answers so only before
acting (IFC-045). IFC-007, IFC-010, IFC-045 and OPS-007 changed together. The row above keeps
its words as written.

### Counts

Counted with a script on the working tree after the verification: 452 promises, 202 of the
layer `language`, 207 `convention` and 43 `interface`; by level, 404 `must`, 44 `must not`, 3
`may` and 1 `should`. The two added, IFC-045 and IFC-046, are of the layer `interface`, so
`interface/implementation.yaml` holds 43. None was removed or moved between files, and no ID
was reused. [spec/README.md](../../../../spec/README.md#contents) and
[INDEPENDENT_BUILDS.md](../../../implementation/INDEPENDENT_BUILDS.md) give 452 and 43. The
implementation-defined list still holds 15 items, and the examples and the checker's rules are
as after the integration.

Changed: ACC-039; CON-006, CON-022, CON-031; IFC-001, IFC-003, IFC-007, IFC-010, IFC-015,
IFC-016, IFC-017, IFC-019, IFC-022, IFC-025, IFC-032, IFC-036, IFC-037; IMP-007; LIF-015,
LIF-024; NAM-024; OPS-005, OPS-007; PLC-013, PLC-023; PRV-007, PRV-025, PRV-026; REC-002;
TPL-017: 30 promises, and the item DL13. Each keeps its ID. Each changed YAML file has a line
in its head comment for this verification, and each changed Markdown file under `spec/` a
sentence in its history. In the core package the descriptions of `register.v1`, `inspect.v1`
and `shared-host.v1` changed. Outside `spec/`, the verification changed the tracker (V6's row
under "Waiting for the owner", V12), the re-audit record (V13), the choices of ruling 43's
verification (V11), `INDEPENDENT_BUILDS.md` (the counts) and the overview (V2-05).

Note of 2026-11-07: V6's row under "Waiting for the owner" was removed from the tracker when
the coordinator settled V6.

### For the owner

**V6. An instance that runs nothing, and the `nodeSelector` of its type and items.** M22 places
on `local` an Agent or a Resource for which no item that is run is selected, and PLC-002 checks
that placement under PLC-007 as any other. A template whose type states a label that `local`
lacks, such as `kubernetes.io/os: linux` on an engine on macOS, is then rejected even where it
selects only a `remote` item and runs nothing. Before M22 the machine provider placed it on a
machine that matched. Options:

1. PLC-007 leaves the type's and the items' `nodeSelector` out of the selector of an instance
   for which no item that is run is selected, since a `nodeSelector`, as decision 8a takes it
   from Kubernetes, says where a workload runs, and nothing of it runs. Such a template is
   accepted wherever `local` stands.
2. As now, with PLC-002 saying why: a type's `nodeSelector` is a placement need of every
   instance of the type, and an instance that runs nothing is placed on `local`, so `local` must
   match. A user whose type needs another machine gives a placement in the document, which
   brings back the unavailability with that machine that M22 removed.
3. PLC-002 places such an instance on `local` where `local` matches its selector, and otherwise
   asks the machine provider, as before M22.

Settled by the coordinator on 2026-11-07, under rulings 37 and 38, as option 1, and applied to
PLC-007 and PLC-002. Recommendation: 1. It removes a rejection that has no reason where nothing runs, which is
friction on one machine that the owner's principles rule out; it adds no exception to M22; and
it is consistent with V2-01, under which an end that runs nothing is taken to run nowhere.

The decisions of this verification that the owner may want to read first, since each settles
a behavior and not only a wording: V2-01, which rejects a shared-host requirement on an end that
runs nothing; V2-02, which forbids a `default` on an input that a later release adds to a
requirement; V2-04, which exempts the machine provider's reads from PLC-023; V2-15, which reads
the protocol's -32021 for a capability that the call did not declare as failed; and V7, which
lets a gate pass the engine's token on to the server.

Note of 2026-11-07: in V6 above, the sentence that records its settlement stands before the
recommendation; the settlement took the recommended option, 1. Ruling 44 reversed V2-15, as
the note under the table of this verification says.

### Checks

Run from the repository root on the working tree after the verification:

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
452 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 174 Markdown files and 2379 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x437, apiVersion x257, metaPrefix x104; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
5. ... Checked 1 controls, the 5 operations that take no `revision`, the 1 that returns one,
the tables of the two READMEs, and 1015 citations of promises in 112 files. ...
6. Applied 25 rules of this tool, which come from 24 promises, to the core package and 3
example directories. 30 sets of documents in 30 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 53 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```

No line of a YAML file under `spec/` is longer than 96 characters, and every promise that a
file under `spec/` cites exists.
