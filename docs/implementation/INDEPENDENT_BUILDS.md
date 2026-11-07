# Independent builds

Status: the roles and independence rules were accepted by the owner on 2026-11-05, and the scope
below follows layer decision LD4 of the same day. **The builds have not started and must not
start until the [readiness gate](#readiness-gate) passes.** The specification exists as a draft
in [spec/](../../spec/README.md), and its [form](../proposals/normative-surface/README.md) is
ruled. `conformance/`, `reference-model/` and the branches named below do not exist yet; their
names are proposals.

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
| Language side | The owner, with the sessions that have been writing the language | Everything | The specification, the reference model, the conformance vectors |
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
| `reference-model/` | The executable reference model in Rust | No | No |
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
| G4 | The reference model for the language layer and the shared conventions passes every vector with no unhandled case | Model test run |
| G5 | Every safety promise in the shared conventions that can be modelled holds under model checking in the stated configurations; the rest are listed with the reason | Stateright test run |
| G6 | Mutation testing of the model against the vectors leaves no unexplained surviving mutant | Mutation report |
| G7 | Defect convergence, as restated by ruling 38 of the plan audit: the latest round of cold readers reports no unsafe finding in the language, the shared conventions or the interface; the reference model and the conformance vectors agree with the promises; and every defect triaged to the language or the shared conventions is closed or declared implementation-defined. It was: two consecutive sweeps find no new defect, which four rounds of reading did not reach | [Defect register](../LANGUAGE_DEFECTS.md) |
| G8 | Every open audit question is closed or declared out of the builders' scope | [Plan audit](../AUDIT.md) |
| G9 | The implementation interface is specified to the level needed for one implementation to work unchanged against both builds | The implementation interface in `spec/` |
| G10 | A pilot run: a fresh session follows the brief for the first milestone only, time-boxed, to test the brief itself. Its work is discarded and that session builds nothing further | Pilot notes |
| G11 | Owner sign-off | Recorded in the plan audit |

### State on 2026-11-06

This is the coordinator's assessment of each condition, written under
[ruling 39](../proposals/ruling-39/README.md), point 8. The owner confirmed it on 2026-11-06
([record](../proposals/ruling-39/README.md#after-the-ruling)). A condition is met only where its
own words are satisfied. It is partly met where some of what it requires is satisfied in full
and the rest is not, and otherwise it is not met. Each state is of the repository as it stood
on that day.

None of the eleven is met. G1 and G3 are partly met, and the other nine are not met. The gate
does not pass, and the brief is not issued.

The assessment was made and confirmed before rulings 40 and 41, of the same day, and is kept
as the owner confirmed it. It has not been made again since the specification was rewritten
under those rulings. These changes since bear on the rows below: the secret type is ruled and
written, in [secrets.yaml](../../spec/promises/secrets.yaml), which settles point 1 of ruling
39 (G1, G2, G8, G9); `spec/` holds 453 promises, 44 of them in
[interface/implementation.yaml](../../spec/interface/implementation.yaml) (G2, G9); a
diagnostic is SARIF's `result`, reduced, and a condition and a record have core schemas, so
only the full shape of an inspection result is left to the vectors (G2, G8); ruling 41 closed
action A19, and with it E8 and D6 (G8); a runtime provider runs an implementation and its run
returns the URL at which it is reached, or the engine's own runner reaches it over stdio
(PRV-007), so DL12 no longer leaves to each engine who starts a process or over which transport
it is reached (G9); the schemas cover `from` and two nested instantiations, not three
(G3); the experiment program has ten scenarios, S1 to S10 (G3); of the 87 defects in the
language and the shared conventions, 13 are closed and 74 are ruled (G7).

[Round 5](../proposals/reaudit/round-5/README.md) of the re-audit read the rewrite at commit
`d0c425f`. Its readers reported ten unsafe findings. All ten are fixed, so no unsafe finding
is open. G7 still asks that the latest round report none, and the text that round 5 changed,
101 promises changed and 3 added, has not been read cold (G7). Its reader 6 checked the
interface against the published protocol. The round then changed 22 of the interface's
promises and added 2, and the records do not say that these were checked again (G9). Round 5
also removed DL11, so the implementation-defined list holds 15 items (G2).

[Round 6](../proposals/reaudit/round-6/README.md), the owner's cold read by Codex, read the
whole repository at `c4d7d33`. It reported six findings, two of them wrong and safety-critical,
conflicts between a specific procedure and a general safety promise that round 5 would have
classed as unsafe. All six are fixed, under the coordinator's decisions that
[ruling 44](../proposals/ruling-44/README.md) accepted, so no such finding is open; but G7 asks
that the latest round report none, and round 6 did not (G7). Its finding 01 checked the
protocol's error -32021 against the published protocol; the interface promises that read it,
and those that present a tool's definition, changed, and IFC-047 was added (G9). Ruling 44 makes the next
milestone a first build by the language side: the S1 scenario through the reference model,
the conformance vectors for the safety cases first, and the first providers (G2, G4). That
build is not one of the two independent builds, which still wait for the gate.

| # | State | Evidence | What would meet it |
| --- | --- | --- | --- |
| G1 | Partly met | The [contents list](../../spec/README.md#contents) names every file under `spec/` other than `spec/README.md`, the file that holds the list: each of the others lies under a path the list gives. Nothing outside `spec/` binds ([spec/README.md](../../spec/README.md); decision DA1 of the [documents audit](../proposals/docs-audit/README.md)), and ruling 39, point 9, removed the superseded material from the tree. Not satisfied in full: behavior the owner accepted on secrets is stated only outside `spec/`. The older text's rule that secrets never appear in configuration, records or events is GP3 of the [conversion list](../proposals/docs-audit/GAPS.md), still unconverted; [ruling 38](../proposals/ruling-38/README.md), point 1, ruled that a secret is referred to and never written as a literal; and points 2 to 4 of ruling 39 are accepted as a direction that is written into `spec/` only once point 1 is ruled. Action A32, whose done-when is this condition, is in progress for that reason ([action items](../AUDIT.md#action-items)). | The secret type ruled, after the research that ruling 39, point 1, asks for ([proposal](../proposals/secrets/README.md)), and written into `spec/` with GP3, ruling 38, point 1, and ruling 39, points 2 to 4, with the contents list naming any file it adds. Or the owner reads behavior that is accepted and not yet written as a gap in `spec/`, as `spec/README.md` reads any missing behavior, rather than as normative text outside it; G1 is then met as it stands. |
| G2 | Not met | `spec/` holds 453 promises ([counts](../../spec/README.md#contents)), and every one has an empty `vectors` list: `conformance/` does not exist, and action A39 is open. No coverage report exists. Nor does the register cover the language and the shared conventions in full: it says nothing of secrets (G1), and it fixes no shape of a diagnostic beyond its `message` ([`diagnostic.v1`](../../spec/core/results.yaml)) and not the full shape of an inspection result, both left to be settled with the vectors ([known to be open](../AUDIT.md#for-the-next-reader)). | The conformance vectors (A39), with each promise citing at least one vector or listed on the [implementation-defined list](../../spec/implementation-defined.yaml); a coverage report by promise and layer; the secret type written (G1); and the shapes of a diagnostic and of an inspection result stated, or left to each engine on that list. |
| G3 | Partly met | The [schemas](../../spec/schemas/README.md) cover the envelope, the template and instance documents of each family, a package's manifest, each of the five kinds of contract, and instantiation: `from` and the three nested instantiations. `tools/check_schemas.py`, run for this assessment, validated the core package and the examples against them with no error. Not satisfied: no schema of a vector exists, since the vectors' format is not defined; no Rust loader exists (A43, its loader open); and none of the scenarios S1 to S9 of the [experiment program](../AUDIT.md#experiment-program) has its documents written. | The vectors' format and its schema (A39); a strict loader in Rust (A43); and the documents of S1 to S9, each validated by that loader, with the negative cases rejected. |
| G4 | Not met | No reference model exists: `reference-model/` does not exist, action A38 is open, and there is no vector for a model to pass. | The reference model in Rust for the language layer and the shared conventions (A38), passing every vector (G2) with no unhandled case. |
| G5 | Not met | [Experiment 004](../../experiments/004-stateright-evaluation/README.md) evaluated Stateright on the at-most-once rules as the older text stated them. It evaluated the tool: the promises themselves are not model-checked (actions A13 and A40). No list exists of the safety promises of the conventions, of the configurations to check them in, or of those that cannot be modelled. | A40: the safety promises of the seven conventions listed; each one that can be modelled checked with Stateright in stated configurations, in the ordinary test run; and the rest listed with the reason. |
| G6 | Not met | No mutation run is possible: neither the reference model nor the vectors exist. Action A44 is open. | A44, once A38 and A39 are done: a mutation report in which every surviving mutant is explained. |
| G7 | Not met | None of its three parts holds. The latest round of cold readers to report, round 4 of the [re-audit](../proposals/reaudit/round-4/README.md), reported 3 unsafe findings in `spec/`. All three are fixed ([decisions](../proposals/reaudit/round-4/DECISIONS.md)), but the condition asks that the round report none, and none of the four rounds was clean. What the fix pass of round 4 changed (92 promises changed, 5 added) and what ruling 38 changed have not been read cold. Neither a reference model nor a vector exists to agree with the promises. Of the 87 defects in the [register](../LANGUAGE_DEFECTS.md#triage-by-layer) triaged to the language or the shared conventions, 3 are closed and 84 are ruled: written as promises with no vector, which the register does not count as closed ([status values](../LANGUAGE_DEFECTS.md#status-values)). | A round of cold readers, the latest at the time, that reports no unsafe finding in the language, the shared conventions or the interface, whether a further round of the re-audit or the owner's cold read by Codex; the vectors and the reference model (G2, G4) agreeing with the promises; and each of the 84 ruled defects covered by a vector, which closes it, or declared implementation-defined. |
| G8 | Not met | Of the plan audit's [findings](../archive/plan-audit-history.md#findings), questions Q3, how a native Agent proves who it is (A11, which waits for the native probe A4), and Q7, what runs with no frontend attached (A17), are open, and so are disagreements D5, D6 and D8 (A20, A19, A24). D3 is still marked open, though A23, which it names, is superseded by ruling 39. Each of the [standing probes](../archive/plan-audit-history.md#standing-probes-and-comparisons) asks a question, and eight are open: E1 to E5, E8, E10 and E11. The tracker also lists as [known to be open](../AUDIT.md#for-the-next-reader) the shape of a diagnostic beyond its `message` and the full shape of an inspection result, both of which a build produces (G2). None of these is declared out of the builders' scope. The secret type is open under ruling 39, point 1. | The owner closes Q3, Q7, D3, D5, D6, D8 and the eight open probes, or declares each out of the builders' scope; the two shapes are settled (G2); and the owner rules on the secret type. |
| G9 | Not met | The interface is written: [35 promises](../../spec/interface/implementation.yaml) on the Model Context Protocol at the baseline revision 2026-07-28 (ruling 31), checked twice against the published protocol. That it is enough for one implementation to work unchanged against two builds is not shown, and parts an implementation depends on are open. Each engine chooses the transport and who starts an implementation's process (DL12 of the [implementation-defined list](../../spec/implementation-defined.yaml)), and what a consumer presents to show which instance it is (DL13), which an implementation's onward requests go through (IFC-015, IFC-022). How a server or an Agent is given a credential is accepted as direction by ruling 39, points 3 and 4, and not written. What the fix pass of round 4 and ruling 38 changed in the interface has not been checked against the protocol again, and the [second check](../proposals/reaudit/round-4/mcp-recheck.md) left 3 claims unconfirmed. The answer to an unknown tool is to be looked at again (ruling 38, point 11). No test implementation exists (A12). | The secret type written, with how an implementation's process is given a credential; DL12 and DL13 shown to leave an implementation nothing engine-specific to do, or narrowed until they do; the changed interface text checked against the protocol again; and the test implementations under `conformance/` written from `spec/` alone. |
| G10 | Not met | No pilot run has been made. The build base is not published: no `build-base` branch exists, locally or on the remote. `conformance/`, whose README the [brief](ENGINE_BUILD_BRIEF.md) relies on for the order of the milestones, does not exist, so there is no first milestone to pilot. Action A45 is open. | The build base published, with the brief as `BUILD.md` and `conformance/` holding at least the first milestone; then a fresh session follows the brief for that milestone alone, time-boxed, its notes are kept, and its work is discarded. |
| G11 | Not met | No sign-off on the gate is recorded in the [ruling log](../AUDIT.md#ruling-log). Confirming these states under ruling 39, point 8, is not a sign-off on the gate. | G1 to G10 met, then the owner's sign-off, recorded as a ruling in the plan audit. |

## Comparison

After each milestone the comparer:

1. Runs both implementations' conformance adapters over the public and held-out vectors.
2. Runs generated documents through both language layers and the reference model and compares
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
