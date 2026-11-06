# Ruling 39: the open points before the cold read by Codex

Status: **ruled by the owner on 2026-11-06** (plan-audit ruling 39). This is the record of a
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
