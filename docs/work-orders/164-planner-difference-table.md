# Work order 164: the planner's difference table, and package contents by digest

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access, and needs none;
it runs no cargo command. The case files run to 50 KB, so never print a whole one: use `sed -n`,
`grep` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or a credential.

## What and why

The full review of 2026-11-14 (order 23 of [its orders](../proposals/full-review/orders.md))
found three gaps in [the plan-and-apply reference](../implementation/plan-and-apply/README.md),
which order 052a builds `conduit plan` and `conduit apply` from:

- Which requests an author's edit becomes is decided only in prose that no program has followed,
  and some differences have no rule: a changed requirement or bound, the flag of a nested
  instance, an instance left registered, and the project's own contracts
  ([FR-293](../proposals/full-review/findings/P03-plan-apply.md#fr-293) as its verifier corrected
  it; [FR-291](../proposals/full-review/findings/P03-plan-apply.md#fr-291) rows 3 and 7; row 4,
  rename, order 154 decided).
- Every plan for a project that names a package stops for want of the package's contents: open
  point O7, which no queue held
  ([FR-294](../proposals/full-review/findings/P03-plan-apply.md#fr-294)).
- An author's edit to a setting that a control covers can only replace the instance, and the plan
  does not say that the control exists
  ([FR-295](../proposals/full-review/findings/P03-plan-apply.md#fr-295), whose verifier left the
  coordinator only a refusal).

This order writes one table that maps every difference to one action or one named stop, ending in
a stop for any difference no other row names. It settles O7 with a cache of archives keyed by
their digest, and names the control in the stop. Under FR-293 rules 3 and 4 as the verifier
limited them, it also sends template steps before any close or retirement, which reorders two
steps of cases 05 and 06, and keeps all-or-nothing, with a line that says nothing was sent. Order
052a is written against the table, and the cases are inputs of the convergence check that is its
done-when (decision D26 of
[the coordinator's decisions](../proposals/full-review/decisions.md#the-s1-rerun-and-its-report)).
[Order 154](154-plan-apply-reference-corrected.md) corrected the reference against `spec/`; this
order builds on it as it stands. The reference binds nothing, no promise changes, and no engine
operation is added. Each decision below is the coordinator's under ruling 48, logged for the owner
to strike.

## The decisions it applies

Line numbers are those of `docs/implementation/plan-and-apply/README.md`, "the reference", on
`main` at `632881df`. If a line has moved, find the phrase by its words.

1. **One table decides every difference.** Source: FR-293 rule 1 with its verifier's
   correction (a row for the project's own contracts and implementation items), FR-291 rows 3
   and 7, and FR-295's refusal. Add a section `## The difference table` after "Read, compare,
   print, keep, send" and before "Approval and command readings under ruling 76" (line 106). It
   opens with a paragraph that says:
   - Step 3 compares three values at each place of each object: the file's, from its translation
     resolved against package contents read from the cache; the ledger's, from the kept state with
     the journal of this working copy's interrupted apply, if any; and the engine's, from list and
     inspect.
   - A place is a JSON Pointer into the object's document as the file translates it or, where only
     its resolution differs, into its resolved definition. A setting is a JSON Pointer into an
     instance's configuration, as a control's `changes` writes it.
   - The rows are read in order, and the first whose condition holds decides. Rows 1 to 6 are
     checked first; row 5 alone looks ahead, and decides only where a step of this plan would
     reach the object, so that it never hides a stop of the rows below. Rows 7 to 16 compare an
     object's presence and lifecycle; rows 17 to 26 compare each place of an object that the
     ledger and the engine hold under one ID and the file holds by name. Each row gives one
     action: nothing, create, update in place, replace, remove, or a stop that names its cause.
     The last row stops on any difference that no other row names, so planning is total by
     construction.
   - Where the places of one object fall under several rows, the plan prints every stop they
     give; with no stop, the object is replaced where any of those rows replaces it, and
     otherwise each row's action is taken. An inline template `N--t`, while N is in the engine and
     row 7, 14 or 15 decides N, and a member that row 14's cascade retires, take no row of their own
     after row 6. The row that acts on them decides them, and rows 1 to 6 still apply to them first,
     as row 5 says of a cascade. Any other object that another object's row acts on, such as an
     incident Conduit that a replacement makes again, keeps its own row. The plan prints every stop
     that either row gives, so replacing an endpoint never takes over a Conduit another actor
     changed (row 10; safety case 5). Where neither row stops, the row that acts on it decides it.
   - An object that neither the file nor the ledger holds, and that the engine did not make, is
     outside the project: no row applies to it, and the plan reads it only for bounds and outside
     Conduits (row 26). An object that only the ledger holds is gone already: nothing is sent for
     it, and the next kept state leaves it out.
   - Lines that report what stays, such as the retained-pin `difference` line and case 07's
     `remains ready` line, are printed as now.

   Then the table below, unindented. Copy it as it stands. You may tighten a row's wording where
   every action, flag, stop line and promise stays; report each such change.

   | # | Difference | What the plan does | Decided by |
   | --- | --- | --- | --- |
   | 1 | A package the manifest pins is not installed at that version with that digest, as `list.v1` reports | Stop: `stop Package/ADDRESS version=VERSION digest=sha256:HEX installed=no requires=conduit-install PKG-006`. Where that version is installed with another digest: `installed=sha256:OTHER`, no `requires=`, and PKG-025, since installing it would conflict | PKG-006, PKG-021, PKG-025 |
   | 2 | The archive of a package the manifest pins, of a package that an archive's manifest lists, through every dependency, other than the core package (NAM-009), or of a release an object holds that row 19 compares, is not in the package cache, or its cached bytes do not hash to its digest | Stop: `stop Package/ADDRESS version=VERSION digest=sha256:HEX archive=absent requires=conduit-install PKG-004`, or `archive=mismatch`. Installing that archive again fills the cache and changes nothing in the engine (PKG-027) | O7; PKG-004, PKG-027 |
   | 3 | A relative `fetch`, or a program given as a relative path | Stop, as O1 says: `stop KIND/NAME place=POINTER SUB-010`, with `Manifest/-` as the subject in the project's manifest | O1; SUB-010 |
   | 4 | A literal or a bare name at a place that `secret-reference.v1` governs | Stop, as O5 says: `stop KIND/NAME place=POINTER SEC-001`, with no step, `detail` or `difference` line | O5; SEC-001, SEC-003 |
   | 5 | An object with an unreconciled invocation, or an instance with a setting reported unknown, that a step of this plan would reach: a step that the rows below give it, read as though this row did not hold, or would give under the flag that one of their stops requires, or a step that another object's row sends to it, as a cascade does | Stop while the invocation is unreconciled or the setting is reported unknown, as "Stops, unknown outcomes and repeats" says: `stop KIND/NAME unreconciled=ID OPS-013`, or `stop KIND/NAME setting=POINTER state=unknown REV-009`, and print beside it every stop that the rows below give the object. Apply sends no reconciliation and no control. Where no step would reach the object, this row does not decide it: the first row below whose condition holds does, as for any other object, whether that is row 17's nothing, row 19's retained-pin line or a stop such as row 27's. Beside that row's decision the plan prints the same cause as a `difference` line: `difference KIND/NAME unreconciled=ID OPS-013`, or `difference KIND/NAME setting=POINTER state=unknown REV-009` | OPS-012 to OPS-014, REV-009, LIF-013; ruling 78 point 5 |
   | 6 | An object the engine made: root's first-start objects, or a FLEET-005 default | Nothing is sent to it, and no later row closes, replaces or adopts it. A default closes only when its Pool or Fleet retires (CON-008), which the plan prints under `affects`. Where the file holds a document of its kind and name: `stop KIND/NAME made-by=engine ACC-024` for a Conduit, or with OBJ-002 for another object; no flag permits it | ACC-024, FLEET-005, CON-008, OBJ-002; safety case 13 |
   | 7 | A retired instance whose shutdown is unconfirmed: one this working copy retired, or one the file holds that this working copy did not retire | A further retirement under the flag that permitted the retirement, which this command names again (LIF-026); never `unregister.v1` (LIF-017). Without the flag: `stop KIND/NAME state=retired requires=FLAG NAME LIF-026`. For an instance that `--prune` named, once it is removed, `unregister.v1` of its inline template `N--t` follows under the same flag, as row 15 says. Where the file holds an instance that this working copy did not retire, because another actor retired it or its node was declared lost (PLC-017, PLC-024): `stop KIND/NAME state=retired LIF-024`, with no `requires=`. No later row decides it; once its shutdown is confirmed, row 11 does (safety case 3) | LIF-024, LIF-026, LIF-017 |
   | 8 | An instance this working copy registered that is still registered: an earlier apply stopped before its setup or open | Where no place of it differs from the file's: inspect and verify it, then set it up or open it under the flag its registration carried, as the journal shows. Where that was a flag this command does not name: `stop KIND/NAME state=registered requires=FLAG NAME LIF-013`. Where a place differs, rows 17 to 26 decide it | LIF-013, LIF-037, TPL-012 |
   | 9 | In the file; absent from the ledger and the engine; no tombstone matches it | Create: register; then, for an instance, inspect, then set up or open, with no flag (cases 01 and 02) | SCT-003, LIF-037, TPL-012 |
   | 10 | In the file and in the engine under its kind and name, where the ledger has no entry for it or holds another ID, or, for a template, another revision | Stop: `stop KIND/NAME id=ID requires=--adopt NAME OBJ-002`, with `revision=REVISION` after `id=` for a template that another actor replaced. With `--adopt`, take it into the ledger as "Approval and command readings under ruling 76" says; the rows below then decide it | OBJ-002, LIF-037; safety case 5 |
   | 11 | In the file and absent from the engine, and a tombstone matches its registration or its open | Lift it with no further flag where "Which tombstones apply lifts with no further flag" allows. Otherwise stop: `stop KIND/NAME tombstone=ID requires=--reopen NAME LIF-041` for a Conduit, `--recreate NAME` for another instance, and `tombstone=(KIND/OLD)` for a tombstone this apply would make, as for a renamed Conduit. With the flag: lift it, register, inspect, then open or set up (case 08) | LIF-040, LIF-041, ACC-053; ruling 76 point 1; cases 08 and 09; safety cases 1 to 4 |
   | 12 | In the file and the ledger; absent from the engine; no tombstone matches it | Stop: `stop KIND/NAME absent=engine requires=--recreate NAME LIF-040`, or `--reopen NAME` for a Conduit. With the flag: register, inspect, then set up or open | LIF-040 |
   | 13 | In the ledger and the engine, absent from the file: a Conduit | Close it, with no flag; its tombstone stays (case 07). A renamed Conduit is this row and row 11 | ruling 76 point 1; CON-008 |
   | 14 | In the ledger and the engine, absent from the file: a Pool or Fleet that owns members | Its cascade under `--with-resources NAME` or `--with-agents NAME`, printed as "Replacement, removal and cascades" shows. Without the flag: `stop Pool/NAME absent=file requires=--with-resources NAME LIF-020`, or the Fleet form. Members that the file also leaves out take no row of their own. Once each is removed, `unregister.v1` of its inline template follows under the same flag | LIF-020; ruling 59 A3 |
   | 15 | In the ledger and the engine, absent from the file: another instance | `--prune NAME`, then its retirement, then, once it is removed, `unregister.v1` of its inline template `N--t` under the same flag. Without the flag: `stop KIND/NAME absent=file requires=--prune NAME LIF-035`. A renamed instance is this row and row 9 | ruling 76 point 1; SCT-006, LIF-035 |
   | 16 | In the ledger and the engine, absent from the file: a template that is not inline, or an inline template `N--t` whose instance N the engine no longer holds | `--prune NAME`, then `unregister.v1` at its revision, carrying `expectedId`. Without the flag: `stop KIND/NAME absent=file requires=--prune NAME INS-020` | INS-020 |
   | 17 | A place where the file's value is the engine's | Nothing, whatever the ledger holds | this table |
   | 18 | A project template whose document differs from the engine's, or that refers to a package whose pin moved, or to a contract document or an implementation item of the project that changed | Update in place: `replace.v1` at its inspected revision, with the project's contracts and manifest, class `replaces`. It changes no instance, so it takes no flag of its own; where the plan replaces its inline instance, the step carries that `--replace` (cases 05 and 06). Rows 19 to 26 decide each instance | SUB-008, SUB-009, INS-020, SCT-006 |
   | 19 | An object whose held pin is not the one the file's resolution gives, where row 18 does not replace it and its resolved meaning and execution would not differ under the file's release | Nothing: it keeps its pins, and the plan prints its retained-pin `difference` line, as cases 05 to 09 do | NAM-009 to NAM-011 |
   | 20 | A nested instance, a declared Conduit or a Pool entry's Resource, whose template, named by its `from`, changed, while the instance that holds it is not replaced | Close it, or retire it through its Pool; list and lift the tombstone that leaves where the new one would match it; then `register-replacement.v1`, which gives it a new name and ID; then inspect, and open or set up. The close or retirement, the lift, the registration and the open or setup carry `--replace NAME`, naming the nested instance as inspection shows it, or no flag for a declared Conduit whose access narrows as row 21 says. Without the flag: `stop KIND/NAME place=/from requires=--replace NAME LIF-009` | LIF-009, LIF-041; ruling 76 points 1 and 2 |
   | 21 | A Conduit whose access after the change is within its access before: the same consumer and target, its resolved operations a subset, and each requirement kept with the same arguments, except that requirements may be added and each list of a `bound.v1` (`pools`, `targets`, `operations`) may shrink | Close, list, lift its tombstone, register, inspect, open, with no flag (case 04, Agent B's Conduit). A change of `as` alone is one | ruling 76 point 1; ACC-029, LIF-041 |
   | 22 | Any other change to a Conduit: its consumer, target, operations, requirements or their arguments, its `from` or its template | Replace it under `--replace NAME` by the same steps (case 04, Agent A's Conduit). A change the planner cannot show to narrow is widening. Without the flag: `stop Conduit/NAME place=POINTER requires=--replace NAME OBJ-011` | ruling 76 point 1; OBJ-011 |
   | 23 | A setting whose value in the engine is not the ledger's: a control changed it | Where the file's value is the ledger's, send nothing for it and print `difference KIND/NAME setting=POINTER changed-by=control REV-007`. Stop instead, with `stop KIND/NAME setting=POINTER changed-by=control REV-007`, where the plan would replace the instance for another difference, which would revert the control's value, or where the file holds a third value. No flag permits that stop; writing the engine's value into the file settles it | REV-007 |
   | 24 | A Pool or Fleet that owns members, whose definition the file changed | Stop: `stop KIND/NAME place=POINTER LIF-020`. No flag permits it, since replacing it would retire its members | LIF-020 |
   | 25 | A setting whose value the file changed, where the engine's value is the ledger's | Replace the instance as row 26 says; apply never sends a control. Without `--replace NAME`: `stop KIND/NAME setting=POINTER control=OPERATION requires=--replace NAME OBJ-011`, one line for each setting. `control=` names a control the instance has, one its Type lists that its template's `controls` map, where it has one, also lists (TPL-010, TPL-013), and whose `changes` holds that pointer. It is given only while the instance is ready, since a control is not dispatched otherwise (CON-024). Calling that control is the author's other remedy. Where the instance has no control whose `changes` holds that pointer, or it is not ready, the line has no `control=` item, and `--replace NAME` is the only remedy | OBJ-011, OPS-020, REV-007, TPL-013, CON-024; decision D67 |
   | 26 | Another instance whose definition would differ: its `from`, `owner`, `description` or `annotations`; its resolved definition other than its configuration, `placement` and `runtime`; or a pin of a package it uses, where row 19 does not hold | Replace it under `--replace NAME`, as "Replacement, removal and cascades" says: its incident Conduits are made again under the same flag; the plan prints the bounds that cease to cover it and the loss of an Agent's sign-in; and a Conduit outside the file whose consumer or target it is stops the plan, as "Approval and command readings under ruling 76" says. An owner change is a replacement, never a move. Without the flag: `stop KIND/NAME place=POINTER requires=--replace NAME OBJ-011`, with `pin=ADDRESS@VERSION` in place of `place=` for a moved pin, one line for each | OBJ-011, INS-020, FLEET-002, NAM-010, ACC-029 |
   | 27 | Any other difference | Stop, naming the field: `stop SUBJECT place=POINTER rule=none -`. No flag permits it; a later order adds a row for it. In v0 it takes, among others, a changed `placement` or `runtime` (safety case 12), `apiVersion`, and the manifest's `address`. `state=STATE` takes the place of `place=` for a lifecycle state that no row above decides | this table |

   Text to change to match:
   - **Step 3** of "Read, compare, print, keep, send" (lines 71 to 77) ends with a sentence that
     the difference table decides each difference, linked to the new section.
   - **Nested instances** (lines 219 to 224). In the printed line, `flag=...` becomes
     `flag=--replace NAME`, or `flag=-` for a declared Conduit whose access narrows. The sentence
     "It lifts no tombstone, and any matching tombstone must already have been lifted by an
     independently permitted step." becomes: before it, the plan lifts the tombstone that its own
     close or retirement of the nested instance left, where the new one would match it (LIF-041),
     under the same flag (row 20).
   - **The lift rule** (line 231). Apply also lifts with no further flag a tombstone that this
     command left by closing or retiring a nested instance that it makes again by
     `register-replacement.v1` under a new name (row 20). On line 234, "because its resolved
     operations narrow" becomes "because its access narrows as row 21 says", so that a Conduit
     narrowed only by an added requirement or a shrunk bound is lifted with no flag too.
   - **Controls** (lines 291 and 292). Keep both sentences, then add two: an author's edit to a
     setting is a replacement under `--replace`, and while the instance is ready its stop names a
     control that covers the setting (rows 23 and 25); whether apply may send the control instead
     is the owner's, asked with finding 4-01 after the S1 rerun (decision D67). Link D67 to the
     section "The experiments' documents and the register" of the decisions file, as the
     reference links D26.

2. **Stop lines have one form.** Source: FR-293 ("refuse with a named reason") and the
   explicitness ruling 52 asks of every refusal. In "Text grammar and statuses", after line 429
   ("The last line is ...") and before the table of classes, add the form below in a `text`
   block, and the rules after it:

   ```text
   stop SUBJECT CAUSE... [requires=REMEDY] PROMISE
   ```

   - SUBJECT is `KIND/NAME` for an object, `Package/ADDRESS` for a package, or `Manifest/-` for
     the project's manifest.
   - CAUSE is one or more `key=value` items, as the row of the difference table gives them. The
     keys are `place`, `setting`, `pin`, `control`, `tombstone`, `id`, `revision`, `absent`,
     `state`, `unreconciled`, `made-by`, `changed-by`, `installed`, `archive`, `version`, `digest`
     and `rule`.
   - `requires=` gives the flag and its operand that would permit the step, or `conduit-install`;
     it is absent where nothing would.
   - PROMISE is the promise that decides, or `-` where none does.
   - A plan prints one stop line for each cause. Stop lines follow the steps and their `detail`
     lines and come before `affects`, sorted by subject and then by the whole line, in byte
     order. The `difference` lines of rows 5 and 23 take the same form, with `difference` in
     place of `stop` and no `requires=`; the retained-pin line of row 19 and case 07's
     `remains ready` line keep their present form.
   - A stopped plan has one form, case 09's: `plan`, its steps and their `detail` lines, its stop
     lines, any `affects` and `difference` lines, its `flags` line, then `stopped`. A stop of row 4
     still prints no step, `detail` or `difference` line, but prints `flags []` before `stopped`:
     add that line where step 1 (lines 56 and 57), safety case 15 (line 554) and O5 (line 587)
     give that stop's lines. This settles the two stopped forms that order 154's Record left.
   - Case 09's stop line and O5's stop lines already have this form, and no case has a row 4
     stop, so no case changes for it.

3. **Package contents by digest: O7 is decided.** Source: FR-294, with its verifier's
   counterargument (a package fetched by a provider, or installed elsewhere, is a refusal with
   the `conduit install` remedy; dependencies need the same lookup). The design is the
   coordinator's: the cache is tooling, and `conduit install` is not an engine operation
   (`spec/README.md`, lines 119 and 120).
   - **O7** (lines 595 to 604) begins "O7: Decided." and says, in place of its text:
     - The planner reads package contents from the person's package cache, keyed by the
       archive's digest, as Go's module cache, Terraform's provider cache and npm's cache are.
       This needs no engine operation and discloses nothing new: the pin already names the bytes
       (PKG-004).
     - `conduit install ARCHIVE#HEX` keeps a copy of the archive in the cache under HEX once the
       bytes hash to HEX, whatever `install.v1` then answers: a copy named by its digest is never
       wrong. Nothing else writes the cache; plan and apply only read it.
     - The cache belongs to the person on that machine and lies outside every project and its
       `.conduit/`, so every working copy shares it. Order 052a fixes its directory with O8's
       context data. It holds archives only: no credential, and nothing from the engine's store.
     - The planner reads the archive of each package the manifest pins, of each package that an
       archive's manifest lists, through every dependency, and of each release an object holds
       that the comparison of row 19 needs. It hashes the bytes it read and uses them only where
       the hash is the digest. Rows 1 and 2 of the difference table give the stops.
     - The planner never fetches: not a pin's `fetch`, not through a package-source provider, and
       not from the engine's store. A package that the person installed with `conduit install` on
       this machine after the cache existed, from any of their working copies, is in the cache,
       since every working copy shares it. A miss is only a package that no `conduit install` of
       theirs on this machine has kept: one installed from another machine, one a provider
       fetched, or one installed before the cache existed. Running `conduit install` with its
       archive fills the cache and changes nothing in the engine (PKG-027).
     - The core package has no archive (NAM-009): the planner uses the core contents of its own
       build, as the engine does. So a CLI of another version than the serving engine stops with
       status 2 before it sends any request, naming both versions; order 052a fixes how the CLI
       learns the serving engine's version, with O8's context data.
   - **Commands** (lines 35 to 37). The reference still defines no install command, apart from
     the copy `conduit install` keeps in the cache (O7). Plan reads installed release identities
     with `list.v1`, and each release's contents from the cache. The sentence that installation
     alone does not supply that input goes. In "S1 commands and counts under ruling 76", line 616
     ("their behavior is outside this reference") gains the same exception.
   - **Step 1** (lines 48 to 50). Resolution reads the contents of each pinned release from the
     cache by digest (O7); list and inspect do not supply them. "Their CLI-side source is
     unspecified" goes.
   - **Step 3** (lines 73 to 77). Comparisons that need package contents read them from the cache
     (O7), in place of "are conditional on that input". A package not installed, or an archive not
     in the cache, stops the plan with status 2, as rows 1 and 2 say, in place of the sentence
     that begins "If required contents are unavailable".
   - **Contents needed** (line 217): the comparison reads both releases' archives from the cache
     (O7).
   - **The paper cases** (lines 484 to 489). All nine cases assume the person's package cache
     holds the archive of every listed address, version and digest, put there by the install
     commands under "S1 commands and counts under ruling 76" (T1, T2 and T5); the core package
     is built in. Schemas, defaults, operation contracts and implementation maps come from those
     archives. The compact `engine` notation lists identities only, and the cache is the
     planner's other input (O7). The words "separately supplied" go.

4. **Templates first.** Source: FR-293 rule 3, as its verifier limited it: only a template can be
   sent before what it replaces, since OBJ-002 holds an instance's name until the old one is
   removed.
   - **Ordering and access.** After "Register inert documents between these phases where
     dependencies need them." (line 148), add: where a plan closes or retires anything, it first
     sends every template step, before the first close or retirement: `replace.v1` for each
     template it replaces, then, where it adds templates, one `register.v1` holding them. A
     template step changes no access and no existing instance (INS-020), so a template the engine
     rejects stops the plan before anything is closed or retired. Cases 05 and 06 replace
     `shared--t` at step 1 and retire `shared` at step 2.
   - **Cases 05 and 06**, and only these. In `recorded.text` and `printed`, the step
     `management.replace.v1 Resource/shared--t` and its two `detail` lines become step 1, and
     `team.retire.v1 Resource/shared` becomes step 2; no other plan line changes. In `printed`,
     the result lines of the two steps change places and numbers. In `requests`, the two entries
     change places whole, and each key is `plan:HEX:1:N` with N its step: only the first two keys
     change their step number, so do not replace `:1:1` as plain text, which also matches `:1:10`
     and `:1:11`.
   - **The digests.** Re-derive each with `hashlib.sha256` over the UTF-8 bytes of
     `recorded.text`. Then, with a script you run and do not commit, replace each old hex wherever
     it occurs in cases 05 to 09: `recorded.path`, `recorded.digest`, request keys, `kept`, and the
     later cases' `applied` and `kept`. The old hex are
     `6040897a880f758ed49031be6fd2b6cfbd7378316e02febc87b819b423af5bd2` (05) and
     `a5a8504c38faea7f70985cab32743458e1065f03534d600437a4878e58c33ade` (06). Edit the files as
     text, not by loading and dumping YAML, so that no other line changes.
   - **The counts paragraph** (lines 691 to 697). Sending templates first changed the plan text of
     cases 05 and 06, and their digests were re-derived the same way; cases 02 and 03 keep their
     plan digests. The counts table does not change: no case gains or loses a step or a line.

5. **All or nothing stays.** Source: FR-293 rule 4, which its verifier rejected: sending the
   narrowings of a refused plan departs from Terraform and makes the rest another plan. The
   verifier's cheaper alternative is taken instead.
   - **Ordering and access** (lines 157 to 161) keeps its choice, and adds after line 161: it holds
     for every stop the difference table gives, and a narrowing so waits only until the command is
     run again, with the missing flag or without the change that needs it.
   - **Step 5** (line 80): if stopped, apply prints `not sent 1..N` when the plan has N steps, then
     exits without recording or sending anything, so its output says that no step was sent,
     narrowing included. A stopped plan with no step prints no such line, so no case changes:
     case 09 has no step.
   - **Stops, unknown outcomes and repeats** adds, at the end of its first paragraph (line 344):
     the difference table makes planning total, not applying atomic. A step can still fail after
     earlier steps succeeded; apply then stops, prints `not sent N..M`, exits 1 and rolls nothing
     back, and the next plan starts from what the engine shows. Leave the section's text on
     unknown outcomes and reconciliation as it is: [ruling 78](../proposals/ruling-78/README.md),
     points 5 and 6, changes those promises through a later specification order.

## Scope

Change only `docs/implementation/plan-and-apply/README.md` and, as decision 4 says, the case files
`05-t5.yaml`, `06-t6.yaml`, `07-t7-revoke.yaml`, `08-t7-restore.yaml` and
`09-t7-restore-unflagged.yaml` in `docs/implementation/plan-and-apply/cases/`. In the cases,
change nothing but decision 4's step order in 05 and 06 and the two old digests. Do not change
`spec/`, `engine/`, `conformance/`, `experiments/`, any work order, ruling record, `IN_FORCE.md`,
the ledger or the full review's files. Report, without changing them, other places that call O7
open or cite the reference's line numbers.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

This prints nothing, so every promise ID the reference and its cases cite exists in `spec/`:

```bash
for id in $(grep -rhoE '\b[A-Z]{3,5}-[0-9]{3}\b' docs/implementation/plan-and-apply | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
```

The check below runs with `/usr/bin/python3` and PyYAML. It compares each case with `main` at
`632881df`: no case's command, files, engine state, exit status or sources changes; only cases 05
and 06 change their plan, and only as decision 4 says; every digest, plan path, key and link of
the chain holds. It also checks that the reference's S1 commands and both counts tables are
unchanged, that the difference table has rows 1 to 27 and its last row stops naming a place, with
`rule=none` and no `requires=`, that O7 is decided, and that the old phrases are gone. Run as
given, it must print `9 cases; ok` and exit 0:

```bash
/usr/bin/python3 - <<'EOF'
import hashlib, json, re, subprocess, sys, yaml
at = sys.argv[1] if len(sys.argv) > 1 else None
d = "docs/implementation/plan-and-apply/"
def read(path, rev):
    if rev is None:
        return open(path, encoding="utf-8").read()
    return subprocess.run(["git", "show", rev + ":" + path], capture_output=True, text=True, check=True).stdout
seq = ["01-t1", "02-t2", "03-t3-noop", "04-t4", "05-t5", "06-t6", "07-t7-revoke",
       "09-t7-restore-unflagged", "08-t7-restore"]
old = {s: yaml.safe_load(read(d + "cases/" + s + ".yaml", "632881df")) for s in seq}
new = {s: yaml.safe_load(read(d + "cases/" + s + ".yaml", at)) for s in seq}
moved = {"05-t5": "4322e82635da43bfafe2f1ab371dbcfc6e5e8a610fb2ea8bea30b0f90cded1d8",
         "06-t6": "351bea0d6d4ce16f63560186537f7b701426ec25dd92cfccbf8eac63c0efc38f"}
swap = {old[s]["recorded"]["digest"][7:]: h for s, h in moved.items()}
def renamed(value):
    text = json.dumps(value)
    for a, b in swap.items():
        text = text.replace(a, b)
    return json.loads(text)
def results(x):
    return x["printed"][len(x["recorded"]["text"]):].splitlines()
bad = []
for s in seq:
    o, n = old[s], new[s]
    bad += [s + ": " + k for k in ("id", "area", "command", "files", "engine", "exit", "sources") if o[k] != n[k]]
    bad += [s + ": " + k for k in ("applied", "kept") if renamed(o[k]) != n[k]]
    t = n["recorded"]["text"]
    h = hashlib.sha256(t.encode()).hexdigest()
    if n["recorded"]["digest"] != "sha256:" + h or n["recorded"]["path"] not in (None, ".conduit/plans/" + h + ".txt"):
        bad.append(s + ": digest")
    if [q["idempotencyKey"] for q in n["requests"]] != ["plan:%s:1:%d" % (h, i + 1) for i in range(len(n["requests"]))]:
        bad.append(s + ": keys")
    if s in moved:
        a = o["recorded"]["text"].splitlines()
        if t.splitlines() != [a[0], "1" + a[2][1:], a[3], a[4], "2" + a[1][1:]] + a[5:] or h != moved[s]:
            bad.append(s + ": templates first")
        q = [dict(x, idempotencyKey=None) for x in o["requests"]]
        q[0], q[1] = q[1], q[0]
        if [dict(x, idempotencyKey=None) for x in n["requests"]] != q:
            bad.append(s + ": requests")
        r = results(o)
        if n["printed"] != t + "\n".join(["result 1" + r[1][8:], "result 2" + r[0][8:]] + r[2:]) + "\n":
            bad.append(s + ": printed")
    elif (t, n["requests"], n["printed"]) != (o["recorded"]["text"], o["requests"], o["printed"]):
        bad.append(s + ": plan changed")
for a, b in zip(seq, seq[1:]):
    if new[a]["kept"] != new[b]["applied"]:
        bad.append(a + " -> " + b + ": chain")
text, base = read(d + "README.md", at), read(d + "README.md", "632881df")
flat = " ".join(text.split())
part = text.split("\n## The difference table\n", 1)
rows = [l for l in part[-1].split("\n## ", 1)[0].splitlines() if re.match(r"\| \d+ \|", l)] if len(part) == 2 else []
if (not rows or [int(l.split("|")[1]) for l in rows] != list(range(1, 28)) or "rule=none" not in rows[-1]
        or "place=" not in rows[-1] or "requires=" in rows[-1]):
    bad.append("README: the table's last row")
if "O7: Open" in flat or "O7: Decided" not in flat:
    bad.append("README: O7")
gone = ["Their CLI-side source is unspecified", "installation alone does not supply that input",
        "The admin must settle", "separately supplied", "flag=...", "It lifts no tombstone",
        "Cases 02, 03, 05 and 06 keep"]
need = ["stop SUBJECT CAUSE", "conduit-install", "Manifest/-", "changed-by=control", "control=OPERATION",
        "rule=none", "not sent 1..N", "D67", "no row applies to it"]
bad += ["README still says: " + g for g in gone if g in flat]
bad += ["README lacks: " + g for g in need if g not in flat]
fence = "`" * 3
block = lambda x: re.search(fence + r"text\nT1: .*?" + fence, x, re.S)
counts = lambda x: [l for l in x.splitlines() if re.match(r"\| (T\d|Total|\[0)", l)]
if block(text) is None or block(text).group(0) != block(base).group(0) or counts(text) != counts(base):
    bad.append("README: S1 commands or counts changed")
print(len(new), "cases;", bad or "ok")
raise SystemExit(bool(bad))
EOF
```

Run it again with `632881df` after the `-`, so that it reads `main` in place of your tree: it must
exit 1, listing among others `05-t5: templates first`, `README: the table's last row` and
`README: O7`. That is the check failing with the fix removed. With decision 4 done as given, the
new digests are `sha256:4322e82635da43bfafe2f1ab371dbcfc6e5e8a610fb2ea8bea30b0f90cded1d8` (05)
and `sha256:351bea0d6d4ce16f63560186537f7b701426ec25dd92cfccbf8eac63c0efc38f` (06); if yours
differ, report why.

Do not stage anything. In your report, give each choice you made yourself, each wording you
tightened in the table, the two new digests and the tool that computed them, the output of
`git diff --stat`, every check's last line, and, without fixing it, any other place where the
reference contradicts `spec/` or the table. The admin then runs these checks itself, and a fresh
Codex verifier and a fresh Opus reader read the diff before it merges (ruling 79).

## Record

Run on 2026-11-15 on the Mac from `main` `71c725bc`, in two rounds: the worker (521 s) and a
fresh Codex verifier (356 s), then the worker again (100 s) and a second fresh Codex verifier
(338 s). The first verifier found one should: row 5's condition held only where a later row would
send a step, so the `difference` line that its action gave where no step would could never be
printed. The worker moved that limit from the condition into the action. The second verifier
found one must and one should. The must: read first-match, row 5 then decided every object with
an unreconciled invocation, so where no step would reach it, its `difference` line hid a later
row's stop, such as row 27's for a changed `runtime`, and the plan could read as no changes. The
should: O7 said that every working copy shares the person's cache, then that a package another
working copy installed is a miss.

The reference gained "The difference table": three values compared at each place, and 27 rows
read in order, the last a stop naming the field, with `rule=none` and no flag, for any
difference that no other row names. Stop lines have one form,
`stop SUBJECT CAUSE... [requires=REMEDY] PROMISE`, and a stopped plan has case 09's, with
`flags []` after a secret-reference stop. O7 is decided: the planner reads package contents from
the person's cache of archives keyed by digest, which only `conduit install` writes, and a miss
stops with the `conduit install` remedy. A plan sends its template steps before any close or
retirement, which swaps the first two steps of cases 05 and 06; their plan digests are those
decision 4 gives, computed with `hashlib.sha256` over `recorded.text` and carried through cases
05 to 09. A stopped apply prints `not sent 1..N`, and all-or-nothing holds for every stop the
table gives. An author's edit to a setting is a replacement, whose stop names a control that
covers it while the instance is ready (decision D67).

The coordinator finished it after the two rounds. Both of the second verifier's findings came
from this order's own text, so the coordinator fixed them in the reference and amended decisions
1 and 3 to match. Row 5 now decides only where a step of this plan would reach the object, and
prints beside its stop every stop that the rows below give the object. Otherwise the first row
below whose condition holds decides, and the plan prints the cause as a `difference` line beside
that row's decision; the table's opening says that row 5 alone looks ahead. O7 now says that a
package the person installed with `conduit install` on that machine, from any of their working
copies, is in the cache, and that a miss is only one installed from another machine, one a
provider fetched, or one installed before the cache existed. No case holds an unreconciled
invocation or a setting reported unknown (every case's `engine.unknown` is empty, and every
result the cases print succeeded), so row 5 decides no case and no case changed for it. The
worker reported, and left for later orders, that other documents still call O7 open or cite
the reference's line numbers.

The coordinator's second finishing pass applied the second reader's two musts and four shoulds
to both copies of the table, to the table's opening and decision 1, and to O7 and decision 3.
The musts: a retired instance that this working copy did not retire stops, with
`stop KIND/NAME state=retired LIF-024` and no flag in row 7, whose condition now names it, and
with `state=STATE` in place of `place=` in row 27 for a lifecycle state that no row above
decides; and objects that another object's row acts on have no row of their own, and inline
templates of a removed instance are unregistered under the same flag (the opening, rows 7, 14
and 16, where an orphaned `N--t` takes `--prune N`). The shoulds: row 5 also looks ahead to a
step that the rows below would give under the flag one of their stops requires; row 2 leaves
out the core package (NAM-009); O7 puts in the cache only what `conduit install` kept after the
cache existed; and rows 9 and 25 settle the worker's two findings, so a new template is only
registered, and a setting with no covering control, or on an instance that is not ready, stops
with no `control=` item and `--replace NAME` as its only remedy. The table keeps 27 rows, and no
case changed: every object in the cases' engines is ready or a template, no Pool or Fleet
leaves the file, the templates of cases 01 and 02 are only registered, and the setting change
of case 06 carries `--replace shared`.

A third fresh Opus reader read the second pass. It found one must, in the general sentence the
second pass had added to the table's opening: as written, an incident Conduit that a replacement
makes again lost its own row, so a replacement could take over a Conduit another actor changed.
The coordinator replaced that sentence with the reader's narrower text, in both copies, and dropped
row 16's `--prune N` for an orphaned inline template, whose operand could not resolve. Three edge
cases it found are left for order 052a, which checks this table by convergence: a cascade's own
inline templates (`P--t`, and a member's `N--t` once its shutdown is confirmed later) are not
unregistered under the cascade's flag; an inline instance changed to a referenced one leaves its
`N--t` to row 27, so that change cannot be applied; and an instance this working copy pruned and
then added back to the file, with its shutdown unconfirmed, gets a stop whose remedy cannot succeed.
No case changed.
