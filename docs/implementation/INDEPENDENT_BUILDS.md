# Independent builds

Status: the roles and independence rules were accepted by the owner on 2026-11-05, and the scope
below follows layer decision LD4 of the same day. **The builds have not started and must not
start until the [readiness gate](#readiness-gate) passes.** The specification exists as a draft
in [spec/](../../spec/README.md), and its [form](../proposals/normative-surface/README.md) is
ruled. `conformance/` and the branches named below do not exist yet; their names are
proposals. `engine/` holds the code of the first build (ruling 46), which is not one of
these builds.

Where the readiness gate and the builds come in the work: step 7 of the
[roadmap](../../ROADMAP.md#the-order-of-the-work).

## What the builds prove

The claim under test is the [layered claim](../AUDIT.md#layered-claim): the language fully
determines what a set of documents means, and the shared conventions hold as invariants. The
recognized public evidence for a claim of this kind is two independent implementations passing
one shared test suite.

Two implementations are built in Rust, one by a Codex session and one by a Claude session.
Neither checks the other's work. Each covers three things:

| Part | What it is | How the two are compared |
| --- | --- | --- |
| Language layer | Documents in; resolved objects and diagnostics out. A pure function with no effects. | On the public and held-out vectors, and on generated documents, where any difference in output is a finding. |
| Shared conventions | A minimal engine that honors the seven conventions: idempotency keys and outcomes, revision-checked change, access, declared against observed state, ownership and lifecycle, records, and that nothing starts model work or repeats an effect implicitly. | On convention vectors. |
| Implementation interface | The shared wire between an engine and an implementation. | The same test implementations must work unchanged against both engines. |

Every place where a builder has to ask, guess or disagree with the other on these three parts is
a defect in the language, and is fixed in the language.

The builds are not full engines. Behavior that belongs to a type contract or to the conformance
document is outside them; it is built once, in the reference engine, and may legitimately differ
in any other engine. What a provider does behind its contract is outside them too (ruling 40):
the first providers are built apart, as packages (action A63 of the plan audit).

## Roles

| Role | Who | May see | Produces |
| --- | --- | --- | --- |
| Language side | The owner, with the sessions that have been writing the language | Everything | The specification, the engine, the conformance vectors |
| Builder | One fresh Codex session and one fresh Claude session that did not write the language | Only the contents of the build base (below) | One implementation each, with a defect log |
| Comparer | The owner, or a neutral harness run by the owner | Everything | The comparison of the two implementations |

A session that helped write the language is not a cold reader and does not act as a builder.

## Repository layout and isolation

Isolation is enforced by what a builder's checkout contains, not by trust.

| Branch or tag | Contents | Purpose |
| --- | --- | --- |
| Working branches and `main` | Everything | Language work |
| `build-base` (orphan branch) | `spec/`, `conformance/`, `tools/`, the build brief as `BUILD.md`, `AGENTS.md` and `CLAUDE.md` pointing at it, and the licence. Nothing else, and no shared history with the working branches. | The only input a builder receives |
| `spec-v1`, `spec-v2`, … (tags on `build-base`) | One published revision of the specification | Both builders work from the same revision |
| `build/codex`, `build/claude` | Forked from a `spec-vN` tag; each adds its implementation under `impl/` | One per builder |

The tags number published revisions. They are not versions of the specification: its version
is `v0`, the one [spec/names.yaml](../../spec/names.yaml) states, and every `spec-vN` tag is a
revision of that version.

`tools/` ships on the build base because files under `spec/` tell the reader to run
`tools/check_schemas.py` and `tools/set_names.py`. `spec/README.md` lists a third check,
`tools/check_docs.py`. That one is for the whole repository: it checks the links between the
Markdown files, and on the build base the links from `spec/` into `docs/` lead nowhere. The
brief therefore names only the first two. Files under `spec/` also link to records
under `docs/`, which the build base does not hold; the brief tells builders that they do not
need them.

Each builder works in its own clone created with a single branch, so no other branch exists
locally:

```sh
git clone --single-branch --branch build/codex <repository> conduit-build-codex
```

Top-level directories on the working branches. `spec/`, `tools/`, `docs/` and `experiments/`
exist. The others are proposed and do not exist yet.

| Directory | Contents | Exists | Visible to builders |
| --- | --- | --- | --- |
| `spec/` | The specification: the language, the shared conventions and the implementation interface, as a contents list, promises, definitions, schemas and the core package | Yes | Yes |
| `tools/` | The checkers the specification refers to | Yes | Yes |
| `conformance/` | Public conformance vectors and the adapter contract | No | Yes |
| `conformance-held-out/` | Vectors withheld from builders, run only by the comparer | No | No |
| `engine/` | The reference engine in Rust | No | No |
| `packages/` | The first providers, one Rust crate per provider (ruling 40) | No | No |
| `docs/`, `experiments/` | Explanation, research, history, proposals, exploratory code | Yes | No |
| The reference engine, its standard library of type contracts and its conformance document | The one full engine | No | No |

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
| G1 | The specification is defined: a contents list names every normative file and nothing normative lives outside it | `spec/README.md` |
| G2 | The promise register covers the language and the shared conventions; each promise has at least one vector or is on the explicit implementation-defined list | Coverage report |
| G3 | Schemas exist for the envelope, each kind, packages, contracts, instantiation and vectors; every scenario's documents validate with the Rust loader | Loader test run |
| G4 | The core for the language layer and the shared conventions passes every vector with no unhandled case | Model test run |
| G5 | Every safety promise in the shared conventions that can be modelled holds under model checking in the stated configurations; the rest are listed with the reason | Stateright test run |
| G6 | Mutation testing of the model against the vectors leaves no unexplained surviving mutant | Mutation report |
| G7 | Defect convergence, as restated by ruling 38 of the plan audit: the latest round of cold readers reports no unsafe finding in the language, the shared conventions or the interface; the core and the conformance vectors agree with the promises; and every defect triaged to the language or the shared conventions is closed or declared implementation-defined. It was: two consecutive sweeps find no new defect, which four rounds of reading did not reach | [Defect register](../LANGUAGE_DEFECTS.md) |
| G8 | Every open audit question is closed or declared out of the builders' scope | [Plan audit](../AUDIT.md) |
| G9 | The implementation interface is specified to the level needed for one implementation to work unchanged against both builds | The implementation interface in `spec/` |
| G10 | A pilot run: a fresh session follows the brief for the first milestone only, time-boxed, to test the brief itself. Its work is discarded and that session builds nothing further | Pilot notes |
| G11 | Owner sign-off | Recorded in the plan audit |

### Current assessment of G8, G9 and G10

Coordinator's assessment on 2026-11-11, under ruling 58, point 7; it remains the
coordinator's until G11, the owner's sign-off. The assessment confirmed on 2026-11-06 is
preserved in [ruling 39](../proposals/ruling-39/README.md#the-assessment-point-8-confirmed).
This updates G8 to G10 only and does not declare that the gate passes.

| # | State | Evidence | What would meet it |
| --- | --- | --- | --- |
| G8 | Not met | Q3 now has standard identity forms (IFC-022, IFC-042, PRV-022) and native launch evidence in [A4](../../experiments/009-native-launch/results-2026-11-09.md). Q7's no-frontend case remains open as [A17](../AUDIT.md#action-items). D3/A23 is superseded by the strict loader; D6/A19 was closed by ruling 41. D5/A20 and D8/A24 still have open remainders in the [tracker](../AUDIT.md#waiting-for-the-owner); D5 has a specified gate and trust domain, but the bypass probe A7 has not run. The [probe register](../../ROADMAP.md#experiments-and-probes) records A4 and A8 done, A22 partial, A19 superseded, and A5, A6, A7 and A10 open. Secrets and the diagnostic shape are specified; the full inspection shape beyond S1 is still open. | Close Q3, Q7, D3, D5, D8 and every remaining probe question, or explicitly declare each outside the builders' scope; settle the inspection shape or leave it to each engine explicitly. The owner decides the exclusions. |
| G9 | Not met | The interface and provider contracts specify identity delivery and process startup: DL12 no longer leaves each engine to choose how an implementation is started, and DL13 is narrowed to the standard forms in IFC-022, IFC-042 and PRV-022. The [cold readers' protocol guide](../guides/cold-readers.md#the-protocol) preserves the checks and unconfirmed claims; later interface changes have not had a complete fresh protocol check. Shared test implementations written from spec/ alone are still needed. | Show DL12 and DL13 leave an implementation nothing engine-specific to do; recheck the changed interface against the protocol; write shared test implementations under conformance/ from spec/ alone. |
| G10 | Not met | The build base remains unpublished and no time-boxed pilot is recorded. The [conformance suite](../../conformance/README.md) and the first build exist, but do not constitute the independent pilot. A45 remains open. | Publish build-base with the brief as BUILD.md and the first milestone's vectors; have a fresh session follow only that milestone, time-box it, preserve its notes, discard its work, and use no further work from that session. |


## Comparison

After each milestone the comparer:

1. Runs both implementations' conformance adapters over the public and held-out vectors.
2. Runs generated documents through both language layers and the core and compares
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
