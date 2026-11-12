# The coordinator's decisions to go over with the owner

Not binding. The specification in [spec/](../../../spec/README.md) alone binds. On 2026-11-09
the owner said of the decisions the coordinator made in the first build: "I accept your
decisions for now but we'll need to go over the impactful ones together later". This page lists
those that change what the language says or how the build is judged, most consequential
first, so that the review can start from it. Each is already logged where it was made, and each
can still be struck. The coordinator keeps the list current as it decides more.

## Changes to what the language says

| # | Decision | Why it matters | Where |
| --- | --- | --- | --- |
| 1 | **A missing object is reached only through `management`** (finding 2-08). A caller with only a Pool's authority who names an object that does not exist is refused, not told it is invalid | It reverses OPS-029, which had said invalid through either, on purpose. The reason: names are global, so the old text let a Pool's caller learn whether a name exists outside its Pool. Striking it restores OPS-029 | [log of increment 2](README.md#2-08), 2-08; OPS-029, ACC-033, LIF-037 |
| 2 | **`-32602` and `-32020` from a `plain` server are not proof that it did nothing** (decision 8a, narrowed by finding 2-07). An effect after either ends unknown, and after `-32020` only an operation with no effect is sent again | It corrects a safety reading of the specification. Allowing an effectful call to be sent again would need a contract to declare it safe to repeat, which would be a feature, and the owner's | [the check](increment-2/decisions.md#evidence-for-decision-8a); [decisions](increment-2/decisions.md), 8a; IFC-010, IFC-046 |
| 3 | **A `plain` call's content is kept and given again on a repeat** (decision 5) | What a read returned is kept in the invocation index for as long as DL10 keeps outputs, and a purge of records does not remove it: one unknown write kept 373,365 bytes. A cost in storage and in what the engine holds | [decisions](increment-2/decisions.md), 5; OPS-009, IFC-025, DL10; [implemented cap](../../../engine/CONFORMANCE.md): 64 MiB and 7 days |
| 4 | **A new promise, SUB-011**: a document, file, submission or archive beyond an engine's stated limit is rejected, and every engine takes at least 64 levels of nesting | The only promise the first build added | [log of increment 1](README.md#1-11), 1-11 and 1-16 |
| 5 | **A rejection by a schema cites SUB-005**, with the failing keyword's location, rather than the promise the schema encodes | The first build's thesis that every decision cites a promise is met, for rules a schema encodes, through SUB-005 and the keyword's location | [log of increment 1](README.md#1-01), 1-01 and 1-14 |
| 6 | **What a project reads**: a name beginning with a dot is skipped; files are UTF-8, with one leading byte order mark allowed; a JSON value that is not a map is judged as a YAML one is | They decide which files are a project and how they are read | [log of increment 1](README.md#1-05), 1-05, 1-06, 1-07, 1-15; 2-01 |
| 7 | **Numbers and grammars**: a number whose binary64 value is whole is held to the integer range; the letters of a cited grammar match either case; of I-JSON, two rules apply | Each narrows or widens what a document may hold | [log of increment 1](README.md#1-10), 1-10, 1-12, 1-13 |
| 8 | **Schemas an author writes**: a root `$defs` key in a contract copy's form is reserved; two different unconditional defaults for one place are rejected | Each rejects something an author could write | [decisions](increment-2/decisions.md), 6; [log of increment 2](README.md#2-03), 2-03; IFC-033, SCH-009 |

## How the build is judged

| # | Decision | Why it matters | Where |
| --- | --- | --- | --- |
| 9 | **Counting rules added after the baseline ran**: provider work listed apart, client concepts counted, requests and time without access listed, T4 and T5 against an allow list too, T3 to cover reach around Conduit | Rules set after one side's numbers are known are the easiest to challenge | [the first build](../../implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran) |
| 10 | **The choices on the implementation-defined list** for the first build, such as the digest of contents, generated names and the limits | They fix what the reference engine does where the language leaves it open | `reference-model/CONFORMANCE.md`, written by work order 019 |

## The first review, 2026-11-10

The coordinator walked the owner through items 1 to 10, recommending that each be kept, that a
cap be added later to what item 3 keeps, and that item 9 be looked at together. The owner
answered "Agree on all". Items 1 to 10 stand as decided. For item 3, the coordinator owes a
limit on the content kept for repeats (taken into increment 3's work). Item 9's counting rules
stand; they are reported apart from the rules set before the baseline, so a reader can judge
the comparison by either set. A decision made after this date goes below as a new item.

The content-retention cap owed by item 3 is now implemented: 64 MiB and 7 days, as stated in
[CONFORMANCE.md](../../../engine/CONFORMANCE.md).

## Since the first review

| # | Decision | Why it matters | Where |
| --- | --- | --- | --- |
| 11 | **An operation that declares no outputs is presented with no `outputSchema`, and its results carry no `structuredContent`** (finding 3-03). The specification had given it an empty map in both | Found with the real Claude Code: shown a tool with an `outputSchema`, it gave its model the empty structured result and not the file's text, so every read of a `plain` server looked empty. It changes IFC-025 and IFC-033, the tool contract every engine presents | [log of increment 3](README.md#3-03), 3-03; WORK_ORDERS record, 024 |
| 12 | **An Agent's own machine tools are off by default**: a client's shell, code execution, browser, other Agents, hooks and plugins, beside the account services and messaging that ruling 48 turned off | Found with the real Codex: with its shell on, it read the shared directory around the engine and made no request of it. It extends ruling 48's point 1, which was the owner's, from the Agent's account to its machine | [log of increment 3](README.md#3-05), 3-05; WORK_ORDERS record, 024 |
| 13 | **Each Agent's client loads no instruction file or setting source of its own**: Claude Code with no setting source, Codex with no project document | Found with planted sentinels, which both clients loaded from a directory above their private working directory. It decides what an Agent knows beyond its turn and its tools | [log of increment 3](README.md#3-04), 3-04 and 3-05 |
| 14 | **The engine's credentials name an expiry 100 years ahead** (finding 3-01) | DL13 and JWT-SVID require an expiry; a credential lasts as long as its instance in this version, so the expiry only has to be far enough away | [log of increment 3](README.md#3-01), 3-01 |

On 2026-11-10 the coordinator walked the owner through items 11 to 14, recommending each be
kept, and the owner answered that everything stands. Items 11 to 14 stand as decided.

## Since items 11 to 14 were confirmed

| # | Decision | Why it matters | Where |
| --- | --- | --- | --- |
| 15 | **A Codex Agent's sub-agents are a stated route, not a bypass** (finding 4-06). Codex keeps its `collaboration.*` tools under every switch tried; a sub-agent it spawns had no shell, could not read outside its sandbox and was refused a write | A Codex Agent can run more model work in parallel than one conversation, within its own access; what offers the tools is still unknown | [log of increment 4](log-increment-4.md), 4-06 |
| 16 | **The Codex package also disables `multi_agent_v2` and `multi_agent_v2_dynamic_tools`** (work order 027), though the trial did not show them to be the cause | Harmless if useless; the package's arguments grow | finding 4-06; WORK_ORDERS record, 027 ([current record](../../implementation/WORK_ORDERS.md#order-027)) |
| 17 | **The engine preinstalls no package of S1** (work order 028): S1's filesystem and history packages are installed like any other | The engine knows nothing of a scenario; tests and the live project install them | work order 028, decision 5, quoted in WORK_ORDERS record, 028; [C0 decision 5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/work-orders/028-i4-03-commands.md#L45-L49) |
| 18 | **An ended condition is shown apart, with when it ended, in the CLI's output only** (finding 4-17), since the specification has no form for it | Inspection through the core still shows conditions as the specification has them | [log of increment 4](log-increment-4.md), 4-17 |
| 19 | **A retire hook's part is unavailable, not unknown, where the run's state is unknown** (work order 027, TPL-018, OPS-022) | Decides how such a retirement ends | finding 4-04 (TPL-018, OPS-022); WORK_ORDERS record, 027 ([current record](../../implementation/WORK_ORDERS.md#order-027)) |
| 20 | **Hosted search is an account service** (order 050a, ruling 48 point 1): Codex's `web_search` and Claude Code's WebSearch are on only when both `accountServices` and `ownTools` are true | Search runs on the vendor's side under the account, like the account's connectors | [Order 050a](../../work-orders/050a-agents-v2-own-tools.md) |
| 21 | **The turn deadline is an input of `.v2`**, `turnDeadlineSeconds`, default 180 as `.v1`, at most 604800 (order 050a) | Real work runs for hours; NAM-024 fixes the bounds for the life of `.v2` | [Order 050a](../../work-orders/050a-agents-v2-own-tools.md) |
| 22 | **`effort` is an input of `.v2`**, whose empty default leaves the client's own (order 050a) | The dogfood workers run at high effort, as `run-order.sh` does | [Order 050a](../../work-orders/050a-agents-v2-own-tools.md) |
| 23 | **The client's MCP tool timeout equals the turn deadline** (order 050a) | Otherwise the client cuts off a long call to the engine inside a long turn | [Order 050a](../../work-orders/050a-agents-v2-own-tools.md) |
| 24 | **Project instruction files stay off in `.v2`** (order 050a): an order tells its worker to read AGENTS.md | The Agent's instructions come from its turns, not from files the client loads on its own | [Order 050a](../../work-orders/050a-agents-v2-own-tools.md) |
| 25 | **On another machine, an Agent's own tools run as that runtime provider's contract states** (order 078): ruling 70 point 2's "as the engine's user" read as the engine's own runner | Names no user, so it leaves the held M2-2 open | [Order 078](../../work-orders/078-agents-codex-path-item.md) |
| 26 | **The https listener admits a token by its audience alone** (order 079), as IFC-042 says, without the plan's extra placement clause | The plan's clause would have narrowed ruled behavior | [Order 079](../../work-orders/079-shell-https-address.md) |
| 27 | **A run the provider reports running, with no bridge proving it, stays unknown** (order 080), the cautious reading of IFC-017; the tension with PRV-009 is a finding | Setup does not succeed on an unverified session | [Order 080](../../work-orders/080-shell-runtime-provider-calls.md) |
| 28 | **A newly admitted node starts `Ready: Unknown`** (order 049a1), the cautious reading of PLC-023 | The node is unavailable until its first report | [Order 049a1](../../work-orders/049a1-build-enrollment-core.md) |
| 29 | **Records promise what the engine does and can detect** (order 082, MC7b-001): REC-003 and REC-006 narrowed, with DL8 stating which losses outside the engine it tells | A loss or alteration outside the engine that it cannot detect is a stated limit, not a broken promise; tamper evidence stays a possible feature of a v1 provider | [Order 082](../../work-orders/082-spec-records-and-installs.md); REC-003, REC-006, DL8 |

On 2026-11-12 the coordinator put items 15 to 28 to the owner, recommending each be kept, and
item 29 apart, recommending it be kept with the missing limit sentence added to `spec/README.md`.
The owner answered "Agree with all decisions except need more info on 2", where 2 was a question
not on this list. Items 15 to 29 stand as decided.

## What the owner ruled, for reference

Ruling 48 settled four points with the owner: an Agent's own reach off by default, the shared
login until A4 and A5, patterns kept to JSON Schema's interoperable tokens, and a fresh Codex
author for S1. They are the owner's, not on this list.
