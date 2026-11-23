# Conduit CLI format reference

Start with the author’s reading path in `spec/README.md` and the short form in
`spec/promises/sections.yaml`. The specification defines the document format and binds.
This guide describes the CLI. Use `conduit COMMAND --help` for a command’s own usage.
The approval flags `--prune`, `--adopt`, `--with-resources` and `--with-agents` are not yet
built.

## From an empty directory to a plan

1. Run `conduit init --store DIR` once to initialize the store.
2. Run `conduit serve --store DIR` with that same store and leave it running. Use another
   terminal for the remaining commands. Install and plan both need the serving engine.
3. In the empty project directory, run `conduit init` to create the project manifest.
4. Write the project documents using the author’s reading path in `spec/README.md`.
5. Still in the project directory, run `conduit install ARCHIVE#SHA256` for each package
   the documents use directly. Install resolves its dependencies and pins the named archive
   in the project manifest.
6. Run `conduit plan` in that directory to print the proposed steps.

`conduit apply` computes, prints and keeps its own ready plan and sends the approved steps.
Running `conduit plan` first is optional.

## Engine context

```text
conduit serve --store DIR [--https-url URL] [--https-listen ADDRESS] [--https-roots FILE]
```

`conduit serve` attempts to record the current engine in `context.json` before printing its
ready line. If the write fails, it warns on stderr, prints the ready line and continues serving.
The file contains the canonical store path, its loopback URL and the `conduit --version` line
with the crate version and build commit. The record remains after the engine stops. The file
lives in the absolute directory named by `CONDUIT_CONFIG_DIR`, or in `$HOME/.config/conduit`
when that variable is unset or empty.

Serving commands verify that the store is initialized, that its fixed URL matches the file and
that the engine at that URL answers as root. They name the context file, store and URL on stderr.
A missing, invalid or stale context exits 2 and tells you to start `conduit serve --store DIR`
or give `--store DIR`. An explicit `--store DIR` selects the store for that command and bypasses
the context. Serving commands use the store’s fixed URL. The context directory and the package
cache live outside every project.

## Initialize a project or a store

```text
conduit init
conduit init --store DIR [--label KEY=VALUE] [--trust-domain DOMAIN]
```

Bare `conduit init` writes `conduit.yaml` with exactly these two lines.

```yaml
apiVersion: github.com/shrav-k/conduit/v0
packages: {}
```

If `conduit.yaml`, `conduit.yml` or `conduit.json` already exists, the command reports that
and leaves it unchanged (MAN-001). Both creating the skeleton and finding an existing
manifest exit 0. Use `conduit init --store DIR` to initialize the private store and root
identity instead. That form writes only within the store.

## Install packages

```text
conduit install [--store DIR] [ARCHIVE#SHA256] [--mirror DIR] [--dry-run]
```

Give an archive path and its hex SHA-256 digest. Install walks that release’s pinned dependencies,
leaves first, and sends one `install.v1` request for each missing release before installing the
named archive (PKG-006, PKG-028). An installed dependency with the required version and digest
already has its own dependencies. The named archive is sent last. With no archive argument,
install restores the current manifest’s pinned releases and their dependencies without editing
the manifest. That form needs a manifest in the current directory (MAN-001).

For a dependency, install tries the following sources in order and accepts bytes only when their
digest matches (NAM-012, PKG-021, PKG-026).

1. The package cache.
2. An absolute `file:` URL given by the pin’s `fetch`.
3. Each `--mirror DIR`, in command-line order.
4. Each directory in the `mirrors` list in `config.json` in the context directory.

Each configured mirror path is absolute. A mirror contains archives named by lowercase hex
digest without the `sha256:` prefix. A relative or `https:` fetch is unbuilt in this CLI and
appears as `(unbuilt)` in `tried=` when the search fails. A candidate whose digest differs
appears as `(mismatch)`.

The per-user cache is the `packages` subdirectory of the context directory. It holds verified
archives named by lowercase hex digest, shared by every working copy. Install keeps each
archive it reads whose bytes match its digest, including the named archive and any dependency.
Plan and apply only read the cache and use the built-in core package from their own build.

After the named archive succeeds, install writes its exact version and digest to the current
manifest, whether the version moves up or down. It writes only that archive’s pin and preserves
the rest of the manifest, including its `apiVersion`. It writes no dependency pin or `fetch`.
The archive form refuses a pin containing `fetch`, even when the version and digest already
match. Remove `fetch` before using install to change that pin, or edit the pin by hand.
Without a manifest, the archive form can install without pinning and reports `pin none`.

`--dry-run` reads installed releases and prints the proposed installs, sources and pin
change. It sends no `install.v1` request and writes no cache or manifest files.

```text
list Package/* as=root
present Package/ADDRESS version=VERSION digest=sha256:HEX needed-by=[...] with its dependencies (PKG-028)
install Package/ADDRESS version=VERSION digest=sha256:HEX from=SOURCE needed-by=[...]
pin added ADDRESS version=VERSION digest=sha256:HEX file=FILE
pin moved up ADDRESS version=OLD->NEW digest=OLD->NEW file=FILE
pin unchanged ADDRESS version=VERSION digest=sha256:HEX file=FILE
pin none: no manifest in DIR
stop Package/ADDRESS version=VERSION digest=sha256:HEX CAUSE needed-by=[...] PROMISE
stop Manifest/- file=FILE cause=CAUSE pin=ADDRESS@VERSION digest=sha256:HEX requires=edit -
not sent [ADDRESS@VERSION,...]
dry-run: no install.v1 sent, nothing written
```

`list` precedes the inventory read. A `present` release is already installed. `from=` names
`argument`, `cache`, `fetch:URL` or `mirror:DIR`. `needed-by=` lists the releases that require
that archive, or `Manifest/-` when restoring a direct pin. The `install` line precedes the
request, and an engine reply prints as JSON when it arrives. A transport failure can leave
an `install` line without a reply. After an unsuccessful request, `not sent` lists the
remaining releases. Successful installs stand. Pin changes can also say `moved down` or
`moved`, the latter for changes at equal version precedence. A `stop` gives the cause, such
as `archive=absent tried=[...]` or a conflicting digest (PKG-025). A successful dry run
ends with the `dry-run` line shown above.

## Plan and apply

```text
conduit plan [PROJECT_DIR] [--store DIR] [--reopen OBJECT] [--recreate OBJECT] [--prune OBJECT] [--replace OBJECT] [--adopt OBJECT] [--with-resources OBJECT] [--with-agents OBJECT]
conduit apply [PROJECT_DIR] [--store DIR] [--reopen OBJECT] [--recreate OBJECT] [--prune OBJECT] [--replace OBJECT] [--adopt OBJECT] [--with-resources OBJECT] [--with-agents OBJECT] [--expect sha256:HEX]
```

`PROJECT_DIR` defaults to the current directory. Planning resolves the project with the pinned
archives in the cache and compares it with the working copy’s kept state and the serving
engine. As root, `conduit plan` sends only `list.v1` and `inspect.v1` to read the engine and
writes nothing (OPS-036, LIF-013, ACC-033). An invalid value in a secret-reference place stops
before it connects and names only that place (SEC-001, SEC-003). A relative fetch or program
in a project handed over as values stops under SUB-010.

A version mismatch stops with status 2 before any `list.v1`, `inspect.v1` or planned step.
With the context, the shipped planner compares the recorded crate version and build commit
with its own. With `--store`, it compares the crate version reported by the engine.

Apply computes and prints the same plan. When it sends steps, it keeps the plan, sends the
steps one by one and waits for each result. The command is the approval. It runs without a
prompt or a prior plan command. A stopped plan sends none of its numbered steps, including
narrowings. A no-op sends no steps and writes no state.

### Apply the plan you reviewed

`conduit plan` and `conduit apply` compute the same plan given the same project, store and
approval flags. The digest is `sha256:HEX`, where HEX is the lowercase SHA-256 of the UTF-8
plan text with LF line ends and one final newline. It starts with `plan` and ends with
`ready`, `no changes` or `stopped`. These are the exact bytes of plan's stdout and of the
file apply keeps at `.conduit/plans/HEX.txt` when it sends steps. Results, `verify`, `not sent`
and panel lines, and stderr are outside the digest. When a plan reaches approval evaluation,
a used flag appears on its step and the `flags` line. An unused or ambiguous flag produces a
stop line. Preflight stops such as SEC-001 or a missing package archive occur before approval
evaluation and print `flags []`. These early stops can have the same digest with or without
an approval flag. `--expect` and the command spelling are outside the plan text.

```sh
conduit plan > plan.txt; shasum -a 256 plan.txt
```

After printing a plan, `conduit plan` flushes stdout and writes this one line to stderr,
including for stopped plans and plans with no changes. It keeps the plan's exit status.

```text
conduit plan: digest sha256:HEX; conduit apply --expect sha256:HEX, given the same project, store and approval flags, sends nothing unless the plan it computes has this digest
```

When plan prints no plan text, it prints no digest. Apply prints no digest announcement.

`conduit apply --expect sha256:HEX` accepts one digest operand among its arguments. The
following usage errors exit 2 before reading the project or engine, with empty stdout and
one stderr line. `{VALUE:?}` is the operand as Rust's debug format writes it, quotes included.

Missing operand or operand starting with `-`

```text
conduit apply: --expect needs sha256:HEX, the digest conduit plan prints; nothing was read or sent
```

Malformed operand

```text
conduit apply: --expect {VALUE:?} is not sha256: and 64 lowercase hex digits; nothing was read or sent
```

`--expect` given twice

```text
conduit apply: --expect is given twice; give one digest; nothing was read or sent
```

`--expect` given to plan

```text
conduit plan: --expect is apply's; conduit plan prints the digest to give it; nothing was read
```

`conduit apply --expect --help` shows help with status 0.

Apply prints its plan from live state and compares the digest before writing any state or
sending any step. A match continues with the plan as usual, including a stop if one was
printed. A mismatch exits 2, writes nothing under `.conduit/` and sends no request after
the initial reads. Stdout also gets `not sent 1..N` if the plan has N steps. A SEC-001 stop
has no numbered steps, so stdout contains only its plan text regardless of `--expect`.

The caller supplies only the digest for approval and needs no prior plan invocation or
plan file. Apply can still read its own kept plan and journal to recover an interrupted
replacement.

On a mismatch, apply writes one of these lines to stderr, depending on the computed plan.

Ready plan

```text
conduit apply: the plan computed now has digest sha256:NEW, not sha256:OLD as --expect gives; nothing was sent or kept; stdout holds that plan; the digest covers what it is computed from: the engine, the project files, .conduit/ and the approval flags given; no earlier plan is kept, so the lines that changed are not named; review it, then rerun this command with --expect sha256:NEW to send it
```

Stopped plan

```text
conduit apply: the plan computed now has digest sha256:NEW, not sha256:OLD as --expect gives; nothing was sent or kept; stdout holds that plan; the digest covers what it is computed from: the engine, the project files, .conduit/ and the approval flags given; no earlier plan is kept, so the lines that changed are not named; it is stopped, so --expect sha256:NEW sends nothing either: resolve each stop line, run conduit plan with the approval flags apply will be given, and give apply the digest it prints
```

No changes

```text
conduit apply: the plan computed now has digest sha256:NEW, not sha256:OLD as --expect gives; nothing was sent or kept; stdout holds that plan; the digest covers what it is computed from: the engine, the project files, .conduit/ and the approval flags given; no earlier plan is kept, so the lines that changed are not named; it has no changes, so there is nothing to send
```

### Approval flags

Each approval flag is repeatable and takes one generated object name. When the planner
evaluates approvals, an unused, ambiguous or contradictory operand stops the plan. The
inline template of a definition `N` is `N--t`, with instance `N` (SCT-006). A `consumes`
item `L` on `N` makes
Conduit `N--L` and, for an inline item, template `N--L--t` (SCT-007). An `operator` makes
`N--operator` and, if
inline, `N--operator--t` (SCT-009). Authored short-form names contain no `--`, and generated
names are at most 63 characters (SCT-013).

| Change | Permission and effect | Rules |
| --- | --- | --- |
| Add an object with an unused name | No flag. Register it, then inspect an instance before opening or setting it up. | SCT-003, LIF-037, TPL-012 |
| Update a template in place | No flag for the template update. Existing instances keep their resolution. | SUB-009, INS-020 |
| Move a package pin without changing an instance’s meaning or execution | The instance keeps its held pin and the plan prints a difference. | NAM-009 to NAM-011 |
| Drop a Conduit recorded under the same ID | No flag. Close it and keep its tombstone. | CON-008, LIF-040 |
| Narrow a Conduit, including a smaller operation set | No flag. Close it, lift a matching tombstone made by that change, and register, inspect and open the narrower replacement. The consumer and target stay the same. Requirements may be added. The core `bound.v1` lists `pools`, `targets` and `operations` may shrink. | ACC-029, LIF-041, ACC-053 |
| Restore a Conduit the engine no longer holds, including a renamed Conduit whose registration matches the old tombstone | `--reopen NAME`. Lift a matching tombstone before registration or opening. | LIF-040, LIF-041, ACC-053 |
| Recreate another missing instance this copy kept, or an instance with a matching tombstone | `--recreate NAME`. Lift a matching tombstone, then register, inspect and set up. | LIF-040, LIF-041, LIF-037 |
| Remove another object dropped from the file | `--prune NAME`. Retire an instance first, then remove its inline template when eligible. A standalone template is unregistered at its revision. | LIF-017, LIF-035, INS-020 |
| Replace a standalone instance, change its owner, or change a standalone Conduit where narrowing cannot be proved | `--replace NAME`. Remove the old instance before registering its replacement under the same name. Inspect before activation. | OBJ-011, INS-020, LIF-037, LIF-041, ACC-029 |
| Replace a nested instance, such as a declared Conduit or the Resource of a Pool entry | `--replace NAME` names the old child as inspection shows it. A declared Conduit that narrows needs no flag. Close the Conduit or retire the Resource through its Pool, then find and lift its tombstone if it would match. `management.register-replacement.v1` takes the parent as `instance` and the slot name as `nested`. It returns a new child name and ID for inspection and activation. | LIF-009, LIF-041, LIF-037 |
| Use a live name whose ID, or template revision, this copy did not keep | `--adopt NAME`. Verify identity and definition; further changes still need their own permission. | OBJ-002, LIF-037 |
| Drop a Pool with members or a Fleet with members | `--with-resources NAME` for the Pool, `--with-agents NAME` for the Fleet. The cascade is one approval. | LIF-020, LIF-021, CON-008 |
| Protected engine object, change to placement or runtime, or replacement of an owner with members | Stop. No flag permits these changes. | ACC-024, FLEET-005, LIF-020 |
| Control changed a setting | Print a difference if the file still holds the ledger value. Stop if the file has a third value or the plan would replace the instance and revert the control’s change. No flag permits that stop. | REV-007 |
| Unreconciled outcome | Stop if a numbered step would reach the affected object. Reconcile an unknown invocation explicitly. A setting's stop holds after reconciliation until a later control that was sent a value for that setting, or for one that contains it, succeeds other than by reconciliation (REV-007, REV-010); apply never sends that control. Then plan again. | OPS-012 to OPS-014, REV-009 |
| Unsupported input, such as a relative program in a submission of values | Stop and name the input. No flag permits it. | SUB-010 |

When a change needs replacement permission, the shipped planner prints a `stop` with
`requires=--replace NAME`. The stop names the changed place, setting, pin or selected item.
Passing an unbuilt approval flag (`--prune`, `--adopt`, `--with-resources` or `--with-agents`)
to the planner exits 2 and tells you to plan without it.
The planner can also show proposed close steps for flag-free narrowing. Previews of
bounds that would lose coverage and sign-ins that would be lost are planned with the
replacement flags.

Narrowing steps precede widening steps. When a plan closes or retires objects, template
updates and registrations precede the first close or retirement. Dependencies come before
their dependents, with stable kind and name ordering for ties. Batched registration orders
documents by Pool, Resource, Agent, Fleet, Interface and Conduit, then by name, with an
inline template immediately before its instance. Independent setups order Pool, Resource,
Fleet, Agent and Interface, then name. Each verification read immediately precedes the
activation it verifies.

The planned cascade temporarily opens a Conduit for the cascade operation alone. That is
the exception to narrowing before widening. Retirement removes the Conduit within the
cascade and the plan shows that closure under `affects` (CON-008, LIF-035). The final two
steps find and lift its tombstone (LIF-040, ACC-053). This example uses a Pool named `archive`.

```text
plan
1 management.register.v1 Conduit/cascade-archive expected=(step 1) adds flag=--with-resources archive ACC-054
2 management.inspect.v1 Conduit/cascade-archive expected=(step 1) neither flag=- LIF-013
3 access.open.v1 Conduit/cascade-archive expected=(step 1) widens flag=--with-resources archive LIF-037
4 cascade-archive.retire-with-resources.v1 Pool/archive expected=POOL-ID narrows flag=--with-resources archive LIF-021
5 management.list.v1 Tombstone/cascade-archive expected=- neither flag=- LIF-040
6 access.lift.v1 Tombstone/cascade-archive expected=(step 5) neither flag=--with-resources archive ACC-053
affects step=4 closes=[Conduit/cascade-archive] CON-008
flags [--with-resources archive]
ready
```

### Plan text

The plan starts with `plan` and ends with `ready`, `no changes` or `stopped`. Each numbered
step gives the tool, its subject, the expected ID, effect class, approval and deciding promise.

```text
N TOOL KIND/NAME expected=ID CLASS flag=PERMISSION PROMISE
```

`expected=-` means there is no single expected ID. `(step N)` refers to an ID supplied by
that step’s result, such as a registration or a list of tombstones. Later requests use the
returned ID where their schema allows `expectedId` (LIF-037). `flag=-` means the step needs
no approval flag. A batch can carry several permissions, separated by commas.

| Class | Meaning |
| --- | --- |
| adds | Registers an absent object. A registration can include a root default (FLEET-005). |
| narrows | Closes or retires access, or restricts it to a subset. |
| widens | Opens access or activates an instance whose setup can open a declared Conduit. |
| replaces | Updates a stored template. Existing instances keep their resolution (INS-020). |
| removes | Unregisters a template or a settled retired instance. |
| neither | Reads, lifts a tombstone or runs a setup that adds no access. |

`detail` follows its step. It can show arguments, a registration’s object list and defaults,
package identities, resolved operations or a fingerprint. The planner emits the first form
for each Agent instance in a registration. It emits the second below an open step’s
`operations=` detail when an operation starts model work and the Agent target’s tools
information is available (CTR-014, LIF-013).

```text
detail ownTools Agent/NAME JSON CTR-014
detail targetOwnTools Agent/NAME JSON CTR-014
```

`NAME` is the Agent in the registration or the target of the Conduit. The compact JSON is
`{"stated":false}` when the Type has no tools-directory declaration. Otherwise `stated` is
`true`, `pointer` gives the configuration location and `directoryNamed` says whether a
directory is set. A named directory adds `directory` and sets `enforcement` to `"neither"`.
An absolute directory is shown as `{"pathSha256":"HEX"}` in the `directory` field.

After all steps and details, `stop` lines appear in byte order by subject and then by line.
When there are numbered steps, the shipped planner follows the stops with this line.

```text
affects bounds=[...] closed-conduits=[...]
```

`bounds` is currently empty. `closed-conduits` lists the Conduit names in proposed close
steps in byte order, separated by commas. `difference` follows and explains observations
such as retained pins, control drift and unknown outcomes. The shipped planner prints `flags []` when it
has steps or stops. The planned flag handling will list used flags sorted by flag and
operand. The final plan status follows.

```text
stop SUBJECT CAUSE... [requires=REMEDY] PROMISE
```

A subject is `KIND/NAME`, `Package/ADDRESS` or `Manifest/-`. Causes use `key=value` fields.
`place` and `setting` are JSON Pointers. `id`, `revision`, `tombstone`, `unreconciled`,
`absent`, `state` and `made-by` describe identity or lifecycle. `control`, `invocation`,
`changed-by`, `item` and `secret` explain controls or hidden item differences. A setting that the
engine's status holds as unknown after a control (REV-009) has this stop line:

```text
stop KIND/NAME setting=POINTER state=unknown control=CONTROL invocation=ID REV-009
```

It holds after reconciliation until a later control that was sent a value for that setting, or
for one that contains it, succeeds other than by reconciliation (REV-007, REV-010). Apply never
sends that control; after it succeeds, plan again. `pin`, `version`, `digest`, `installed` and
`archive` describe packages. `rule` identifies a stop with no deciding promise, whose trailing
promise is `-`. A `requires=` field gives a remedy, such as a flag and its operand or
`conduit-install`.

```text
plan
stop Conduit/curator--index tombstone=STONE-ID requires=--reopen curator--index LIF-041
flags []
stopped
```

With neither a step nor a stop, any differences are followed by `no changes`. There are
no `affects` or `flags` lines in that case.

```text
plan
no changes
```

### Apply results and exit statuses

Apply prints `result N OUTCOME JSON` after each step. OUTCOME is
`succeeded`, `failed` or `unknown`, or a reason such as `refused`, `invalid`, `conflict` or
`unavailable`. The JSON is a safe projection in compact, sorted-key form. If a stopped
plan has steps, `not sent 1..N` lists
them. After a step fails or verification detects a mismatch, `not sent N..M` gives the
remaining range, where present.

Apply lists each pending sign-in as `panel action ID: sign-in required` apart from the
numbered steps. It continues after a successful setup and leaves the sign-in to the person.
A failed setup still stops apply, and any action it raised is printed (IFC-025, PND-005).

| Status | Plan and apply | Install |
| --- | --- | --- |
| 0 | Valid plan, successful apply or no changes. Pending sign-in alone is success. | Successful install, restore or dry run. |
| 1 | Engine read or transport failed, a step did not succeed, or verification failed. | Shared argument-parser error, such as an unknown option or `--store` with no following argument, or an engine request or connection failure. |
| 2 | Usage, local input or state error, persistence failure, version mismatch, stopped plan or a digest other than `--expect`'s. | Missing `--mirror` operand, missing, invalid or stale context, installer input or preflight stop, or a pin left unwritten after successful installs. |

### Stops, repeats and kept state

Apply stops on an unsuccessful step and keeps completed effects. It sends no automatic
retry and performs no rollback. An explicit new apply reads the engine
and the journal, then resumes verified work still pending. It does not claim a registration with unknown
delivery from the object’s name alone. Reconcile an invocation reported unknown before
sending work that reaches its affected objects (OPS-012 to OPS-014).

The shipped planner stops on an instance whose shutdown remains unconfirmed after retirement.
It prints `state=retired rule=none -` for a matching ledger object dropped from the file or
already recorded as retired in the ledger. When the file still names the instance and the
ledger has no matching retired entry, it prints `state=retired LIF-024`.

Planned recovery lists and inspects the instance again. If it still has the recorded ID and
is retired with shutdown unconfirmed, a new explicit apply sends a further retirement
under the permission used originally, such as `--replace NAME` or `--prune NAME` (LIF-026,
LIF-037). If the instance is gone or the name has another ID, plan from the observed identity
and state. `unregister.v1` is unavailable as a recovery shortcut because it asserts that
shutdown was settled elsewhere (LIF-017).

Each post-print request carries `plan:<hex digest>:<attempt>:<step>` under the `_meta`
key `io.github.shrav-k.conduit/idempotencyKey` (IFC-023, OPS-008). A plan digest starts at
attempt 1. An explicit new command for the same digest increases the attempt after a known
unsuccessful step. A completed or interrupted attempt keeps its number, including a send
without a known result. The engine decides repeats and conflicts (OPS-009, OPS-012, OPS-034).

| Path relative to the project | Kept content |
| --- | --- |
| `.conduit/plans/HEX.txt` | The exact UTF-8 plan, with LF and one final newline. HEX is its SHA-256. Results are outside the digest. |
| `.conduit/state.yaml` | Format 1, file digests, object IDs and revisions, definitions and held pins, tombstones and plan and journal references. |
| `.conduit/results/HEX.yaml` | The ordered journal. Each step’s safe arguments, flag and key are written before sending, then its safe result is kept. |

The shipped planner treats a missing state file as an empty ledger. Malformed state exits 2
with a diagnostic to repair it before planning. Plan, stopped apply and no-op apply write
none of these files. Keep `.conduit/` out of version control. The ledger records ownership
for comparison. Engine authorization remains the boundary. Kept content holds no absolute
path, store path or credential. A non-secret
absolute path or file URL can be represented as `{"pathSha256":"HEX"}`, the SHA-256 of its
UTF-8 bytes. A secret reference is kept as a reference to a Resource, with its resolved ID,
without the secret itself (OBJ-009, SEC-003, SEC-004).
