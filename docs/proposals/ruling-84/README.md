# Ruling 84: isolation comes later from a sandbox method, Docker first, in place of a dedicated macOS user

Status: in force (ruled by the owner on 2026-11-18). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 9 (a direction, not a design, and not spec pending, for point 1) and topic 15 (points 2 and 3, and the amendment of D61), and A22's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes). This record says why.

[Ruling 75](../ruling-75/README.md), B3, has the engine that runs Conduit's own work run as a
dedicated macOS user before a coordinator Agent drives workers (A78's orders 4 and 5). The full
review's [D61](../full-review/decisions.md#dogfooding-and-the-evidence-kept-in-the-tree) prepares
for that user without moving to it, and runs D58's harness change and A22's open case, a keychain
login used with no login session, before any Claude Agent becomes a dogfood slot and before batch 2.
A22's sessions (b) and (c) run only once that user exists, and creating it needs the owner's sudo.
On 2026-11-18 the coordinator put to the owner, through the session's question tool, whether to
create the user now. The owner's typed answer was cut off, and the owner then interrupted and asked
to be asked again. The coordinator put a second set of two questions, with options and a
recommendation, and the owner answered both. The questions as put, the options the coordinator's
notes hold, the owner's answers and the interruption follow, verbatim where the notes hold them so.

## The first questions

The first question, as put: "Claude slots can't join the Fleet until a dedicated macOS user exists
for the engine (A22, decision D61), which needs your sudo. Setting it up now, in parallel, removes
it from the path and saves a second round of sign-ins later. Do it now?"

The owner's typed answer, cut off as it reached the coordinator: "Why do we need a dedicated new y"

A second question in the same set asked whether the second accounts come before the S1 pin.
The coordinator's notes hold it only in that summary, and hold neither its exact wording nor its
options. The owner's answer: "Keep the S1-pin staging".

## The interruption

The owner then interrupted, verbatim: "Ask me again, another user can't be the only way to run
multiple accounts, there has to be a better way, let's discuss, that's a horrible abstraction. We
need something similar to vms or virtual envs"

The coordinator's notes give, in summary and not verbatim, the facts it brought to the discussion
that followed: ruling 75 B3, as above; B2, under which confinement limits live on the runtime
provider, and the first confining provider wraps an existing whole-process sandbox and is built
later; D60, under which, with one operating-system user, an Agent can read another's login; D61, as
above; and Claude Code's documentation as order 050g's read quotes it ([its results, section
L3](../../../experiments/027-a5-sign-in/results-2026-11-17-logout-source.md#l3-one-login-for-each-home)):
with `CLAUDE_CONFIG_DIR` set, the credentials file and the Keychain entry are scoped to that
directory, so per-Agent homes already hold separate accounts (order
[050c](../../work-orders/050c-agents-v2-client-home.md)). Those statements are the coordinator's.

## The second questions

The first, as put: "How should Conduit wall Agents off from each other and from your own files?
(Several accounts already work through per-Agent homes, venv-style; this is about isolation.)"

Its options, as the coordinator's notes hold them, by label only:

- "Per-Agent sandbox (Recommended)"
- "Sandbox + dedicated user"
- "Dedicated user, as ruled"

The owner chose no option and typed: "Let’s start with docker, but this should be abstracted by
sandbox method. This is not an immediate concern, for now separate mediated accounts is enough"

The second, as put: "Until the sandbox exists, may primary-account Agents (Codex and Claude) run as
dogfood slots under your user, with per-Agent homes but no wall, as ruling 70 allows today? Results
would say they show no isolation (decision D60)."

The owner chose: "Yes, primaries only (Recommended)". The coordinator's notes hold no other option
of this question.

## The owner's answers

The answers are this ruling, as ruling 67 has it. The coordinator's notes record no time for any of
them. What each decides, below, is the coordinator's reading where it says so. That the typed answer
to the first of the second questions sets the dedicated user aside as the planned wall, and is
closest to "Per-Agent sandbox (Recommended)" while choosing no option, is the coordinator's reading:
the owner had called another user "a horrible abstraction" and asked for "something similar to vms
or virtual envs". That "separate mediated accounts" means separate per-Agent homes, each signed in
on its own through Conduit's sign-in flow, is the coordinator's reading too, from the question's own
words, "Several accounts already work through per-Agent homes, venv-style". The cut-off first answer
decides nothing by itself: it asked why, and the interruption and the second questions followed.

## The decisions

1. **Isolation comes from a sandbox method, Docker first, and not now** (the typed answer to the
   first of the second questions). Isolation between Agents, and between Agents and the owner's own
   files, is to come from a sandbox abstraction, a "sandbox method", with Docker first. This is a
   direction, recorded so that later designs keep to it. It is "not an immediate concern", in the
   owner's words. That it is not a design, and that nothing in `spec/` or any order is written from
   it before a grounded study (under "What follows") and a design the owner rules on, is the
   coordinator's reading of "not an immediate concern"; the owner's answers name no study.

   By the coordinator's reading, a sandbox method is chosen per runtime, and is what
   [ruling 75](../ruling-75/README.md), B2, already places on the runtime provider: a runtime
   provider that confines its consumer, chosen per Agent through `runtime:` (ACC-027, PRV-006),
   whose limits are not Agent inputs, and which wraps an existing whole-process sandbox. Docker is
   such a sandbox, and the abstraction is the provider contract, so that each further method is
   another provider behind it, as [ruling 40](../ruling-40/README.md), point 1, has an existing tool
   behind a provider for everything but connection and access. Ruling 40, point 2, leaves
   "identity, isolation, the storage of records and the engine's own state" each engine's choice
   for now. Isolation does not become a provider contract of its own here: confinement comes
   through the runtime contract, which is one of v0's provider contracts, as ruling 75, B2, and
   ruling 70, point 4, already ruled. Each sandbox method being a provider the user can swap goes
   the way the owner's answer to ruling 40's point 2 asked, "as far as we can in the 'swappable by
   user' direction". [Ruling 70](../ruling-70/README.md), point 4, gave the same route, with "a
   container with only the Agent's directory and home mounted" as its example, and
   A63's row in [the tracker](../../AUDIT.md) already lists a runtime on Docker among the first
   providers' candidates. That this fits B2, ruling 40 and ruling 70, point 4, and amends none of
   them, is the coordinator's reading.

   The sandbox replaces the dedicated macOS user of ruling 75, B3, as the planned wall. B3 is
   amended: the engine that runs Conduit's own work is not planned to move to a dedicated macOS
   user. B3's timing, before a coordinator Agent drives workers (A78's orders 4 and 5), is not
   carried over to the sandbox by this ruling. When the sandbox must be in place, and what it gates,
   is the design's, and waits for the owner with D59's question on what orders 4 and 5 test, which
   is asked with batch 2's plan once confinement is known. Orders 4 and 5 wait for appointment, in
   batch 2, in any case. Both of those sentences are the coordinator's reading: the owner's answer
   set no gate.

   What hung on the dedicated user follows it, by the coordinator's reading:

   - **D61** is amended. B3 is no longer prepared for. A22's open case no longer runs before any
     Claude Agent becomes a dogfood slot, nor before batch 2, and the message that creating the user
     at order 050's sitting would save a second round of sign-ins, and the warning that B3 may not
     work for Claude Agents as ruled, go with the user. D61's other precondition stands: D58's
     harness change runs before any Claude Agent becomes a dogfood slot and before batch 2.
   - **A22 is narrowed, not withdrawn.** Its session (c), the dedicated user over SSH, and its
     case of where a Claude Code login lands for that user, lose their purpose and are withdrawn.
     Its session (b) loses the dedicated user and stays, optional: a process started by launchd
     with no graphical login, a daemon outside the owner's graphical session, on a throwaway
     keychain, the user it runs as fixed when it is pre-registered. That is the case experiment
     005 left open ("A daemon runs in another session, as root or another user"), and the case
     A63's row names as "a real background daemon is untested (A22)". Its reading stays fixed
     before it runs: if (b) cannot read an unlocked keychain without its password, the keychain
     store's contract says "graphical login session only". Its session (a), a keychain item
     written and read unattended by a LaunchAgent in the owner's graphical session, stays,
     optional. Both serve the keychain store of A63 and the wording of its contract. A22 gates no
     Agent's becoming a dogfood slot, and nothing in batch 2. Where a client's login lands inside a
     sandbox method is the study's question, not A22's.
   - **[Ruling 82](../ruling-82/README.md), point 6**, which kept A22 "owed before any Claude Agent
     is a slot (the record's section 2 (c))", is amended to match, and so is its line that A81 and
     A22 "stay owed as before", which now holds for A81 only. Its o7, that A81 and A22 gate no
     member's freeze, stands.
   - **D58 stands.** The dogfood harness never runs anything an Agent could have written. It does
     not depend on a move to another user, so its sentence "This is in place before the engine
     moves to another user" is met earlier, under D61's remaining precondition.
   - **D60 stands.** Every result obtained under one operating-system user opens by saying what it
     cannot show. Its words "before and after your dedicated macOS user" now have one case, the
     owner's user, until a sandbox method exists. Whether a result obtained inside a sandbox shows
     Agents kept apart is the design's to state.

2. **Until the sandbox exists, separate per-Agent homes are enough for primary-account dogfood
   slots** (the typed answer's "for now separate mediated accounts is enough", and the second of the
   second questions). Primary-account Codex and Claude Agents may be dogfood slots under the owner's
   operating-system user, each with its own client home inside its own state directory (order 050c),
   held apart by the client's own directory scoping (`CLAUDE_CONFIG_DIR` for Claude Code, as order
   050g's read quotes it, and Codex's own home directory), and unconfined, as ruling 70 allows.
   Every result they give states, under D60, that it shows no isolation: under one user an Agent
   that runs code can read, use or replace another Agent's login and act as root's Interface. Per-
   Agent homes keep accounts apart for the clients; they are no wall against an Agent that runs
   code. That last sentence restates D60 and is the coordinator's. No Claude Agent becomes a dogfood
   slot before D58's harness change is in place (point 1), and, as [ruling
   82](../ruling-82/README.md), point 3, has it, a Claude Agent's sign-in follows order 051b.
3. **Second accounts keep the S1-pin staging** (the answer to the second of the first questions).
   A5's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes) stands: the second
   accounts' first sign-ins are A5's stage 2, at the S1 rerun's pin, signed in only through
   Conduit's Interface, and no Agent doing Conduit's own orders names a second account until the
   owner says so. Point 2 reaches primary accounts only, as "Yes, primaries only" says.

## What it amends

- **Ruling 75, B3**: the planned wall is a sandbox method, Docker first, not a dedicated macOS
  user (point 1); that B3's timing is not carried over is the coordinator's reading. Its later
  answer of 2026-11-12, "B3's timing", which kept the user where B3 put it, falls with it. That
  answer's guard until then, `main`'s branch protection ([ruling 74](../ruling-74/README.md), point
  1), stands until a sandbox method is in place, by the coordinator's reading: the owner said
  nothing of the guard.
- **D61**: its A22 precondition, and its preparation for B3, go; its D58 precondition stands
  (point 1). D61 is the coordinator's decision, logged for the owner to strike (ruling 48); this
  ruling settles that part of it.
- **Ruling 82, point 6**: A22 is no longer owed before any Claude Agent is a slot (point 1), and
  its line that A81 and A22 "stay owed as before" holds for A81 only. That point 6 kept A22 there
  only as D61 placed it, so that amending D61 reaches it, is the coordinator's reading.
- **A22**: its session (c) and its Claude login case are withdrawn, and its session (b) runs
  without the dedicated user; sessions (a) and (b) are optional (point 1).

## What it does not change

- **Ruling 75, B1 and B2, and C3 as ruling 80 amends it**: the folder is the grant; confinement
  limits live on the runtime provider, which point 1 relies on; Conduit manages the connection a
  Claude sign-in is completed over and never carries the credential.
- **Ruling 70**: Agents run code, permissively for now, and confinement comes later through a
  runtime provider (its point 4). Point 2 relies on it.
- **Ruling 40**: point 1 places the sandbox methods within it, as providers; point 2's isolation
  comes through the runtime contract, as point 1 of this ruling reads it.
- **[Ruling 77](../ruling-77/README.md), point 2**: `conduit-run` stays the single run user on the
  Windows computer, and the SSH provider's contract still states that PRV-025's isolation is unmet
  "until each Agent has a user of its own"; that statement stays true as a limit. Whether it comes
  to read "until a sandbox method confines each Agent" is left to the study and the owner.
- **D58, D59 and D60**: they stand (point 1).
- **A5's staging and its guards**: point 3 keeps them.
- **The Agent packages 0.2.0**: nothing changes in 0.2.0. No member, order or archive of 0.2.0
  depends on the dedicated user.
- **[Ruling 78](../ruling-78/README.md), point 1, part 2, and ruling 82, point 3**: the pin waits
  for the sign-in orders, and the owner signs each dogfood Agent in once, through Conduit's sign-in
  flow, after order 051b.
- **[Ruling 80](../ruling-80/README.md), point 7**: A78's orders 2 and 3 run through Conduit one at
  a time until order 050 and the review's order 62 let orders run in parallel as a Fleet.
- **[Ruling 83](../ruling-83/README.md)**: the Agent packages 0.2.0 stay subscription-only.

## Facts recorded with it

- The owner's time, on 2026-11-18: one cut-off typed answer and one answer to the first questions,
  the interruption, and two answers to the second questions. The coordinator's notes record no
  times.

## What follows

- A grounded study of sandbox methods, before any design, and later: Docker first, then the
  abstraction across Apple's container, OrbStack, Lima, Podman, and bubblewrap on Linux. It reads,
  from their documentation and source, how each confines a whole process, what each mounts and lets
  out, and how each maps onto the runtime provider's contract (ACC-027, PRV-006) and ruling 40;
  and, for each client, where its login lands inside a method and how a sign-in completes there,
  the question A22 asked for the dedicated user. It needs no client run. It decides nothing; the
  design, with the study, comes to the owner. That scope is the coordinator's reading. When its
  order is written, the study gains a planned row, with an A-number, in
  [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes)'s register.
- Nothing changes in 0.2.0, and no order now in the chain changes for this ruling.
- [IN_FORCE.md](../../../IN_FORCE.md): topic 9 gains point 1 as a direction, not spec pending, and
  its *Open* line on A81 and A22 keeps A81 only, with A22's narrowed place in a line of its own;
  topic 15's line on Conduit building Conduit gives up B3's dedicated user for points 2 and 3; and
  its coordinator's line records that this ruling settles part of D61.
- [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A22's row is narrowed: its session (c)
  and its Claude login case go, its session (b) keeps its daemon case without the dedicated user,
  both remaining sessions are optional, and the row names no dedicated user and no gate. No other
  row names the dedicated user as a precondition. A82's row, which said the dedicated user of B3
  does not change its placement, now says that a sandbox method (ruling 84) is what would change
  it.
- [The ledger](../ruling-log.md) gains row 84, and the status of rulings 75 and 82, there and in
  their records, notes this ruling's amendments. Ruling 75's record also points to this ruling
  after its later answers.
- An order corrects, with a dated amendment that changes no step, measure or pass condition, the
  places that still name the dedicated user or A22's gate: experiment 028's registration of the
  Claude verifier slot, added "only after D58's guards and A22's open case have run (D61)", which
  becomes after D58's guards only; experiment 028's and experiment 027's sections on what a result
  cannot show, which name the dedicated user as what comes later; and the limits in
  `engine/CONFORMANCE.md`, which hold "before and after ruling 75 B3's dedicated macOS user". The
  merged work orders that cite B3 or D61 are history and are not edited.
- [The tracker](../../AUDIT.md#waiting-for-the-owner) gains a row under "Waiting for the owner":
  with batch 2's plan, what A78's orders 4 and 5 test (D59), and when a sandbox method must be in
  place and what it gates (point 1).
- The full review's
  [decisions](../full-review/decisions.md#dogfooding-and-the-evidence-kept-in-the-tree): D61's
  row gains a dated note citing this ruling, as D55's and D80's carry ruling 82's. In
  [its experiments](../full-review/experiments.md), A22's row gains a like note, and in
  [its questions](../full-review/questions.md), so does the statement that needs no answer, on
  creating the dedicated user at order 050's sitting.
