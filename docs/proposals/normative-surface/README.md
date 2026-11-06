# Form of the specification — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit ruling 17), for plan-audit items
[A32 and A33](../../AUDIT.md#p0--make-the-language-concrete-and-complete). Names of
directories, files and fields are illustrative. By vocabulary decision V8 the "normative surface"
is called the specification from here on.

**Revised 2026-11-05 for the [layered claim](../../AUDIT.md#layered-claim) and layer decisions LD1 to
LD5.** The normative surface holds only the language and the shared conventions. Type contracts
live with their types in a standard library, and the runtime profile lives with the reference
engine; neither is part of the language.

## Why this is needed

"Rebuild the engine from the language alone" needs a finite answer to "which documents are the
language?". Today accepted rules are spread across seven protocol topics, about 120 "Accepted"
paragraphs in the proposals folder, and three archived reviews. A builder cannot tell which
sentences bind. This proposal draws the line and fixes one form for everything inside it.

## Decision NS1 — what the language consists of

Proposed: the language is exactly the contents of one directory, `spec/`. Everything else in the
repository explains, records or tests it.

| Part | Contents | Form |
| --- | --- | --- |
| Manifest | The list of normative files, the reading order, and what a conformance claim must state | One Markdown file |
| Definitions | The five families and their forms; the object envelope; identity; and, for each kind of object, the states it can be in and the events that move it between them | Markdown with state tables |
| Schemas | The envelope, each family and form, type contracts, operation and requirement contracts, the package entry document, the instantiation request | JSON Schema 2020-12, written as strict YAML |
| Promises | Every required behavior, one file per topic | Strict YAML data (NS2) |
| Shared conventions | The six behaviors every part must agree on, stated as invariants | Promises |
| Plugin interface | The shared wire between an engine and a plugin, and the profile carried on capability calls | Schemas plus promises |
| Implementation-defined list | Every behavior deliberately left to implementations, and what each implementation must document about its choice | Strict YAML data |

Outside the language, and so not binding on it: `docs/` in full, `experiments/`, the reference model,
the conformance vectors, every type contract including the core types the scenarios need, and
the runtime profile. A type contract binds implementations of that type; the runtime profile
binds the reference engine. The vectors are binding on a conformance claim, not on meaning: if a
vector and a promise disagree, the promise stands and the vector is corrected. The same holds
for the reference model.

## Decision NS2 — the form of a promise

Proposed: a promise is a data record, not a paragraph.

```yaml
- id: OUT-004
  level: must
  when: >
    An effect was dispatched to a provider whose contract does not declare deduplication by
    operation identity, and no result was recorded.
  then: >
    The operation's outcome is unknown. The effect is not dispatched again.
  observable: >
    Inspecting the operation returns the unknown outcome. The dispatch record shows one
    dispatch for the operation identity.
  vectors: [s4/uncertain-effect-plain-provider]
```

Rules for writing one:

1. **One obligation.** A promise with "and" in its `then` is two promises.
2. **Stated as an observable outcome.** If no observer could tell whether it was kept, it is not
   a promise and does not belong in the language.
3. **Levels are must, should and may**, with their usual standards meaning. A prohibition is a
   `must not` with an observable.
4. **No sentences of the form "this does not imply".** If a reader might wrongly infer a
   behavior, state the correct behavior as its own promise.
5. **Every term is defined** in Definitions. A promise may not rely on a mechanism that is "open".
6. **Stable identifiers.** An identifier is never reused. A changed promise keeps its identifier
   and records the language revision in which it changed.
7. **Every promise names at least one vector**, or the register does not pass its coverage check.

Because promises are data, a small Rust tool can check the register mechanically: unique
identifiers, every promise covered by a vector, every vector naming promises that exist, and
every term used present in Definitions. A reader runs the same tool.

## Decision NS3 — what happens to the current protocol topics

Proposed: each topic in `docs/protocol/` is converted once, applying the LD1 test. Every sentence
becomes one of six things: a promise in the language or a shared convention, a definition, an
entry on the implementation-defined list, material for a named type contract, material for the
runtime profile, or explanation.
Explanation moves to a non-normative guide beside the promises and may be shortened freely. When
a topic is converted, its file under `docs/protocol/` is replaced by a pointer.

The accepted rulings recorded in the [placement](../placement/README.md) and
[instantiation](../instantiation/README.md) records are converted the same way and those records
then become history.

## Decision NS4 — the form of a conformance vector

Proposed: a vector is a data file with three parts.

```yaml
id: s4/uncertain-effect-plain-provider
promises: [OUT-004, OUT-002]
given:
  definitions: [...]        # objects in the language, exactly as an author would write them
  services: [...]           # which services exist and where
when:
  - request: {...}          # a request through a public surface
  - fault: {lose: {message: apply-result, from: provider, to: execution-1}}
  - request: {...}
then:
  - inspect: {operation: op-1}
    expect: {outcome: unknown, dispatches: 1}
```

Events and faults are named in terms of the public surfaces and the plugin interface, never an
engine's internals, so any build can run them. A vector for the language layer needs no events at
all: it gives declarations and expects resolved objects or diagnostics.

## Open points

- Whether promises are authored as YAML with a generated readable page, or as Markdown in a
  rigid, machine-readable layout. This proposal recommends YAML because coverage can then be
  checked without parsing prose.
- The granularity of promise topics and their identifier prefixes.
- Whether the readable guide is written by hand or generated from the promise data.

## Rulings

All four were accepted as written.

1. NS1: is the normative surface exactly the contents of `spec/`, with those parts?
2. NS2: is a promise a data record with these fields and these seven writing rules?
3. NS3: are the protocol topics converted, sentence by sentence, into the six kinds?
4. NS4: is a vector a given, when, then data file whose events are named against public
   surfaces and the plugin interface?
