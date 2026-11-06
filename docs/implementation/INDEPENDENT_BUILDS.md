# Independent builds

Status: the roles and independence rules were accepted by the owner on 2026-11-05, and the scope
below follows layer decision LD4 of the same day. **The builds have not started and must not
start until the [readiness gate](#readiness-gate) passes.** Directory and branch names are
proposals; the [form of the specification](../proposals/normative-surface/README.md) is ruled.

## What the builds prove

The claim under test is the [layered claim](../AUDIT.md#layered-claim): the language fully
determines what a set of declarations means, and the shared conventions hold as invariants. The
recognized public evidence for a claim of this kind is two independent implementations passing
one shared test suite.

Two implementations are built in Rust, one by a Codex session and one by a Claude session.
Neither checks the other's work. Each covers three things:

| Part | What it is | How the two are compared |
| --- | --- | --- |
| Language layer | Declarations in; resolved objects and diagnostics out. A pure function with no effects. | On the public and held-out vectors, and on generated declarations, where any difference in output is a finding. |
| Shared conventions | A minimal engine that honors the seven conventions: operation identity and outcomes, revision-checked change, access, declared against observed state, ownership and lifecycle, records, and that nothing starts model work or repeats an effect implicitly. | On convention vectors. |
| Implementation interface | The shared wire between an engine and an implementation. | The same test implementations must work unchanged against both engines. |

Every place where a builder has to ask, guess or disagree with the other on these three parts is
a defect in the language, and is fixed in the language.

The builds are not full engines. Behavior that belongs to a type contract or to the runtime
profile is outside them; it is built once, in the reference engine, and may legitimately differ
in any other engine.

## Roles

| Role | Who | May see | Produces |
| --- | --- | --- | --- |
| Language side | The owner, with the sessions that have been writing the language | Everything | The normative surface, the reference model, the conformance vectors |
| Builder | One fresh Codex session and one fresh Claude session that did not write the language | Only the contents of the build base (below) | One implementation each, with a defect log |
| Comparer | The owner, or a neutral harness run by the owner | Everything | The comparison of the two implementations |

A session that helped write the language is not a cold reader and does not act as a builder.

## Repository layout and isolation

Isolation is enforced by what a builder's checkout contains, not by trust.

| Branch or tag | Contents | Purpose |
| --- | --- | --- |
| Working branches and `main` | Everything | Language work |
| `build-base` (orphan branch) | `spec/`, `conformance/`, the build brief as `BUILD.md`, `AGENTS.md` and `CLAUDE.md` pointing at it, and the licence. Nothing else, and no shared history with the working branches. | The only input a builder receives |
| `spec-v1`, `spec-v2`, … (tags on `build-base`) | One published revision of the language | Both builders work from the same revision |
| `build/codex`, `build/claude` | Forked from a `spec-vN` tag; each adds its implementation under `impl/` | One per builder |

Each builder works in its own clone created with a single branch, so no other branch exists
locally:

```sh
git clone --single-branch --branch build/codex <repository> conduit-build-codex
```

Proposed top-level directories on the working branches:

| Directory | Contents | Visible to builders |
| --- | --- | --- |
| `spec/` | The normative surface: the language and the shared conventions, as a manifest, promises and schemas | Yes |
| `conformance/` | Public conformance vectors and the adapter contract | Yes |
| `conformance-held-out/` | Vectors withheld from builders, run only by the comparer | No |
| `reference-model/` | The executable reference model in Rust | No |
| `docs/`, `experiments/` | Explanation, research, history, proposals, exploratory code | No |
| The reference engine, its standard library of type contracts and its runtime profile | The one full engine | No |

## Rules

1. **No private clarification.** A builder's question is answered only by changing the language
   and publishing a new `spec-vN` tag, so both builders always hold identical information.
2. **No contact between builders.** Neither sees the other's branch, logs or results.
3. **Guesses are recorded.** When the language is silent, ambiguous or contradictory, the builder
   records a defect, takes the most conservative reading, marks the code, and continues.
4. **The language and vectors are read-only for builders.**
5. **No fitting to the vectors.** An implementation may not key behavior on vector identifiers or
   contents. Held-out vectors exist to detect this.
6. **Rust only**, by the owner's ruling of 2026-11-05.

## Readiness gate

All of these must hold before the brief is issued. The gate exists because a build started on an
immature language spends its effort rediscovering defects that cheaper methods find first.

| # | Condition | Evidence |
| --- | --- | --- |
| G1 | The normative surface is defined: a manifest lists every normative file and nothing normative lives outside it | `spec/README.md` |
| G2 | The promise register covers the language and the shared conventions; each promise has at least one vector or is on the explicit implementation-defined list | Coverage report |
| G3 | Schemas exist for the envelope, each family and form, packages, contracts, instantiation and vectors; every scenario's declarations validate with the Rust loader | Loader test run |
| G4 | The reference model for the language layer and the shared conventions passes every vector with no unhandled case | Model test run |
| G5 | Every safety promise in the shared conventions that can be modelled holds under model checking in the stated configurations; the rest are listed with the reason | Stateright test run |
| G6 | Mutation testing of the model against the vectors leaves no unexplained surviving mutant | Mutation report |
| G7 | Defect convergence: two consecutive sweeps of the language and the shared conventions find no new defect, and every defect triaged to those two layers is closed or declared implementation-defined | [Defect register](../LANGUAGE_DEFECTS.md) |
| G8 | Every open audit question is closed or declared out of the builders' scope | [Plan audit](../AUDIT.md) |
| G9 | The implementation interface is specified to the level needed for one implementation to work unchanged against both builds | The implementation interface in `spec/` |
| G10 | A pilot run: a fresh session follows the brief for the first milestone only, time-boxed, to test the brief itself. Its work is discarded and that session builds nothing further | Pilot notes |
| G11 | Owner sign-off | Recorded in the plan audit |

## Comparison

After each milestone the comparer:

1. Runs both implementations' conformance adapters over the public and held-out vectors.
2. Runs generated declarations through both language layers and the reference model and compares
   the resolved objects and diagnostics.
3. Reads both defect logs side by side. A situation logged by only one builder is still a defect:
   the other builder guessed without noticing.
4. Runs the same test implementations against both engines.

Every divergence is traced to the language. The fix goes into the language, a new `spec-vN` tag
is published, and both builders update.

## Limits of the evidence

Two builders from different model families are more independent than two sessions of one model,
but they share training on the same public material and can share blind spots. Agreement between
them is strong evidence, not proof. The claim published to users is exactly what was shown: two
independent implementations of the language layer, the shared conventions and the implementation
interface pass the public and held-out vectors at a named language revision.
