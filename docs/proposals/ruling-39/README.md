# Ruling 39: the open points before the cold read by Codex

Status: in force; amended by 41 (points 1 to 5; logged decisions on composition, `obliges`, `narrows`, re-exports); amended by 42 (R5); amended by 44 (C42, accepted again under point 6); amended by 57 (point 11); amended by 58 (point 6 record notes; point 8 assessment); done (points 8 to 14); amended by 75 (G5: every convention-layer safety promise) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 1. Purpose, scope and principles](../../../IN_FORCE.md#1-purpose-scope-and-principles), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed); [CON-001](../../../spec/promises/conduits.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After ruling 38 the coordinator put fifteen points to the owner: the secret type, the rest of
the list of what waited for the owner, the removal of superseded material, and two questions
of fact. The table gives what was put, the coordinator's recommendation, and the ruling in the
owner's words.

| # | What was put | Recommended | The owner's ruling |
| --- | --- | --- | --- |
| 1 | The core of the [secrets proposal](../secrets/README.md): a secret is a Resource of a core type `secret.v1` owned by a Pool; a document refers to it through `secret-reference.v1`, and a literal in its place is rejected; an implementation reads the value on demand through its own Conduit with `reveal-secret.v1`, and the engine keeps nothing; values stay in existing stores, the engine reading a file or an environment variable, with keychains and cloud managers as packages; the store is read at each request; the proposed names become final. | Accept | "I feel we need more research into how secrets are handled by cloud providers like AWS and in kubernetes before making a determination" |
| 2 | Who may read a secret: only a Resource, and only the root Interface may open a Conduit that reads one, for the first version. | Accept | "Accepted" |
| 3 | Existing servers that take a credential from an environment variable: the engine reads the secret through the instance's own Conduit and sets the variable when it starts the process. | Accept | "Accepted" |
| 4 | An Agent's own provider credential: the same, for an Agent's own process; the value is set in its environment at start and never presented to it as a tool. | Accept | "Accepted" |
| 5 | A composite Conduit at setup: the decision to open is whole, and each component then opens with its own outcome. | Accept | "Accept" |
| 6 | Every decision of the coordinator before and since ruling 37 that is logged, every choice of an editor that is logged, and the three corrections to the interface that the first check against the Model Context Protocol set aside, are accepted as of this ruling. Codex, or any later reader, may still challenge any of them. | Accept | "Accept" |
| 7 | "The grant is an object", in the purpose, becomes "the Conduit is an object", noted as amended. | Accept | "Accept" |
| 8 | The coordinator writes an assessed state for each readiness condition G1 to G11, which the owner confirms on the pull request. | Accept | "Accept" |
| 9 | Superseded material leaves the tree, and every link to it is pinned to the merge commit on `main`: `docs/protocol/`, `docs/authoring/`, `docs/proposals/authoring-syntax/`, `docs/REVIEW.md`, `docs/VOCABULARY.md`, the three proposals in `docs/implementation/` written before the rulings, and experiments 002 and 003. | Accept | "Accept" |
| 10 | The historical sections of `docs/AUDIT.md` move to one file in the archive. | Accept | "Accept" |
| 11 | The rule against editing the archive and the research is lifted once, to remove the names of the two private repositories. | Accept | "Accept" |
| 12 | The stale remote branches are deleted once nothing unrecorded is on them, and `docs/plan-audit` once the links that pin its commits are repinned. | Accept | "Accept" |
| 13 | The personal address on the merge commit of pull request 6 stays, since changing it means rewriting `main`; the owner turns on GitHub's private-email setting for later merges. | Accept | "Accept" |
| 14 | Was the external audit of 2026-11-05 made by a Claude session, at the owner's request, that had not written the plans? | — | "Yes, just strike" |
| 15 | May the rows of the ruling log be labelled as the coordinator's summaries of the owner's messages, with the owner's words quoted where marked? | — | "Yes" |

## What follows from each

1. The secret type is not decided. A research packet on how secrets are handled by AWS, by
   Kubernetes and by the systems around them is written, and the proposal is revised from it,
   keeping points 2 to 4 unless the research gives a reason not to, which it then states. The
   owner rules on the revised proposal. Until then the specification says nothing of secrets.
2. to 4. Accepted as the direction for the revised proposal. They are written into the
   specification with the rest of the secret type, once point 1 is ruled.
5. The composite Conduit leaves the list of what waits for the owner.
6. The specification's standing becomes one sentence: the owner accepted it as it stands at
   ruling 39. Every record that says a decision is the coordinator's and can be struck says it
   was accepted by ruling 39. The list of what waits for the owner loses those rows.
7. The purpose section is amended, and the later amendments say so.
8. The tracker gains a state for each condition, which the owner confirms on the pull request.
9. The files are removed. Every link to them points at commit `e271d1c` on `main`. The archive
   index lists each removed path with a link to its last version.
10. The tracker keeps what is current. The rest moves to the archive, with a link.
11. The names are removed. The rule stands for every other edit.
12. The branches are deleted after their content is checked.
13. Nothing changes in the repository.
14. The sentence is struck.
15. The ruling log says so above its first row.

## After the ruling

Two further answers of the owner, on 2026-11-06:

| What was put | The owner's answer |
| --- | --- |
| Confirm the coordinator's assessed state of each readiness condition (point 8): none met; G1 and G3 partly met; the other nine not met. | "Confirm" |
| The name "Sealmesh" in an archived collaborator proposal, which links a public repository of that name: leave it, as it is not one of the private names that point 11 removed. | "Agree, leave it" |

The owner will rule on the secret type once the research of point 1 is finished.

## The assessment point 8 confirmed

This is the coordinator's assessment of each condition, written under
[ruling 39](README.md), point 8. The owner confirmed it on 2026-11-06
([record](README.md#after-the-ruling)). A condition is met only where its
own words are satisfied. It is partly met where some of what it requires is satisfied in full
and the rest is not, and otherwise it is not met. Each state is of the repository as it stood
on that day.

None of the eleven is met. G1 and G3 are partly met, and the other nine are not met. The gate
does not pass, and the brief is not issued.

The assessment was made and confirmed before rulings 40 and 41, of the same day, and is kept
as the owner confirmed it. It has not been made again since the specification was rewritten
under those rulings. These changes since bear on the rows below: the secret type is ruled and
written, in [secrets.yaml](../../../spec/promises/secrets.yaml), which settles point 1 of ruling
39 (G1, G2, G8, G9); `spec/` holds 455 promises, 44 of them in
[interface/implementation.yaml](../../../spec/interface/implementation.yaml) (G2, G9); a
diagnostic is SARIF's `result`, reduced, and a condition and a record have core schemas, so
only the full shape of an inspection result is left to the vectors (G2, G8); ruling 41 closed
action A19, and with it E8 and D6 (G8); a runtime provider runs an implementation and its run
returns the URL at which it is reached, or the engine's own runner reaches it over stdio
(PRV-007), so DL12 no longer leaves to each engine who starts a process or over which transport
it is reached (G9); the schemas cover `from` and two nested instantiations, not three
(G3); the experiment program has ten scenarios, S1 to S10 (G3); of the 87 defects in the
language and the shared conventions, 13 are closed and 74 are ruled (G7); and
`engine/` exists and holds the strict loader in Rust (A43), increment 1 of the first
build, though no pure core yet (G3, G4).

[Round 5](../reaudit/round-5/README.md) of the re-audit read the rewrite at commit
`d0c425f`. Its readers reported ten unsafe findings. All ten are fixed, so no unsafe finding
is open. G7 still asks that the latest round report none, and the text that round 5 changed,
101 promises changed and 3 added, has not been read cold (G7). Its reader 6 checked the
interface against the published protocol. The round then changed 22 of the interface's
promises and added 2, and the records do not say that these were checked again (G9). Round 5
also removed DL11, so the implementation-defined list holds 15 items (G2).

[Round 6](../reaudit/round-6/README.md), the owner's cold read by Codex, read the
whole repository at `c4d7d33`. It reported six findings, two of them wrong and safety-critical,
conflicts between a specific procedure and a general safety promise that round 5 would have
classed as unsafe. All six are fixed, under the coordinator's decisions that
[ruling 44](../ruling-44/README.md) accepted, so no such finding is open; but G7 asks
that the latest round report none, and round 6 did not (G7). Its finding 01 checked the
protocol's error -32021 against the published protocol; the interface promises that read it,
and those that present a tool's definition, changed, and IFC-047 was added (G9). Ruling 44 makes the next
milestone a first build by the language side: the S1 scenario through the core,
the conformance vectors for the safety cases first, and the first providers (G2, G4). That
build is not one of the two independent builds, which still wait for the gate.

| # | State | Evidence | What would meet it |
| --- | --- | --- | --- |
| G1 | Partly met | The [contents list](../../../spec/README.md#contents) names every file under `spec/` other than `spec/README.md`, the file that holds the list: each of the others lies under a path the list gives. Nothing outside `spec/` binds ([spec/README.md](../../../spec/README.md); decision DA1 of the [documents audit](../docs-audit/README.md)), and ruling 39, point 9, removed the superseded material from the tree. Not satisfied in full: behavior the owner accepted on secrets is stated only outside `spec/`. The older text's rule that secrets never appear in configuration, records or events is GP3 of the [conversion list](../docs-audit/GAPS.md), still unconverted; [ruling 38](../ruling-38/README.md), point 1, ruled that a secret is referred to and never written as a literal; and points 2 to 4 of ruling 39 are accepted as a direction that is written into `spec/` only once point 1 is ruled. Action A32, whose done-when is this condition, is in progress for that reason ([action items](../../AUDIT.md#action-items)). | The secret type ruled, after the research that ruling 39, point 1, asks for ([proposal](../secrets/README.md)), and written into `spec/` with GP3, ruling 38, point 1, and ruling 39, points 2 to 4, with the contents list naming any file it adds. Or the owner reads behavior that is accepted and not yet written as a gap in `spec/`, as `spec/README.md` reads any missing behavior, rather than as normative text outside it; G1 is then met as it stands. |
| G2 | Not met | `spec/` holds 455 promises ([counts](../../../spec/README.md#contents)), and every one has an empty `vectors` list: `conformance/` does not exist, and action A39 is open. No coverage report exists. Nor does the register cover the language and the shared conventions in full: it says nothing of secrets (G1), and it fixes no shape of a diagnostic beyond its `message` ([`diagnostic.v1`](../../../spec/core/results.yaml)) and not the full shape of an inspection result, both left to be settled with the vectors ([known to be open](../../AUDIT.md#for-the-next-reader)). | The conformance vectors (A39), with each promise citing at least one vector or listed on the [implementation-defined list](../../../spec/implementation-defined.yaml); a coverage report by promise and layer; the secret type written (G1); and the shapes of a diagnostic and of an inspection result stated, or left to each engine on that list. |
| G3 | Partly met | The [schemas](../../../spec/schemas/README.md) cover the envelope, the template and instance documents of each family, a package's manifest, each of the five kinds of contract, and instantiation: `from` and the three nested instantiations. `tools/check_schemas.py`, run for this assessment, validated the core package and the examples against them with no error. Not satisfied: no schema of a vector exists, since the vectors' format is not defined; no Rust loader exists (A43, its loader open); and none of the scenarios S1 to S9 of the [experiment program](../../AUDIT.md#experiment-program) has its documents written. | The vectors' format and its schema (A39); a strict loader in Rust (A43); and the documents of S1 to S9, each validated by that loader, with the negative cases rejected. |
| G4 | Not met | No core exists: `engine/` does not exist, action A38 is open, and there is no vector for a model to pass. | The core in Rust for the language layer and the shared conventions (A38), passing every vector (G2) with no unhandled case. |
| G5 | Not met | [Experiment 004](../../../experiments/004-stateright-evaluation/README.md) evaluated Stateright on the at-most-once rules as the older text stated them. It evaluated the tool: the promises themselves are not model-checked (actions A13 and A40). No list exists of the safety promises of the conventions, of the configurations to check them in, or of those that cannot be modelled. | A40: the safety promises of the seven conventions listed; each one that can be modelled checked with Stateright in stated configurations, in the ordinary test run; and the rest listed with the reason. |
| G6 | Not met | No mutation run is possible: neither the core nor the vectors exist. Action A44 is open. | A44, once A38 and A39 are done: a mutation report in which every surviving mutant is explained. |
| G7 | Not met | None of its three parts holds. The latest round of cold readers to report, round 4 of the [re-audit](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/README.md), reported 3 unsafe findings in `spec/`. All three are fixed ([decisions](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/DECISIONS.md)), but the condition asks that the round report none, and none of the four rounds was clean. What the fix pass of round 4 changed (92 promises changed, 5 added) and what ruling 38 changed have not been read cold. Neither a core nor a vector exists to agree with the promises. Of the 87 defects in the [register](../../LANGUAGE_DEFECTS.md#triage-by-layer) triaged to the language or the shared conventions, 3 are closed and 84 are ruled: written as promises with no vector, which the register does not count as closed ([status values](../../LANGUAGE_DEFECTS.md#status-values)). | A round of cold readers, the latest at the time, that reports no unsafe finding in the language, the shared conventions or the interface, whether a further round of the re-audit or the owner's cold read by Codex; the vectors and the core (G2, G4) agreeing with the promises; and each of the 84 ruled defects covered by a vector, which closes it, or declared implementation-defined. |
| G8 | Not met | Of the plan audit's [findings](../../archive/plan-audit-history.md#findings), questions Q3, how a native Agent proves who it is (A11, which waits for the native probe A4), and Q7, what runs with no frontend attached (A17), are open, and so are disagreements D5, D6 and D8 (A20, A19, A24). D3 is still marked open, though A23, which it names, is superseded by ruling 39. Each of the [standing probes](../../archive/plan-audit-history.md#standing-probes-and-comparisons) asks a question, and eight are open: E1 to E5, E8, E10 and E11. The tracker also lists as [known to be open](../../AUDIT.md#for-the-next-reader) the shape of a diagnostic beyond its `message` and the full shape of an inspection result, both of which a build produces (G2). None of these is declared out of the builders' scope. The secret type is open under ruling 39, point 1. | The owner closes Q3, Q7, D3, D5, D6, D8 and the eight open probes, or declares each out of the builders' scope; the two shapes are settled (G2); and the owner rules on the secret type. |
| G9 | Not met | The interface is written: [35 promises](../../../spec/interface/implementation.yaml) on the Model Context Protocol at the baseline revision 2026-07-28 (ruling 31), checked twice against the published protocol. That it is enough for one implementation to work unchanged against two builds is not shown, and parts an implementation depends on are open. Each engine chooses the transport and who starts an implementation's process (DL12 of the [implementation-defined list](../../../spec/implementation-defined.yaml)), and what a consumer presents to show which instance it is (DL13), which an implementation's onward requests go through (IFC-015, IFC-022). How a server or an Agent is given a credential is accepted as direction by ruling 39, points 3 and 4, and not written. What the fix pass of round 4 and ruling 38 changed in the interface has not been checked against the protocol again, and the [second check](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md) left 3 claims unconfirmed. The answer to an unknown tool is to be looked at again (ruling 38, point 11). No test implementation exists (A12). | The secret type written, with how an implementation's process is given a credential; DL12 and DL13 shown to leave an implementation nothing engine-specific to do, or narrowed until they do; the changed interface text checked against the protocol again; and the test implementations under `conformance/` written from `spec/` alone. |
| G10 | Not met | No pilot run has been made. The build base is not published: no `build-base` branch exists, locally or on the remote. `conformance/`, whose README the [brief](../../implementation/ENGINE_BUILD_BRIEF.md) relies on for the order of the milestones, does not exist, so there is no first milestone to pilot. Action A45 is open. | The build base published, with the brief as `BUILD.md` and `conformance/` holding at least the first milestone; then a fresh session follows the brief for that milestone alone, time-boxed, its notes are kept, and its work is discarded. |
| G11 | Not met | No sign-off on the gate is recorded in the [ruling log](../../AUDIT.md#ruling-log). Confirming these states under ruling 39, point 8, is not a sign-off on the gate. | G1 to G10 met, then the owner's sign-off, recorded as a ruling in the plan audit. |
