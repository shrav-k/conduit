# Re-audit, round 6: decisions

Status: decided 2026-11-07 by the coordinator, under rulings 37 and 38, and accepted by the
owner in [ruling 44](../../ruling-44/README.md): "I accept all your recommendations, ask me if
you need more rulings, once done we should discuss the nature of the first build and how we
will use it to test our theses and refine the conduit language". After the ruling, an
independent check found facts that bear on two of the simplifications, D06 and D12, and the
coordinator put each back to the owner with a revised recommendation. The owner answered "Agree
on both" ([after the ruling](../../ruling-44/README.md#after-the-ruling)). Everything below is
applied, except D06 and D12 as revised, which are not yet applied (action A69 of the
[tracker](../../../AUDIT.md#action-items)).

Round 6 is the owner's cold read by Codex. One reader read the whole repository at `c4d7d33`,
the merge of pull request 7, and wrote two reports: [six findings](reader-1.md) and
[a critique of the design](design-critique.md) in 22 points, D01 to D22, with its four open
points and twenty evidence cases, E01 to E20. Three reviewers that the reader tasked itself
helped it; they are not separate cold reads.

Ruling 44 accepted the coordinator's recommendations. The details that the coordinator fixed
in applying them after the ruling are marked "Detail after the ruling" below. They, and the
choices that the editors' logs beside this record list, have the standing of decisions the
owner can strike ([who decides what](../../../AUDIT.md#who-decides-what)).

The decision on each item is one of four:

| Decision | Meaning |
| --- | --- |
| Accepted and applied | The text under `spec/`, the tools or the entry points changed as the row says. |
| Accepted for the experiment program | The case or the measurement is placed in the [experiment program](../../../AUDIT.md#round-6s-evidence-and-measurements); nothing under `spec/` changed for it. |
| Accepted as revised, not yet applied | The owner accepted the revised recommendation after the ruling; action A69 applies it. |
| No change | The critique itself recommends keeping the rule; its evidence case is in the program. |

## The independent check

Before the decisions were applied, the coordinator ran seven checks, each in its own session:
one on each of the findings 01 to 05, one on the statements of the repository's status that
finding 06 and the four open points rest on, and one on the facts behind the simplifications
D06, D07, D11, D12, D14, D18 and D19. Each read `spec/` at `c4d7d33` and the primary sources:
the Model Context Protocol at revision 2026-07-28 and its tasks extension, JSON Schema 2020-12,
RFC 6901, 6902, 8259 and 9110, the draft of the `Idempotency-Key` header, and the Kubernetes
API reference. Three ran small schema checks with the repository's interpreter; none ran an
engine, since none exists.

Its verdicts: **all six findings hold.** Of the seven simplifications whose facts it checked,
six hold, and **D06 holds in part**: its attribution is incomplete and its cost understated.
Choice C42, which D06 changes, was accepted by the owner in ruling 37 as written, before ruling
39 accepted it again; every Pool Type in every package would have to map the seven operations
of `pool-management.v1` to the engine, where fourteen core Operation contracts state that
default today; and removing the defaults does not remove the failure D06 describes, since a
newly added operation would still make a new template invalid, under TPL-004's "neither" in
place of TPL-019. On D12 it found that X2 is the owner's own change in ruling 37, and that the
existing standard, Kubernetes, keeps names as the address and checks identity by a
precondition. These facts are what the coordinator put back to the owner.

The check also found what the reports did not say, and the decisions below use it:

- Finding 03: the empty pointer, the whole configuration, lies over every other pointer, so the
  rule compares pointers by their unescaped reference tokens and not as strings.
- Finding 01: the error -32021 can reach the engine as the failed error of a task, after the
  task did part of its work, which is within the published rules. Two nearby readings rest on
  the same kind of inference and were not checked: IFC-010 reads -32602 as failed whatever the
  operation's effect, and IFC-046 says the protocol has a server reject -32020 before acting.
  The tracker lists both as [open](../../../AUDIT.md#for-the-next-reader).
- Finding 02: "declares no effect" alone is the wrong test, since a control need not state an
  effect and an operation may start model work; IFC-033 already has the right one. A hook
  states an effect, declares no outputs and carries filters, and OPS-028 relies on a repeat of
  one that succeeded being answered from its outcome.
- Finding 04: a contract that refers to itself by `#` gives a silently wrong answer under literal
  replacement, and the core package's own `register.v1` and `replace.v1` give tool schemas that
  cannot be resolved.
- Finding 06: four more places than the reader named still said that V6 waited for the owner.
- The four open points: the text already settled the behavior of three; the fourth, the form of
  a machine's loss in a record, was open. The schema of a Package's arguments has `isRepeated`,
  and no promise says how one key binds the values of a repeated argument.
- D14: at revision 2026-07-28 the protocol's tool definition lets an `inputSchema` hold any
  keyword of JSON Schema 2020-12 beside `type: object`; CTR-012's four keys matched the earlier
  revision, 2025-11-25.

## The six findings

The severities are the reader's. Round 5 classed a conflict between a specific procedure and a
general safety promise as unsafe (M32); the reader kept "wrong, safety-critical" for 01 and 02.
This record keeps the reader's labels, and does not count the round as clean under G7.

| Finding | Severity | Check | Decision | What changed | Reason |
| --- | --- | --- | --- | --- | --- |
| 6-1-01 | wrong, safety-critical | holds | Accepted and applied | The protocol's error -32021 for a client capability that the call did not declare is read as any other error: failed for an operation that declares no effect, unknown for one that declares one (IFC-010, IFC-045, IFC-007). It stays failed only where Conduit's own contract binds the server: the -32021 that names Conduit's extension (IFC-008), and an implementation that carries Conduit's keys, which answers so only where it has performed nothing (IFC-047, added; OPS-007). V2-15 of round 5 is reversed. | The protocol has a server answer so where "processing" a request needs a capability, and sets no order against its effects. Read as failed, a partial effect let OPS-024 dispatch a repeat to a plain server, which is sent no idempotency key, against OPS-018. Detail after the ruling: the obligation on an implementation that carries Conduit's keys, which the ruling's "unless Conduit's own contract binds the server to have done nothing" names, written as a promise of its own. |
| 6-1-02 | wrong, safety-critical | holds | Accepted and applied | Where a filter applies to a repeat of an invocation that succeeded: an operation that states no effect, is not a control and starts no model work, as IFC-033 tells them, is dispatched again as a new attempt, so that its filters decide afresh; one that states an effect and declares outputs reports conflict under OPS-034 in place of the outputs kept; one that states an effect and declares no outputs keeps OPS-009's answer from its outcome. A repeat that is not dispatched is judged by no filter and gives only an outcome already recorded. OPS-005, OPS-008, OPS-009, OPS-024, OPS-032, OPS-034, ACC-008 and IMP-007 changed. | A filter with equal arguments need not allow the delivery now, as a deadline or changed authorization data shows, and ACC-008 and CTR-005 apply a filter to each request. Nothing limits filters to decisions that never change, and D05 advises against assuming it. A conflict for an operation with no outputs would reveal the same fact, and hooks rely on that answer. Costs: a repeat of a read returns what the target holds now; an operation with an effect and outputs, through a filtered Conduit, cannot recover a lost result by repeating. Detail after the ruling: the three cases, and IFC-033's test in place of "no effect" alone. |
| 6-1-03 | undetermined | holds | Accepted and applied | CTR-006 rejects a control whose `changes` holds two equal pointers, or one inside another, compared by their unescaped reference tokens, with the empty pointer over every other. `tools/check_schemas.py` applies the rule, with two examples under `invalid-by-rule/`. | Such a control has no consistent meaning: REV-006 and REV-007 contradict each other for it, and a map gives no order to apply it in (SCH-007, RFC 8259). INS-010 already rejects overlapping destinations of a template's inputs. No core control and no example relies on two inputs for one setting. Cost: an author cannot keep an old input name beside a new one. Detail after the ruling: the empty pointer. |
| 6-1-04 | wrong | holds | Accepted and applied | IFC-033 presents a self-contained bundle: each Schema contract reached is copied under the root `$defs`, under a key that cannot collide between packages; each local pointer in a copy is rewritten to point into it, and each reference to a contract to that contract's copy; keywords beside a `$ref` are kept; recursion is handled with one copy for each contract. A contract whose schema holds no `$ref` may be inlined. | JSON Schema resolves a fragment against its schema resource (2020-12, sections 8.2.3.1 and 9.3.1), so literal replacement breaks local pointers, changes what `#` means and never ends for a recursive contract; the definition must be readable with no package at hand. `tools/check_schemas.py` already bundles so for its own checks. Cost: a tool whose contracts refer onward shows a root `$defs`. |
| 6-1-05 | undetermined | holds | Accepted and applied | PRV-016: a package-source provider's Type lists no control whose `changes` reaches `/schemes` or a place within it, the whole configuration included, and such a Type is rejected when its package is installed or its project submitted; the schemes a provider claims are fixed for the instance's life. PRV-013 follows. An example under `invalid-by-rule/` shows the rejection. | A claim that controls could change needed a reservation, while a control was in progress or its outcome unknown, that no promise gave; a fixed claim needs none, and PRV-013's "at any time" holds with no procedure. The runtime (PRV-006), the placement (PLC-009) and a secret Resource's configuration (SEC-005) are fixed so already, and a Kubernetes CSIDriver is named by a name that never changes. Cost: to claim other schemes, a provider is retired and another registered. No package-source type exists in the repository. |
| 6-1-06 | misleading | holds | Accepted and applied | The root README, the tracker and the re-audit record say that the 29 findings of round 5's verification are resolved, 28 by fixes and V6 by the coordinator, as its option 1, in PLC-007 and PLC-002. Round 5's README and fix log keep their history and carry dated notes; the heading "For the owner" is kept, since links point at it. | The text already applied V6. The check found it stale in four more places than the reader named, among them the re-audit record and round 5's README. |

Promises that the six findings changed, as counted on the working tree against `c4d7d33`:
ACC-008; CTR-006; IFC-007, IFC-010, IFC-016, IFC-025, IFC-033, IFC-045; IMP-007; OPS-005,
OPS-007, OPS-008, OPS-009, OPS-024, OPS-032, OPS-034; PRV-013, PRV-016; and IFC-047, added.
The logs beside this record list each change and its reason, and
[the integration's](log-integration.md) the changes made to make the groups' files agree.

## The design critique

| Item | Decision | What it means | Reason |
| --- | --- | --- | --- |
| D01 | Accepted for the experiment program | Its qualification of a plain server, a small adapter and a Conduit-aware implementation, on one user task with the adapter work counted, is placed under S2 and S7, with A63. | A plain server supports no filter (PKG-020, CON-022), so restricted sharing depends on integration work that only a measurement can size. |
| D02 | Accepted for the experiment program | Its four cases, clients that keep the idempotency key and clients that send none, against plain and deduplicating implementations, and its table of benefits, are placed under S4. | A request without its key is new work by design (OPS-011); what a client keeps is a property of the client. |
| D03 | Accepted and applied, option a | v0 says plainly that a bound does not limit the filters of the Conduits that its holder opens, and that closing a delegator's own Conduit closes nothing it opened: [what access does not narrow](../../../../spec/README.md#what-access-does-not-narrow) in `spec/README.md`, and the observables of ACC-029 and ACC-030. E07 is in S2. | Ruling 44, point 2. Implication between arbitrary filters is not defined, and S1 and S2 measure whether the limit matters; a focused constraint comes back to the owner if it does. |
| D04 | Accepted and applied, option a | v0 says plainly that access to `management` cannot be narrowed to one template, and names the broker: a trusted operation of a Resource whose own contract allows only what it should (`spec/README.md`, ACC-030). E08 is in S1. | Ruling 44, point 2. A filter cannot be applied to `management`'s operations (CON-022), and a new policy language is not wanted for it. |
| D05 | Accepted for the experiment program | Under S2, the first restrictions are complete contracts with vectors shared by two implementations, covering changed authorization data, a repeat, onward calls and revocation. | Support by name does not make two implementations substitutable. Finding 02 is not solved by assuming that filters never change. |
| D06 | Accepted as revised, not yet applied | As the owner accepted it after the ruling: a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule, with no mapping. Action A69. | The check: C42 was accepted in ruling 37, removing every default would put seven mappings in every Pool Type, and the failure D06 describes would remain under another rule. The revision keeps the separation D06 wants without that cost, as Kubernetes's API server serves its built-in resources while a custom resource's definition names no controller. |
| D07 | Accepted and applied | IFC-033 presents a control's input narrowed by the target's template as `allOf` of the contract's schema and the template's, and IFC-012 presents no tool for a control that the target does not have, because the target's template leaves it out of its `controls` or its type does not list it (TPL-013). A call that names the tool such a control would have is answered in the protocol's form for an unknown tool, and reports invalid under TPL-013, as before; every other call to a tool that is not presented reports refused (IFC-012, OPS-005). | The protocol lets the tools presented vary with the authorization of the request; the agent otherwise learns a fixed limit only from an invalid call (TPL-011, TPL-013). Cost: a call to a control that is left out is answered with the protocol's error for an unknown tool and no tool result, its reason in the error's `data`. Choices of the editors: the fixed form `allOf`, both cases of TPL-013, and invalid rather than refused for such a call, which keeps TPL-013 as it was. |
| D08 | Accepted for the experiment program | A chapter of change and repair on the S1 tracer: a live setting changed, a template upgraded, a nested Resource replaced and the intended access restored. | Frozen instances buy explicit effects at a cost that ends after setup would not show. |
| D09 | Accepted for the experiment program | Rotation of the shared service's credential by explicit replacement, in S2 and S10 (E13); the restart verb of A67 comes to the owner only if that is disproportionately disruptive. | No operation ends a ready instance's run and keeps the instance; the cost is to be measured before a feature is proposed. |
| D10 | Accepted for the experiment program | Under S4 and S5: what an actor can conclude, verify and do next after an unknown outcome, a conflict and a partial setup, from `ruleId`, the invocation's identity and its status. | The small set of reasons stays; whether the portable facts suffice is a question for the vectors. |
| D11 | Accepted and applied | `replace.v1` takes the project as it is submitted with the replacement, its contracts and its manifest, and resolves the template against it as registration does, under the existing check of revisions, reusing SUB-008's digests and the address rules of PKG-029 and MAN-016 exactly. Instances made before keep what they resolved. It replaces one template and is not an apply of several. SUB-009, NAM-022 and PKG-029 changed. E11 is in S1. | Ruling 44, point 3. The earlier rule resolved a replacing template's `#name` against its original project, so a local contract edited with it was not loaded. |
| D12 | Accepted as revised, not yet applied | As the owner accepted it after the ruling: names stay the only address, LIF-031 stands, and `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take an optional expected ID, which reports conflict where the object that the name resolves to has another. Action A69. E12 is in S1. | The check: X2 is the owner's own change in ruling 37, and Kubernetes checks identity by a precondition on a named request (`preconditions.uid`, 409 Conflict) and has no request addressed by ID. |
| D13 | No change | Default insertion keeps its semantics until a comparison shows a net gain, as the critique recommends; E16 is in S5. | The smaller models each lose something; the critique does not recommend a change now. |
| D14 | Accepted and applied | The schemas of an Operation contract that is not a control may hold any keyword of the profile at their root beside `type: object`, as the protocol's tool definition allows at revision 2026-07-28; the inputs and outputs are still the members of the root's `properties`, a control keeps the limited root, and SCH-008's ban on a `default` under a conditional keyword stands (CTR-012). | Ruling 44, point 4. The four keys followed the earlier revision of the protocol; wrapping an input only to reach standard composition adds structure without meaning. A control's partial update (REV-006) still needs the limited root. |
| D15 | Accepted for the experiment program | With A63: what each provider implements, delegates, keeps and needs configured. | A thin adapter is a claim until it handles start, secrets, restart, a lost machine and a gate. |
| D16 | Accepted for the experiment program | With S10: a comparison defined before it runs, with the provider-specific edits and the replacements it allows (E14). | Swapping proves alternative implementations of operations, not migration of live runs. |
| D17 | No change | Registration stays non-fetching. Tooling may prepare explicit, ordered installation requests; the author measurements of D22 count the packages and steps an author needs. | The critique recommends no change to the rules, and warns against version ranges or implicit installation to shorten an example. |
| D18 | Accepted and applied | ACC-033's `then` is one enumerated decision list, of the request, the family and state of the object, and the target that authorizes it, derived from the earlier text and replacing its prose, with no answer changed. E18 is in S4. | Its `then` was the longest of all the promises, 406 words. A list that replaces the prose keeps one source; one beside it would become a second. |
| D19 | Accepted for the experiment program | The reference model measures the state it keeps for each keyed, unkeyed, failed and unknown invocation (A38), projected for A14. | Dropping any of that state would change OPS-008 and OPS-030, which is the owner's; a key is never let expire. |
| D20 | Accepted for the experiment program | Under S2 and S3, with A64: a local deployment and an isolated one with the same relationships, which credentials an Agent process can read, and what a gate protects. | The trust limits are stated in `spec/`; whether a user sees them at the point of choosing a runtime is to be measured. |
| D21 | Accepted and applied | `spec/README.md` has reading paths for an author, a writer of contracts and providers, and an engine builder; the root README puts the current design before its history; the documentation map, the contributor guide and the tracker link the paths. | Ruling 44 accepted it as proposed. One source for each rule: the entry points link the paths rather than repeat them. |
| D22 | Accepted and applied | The next milestone is the first build, A68: S1 through the reference model, the vectors for the safety cases first and the first providers, with the author measurements on each run. The tracker's order of work and its action items say so. | Ruling 44, point 1. The nature of the first build is discussed with the owner first. |

## The four open points

| Point | Decision | What changed or where it waits | Reason |
| --- | --- | --- | --- |
| A default that a Type states for a hook it does not require | Accepted and applied | It is not used: a template that selects no implementation for the hook has none, the hook is not dispatched, and the template is registered with a diagnostic that names the default (TPL-004). | The hooks are in no capability, so TPL-004 already covered only a hook the type requires; the diagnostic keeps an inert default from passing unseen. |
| A named argument that a Package repeats, such as `-e` | Accepted for the experiment program | It waits on a real package that needs it, in S7 and A63. MAN-010 still rejects a key that names more than one argument. The smaller gap the check found, how one key binds the values of an argument marked `isRepeated`, waits with it. | The behavior is defined; whether its limit matters is a question for a real package. |
| The cause in a record of a machine's loss (PLC-024, REC-002) | Accepted for the experiment program | A portable field of the record for the cause is settled with the conformance vectors (A39). | The obligation to state the cause is in force; only its form was open. |
| A late onward request of an earlier hook attempt (OPS-028) | Accepted for the experiment program | Decided for the latest caller, kept for v0, tested (E20, in S4), and revisited with local enforcement (A65). | A documented limit of v0, not unspecified behavior. |

## For the owner

- **The first build.** The owner asked to discuss its nature, and how it will test the theses
  and refine the language, once ruling 44 is applied. The tracker lists it under
  [waiting for the owner](../../../AUDIT.md#waiting-for-the-owner).
- **D06 and D12** are settled as revised ("Agree on both") and are applied under A69.
- **Details after the ruling** that the owner may want to read first, since each settles a
  behavior: the obligation that an implementation carrying Conduit's keys answer -32021 only
  where it has performed nothing (IFC-047; the check had called the choice between this and a
  read like a plain server's the owner's, and the coordinator read the ruling's words on 01 as
  naming it); the third case of 02, under which a repeat of an operation with an effect and no
  outputs keeps its answer from the outcome where a filter applies; and the empty pointer in
  03. Of the editors' choices, one settles a behavior: a call that names the tool of a control
  that the target does not have reports invalid, under TPL-013 as it stood, and not refused as
  a call to any other tool that is not presented does (D07; IFC-012).
- **The severity of 01 and 02.** This record keeps the reader's "wrong, safety-critical" and
  does not count round 6 as clean. The tracker now asks a reader to mark such a conflict as
  unsafe, as round 5 did with M32.
