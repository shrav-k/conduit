# Format and schema decision brief

Reviewed **2026-11-02** after the owner requested a final comparison before committing to the
serialization and data-schema foundation. The owner subsequently accepted constrained YAML,
equivalent JSON input/interchange and JSON Schema 2020-12. The accepted
[configuration foundation](../../protocol/CONFIGURATION.md#configuration-language) records that
ruling. This brief preserves the comparison; remaining profile recommendations are not accepted
merely by appearing here. The [current review](../../REVIEW.md) owns the remaining decisions.
The subsequent [schema-resolution ruling](../../protocol/CONFIGURATION.md#schema-resolution)
also accepts supplied/declared schema dependencies with no automatic fetching during validation.
The owner also accepted [rejecting unrecognized schema keywords](../../protocol/CONFIGURATION.md#schema-keyword-acceptance)
in Conduit-authored contracts. A subsequent ruling selects one dedicated map of namespaced custom
descriptive annotations, reusing standard descriptive annotations where applicable. Its keyword,
namespace syntax and detailed value schemas remain open; custom hints do not implement validation,
authorization or runtime behavior.
V0 subsequently selects all 19 JSON Schema 2020-12 standard formats and defers plugin-defined format
validators. Offline syntactic checks and conformance tests before support claims are accepted;
custom operations and requirement checks remain extensible, and provider MCP schemas retain their
dialect semantics.

## Recommendation and scope

Retain constrained YAML 1.2 for human authoring, explicitly accept JSON as an equivalent input
and interchange representation, and use JSON Schema 2020-12 for data contracts. Define one
JSON-compatible value model and one Conduit interpretation regardless of serialization.
Schemas themselves can be authored as YAML mappings or JSON; the name JSON Schema does not
require authors to maintain a second JSON file beside a YAML source.

This selects a foundation, not every schema keyword, package field, validator or canonical byte
encoding. Conduit's public behavioral contracts still specify graph execution, default
materialization, controls, authority, effects and recovery. Those semantics are necessary with
any of the compared formats. Optional generators can emit accepted documents without becoming
mandatory dependencies of a Conduit runtime.

## Is JSON Schema canonical here?

There is no single schema language shared by all configuration systems, RPC frameworks and
agent protocols. JSON Schema is the strongest directly aligned choice for our MCP/JSON boundary:

| Evidence | Consequence for Conduit |
| --- | --- |
| The [JSON Schema specification index](https://json-schema.org/specification) identifies 2020-12 as the current published version. | The date in its name does not mean we must choose a newer, unrelated language. Pin the dialect rather than a moving “latest.” |
| [MCP 2026-07-28](https://modelcontextprotocol.io/specification/2026-07-28/basic#json-schema-usage) requires support for 2020-12, uses it when a dialect is omitted, and recommends it. | Plugin data contracts can align with our chosen invocation protocol. MCP's overall message source of truth is separately maintained in TypeScript; this does not make TypeScript authoritative for Conduit contracts. |
| [OpenAPI 3.2.0](https://spec.openapis.org/oas/v3.2.0.html#schema-object) bases its Schema Object on 2020-12 with an OAS vocabulary. Its documents can use JSON or YAML. | This is a concrete precedent for human-readable authoring plus a shared JSON data/schema foundation, not a reason to adopt the entire OpenAPI protocol. |
| [Kubernetes CRDs](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#specifying-a-structural-schema) require a structural OpenAPI v3.0 schema with additional restrictions. | Kubernetes is useful design evidence, but its schema is not interchangeable with arbitrary JSON Schema 2020-12. Do not claim universal ecosystem compatibility. |

These are version-specific specification checks, not measurements of market share or proof that
every client, code generator or model API supports every schema feature. OpenAPI 3.2.0 is the
inspected comparison version, not a claim that it is the latest patch release.

## Strong alternatives

The judgments below apply to our accepted MCP-based, language-neutral, plugin/template protocol.

| Alternative | What it offers | Assessment for Conduit |
| --- | --- | --- |
| JSON-only authoring with JSON Schema | A smaller parsing surface and explicit scalar syntax. | Viable runner-up. It makes hand-authored graphs, documentation and multiline instructions less convenient. We can accept JSON without removing YAML. Neither representation adds a second semantic model. |
| CUE as the authoritative configuration/constraint language | Concise reusable constraints, unification and configuration generation; [JSON Schema interoperation](https://cuelang.org/docs/concept/how-cue-works-with-json-schema/). | Strongest alternative for configuration composition. It would add a required language/evaluation and translation boundary for portable contracts. Keep it possible as author tooling; verify preservation of the particular constraints emitted rather than assuming every CUE construct maps to JSON Schema. |
| Dhall as authoritative configuration | Typed, total configuration expressions and reusable imports with [documented safety properties](https://docs.dhall-lang.org/discussions/Safety-guarantees.html). | Useful when programmable configuration is the central requirement. Conduit already has explicit templates and executable graphs; making another evaluator mandatory would add semantics to document and implement. It can remain an optional producer of validated configuration. |
| Smithy or WIT as the main contract language | [Smithy](https://smithy.io/2.0/spec/model.html) models service shapes and traits; [WIT](https://component-model.bytecodealliance.org/design/wit.html) models component interfaces and typed imports/exports. | Valuable sources for our behavioral and linking contracts. Adopting either wholesale would require an additional mapping for configuration and dynamic MCP schemas. JSON Schema alone does not replace their richer contract concepts; Conduit must specify its own behavioral layer explicitly. |
| A new Conduit-specific type notation | Exact control over syntax and specialized types. | Largest authoring/tooling burden. No current example demonstrates a need to replace the established data-schema vocabulary. Special behavioral contracts can remain explicit without inventing basic object/array/string validation again. |

JSON-only is the simpler implementation choice; YAML plus JSON is the better authoring choice
for this project in our assessment. CUE is worth reconsidering only if the concrete authoring
walkthrough exposes essential constraint or generation behavior that our current model cannot
represent cleanly. Choosing JSON Schema does not promise automatic code generation, full static
graph compatibility checking or model-provider acceptance of every published schema.

## What YAML and JSON would mean together

For a field whose contract is an integer between 1 and 100, these documents have the same value:

```yaml
limit: 15
```

```json
{"limit": 15}
```

Either document is checked against the same schema, which can itself be written as YAML:

```yaml
$schema: https://json-schema.org/draft/2020-12/schema
type: object
properties:
  limit:
    type: integer
    minimum: 1
    maximum: 100
required: [limit]
additionalProperties: false
```

The schema validates data. It does not authorize changing a running instance's limit, identify
the relevant control handler, confirm that the change happened, or define retry safety.
Those remain Conduit contracts. JSON Schema composition also does not establish that two
different versioned capability contracts mean the same thing.

The [YAML 1.2.2 specification](https://yaml.org/spec/1.2.2/#103-core-schema) describes Core scalar
resolution; its term “schema” concerns how YAML values are interpreted, separately from JSON
Schema's validation vocabulary. Our accepted restrictions remove non-string keys, non-finite
numbers, aliases, merge behavior and custom tags. Comments and multiline strings remain.
Duplicate keys and unsupported numeric precision must be rejected before values are lost.
The JSON input path needs the same protections: ordinary JSON parsers can also collapse duplicate
keys or round numbers. [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259.html) describes these
interoperability limits.

Equivalent values need not preserve source formatting, comments, mapping order or scalar spelling
on export. Values must retain their meaning, including missing versus null versus empty. A future
signing or content-addressing scheme needs its own canonical-byte rule; choosing JSON does not
settle that rule. Native documents and binary Resource payloads need not become JSON/YAML merely
because configuration and protocol metadata use this model.

## Profile details still required

These are parts of making the chosen foundation precise, not reasons to design a new type language:

- Specify supported authoring-schema features, closure/composition rules and diagnostics. Keep
  any authoring restrictions distinct from the obligations of the selected MCP version; never
  silently reinterpret an unsupported upstream schema or claim full conformance from a subset.
- Refine reference syntax and resolver details within the accepted supplied/declared dependency
  boundary. A schema `$ref` is not a Resource connection, a grant, or permission to fetch arbitrary
  URLs. MCP itself disables automatic network dereferencing by default.
- Specify default materialization separately. JSON Schema `default` is an annotation, and
  `readOnly` does not enforce authority. The [validation specification](https://json-schema.org/draft/2020-12/json-schema-validation)
  also distinguishes format annotation from format assertion. Conduit's subsequent ruling
  chooses enforced standard formats for its authored schemas, deferring plugin-defined format
  validators in v0. All 19 standard formats are selected. Required Conduit semantics must not depend on a stock
  validator recognizing invented keywords or on its default format-checking settings.
- Define portable number limits and unit/identifier encodings. Unsupported precision must fail
  rather than change a value; schema `integer` describes a mathematical value, not a particular
  YAML or JSON spelling.
- Test the chosen parser and validator against positive and rejection fixtures, including
  composition, defaults, references and cross-client interpretation. Preserve useful source
  locations for author diagnostics.

The existing [20 probes](../../../experiments/003-language-probes/README.md) already demonstrate
several of these traps. They remain Python library observations, not evidence that a Rust parser,
Conduit runtime or arbitrary client conforms. No additional runtime implementation was tested in
this comparison. Sources were rechecked through official web documentation; this brief does not
add response hashes to the earlier retrieval manifest.

## Standard-format set follow-up

Rechecked **2026-11-03**. **Accepted:** support all 19 formats defined in
[JSON Schema 2020-12 section 7.3](https://json-schema.org/draft/2020-12/json-schema-validation#section-7.3):

| Area | Formats |
| --- | --- |
| Time | `date-time`, `date`, `time`, `duration` |
| Email and hostnames | `email`, `idn-email`, `hostname`, `idn-hostname` |
| IP addresses | `ipv4`, `ipv6` |
| Identifiers/references | `uri`, `uri-reference`, `iri`, `iri-reference`, `uuid` |
| Templates, pointers and expressions | `uri-template`, `json-pointer`, `relative-json-pointer`, `regex` |

The standard [Format-Assertion requirements](https://json-schema.org/draft/2020-12/json-schema-validation#section-7.2.2)
cover the full set and require syntactic checks; they do not establish endpoint reachability or entity
existence. The accepted set uses offline checks against the referenced standards, with conformance
fixtures before claiming support. Supporting a format does not choose Conduit's object-ID encoding
or graph-reference syntax.

The candidate Rust [jsonschema documentation](https://docs.rs/jsonschema/latest/jsonschema/#note-on-format-keyword)
documents enabling format checks and an `idna` feature for internationalized email/hostnames.
These are implementation leads, not evidence that Conduit implements the accepted set. No new
runtime or validator tests were executed in this follow-up; library selection remains separate.

## Number-portability follow-up

Rechecked **2026-11-03**. **Accepted:** v0 Conduit-declared integer configuration,
inputs and outputs use the inclusive range −9,007,199,254,740,991 to 9,007,199,254,740,991.
[RFC 8259 section 6](https://www.rfc-editor.org/rfc/rfc8259.html#section-6) identifies this range as
exactly interoperable among implementations using binary64 numbers. Wider JSON integers can exceed
an ordinary JavaScript client's exact representation even when a Rust implementation preserves them.

A contract needing a larger exact value can explicitly declare a decimal string and its constraints;
no implicit numeric-to-string conversion is allowed. This decision does not settle fractional-number
semantics, change provider-native schemas or choose a Rust storage type. The existing rule against
silent precision loss already applies; no new runtime tests were run here.

## Fractional-number follow-up

Rechecked **2026-11-03**. **Pending recommendation:** use finite IEEE 754 binary64 semantics for
ordinary fractional values, with an explicitly declared decimal string or scaled integer when the
domain needs exact decimals. For example, temperature `0.7` would have the ordinary double-precision
approximation; a duration could instead declare integer milliseconds when that unit is appropriate.
This would explicitly allow normal fractional representational rounding, refining the current
precision rule without changing exact integer limits or authorizing silent overflow/underflow.

[I-JSON section 2.2](https://www.rfc-editor.org/rfc/rfc7493.html#section-2.2) uses binary64 range and
precision as its interoperability boundary and recommends strings for exact values beyond it.
This informs the recommendation, but does not select I-JSON wholesale. The alternative is exact
decimal numeric values with precision-preserving parsers, serializers and client representations.
Schema comparison/`multipleOf` behavior and round-trip fixtures still need specification and testing;
choosing binary64 alone would not justify a validator's approximate constraint checks. Provider
schemas remain separate. No new parser/runtime tests were executed for this proposal.
