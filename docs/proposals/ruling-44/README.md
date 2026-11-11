# Ruling 44: the Codex read, and building next

Status: in force; amended by 61 (B8, point 2: Conduits only) (ruled by the owner on 2026-11-07). Holds now: [IN_FORCE.md: 6. Conduits and access](../../../IN_FORCE.md#6-conduits-and-access), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement); [CTR-012](../../../spec/promises/contracts.yaml), [LIF-037](../../../spec/conventions/lifecycle.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

The owner's cold read by Codex, round 6 of the re-audit, read `main` at `c4d7d33` and returned
two reports ([round 6](../reaudit/round-6/README.md)): six findings, and a critique of whether
Conduit is simple and useful, with 22 design critiques, D01 to D22. The coordinator summarized
both, said which it would settle itself under rulings 37 and 38, and put four questions to the
owner, each with a recommendation.

The owner's ruling: "I accept all your recommendations, ask me if you need more rulings, once
done we should discuss the nature of the first build and how we will use it to test our theses
and refine the conduit language".

## What follows

1. **Build next.** The next milestone is a working demonstration, not another round of review
   of the prose: the S1 scenario through a reference model in Rust, conformance vectors for the
   safety cases first, and the first providers. Each run also measures what a new author must
   do: the files edited, the concepts met, the grants made by hand, the adapters written and the
   steps of recovery that need a person (D22). The nature of the first build is discussed with
   the owner next.
2. **Narrow delegation and narrow provisioning (D03, D04): option a.** v0 says plainly that a
   bound does not limit the filters of the Conduits its holder opens, and that access to
   management cannot be narrowed to one template. Narrow provisioning goes through a trusted
   operation of a Resource, a broker, whose own contract allows only what it should. S1 and S2
   measure whether that is good enough; if it is clumsy, a focused constraint, such as a list of
   allowed templates on access to management, comes back to the owner.
3. **Four simplifications, each accepted:**
   - **D06.** An Operation contract states no default implementation. A Type's default or a
     template's selection chooses what runs. This changes the core-and-schemas choice C42,
     which ruling 39 accepted.
   - **D07.** The tools that the engine presents show the limits that the target enforces: a
     template's narrowing of a control's inputs is in the schema presented, and a control the
     instance does not offer is not presented. Templates keep their narrowing.
   - **D11.** Replacing a template resolves it against the project as it is submitted with the
     replacement, as registration does, under the existing check of revisions. Instances made
     before keep what they resolved.
   - **D12.** An actor may name an object in its own request by the resolved reference that an
     inspection showed, `{kind, id}`, as well as by its name. An ID grants nothing: every check
     of kind, existence and access applies, and the ID of a removed object names nothing, even
     where its name was reused. This changes LIF-031 and the core-and-schemas choice D10/X2.
4. **D14.** An ordinary Operation's `inputSchema` and `outputSchema` may use the whole schema
   profile at their root, keeping `type: object`, where the published protocol allows it.
   Controls keep the stricter rule, since their partial updates need it. This changes item 16a
   of the principles review.

Accepted as the coordinator proposed to settle them:

- **The six findings** of [round 6](../reaudit/round-6/reader-1.md): 01, a missing client
  capability is read as unknown for an operation with an effect, unless Conduit's own contract
  binds the server to have done nothing; 02, where a filter applies, a repeat of an operation
  with no effect is dispatched again so that the filter decides afresh, and a repeat of one with
  an effect reports conflict rather than delivering stored outputs; 03, a control whose `changes`
  writes one place twice, or a place inside another, is rejected; 04, the tool schema the engine
  presents is a self-contained bundle that keeps the meaning of every reference; 05, the schemes
  a package-source provider claims are fixed for its life; 06, the entry points no longer say
  that V6 waits for the owner.
- Management routing (ACC-033) restated as one decision table, with no change of behavior
  (D18).
- The entry points put the current design before its history, with reading paths for authors,
  for writers of contracts and providers, and for engine builders (D21).
- The critique's evidence cases E01 to E20 and its qualification tables (D01, D02) go into the
  experiment program, with D05, D08 to D10, D15, D16, D19 and D20 as measurements of S1, S2 and
  S10.
- The four open points, as the critique recommends: a Type's default for a hook that is not
  required is ignored, with a diagnostic; a repeated Package argument waits on a real package
  (S7, A63); the cause in a record of a machine's loss gets a portable field with the vectors;
  OPS-028's attribution of a late onward request to the latest caller is kept for v0, tested,
  and revisited with local enforcement (A65).
- Round 6's decisions are recorded beside its report, applied, and merged with it as one pull
  request (pull request 8).

## Earlier rulings this changes

| Ruling | What it said | Now |
| --- | --- | --- |
| 39 (core-and-schemas choice C42) | An Operation contract may state a default implementation. | A package's own Operation contract states none; the engine implements the core package's operations by rule (D06, as revised after the ruling). |
| 39 (core-and-schemas choice D10/X2; LIF-031) | An actor's own request names objects by reference only, not by resolved ID. | Unchanged; the lifecycle actions take an optional expected ID instead (D12, as revised after the ruling). |
| 41 (item 16a) | An Operation's schemas have a restricted root. | Ordinary Operations may use the whole profile at the root (D14). |

## After the ruling

An independent check of the round-6 report, which the coordinator ran before applying it, found
new facts on two of the accepted simplifications, and the coordinator put each back to the
owner with a revised recommendation.

- **D06.** The operations of the core capability `pool-management.v1` get their implementation,
  the engine, from the defaults of their Operation contracts, so without those defaults every
  Pool Type in every package would map seven operations by hand. Removing the defaults also does
  not remove the failure the critique describes, where a new operation makes a new template
  invalid; it changes which rule rejects it. Revised recommendation: a package's own Operation
  contract states no default implementation, and the operations of the core package that the
  engine implements are implemented by the engine as a rule, with no mapping, as Kubernetes's API
  server serves its built-in resources while a custom resource's definition never names its
  controller.
- **D12.** Kubernetes addresses an object by name and lets an action carry an expected ID as a
  precondition, `preconditions.uid`, which fails with 409 Conflict on a mismatch; it has no
  request addressed by ID. And the rule that an actor's request names objects by reference only
  is the owner's own change X2 in ruling 37. Revised recommendation: names stay the only
  address, LIF-031 stands, and the lifecycle actions (`set-up.v1`, `retire.v1`, `inspect.v1`
  and `unregister.v1`) take an optional expected ID, which reports conflict where the object the
  name resolves to has another ID.

The owner's answer, on 2026-11-07: "Agree on both". Items D06 and D12 of "What follows" above
are read as revised here, and the table of earlier rulings changes accordingly: C42 is changed
for packages' own Operation contracts only, and D10/X2 and LIF-031 are not changed.
