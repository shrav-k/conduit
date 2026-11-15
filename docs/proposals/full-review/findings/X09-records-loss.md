# X09-records-loss: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** **The records promises are not yet enough for the purpose claim that someone answers for what an agent did.** They are enough for a narrower claim: while the store is intact and the objects or their tombstones remain, a reader can tell which instance asked which operation of which target, through which Conduit, serving which invocation, and how it ended. The caller is established per request (IFC-022), an onward request keeps its original caller, refusals are recorded with their rule, and only root's purge removes a record.

Answers to the questions put:

- **A loss the engine cannot tell** is a limit, not a broken promise, where its cause is outside the engine. REC-003, REC-006, DL8 and the limits section agree, and the owner reviewed it on 2026-11-12 (review item 29). The line between the engine's acts and outside faults is ambiguous in three reachable cases (F3).
- **A store that cannot be written** has no promise-keeping answer today, and DL10 and SEC-016 leave the choice to each engine (F1). The least text extends ACC-037 and SEC-016; the coordinator decides.
- **State lost or rolled back** is covered by no limit and no promise, and it fails open (F2).
- **Purge.** By the text a purged record is not a gap; two portable vectors say it is (F4). Records restored from a copy older than a purge are read as found, under the limit the owner kept. The limits section does not say so in words, and the lost purge record leaves root nothing to act on; F2's text names it.
- **Races across addresses** are decided by nothing for a one-part engine. The closing rule already stands in DL17 (F8).
- **Reader rules** exist only for inspect.v1, and a record does disclose what its reader could not otherwise learn (F5).

What "someone answers" still lacks:

- a name behind the ID once an Agent is recreated (F6);
- the object a management verb acted on, and which package an install installed (F7);
- any evidence against an agent that runs as the engine's operating-system user, which is every unconfined Agent in v0. This is a stated limit: such a program can change the store and read root's token.

Own tools leave no record by statement (CTR-014).

**What the reader read.** Reviewed at 2eb8f258 (2026-11-12), read only. Nothing was built or run, so no vector, model or engine behavior was observed; engine and vector files are cited as evidence of divergence, never as what holds. Recounted: 519 promises, 17 implementation-defined items, 6 REC, 5 OBS and 24 PKG promises.

Read completely:
- spec/conventions/records.yaml and observation.yaml, and spec/implementation-defined.yaml;
- spec/README.md, 'What this is' and 'Limits of this version' with the records section;
- order 082 with its Record;
- MC7b's catalogue accounting, property text, findings and 'What MC7b establishes' in experiment 012's README, and G5's rows for REC and MC7b;
- PKG-021 to PKG-032, CTR-013, MAN-016;
- docs/AUDIT.md 'Who decides what' and 'Known to be open', the cold readers' known-open list, the full-review brief, IN_FORCE topics 1, 7 and 15, and review items 3 and 29.

Read in the parts cited:
- OPS-001 to OPS-014, OPS-018, OPS-023, the IMP promises;
- LIF-013 to LIF-019, LIF-036's end, LIF-037, LIF-040, LIF-041;
- ACC-001 to ACC-003, ACC-011, ACC-032 to ACC-039, ACC-044, ACC-053, ACC-056;
- PND-001 to PND-005, PND-009, PND-011, PND-012; FLEET-004, FLEET-005; PRV-021, PRV-029; SEC-015 to SEC-017; IFC-022, IFC-028; OBJ-009 to OBJ-011;
- the core contracts inspect.v1, install.v1, purge.v1, reconcile.v1, lift.v1, stop.v1, list.v1, record.v1, tombstone.v1, provenance.v1, and the head of types.yaml;
- the definitions rows for engine, ID, target, record, gap and purge;
- engine/CONFORMANCE.md's DL2, DL8 and DL10 rows, engine/crates/core/src/records.rs, and the purge and lost record vectors.

Not read: revisions.yaml, enrollment.yaml, placement.yaml, the rest of providers.yaml and of the interface file, OPS-024 onward except in passing, and the model's source.

Known points are reported only with a new situation: F1, F2, F4 and F8 are marked known-new-situation. Not reported because they are stated limits or already queued: tamper evidence, the shape of a gap, a bound on a fetch, recreating a lost record, and PRV-021's split.


## FR-236

**No promise-keeping answer exists when the store cannot be written, and DL10 and SEC-016 hand the choice to each engine**

X09/F1 · contradiction · unsafe · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** REC-001, SEC-016, OPS-005, OPS-008, OPS-018, IMP-007, LIF-040, ACC-044, REC-004, PND-012, ACC-037, DL10

**Where.** spec/promises/secrets.yaml:415

**Quote.** "No value is read without a record of the read under REC-001, and the run or the connection that it was for is then not asked for or made, under SEC-012. How the engine behaves for other requests when its storage is exhausted is DL10."

**Situation.** The store of records and state cannot be written (full disk, I/O error, read-only remount; DL10 names only exhaustion, spec/implementation-defined.yaml:88). REC-001 has a record kept of every decision, undispatched ending, dispatch and end, and exempts one request, IFC-036's (spec/conventions/records.yaml:17-23, 30-31). OPS-005 has result and record carry the same reason (spec/conventions/operations.yaml:74-75). OPS-008 has invocation state kept (operations.yaml:155). So no answer keeps every promise, a refusal included, and SEC-016's own unavailable breaks REC-001.

Three builders diverge:
- A, the first build, stops with an 'embedding error', none of the four reasons (engine/CONFORMANCE.md:58).
- B answers unavailable, unrecorded.
- C reads SEC-016's last sentence as leave to go on. The effect lands with no invocation state and the caller's repeat is dispatched again (OPS-018). A close leaves no tombstone and is reopened without a lift (LIF-041).

What a failed write does to each promise that presumes kept state:
- REC-001, ACC-002, ACC-036: no record of the decision, so nothing may be decided.
- OPS-008, OPS-018, IMP-007, OPS-012, OPS-013: state must be kept before dispatch (only DL2 implies it); an end that cannot be kept must read unknown.
- OPS-009, OPS-034: already safe, the repeat reports conflict.
- OPS-014, ACC-053: reconcile and lift cannot proceed.
- LIF-040, ACC-044, CON-008: no close, retire or open without its tombstone or provenance.
- PND-003, PND-011, PND-012: no answer; the SignInUndone hold must be kept before model work resumes.
- PKG-025, PKG-027, PKG-032, CTR-013, the ENR promises, PLC-024, revisions: the change is not made.
- OBS-003: the last observation time is not kept.
- PRV-021: a provider that cannot keep an owed stop answers unknown either way; stop.v1 has no output to say so.
- REC-004: the purge needs its own record, so the one operation that frees records cannot run on a full store.

New beside the tracker's bullet (docs/AUDIT.md:238-241): the record itself, the unrecorded refusal, an end lost after dispatch, failures other than exhaustion, the purge deadlock, and the three texts that say the opposite today (DL10, SEC-016, spec/promises/submissions.yaml:296-297).

**Counterargument.** A DL10 choice 'must keep every promise' (spec/implementation-defined.yaml:3), so builder C already breaks OPS-008 and REC-001 by the letter. The secrets proposal kept the rule to reads on purpose: 'nothing else in this version needs it, and DL10 leaves storage exhaustion otherwise to each engine' (docs/proposals/secrets/README.md:1794-1795). The first build already commits before any intent. A full disk on one developer's machine is rare.

**Least change.** Extend the two fail-closed rules the specification already has; add no outcome.

1. ACC-037's `when`, after 'cannot be established': 'or the record of the decision, or what a promise has the engine keep before the request is dispatched (OPS-008, LIF-040, ACC-044, PND-012), cannot be kept, or the state that a promise has the decision read cannot be read'.
2. REC-001, beside the IFC-036 exception: 'Nor is a request recorded that reports unavailable because its record cannot be kept; its diagnostic names ACC-037 and says that the engine's store cannot be written.' OPS-005's observable cites it.
3. OPS-007's list: 'or the engine cannot keep the invocation's end', so the outcome is unknown and OPS-014 settles it once the store can be written.
4. DL10: replace 'behavior when storage is exhausted' with 'what else the engine does when its store cannot be written, such as stop, and how a purge by the root Interface is still recorded then'.
5. SEC-016 and SUB-011 cite ACC-037 in place of DL10.

**Precedent.** HashiCorp Vault refuses a request that no audit device can log, the rule SEC-016 already follows (docs/research/SECRETS.md:703-704). The Kubernetes API server's audit mode blocking-strict fails the request when its audit event cannot be written. etcd under its space alarm accepts only reads and deletes, so the operator can recover.

**Who decides.** coordinator. The tracker gives it to the coordinator as a defect settled with existing outcomes (docs/AUDIT.md:238-241; rulings 37 and 46 point 4). The text removes a contradiction and adds no outcome. It also reverses the secrets proposal's narrowing, so it is logged for the owner to strike. Whether a purge or an inspection may still run on a store that cannot be written adds behavior: ruling 71 point 4 if small, otherwise the owner's.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/secrets.yaml:415-417. The core is already recorded: docs/AUDIT.md:238-240 "what an engine owes when it cannot store invocation state, tombstones, provenance or installed packages (with existing outcomes, failing the request rather than acting unrecorded)"; docs/guides/full-review.md:138 "Order 082's questions: state that cannot be stored". The title overstates: stopping without an answer keeps every promise, as a crash does (spec/conventions/implicit.yaml:75 "An invocation that was in progress across a restart has the outcome unknown"); what no engine can do is answer and keep REC-001. Builder C breaks OPS-008 (spec/conventions/operations.yaml:155-160) and REC-001, which spec/implementation-defined.yaml:3 forbids: "A choice must keep every promise." Three sub-situations are new and real: (1) SEC-016 orders an answer (secrets.yaml:413 "It is not dispatched, and reports unavailable") that REC-001 has recorded, with one exception that is not this one (spec/conventions/records.yaml:30-32 "One request is not recorded: ... under IFC-036"; secrets.yaml:398 "Every read is recorded as a request, whether or not it succeeds"); (2) DL10 names exhaustion only (implementation-defined.yaml:88 "behavior when storage is exhausted"); (3) an end that cannot be kept after dispatch is in no list of OPS-007. Reading C is invited by SEC-016's last sentence, and docs/proposals/secrets/README.md:1793-1795 shows it was meant: "nothing else in this version needs it, and DL10 leaves storage exhaustion otherwise to each engine". The purge deadlock is not a fault of today's text: it follows from the proposed fix.

**Verifier's corrections.** Class: a known gap, plus one narrow new contradiction (SEC-016 against REC-001, SEC-015 and OPS-005), not a contradiction across twelve promises. Unsafe: conditional. Only reading C does harm, and it breaks OPS-008 and REC-001 by the letter; say that the mark rests on SEC-016's a-contrario reading. Title: drop "no promise-keeping answer exists"; stopping is one. Who decides: the tracker gives it to the coordinator, but docs/work-orders/082-spec-records-and-installs.md:131-132 and its Record :735-737 put it to the owner, and the fix reverses the secrets proposal's narrowing. Log it to strike and add it to the review list. Least change stands; the REC-001 exception for SEC-016's unavailable alone is the smallest part and adds no behavior.


## FR-237

**The limit on outside loss covers records only: a rolled-back store reopens closed Conduits and repeats effects, and the state promises stay unconditional**

X09/F2 · gap · unsafe · verifier: confirmed, high · decides: owner · known-new-situation

**Promises.** OPS-018, OPS-008, ACC-011, LIF-040, LIF-041, ACC-044, IFC-022, PND-011, REC-003, REC-004, ACC-037, DL13

**Where.** spec/README.md:612

**Quote.** "A loss of records outside the engine that the engine cannot tell is not reported, and a record altered outside the engine where the engine does not find the change is read as it now holds."

**Situation.** REC-003 names 'someone other than the engine restoring an older copy of the store' as an expected outside fault (spec/conventions/records.yaml:107-108). The first build keeps 'the whole core State, including records, in one JSON file' (engine/CONFORMANCE.md:18), so that restore also rolls the engine's own state back.

Steps to the harm:
1. Root closes Agent A's Conduit to a Resource, retires and removes Agent B, and a keyed write succeeds.
2. The operator restores yesterday's copy of the store after a disk fault.
3. The engine reads the state as it stands and keeps every promise it can.

What follows:
- The closed Conduit is open, and A's refused request is permitted, against ACC-011.
- The tombstone is gone, so nothing needs a lift (LIF-041).
- B exists again. Its credential never lapses, and 'the engine's check on every request that the instance still exists is the only revocation' (spec/implementation-defined.yaml:153-157), so a run left behind reaches the engine as B.
- The succeeded invocation is forgotten, and its repeat is dispatched as new work, the effect OPS-018 forbids.
- A SignInUndone hold (PND-011) and a refused enrollment are forgotten.
- Records purged since come back, and the purge's own record is lost.

Order 082 narrowed REC-003 because a promise against outside faults 'says what no engine can keep'. OPS-018, ACC-011, LIF-040, ACC-044 and PND-012 still say exactly that, and the limits section speaks of records alone. Losing the whole store fails closed, as a new first start with new keys. A partial rollback fails open.

New beside the tracker's bullet: failing the request cannot help, because the engine sees no failure.

**Counterargument.** The store, its backups and its operators are already inside the domain of trust for the signing keys (DL13, spec/README.md:498-499), and restoring a copy is the operator's deliberate act. Order 082's questions for the owner named state 'once it is lost', and the owner kept the records limit on 2026-11-12. Kubernetes has the same hazard when etcd is restored. Detecting a rollback needs a means outside the store, which order 082 ruled out for a coordinator.

**Least change.** Least, with no new behavior: add a second point to 'What the promises on records cover' (or a sibling limits section) and one clause to DL8.

'The engine's own state is kept as its records are: the Conduits, invocations (OPS-008), tombstones (LIF-040), provenance (ACC-044), pending and answered actions (PND-012) and installed packages. Where the engine can tell that it cannot read that state, a request reports unavailable under ACC-037. A loss or an older copy of it that the engine cannot tell is not detected: a Conduit closed since is open, a tombstone made since is gone, an instance removed since exists and its credential reaches the engine again (DL13), a repeat of an invocation that ended since is dispatched as a new one, and records purged since are read as found. Whoever restores a copy closes, retires and purges again what it brought back, before the engine serves.'

DL8 adds: 'and which losses of its own state outside it the engine can tell'. OPS-018's observable points at the limit.

The owner's alternative: an engine refuses to serve a store older than the one it last wrote, which needs a generation kept outside the store.

**Precedent.** etcd's snapshot restore gives the restored member a new member ID and cluster ID, so a restore is never silent to its peers. Active Directory detects a domain controller restored from an old image (USN rollback) and stops it replicating. ACC-037 is Conduit's own rule for state that cannot be established.

**Who decides.** owner. Writing the limit down adds no behavior and is the coordinator's (rulings 37 and 46 point 4). Whether v0 accepts that a restore returns revoked access and repeats effects, or requires an engine to tell a rollback, is scope and a requirement on the means. Order 082's 'What stops a fix' reserves both for the owner (docs/work-orders/082-spec-records-and-installs.md:127-132).

**Verifier.** confirmed, high. Quote verbatim at spec/README.md:612-614; the limit cites only "(REC-003, REC-006, DL8)" (:615). REC-003 names the restore as expected (spec/conventions/records.yaml:107-108 "someone other than the engine restoring an older copy of the store"). No text in spec/ speaks of the engine's own state after such a restore: a search for restore, older copy, rollback and backup finds only records.yaml:105-108 and DL13's "with its backups and operators" (spec/implementation-defined.yaml:129). The state promises are unconditional: OPS-018 (spec/conventions/operations.yaml:388-392), ACC-011 (spec/conventions/access.yaml:184-187 "All the access it carried ends"), LIF-040 (spec/conventions/lifecycle.yaml:832-833). DL13 makes the engine's check the only revocation (implementation-defined.yaml:155-157), and engine/CONFORMANCE.md:18 keeps "the whole core State, including records, in one JSON file". Order 082 raised state "once it is lost" for the owner (docs/work-orders/082-spec-records-and-installs.md:346-348), but its Record (:735-737) and docs/AUDIT.md:239 kept only "cannot store", so the rollback the engine cannot see is in no known-open list. The known-new-situation mark is right.

**Verifier's corrections.** None to class, unsafe or who decides. Add to the counterargument that whoever restores a store is inside the domain of trust (spec/README.md:496-499) and could edit the state directly, so the remedy is a stated limit and an operator's duty, not machinery. "Losing the whole store fails closed" holds only where the signing keys live in the store; DL13 also allows an outside issuer. The unsafe mark rests on reading the state promises as order 082 read REC-003, as binding the engine's acts; on the unconditional reading the class is defect (a promise no engine can keep). Either way the text needs the limit.


## FR-238

**REC-003's line between the engine's acts and outside faults leaves power loss, the engine's chosen store and its own restore tool undecided**

X09/F3 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** REC-003, REC-006, DL8

**Where.** spec/conventions/records.yaml:103

**Quote.** "The engine's own crash and restart are its acts, as are every write it makes to the storage of its records, including one interrupted by its crash, and every compaction, copy or rollback to an earlier copy that it makes itself."

**Situation.** The promise sorts every loss into the engine's acts (a breach) or one of three outside faults (no breach): the medium failing, someone else restoring an older copy, or another process changing the file (records.yaml:106-109). The engine is 'a running program' (spec/definitions.md:100). Three reachable cases fall between the lists.

1. Power loss or an operating-system crash while a record sits in the page cache.
   - Reading A: the write is the engine's, so the record must be on the medium before the request is answered. The first build fsyncs the file and its directory (engine/CONFORMANCE.md:18).
   - Reading B: only the engine's own crash is named. A buffered log survives a killed process and passes every kill test. A power loss is then outside, and DL8 may say the engine tells no such loss.
   - Under B the last records of dispatched effects vanish at every power cut with no gap reported.

2. The store the engine chose discards by its own configuration: a journal with a size cap, a log service with retention, a database that compacts.
   - Reading A: the engine chose it, so its removals are the engine's.
   - Reading B: it is another process, so they are outside faults. An engine may then keep records for an hour and break nothing.

3. The engine's own command restores a backup.
   - Reading A: it is 'rollback to an earlier copy that it makes itself', a breach, so a conforming engine can ship no restore command while the same restore by file copy is no breach.
   - Reading B: the person restores, so it is outside.

**Counterargument.** The first sentence of the observable is flat: 'The record can still be read after the engine has restarted.' A careful builder takes it to cover any stop. Storage 'stays each engine's choice' (spec/README.md:40-41), and order 082 treated a requirement on how an engine stores records as the owner's. The owner reviewed the narrowing on 2026-11-12 (review item 29).

**Least change.** Add three sentences to REC-003's observable:

'A stop of the engine for any cause, a loss of power or a crash of its operating system among them, is its restart: a record whose request the engine has answered can still be read after it. The storage that the engine uses for its records acts for the engine: what that storage removes or alters by its own configuration, such as rotation, retention or compaction, the engine removes or alters. A copy that a person restores is that person's act, whatever program restores it.'

DL8 keeps naming which outside losses the engine can tell.

**Precedent.** The Kubernetes audit log backend has both modes and says which loses events: `batch` may drop events on a crash, `blocking` waits for the write. Its rotation flags are the API server's own configuration. PostgreSQL's commit returns only once the write-ahead log is flushed.

**Who decides.** coordinator. An ambiguity in text the coordinator wrote in order 082. Each sentence picks one of two sound readings of REC-003's own first sentence and adds no operation, key or outcome (rulings 37 and 46 point 4). If the coordinator reads the first sentence as a requirement on how records are stored, it goes to the owner under order 082's own list.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/records.yaml:103-105. The outside faults are listed at :106-109 ("the medium failing, someone other than the engine restoring an older copy of the store, or another process removing or changing its file") and neither list names a power loss, the store's own retention or the engine's own restore command. "engine" is "A running program" (spec/definitions.md:100). spec/README.md:40-41 leaves "The storage of records and the engine's own state ... each engine's choice". Nothing in spec/, docs/AUDIT.md, IN_FORCE.md, the review list (item 29, to-review-with-the-owner.md:72) or order 082 decides the three cases; a search for power, fsync, retention and rotation finds nothing on records.

**Verifier's corrections.** Case 1 is the one that matters, and reading A is the stronger: the first sentence is flat (records.yaml:102 "The record can still be read after the engine has restarted") and the outside faults read as an enumeration. Cases 2 and 3 are thinner. Who decides: the first proposed sentence requires durability across a power loss of every engine, which docs/work-orders/082-spec-records-and-installs.md:127-128 lists as the owner's ("a requirement on how an engine stores its records"); the coordinator may settle it only as the reading of the existing first sentence, logged to strike. The other two sentences are the coordinator's.


## FR-239

**Whether a purge leaves a gap: the definition says no, the portable vectors say yes under REC-006, and a reader below management cannot see that a purge happened**

X09/F4 · ambiguity · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** REC-004, REC-006, LIF-013

**Where.** spec/definitions.md:375

**Quote.** "Where a read cannot return records that should exist, because they cannot be read or the engine can tell that they were lost (REC-006), including records lost by a fault outside the engine (REC-003)."

**Situation.** Root purges records made before T. An actor with only a Pool's authority then inspects a Resource.

Reading A, the text. Purged records are not ones 'that should exist', and REC-006 adds that 'a loss of records is not presented as a purge' (spec/conventions/records.yaml:174-175). The read is complete. MC7b modelled this reading (docs/work-orders/082-spec-records-and-installs.md:84-85).

Reading B, the vectors. `records-purge` expects `recordsComplete: false` from that inspection and cites REC-006 (conformance/conventions/records/purge/case.yaml:589-595). `records-purge-succeeded` does the same (lines 598-603). The gap's shape is 'settled with the conformance vectors' (spec/core/README.md:703-705), so these vectors are becoming the rule.

A second builder who follows `spec/` fails two portable vectors.

Under A the reader is told nothing. The purge's record names only root and `management`, so a Pool-level inspector cannot find it, and a shortened list looks whole. Under B the reader cannot tell root's deliberate purge from a loss.

A second ambiguity sits in REC-004: 'a record is kept stating which records were purged' (records.yaml:122-124). It may mean the cutoff or a list. The first build lists every purged ID in a record that 'no later purge removes' (engine/crates/core/src/records.rs:489-499), so purging never shrinks that list.

New beside order 082's Record: the definition of gap that 082 itself added now excludes purged records by its words, and the reader who cannot see the purge.

**Counterargument.** Order 082 left this to the coordinator on purpose and changed no case (its Record, line 740). The first-build profile calls its behavior conservative. Root, the only actor that may purge, can always read the purge's record. A reader told 'incomplete' is not misled into thinking the list whole.

**Least change.** Say it as a third thing, neither gap nor silence.

REC-004's observable adds: 'A read that would have returned records that a purge removed says that records made before that purge's time were purged, naming the time. This is not a gap under REC-006, and the result is complete apart from it. The record of a purge states the time it was given and the time it ran; it need not list the records.'

The definition of gap adds: 'Records that a purge removed are not a gap.'

Then the two vectors assert the purge notice and not `recordsComplete: false`, in a build order.

**Precedent.** etcd answers a read below its compacted revision with an error that names the compact revision. Kubernetes answers 410 Gone for a resource version that is too old. Kafka reports a log's start offset, so a reader knows where retention cut it. None of them calls it data loss.

**Who decides.** coordinator. Order 082's Record gives it to the coordinator. The text settles an ambiguity between the definition and two vectors (rulings 37 and 46 point 4). The notice in a read is a small addition whose shape the vectors fix (ruling 71 point 4), and it follows ruling 52: a result says what happened and why.

**Verifier.** weakened, medium. Quote verbatim at spec/definitions.md:375. The main question is already recorded: docs/work-orders/082-spec-records-and-installs.md:80-86 ("`records-purge` and `records-purge-succeeded` expect `complete: false` after a purge ... MC7b read a purged record as no gap ... This order does not settle that question") and its Record :739-740 ("For the coordinator, whether a purge leaves a gap"), which docs/AUDIT.md:245 links. spec/ itself is not ambiguous: purged records are not ones "that should exist", and REC-006 keeps loss and purge apart (spec/conventions/records.yaml:174-175 "A loss of records is not presented as a purge under REC-004"). spec/core/README.md:703-705 leaves to the vectors only "how a gap in the records is shown", the shape, not whether there is one. The conflict is between spec/ and two vectors: conformance/conventions/records/purge/case.yaml:589 and :594, purge-succeeded/case.yaml:598 and :603, listed under REC-006 at conformance/manifest.yaml:3994 and :4016. New and real: a reader below `management` cannot see that a purge happened (purge.v1 is "authorized against `management`", spec/core/operations.yaml:701), and "stating which records were purged" (records.yaml:123-124) can mean a cutoff or a list.

**Verifier's corrections.** Class: not an ambiguity in spec/. It is a known conflict between spec/ and two vectors (a defect of the vectors and the first-build profile), plus a small new gap under ruling 52 (no notice of a purge to a reader who cannot read its record) and a low ambiguity in REC-004. Who decides: one sentence that purged records are no gap, and correcting the two vectors, is the coordinator's and adds no behavior. The purge notice is a new output; order 082:123-125 counts a key or value of a gap's shape as a feature, so it is ruling 71 point 4 at most. "It need not list the records" changes what a purge records, which order 082:133 lists as the owner's.


## FR-240

**A record is returned whole to whoever may inspect any one object it names, so a caller's inspector reads what a target returned; no reader rule exists outside inspect.v1**

X09/F5 · gap · unsafe · verifier: weakened, medium · decides: owner · new

**Promises.** LIF-013, REC-002, CTR-004, ACC-033, ACC-056, DL8

**Where.** spec/conventions/lifecycle.yaml:247

**Quote.** "The records are those that name the instance."

**Situation.** One record names its caller, its Conduit, the consumer and the target, and holds inputs and outputs where the target's Type states `records` (spec/conventions/records.yaml:45-53; spec/promises/contracts.yaml:102-104). LIF-013 returns it to an inspector of any one of those objects. No promise trims it by reader, except code and fingerprint values (PND-004) and secrets (SEC-015).

Steps:
1. Resource R, in Pool P, has a Type with `records: {outputs: true}`.
2. Agent A2 reads R through an unfiltered Conduit. Agent A1's Conduit to R applies a filter.
3. A scoped Interface, or A1 itself, holds an open Conduit to their Fleet that lists inspect.v1. LIF-013 foresees it: 'For an Agent the scope includes its own Fleet only' (lifecycle.yaml:259).
4. It inspects A2 under ACC-033 row 6A and receives every record that names A2 as caller, with R's outputs.

The reader holds no Conduit to R or P. It reads what R returned, A1 past its own filter. Closing every Conduit to R does not end this, since records outlast their objects (REC-003). Only root's purge does.

The specification is explicit everywhere else:
- a refusal's tier 2 needs authority over 'every object the request names' (spec/conventions/access.yaml:921-923);
- inventory is shown 'in full' by statement (lifecycle.yaml:255-257);
- pending actions show 'only subjects and evidence about objects it may inspect' (PND-002).

For records it says nothing. The limits section does not list it.

DL8 also leaves feeds, exports and the reading of records that name no existing object to each engine, with no word on who may read them.

**Counterargument.** Keeping inputs or outputs is the Type author's choice, off by default; no core Type and no shipped package turns it on. Inspecting a Fleet is a deliberate administrative grant, and whoever administers an Agent can usually read its home directory anyway. IN_FORCE lists 'records filtered by Fleet' as open after the S1 rerun (62 C10). The interface promises keep an engine from presenting a feed as a tool to a consumer.

**Least change.** Least, with no new behavior: state the limit in spec/README.md's records section.

'Whoever may inspect one object that a record names reads the whole record: the IDs of the others, and the inputs and outputs that the target's Type keeps. So an actor that may inspect an Agent reads what the Resources it used returned to it.'

For the owner, in ACC-056's own form, one sentence in LIF-013: 'A record's `inputs`, `outputs` and `answer` are returned only to a reader that may also inspect the record's target; another reader receives the record without them.'

DL8 adds: 'whatever means it documents admits only a reader that `inspect.v1` would admit, and `management`'s authority for a record that names no object that still exists (ACC-033).'

**Precedent.** ACC-056's tier 2 is Conduit's own rule: full detail only to a caller that may inspect every object named. AWS CloudTrail, which records.yaml follows, writes a cross-account event as a separate copy to each account, linked by `sharedEventID`, and the resource owner's copy identifies the caller only by account and principal ID.

**Who decides.** owner. Who may read what is access, which the owner rules. Writing the limit down is the coordinator's (rulings 37 and 46 point 4). Trimming a record by reader changes behavior and touches the open item 62 C10.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/lifecycle.yaml:247-248 (it spans a line break). The path holds: ACC-033 row 6A (spec/conventions/access.yaml:611 "Inspecting an Agent: its owning Fleet"), an Agent's own-Fleet scope (lifecycle.yaml:259-260; spec/README.md:383), REC-002 (spec/conventions/records.yaml:50-53), and no promise trims a record by reader. But the question is recorded as open with the owner: IN_FORCE.md:178 "Open, after the S1 rerun: 62 C6 to C12 ... records filtered by Fleet"; docs/proposals/ruling-62/README.md:24-26; and the projects brief put this very trimming to the owner (docs/proposals/projects/brief.md:1528-1530 "Inputs a Type records are visible only to the caller's project and root"; :340 "v0 does not filter records by Project (ruling C10)"). What is new: spec/README.md states no such limit, and the concrete route past a filter. The harm needs a Type that keeps outputs: spec/promises/contracts.yaml:119-120 "one that is left out is false"; all seven packages' Types state false (for example packages/provider-file/contracts.yaml:34-36). Secrets are excluded whatever `records` says (spec/promises/secrets.yaml:395-396).

**Verifier's corrections.** Known mark: known-new-situation, not new; cite IN_FORCE.md:178 and ruling 62 C10. Unsafe: conditional on a Type author turning `records.outputs` or `inputs` on and on a deliberate grant of Fleet inspection; no secret leaks. spec/README.md:374 already says "A Fleet organizes administration and supplies no confinement". Who decides stands: the limit sentence is the coordinator's, the trimming rule is the owner's and belongs with C10. The proposed rule (reader must also inspect the target) is stricter than the brief's option 17(a), which let the caller's scope read recorded inputs; say so when it goes to the owner.


## FR-241

**A record names its objects by ID alone, and the only link from an ID to a name is a tombstone that a routine lift removes**

X09/F6 · gap · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-206, FR-215.

**Promises.** REC-002, REC-003, LIF-040, ACC-053, LIF-019

**Where.** spec/core/results.yaml:156

**Quote.** "One record, as a CloudEvent of CloudEvents 1.0 whose `data` holds the fields of a record: the IDs of the caller, of the Conduit used and of its consumer, where a Conduit was used, and of the target"

**Situation.** Steps:
1. Agent `codex` (ID X) writes to a shared Resource. The record holds `caller: X`.
2. The author recreates the Agent under its stable name. The stated path is retire, remove, lift the tombstone, register again (spec/README.md:365-367). The new `codex` has ID Y.
3. A week later someone inspects the Resource to learn who made the write.

REC-003 keeps the record readable 'after the objects it names are unregistered or removed' (spec/conventions/records.yaml:102-103). It says `caller: X`, and X names nothing.

The link from X to a kind, a name, a Type and a Fleet lived in two places:
- the object, removed under LIF-035;
- its tombstone, which 'retains the object's kind, name and ID' and `by` (spec/conventions/lifecycle.yaml:832-835).

'The lift removes the named tombstone' (spec/conventions/access.yaml:837). Any holder of lift.v1 within its bound may lift; it is not reserved to root. The lift's own record keeps the tombstone's ID, not what it held.

Registration's record keeps no outputs, since no core Type states `records` (spec/core/types.yaml:18-20).

So after one ordinary recreate, nothing a promise requires says which agent X was. Purpose and preservation test 7 need 'attributable' to mean an agent a person can name.

No promise says an ID is never given to a second object. PRV-029 and IFC-022 assume it, and so does every record.

**Counterargument.** An engine 'may add members' to a record, and the first build's object-change records carry more. The registration's record still exists until purged and could be matched by time. Names are reused by design, so an ID is the only stable identity, and a tombstone is kept until someone with authority decides to lift it. In a short S1 session nothing is recreated.

**Least change.** REC-002 adds: 'A record names each object it holds by kind, name and ID, as they stood when the record was made', in the form `object.v1` that results and tombstone.v1 already use, in place of the bare IDs of record.v1.

Or, smaller: ACC-053's observable adds 'the record of a lift keeps what each tombstone it removed held'.

The ID row of definitions.md adds: 'An ID is given to one object or invocation and never to another.'

**Precedent.** A Kubernetes audit event's `objectRef` carries resource, namespace, name and UID, and its `user` carries the username beside the UID. A CloudTrail event's `userIdentity` carries the ARN beside the principal ID. Kubernetes states that UIDs are unique across time.

**Who decides.** coordinator. A small finding that adds a feature, the kind and name beside an ID that records already hold: ruling 71 point 4, logged for the owner to strike. It moves none of S1's author counts. The sentence on IDs states what PRV-029, IFC-022 and OBJ-010 already assume (rulings 37 and 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/core/results.yaml:156-158; the schema's caller, consumer, conduit and target are bare `id.v1` (:189-192). REC-003 keeps a record readable after its objects are removed (spec/conventions/records.yaml:102-103). The tombstone alone keeps "the object's kind, name and ID" (spec/conventions/lifecycle.yaml:833), a lift removes it (spec/conventions/access.yaml:837 "The lift removes the named tombstone"), the lift "is recorded with its input" (:846-847), which is the tombstone's ID, and lifting has "no root-only reservation" (:842). The stated way to recreate under a stable name lifts it (spec/README.md:365-367). Core Types keep no inputs or outputs (spec/core/types.yaml:18-20). No statement in spec/ says an ID is never given to a second object; OBJ-010 (spec/promises/objects.yaml:140-148) implies it only while an instance holds the reference. Nothing in the tracker, IN_FORCE.md, the increment logs or the review list records this.

**Verifier's corrections.** Who decides: replacing the bare IDs of record.v1 by `object.v1` changes a core schema that every records vector compares; that is more than ruling 71's small finding, so it goes to the owner or is logged prominently. The smaller option (the lift's record keeps what each removed tombstone held) and the sentence on IDs are the coordinator's. Check the larger fix against disclosure: a name in a record reaches every inspector of either end, and finding 2-08 (to-review-with-the-owner.md:14) withheld whether a name exists. A Conduit's tombstone also keeps both ends by name (lifecycle.yaml:836), but ACC-053 removes caused tombstones with the lift, so it does not close the gap.


## FR-242

**A management verb's record has no place for the object it acted on, and three promises require record content that REC-002 says is not kept**

X09/F7 · contradiction · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-028.

**Promises.** REC-001, REC-002, REC-004, ACC-053, OPS-014, CTR-004, LIF-013

**Where.** spec/conventions/records.yaml:89

**Quote.** "Exception to management's keeping no inputs: an install.v1 request confirming shared client IDs (CTR-013) keeps data.inputs.confirmClientIds, the client IDs confirmed, and no other input."

**Situation.** A management verb's target is `management`, `access`, a Pool or a Fleet; the object it acts on is an input. 'No core type states `records`, so the records about its instances keep neither inputs nor outputs' (spec/core/types.yaml:18-20). record.v1 has fields for caller, consumer, conduit, target and operation, and `subject` only for run.v1, stop.v1 and inspect-run.v1 (spec/conventions/records.yaml:68-70).

What a record built from record.v1 alone says:
- Agent X retires Resource R through Pool P: 'X asked retire.v1 of P, succeeded'. It does not say which Resource.
- install.v1: not the address, version or digest, 'and no other input'. Installation is not in REC-001's list. Yet the stated defense against a package that takes a login is 'choosing packages, pinned by digest' (spec/README.md:418-419). Records cannot say who installed which package.

REC-001 promises that each occurrence 'can be found by inspecting the objects it names' (records.yaml:25), and LIF-013 returns 'the records that name the instance'. Two builders diverge on what inspecting R returns:
- The first build adds its own `data.object` and joins records to objects through invocation state. It calls this 'runner conventions' (engine/crates/core/src/records.rs:211-217; conformance/README.md:98-100).
- A second builder returns only records whose caller, consumer, conduit, target or subject is R, so none of R's setups or retirements.

Three promises require content that REC-002 and CTR-004 say is not kept:
- REC-004: 'a record is kept stating which records were purged' (records.yaml:122-124); `before` is an input.
- ACC-053: a lift 'is recorded with its input' (spec/conventions/access.yaml:846-847).
- OPS-014: 'A record states ... what the actor checked' (spec/conventions/operations.yaml:320).

REC-002 lists one exception, and types.yaml repeats that it is the only one.

**Counterargument.** REC-001 also records each lifecycle change, registration and revision change apart from the request, and an engine 'may add members', so an engine can name the object there. Core Types keep no inputs on purpose: a registration's documents are large, and a record must hold no secret. The object can be found while it or its tombstone exists, through status and `cause`. The core README says shapes are settled with the vectors.

**Least change.** REC-002 adds: 'The record of a management verb, and of a change to an object under REC-001, names in the CloudEvent's `subject` the object that it acts on or changes, as a record of `run.v1` does, so inspecting that object finds it. The record of `install.v1` keeps `address`, `version` and `digest`; of `purge.v1`, `before`; of `lift.v1`, `id`; of `reconcile.v1`, `invocation`, `occurred` and `checked`. These, with the confirmed client IDs, are the inputs that a core Type's records keep.'

The comment at spec/core/types.yaml:18-20 follows.

**Precedent.** CloudEvents defines `subject` for exactly this, and REC-002 already uses it for provider calls. A Kubernetes audit event names the object in `objectRef`. CloudTrail keeps `requestParameters` and `resources` for management events.

**Who decides.** coordinator. Listing the exceptions removes a contradiction between REC-002 and REC-004, ACC-053 and OPS-014 and adds no behavior (rulings 37 and 46 point 4). The `subject` rule and the install inputs are a small added feature that makes the first build's convention portable: ruling 71 point 4, logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/records.yaml:89-91. record.v1 has no member for the object a management verb acts on; `subject` is defined only for run.v1, stop.v1 and inspect-run.v1 (records.yaml:68-70). REC-001 still has each occurrence "found by inspecting the objects it names" (:25), and what it is for a record to name an object is nowhere defined, unlike `unreconciled` (spec/core/operations.yaml:290-291 "names it in an input or as its target"). The first build adds its own member and says it is not portable: conformance/README.md:98-101 "runner conventions ... Lifecycle records retain the named object in `data.object`". Three promises require content of a core operation's record: REC-004 (records.yaml:123-124), ACC-053 (spec/conventions/access.yaml:846-847 "it is recorded with its input"), OPS-014 (spec/conventions/operations.yaml:320 "what the actor checked"); against spec/core/types.yaml:18-20 "keep neither inputs nor outputs ... except install.v1's confirmed client IDs". No known-open record holds it; A39 asks a portable field only for PLC-024's cause (docs/AUDIT.md:226).

**Verifier's corrections.** Class: two findings in one. (a) A gap: no portable place names the object of a management verb or of an object change, so an export in record.v1 form cannot say which object. (b) A defect of wording more than a contradiction: REC-002 says inputs "need not be" present (records.yaml:50), so each of the three promises can be followed; what is false is that install is the only exception, and where the content sits is unfixed. The divergence on what inspecting R returns is narrower than stated: REC-001 already obliges R's lifecycle changes to be found by inspecting R; the open part is the request records and the member that carries the object. Who decides stands; keeping install's address, version and digest reverses "and no other input" (records.yaml:91; spec/core/README.md:501), so log it to strike.


## FR-243

**Races across addresses: the rule that closes them exists only in DL17, for an engine of several parts, and an install's window is its whole fetch**

X09/F8 · gap · unsafe · verifier: known, medium · decides: coordinator · known-new-situation

**Promises.** PKG-029, MAN-016, CTR-013, PKG-032, OPS-023, DL17

**Where.** spec/promises/packages.yaml:465

**Quote.** "Requests for another version at that address, under NAM-011, or for another address are not held back."

**Situation.** What `spec/` says. PKG-032 orders installs of one address and version only. PKG-029, MAN-016 and CTR-013 each describe a decision against 'what is installed' or what a template 'holds', and each calls the other request 'the other order', as if requests came one after another (spec/promises/manifest.yaml:357-359; spec/promises/contracts.yaml:313-321). The rule that would close the race stands in DL17 alone and binds only an engine that moves or runs as several parts: a decision 'is committed against the state that it read' (spec/implementation-defined.yaml:204-208).

The window is wide on one process. OPS-023 puts conflict before unavailable (spec/conventions/operations.yaml:493-495), so an engine checks PKG-029 before it fetches. 'Deciding includes fetching the archive, before dispatch' (packages.yaml:467), and nothing bounds a fetch.

Install against registration:
1. An actor asks to install address X. PKG-029 finds no template holding X. The fetch starts.
2. Another actor registers a project whose manifest states X. MAN-016 finds no package installed at X. Accepted.
3. The fetch ends and the package is installed.

Both hold X. 'A qualified reference at the address would otherwise name both' (packages.yaml:391-393). A bound's item `X#name`, written for one, covers the other's operation of that name (ACC-029).

Two Types, one client ID:
1. A first install of A is fetching.
2. A holder of install.v1 who is not root installs B, whose Type states A's client ID.
3. Each is compared with the Types held, and neither is held yet.

Both install, and root's confirmation, CTR-013's one gate against 'a Type claiming another Type's client' (spec/promises/pending.yaml:281-282), is never asked. Two registrations race the same way.

New beside the tracker's bullet: the rule already exists in DL17, and the window is the fetch, not an instant.

**Counterargument.** The first build decides every request in one transition under a mutex and rechecks after the fetch (engine/CONFORMANCE.md:238-242), so it has no such race. Both races need two actors who may install or register, who in v0 are root or trusted by root. Order 082 left an order between addresses to the owner. A general promise is a wider change than two sentences.

**Least change.** Make DL17's two sentences a convention promise that binds every engine. DL17 keeps only how an engine of several parts meets it.

'Every decision that a promise has the engine make reads the state as it stands. A decision that admits one of several competing changes is committed against the state that it read: where what it read has changed, the request is decided again from the state as it stands.'

Its observable gives the two cases:
- an install whose fetch has ended is decided again, and reports conflict under PKG-029 or CTR-013 where a registration or another install got there first;
- a registration decided across an install's commit is rejected under MAN-016.

No request waits, nothing is queued, and no order between addresses is promised.

Narrower alternative: state the invariant in PKG-029 and CTR-013 in PRV-013's form ('at no time ...').

**Precedent.** etcd transactions compare a key's revision before they write, and Kubernetes checks `resourceVersion` and answers 409 Conflict. DL17 already cites both. PRV-013 and OBJ-002 already state their rules as invariants ('at any time at most one').

**Who decides.** coordinator. The tracker gives the races to the coordinator as defects (docs/AUDIT.md:238-242; rulings 37 and 46 point 4). The text moves an existing rule from DL17 to every engine. It adds no lock, no queue, no order between addresses and no new reason, which are the things order 082 reserved for the owner.

**Verifier.** known, medium. Quote verbatim at spec/promises/packages.yaml:465-466. Both races are in the tracker by name: docs/AUDIT.md:240-242 "races across addresses, between two Types claiming one client ID or an install and a project's registration", given to the coordinator at :238; also docs/guides/full-review.md:138 and order 082's Record (docs/work-orders/082-spec-records-and-installs.md:738-739 "races across addresses (client IDs, installation against registration)"). The two points offered as new are not situations those records lack: that the window is the fetch is PKG-032's own text (packages.yaml:467 "Deciding includes fetching the archive, before dispatch") and MC7b-002's witness; that DL17 holds the closing rule (spec/implementation-defined.yaml:199-208) is a basis for the fix. Not refuted: no promise binds a one-part engine to commit against the state it read; a search of spec/ for "as it stands", "competing" and "one at a time" finds only DL17, PKG-032 and REV-012/REV-014 for one instance.

**Verifier's corrections.** Known mark: known, not known-new-situation. Keep the content as the fix for the tracker's item. The unsafe mark is sound for the item itself: each promise holds at its own decision time, as in MC7b-002, and the forbidden state arises (a bound's `address#name` covering the other's operation, packages.yaml:396-397; root's confirmation never asked, spec/promises/contracts.yaml:317-319). Who decides: the coordinator, as the tracker says. The fix must add no new reason and no order between addresses, which order 082:134-135 reserves for the owner; re-deciding at commit under PKG-029, MAN-016 and CTR-013 meets that. A general convention promise taken from DL17 is wider than the two races need; the narrower invariant form in PKG-029 and CTR-013 is the least change.


## FR-244

**Invocation state is kept for every invocation with no end and no purge, so the store grows without bound and a purge does not remove who did what**

X09/F9 · gap · verifier: weakened, low · decides: owner · new

**Promises.** OPS-008, OPS-018, OPS-034, REC-004, ACC-011, DL10

**Where.** spec/conventions/operations.yaml:155

**Quote.** "The engine keeps, as its own state and apart from the records, what repeats and reconciliations need, for every invocation"

**Situation.** OPS-008 keeps, for every invocation, keyed or not, its caller, operation, target, Conduit, state, a digest and, for a control, the values sent. It ends: 'No purge touches that state' (operations.yaml:155-160). No promise or DL item says when it may be dropped. DL10 bounds only the outputs. OPS-018 and OPS-034 set no time after which a key may be forgotten. The state outlives its objects: 'the engine still holds the closed Conduit's ID in the state of the invocations dispatched through it' (spec/conventions/access.yaml:193-194). The first build says so: 'No key expires and invocation state is never compacted' (engine/CONFORMANCE.md:30).

Two situations follow.

1. Growth. An engine in daily use adds permanent state with every request. Tombstones accumulate until lifted, and each purge leaves a record that 'no later purge removes'. The store only grows. `purge.v1` is the one operation that frees anything and reaches none of this, so the storage failure of F1 is certain in time and the language offers no remedy.

2. Erasure. Root purges records before T to remove who did what. The engine still holds, for each of those invocations, who called which operation on which target and how it ended. That is a record's content without its time, readable by any program of the engine's operating-system user (spec/README.md:424-425).

Much of the state is dead weight. A key is scoped to its caller, sender and target by ID (OPS-008), so once any of the three is removed no repeat can match it. A request with no key is never a repeat (OPS-011). Whether an engine may drop that state is undecided: 'what repeats and reconciliations need' against 'for every invocation'.

**Counterargument.** Keeping the state apart from records is deliberate: it is why a purge cannot make a repeat apply an effect twice, and the owner reviewed the cost for kept outputs (review item 3). A few hundred bytes for each invocation is years of use on one machine. Forgetting keys early is how other systems come to repeat effects.

**Least change.** For the owner, three options.

A, recommended. REC-004 adds: 'A purge also removes the state that OPS-008 keeps of an invocation that ended before the time the request names, whose outcome is not unknown, and whose caller, sending instance or target no longer exists; no repeat can match it.' OPS-008 keeps 'No purge touches that state' for every other invocation. No repeat changes its answer.

B. State it as a limit: 'the engine's own state grows with every invocation and no operation shrinks it'.

C. A DL10 period after which a key is forgotten and a repeat is new work, as the IETF draft allows. This weakens OPS-018.

**Precedent.** The IETF Idempotency-Key draft, which the definition of idempotency key cites, lets a resource expire keys and asks it to publish the policy; Stripe keeps keys for 24 hours. etcd compacts its history on request. DL10 already bounds the kept outputs in the same way.

**Who decides.** owner. Each option changes what a purge removes or what OPS-018 promises over time, which is behavior. Order 082's 'What stops a fix' lists 'a change to who may purge, or to what a purge removes' as the owner's (docs/work-orders/082-spec-records-and-installs.md:133).

**Verifier.** weakened, low. Quote verbatim at spec/conventions/operations.yaml:155-156. The facts hold: "No purge touches that state" (:160); DL10 bounds only kept outputs (spec/implementation-defined.yaml:84-88); engine/CONFORMANCE.md:30 "No key expires and invocation state is never compacted". No known-open record holds it; review item 3 (docs/proposals/first-build/to-review-with-the-owner.md:16) covers kept outputs only. But the text decides both situations. Growth: "for every invocation" with no purge and no expiry means kept for as long as the engine runs. Erasure: a purge is defined as removing "records and nothing else" (spec/definitions.md:376; spec/conventions/records.yaml:129), and nothing in spec/ gives purge the purpose of erasing who did what. Dropping dead state is also observable: reconciling an ended invocation reports conflict, and an ID the engine "gave to no invocation" reports invalid (operations.yaml:334-337).

**Verifier's corrections.** Class: not a gap. It is an unstated limit (the engine's own state only grows and no operation shrinks it) and a design cost for the owner. Severity low for v0's first user, one developer on one machine. Who decides stands: options A and C change what a purge removes or what OPS-018 promises (order 082:133), so they are the owner's; option B, the limit sentence, is the coordinator's and is the least change. Drop the erasure situation or mark it as by statement.


## Further points the verifier noticed

Not verified by a second reader.

- **The tracker and order 082 disagree on who settles the storage and race questions** (docs/AUDIT.md:238). The tracker says "Order 082's remaining questions, the coordinator's to settle as defects, logged for the owner to strike". Order 082 counts "a promise on what the engine owes when the storage of its own state fails" among changes that are the owner's (docs/work-orders/082-spec-records-and-installs.md:131-132), and its Record files the same questions "for the owner" (:735-739). The reassignment entered with commit 2a9b25ae, in the coordinator's words, not the owner's. F1, F2 and F8 each lean on one side. Settle the routing in the review's record before any of the three fixes is ordered.
- **SEC-016 cannot be kept together with REC-001 and SEC-015 when no record can be written** (spec/promises/secrets.yaml:411). SEC-016's `when` is "the record of its dispatch cannot be kept" and its `then` "reports unavailable". SEC-015 says "Every read is recorded as a request, whether or not it succeeds" (secrets.yaml:398), and REC-001 exempts only IFC-036's request (spec/conventions/records.yaml:30-32). On a store that cannot be written, the refusal SEC-016 orders is itself unrecorded. This is the one hard contradiction inside F1 and can be fixed alone, by a second exception in REC-001, with no change to DL10.

## The verifier's notes

All nine quotes are verbatim at the cited lines at commit 2eb8f258 (F5's spans a line break). I ran no build and changed nothing.

Result: F2, F3, F6 and F7 confirmed; F1, F4, F5 and F9 weakened; F8 known; none refuted.

Three patterns in the reviewer's work:
- **Known items marked too generously.** F8 is the tracker's item with a fix attached, and F5 is ruling 62 C10 (IN_FORCE.md:178) with a new route, yet it was marked new.
- **Unsafe marks that need a qualifier.** F1's harm needs reading C, which breaks OPS-008 and REC-001 by the letter. F5's needs a Type that keeps outputs, which no core or shipped Type does.
- **Who decides rests on a contested line.** The tracker gives the storage and race questions to the coordinator; order 082 and its Record give them to the owner (first extra).

On the assessment:
- Its main conclusion stands: the records promises support the narrower claim only. F2, F6 and F7 are the new evidence for that.
- "A store that cannot be written has no promise-keeping answer" should read "no answer that is also recorded". Stopping keeps every promise.
- "Races across addresses are decided by nothing for a one-part engine" is correct and already recorded.
- "By the text a purged record is not a gap" is correct, which makes F4 a fault of two vectors, not of spec/.

Not checked: the MC7b sections of experiments/012-model-checking/README.md beyond G5.md's rows, and PRV-021's stop.v1 claim inside F1.
