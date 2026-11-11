# Ruling 38: the open points and the coordinator's least certain decisions

Status: in force; done (points 2, 14); amended by 41 (points 1, 3, 7, 8, 11, 12); amended by 42 (N1) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 2. The specification: form, layers and conformance](../../../IN_FORCE.md#2-the-specification-form-layers-and-conformance), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed); [ACC-019](../../../spec/conventions/access.yaml). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

After round 4 of the [re-audit](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/README.md) the coordinator put fourteen
points to the owner: five that were still open, six decisions of its own that it was least
sure of, two groups to confirm, and what to do before the work is merged. The owner ruled on
each. The table gives what was put, the coordinator's recommendation, and the ruling in the
owner's words.

| # | What was put | Recommended | The owner's ruling |
| --- | --- | --- | --- |
| 1 | How a value is marked as secret. A: the schema keyword `writeOnly: true`, with the engine keeping such a value out of inspection results, records, diagnostics and events. B: a marked property may hold only a reference to a secret Resource, and a literal is rejected, which needs a secret type in the core. | A | "B I think we need to explore a secret type as this will be extremely important for many areas including access and authentication" |
| 2 | Six behaviors of the older design that the specification does not carry: confirm each as dropped. | Confirm | "Confirm" |
| 3 | A composite Conduit opens whole or not at all at setup; afterwards its components are independent Conduits, and one can be opened or closed alone. Keep that, and fix the wording that calls the setup step one invocation. | Keep, fix the wording | "Confirm and fix wording" |
| 4 | The gate of two clean rounds of cold reading looks out of reach by reading alone. Replace it with: no unsafe finding in the latest round, and convergence judged on the reference model and the conformance vectors. | Replace | "Understood, make this clear to codex in the audit file" |
| 5 | Ruling 31 required a domain to be named before the engine builds. Drop that, since the names come from the repository and one command changes them. | Drop | "Drop" |
| 6 | Later work. A: keep the mechanism of rounds 2 to 4 (`start-later-work.v1`, the chain of Conduits). B: defer later work from v0, so that no operation arranges work that outlasts its invocation, which also drops scheduled starts of model work. | A, weakly | "B, I don't think this is necessary for v0, drop it for now" |
| 7 | A policy on an actor makes its requests to `management`, `access` or a Pool report unavailable. A: keep that, and have `apply-policy.v1` warn when a policy will have this effect. B: policies never apply to targets that the engine implements. | A | "A" |
| 8 | Handing an engine a directory: the coordinator had left it to each engine whether the manifest's packages are installed first. Reverse that: it never installs, missing packages are reported, and `install.v1` is always its own request. | Reverse | "Accepted" |
| 9 | A retired consumer's Conduits stay open until its shutdown is confirmed, and permit only the steps of its own retirement. | Keep | "Accepted" |
| 10 | A step and an onward request may name an object by the resolved form `{kind, id}`; a request that an actor makes itself names it by reference only, as ruling 37 has it. | Keep | "Accepted" |
| 11 | The interface follows the Model Context Protocol on errors: an unknown tool is the protocol's error, and a result that carries none of its declared outputs is an error result. A model may then never see a refusal, and an open proposal to the protocol (SEP-2145) would reverse the first. | Keep, and look again when the protocol settles | "Ok, note this for codex to look at in the audit" |
| 12 | Eight changes to confirm, each of which changed something the owner had accepted or that an author sees: three conformance levels, with Implementation, and "Host" renamed "Engine"; a control's `changes` as a map from an input's name to a pointer; an operation identity scoped also to the instance that sent the request; unavailable standing first where the decision on access cannot be made; no `optional` on a runtime; a rename advancing the revision, and a reference stored as `{kind, id}`; a template needing an implementation for every operation its type offers; and the defaults of a schema's properties being filled in every supplied value before it is checked. | Confirm | "Accepted" |
| 13 | The coordinator's two readings of the owner's instructions: that a finding goes back to the owner only where settling it would change what Conduit is for, and that the work is committed and pushed to pull request 6 at each milestone. | Confirm | "Accepted" |
| 14 | Before merging: the commits of the branch carry addresses formed from a machine's name and a link to a session, and two private repositories are named in the documents. Squash the pull request when it is merged, and remove the two names. | Squash, and remove the names | "Squash merge yourself when done with these fixes" |

## What follows from each

1. The direction is B: a secret is its own thing, referred to and never written as a literal.
   The secret type is not designed yet. A [proposal](../secrets/README.md) sets out the
   decisions with a recommendation for each, and waits for the owner. Until it is ruled, the
   specification says nothing of secrets, and GP3 of the
   [documents audit](../docs-audit/GAPS.md) stays unconverted.
2. The six are dropped by this ruling. The tracker's
   [section](../../archive/plan-audit-history.md#behaviors-not-carried-into-the-specification)
   says so.
3. CON-004 is reworded. Applying it showed that only the decision to open is whole; each
   component is then opened by a request of its own, with its own outcome. The
   [log](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/log-spec.md) says why, and the tracker lists it for the owner.
4. The done-when of action A59 and the readiness condition G7 are restated in the tracker and
   in the [build document](../../implementation/INDEPENDENT_BUILDS.md). The coordinator reads
   "Understood" as accepting the recommendation, and the tracker's section for the next reader
   states the gate as restated.
5. The sentence of ruling 31 is withdrawn. The tracker's later amendments say so.
6. Later work leaves the specification. See the [work order below](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/README.md#work-order-for-the-editor-of-spec).
7. `apply-policy.v1` gains the warning.
8. SUB-010 and DL7 change.
9, 10, 12. No text changes. What was the coordinator's decision is now accepted by the owner,
   and the tracker's later amendments say "accepted by ruling 38".
11. A note for the next reader in the tracker.
13. `AGENTS.md` and the tracker give both as confirmed.
14. The two names are removed from the tracker and from `AGENTS.md`. Research packets,
   archived files and kept reports still hold them, which the rules of this repository say not
   to edit; the tracker lists that for the owner. Pull request 6 is merged by squashing.

Every other decision that the coordinator made on a finding since ruling 37 keeps its standing:
it is the coordinator's, it is logged, and the owner can strike it.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.
