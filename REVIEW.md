# Collaborator review

Updated: 2026-10-30. [SPEC.md](SPEC.md) and [DESIGN.md](DESIGN.md) contain the current
non-communication contracts. Intersession communication is reopened and consolidated
here; prior room choices below are material to reconsider, not implementation requirements.

## Review order

1. Review the completed [Pi investigation](RESEARCH_PI.md) and settle the focused
   integration choices below. Pi is the user-selected first frontend.
2. Design in detail what it means to establish a “conduit” between sessions, without
   assuming the former room model.
3. Review concrete YAML examples, then the initial experiment in detail, before implementation.

The four-account objective remains: two Codex and two Claude subscription accounts
running concurrently with separate native state and scoped shared knowledge. The
communication portion of that objective will be specified after its redesign. Native
execution remains the default with optional/required isolation as already defined.
The preceding non-communication policy review is complete; Pi research adds the
specific integration questions below. Actual account, context, usage, and
recovery behavior still needs adapter validation in the experiment. Storage engines,
exact schemas, and test cases remain implementation-planning choices; the accepted
contracts do not prescribe an intersession communication model.

## Pi coding-agent research

The supplied [Pi docs](https://pi.dev/docs/latest) and
[repository index](https://github.com/earendil-works/pi/blob/main/packages/coding-agent/docs/index.md)
have been investigated alongside source/tests, pstack, and representative ecosystem
packages. [RESEARCH_PI.md](RESEARCH_PI.md) records the evidence, release/development
distinctions, limited offline checks, and proposed experiment. Its recommendations
are separate from accepted contracts.

Resolve these larger choices before returning to communication:

| Choice | Recommendation and implication |
| --- | --- |
| First runtime composition | Start with four Pi sessions using two Anthropic and two OpenAI subscription identities, then validate native Claude Code/Codex integrations. This tests different models/accounts first; it does not yet prove different runtime interoperability. |
| Integration boundary | Reuse stock Pi's terminal/runtime with a thin extension and Rust Conduit services. Use RPC for headless cases; avoid building a replacement frontend or model/skill framework. Choose a supervision approach before promising detach/reconnect. |
| Account evidence | Keep separate stores/profiles and session-private state. Determine supported identity evidence; Pi login alone does not meet our verified-identity requirement. User-attested enrollment would need an explicit ruling, not silent acceptance. |
| Durable scope | Keep Pi Durable optional and evaluate later for recoverable unattended execution. Reuse its intent/outcome and conformance patterns without making its storage or terminology canonical for Conduit. |

The existing memory lifecycle, scoped retrieval, BYOS, native-default execution,
portable checkpoint, and honest usage contracts remain applicable. The report proposes
targeted validation rather than another exhaustive decision ledger. Exact package
versions, native loading controls, and a minimal backend belong to experiment planning.

## Deferred communication redesign

**Status: reopened, after review of the Pi findings.** This is the sole review home for room-specific
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

Use the local `agentwatch` reference for concrete communication behavior and the
[Pi communication research](RESEARCH_PI.md#communication-research-for-the-upcoming-redesign)
for native input timing, pi-messenger delivery, and pi-chat supervision. Separate
addressing, authorization, retained exchange, attention, and execution when comparing
options; this is a discussion frame, not a selected object model.
`jxp-skills` and provider-native tracking remain
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

Use the [proposed trial sequence](RESEARCH_PI.md#proposed-experiment-sequence) as input.
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
automatic classification/extraction, automatic expiry, exact cost accounting, plugin ABI, and crate
layout remain staged. Communication delivery order is reopened above. Pi is the first
frontend; Conduit's own chat and additional native runtime integrations remain goals.

Keep reasoning beside accepted contracts in DESIGN.md. Remove answered questions here
and use Git history for earlier versions. The communication material above is a focused
redesign brief, not an additional historical decision register.
