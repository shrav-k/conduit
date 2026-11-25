# Conduit CLI format reference

Start with the author’s reading path in `spec/README.md` and the short form in
`spec/promises/sections.yaml`. The specification defines the document format and binds.
This guide describes the CLI. None of its commands is an engine operation. Use `conduit
COMMAND --help` for a command’s own usage.
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
conduit serve --store DIR [--https-url URL] [--https-listen ADDRESS] [--https-roots FILE] [--grace-period-seconds N]
```

`conduit serve` attempts to record the current engine in `context.json` before printing its
ready line. If the write fails, it warns on stderr, prints the ready line and continues serving.
The file contains the canonical store path, its loopback URL and the `conduit --version` line
with the crate version and build commit. The record remains after the engine stops. The file
lives in the absolute directory named by `CONDUIT_CONFIG_DIR`, or in `$HOME/.config/conduit`
when that variable is unset or empty.

SIGTERM or SIGINT asks `conduit serve` to stop (OPS-037). From then on it takes no new
request: each reports unavailable, naming OPS-037, and may be made again once serve has
started again. Calls in progress, and the calls they make, may finish within the grace
period, `--grace-period-seconds N` (30 by default). Serve stops as soon as none is in
progress, and at the latest when the grace period ends. At that bound each remaining
call is cancelled and its caller answered, a setup or a retirement as a cancellation
ends it, any other call with the outcome unknown. A second signal ends the grace period
at once. Serve reports the stop and how many calls were cut on stderr; inspecting
`system` lists the two records of the stop.

After the engine stops, serve allows up to three seconds for responses to drain and
up to three seconds for required provider stops to be sent. It then ends its runs and
exits, with DL16’s TERM-to-KILL grace (10 seconds by default) and three-second check
that the runs have ended. An external supervisor must allow more than the grace period
plus 19 seconds with these defaults.

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

## Installation values

```text
conduit value set NAME [--node NODE] [--store DIR]
conduit value list [--show] [--store DIR]
conduit value unset NAME [--store DIR]
```

Each command uses the engine context or `--store DIR` and sends root's requests through
that serving engine. Status 0 means saved, unchanged, listed or unset; 2 means a usage
error, invalid local context or input, refusal or declined save; 1 means an engine read,
transport or request failure. Here `NAME`, `ID`, `OLD`, `T`, `TIME`, `POOL`, `TYPE`, `URL`
and `NODE` stand for the corresponding names, IDs and metadata. Quoted values are JSON
strings, preserving spaces and quotes. Progress goes to stdout; prompts, echoes and
refusals go to stderr. “Nothing sent” means no write request; preflight can read the engine.

`set` checks names held by Resource templates as well as instances before reading input.
Each of these refusals is one stderr line, status 2, with no question or write:

```text
conduit value set: "NAME" is not a name.v1; nothing was read or sent
conduit value set: root sees no node named "NODE"; its nodes are [N,...]; nothing was read or sent
conduit value set: Resource/NAME exists with Type TYPE, not value.v1; choose another name; nothing was read or sent
conduit value set: Resource/NAME is a template, not a value instance; choose another name; nothing was read or sent
conduit value set: Tombstone id=T of Resource/NAME id=OLD, owned by Pool/POOL, retired at TIME (cause CAUSE), holds the name; to reuse the name, lift it as root first: conduit call --store 'DIR' access.lift.v1 '{"id":"T"}', then run conduit value set NAME again; nothing was read or sent
conduit value set: reads the value from your terminal, and standard input is not a terminal; a script registers a value Resource as root with management.register.v1 (MCP or conduit call); nothing was read or sent
```

The name and node in the first two lines use Rust's debug string quoting. `TYPE` is the
inspected Type reference (for example, `#worker.v1` on a template). `CAUSE` is JSON;
`DIR` and the lift argument are shell-quoted. The template-specific refusal applies when
the template has Type `value.v1`; another Type gets the other-Type refusal. A name held
only by a tombstone is always refused: a removed ID cannot be inspected (LIF-035,
OPS-029). Root must explicitly lift it before reusing the name. `set` lifts only the
matching tombstone of the value ID it has itself just retired. It never offers `--file`;
scripts use `management.register.v1` as root.

At the terminal, `set` tells this warning, then the held-value line if applicable, then
asks for a line with echo on:

```text
A value is not secret: the engine keeps it as configuration, conduit inspect and conduit value list --show show it, and conduit records exports it. A password or token goes on a secret Resource, named by a secret reference (SEC-001), never in a value.
NAME holds "OLD-VALUE" (WHERE) as Resource/NAME id=ID and fills [KIND/NAME POINTER,...]; each keeps "OLD-VALUE" until it is replaced (OBJ-010)
Type the value of NAME, then Enter:
NAME = "VALUE" (WHERE)
NAME = "VALUE" (WHERE), replacing "OLD-VALUE" (OLD-WHERE)
```

Only one echo line is printed; the replacement form applies when the held value or node
differs. `WHERE` is `not tied to a node` or `tied to node NODE`. The line ending is removed
from the typed value. Saving requires `y` or `yes`, in any case, at one of these questions
(each prompt ends in a space):

```text
Save it as root in the engine at URL? [y/N]
Save it as root in the engine at URL? This retires Resource/NAME id=OLD, lifts its tombstone and registers the new value. [y/N]
Resource/NAME id=ID holds this value and node but is not set up; set it up as root in the engine at URL? [y/N]
```

If the value and node already match and the Resource is ready, there is no save question;
stdout is the following line, status 0:

```text
unchanged Resource/NAME id=ID: the engine holds this value and node; nothing was sent
```

Each input error or declined save is one stderr line, status 2, with no write. The control
character position counts Unicode characters from 1 (U+0000–U+001F or U+007F–U+009F).
A non-UTF-8 line is a terminal read error, including at the save question.

```text
conduit value set: no value was read: standard input ended; nothing was sent
conduit value set: the value is empty; nothing was sent
conduit value set: the value holds a control character at character N; a value is one line of text; nothing was sent
conduit value set: not saved: the answer was not y; nothing was sent
conduit value set: could not read the terminal: ERROR; nothing was sent
conduit value VERB: store is not initialized; run conduit init; nothing was sent
```

After consent, requests run in this order, each only after the preceding step succeeds:

| State before saving | Requests (in order) |
| --- | --- |
| New name | `management.register.v1 Resource/NAME`, `system.set-up.v1 Resource/NAME` |
| Held value, changed value or node | `system.retire.v1 Resource/NAME`, `management.list.v1 Tombstone/NAME`, `access.lift.v1 Tombstone/NAME`, `management.register.v1 Resource/NAME`, `system.set-up.v1 Resource/NAME` |
| Same value and node, registered but not ready | `system.set-up.v1 Resource/NAME` |
| Name held only by a tombstone | Refusal before input; root explicitly lifts it, then reruns `set` as a new name |

The listing finds the tombstone by the retired Resource's ID. A new registration is one
value Resource document, owned by `system`, from the core `value` template, with `value`
and optional `node` arguments. Existing instances keep what they hold until replaced
(OBJ-010). Each request prints the first stdout line below, numbered from 1; a failed
reply follows as one JSON line. A transport failure has `outcome=unknown`. Success ends
with `saved`, status 0; `node=-` means no node. No progress line includes the typed value.

```text
N TOOL SUBJECT outcome=OUTCOME
saved Resource/NAME id=ID node=NODE
```

After a failure, remaining requests are reported as `not sent K..M` on stdout (omitted
if none remain). Status is 1. Stderr identifies the failed step; successful replies
missing the required result instead identify that missing result:

```text
conduit value set: N TOOL did not succeed; nothing more was sent
conduit value set: N management.list.v1 listed no tombstone of Resource/NAME id=OLD; nothing more was sent
conduit value set: N management.register.v1 returned no ID for Resource/NAME; nothing more was sent
```

The line has one of these endings where applicable. The first is for set-up failure;
the second for failure after retirement but before a confirmed lift; the third for
failure after retirement and confirmed lift. Nothing is retried or undone.

```text
; Resource/NAME id=ID is registered but not set up; conduit value set NAME again sets it up
; Resource/NAME id=OLD is retired and its new value is not registered; instances keep what they hold; check management.list.v1 for its tombstone and lift it as root with access.lift.v1 before running conduit value set NAME again
; Resource/NAME id=OLD is retired and its new value is not registered; instances keep what they hold; conduit value set NAME again registers it
```

`list` prints one compact, sorted-key JSON line per value Resource root sees, sorted by
name then ID. `fills` lists live instances, excluding templates and retired instances,
whose inspected source maps name that ID; entries sort by kind, name and place:

```text
{"fills":[{"kind":"KIND","name":"NAME","place":"POINTER"}],"id":"ID","live":true,"name":"NAME","node":"NODE"}
{"fills":[],"id":"ID","live":true,"name":"NAME","node":null,"value":"VALUE"}
```

The second line illustrates `--show`, which alone adds `value`; `node` is a node's name
or null. An ID still named by an instance but absent from live value Resources gets
`"live":false`. Its name and node come from `management.inspect.v1` by ID, each null if
that inspection fails; with `--show`, its value is also null if unavailable. No rows gives
this stderr line and status 0:

```text
conduit value list: root sees no value Resource and no instance filled by one
```

If inspection of a current instance fails, stdout includes its reply as a JSON line,
stderr gives the following line, and status is 1:

```text
conduit value list: inspect.v1 of KIND/NAME did not succeed, so fills may be incomplete
```

`unset` asks nothing. Either refusal below is stderr, status 2, with no write:

```text
conduit value unset: root sees no value Resource named NAME; conduit value list names those it sees; nothing was sent
conduit value unset: Resource/NAME fills [KIND/NAME POINTER,...]; each keeps its value until it is replaced or retired, so a value an instance names is not retired; retire or replace each, then unset again; nothing was sent
```

Otherwise it sends `system.retire.v1`, with the same outcome and failure-JSON format as
`set`. On success stdout has these two lines, status 0:

```text
1 system.retire.v1 Resource/NAME outcome=succeeded
unset Resource/NAME id=ID: retired; its tombstone holds the name until root lifts it with access.lift.v1; conduit value set NAME then registers it again
```

A retirement failure exits 1 with stderr
`conduit value unset: 1 system.retire.v1 did not succeed; nothing more was sent`.

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

### Duties under the short form

The current CLI behavior is described above. These are the reference CLI's duties under
the short form, which defines registration by translation
([specification](../spec/README.md#what-the-short-forms-meaning-covers)); the approval
flags listed above as not yet built, `--withhold` and the `values` stop with
`conduit value set <name>` are not yet built.

Plan and apply are tooling composed of existing requests. The plan is printed, and apply
records it; nothing runs automatically. `conduit apply` computes and prints its own plan,
records it when it has a request to send, and then sends its requests; `conduit plan`
prints the same plan, sends none of its requests and records no plan; the reads it makes
to compute the plan are inspections, which change nothing and are recorded as any request
is (REC-001). Plan is available and never required. A plan resolves each `values` entry
from the value Resources it inspects, under OBJ-009, and a plan and its record name each
value by its value Resource and hold no value. Where an entry does not resolve, the plan
stops at its document, naming the place, the value Resource and `conduit value set
<name>`: that command registers a value Resource as root, from the person's terminal,
showing the value as typed and saving it only on y. In v0 the command is the approval.
Every dangerous act needs a named flag, recorded with the plan when used: `--reopen`,
`--recreate`, `--prune`, `--replace`, `--adopt`, and the cascade flags `--with-resources`
and `--with-agents`. Each flag names one object by its name, the generated name where the
short form makes one (SCT-013), such as `--reopen auditor--catalog`. Each cascade is one
command under one flag, printed and recorded with the plan: register a one-time Conduit to
the Pool or Fleet, open it, and call the cascade operation through it. The target's
retirement closes and removes that Conduit (CON-008, LIF-035), so apply sends no close;
root then lifts the Conduit's tombstone (LIF-040, ACC-053). A cascade that does not
succeed stops apply as any failure does, and its one-time Conduit may already be removed:
a later apply under the same flag closes it only if inspection finds that same Conduit,
and lifts only the tombstone that retains its ID. The `--withhold` flag belongs with the
withhold behavior in batch 2. Narrowing never waits and never needs a flag: a Conduit the
file no longer names is closed on apply, and so is a narrowing of a Conduit's operations,
and the plan prints and records each closing; `--prune` is for removing another object the
file no longer names. A plan prints and records every widening, including one caused by
moving a package pin or changing a reusable template's operations (PKG-031). A package
should keep a standard template's operations across releases; the plan still checks the
operations that actually resolve. A plan prints, beside each Agent it registers or
replaces, the `ownTools` member that inspection would show (CTR-014, LIF-013). Beside each
Conduit it opens whose target is an Agent and that lists an operation marked
`startsModelWork: true`, it prints the target's `ownTools` member.

A command that talks to a serving engine finds it through the person's current engine
context, as kubectl's and Docker's commands do: `conduit serve` records the engine it
serves as current, and `--store` on a command overrides the context for that command. When
a set-up that apply sends raises pending sign-in actions (IFC-025, PND-005), apply goes on
to its next step: a sign-in never stops it. It prints each as the person's action at the
panel, listed apart from authored lines, and answers none. A set-up that does not succeed
stops apply as any failure does, and any sign-in it raised is still printed.

Replacement keeps names stable (SCT-004, SCT-013): retire and remove the old instance
before making its replacement under the same name, lifting its tombstone wherever
necessary (LIF-035, LIF-041, ACC-053). This accepts a longer window without access. Plans
print affected bounds as well as the Conduits they recreate: an individual target's bound
keeps its old ID, while an owning Pool's bound covers a new Resource in that same Pool
(ACC-029). These are tooling duties and limits of the registration equivalence, not
further engine operations.

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
