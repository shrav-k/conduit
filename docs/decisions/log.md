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
