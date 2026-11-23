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

Status: done (ruling 76 point 1 by order 217, #453; points 2 to 5 by order 240)

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

## DEC-034, 2026-11-20, the github package's server and its credentials

Owner's words: GitHub implementation -> "Wrap github-mcp-server (Recommended)"; push
credential -> "Working copy references the secret (Rec.)"; token kind -> "Fine-grained personal
token"

Lands in: work order 232 (the server and the token); the `git` package's push, a slice after
order 229

Status: accepted, not done (order 232)

## DEC-035, 2026-11-20, the packages' public address and platforms

Owner's words: public address -> "Stay on example.org until first publish (Rec.)"; platforms
(after the explanation) -> "Decide at first publish"

Lands in: the packages' addresses under `example.org/conduit/` until first publish; the platforms
each package ships for, at first publish

Status: accepted, not done (first publish)

## DEC-036, 2026-11-20, the keychain store's scope

Owner's words: which keychains may the store read -> "Discuss"; after the comparison -> "A:
dedicated only (Rec.)"

Lands in: order 224 (the keychain store reads a dedicated keychain only, and always refuses the
login keychain and the default keychain)

Status: done (order 224)

## DEC-037, 2026-11-21, the post-dogfood review

Owner's words: when -> "Tomorrow, 11-22 (Recommended)"; format -> "Yes, as proposed
(Recommended)"

Lands in: a Fable review of the language and system on 2026-11-22, its report handed to the
coordinator outside the tree; experiment 028's stage 2 ends by the owner's word once orders
236 to 239 are in

Status: accepted, not done (the review)

## DEC-038, 2026-11-21, the S1 rerun waits for the simplification chain

Owner's words: the pin lacks the simplification the rerun measures -> "Hold it; write the chain
now (Recommended)"

Lands in: ROADMAP step 3 (the rerun's build is a new pin from main, after the chain); work
orders 240 to 250; the pin `s1-rerun-pin` is not the rerun's build

Status: accepted, not done (orders 240 to 250, the new pin)

## DEC-039, 2026-11-21, the chain's scope

Owner's words: 051a after the pin, its cascades as apply's flags, render dropped, the pin waits
for 249 -> "Yes, as written (Recommended)"; the S1 sectioned-package example -> "Replace with a
non-S1 package (Recommended)"; init's skeleton and install's pin rule -> "Decide at tomorrow's
review"

Lands in: cites DEC-004 and DEC-008; work orders 240 (the example replaced), 249 and 251; order
243 waits for the review's answer

Status: accepted, not done (orders 240, 249, 251)

## DEC-040, 2026-11-21, the S1 chain's init, templates and pins

Owner's words: bare `conduit init` -> "Skeleton manifest only (Recommended)"; root's turn Conduits -> "Templates in Agent pkgs 0.2.2 (Recommended)"; install's pin rule -> "We should research resolvers as this is a key part of conduit, we should not build hacks around this"

Lands in: work order 243 (init writes only `apiVersion` and `packages: {}`; no pin writing), work order 253 (the Agent packages' `turn` and `conversation` templates); a resolver design after research, before the S1 rerun's pin

Status: accepted, not done (orders 243, 253; the resolver)

## DEC-041, 2026-11-21, model checking's first slice now

Owner's words: order 237 -> "First slice now, as filler (Recommended)"

Lands in: DEC-005's sequence read as priority for 237's first slice only; later slices after S2 and the runtime-provider stream

Status: accepted, not done (order 237)

## DEC-042, 2026-11-21, model logins are Pool-owned Resources

Owner's words: a model login as a Pool-owned Resource -> "Pool-owned login Resource (Recommended)"; the Claude half -> "Account home per machine (Recommended)"; the Codex half -> "No business plan, what? How would we support our existing accounts for pro? This is not a real solution we should support every subscription type"; the second-account sitting -> "Hold it; decouple the S1 pin (Recommended)"

Lands in: IN_FORCE.md topic 1 ("subscription and context are its own" becomes "context is its own; its login may be a Pool's Resource"); a spec order (CTR-013, the .v2 Types' login place; retire and sign-out hooks log out only a login the instance owns); package orders for both Agent packages, for every subscription type; the second-account sitting after them; the S1 rerun's pin does not wait for it

Status: accepted, not done (the spec and package orders, the sitting)

## DEC-043, 2026-11-21, the sign-in freeze ends in sequence

Owner's words: the freeze's end -> "Lift in sequence (Recommended)"; late account checks -> "Late answers change nothing (Recommended)"

Lands in: cites DEC-023; PND and OPS-013 (a late answer leaves an ended sign-in as it ended, and its diagnostic names the way out); the carried-return orders wait for a remote node's need (A81); the Fleet-wide sign-in command is closed

Status: accepted, not done (a spec and engine order)

## DEC-044, 2026-11-21, records fail closed, and research before deciding

Owner's words: SEC-016 -> "Unavailable now, record late (Recommended)"; any request whose record cannot be kept -> "Can this work in parallel? Then we should do spec and engine now"; a remote runtime's program (PRV-007) -> "Research it first"

Lands in: cites ruling 41 and DEC-014: SEC-016 becomes one case of a general rule (a request not yet dispatched whose record cannot be kept reports unavailable, naming the promise, and is recorded once records can be kept; a dispatched one whose end cannot be recorded ends unknown at recovery); SEC-024's IPv4 is RFC 3986's IPv4address; a spec order and an engine order now; PRV-007 after its research

Status: accepted, not done (the spec and engine orders; the research)

## DEC-045, 2026-11-21, the shared Agent contracts, the schedule, and stage 2's record

Owner's words: where the shared Agent contracts live -> "A contracts-only agent package (Recommended)"; the schedule -> "Add capacity first"; stage 2's readings -> "Confirm both (Recommended)"

Lands in: a package rule: the core package exports only what the engine implements, invokes or needs at first start; an `agent` package at the address move (DEC-035); the login and package orders and the second-account sitting come before the S1 chain where they contend; experiment 028's stage 2 results stand

Status: accepted, not done (the agent package; the capacity work)

## DEC-046, 2026-11-21, the Fleet grows on every account, by measurement

Owner's words: the account map -> "Let’s discuss, we should use all the accounts for the test and to maximize agents for the fleet", then "Yes, grow by measurement (Recommended)"; Codex logins -> "What is this framing, we can have as many accounts and as many agents per account as we want right?", then "One login per account, shared"; the reuse rule -> "OpenAI's word + live watch (Recommended)"; at a limit -> "Pause that account (Recommended)"; reads -> "Both at once, blind (Recommended)"

Lands in: cites DEC-042 and DEC-045; every account runs Agents: ChatGPT accounts worker pairs, two each to start, Claude accounts readers and a drafter, a pair added at a time while an account's week, the machine and the gate have room; the S3, S7 and S10 tests share the accounts; one Codex login per account per machine, a Pool's `codex-login` Resource linked into each Agent's own home, turns in parallel, a refresh failure that fails a turn raising one sign-in action and the login then taking one turn at a time until lifted, OpenAI's rule from its own word and the live login; a usage limit pauses only that account's lanes until its reset, and work never moves between accounts; for A77's ten orders the verifier and the reader read at once, blind, with one fix turn (amends rulings 79 and 87)

Status: accepted, not done (the login spec, engine and package orders; the sitting)

## DEC-047, 2026-11-21, usage in records, and harnesses in role templates

Owner's words: usage -> "Engine keeps them now (Recommended)"; the harness -> "Role templates + inputs (Recommended)"

Lands in: amends DEC-016's order for one measure: from the first Agent-package release after DEC-046's login, each turn's result carries the tokens its client reported, the model and the client version, and the engine keeps them in that call's end record (ruling 68 M1); numbers decide nothing in the engine; both Agent packages gain optional `instructions` and `outputSchema` inputs, mapped to each client's flags; pinned role templates fix each role's text, report schema, model and effort; auto memory, hooks, plugins, in-client subagents and repository discovery stay off

Status: accepted, not done (a spec order, an engine order and the package orders)

## DEC-048, 2026-11-21, exact pins and a closure walk

Owner's words: the model -> "Exact digests + closure walk (Recommended)"; install's pin -> "Exact, up or down (Recommended)"; dependencies -> "Install the closure (Recommended)"; sources -> "Cache, file: fetch, mirrors (Recommended)"; S1's T2 -> "As written (Recommended)"; https -> "Nothing remote in v0 (Recommended)"

Lands in: cites DEC-040 and ends B6's provisional rule: a manifest's exact pins are its lock, each package resolving through its own (NAM-014), with no lock file while there are no ranges; `conduit install ARCHIVE#HEX` pins that archive exactly, up or down, after installing its missing closure, each by its own printed install.v1, with `--dry-run`; a bare install restores; archives come from the cache, a `file:` fetch, then mirrors in the cache's layout; S1's T2 is one line (34/40), a tooling change; https waits for the first remote root

Status: accepted, not done (a CLI order beside 245; orders 246 and 250 amended)

## DEC-049, 2026-11-21, per-platform programs in one archive

Owner's words: PRV-007 -> "Per-platform in one archive (Recommended)"

Lands in: closes DEC-044's research: an implementation item may state `platforms`; its archive holds `platforms/<os>/<arch>/<program>`; such an item runs only on a matching node, through a runtime provider with `run.v2`, which checks the archive's digest on the node before unpacking; the engine keeps each archive's bytes as distributed; narrows DEC-035 for the two Agent packages, whose Linux builds need a cross toolchain installed only on the owner's approval

Status: accepted, not done (spec, engine, kit and ssh-runtime, package and vendor-client orders)

## DEC-050, 2026-11-22, both reads on every order

Owner's words: A77's reads after the window -> "Both, every order (Recommended)"

Lands in: closes A77's ten-order window (15 orders read: the Codex verifier found 28 musts, the Claude reader none, and its shoulds and nits made refusals, help and diagnostics more explicit for agent callers); every order keeps both reads, blind and at the same time, one fix turn covering both; the reader moves to the second Claude account after the second-account sitting; at a usage limit that account pauses, and nothing merges unread

Status: accepted, in force

## DEC-051, 2026-11-22, the Linux programs built in Docker

Owner's words: the cross toolchain for DEC-049's Linux programs -> "Build in Docker"

Lands in: DEC-049's packages order: the static musl programs for `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` are built in a Linux container on the building machine, with nothing new installed on its host; the order names the image by digest, as an exact pin, and pulling it is the order's one download, approved by this entry

Status: accepted, not done (DEC-049's packages order)

## DEC-052, 2026-11-22, a release reached through a project contract must be installed

Owner's words: order 273's reading of PKG-007 -> "Keep the refusal (Recommended)"

Lands in: order 273: `inspect.v1`'s `packages` lists each package an object reaches through a contract of its submitted project, and PKG-007's "uses a package" includes such a package, so an instance of a template that reaches a release not installed is refused, naming the release, the template and the contract

Status: accepted, done by order 273

## DEC-053, 2026-11-22, an Opus drafter on the second Claude account

Owner's words: the drafter Agent -> "Yes, at the sitting (Recommended)"

Lands in: the second-account sitting creates an Opus drafter Agent on the second Claude account, through Conduit, which writes first drafts of order texts from the coordinator's brief; the coordinator approves each draft, the drafter's turns are recorded, and its first five drafts are compared with the coordinator's own; it shares that account's week with the readers (DEC-046)

Status: accepted, not done (the second-account sitting)

## DEC-054, 2026-11-22, pair 4 by measurement

Owner's words: pair 4 and the readers -> "Hold pair 4 by measurement (Recommended)"

Lands in: amends DEC-046's "two each to start" for the Mac's measured limit (S2 timeouts and a full disk at two pairs and the gate on 2026-11-22): stage 3 starts pairs 1 to 3; `worker-4` and `verifier-4` are set up and sent no turn until a dated amendment shows five consecutive orders on pairs 1 to 3 with no usage-limit failure on `second-chatgpt`, no turn lost to the machine and each gate run passing or failing only on the order's change; one Claude reader reads for every pair, and a second comes the same way

Status: accepted, not done (order 277; stage 3)

## DEC-055, 2026-11-22, stage 3 sets condition 4 aside

Owner's words: stage 3's pass condition 4 -> "Set aside for stage 3 (Recommended)"

Lands in: experiment 028's stage 3: condition 4 (no guard answers broken) is recorded as not shown and judges no order, since under ruling 70 the Agents run unconfined and no guard runs; orders pass or fail on the other conditions; the guards return with the sandbox (ruling 84)

Status: accepted, not done (order 277; stage 3)

## DEC-056, 2026-11-22, the 0.2.1 install ended stage 2

Owner's words: stage 2's results, item 1 -> "It ended stage 2 (Recommended)"

Lands in: experiment 028's stage 2 results: a registration's end condition holds as written when no amendment merged before the event says otherwise; the 0.2.1 install at 12:14 on 2026-11-20 was an upgrade of the store's Agent packages, so stage 2's registered results are turns 1 to 10, and turns 11 to 104 are reported as observations outside the stage

Status: accepted, not done (a results amendment of experiment 028)

## DEC-057, 2026-11-22, order 235's change was no breach

Owner's words: stage 2's results, item 2 -> "No breach (Recommended)"

Lands in: experiment 028's stage 2 results: the bar on orders that change the experiment's directory covers orders sent through Conduit's Fleet; order 235's link pinning (#488), made by the coordinator outside every turn under the amendment of 2026-11-21, neither breached it nor ended the stage

Status: accepted, not done (a results amendment of experiment 028)

## DEC-058, 2026-11-22, records exports committed by order

Owner's words: stage 2's results, item 3 -> "Commit it, split by order (Recommended)"

Lands in: experiment 028: each order's results directory holds its scrubbed records export, every account name replaced by its label, checked by the private-paths guard before merge; stage 2's export of 14:44 on 2026-11-21 first, then each stage's as its results are written

Status: accepted, not done (a results amendment of experiment 028)

## DEC-059, 2026-11-22, stage 2 ended at order 239's merge

Owner's words: stage 2's results, item 4 -> "Ended at 239's merge (Recommended)"

Lands in: DEC-037's "once orders 236 to 239 are in": an order the coordinator held, which sent no turn, keeps no stage open; stage 2 ended at order 239's merge, and order 237 (held under DEC-005, run later on the owner's answer) and orders 240 to 242 fall outside it

Status: accepted, not done (a results amendment of experiment 028)

## DEC-060, 2026-11-22, the Windows PC offloads and tries, and never decides a merge

Owner's words: the second machine -> "Offload + try runs (Recommended)"

Lands in: every order's gate runs in full on the Mac before it merges (ruling 80 point 10 unchanged); the Windows PC, while the owner has it resumed, runs the readers' tests and a try run of each order's tree before the order enters the queue, and the Codex workers' runs once Conduit places their Agents there (DEC-049, DEC-051); its results never decide a merge

Status: accepted, not done (the PC's setup; the gate and queue orders)

## DEC-061, 2026-11-22, batches once failures fall

Owner's words: batching -> "Batch once failures fall (Recommended)"

Lands in: the queue may gate up to three waiting orders as one tree when fewer than 10% of the last 20 gate runs failed and each order's try run passed; on a failure it splits the batch and gates the halves; an order marked to be gated alone is; commits inside a batch are not tested alone on main

Status: accepted, not done (after the queue's skip-and-continue and the try runs)

## DEC-062, 2026-11-22, no machine bought or rented for the gate until measured

Owner's words: another machine -> "Measure first (Recommended)"

Lands in: no machine is bought or rented for the gate until the gate's speed-ups and the PC's offload have run for a day and measured merges per day fall below the orders built per day; each gate run records the machine's load average

Status: accepted, in force

## DEC-063, 2026-11-22, an owner's app acts for the owner, with its own credential

Owner's words: which Interface the app is -> "1, but we should aim for 2 in v0, let's discuss"; whose authority its credential carries -> "Acts for the owner (Recommended)"

Lands in: until the engine issues Interface credentials, an app such as the Mac app is a client of root's Interface, on the engine's machine, presenting root's token; for v0 the engine issues each such app its own credential through IFC-042's authorization server (MCP's OAuth route, RFC 9728), approved once by the owner on the engine's own panel, carrying root's authority narrowed to the scopes the owner grants, and records name the client; sign-in codes stay on the engine's panel, never in an app (ruling 78 point 9)

Status: accepted, not done (a spec order and engine orders)

## DEC-064, 2026-11-22, an Agent may ask for access

Owner's words: may an Agent ask for more access -> "Access-request actions"

Lands in: the established route (Teleport Access Requests, Kubernetes CertificateSigningRequest): an Agent may ask for an operation it cannot call; the engine raises one pending action of a new kind, which permits nothing; root answers by applying a plan that opens exactly that access, or by declining; a request for get-secret.v1 or an operation reserved to root is refused and raises nothing; the questions of grant expiry and of how records name the action come back to the owner

Status: accepted, not done (a design study, then spec, engine, CLI and package orders)

## DEC-065, 2026-11-22, apply checks the approved plan's digest

Owner's words: must apply send exactly the plan approved -> "discuss 1 vs 2", then "Digest check (Recommended)"

Lands in: `conduit apply --expect sha256:HEX` recomputes the plan from live state and sends nothing when its digest differs from the one given; no plan file is required (ruling 76 point 5 stands)

Status: accepted, not done (a CLI order after orders 245, 247 and 248)

## DEC-066, 2026-11-22, a Resource's health is reported by its implementation

Owner's words: how a Resource's own health is known -> "Reported by its implementation (Recommended)"

Lands in: the Kubernetes and OpenShift route: an implementation reports conditions its Type names (for example CredentialAccepted) in a result's `_meta`; the engine shows each with its source and time until the run ends or a later report replaces it, drops a condition the Type does not name with a warning, and acts on none; a condition's message holds no secret value

Status: accepted, not done (an interface spec order, an engine order, a github package release)

## DEC-067, 2026-11-22, an Agent speaks to the owner only in replies, made readable

Owner's words: how an Agent may speak to the owner -> "Replies only, made readable (Recommended)"

Lands in: an Agent speaks only in the reply to a turn someone asked for; both Agent packages offer `history.v1` (each turn's message, final text, invocation, caller and times; no effect and no model work) to whoever holds a Conduit listing it; a role template's report may carry a field for the owner; states such as an Agent held by a lapsed login come from the engine, never from an Agent's text

Status: accepted, not done (one order per Agent package and a role-template update)

## DEC-068, 2026-11-22, the second-account sitting runs by hand tonight

Owner's words: tonight's sitting -> "Sit tonight by hand (Recommended)"

Lands in: the second accounts' login Resources are registered from copies outside the repository whose `account:` lines the owner types in their own editor; the coordinator writes every other line first and never opens the copies after, checking only their mode and that no placeholder is left; the owner tells the coordinator each account's plan and weekly reset day by label before the sign-ins; this is the procedure stage 3's registration names, and `value.v1` (DEC-076) replaces it once it lands

Status: accepted, not done (the sitting)

## DEC-069, 2026-11-22, pairs 1 and 2 and one reader take turns first

Owner's words: tonight's load -> "Pairs 1–2 + reader first (Recommended)"

Lands in: amends DEC-053 and DEC-054: the sitting signs in every account and creates every Agent through Conduit; pairs 1 and 2 and one reader take turns; pair 3 and the drafter receive no turn until orders 280 to 282 merge and one day's engine gates average under 20 minutes; pair 4 stays held by DEC-054's measure, read as no window reached

Status: accepted, not done (the sitting)

## DEC-070, 2026-11-22, stage 3 is judged on access

Owner's words: what stage 3 must prove -> "Judge on access (Recommended)"

Lands in: experiment 028's stage 3: an order passes on conditions 1, 3, 5, 8, 11 and 12; conditions 6 and 9 are measured and reported; condition 10 judges nothing until each package names every reason a turn lacks usage; a dated amendment, merged before stage 3's first turn, says so and says which store upgrade, if any, does not end the stage

Status: accepted, not done (a text order amending experiment 028)

## DEC-071, 2026-11-22, the Claude reader reads text-facing orders

Owner's words: the reader's scope -> "Reader on text-facing orders (Recommended)"

Lands in: amends DEC-050: the Codex verifier reads every order blind; the Claude reader also reads, blind and at the same time, an order that changes `spec/`, a diagnostic or refusal, agent- or owner-facing package text, or a README; one fix turn covers both; an engine-internal order merges on the verifier's clean read and the gate

Status: accepted, in force

## DEC-072, 2026-11-22, the S1 chain merges next

Owner's words: the merge order -> "S1 chain next (Recommended)"

Lands in: DEC-045's "capacity first" is read as the login and package orders it named, now merged; from today the S1 chain (orders 245 to 250) merges ahead of every engine order not on the second-account sitting's path; the gate speed-ups merge as they are ready

Status: accepted, in force

## DEC-073, 2026-11-22, v0's seven items, full v0 by the end of the year

Owner's words: what gates v0 -> "We need the same items, but if more resources, usage, and accounts can speed up our work then we should do so, target full v0 by the end of the year"

Lands in: v0 publishes on the seven items as ruled (ruling 47 as DEC-005 confirmed it); the target is full v0 by 2026-12-31; more machines, accounts and usage are added wherever measurement shows they speed the work (DEC-046, DEC-062); model checking's order 237 runs in parallel as information until S2-4 live

Status: accepted, in force

## DEC-074, 2026-11-22, the owner's app is root's client, then its own Interface by steps

Owner's words: what the app is in v0 -> "Client now, own Interface by steps (Recommended)"

Lands in: amends DEC-063: in v0 the owner's app is a client of root's Interface on the engine's machine, presenting a token whose subject is root with a claim naming the client and no scope; Conduits alone decide (IFC-022); records name the client; it becomes an Interface of its own by opening root-only acts (sign-in of login Resources, the login notice, registering Agents, get-secret Conduits, install) to a scoped Interface one at a time, each with its own guard; the authorization server (DL13, OAuth 2.1, RFC 9728) lands once, after the engine's panel

Status: accepted, not done (the panel first, then a spec and an engine order, then one order per opened act)

## DEC-075, 2026-11-22, the engine's panel first, then the app

Owner's words: the v0 surface -> "Panel first, then the app (Recommended)"

Lands in: the engine's own panel comes first, in Rust, rendered to a terminal `serve` owns (order 051c), showing pending actions with their answers, login holds, nodes and a records tail; the owner's app comes second as DEC-074's client; an Interface client adds no rule of its own (one spec paragraph); a document change from an app reaches the engine only through `conduit plan --output json` and `conduit apply --expect sha256:HEX`

Status: accepted, not done (051c, the spec paragraph, the CLI order)

## DEC-076, 2026-11-22, installation values are value.v1 Resources

Owner's words: the durable form of private values -> "value.v1 Resources (Recommended)"

Lands in: an installation's private values live in its engine as `value.v1` Resources registered by root; documents name them under `values:` beside `arguments:`; the engine writes each into configuration at registration (OBJ-009's moment), keeps a source map on the instance and shows where each came from; `conduit value set` takes a value shown as typed and saves it only on y; every stop names `conduit value set`; `conduit records` redacts by value name on export; secrets stay on secret Resources (the ConfigMap-beside-Secret route)

Status: accepted, not done (spec, engine, CLI and dogfood orders)

## DEC-077, 2026-11-22, usage and limits advertised, in v0

Owner's words: usage limits -> "2, and an agent must keep track of and advertise its usage, its a core feature of conduit, let's discuss"; then "Design B in v0 (Recommended)"

Lands in: in v0, every turn's result reports the usage limits its client saw; the engine keeps the newest report for each window on the Agent and on its login Resource, marks a window unknown after its reset, adds nothing up and acts on none of it; a Codex login answers `limits.v1`, a read that starts no model work and never refreshes the login; a Claude login answers through its status; the CLI tables usage and limits; nothing is relayed to the Agent's model (ruling 68); a Type with a model-work operation or `signIn` states what it reports

Status: accepted, not done (seven orders: a probe, spec, engine, two package releases, CLI, the dispatcher)

## DEC-078, 2026-11-22, only login Resources sign in, trimmed after the sitting

Owner's words: the sign-in path -> "Trim after the sitting (Recommended)"

Lands in: only a Resource whose Type states `signIn` is signed in; an Agent's login place refers to one; no Agent Type states `signIn`; both Agent packages mark their per-Agent Types as compatibility now, and the spec's per-Agent branch and the old wrappers are removed in the first package release after the sitting

Status: accepted, not done (two README orders now; a spec order and two package orders after the sitting)

## DEC-079, 2026-11-22, access requests, health and history now

Owner's words: when DEC-064, DEC-066 and DEC-067 land -> "All three now"

Lands in: access requests (DEC-064), Resource health reported by implementations (DEC-066) and Agent replies through `history.v1` (DEC-067) are built now, in v0, each through its design study or orders; DEC-064's study carries the full refusal list (every operation no Conduit with an Agent consumer may list) and says where `request-access.v1` is offered; DEC-067's package orders keep tool calls, tool results, file contents and usage limits out of `history.v1`

Status: accepted, not done (about eleven orders)

## DEC-080, 2026-11-22, the per-node chain is finished now

Owner's words: where the per-node chain stops -> "Finish the chain now"

Lands in: DEC-049's and DEC-051's orders are finished now (the run side with `run.v2`, the kit and ssh-runtime, the packages' Linux programs built in Docker), so the Windows PC can carry Agents through Conduit; the Pi stays the hub and builds nothing; a package install in the dogfood store waits for a stage boundary or a dated amendment (order 277 item 8)

Status: accepted, not done (about three orders)

## DEC-081, 2026-11-22, a default Pool by `pool:`, as a Fleet by `fleet:`

Owner's words: the default Pool -> "pool: like fleet: (Recommended)"

Lands in: ruling 53 point 6: a Resource that states no owner belongs to the file's `pool:` Pool, as an Agent belongs to its `fleet:` Fleet; `conduit init` writes neither; lands before the S1 rerun's pin or not before it

Status: accepted, not done (a spec order with vectors and a loader order)

## DEC-082, 2026-11-22, an access grant expires, and root sets its length

Owner's words: does a granted access expire, and who sets how long -> "Expires; root sets it (Recommended)"

Lands in: DEC-064's study, question 1: the grant's Conduit carries its own end time, checked at each request; `conduit grant` writes the shorter of the Agent's asked duration and 24 hours, and root may choose from 10 minutes to 14 days; a core requirement `expires.v1` and a promise ACC-059; nothing runs on a timer

Status: accepted, not done (order 299 and the engine order after it)

## DEC-083, 2026-11-22, records name an access request through ordinary records and the granted Conduit

Owner's words: how records name a request and its answer -> "Records + the Conduit (Recommended)"

Lands in: DEC-064's study, question 2: the ask's record and each answering request's record name the action's ID, as PND-012 does; the granted Conduit's provenance gains `answers`, the actions its open answered; no new record type

Status: accepted, not done (order 299)

## DEC-084, 2026-11-22, an access request carries no reason text

Owner's words: may an Agent's ask carry its own reason in words -> "No; say why in the reply (Recommended)"

Lands in: DEC-064's study, question 3: no `reason` input; the action names the turn that asked, and the Agent says why in its reply, read through `history.v1` (DEC-067)

Status: accepted, not done (order 299)

## DEC-085, 2026-11-22, asks for install.v1, management or access are refused

Owner's words: beyond the operations never asked for, what else is refused at once -> "Also install, management, access (Recommended)"

Lands in: DEC-064's study, question 4, ACC-058: an ask for `install.v1`, or whose target is `management` or `access`, is refused and raises nothing

Status: accepted, not done (order 298)

## DEC-086, 2026-11-22, a client outside the repository is not bound by the Rust rule

Owner's words: where the owner's app lives and its language -> "Own repo, his language"

Lands in: the owner's app lives in its own repository, in its builder's language; DEC-025's Rust rule covers the engine, the packages and the tooling in this repository, not clients outside it

Status: accepted (no change in this repository)

## DEC-087, 2026-11-22, the owner's app is its builder's to decide within the client contract

Owner's words: what a second contributor decides -> "The app is his (Recommended)"

Lands in: the app's screens, interaction, stack and code are its builder's to decide, within the client contract; a change to a spec promise, the engine, the client contract, the decision log or how the work is done is proposed to the owner, who rules

Status: accepted

## DEC-088, 2026-11-22, a human contributor's commits carry their own name

Owner's words: whose name goes on a second contributor's commits -> "His own, if he agrees (Recommended)"

Lands in: HOW-WE-WORK.md: the neutral work-order identity is only for commits made by Agents and the coordinator; a human contributor commits under their own name, if they agree

Status: accepted, not done (order 300)

## DEC-089, 2026-11-22, the gate admits commits by listed human contributors

Owner's words: how a human contributor's commits pass the gate's identity row -> "Gate admits listed humans (Recommended)"

Lands in: tools/gate.py's identity row admits, beside the neutral identity, the contributors a root file lists, which the owner approves; squash merges keep the contributor as author

Status: accepted, not done (order 300)

## DEC-090, 2026-11-22, a client uses the conduit CLI until DEC-074's client token exists

Owner's words: how a client authenticates before DEC-074's token exists -> "Through the CLI only (Recommended)"

Lands in: a client outside the engine runs `conduit` commands and reads their JSON; it never opens root's token file; direct MCP comes with DEC-074's client token

Status: accepted (docs/clients.md, order 301)

## DEC-091, 2026-11-22, the owner's app is built now, and its engine needs come later

Owner's words: is the app built now, with its engine needs after the panel -> "Yes, app now; its needs after (Recommended)"

Lands in: the app is built in parallel against what exists; the engine work it needs (its client token, `conduit plan --output json`, the access-request answers) follows

Status: accepted

## DEC-092, 2026-11-22, the public client page goes into the repository

Owner's words: where the page for client builders goes -> "Into the repo now (Recommended)"

Lands in: docs/clients.md, linked from CONTRIBUTING.md, marked a guide; spec/ and engine/CONFORMANCE.md bind

Status: accepted, not done (order 301)

## DEC-093, 2026-11-23, the usage guard is a feature Conduit offers, declared on the login Resource with explicit flags

Owner's words: how a team seat's use is guarded -> "this guard should be something conduit offers"; where -> "on the login resource"; "we should have an explicit flag for this, and flags for allowing for usage credits etc, this should be a core feature of conduit being able to tune this"

Lands in: a guard on each login Resource, tuned by explicit flags including whether usage credits are allowed; general for refusal across account kinds; amends DEC-077's "the engine acts on none"

Status: accepted, not done (a design, then spec, engine and package orders)

## DEC-094, 2026-11-23, usage is self-reported by packages, and the engine trusts the report

Owner's words: who computes usage and its levers -> "the codex package itself should define how to calculate usage statistics and levers. Conduit specifies what must be reported and what contracts must be upheld. The engine then trusts this reporting"

Lands in: each login Type's package defines its levers and how it measures; the spec defines the report every login implementation gives and the contracts it upholds; the engine holds work on the package's report and computes nothing itself

Status: accepted, not done (the design's second version, then its orders)

## DEC-095, 2026-11-23, a guarded hold reports unavailable at the login hold

Owner's words: how the engine stops a turn on a guarded login -> "Accept (Recommended)"

Lands in: the engine holds each call that starts model work on an Agent of a guarded login before dispatch, at PND-005's login hold, reporting unavailable and never queued

Status: accepted, not done (DEC-094's orders)

## DEC-096, 2026-11-23, a usage report may carry paid-usage state

Owner's words: may a limits report carry paid-usage state, with no amounts -> "Accept (Recommended)"

Lands in: IFC-052 amended: a report may carry whether paid usage is possible and in use, each true, false or null, mapped only from a field the client gave; no amount, balance, limit, plan or currency

Status: accepted, not done (DEC-094's orders)

## DEC-097, 2026-11-23, a guard declares the login's billing

Owner's words: should a guard declare included or paid-from-first-token billing -> "Accept (Recommended)"

Lands in: every guard states `billing`: `included` or `paid-from-first-token`; a paid-from-first-token login whose guard refuses paid usage is held for every call

Status: accepted, not done (DEC-094's orders)

## DEC-098, 2026-11-23, an unknown usage report is decided per account kind

Owner's words: what the engine does with no current report -> "Per account kind (Recommended)"

Lands in: Codex logins refuse after a read, stale after 15 minutes; Claude logins allow one warned trial, stale after 60 minutes; under DEC-094 these are each package's levers

Status: accepted, not done (DEC-094's orders)

## DEC-099, 2026-11-23, a guarded hold tells the caller everything but amounts

Owner's words: what the caller is told when a call is held -> "Accept (Recommended)"

Lands in: the hold names the login, the lever and its value, the window, its used percentage, the threshold, the reset, how it lifts and which report, in text and a structured field; amends DEC-077's "nothing relayed to the model" for this diagnostic only

Status: accepted, not done (DEC-094's orders)

## DEC-100, 2026-11-23, any holder of a Conduit listing it tunes a guard

Owner's words: who tunes a login's guard -> "Any holder of a Conduit"

Lands in: the guard's levers are changed by a control that any holder of a Conduit listing it may send

Status: accepted, not done (DEC-094's orders)

## DEC-101, 2026-11-23, the UI is a package, not part of the core

Owner's words: how a simple web page for using Conduit is built -> "UI is not a core part of conduit its a package, reflecting the modular nature of the engine and language"

Lands in: a web UI is an Interface package, held to the same promises as any client; amends DEC-075's engine-owned panel

Status: accepted, not done (a design, then its orders)

## DEC-102, 2026-11-23, team-seat Agents take no turn until the usage guard is built

Owner's words: what the team seats do until the guard exists -> "Pairs 1-2 now; team sign-ins too (Recommended)"

Lands in: the team accounts are signed in and their Agents set up, but no Agent on a team seat takes a turn until DEC-093's guard is built and merged; amends DEC-069 for those Agents

Status: accepted, not done (DEC-093's orders)

## DEC-103, 2026-11-23, Interfaces show what only a person may see; the engine renders nothing

Owner's words: what the engine itself still renders -> "What can no client show? Shouldn't the engine just expose a bunch of endpoints or hooks that a display or tool can connect to?"; then "I like the idea of a trusted display, let's discuss, even a cli could be a trusted display or interface. Isn't this just the concept of interfaces from conduit?"; then "Yes, Interfaces show them (Recommended)"

Lands in: PND-004 and its neighbours: sign-in codes, a joining machine's fingerprint and the approval of an app's credential are delivered only to an Interface's client (the CLI, the web UI package, the owner's app), never to an Agent or into a model's turn; the engine exposes everything as endpoints and renders nothing itself; amends DEC-075, whose engine panel (order 051c) is dropped

Status: accepted, not done (a spec order, then an engine order)

## DEC-104, 2026-11-23, the usage report's shape stays in the core, after a study of Agent accounts

Owner's words: where the usage report's shape lives -> "Core, since agents are at the core of conduit as a whole, we should do a study to figure out deeply what contracts we need and if these accounts are better enforced within fleets rather than the pool resource. This has turned into a big design issue so maybe it's worth getting fable's perspective"

Lands in: the report every login implementation gives stays one shape in the spec; a study of what contracts Agents and their model accounts need, and whether accounts and their limits are better enforced by Fleets than by Pool-owned login Resources, comes before DEC-093's orders

Status: accepted, not done (the study is under way)

## DEC-105, 2026-11-23, vendor names and one tool's flags leave the spec now

Owner's words: whether product names and the plan and apply flags leave the specification's own text -> "Now, one docs order (Recommended)"

Lands in: vendor names move to the Agent packages' READMEs and Type descriptions; the plan and apply text moves to the CLI's own guide; the spec README leads with the six families; amends DEC-008's placement of that text

Status: accepted, not done (one or two documentation orders)

## DEC-106, 2026-11-23, plan, apply and author leave the engine, and later become an Interface package

Owner's words: where the command-line client's plan, apply and author go -> "Later an Interface package"

Lands in: plan, apply and author move to a client crate of their own, and once client tokens exist (DEC-108) the command line becomes an Interface package beside the web UI; amends DEC-008's placement

Status: accepted, not done (a move order, then the package after the client token)

## DEC-107, 2026-11-23, an agent kit library now, and the engine's runner pinned

Owner's words: the two Agent packages' shared code and the two ways of running a process -> "Library now, runner pinned (Recommended)"

Lands in: a shared agent kit library both Agent packages depend on, which the engine never depends on; the engine's own runner stays pinned to minimal local runs, and new runtime features go to the runtime kit and local-process; amends DEC-021's timing

Status: accepted, not done (package orders)

## DEC-108, 2026-11-23, the client token first, then the web UI

Owner's words: the web UI's build order -> "Client token, then the page (Recommended)"

Lands in: DEC-074's client token is specified and built (minting, acceptance, records naming the client) before the web UI package's orders; how long it lives waits for the study of how Interfaces are granted access

Status: accepted, not done (a spec order and an engine order first)

## DEC-109, 2026-11-23, the web UI is a Resource the engine runs, given its token as a secret

Owner's words: how the token reaches the page -> "As a secret Resource (Recommended)"; the package's shape -> "A Resource the engine runs (Recommended)"

Lands in: the web UI package's Type is a Resource whose run the engine starts and stops, reporting Running; its client token reaches the run through a secret reference, a file-secret Resource and the get-secret route, with no new mechanism

Status: accepted, not done (the package's orders)

## DEC-110, 2026-11-23, the web UI package is a lightweight proof

Owner's words: where the web UI package lives and who builds it -> "This repo with codex and opus workers, but focus on something super lightweight, Santiago will be building the polished version, our goal is to prove it's possible and promises are met"

Lands in: packages/ in this repository, built through Conduit by Codex or Opus workers and read like every order; as small as proves that an Interface package can meet the client promises; the polished app is the second contributor's

Status: accepted, not done (the package's orders)

## DEC-111, 2026-11-23, only the runtime engine must be Rust

Owner's words: the web page's script -> "The web ui can be built from anything, conduit packages do not need to be in rust, only the runtime engine must be"

Lands in: HOW-WE-WORK's language rule: the engine is Rust; a package may be written in any language; amends DEC-025

Status: accepted, not done (HOW-WE-WORK's rule)

## DEC-112, 2026-11-23, a client's read of the records is itself recorded

Owner's words: how the records pane's reads are recorded -> "Record the export read (Recommended)"

Lands in: reading the records export with a client token writes one record of the read naming the client

Status: accepted, not done (with the client token's engine order)

## DEC-113, 2026-11-23, the web UI first runs on the dogfood store, and the store is studied

Owner's words: which store the web UI first runs against -> "Dogfood store, but also I need a deep analysis of the store, what is this store and how is it structured? Why is it built this way? What are examples and precedents? Use a fable agent if you need to"

Lands in: the web UI package is installed in experiment 028's store after a dated amendment that says what a turn sent from the page counts for; a study of the store's structure, its reasons and precedents

Status: accepted, not done (the amendment, and the study under way)

## DEC-114, 2026-11-23, the instance list gives owner, placement, revision and Running

Owner's words: what list.v1 gives -> "Widen list.v1 (Recommended)"

Lands in: list.v1 gives, for each instance, its owner, its placement, its revision and the status of its Running condition

Status: accepted, not done (a spec order and an engine order)

## DEC-115, 2026-11-23, local runs come back when serve restarts

Owner's words: what happens to runs after serve restarts -> "Restart all local runs (Recommended)"

Lands in: PRV-026 extends to every Resource run on the engine's own runner, read secrets included; Agents stay as they are

Status: accepted, not done (a spec order and an engine order)

## DEC-116, 2026-11-23, a model account is its login Resources; limits are enforced there, not in Fleets

Owner's words: where a model account is owned and its limit enforced, after the study DEC-104 asked for -> "Login Resource, as now (Recommended)"

Lands in: a model account is represented by its login Resources, owned by a Pool, one per account per machine; its limits are declared and enforced there; a Fleet owns Agents and nothing else (FLEET-001, FLEET-004 stand); the usage and limit schemas are core because the engine checks them (DEC-045 applied, DEC-104's "Core")

Status: accepted, not done (the definitions row, with the guard's spec orders)

## DEC-117, 2026-11-23, every model turn's record names its Fleet and login

Owner's words: attribution -> "Yes, in v0 (Recommended)"

Lands in: the end record of every call that starts model work names the Agent's Fleet and its login Resource beside the Agent; the CLI lists those records for a login or a Fleet with each one's reported usage and adds nothing up; any total is a UI package's

Status: accepted, not done (a spec order, an engine order and the CLI's listing)

## DEC-118, 2026-11-23, a cap on turns per Conduit waits for v1

Owner's words: a limit on the model turns started through one Conduit -> "v1, on the roadmap (Recommended)"

Lands in: a ROADMAP row for v1; whether the engine applies it from its own dispatch records or an Agent package applies it as a filter is decided when it is designed

Status: accepted, not done (the ROADMAP row)

## DEC-119, 2026-11-23, no Agent tunes a login's usage levers

Owner's words: who may tune a login's levers -> "Not Agents, ever (Recommended)"

Lands in: no Conduit whose consumer is an Agent lists a login Type's admission control, and such an open reports refused; an Interface's client tunes; amends DEC-100

Status: accepted, not done (the guard's spec orders)

## DEC-120, 2026-11-23, the guard checks before each turn and a started turn runs to its end

Owner's words: a turn in flight when a lever says stop -> first "Both packages, after probe (Recommended)"; then, asked how the levers would reach a turn -> "I'm confused as to what this means, let's discuss"; "Would this send this key to Claude/codex, I'm confused? Maybe we should just allow the turn then? I don't want to modify how the models operate just police their usage"; then "Yes, check before turns only (Recommended)"

Lands in: the engine checks the login's report before every call that starts model work and holds the call where the levers refuse it; the engine never stops a dispatched turn and no package ends one for the guard; no lever reaches a turn, and turns stay as they are; an allowed-models setting is met by the Agent's own template; the margin past a threshold is covered by a threshold below 100 and the seat's own console

Status: accepted, not done (the guard's orders)

## DEC-121, 2026-11-23, one login Resource per machine, each with its own levers

Owner's words: one account used on two machines -> "Per machine is fine, if it's a shared account there should be no drift since it should have the same value queried under the hood right?"

Lands in: one login Resource per account per machine, each with its own levers; usage is the account's own and reads the same from both; only declared levers could differ, which declaring both logins from one template prevents

Status: accepted, not done (nothing to build until a second machine runs Agents)

## DEC-122, 2026-11-23, a contributor's Agents use the contributor's own accounts

Owner's words: another contributor's Agents -> "Their own accounts only (Recommended)"

Lands in: a contributor's Agents never name a login Resource of the owner's accounts; they name logins of the contributor's own accounts, which root registers in a Pool and signs in, with the contributor present for their own code through an Interface's client (DEC-103)

Status: accepted, not done (the contributor's onboarding)

## DEC-123, 2026-11-23, a call waits in the request for a fresh usage report

Owner's words: how the engine gets a new report when the last has ended -> "Wait in the request (Recommended)"

Lands in: with no current report, a call that starts model work waits for the login's one ask, bounded by an implementation-defined limit, and is then dispatched or held on its answer, as an admission webhook does; amends DL3 and DL4 for this wait

Status: accepted, not done (the guard's spec and engine orders)

## DEC-124, 2026-11-23, the levers come on new Type versions

Owner's words: whether the guard's required parts come on new Types -> "New Type versions (Recommended)"

Lands in: codex-login.v2, account-home.v2 and the Agent Types' next versions carry the levers, the admission hook and its control; each team seat is registered and signed in once more and each Agent registered again under a Type that lists its login's Type, at one sitting; logins on the old Types have no check, and their status says so

Status: accepted, not done (the package orders and a sitting)

## DEC-125, 2026-11-23, one token per client now; expiry later, documented

Owner's words: the shape of a client's credential -> "Let's do one token for now, we can add expiry later but I don't want to slow down development"; how long a token lasts -> "Let's focus on expiry later, and document it, for now I don't want to pause work"

Lands in: each admitted client of an Interface holds one token that lasts as long as its admission and is revoked by removing the client, as runs' tokens are today; no token endpoint yet; the missing expiry is documented as a known gap; fixes DEC-108's open question on the token's life

Status: accepted, not done (the client token's spec and engine orders)

## DEC-126, 2026-11-23, when expiry comes, a client ends at a fixed end and when idle

Owner's words: when an admitted client ends without a revocation -> "Fixed end + idle end (Recommended)"

Lands in: once expiry is added (DEC-125), root sets a fixed end at admission, 1 to 365 days and 180 by default, and a client also ends after 90 days with no use; both checked at the next request, nothing on a timer

Status: accepted, not done (after DEC-125's orders)

## DEC-127, 2026-11-23, in v0 a client's token is a file the owner hands over

Owner's words: how a client is admitted -> "Key file only in v0"

Lands in: the owner makes a client's token and hands the file to the client (the web UI's as a secret Resource, DEC-109; a script's; the second contributor's app's); no invite route in v0

Status: accepted, not done (the client token's orders)

## DEC-128, 2026-11-23, no display mark; a model-driven program gets an Interface of its own

Owner's words: how sign-in codes are kept from models that run the CLI -> "No mark; models get own Interface"

Lands in: every client of root is the owner's and may receive codes, fingerprints and approvals (DEC-103); a program a model drives (the coordinator, the workers, an app's model) acts through an Interface of its own, under DEC-074's steps; no client scope is added, and DEC-074's "no scope" stands. The coordinator's reading, open to the owner's strike (DEC-029): until those Interfaces' orders land, the model-driven programs keep acting as root as today, and no model runs a sign-in or an enrollment; the owner does those at the owner's own terminal

Status: accepted, not done (the Interface orders)

## DEC-129, 2026-11-23, root's helper token stays for now

Owner's words: root's 100-year helper token -> "Keep it for now (Recommended)"

Lands in: root's helper token stays as the store owner's documented bootstrap, its risk documented (a copy is root until the store is replaced); it is retired, refusing every copy, when expiry lands

Status: accepted, not done (the documentation, then with expiry)

## DEC-130, 2026-11-23, the web UI is a display, behind a browser cookie

Owner's words: the web UI's mark and a credential for its browser leg -> "A display, with the cookie"

Lands in: the web UI package, a client of root, receives codes and fingerprints once DEC-103's orders land, and keeps them out of every log and off the page once answered; its browser leg takes an HttpOnly, SameSite=Strict cookie set from a start secret only the owner's user can read

Status: accepted, not done (the web UI package's orders)

## DEC-131, 2026-11-23, clients use plain http on loopback for now

Owner's words: the address clients connect to -> "Plain http on loopback (Recommended)"

Lands in: clients reach the engine over plain http on loopback, with the risk while serve is down documented; TLS on loopback later, the store's CA in the client's file

Status: accepted, not done (the documentation; TLS later)

## DEC-132, 2026-11-23, DEC-111 frees packages only

Owner's words: whether "only the runtime engine must be Rust" also frees experiments and tooling -> "Packages only (Recommended)"

Lands in: the engine is Rust; a package may be written in any language; experiments and the coordinator's tooling stay Rust, and orchestration around the engine stays an engine gap written as orders; narrows DEC-111

Status: accepted, not done (order 306)

## DEC-133, 2026-11-23, model-driven programs keep root until their Interface lands

Owner's words: the transition DEC-128 leaves -> "Confirm (Recommended)"

Lands in: until model-driven programs have an Interface of their own, the coordinator and the workers keep acting as root through the CLI, and no model runs a sign-in or an enrollment; the owner does those at the owner's own terminal; confirms DEC-128's reading

Status: accepted, not done (ends with the model-driven Interface's orders)

## DEC-134, 2026-11-23, a turn's record is readable through its login's Pool

Owner's words: whether naming the login in a turn's record widens who may read it -> "Accept (Recommended)"

Lands in: a model turn's end record that names a login Resource (DEC-117) is visible to whoever may inspect that login's Pool, and to management, beside the Agent's side

Status: accepted, not done (order 305 and its engine order)

## DEC-135, 2026-11-23, the usage guard's probe runs at a sitting on 2026-11-24

Owner's words: when the guard's first probe runs -> "Tomorrow morning (Recommended)"

Lands in: the probe (A0 of the guard's design) is written on 2026-11-23 and run with the owner present on the morning of 2026-11-24; it reads each team seat's usage and does no model work unless the owner says so (DEC-102)

Status: accepted, not done (the probe's order, then the sitting)
