# Open questions after the rewrite

Date: 2026-11-06. This is the record of the questions that the rewrite under
[ruling 40](../ruling-40/README.md) and [ruling 41](README.md) left open: the tables
[Still needs a decision](log-integration.md#still-needs-a-decision) (D1 to D20) and
[Needs a decision](log-integration.md#needs-a-decision) (R1 to R28) of the integration log, the
rows of its coverage check that list a decision, and the editors' choices beyond the plan that it
keeps for the coordinator. Six analysts answered the questions, a skeptic challenged each answer,
and the coordinator reconciled the two, read the promises where they disagreed, and checked the
answers against each other. The coordinator settles the items marked coordinator under rulings 37
and 38 ([who decides what](../../AUDIT.md#who-decides-what)): each follows from a ruling, a
decision of the [principles review](../principles-review/README.md), a planner's choice that
the owner accepted, or the behavior of a standard that the rulings adopted, and none changes
what Conduit is for. The owner rules on the items marked owner, under
[For the owner](#for-the-owner). Nothing here is applied until an editor applies it, and the
owner can strike any coordinator decision.

How to read it. The numbers D1 to D20 and R1 to R28 are the integration log's. An item marked
"part" is settled in one part here, and the rest is an owner item. R1 was settled by the
coverage check (row 10). Rows 23 and 44 of the coverage check are entries for the tracker, not
decisions. New names, such as the member `serving`, the status member `unreconciled` and the
promise OPS-034, are working names for the vocabulary audit that ruling 41 orders.

The promise files: [placement](../../../spec/promises/placement.yaml),
[providers](../../../spec/promises/providers.yaml), [pools](../../../spec/promises/pools.yaml),
[secrets](../../../spec/promises/secrets.yaml),
[templates](../../../spec/promises/templates.yaml),
[conduits](../../../spec/promises/conduits.yaml),
[manifest](../../../spec/promises/manifest.yaml),
[packages](../../../spec/promises/packages.yaml),
[submissions](../../../spec/promises/submissions.yaml),
[contracts](../../../spec/promises/contracts.yaml),
[first start](../../../spec/promises/first-start.yaml),
[access](../../../spec/conventions/access.yaml),
[lifecycle](../../../spec/conventions/lifecycle.yaml),
[operations](../../../spec/conventions/operations.yaml),
[records](../../../spec/conventions/records.yaml),
[revisions](../../../spec/conventions/revisions.yaml),
[implicit](../../../spec/conventions/implicit.yaml),
[implementation](../../../spec/interface/implementation.yaml), the
[implementation-defined list](../../../spec/implementation-defined.yaml) and the core package's
[operations](../../../spec/core/operations.yaml) and [results](../../../spec/core/results.yaml).

## Settled by the coordinator

| # | Question | Decision | Why | Edits (promise IDs) |
| --- | --- | --- | --- | --- |
| 1 | D1. An instance that names no `runtime` runs on the engine's own runner, which runs only on the engine's machine, but the machines provider may place it on another machine, where every setup fails | For an instance that has an item that is run and names no `runtime`, the label selector of PLC-007 also holds `kubernetes.io/hostname` with the value in `local`'s status, as a RuntimeClass's `scheduling.nodeSelector` is merged into a Pod's. With no placement given, `place.v1` gets that selector and names the engine's machine or none. A placement given elsewhere is rejected when the document is resolved, and the diagnostic names the engine's own runner as the source of the label. An instance with no item that is run, such as one reached only by `remote`, is placed as now | Decision 8a merges the selectors as Kubernetes merges a RuntimeClass's with a Pod's, and 8c has the provider place every document; Kubernetes then schedules the merged Pod and skips no step ([RuntimeClass scheduling](https://kubernetes.io/docs/concepts/containers/runtime-class/#scheduling), [kube-scheduler](https://kubernetes.io/docs/concepts/scheduling-eviction/kube-scheduler/)). Placing such instances on `local` without asking the provider would extend P6 beyond providers, and is the owner's to choose. Cost, accepted: with no placement given, such an instance waits on the machines provider (row 28) and on its report of the engine's machine (row 13); `placement: local` always passes. Settled with rows 13, 14 and 28 | [PLC-007](../../../spec/promises/placement.yaml), [PRV-015, PRV-006, PRV-003, PRV-019](../../../spec/promises/providers.yaml), [POOL-009](../../../spec/promises/pools.yaml); definitions; `docs/OVERVIEW.md` |
| 2 | D2. May a runtime provider be retired while an instance names it? | Confirm PRV-020: the retirement reports conflict while any instance that names the provider exists. PRV-008's "there is always one to ask" holds only while the instance exists; a run left behind once it is removed is the owner's D11 | Kubernetes postpones removing a claim while a Pod that uses it exists ([storage object in use protection](https://kubernetes.io/docs/concepts/storage/persistent-volumes/#storage-object-in-use-protection)), and PLC-011 does the same for a location, answering conflict. Retiring a provider stops none of its runs (inference), so the other option would leave runs and their secrets that nothing can stop. A forced removal would be a new feature | [PRV-020, PRV-008](../../../spec/promises/providers.yaml) |
| 3 | D3 (R4). Does the stop wait for the retire hook to succeed? | No: TPL-009 stands. The stop is asked for whatever came of the hook. A hook that did not succeed keeps the shutdown pending, and a further retirement dispatches it again against a new run, so it must not depend on the run that was stopped. A hook that hangs is bounded by the time that DL10 allows a call | Decision 1a (the hook before the stop), P4 (a retirement asks for a run only to serve a hook) and OPS-022. Kubernetes kills a container whose PreStop hook failed ([container lifecycle hooks](https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/)); Terraform runs a failed destroy-time provisioner again at the next apply ([provisioners](https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax)) | [TPL-009, TPL-018](../../../spec/promises/templates.yaml), [DL10](../../../spec/implementation-defined.yaml) |
| 4 | D4. On a location declared lost a retire hook can never run. Does it keep the retirement from succeeding, so the instance is removed only by a request to unregister? | Confirm PLC-027. Its observable cites finalizers; says that a machine that comes back gets a new location, so the hook is never dispatched; and says that this holds for a hook on a `remote` item too, since the instance's location decides | Decision 17 keeps removal by request on a location declared lost, and OPS-022 counts the hook. An object with a finalizer stays terminating, and removing one by hand asserts that its work "is accomplished in another way" ([finalizers](https://kubernetes.io/docs/concepts/overview/working-with-objects/finalizers/)); the request to unregister is that assertion | [PLC-027](../../../spec/promises/placement.yaml) |
| 5 | D6 (R2). Refused or invalid for a request through a closed Conduit? | The owner's: [D6](#d6-a-request-through-a-closed-conduit). The coordinator's first reading, that decision 17's cost line covers only the requests that name a closed Conduit, would set aside the words the owner accepted | Decision 17's cost line: "a request through it reports invalid rather than refused" | [ACC-011](../../../spec/conventions/access.yaml) |
| 6 | D7. Is each item of a bound's `operations` a name, compared and never resolved? | Yes: the editor's choice is confirmed. An item covers an operation of a package installed later at that address. The log records that a check made before ruling 41, NAM-007 on a bound's addresses, is dropped, and that keeping it for every address but the project's own was considered | Decision 11 makes `address#name` the name everywhere, and OBJ-021 makes the items data. Kubernetes RBAC checks only a rule's structure when it is written and matches it by name when a request is decided ([validation.go](https://github.com/kubernetes/kubernetes/blob/master/pkg/apis/rbac/validation/validation.go), [rbac.go](https://github.com/kubernetes/kubernetes/blob/master/plugin/pkg/auth/authorizer/rbac/rbac.go)). A typo covers nothing. Settled with row 10 | [ACC-029](../../../spec/conventions/access.yaml) |
| 7 | D8 (R12), part. What does the engine's own runner give a process beyond what the item binds? | For now, a new item of the implementation-defined list, "The engine's own runner": which variables of the engine's environment it passes, the working directory, the user and the login session. The built-in runner is the provider here, so the item is the contract that PRV-011 leaves to a runtime provider. `spec/README.md` says that a variable exported in the engine's environment may reach such a process. Whether the specification fixes a set is the owner's D8 | Section 4 of the review, line 18: "what else a run receives is the provider's (2c, 13b)"; SEC-018 already answers PRV-011 for the built-in runner in the same way | [SEC-020](../../../spec/promises/secrets.yaml), [PRV-003](../../../spec/promises/providers.yaml), new item of the [list](../../../spec/implementation-defined.yaml); `spec/README.md` |
| 8 | D9. May an actor end the run of a ready instance so that a rotated value arrives? | No operation does so in v0. SEC-014 adds that once a run has ended, a further setup asks for another with reads of its own. `spec/README.md` moves rotation from what the text leaves outside to the stated limits, with its cost: through Conduit alone, a rotated value reaches a running instance only in another instance made in its place, which closes every Conduit to it. A restart verb is a candidate feature for the tracker | Section 4 point 5 ("fixed for the life of the run") and decision 4's cost line; OBS-004 makes a further setup the request that repairs a run that ended. Kubernetes and ECS change no running process's environment ([ConfigMap](https://kubernetes.io/docs/concepts/configuration/configmap/), [ECS](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/secrets-envvar-secrets-manager.html)) | [SEC-014](../../../spec/promises/secrets.yaml); `spec/README.md` |
| 9 | D12 (R8), part. A retire hook on a `remote` item with no connection, and what ends a connection | A retirement whose hook is to go to a `remote` item with no connection that lasts connects within the retirement, with reads that belong to it, as TPL-018 asks for a run. A connection lasts until a later setup's connection takes its place or the retirement ends; a failed HTTP request does not end it. Where a later setup's read fails, the connection that lasts is kept, and closing the reading Conduit recalls nothing, as for a run. What happens after the engine restarts is the owner's D12 | Decisions 4c and 25c ("as before a run (decision 4)"), section 4's line 8h, and ACC-019, which already speaks of a retirement that asks for a connection. MCP 2026-07-28 has no session ([Streamable HTTP][shttp]) and includes authorization in every HTTP request ([authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)), so a connection is the values the engine holds. SEC-014 keeps a run's value after its Conduit closes | [SEC-013, SEC-011, SEC-012](../../../spec/promises/secrets.yaml), [TPL-018, TPL-009, TPL-012](../../../spec/promises/templates.yaml), [ACC-039](../../../spec/conventions/access.yaml), [IFC-041](../../../spec/interface/implementation.yaml); definitions |
| 10 | D12 (R27). May two submitted projects state one address? | Yes, as the text allows. Each template resolves its own submission by its digest; `address#name` names the operation in every submission at that address, as it does in every version of a package, and a record does not tell them apart. An address owned by its first submitter is a candidate access feature for the tracker | Decision 11 adopted Go's `module` line, which claims nothing: a fork shares a path through `replace`, and the only error is one workspace with a path twice ([go.mod reference](https://go.dev/ref/mod), [init.go](https://go.googlesource.com/go/+/refs/heads/master/src/cmd/go/internal/modload/init.go)). Registering already needs a grant through `management` (ACC-030, ACC-033). Settled with row 6 | [SUB-008](../../../spec/promises/submissions.yaml), [MAN-015](../../../spec/promises/manifest.yaml), [ACC-029](../../../spec/conventions/access.yaml); definitions |
| 11 | D13 (R24). The engine's own retirements and closings at once while a setup, retirement or change of the instance is in progress | They take effect at once and dispatch nothing, so REV-012 does not hold them back. No part of a setup begins once the instance is retired; parts already dispatched end as they end, the outcome is OPS-022's, and the instance stays retired. A run that such a part started is stopped by a further retirement, or as the owner decides D11 | Decision 8e and CON-008's "closed at once"; decision 23 is about requests. The kubelet starts no new container of a Pod that is terminating ([pod_workers.go](https://github.com/kubernetes/kubernetes/blob/master/pkg/kubelet/pod_workers.go)). Settled with row 33 | [REV-012, REV-014](../../../spec/conventions/revisions.yaml), [TPL-012](../../../spec/promises/templates.yaml), [PLC-017, PLC-024, PLC-026](../../../spec/promises/placement.yaml), [PRV-007](../../../spec/promises/providers.yaml) |
| 12 | D14. `record.v1` has one `invocation`, but the record of a read, a hook, a part or an onward request needs two | A member `serving` in `data` names the invocation that a separate invocation serves. `invocation` holds what the recorded call carried: its own ID, or for a provider call the invocation it was made within, and a provider call's record has no `serving`. The parts of an invocation are the records whose `invocation` or `serving` names it. A record of `run.v1`, `stop.v1` or `inspect-run.v1` names the instance whose run it is in the CloudEvent's `subject`, so inspecting the instance finds it | SEC-011, SEC-015 and PRV-005 already need the two IDs, and IFC-020 names the key `serving`. W3C Trace Context gives each request a `parent-id` ([Trace Context](https://www.w3.org/TR/trace-context/)); the CloudEvents tracing extension carries only the trace a transmission started with ([distributed tracing](https://github.com/cloudevents/spec/blob/main/cloudevents/extensions/distributed-tracing.md)). `subject` "identifies the subject of the event" ([CloudEvents](https://github.com/cloudevents/spec/blob/main/cloudevents/spec.md)). Settled with row 18 | [REC-002](../../../spec/conventions/records.yaml), [SEC-015, SEC-011](../../../spec/promises/secrets.yaml), [PRV-005](../../../spec/promises/providers.yaml), [IFC-020](../../../spec/interface/implementation.yaml), [`record.v1`](../../../spec/core/results.yaml); definitions |
| 13 | D15 (R15). How the engine knows its own machine in a report; a machine that a report omits; a machines provider retired or replaced; retiring the location of a machine still reported | The Kubernetes Node model. A hostname is compared in its canonical form, trimmed and lowercased. The engine's machine is the one whose hostname is the engine's own: the operating system's, unless the engine is configured with another (DL5). An omitted machine counts as reported unreachable until a report gives it a state, and where no report arrives in DL5's time every machine but `local`'s counts as omitted. A location belongs to a hostname, not to a provider, so retiring or replacing the provider retires nothing. An actor may retire the idle location of a reported machine, and the next report registers another. A report names each hostname once, and one that names a hostname twice is not used. The provider's contract states which name it reports | Decision 8d registers a location "as a Kubernetes kubelet registers its own Node", and C58 adopts `kubernetes.io/hostname`. The kubelet canonicalizes its hostname ([hostname.go](https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/component-helpers/node/util/hostname.go)) and takes `--hostname-override` ([kubelet](https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/)); a Node that stops reporting is Unknown, and a Node of the same name is the same object ([nodes](https://kubernetes.io/docs/concepts/architecture/nodes/)). That only gone retires is decision 8e's | [PLC-026, PLC-023](../../../spec/promises/placement.yaml), [PRV-014, PRV-013](../../../spec/promises/providers.yaml), [FST-001](../../../spec/promises/first-start.yaml), [`list-machines.v1`](../../../spec/core/operations.yaml), [DL5](../../../spec/implementation-defined.yaml); definitions |
| 14 | D16 (R14). The layers of PLC-007, PLC-013 and POOL-016 | PLC-007 and PLC-013 stay `language`; POOL-016 becomes `convention`. The row for `language` in `spec/README.md` says that resolving reads the lifecycle state and the status of objects that exist | `spec/README.md` already admits "what already exists", PLC-018 is `language` and reads a lifecycle state, and PLC-002 moved because it calls a provider. Kubernetes admission validates against stored objects ([admission controllers](https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/#namespacelifecycle)) | [POOL-016](../../../spec/promises/pools.yaml); `spec/README.md` |
| 15 | D18 (R20, first half). Does a provider call declare Conduit's extension; what if the provider lacks it; may a provider be plain; how is its result read? | IFC-037 covers a provider call to a provider's implementation item, not the engine's own implementations of PRV-003: the engine reads the discovery result and declares the extension, and where the extension is missing the call is not dispatched and reports unavailable, with the consequences that PRV-007, PRV-008, PKG-024 and row 28 give; its record names the missing extension. The result is read as IFC-009 and OPS-007 read a request's. No plain implementation implements an operation of a provider capability. The engine reads the discovery result before its first request to each run, and to a `remote` item at each connection, and may read it again at any time | Decision 18 and the protocol's rule that a party that supports an extension falls back or rejects ([versioning](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning)); the same page has a client keep what it learned of a server for the server's process on stdio and its origin on HTTP, stated for the protocol's era and applied here by inference. PKG-020 defines plain as a server not written for this specification. The kubelet registers no node whose runtime lacks the v1 CRI API ([CRI](https://kubernetes.io/docs/concepts/architecture/cri/)). Settled with rows 16 and 30 | [IFC-037](../../../spec/interface/implementation.yaml), [PRV-005, PRV-001](../../../spec/promises/providers.yaml), [PKG-020](../../../spec/promises/packages.yaml); checker rule and invalid example; definitions |
| 16 | D19 (R18). How often the engine asks `inspect-run.v1`; a run or a stop on an unreachable machine; a run that returns no `endpoint` | How often goes into DL5. The second part was fixed in PLC-023 (coverage row 41). `run.v1` requires `endpoint`, and the engine's own runner, which no provider call reaches, is the one exception. A result without it fails validation and the part ends unknown under row 15's reading, since the run may have started; the retirement's stop ends it | P18 (only the engine's own runner gives no endpoint). The kubelet's relist period and status frequency are its own settings ([kubelet.go](https://github.com/kubernetes/kubernetes/blob/master/pkg/kubelet/kubelet.go)). MCP: "Servers MUST provide structured results that conform to this schema" ([tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools)), and OPS-007 gives unknown | [DL5](../../../spec/implementation-defined.yaml), [PRV-009, PRV-007, PRV-003](../../../spec/promises/providers.yaml), [IFC-017](../../../spec/interface/implementation.yaml), [`run.v1`](../../../spec/core/operations.yaml); definitions |
| 17 | D20, part. A `command` item whose program is a relative path names a file on the engine's machine. How does another runtime provider get it? | For now a guard: the engine never asks a runtime provider other than its own runner to run such an item; that part of the setup ends failed, and nothing runs. Whether such items stay on the engine's runner for good, or a provider fetches the package, is the owner's D20 | MAN-010 takes the program from the directory the manifest was read from, on the engine's machine, and PRV-019 ends failed a run that a provider cannot make there. Today `run.v1` hands over the path as written, and a provider on another machine would run whatever file that path names there | [PRV-007](../../../spec/promises/providers.yaml), [MAN-010](../../../spec/promises/manifest.yaml), [`run.v1`](../../../spec/core/operations.yaml) |
| 18 | R3. Once a closed Conduit is removed, what says why it closed? | The record of a change that no request asked for itself, such as a closing under CON-008 or a removal under LIF-035, holds in `invocation` the invocation within which it happened, and that invocation's caller: the target's retirement, the `retire.v1` or `reconcile.v1` that confirmed the consumer's shutdown, the declaration of loss, or the close itself. Where it happened within none, as under PLC-024, the record states the cause, as PLC-024 already has it. No new attribute | Before ruling 41, inspection showed which event closed a Conduit; decision 17 leaves only the records. REC-002 already has the fields, and this is row 12's rule for a record with no ID of its own. The CloudEvents correlation extension that the analyst proposed is not among the documented extensions, which "have no official standing" ([extensions](https://github.com/cloudevents/spec/blob/main/cloudevents/extensions/README.md)) | [REC-002](../../../spec/conventions/records.yaml), [CON-008](../../../spec/promises/conduits.yaml), [`record.v1`](../../../spec/core/results.yaml) |
| 19 | R5, part. A Resource of `retire-with-resources.v1` whose retirement reports conflict | Whatever the owner decides: a retirement that reports a reason in place of being dispatched has not ended, and LIF-021's order holds back each Resource that it puts after that Resource. Whether the request then goes on with the rest is the owner's R5 | LIF-021's order. Without this, a conflict could let the Resources that a busy Resource depends on be retired under it | [LIF-021](../../../spec/conventions/lifecycle.yaml), [`retire-with-resources.v1`](../../../spec/core/operations.yaml) |
| 20 | R6. May a Type require a hook and state its default? | Yes, as for any operation, under TPL-004. The hooks' own contracts state none. TPL-017's observable says that a type's default hook that is an item that is run counts under TPL-019. A rule rejecting a capability outside the core package that lists a hook is not adopted | Decision 1a: hooks are chosen "through the `implementation` map like any operation", and TPL-004 is how any operation gets a default. Terraform's resource types implement their own create and delete | [TPL-017, TPL-018](../../../spec/promises/templates.yaml), [`setup-hook.v1`, `retire-hook.v1`](../../../spec/core/operations.yaml); definitions |
| 21 | R7. Actor B's setup dispatches a new attempt of a hook that actor A's setup dispatched first. For which caller? | For B, the caller of the setup or retirement that dispatches it, as TPL-017 and TPL-018 say. From then on B is the invocation's caller, as OPS-024 makes the latest attempt's Conduit the invocation's. Each attempt's record names its own caller. A stated limit: an onward request from an earlier attempt still running on a machine cut off is decided for the latest caller | P20 (accepted): a hook is dispatched "for the caller of the setup or retirement". IMP-007 allows a new attempt only for a repeat from the caller, which holds only if the caller is the latest. Terraform reruns a failed provisioner within the next apply ([provisioners](https://developer.hashicorp.com/terraform/language/resources/provisioners/syntax)). Settled with row 34 | [OPS-028, OPS-008](../../../spec/conventions/operations.yaml), [IMP-007](../../../spec/conventions/implicit.yaml) |
| 22 | R9. A value that is not a string at a place that `environment`, `arguments` or `headers` points to | A string is passed as it is, a number in the form RFC 8785 gives it, a boolean as `true` or `false`, and null as row 24 says. A template whose item points to a place whose schema admits an object or an array, by `type`, `enum` or `const`, is rejected, unless `secret-reference.v1` governs the place | Decision 25 adopted server.json, whose Input says booleans are "true" and "false" and numbers "decimal values" ([server.json](https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json)). RFC 8785 fixes one decimal form ([RFC 8785](https://www.rfc-editor.org/rfc/rfc8785#section-3.2.2.3)), which DL2 already uses. Terraform converts numbers and booleans and rejects a tuple ([type constraints](https://developer.hashicorp.com/terraform/language/expressions/type-constraints)) | [MAN-017](../../../spec/promises/manifest.yaml), [PRV-007](../../../spec/promises/providers.yaml), [IFC-041](../../../spec/interface/implementation.yaml), [`run.v1`](../../../spec/core/operations.yaml) |
| 23 | R10. The keys of `run.v1`'s `secrets` | Two maps keyed by name, `environment` and `arguments`, as the plain ones are; an argument by its key under row 35. The engine joins each pointer to its names, and a name is in `environment` or in `secrets.environment`, never both. Core choice 3 is struck. `spec/README.md` adds that a secret bound to an argument is in the process's arguments, which process listings show | Decision 2c ("delivers each as the variable the item names"). ECS keeps secrets apart from the environment, under the variable's name ([ECS](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/secrets-envvar-secrets-manager.html)), and CRI keys a container's variables by name ([api.proto](https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto)) | [PRV-007, PRV-011](../../../spec/promises/providers.yaml), [SEC-010](../../../spec/promises/secrets.yaml), [`run.v1`](../../../spec/core/operations.yaml); `spec/README.md` |
| 24 | R11. A place that holds null | The variable, argument or header is left out. A template whose item binds an input that the Package or the Remote marks `isRequired: true` to a place whose schema admits null is rejected. What is done with a declared input given no value, its `default` among it, is server.json's, applied by whoever composes the command line | SEC-001's branch `{type: null}` is how an optional secret is written. Terraform treats null "as though you had completely omitted it", and errors where the argument is required ([types](https://developer.hashicorp.com/terraform/language/expressions/types)); the kubelet skips an optional `secretKeyRef` ([kubelet_pods.go](https://github.com/kubernetes/kubernetes/blob/master/pkg/kubelet/kubelet_pods.go)) | [PRV-007](../../../spec/promises/providers.yaml), [IFC-041](../../../spec/interface/implementation.yaml), [SEC-010](../../../spec/promises/secrets.yaml), [MAN-017](../../../spec/promises/manifest.yaml), [`run.v1`](../../../spec/core/operations.yaml) |
| 25 | R13. A reference that is not resolved when its record is kept | SEC-004 stands as ruled: a record holds such a place only in the resolved form. Where a reference did not resolve, the record holds no part of it, names the place and says that it did not resolve. Keeping a reference as written would change section 4's line 2c, and is the owner's | Section 4's table: "2c, a record keeps only the resolved form: Kept". OPS-029 resolves every reference in a request's inputs, so only one that names nothing reaches a record, and its `name` may be a pasted token | [SEC-004](../../../spec/promises/secrets.yaml) |
| 26 | R16. Two selected items whose `placement` name one label with different values | Rejected when the template is registered, as a type's and an item's already are. The checker's rule `label-conflict` compares every pair, with an invalid example | Decision 8a: "a label that both name with different values is rejected"; RuntimeClass admission rejects selectors that conflict ([admission.go](https://github.com/kubernetes/kubernetes/blob/master/plugin/pkg/admission/runtimeclass/admission.go)). PLC-007 already claims it | [PLC-025](../../../spec/promises/placement.yaml); [checker](../../../tools/check_schemas.py) |
| 27 | R17. A `default` for an input whose destination is an entry's `runtime`; a `runtime` that names a retired runtime provider | Both rejected. An instance names its runtime as it names its placement, so no template supplies it. A retired runtime provider can never run or stop anything, so an instance that named one could never be confirmed shut down or be removed. A provider that is registered and not yet ready is accepted | Decision 31b and P14; PLC-018 for a location. Kubernetes admits nothing new into a namespace that is terminating ([admission controllers](https://kubernetes.io/docs/reference/access-authn-authz/admission-controllers/#namespacelifecycle)) | [PLC-005](../../../spec/promises/placement.yaml), [POOL-009](../../../spec/promises/pools.yaml), [PRV-006, PRV-020](../../../spec/promises/providers.yaml), [LIF-009](../../../spec/conventions/lifecycle.yaml) |
| 28 | R19. The machines provider is not ready, or cannot place now | Unavailable where the provider is not ready, its call ends with no result, or it names a machine whose location no report has registered yet; and where it names none but a location that is not retired, other than `local`, has matching labels and its machine is reported unreachable or omitted. Rejected only where no location matches. G2's choice 25 and its alternative are superseded for these cases | PKG-024, PRV-007, PRV-008 and PLC-019 report unavailable for a provider that is not ready, and decision 8e makes unreachability temporary. The scheduler keeps a Pod pending "until the scheduler is able to place it" ([kube-scheduler](https://kubernetes.io/docs/concepts/scheduling-eviction/kube-scheduler/)) | [PRV-015](../../../spec/promises/providers.yaml), [POOL-016](../../../spec/promises/pools.yaml), [LIF-009](../../../spec/conventions/lifecycle.yaml); definitions |
| 29 | R20, second half. Does the engine's server list Conduit's extension, and must a sender of Conduit's keys declare it? | Yes to both. The engine's discovery result lists the extension at `v0`. A request that carries any of Conduit's keys and does not declare the extension at a revision the engine lists is answered with -32021 and reports invalid, as IFC-036 has it. A request with none of the keys needs no declaration, so a consumer that knows nothing of Conduit is unaffected | Decision 18 adopted the protocol's negotiation: servers advertise extensions in `server/discover`, and "A server MUST NOT rely on capabilities the client has not declared" ([base protocol](https://modelcontextprotocol.io/specification/2026-07-28/basic)); the tasks extension applies it to each request ([tasks](https://github.com/modelcontextprotocol/ext-tasks/blob/main/specification/2026-07-28/tasks.md)). IFC-001 binds the engine's server to the protocol. Ignoring the keys instead would drop the caller of the invocation served, against ACC-017 | [IFC-038, IFC-036, IFC-029, IFC-020, IFC-023](../../../spec/interface/implementation.yaml); definitions |
| 30 | R21. IFC-008's outcome | Failed, as IFC-008 and OPS-007 now say. No sentence on reading discovery again after the error: row 15's read for each run covers a server replaced by another version | OPS-003: failed means the effect has not occurred, and -32021 says that the implementation performed nothing. The log corrects the analyst's precedent: the task status `failed` is for errors during execution and does not reach this case | None |
| 31 | R22. A conflict under OPS-032 does not say whether the invocation is in progress or its outputs are gone | Split it. OPS-032 keeps a repeat of an invocation that has not ended; a new OPS-034 takes a repeat of one that succeeded and whose outputs are no longer kept. The error diagnostic's `ruleId` tells them apart. Only an operation that declares outputs reaches OPS-034 | Decision 24a adopted `ruleId` so that conflicts are told apart, and P8 fixed both behaviors. The idempotency key draft answers a request still in process with 409 ([draft](https://datatracker.ietf.org/doc/html/draft-ietf-httpapi-idempotency-key-header)), and Stripe gives it its own code ([error codes](https://docs.stripe.com/error-codes)). The ID moves before `v0`, which 24a's cost allows | [OPS-032, OPS-034 (new), OPS-005, OPS-008, OPS-009, OPS-024](../../../spec/conventions/operations.yaml), [IFC-025, IFC-016](../../../spec/interface/implementation.yaml), [REC-004](../../../spec/conventions/records.yaml), [ACC-012](../../../spec/conventions/access.yaml), [REV-014](../../../spec/conventions/revisions.yaml), [DL10](../../../spec/implementation-defined.yaml); definitions |
| 32 | R23. After a purge, an invocation of unknown outcome whose ID no actor holds | `inspect.v1`'s status lists in `unreconciled` the engine's IDs for the invocations of unknown outcome, not yet reconciled, that were dispatched to the instance or that name it in an input or as their target, read from the state of OPS-008. OPS-014 says that reconciling the retirement can confirm the shutdown, while every further setup or retirement ends unknown until the hook itself is reconciled | C33 keeps this state out of a purge, REC-004 promises a reconciliation after one, and decision 17 keeps an unknown shutdown "until it is settled". Kubernetes keeps in status what Events, "best-effort, supplemental data", cannot carry ([events types.go](https://github.com/kubernetes/api/blob/master/events/v1/types.go)) | [LIF-013](../../../spec/conventions/lifecycle.yaml), [OPS-014](../../../spec/conventions/operations.yaml), [REC-004](../../../spec/conventions/records.yaml), [`inspect.v1`](../../../spec/core/operations.yaml); definitions |
| 33 | R25. A cancelled `set-up.v1`, `retire.v1` or `retire-with-resources.v1` | The engine passes the cancellation on to each part in progress: by the protocol's means for a call it sent, and as OPS-027 cancels an invocation for a part that the engine itself implements. No later part begins, and `retire-with-resources.v1` dispatches no further retirement. Every retirement still asks for its stop. A cancelled part's invocation ends when the cancellation is sent, so its onward requests are refused from then on | Decision 22 passes the cancellation on and rejected keeping the work running; TPL-012 and TPL-009 decide what follows, and LIF-006 means that leaving out the stop gains nothing. A receiver may ignore what cannot be cancelled ([cancellation](https://modelcontextprotocol.io/specification/2026-07-28/basic/utilities/cancellation)). Settled with row 11 | [OPS-027](../../../spec/conventions/operations.yaml), [TPL-012, TPL-009](../../../spec/promises/templates.yaml), [LIF-021](../../../spec/conventions/lifecycle.yaml); definitions |
| 34 | R26. Can a hook carry a filter? | Not in this version, and one sentence in ACC-018 says so. The general rule stays | MAN-012 and CON-022: `engine` supports no filter, so no setup or retirement carries one. ACC-019 states a consequence in the same form | [ACC-018](../../../spec/conventions/access.yaml) |
| 35 | R27. Which key names a positional argument; replacing a template while a listed package is missing | A named argument by its `name`, a positional one by its `valueHint`. A key that names no argument, more than one, or one that states a fixed `value` is rejected. The second question was fixed (coverage row 32) | Decision 25: server.json uses `valueHint` "as a label identifying the argument", and an input whose `value` is set "should not be configurable by end users" ([server.json](https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json)) | [MAN-010](../../../spec/promises/manifest.yaml), `implementation-item.v1` |
| 36 | R27. How long a submitted project's address is held against installing a package there | While a template or an instance holds a submission at it. After that, a bound that names `address#name` covers the installed package's operation of that name, as decision 11 intends | SUB-008: "No object exists for the contract after the submission", so only templates and instances hold it, and MAN-016 is written in the present tense | [PKG-029](../../../spec/promises/packages.yaml), [SUB-008](../../../spec/promises/submissions.yaml), [MAN-016](../../../spec/promises/manifest.yaml) |
| 37 | G2's choice 16. PRV-004 refuses a Conduit that lists an operation of a provider capability under any capability | Confirmed. The operations are matched by their qualified references in the core package | P5: "no Conduit permits an operation of those capabilities", which PRV-001's allowance would otherwise defeat. The kubelet is the client of the container runtime ([CRI](https://kubernetes.io/docs/concepts/architecture/cri/)), and no API object grants a workload that endpoint (inference) | [PRV-004](../../../spec/promises/providers.yaml) |
| 38 | G2's choice 17. A provider call carries the provider's own `instance`, `config` and `revision` | Confirmed. PRV-005 says that `_meta`'s `instance` names the provider and the input `instance` the instance whose run is meant, and that `config` on a call is current while a run keeps the variables it started with | Decision 1b, and the protocol's statelessness. CSI sends a class's `parameters` on each call ([csi.proto](https://github.com/container-storage-interface/spec/blob/master/csi.proto)) | [PRV-005](../../../spec/promises/providers.yaml) |
| 39 | G4's E16. REV-014's exception for a repeat | Every repeat answered without a dispatch is excepted: under OPS-009, OPS-012, OPS-032 and OPS-034. A new attempt stays subject to REV-014 | Decision 23 excepted "a repeat under OPS-009", which then held what is now OPS-009 and OPS-032. Stripe answers a duplicate in flight apart from a busy object ([error codes](https://docs.stripe.com/error-codes)) | [REV-014](../../../spec/conventions/revisions.yaml) |
| 40 | G1. TPL-012 connects to each `remote` item | Confirmed. The run and the connections stand at one position in the order, and the hook waits for all of them | TPL-019 limits only the items that are run, under decision 2a | [TPL-012](../../../spec/promises/templates.yaml); definitions |
| 41 | G5. IFC-009's clause "as IFC-008 states for one such result" cut | Confirmed | IFC-008 now answers with an error and no result (decision 18) | None |
| 42 | G6. NAM-008, NAM-021 and PKG-017 changed to agree with PKG-006, MAN-016 and MAN-015 | Confirmed | Decisions 11 and 12. Cargo's `version` is optional until a package is published ([manifest](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field)) | None |
| 43 | G6's choice 5. OBJ-019 exempts the keys of a label selector | Confirmed | Decision 8a and C58; a `nodeSelector` is a map keyed by labels ([types.go](https://github.com/kubernetes/api/blob/master/core/v1/types.go)) | None |
| 44 | G6's choice 8. CTR-012 requires `type: object` at the root of both schemas | Confirmed, with the reason corrected: the protocol requires it of `inputSchema` only. For `outputSchema`, IFC-001 keys outputs by name, and without it `properties` would admit a result that is not an object | The protocol's schema ([schema.ts](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/schema/2026-07-28/schema.ts)); decision 16a | [CTR-012](../../../spec/promises/contracts.yaml) |

Settled together, because two answers touched one question or one promise:

- Rows 1, 13 and 28. Row 1 sends the placement of an instance on the engine's own runner
  through `place.v1`, so it relies on row 28 for a provider that is not ready and on row 13 for
  how the engine's machine is named. Row 28's rule for an unreachable match leaves out `local`,
  which is never unreachable.
- Rows 15, 16 and 30. The outcome of a `run.v1` result without `endpoint` follows from how row
  15 reads a provider call's result; row 15 reads discovery for each run, which also removes
  the optional sentence of R21 and the claim that the check keeps no state.
- Rows 12 and 18. The analyst of R3 proposed the attribute `causationid` beside D14's
  `serving`. Row 18 needs no attribute: a record with no ID of its own holds the invocation
  within which it happened, which is row 12's rule for a provider call.
- Rows 11 and 33. "No later part begins" is written once in TPL-012, for an instance retired
  during its setup and for a cancelled setup.
- Rows 21 and 34. R7's clauses on restrictions never apply in this version, so they are left
  out.
- Rows 31 and 39. REV-014's exception names OPS-034, so row 31 is applied first.
- Rows 6, 10 and 36. A bound compares names, so it covers an operation in every submission at an
  address, and later in a package installed there.
- Rows 22, 23, 24 and 35. Null is the one value not converted. An argument is keyed by its
  `name` or `valueHint` in `arguments` and in `secrets.arguments` alike, and a positional
  argument left out is composed as server.json composes an optional one.
- Rows 2 and 9 with the owner's D11. PRV-008 no longer says that there is always a provider to
  ask, and D12's rule that closing a Conduit recalls nothing agrees with SEC-014 and row 8.
- Several rows edit one promise, and the editor applies them as one change: PRV-007 (rows 11,
  16, 17, 22, 23, 24), PRV-006 (1, 27), PRV-015 (1, 28), POOL-016 (14, 28), PRV-005 (12, 15,
  38), REC-002 (12, 18), REV-014 (11, 31, 39), TPL-012 (9, 11, 33, 40), LIF-021 (19, 33), DL5
  (13, 16), `run.v1` (16, 17, 22, 23, 24). The analyst's D1 edit to PRV-006 is replaced by row
  1's edit to PLC-007, so it no longer meets R17's.

Candidate features for the tracker, which no one has asked for yet: a forced removal of an
instance whose shutdown can never be confirmed (D2, D3); a verb that restarts a ready instance's
run and keeps its identity and Conduits, as `kubectl rollout restart` does (D9); an address owned
by its first submitter (D12).

## For the owner

Nine questions. Eight change what a user sees or what a guarantee covers, and no ruling or
adopted standard picks the answer. One, D6, asks to correct a line the owner accepted. D5, D10 and D17 are about the same path, between the engine
and a server on another machine, and are best ruled together.

### D5. Requests from the engine to a server on another machine

The question. Ruling 40 point 3e says that a request that crosses machines carries proof of the
instance that sent it. Does that also bind requests that the engine sends to a server on another
machine, such as a secret store's server, so that the server accepts only the engine?

What the text says now. Only requests to the engine are covered
([IFC-022](../../../spec/interface/implementation.yaml)). Nothing limits who can reach a run's
`endpoint`, and `spec/README.md` lists this as outside the guarantee for secrets. What is at
stake: a server takes `config` as a request gives it (IFC-006), so anyone who reaches a store's
endpoint can send a locator of their choosing and read any secret that the store's own
credential reaches.

The options:

- Narrow. The endpoint of a secret Resource's run accepts only the engine, on any machine, the
  engine's own included: bound to loopback where the engine reaches it on its own machine, or
  authenticated at both ends by a mechanism of the kind DL13 names. For every other run, the
  runtime provider's contract states who else can reach its endpoints. Precedent: Kubernetes
  protects the path of a Secret's value, over TLS to the kubelet and over a Unix socket on the
  node to a secrets provider ([CSI providers][csi-providers]), while traffic from the control
  plane to a Pod is "neither authenticated nor encrypted" ([control plane to node][cp-node]).
  The protocol says that servers should authenticate every connection and, when local, bind to
  localhost ([Streamable HTTP][shttp]).
- Full. Every endpoint that a runtime provider returns accepts only the engine, which proves
  each request and checks the endpoint against a trust anchor in the provider's configuration,
  as the API server checks a kubelet with `--kubelet-certificate-authority`. Precedent:
  [kubelet authentication][kubelet-authn], and Istio's strict mutual TLS
  ([Istio security](https://istio.io/latest/docs/concepts/security/)).
- Requests to the engine only. Record that 3e covers requests to the engine. Each runtime
  provider's contract states who can reach its endpoints, and the limit stays in
  `spec/README.md`. Precedent: Kubernetes leaves who reaches a Pod to network policy or a mesh.
- No run off the engine's machine in v0. This contradicts ruling 40 point 3b, revised.

A second, smaller question goes with it: may a header that carries a secret be sent to an
`http` URL? server.json allows `http`, while OAuth 2.1 requires `https` for its own URLs
([OAuth 2.1](https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-1-13#section-1.5)).

Recommendation: narrow, and a header that carries a secret only to an `https` URL or to a
loopback host. It closes the one hole that defeats section 4 point 4 ("only by the engine's
read"), and keeps the shape of decision 2d and ACC-027, under which every restriction is
mediated in v0. Full authentication of every run can come with local enforcement in v1.

If you pick another option. Full adds a duty and a trust anchor to every runtime provider, and
needs D17's channel to carry what a run checks the engine with. Requests to the engine only, or
no answer, keep the store open to anything on the network that reaches it.

### D6. A request through a closed Conduit

The question. Once a Conduit is closed, what does a request through it report: refused or
invalid?

What the text says now. Refused ([ACC-011](../../../spec/conventions/access.yaml)). Decision 17's
cost line, which ruling 41 accepted, says the opposite: "a closed Conduit is removed when it
closes, so a request through it reports invalid rather than refused".

The options:

- Refused, as the text says, and the cost line is corrected. The engine decides access from the
  Conduits that are open (ACC-001, ACC-036), puts refused before invalid (OPS-023), and still
  holds the closed Conduit's ID (OPS-008, LIF-019), so it can tell a closed Conduit from one that
  never existed. A request to open, close or inspect the closed Conduit itself reports invalid.
  Precedent: Kubernetes authorizes before it looks an object up, so a request whose role binding
  was deleted is forbidden (403), not missing
  ([authorization](https://kubernetes.io/docs/reference/access-authn-authz/authorization/)).
- Invalid, as the cost line says. ACC-011, ACC-034, OPS-009, IFC-012 and IFC-013 change together.
  An agent whose access was taken away is told its request was malformed.

Recommendation: refused. It tells the agent the truth, that its access ended, and no other
promise changes.

### D8. What the engine's own runner passes to the processes it starts

The question. When the engine's built-in runner starts a server or an Agent, which variables of
the engine's own environment does the process get? Today a token exported in the shell that
started the engine, such as `GITHUB_TOKEN`, may reach every process it starts, an Agent's
included, outside every Conduit.

What the text says now. [SEC-020](../../../spec/promises/secrets.yaml) forbids the engine to
pass such a value in what it binds, and says nothing more. The coordinator settled (row 7) that
each engine states its choice in an item of the implementation-defined list.

The options:

- Each engine states it (settled for now). Precedent: section 4 of the review, line 18, "what
  else a run receives is the provider's"; systemd's user services inherit the manager's
  environment ([systemd.exec][systemd-exec]).
- A fixed set: `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM` and `USER`, skipping a value that begins
  with `()`, the item's own binding winning. Precedent: on Linux and macOS, the stdio clients
  of the protocol's TypeScript and Python SDKs pass exactly these by default
  ([stdio.py][sdk-stdio]). Cost: it withholds proxy settings, `AWS_PROFILE`, `KUBECONFIG`,
  `LANG` and `SSH_AUTH_SOCK`, so a package behind a corporate proxy cannot fetch, and a package
  item cannot bind a variable that its Package does not declare. The secrets proposal rejected a
  fixed set for that reason (its S18).

Recommendation: each engine states it, as now, with the limit in `spec/README.md`. An engine may
choose the SDKs' set. A fixed set is the stronger guarantee, at the cost above.

If you pick the fixed set. SEC-020 names the six, PRV-003 points to it, and the new item of the
list keeps only the working directory and what other operating systems use instead.

### D10. Other capabilities of a secret Resource's type

The question. May a store's Type offer something besides reading a secret, such as listing or
writing secrets? If it may, an operation of that other capability could return what the store
holds, through any Conduit an actor may open, outside the rule that only the engine reads a
secret.

What the text says now. [SEC-005](../../../spec/promises/secrets.yaml) forbids a control only,
and PRV-001 lets a provider's type offer other capabilities. `spec/README.md` names the gap.

The options:

- One capability. A Type that offers `secret-reveal.v1` offers nothing else, and is rejected when
  its package is installed if it does. A store package that lists or writes secrets does so
  through another Type, whose Resources are not secret Resources and need their own grant from
  the root Interface. Hooks still reach the store's implementation. Precedent: External Secrets
  Operator's controller is what "fetches secrets from an external API" ([ESO][eso]), and the
  CSI secrets driver alone calls its provider, over a socket on the node
  ([CSI providers][csi-providers]).
- Every operation reserved. No actor's request to a secret Resource is dispatched, whatever its
  operation, so another capability is allowed but no actor can use it. Precedent: the same
  controller-only shape.
- A duty on store packages. Every store's implementation returns what its store holds only
  through `reveal-secret.v1`, as section 4 point 3 lists what every store must do. The engine
  cannot check it. Precedent: Kubernetes shows why a check matters: "granting list or watch
  permissions on Secrets allows a subject to read all Secret data"
  ([Secrets](https://kubernetes.io/docs/concepts/configuration/secret/)).

Recommendation: one capability. It is the only option the engine checks, and it is the same
structural rule that decision 19 chose for controls.

If you pick another option. The reserved form keeps such types valid but makes the other
capability useless to actors. The duty keeps the gap stated in `spec/README.md`.

### D11. A run left on a machine that comes back

The question. When a machine is reported gone, or its location declared lost, its instances are
retired and their Conduits closed, but a process may still run there with the secrets it was
given. If the machine comes back, what stops that process?

What the text says now. Only an actor's further retirement asks for the stop
([PLC-027](../../../spec/promises/placement.yaml)). A request to unregister removes the instance
and asks for no stop (LIF-017). `spec/README.md` names the exposure.

The options:

- The engine keeps asking. For each such run the engine asks `stop.v1` again and again, after the
  instance is removed too, until a stop succeeds or the runtime provider is retired. Precedent:
  Kubernetes and Nomad stop what a returning node no longer should run, but their node agents
  do it: the kubelet lists the runtime's Pods ([kubelet_pods.go][kubelet-pods]), and "a
  disconnected Nomad client will stop its allocations"
  ([disconnect](https://developer.hashicorp.com/nomad/docs/job-specification/disconnect)). Here
  the control plane would keep that memory, and it ends when the provider is retired, which
  PRV-020 allows once no instance names it.
- One more stop, and the provider's contract. When an actor unregisters such an instance, the
  engine asks `stop.v1` once more, for that request. The runtime provider's contract states what
  it does with a stop it cannot carry out now, such as carry it out once it reaches the machine.
  Precedent: the same node agents; in Conduit the runtime provider plays their part.
- As now. Precedent: Kubernetes force deletion "does not wait for confirmation", and the resource
  "may continue to run on the cluster indefinitely" ([forced termination][forced]).

Recommendation: one more stop, and the provider's contract. It adds the least to the engine,
keeps the mechanism with the provider the user picks, and closes the case where an instance is
unregistered with no stop at all. It matters most where the root Interface declares lost a
machine that was only cut off, which is the case where a stop actually finds something.

If you pick another option. Keeping asking adds engine state that outlives the instance and a
new kind of call of the engine's own accord. As now keeps the line in `spec/README.md`.

### D12. Remote servers whose headers carry a secret, after the engine restarts

The question. The engine holds a secret header for a remote server only in memory, for as long
as its connection lasts. After the engine restarts, or fails over to another machine, the value
is gone, and reading it again needs a request with a caller (decision 4c). What should requests
to such a server do then?

What the text says now. Nothing says what a request to such a server reports once no
connection lasts, and the engine keeps no copy of a value it read
([SEC-017](../../../spec/promises/secrets.yaml)). Row 9 settles the rest of D12.

The options:

- Unavailable until set up again. Requests to the server report unavailable until an actor sends
  `set-up.v1` for the instance, which connects and reads again. Precedent: the protocol keeps no
  session, so a connection is only the credential the client holds ([Streamable HTTP][shttp]).
- Kept in the engine's state. The engine keeps the header values in its state store for the life
  of the connection, so a restart or a failover goes on with no read. Precedent: decision 2c lets
  a runtime provider keep a run's values, as Docker keeps them in a container's configuration.
  It reverses section 4 point 7 (no copy that stands in for a later read) and puts the state
  store inside the trust domain.
- Read again for the first caller. Precedent: the kubelet reads a secret again under its node's
  identity ([node authorization](https://kubernetes.io/docs/reference/access-authn-authz/node/)).
  It reverses decision 4c.

Recommendation: unavailable until set up again, with its cost stated in `spec/README.md`: after a
restart or a failover, each such instance needs a further setup. It follows from decision 4c and
section 4 point 7 without changing either.

If you pick another option. Keeping the values changes SEC-017 and SEC-018 and the trust domain.
Reading again for the first caller changes SEC-011 and decision 4c.

### D17. How a run elsewhere reaches the engine and proves which instance it is

The question. An instance that a runtime provider other than the engine's own runner runs, such
as the example Agent `reviewer-1` in Docker, must reach the engine's server and present proof of
which instance it is (IFC-022, ruling 40 point 3e). Nothing gives it the engine's address or that
proof.

What the text says now. [DL13](../../../spec/implementation-defined.yaml) covers stdio on one
machine only, and `run.v1` has no input for it (coverage row 38).

The options:

- A channel in `run.v1` for variables and files. An input, filled by DL13's mechanism, carries
  what the run needs, as CRI's `ContainerConfig` carries `envs` and `mounts`
  ([api.proto](https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto)):
  the engine's address as a variable, which the provider may give as the run sees it; a
  credential as a file, which the provider may refresh in place where the mechanism rotates it,
  as the kubelet refreshes a projected token
  ([service accounts](https://kubernetes.io/docs/concepts/security/service-accounts/)); or a
  socket the provider mounts, as SPIFFE's Workload API needs ([Workload Endpoint][spiffe-we]).
  The provider keeps them as PRV-011 has it for secrets, and no record holds them.
- The same channel, variables only. Simpler, but a variable cannot change during a run, so the
  credential would be valid for the whole run, which makes local enforcement in v1 harder.
- The provider's platform proves the run. A SPIRE agent or a Kubernetes token attests the run,
  and the engine trusts that issuer. Precedent: the SPIFFE Workload Endpoint. It ties identity
  to each provider, which PRV-018 keeps from counting by itself.
- No run off the engine's own runner for an instance that sends requests, until decisions 26b
  and 26c. This narrows ruling 40 point 3b, revised, and the Docker example could send nothing.

Recommendation: the channel for variables and files, ruled with D5. It keeps the mechanism the
engine's (decision 26a) and lets any runtime provider carry any mechanism. Its cost, to be stated
in `spec/README.md`: the provider's place and the run's machine hold a credential that presents
as the instance while the instance exists.

If you pick another option. Variables only is less to write and to rotate later. The provider's
platform moves identity into each provider's contract. The last option makes the example and
every multi-machine Agent unworkable until 26b and 26c.

### D20. Programs inside a package, under another runtime provider

The question. A `command` item may run a program shipped inside a package, such as
`bin/files-server`, which sits on the engine's machine. Should such an item run only on the
engine's own runner, or may another runtime provider fetch the package and run it, on any
machine?

What the text says now. Row 17 adds a guard: another runtime provider is never asked, and the
part ends failed. The valid example's Pool template `workspace` routes a `runtime` input to an
entry whose item is such a program
([templates.yaml](../../../spec/schemas/examples/valid/project/templates.yaml)).

The options:

- The engine's own runner only. An instance that names a `runtime` and selects such an item is
  rejected when its document is resolved. A server meant to run elsewhere is published as a
  `package` item, such as npm or an OCI image, which the provider fetches and checks against
  the digest that decision 25a requires. Precedent: a container runtime runs only what it pulls
  by reference or digest
  ([api.proto](https://github.com/kubernetes/cri-api/blob/master/pkg/apis/runtime/v1/api.proto)),
  and server.json has a client check a downloaded file against its `fileSha256`.
- The provider fetches the package. A provider whose contract says so is given the package's
  address, version, digest and `fetch`, checks the digest, unpacks it and runs the program from
  its root; any other ends the run failed, as PRV-012 has it for a registry type. Submitted
  projects stay on the engine's runner. Precedent: Nomad's `artifact` block fetches and unpacks
  with a checksum ([artifact][nomad-artifact]).

Recommendation: the engine's own runner only, for v0. It adds nothing, follows the norm of the
protocol's ecosystem, and the other option can be added later without breaking any document.
The example changes under either answer.

If you pick the other option. `run.v1` gains the package's address, version, digest and `fetch`,
and the guard becomes PRV-012's form.

### R5. Retiring a Pool with its Resources when one cannot be retired

The question. `retire-with-resources.v1` retires a Pool's Resources in order. When one Resource's
retirement does not complete, for example because its machine is unreachable, should the
request stop there, or go on retiring every Resource that does not depend on it?

What the text says now. [LIF-021](../../../spec/conventions/lifecycle.yaml) stops after a
retirement it dispatched ends without the shutdown confirmed, as round 3's decision 5 chose and
ruling 39 accepted; read literally, it goes on after a conflict. Row 19 makes a Resource that
reports a reason hold back the Resources ordered after it.

The options:

- Stop, after a conflict too. One rule. Which Resources were retired then depends on the engine's
  order and on timing, and the rest keep running, with their secrets and Conduits, until the
  request is made again.
- Go on. Only what the order puts after the Resource that could not be retired waits; every
  other Resource is retired; the Pool is not; a repeat finishes. Precedent: Kubernetes deletes
  the "dependents it knows about" before the owner
  ([garbage collection](https://kubernetes.io/docs/concepts/architecture/garbage-collection/)),
  and Terraform skips only what is downstream of an error
  ([walk.go](https://github.com/hashicorp/terraform/blob/main/internal/dag/walk.go)).

Recommendation: go on. Across machines, one unreachable machine would otherwise keep every later
Resource of the Pool running, and access to it open, until that machine returns.

If you pick stop. LIF-021 and the operation's description add the conflict to what stops the
request, and round 3's decision 5 stands.

## Left for round 5

To check against the protocol (R28): the extension negotiation, `listChanged` and `cacheScope`,
and the reserved trace keys (IFC-004, IFC-008, IFC-037 to IFC-039); with them, rows 15 and 29,
which declare the extension on provider calls and at the engine's server; and that the 2026-07-28
schema allows keywords such as `$ref` and `oneOf` at the root of `inputSchema`, which decision 16a
leaves out by Conduit's own choice (CTR-012).

For a reader:

- A control in progress when the engine retires its instance under PLC-017 or PLC-024: whether
  its result is stored in the retired instance (REV-010, LIF-015, row 11).
- A type that states a default for a hook it does not require: whether TPL-004 ignores it or
  CTR-004 should reject it (row 20).
- A named argument that a Package repeats, such as `-e`, cannot be bound under row 35; whether
  that matters, with row 22.
- PLC-024 and row 18 say that a record states a cause where no invocation holds it, in no fixed
  form.
- The late onward request of an earlier hook attempt, decided for the latest caller (row 21),
  before local enforcement in v1.
- For the vocabulary audit: `serving` as a member of a record, `unreconciled`, OPS-034's subject,
  and the member `instance` of a record (the instance that acted for the caller) beside the wire
  key `instance` (the instance whose configuration a request carries).

[csi-providers]: https://secrets-store-csi-driver.sigs.k8s.io/providers
[cp-node]: https://kubernetes.io/docs/concepts/architecture/control-plane-node-communication/
[shttp]: https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http
[kubelet-authn]: https://kubernetes.io/docs/reference/access-authn-authz/kubelet-authn-authz/
[systemd-exec]: https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html
[sdk-stdio]: https://github.com/modelcontextprotocol/python-sdk/blob/main/src/mcp/client/stdio.py
[eso]: https://external-secrets.io/latest/introduction/overview/
[kubelet-pods]: https://github.com/kubernetes/kubernetes/blob/master/pkg/kubelet/kubelet_pods.go
[forced]: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#pod-termination-forced
[spiffe-we]: https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_Endpoint.md
[nomad-artifact]: https://developer.hashicorp.com/nomad/docs/job-specification/artifact
