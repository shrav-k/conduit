# Converting what the older text accepted and the specification does not state

Decision DA3 of the [docs audit](README.md). The older text under `docs/protocol/` and the
proposal records accepted nineteen behaviors that `spec/` does not state. The owner ruled that
findings are resolved without a ruling on each (plan-audit ruling 37), so each is converted
below into a promise, a field or an item of the implementation-defined list. The same pass
settles the points that the editors who applied ruling 37 left open.

Every row is a choice made by the coordinator from text that was already accepted. The owner
can strike any of them.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

GP3 was settled by [ruling 41](../ruling-41/README.md), section 4 of the
[principles review](../principles-review/README.md#4-secrets-reduced), and is written in
[secrets.yaml](../../../spec/promises/secrets.yaml). The rows below keep their original wording.

## The nineteen behaviors

| ID | Behavior the older text accepted | How the specification states it | Layer |
| --- | --- | --- | --- |
| GP1 | A requirement composed of others forwards its own inputs to its constituents. | A constituent's `arguments` are value bindings whose only source is `input`, naming an input of the composition. | language |
| GP2 | No cached or offline permission. | An access decision is made anew for each request, from the Conduits and policies as they stand. No earlier decision is reused. When their state cannot be established the request is not dispatched and reports unavailable. | convention |
| GP3 | Secrets never appear in configuration, records or events, and a credential is never handed to a caller. | Not converted in this pass. It needs a way to mark a value as secret, which is the owner's decision. Listed in the plan audit. | — |
| GP4 | A type states whether its implementation may restore its own runtime, and what content its records keep. | A Type contract may hold `restoresRuntimes: true`, and `records`, with `inputs` and `outputs`, each true or false and false when left out. | language |
| GP5 | A declared default that fails its own schema is rejected. | Rejected when the schema's package is installed or its template registered. | language |
| GP6 | Fractional numbers are finite binary64 values. | A number that is not an integer is read as an IEEE 754 binary64 value; one outside its range is rejected. | language |
| GP7 | Defaults are filled in for an operation's omitted inputs before it is invoked, the same for code and for a graph. | An Operation's input may state `default`. The engine fills it in before dispatch. A control applies none, as REV-006 states. | convention |
| GP8 | A step's arguments are checked before the step is dispatched. | A step whose resolved arguments do not satisfy its operation's inputs is not dispatched and reports invalid. | convention |
| GP9 | A value reference whose pointer selects nothing is an error. | The step is not dispatched and reports invalid. No null and no default stands in. | convention |
| GP10 | Configuration read by a graph is captured when the graph begins. | A graph reads the configuration as it stands when its invocation is dispatched; a change made while it runs does not reach it. | convention |
| GP11 | When a graph's outputs become available. | Item DL1 of the implementation-defined list gains it. | — |
| GP12 | Values bound to a location, such as an absolute path, are interpreted only where the instance is placed. | Such a value is interpreted only by the instance's implementation at its placement; the engine never interprets it elsewhere. | convention |
| GP13 | The caller is identified separately from anything a provider needs, and nothing a caller presents is passed on. | What a consumer presents to identify itself is never sent to an implementation. | interface |
| GP14 | Conduit's context is not forwarded to a provider outside. | An implementation that calls a service outside never forwards Conduit's keys to it. | interface |
| GP15 | A requirement states which end it obliges, and an open Conduit shows which implementation applies it. | A Requirement contract may hold `obliges`, a list drawn from `consumer` and `target`. Inspecting an open Conduit names, for each requirement, the implementation that applies it. | language, convention |
| GP16 | A conformance claim by something other than an engine. | `spec/README.md` gains a third level, Implementation: every `interface` promise that binds an implementation. | — |
| GP17 | Whether a client that is not an instance can use a tool. | A request that the engine cannot establish as coming from an instance is not dispatched and reports refused. | interface |
| GP18 | What a failed setup leaves behind. | A setup that does not succeed closes no Conduit it opened and retires nothing it set up. | convention |
| GP19 | Cancelling an invocation that is in progress. | A new item of the implementation-defined list, with one invariant: a cancelled invocation's outcome is unknown unless its implementation confirms that nothing was done. | convention |

## Points the editors left open

| ID | Point | Settlement |
| --- | --- | --- |
| GQ1 | The rule on undefined keys does not exempt an input's `default`, which the schemas leave open. | A `default` is a literal value, and the rule says so. |
| GQ2 | On the wire an operation and a requirement are named by qualified reference, which a contract of an unpublished project does not have. | Such a contract is named `#name`, and the request also carries the unpublished package's digest under a new key, `package`. |
| GQ3 | The digest of an unpublished project orders documents by kind and name without saying how. | By the bytes of the UTF-8 strings. |
| GQ4 | A composite inside a composite can make component names longer than 63 characters even when nothing is generated. | Such a Conduit is not instantiated. |
| GQ5 | A template with no `config`. | It is treated as present and empty for the type's schema, for defaults and for an input's destination. |
| GQ6 | A `default` reached only through a `$ref` inside a combiner. | Not applied. |
| GQ7 | Retrieving a result after its target is unregistered. | It can no longer be retrieved through the tool. The records of the invocation remain. |
| GQ8 | Whether the engine's own refusal names the restriction. | Yes, under the same key an implementation uses. |
| GQ9 | "Isolation plugin" is used in the definition of enforced and defined nowhere. | Defined as an implementation that confines a consumer so that it reaches a target only through the engine. None is specified in this version, so every restriction is reported as mediated. |
| GQ10 | The shape of a control's `settings` output. | An object keyed by the pointers of the control's `changes`, holding the value now stored at each place confirmed. A control declares the input `revision` and the output `settings`. |
| GQ11 | `#name` and implementations. | `#name` refers to a document or an implementation of the same project or package, and no implementation shares a name with a document there. |
| GQ12 | `spec/README.md` still says that accepted behavior lives in `docs/protocol/`. | It says that the specification alone binds (decision DA1). |
| GQ13 | The three promises for a rejected installation, where one was asked for. | Kept as three, since a promise names one reason. |

## Left for the owner

- GP3, how a value is marked as secret.
- Six behaviors that the older text accepted and that appear to have been dropped: a composite's
  result contract derived by default; a combined create-and-start; per-request required
  features; replaceable package dependencies; exporting an instance; and the check of an
  expected ID on an update. They are listed in the plan audit.

Since settled: ruling 38, point 2, dropped the six behaviors ([record](../ruling-38/README.md)),
and ruling 41 settled GP3, written in [secrets.yaml](../../../spec/promises/secrets.yaml).
