# Round 6 of the re-audit: log of the interface pass

Date: 2026-11-07. This pass applies [ruling 44](../../ruling-44/README.md) to
`spec/interface/implementation.yaml` and `spec/implementation-defined.yaml`: finding 6-1-01
(IFC-007, IFC-010, IFC-045) and finding 6-1-04 (IFC-033) of the [findings](reader-1.md), the
critique D07 (IFC-033, IFC-012) of the [design critique](design-critique.md), and what D14 asks
of an interface promise. It edits no other file under `spec/`. One promise is added, IFC-047;
none is removed or moved, and each changed promise keeps its ID. Both files gain one
head-comment entry for ruling 44 and round 6.

The independent check (the journal's `verify:01`, `verify:04` and `verify:simplify`) was read
first. Its recommended least changes were followed, and the pitfalls it named were kept out:
-32602 and IFC-046's -32020 are not touched, the copies of IFC-033 are keyed so that two
packages cannot collide, recursion is handled with one copy for each contract, and the
narrowing of D07 is built on the copies rather than on literal replacement.

## Applied

| What | Promise IDs | From | Why |
| --- | --- | --- | --- |
| `then`: -32021 for a client capability that the call did not declare is no longer listed among the errors read as failed, so it falls under the table's two other rows: failed on an operation whose contract declares no effect, and otherwise unknown. Observable: the protocol has a server answer -32021 where "processing" needs a capability, and does not say that it answers before acting; the error is read as any other, and a repeat of an invocation so left unknown is not dispatched to the plain implementation (OPS-012); the failed error of a task may be -32021. | IFC-010 | 6-1-01 | Decided detail 01. Reverses V2-15 of round 5, whose reason inferred no effect from the need for a capability. |
| Observable: IFC-010 reads the -32021 of a plain server as any other error, failed for an operation with no declared effect and unknown for one with an effect, where it said "as failed". | IFC-007 | 6-1-01 | Decided detail 01; IFC-007, IFC-010, IFC-045 and OPS-007 change together. |
| Observable: "A server that answers a call so has not acted on it, and the outcome is failed" is replaced: the protocol does not say that a server answers so before acting; from a plain implementation the error is read as IFC-010 reads any other; an implementation that carries Conduit's keys answers so only where it has performed nothing, under IFC-047, so the outcome is then failed under OPS-007, as for IFC-008's error. | IFC-045 | 6-1-01 | Decided detail 01: failed stays only where Conduit's own contract binds the server. |
| Added, `must`, layer `interface`: an implementation that carries Conduit's keys answers a call with -32021, because processing it needs a client capability that the call does not declare, only where it has performed nothing for the call. Observable: why this is Conduit's own obligation, as IFC-008's is; that the call states what it declares, so such an implementation can tell before acting; the outcome failed under OPS-007 and a repeat dispatched under OPS-024; a plain implementation is not bound by it. | IFC-047 | 6-1-01 | Decided detail 01, "a duty ... that it answers so only before acting". Written as its own promise; see the choices. |
| `then`: the tool's schemas are made to hold every schema they refer to. Each Schema contract reached, directly or through another, has one copy under the root's `$defs`, keyed by its qualified reference with `@` and its package's or project's version after the address; each `$ref` is rewritten to refer to what it referred to where it was written, as SCH-015 resolves it there, a JSON Pointer to the same place and a contract reference to the copy; every keyword beside a `$ref` is kept; a `$ref` to a contract whose schema holds no `$ref`, with no keyword beside it, may be replaced by that schema. Observable: why literal replacement fails (pointers resolve against the root; `#` and recursion); that validity is the same as against the contract's own schemas; that every `$ref` is local; why the key holds the version (NAM-014) and how a `$ref` to a copy is escaped (RFC 6901, `%23`), with an example; that the root's own `$defs` of an ordinary operation keeps its keys beside the copies (D14); that CTR-012 and the profile govern the contract and not the definition, and the engine checks against the contract; that the choice to replace in place is DL12. | IFC-033 | 6-1-04, D14 | Decided detail 04. |
| `then`: for a control, an input that the target's definition narrows under TPL-010 has the schema `{allOf: [a, b]}`, `a` the contract's schema for it and `b` the schema the template added. Observable: the 100/50 example, and that a control the target does not have has no tool, under IFC-012. | IFC-033 | D07 | Decided detail D07. |
| `then`: no tool is presented for a control that the Conduit's target does not have, under TPL-013. Observable: such a control is named beside an operation the target holds no contract of and `get-secret.v1`; a Conduit may list it, and no tool is shown all the same; a call that names the tool an open Conduit would give it is a request through that Conduit and reports invalid under TPL-013, unless OPS-023 gives another reason first; it is answered in the protocol's form for an unknown tool, with the request's reason in `data`, and recorded with that reason. | IFC-012 | D07 | Decided detail D07; see the choices for the reason and for the two cases of TPL-013. |
| `subject` becomes "Protocol revisions, `content`, `Origin` and the schemas of tools"; `document` adds which `$ref` to a Schema contract whose schema holds no `$ref` the engine replaces by that schema in a tool's definition, where IFC-033 lets it. | DL12 | 6-1-04 | IFC-033's "may" gives two wire forms; an engine's conformance document states which it presents, as DL12 already does for `content`. |
| One head-comment entry in each file, naming every promise changed and added and the finding or critique. | none | all | The rules of the pass. |

### What D14 asks of the interface

The protocol's schema at revision 2026-07-28
([`schema.ts`](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts),
`interface Tool`) types `inputSchema` as `{ $schema?: string; type: "object"; [key: string]:
unknown }`, says "any JSON Schema 2020-12 keyword may appear alongside `type`", and advises
declaring `properties` at the root and keeping composition branches to presence assertions; it
types `outputSchema` as `{ $schema?: string; [key: string]: unknown }`, which "can be any valid
JSON Schema 2020-12", with no `type: object` required. CTR-012, as the contracts editor changed
it, follows both for an Operation that is not a control and keeps `type: object` at both roots
and the four keys for a control. The interface promises that read an Operation's schemas read
inputs and outputs as the members of the root's `properties` (IFC-001, IFC-011, IFC-025), which
CTR-012 keeps, and IFC-033's line that a control's `required` lists `revision` alone still
holds, since a control keeps the limited root. The one interface change D14 needs is in
IFC-033: the root of an ordinary operation's schema may now state `$defs` of its own, beside
which the copies stand. No other interface promise changed for D14.

## Choices beyond the record

| Choice | Why |
| --- | --- |
| The duty of an implementation that carries Conduit's keys is a new promise, IFC-047, and IFC-045's observable points to it, where the decided detail said "a duty in IFC-045". | IFC-045 is an obligation of the engine, on what a call declares; the duty falls on the implementation. A promise holds one obligation (rule 1 of [the form of a promise](../../../../spec/README.md#the-form-of-a-promise)), and the conformance levels count a promise by who bears its obligation, so one promise cannot bind both. The coordinator's [decisions](DECISIONS.md) and OPS-007, as the conventions pass wrote it, already cite IFC-047. |
| IFC-047's `when` covers every call that the engine sends such an implementation: for a request, for a hook, and a provider call. | IFC-045's `when` covers all three; the duty is the same for each. Only the outcome of a call for a request or a hook is read under OPS-007, so the observable names that case alone. |
| IFC-010 keeps -32602 failed whatever the effect. | `verify:01` named it, and IFC-046's -32020, as resting on a similar inference, and advised a check of their own; the tracker lists both as open. Decided detail 01 is about -32021 alone. |
| The copies are made by rewriting every `$ref`, not by embedding each with its own `$id` as JSON Schema 2020-12 section 9.3.1 describes. | The profile keeps `$id` out of every schema, rewriting needs no new keyword, and `tools/check_schemas.py` already bundles so. IFC-033 promises validity, not error paths, so the paths that rewriting changes do not matter. One method stated gives two engines the same wire schema. |
| A copy's key is the contract's qualified reference with `@` and the version after the address, such as `example.org/acme/files@0.3.0#path.v1`, the version left out only for a submitted project whose manifest states none. | Under NAM-014 a package resolves through its own manifest, so one definition can reach the same qualified reference at two versions of one package; the version keeps them apart. `@` is not a character of an address (NAM-005), so the key cannot be read two ways. A project's contracts are reached by no package, and PKG-029 and MAN-016 keep an installed package off its address, so its key needs no version to be unique. |
| The example in IFC-033 uses the address `example.org/acme/files`. | `tools/set_names.py --check` reads a token of a dotted name, `/` and one word in this file as a `_meta` key, so `example.org/files` failed it. |
| A `$ref` may be replaced in place only where no keyword is written beside it; a copy then unused may be left out. | Merging a contract's schema into keywords beside its `$ref` can lose one of two equal keywords, such as two `maximum`s; with nothing beside it, the replacement is exact. |
| D07's narrowing is written as the fixed form `{allOf: [a, b]}`, where the decided detail said "for example". | One stated form gives two engines the same wire schema. `b`'s own `$ref`s are rewritten like any other, as SCH-015 resolves them in the template, so a narrowing that refers to a contract or holds a pointer stays correct. |
| D07: no tool is presented for a control that the target does not have in either case of TPL-013, its type not listing it or its template's `controls` leaving it out. The decided detail named the template's case. | Ruling 44's words are "a control the instance does not offer is not presented", and TPL-013 defines an instance that does not have a control by both cases. A type that offers a capability need not list its controls (CTR-004), so the first case can arise. |
| D07: a call that names the tool an open Conduit would give such a control reports invalid under TPL-013, unless OPS-023 gives refused or another reason first, and is answered in the protocol's form for an unknown tool, with the reason in `data`, where every other call to a tool that is not presented reports refused. | It keeps TPL-013, which the pass of the templates did not change, true as written: the ruling changed what is presented, not what such a request reports. The Conduit does list the operation, so ACC-001 would not refuse it, and invalid says what is wrong. The form is the protocol's, as IFC-036 already answers a request that reports invalid with an error and no tool result. The alternative is refused, as for every other tool that is not presented and for the operation of NAM-024 in IFC-033, which [the decisions](DECISIONS.md)' cost for D07 could be read as; it would need TPL-013 changed. |

## For the coordinator

- **`spec/definitions.md`, row "tool"**: it says "A call that names a tool the engine does not
  present to the instance reports refused". A call that names the tool of a control the target
  does not have now reports invalid under TPL-013 (IFC-012). The row could also say that the
  schemas presented hold a copy of each contract they refer to, and a control's input the
  narrowing of the target's template (IFC-033).
- **`spec/README.md`**: its summary of ruling 44 cites "IFC-010, IFC-045, OPS-007" for 01;
  IFC-047 is added beside them. The layer `interface` now holds 44 promises, and the whole
  specification 453 (202 language, 207 convention), by the validation command.
- **TPL-013** could say in its observable that no tool is presented for such a control
  (IFC-012). It is true as it stands; nothing requires the change.
- **A residual collision.** Since D14, the root of an ordinary operation's schema may state
  `$defs`, whose keys are any strings. A key written there could equal a copy's key, such as
  `example.org/acme/files@0.3.0#path.v1`. No author would write one, and IFC-033 keeps both, but
  nothing rejects it. A rule in CTR-012 that rejects a root `$defs` key holding `#` would close
  it; that is the contracts pass's file, and a decision this pass did not make.
- **The wire shape changes**, as `verify:04` warned: a tool whose contracts refer onward now
  shows a root `$defs`. In the scratch check below, the tool of `replace.v1` holds 40 copies and
  that of `register.v1` 46. The protocol asks implementations to bound the validation of such
  schemas (section "Composition-Keyword Resource Use"), so a client's limits may bite; the size
  is an engine's limit under DL10 only for what the engine itself validates.
- **A `default` under `allOf`.** Where a control's input schema `a` states a `default` on a
  property, the presented `{allOf: [a, b]}` places it under `allOf`. The definition is not an
  authored schema, so SCH-008 does not reach it, and a control applies no default (REV-006); it
  is noted in case the vectors compare definitions.
- **Vectors to write** (E05 of the critique lists most): a contract with local `$defs`; two
  contracts with the same local name; `$ref: '#'`; a contract that refers to itself by name, and
  two that refer to each other; a `$ref` with a keyword beside it; two versions of one package
  reached from one definition; `register.v1` and `replace.v1` against a valid template; the
  50/100 narrowing; a control left out of `controls`; a plain server that does part of an
  effect and answers -32021, directly and as a task's failed error, with no repeat dispatched.

## A scratch check of IFC-033's construction

Not repository evidence: a Python script in the session's scratch directory, run with
`/usr/bin/python3`, PyYAML 6.0.1 and jsonschema 4.20.0, that builds a tool's `inputSchema` as
IFC-033's `then` states it and validates values against it. No engine ran. With keys and
escaping as stated, it gave the expected answer in all 18 cases: a contract with local `$defs`;
two contracts with the same local name; `$ref: '#'`; a contract that refers to itself by name;
two that refer to each other; a `required` beside a `$ref`; one package reached at versions
1.0.0 and 2.0.0 through two manifests, a string at the first and an integer at the second; the
control narrowing, rejecting 75 and 0 and accepting 50; and `replace.v1` and `register.v1` of the
core package, which accepted the example templates of `spec/schemas/examples/valid/project/`
and rejected one whose `kind` is `Nonsense`.

## Validation

Run from the repository root on the working tree, with the other passes' changes in it:

- The validation command of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): `453 promises
  ok`.
- No line of either file is over 96 characters.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors, all six steps run.
- `git diff --check`: no error.
- `python3 tools/check_docs.py`: 1 error, not caused by this pass:
  `docs/proposals/reaudit/round-6/README.md:23: missing path log-tracker.md`, a log that another
  pass has not written yet.
