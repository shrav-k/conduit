# Instantiation arguments — accepted direction

Status: owner rulings of 2026-11-05 for
[plan-audit item A36](../../AUDIT.md#p0--make-the-language-concrete-and-complete) (question Q12).
The decisions below are accepted. Their spellings were illustrative when this was written, under
a [vocabulary boundary](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#vocabulary-review-boundary) that ruling 16 has
since ended. They are now written as numbered promises in the specification:
[instantiation](../../../spec/promises/instantiation.yaml), INS-001 to INS-023, less those the
comment at the head of that file says were folded elsewhere. Nothing here has been executed.

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- rulings 18 and 19: one kind per family, and the name alone where a field accepts one kind
  (already written into IN8 to IN10 below);
- rulings 28 and 29, the [consolidation rulings](../consolidation/README.md#rulings): an
  instance writes its values under `arguments`, not `inputs`; a nested instantiation has the
  shape of an instance document; and the input's default is applied first, then the defaults of
  the type's schema;
- ruling 31: a template's `spec` also holds its `implementation` map, `runtimes`, an optional
  `retire` graph and `controls`, none of which an input may fill;
- ruling 32: an "ordinary schema reference" to a built-in schema is a reference to one of the
  core package's reference schemas, and a value is a reference only where such a schema governs
  its place;
- ruling 33: the core package's address comes from the repository, so the schema in the example
  below is now `github.com/shrav-k/conduit/core#resource-reference.v1`;
- ruling 37: changed INS-006, INS-012 and INS-014.

## What this replaces

Until now, creating an instance meant copying its template into a draft, editing "ordinary typed
fields" and validating the result. Named template parameters were explicitly excluded. Values
were passed three different ways: draft edits, the configuration inside a connection
declaration's `open` block, and literals that only the Pool composition operation interpreted.
No template stated what its instantiator must or may supply.

## Accepted decisions

| ID | Decision |
| --- | --- |
| IN1 | A template has a closed, declared list of inputs. An instantiator can supply those and nothing else. |
| IN2 | The structural parts of a template are never inputs: its setup graph, runtimes, controls and implementation selection. Changing them makes a different template. |
| IN3 | One instantiation shape is used everywhere something creates an instance: direct creation, a Pool's composition entries and a connection declaration's `open` block. The Pool composition literals are retired. |
| IN4 | Each input is declared explicitly with its own schema. There is no shorthand that infers an input's type from the field it lands on. An author who wants to reuse a type contract's definition references it with an ordinary schema reference. |
| IN5 | Each input names its destination in the instance, and may name several. Supplying an input assigns the whole validated value to each destination. There is no string interpolation, expression or merge. |
| IN6 | The instance stores only the materialized result. Inputs are not retained as a second configuration map or an override layer. |
| IN7 | Name, owner and placement are standard arguments defined once for the family. A template does not redeclare them. |
| IN8 | There is one kind per family. A document that declares inputs and has no `from` is a template. A document with `from` is an instance of that template. The word "form" and its field are dropped (vocabulary decision V2, ruling 18). |
| IN9 | Every instance comes from a template. A complete instance cannot be created directly; a one-off is a template with no inputs, instantiated once. |
| IN10 | A reference is a kind and a name. The field it sits in decides whether a template or an instance is expected, and the wrong one is rejected. `from` on an instance records its origin and is not a live link. Where a field accepts exactly one kind, the name alone may be written (ruling 19). |

Instantiation is therefore a pure function from a template and its arguments to a complete
instance definition, and an ordinary typed operation. The accepted rules for operation inputs
apply unchanged: declared defaults are materialized, the prepared input is validated before any
effect, and binding forms apply when the operation is called from a graph. The materialized
instance must still validate against its type contract, so a template can narrow what its type
allows and can never widen it.

IN7 settles the open placement decision
([PL2](../placement/README.md#pl2--where-placement-is-declared)): placement is a standard
argument, a Pool entry passes it down through this same shape, and a template never names a
concrete location.

## Worked example

The `apiVersion` line is omitted for brevity.

> Later rulings changed two shapes below. The instance's values are now written under
> `arguments`, not `inputs` (rulings 28 and 29; OBJ-013). `conduit:reference#/$defs/resource` is
> now `github.com/shrav-k/conduit/core#resource-reference.v1` (rulings 28 and 33; the names are
> in [`spec/names.yaml`](../../../spec/names.yaml)). A template also selects a `type` (TPL-002),
> which this example leaves out. The current worked example is
> [`spec/schemas/examples/valid/`](../../../spec/schemas/examples/valid/).

```yaml
kind: Agent
metadata: {name: collaborator}
spec:
  inputs:
    account:
      schema: {type: string, minLength: 1}
      to: [/config/account]
    model:
      schema: {type: string}
      default: fixture-small
      to: [/config/model]
    checkout:
      schema: {$ref: "conduit:reference#/$defs/resource"}
      to: [/conduits/checkout/target]
  config:
    providerDomain: fixture-models
    execution: native
  conduits:
    checkout: {from: capability-route}
```

```yaml
kind: Agent
metadata: {name: collaborator-0001}
from: collaborator
placement: laptop
inputs:
  account: agent-account-a
  checkout: {kind: Resource, name: project-directory-0001}
```

The first document declares inputs and has no `from`, so it is a template. The second has
`from`, so it is an instance of that template. The result is a complete Agent instance whose
configuration holds `account`, the defaulted `model`, the template's fixed `providerDomain` and
`execution`, and a Conduit whose target is bound. A second instantiation with different
arguments reuses the same registered template.

## Known costs

- More lines per template: each exposed value needs a name, a schema and a destination.
- A closed list means an unexposed field needs a new template. Without inheritance, copies can
  drift.
- An input's schema and a later control's schema are declared separately and can disagree. The
  type contract's validation catches a value outside what the type allows; it does not catch an
  input that is stricter or looser than the control.
- A reader follows one indirection from an input to where it lands.

## Still to specify

> Later rulings: the first two are specified. INS-010 to INS-012 and INS-021 say which
> destinations are legal, and an input reaches an entry by naming the entry's `placement` or its
> `arguments` as a destination (INS-012, POOL-009). The third will not happen: Experiment 001
> was removed, and `spec/schemas/examples/valid/` is the worked example
> ([docs audit](../docs-audit/README.md), decision DA4).

- The exact rule checked when a template is registered: which destinations are legal, and which
  mismatches between an input's schema and its destination are provable early.
- How a composition entry's arguments refer to the enclosing template's own inputs.
- The rewrite of the Experiment 001 fixtures, whose templates name concrete instances and use
  the dropped `form` field.
