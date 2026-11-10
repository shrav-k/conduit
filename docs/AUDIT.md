# Plan audit and refinement tracker

Opened: 2026-11-05. Baseline: commit `37de771` (`research/vocabulary-precedents`).

This is the working tracker for the direction the owner set on 2026-11-05 after an external audit
of the plans. It records where things stand, the rulings, what waits for the owner, the
priorities and the action items. The order of the work and the experiment program, with the row
of each experiment and probe, are in the [roadmap](../ROADMAP.md), which A72 moved out of this
tracker (ruling 45). Between them, this tracker and the roadmap hold the state of the work: the
earlier review queue is superseded and removed. Neither is a specification. The specification
is [spec/](../spec/README.md), which alone binds, and a ruling changes nothing until it is
written there.

The external audit's report is not kept as a file. What survives of it is the
[findings](archive/plan-audit-history.md#findings) and the other sections marked **Historical**,
which ruling 39 moved to the [history of this tracker](archive/plan-audit-history.md). "The audit" in ruling 1
means it. This document is called the plan audit after it. Later audits have other names: the
consolidation audit, the re-audit of the specification and the audit of the documents outside
`spec/`, each with a record under [proposals/](README.md#records-of-rulings).

## Identifiers

A bare identifier in this document is one of the tracker's own:

| Prefix | Meaning |
| --- | --- |
| **A** | Action item |
| **Q**, **D** | Question and disagreement, in the [findings](archive/plan-audit-history.md#findings) |
| **E** | Standing probe or comparison of the plan audit, E1 to E5, E8, E10 and E11, first worded in [its history](archive/plan-audit-history.md#standing-probes-and-comparisons). Each is an action item now (A4, A5, A6, A7, A10, A19, A22 and A8), whose row is in the roadmap's [experiments and probes](../ROADMAP.md#experiments-and-probes) and is written there as "probe E10 of the plan audit". The earlier ideas E0, E6, E7, E9 and E12 were superseded by A42, A14, A40, A43 and A34, whose rows are here |
| **S** | Scenario of the experiment program, in the [roadmap](../ROADMAP.md#scenarios) |
| **P** | Priority, P0 to P3 |
| **LD** | [Layer decision](#layer-decisions) |
| **G** | Condition of the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate), G1 to G11 |
| **L** | Language defect, in the [register](LANGUAGE_DEFECTS.md) |

The records of the rulings number their own decisions, and some reuse a letter. An identifier
from a record is therefore written with the record's name, as in "re-audit decision G1".
The rows of the ruling log are not rewritten, so in the rows of rulings 42 and 43, and wherever
an identifier follows the number of one of them, as in "ruling 42 (D6)" or "ruling 43 (Q4)", a
D, R or Q is one of the open questions after the rewrite or of the multi-machine proposal, in
the table below. An M is round 5's, as is a V or a V2- written with round 5 or its
verification, as in "V6 of the verification of round 5's fixes". In the row of ruling 44, and
wherever a D or an E is written with round 6, as in "D07 of round 6", it is one of round 6's
design critiques or evidence cases, in the last row.

| Prefix | Meaning | Record |
| --- | --- | --- |
| **PL**, **IN** | Placement and instantiation decisions, written with no hyphen: PL1 to PL8 and IN1 to IN10. With a hyphen, **PL-1** to **PL-10** are something else: the earlier wording of the placement rules, which are now the promises `PLC-001` onward | [placement](proposals/placement/README.md), [instantiation](proposals/instantiation/README.md) |
| **NS** | Decisions on the form of the specification, NS1 to NS4 | [normative-surface](proposals/normative-surface/README.md) |
| **V** | Vocabulary decisions, V1 to V8. The removed [vocabulary map](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) also numbered its topics V01 to V37. In a sentence on round 5 of the re-audit, a V is the verification's, in the last row | [vocabulary](proposals/vocabulary/README.md) |
| **RD**, **RR** | Decisions and rules on relationships and access | [relationships](proposals/relationships/README.md) |
| **OD**, **OR** | Decisions and rules on operations, outcomes and revisions | [operations](proposals/operations/README.md) |
| **DD**, **DR** | Decisions and rules of the declarations batch | [declarations](proposals/declarations/README.md) |
| **GD**, **SD**, **GR**, **SR** | Decisions and rules on graphs and on the schema profile | [graphs-and-schemas](proposals/graphs-and-schemas/README.md) |
| **PD**, **PR** | Decisions and rules on packages, first start and conformance | [packages-and-start](proposals/packages-and-start/README.md) |
| **NA** to **NF** | Naming decisions | [naming](proposals/naming/README.md) |
| **CS**, **CC** | Consolidation findings, on the shape of documents and on concepts | [consolidation](proposals/consolidation/README.md) |
| **OL**, **DO**, **RC**, **NI** | Decisions on the four remaining shared conventions | [conventions](proposals/conventions/README.md) |
| **D**, **I**, **X** | Decisions on the shape of documents, on the interface, and on the name the owner controls | [schemas-and-interface](proposals/schemas-and-interface/README.md) |
| **DL** | An item of the [implementation-defined list](../spec/implementation-defined.yaml), DL1 onward | The specification |
| **U**, **Q**, **A**, **G**, **W** | Re-audit decisions, by group. **F** is a follow-up decision | [reaudit](proposals/reaudit/README.md) |
| **C**, **N**, **P**, **D**, **X** | Choices, names, open points, disagreements and accepted changes of the core package and schema batch | [core-and-schemas](proposals/core-and-schemas/README.md) |
| **DA** | Decisions of the audit of the documents outside `spec/` | [docs-audit](proposals/docs-audit/README.md) |
| Items 1 to 31, **C** | The decisions of the principles review, which ruling 41 accepted: items 1 to 31, with lettered lines such as 3a, and its candidates C1 to C58. C14, C20 and C21 are its changes to choices that rulings had accepted | [principles-review](proposals/principles-review/README.md) |
| **P** | The planner's choices P1 to P20 for the rewrite under ruling 41, which the owner accepted with P2 changed | [ruling-41 work order](proposals/ruling-41/PLAN.md#planners-choices) |
| **D**, **R** | The questions that the rewrite under ruling 41 left open, D1 to D20 and R1 to R28, numbered as in [its integration log](proposals/ruling-41/log-integration.md#what-remains) | [open questions after the rewrite](proposals/ruling-41/open-decisions.md) |
| **Q** | The five smaller questions on work across machines, Q1 to Q5 | [multi-machine](proposals/multi-machine/README.md) |
| **M**; **V**, **V2-** | Round 5 of the re-audit: its items M01 to M111, into which the coordinator merged the readers' findings, and the findings of the verification of its fixes, V1 to V13 and V2-01 to V2-16. A V here is never a vocabulary decision, and M is not the M ruling set of the archived reviews | [round 5's decisions](proposals/reaudit/round-5/DECISIONS.md), [its fix log](proposals/reaudit/round-5/log-fixes.md#verification) |
| **6-1-**; **D**, **E** | Round 6 of the re-audit, the owner's cold read by Codex: its findings 6-1-01 to 6-1-06, its design critiques D01 to D22 and its evidence cases E01 to E20, always written with two digits and with round 6. A D here is never a disagreement of the plan audit, and an E never one of its probes | [round 6's decisions](proposals/reaudit/round-6/DECISIONS.md), [its report](proposals/reaudit/round-6/design-critique.md) |
| **X** | The five experiments that pull request 10 proposed, X1 to X5, always written with "of pull request 10", as in "X3 of pull request 10". They are not the X of the two records above | [ruling 45](proposals/ruling-45/README.md) |
| **H** | The hypotheses H1 to H10 of the experiment across two machines (A74) | [multi-machine](proposals/multi-machine/README.md#11-an-experiment-across-two-machines) |

A promise has an identifier of letters, a hyphen and three digits, such as `CON-001`. The
letters name its file; [spec/README.md](../spec/README.md#contents) lists the files. None of
these sets reuses the earlier R, I, C or M ruling sets of the archived reviews.

## How to use this document

- Statuses are `open`, `in progress`, `ruled`, `done`, `answered`, `dropped` or `superseded`,
  optionally followed by a colon and a note. `ruled` means decided by the owner and written as
  promises, with vectors still to come. `answered` is used for a question.
- A ruling is recorded once in the [ruling log](#ruling-log), then applied to the documents it
  affects. The rows of the log are not rewritten; a [note under it](#later-amendments) lists
  which rulings later ones amended.
- Promise ranges and counts in status notes are as first written. Later passes folded some
  promises into others and added more; the current files and counts are in
  [spec/README.md](../spec/README.md#contents).
- The order of the work and the experiment program are in the [roadmap](../ROADMAP.md). An
  experiment's or a probe's action item has its row there, in the
  [register](../ROADMAP.md#experiments-and-probes), and not here; every other action item has
  its row here.
- Whoever starts an item writes `in progress` in its status, or in its State on the roadmap,
  with the branch or the pull request that carries the work.
- Experiment fixtures and dated results live under [experiments](../experiments/README.md).
- A finding against `spec/` goes in the [re-audit record](proposals/reaudit/README.md). The
  [defect register](LANGUAGE_DEFECTS.md) holds the first sweep, which read the protocol topics
  that ruling 39 removed.
- The sections that recorded the audit as it was first made, and the action items that are done
  or superseded, are in the [history of this tracker](archive/plan-audit-history.md).
- When every item is closed, move this file to the [archive](archive/README.md).

## Where things stand

| Item | State on 2026-11-09 |
| --- | --- |
| Roadmap | The order of the work and every experiment and probe are in the [roadmap](../ROADMAP.md). |
| Specification | A draft, in [spec/](../spec/README.md): numbered promises in three layers (the language, the shared conventions and the implementation interface), with the definitions, the schema profile, the implementation-defined list, the core package as documents, the JSON Schemas of every document and valid and invalid examples. Its [contents table](../spec/README.md#contents) gives the files and the counts. The owner accepted it as it stood at ruling 39, and it was rewritten around provider contracts under rulings 40 to 43 ([plan and logs](proposals/ruling-41/PLAN.md), [ruling 42](proposals/ruling-42/README.md), [ruling 43](proposals/ruling-43/README.md)). Round 6's decisions and ruling 44 changed it again ([decisions](proposals/reaudit/round-6/DECISIONS.md), [ruling 44](proposals/ruling-44/README.md)), and ruling 45 added six [core terms](../spec/definitions.md#core-terms) to its definitions. No promise has a conformance vector. |
| How it was checked | Four rounds of cold readers read `spec/` before the rewrite under rulings 40 to 43 ([re-audit](proposals/reaudit/README.md)): about 170 problems in round 1, counted by class, then 125, 44 and 38 findings in the three reports on `spec/`. The interface was checked twice against the published Model Context Protocol ([first](proposals/reaudit/round-2/mcp-citations.md), [second](proposals/reaudit/round-4/mcp-recheck.md)). [Round 5](proposals/reaudit/round-5/README.md) read the rewritten specification at commit `d0c425f`: 105 findings in the five reports on `spec/`, 10 of them unsafe, and 14 from the reader of the whole repository. One of the five reports was a third check against the protocol. Every finding of round 5 is resolved and logged: 110 of the 111 items that the findings were merged into are fixed and 1 is rejected. Two verifiers then checked round 5's fixes and reported 29 findings, none unsafe: 28 were fixed, and the coordinator then settled the last, V6, as its option 1, in PLC-007 and PLC-002, so all 29 are resolved. [Round 6](proposals/reaudit/round-6/README.md), the owner's cold read by Codex, read the whole repository at `c4d7d33`, the merge of pull request 7: six findings, two of them wrong and safety-critical, and a critique of the design's simplicity and usefulness in 22 points. An independent check confirmed all six findings. Every finding is fixed, under the coordinator's decisions that ruling 44 accepted. No round was clean. The text that rounds 5 and 6 and rulings 44 and 45 changed has not been read cold since: round 6 read round 5's changes, but selectively, as its report says. |
| Secrets | In the specification since ruling 41: [secrets.yaml](../spec/promises/secrets.yaml), a core capability that each store's package implements, read only by the engine, for a run or a connection, and again after a restart (SEC-010, SEC-013, SEC-023, PRV-026). Ruling 42 named the capability `secret-provider.v1` and its operation `get-secret.v1`. The [research packet](research/SECRETS.md) and the [proposal](proposals/secrets/README.md) are the reasoning. |
| Documents outside the specification | Nothing outside `spec/` binds. What the specification superseded was removed from the tree by ruling 39; the [archive's index](archive/README.md#removed-from-the-tree) lists it, with links to its last version. The historical sections of this tracker are in [its history](archive/plan-audit-history.md). |
| Language defects | 167 recorded in the [first sweep](LANGUAGE_DEFECTS.md), of the protocol topics. Triage by layer: 48 language, 39 shared convention, 30 type contract, 50 conformance document (the register's runtime profile). Of the 87 in the first two layers, 13 are closed and 74 are ruled; ten of those closed went with a feature that ruling 41 removed. Of the 50 left to the conformance document, 5 are closed, all with a feature that ruling 41 removed, 16 are ruled and 29 are answered by an item of the implementation-defined list. Two of the 16, L155 and L156, named DL11 until round 5 removed it (M69), and now cite the promises that decide them. The 30 type-contract defects wait for their contracts. None has a vector. |
| Readiness gate | G1 to G11, with G7 as ruling 38 restated it. As assessed on 2026-11-06 under ruling 39 and confirmed by the owner, before rulings 40 to 43, none is met, and G1 and G3 are partly met. That assessment has not been made again since. The [build process](implementation/INDEPENDENT_BUILDS.md#state-on-2026-11-06) keeps it as confirmed, with the changes since that bear on its rows: among them, secrets, whose absence was G1's only stated shortfall, are written into `spec/`, and the rows for G7, G8 and G9 still cite round 4, an open secret type and 35 interface promises. Round 5 reported ten unsafe findings, and all ten are fixed. Round 6, the latest round, reported two findings that its reader classed as wrong and safety-critical, 6-1-01 and 6-1-02; round 5 classed such a conflict between a specific procedure and a general safety promise as unsafe (M32). Both are fixed (ruling 44), so no such finding is open. G7 asks that the latest round report none, and round 6 did not: the next round it can be judged on is the read that follows the first build. |
| Vocabulary | Ruled ([record](proposals/vocabulary/README.md)) and applied in `spec/` and the overview. The words that the promises no longer use are listed, with what replaced each, in [spec/definitions.md](../spec/definitions.md#words-the-promises-do-not-use) (A29). Ruling 42 accepted the twenty names of the [vocabulary audit](proposals/vocabulary-audit/README.md) of the rewritten specification, which ruling 41 asked for, and `spec/` uses them. |
| Build process and brief | Drafted: [independent builds](implementation/INDEPENDENT_BUILDS.md), [engine build brief](implementation/ENGINE_BUILD_BRIEF.md). Not issued. |
| Code | [Experiment 004](../experiments/004-stateright-evaluation/README.md), a Rust model-checking evaluation, and [Experiment 005](../experiments/005-macos-keychain/README.md), a probe of the macOS keychain for the first secret store package, executed and re-run. The Python checkers in `tools/` check the documents; round 5 added one rule to `check_schemas.py` and changed another, and round 6 added two, for CTR-006 and PRV-016. Since A69 changed its rules, and the coordinator's decision on finding 1-03 of the first build added one, for SCH-016, its run on 2026-11-09 applies 29 rules, from 28 promises. Since A72, `check_roadmap.py` checks that each action item has one row, in this tracker, the roadmap or the history, and that the roadmap's order cites each open one. The first build has begun in [reference-model/](../reference-model/README.md): the strict loader in Rust (A43), increment 1, which reads and validates documents; the [roadmap](../ROADMAP.md#the-order-of-the-work) says where it stands. No engine, no core of the reference model and no package yet. |

### What happens next

What comes next, and the order of the work, are in the
[roadmap](../ROADMAP.md#the-order-of-the-work). What has been done is in the
[ruling log](#ruling-log) and the records that its rows link.

### For the next reader

Written for anyone who audits this repository cold. Round 6, the owner's cold read by Codex,
read it at `c4d7d33`, and its findings are decided and applied. The next read by cold readers
follows the first build ([the roadmap](../ROADMAP.md#the-order-of-the-work)); a reader who
comes before it is welcome, and should know that the build may change the text.

**What to read.** `README.md`, then [spec/README.md](../spec/README.md) and
[spec/definitions.md](../spec/definitions.md), then the promises. Only `spec/` binds. The
[overview](OVERVIEW.md) is a summary and does not bind. The
[documentation map](README.md#reading-paths) has shorter paths for an author, for a writer of
contracts and providers, and for an implementer.

**Which text.** The head of `main`, from the merge of pull request 8 on. Not commit `c4d7d33`:
round 6 read that commit, and its decisions, ruling 44 and the changes they made came after
it, on pull request 8.

**What has been read, and by whom.** Cold readers, each a separate session that the
coordinator started with no context, or that the owner started ([who decides what](#who-decides-what)).
Four rounds read `spec/` before the rewrite under rulings 40 to 43.
[Round 5](proposals/reaudit/round-5/README.md) read the rewritten specification at commit
`d0c425f`, with six readers: four of `spec/` alone, one of the whole repository, and one who
checked the interface against the published Model Context Protocol. Two verifiers then checked
round 5's fixes. [Round 6](proposals/reaudit/round-6/README.md) was one reader, Codex, started
by the owner, who read the whole repository at `c4d7d33`, with round 5's changes in it, and
wrote six findings and a critique of the design; three reviewers that it tasked itself helped
it, and they are not separate cold reads. [Round 5's record](proposals/reaudit/round-5/README.md)
has the counts of rounds 2 to 5, and the [re-audit](proposals/reaudit/README.md) has round 1's
and links each round. Every finding of the six rounds is resolved and logged. What round 6 and
ruling 44 changed has been read by its editors, and not by a cold reader.

**How the readiness gate now reads (ruling 38, point 4).** The gate of two consecutive clean
rounds of cold reading is withdrawn: four rounds showed that reading alone does not reach it.
The scenario reader found fifteen gaps in each of rounds 3 and 4, where features that were
specified apart meet. In round 5 the readers who walked scenarios on one engine and across
machines reported 16 and 23 findings. Convergence is judged instead on two things: the latest
round of readers reports no unsafe finding, and the reference model and the conformance
vectors, once they exist, agree with the promises. Round 5 reported ten unsafe findings, and
all ten are fixed; the verifiers of its fixes classed none of theirs as unsafe. Round 6
reported two findings that its reader classed as wrong and safety-critical: specific
procedures that could not be followed without breaking a general safety promise, a conflict
that round 5 classed as unsafe (M32). Both are fixed, so no such finding is open, but round 6
was not clean. The first of the two things is judged next on the read that follows the first
build, and the second on the build itself. A reader is asked to report every finding, and to
mark as unsafe any by which an engine that keeps every promise could grant access it should
not, leak a secret, lose or repeat an effect, or report a wrong result, and also any such
conflict between a procedure and a safety promise.

**Known to be open.** Do not report these again unless there is something new:

- Secrets. The specification has them since ruling 41
  ([secrets.yaml](../spec/promises/secrets.yaml)). What its guarantee does not cover is listed
  in [spec/README.md](../spec/README.md).
- The full shape of an inspection result, which is settled with the conformance vectors
  ([what the core package leaves](../spec/core/README.md#what-this-package-does-not-settle)).
  Since ruling 41 (item 24) a diagnostic is SARIF's `result`, reduced, and a condition and a
  record each have a core schema.
- No promise has a conformance vector. No reference model and no engine exist.
- Later work is not part of v0 (ruling 38, point 6): no operation arranges, through the
  engine, work that outlasts its invocation.
- Policies, composite Conduits, requirement composition, `obliges`, re-exports and rename are
  not in v0 (ruling 41). Policies are a candidate access feature for a later version (A66).
- `tools/check_schemas.py` does not reject an item selected for `get-secret.v1` at an `http`
  URL whose host is not a loopback address, nor an item that is run for an Interface or a Pool.
  The [schemas README](../spec/schemas/README.md) says what it checks.
- A consumer that can speak only stdio. Round 5 rejected a finding on it (M78 of its
  [decisions](proposals/reaudit/round-5/DECISIONS.md)): every engine accepts the token in the
  `Authorization` header, so an existing stdio bridge that sends it works under all of them.
- Two limits of delegation that v0 keeps on purpose, and that it states plainly since ruling
  44 (option a of D03 and D04 of round 6): a bound does not limit the filters of the Conduits
  that its holder opens, and access to management cannot be narrowed to one template
  ([what access does not narrow](../spec/README.md#what-access-does-not-narrow), ACC-029,
  ACC-030). Narrow provisioning goes through a broker, a trusted operation of a Resource whose own contract
  allows only what it should. S1 and S2 measure whether that is good enough.
- What a client and a model see. The protocol lets a client keep a protocol error from its
  model, and round 6 found no further defect in IFC-012 or IFC-025 on it
  ([its report](proposals/reaudit/round-6/reader-1.md#the-two-interface-questions-singled-out-by-the-tracker)).
  Which clients show a refusal to their model is measured in S2 (E17 of round 6).
- A request repeated without its idempotency key is new work (OPS-011). Whether a client keeps
  the key across the retries it makes is a property of the client, measured in S4 (D02 of
  round 6), and not a gap in the language.

**Left for a reader by the records.** The open questions after the rewrite left for round 5 a
check against the protocol and
[five points for a reader](proposals/ruling-41/open-decisions.md#left-for-round-5), the log of
ruling 42's language group
[two more](proposals/ruling-42/log-language.md#needs-a-decision-or-a-later-stage), and item 5
of [ruling 42's integration log](proposals/ruling-42/log-integration.md#needs-a-decision) six
small points. Most are settled since, and need not be reported again:

- The check against the protocol was made by round 5's reader of the protocol
  ([report](proposals/reaudit/round-5/reader-6.md)), who also checked the notification of a
  changed tool list on `subscriptions/listen` (IFC-038), which the language group's log left
  for that check.
- A control in progress when the engine retires its instance records its outcome and stores
  nothing (REV-010, LIF-015): M35 of round 5.
- MAN-017's check of a place whose schema states none of `type`, `enum` and `const`, which the
  language group's log and item 5 both name: M30 of round 5.
- Of item 5, a value that begins with `()` (PRV-023), the rule that no record holds `identity`
  (PRV-024), and the end of the connection of an instance whose node is declared lost (SEC-013)
  were settled by
  [ruling 42's own verification](proposals/ruling-42/log-integration.md#verification); a stop
  that a retired provider holds (PRV-021), and whether SEC-023 stops trying a read that keeps
  failing, which it does not, since SEC-012 reads again before each request, by
  [the items that waited on ruling 43](proposals/ruling-43/log-integration.md#the-items-that-waited-on-this-ruling).

The four points that were left open after round 5 are decided by ruling 44, as round 6's
critique recommended ([its four open points](proposals/reaudit/round-6/design-critique.md#the-four-open-points)):

- A default that a type states for a hook it does not require is ignored: a template that
  selects no implementation for the hook has none (TPL-004), and installation or resolving
  reports the default, when it reads the type, in a diagnostic that does not reject (CTR-004).
- A named argument that a Package repeats, such as `-e`, cannot be bound (MAN-010). That waits
  on a real package that needs it, in S7 and A63. The schema of a server's arguments has
  `isRepeated`, a boolean taken from the registry's `server.json`, and no promise says how one
  key binds the several values of a repeated argument; the independent check found that
  smaller gap, which waits with it.
- The cause that PLC-024 has a record state, where no invocation holds it, gets a portable
  field of the record with the conformance vectors (A39).
- The late onward request of an earlier hook attempt is decided for the latest caller
  (OPS-028). That is kept for v0, tested (E20 of round 6), and revisited with local
  enforcement (A65).

Settled on 2026-11-09, by the first build: the two cases that the independent check named
beside finding 6-1-01 and did not check, IFC-010's reading of -32602 as failed whatever the
operation's effect, and IFC-046's statement that the protocol has a server reject -32020 before
acting. [Work order 018](proposals/first-build/check-plain-errors.md) found that neither code
proves that a server has not acted, and work order 015 corrected both promises under the
coordinator's decision 8a of [increment 2](proposals/first-build/increment-2/decisions.md),
which the owner can strike. The editors who
applied ruling 44 left five smaller points for a decision, which
[the integration of round 6](proposals/reaudit/round-6/log-integration.md#left-for-a-decision)
lists: whether a replace may be read from a directory, a key of an ordinary operation's own
root `$defs` that equals a copy's key in a tool's bundle, how INS-010 compares two
destinations, a rule of the checker for SEC-005, and a vector for the escaped form of a `$ref`
to a copy. The integration's
[verification](proposals/reaudit/round-6/log-integration.md#verification), by two further
readers of the result, fixed what they found and left the second of these open, with options.

**To look at in particular (ruling 38, point 11).** The interface follows the Model Context
Protocol on errors: a call to a tool that the engine does not present is answered with the
protocol's error, code -32602. A result has `isError` true where the request ended with a
reason or its outcome is failed or unknown (IFC-025). Ruling 41 (item 6b) put that in place of
the earlier rule, under which every result that carried none of its operation's declared
outputs had it. Two things follow that the owner wants examined. The protocol lets
a client keep protocol errors from its model, so an Agent's model may never see that a request
was refused. And an open proposal to the protocol, SEP-2145, would turn an unknown tool into
an error result, which would reverse the first. Round 5 changed the text on both (M71 and M75
of its [decisions](proposals/reaudit/round-5/DECISIONS.md)). A result whose `isError` is true
now begins its `content` with one text block that states the reason or the outcome (IFC-025).
A call to a tool that is not presented is answered as the protocol prescribes at the revision
in use, -32602 at the baseline revision, and the error's `message` says that no tool of that
name is presented to the instance (IFC-012). Round 6 looked at both as they stood at
`c4d7d33` and found no further defect: the message helps a client that shows the error to its
model and cannot make one do so, which is the protocol's limit, and SEP-2145 was still open on
2026-11-07. What real clients show is measured in S2 (E17 of round 6).

**To look at after round 6.** The text that round 6 and ruling 44 changed has not been read
cold. [Round 6's decisions](proposals/reaudit/round-6/DECISIONS.md) say what changed, and the
logs beside them list each promise. Among them are interface promises that rest on the published
protocol at the baseline revision: the reading of the error -32021 (IFC-007, IFC-010, IFC-045,
IFC-047 and OPS-007), the tool definition presented as a self-contained bundle, narrowed by the
target's template (IFC-033, IFC-012), and the root of an ordinary Operation's schemas (CTR-012).
The independent check read the published protocol on each of the three; the editors' logs say
what they read. Round 5's own decisions that the owner may want to read first are in
[its record](proposals/reaudit/round-5/README.md#known-after-the-round), where V2-15 is now
reversed by ruling 44.

**Who decided what.** The next section. In short: rulings 1 to 46 are the owner's. Ruling 39
accepted every logged decision of the coordinator and choice of an editor made before it, and
ruling 41 the planner's choices for the rewrite. Ruling 44 accepted the coordinator's
recommendations on round 6. The others made since ruling 39 have the standing of decisions the
owner can strike, and the next section lists where they are logged. Among them are the
coordinator's decisions on round 5's findings and on the findings of its verification, the
choice that its integration made on PLC-023 and LIF-036, and, for round 6, the details that
the coordinator fixed after ruling 44 and the choices of the editors who applied it. A reader
who disagrees with any of them should say so.

**Where findings go.** One report per reader, in a new directory,
`docs/proposals/reaudit/round-7/`, the next after
[round 6](proposals/reaudit/round-6/README.md), in the form of the earlier reports: an ID for
each finding, a severity, the promise IDs, short exact quotes, the situation, and the least
change that fixes it. A reader changes nothing under `spec/`. The owner rules, or the
coordinator decides under ruling 37, and the decision is logged beside the report.

### Who decides what

The owner makes the rulings, logged [below](#ruling-log). In ruling 37 the owner also said that
the findings of the cold readers are to be resolved without waiting for a ruling on each, and
in ruling 38 confirmed the coordinator's reading of that: a finding goes back to the owner only
where settling it would change what Conduit is for. So the specification has changed in three
ways, and the records keep them apart:

- **By ruling.** The rows of the ruling log, each with its record under
  [proposals/](README.md#records-of-rulings).
- **By the coordinator's decision on a finding**, logged with its reason where the finding is
  recorded: under [proposals/reaudit/](proposals/reaudit/README.md) and
  [proposals/docs-audit/](proposals/docs-audit/README.md).
- **By an editor's choice**, where a decision left the wording or a detail open, logged under
  "Choices" in the editor's log.

Ruling 39 accepted every decision of the coordinator and choice of an editor logged before it,
including the choices that the passes applying rulings made before ruling 37. A later one has
the standing of a decision the owner can strike, until a ruling accepts it. Ruling 41 accepted
the planner's choices P1 to P20 of the rewrite's
[work order](proposals/ruling-41/PLAN.md#planners-choices), with P2 changed. The choices that
the editors of the rewrite logged have that standing: no ruling has accepted them.

The decisions and choices made since ruling 39 are logged in these places, and each has that
standing: the coordinator's 43
[decisions](proposals/ruling-41/open-decisions.md#settled-by-the-coordinator) on the open
questions after the rewrite, which were put to the owner with ruling 42 and of which the owner
struck none; the coordinator's decisions in the sections "After the ruling" of the records of
[ruling 42](proposals/ruling-42/README.md#after-the-ruling) and
[ruling 43](proposals/ruling-43/README.md#after-the-ruling), the second with the names it
coined for the owner to strike; the
[choices](proposals/ruling-43/log-integration.md#choices-of-the-verification) of ruling 43's
verification; the choices that the editors' logs beside the records of rulings 41 to 43 list;
and the decisions of [round 5](proposals/reaudit/round-5/DECISIONS.md) of the re-audit, with
the choices that its [fix log](proposals/reaudit/round-5/log-fixes.md) records: the
integration's choice on PLC-023 and LIF-036, and the decisions of its verification.

[Ruling 44](proposals/ruling-44/README.md) accepted the coordinator's recommendations on
[round 6](proposals/reaudit/round-6/DECISIONS.md). The details that the coordinator fixed in
applying them after the ruling, which that record marks as such, and the choices that the
editors' logs beside it list, have the standing of decisions the owner can strike.

[Ruling 45](proposals/ruling-45/README.md) left to the coordinator how the roadmap is shown
(A72). The coordinator's decisions on it are in
[the log of the roadmap](proposals/ruling-45/log-roadmap.md#the-coordinators-decisions), and
the owner can strike each.

[Ruling 46](proposals/ruling-46/README.md), point 4, extends that standing to what the first
build finds: each increment logs its findings and the coordinator's fixes under
`docs/proposals/first-build/`, and the owner can strike each fix. A finding whose fix adds or
removes a feature, or changes what Conduit is for, goes to the owner.

On 2026-11-09 the owner accepted the coordinator's decisions in the first build for now, and
asked to go over the impactful ones together later: they are listed, most consequential first,
in [the decisions to go over with the owner](proposals/first-build/to-review-with-the-owner.md).

[Ruling 48](proposals/ruling-48/README.md) makes the coordinator the admin session: Codex
workers carry out work orders that the coordinator writes, under `docs/work-orders/`, and the
decisions an order applies are the coordinator's, logged and strikeable as above. A choice a
worker makes for itself is listed in its report, and the coordinator accepts or undoes it
before it merges ([work orders](implementation/WORK_ORDERS.md)).

| Word | Who it is |
| --- | --- |
| The coordinator | The Claude session that the owner asked to write the language, and that keeps this tracker. It decides a finding where ruling 37 lets it, starts the readers and editors, checks their work and commits it. Since ruling 48 it is the admin session, which writes the work orders that Codex workers carry out. It is not the owner. |
| A worker | A Codex session that carries out one work order (ruling 48). It changes only what the order allows, and commits nothing. |
| A cold reader | A separate session that the coordinator starts with no context. It is given only `spec/`, or the whole repository, and writes a report, which is kept. It changes nothing. The counts of findings for a round are the counts of these reports. In round 2 the editors of the closing pass and of the last pass also started three readers of their own, whose findings are rows of [the closing log](proposals/reaudit/round-2/log-closing.md) and of [the log of the last pass](proposals/reaudit/round-2/log-tail.md), outside the 125. |
| An editor | A separate session that the coordinator starts to apply decisions to named files. "Editor P" and "Editor S" in round 2 were the editors of the promises and of the core package and schemas. An editor logs each choice it makes, and lists what needs a new decision without deciding it. |

The work is committed and pushed at each milestone, to a branch with its own pull request, as
the owner confirmed in ruling 38.

## Waiting for the owner

One list of the open points that only the owner can settle. Beside them, the readiness gate
needs the owner: G8 asks the owner to close the open questions of the plan audit or put them
out of the builders' scope, and G11 is the owner's sign-off
([build process](implementation/INDEPENDENT_BUILDS.md#readiness-gate)). The proposals that
A18, A65, A66 and A67 ask for come to the owner once they are written.

| What | Where it is recorded |
| --- | --- |
| A20: rule on the scope of authorization in v0, the plan audit's disagreement D5, with its cross-path, issuer-independence and provider-scope questions. The plan audit has it informed by the bypass probe (A7), whose row is in the [roadmap](../ROADMAP.md#experiments-and-probes). | [Action items](#p2--prune-and-re-baseline-under-the-scope-guard), [findings](archive/plan-audit-history.md#findings) |
| A21: five of the eleven known inconsistencies stand as written in files that are marked as not binding or are not edited; whether to drop them is the owner's call. | [Action items](#p0--make-the-language-concrete-and-complete), [known inconsistencies](archive/plan-audit-history.md#known-inconsistencies) |

Settled, and no longer waiting: ruling 46 settled the nature of the first build (A68) and the
question of what S2 to S9 run on, which the roadmap had left open. Ruling 43 settled the last two questions that the rewrite left
for the owner, D5 and D20 of the
[open questions after the rewrite](proposals/ruling-41/open-decisions.md#for-the-owner). Q5 of
the [multi-machine proposal](proposals/multi-machine/README.md#q5-who-on-the-engines-machine-can-get-past-a-gate),
which it accepted, also settled the optional hardening of the engine's own environment that
item 7 of the
[verification of ruling 42](proposals/ruling-42/log-integration.md#still-needs-a-decision) left
to the owner: the engine's own operating-system user is inside the domain of trust, an Agent
not trusted that far runs under a runtime provider that isolates it, and the user a process runs
as stays each engine's (DL16). See [ruling 43](proposals/ruling-43/README.md).

Also settled: D06 and D12 of round 6. After ruling 44 accepted both, an independent check found
facts that the critique left out, and the coordinator put each back to the owner with a revised
recommendation, which the owner accepted on 2026-11-07 ("Agree on both"; [ruling 44, after the
ruling](proposals/ruling-44/README.md#after-the-ruling)). They are applied ([log](proposals/reaudit/round-6/log-d06-d12.md)).

## Purpose

Ruled 2026-11-05 (ruling 15).

This section is kept in the words of the ruling, except one sentence that ruling 39 amended.
Since rulings 27 to 29 there is no separate grant: a grant is the access an open Conduit carries, to issue one is to open a Conduit, and to
revoke one is to close it. "Provision" is setting up, and a plugin is an implementation. Read
the section with those words.

**The relationship is the product.** Infrastructure languages such as Kubernetes, Docker Compose
and Terraform declare workloads, which do what their specification says. An agent does what it
decides. An agent-first language therefore declares what surrounds the agent: what it may reach,
under which limits, and who answers for what it did. That is why the family that carries grants
gives the project its name.

- **First user:** a single developer running agents from several vendors on their own machine.
- **Teams:** not in v0, but the language must be able to support them later, most likely through
  Interfaces. No rule may assume there is exactly one owner.
- **Enforcement:** a future, optional part of the language. By default a restriction applies to
  what passes through Conduit. An author can opt in to a selected isolation framework through the
  language and plugins, as with Docker, to have it enforced.

What is distinctly Conduit's, and must survive the rework:

- **Establish once, share under restriction.** A login, checkout or memory is owned by a Pool and
  used by agents from any vendor, each under its own limits, with the credential never handed
  over.
- **The Conduit is an object.** Each agent's access to each resource can be inspected, narrowed
  and revoked independently of both ends. (Amended by ruling 39, point 7: the ruling said "the
  grant".)
- **Agents can operate the system.** An agent can provision a resource, start a peer and delegate
  a narrower slice of access, through the same contract a human uses.
- **Nothing implicit spends tokens or repeats an effect.** Actions are explicit, an unknown
  outcome stays unknown, and there is no background reconciler or automatic wake.
- **Private and shared stay separate.** An agent's subscription and context are its own; anything
  shared goes through a Resource.

Not differentiating, and so free to simplify: the graph engine, the package system, the schema
profile, persistence and export. For graphs and schemas the owner's standing instruction
(ruling 23) is to do nothing revolutionary and to rely heavily on existing frameworks and
paradigms; the same applies to packages (ruling 24).

### Preservation tests

Accepted 2026-11-05 (ruling 17). Each rework decision is checked against them.

Since rulings 27 to 29 a grant is the access an open Conduit carries, and "provision" is setting
up; read the tests with those words. The tests themselves are not reworded.

| # | Test |
| --- | --- |
| 1 | An agent is a participant, never a workload: the language does not specify its reasoning. |
| 2 | Every access is a relationship with a grant that can be inspected and revoked. |
| 3 | A shared thing is owned once and never copied to an agent. |
| 4 | Nothing assumes a particular vendor's agent. |
| 5 | Every management action is available to an agent under a grant, and delegation only narrows. |
| 6 | Nothing starts model work or repeats an effect implicitly. |
| 7 | Everything an agent does through Conduit is attributable. |
| 8 | No rule assumes exactly one owner: a human's access is an Interface holding grants. |
| 9 | A restriction always reports whether it is enforced or only mediated, and enforcement is opted into through the language. |

Two additions to the shared conventions were accepted with them. There is a seventh convention,
for test 6: nothing starts model work or repeats an effect implicitly. And the access convention
states, for test 5, that an agent may issue grants to others only within a bound it was given.

## Current direction

- **Conduit innovates only on connection and access** (ruling 40). It owns its documents,
  connection (Conduits), access, accountability (what must be recorded) and the shape of its
  contracts. Everything else is a provider behind a contract, swappable by the user wherever
  possible: secret stores, the runtime that runs implementations, the machines and their
  placement, the source of packages, and later identity, isolation and the storage of records.
  Where an existing tool or standard does the job, Conduit adopts it.
- **It works across machines.** Coordination of agents and resources on several machines is part
  of what Conduit is for. Conduit owns what placement, a lost machine and a failed request mean
  for access; providers own the mechanics. One control plane decides every request in the first
  version; local enforcement with a bound on how long a revocation takes is likely needed in v1.
- **The language is the core; the runtime fulfills its promise.**
- **Conduit is a configuration language, not a behavioral specification.** The language fully
  determines what a set of documents means. Behavior beyond a few shared conventions lives in
  type contracts, providers and a conformance document ([layered claim](#layered-claim)).
- **Current work makes the language concrete and provable**: the specification, written as
  numbered promises for the language, the shared conventions and the implementation interface;
  schemas; an executable reference model in Rust; public conformance vectors; and model
  checking.
- **Proof of sufficiency** is two independent builds, by fresh sessions of two coding agents
  from different vendors, Codex and Claude. Each sees only the specification and the vectors,
  and never the other's work. They cover the language layer, the shared conventions and the
  implementation interface; one reference engine is built separately. They do not start until
  the [readiness gate](implementation/INDEPENDENT_BUILDS.md#readiness-gate) passes; the owner
  wants high confidence first.
- **Language defects are closed before the builds.** The first sweep found 167 against a stronger
  claim; they were re-triaged by layer after the layer decisions
  ([defect closure plan](archive/plan-audit-history.md#defect-closure-plan)).
- **Any code is Rust**, under `experiments/`, and under `packages/` for the first providers, one
  crate per provider (ruling 40), until the reference model has a home.
- **The vocabulary was decided at once and applied with the rework** (ruling 16), so that names
  and behavior were fixed together and the text was rewritten once. The pass merged concepts as
  well as renaming them ([vocabulary record](proposals/vocabulary/README.md)).
- **The relationship is the product** ([purpose](#purpose)). Rework decisions are checked against
  the preservation tests.
- **The acceptance scenario is a first proof, not the scope** ([scope guard](archive/plan-audit-history.md#scope-guard)).

## Ruling log

All rulings are the owner's, in this order: rulings 1 to 37 on 2026-11-05, rulings 38 to 43
on 2026-11-06, ruling 44 on 2026-11-07, and rulings 45 and 46 on 2026-11-09. Each row is the
coordinator's summary of the owner's messages, with the owner's own words quoted where they are
marked as quoted; the owner confirmed that labelling in ruling 39, point 15. The record linked
from a row holds the owner's words in full where the ruling was given point by point.

| # | Ruling |
| --- | --- |
| 1 | The audit is the basis for action items and priorities. Unneeded reference clones are removed. |
| 2 | Vocabulary stays provisional until the language is shown to be expressive enough. |
| 3 | Exploratory code may be written and run whenever useful, in Rust only. |
| 4 | Claude and Codex collaborating on one project on one machine is the starting acceptance scenario; success there must not narrow the design below its broad scope. |
| 5 | Multi-system support: settle placement in the language first, then build separated components under a simulated network. |
| 6 | The language is the core and the runtime fulfills its promise; whatever makes the language more concise, concrete and useful should be done. |
| 7 | Placement decisions PL1 and PL3 to PL8 accepted, including that a system cut off from management starts no new managed work in v0, that types declare their location needs, and that a declare-lost operation is needed. |
| 8 | Instantiation: a closed, declared input list; structure is never an input; one shape everywhere; explicit inputs with no shorthand; each input names its destinations; the instance stores only the result; name, owner and placement are standard arguments. This also settles PL2. |
| 9 | The nine-scenario experiment program, its order and its parallel work are accepted. Old experiments are rewritten or removed as replacements land and need not be preserved. |
| 10 | Sufficiency must be provable to a user of the protocol. The executable reference model in Rust is accepted. A full rebuild is wanted if proof needs it: Codex and Claude each build the whole engine and do not check each other's work. |
| 11 | The independence rules for the builds are accepted, and the session that wrote the language does not build an engine. |
| 12 | The repository is to be prepared with an explicit build brief, with builders on separate branches. All identifiable language defects are to be closed first, and the builds wait until success is very likely. |
| 13 | The layered claim is adopted: the language determines what declarations mean; behavior beyond shared conventions lives in type contracts and a runtime profile. Layer decisions come next, then the defects are re-triaged. |
| 14 | Layer decisions LD1 to LD5 accepted as recommended. Setup graphs follow the Docker Compose and Terraform model: the language defines the dependency structure and the engine decides how to walk it. The independent builds cover the language layer, the shared conventions and the plugin interface, with one reference engine. Before fixing proceeds, the language's purpose and its agent-first differentiation are to be discussed and then preserved through the rework. |
| 15 | Purpose: the relationship is the product. The first user is a single developer; teams come later and the language must be able to support them, likely through Interfaces. Enforcement is a future, optional part of the language, opted into through the language and plugins that select an isolation framework. |
| 16 | The vocabulary is decided now, folding in the reviewer's simplification pass on pull request 5, and applied alongside the rework to avoid churn. This replaces the earlier ruling that vocabulary comes last. Then the defects are reassessed and fixed. |
| 17 | Accepted: the nine preservation tests; a seventh shared convention and explicit agent delegation in the access convention; the form of the specification (NS1 to NS4); and vocabulary decisions V1 and V3 to V8. V3 also settles that a Conduit is pairwise: one consumer, one target, opened and closed. V2, how a document marks a template, is under discussion: two kinds per family is rejected as cumbersome, since a template and its instance are two variants of one kind and the template flows into the instance. |
| 18 | Vocabulary decision V2: one kind per family. A document that declares inputs and has no `from` is a template; a document with `from` is an instance of it. Every instance therefore comes from a template, and a reference is a kind and a name. |
| 19 | The first batch of fixes, relationships and access, is accepted: decisions RD1 to RD8 and rules RR1 to RR16. A reference may give the name alone where its field accepts exactly one kind. The owner also stated that issuing a Conduit to a live instance must be a core part of the language. |
| 20 | Accepted: Conduits to live instances (CON-012 to CON-014), with consumer and target as standard arguments of the Conduit family; the `layer` field on a promise; and the second batch of fixes, operations, outcomes and revisions (decisions OD1 to OD5 and rules OR1 to OR9). |
| 21 | The third batch of fixes, declarations, is accepted: decisions DD1 to DD5 and rules DR1 to DR13. On DD2 the owner asked what a document is concretely before moving on. |
| 22 | After a concrete explanation of what a document and a submission are, decision DD2 stands as accepted: a submission is a set of documents, checked together, with order derived from their references, and submitting only registers. |
| 23 | The fourth batch of fixes, graphs and the schema profile, is accepted: decisions GD1 and SD1 to SD3 and rules GR1 to GR4 and SR1 to SR5. Every type must state its location needs, including none. Standing instruction: for graphs and schemas, do nothing revolutionary and rely heavily on existing frameworks and paradigms, since they are not what differentiates Conduit. |
| 24 | The fifth batch of fixes, packages, first start and conformance, is accepted except decision PD1: decisions PD2 to PD5 and rules PR1 to PR8. Applying the existing-paradigms instruction to packages is accepted with it. PD1, how a package's exports are named and referred to, needs more research and discussion of naming patterns. |
| 25 | Naming direction: a package's identity is address-like, as in Go, because people will have entirely custom package setups; references to a package's exports are fully qualified, with no aliases; versions are pinned once per project, as Go and Cargo do; contracts having an identity independent of the package release is acceptable; the reference syntax is to be discussed. The owner is concerned that so large an inconsistency was found so late, and asked for an audit of the language for any other consolidation that is needed. |
| 26 | The owner raised that connections must work across package versions: the two ends of a Conduit may come from different versions, instances run different versions, and compatibility should be as wide as requirements allow. Whether packages must be recorded directly in templates and instances is to be discussed. |
| 27 | Accepted: the naming decisions NA to NF and the answers on versions across connections. A reference to a package's export is its address, `#` and the export's name; one manifest per project holds versions and digests; a registered template records its packages; any number of versions can be installed; a contract's version is part of its name, changes only compatibly, and a type can offer several; a breaking package version takes a new address. Consolidation: grants are merged into Conduits, and Pool entries are folded into the mechanism Conduits use. The remaining consolidation findings are to be discussed. |
| 28 | Consolidation rulings: every recommendation of the consolidation audit is accepted except two that are to be discussed. The right to open Conduits to a target is itself an open Conduit to the `access` Resource, so every permission is a Conduit. Open: whether Pool ownership should itself be a Conduit; and the common lifecycle, including whether a closed Conduit can be reopened at all or is simply retired. |
| 29 | Accepted: Pool ownership stays one recorded fact, fixed at creation, that permits nothing; every power over a Pool or its Resources is an open Conduit; a right to open stated for a Pool covers the Resources it owns; a Pool's entries are created with the Pool and "provision" becomes setting up that Resource. Accepted: one forward-only lifecycle for all five families, whose states the owner named registered, ready and retired; a closed Conduit is retired and never opened again; there is no suspended state; setup never replaces a retired declared Conduit or entry Resource, and a replacement is a new instance made only on an explicit request; Conduits keep the verbs open and close; running, paused and idle belong to type contracts. |
| 30 | Accepted: all fourteen decisions on the four remaining shared conventions ([record](proposals/conventions/README.md)). An instance is retired from the moment the request is dispatched, and is unregistered only once its shutdown is confirmed; one authority covers an object's whole life; first-start objects are permanent; a difference between declared and observed state is reported and never repaired without a request; a closed list of events is always recorded, and only `root` may purge; model work starts only through an operation whose contract marks it, never from setup. The owner also ruled the order of what remains: one more batch of rulings, then the promises are consolidated and the whole specification is audited again. |
| 31 | Accepted: the last batch ([record](proposals/schemas-and-interface/README.md)), every recommendation, with one amendment. Documents: one `implementation` map; camelCase keys and `apiVersion`; structured value references from five sources; a runtime is a pair of start and stop operations; an optional `retire` graph; controls are defined only by a type; the Conduit template holds one capability, its operations and one `requirements` map; setting up a Pool sets up its entries; contracts are documents of four kinds. Interface: the Model Context Protocol is the wire, an operation is a tool, Conduit adds keys in `_meta`, a plain server's answers map to outcomes by a fixed table, and the engine presents each consumer its permitted operations as tools. Amendment, at the owner's request: one baseline revision, 2026-07-28, that everyone supports, negotiation above it, and a new version of the specification may raise the baseline. The implementation-defined list of twelve entries is accepted. `conduit.dev` stays a placeholder until the owner names a domain, which must happen before the engine builds. |
| 32 | Accepted: all 37 decisions of the first re-audit ([record](proposals/reaudit/README.md)), as recommended. Among them: every request carries the Conduit and the restrictions that apply, and an onward request names the invocation it serves; a plain server never counts as deduplicating; a delegate's bound limits the policies it may apply; a repeat of an identity is the new attempt; one order among the four reasons; a location's facts are stored configuration; a control is an Operation contract; and the core package, the JSON Schemas and the manifest format are to be written as normative parts of the specification and reviewed as one batch. |
| 33 | Accepted: the 26 follow-up decisions from consolidation pass 1 and the editors' five readings ([record](proposals/reaudit/README.md#follow-up-decisions--accepted)), except F3. The owner ruled that until there is a domain the names come from this repository: the core package is `github.com/shrav-k/conduit/core`, a document's `apiVersion` is `github.com/shrav-k/conduit/v0`, and Conduit's `_meta` keys have the prefix `io.github.shrav-k.conduit/`. This replaces the `conduit.dev` placeholder and settles decision X1 of ruling 31. |
| 34 | The owner asked for a way to change the names later, since they come from the repository only until there is a domain. Done in the repository: `spec/names.yaml` is the one place the names are defined, `python3 tools/set_names.py --check` verifies that every file under `spec/` agrees with it and refuses a retired name, and the same tool rewrites every occurrence for a new domain or repository. Proposed and not yet ruled: a rule in the language that the names belong to a version of the specification, so that a later version can change them. |
| 35 | Accepted: the names belong to a version of the specification. Each version states its names in `spec/names.yaml`, and only a new version may change them, as with the protocol baseline. A document's `apiVersion` says which version it is written for, so an engine that supports two versions loads documents of both. To be written as promises once the editors applying ruling 33 have finished with the files it touches. |
| 36 | The owner asked for the work to be tracked on a pull request ([#6](https://github.com/shrav-k/conduit/pull/6)), and for the vocabulary research pull request ([#5](https://github.com/shrav-k/conduit/pull/5)) to be merged or closed and this branch rebased, keeping a record of the reviewer's contribution and discarding what is no longer needed. Done in the tree: the reviewer's simplification pass is kept as written with a note of its outcome; the precedents and sources are kept as evidence; the candidate tables and walkthroughs, which the rulings replaced, are removed, with links pointing at the last commit that held them. The owner then confirmed the merge: #5 was merged into `main` with a merge commit, which keeps the reviewer's commit under their name, and this branch was rebased onto it. |
| 37 | Accepted: the core package, the document schemas and the manifest format as drafted ([record](proposals/core-and-schemas/README.md)), with five changes: a control that sets a location's facts; requests name objects by reference only; a bound is rejected on a Conduit whose target is not `access` or when applied twice; a package outside the core may not mark an operation as reserved to `root`; and names are unique across kinds within a project. Also accepted: the definitions of the four families, model work, enforced and mediated, project and published. The owner further ruled that findings of the cold readers are to be resolved without waiting for a ruling on each, that the documents outside `spec/` are to be audited once more, and that the repository is to be made ready for a cold read by Codex after these rounds. How each item is applied is in [the instructions](proposals/core-and-schemas/APPLY.md). |
| 38 | After four rounds of the re-audit the coordinator put fourteen points to the owner ([record](proposals/ruling-38/README.md)). Ruled: a secret is referred to and never written as a literal, and a secret type is to be explored before anything is written; the six behaviors of the older design that the specification does not carry are dropped; a composite Conduit opens whole or not at all at setup and its components are independent afterwards; the gate of two clean rounds of cold reading is withdrawn, and convergence is judged on no unsafe finding in the latest round and on the reference model and the vectors; no domain need be named before the engine builds; later work is dropped from v0; a policy that makes an actor's requests to an engine-implemented target unavailable stands, and `apply-policy.v1` warns of it; handing an engine a directory installs no package. Accepted, of the coordinator's decisions: a retired consumer's Conduits stay open for its retirement alone; the resolved form in a step and in an onward request; the interface following the protocol on errors, to be looked at again; and eight changes to what earlier rulings accepted, listed in its record; those that changed what an earlier ruling had accepted are under later amendments. Confirmed: the coordinator's reading of ruling 37, and committing and pushing at each milestone. Pull request 6 is merged by squashing. |
| 39 | Before the cold read by Codex the coordinator put fifteen points to the owner ([record](proposals/ruling-39/README.md)). Ruled: the secret type is not decided: "I feel we need more research into how secrets are handled by cloud providers like AWS and in kubernetes before making a determination"; accepted for it, as direction, that only a Resource reads a secret and only the root Interface may open a Conduit that reads one, that the engine sets a credential in a process's environment at start for servers that take one there, and that an Agent's own provider credential is delivered the same way. Accepted: a composite Conduit opens with a whole decision and an outcome for each component; every logged decision of the coordinator and choice of an editor, before and since ruling 37, and the three corrections to the interface; "the grant is an object" becomes "the Conduit is an object"; an assessed state for each readiness condition, for the owner to confirm; the removal from the tree of the material that the specification superseded, with links pinned to `main`; the historical sections of this tracker moved to the archive; the names of two private repositories removed; the stale branches deleted; the personal address on the merge of pull request 6 left as it is. The sentence on who made the external audit is struck. The rows of this log are labelled as the coordinator's summaries. |
| 40 | Conduit innovates only on connection and access ([record](proposals/ruling-40/README.md)). In the owner's words: "The point of conduit was always to leverage existing infrastructure where possible and only innovate on connection and access"; "We should not innovate on the architecture ourselves just the contracts". Ruled: Conduit owns its documents, connection, access, accountability (what must be recorded) and the shape of its contracts, and everything else is a provider behind a contract, swappable by the user wherever possible, "leaving decisions to the user on architecture and existing tools as long as they meet requirements"; the runtime model becomes a runtime provider contract (run, stop, status); locations stay in the language but a provider reports them, places instances by label and reports a lost machine, while what placement and loss mean for access stays Conduit's; setup graphs reduce to declared dependencies, and an operation built from other operations becomes an ordinary implementation; a request that crosses machines carries cryptographic proof of its sender; one control plane decides every request in the first version, and "a robust version of B [local enforcement] will likely be a requirement for v1"; the changes are made now, before round 5 and the cold read by Codex. |
| 41 | The review of the whole design against ruling 40, and the secret type ([record](proposals/ruling-41/README.md)). Accepted as recommended: every decision of the [principles review](proposals/principles-review/README.md), among them setup and retirement as two hook operations in place of graphs; a runtime provider contract in the shape of CRI; placement by labels, with machines registered from what a provider reports; policies, composite Conduits, requirement composition, `obliges`, re-exports and rename removed from v0; results, cancellation, diagnostics, conditions, records, implementation items, schemas and identity in the standards' own forms; providers as packages that implement core provider capabilities, picked by the user; the secret type as a core capability that each store's package implements, read only by the engine before a run. "Accept all, though I don't necessarily like the name secret reveal for the contract, we should do another vocab audit as well before we finish up pr 7". Then, on the planner's twenty choices for the rewrite: "Accept all with p2 changed", so `graphs.yaml` is deleted and listed with its IDs in `spec/README.md` ([after the ruling](proposals/ruling-41/README.md#after-the-ruling)). |
| 42 | The questions the rewrite left open, and the names of the vocabulary audit ([record](proposals/ruling-42/README.md)). "I accept every suggestion except for 1 and 8, I'd like some more research and clarification on how to solve the multi machine issue and be forward looking even if some functionality doesn't exist in v0". Accepted: a request through a closed Conduit reports refused (D6); the engine's own runner passes six variables of its environment and a list set on it (D8); a secret Resource's type offers no other capability (D10); a stop is asked again when an instance on a node declared lost is unregistered (D11); after a restart the engine reads a remote connection's secret again for the setup's caller (D12); `run.v1` carries the identity mechanism's variables and files (D17); `retire-with-resources.v1` goes on past a Resource it cannot retire (R5); the twenty names, among them `secret-provider.v1` and `get-secret.v1`, node for location, idempotency key for operation identity and source for origin. The owner struck none of the coordinator's 43 decisions on the rest ([record](proposals/ruling-41/open-decisions.md)). D5 and D20 wait on research into working across machines. |
| 43 | Work across machines ([record](proposals/ruling-43/README.md)), after the [research](research/MULTI_MACHINE.md) and the [proposal](proposals/multi-machine/README.md) that ruling 42 asked for, and the coordinator's explanation of each question and of its cost on one machine and for the Model Context Protocol. "Agree all, prepare for the codex audit". Accepted: every server the engine does not run itself sits behind a gate that admits only an enforcement point with proof (D5); a secret header goes only to `https`, or to loopback on an engine that cannot fail over (D5, second part); packaged programs run only on the engine's own runner in v0 (D20); two standard forms of proof, the token form always accepted at the engine's address (Q1); failover stated as outcomes (Q2); providers run again when first needed after a restart (Q3); the shape of v1 recorded in A65 (Q4); the engine's own user said plainly to be inside the domain of trust (Q5); the name `identity` kept. |
| 44 | The owner's cold read by Codex, and building next ([record](proposals/ruling-44/README.md)). Round 6 of the re-audit returned six findings and a critique of the design in 22 points, D01 to D22; the coordinator summarized both, said which it would settle itself under rulings 37 and 38, and put four questions to the owner, each with a recommendation. "I accept all your recommendations, ask me if you need more rulings, once done we should discuss the nature of the first build and how we will use it to test our theses and refine the conduit language". Accepted: the next milestone is a working demonstration, the S1 scenario through a reference model in Rust, conformance vectors for the safety cases first and the first providers, each run measuring what a new author must do (D22); v0 says plainly that a bound does not limit the filters of the Conduits its holder opens and that access to management cannot be narrowed to one template, and narrow provisioning goes through a broker, a trusted operation of a Resource (D03, D04, option a); an Operation contract states no default implementation (D06); the tools presented show the limits that the target enforces (D07); replacing a template resolves it against the project submitted with it (D11); an actor may name an object in its own request by its resolved reference (D12); an ordinary Operation's schemas may use the whole schema profile at their root (D14). After an independent check, the coordinator put D06 and D12 back with revised recommendations, and the owner answered "Agree on both" ([after the ruling](proposals/ruling-44/README.md#after-the-ruling)): a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule; names stay the only address, LIF-031 stands, and the lifecycle actions take an optional expected ID, which reports conflict where the object that the name resolves to has another. Accepted as the coordinator proposed to settle them: the six findings; ACC-033 as one decision table (D18); reading paths in the entry points (D21); the evidence cases E01 to E20 and the qualification tables of D01 and D02 in the experiment program, with D05, D08 to D10, D15, D16, D19 and D20 as measurements; the four points left open after round 5; round 6's decisions recorded, applied and merged as one pull request, pull request 8. |
| 45 | A collaborator's two proposals, and the roadmap ([record](proposals/ruling-45/README.md)). The coordinator reviewed pull requests 9 and 10 of `allocmemory` and put five points to the owner. "Let's do as you recommend, but it seems from the overlap our roadmap for the experiments was not easily discoverable. After you merge these two do a pass over the docs and decide the cleanest way to display the roadmap". Accepted: the description of Conduit stays the specification's own (DA11), and one concrete sentence from pull request 9 follows it in the README, corrected to what this version gives; the components page is not kept, and six of its terms become rows of the core-terms glossary; pull request 10 is closed, and what is kept of it goes into the program: a baseline for S1 with today's tools (A70) and what a real client shows its model (A71), both now, the Rust loader as the first piece of the build (A43), and a probe of 1Password later (A63); each experiment states before it runs what it must show and how each measure is counted. The coordinator decides how the roadmap is shown (A72). |
| 46 | The first build ([record](proposals/ruling-46/README.md)). The owner asked what the first step should be and what to rule on; the coordinator proposed the strict loader in Rust (A43) first, with the pre-registration and the baseline for S1 (A70) beside it, and put five points. "I accept all your recommendations, let's proceed". Accepted: the first build is one codebase in Rust, a pure core, the reference model (A38), in which every decision cites a promise, and a thin shell that makes it runnable, which is the first slice of the reference engine (A50), so that S2 to S9 run on the same codebase; S1's real Agents are Claude Code and Codex, or two instances of Claude Code if one cannot reach an MCP server over HTTP with a token; round 6's suggested success criteria for a first author trial, with the comparison against A70, are the success criteria; each increment logs its findings against the promises, and the coordinator fixes them with a vector as it settles a cold reader's findings, the owner able to strike each, while a fix that adds or removes a feature or changes what Conduit is for goes to the owner; S1's first providers are the engine's built-ins, and the first packaged provider, a secret store, begins with S2. |
| 48 | What the first build's review put to the owner, and who does the work ([record](proposals/ruling-48/README.md)). After increment 1 and the baseline for S1 (A70), a review put four points to the owner. "Agree with all recs, except for 4 let's do fresh reads with codex instead for testing lack of context. I also want codex to do some of these experiments going forward as well so as to not burn all my tokens here but you are the admin session and should handle all of the meticulous planning and decisions." Accepted: the first build's Agent types turn off by default what an Agent reaches through its own login, each turned on for one Agent by an input of its template; in increment 3 the Agents share the owner's login, kept apart by flags, and each has a home and a login of its own once A4 and A5 have run, with A8 widened to Agents the engine runs; a `pattern` keeps to the tokens of JSON Schema 2020-12, core, section 6.4, read as ECMA-262 in Unicode mode; a fresh Codex session tests the success criterion that a competent reader completes S1. Codex sessions carry out work orders, and the coordinator, the admin session, plans, decides, reviews and merges ([work orders](implementation/WORK_ORDERS.md)); once increment 3 runs, the orders are to go through Conduit itself (A77, A78). The number 47 is held by the owner's open pull request 14. |
| 49 | Signing in through the engine ([record](proposals/ruling-49/README.md)). The owner asked how an Agent's login would work with a browser, and how prompts would reach a person and many accounts be kept apart; the coordinator researched the clients' flows and URL elicitation and put three points. "1. Agreed 2. Yes for v0, but we should be more granular in the future, and allow for others to authenticate. Fleshing out interfaces more will likely be a lot of the work for v1. 3. Yes, check it". Accepted: sign-in prompts are URL elicitations that the engine passes to a person's Interface and never to an Agent, changing IFC-007 and IFC-016; in v0 the root Interface alone receives them, with finer delivery and sign-ins by others left to v1; the account each home or Resource is for is declared, and a sign-in to any other is refused and undone. |
| 50 | Adding machines, and the Interface as a control panel ([record](proposals/ruling-50/README.md)). The owner asked that machines be added through Conduit in a standardized way, with Tailscale a provider, rather than set up by hand; the coordinator proposed enrollment through the machine provider and put three points. "1. Yes enrollment should become a part of the language, but we should likely do our research and go over this in detail 2. Confirm for v0 3. Yes, this is the purpose of the interface to provide a control panel and bubble up all pending actions". Accepted: enrollment becomes part of the language, its design to be researched and reviewed with the owner first; a joined machine becomes a node only once a person confirms, in v0; every action waiting on a person, sign-ins and enrollment alike, is one mechanism raised at the Interface, a control panel. |

### Later amendments

The rows above are kept as the rulings were given. Later rulings amended some of them. In a
row that cites ruling 41, an item is a decision of the
[principles review](proposals/principles-review/README.md), C14, C20 and C21 are the changes
of its section 3, and a planner's choice is one of P1 to P20 of the
[work order](proposals/ruling-41/PLAN.md#planners-choices), which ruling 41 also accepted:

| Ruling | What it says | Amended by |
| --- | --- | --- |
| 2 | Vocabulary stays provisional. | Replaced by ruling 16; the vocabulary was then ruled (17, 18). |
| 10 | Codex and Claude each build the whole engine. | Narrowed by ruling 14 and layer decision LD4: the two builds cover the language layer, the shared conventions and the implementation interface, and one reference engine is built separately. |
| 14, 15 | "Plugin interface", "plugins". | Vocabulary decision V7 (ruling 17): the implementation interface, and implementations. |
| 15, 17, 19 | Grants, and issuing them. | Rulings 27 to 29: grants are merged into Conduits. The access is what an open Conduit carries, and to issue is to open. |
| 24 | Decision PD1, how a package's exports are named, is held back. | Settled by rulings 25 and 27. |
| 31 | Contracts are documents of four kinds. | Five, with `Schema`, since follow-up decision F21 of the re-audit (ruling 33). |
| 31 | The implementation-defined list has twelve entries. | Fifteen. Ruling 32 added DL13. Decision DA3 of the documents audit, taken under ruling 37, added DL14 when the behaviors that the older text accepted were converted. Round 2 of the re-audit, under ruling 37, added DL15. Ruling 41 removed DL14, since cancellation is the protocol's own (item 22), which leaves fourteen. Ruling 42 (D8) added DL16, what the engine's own runner gives a process, and ruling 43 (Q2) added DL17, an engine that restarts on another machine or runs as several parts, which makes sixteen. Round 5 of the re-audit removed DL11, since the access convention already decides what it left to each engine (M69). That leaves fifteen. |
| 31 | `conduit.dev` stays a placeholder. | Replaced by ruling 33: the names come from this repository. |
| 7, 8, 9 | Placement as Conduit's own model: locations registered as Resources with facts, a default location, Conduit's own rules for choosing one. | Ruling 40, point 3b: a provider reports the machines, places instances by label and reports a lost one; what placement and loss mean for access stays Conduit's. Ruling 41, item 8, wrote it so. The engine registers a node for each machine that the machine provider reports (PLC-026), and that provider places an instance that is given no placement (PRV-015). Since round 5 of the re-audit it places only one for which an item that is run is selected, and one that runs nothing is placed on `local` (PLC-002, M22, [below](#changes-that-no-ruling-has-accepted)). A machine reported unreachable makes requests to its instances report unavailable (PLC-023), and the engine declares lost the node of one reported gone (PLC-024). The "default location", facts and `set-facts.v1` are removed. |
| 13, 14 (LD2), 23, 28 (CC11) | Setup graphs, with the static meaning of steps, bindings and dependencies, in the language. | Ruling 40, point 3c: reduced to declared dependencies; an operation built from other operations is an ordinary implementation. Ruling 41, item 1, then removed graphs and steps, with GD1 and GR1 to GR4 of ruling 23 and CC11 of ruling 28. An instance's own setup and retirement work are a setup hook and a retire hook, which its template selects in its `implementation` map (TPL-017, TPL-018). Only declared Conduits and entries keep a dependency order (CON-004, POOL-013). `graphs.yaml` is deleted, as the owner changed planner's choice P2, and [spec/README.md](../spec/README.md#promise-files-that-rulings-removed) lists its IDs. |
| 13 | Process layout is part of each engine's runtime profile. | Ruling 40: processes are run by a runtime provider that the user chooses. Ruling 41, items 2 and 31: a runtime provider is a Resource whose type offers the core capability `runtime-provider.v1` (PRV-001), and an Agent or a Resource names the one that runs it by its standard argument `runtime`. With none, the engine's own runner runs it (PRV-006). |
| 31 | Runtimes declared in templates, with start and stop operations and handles the engine holds. | Ruling 40, point 3a: a runtime provider contract (run, stop, status). Ruling 41, item 2, wrote it in the shape of CRI. `runtimes`, held handles and `restoresRuntimes` are removed, with the runtime decisions of rounds 2 to 4 that ruling 39 accepted. The engine names each run by the instance's ID, so one run serves one instance (item 2a, PRV-007, TPL-019), and a restart is the provider's own restart policy (item 2b, PRV-010). So decision I7 and DL12 no longer leave to each engine who starts a process, over which transport it is reached, or how many instances share one. |
| 15 | "The grant is an object", in the purpose. | "The Conduit is an object", by ruling 39, point 7. |
| 31 | A domain must be named before the engine builds. | Withdrawn by ruling 38, point 5. |
| 19, 20 | Rule RR8 of the relationships record: work that a caller started stays subject to that caller's access whenever it later runs. | Later work is dropped from v0 by ruling 38, point 6: no operation arranges, through the engine, work that outlasts its invocation. ACC-019 states that instead, and IMP-006 is removed. |
| 34 | Proposed: the names belong to a version of the specification. | Accepted as ruling 35. |
| 7, 23, 28 | Location needs: types declare them (ruling 7), and every type states them, including none (ruling 23). They are requirements on an instance's placement (CC9, ruling 28). | Ruling 41, item 8a, with planner's choice P12: a placement need is a label selector, a map of label to value written inline as `nodeSelector` (`placement` until ruling 42) in the Type and in the implementation item, and matched against the labels that the machine provider reports (PLC-007, PLC-025). Requirement contracts no longer apply to placement, and a Type's `nodeSelector` is optional. |
| 8 | Name, owner and placement are standard arguments. | Ruling 41, item 31b: `runtime` is one more, for an Agent and a Resource. It names the runtime provider that runs the instance, and is fixed for life (OBJ-013, PRV-006). |
| 15 | Enforcement is opted into through the language and "plugins that select an isolation framework". | Ruling 41, item 2d, reads such plugins as a runtime provider whose contract confines its consumer, which the user picks by the standard argument `runtime` (item 31b). In v0 every restriction is mediated (ACC-027). |
| 15 | Each agent's access "can be inspected, narrowed and revoked independently of both ends", in the purpose. | Ruling 41, item 3a, removed policies, so an open Conduit is not narrowed in place. To narrow an access, an actor closes the Conduit and opens a narrower one (ACC-011). A limit placed on an open Conduit is a candidate for a later version (A66). |
| 19, 28, 29, 32, 38 | Policies. A policy on an Agent is the tool for restricting it across its Conduits (RD8, ruling 19). A policy is applied through an operation on `access` and recorded there (CC12, ruling 28). A temporary limit is a policy (CC3, ruling 29). A delegate's bound limits the policies it may apply (U7, ruling 32). `apply-policy.v1` warns when a policy makes an actor's requests to a target that the engine implements unavailable (ruling 38, point 7). | Ruling 41, item 3a: policies, `apply-policy.v1` and `remove-policy.v1` are removed from v0. Policies are a candidate access feature for a later version (A66). |
| 19, 38, 39 | Composite Conduit templates: a template that expands into several pairwise Conduits (RD4, ruling 19), which opens with a whole decision at setup (ruling 38, point 3) and an outcome for each component (ruling 39, point 5). | Ruling 41, item 5: removed. A template that needs two capabilities of one target declares two Conduits. |
| 20 | Rule OR8: a control's result states the settings it confirmed, and the engine stores exactly those. | Ruling 41, item 15: a control applies exactly what it was sent or fails, and on success the engine stores what was sent (REV-007). An implementation never applies another value in the place of one it was sent. |
| 20, 32, 37 | Retained results. Only the original caller retrieves one, checked against its access at retrieval (OD3, ruling 20). A call whose Conduit closes while it runs delivers its content only if the caller's access then permits (U6, ruling 32). A tool for retrieving one's own results needs no Conduit (Q7, ruling 32). The core export `retrieve-result.v1` (ruling 37). | Ruling 41, items 6a and 6c: an invocation permitted when it was dispatched delivers its outputs even if its Conduit closes while it runs (ACC-034). Retained results and `retrieve-result.v1` are removed: a caller that lost a result repeats the request with the same idempotency key (OPS-009). |
| 21, 38 | Rule DR5: renaming an instance is its own operation, checked against the revision (ruling 21). A rename advances the revision (ruling 38, point 12). | Ruling 41, item 10: there is no rename. A name is fixed once the object is made (OBJ-011). |
| 23 | Decisions SD1 and SD2: a short closed list of schema keywords, and five string formats. | Ruling 41, item 20, partly reversed them: JSON Schema 2020-12 with the applicator and validation vocabularies, less stated exclusions, and with `uri-reference` ([schema profile](../spec/schema-profile.md), SCH-008). |
| 24, 38 | Rule PR2: a template whose dependency is not installed is registered, and the missing dependency is reported (ruling 24). Ruling 38, point 8, kept that report for a directory handed to an engine. | Ruling 41, item 12: such a registration, or such an installation, is rejected, with a diagnostic naming each package that is not installed (PKG-006, PKG-028). The rest of point 8 stands: an engine handed a directory installs nothing (SUB-010). |
| 24, 37, 39 | Re-exports: rule PR5 (ruling 24), choice C49 of the core-and-schemas record (ruling 37), and the decisions of rounds 2 and 4 on them that ruling 39 accepted. | Ruling 41, item 9: removed from v0. `exports` lists only the package's own documents (MAN-007). |
| 27 | Decision NE: a package's address is by default also where it is fetched from. | Ruling 41, item 21: every engine fetches `https:` and `file:` URLs, and another scheme only through a package-source provider. The address is not a source (NAM-012). |
| 28 | Decision CS6: one marker, `optional: true`, for inputs, steps, declared Conduits and outputs. | Ruling 41, item 16a, narrowed it: an Operation contract's input or output is optional where its schema's `required` does not list it, as the protocol's tool definitions write it (CTR-002). Template inputs, a Requirement's inputs, declared Conduits and entries keep `optional: true`. There are no steps. |
| 29, 30 | An object exists until it is unregistered (ruling 29), and an instance is unregistered only once its shutdown is confirmed (OL3, ruling 30). | Ruling 41, item 17: a retired instance is removed, with no request, once its shutdown is confirmed, and a Conduit when it is closed (LIF-035). `unregister.v1` stays for templates, and for an instance whose node was declared lost (LIF-017). Since round 5 of the re-audit it removes any retired instance, as an assertion that its shutdown is settled another way (LIF-017, LIF-036, M41, [below](#changes-that-no-ruling-has-accepted)). |
| 31 | Decisions D3 and D5: value references from five sources, and an optional `retire` graph. | Ruling 41, item 1: removed with graphs. An instance's own retirement work is its retire hook, a core operation that its template selects in its `implementation` map (TPL-018). |
| 31 | DL9: whether a request to set up, retire or change a busy instance is queued or rejected is each engine's. | Ruling 41, item 23: it reports conflict at once (REV-014). DL9 keeps only the order of failed checks. |
| 31, 32, 33 | Decision I2: Conduit's keys in a request's `_meta`, the interface version among them. | Ruling 41, item 18: the keys travel as one extension of the Model Context Protocol, with the setting `revision: v0`, and the key for the interface version goes. A request to an implementation that does not declare the extension reports unavailable (IFC-037). |
| 32, 37 | A location's facts are stored configuration, changed only by a control, and a requirement on placement is a schema over them (G5, ruling 32). The owner's change of ruling 37: a control that sets a location's facts. The shared-host requirement is checked when the Conduit is opened and on each request (G13, ruling 32). | Ruling 41, items 8 and 8b: facts, `set-facts.v1` and `location-settings.v1` are removed, and a node's labels are what the machine provider reports. The shared-host requirement compares the hostname labels of the two ends, and is decided once, when the Conduit is instantiated (PLC-013). |
| 32 | Decision U1: each request's `_meta` names the Conduit and carries every restriction that applies. | Planner's choice P20 of ruling 41: a setup or retire hook is dispatched through no Conduit, for the caller of the setup or the retirement. It carries that invocation's filters and no Conduit (TPL-017, TPL-018). |
| 32 | Decision U5: when setup or retirement runs again, a step whose outcome is unknown is not dispatched again unless a new attempt is allowed, a runtime the instance holds is not started again, and the rest is DL1. | Ruling 41, item 1c, with planner's choices P4 and P20: a further setup leaves what is already done as it is, asks for a run only where the provider does not report one running, and dispatches a hook again only as its idempotency key allows (OPS-021, OPS-028). DL1 keeps only the order of a Pool's Resources. |
| 32, 33 | Re-audit decisions that rest on what ruling 41 removed: Q3, G4 and the `retire` graph of G12 (ruling 32); F5's component segment and its check on a rename, F8, F15's default placement, F19 and F25 (ruling 33). | Ruling 41, items 1, 5, 8 and 10: graphs and steps, composite Conduit templates, the "default location" and rename are removed, so these no longer apply as written. Q3's rule for an outcome holds of a setup and a retirement, counted by their parts (OPS-022). What F8 and G12 said of the `retire` graph holds of the retire hook: it carries no mark of model work, and it is not dispatched for an instance for which no setup was ever dispatched (TPL-018). An entry that no input places is placed by the machine provider (PLC-002), and since round 5 of the re-audit only where an item that is run is selected for it; one that runs nothing is placed on `local` (PLC-002, M22, [below](#changes-that-no-ruling-has-accepted)). |
| 37 | Choices C43 and C53 of the core-and-schemas record: the manifest is `conduit.yaml` or `conduit.json`, and `.yml` files are not read. | Ruling 41, change C14: `.yml` files are read and `conduit.yml` is accepted, and a directory with two manifest names is rejected (MAN-001, MAN-002). |
| 37 | Choice C15 of the core-and-schemas record: a revision is a whole number from 0. | Ruling 41, change C21: a revision is an opaque non-empty string, compared only for equality, as an HTTP entity tag is (REV-001). |
| 37 | Choices C16 and C38 of the core-and-schemas record: only the `message` of a diagnostic and the top-level names of an inspection result are fixed, and the rest is settled with the conformance vectors. | Ruling 41, item 24: a diagnostic is SARIF 2.1.0's `result`, reduced, whose `ruleId` is a promise's ID (24a). A status reports `conditions` in the shape of Kubernetes `metav1.Condition` (24b). A record has the core schema `record.v1`, a CloudEvent (24c). The rest of an inspection result is still settled with the vectors. |
| 38 | Point 1: a secret is referred to and never written as a literal, which needs a secret type in the core package. | Ruling 41, item 19: no core type. A core capability, `secret-provider.v1`, is offered by each store's own Type, which has a schema for its locator (SEC-002, SEC-005). Ruling 42 gave the capability that name in place of `secret-reveal.v1` (vocabulary audit, N1). |
| 38 | Point 11: a result that carries none of its operation's declared outputs is an error result, to be looked at again. | Ruling 41, item 6b: `isError` is true only where the request ended with a reason or its outcome is failed or unknown (IFC-025). A repeat of an invocation still running reports conflict (OPS-032). |
| 39 | Points 1 to 4: the secret type is not decided. Only a Resource reads a secret, and only the root Interface may open a Conduit that reads one. The engine reads a secret through the instance's own Conduit and sets it in a process's environment at start, for a server and for an Agent's own credential. | Ruling 41 settled point 1 with section 4 of the principles review ([secrets.yaml](../spec/promises/secrets.yaml)). Points 3 and 4 stand: before each run that it asks a runtime provider for, the engine reads each secret that the item needs (items 2c and 4a, SEC-010). Point 2 is restated: no actor's request for `get-secret.v1` is dispatched (item 4b, SEC-007), and the root Interface may open a Conduit that lists it for a Resource or an Agent (SEC-008). |
| 39 | Requirement composition: GP1 of the documents audit (decision DA3), and round-2 decisions R2-G3 and 10, accepted with every logged decision. | Ruling 41, item 7: removed. A Requirement contract has no `constituents`, and several requirements are applied side by side in a Conduit template's `requirements` map. CTR-010 and CTR-011 are removed. |
| 39 | `obliges`: round-2 decisions 8 and 10, part of decision 7, and the `obliges` half of GP15 (decision DA3). | Ruling 41, item 14: removed. A filter binds only the target, whose implementations apply it (CON-022). IFC-034 is removed. |
| 39 | `narrows` on a Requirement contract, and CON-030, accepted with the specification. | Ruling 41, change C20: `narrows` is dropped. A requirement of the core package is applied by the engine and never sent, and any other is a filter that the target's implementations apply (CTR-005, CON-018). CON-030 is removed. |
| 39 | Coordinator decisions 4B-4, R2-S6 and R1-2.6/R2-C4: a project's contracts are kept as an unpublished package, named by a digest of its documents and written `<digest>#<name>`. | Ruling 41, item 11: a project that holds contracts states `address`, and outside its own documents they are `address#name` (MAN-015, SUB-008). The digest still identifies one submission (planner's choice P11). |
| 41 (item 17) | A request through a closed Conduit reports invalid, as the cost line of item 17 says. | Refused, by ruling 42 (D6). |
| 41 (item 4c) | Every read of a secret belongs to a request in progress. | Also a read again for the caller of the setup, where a remote connection's values were lost, by ruling 42 (D12). |
| 41 (section 4 of the review, line 18) | What else a run receives is the provider's. | For the engine's own runner, six variables and a list set on the runner, by ruling 42 (D8). |
| 39 (decision 5 of round 3) | `retire-with-resources.v1` stops at the first Resource whose retirement does not complete. | It goes on: only the Resources that the order puts after the one it could not retire wait, by ruling 42 (R5). |
| 17, 18 (V4), 33, 41 | Location, origin, operation identity and the names that ruling 41's work order coined. | The names of the [vocabulary audit](proposals/vocabulary-audit/README.md), by ruling 42. |
| 42 (D17) | `run.v1` carries a credential as a file that the runtime provider may refresh in place. | In this version a credential lasts as long as its instance and nothing renews it (DL13), as the [multi-machine proposal](proposals/multi-machine/README.md) that ruling 43 accepted has it; renewal on the run's machine is v1's (A65). Ruling 43's verification therefore moved the leave to replace a file in place out of PRV-022's `then` into its observable (MX-21), a [choice](proposals/ruling-43/log-integration.md#choices-of-the-verification) that the owner can strike. |
| 41 (item 16a) | An Operation contract's `inputSchema` and `outputSchema` have a root limited to `type: object`, `properties`, `required` and `additionalProperties` (CTR-012). | Ruling 44 (D14 of round 6): the schemas of an Operation contract that is not a control may hold any keyword of the schema profile at their root beside `type: object`, as the published protocol's tool definition allows at the baseline revision; the inputs and outputs are still the members of the root's `properties`. A control keeps the limited root, which its partial updates need (REV-006), and SCH-008's ban on a `default` under a conditional keyword stands (CTR-012). The rest of item 16a stands. |
| 37 (choice C42 of the core-and-schemas record) | The default implementation of an operation is stated by each Operation contract. | Ruling 44 (D06 of round 6, as revised after the ruling): a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule, with no mapping. Applied in CTR-002, TPL-003 and TPL-004 ([log](proposals/reaudit/round-6/log-d06-d12.md)). |

Ruling 44 does not change X2, the owner's own change in ruling 37 that an actor's request names
objects by reference only, nor LIF-031: as revised after the ruling, D12 of round 6 keeps names
as the only address and gives the lifecycle actions an optional expected ID instead (LIF-037). The record's table of
[earlier rulings this changes](proposals/ruling-44/README.md#earlier-rulings-this-changes) cites
ruling 39 for C42 and X2; ruling 39 accepted them again with every logged decision, and ruling
37 accepted them first.

Decisions of the coordinator, taken under ruling 37, also changed some of what a ruling had
accepted. The owner accepted each of the six below in ruling 38 (points 10 and 12):

| Ruling | What it says | Changed by the coordinator, and accepted by ruling 38 |
| --- | --- | --- |
| 24 | Decision PD5: two conformance levels. | Three, with Implementation, since row GP16 of the [documents audit](proposals/docs-audit/GAPS.md). |
| 31 | Decision D4: a runtime may be marked `optional: true`. | A runtime carries no such mark; a step that starts it can be optional. [Round 2](proposals/reaudit/round-2/DECISIONS.md). No longer applies after ruling 41: runtimes and steps are removed (items 1 and 2). |
| 32 | An operation identity is scoped to one caller and one target. | And to the instance that sent the request (OPS-008). Round 2, finding R3-3. Since ruling 41 the idempotency key of a setup or retire hook is scoped to the instance alone (OPS-028, planner's choice P20). |
| 32 | Decision Q5: the order of reasons is refused, invalid, conflict, unavailable. | Where the decision on access cannot be made, unavailable stands in the place of refused (OPS-023). [Round 3](proposals/reaudit/round-3/DECISIONS.md), finding 3A-1. |
| 37 | The owner's change X1: a control's `changes` is a list of pointers. | A map from an input's name to a pointer. Round 2, finding R2-G2. |
| 37 | Requests name objects by reference only. | A step and an onward request may hold the resolved form. Round 3, finding 3B-2. After ruling 41 there are no steps (item 1); an onward request still may (LIF-031). |

#### Changes that no ruling has accepted

[Round 5](proposals/reaudit/round-5/README.md) of the re-audit, under ruling 37, also changed
what rulings had accepted. No ruling has accepted these changes, and the owner can strike each
([who decides what](#who-decides-what)). The three below change what the tables above give.
Others that come close to a ruling or to a decision of the coordinator are among the
[decisions the owner may want to read first](proposals/reaudit/round-5/DECISIONS.md#for-the-owner).

| Ruling | What it says | Changed by round 5 |
| --- | --- | --- |
| 41 (item 8c) | The machine provider places an instance that is given no placement (PRV-015). | M22: an Agent or a Resource for which no item that is run is selected is placed on `local`, and no machine provider is asked (PLC-002). Such an instance is not checked against its type's and its items' `nodeSelector`, since nothing of it runs (PLC-007), as the coordinator settled V6 of the verification. |
| 41 (item 17) | `unregister.v1` removes an instance only where its node was declared lost (LIF-017). | M41: it removes any retired instance (LIF-017). Following from it, the integration's [choice](proposals/reaudit/round-5/log-fixes.md#the-choice-of-this-pass) has PLC-023 not hold back the stop that LIF-036 asks for before the instance is removed, so the stop is asked for even while the machine is reported unreachable or omitted. |
| 31 | The implementation-defined list, with DL11, on Conduits during setup. | M69: DL11 is removed, since ACC-003 and ACC-036 decide what it left to each engine; its ID is not reused. The row for ruling 31 in the first table gives the count. |

Ruling 44 reversed one of round 5's decisions. V2-15 of the
[verification of its fixes](proposals/reaudit/round-5/log-fixes.md#verification) read the
protocol's error -32021, for a client capability that the call did not declare, as failed,
since it took the protocol to have a server answer so before acting. The protocol says
"processing ... requires", and nothing of the order of effects. Round 6's finding 6-1-01 reads
that error as any other error, failed for an operation that states no effect, is not a control
and starts no model work, as IFC-033 tests, and unknown for any other, except where Conduit's
own contract binds the server to have done nothing: the error that names Conduit's extension
(IFC-008), and an implementation that carries Conduit's keys, which answers so only where it
has performed nothing for the call (IFC-047). IFC-007, IFC-010, IFC-045, IFC-047, added, and
OPS-007 changed together.

## Layered claim

Ruling 42 renamed the runtime profile the conformance document, and operation identity the
idempotency key (vocabulary audit, N18 and N15); this section and its table of layer decisions
keep the words of rulings 13 and 14.

Ruled 2026-11-05 (ruling 13). An earlier working claim, that the documents determine one
observable outcome for every situation, is the bar for a programming-language specification. It
is withdrawn. Conduit is a configuration language, and by that bar even Kubernetes would fail:
its control plane has one implementation, its conformance suite tests a core subset, and most of
its behavior lives in controllers rather than in the language.

**The claim:** the language fully determines what a set of declarations means: which objects
exist, how they relate and which invariants always hold. How a runtime gets there is the
runtime's contract.

| Layer | What it holds | Comparable part of Kubernetes | How precise |
| --- | --- | --- | --- |
| Language | The five families, template and instance, identity and references, instantiation, placement, Conduits and grants as declared data, schemas | Resource schemas and API conventions | Fully specified; small |
| Shared conventions | The few behaviors every part must agree on, such as operation identity, the outcome set, revisions and what a grant permits | Resource versions, optimistic concurrency, finalizers | Specified as invariants, not step by step |
| Type contracts | How each type behaves: an Agent's conditions (running, paused, idle), a directory's edge cases, message read positions | Each controller's behavior | Owned by the type's author; versioned separately |
| Runtime profile | Engine choices: outage handling, retries, cursors, process layout | The reference control plane's behavior | Decided and tested for the reference engine; other engines may differ |

Behavioral decisions are still made and recorded, in the type contracts and the runtime profile
rather than in the language. The critical concepts are unchanged: Pools owning Resources and
shared logins, Conduits carrying grants, placement, instantiation, explicit actions rather than
a background reconciler, control and observation through the Interface, and plugins over MCP.

The specification holds three layers: language, convention and interface. The third is the
implementation interface of layer decisions LD3 and LD4, which this section calls the plugin
interface. Type contracts and the runtime profile stay outside the specification. This section
and the table of layer decisions below are kept in the words of rulings 13 and 14: read
"declarations" as documents, "grant" as the access an open Conduit carries, "plugin" as
implementation, and "normative surface" as the specification. MCP is the Model Context
Protocol.

Ruling 40 adds one layer beside these: **provider contracts**. A provider is existing
infrastructure that the user chooses (a secret store, a runtime, a set of machines, a source of
packages), reached through a contract that Conduit states and any tool can implement, as
Kubernetes reaches runtimes, storage and networks through CRI, CSI and CNI. The contracts belong
to the specification; the providers do not. What a runtime profile held about process layout and
placement now belongs to the chosen provider.

### Layer decisions

| ID | Decision | Ruling | Status |
| --- | --- | --- | --- |
| LD1 | The test that assigns a rule to a layer. | Language if it can be answered by reading the declarations alone. Shared convention if two independently written parts would break each other, or a safety promise, by disagreeing. Type contract if it matters only to instances of one type. Runtime profile otherwise. | done: accepted as recommended |
| LD2 | Where setup graphs sit. | Keep graphs. The language defines their static meaning (steps, bindings, dependencies, validity) and two invariants: a step never runs before its prerequisites succeed, and setup succeeds only if every required step does. Scheduling, retry, cancellation and partial outputs move to the runtime profile. | done: accepted as recommended. Overtaken: ruling 40, point 3c, reduced graphs to declared dependencies, and ruling 41, item 1, removed them. An instance's own setup and retirement work are two hooks |
| LD3 | The list of shared conventions, and whether one engine's internal services must work with another's. | Six conventions: operation identity and outcome set; revision-checked change; no managed access without a recorded grant, checked at the target, with restrictions only narrowing; declarations distinct from observed state, with no effect from registration; single Pool ownership and retire-before-unregister; an inspectable record of every managed operation. Cross-engine interoperation of internal services is not a v0 goal; the plugin interface is the shared wire. | done: accepted as recommended. There are seven conventions since ruling 17 |
| LD4 | What the two independent builds must cover. | The language layer in full (declarations in, resolved objects and diagnostics out), the shared conventions through vectors, and the plugin interface. One reference engine; a second full engine is optional later. | done: accepted as recommended |
| LD5 | Where each layer is written down. | The language and shared conventions in the normative surface as promises with vectors. Each type contract with its type, in a standard library shipped beside the reference engine. The runtime profile with the reference engine. | done: accepted as recommended |

## Priorities

The priorities group the action items by theme. The order of the work is the
[roadmap](../ROADMAP.md#the-order-of-the-work).

| Priority | Theme | Why it comes here |
| --- | --- | --- |
| P0 | Make the language concrete, complete and free of known defects | The owner's goal, and the precondition for everything else. |
| P1 | Prove the language can be rebuilt and its promises kept | The reference model, vectors, model checking and the two engine builds; native probes run alongside. |
| P2 | Prune and re-baseline, under the scope guard | Uses the evidence to re-sequence what the first slice does not need. |
| P3 | Vocabulary | Moved up by ruling 16: decided now and applied with the rework. |

## Action items

Open, in progress and ruled items are below, except the experiments and probes: they are
action items too, and their rows are in the roadmap's
[experiments and probes](../ROADMAP.md#experiments-and-probes). Completed and superseded items
are in [the history](archive/plan-audit-history.md#completed-and-superseded-action-items),
except the experiments and probes, which the roadmap lists among
[what is done](../ROADMAP.md#done-removed-dropped-or-not-adopted).

Promise ranges and counts in the status notes are as first written; the current files and
counts are in [spec/README.md](../spec/README.md#contents).

### P0 — make the language concrete and complete

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A32 | Define the specification: the exact set of documents from which an engine must be rebuildable. | Goal, Q2, Q10 | The contents list exists and everything outside it is marked as not binding (G1). | in progress: `spec/` exists and [spec/README.md](../spec/README.md) lists its parts. Nothing outside it binds (decision DA1 of the documents audit). Eighteen of the nineteen accepted behaviors that only the older text stated are converted (DA3; [list](proposals/docs-audit/GAPS.md), [log](proposals/docs-audit/log-gaps.md)). The nineteenth, on secrets (GP3 of the list), is written since ruling 41, in [secrets.yaml](../spec/promises/secrets.yaml). Ruling 41 also removed two that were converted: GP1, requirement composition (item 7), and the `obliges` half of GP15 (item 14) |
| A33 | Build the promise register: restate accepted behavior as numbered promises, each with a vector. | Goal, D8 | Every topic is converted, with word counts before and after (G2). | in progress: every area is written as promises; no vectors; the word counts are not done |
| A42 | Re-triage the defects by layer and close those in the language and shared conventions, per the [defect closure plan](archive/plan-audit-history.md#defect-closure-plan). | Rulings 12, 13 | Readiness condition G7, as ruling 38 restated it: the latest round of readers reports no unsafe finding, and the reference model and the vectors agree with the promises. It was: two consecutive sweeps of the language and shared conventions are clean. | in progress: triage done; 87 of 167 are in the language or shared conventions |
| A51 | First batch of fixes: relationships and access. Eight decisions and sixteen rules are proposed in the [relationships record](proposals/relationships/README.md). | Rulings 15, 17 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 19, 20) and written as 41 promises in [spec](../spec/README.md); vectors pending. Ruling 41 removed composite Conduits, decision RD4 (item 5), and policies, on which RD8 rests (item 3a) |
| A13 | Second batch of fixes: operations, outcomes and revisions. Five decisions and nine rules are proposed in the [operations record](proposals/operations/README.md). | Q6, L1, L10 | It is ruled, model-checked and swept clean. | in progress: ruled (ruling 20) and written as 27 promises; Experiment 004 model-checked the at-most-once rules as the older text stated them, as an evaluation of the tool and not as evidence about the promises; the promises themselves are not model-checked; vectors and a sweep pending. Ruling 41 removed retained results and their retrieval, part of OD3 (item 6), and the settings that a control confirms, part of OR8 (item 15) |
| A52 | Third batch of fixes: declarations. Five decisions and thirteen rules, in the [declarations record](proposals/declarations/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (rulings 21, 22) and written as 45 promises; vectors pending. Ruling 41 removed rename, the second half of rule DR5 (item 10); its first half, that no document rewrites an instance's definition, stands (OBJ-011) |
| A53 | Fourth batch of fixes: graphs and the schema profile. Four decisions and nine rules, in the [graphs and schemas record](proposals/graphs-and-schemas/README.md). | Ruling 14 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled (ruling 23) and written as 27 promises. Ruling 41 removed the half on graphs: graphs went with GD1 and GR1 to GR4 (item 1), and `graphs.yaml` was deleted, as the owner changed planner's choice P2. It also partly reversed SD1 and SD2 (item 20). What remains is the schema profile, in [schema-profile.md](../spec/schema-profile.md) and `schemas.yaml`; vectors pending |
| A54 | Fifth batch of fixes: packages, first start and conformance. Five decisions and eight rules, in the [packages and start record](proposals/packages-and-start/README.md). | Rulings 14, 23 | The owner has ruled, and the rules are promises in the specification with vectors. | in progress: ruled except PD1 (ruling 24), which rulings 25 and 27 then settled (A55), and written as 20 promises; vectors pending. Ruling 41 reversed PR2, so a document whose package is not installed is rejected (item 12), and removed re-exports, PR5 (item 9) |
| A59 | Consolidate the promises and audit the whole specification again with cold readers who see only `spec/`. | Ruling 30 | As restated by ruling 38, point 4: the latest round of readers reports no unsafe finding, and the reference model and the conformance vectors agree with the promises. It was: two consecutive rounds with no finding in the language, conventions or interface, which four rounds showed reading alone does not reach. | in progress: round 1 done, with about 170 findings, 11 of them unsafe; 37 decisions accepted (ruling 32) and applied by consolidation pass 1, which changed 212 promises, removed 21 and added 48 ([record](proposals/reaudit/README.md)). The follow-up decisions (rulings 33 to 35) are applied. Round 2 is done: four readers, 125 findings in `spec/`, of which 8 classed as unsafe (7 by a reader, 1 by the coordinator), the decisions in [DECISIONS.md](proposals/reaudit/round-2/DECISIONS.md), and the fixes applied in four passes ([record](proposals/reaudit/round-2/README.md)). Round 3 is done: 44 findings in `spec/`, 4 classed as unsafe, the decisions in [DECISIONS.md](proposals/reaudit/round-3/DECISIONS.md), and the fixes applied ([record](proposals/reaudit/round-3/README.md)). No round has been clean. Round 4 is done: 38 findings in `spec/`, 3 classed as unsafe, the decisions in [DECISIONS.md](proposals/reaudit/round-4/DECISIONS.md), and the fixes applied ([record](proposals/reaudit/round-4/README.md)). Ruling 38 withdrew the gate of two clean rounds. Ruling 41 ordered a fifth round on the rewritten specification, after the vocabulary audit and before pull request 7 is merged. Round 5 is done: six readers read the text at commit `d0c425f` on 2026-11-07 and reported 119 findings, 105 of them in `spec/` and 10 classed as unsafe, merged into 111 items, of which 110 are fixed and 1 is rejected ([decisions](proposals/reaudit/round-5/DECISIONS.md)); the fixes changed 101 promises and added 3, and two verifiers checked them and reported 29 findings, none unsafe, of which 28 are fixed and the last, V6, the coordinator settled ([record](proposals/reaudit/round-5/README.md)). Round 6, the owner's cold read by Codex, is done: one reader read the whole repository at `c4d7d33` on 2026-11-07 and reported six findings, three wrong, two of them safety-critical, two undetermined and one misleading, with a critique of the design in 22 points; an independent check confirmed all six, the coordinator decided each, and the owner accepted the recommendations in ruling 44 ([decisions](proposals/reaudit/round-6/DECISIONS.md)). All six are fixed; D06 and D12 of the critique, which the owner accepted as revised after the ruling, are applied too (A69). No round has been clean, and no unsafe or safety-critical finding is open. Where the next read comes is the row "Read" of the [roadmap](../ROADMAP.md#the-order-of-the-work) |
| A61 | Audit the documents outside `spec/` and make the repository ready for a cold read. | Ruling 37 | Every decision of the audit is applied, and a cold reader can tell from the entry points what binds. | in progress: audited, and decisions DA1 to DA13 are applied ([record](proposals/docs-audit/README.md), which links the logs). The second half of the done-when was put to the fourth reader of round 2 of the re-audit, who read the whole repository cold and could tell from the entry points what binds ([report](proposals/reaudit/round-2/reader-4.md)). What was done about that reader's other findings, and what is left for the owner, is in [the log](proposals/reaudit/round-2/log-reader-4-fixes.md). The fourth reader of round 3 reported 23 findings, 9 of them misleading, and the fourth reader of round 4 reported 26, 8 of them misleading; each is fixed or is listed for the owner ([round 3](proposals/reaudit/round-3/DECISIONS.md), [round 4](proposals/reaudit/round-4/README.md)). Reader 5 of round 5 read the whole repository at `d0c425f` and reported 14 findings, 7 of them misleading ([report](proposals/reaudit/round-5/reader-5.md)); all 14 are fixed, as M100 to M111 ([decisions](proposals/reaudit/round-5/DECISIONS.md#f7-the-tracker-and-the-entry-documents)) and M30 and M87. The action is done when such a reader reports nothing that misleads. The cold read by Codex that ruling 37 names, round 6, read the whole repository at `c4d7d33` and reported one misleading finding, 6-1-06: the entry points still said that V6 waited for the owner. It is fixed, with the other statements that the independent check found stale ([decisions](proposals/reaudit/round-6/DECISIONS.md)), and the entry points now give reading paths for an author, a writer of contracts and providers, and an implementer (D21 of round 6) |
| A72 | Show the roadmap where a reader finds it: the order of the work, what comes next, and each experiment and probe with its state and what it feeds. | Ruling 45 | A reader can tell from the entry points, without reading this tracker, what is next and whether an experiment they have in mind is already planned. | done on 2026-11-09: the coordinator decided the form after a pass over the documents (ruling 45). The [roadmap](../ROADMAP.md), at the root, holds the order of the work and the row of every experiment and probe, and each entry point links it. What moved from where, and the coordinator's decisions, which the owner can strike, are in [the log](proposals/ruling-45/log-roadmap.md) |
| A69 | Apply D06 and D12 of round 6 as the owner accepted them, revised, after ruling 44: a package's own Operation contract states no default implementation, and the engine implements the operations of the core package by rule; names stay the only address, and `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` take an optional expected ID, which reports conflict where the object that the name resolves to has another. | Ruling 44, [after the ruling](proposals/ruling-44/README.md#after-the-ruling) | Written as promises, in the core package, the schemas and the examples, and logged beside round 6's decisions. | done on 2026-11-07: D06 in CTR-002, TPL-003 and TPL-004, with the checker's rules `implementation-selected` and `engine-selected`; D12 in LIF-037 and the optional `expectedId` of the four actions; LIF-031 unchanged ([log](proposals/reaudit/round-6/log-d06-d12.md)). Two choices of its editor are for the owner to strike: a template may not select another implementation for an operation that the engine implements, and a reference that resolves to nothing still reports invalid |
| A41 | Fix the uncertain-write rule (L1) with A13. | L1 | The convention states when reissue is safe and a vector covers it. | ruled: ruling 20, OPS-012 to OPS-014; vector pending |
| A12 | Name the target MCP revision and specify the implementation interface and its profile. Whether the wire between an engine's own services is shared is layer decision LD3. | Q2, G9 | An implementation written against the interface works with any conforming engine. | ruled: ruling 31, IFC-001 to IFC-017; the baseline revision is 2026-07-28; vectors and test implementations pending. Ruling 41 carried Conduit's keys as an extension of the protocol (item 18), changed when `isError` is true (item 6b) and removed the retrieval tool (item 6c) |
| A11 | State what the language requires of caller authentication, then choose the mechanism from the native probes. | Q3 | Both are in the specification. | in progress: the requirement is stated. The engine establishes which instance the consumer is for each request, and no consumer can present as another (IFC-022); what a consumer presents is never sent to an implementation (IFC-030); how a request shows which instance it comes from is item DL13 of the implementation-defined list (re-audit decisions U10 and F2). Ruling 41, item 26a, made DL13 the place where an engine names its mechanism, which mutual TLS, SPIFFE or the protocol's authorization could then each be, with a cryptographic proof wherever a request crosses machines (IFC-022). Ruling 43 (Q1) fixed two forms of proof: a signed token in the form of a JWT-SVID, which the engine's address always accepts as the protocol's own authorization, and mutual TLS, which it may accept beside it (IFC-042, DL13); SPIFFE's form of ID is recommended and not required. What waits for the native probe (A4) is how a native Agent obtains and presents the token |
| A16 | Conduit instance granularity, participation and grant lifetime. | Q5 | Ruled in the layer it is assigned to and swept clean. | ruled: a Conduit has one consumer and one target (ruling 17), and what closes one is ruled (rulings 19, 28, 29); vectors pending. Since ruling 41 a closed Conduit is removed when it closes (item 17, LIF-035) |
| A25 | Agent lifecycle, including what survives replacement. | Q8 | Ruled in the layer it is assigned to and swept clean. | ruled by the specification, settled by the coordinator under ruling 37 as ruling 38, point 13, confirmed it, and accepted by ruling 39: a replacement is a new instance, with an ID of its own: LIF-009 for a nested one, and the registration of a new document for any other, as the definition of an ID in `spec/definitions.md` has it. It holds none of the retired instance's Conduits, which closed when that instance was retired (CON-008), and its own declared Conduits are opened by its setup (CON-004). Since ruling 41 no runtime is restored (item 2): a run is named by its instance's ID, so a replacement has a run of its own (PRV-007). Vector pending |
| A17 | State what runs with no frontend attached and what starts it. | Q7 | It is a promise. | open |
| A35 | Specify the declare-lost operation, which is reserved to the root Interface. | Ruling 7 | Its contract, fencing and effects are promises. | ruled: PLC-017, reserved to the root Interface; vector pending. Since ruling 41 a machine that the machine provider reports gone has the same effect (item 8e, PLC-024) |
| A43 | Write the schemas and a strict loader in Rust. | G3 | Every scenario file validates and the negative cases are rejected. | done on 2026-11-09: the schemas are written in `spec/schemas` and accepted (A60); the Rust loader is the crate `conduit-loader` in [reference-model/](../reference-model/README.md), and meets the done-when of increment 1; its thirteen findings are settled in [the log of increment 1](proposals/first-build/log-increment-1.md). Ruling 45 makes the loader the first piece of the first build (X4 of pull request 10). After the Schema contracts are bundled as `check_schemas.py` bundles them, a named crate accepts every document under `valid/` and `invalid-by-rule/` and rejects every one under `invalid/`, by the step that `check_schemas.py` rejects it by, with formats asserted; SCH-016's patterns and SCH-004's uuid are tested on cases of their own. The rules of `check_schemas.py`'s steps 5 and 6, which need several documents or the core package's contracts, are ported with the core, in increment 2 (A38). It is increment 1 of the [first build](implementation/FIRST_BUILD.md#increments) (ruling 46) |
| A21 | Repair status and consistency defects in the documents. | Q10 | Each item under [known inconsistencies](archive/plan-audit-history.md#known-inconsistencies) is fixed or dropped. | in progress: readiness audit applied 2026-11-05. On 2026-11-06 each of the eleven items was checked against the tree and its state is noted beside it. Three are overtaken, two partly. Five stand as written in files that are marked as not binding or are not edited; whether to drop them is the owner's call. One, a stale branch, was deleted under ruling 39, point 12 |

### P1 — prove the language can be rebuilt and its promises kept

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A68 | The first build: a working demonstration, not another round of review of the prose. The strict loader in Rust first (A43, ruling 45), then the S1 scenario through a reference model in Rust (A38), conformance vectors for the safety cases first (A39), and the first providers (A63), with round 6's evidence cases placed in the [roadmap](../ROADMAP.md#round-6s-evidence-and-measurements). Each run measures what a new author must do: the files edited, the concepts met, the grants made by hand, the adapters written, and the steps of recovery that need a person (D22 of round 6). | Ruling 44 | S1 runs end to end on the reference model and the first providers, its safety vectors pass, and the author measurements are recorded with dated results, beside what they show for the simplifications that round 6 proposed. | in progress: its nature is settled by ruling 46, and its increments, success criteria and measures are pre-registered in [the first build](implementation/FIRST_BUILD.md). It is step 3 of the [roadmap](../ROADMAP.md#the-order-of-the-work) |
| A38 | Write the executable reference model in Rust for the language layer and the shared conventions: a pure, deterministic library in which every case cites a promise. | Rulings 10, 14 | It passes every vector with no unhandled case (G4). | in progress: the core of the first build (A68), on S1 from its increment 2 (ruling 46), which ran S1 whole on the core on 2026-11-10 ([reference-model/](../reference-model/README.md)); S2 to S9 extend it. It also measures the bytes it keeps for each keyed, unkeyed, failed and unknown invocation (D19 of round 6), for A14 |
| A39 | Publish the conformance vectors, with a held-out set. | Ruling 10 | Any engine can run them through an adapter; coverage by promise is reported. | in progress: 362 portable vectors under [conformance/](../conformance/README.md) since increment 2 of the first build, the safety cases first (ruling 44), among them round 6's E01 to E06. A record of a machine's loss gets a portable field for its cause with these vectors (PLC-024, REC-002), as ruling 44 settled the open point |
| A44 | Mutation-test the reference model against the vectors. | G6 | No unexplained surviving mutant. | open |
| A40 | Model-check the safety promises with Stateright in small configurations, in the ordinary test run, including two machines with message loss, reordering, a partition and a lost machine (ruling 40). | [Experiment 004](../experiments/004-stateright-evaluation/README.md) | Each safety promise holds or has a recorded reason it cannot be checked this way (G5). | open: tool evaluated |
| A45 | Publish the build base: the orphan branch, the brief as `BUILD.md`, agent instruction files, and a pilot run of the brief on S1. | Ruling 12 | G10 passes and the owner signs off (G11). | open: brief and process drafted |
| A34 | Two independent builds in Rust, by Codex and by Claude, of the language layer, the shared conventions and the implementation interface (LD4). | Rulings 10, 14 | Both pass the public and held-out vectors; every divergence is traced to the language and fixed. | open: blocked by the readiness gate |
| A50 | Build one reference engine, with a standard library of core type contracts and a recorded conformance document. | LD4, LD5 | It runs the scenarios S1 to S10. | open: ruling 46 makes the shell of the first build (A68) its first slice, so that S2 to S9 run on the same codebase as S1 |
| A62 | Write the provider contracts of ruling 40 in the specification: the runtime that runs an instance's implementation (run, stop, status); the machines, their labels and the loss of one; the secret store; the source of packages. | Ruling 40 | Each is a contract any existing tool can implement, and the specification names no backing system. | in progress: written under ruling 41, in [providers.yaml](../spec/promises/providers.yaml) and [secrets.yaml](../spec/promises/secrets.yaml); the questions the rewrite left open were settled in the [open questions after the rewrite](proposals/ruling-41/open-decisions.md) and by rulings 42 and 43; no vector |
| A63 | Build the first providers, as packages in Rust under a top-level `packages/` directory, one crate per provider (ruling 40, after the ruling): two for each provider contract, on different existing tools. Candidates: secret stores on a file (the file store of the standard library: under ruling 41, item 13a, no store is built into the engine, and none reads the engine's own variables), the macOS keychain and 1Password; runtimes on a local process and Docker; machines on the local machine and a second host reached over the network; package sources on Git and a local directory. | Ruling 40 | Each passes the vectors of its contract, and S10 passes with every provider swapped. | open: needs A62. Where its providers come in the order of the work is in steps 3 and 4 of the [roadmap](../ROADMAP.md#the-order-of-the-work). For the keychain store, what [Experiment 005](../experiments/005-macos-keychain/README.md) measured, in the user's logged-in session, and what it means for the package, is in [its record](../experiments/005-macos-keychain/README.md#what-it-means-for-a-macos-keychain-provider-package); a real background daemon is still untested (A22). The 1Password store has its own probe (A73). What round 6 measures of the providers under ruling 44 (D01, D15 and D16 of round 6, and a real package whose server repeats a named argument, MAN-010) is placed in the roadmap's [table of round 6's evidence](../ROADMAP.md#round-6s-evidence-and-measurements), under S2, S7 and S10 |
| A64 | Require proof of the sender on every request that crosses machines, in both directions, with the mechanism swappable (mutual TLS, SPIFFE, the authorization of the Model Context Protocol). | Ruling 40, point 3e; ruling 43 (D5, Q1) | Written as promises; one vector shows a request to the engine without proof refused, and one shows a direct call to a gated server, from anything but an enforcement point, refused. | in progress: requests to the engine in IFC-022 and IFC-032 (ruling 41, item 26a); ruling 43 adds the gate in front of every server the engine does not run itself, other than a `plain` server reached by `remote` (PRV-025). A `plain` server is outside the gate, by design: it takes none of Conduit's keys, a request to it carries no proof of its sender, even across machines, and whatever can reach its URL reaches it (IFC-007, IFC-041). Ruling 43 also adds the two standard forms of proof with the token form always accepted at the engine's address (IFC-042, DL13), and the engine's check of who answers (IFC-017); no vector. Round 6's D20 asks the first example to show, for a local deployment and an isolated one with the same relationships, which credentials an Agent process can read and what a gate protects, which S2 and S3 measure (ruling 44) |
| A65 | Design local enforcement for v1: each machine enforces access from configuration pushed to it, with a stated bound on how long a revocation takes to arrive. | Ruling 40, point 3f; ruling 43 (Q4) | A proposal for the owner, before v1. Nothing in v0 may assume that every decision is made in one place. | in progress: the shape is recorded by ruling 43 (Q4), in [the multi-machine proposal](proposals/multi-machine/README.md#q4-the-shape-of-v1-recorded-in-a65): on each machine an existing proxy that speaks Envoy's external authorization API enforces; Conduit's own decision point there decides from a versioned copy of the Conduits held under a lease, measured on the machine's monotonic clock, and refuses new requests once it lapses; the bound is the lease term plus the allowed clock error, a value of the conformance document; credentials live no longer than the bound and are renewed on each machine. The promises v1 would change are listed [there](proposals/multi-machine/README.md#in-v1-for-a65): ACC-036, ACC-037, PLC-023, ACC-011, ACC-019, ACC-034, ACC-039, IFC-028, CON-008, IFC-012, IFC-013 and IFC-038; OPS-028's limit on a late onward request of an earlier hook attempt is among them. Ruling 44 keeps that limit for v0, has it tested (E20 of round 6, in S4), and revisits it with this design. The detailed design is still to write |
| A66 | Design policies again as an access feature of Conduit's own, for a later version: a limit placed on an open Conduit without closing it. | Ruling 41, on item 3a of the principles review | A proposal for the owner, before the version that would carry it. | open |
| A67 | Consider three candidate features that no one has asked for yet, which the questions after the rewrite turned up: a forced removal of an instance whose shutdown can never be confirmed; a verb that restarts a ready instance's run and keeps its identity and Conduits, as `kubectl rollout restart` does; an address owned by the project that first submitted it. | [Open questions after the rewrite](proposals/ruling-41/open-decisions.md) | A proposal for the owner, if one is wanted. | open. Round 6's D09 measures a credential's rotation by explicit replacement first, in S2 and S10; if that is disproportionately disruptive, the restart verb comes to the owner with its exact semantics |
| A14 | Run the acceptance scenario on a built engine and use it for a week. | Ruling 4 | Each promise is scored exercised, contradicted or never needed. | open. With it, the state kept for every invocation, measured by the reference model (D19 of round 6), is projected for the week's workload; dropping any of it would change OPS-008 and is the owner's |

### P2 — prune and re-baseline, under the scope guard

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A18 | Present promises that no scenario exercised as candidates for later sequencing, each for an owner ruling. Nothing leaves the accepted scope by default. | D1 | Each candidate under [deferrable scope](archive/plan-audit-history.md#deferrable-scope) has a ruling. | open |
| A20 | Rule on v0 authorization scope. | D5 | The cross-path, issuer-independence and provider-scope questions each have an answer. | open |
| A24 | Adopt a review rule: a new ruling carries a vector that can fail. | D8 | The contributor guide states it. | open |
| A27 | Citation hygiene. | [Reference repositories](archive/plan-audit-history.md#reference-repositories) | The Pi pins are confirmed upstream and the Pi helper probes are retained or their claims removed. | open |

### P3 — vocabulary

| ID | Action | Source | Done when | Status |
| --- | --- | --- | --- | --- |
| A29 | Apply the chosen names as each area is converted into the specification, recording old and new spellings in the vocabulary map. | A28 | No area is rewritten twice for naming. | in progress: the names are applied in `spec/` as each area was written; the vocabulary map was removed by ruling 39, and the words that the promises no longer use are listed with what replaced each in [spec/definitions.md](../spec/definitions.md#words-the-promises-do-not-use). The names of the vocabulary audit of the rewritten specification, which ruling 41 asked for, are ruled (ruling 42) and applied in `spec/` |

## Experiment program

The experiment program, accepted by ruling 9 and amended by later rulings, among them 16, 40,
41, 44 and 45, is in the roadmap: its
[claim, methods and scenarios](../ROADMAP.md#the-experiment-program), the
[order of the work](../ROADMAP.md#the-order-of-the-work) and the row of every
[experiment and probe](../ROADMAP.md#experiments-and-probes). It moved there under A72, and
[the log](proposals/ruling-45/log-roadmap.md) says what moved from where.

### Round 6's evidence and measurements

Round 6's evidence cases and measurements, placed by scenario, are in the
[roadmap](../ROADMAP.md#round-6s-evidence-and-measurements).
