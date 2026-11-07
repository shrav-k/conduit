# Log: the entry points and the tracker

Log of the editor that applied decisions DA1 to DA13 of the [audit record](README.md) to the
entry points and the tracker. Finding numbers are those of the
[entry-points report](reader-entry-points.md); "older text" marks a finding of the
[older-text report](reader-older-text.md). Nothing under `spec/`, `docs/protocol/`,
`docs/research/`, `docs/archive/`, `docs/authoring/` or the proposal records was edited here.

Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

## What changed, by file

| File | What changed | Findings and decisions |
| --- | --- | --- |
| `README.md` | Rewritten. Opens with the specification's sentence and the reason for the language. States what exists and what does not, what binds, and one reading order that begins with `spec/README.md` and `spec/definitions.md`. Links `AGENTS.md`. Expands the Model Context Protocol on first use and defines promise, conformance vector, core package and reference model where they first appear. "Vocabulary comes last" and the row that offered `docs/protocol/` as accepted contracts are gone. | A4, B2, B4, C1, C3, C4, C7, E1, E2, E4; DA1, DA10, DA11 |
| `AGENTS.md` | "Read first" is the specification, the definitions, then the plan audit. The specification is described as three layers, every area written, no vectors. What binds is stated as DA1. The "five names" bullet is replaced. The checks block lists four checks, with what `check_schemas.py` needs. `set_names.py` needs `--write`. The `build-base` branch is marked as not yet published, and the sibling clones as present only on the owner's machine. The standing instruction on pull request 6 is recorded. `docs/protocol/` topic files join the list of what is not edited. | A4, A5, A16, A18, B2, B4, B9, B10, C2, E5; DA1, DA10, DA11, DA12 |
| `CONTRIBUTING.md` | The top half is rewritten: one reading order, what binds, where each kind of work belongs, the same checks block as `AGENTS.md`, and what each check covers. Accepted behavior is a promise in `spec/` changed by a logged ruling; `docs/protocol/` is no longer edited. The claim that there is no backward-compatibility promise is replaced by a pointer to the naming and package promises. The vocabulary section points at `spec/definitions.md`. The licence and ownership sections keep their anchors; one sentence on provisional names now cites rulings 33 to 35. | A4, B1, B9, B11, C2, C3, E5; older text A5; DA1, DA10, DA12 |
| `docs/README.md` | Rewritten as a map. Every wrong row is corrected. The proposal records are listed in the order of their rulings, with the ruling number. Rows are added for `proposals/docs-audit/`, `tools/` and `AGENTS.md`. `docs/protocol/` and the other documents written before the specification are grouped as superseded. Experiment 001 is not listed. "Maintaining the design" states the rule of B1. | A2, A5, A8, A15, A16, A17, B1, B2, B11, C2, C3, C5; older text A1, C7; DA1, DA2, DA4, DA10 |
| `docs/OVERVIEW.md` | Rewritten from `spec/README.md` and `spec/definitions.md`: the five families; documents, templates and instances; Pools, ownership and placement; Conduits and the right to open; a worked example; the lifecycle; operations, outcomes and reasons; packages, contracts and the core package; the three layers and what is outside them. Each section links its promise files. The example is quoted exactly from `spec/schemas/examples/valid/project/`. A short section keeps the heading "One user journey", because `docs/research/CONFIGURATION.md` links to it. | B4, B5; DA7, DA11 |
| `docs/VOCABULARY.md` | The banner is replaced; the body is untouched. The 17 links into Experiment 001 point at commit `7927d68`. | A4, C4, E3, F5; older text C8; DA4 |
| `docs/REVIEW.md` | The banner is replaced; the body is untouched except for one stale phrase about candidate tables and walkthroughs. | A4, A15, B1, B11, E3, F5 |
| `docs/AUDIT.md` | "Where things stand" and "Purpose" are moved above the ruling log, and brought up to ruling 37. The ruling log's rows are unchanged; a table under it lists later amendments. The list of identifier prefixes is completed. The statuses gain `ruled` and `answered`. Action statuses are corrected, the five fix batches are put in order, and A61 is added. A new section lists, for the owner, the six behaviors that appear to have been dropped and the one behavior of the conversion list that needs a decision. Notes say how to read the purpose, the preservation tests and the layered claim, whose wording is kept. Historical banners mark the defect closure plan, the findings, the multi-system options, the three parts of the supporting detail and the reference repositories. Experiment 001's removal is recorded. | A1, A2, A3, A6, A7, A8, A9, A10, A11, A12, A13, A19, A20, B3, C8, D1 to D8, E4; DA3, DA4, DA9 |
| `docs/LANGUAGE_DEFECTS.md` | Fourteen rows that cited removed promises now cite the promises that replaced them; each replacement was checked to exist. A paragraph says the register covers the first sweep and that findings against `spec/` are in the re-audit record. `DLn` is defined, `ruled` and `closed` are redefined to match the rows, and the tense is corrected. A note marks the sweep table's ranges as first written. | A6, A9, B12, D8, E6 |
| `docs/implementation/ENGINE_BUILD_BRIEF.md` | "Documents" and "contents list" replace "declarations" and "manifest". `spec/` and `tools/` are marked as existing. `tools/` is listed in what a builder has. A runtime profile, `PROFILE.md`, is a deliverable. The brief says that a `spec-vN` tag is a revision of version `v0`, and that links from `spec/` into `docs/` lead to records a builder does not need. | A14, B6, B7, B8; DA13 |
| `docs/implementation/INDEPENDENT_BUILDS.md` | The same word changes. `tools/` ships on the build base. The tags are revisions of version `v0`. The directory table says which directories exist. G1 and G3 use "contents list", "each kind" and "documents". | A14, B6, B7, B8; DA13 |
| `docs/implementation/README.md`, `ARCHITECTURE.md`, `ORGANIZATION.md` | A banner on each: not binding, older words, the plan audit is the tracker. Two link labels are corrected. Nothing else is changed. | C6, E3 |
| `experiments/README.md` | The specification is named as what binds. Experiment 001 is recorded as removed, with a link to its last commit and a pointer to `spec/schemas/examples/`. | C6; older text C8; DA4 |
| `tools/set_names.py` | `--check` now reads the list of `_meta` keys from `spec/interface/implementation.yaml` and checks the prefix on every one of them. It also reports a value that looks like one of the three names and differs from `spec/names.yaml`. It still needs no third-party module, and its last line has the same form. | B10 |

## How the fourteen defect rows were repointed

The comment at the head of each file under `spec/` says where a removed promise went.

| Row | Was | Now |
| --- | --- | --- |
| L29 | FST-001 to FST-004 | FST-001 to FST-003, FST-005, OBJ-002 |
| L30 | PKG-015 | PLC-007 |
| L35 | PKG-002, PKG-003 | NAM-006, PKG-003 |
| L37 | PKG-009 to PKG-011 | TPL-003, TPL-004, CTR-004 |
| L38 | PKG-006 to PKG-008 | PKG-006, PKG-007, NAM-011 |
| L49 | INS-001 to INS-017 | INS-001 to INS-014, INS-016, INS-017, OBJ-013 |
| L50 | ACC-025 | ACC-033 |
| L121 | CON-010, LIF-008 | LIF-008 |
| L122 | ACC-003, ACC-004 | ACC-003 |
| L123 | ACC-007 | ACC-006 |
| L133 | ACC-013 | CON-022 |
| L142 | ACC-020, ACC-021 | REC-001, OPS-005 |
| L147 | LIF-007, CON-010 | LIF-007, LIF-008 |
| L148 | ACC-005, CON-010 | ACC-005, LIF-008 |

## What `set_names.py --check` now reports

- A retired name, anywhere under `spec/`.
- One of Conduit's `_meta` keys written with another prefix. The keys are every word that
  `spec/interface/implementation.yaml` writes after the prefix, so a key added there is checked
  everywhere. In that file, any token made of a dotted name, a slash and one word is read as a
  `_meta` key, apart from the Model Context Protocol's own.
- An `apiVersion` whose value is not the one in `spec/names.yaml`.
- An address whose last part is the core package's or the `apiVersion`'s and that is neither,
  and anything that begins with the repository's name and is neither.
- A key under the prefix that the interface file does not define, and a prefix that is close to
  the current one without being it.

It was tested on copies of `spec/` made outside the repository, each with one occurrence
changed. Fourteen changes were tried. Twelve were reported, each with exit status 1:

- a core address with another repository, with a wholly new value, and with a misspelt last
  part;
- an `apiVersion` in a document, with a later version and with a wholly new value, and an
  `apiVersion` in prose with another base;
- the key `outcome`, which the old list of four did not hold, with another prefix;
- the key `instance`, which occurs once, with another prefix;
- a key with a misspelt prefix, and a misspelt key, each outside the interface file;
- a misspelt bare prefix;
- a retired name.

Two were not reported, and they are the tool's limits. It cannot tell a misspelt key in the
interface file itself, because that file is where the keys are defined. It does not recognize a
bare prefix, with no key after it, that is replaced by a wholly different one.

The check passes on the repository as it is, and a full change of names with `--write` on a
copy still ends with a clean check. The tool reads text and knows nothing of YAML. A document
under `spec/` that states a wrong `apiVersion` on purpose, as an invalid example might, would
be reported.

## Findings not carried out, and why

| Finding | Why |
| --- | --- |
| A12, the sentence "The grant is an object" | Left for the owner, as the audit record says. A note says how to read it. |
| A7, action A25 and question Q8 on the Agent lifecycle | The report marks the new status as one for the owner to confirm, and no decision covers it. Both stay `open`. |
| D2, whether ruling 31's "a domain must be named before the engine builds" still stands | An owner decision that no DA decision covers. Nothing was added. |
| D8, whether readiness condition G7 is judged on two clean rounds of the re-audit | Left for the owner, as the audit record says. The register now says what it covers. |
| D9, a state for each readiness condition | An owner call. "None of G1 to G11 is met" is unchanged. |
| D6, moving the historical sections to the archive | The smaller fix was taken: a banner on each. |
| F2, uncommitted work | Nothing was committed, as instructed. |
| F3, the private repository named under "Reference repositories" | Left for the owner, as the audit record says. |
| F4, the branch `implementation/rust-runtime-plugins` | Left for the owner, as the audit record says. |
| F6, the word "proposed" in `tools/check_schemas.py` | Not one of this editor's files. Its header no longer uses the word. |
| F7 and C6 in `experiments/002-runtime-plugins/README.md` | Not this editor's files. |
| Reading `ARCHITECTURE.md` and `ORGANIZATION.md` in full | Only their first 30 lines and every link label were read, since only the banner and one label were to change. |

## Things seen in files this editor does not own

- `spec/README.md` still says in its status line that accepted behavior the promises do not
  cover lives in `docs/protocol`. That differs from decision DA1. Point GQ12 of the
  [conversion list](GAPS.md) already records it.
- The [conversion list](GAPS.md) appeared while this pass ran. It says that behavior GP3, on
  secrets, and the six dropped behaviors are listed in the plan audit. The plan audit's section
  "Behaviors not carried into the specification" now lists all seven.
- The plan audit and the overview describe the specification as it stood before the conversion
  list was applied. The overview names the Language and Engine levels of conformance without
  giving a count, because the list adds a third.
- `docs/research/CONFIGURATION.md` links to `OVERVIEW.md#one-user-journey`. The heading is kept
  for it, with a pointer to the worked example.
- Three archive files link to Experiment 001, which is removed:
  `archive/collaborator-proposals/PR-002-protocol.md`, `archive/reviews/AUTHORING-2026-11-04.md`
  and `archive/reviews/CONCEPTUAL.md`. They were reported as broken while this pass ran and
  pass now.

## Checks

Run from the repository root at the end of this pass. The output is in the editor's report to
the owner's session; this log does not repeat numbers that the other editors are still
changing.

```sh
python3 tools/check_docs.py
python3 tools/set_names.py --check
python3 tools/check_doc_migration.py
```
