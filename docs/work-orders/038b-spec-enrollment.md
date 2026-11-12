# Work order 038b (E8 spec): enrollment in the specification

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access, to fetch
crates and for tests that bind a loopback port; use it for nothing else. Never print a whole large
file or log: use `head`, `tail`, `grep` or a count. Run one cargo command at a time.

## What and why

Machine enrollment is ruled but not yet written. A machine that joins the machine provider's
network becomes a node only when root admits it, on evidence the provider reports and a minimum
the engine enforces; ruling 68, point 3 (R2) adds that the person types the fingerprint read on
the machine and the engine compares it. This order writes enrollment into `spec/`, as SPEC-C2 of
[the batch 1 plan](../proposals/spec-batch-1/README.md) (its order 038b, lines 457 to 465, about
1 to 1.2k lines). It stages, as exact patches, the edits the engine would reject today, for build
orders 049a and 049b. It is off the rerun path. [IN_FORCE.md](../../IN_FORCE.md) holds the
decisions as *spec pending* at lines 215 to 222, and names this order at lines 182 and 210.

Line numbers in this order are those of `main` at cfd7b23. Where a line has moved since, find it
by the words quoted.

Read, in this order:

1. The plan: "The order of the edits" (038b at line 225; conflicts C1, C5 to C8, C11, C25 and
   C27); "The work orders" for 038b (line 457), 049a (line 581), 049b (line 590) and 051c
   (line 639); its question R2 (line 131); and its readings 18 and 19 (lines 183 and 184).
2. The area draft [machines-and-trust](../proposals/spec-batch-1/areas/machines-and-trust.md):
   the drafts of ENR-001 to ENR-012 (section A), the promises it changes (B), the core package
   (C), the definitions (D), `spec/README.md` (E), its vectors, and its coordinator choices C1
   to C8. Its paths under `reference-model/` are under `engine/` now. Its "Build" section is
   for 049a, 049b and 051c, not for this order.
3. What order 038a wrote, on which this order builds: `spec/promises/pending.yaml`, IFC-049,
   ACC-056, IFC-050, and `evidence-item.v1` and `pending-action.v1` in
   `spec/core/results.yaml`. Then [the K1 check](../proposals/spec-batch-1/k1-mcp-elicitation.md)
   of the protocol: where it differs from a draft, K1 governs.
4. The records of rulings 50, 51, 52, 56 and 66, and [ruling 68](../proposals/ruling-68/README.md),
   point 3 (lines 40 to 49).
5. How orders 036b and 038a staged edits: [036b-staged/](036b-staged/README.md), with its
   `049a.md`, `049b.md` and `048a.md`.

The drafts are drafts. Write each item in the specification's form and terms
([CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises"; `spec/definitions.md`), with the
meaning the plan gives it. Where a draft and a ruling's record differ, the record governs: write
the record's meaning and report the difference.

## What to do

1. **Write the promises.**
   - In a new `spec/promises/enrollment.yaml`: ENR-001 to ENR-012.
   - In `spec/promises/pending.yaml`: PND-008, for the kinds `enrollment`, `machine` and
     `machine-removal` (C6), with the subjects step 2 gives. Update the text that still reserves
     them: the head comment (line 3), PND-001 (lines 16 and 17), PND-003 (line 52), PND-004
     (line 80) and PND-009 (line 102). In PND-002, "Peer delivery waits on R3 and is not defined
     here" (line 41) is stale: R3 is answered, and peer delivery is written in batch 2, with
     appointment, keep and withhold (ruling 68, point 3). Say so in its place.
   - Amend, as the area's section B drafts them: PLC-026 (rewritten), PLC-008, PLC-017, PLC-023,
     PLC-024 and PLC-028; PRV-003, PRV-005, PRV-013, PRV-014, PRV-015 and PRV-018; FST-001 and
     FST-006; ACC-032 (five operations reserved to root); REC-001; and DL5 (the longest an
     enrollment may last, beside its timing of reports). Amend IFC-049 where `enroll-machine.v1`
     needs it (C7; step 2, "The join action").
   - **Labels.** ENR-010 fixes a node's labels at admission, and these texts still treat them as
     reported status:
     - PLC-007: "the labels reported for that node" (`spec/promises/placement.yaml` line 90) and
       "a later report of other labels changes no instance placed on the node" (line 112);
     - PLC-013: "labels reported for its consumer's node and for its target's node" (line 182);
     - PLC-026: "those the provider last reported for its machine" (line 386) and "Labels are
       reported status and never configuration: a report of other labels changes the node's
       status" (lines 392 and 393);
     - PRV-014: "Labels are reported status and never configuration" (`providers.yaml` line 410);
     - the head comment of `placement.yaml` (lines 2 to 4).

     Make each agree with ENR-010. PLC-025, which the plan and IN_FORCE.md line 215 name, is
     about the selectors of a type and its items that disagree; nothing in enrollment touches
     it, and PLC-007 was probably meant. Leave PLC-025 unless you find a reason to change it, and
     say in your report what you did.
   - Order 036a wrote the trust domain: FST-007, DL6, DL13 and PRV-022. Cite them; do not
     rewrite them.
2. **Apply these decisions.** Do not reopen them. Each is cited to its source.
   - **R2** (ruling 68, point 3, which accepted the plan's option (a), line 137): `admit-machine.v1`
     takes `fingerprint`, the value the person read on the machine. The engine compares it with
     the `fingerprint` item the provider reported for that machine. Every admission needs it,
     with one exception: an enrollment opened `headless` with `acceptWeakerCheck`. Attestation
     does not replace it. Write R2 in three places:
     - **ENR-001, at opening.** The evidence kinds that the provider declares in `enroll.v1`'s
       result must include:
       - `fingerprint`, for an enrollment that is not `headless`;
       - `fingerprint` and `attestation`, for a `headless` enrollment without
         `acceptWeakerCheck`;
       - ENR-005's minimum, `fingerprint` or `attestation`, for a `headless` enrollment with the
         flag.

       `require: [attestation]` adds `attestation` to any of these. Where the declared kinds fall
       short, the request ends failed with a diagnostic naming the missing kinds: no action is
       shown, no pending action is raised and no enrollment stays open, as the draft's ENR-001
       says for a missing attestation. Without this, an enrollment through a provider that
       declares only `attestation` would open, and its machine could never be admitted.
     - **ENR-006, at admission.** Two checks replace the draft's check 7, except on a `headless`
       enrollment with the flag:
       - The provider reports one `fingerprint` item for the machine. None, or more than one,
         is a failed check that reports conflict, with a diagnostic naming it.
       - The request carries `fingerprint`, and it equals that item's `value`. A missing or
         unequal input reports invalid, with a diagnostic naming the check.

       Where both fail, OPS-023 decides which reason is reported. No diagnostic holds either
       value. On a `headless` enrollment with the flag, check 5's minimum applies and nothing is
       compared; say what a `fingerprint` given on such an admission does, and report it.
     - **The comparison.** Equal means equal character for character. The engine folds no case
       and trims nothing. ENR-002's list of what the provider's contract states therefore
       includes the fingerprint's alphabet, and ENR-005's "the same on the machine and in the
       panel" holds the panel to the value as the provider reported it.

     A `headless` enrollment without the flag needs attestation (ruling 56, point 9) and the
     typed fingerprint, read where the item's `where` says, for example over a remote shell.
     Ruling 68's words, "a machine with no screen is admitted only with `--accept-weaker-check`,
     recorded" (`ruling-68/README.md` lines 44 and 45), cover a machine on which the person can
     read no fingerprint at all: it meets R2 only with the flag. This narrows ruling 56,
     point 9's "Attestation meets the minimum": attestation alone no longer admits such a
     machine. Report this as a reading. "The provider reports one `fingerprint` item" is the
     admin's reading: report it as applied.
   - **`require`** keeps only `attestation`, since R2 already requires the fingerprint wherever
     a person can read it. This is the admin's reading: report it as applied.
   - **The flag** (ruling 56, point 9, as R2 narrows it): `acceptWeakerCheck` is valid only with
     `headless`; given without it, the request reports invalid (the draft's ENR-001). The flag is
     recorded on the enrollment and in the answer's record, and it is the named flag
     `--accept-weaker-check` that order 051c counts (ruling 55, A4).
   - **No code or fingerprint on the wire** (C5, PND-004, PND-012): the typed fingerprint is in
     no record of the request, result, diagnostic or `_meta`. The answer's record holds the
     evidence shown, and whether the fingerprint was compared or the weaker check accepted. A
     repeat is matched under OPS-009 and OPS-010 by the digest of the request's contents that
     OPS-008 keeps as the engine's own state, apart from the records (`spec/definitions.md`
     line 217, "the invocation's state"). That digest covers the typed fingerprint, and it is
     the only thing derived from it that the engine keeps. Say so.
   - **The join action** (C7, ENR-003, IFC-049): no part of `enroll.v1`'s action is in a record,
     result, inspection, observation, diagnostic or `_meta`. A `url` action that carries no
     secret is relayed under IFC-049. An action that carries a secret, and every `command`
     action, is shown only in the engine's own panel, because IFC-049 forbids a URL that is
     pre-authenticated or holds a credential. The result of `enroll-machine.v1` gives only the
     enrollment's ID and the pending action's ID.

     IFC-049 (`spec/interface/implementation.yaml` lines 1348 to 1379) defines a retry only for
     sign-in, which polls under PND-010. Write what it means for `enroll-machine.v1`. The
     admin's reading, to report as applied: the enrollment opens and its pending action is
     raised once `enroll.v1` succeeds, and nothing the client does afterwards changes either. A
     retry, whether it accepts, declines or cancels, gets the two IDs at once, with no polling,
     and the invocation succeeds; the person sees the join at the panel (ENR-004). If no retry
     comes within DL18, the invocation ends succeeded with the same outputs, which a repeat
     gets under OPS-009. A request that does not declare URL elicitation gets the two IDs at
     once, with no -32021, and the action is shown only at the panel.
   - **Root alone** (ruling 56, point 11; ruling 66, point 3): the three verbs are marked
     `reservedToRoot`, and enrollment actions are raised at root alone. PND-008 states each
     kind's subjects (PND-001, PND-002; `spec/definitions.md` line 163, "scope"). A pending
     machine is not an object, so it is no subject:
     - kinds `enrollment` and `machine` name the machine provider Resource;
     - kind `machine-removal` names the node.

     `system` owns both (PRV-013, PLC-026), and `system` is in no scope, so no Interface but
     root is shown these actions.
   - **Peers:** write no sentence about them. Ruling 68, point 3 (R3) keeps the operations
     reserved to root with the peer's own root, and `reservedToRoot` already does that: ACC-032
     dispatches a reserved operation only for the root Interface.
   - **A node's binding** (reading 19; the area's O2): a node represents the triple of provider
     Resource, machine `id` and hostname. PLC-026's sentences that a node represents a hostname
     "whichever machine provider reports it" go. Replacing the provider leaves every node omitted
     until its machine is enrolled again (PRV-013).
   - **The evidence minimum** (ruling 56, points 2 and 10; C5; the plan's fidelity fixes 6 and 7,
     line 700): `id` is required, opaque, scoped to the provider, and derived from the machine's
     key where the provider offers one (ENR-002). At least one item is a `fingerprint` or an
     `attestation` (ENR-005). `where` is required. A fingerprint has 6 to 8 characters.
   - **Edited in place** (reading 18; the area's C8): `list-machines.v1` and `place.v1` change in
     place at version 0.1.0. Both edits are staged for 049b (step 5), so the sentence of
     `spec/core/README.md` that records this exception goes into `049b.patch` with them. No live
     text describes an edit that is staged.
   - **Nodes get no tombstone** (C27): LIF-040 already exempts what the engine makes, and a node
     made on admission is one. A re-admitted machine's node may therefore take its hostname
     again. Say so where the draft's PLC-024 or PLC-026 needs it.
   - **The area's choices C1 to C4 and C7** (the ENR file; the operation names; expiry removes
     nothing at the provider; the removal question also on a retirement by request; labels only
     the three descriptive ones plus the enrollment's): write them as drafted, and list each in
     your report as a reading the owner may strike.
   - **Explicit refusals** (ruling 52; ACC-056; IFC-050): every reason the new verbs report has
     a diagnostic naming its deciding rule and cause. ENR-006 gives one diagnostic per failed
     check. No diagnostic holds a code, a fingerprint or any part of the join action.
3. **Write the contracts** of the area's section C and the plan's line 72:
   - the root verbs `enroll-machine.v1`, `admit-machine.v1` and `refuse-machine.v1`,
     implemented by the engine, not `deduplicates`, with descriptions that name what they do
     (IFC-033);
   - the provider operations `enroll.v1`, `accept-machine.v1` and `remove-machine.v1`;
   - the changes to `list-machines.v1` (`id` required; the state `pending`; `enrollment`;
     `evidence`), `place.v1` (the output `id`) and `declare-lost.v1`'s description;
   - `object-management.v1` and `machine-provider.v1` (three operations each), `management.v1`'s
     `requires`, `node.v1`, the templates `node`, `system` and `root-management`, and the
     exports and the item `engine` in `spec/core/conduit.yaml`.

   `evidence-item.v1` and `trust-domain.v1` exist already: change `evidence-item.v1` only if a
   promise needs it. Each file's head comment cites the promises its contracts encode, as the
   other entries there do.
4. **Definitions and READMEs.**
   - `spec/definitions.md`: the rows of the area's section D, except those for the trust domain,
     which order 036a wrote. Among them: node (lines 48 and 124), represent, machine, machine
     provider, label, provider call, prompt (line 269), reserved to the root Interface (line 150,
     "The two are"), and the facts row (line 425), which must now say that `fact` returns only as
     an evidence kind.
   - `spec/README.md`: leave the Status paragraph (lines 3 to 11) as it is. Order 071 edits its
     lines 7 and 8, and an edit beside them would conflict at merge. Write the status instead in
     the Contents row for `promises/enrollment.yaml`, in every count you change, and in the limits
     at lines 420 to 424: managed mode now points to its promises; R3 is answered, so peer mode
     is written in batch 2, with appointment, keep and withhold (ruling 68, point 3).
   - `spec/core/README.md`: the export counts (lines 29 and 32), its tables of operations and of
     every export, and lines 42 to 44 on staged edits: make them also name `038b-staged/` and
     orders 049a and 049b. The exception of reading 18 goes into `049b.patch` (step 2).
5. **Stage what the engine would reject.** C25: a spec order lands only edits that keep the
   engine's tests green, and stages the rest verbatim.
   - **For 049a:** the three root verbs wherever a list names them: `root-management`'s
     operations and description (C25; its five first-start cases change with it, C11), and with
     it `object-management.v1` and its file's head comment, `management.v1`'s `requires`, and the
     engine's `implements` in `conduit.yaml`. These move together: `tools/check_schemas.py`
     lines 1813 to 1835 (TPL-004) require the engine to implement every operation that
     `management.v1` requires or that `object-management.v1` lists. No live text may say that the
     engine offers a verb it has not built, and the rerun keeps no stub (C11).
   - **Order 072's shell test:** 072 adds
     `every_engine_operation_is_executable_or_unfinished_with_its_order`, which compares
     `implements` with the shell's executable and unfinished lists. It is not on `main` at
     cfd7b23. Staging `implements` keeps it green; do not change it if your tree has it.
     `049a.md` must say that landing `implements` obliges 049a to add the three verbs to one of
     the shell's two lists.
   - **For 049b:**
     - the edits to `list-machines.v1`, `place.v1` and `machine-provider.v1` (C25);
     - the sentence of `spec/core/README.md` that records reading 18's exception;
     - in `tools/check_schemas.py`, the three provider operations added to `CALLED_OPERATIONS`
       (lines 167 to 169). That tuple drives PRV-004's check (lines 2093 to 2109) and PKG-020's
       and PKG-030's (lines 2155 to 2167). It changes with `machine-provider.v1`, so that the
       checker and the engine agree before and after 049b. The engine's own check reads the
       capability (`engine/crates/core/src/packages.rs` lines 903 to 911), so it follows the
       staged edit with no change. Say so in `049b.md`.
   - **Independent patches:** the plan's 049a builds the MachineReport input and the
     provider-call intents (lines 581 to 588), and may need some of 049b's hunks. Make
     `049a.patch` and `049b.patch` each apply alone after
     `docs/work-orders/036b-staged/048a.patch`, and both apply together in either order. Where
     two hunks sit too close for that, move the edit into `049a.patch` and say why in both
     handoffs.
   - **Live:** everything else, including the six new contract documents and their exports,
     where the engine's tests and the checks pass. Stage anything they reject, for the build
     order that lands it.
   - Write a promise that names a staged contract as the plan gives it. If a check then fails,
     stage that sentence too and report it.
   - **The form:** a new `docs/work-orders/038b-staged/`, laid out as `036b-staged/` is:
     - `049a.md` and `049b.md`, each linking its exact `049a.patch` or `049b.patch`: unified
       diffs from the repository root, made against your finished tree with
       `docs/work-orders/036b-staged/048a.patch` applied;
     - a `.gitattributes` holding `*.patch -whitespace`;
     - a `README.md`;
     - `vector-impact.md` (step 7).
   - Each handoff also lists the vectors that the area draft plans for its order, corrected for
     the text you wrote, each with the promises it tests. R2 changes cases 5, 9, 10 and 11: a
     provider that declares only `attestation` fails at opening; attestation without a typed
     fingerprint is no longer admitted; and a machine with no reported `fingerprint` item is a
     conflict.
   - In `docs/work-orders/036b-staged/049a.md` and `049b.md`, add one line each pointing to the
     new files.
   - **The scratch check:** copy your working tree, with your changes and without
     `engine/target/`, to a directory outside the repository. There, apply each of these
     sequences to a fresh copy, running `git apply --check` and then `git apply` for each patch
     in turn:
     1. `036b-staged/048a.patch`, `038b-staged/049a.patch`;
     2. `036b-staged/048a.patch`, `038b-staged/049b.patch`;
     3. `036b-staged/048a.patch`, `038b-staged/049a.patch`, `038b-staged/049b.patch`;
     4. `036b-staged/048a.patch`, `038b-staged/049b.patch`, `038b-staged/049a.patch`.

     After each sequence, run `python3 tools/check_schemas.py` and `python3 tools/check_docs.py`
     in that copy. Report each command's last line, then delete the copies. Write their location
     in no file.
6. **IN_FORCE.md.** A *spec pending* line that this order writes in full becomes a pointer to
   the promises that now hold it, in that file's form. A line written in part, or held by a
   staged edit, says which part remains and which order lands it. The lines are:
   - 182, in topic 9: "Enrollment kinds remain for 038b". Its "peer delivery waits on R3" is
     stale too: peer delivery is written in batch 2 (68 point 3);
   - 210: "Managed enrollment remains for 038b";
   - 215 to 222: topic 10's enrollment line and its sub-lines. "This changes PLC-025 and
     PLC-026" names what you actually changed. The Tailscale line (222) stays with its provider
     work, as it stands, including A74 and any sentence a later ruling added to it.

   Line 221's last sentence, "A machine with no screen needs attestation, or the explicit flag
   `--accept-weaker-check`, which is recorded", reads as if attestation alone admits such a
   machine. R2 supersedes it: the owner's words in ruling 68, point 3 govern (IN_FORCE.md line
   5). The pointer that replaces it says that a machine with no screen is admitted only with
   `--accept-weaker-check`, recorded, and cites ENR-006. Do not keep the old sentence, and do
   not write ENR-006 to match it.
7. **Change no vector.** List, by path and zero-based step, every existing expectation that the
   new text contradicts. Put the list in `038b-staged/vector-impact.md` and in your report's
   findings. Start from the area draft's "Existing vectors whose outcome changes", and check each
   one against today's tree: order 040 already made several of them (`trustDomain`).
8. **Before you finish, check** for the faults behind batch 1's second rounds:
   - every request that is not dispatched names exactly one of refused, invalid, conflict and
     unavailable, in OPS-023's order (CONTRIBUTING.md, rule 5), ENR-006's cases among them;
   - what each caller may see, by ACC-056's tiers, OPS-036, LIF-013 and PND-002: no caller but
     root's panel learns of a pending machine's evidence, a code, a fingerprint or the join
     action;
   - every case of each promise: each check of ENR-006; each kind of PND-008 and each of its
     answers; expiry before and after a join; a retired or replaced provider;
   - every closed list that the new text extends:
     - the operations reserved to the root Interface: ACC-032 (`spec/conventions/access.yaml`
       line 572), CTR-009 ("as `declare-lost.v1` and `purge.v1` do",
       `spec/promises/contracts.yaml` line 234), `spec/definitions.md` line 150 and
       `spec/core/capabilities.yaml`'s head comment (lines 8 and 9). A list of the contracts
       that carry the mark counts the live contracts. A list of what a capability lists, or of
       what the engine offers, changes in the patch that changes that capability;
     - the documented kinds (PND-001) and the answers that `answer.v1` accepts (PND-009);
     - every count in `spec/README.md` and `spec/core/README.md`, recounted from the files;
   - no trailing white space in Markdown, and every patch exact.

## Scope

Change:

- in `spec/`:
  - `spec/promises/`: the new `enrollment.yaml`, and the promises step 1 names;
  - `spec/conventions/access.yaml` (ACC-032) and `spec/conventions/records.yaml` (REC-001);
  - `spec/interface/implementation.yaml` (IFC-049);
  - `spec/implementation-defined.yaml` (DL5);
  - `spec/definitions.md`;
  - `spec/README.md`, except lines 3 to 11;
  - `spec/core/README.md`;
  - the other files under `spec/core/` and `spec/schemas/`: only what lands live by step 5;
- `docs/work-orders/038b-staged/`, which you create;
- one line each in `docs/work-orders/036b-staged/049a.md` and `049b.md`;
- `IN_FORCE.md`, at the lines step 6 names.

Do not change:

- `engine/`, `conformance/`, `packages/` or `tools/`. The checker change of step 5 goes into
  `049b.patch`, and any other checker change the staged text needs goes into a patch too;
- `spec/names.yaml`: report any name the text needs;
- FST-007, DL6, DL13 or PRV-022, except to cite them;
- the patches in `docs/work-orders/036b-staged/`, or `docs/work-orders/README.md`;
- anything for peer engines or for the grant review and keep, which batch 2 writes;
- a record: the rulings' records, the ledger, `docs/AUDIT.md`, `ROADMAP.md`,
  `docs/implementation/` and `docs/proposals/`.

Report anything else that needs changing.

## Done when

Every item the plan assigns to 038b is written, or staged, or reported absent with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD`, with that interpreter: 0 changes. This order changes no
  vector, so it writes no declaration
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `git diff --check`

In the scratch copies of step 5: for each of the four sequences, `git apply --check` and
`git apply` for each patch in turn, then `python3 tools/check_schemas.py` and
`python3 tools/check_docs.py`.

In `engine/`:

- While you decide what lands live and what is staged, run
  `cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance` as often as you
  need. These crates read `spec/core/` and `spec/schemas/`.
- Once the split is final, run `cargo test --release` once. If it fails because of a live edit,
  stage that edit and run it once more.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers
only. If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load
(finding 4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail
in your sandbox: say so, name each, and the admin reruns them.

Report:

- the promises added and amended, by ID;
- the contracts added and changed, each marked live or staged, with the order that lands it;
- every reading you applied, and every difference between a draft and a ruling's record;
- every expectation the new text contradicts, by path and step;
- each check's last line.

Do not `git add` or commit anything.

## Record

Run on 2026-11-11 on a heavily loaded Mac: the worker in 86 minutes, then three rounds, each fixing
what a fresh verifier found. The enrollment promises ENR-001 to ENR-012 landed in
`spec/promises/enrollment.yaml`, with PND-008, the edits to PLC-023, PLC-026, PRV-014, PRV-015 and
ACC-032, the six operation contracts, and staged patches for orders 049a and 049b. They follow
ruling 68's R2: admission requires the fingerprint the person read on the machine, compared
character for character, and a machine with no screen is admitted only with
`--accept-weaker-check`, recorded.
- Round 2 fixed an action-ID rule that required an invocation when a machine action is raised
  without one, and an enrollment URL flow that contradicted IFC-016's relay rule.
- Round 3's verifier found one more: two definitions in `spec/definitions.md` still said the engine
  makes a node from a provider's report. The admin fixed both: a node is made only at first start
  or on admission, and a report makes none.
- The coordinator's readings applied: a headless machine without the flag is admitted with
  attestation and a fingerprint read remotely; the provider reports one `fingerprint` item; a
  decline under IFC-049's retry is the worker's reading as reported. Ruling 68's amendment of ruling
  56 point 9 was recorded separately.
- Checks: the documentation, rulings, names and schemas checks passed; in round 3 the engine's full
  `cargo test --release` passed in the worker's sandbox.
