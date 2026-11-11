# Ruling 47 — Organization and publication after v0

Status: in force; amended by 65 (point 1: public name); directive (A75, A76: namespace after organization); amended by 68 (point 2: safety vectors defined) (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), [IN_FORCE.md: 14. Publication and organization](../../../IN_FORCE.md#14-publication-and-organization). This record says why.

The
owner confirmed the direction on 2026-11-10, settled four points of the coordinator's review and
accepted the list of required experiments, recorded under
[the review](#the-review-with-the-owner); one point, the public name and namespace, stays open:
the owner has not chosen the public name, and the namespace waits for a domain. Settling them is
the plan's last step. This record governs project organization and release planning. The
[specification](../../../spec/README.md) governs engine behavior.

## When to publish

The owner set the condition for starting the transition in these words.

> We need to prove to ourselves with a v0 of the engine that passes all of our experiments that the system works and is worth publishing

Continue development and testing in the current repository. Bring the owner the v0 revision
and the recorded results for the whole adopted [experiment program](../../../ROADMAP.md#the-experiment-program).
Use its existing scenarios, methods and [probe register](../../../ROADMAP.md#experiments-and-probes).
For each experiment with a pass condition, show that it passed. For observational probes,
link the dated observations and explain what they establish or call into question. Account for
missing runs and failures. An outstanding required run or an unmet pass condition keeps the
transition on hold. The owner judges from the complete evidence whether Conduit works and is
worth publishing. Passing the first build alone, or obtaining sign-off to start the independent
builds, cannot establish that judgment.

**v0 is the reference engine**, the one the first build grows into (A50). The two independent
builds of step 7 test whether the specification is clear enough to build from; their results
inform the owner's judgment and do not gate it.

**The experiments that must pass**, a list the coordinator drafted and the owner accepted on
2026-11-10:

| Required to pass | What passing means |
| --- | --- |
| S1 to S10 on the reference engine | Each scenario runs end to end with its vectors passing |
| S1 and S2 with real Agents | Claude Code and Codex, run by the engine as Agents, carry out S1's and S2's tasks through it, and each request has its record. Simulated clients alone do not pass v0 |
| The safety vectors | Every safety vector passes |
| The author measurements against the baseline (A70) | Recorded for the coordinator and for the fresh Codex author (ruling 48, point 4), and compared task by task. Conduit passes when it takes less of an author's work than the baseline on most of S1's tasks and much more on none; the counting rules set after the baseline ran are reported apart, so the comparison can be read by either set |
| Model checking of the safety promises (step 6, A40) | The safety promises hold under loss, reordering and partition |
| Subscription terms (A8) | The current terms allow the engine to run the native clients headless on a subscription, or v0 names the alternative it uses |

The other probes are observational and do not block: A4, A5, A6, A10, A22, A71, A73 and A74 are
reported with their dated observations, and with what they establish or call into question.
A7 does not block either, but every route around the engine that it finds is written into
v0's public security claim.

## What the organization owns

Conduit is the language's working name; its public name is open with the namespace
([the review](#the-review-with-the-owner), point 1). Independent open-source stewardship is the
direction. The project keeps its Apache-2.0 license.

| Home | Responsibility |
| --- | --- |
| Main Conduit repository | Language specification, package format and contracts, conformance cases, the engine and its CLI, and the project roadmap. Keep the site and documentation source here initially so a contract change and its documentation can be reviewed together. Rust crates can share this repository. |
| Official integration repositories | Each independently maintained and released integration has its own source, issues, compatibility tests, documentation and releases under the project organization. Codex and Claude are the first candidates: the Agent types of the first build's increment 3,
built from the start as self-contained packages so that each can move to a repository of its
own. The main repository keeps a pinned copy of each for its conformance vectors. Each can ship
on its own schedule. |
| Organization's public `.github` repository | Common contribution guidance, support and security reporting contacts, and issue templates. |
| Community repositories | Authors own, host and release their packages. They use the same documented format and contracts as official integrations and choose their implementation language. |

The owner has final authority over language direction, designates official packages and
appoints their maintainers. An official package names its maintainer and support contact,
documents which engine versions it supports, and tests installation and use of the artifact
it distributes against a released engine. Its maintainer can ship routine fixes. A shared
contract change goes through the main repo. If maintenance lapses, the owner appoints a
successor or marks the package unmaintained wherever the project lists it.

## Actions after a decision to publish

| Who | Action | Completion evidence |
| --- | --- | --- |
| Project owner | Select the organization handle, arrange recovery access and transfer the existing repository with its history, issues and pull requests. Delegate package release permissions to the appointed maintainers. | The organization owns the main repo and the release responsibilities are visible. |
| Project owner and main repo maintainers | Prefer a project-controlled domain for public identifiers. If there is no domain by preview, the owner explicitly decides whether to commit to an organization-based namespace before external users adopt it. Exact handles and package publishing names can wait. | The owner records the namespace choice. Apply the versioning rule for names (ruling 35) and use `tools/set_names.py` for the [specification names](../../../spec/names.yaml). Update affected docs and examples and document how to migrate configurations using the earlier names. |
| Integration maintainer | Move the first qualifying integration into its own repo and publish a versioned archive. Run its install-and-use test from outside the main repo. Repeat for the next integration when it qualifies. | An external user can fetch the documented artifact, verify its digest and use it with the supported runtime. A package fix can ship independently. |
| Main repo and integration maintainers | Publish the standalone engine and CLI and one website, with equally usable entry points for engine users and package authors. List official packages, maintainers and support links. | From clean environments a user can run the released runtime with an official integration, and an outside author can build, package, test and use an integration through the public contracts. Include a non-Rust implementation in the author check, building on [scenario S7](../../../ROADMAP.md#scenarios). |

**The first public preview needs one released engine and CLI and one independently released official
integration.** Either candidate can qualify first. The two audiences both need a working path
at preview. The prototype's in-tree Rust provider convention continues through validation.
The published package format is independent of both repository layout and implementation language.

The initial distribution plan uses versioned release assets over HTTPS and a small index on
the project website. The package promises already describe fetching, exact versions and archive
digests. Conduit installs no optional integration packages by default. Users should be able to
select a prepared flow and fetch its predetermined set of packages together. Selection and
curation of examples come later. [Package promises](../../../spec/promises/packages.yaml)

## Precedents used in the plan

Pi keeps multiple packages in one workspace. Docker develops Buildx as a separately released
CLI integration. Conduit can keep components that evolve with the language in the main repo
and give an integration its own maintainer and release cycle. GitHub supports transferring the existing repo and sharing
community files across the organization. [Pi workspace](https://github.com/earendil-works/pi/blob/main/package.json) ·
[Docker Buildx](https://github.com/docker/buildx) ·
[Repository transfers](https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository) ·
[Organization defaults](https://docs.github.com/en/communities/setting-up-your-project-for-healthy-contributions/creating-a-default-community-health-file)

## The review with the owner

The coordinator reviewed this record on 2026-11-10, and the owner answered.

| # | Point | Answer |
| --- | --- | --- |
| 1 | The public name and namespace. The owner has not chosen the public name. Today `spec/names.yaml` names the core package `github.com/shrav-k/conduit/core`, `apiVersion` `github.com/shrav-k/conduit/v0` and the prefix of `_meta` keys `io.github.shrav-k.conduit/`, all on the owner's account; a move to an organization changes each, and only a new version may (ruling 35). The vectors and the stubs already hold them many times, so the change costs more the later it comes | **Open.** The owner has not chosen the public name and cannot yet procure a domain. When they are chosen, the names change by `tools/set_names.py` as the last step of this plan, and before any outside user adopts them |
| 2 | Which engine is v0, and which experiments must pass | The reference engine; the independent builds inform and do not gate. The required experiments are the named list above, which the owner accepted on 2026-11-10 with three additions: a bar for the author measurements, S1 and S2 with real Agents, and A8; A7's findings go into the security claim |
| 3 | "Runtime" in this record meant the engine and its CLI, though the vocabulary gives the word to a runtime provider | Written as "the engine and its CLI" |
| 4 | How the first official integrations relate to the main repository | The Agent types for Claude and Codex are built as self-contained packages from increment 3 on, with a pinned copy kept for the conformance vectors |
| 5 | A future company's involvement, in a public record | Dropped: "independent open-source stewardship" alone |
