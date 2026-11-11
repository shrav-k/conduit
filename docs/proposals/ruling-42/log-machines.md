# Ruling 42: log of the machines group

Date: 2026-11-06. This is the account of the editor who applied [ruling 42](README.md) to four
promise files: [placement](../../../spec/promises/placement.yaml),
[providers](../../../spec/promises/providers.yaml), [pools](../../../spec/promises/pools.yaml) and
[first start](../../../spec/promises/first-start.yaml). The rulings are the owner's, and the
decisions applied are the coordinator's and the owner's. The choices listed under
[Choices beyond the record](#choices-beyond-the-record) are this editor's, and each stands unless
the coordinator or the owner strikes it.

What was applied: parts (a) and (b) of the ruling, the decisions of the coordinator in the table
[Settled by the coordinator](../ruling-41/open-decisions.md#settled-by-the-coordinator), every
row but row 5, with the notes "Settled together" below it, and the owner's answers D8 (option
c), D10, D11 and D17, as the [ruling's record](README.md#what-follows) states them. D6, D12 and
R5 touch none of these four files. The names of the vocabulary audit (N1 to N20) and its
editorial fixes (E1 to E23) were not applied: another stage applies them, so the text keeps the
names in force today, such as `secret-reveal.v1`, location and operation identity.

Not applied, because the owner did not rule them: D5 and D20. For D20 the text keeps the guard
of row 17, under which no runtime provider other than the engine's own runner is asked to run a
`command` item whose program is a relative path. For D5 these files say nothing new.

## Checks

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) | 439 promises ok, with every group's files as they stood when it ran |
| Every promise in the four files has exactly the seven fields, its text fields folded with `>-` | Yes |
| A line over 96 characters in the four files | None |
| An ID or a DL item cited in the four files that does not exist | None. LIF-036, SEC-022, PKG-020's new sentence and DL16 were already written by their groups when this check ran |
| `python3 tools/check_docs.py` | 1 error, not in these files: `spec/core/README.md` links to `docs/proposals/ruling-42/log-core.md`, which its group had not yet written |
| `python3 tools/set_names.py --check` | 0 errors |
| `python3 tools/check_schemas.py` | 0 errors |
| `git diff --check` | Clean |

## Counts

26 promises changed and 3 added; none removed and none moved between files.

| File | Changed | Added |
| --- | --- | --- |
| `spec/promises/placement.yaml` | PLC-005, PLC-007, PLC-017, PLC-023, PLC-024, PLC-025, PLC-026, PLC-027 | None |
| `spec/promises/providers.yaml` | PRV-001, PRV-003, PRV-004, PRV-005, PRV-006, PRV-007, PRV-008, PRV-009, PRV-011, PRV-013, PRV-014, PRV-015, PRV-018, PRV-019, PRV-020 | PRV-021, PRV-022, PRV-023 |
| `spec/promises/pools.yaml` | POOL-009, POOL-016 (layer `language` to `convention`) | None |
| `spec/promises/first-start.yaml` | FST-001 | None |

Each of the four files has a line in its head comment naming ruling 42, what changed and this
log.

## New IDs

The three new IDs were assigned in advance by the coordinator, so that other groups could cite
them. This group added no other.

| ID | Answer | What it says |
| --- | --- | --- |
| PRV-021 | D11 | A runtime provider asked to stop a run that it cannot stop now carries out the stop once it can, as its contract states, whether or not the instance still exists. |
| PRV-022 | D17 | A runtime provider given `identity` in `run.v1` delivers its variables and files to the run, puts them nowhere else, keeps them as it keeps a secret's value, and may replace a file's content in place. No record holds them. |
| PRV-023 | D8 | The engine's own runner gives a process what the run binds and, of the engine's environment, only `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER` and the variables of a list that the user sets on the runner; what the run binds wins. |

## Decisions applied

| Row or answer | Promises | What changed |
| --- | --- | --- |
| Row 1 (D1) | PLC-007, PRV-003, PRV-006, PRV-015, PRV-019, POOL-009 | For an instance that has an item that is run and names no `runtime`, the label selector of PLC-007 also holds `kubernetes.io/hostname` with the value in `local`'s status, as RuntimeClass scheduling merges into a Pod's. The diagnostic names the engine's own runner as the label's source. `place.v1` is given that selector, so it names the engine's machine or none. An instance with no item that is run is placed as before. |
| Row 2 (D2), with D11 | PRV-008, PRV-020 | PRV-020 confirmed, with the Kubernetes precedent and PLC-011's conflict. PRV-008 says that there is one to ask while the instance exists, no longer "always". |
| Row 4 (D4) | PLC-027 | The observable cites finalizers: a machine that comes back gets a new location, so the hook is never dispatched, and the request to unregister is the assertion that its work is done another way. It holds for a hook on a `remote` item too. |
| Row 7 (D8, part) | PRV-003 | Superseded by the owner's D8: PRV-003 points to PRV-023, and the working directory, user and login session are DL16, which the group of the implementation-defined list wrote. |
| Row 11 (D13) | PLC-017, PLC-024, PLC-026, PRV-007 | A declaration of loss and a report of a machine gone take effect at once and dispatch nothing, so REV-012 does not hold them back; no later part of a setup begins (TPL-012). A location's setup dispatches nothing and succeeds at once. PRV-007 cites TPL-012 beside PLC-027. |
| Row 12 (D14) | PRV-005 | A provider call's record holds in `invocation` the invocation within which it is made, and no `serving`; the instance whose run is meant is named in the record's `subject`. |
| Row 13 (D15) | PLC-023, PLC-026, PRV-003, PRV-013, PRV-014, FST-001 | Hostnames are compared in canonical form, trimmed and lowercased. The engine's machine is the one whose hostname is the engine's own, the operating system's unless configured otherwise (DL5). An omitted machine counts as unreachable until a report gives it a state; with no report in DL5's time every machine but `local`'s counts as omitted. A location stands for a hostname whichever provider reports it, so retiring or replacing the machines provider retires no location. A report that names a hostname twice is not used. The provider's contract states which name it reports. |
| Row 14 (D16) | POOL-016 | Layer `language` becomes `convention`; the promise stays in its file. PLC-007 and PLC-013 stay `language`. |
| Row 15 (D18) | PRV-001, PRV-005 | A provider call declares Conduit's extension under IFC-037, and its result is read as IFC-009 and OPS-007 read a request's. Where the extension is missing, the call is not dispatched, its record names the extension, and what needed it reports unavailable. PRV-001 points to PKG-020: no plain implementation implements an operation of a provider capability. |
| Row 16 (D19) | PRV-003, PRV-007, PRV-009 | `run.v1` requires `endpoint`, and the engine's own runner is the one exception. A result without it fails validation and the part ends unknown; the retirement's stop ends the run. How often the engine asks `inspect-run.v1` is DL5. |
| Row 17 (D20, part) | PRV-007 | The guard: the engine asks no runtime provider other than its own runner to run a `command` item whose program is a relative path; that part ends failed and nothing runs. Kept as the text for D20, which the owner did not rule. |
| Row 22 (R9) | PRV-007 | Each value is given as a string: a string as it is, a number in RFC 8785's form, a boolean as `true` or `false`. |
| Row 23 (R10) | PRV-007, PRV-011 | `secrets` holds two maps, `environment` and `arguments`, keyed by name; a name is in `environment` or in `secrets`'s `environment`, never both. A provider delivers each value under the name it is given. |
| Row 24 (R11) | PRV-007 | A place that holds null gives no variable and no argument, left out of `environment`, `arguments` and `secrets` alike. What becomes of an input of the Package that is given no value is server.json's. |
| Row 26 (R16) | PLC-025 | Two selected items that name one label with different values are rejected, as a type and an item are. |
| Row 27 (R17) | PLC-005, PRV-006, PRV-020, POOL-009 | A template that gives a `runtime`, or an input's `default` for an entry's `runtime`, is rejected. A `runtime` that names a retired runtime provider is rejected; one registered and not yet ready is accepted. |
| Row 28 (R19) | PRV-015, POOL-016 | Unavailable, not rejected, where the machines provider is not ready, its call ends with no result, it names a machine that has no location yet, or it names none while a matching location other than `local` is on a machine reported unreachable or omitted. |
| Row 37 | PRV-004 | Confirmed: the rule is keyed on the operation, by its qualified reference in the core package, whatever capability lists it. |
| Row 38 | PRV-005 | Confirmed: `_meta`'s `instance` names the provider, the input `instance` names the instance whose run is meant, and `config` is current on each call while a run keeps what it started with. |
| D8 (option c) | PRV-023 (new), PRV-003, PRV-007 | The engine's own runner passes six variables of the engine's environment and those of a list set on it, as systemd's `PassEnvironment=` does, and what the run binds wins. PRV-007 says that the run's inputs hold no value of the engine's environment, and points to PRV-023 for what the runner passes beside them. |
| D10 | PRV-001 | A provider's type may offer other capabilities, except a Type that offers `secret-reveal.v1`, which offers no other, under SEC-022. |
| D11 | PRV-021 (new), PLC-017, PLC-027, PRV-008, PRV-020, PRV-005 | A request to unregister an instance on a location declared lost asks the runtime provider for the stop once more (LIF-036, of the lifecycle group). A runtime provider carries out a stop that it cannot carry out now once it can (PRV-021). PRV-005 says that this stop is made within the invocation of `unregister.v1`. |
| D17 | PRV-022 (new), PRV-007, PRV-018 | `run.v1` is given `identity`, which DL13's mechanism fills: the engine's address as a variable and a credential as a file. The provider delivers and keeps them as it keeps a secret's value, may refresh a file in place, and no record holds them. PRV-018 points to it. |

The rows of the table that touch no promise of these four files were left to the groups whose
files they touch, among them rows 3, 6, 8, 9, 10, 18 to 21, 25, 29 to 36 and 39 to 44.

## Choices beyond the record

1. PLC-007's `when` now defines the selector, the union of the type's, the selected items' and,
   for an instance on the engine's own runner, the hostname label. Before, it held only where a
   type or an item states `placement`. The diagnostic names "the source of the label": the type,
   the item or the engine's own runner.
2. PLC-007 says that where the type or an item names `kubernetes.io/hostname` with another value
   for an instance on the engine's own runner, no location matches. This follows from "hold each
   label with the value it gives". See [open item 6](#needs-a-decision).
3. PLC-023's `then` says that nothing is retired because a machine is unreachable or omitted,
   matching its new `when`. PRV-007 and PRV-008 also say "unreachable or omitted".
4. PLC-017, PLC-027, PRV-005, PRV-008 and PRV-020 cite LIF-036 for D11, in the wording of
   LIF-036 as the lifecycle group wrote it: the stop is asked before the instance is removed,
   within the invocation of `unregister.v1`.
5. PRV-007 keeps row 17's guard in its `then`, as an exception to its one obligation, since the
   record applies the rows of PRV-007 as one change. Its observable explains the guard: a program
   at a relative path is a file on the machine the engine runs on.
6. PRV-007 and PRV-011 say "names" for the keys of `arguments`, `environment` and `secrets`, as
   the description of `run.v1` that the core group wrote does. Row 35 keys an argument by its
   `name` or its `valueHint`.
7. PRV-009: the record's entry asked to say that "the condition always carries the time of the
   answer". That was not written, since OBS-003 gives `Unknown` with no time where nothing was
   reported; the time of the answer is already in PRV-009's `then`. Only the pointer to DL5 was
   added.
8. PRV-006 gives the reason for rejecting a retired runtime provider as "the shutdown of an
   instance that named it could never be confirmed, under LIF-024". The record's entry said that
   LIF-017 would never remove the instance, but LIF-017 does remove one on a location declared
   lost.
9. PRV-013: the record's entry said that the locations left behind when a provider reports
   another form of a hostname are cleared "only by a declaration of loss". The text says that an
   actor retires one once no instance is placed on it (PLC-011), or the root Interface declares
   it lost (PLC-017).
10. PRV-015: "A provider of one machine, and the engine's own machine, place every instance on
    that machine" became "name that machine or none". Under row 1, a provider of one machine
    other than the engine's names none for an instance on the engine's own runner.
11. PRV-018 points to `identity` (D17). The record did not list PRV-018.
12. PRV-021's `when` reads "a run that it cannot stop now, as where it cannot reach the machine
    that the run is on", and its observable says that the stop then ends unknown, as `stop.v1`'s
    description has it where the provider cannot tell whether the run stopped. The precedents are
    the kubelet and Nomad's `disconnect`, from the analysis of D11.
13. PRV-022 says that a file is delivered "at that path, as the run sees it", matching `run.v1`'s
    "under the path at which the run finds each". Its `then` says that the provider "may"
    replace a file in place, as the ruling says. "No record holds them" is in the observable;
    the rule for the record is in `run.v1`'s description, written by the core group: "The
    record of the call holds no value of `secrets` and nothing of `identity`".
14. PRV-023 says "a list set by the user on the engine's own runner", and cites DL16 for how it
    is set, for an operating system without the six variables, and for the working directory,
    user and login session. DL16 is the implementation-defined list's group's new item.
15. Coined name, for the owner to strike: the input `identity` of `run.v1`, an object with
    `environment` (names to strings) and `files` (paths to contents). The coordinator gave the
    name in this stage's work order; PRV-007 and PRV-022 use it, as does `run.v1`.

## Needs a decision

1. **D20 (not ruled).** Row 17's guard stays in PRV-007. When the owner rules D20, PRV-007 changes:
   to a rejection when the document is resolved (the engine's own runner only), or to PRV-012's
   form with the package's address, version, digest and `fetch` in `run.v1` (the provider fetches).
2. **D5 (not ruled), and the research the owner asks for.** These files make no change for D5.
   The questions in these files that the research on work across machines should take up,
   looking ahead to versions after v0:
   - Who may reach a run's `endpoint` (PRV-018, PRV-019). Today nothing limits it. Under D5's
     "full" option the run would check the engine against a trust anchor, and PRV-022's channel
     is where that anchor would travel, beside the run's own credential.
   - How a refreshed credential reaches the runtime provider during a run (PRV-022). The engine
     gives `identity` only in `run.v1`, and DL13 now leaves "whether and when it is refreshed"
     to each engine. A later version may need a provider operation that updates `identity`, or
     a mechanism from which the provider takes the credential itself, as SPIFFE's Workload API
     serves it on the node.
   - How a run on another machine reaches the engine at an address that works from there
     (PRV-022 lets the provider "give it as the run sees it"), and what the engine does when the
     machine's network changes during a run.
   - Whether a program inside a package can run on another machine (D20), which needs the
     provider to fetch and check the package.
3. **PRV-021: a stop held by a runtime provider that is then retired.** Once the instance is
   removed, PRV-020 allows the provider to be retired. What becomes of a stop it still holds is
   not stated.
4. **PRV-023: values that begin with `()`.** The SDKs' stdio clients also skip a variable whose
   value begins with `()`, a shell function. The ruling names the six variables and the list,
   and not the skip, so it was not written. The owner may add it.
5. **PRV-022 and PRV-023: a name in both.** Which value wins where a variable of `identity` has
   the name of one that the item binds is not stated.
6. **PLC-007 and PRV-015: a selector that cannot be written as a map.** Where the type names
   `kubernetes.io/hostname` with another value for an instance on the engine's own runner, the
   union names one label twice. PLC-007 says that no location matches. Whether the engine then
   asks `place.v1` at all, and with which selector, is not stated.
7. **For the docs editor** (not this group's files): `spec/definitions.md` should follow these
   changes in the entries for location, placement, unreachable and gone (an omitted machine),
   the engine's own runner (PRV-023), runtime provider (PRV-021, PRV-022) and run (`identity`).
   `spec/README.md` should state the costs: for D8, that a variable the list names reaches every
   process the runner starts, and that a provider's own server, which the engine's own runner
   runs under PRV-002, gets only the six and the list; for D17, that the provider's place and
   the run's machine hold a credential that presents as the instance while it exists.
