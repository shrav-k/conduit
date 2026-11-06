# Graphs and the schema profile — accepted

Status: accepted by the owner on 2026-11-05 (plan-audit ruling 23, action A53). Written as promises
in the specification: [graphs](../../../spec/promises/graphs.yaml),
[schemas](../../../spec/promises/schemas.yaml) and the
[schema profile](../../../spec/schema-profile.md). The owner added a standing instruction with the
acceptance: for graphs and schemas, do nothing revolutionary and rely heavily on existing
frameworks and paradigms. Spellings follow the [ruled vocabulary](../vocabulary/README.md).

This is the fourth batch of fixes, in the language layer. By layer decision LD2 the language
defines a graph's static meaning and two invariants; how an engine walks a graph is its runtime
profile. The owner has also ruled that the graph engine and the schema profile are not what
differentiates Conduit, so this batch prefers the smallest rule that works.

Defects covered, from the [register](../../LANGUAGE_DEFECTS.md): L2, L13, L16 to L23, L26 and
L111.

## Decisions

The owner accepted every recommendation.

| ID | Decision | Options | Accepted | Defects |
| --- | --- | --- | --- | --- |
| GD1 | What a step's use of another step's output means. | (a) It is a dependency like any other: the step runs only after the step that produces the output has succeeded. (b) The step may run as soon as the output exists, even if the producing step later fails. | (a). It is how Terraform treats a reference, and it leaves one rule for ordering instead of two. | L2 |
| SD1 | Which schema keywords an author may use. | (a) A short closed list: `type`, `enum`, `const`, `properties`, `required`, `additionalProperties`, `items`, `minItems`, `maxItems`, `uniqueItems`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`, `minLength`, `maxLength`, `pattern`, `format`, `default`, `title`, `description`, `examples`, `$ref`, `$defs`, and `allOf`, `anyOf`, `oneOf`. Anything else is rejected when the schema is accepted. (b) All of JSON Schema 2020-12 except what is specifically rejected. | (a). Each further keyword is a place two implementations can differ. The list drops conditionals, `not`, `multipleOf`, `patternProperties` and the dynamic and unevaluated keywords. | L17, L19, L20 |
| SD2 | Which string formats are enforced. | (a) Five, each with its grammar cited: `date-time`, `date`, `duration`, `uri` and `uuid`. An unknown format is rejected when the schema is accepted. (b) All 19 standard formats, as accepted earlier. | (a). Validator libraries disagree on the edge cases of several of the 19, and nothing in the scenarios needs them. This reverses an earlier ruling. | L21 |
| SD3 | Whether `15.0` is an integer. | (a) Yes. Integer means a number with no fractional part, however it is written, as JSON Schema defines it, so YAML and JSON inputs mean the same. (b) No. An integer field requires integer notation. | (a). | L16 |

## Rules

These involved no real choice once the decisions were fixed. The owner accepted them as a group.

| ID | Rule | Defects |
| --- | --- | --- |
| GR1 | A graph is rejected when its template is registered if it names a step that does not exist, contains a cycle once both stated dependencies and uses of outputs are counted, refers to an output the producing operation's contract does not declare, omits a required input, or gives a literal that fails the input's schema. | L13 |
| GR2 | Listing a dependency twice, or listing a step that is also depended on through an output, is accepted and has no further meaning. | L26 |
| GR3 | A template's setup graph may refer to the instance's configuration. A graph that implements an operation may refer only to that operation's declared inputs, whether or not the operation is exported. | L22 |
| GR4 | A pointer into a value applies to inputs, configuration and step outputs. It does not apply to a handle. | L23 |
| SR1 | A number with no fractional part that lies outside the range from −9,007,199,254,740,991 to 9,007,199,254,740,991 is rejected wherever it appears in a document. | L16 |
| SR2 | Two values are equal when they have the same type and the same content: numbers by value, so 1 equals 1.0; strings character by character; lists in order; maps by their keys and values regardless of order. | L17, L111 |
| SR3 | A `default` is honored only on a property reached through `properties`, directly or through `$ref`. A `default` anywhere else, including inside `allOf`, `anyOf`, `oneOf` or `items`, is rejected when the schema is accepted. | L18, L19 |
| SR4 | When a default supplies an object, the defaults of that object's own properties then apply to it, as they would to an object the author supplied. | L18 |
| SR5 | A schema keyword or format outside the lists above is rejected when the schema is accepted, never ignored. Schemas published by an external MCP service keep their own rules. | L20, L21 |

## Where each is written, and what it relies on

| Decision or rule | Promises | Existing standard or practice |
| --- | --- | --- |
| GD1 | GRA-001, GRA-002 | Terraform: a reference is a dependency |
| GR1 | GRA-004 to GRA-008 | — |
| GR2 | GRA-009 | — |
| GR3 | GRA-010 | — |
| GR4 | GRA-011 | JSON Pointer, RFC 6901 |
| SD1, SR5 | SCH-001, SCH-002, SCH-014 | JSON Schema 2020-12, narrowed as Kubernetes narrows OpenAPI for structural schemas |
| SD2, SR5 | SCH-003, SCH-004 | RFC 3339, RFC 3986, RFC 9562 |
| SD3 | SCH-005 | JSON Schema's definition of integer |
| SR1 | SCH-006 | I-JSON, RFC 7493 |
| SR2 | SCH-007 | JSON Schema's definition of equality |
| SR3 | SCH-008 | Kubernetes structural schemas |
| SR4 | SCH-009, SCH-010 | Kubernetes defaulting |
| The two invariants of layer decision LD2 | GRA-002, GRA-003 | Docker Compose and Terraform ordering |
| Accepted earlier: step order has no meaning; the four binding forms; no conversion; the YAML restrictions; JSON equivalence | GRA-012, GRA-013, SCH-011, SCH-012, SCH-013 | YAML 1.2.2, RFC 8259, I-JSON |

One part of the graph language has no counterpart in an existing framework: the object and list
binding forms. The literal and reference forms correspond to Kubernetes's `value` and
`valueFrom`. Whether the object and list forms stay is already queued as a scope question
(plan-audit action A18).

## Not in this batch

- Scheduling, retry, cancellation, what happens to outputs when a later step fails, and when a
  graph is reported complete: the runtime profile.
- Which parts of the graph language stay in the first slice, such as the object and list forms
  and graph-backed operations: plan-audit action A18, after the scenarios have been run.
- Packages, first start and the conformance profile: the next and last batch.
