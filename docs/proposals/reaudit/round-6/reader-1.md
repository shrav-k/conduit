# Round 6, reader 1, Codex

Read on 2026-11-07 at `c4d7d33`, the head of `main` after pull request 7. `origin/main`
was fetched and matched it. Work is on `audit/round-6-codex`.

This is the owner's next read described in
[AUDIT.md](../../../AUDIT.md#for-the-next-reader). It is a whole-repository reader's report,
not a reader restricted to `spec/`: the entry documents and the tracker supplied the scope,
and earlier decisions were consulted to distinguish new findings from known limits. The read
covered the obligation text of all 452 promises, followed the observables and definitions of
the request, access, outcome, lifecycle, provider and secret paths, and checked the relevant
core contracts and document schemas. It is not an independent execution of every example or
a conformance run. No engine exists to run these situations on.

Six findings for adjudication: **three wrong, two undetermined, one misleading**. Two of the
wrong findings are safety-critical contradictions. Finding 05 has a substantial counterargument
and may instead merit a clarification. Each recommendation below is the reader's, pending a
decision. Nothing under `spec/` was changed. The owner's subsequently requested broader review
of simplicity and usefulness is the companion [design critique](design-critique.md); together
these are one reader's report, not two independent cold reads.

The expanded review changed findings 01 and 02 from `unsafe` to `wrong, safety-critical`.
AUDIT's literal definition asks for a bad result while an engine keeps every promise; these
traces instead show specific procedures conflicting with general safety obligations. Round 5's
M32 used `unsafe` for such a conflict, so the terminology has not been entirely consistent.
This classification change resolves no safety issue and establishes no readiness gate. A
coordinator may use the earlier convention, but should adjudicate the conflicting rules rather
than infer safety from the count of labels.

The important remaining problem is the boundary between an invocation and a later delivery or
retry. One protocol error is being treated as a guarantee about effects that the protocol does
not give. One cache rule treats equality of filter arguments as proof that the filter still
permits returning the result. Both need a failing vector before the readiness gate can rely
on them.

## 6-1-01 — wrong, safety-critical — a missing client capability does not prove that no effect occurred

Promises: IFC-010, IFC-045, OPS-007, OPS-003, OPS-024, OPS-018.

Quotes: IFC-010: "the protocol's error -32021 for a client capability that the call did not
declare, is failed". IFC-045: "A server that answers a call so has not acted on it".
OPS-024: "The repeat is dispatched as a new attempt."

Situation: an existing server, packaged as `plain`, performs the first part of a tool's
effect, such as creating a ticket, then finds that completing the operation requires user
input. The request declared no elicitation capability. The server answers -32021 with
`data.requiredCapabilities` naming elicitation. Conduit records failed. A repeat with the
same idempotency key is dispatched again under OPS-024, without that key reaching the plain
server, and the ticket can be created twice.

The baseline's rule requires -32021 when processing needs a capability that the client did
not declare. It does not say that this check precedes every effect, or that earlier effects
have been rolled back. This is a conclusion from the published definition, not an observed
failure of a server. See the
[MCP per-request fields](https://modelcontextprotocol.io/specification/2026-07-28/basic#meta)
and the
[error's schema](https://modelcontextprotocol.io/specification/2026-07-28/schema#missingrequiredclientcapabilityerror).
The tasks schema also permits a task to fail with a JSON-RPC error during execution; IFC-010
explicitly applies its table to that error. See
[FailedTask](https://github.com/modelcontextprotocol/ext-tasks/blob/main/schema/2026-07-28/schema.ts).

IFC-008 is different: Conduit itself obliges an implementation that rejects its extension's
revision to perform nothing. The generic missing-capability error, especially from a plain
server, carries no corresponding Conduit guarantee. OPS-018's general at-most-once promise
does not supply the missing proof needed to implement this retry rule. The described trace
violates that promise: it is not a fully conforming engine's exploit. Rather, the specific
classification and retry procedures cannot guarantee the general safety obligation for this
otherwise permitted integration. IFC-045's assertion cannot strengthen the external protocol's
guarantee for a plain server that was not written for Conduit.

This disagrees with **V2-15 of the verification of round 5**: its reason infers no effect
from the need for a capability. The error's published meaning does not establish that
inference.

Least change: for an operation that declares an effect, read a generic -32021 as unknown,
unless an explicit contract guarantees that this particular rejection precedes every effect.
Keep IFC-008's narrower failed case. Adjust IFC-010, IFC-045 and OPS-007 together. A vector
should return -32021 after a partial effect and show that a repeat is not dispatched to a
plain server.

## 6-1-02 — wrong, safety-critical — an unchanged filter can forbid delivery of a cached result now

Promises: OPS-009, OPS-034, ACC-008, ACC-036, CTR-005, IFC-019.

Quotes: OPS-009: "every filter that applies to the request was sent with its latest attempt".
OPS-034: "with `arguments` equal under SCH-007". CTR-005: "apply it to each request through
the Conduit". IFC-019: "to what the call asks it to do and to what it returns".

Situation: a filter `before.v1` takes a deadline and permits returning a Resource's content
only before that deadline. Such a filter is not prohibited by CTR-005 or the definitions of
a requirement and a filter. A read with an idempotency key succeeds before the deadline. The
engine keeps its outputs. After the deadline, the same actor repeats the same request through
the same open Conduit. The requirement and its arguments are identical to those sent on the
first attempt. OPS-009 therefore requires the old outputs to be returned without dispatch;
OPS-034's guard does not apply. The implementation never gets to refuse the later delivery.

The same problem arises with a filter that consults changing authorization data held by its
implementation. Equality of the requirement's reference and arguments proves neither equality
of that data nor permission to return the earlier content now. ACC-036 checks the current
Conduits, but the engine does not implement the filter. There is no promise restricting
filters to decisions that are invariant over time, and no exemption in the filter contract
for a repeat answered by the engine. ACC-008's general restriction rule and the specific
replay rule therefore cannot both be implemented for this allowed case.

This is a remaining case of **M32 of round 5**, not its original case of a newly added
filter. The fix checks which filters were sent; the situation here uses the same filter and
the same arguments throughout.

The counterargument that this is the same invocation does not settle it: OPS-009 explicitly
calls the repeat a request subject to ACC-036, and IFC-019 covers what is returned. Following
the specific cache procedure violates the general current-access rule; this is an incompatible
pair of obligations, not permission for a fully conforming engine to disclose the result.

Least change: conservatively answer a repeat from stored outputs only when no custom filter applies;
otherwise report conflict without redispatch. If replay through filters is wanted, first
specify a contract for rechecking delivery, or an explicit guarantee under which a filter's
earlier approval remains sufficient. The vector should advance the filter's deadline between
the successful invocation and its repeat, keeping the Conduit and arguments unchanged.
Cost: even a stable filter now prevents otherwise harmless cached delivery. A replay-safe
contract would recover that convenience, at the cost of another guarantee to specify and test.
Also decide explicitly whether filters apply to replay of a successful operation with no
declared outputs, which OPS-009 always answers from its outcome. That is a remaining contract
question; this trace demonstrates the disclosure case, not a new effect from such a replay.

## 6-1-03 — undetermined — two inputs of a control can write the same setting

Promises: CTR-006, OBJ-016, REV-006, REV-007, REV-015; contrast INS-010.

Quotes: CTR-006: "a map from the name of one of its inputs to a JSON Pointer into the
configuration". REV-007: "exactly those values are stored, each at the pointer that the
control's `changes` gives for its input".

Situation: a Resource type describes the integer setting `limit` and lists this control:

```yaml
inputSchema:
  type: object
  properties:
    revision: {type: string}
    a: {type: integer}
    b: {type: integer}
  required: [revision]
changes:
  a: /limit
  b: /limit
```

Every input is declared and both pointers address a place that the type describes. Neither
CTR-006 nor the operation schema rejects it. With `a: 1` and `b: 2`, both values cannot be
stored at `/limit`. Applying the map in either order produces a different configuration;
maps have no semantic key order under SCH-007. The rule that validates the prospective
configuration, REV-015, supplies neither an order nor a rejection for conflicting writes.
The same problem occurs with `/options` and `/options/limit`.
Even supplying `a` alone is contradictory: start with `/limit: 0`, supply `a: 1` and omit `b`.
REV-007 requires `/limit: 1`, while REV-006 leaves the setting of the omitted input `b`, also
`/limit`, at 0.

Verification: both a complete Operation document with this `changes` and its Type document
passed `document.v1` using the repository's bundled schemas and jsonschema 4.20.0. The
repository's `check_controls` returned no error for the pair. This only confirms that the
current document checks accept the example; no control was executed.

Least change: reject a control whose `changes` contains equal pointers or a pointer inside
another, as INS-010 already rejects overlapping destinations of template inputs. State this
in CTR-006, and cover both equal pointers and parent/child pointers. If aliases are desired
instead, specify when conflicting supplied values are rejected before dispatch.

## 6-1-04 — wrong — replacing a contract reference can break its local schema references

Promises: IFC-033, SCH-001, SCH-015, CTR-008, CTR-012.

Quotes: IFC-033: "each `$ref` in them that names a Schema contract replaced by that contract's
schema"; "so the definition can be read with no package at hand". SCH-015: "a JSON Pointer
into the same schema".

Situation: `payload.v1` is a Schema contract with this `spec`:

```yaml
type: object
$defs:
  name: {type: string}
properties:
  name: {$ref: '#/$defs/name'}
required: [name]
```

An operation's `inputSchema.properties.payload` refers to `#payload.v1`. Replacing that
reference with the contract's schema, as IFC-033 says, places `$defs` below
`/properties/payload`, while the local reference still points to `/$defs/name` at the tool
schema's root. That location does not exist. A package-qualified reference inside the
inserted contract needs transformation too. Recursive contract references make unbounded
literal substitution impossible.

JSON Schema resolves a fragment against its schema resource, not against the nearest
enclosing object with `$defs`. Moving a schema therefore requires preserving or rewriting
its reference targets. See
[JSON Schema 2020-12, schema resources and references](https://json-schema.org/draft/2020-12/json-schema-core#section-8.2.3.1)
and [bundling](https://json-schema.org/draft/2020-12/json-schema-core#section-9.3.1).

Verification: jsonschema 4.20.0 accepts `{name: x}` against the standalone schema above.
Validating `{payload: {name: x}}` against its literal replacement in the operation schema
raises `PointerToNowhere` for `/$defs/name`. This is a schema-level reproduction, not a
Conduit engine test.

The repository's `tools/check_schemas.py` already uses a better method for its own checks:
`bundle` places contracts under `$defs` and rewrites both contract references and local
pointers. That Python function does not bind an engine, and IFC-033 does not specify an
equivalent transformation for the tool definition it presents. A semantics-preserving reading
of "replaced" could produce a correct schema. The finding is that the stated construction's
literal reading is wrong and its necessary transformation is not specified, not that every
reasonable implementation must make this mistake.

Least change: require a self-contained tool schema that preserves every original reference's
target and validation meaning, including sibling keywords and recursion. Permit a finite
bundle with rewritten local references instead of requiring literal replacement. Test a
contract with local `$defs`, two contracts that use the same local name, and a recursive
contract. The generated wire schema need not be restricted to CTR-012's authoring root.

## 6-1-05 — undetermined — package-source controls have no explicit admission rule

Promises: PRV-013, PRV-016, PRV-017, CTR-004, CTR-006, REV-007, REV-015; DL17.

Quotes: PRV-013's condition: "A Resource would be instantiated"; its obligation: "for each
URI scheme at most one package-source provider, is not retired". PRV-016: "the schemes that
its configuration lists under `schemes`".

Situation: two package-source Resources owned by `system` are ready. A claims `git`; B claims
`https`. Their Type contracts require `schemes` and expose a control that changes `/schemes`.
Unlike a secret Resource's type, a package-source type is not prohibited from listing
controls. A permitted caller changes B's schemes to `[git]` with the current revision. The
new value satisfies its type's schema, and REV-007 says to store it when the control succeeds.

PRV-013's `when` describes instantiation, but its obligation expressly says "at any time".
The global uniqueness invariant therefore already forbids accepting two live claimants.
DL17 also names a competing provider under PRV-013, and the definitions describe a clash with
something existing as conflict. These are substantial counterarguments to this finding: an
engine can reasonably infer that it must reject the control before changing anything. This
is not a demonstrated permission to create duplicate providers.

What is not explicit is the control's admission procedure: which reason rejects a competing
claim, and how the engine reserves old and proposed claims before an external control acts,
including while its outcome is unknown. Per-instance exclusion in REV-012 alone cannot order
two controls on different providers. The registration rule and the broad definitions admit
different readings of invalid versus conflict here. If the coordinator regards the invariant,
definitions and DL17 as a sufficient answer to all of this, demote this item to a clarification.

Least clarification: state the pre-dispatch uniqueness check and result reason for this control,
with the treatment of pending and unknown changes. A separate simplification recommendation is
to make the claimed schemes fixed for the instance's life, rejecting controls that can change
them. That removes mutable registry semantics, but requires a behavior decision and makes
replacement necessary to change a provider's role. Cover whole-configuration and ancestor-pointer
changes as well as a direct `/schemes` change.

## 6-1-06 — misleading — the entry points still say that V6 waits for the owner

Promises: none; this is repository status.

Quotes: root `README.md`, "What exists": "One point that was found when the fixes were
checked waits for the owner." `docs/AUDIT.md`, "Where things stand": "28 are fixed, and 1
waits for the owner." Round 5's fix log: "Settled by the coordinator on 2026-11-07, under
rulings 37 and 38, as option 1".

Situation: a reader following the root README or the tracker's first table is told to wait
for an owner decision. The tracker's later order of work says that V6 was settled, and
[the end of the fix log](../round-5/log-fixes.md#for-the-owner) records the decision and says
that it was applied. PLC-007 already uses the empty selector for an instance that runs
nothing. The verification table still labels V6 `owner`, and the beginning of that section
still describes one outstanding item.

Least change: update the current summaries to say that all 29 verification findings were
settled, with 28 fixes and V6's coordinator decision. Preserve the historical proposal and
its options, and clearly mark its resolution. No new owner ruling is needed to correct
these summaries.

## The two interface questions singled out by the tracker

At the named baseline, the published
[MCP tools rules](https://modelcontextprotocol.io/specification/2026-07-28/server/tools#error-handling)
still distinguish protocol errors from tool execution errors and let clients withhold the
former from their models. IFC-012's informative `message` helps a client that exposes the
error; it cannot require a client to do so. IFC-025's first text block makes a returned tool
error intelligible without exposing `_meta`. These changes improve the information available
to a client but do not establish that an Agent's model always sees a refusal. This is the
already documented protocol limit, not a new finding.

[SEP-2145](https://github.com/modelcontextprotocol/modelcontextprotocol/pull/2145) was still
open when checked on 2026-11-07. IFC-012's revision-dependent wording avoids fixing the
baseline's unknown-tool response for every later revision. No additional defect was found
on that specific point. The new protocol finding above concerns the distinct -32021 case.

## Verification and limits

At the audited commit, the repository checks passed: `check_docs.py` checked 176 Markdown
files and 2,432 local links; `set_names.py --check` reported no error; `check_schemas.py`
completed all six steps with PyYAML 6.0.1 and jsonschema 4.20.0 and reported zero errors;
`git diff --check` passed. The promise count is 452: 202 language, 207 convention and 43
interface. These counts include IFC-045 and IFC-046.

The two small schema checks in findings 03 and 04 ran in memory with that same interpreter
and validator. They do not establish implementation behavior. Findings 01, 02 and 05 are
counterexample traces against the written rules; no external effect, provider or engine was
run to manufacture evidence for them.

Known absences of conformance vectors and a reference model, the documented trust limits of
secrets, and the known limits of the checker were not counted as new findings. The tracker
separately permits a reader to report four still-open points, including OPS-028's late hook
request, without requiring new evidence. The expanded [design critique](design-critique.md#the-four-open-points)
gives a recommendation on each without counting it as a new discovery. Coverage of supporting definitions, observables,
contracts and schemas was selective, as stated above; it does not establish exhaustive
coverage of every round-5 change or claim that other defects are absent.
