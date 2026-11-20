# The decision log

Status: draft, not yet in force; [the README](README.md) says what holds until it is.

Each entry below is one decision of the owner's, in the owner's words. Before the review sitting
of 2026-11-20 the coordinator folded rulings 1 to 92 and the owner's answers since into 32
decisions. The owner confirmed them, struck some readings, and answered the open points; the five
on which the sitting raised no question (DEC-007, DEC-010, DEC-013, DEC-014 and DEC-020) were
confirmed together afterwards, the same day. Most answers were given through the session's question tool on 2026-11-20; a quote from an
earlier day names its ruling or date. "Ruling N" cites an archived ruling, whose words are the
owner's.

## DEC-001, 2026-11-20, the decision framework

Owner's words: framework -> "Adopt it (Recommended)"; done means -> "Spec text + passing case
(Rec.)"; numbering -> "New series, and we should try to consolidate the existing rulings into fewer
more concrete rulings and decisions under this new framework"; decision prefix -> "DEC-001
(Rec.)"; frozen records -> "None (Rec.)"; where how-we-work decisions live -> "A separate file
(Recommended)"; HOW-WE-WORK -> "The root, beside AGENTS.md (Rec.)"; the not-done ratchet -> "Fails
(Rec.)"

Lands in: this directory, [spec/CHANGES.md](../../spec/CHANGES.md),
[HOW-WE-WORK.md](../../HOW-WE-WORK.md); gate checks for coupling, the append-only log and the
ratchet (a ROADMAP row)

Status: accepted, not done (these files are drafts; the checks are not written)

## DEC-002, 2026-11-20, what Conduit is for

Owner's words: prose decisions -> "Done by spec text alone (Rec.)"; wording -> "Confirm all
(Rec.)"; and rulings 6, 10, 13, 15 and 17

Lands in: `spec/README.md` "What this is": the five things every change keeps and the nine tests,
in today's words, with "checkout" no longer a shared thing

Status: accepted, not done (the wording is not yet in `spec/`)

## DEC-003, 2026-11-20, innovate only on connection and access

Owner's words: "we should go as far as we can in the 'swappable by user' direction" (ruling 40);
prose decisions -> "Done by spec text alone (Rec.)"

Lands in: `spec/README.md` "Existing standards first"; HOW-WE-WORK rule 3

Status: done (the spec text exists)

## DEC-004, 2026-11-20, the scope guard and v0's scope

Owner's words: ENR/PLC -> "I’m leaning keep them as this is a core part of conduit’s offering, it
should likely be done before the org and split"; enforced -> "Yes; in v0, after S2 (Rec.)";
settled things -> "Confirm all three (Rec.)"

Lands in: a HOW-WE-WORK line: deferring or dropping an accepted capability needs a log entry;
ROADMAP rows for enrollment and placement, and for enforcement after S2

Status: accepted, not done

## DEC-005, 2026-11-20, v0's gate, publication and stewardship

Owner's words: model checking -> "A: check the real core (Rec.)"; releases -> "We still need to
split and reorganize this whole repo after v0 into a GitHub org and everything, was that part of
the plan dropped?"; and rulings 47 and 65

Lands in: ROADMAP rows "Publication" (after v0) and "model checking of the real core" (3 to 5
orders after S2 and the runtime-provider stream, before v0's gate; experiment 012 stays frozen);
a spec line for the safety-vector field (ruling 68)

Status: accepted, not done

## DEC-006, 2026-11-20, the specification's form, layers and names

Owner's words: vectors field -> "Delete it (Rec.)"; and rulings 13, 14, 16, 17, 42 and 78

Lands in: `spec/README.md` "The form of a promise"; `spec/definitions.md`; `spec/names.yaml`;
the promises' `vectors:` field removed (order 220)

Status: accepted, not done (order 220; ruling 78's points 15 and 17)

## DEC-007, 2026-11-20, documents, instantiation, packages and the short form

Owner's words: five DECs (007, 010, 013, 014, 020) -> "Confirm all five (Rec.)"; the words of rulings 8, 18, 21 to 27, 37 and 55
carry its content.

Lands in: OBJ, INS, SUB, NAM, PKG, MAN, CTR, SCH, SCT and FST promises; `spec/core/`

Status: accepted, not done (SCT has no passing case; order 217's S1 example)

## DEC-008, 2026-11-20, plan and apply

Owner's words: prose decisions -> "Done by spec text alone (Rec.)"; and rulings 53, 55 and 76

Lands in: `spec/README.md` "What the short form's meaning covers"; ruling 76's points 1 to 5,
through order 217 and a new spec order

Status: accepted, not done (ruling 76's wording)

## DEC-009, 2026-11-20, operations and unknown outcomes

Owner's words: reconcile.v1 occurred -> "Keep it; false is refused (Rec.)"; spec-detail
readings -> "Confirm the group (Rec.)"

Lands in: OPS, REV, IMP, OBS and REC promises; order 217; a new spec order for ruling 78's
points 6 to 8, F15 and F39

Status: accepted, not done

## DEC-010, 2026-11-20, every result says what happened and why

Owner's words: five DECs (007, 010, 013, 014, 020) -> "Confirm all five (Rec.)"; ruling 52's words carry its content.

Lands in: ACC-056, IFC-050, OPS-023, IFC-025

Status: done (each has a passing case in `engine/STANDING.md`)

## DEC-011, 2026-11-20, machines: placement, nodes, enrollment, peers

Owner's words: ENR/PLC -> "I’m leaning keep them as this is a core part of conduit’s offering, it
should likely be done before the org and split"; settled things -> "Confirm all three (Rec.)";
spec-detail readings -> "Confirm the group (Rec.)"

Lands in: PLC and ENR promises; a new spec order for ruling 78's points 13 and 14 and F72; a
ROADMAP row: enrollment and placement before Publication

Status: accepted, not done (ENR has no passing case)

## DEC-012, 2026-11-20, Conduits, access and lifecycle

Owner's words: spec-detail readings -> "Confirm the group (Rec.)"; and rulings 17, 19, 27 to 30
and 80

Lands in: CON, ACC, LIF and POOL promises; a new spec order for ruling 78's points 10 and 12

Status: accepted, not done

## DEC-013, 2026-11-20, pending actions, the panel and scopes

Owner's words: "I'm not sure we even need a grouping shorthand, declaring a new interface should
be sufficient" (ruling 66); five DECs (007, 010, 013, 014, 020) -> "Confirm all five (Rec.)"

Lands in: PND promises; `manages:` removed (order 217); a new spec order for the panel (ruling
78, point 9)

Status: accepted, not done

## DEC-014, 2026-11-20, secrets

Owner's words: ruled-not-built (46 pt 5 keychain, 77 SSH-M, 69 metrics) -> "Keep pending, tied to
S2/S3 (Rec.)"; five DECs (007, 010, 013, 014, 020) -> "Confirm all five (Rec.)"; and rulings 39 and 41

Lands in: SEC promises; a new spec order for ruling 78's point 11; ROADMAP rows: the keychain
provider with S2, and batch 2's rule on `get-secret`

Status: accepted, not done (SEC has 2 passing of 24)

## DEC-015, 2026-11-20, shared auth, designed after the dogfood

Owner's words: "The whole point of conduit was to streamline logins and have shared
authentication flows." (2026-11-19); "this and how we handle shared auth in general in this way
should be part of the post experiment fable review of the conduit language and system after we do
the dogfood. Multiple accounts will also provide a better test."

Lands in: a ROADMAP row: the post-dogfood review (shared auth, shared Agent contracts, multiple
accounts)

Status: accepted, not done

## DEC-016, 2026-11-20, metrics

Owner's words: batch 2/metrics -> "Tracker rows (Rec.)"

Lands in: a ROADMAP row: metrics, after batch 2 and outside v0's gate

Status: accepted, not done

## DEC-017, 2026-11-20, Fleets, their authority, and coordination

Owner's words: "1 for now is good, we may rework fleets later to allow for coordination, but let's
do 1 and iron out the rest of conduit first"; batch 2/metrics -> "Tracker rows (Rec.)"

Lands in: FLEET promises; a ROADMAP row: batch 2, a Fleet's authority, after the S1 rerun

Status: accepted, not done (batch 2)

## DEC-018, 2026-11-20, what an Agent reaches; enforcement in v0 after S2

Owner's words: enforced -> "Yes; in v0, after S2 (Rec.)"; reader-1 -> "for now to assess conduit
we should allow reads and freedom for agents and discuss the difference between these two"; the
runtime-provider stream -> "Yes, start R-1 now (Rec.)"

Lands in: CTR-014, LIF-013, ACC-027, PRV-006; ROADMAP rows: the runtime-provider stream, and the
sandbox study before any isolation text

Status: accepted, not done

## DEC-019, 2026-11-20, model access: subscriptions now, a gateway route later

Owner's words: "It’s ok if the agents can read the keys for the dogfood, isolation is a later
concern" (ruling 89); keys -> "Keep it; passwords only via secrets (Rec.)"; and ruling 83

Lands in: the `.v2` Agent schemas and NAM-024 (no `apiKey`); a HOW-WE-WORK line: no Agent holds a
model service's API key; a HOW-WE-WORK section 6 line: a password reaches an Agent only through a
declared secret Resource

Status: accepted, not done (order 217's S1 example)

## DEC-020, 2026-11-20, providers and the implementation interface

Owner's words: five DECs (007, 010, 013, 014, 020) -> "Confirm all five (Rec.)"; the words of rulings 31, 41 and 86 carry its
content.

Lands in: IFC, PRV and TPL promises; handles removed (order 217)

Status: accepted, not done (order 217; PRV has 9 passing of 30)

## DEC-021, 2026-11-20, how packages are organized

Owner's words: the per-package split -> "Adopt this split (Recommended)"; agent kit -> "In core,
built at the repo split (Rec.)"; GitHub unit -> "github + git (study's rec)"; package split
amendments -> "Confirm both (Rec.)"

Lands in: a ROADMAP row: Publication, with the split, the agent kit, `github` and `git`

Status: accepted, not done

## DEC-022, 2026-11-20, sign-in through the engine

Owner's words: spec-detail readings -> "Confirm the group (Rec.)"; and rulings 49, 73 and 91

Lands in: PND-004, PND-005, PND-010, PND-011, PND-013, PND-014, IFC-049 and NAM-024; ruling 91
is in `spec/` (#447)

Status: accepted, not done (PND-004 has no case)

## DEC-023, 2026-11-20, sign-in frozen after order 216

Owner's words: sign-in work -> "Land 216 too, then freeze"; freeze end -> "At the post-dogfood
review (Rec.)"; the late check for the expected/no account -> "After the freeze"

Lands in: a ROADMAP row: the waiting sign-in orders, after the post-dogfood review

Status: accepted, not done (the ROADMAP row; order 216 merged, #447)

## DEC-024, 2026-11-20, accounts for testing

Private, held by the coordinator.

## DEC-025, 2026-11-20, the engine is Rust; no private code

Owner's words: "This should have all been in the rust engine"; the Workflow tool's JavaScript ->
"Allow, named exception (Recommended)"; leftovers -> "Port scenario-s4’s install.sh to Rust"

Lands in: HOW-WE-WORK rule 2; a ROADMAP row: port that script, then check for non-Rust code

Status: accepted, not done

## DEC-026, 2026-11-20, proof: cases, experiments, measurement

Owner's words: the 136 promises -> "Sort and ratchet (Rec.)"; disputed case -> "Mark it disputed
(Rec.)"; reviewers -> "Gate alone (Rec.)"; a second implementation -> "Keep it, later (Rec.)";
experiment 012 -> "Freeze it (Rec.)"

Lands in: HOW-WE-WORK section 4; `tools/promise_standing.py` (the ratchet, `disputed`); ROADMAP
rows

Status: accepted, not done

## DEC-027, 2026-11-20, Conduit builds Conduit, directly through Conduit

Owner's words: how the dogfood runs orders -> "Directly through Conduit (Recommended)"; milestone
1 -> "After the Claude reader’s first read"; the two streams’ order lists -> "Approve as listed
(Rec.)"

Lands in: HOW-WE-WORK section 5; ROADMAP rows for the milestones

Status: accepted, not done (milestone 1 is the Claude reader's read of order 217)

## DEC-028, 2026-11-20, ruling 92's rules

Owner's words: the going-forward rules as one ruling -> "Adopt all eight (Recommended)"; and
ruling 92

Lands in: HOW-WE-WORK section 1; each rule moves from review to a Conduit refusal as the dogfood
allows

Status: accepted, not done (every rule is checked by review)

## DEC-029, 2026-11-20, who decides and who does

Owner's words: my logged decisions -> "Yes, until you confirm or strike (Rec.)"; and rulings 37,
79 and 92

Lands in: HOW-WE-WORK section 2

Status: accepted, not done

## DEC-030, 2026-11-20, how an order runs and merges

Owner's words: orders/records readings -> "Confirm, strike overruled parts (Rec.)"; A77 ->
"Resume inside the dogfood (Rec.)"; two process rules -> "As you propose (Rec.)"; on whether an
order keeps its verifier turn: "Keep the verifier for now, but we should discuss later the most
efficient way to set up fleets to work on conduit"

Lands in: HOW-WE-WORK section 3; AGENTS.md at most 150 lines, gate-checked; a ROADMAP row for
A77

Status: accepted, not done

## DEC-031, 2026-11-20, records, the archive and the documents

Owner's words: archive approach -> "Git is the archive (Rec.)"; archive clean groups -> "A: now
(Rec.), B: at the freeze (Rec.), C: at retirement (Rec.), D: finished experiments"; GitHub
issues/tracking -> "We should keep everything in the repo for now ... We can discuss this scheme
later when everything becomes a proper GitHub org"

Lands in: HOW-WE-WORK section 8; [the archive's index](../archive/README.md); ROADMAP rows

Status: accepted, not done (each group is still in the tree)

## DEC-032, 2026-11-20, the owner's machines

Private, held by the coordinator.

## DEC-033, 2026-11-20, the testing audit

Owner's words: "We should also do an audit of our testing infrastructure, what is and isn't
needed, and how to keep it minimal until v0 is shipped to improve velocity"; then gate speed-ups
-> "Yes, all (Rec.)"; suite reruns -> "Trim now (Rec.)"; the 012 account rule -> "Allow it
(Rec.)"; what gates v0 -> "DEC-005 + mutation run as info (Rec.)"

Lands in: AGENTS.md's work-order rules; HOW-WE-WORK section 3; work order 223 (the gate's
speed-ups); ROADMAP.md's "What v0 must pass" (ruling 47's six gate v0; mutation testing runs once
on the core before the owner's judgment, as information; independent builds inform after)

Status: accepted, not done (order 223)
