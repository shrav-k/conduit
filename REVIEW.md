# Collaborator review

Updated: 2026-10-30. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) contain the current
non-communication contracts. Intersession communication is reopened and consolidated
here; prior room choices below are material to reconsider, not implementation requirements.

## Review order

1. Resolve the remaining non-communication behavior: recovery and observability.
2. Review the user's Pi coding-agent research and its implications for integration.
3. Design in detail what it means to establish a “conduit” between sessions, without
   assuming the former room model.
4. Review concrete YAML examples, then the initial experiment in detail, before implementation.

The four-account objective remains: two Codex and two Claude subscription accounts
running concurrently with separate native state and scoped shared knowledge. The
communication portion of that objective will be specified after its redesign. Native
execution remains the default with optional/required isolation as already defined.
Account authentication, native-source selection, and skill/plugin lifecycle ownership
remain settled; runtime-specific behavior still needs experiment evidence.

## Active non-communication review

Remaining choices concern retention for knowledge/checkpoints/native history/observations
and operation outcomes, plus usable account/token/context evidence. Communication delivery,
cursors, and acknowledgements are deferred below.

The active review is complete when these behaviors have concrete v0 contracts or an explicit
deferral compatible with the four-account objective. Unverified adapter behavior becomes
an experiment check. Storage engines, exact wire schemas, and every possible failure
mode need not be selected here. Existing portable checkpoint, session-continuity,
resource/MCP, and memory-access contracts remain active; they do not prescribe a messaging model.

## Pi coding-agent research

Review the user's research before returning to communication. Sources and specific
questions are still to be supplied; do not assume conclusions from the earlier Pi
integration example. Revisit the local `pstack-pi` reference alongside that material.

Useful questions to connect to the design include Pi's roles as runtime and interface,
extension and event hooks, session/context ownership, account handling, native tools,
and MCP compatibility. Determine which capabilities an adapter can preserve and which
need Conduit support. These are research topics, not new requirements or a decision to
make Pi the core runtime. Revisit integration boundaries or delivery order only if the
evidence warrants an explicit change.

## Deferred communication redesign

**Status: reopened, after Pi research.** This is the sole review home for room-specific
and intersession communication choices. The user wants a more extensible way to open
lines of communication between sessions and explore a “conduit” as the abstraction.
The term is provisional: this is a semantic redesign, not a rename of the room API.
No topology, communication record kind, participant binding, tracker requirement, or
communication acceptance criterion below is currently fixed.

### Material carried forward for reconsideration

| Area | Previous approach and reasoning to revisit |
| --- | --- |
| Core abstraction | A shared room/message board modeled after group chat, with durable append-only history. Messages were Conduit records with common identity/scope/provenance metadata, immutable after acceptance, with corrections as later posts to avoid shared-document merge conflicts. |
| Memory boundary | No editable room-owned working-memory document. Sessions kept their own working context; scoped knowledge/checkpoints could be linked or quoted. This was motivated by concurrent editing and ownership concerns. |
| Participants and attribution | The Conduit session identified the participant across resumed executions; fresh/fork created new participants. Each post identified its producing execution. Restart/account/frontend changes preserved participant identity. |
| History and catch-up | Shared history, optional participant-local copies, and one independent read position per session per room. One reader did not consume another's unread history. Late joiners/restarted sessions could recover retained messages, with gaps reported. |
| Replies and addressing | One timeline with reply-to links and optional recipient-session hints. No separate direct-message inboxes or thread cursors; the same-room approach was intended to cover small/private conversations too. Hints did not alter visibility or permissions. |
| Access and membership | Effective scopes governed discovery, joining, reading, and posting. Joining was explicit, without a second invitation gate, and enabled participation/notices without granting extra access. Merely acquiring scope access did not join a room or merge participants' memory permissions. |
| Attention and execution | A brief unread notice on the next normal turn, content retrieved on demand, and no automatic wake/model turn. Paused/closed sessions needed explicit resume; role-specific wake hooks were a possible later capability. |
| Delivery and context | Queued content, retries, and replays were to recheck current recipient access. Whole-record checks and scoped provenance could omit inaccessible content, without erasing context already received. Message acknowledgement, retry identity, ordering, and cursor advancement remained unresolved. |
| Tracker relationship | Independent room identity; tracker-free initial testing, followed eventually by required tracker linkage. The transition milestone/cardinality were unresolved. Provider-native tracking was preferred, and a post did not automatically become an external issue comment. |
| Primary tracker proposal | One primary tracker item plus related references was proposed immediately before this deferral. It was not accepted. Do not treat it as a constraint on the new model. |
| Transport and MCP | Local communication first, with room for remote transport later. Candidate MCP tools covered posts/reads, resources exposed history, and notifications signaled unread content. Protocol primitives alone did not establish application-level durability or delivery guarantees. These mappings and sequencing are reopened. |
| Original experiment | Four accounts discussing work through one room, independent catch-up/read positions, visible message provenance and delivery gaps, plus scope-aware context exchange. These communication checks await redesign; the four-account and memory-access checks remain active. |

### Questions for the future deep review

Start with what a line of communication is for, how it is opened/closed, and what
sessions need to exchange. Compare addressing and topology, directionality, message
or event/request semantics, lifecycle, extensibility, and compatibility with native
agent interfaces. Then choose identity/authority, persistence, attention, delivery
assurances, and relationships to work, projects, trackers, and shared knowledge.
Do not constrain that analysis to group chat, a single log, or the previous membership model.

Use the local `agentwatch` reference for concrete communication behavior and the Pi
research for integration possibilities. `jxp-skills` and provider-native tracking remain
useful coordination references. Bring the conclusions back into SPEC.md and DESIGN.md
only after review, replacing this provisional material rather than creating a decision ledger.

## After the review: first YAML design

Discuss concrete examples together before freezing a schema or implementing:

- Choose the configuration schema-version convention, object shapes, names/references,
  file layout, validation, path/endpoint matching, and merge/override rules. Keep versions
  distinct from MCP protocol and plugin versions.
- Define search-limit settings and text-size units; tune default values and caps in the
  experiment.
- Specify managed-worktree creation settings: base revision, branch selection, directory
  placement, and treatment of any existing local changes.
- Express roles, account references, shared connection definitions, scopes, projects,
  and execution policy. Show resolved launch settings and native adapter output. Keep
  credentials out of shareable configuration. Add communication objects only after redesign.
- Consider explicitly writing execution mode in generated role files; decide whether
  handwritten files may omit it and use the native default.
- Check ordinary use and conflicting/incomplete configurations. Preserve native tooling;
  do not require Kubernetes machinery, a replacement skills language, or an internal ticket engine.

## Then: initial experiment design

Select supported host/runtime versions, account enrollment, a minimal memory backend,
and an isolation backend to evaluate. Derive file/service grants and native capabilities,
then verify the agreed contracts. Review setup, expected observations, failure criteria,
restart/cleanup, and trial order before running anything.

Cover four simultaneous identities, permitted/shared and private knowledge, independent
account lifecycle, session continuity, visible execution modes, and honest usage coverage.
Add the communication scenario after its redesign. Specify isolation negative checks
and compatibility gaps, and decide which checks gate the first usable version. A native
run does not establish that isolation works. No account registration or live experiment
is implied by this documentation review.

## Deferred implementation scope

Comprehensive MCP operation inspection/enforcement, advanced memory-sharing delivery,
automatic classification/extraction, exact cost accounting, plugin ABI, and crate
layout remain staged. Communication delivery order is reopened above. Conduit's own
chat and Pi integration remain goals; Pi research now precedes communication review.

Keep reasoning beside accepted contracts in DESIGN.md. Remove answered questions here
and use Git history for earlier versions. The communication material above is a focused
redesign brief, not an additional historical decision register.
