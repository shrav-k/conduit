# Ruling 38: check of the handoff

Date: 2026-11-06. Read at commit `96ed241` on the branch of pull request 6, with a clean
working tree, as a reviewer with no context would read it after the merge: `README.md`,
`AGENTS.md`, "For the next reader" and the rest of `docs/AUDIT.md`, the record, log and check
of ruling 38, the proposal on secrets, `spec/README.md`, `spec/definitions.md`, the overview,
the documentation map, `CONTRIBUTING.md` and the build document, then samples of `spec/`
(`conventions/access.yaml`, `interface/implementation.yaml`, `core/README.md`,
`schemas/README.md` and the heads of the other files). Nothing but this file was edited.

## Summary

| Severity | Count |
| --- | --- |
| blocks-handoff | 1 |
| stale | 4 |
| minor | 7 |
| **Total** | **12** |

The handoff mostly works. From `README.md`, `AGENTS.md` and "For the next reader" a reviewer
can tell what binds, what to read, what was not read cold, the point the owner asked to have
examined (point 11), how readiness is now judged (point 4), where findings go, and that it may
not edit `spec/` or build an engine. Every count checked agrees with the tree. The secrets
proposal is marked proposed and not ruled everywhere. No link names the branch, and nothing
tells a session to push to it.

The one point that misleads: the composite Conduit. Applying ruling 38, the coordinator gave
each component's opening its own outcome, so a composite can be left partly open at setup. The
log says that this was put to the owner. Neither "Known to be open" nor "Waiting for the
owner" lists it (H-1).

## Output of the checks

Run from the repository root at `96ed241`.

```text
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 139 Markdown files and 1781 local links (offline).
Errors: 0                                                    (status 0)

$ python3 tools/set_names.py --check
Names: corePackage x435, apiVersion x241, metaPrefix x90. Errors: 0   (status 0)

$ python3 tools/check_schemas.py
Python 3.9.6 at /Applications/Xcode.app/.../python3; PyYAML 6.0.1; jsonschema 4.20.0
...
6. Applied 23 rules of this tool, which come from 21 promises, ... 24 sets of documents
   in 24 files under examples/invalid-by-rule/ were each rejected by the rule they are named for
   SCH-019: validated 10 defaults ...; 0 were not validated ...
Errors: 0                                                    (status 0)

$ git diff --check
(no output, status 0)
```

The counts were checked against the files:

| What | Entry documents | Tree |
| --- | --- | --- |
| Promises | 434: 220 language, 179 convention, 35 interface | 434: 220, 179, 35 (count of `layer` in every promise file) |
| Promises by file | `spec/README.md` contents table | Every file agrees |
| Core exports | 98, 20 Operation contracts | 98 in `spec/core/conduit.yaml`; 20 in the core README table |
| Examples | 7 valid, 63 invalid, 24 invalid-by-rule | 7, 63, 24 |
| Rounds | 125, 44, 38 findings; 8, 4, 3 unsafe; 26 for the reader of the repository in round 4 | As in `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/README.md` |
| Fix pass of round 4 | 92 changed, 5 added | `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/log-fixes.md`: 90 and 5 in the pass, 92 and 5 after its follow-up (line 301) |
| Scenario reader | 15 in each of the last two rounds | 15 and 15 |

Searches outside `docs/archive/`, `docs/research/` and the kept `reader-*.md` reports found no
"437", "442" or "99 exports", no "two tools" that need no Conduit, no `start-later-work.v1` or
`arrangesLaterWork` except in comments that record the removal and in the invalid example, no
domain required before the builds except in the amended ruling 31 row, and the six behaviors
given as dropped. The only leftover of the gate of two clean rounds is H-3.

## Findings

### H-1 blocks-handoff: the composite Conduit is open, and neither list says so

- `docs/proposals/ruling-38/log-spec.md:35`, under "Choices": each component's opening "is an
  invocation of its own, with its own outcome", "Where one fails or is unknown, those that
  opened stay open", and "It gives up something round 4 had: one outcome for the whole
  opening". It ends: "Put to the owner in the summary of this ruling's work."
- `docs/AUDIT.md:206`: "One list of everything that only the owner can settle." The list
  (lines 209 to 220) has no row for it, and "Known to be open" (lines 131 to 140) has no
  bullet.
- `docs/AUDIT.md:356` gives the ruling as "a composite Conduit opens whole or not at all at
  setup". The specification now opens it whole or not at all only as a decision.
- `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/README.md#L56` still lists "The two points at the end of
  DECISIONS.md, on opening a composite Conduit" as open. Nothing there says
  that ruling 38, point 3, and the choice above settled them.
- What it does: the reviewer either reports the partial opening as new, against the owner's
  words, or cannot tell whether the owner accepted it.
- Least change: add to "Known to be open" and to "Waiting for the owner": "A composite
  declared Conduit at setup: since ruling 38 each component is opened by its own request with
  its own outcome, so a failed or unknown opening can leave it partly open; only the decision
  is whole or nothing. Put to the owner", linking the choice in
  `proposals/ruling-38/log-spec.md#choices`.
  In the round-4 README, add after the bullet: "Settled by ruling 38, point 3, and the choice
  in its log."

### H-2 stale: ruling 38's changes are said to have been read by their editors alone

- `docs/AUDIT.md:114-115`: "Two things have been read by their editors alone", the second
  being "what ruling 38 changed".
- `docs/proposals/ruling-38/log-spec.md:66`: "A reviewer read these changes before the merge
  and reported fourteen findings, none unsafe (report)."
- Least change: in the second bullet, "what ruling 38 changed: ... A reviewer checked the
  change before its fixes (check); the fixes were read by the coordinator alone", with
  "check" linking `proposals/ruling-38/check.md`.

### H-3 stale: action A42 still has the old done-when for G7

- `docs/AUDIT.md:472`, done-when of A42: "Two consecutive sweeps of the language and shared
  conventions are clean (G7)."
- G7 is restated at `docs/implementation/INDEPENDENT_BUILDS.md:111` and A59 at
  `docs/AUDIT.md:482`. A42 was not changed.
- Least change: "G7, as restated by ruling 38, point 4: no unsafe finding in the latest round,
  and the reference model and the vectors agree with the promises."

### H-4 stale: the schemas say that the owner has ruled on none of their changes

- `spec/schemas/README.md:5-6`: "**Changed since by decisions that the owner has not ruled
  on.**" and line 12: "None of it is a choice the owner accepted."
- `spec/schemas/templates.yaml:4-5`: "rests on the coordinator's logged decisions, which the
  owner has not ruled on." The same head, with no note of ruling 38, is in `document.yaml`,
  `envelope.yaml`, `graphs.yaml`, `instances.yaml`, `manifest.yaml`, `schema.yaml` and
  `values.yaml`.
- Ruling 38, point 12, accepted shapes that these files hold: no `optional` on a runtime
  (`templates.yaml:313`) and a control's `changes` as a map. Point 6 made the removal of
  `arrangesLaterWork` (`schemas/README.md:257`). Check finding R38-8 fixed the same sentence
  in the promise files and in `spec/core/README.md:5-6`, but not here.
- Least change: in the README and each head, the clause `spec/core/README.md` uses: "apart
  from those that ruling 38 accepted or made".

### H-5 minor: the ruling log says eight changes are listed, and the table lists five

- `docs/AUDIT.md:356`: "eight changes to what earlier rulings accepted, listed under later
  amendments."
- `docs/AUDIT.md:377-386`: "each of the six below ... (points 10 and 12)": five changes of
  point 12 and the resolved form of point 10. Not listed: a rename advancing the revision, a
  template needing an implementation for every operation, and the defaults filled in before
  checking.
- Least change: in row 38, "eight changes, named in its point 12; those that changed an earlier
  ruling are listed under later amendments."

### H-6 minor: the ruling log gives one date for every ruling

- `docs/AUDIT.md:315`: "All rulings are the owner's, given on 2026-11-05, in this order."
- Ruling 38 was given on 2026-11-06 (`docs/AUDIT.md:206`, the record's status line).
- Least change: "given on 2026-11-05, apart from ruling 38, of 2026-11-06".

### H-7 minor: the record promises a sentence that the note for the next reader does not hold

- `docs/proposals/ruling-38/README.md:40-41`: "The coordinator reads 'Understood' as accepting
  the recommendation; the note for the next reader says so."
- `docs/AUDIT.md:123-124` says only that the gate "is withdrawn".
- Least change: add to that paragraph "The owner answered 'Understood', which the coordinator
  reads as accepting the recommendation."

### H-8 minor: one private repository name stays in an entry document, and a count is off

- `AGENTS.md:80`: "Do not edit the sibling directories [a private repository] and
  `../pi-reference`". That directory's name is one of the two names that ruling 38, point 14,
  has removed;
  `docs/proposals/ruling-38/README.md:50` says "The two names are removed from the documents."
  `docs/AUDIT.md:218` leaves the sibling directories to the owner, so this is known. The
  record is not true as written.
- `docs/AUDIT.md:218` says the names remain "in three readers' reports". Only two kept reports
  hold either name: `docs/proposals/docs-audit/reader-entry-points.md` and
  `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/reader-4.md`.
- No absolute local path, user name or address was found outside research, archive and the
  kept reports.
- Least change: in the record, "removed from the documents, apart from the sibling directory
  named in `AGENTS.md`, which is left to the owner"; in the tracker, "two readers' reports".

### H-9 minor: the squash merge keeps the session lines unless its message is set

- `docs/AUDIT.md:219`: "Ruling 38, point 14, has the pull request merged by squashing, so
  `main` gets one commit without them."
- All 26 commits of the branch end with a session link, and 26 carry an address formed from
  the machine's name (`git log origin/main..HEAD`). GitHub's default message for a squash of
  several commits can list every commit's message, session links included.
- Least change: when merging, write the squash commit's message by hand, with no session
  links. The text needs no change.

### H-10 minor: 39 links pin a commit that will not be on `main`

- `docs/AUDIT.md:561`: "The links show the older text as it stood at commit `7927d68`". The
  same commit is pinned by 39 links in `docs/AUDIT.md`, `docs/VOCABULARY.md`,
  `docs/proposals/authoring-syntax/`, `docs/proposals/vocabulary/README.md` and
  `experiments/`.
- `7927d68` is on this branch and not on `origin/main`. After a squash merge GitHub still
  serves it through the pull request's ref, so the links resolve, also if the branch is
  deleted. A clone of `main` does not hold it.
- Least change: none needed for the links. Optionally add to `docs/AUDIT.md:561`: "That commit
  is reachable through pull request 6, not from `main`."

### H-11 minor: round 3 still says round 4 is next

- `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3/README.md#L3`: "Status: done, 2026-11-06 (plan-audit action
  A59), and round 4 is next." (with a link on "round 4")
- Least change: "and round 4 followed."

### H-12 minor: loose ends in the log of ruling 38

- `docs/proposals/ruling-38/log-spec.md:62`: an empty heading, "## Not done here", directly
  followed by "## The check of the changes".
- Line 53: "Errors: 0 once the proposal on secrets exists". The proposal exists, and the check
  now gives 0.
- Least change: delete the empty heading; write "Errors: 0".

## What the next reviewer can rely on

- **What binds.** `spec/` alone, as `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, the
  documentation map and `spec/README.md` all say. No document says otherwise.
- **What to read and where to start.** `AGENTS.md:42-44` sends a cold reviewer to "For the
  next reader". That section and the reading orders of the other entry documents agree.
- **What has not been read cold.** The fix pass of round 4 (92 changed, 5 added) and ruling
  38's changes, with the qualification of H-2.
- **What is known to be open.** Secrets, the shapes of a diagnostic and an inspection result,
  the missing vectors, reference model and engine, and later work. Add the composite Conduit
  (H-1).
- **What the owner asked to have examined.** Point 11: an unknown tool answered with -32602
  and no result, `isError` on a result with no outputs, a client keeping protocol errors from
  its model, and SEP-2145. This agrees with IFC-012 and IFC-032 and with
  `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/mcp-recheck.md`.
- **How readiness is judged.** By ruling 38, point 4: no unsafe finding in the latest round,
  and the reference model and the vectors agree with the promises. G7, A59, the round-4 README
  and `spec/README.md:29-30` agree. A42 is the exception (H-3).
- **Where findings go.** A new `docs/proposals/reaudit/round-5/`, one report per reader, with
  IDs, severity, promise IDs, quotes, situation and least change. Nothing under `spec/` is
  edited, and no engine is built (`AGENTS.md:6-8`).
- **Who decided what.** Rulings 1 to 38 are the owner's. Every other change since ruling 37 is
  the coordinator's, logged and open to being struck. The heads of the schemas are the
  exception (H-4).
- **Secrets.** Marked "proposed, not ruled" in the proposal's status line, the tracker (lines
  98, 135, 211, 556), the documentation map and the record. `spec/` does not state any rule
  about secrets.
- **Counts.** Every count in the table above agrees with the tree, and the four checks pass.
- **After the merge.** No link names the branch, and no link to `pull/6` calls the pull request
  open or a draft. `AGENTS.md:88-89` says later work goes on a new branch with its own pull
  request.
