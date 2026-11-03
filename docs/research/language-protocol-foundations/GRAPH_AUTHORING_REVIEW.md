# Graph authoring: comparison and decision audit

Researched 2026-11-03, following the request to step back from individual syntax rulings.
This is evidence and recommendations, not an amendment to accepted contracts. The direct versus
nested `reference` payload is still unaccepted. Previously accepted choices remain in force unless
the owner revises them. See the [current review](../../REVIEW.md) and
[lifecycle contract](../../protocol/LIFECYCLE.md#lifecycle-protocol).

Scope: the recent configuration, binding, graph and recovery decisions. This is not a fresh audit
of every account, authorization, distribution or storage decision. Official specifications and
project documentation were inspected; no cluster, workflow engine, usability study or performance
benchmark was run. Moving documentation is identified by retrieval date, not presented as a pinned
implementation. Recommendations below are Conduit-specific judgments, not conclusions proved by
the source systems.

## Assessment

The foundation is defensible. The larger risk is accumulating individually simple restrictions
until writing a useful template requires too much ceremony. The reference's extra nesting level
is a relatively small choice. Expression limits, graph granularity, failure behavior and the
boundary between data and instructions have larger effects on authors and implementations.

Separate three questions during review:

1. **Meaning:** which value is selected, what must happen first, and what happens after failure?
2. **Representation:** which YAML/JSON structure expresses that meaning?
3. **Author experience:** how much must a person or Agent write, and can they understand errors?

A compact representation is not automatically ambiguous. A more explicit representation is not
automatically easier to author. Strictness prevents some mistakes but can create maintenance work;
each restriction should have a concrete benefit and an example demonstrating its cost.

## What the comparison systems actually establish

| System / primary source | Observed design | Useful lesson and boundary |
| --- | --- | --- |
| [Kubernetes Pod API: EnvVar / EnvVarSource](https://kubernetes.io/docs/reference/kubernetes-api/core/pod-v1/) | An environment variable can use `value` or a structured `valueFrom` source such as `configMapKeyRef`. | Structured sources are established practice. This is a specialized API, not a generic graph-value language or a mandate for our exact nesting. |
| [Kubernetes controllers](https://kubernetes.io/docs/concepts/architecture/controller/) | Controllers move observed state toward declared desired state. | Useful for instance supervision. Conduit's explicitly requested setup/control graphs have a different execution contract; copying manifest shape does not select reconciliation semantics. |
| [Kubernetes API validation](https://kubernetes.io/docs/reference/using-api/api-concepts/#validation-for-unrecognized-or-duplicate-fields) | Strict validation rejects unknown/duplicate fields. The API also offers other modes; kubectl defaults to strict validation. | Our strict acceptance rule has a precedent. It should be described as our profile, not as the only Kubernetes behavior. |
| [CWL Workflow v1.2.1](https://www.commonwl.org/v1.2/Workflow) | Typed process inputs/outputs, source links, DAG execution, concurrent independent steps and reusable workflows. Optional expression features and default/null handling add flexibility. | Strong precedent for a portable dataflow contract. Its transformation and missing-value semantics differ from ours; adopting its topology does not require adopting those policies. |
| [Tekton Pipelines](https://tekton.dev/docs/pipelines/pipelines/) | Result references establish dependencies; `runAfter` expresses ordering without data. Result use commonly uses substitution strings. A consumed but unproduced result fails. | Supports our distinction between value dependencies and explicit ordering, and our refusal to pretend a missing result exists. Compact references are nevertheless a viable design in a mature system. |
| [Argo DAGs](https://argo-workflows.readthedocs.io/en/latest/walk-through/dag/) | Dependencies enable parallel branches. The default stops scheduling new tasks after failure; `failFast: false` allows branches to continue. | Continuing independent work is a policy choice, not an inherent property of DAGs. Our selected default needs examples involving both useful partial setup and unwanted extra effects. |
| [Compose interpolation](https://docs.docker.com/reference/compose-file/interpolation/) | String substitution has explicit default/required forms and escaping. An unresolved variable without a default warns and becomes an empty string. | Convenience creates another interpretation layer. Conduit's literal boundary and missing-reference errors deliberately avoid this behavior. |
| [Compose startup order](https://docs.docker.com/compose/how-tos/startup-order/) | Started, healthy and successfully completed dependencies are distinct. | A process existing is not proof a setup dependency is ready. Preserve our instance-managed runtime/readiness distinction. |
| [AWS Step Functions parameters](https://docs.aws.amazon.com/step-functions/latest/dg/connect-parameters.html) and [JSONata](https://docs.aws.amazon.com/step-functions/latest/dg/transforming-data.html) | JSONPath payload templates mark selected values with `.$`; JSONata uses delimited expressions in supported fields. | Existing systems support literal/dynamic mixtures with less wrapping. They also specify parsing, evaluation, errors and transformation semantics that our v0 deliberately avoids. |
| [Serde enum representations](https://serde.rs/enum-representations.html) | Both an outer variant key and an internal discriminator are supported representations. | Our one-form-key encoding is a familiar tagged-union pattern. Rust convenience is supporting evidence, not authority over the language-neutral protocol. |
| [JSON Pointer, RFC 6901](https://www.rfc-editor.org/rfc/rfc6901.html) | A standard path selects a particular nested value; applications define the consequences of failed selection. | Good fit for our field selection. The RFC does not specify graph sources, dependencies, authorization or our missing-value policy. |
| [Temporal activity execution](https://docs.temporal.io/activity-execution) | Activity execution tracks attempts/timeouts; cancellation is delivered to activity code, which may accept or ignore it. | Preserve distinctions between requests, observed outcomes and external effects. A lifecycle mechanism alone does not prove an external write was undone. |

No inspected system provides a universal canonical syntax for Conduit's combined object,
graph, authorization and plugin model. Kubernetes supplies useful API patterns; CWL, Tekton and
workflow engines are closer comparisons for graph inputs and outputs.

## The pending flat-versus-nested ruling

Suppose `connect` publishes a declared output `connection` whose value has an `endpoint` member.
These are equivalent candidate references; neither payload layout is accepted:

```yaml
# A: direct fields
reference:
  from: output
  node: connect
  port: connection
  path: /endpoint
```

```yaml
# B: grouped source selector
reference:
  source:
    from: output
    node: connect
    port: connection
  path: /endpoint
```

The accepted `reference` form already creates a boundary around the binding. B adds one more
boundary inside it. Neither layout changes the dependency, selected value, grant or failure behavior.

| Criterion | A: direct fields | B: source object |
| --- | --- | --- |
| Writing and scanning common cases | Fewer levels; especially small for configuration/input references | More indentation and one repeated key |
| Grouping source identity separately from selection | Expressed by the schema and field meanings | Visible directly in the document |
| Adding more source variants | Conditional rules distinguish applicable fields | Source-specific rules can be encapsulated in the nested object |
| Rejecting invalid field combinations | Possible with a discriminated schema | Equally possible with a discriminated source schema |
| Reusing a source selector as a value | Would need an explicit reuse mechanism | Has a convenient boundary, but nesting alone creates no reuse feature |
| Runtime cost / security | No measured advantage; same required checks | No measured advantage; same required checks |

Both can be expressed with ordinary schema alternatives. JSON Schema's `oneOf` describes exactly
one matching alternative; meaningful required/closed-field rules are still needed. The presence
or absence of a `source` object does not itself determine validation quality.
[JSON Schema combination](https://json-schema.org/understanding-json-schema/reference/combining)

**Recommendation: retain a modest preference for A**, because our current sources are small and
we have no accepted independently reusable source-selector object. B is reasonable if worked
templates expose a real need for that grouping. Kubernetes' nested source types do not prove B
is necessary here. Do not approve A merely because earlier examples already use it.

The names deserve a separate readability check: `node` means a graph step; `port` means a declared
output, not a host or TCP port. `step`/`output` might be easier for new authors. This is a vocabulary
observation, not a recommendation to restart the object-family naming discussion.

## Review of the accepted choices

The judgments in this table are recommendations for discussion. None changes accepted status.

| Accepted choice | Benefit | Cost or limitation | Assessment |
| --- | --- | --- | --- |
| YAML/JSON data with typed contracts | Familiar tooling and a language-neutral boundary | YAML alone supplies no lifecycle, ordering or recovery behavior | Keep; be precise about the semantics Conduit adds. |
| Strict fields, duplicate rejection and no implicit conversions | Catches misspellings and contradictory inputs | More rejections; authors need field locations and useful diagnostics | Keep for v0, with good errors rather than silent repair. |
| Constrained YAML without anchors or implicit environment expansion | Fewer hidden sources and expansion rules | Less file-local shorthand | Reasonable v0 scope; show reuse through actual packages/templates. |
| Explicit literal/reference/object/list forms | Unambiguous parsing, predictable nesting and inspectable dataflow | More writing for small values and mixed payloads | Keep as the current canonical contract; test authoring effort before treating it as ergonomic proof. |
| One form key with payload | Direct relationship between form and payload; little redundant tagging | Generic tooling must understand the union; errors must identify the chosen form | Sound representation. No research finding requires reopening it. |
| Structured reference source plus JSON Pointer | Separates source identity from nested data selection; no new query language | Longer than a compact reference string; pointer escaping is unfamiliar to some authors | Keep. Teach with concrete values rather than terminology alone. |
| Missing explicit references fail; null remains a value | Broken wiring cannot silently choose another input | Optional data needs deliberate handling | Keep. An optional-selection feature, if needed later, should be explicit. |
| Finite DAG; output references imply dependencies; ordering edges available | Reusable composition and parallelism with visible prerequisites | Authors must identify dependencies that have no value edge | Keep; the two dependency mechanisms have direct workflow precedents. |
| Calculations/transformations implemented by operations | Small graph evaluator; ordinary plugin contracts cover custom behavior | Trivial string/path shaping may require many steps or opaque helper operations | Highest authoring-friction concern. Measure complete examples before adding expressions or defending the restriction indefinitely. |
| Capture configuration for an execution | Avoids silently mixing settings mid-run | Captured values can differ from current settings | Keep with visible captured/current views and existing live-authority checks. |
| Early provable checks plus actual input/output validation | Handles dynamic data without demanding a general schema proof | Some errors appear only during execution, possibly after effects | Keep; say what was checked, and retain effects when later validation fails. |
| Independent branches continue after failure | Useful partial setup survives unrelated outages | Work may continue spending resources after an overall failure | Keep the accepted default for now; test whether explicit prerequisites/gates express intended stopping. Do not call it an industry consensus. |
| Progress-aware retry, effect evidence and no automatic rollback claim | Honest recovery around external systems | More demanding plugin contracts and inspection | Keep. Recovery quality is more consequential than reference punctuation. |
| Prevalidated registration DAG, partial writes and one write per target | One execution model with explicit order and outcomes | A simple multi-file registration may feel too elaborate | Keep semantics; a frontend can construct trivial graphs without changing them. Test this separately from provisioning multiple instances. |

Strict defaults and validation also need a clear standards boundary. JSON Schema's `default`
annotation does not fill missing fields during validation; Conduit owns that materialization step.
Likewise, format assertion is an explicit vocabulary/support obligation, rather than a universal
validator default. Our decisions to enforce standard formats and report unsupported validation
remain accepted, but they carry implementation/conformance cost. The earlier
[format and numeric study](FORMATS.md) remains relevant; this pass does not rerun its probes.
[JSON Schema annotations](https://json-schema.org/understanding-json-schema/reference/annotations) ·
[2020-12 format vocabularies](https://json-schema.org/draft/2020-12/json-schema-validation#section-7.2)

## Three concrete pressure tests

These are authoring walkthroughs, not implemented experiments or new capability requirements.
Keep the currently accepted rules as the baseline and record any proposed revisions separately.

### 1. Local directory with a layout check

The setup attaches an explicitly configured directory, then checks for `Cargo.toml` and `src`.
The directory is passed by reference; the required filenames can be one literal list:

```yaml
# Fragment; surrounding graph fields and reference payload fields remain proposals.
inputs:
  directory:
    reference: {from: output, node: attach, port: directory, path: ""}
  requiredEntries:
    literal: [Cargo.toml, src]
```

This is a favorable case for the accepted forms: the static list needs one wrapper, not a wrapper
per filename. The remaining usability questions are whether the error identifies `attach` and
`directory` clearly, and whether authors understand the empty pointer as the whole selected value.
This example supplies the pointer explicitly and does not settle omitted-path behavior.

### 2. Authenticated MCP setup

An authorized authentication operation supplies a protected credential binding to a connection
operation. The connection operation exposes an endpoint descriptor. The sensitive handle is not
a token pasted into ordinary YAML, and wiring it does not create a grant.

```text
obtain authorized authentication binding -> establish connection -> use endpoint
```

Consider three distinct conditions:

- Authentication is waiting for a person: dependent setup has no usable prerequisite yet.
- Connection setup returns a valid result omitting an optional endpoint: explicitly selecting
  that endpoint fails under our accepted rule.
- The connection later drops: the instance's runtime/reconnection contract applies; a successful
  earlier pointer lookup does not prove present readiness.

The important contract is which of these occurred and what action can recover it. Flat versus
nested reference fields has little effect on that distinction. Both remain subject to authorized
Conduit access and the selected plugin's actual guarantees.

Optional data does not require silent fallback. An operation can receive the whole optional-data
object, validate it and implement a deliberate policy. This works with current rules but can become
too much helper code if it appears in every template. That frequency should inform future scope.

### 3. Database setup with independent work

```text
start managed database -> migrate -> dependent application preparation
prepare workspace
```

With our current default, workspace preparation can finish after migration fails. This is useful
if the workspace remains valuable for inspection or repair. It is surprising if the author intended
all work to stop after migration failure. In that case, a real dependency/gate must be expressed;
changing mapping order cannot impose it.

Now add an uncertain migration response, a database still running, and a captured query-limit
setting. The Interface must expose retained effects and unresolved outcomes; an explicit retry
reconciles the migration before repeating it. This case tests our protocol much more effectively
than approving another field name in isolation.

## What to keep open, and how to continue

Keep the reference layout open. Do not revise accepted forms or failure behavior by inference from
this research. My recommended next discussion is a complete pass through the three cases above,
with five questions for each:

1. Can an author explain each value's source without understanding the executor implementation?
2. Does the template need an operation solely for trivial data shaping? How many, and why?
3. Which configuration is ordinary data, which binding is evaluated, and what reaches the plugin?
4. When something fails or becomes unknown, what continues, what is blocked and what can be retried?
5. Could an independent implementation reach the same answer from the public contract?

Once those answers are clear, settle the remaining reference fields as one coherent example.
If an accepted choice needs revision, identify that choice and its concrete failure case rather
than silently changing the docs. Keep accepted protocol, candidate syntax and implemented support
distinct. Neither early acceptance nor the lack of a compatibility promise makes changes costless,
but neither should prevent an evidence-backed revision before implementation.
