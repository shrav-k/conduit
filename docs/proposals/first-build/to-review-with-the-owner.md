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
| 1 | **A missing object is reached only through `management`** (finding 2-08). A caller with only a Pool's authority who names an object that does not exist is refused, not told it is invalid | It reverses OPS-029, which had said invalid through either, on purpose. The reason: names are global, so the old text let a Pool's caller learn whether a name exists outside its Pool. Striking it restores OPS-029 | [log of increment 2](log-increment-2.md), 2-08; OPS-029, ACC-033, LIF-037 |
| 2 | **`-32602` and `-32020` from a `plain` server are not proof that it did nothing** (decision 8a, narrowed by finding 2-07). An effect after either ends unknown, and after `-32020` only an operation with no effect is sent again | It corrects a safety reading of the specification. Allowing an effectful call to be sent again would need a contract to declare it safe to repeat, which would be a feature, and the owner's | [the check](check-plain-errors.md); [decisions](increment-2/decisions.md), 8a; IFC-010, IFC-046 |
| 3 | **A `plain` call's content is kept and given again on a repeat** (decision 5) | What a read returned is kept in the invocation index for as long as DL10 keeps outputs, and a purge of records does not remove it: one unknown write kept 373,365 bytes. A cost in storage and in what the engine holds | [decisions](increment-2/decisions.md), 5; OPS-009, IFC-025, DL10 |
| 4 | **A new promise, SUB-011**: a document, file, submission or archive beyond an engine's stated limit is rejected, and every engine takes at least 64 levels of nesting | The only promise the first build added | [log of increment 1](log-increment-1.md), 1-11 and 1-16 |
| 5 | **A rejection by a schema cites SUB-005**, with the failing keyword's location, rather than the promise the schema encodes | The first build's thesis that every decision cites a promise is met, for rules a schema encodes, through SUB-005 and the keyword's location | [log of increment 1](log-increment-1.md), 1-01 and 1-14 |
| 6 | **What a project reads**: a name beginning with a dot is skipped; files are UTF-8, with one leading byte order mark allowed; a JSON value that is not a map is judged as a YAML one is | They decide which files are a project and how they are read | [log of increment 1](log-increment-1.md), 1-05, 1-06, 1-07, 1-15; 2-01 |
| 7 | **Numbers and grammars**: a number whose binary64 value is whole is held to the integer range; the letters of a cited grammar match either case; of I-JSON, two rules apply | Each narrows or widens what a document may hold | [log of increment 1](log-increment-1.md), 1-10, 1-12, 1-13 |
| 8 | **Schemas an author writes**: a root `$defs` key in a contract copy's form is reserved; two different unconditional defaults for one place are rejected | Each rejects something an author could write | [decisions](increment-2/decisions.md), 6; [log of increment 2](log-increment-2.md), 2-03; IFC-033, SCH-009 |

## How the build is judged

| # | Decision | Why it matters | Where |
| --- | --- | --- | --- |
| 9 | **Counting rules added after the baseline ran**: provider work listed apart, client concepts counted, requests and time without access listed, T4 and T5 against an allow list too, T3 to cover reach around Conduit | Rules set after one side's numbers are known are the easiest to challenge | [the first build](../../implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran) |
| 10 | **The choices on the implementation-defined list** for the first build, such as the digest of contents, generated names and the limits | They fix what the reference engine does where the language leaves it open | `reference-model/CONFORMANCE.md`, written by work order 019 |

## What the owner ruled, for reference

Ruling 48 settled four points with the owner: an Agent's own reach off by default, the shared
login until A4 and A5, patterns kept to JSON Schema's interoperable tokens, and a fresh Codex
author for S1. They are the owner's, not on this list.
