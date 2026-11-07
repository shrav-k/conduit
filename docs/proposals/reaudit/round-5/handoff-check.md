# Round 5: check of the handoff to the next reader

Read cold on 2026-11-07, as the next reader would arrive, in the working tree of
`docs/pre-codex`: `HEAD` is `d0c425f`, 50 tracked files are modified, and
`docs/proposals/reaudit/round-5/` and two examples under `invalid-by-rule/` are untracked.
Nothing was edited but this file. Paths are relative to the root of the repository.

## Opening

**What I read.** In full: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `CLAUDE.md`,
`docs/README.md` and `docs/AUDIT.md`. Following their links as far as needed to know where to
start: `spec/README.md` (status, contents, conformance, the limits on secrets),
`docs/proposals/reaudit/README.md`, the round 5 README, the head and tables of its
`DECISIONS.md`, its `log-fixes.md` (result, integration choice, verification, for the owner),
the headings of the seven per-area logs and of the six reports, the round 4 README, the gate
section of `docs/implementation/INDEPENDENT_BUILDS.md`, the head and triage of
`docs/LANGUAGE_DEFECTS.md`, the "Settled by the coordinator" and "Left for round 5" sections of
`docs/proposals/ruling-41/open-decisions.md`, the "Needs a decision" sections of ruling 42's
language and integration logs, the head of `docs/OVERVIEW.md`, the head of
`tools/check_schemas.py`, and the promises that the findings below cite, printed from the YAML.

**The counts, checked with a script.** Every count that the entry points and the round 5
record give agrees with the files, with one exception (H-02).

- Promises: 452 in 23 files, 202 `language`, 207 `convention`, 43 `interface`, matching
  `spec/README.md` file by file and layer by layer. Against `d0c425f`: 449 (202, 206, 41)
  before; added PKG-030, IFC-045 and IFC-046; none removed or moved; SEC-024 moved from
  `language` to `convention`; 101 changed, which is exactly the union of the fix log's 91 and
  the verification's 30, 20 in both. `interface/implementation.yaml`: 41 before, 43 after, 22
  of the 41 changed.
- Implementation-defined list: 15 items (DL1 to DL17 without DL11 and DL14); DL4, DL10,
  DL12, DL13, DL16 and DL17 changed.
- Core package: 102 exports in its manifest (61 documents under `core/`, 41 Schema contracts
  in 8 files under `schemas/`) and the implementation `engine`; every per-file count in the
  contents table holds. Examples: 7 valid files in 3 directories, 56 invalid, 30
  invalid-by-rule.
- Round 5: 24, 21, 16, 23, 14 and 21 findings, 119 in all, 105 by the readers of `spec/` and
  61 by readers 1 to 3; each reader's severities match the table of `DECISIONS.md`; 10 unsafe
  (1, 1, 7 and 1). 111 items M01 to M111, eight of them joining two findings, every one of the
  119 source IDs used once; 10 unsafe, 8 contradiction, 32 undetermined, 19 wrong, 16
  misleading, 26 minor; 110 fix, 1 reject. Verification: V1 to V13 and V2-01 to V2-16, 29,
  none unsafe, 28 fixed, V6 for the owner.
- The coordinator's 43 decisions on the open questions: 44 rows, one of which (D6) is the
  owner's. The defect register: 167 rows, 48 / 39 / 30 / 50 by layer; 13 closed and 74 ruled
  in the first two, ten of the 13 by ruling 41; 5 closed, 14 ruled and 31 implementation-defined
  in the runtime layer.
- The checks: `check_docs.py` 0 errors (175 files, 2411 links), `set_names.py --check` 0,
  `check_schemas.py` 0 (25 rules from 24 promises; 30 sets rejected), `git diff --check`
  clean; the untracked round 5 files have no trailing whitespace.

**Where this report goes.** "For the next reader" sends a reader's report to a new directory,
`docs/proposals/reaudit/round-6/`. This check was asked to write here instead, in round 5's
directory, whose README does not list it.

**Judgment.** The entry points agree with each other and with the files on nearly every count,
and what binds is said plainly and in the same words everywhere: `spec/` alone, with the
standing of each kind of decision set out in "Who decides what". Where they fail is at the
edges that round 5 moved. Older sections of the tracker still state, as current, rules that
round 5 changed (the scope of the gate, placement of an instance that runs nothing, what
`unregister.v1` removes, DL11). Two lists that claim to be complete are not ("Waiting for the
owner", and what is left for a reader). And no commit holds the text that the next reader is
to read. None of this binds, so nothing here is unsafe in the scale's sense. The three that
matter most for the next read are H-05 (which text to read), H-06 (what counts as unsafe) and
H-01 (the scope of the gate, the very error that round 5's unsafe M01 corrected in
`spec/README.md`'s prose).

---

## H-01. wrong. Two summaries put every server that the engine does not run itself behind a gate

- **Promises:** PRV-025, IFC-041, IFC-007, SEC-024
- **Quotes:** `docs/AUDIT.md`, A64: "ruling 43 adds the gate in front of every server the
  engine does not run itself (PRV-025)". `spec/README.md`, contents row for `providers.yaml`:
  "the gate before every server that the engine does not run itself". PRV-025 `when`: "A
  runtime provider other than the engine's own runner runs an item for an instance, under
  PRV-007; or an item reached by `remote` that is not `plain` serves an instance". The prose
  of `spec/README.md`: "A `plain` server reached by `remote` takes none of Conduit's keys, and
  whatever can reach its URL reaches it (IFC-007, IFC-041)."
- **Situation:** A reader who takes A64 or the contents table at their word believes that a
  `plain` server reached by `remote`, a secret store's among them, admits only the engine. It
  admits anything that can reach its URL. That belief is the one that round 5 found unsafe
  (M01, 5-4-03) and corrected in the prose of `spec/README.md` and in SEC-006 and SEC-024, but
  not in these two lines. A64's status also reads as if its done-when, proof of the sender on
  every request that crosses machines, were met for servers, while a request from the engine
  to a plain server at an `https` URL on another machine carries no proof of the sender by
  design.
- **Least change:** In both lines, add "other than a `plain` server reached by `remote`", as
  the prose of `spec/README.md` says. In A64's status, say that a plain server is outside the
  gate.

## H-02. wrong. Two defects are answered by DL11, which round 5 removed, so "31" is stale

- **Promises:** DL11 (removed), ACC-003, ACC-036, CON-004, ACC-006, LIF-027
- **Quotes:** `docs/AUDIT.md` and `docs/LANGUAGE_DEFECTS.md`: "31 are answered by an item of
  the implementation-defined list". Rows L155 and L156 of the register: "implementation-defined:
  DL11". The register's status values: "The row names the item, `DLn`, of the
  implementation-defined list that covers it." `spec/implementation-defined.yaml`, head:
  "removed DL11, on Conduits during setup, because ACC-003 and ACC-036 decide what it left
  open", and from round 2's last pass, "from DL11 the later opening of an optional Conduit,
  which CON-004 and ACC-006 decide".
- **Situation:** Counted with a script, 31 runtime rows have the status
  `implementation-defined`, and two of them, L155 (what a partly connected consumer may do)
  and L156 (what later opens an optional Conduit), name DL11, which no longer exists (M69).
  Only 29 rows name an item that exists. L156's pointer was already stale before round 5:
  round 2 moved its subject out of DL11 into CON-004 and ACC-006. Round 5's logs do not touch
  the register. A reader who follows L155 to the list finds no DL11, and the tracker's figure
  is false of the files.
- **Least change:** Give L155 the status of the promises that decide it (ACC-003, ACC-036,
  with M69 named) and L156 that of CON-004 and ACC-006; then say "16 are ruled and 29 are
  answered by an item" in the register's triage and in the tracker's row on language defects.

## H-03. wrong. "Later amendments" still states ruling 41's placement and unregistration, which M22 and M41 changed

- **Promises:** PLC-002, PRV-015, LIF-017, LIF-036, PLC-007
- **Quotes:** `docs/AUDIT.md`, "Later amendments", row for rulings 7, 8 and 9: "that provider
  places an instance that is given no placement (PRV-015)". Row for rulings 32 and 33: "An entry
  that no input places is placed by the machine provider (PLC-002)". Row for rulings 29 and
  30: "`unregister.v1` stays for templates, and for an instance whose node was declared lost
  (LIF-017)". PLC-002 `then` now: "Where no item that is run is selected for it, its placement
  is `local`, and no machine provider is asked." LIF-017 `then` now: "The request is dispatched
  only where the instance is retired, and it removes the instance." `docs/OVERVIEW.md`:
  "`placement` may be left out, and the machine provider then places the instance."
- **Situation:** Round 5's M22 placed an instance that runs nothing on `local`, and M41 let
  `unregister.v1` remove any retired instance. `DECISIONS.md` names both among the decisions
  that come closest to what ruling 41 accepted (decisions 8c and 17 of the principles review).
  The tracker's amendments table, which a reader uses to learn how the rulings now read,
  still gives the old rules and cites the very promises that now say otherwise. The same table
  does record M69's removal of DL11 in the row for ruling 31, so a reader would conclude that
  round 5 changed nothing else that a ruling had accepted. The overview's sentence contradicts
  its own correct sentence further down ("an instance that has none ... is placed on
  `local`").
- **Least change:** In the three cells, add what M22 and M41 changed, with their IDs, as the
  row for ruling 31 does for M69; under the table, list M22 and M41 (and the integration's
  choice on PLC-023 and LIF-036) as coordinator changes to what a ruling accepted that no ruling
  has accepted. In the overview, say "the machine provider then places an instance that has a
  program to run".

## H-04. wrong. The tracker sends the reader to "left for a reader" points that are settled

- **Promises:** PRV-023, PRV-024, PRV-021, SEC-013, SEC-023, MAN-017, REV-010, LIF-015
- **Quotes:** `docs/AUDIT.md`, "Left for a reader by the records": "item 5 of ruling 42's
  integration log lists small points that no later record took up" and "The other points for
  a reader may still be reported." Item 5 includes "PRV-023 does not skip a value that begins
  with `()`", "no `then` states that no record holds `identity`" and "SEC-013 does not say when
  the connection of an instance that the engine retires itself (PLC-017, PLC-024) ends". Now
  PRV-023: "only where the value does not begin with `()`"; PRV-024 (`must not`): "It puts what
  `identity` holds, or any part of it, in a record"; SEC-013: "the instance's retirement ends,
  or the instance's node is declared lost".
- **Situation:** Every one of item 5's six points is now addressed: PRV-023's `()` and
  PRV-024 by ruling 42's own verification (the head of `providers.yaml` says so), the held stop
  of a retired provider and SEC-023's failing read by ruling 43's items (as round 5's own
  preface to that log says), SEC-013's end of a connection, and MAN-017 by M30. Of the five
  points of the open questions "for a reader", the first, a control in progress when the
  engine retires its instance, was settled by M35 (REV-010: "it records the outcome and
  stores nothing"). The tracker credits only M30, says no later record took up the rest, and
  invites the reader to report them. A reader who does so spends findings on settled text,
  and one who does not cannot tell which of these points are still open.
- **Least change:** Replace the paragraph with the points that are still open, by my reading
  the TPL-004 and CTR-004 default of a hook, the repeated named argument such as `-e`,
  PLC-024's form of a cause, and the late onward request that A65 carries, and mark each
  settled one with what settled it.

## H-05. misleading. No commit holds the text that the next reader is to read

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`: "read the rewritten specification at commit `d0c425f`"; "4. Pull
  request 7 is merged."; "The work is committed and pushed at each milestone". `AGENTS.md`: "It
  is about to be merged." Round 5's `log-fixes.md`: "Nothing was committed or staged."
- **Situation:** `HEAD` is `d0c425f`, the commit that round 5 read. Round 5's fixes (101
  promises changed, 3 added), its whole record and two examples exist only in the working
  tree: 50 files modified, the round 5 directory untracked. Pull request 7 is open on GitHub.
  A reader given a checkout of `origin/docs/pre-codex` gets `d0c425f`, with the pre-fix
  specification and no round 5 at all, while a reader of this working tree sees `HEAD` at the
  commit the tracker says round 5 read, and could take the files for that text. The tracker
  names the commit that was read but not the one to read next, and step 4 of "What happens
  next", alone of the list without "Done:", is in the present tense, so whether pull request 7
  is merged cannot be told from the text.
- **Least change:** Commit and push round 5's work and this check, then name that commit in
  "For the next reader" and in the round 5 README as the text the next reader reads. Write step
  4 as "Next: pull request 7 is merged" until it is.

## H-06. misleading. The reader is not told that leaking a secret is unsafe

- **Promises:** none (G7)
- **Quotes:** `docs/AUDIT.md`, "How the readiness gate now reads": "A reader is asked to report
  every finding, and to mark as unsafe any that could grant access, lose or repeat an effect,
  or report a wrong result." Round 5's M01, classed unsafe: "its values cross the network in
  clear". Report 5-4-02, unsafe: "A run's credential goes in clear".
- **Situation:** G7 is judged on the latest round reporting no unsafe finding, and this is
  the only definition of unsafe that the entry points give. Since ruling 41 the specification
  carries secrets, and round 5 classed exposure of a secret or a credential as unsafe. A reader
  who finds a path by which a secret's value reaches a log, a record or another machine in
  clear, without granting any access, would by this sentence class it as wrong or
  undetermined, and the round would look closer to G7 than it is. "Could grant access" also
  omits "that it should not", which every permitted request satisfies.
- **Least change:** "to mark as unsafe any by which an engine that keeps every promise could
  grant access it should not, leak a secret, lose or repeat an effect, or report a wrong
  result."

## H-07. misleading. "Waiting for the owner" is not the one list it says it is

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`: "One list of everything that only the owner can settle." Its
  second row: "Ruling 43 settled the last two questions that the rewrite left for the owner".
  A21: "whether to drop them is the owner's call". A20: "Rule on v0 authorization scope.",
  status `open`.
- **Situation:** The list has two rows, and only the first, V6, waits for anything; the
  second records what is settled. Meanwhile the action items carry owner decisions that are
  pending and are not in the list: A21's five inconsistencies, which are explicitly the owner's
  call, and A20's ruling on v0 authorization scope (with G8's closing of the open findings and
  probes behind it). A reader who trusts the heading counts one pending point, as "What happens
  next" also says, and does not learn of the others.
- **Least change:** Move the second row to a sentence under the table; add rows for A21 and
  A20, or narrow the heading to "decisions on the specification that wait for the owner".

## H-08. misleading. The gate's state is given as of 2026-11-07, from an assessment made before the rewrite

- **Promises:** none (G1, G7, G8, G9)
- **Quotes:** `docs/AUDIT.md`, under "State on 2026-11-07": "None is met; G1 and G3 are partly
  met." `INDEPENDENT_BUILDS.md`: "It has not been made again since the specification was
  rewritten"; its G1 row, "Not satisfied in full: behavior the owner accepted on secrets is
  stated only outside `spec/`"; its G7 row, "The latest round of cold readers to report, round
  4"; its G8 row, "The secret type is open under ruling 39, point 1."
- **Situation:** The tracker states the gate's result as current. The table it points to is
  the assessment of 2026-11-06, confirmed before rulings 40 to 43 and kept as confirmed, with a
  paragraph of later changes beside it. G1's only stated shortfall, secrets, has since been
  written (`secrets.yaml`), so a reader cannot tell whether G1 is now met, partly met, or
  short for another reason. G7, G8 and G9 cite round 4, an open secret type and 35 interface
  promises.
- **Least change:** In the tracker's row, say "as assessed on 2026-11-06, before rulings 40 to
  43, and not assessed since"; or assess G1, G7, G8 and G9 again.

## H-09. minor. A61 does not record round 5's reader of the whole repository

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`, A61: "the fourth reader of round 4 reported 26, 8 of them
  misleading"; done-when, "The action is done when such a reader reports nothing that
  misleads."
- **Situation:** Round 5's reader 5 read the whole repository and reported 14 findings, 7 of
  them misleading (`DECISIONS.md`), all fixed under F7. A61's status stops at round 4, so a
  reader judging A61 takes round 4 for the latest such read.
- **Least change:** Add a sentence on round 5's reader 5 and its 14 findings, with a link to
  its report and to M100 to M111.

## H-10. minor. "The re-audit has the counts by round" links a page that has none

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`, "For the next reader": "The re-audit ... has the counts by
  round.", where "re-audit" links `proposals/reaudit/README.md`.
- **Situation:** The linked page is round 1's record. It gives round 5's totals in a sentence
  and links the other rounds, but has no counts for rounds 2 to 4. The table of counts by round
  is in `round-5/README.md`.
- **Least change:** Link `proposals/reaudit/round-5/README.md`.

## H-11. minor. The identifiers table omits round 5's M and V items, and V collides

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`: "A bare identifier in this document is one of the tracker's
  own"; row V: "Vocabulary decisions, V1 to V8". The tracker writes "(M30)", "(M69)", "M78 of
  its decisions" and "V6 of the verification of round 5's fixes".
- **Situation:** The tracker cites round 5's items M01 to M111 and verification findings V1
  to V13 and V2-01 to V2-16, but its table of identifiers lists neither set. "(M69)" in the row
  for ruling 31 is bare. V6 exists both as a vocabulary decision and as the verification point
  that waits for the owner; the text qualifies it, but the table would send a reader to the
  vocabulary record.
- **Least change:** Add a row: "M; V and V2- | Round 5's items M01 to M111; the findings of its
  verification, V1 to V13 and V2-01 to V2-16 | round 5", noting that a V in a sentence on round 5
  is never a vocabulary decision.

## H-12. minor. A11 still leaves the mechanism of proof open among three, after ruling 43 fixed two forms

- **Promises:** IFC-022, IFC-042, DL13
- **Quotes:** `docs/AUDIT.md`, A11: "which mutual TLS, SPIFFE or the protocol's authorization
  can each be" and "The mechanism waits for the native probe (A4)". DL13 now: "Its two forms of
  proof: a signed token in the form of a JWT-SVID ... which the engine's address always
  accepts ... and mutual TLS".
- **Situation:** A64 was brought up to date with ruling 43 (Q1); A11 was not. A reader of A11
  thinks an engine may still pick any of three mechanisms and that the choice waits on a probe,
  while DL13 confines it to two forms and makes the token form always accepted at the engine's
  address. What may still wait on A4 is how a native Agent obtains and presents the token.
- **Least change:** Add to A11's status: "Ruling 43 (Q1) fixed two forms, the token form always
  accepted at the engine's address (IFC-042, DL13); what waits for A4 is how a native Agent
  presents it."

## H-13. minor. "Accepted it as it stands at ruling 39"

- **Promises:** none
- **Quotes:** `docs/AUDIT.md`, row "Specification": "The owner accepted it as it stands at
  ruling 39". `README.md` and `spec/README.md`: "as it stood at ruling 39".
- **Situation:** The present tense can be read as acceptance of the text as it now stands,
  which includes round 5's decisions that no ruling has accepted. The other entry points use
  the past tense.
- **Least change:** "as it stood at ruling 39".
