# Work order 082 (S-MC7b): a record lost outside the engine, and installs of one address and version one at a time

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` each case you add or change, with the promises it
tests and whether it passes. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a credential.

## What and why

Model checking found two defects in the specification: one in its records, one in installing a
package. Work order 113 (MC7b) found them, in
[experiment 012](../../experiments/012-model-checking/README.md), and registered both before its
first run. Order 115 finished that model and kept both as they are. AGENTS.md says where such
findings go (lines 73 to 76): "A defect in the language: fix it in the language, not around it",
and a finding of the first build is fixed "in the specification with a vector (ruling 46, point
4)". This order:

- settles MC7b-001 in `spec/`. REC-003 forbids what the engine does, its own crash, restart,
  writes and rollback among them. A record lost or altered by a fault outside the engine is no
  breach of it, and a read reports such a loss as a gap where the engine can tell it (REC-006);
- fixes MC7b-002 in `spec/`: the engine decides requests to install one address and version one at
  a time;
- adds a vector for a fix where the conformance runner can express it.

It changes no engine code. Where only the model can show a fix, your report says so.

1. **MC7b-001: a record lost with no purge** (an ambiguity).
   - REC-003 says that a record is never "removed or altered by anything other than a purge".
   - REC-006 says what a read reports where records "cannot be read, or were lost".
   - MC7b kept every record in saved state and faulted only the reading of it, which satisfies
     both. It made no ruling on a record that its storage loses: "If 'were lost' instead admits
     deleting that saved record without a purge, it directly violates the other promise's
     prohibition."
   - MC7b's own crash erases nothing: "Crash/recovery pauses E and erases no saved state". Its R3
     "checks continuity between successive source snapshots, so a silent storage change across a
     crash cannot disappear between logged mutations".
2. **MC7b-002: two first installs of one address and version** (a gap).
   - Two requests to install one address and version, with different digests, arrive before
     either has installed its package.
   - Each is decided against an empty slot, and each fetches and checks its archive.
   - Both succeed, and the second replaces the first. No promise orders the two, and none decides
     the second again before its package is installed. The shortest trace has 12 actions.

Both are in experiment 012's README, in the sections "MC7b findings registered before execution"
(lines 5854 to 5866 on `main` at 1091a71) and "MC7b findings and shortest race witness" (lines
5968 to 5997). The crash is in "MC7b nodes, scripts and bounds" (line 5622), and R3's continuity
in "MC7b review additions, before their first run" (lines 5872 and 5873). Where a line has moved
in your tree, find the section by its heading.

Today's text, on `main` at 1091a71. `spec/` is as it was at e9141e6; order 041b, merged since,
changed `engine/` and the manifest, but no line cited here:

- REC-003 (`spec/conventions/records.yaml` lines 91 to 104) has the `then` "It is removed or
  altered by anything other than a purge", and the observable "The record can still be read after
  the engine has restarted and after the objects it names are unregistered or removed". REC-006
  (lines 152 to 161) has the `when` "Records that should exist cannot be read, or were lost", and
  the observable "A result that omits records says so. It is never presented as complete." The
  row for record in `spec/definitions.md` (line 370) says "A record is added and never altered,
  and only a purge removes one (REC-003)". No row defines a gap.
- The storage of records stays each engine's choice (`spec/README.md` lines 40 and 41, DL8), and
  record storage may become a provider (IN_FORCE.md line 239). No promise says what the engine
  owes when that storage fails, or which losses an engine can tell: DL8
  (`spec/implementation-defined.yaml` lines 64 to 70) says how records are delivered and read.
- REC-003's observable already holds the engine to its own crash. Order 070 (E1, not yet run)
  cites REC-003 as "a record is removed or altered only by a purge", which "The shell's crash
  tests guard" (its lines 47 to 49). The shell keeps its records in the same file as its own
  state (`engine/CONFORMANCE.md` line 56: "Physical records are now in the atomic State file").
- The runner can already express a loss. `restore-state` with `records_purged: true` "observes an
  empty record store left by an external purge; it is not a `purge.v1` implementation"
  (`conformance/README.md` lines 360 to 362). The runner itself adds a gap whose cause is
  `purged-observation`, listing each record's source and ID, and empties the records, before it
  calls the core's `recover` (`engine/crates/conformance/src/lib.rs` lines 573 to 575). The core
  then keeps that gap, and `export` answers `complete: false`. Only `requests-unknown-write` uses
  that input, and it tests no REC promise. `records-unreadable` shows a record that cannot be
  read, with a gap of cause `unreadable` that names its source and ID.
- The core also keeps a gap for a purge (`engine/crates/core/src/records.rs` line 471, cause
  `purged`). `records-purge` and `records-purge-succeeded` expect `complete: false` after a purge,
  the manifest lists both under REC-006, and the first-build profile calls this "all known source
  gaps shown conservatively" (`conformance/profiles/first-build.yaml` line 37). MC7b read a purged
  record as no gap (order 113, line 95). This order does not settle that question (step 1).
- PKG-025 and PKG-027 (`spec/promises/packages.yaml` lines 297 to 313 and 333 to 353) say that a
  request "is decided from what is installed", and so do `install.v1`'s description
  (`spec/core/operations.yaml` lines 578 to 595) and "Installing" in `spec/core/README.md` (lines
  277 and 278). No text says what is installed while another install of that address and version
  is under way. PKG-024 (lines 283 to 295) and PKG-026 (lines 317 to 331) are about a package not
  yet installed.
- An install's decision already includes its fetch. `fetch-package.v1` is made "while the engine
  decides a request to register or to install, before it is dispatched" (PRV-005,
  `spec/promises/providers.yaml` lines 146 to 148; PRV-017, line 525).
- REV-014 (`spec/conventions/revisions.yaml` lines 225 to 259) is the form that the specification
  already uses for two requests on one instance: the later "is not dispatched, and reports
  conflict at once", and "The engine neither queues the request nor waits for the other to end".
  Step 2 takes another form for installs, and says why.
- A decision may already wait. DL4 (`spec/implementation-defined.yaml` lines 32 to 36) leaves to
  the engine whether a request whose access decision cannot be made "is not dispatched at once,
  reporting unavailable, or waits, and for how long".
- Three texts say that nothing is queued, and each is about sign-in or a held request:
  `spec/README.md` line 405, the row for sign-in in `spec/definitions.md` (line 270), and PND-005
  (`spec/promises/pending.yaml` line 150).
- The core decides each install in one transition, from an archive whose fetch has completed
  (`engine/crates/core/src/archives.rs`, `pub(crate) fn install(` from line 90, its
  `"already-installed"` decision at line 152), so it never has two installs under way. The shell
  reads an archive before it submits it (`engine/crates/shell/src/boundary.rs`,
  `install_arguments` from line 114 and `install_package` from line 142).

Steps of a case are zero-based in this order, as in a JSON Pointer `/steps/N`.
`tools/vector_delta.py` and the runner's errors print the same step as N + 1.

**Who decided what.** Each fix below is the coordinator's settlement of a defect. It is made under
ruling 46 point 4 and ruling 71, as [IN_FORCE.md](../../IN_FORCE.md) topic 15, "Who decides",
states (lines 301 to 303). The admin logs each one for the owner to strike. Apply each as written,
and report it under `decisions` as coordinator's. You write no record.

**What stops a fix.** A fix that would change what Conduit is for, or add or remove a feature, is
the owner's to rule on (IN_FORCE.md line 302). If you find that a fix as written here would do so,
do not write it. Leave its text as it was, finish the other fix, and report under `findings` what
it would add or remove, and why. For this order, each of these counts as such a change:

- a new operation, input, output, key or message (ruling 40). A key or a value of a gap's shape is
  one: how a gap in the records is shown is settled with the vectors (`spec/core/README.md` lines
  668 and 669);
- a requirement on how an engine stores its records, or on how it tells that some were lost: the
  storage of records stays each engine's choice (`spec/README.md` lines 40 and 41);
- a requirement that an engine tell a loss or an alteration of records outside it, or make one
  evident, such as by a digest or a signature over its records;
- a promise on what the engine owes when the storage of its own state fails: the state of
  invocations (OPS-008), tombstones (LIF-040), provenance (ACC-044) or installed packages;
- a change to who may purge, or to what a purge removes or records (REC-004);
- a lock, queue or lease that an actor can see, name or hold; an order between requests for
  different addresses or versions; or a new reason for a request to install not to be dispatched;
- a change to what S1's author writes.

The same line makes one exception: "a small finding that adds a feature, which a coordinator
settles and logs for the owner to strike". Step 2 adds an obligation of the engine. Should you read
it as a feature, the admin settles it under that exception, so it is no reason to stop. The admin
has judged that steps 1 and 2, as written, are none of the changes listed above:

- step 1 narrows REC-003 and REC-006 to what an engine can keep. It removes no feature: no engine
  can keep a record from a fault outside it, or report a loss that it cannot tell. What it leaves
  goes to the owner as findings;
- DL8's new clause has each engine document which losses it can tell, as every DL item has it
  document a choice. It requires no means.

**Orders in flight.** Line numbers in this order are those of `main` at 1091a71. Where a line has
moved in your tree, find it by the words quoted.

- **Order 038c** (specification, being run, not yet merged) writes the return of a browser login.
  Both orders edit these places:
  - `spec/README.md`: 038c changes the row of `promises/packages.yaml` (line 207: its count of
    slices), the same line as this order's count of promises, and the totals of promises at lines
    251 and 252, since it adds a promise too;
  - `spec/core/operations.yaml`: 038c adds the input `confirmClientIds` and the output `logins` to
    `install.v1`, and a paragraph at the end of its description, after "is the value an
    installation of that item gives here" (line 595);
  - `spec/conventions/records.yaml`: 038c adds three lines at the end of REC-002's observable,
    just above REC-003;
  - `spec/definitions.md` and `spec/core/README.md`, at other lines.

  Through CTR-013, 038c gives a request to install new answers: conflict for a client ID shared
  with a Type at another address, refused for `confirmClientIds` from a caller other than root,
  and invalid for a confirmation of a client ID that is shared with no such Type (its step 2, "A
  client ID two Types claim", lines 163 to 175). So write the new promise's answers generically
  (step 2). Keep each edit in these files to the lines it needs, edit no line of REC-002, and
  this order does not wait for 038c.
- **Order 077** (specification, not yet run) writes ruling 75 A1's fence, and changes the core.
  Both orders edit these files:
  - `spec/definitions.md`: 077 edits the rows for reconcile, attempt, the conflict row, plain,
    diagnostic, target, manifest and core package, among them the core terms at lines 51 and 52,
    and adds a row for fenced; this order edits the rows for record, purge and install, and adds
    a row for gap;
  - the count of portable vectors in `spec/README.md` line 7, and `conformance/manifest.yaml`, if
    you add a case;
  - the safety catalogue's quotes: 077's are OPS rows, this order's REC-003's.

  077 also changes the core, whose `recover` transition `restore-state` calls, and its new cases
  use `restore-state`, though not `records_purged`. If 077 is in your tree, run step 1's case on
  it.
- **Order 081** (specification, not yet run) settles MC4-009, MC4-002, MC4-001, MC6-002 and a
  failed retire hook. Both orders edit:
  - `spec/core/operations.yaml`: 081 the descriptions of `run.v1`, `stop.v1`, `retire-hook.v1`,
    `inspect-run.v1`, `unregister.v1` and `register-replacement.v1`; this order `install.v1`'s;
  - `spec/definitions.md`: 081 the rows for run, runtime provider, shutdown, remove, declare lost
    and unregister;
  - `spec/README.md`: 081 the limit that begins "A run left on a machine whose node was declared
    lost", and line 7; this order line 7, the row of `promises/packages.yaml` in "Contents" and
    the totals at lines 251 and 252;
  - `conformance/manifest.yaml` and the catalogue's quotes.

  081 adds one new promise at level `must not`, and this order one at level `must`. Each may
  change the catalogue's count of 217 (IN_FORCE.md line 262): report yours, as 081 reports its
  own. 081 narrows LIF-019, which DL8 cites; change neither citation. Keep each edit in the shared
  files to the lines it needs, so that whichever order merges second rebases easily. This order
  does not wait for 077 or 081.
- **Order 038b's staged patches.** `docs/work-orders/038b-staged/049b.patch` edits
  `spec/core/operations.yaml` and `spec/core/README.md`. Check at the end that the patches still
  apply (see "Done when").
- **Orders 113, 115 and 116** have merged. 113 and 115 changed only
  `experiments/012-model-checking/`. Order 115's R4 reads REC-006 as allowing "an unreadable or
  lost record, reported as a gap", the reading that step 1 writes, within what the engine can
  tell. Order 116 wrote G5's account of the safety promises
  (`experiments/012-model-checking/G5.md`), which lists MC7b-001 and MC7b-002 as this order's.
  If your tree has no `G5.md`, say so. All three are for step 5's report only.
- **Order 070 (E1)** changes how the shell commits its state. It cites REC-003 as "a record is
  removed or altered only by a purge", which its crash tests guard. Step 1 keeps that for what the
  engine does, its own crash included.

**Size: S.** The text for two findings, one new promise and at most one case.

Read first:

- AGENTS.md lines 73 to 76;
- in IN_FORCE.md:
  - topic 15's "Who decides" (lines 301 to 306);
  - line 18: the language declares what surrounds an agent, and "who answers for what it did";
  - line 22: "An unknown outcome stays unknown, and nothing runs a background reconciler or wakes
    on its own";
  - line 141, records and purging; line 87, installing; line 239, record storage as a provider;
  - line 262: the 217 safety promises of 75 E1;
- order 113's decision 5 (lines 83 to 121); order 115's first decision (lines 22 to 43); and in
  experiment 012's README these MC7b sections: "MC7b nodes, scripts and bounds", "MC7b catalogue
  accounting (20 promises)", "MC7b property text and preconditions", "MC7b findings registered
  before execution", "MC7b review additions, before their first run", "MC7b findings and shortest
  race witness" and "What MC7b establishes and leaves open";
- order 070's lines 47 to 49, and order 038c's step 2, "A client ID two Types claim" (lines 163 to
  175);
- the promises:
  - in `spec/conventions/records.yaml`: REC-001 to REC-006, and the head comment;
  - in `spec/promises/packages.yaml`: PKG-003 and PKG-021 to PKG-029;
  - in `spec/conventions/revisions.yaml`: REV-012 and REV-014;
  - OPS-007, OPS-008, OPS-009, OPS-012, OPS-018, OPS-023, OPS-032 and OPS-034
    (`spec/conventions/operations.yaml`), IMP-007, LIF-013, LIF-040, ACC-033, ACC-044, NAM-011,
    MAN-016, CTR-013, PRV-005, PRV-013, PRV-016, PRV-017, PLC-023, OBS-003 and PND-005;
  - DL2, DL3, DL4, DL7, DL8 and DL10 (`spec/implementation-defined.yaml`);
- in `spec/core/operations.yaml`, `install.v1` and `purge.v1`;
- in `spec/core/README.md`, "Installing" (lines 271 to 279), and lines 668 and 669 on how a gap is
  shown;
- in `spec/README.md`, lines 7, 40 and 41, the row of `promises/packages.yaml` in "Contents" (line
  207), the totals at lines 251 and 252, and line 405;
- in `spec/definitions.md`, the rows for record (lines 53 and 370), purge (371), source (369),
  install (108) and sign-in (270);
- [CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises";
- the safety catalogue's rows for REC-003, REC-006, PKG-024, PKG-025 and PKG-027
  (`docs/implementation/SAFETY_CATALOGUE.md` lines 170 to 173 and 542 to 545);
- these cases: `records-unreadable`, `records-purge` and `records-purge-succeeded` (under
  `conformance/conventions/records/`), `requests-unknown-write` (its `restore-state` step with
  `records_purged`), and `archives-s1`'s steps that cite PKG-025 and PKG-027;
- in `conformance/README.md`: "Assertions", "Reviewing vector changes", "Archive installation
  inputs", lines 358 to 363 on `restore-state`, and "I2-08: records, inspection and repair";
- in the engine, for your report only: `pub(crate) fn export(` and the purge's gap in
  `engine/crates/core/src/records.rs`; `pub(crate) fn install(` in `archives.rs`; the
  `"restore-state"` branch of `engine/crates/conformance/src/lib.rs`; the shell's
  `install_arguments` and `install_package`; and the rows DL4 and DL8 of `engine/CONFORMANCE.md`.

## What to do

1. **MC7b-001: what the engine does, and a fault outside it** (coordinator's settlement, for
   REC-003 and REC-006). Write this reading:
   - **The engine's acts.** REC-003 is a promise about what the engine does. Nothing that the
     engine does removes or alters a record, but a purge under REC-004. Its own crash and restart
     are among its acts, and so are every write it makes to the storage of its records, one that
     its crash interrupts included, and every compaction, copy, or rollback to an earlier copy,
     that it makes itself. A record that the engine loses or alters by any of these is removed or
     altered by the engine, which REC-003 forbids, as today. The record "can still be read after
     the engine has restarted" stays. So does MC7b's R3, which checks continuity across a crash,
     and the shell's crash tests that order 070 cites.
   - **A fault outside the engine.** A fault of storage is one outside the engine: the medium
     fails; someone other than the engine restores the store from an older copy; or another
     process removes or changes the file that holds the records. A record that such a fault
     removes or alters is not removed or altered under REC-003. Nor is it purged: no record of a
     purge is kept for it, and nothing presents it as one.
   - **What a read reports.** REC-006 covers a loss that the engine can tell. A read that should
     return records that the engine can tell were lost says that there is a gap, and where, and
     is never presented as complete. A record that the engine finds damaged is one that cannot be
     read: it is reported as a gap, and never returned as the record. A record that could not be
     read and can be read again is returned, and no gap remains for it, as `records-unreadable`'s
     last step shows.
   - **What an engine can tell.** Which losses outside it an engine can tell is the engine's
     choice, which it documents under DL8. No means is required. A loss that the engine cannot
     tell is not reported: for example the whole store removed, or an older copy restored together
     with whatever the engine would use to tell. An alteration outside the engine that it does not
     find is read as the record now holds it. Both are limits, for the owner (see "Write nothing
     about").

   Why this reading, and not that a fault of storage breaches REC-003:
   - Storage fails on every machine. Under the other reading, every engine breaks a `must not` the
     first time a disk fails or another process changes a file, so the promise says what no
     engine can keep. REC-006's "or were lost" would then describe only a breach.
   - REC-006 read as covering every loss would move the same impossibility into it. No engine can
     report a loss that it cannot tell, and obliging every engine to tell every loss would be a
     requirement on the means, which "What stops a fix" leaves to the owner. So REC-006 covers
     what an engine can tell, and DL8 has each engine say what that is.
   - Under this reading both promises can be checked. MC7b checks REC-003 against the engine's own
     acts (its variant AlterRecord) and across its crash (R3's continuity), and REC-006 against a
     record it cannot read (HideGap).
   - What the reading leaves: a loss outside the engine that it cannot tell, and an alteration
     outside it that it does not find. REC-003's row in the catalogue names the harm that
     "altering an existing record can falsely attribute who performed an operation". Against a
     fault outside the engine, no promise then holds that harm off; today's text holds it off
     only by a promise that no engine can keep. Whether Conduit should require an engine to make
     such a loss or alteration evident is the owner's: it bears on "who answers for what it did"
     (IN_FORCE.md line 18), and it is a requirement on the means.
   - The specification treats other faults that it cannot prevent in the same way: what cannot be
     observed now is reported unknown (OBS-003, PLC-023), never as observed.
   - The storage of records is each engine's choice, and may become a provider (`spec/README.md`
     lines 40 and 41; IN_FORCE.md line 239). A loss is then that storage's fault, and the same
     reading holds.
   - It is the practice of AWS CloudTrail, which `records.yaml`'s head comment follows. Its log
     file integrity validation tells whether a log file was changed or deleted after delivery; it
     does not claim that none can be. Conduit requires no such validation, which is the owner's
     question above.

   Write it in these places:
   - REC-003's `then`: narrow it to what the engine does, at level `must not`. Keep its `when`. In
     its observable, beside "The record can still be read after the engine has restarted", say
     that the engine's own crash, restart, writes and rollback are its acts; that a record lost or
     altered by a fault outside the engine is not removed or altered under this promise; and that
     REC-006 says what a read then reports.
   - REC-006's `when`: records that should exist cannot be read, or were lost and the engine can
     tell that they were; records lost by a fault outside the engine among them, citing REC-003.
     Keep its `then`. Make its observable agree: a result that omits such records says so. Keep
     the sentence "It is never presented as complete." word for word, since the catalogue quotes
     it. Add that a record found damaged counts as one that cannot be read; that a loss is not
     presented as a purge (REC-004); and that which losses an engine can tell is DL8.
   - `spec/definitions.md`:
     - the row for record (line 370), at "A record is added and never altered, and only a purge
       removes one (REC-003)": say it of what the engine does;
     - the row for purge (line 371): a purge is the only way the engine removes records;
     - a new row for gap, beside record in "Inspection, observation and records", pointing to
       REC-006 and REC-003: where a read cannot return records that should exist, because they
       cannot be read or the engine can tell that they were lost. Write it so that it neither
       counts nor excludes the records that a purge removed (see below).
     - Leave the core term record (line 53) unless the new text requires a change there: order
       077 edits lines 51 and 52 beside it. Check the head comment of `records.yaml` (line 4, "A
       record is added and never altered").
   - DL8 (`spec/implementation-defined.yaml` lines 64 to 70): add one clause, which losses of
     records outside the engine it can tell, under REC-006. Name no means.

   Use "lost" for records only where the words cannot be read as a node declared lost.

   Write nothing about these. Report each under `findings`:
   - for the owner: what the engine owes when the storage of its own state fails: the state of
     invocations (OPS-008), tombstones, provenance or installed packages. OPS-018 and REC-004 rely
     on that state, and no promise says what is owed once it is lost;
   - for the owner: the two limits above, a loss outside the engine that it cannot tell and an
     alteration outside it that it does not find, and whether an engine must make either evident;
   - for the owner: records that a purge removed and that come back with a copy restored from
     before the purge, whose own record of the purge is then lost. Whether a read may return them
     is a question of what a purge removes (REC-004);
   - for the admin: whether a read also shows as a gap the records that a purge removed. The core
     does today; `records-purge` and `records-purge-succeeded` expect it, under REC-006; MC7b read
     a purged record as no gap. Change no case for it;
   - for the admin: whether anything should bar the engine from making a record again in place of
     one that was lost. MC7b-001 does not need it.

   Then search `spec/` for "never altered", "only a purge", "can still be read", "were lost",
   "gap" and "purge", and, where they are said of a record, "is kept" and "can be read". Change
   only text that states as a fact about storage what is a promise about the engine, or that
   claims a report of a loss that the engine cannot tell. List each place you changed, and each
   place you checked and left.

   **Vectors.** The runner expresses a loss that the engine is told of with `restore-state` and
   `records_purged: true`. Add this case:
   - ID `records-lost`, in `conformance/conventions/records/lost/`, built from the files and the
     first step of `records-unreadable`, whose fixtures it names by relative path;
   - `/steps/0`: `records-unreadable`'s `/steps/0`, unchanged;
   - `/steps/1`: `restore-state` with `records_purged: true`. Assert that the reply is
     `{"restored": true}` and that `external_intents` is empty, and nothing under `state`: above
     all not `state.records: []`, which `requests-unknown-write` asserts, since an engine may keep
     a record of the loss. If the runner requires more at this step, assert only what no reading
     of the new text changes, and report it;
   - `/steps/2`: `export-records`. The reply is not complete (`complete: false`), and nothing is
     dispatched.

   Expect no `gaps` array and no list of records at `/steps/2`. No promise fixes how a gap is
   shown or how many gaps one loss makes, and an engine may keep a record of the loss. List only
   REC-006 in the case's `promises`: the case shows a loss outside the engine, which REC-003 no
   longer governs. Give it a title that calls the loss a loss, not a purge. If the core fails the
   case, report the failure and leave the case out: this order changes no engine code.

   Change no other case. Report under `findings`, for a build order:
   - that the input `records_purged`, its description in `conformance/README.md` lines 360 to 362,
     the runner's gap cause `purged-observation` and `requests-unknown-write`'s title ("or records
     purge") call a loss a purge;
   - that the runner gives a lost record's gap another shape than the core gives an unreadable
     record's: a `records` list of source and ID pairs, against `source` and `ids`. So the shape
     that `spec/core/README.md` lines 668 and 669 leave to the vectors is not yet one shape;
   - which losses the shell can tell, for example where the file that holds its records and its
     state was removed while it was stopped, or restored from an older copy; and that the row
     DL8 of `engine/CONFORMANCE.md` must then say so.

2. **MC7b-002: installs of one address and version, decided one at a time** (coordinator's
   settlement, for PKG-025 and PKG-027). Write this rule.
   - The engine decides requests to install one address and version one at a time. It begins to
     decide such a request only once every request to install that address and version that it
     began to decide before has ended, and decides it from what is installed then. A repeat
     answered without a dispatch (OPS-009, OPS-012, OPS-032, OPS-034) is answered as those
     promises say, and waits for nothing.
   - No order is promised between requests that arrive together. Each is answered as the same
     request would be if it had been made once every request that the engine began to decide
     before it had ended: by the install promises, and by every other promise under which a
     request to install is not dispatched, in OPS-023's order. For example:
     - where an earlier one installed the package with another digest: conflict under PKG-025,
       with nothing fetched;
     - where an earlier one installed it with the same digest: succeeded under PKG-027, changing
       nothing and fetching nothing;
     - where none installed it: decided as any request for a package not yet installed, under
       PKG-023, PKG-024 and PKG-026. A request with no `fetch` then reports invalid under
       PKG-023.

     Name no closed list of the other reasons, such as ACC-033, PKG-023 and PKG-029: order 038c
     adds others (see "Orders in flight").
   - So of requests that name different digests, at most one installs its package, and nothing is
     put in the place of an installed package. Requests for another version at that address
     (NAM-011), or for another address, are not held back. Two versions are the same as PKG-003
     states.

   Check IMP-007 for a request that is being decided, or waiting, when the engine restarts, and
   say whether the new text needs a word on it.

   Why this form, and not REV-014's, under which the later request reports conflict at once:
   - Under this form, the answer does not depend on timing. The install promises answer each
     request exactly as they answer the same requests made one after the other. Under REV-014's
     form, a later request that names the same digest would report conflict, where PKG-027 answers
     it succeeded.
   - An install's decision already includes its fetch, made before the request is dispatched: the
     engine's own, or the provider call `fetch-package.v1` (PRV-005, PRV-017). Nothing is
     dispatched to an implementation while it is under way, and the engine can end it itself: an
     archive that cannot be fetched now ends the request unavailable (PKG-024), with nothing
     installed and no outcome left unknown. REV-014 holds back setups, retirements and changes,
     which dispatch work to an implementation. No promise limits how long that work is in
     progress, and an invocation for which the engine stops waiting ends unknown (DL3, OPS-007).
   - A decision may already wait: DL4 leaves to the engine whether a request whose access decision
     cannot be made "waits, and for how long".
   - The later request waits as part of its own decision, as any request whose fetch takes time
     does. Nothing wakes on its own (IN_FORCE.md line 22), and no state outlives the request.
   - No two installs wait on each other. Each waits only for requests that the engine began to
     decide before it, and an install fetches and installs no other package (PKG-028).
   - Cargo does the same: it waits for the lock on its package cache ("Blocking waiting for file
     lock on package cache").

   If you find text that requires REV-014's form for installs, or that a request to install be
   decided at once, stop this step and report it. These texts are about sign-in and held requests,
   not installs, and do not count: `spec/README.md` line 405 ("A missing login ends the request
   that encountered it; nothing is queued"), the row for sign-in in `spec/definitions.md` (line
   270, "A held request ends; no work is queued") and PND-005 (`spec/promises/pending.yaml` line
   150, "Nothing is queued, resumed or dispatched after it ends").

   Write it as one new promise at the end of `spec/promises/packages.yaml`, with layer
   `convention`, level `must`, and the next number never used (PKG-032 on 1091a71;
   CONTRIBUTING.md rule 2). Its `when` is a request to install an address and version arriving
   while the engine is deciding another request to install that address and version; its
   `then`, that the engine decides them one at a time, as above, each from what is installed
   then. Its observable gives the answers above, citing each promise, says that no order is
   promised between requests that arrive together, and says what Cargo does.

   Then make these texts agree, each only where the new promise requires:
   - PKG-024, PKG-025 and PKG-027: cite the new promise in each observable. PKG-024's says
     "Nothing is fetched for an address and version that is already installed". Change the
     `then`s of PKG-025 and PKG-027, which say "It is decided from what is installed", only if
     they must, since the catalogue's rows quote them;
   - `install.v1`'s description, in one sentence. Put it before "A package is installed only by
     this operation" (line 593), not at the end, where order 038c adds a paragraph;
   - the row for install in `spec/definitions.md` (line 108);
   - "Installing" in `spec/core/README.md`, at "A request for an address and version that is
     already installed is decided from what is installed, and fetches nothing" (lines 277 and
     278);
   - in `spec/README.md`, the count of promises in the row of `promises/packages.yaml` (line 207,
     23 on 1091a71), and the totals at lines 251 and 252 ("515 in all", "242 of the layer
     `convention`"), recounted from the files.

   Then search `spec/` for "from what is installed", "already installed" and "queue". Change only
   text that the new promise makes wrong, and list each place you changed, and each place you
   checked and left.

   A new safety promise changes the catalogue's count of 217, which IN_FORCE.md line 262 cites.
   IN_FORCE.md line 87 names "PKG-001 to PKG-030", although PKG-031 already exists. Report both
   for the admin, and change no line of IN_FORCE.md.

   Report under `findings`:
   - whether the core and the shell meet the new promise. The admin expects the core to, since it
     decides each install in one transition. Say whether the shell reads or fetches a later
     request's archive before the core decides it, and so fetches for an address and version that
     may already be installed (PKG-025, PKG-027). Change no engine code;
   - for the admin: no promise or DL item bounds how long a fetch may take, so one slow fetch
     holds back every later request to install that address and version;
   - for the owner: the same check-then-act race across addresses, which this order leaves, since
     an order between requests for different addresses is the owner's. Two installs at different
     addresses, each decided before the other ends, can both pass CTR-013's client-ID check once
     order 038c writes it. An install at an address (PKG-029) and a registration of a project
     that states it (MAN-016), each decided before the other ends, can both succeed. Say for each
     what holds it off today, if anything.

   **Vectors: none, unless the runner can express two installs under way at once.** On 1091a71
   it cannot. `install-archive`, and an `install.v1` request with its archive, carry the fetch's
   completion in the same step, and the core decides each in one transition. `archives-s1` already
   shows two requests made one after the other. Add no runner input. Report under `findings` these
   cases, for the build order that lets a step start an install whose fetch completes in a later
   step:
   - two requests with different digests, the second arriving before the first's fetch completes:
     the first succeeds, and the second reports conflict under PKG-025 and fetches nothing;
   - the same with one digest: both succeed, and the second fetches nothing;
   - the first's fetch unavailable: the second is decided as a request for a package not yet
     installed;
   - three requests, the second and the third arriving while the first is fetching, and the
     first's fetch unavailable: the second is decided as a request for a package not yet
     installed, and the third only once the second has ended, from what the second installed.

   Report too that only the model can show this fix for now. MC7b's install part must be run again
   with the decisions ordered, with three requests, and with a property for the new promise.

3. **The vectors.** Write each expectation by hand from the promises, never by copying the
   engine's output, and declare every change.
   - **New case**: step 1's, `records-lost`.
   - **Changed cases.** Change no other case unless the new text requires it, and then only the
     steps it requires. Report each such case, with the promise.
   - **The coverage index.** Add the new case to `conformance/manifest.yaml`, or to its pieces if
     order 071 has landed in your tree, in the form of `records-unreadable`'s entry. Add it to the
     `s1_coverage` list of REC-006. In `spec/README.md` line 7, set the count of portable vectors
     to the number of entries in the manifest's `cases` in your finished tree. On 1091a71 the line
     says 413 and the manifest has 438 entries: order 041b added cases without the count.
   - **The declaration.** If you add or change a case, write `docs/work-orders/082-declared.yaml`,
     in the form of `docs/work-orders/041a-declared.yaml`. Give one shape for each intended change,
     none broader than its change:
     - `added`, for each new case;
     - `input`, for a change under a step's `input`, or to a case's `files` or `initial`;
     - `flip`, for a change to a step's `expect/accepted` or `expect/stateUnchanged`, or to
       `expect/reply`'s `reason`, `outcome`, `accepted` or `tool`;
     - `expectation`, for any other change under a step's `expect`;
     - `metadata`, for a change to a case's `promises`, `title` or other top-level field;
     - `fixture`, for a change to a file that a case names. Such a change is reported once for
       every existing case that names the file, so name `records-unreadable`'s fixtures without
       changing them.

     Run `python3 tools/vector_delta.py HEAD --declared docs/work-orders/082-declared.yaml`
     until it reports no undeclared change.

4. **The catalogue.** In `docs/implementation/SAFETY_CATALOGUE.md`, a promise you change may no
   longer hold the text that its row quotes. Then change that row's quote and keep its kind.
   REC-003's row quotes its `then`, which step 1 changes. REC-006's row quotes "It is never
   presented as complete", which step 1 keeps. Check the rows of PKG-024, PKG-025 and PKG-027 too.
   Add no row for the new promise: report it, and leave its classification and the counts to the
   admin.

5. **Before you finish, check**:
   - every request that is not dispatched names exactly one of refused, invalid, conflict and
     unavailable, in OPS-023's order (CONTRIBUTING.md, "Writing promises", rule 5);
   - MC7b-002's trace, walked against the new text. The admin expects it to stop at action 8,
     where the engine decides the second request while the first, whose fetch is under way, has
     not ended. Say what you find;
   - three requests for one address and version, the second and the third arriving while the
     first is fetching, and the first's fetch unavailable. The admin expects the second to be
     decided against the empty slot, and the third only once the second has ended: conflict
     under PKG-025 where the second installed another digest, succeeded under PKG-027 where it
     installed the same, and decided as a first install where it installed nothing. Say what the
     new text gives, and that no two of them wait on each other;
   - MC7b-001's projection, walked against the new text: a record at S that cannot be read is
     read with a gap, as before. Then the case the model left open, a record removed from S's
     storage by a fault outside the engine, with no purge: no breach of REC-003, and a gap under
     REC-006 where the engine can tell the loss. Then a record that the engine loses in its own
     crash: still a breach of REC-003. Name the sentences that now say each;
   - these cases. Say what the new text gives each:
     - a store restored by someone other than the engine from an older copy, which lacks the
       records made since. The admin expects a gap where those records were, and a read that is
       not complete, where the engine can tell; and nothing reported where the copy also restored
       whatever the engine would use to tell, which is the limit that DL8 documents;
     - the same, with a copy older than a purge: the records that the purge removed come back,
       and the purge's own record is lost. The admin expects the purge's record to be a loss like
       any other, and the new text to say nothing of the records that come back (step 1's
       finding for the owner);
     - a record changed by another process: a gap where the engine finds it damaged, and read as
       it now holds where the engine does not find it, which is the limit for the owner;
     - two requests for one address and version, the first of whose archives has another digest
       than the one its request names (PKG-026);
     - a later request with no `fetch`, made while the first is still fetching;
   - no text you wrote calls a loss a purge, names a means of telling that records were lost or
     altered, or claims that a loss the engine cannot tell is reported;
   - nothing wakes on its own; nothing new is an operation, input, output, key or message; and no
     text requires how an engine stores its records;
   - each promise you rewrote keeps everything that the change does not name, and every ID it
     cites exists;
   - what this order touches outside the scope, for your report only:
     - experiment 012. MC7b's install part lets both first installs succeed, and runs two
       requests. Its R3 stays as it is: the engine's own crash erases no saved state, and R3's
       continuity across a crash is REC-003 as narrowed. Its R6 models a read failure; a fault
       outside the engine is outside MC7b's faults;
     - order 116's account, which lists MC7b-001 and MC7b-002 as this order's;
     - the runner's `records_purged`, its gap cause, `requests-unknown-write`'s title and the
       first-build profile's line on gaps;
     - the shell's path to install a package, its crash tests that order 070 cites, and the rows
       DL4 and DL8 of `engine/CONFORMANCE.md`.

## Scope

Change:

- in `spec/`:
  - `spec/conventions/records.yaml`: REC-003 and REC-006, and its head comment only where step 1
    requires. No other promise, REC-002 above all;
  - `spec/promises/packages.yaml`: one new promise, and PKG-024, PKG-025 and PKG-027 only where
    step 2 requires;
  - `spec/core/operations.yaml`: `install.v1`'s description; `purge.v1`'s description only where
    step 1's search finds it wrong;
  - `spec/core/README.md`: "Installing", at lines 277 and 278;
  - `spec/definitions.md`: the rows that steps 1 and 2 name, and the new row for gap;
  - `spec/implementation-defined.yaml`: DL8, one clause, as step 1 says;
  - `spec/README.md`: the count of portable vectors at line 7 if you add a case, the count of
    promises in the row of `promises/packages.yaml`, and the totals at lines 251 and 252;
  - elsewhere in `spec/`, only text that the search of step 1 or of step 2 finds wrong, each place
    listed in your report;
- `conformance/conventions/records/lost/`: step 1's new case;
- `conformance/manifest.yaml`, or its pieces, for that case;
- `docs/work-orders/082-declared.yaml`, new, if you add or change a case;
- `docs/implementation/SAFETY_CATALOGUE.md`: the quotes that step 4 names.

Do not change:

- `engine/`, `engine/CONFORMANCE.md` among it: this order changes no engine code, and a case that
  the core fails is reported;
- `conformance/case.schema.json`, the runner and its inputs: `records_purged` keeps its name;
- any existing case, `records-purge`, `records-purge-succeeded` and `requests-unknown-write`
  among them, or `conformance/profiles/`;
- `spec/names.yaml`, `tools/` or `packages/`;
- `experiments/`: experiment 012's dated results stand;
- IN_FORCE.md: the admin logs each settlement;
- a record: the ledger, the rulings' records, `docs/AUDIT.md`, `ROADMAP.md` and `docs/proposals/`,
  `docs/proposals/first-build/` among them;
- the files of other work orders, `docs/work-orders/038b-staged/` among them.

Report anything else that needs changing.

## Done when

Every item of steps 1 to 5 is written, or stopped under "What stops a fix", or its absence is
reported with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD --declared docs/work-orders/082-declared.yaml`, with that
  interpreter, if you added or changed a case, and `python3 tools/vector_delta.py HEAD` if not.
  Its last line reads `Vector delta: N changes; F flips; 0 undeclared; 0 errors.`, with N 0 where
  you changed no case, and it exits 0 where F is 0. A declared flip still exits 1: list each flip
  with its path, its before and after, and the promise that requires it.
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

- the first shows that no line you wrote in a promise file has 97 characters or more;
- the second, that every promise ID that your changes to `spec/` cite exists;
- the last two cover the files that are not yet tracked, which `git diff --check` does not see.

The brackets keep this order free of the paths it looks for.

```bash
git diff -U0 -- spec/conventions spec/interface spec/promises | grep -E '^\+[^+]' | awk 'length > 97'
for id in $(git diff -U0 -- spec | grep -E '^\+[^+]' | grep -oE '\b[A-Z]{3,5}-[0-9]{3}\b' | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
grep -rn -E '[[:space:]]+$' conformance/conventions/records $(ls docs/work-orders/082-declared.yaml 2>/dev/null)
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/' conformance/conventions/records $(ls docs/work-orders/082-declared.yaml 2>/dev/null)
```

`git status --porcelain` lists only paths in the scope; report any other line, and why.

If `docs/work-orders/038b-staged/` is in your tree, check that its patches still apply:

1. Copy your finished tree, without `engine/target/`, to a directory outside the repository.
2. There, run `git apply --check` for each patch, in each of the four sequences of order 038b's
   step 5.
3. Report each command's last line, then delete the copy. Write its location in no file.

Change no patch. Report any that no longer applies, with the hunk that conflicts.

In `engine/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`. The tests read `spec/`, so run it once more after your last change to
  `spec/`.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers only.
If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load (finding
4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail in your
sandbox: say so, name each, and the admin reruns them.

Report:

- in `decisions`: each promise, contract, definition row and DL item you changed, with the finding
  behind it (MC7b-001 or MC7b-002), each marked coordinator's;
- under `own_choices`, every choice you made yourself, including:
  - the words of REC-003's narrowed `then`, of REC-006's `when`, and of DL8's clause;
  - the new promise's ID and words, and where else you cited it;
  - the new case's title, and what `/steps/1` asserts;
- under `findings`:
  - for the owner: the engine's own state when its storage fails; the two limits of step 1, and
    whether an engine must make a loss or an alteration outside it evident; purged records that a
    restored copy brings back; and the races across addresses of step 2;
  - for the admin: whether a purge's removal is a gap; whether anything should bar making a record
    again; and that no bound is set on how long a fetch may take;
  - the runner's and the core's words for a loss, the two shapes of a gap, and which losses the
    shell can tell;
  - whether the core and the shell meet the new promise;
  - that the runner cannot express two installs under way at once, and the cases for a build
    order;
  - that only the model can show step 2, and what MC7b then needs;
  - each fix you stopped under "What stops a fix";
  - the new promise's effect on the count of 217, and IN_FORCE.md line 87;
  - any case that the core failed, and any 038b patch that no longer applies;
  - what step 5 found outside the scope;
  - anything else that needs changing;
- in `vectors`, each case added or changed;
- each check's last line.

Do not stage anything.

## Record

Run on 2026-11-11 and 12 on the Windows computer: one worker round and a fresh verifier, who found
no fault. The worker applied all twelve of the coordinator's settlements as written: REC-003 and
REC-006 narrowed to what the engine does and can tell (MC7b-001), with DL8 documenting which
outside losses an engine tells, and PKG-032 deciding the installs of one address and version one
at a time (MC7b-002). It added the vector records-lost; all 439 portable vectors pass. The engine's
full suite did not finish clean in the Windows sandbox (both crash barriers, which main has since
given 600 seconds, the S1 HTTP test and a listener-readiness deadline, each a timing failure
outside this order's files); the admin ran the spec suites on the Mac before merging.

The worker's findings, kept for later work: for the owner, obligations when invocation state,
tombstones, provenance or installed packages cannot be stored; whether an undetected outside loss
or alteration must be made evident; whether records restored from a copy taken before a purge may
be returned; and races across addresses (client IDs, installation against registration). For the
coordinator, whether a purge leaves a gap, recreating a lost record, and a bound on a fetch's
duration. For the build, the shell's install paths, its loss detection under DL8, the runner's lack
of concurrent installs, and MC7b's rerun with PKG-032. PKG-032 is one more convention-layer safety
promise for G5 (218).
