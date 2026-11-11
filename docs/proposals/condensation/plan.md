Not binding. The [specification](../../../spec/README.md) alone binds. The final plan of the condensation audit of 2026-11-10 (fourteen agents: seven area readers, a planner, four information-loss skeptics, a finalizer and a critic), kept as it was returned, with local paths removed. The [proposal](README.md) summarizes it; ruling 58 decides it.

## target_structure
**Principle.** Everything is removed or condensed in place, and nothing that decides or explains current behaviour leaves the tree. The finalizer's rule for each skeptic finding: extract it verbatim when the source is an owner's or a coordinator's decision text. Keep the file (condensed in place, at the same path) when the spec or a ruling record names it as the reasoning, or when it is a live input to an open ruling. I checked a merged "decisions register" and recommend against it. The kept decision records are already one file per batch (round-5/DECISIONS, round-6/DECISIONS, ruling-41/open-decisions, core-and-schemas, principles-review, vocabulary-audit, secrets). spec/ comments and AUDIT cite their row IDs, so merging them would re-key about 350 rows and save nothing. Instead:
- spec/README.md gets one entry point, "Where the reasons are".
- Open items get one home: AUDIT's "Known to be open".
- Git keeps every version.

**What an agent reads first** (the same five files, shorter):
- AGENTS.md, which CLAUDE.md loads into every session. About 2.1k words become about 1.6k: the PR-history paragraph shrinks to two sentences and the amended rules go in.
- spec/README.md.
- spec/definitions.md.
- docs/AUDIT.md, about 7.3k words.
- ROADMAP.md, about 4.3k words.

After that:
- **Reasons:** spec/README "Where the reasons are", then the decision records.
- **Open work:** AUDIT's "Waiting for the owner", "Known to be open" and "Action items".
- **Evidence:** experiments/README "Evidence for v0".
- **History:** docs/proposals/ruling-log.md, then the archive index, then git.

**New or rewritten documents**

1. **docs/proposals/ruling-log.md** (new, about 11k words, off the read-first path). It holds:
   - The preamble at AUDIT.md:545-555, verbatim, so that the labelling sentence stands above the first row (ruling 39, point 15).
   - AUDIT's rows 1-37, verbatim. They are the only record of rulings 1-6, 9-12, 16, 26 and 34-36.
   - Under ruling 5, plan-audit-history's "Multi-system options" (:153-185), verbatim: the options table, why (a) was rejected, the evidence rule (no remote support is claimed before a test across two machines), and the owner's "(c), then (b). After that, the same code runs as real processes, then across two machines".
   - A section "Rulings before the plan audit", verbatim, each with a one-line "superseded by" note where a later ruling or the spec changed it:
     - From AUTHORING-2026-11-04.md: M1 (no backward-compatibility promises; users pin a working set), M2 and M3 (:77-95); "Owner decisions before public distribution" (:96-119); "Explicitly later scope" (:790-798).
     - IMPLEMENTATION.md I1-I7, including I2 (native adapters first, macOS first) and I6 (capture defaults).
     - CONCEPTUAL.md C1-C3 and "Review outcomes".
   - The later-amendments tables (AUDIT.md:612-697), including the six coordinator changes ruling 38 accepted. New rows are added for rulings 45-56 from each record's list of rulings it changes, for example ruling 55 A6, which amends rulings 8 and 18 and the wording of 31, 35 and 37.
   - The round-5 changes no ruling accepted (M22, M41 with the PLC-023/LIF-036 choice, M69, the V2-15 reversal), plus M13, M47, M59, M66 and M107. They are recorded as amendments accepted by ruling 57.
   - The record-specific ID prefixes, with rows mapping R1-/R2-/R3-, 3A-3D, 4A-4D, D/R and P to their records or to C0.
2. **docs/AUDIT.md** (condensed, about 7.3k words), in this order:
   - Header.
   - Identifiers (A, S, P, LD, G, L, DL; the probe-to-A map).
   - How to use. It keeps verbatim the convergence rule and the definition of unsafe (AUDIT:165-186), which ruling 38 point 4 put "in the audit file".
   - Where things stand (one dated line per row).
   - What happens next.
   - For the next reader (stub linking cold-readers).
   - **Who decides what** (about 600 words):
     - The role table at :396-404, verbatim (AGENTS.md links it).
     - Ruling 37 as confirmed by ruling 38, point 13: a finding goes back to the owner only if settling it changes what Conduit is for.
     - Ruling 46, point 4: a fix that adds or removes a feature goes to the owner.
     - Ruling 48's roles.
     - Ruling 57's cutoff: everything logged before ruling 57 is accepted, except items 15-19 of the review list and the decisions of the open increment 4. Those, and everything logged after ruling 57, stay strikeable.
   - Waiting for the owner (corrected; see E2).
   - **Known to be open** (new, about 900 words; the one findings list; see E2).
   - Purpose, the preservation tests and the two convention additions (verbatim).
   - Scope guard (moved from plan-audit-history).
   - Current direction.
   - Ruling log: the labelling preamble, 56 one-line rows linking their records, and a stub "Later amendments" that links ruling-log.md.
   - The layered claim with LD1-LD5, all verbatim, since ruling 14 has no other record.
   - Priorities.
   - Action items: the open rows with their done-when, a short "Done since ruling 39" table, and the new A79 (lean vectors).
   - Experiment program (stub).
   - Every heading that any tracked file links stays, as a stub if needed.
3. **docs/proposals/first-build/README.md**, rewritten as the first build's index plus a **settlements register**:
   - One row per finding 1-01..1-18, 2-01..2-08 and 3-01..3-07, with columns: decision, promises, vector, rejected alternative and why.
   - The D19 measurement with its caveat.
   - Two open rows from increment 1.
4. **docs/proposals/projects/brief.md** (new, about 6k words). It is the one input to the Projects rulings and opens with a **Provenance** section.
5. **conformance/s1-safety-cases.md** (new). A verbatim copy of https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md §3 (:515-594), with the six E05 cases, the 23 remaining safety traces and the adjacent open boundaries. It is the meaning of the manifest's `safety_rows` tags.
6. **docs/guides/cold-readers.md** gains a section "The protocol" (what to watch, and the claims checked against MCP 2026-07-28 with their sources) and links AUDIT for the convergence rule.
7. **docs/archive/README.md**, rewritten:
   - One table, "Removed from the tree": Path | What it was | Source material for | Last version (C0). About 45 rows; the ruling-39 rows are kept.
   - Two short verbatim sections under the table: "Not adopted from the collaborator's proposals, and why" and "Prior art still applied".
   - Plus **docs/proposals/ruling-57/README.md**, the ruling.

**The tree afterwards**
- **Root:**
  - README.md.
  - AGENTS.md, with the rules amended and the history paragraph cut.
  - CONTRIBUTING.md. It gains:
    - The removal procedure.
    - The spec head-comment rule, worded to agree with spec/README:290.
    - "Writing promises" (the promise-editing rules).
    - The raw-records rule.
    - The roadmap maintenance pairs.
    - The authoring rules.
    - The license paragraph.
  - ROADMAP.md, with shorter state cells and A79.
- **spec/:**
  - Promises, schemas and examples are unchanged byte for byte in data.
  - Head comments shrink to: what the file holds, its standing, reading notes, the standards it follows, open issues, **the removed-ID register**, and one pointer to the reasons.
  - spec/README "History" becomes "Where the reasons are".
- **docs/:**
  - AUDIT.md.
  - README.md: the documentation map, about 1.7k words.
  - OVERVIEW.md: kept; rewritten after the spec batches.
  - LANGUAGE_DEFECTS.md, condensed. It keeps the 30 open type-layer rows and the 16 "ruled; vector pending" runtime rows in full, plus a crosswalk of every other decided row with its one-line scenario and the L-to-DL map.
  - guides/.
  - implementation/:
    - FIRST_BUILD.md, with "What increment 3 needs" kept unchanged.
    - INDEPENDENT_BUILDS.md (state now).
    - ENGINE_BUILD_BRIEF.md.
    - WORK_ORDERS.md, which gains "The record" for orders 001-029, with each order's decisions, and the lessons.
  - work-orders/:
    - README (about 150 words), verify.md, verify.schema.json.
    - Four reusable schemas: report.schema.json (from 004), results.schema.json (from 005), harness.schema.json (from 002) and study.schema.json (from 003).
    - Open orders only.
  - research/: README (about 150 words), SECRETS.md, MULTI_MACHINE.md, naming/README.md, and vocabulary/SIMPLIFICATION.md (kept as written, ruling 36).
  - archive/: README.md and plan-audit-history.md (condensed; check_roadmap reads it).
- **docs/proposals/:**
  - ruling-log.md.
  - The READMEs of rulings 38-57:
    - Ruling 38's is condensed.
    - Ruling 39's gains "The assessment point 8 confirmed", verbatim from INDEPENDENT_BUILDS.
    - Ruling 41's gains "Planner's choices" and "Choices that set behavior".
    - Rulings 42 and 43 gain "Choices that set behavior" and lose their log-list paragraphs.
  - ruling-41/open-decisions.md: "For the owner" (the option essays) is kept verbatim; only "Left for round 5" goes.
  - **secrets/README.md**, condensed in place at the same path. The spec names it as the reasoning behind the secret type.
  - principles-review/.
  - multi-machine/ (lightly condensed).
  - vocabulary-audit/, core-and-schemas/README.md, docs-audit/{README, GAPS}.
  - The 13 older batch records (rulings 7-31), condensed in place at the same paths.
  - reaudit/:
    - README.md: the decision tables, the follow-up pass and its disagreements, "Choices that set behavior", and the index of rounds 1-6.
    - round-5/{README, DECISIONS, log-fixes}.
    - round-6/{README, DECISIONS, design-critique, reader-1, log-integration}.
    - round-7/ when that round starts.
  - first-build/:
    - README.md (the register), to-review-with-the-owner.md, log-increment-4.md.
    - increment-2/decisions.md.
    - increment-3/closeout-results-2026-11-10.md.
    - increment-4/{measurement-plan, coordinator-run-2026-11-10, fresh-author-run-2026-11-10}.md, plus the two raw run directories until the S1 rerun.
  - language-shape/:
    - README.md.
    - short-form/{synthesis (with "Carried from the papers"), critic}.md until the spec batch.
    - [project/](../projects/brief.md): brief.md, plus [synthesis, critic, design-P4, inventory-use-cases and inventory-prior-art](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project) until the Projects rulings are recorded.
  - simplification/README.md.
  - enrollment/ (3 files), sign-in/ and owner-direction/: kept until written into the spec, with superseded notes.
- **experiments/:**
  - README.md, with "Evidence for v0".
  - 004: README and model, until A40.
  - 005: README and probe.
  - 007: README, results, setup/ and evidence-2026-11-09/prompts.txt.
  - 008: README, HARNESS.md, two results files, and the two evidence READMEs.
  - 009: README, HARNESS.md, results, and the evidence README.
  - 010: README, results.
- **conformance/:**
  - s1-promises.yaml (the frozen S1 set).
  - s1-safety-cases.md.
  - README gains "Planned work": lean vectors, coverage, safety tagging, runtime scenarios, and the evidence of vector strength so far.
  - Vectors unchanged.

## removal_list
E-numbers refer to the extractions list. Each extraction is done and verified first. "Last version" always means one pinned commit, C0 (see the sequence). Anything a skeptic showed to hold unextracted reasoning is now either extracted or kept.

**A. docs/proposals/reaudit/: 57 files, about 219k words**
- **Extract first:**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md: E15 (the form check) and E19 (rules 1-3, 6 and 7 go to CONTRIBUTING "Writing promises").
  - https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4 (whole: README, DECISIONS, log-fixes, mcp-recheck, reader-1..4): E2 and E3 (mcp-recheck:241-261), E5 (summary) and E1 (4A-4D).
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-6.md: E3 (:568-624).
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-d06-d12.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-conventions.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-core.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-interface.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-promises.md: E4.
- **Choices extracted (E35), then removed:**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-group-a.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-group-b.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-group-c.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-c.md.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F1.md to https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F7.md.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-specdocs.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/log-tracker.md. Their open item, choice 7, is log-integration item 3, which is kept with its status.
- **Summary or ID-prefix row only:**
  - https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2 (whole, 12 files): E5 and E1 (R1-/R2-/R3-).
  - https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3 (whole, 7 files): E5 and E1 (3A-3D).
- **No extraction (findings all applied; the decisions are in the kept DECISIONS files):**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/reader-1-duplicates-and-contradictions.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/reader-2-terms-and-shapes.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/reader-3-undetermined-behavior.md.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/handoff-check.md.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-1.md to https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-5.md.
- **Condensed by deleting sections:** README.md (drop only :1-37, replaced by the index, and :125-157; keep :241-284), round-5/README.md, round-5/log-fixes.md and round-6/log-integration.md (E5).

**B. docs/proposals/first-build/: about 44k words now, 8.8k later**
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md go after:
  - E6: register rows with reasons; D19 with its caveat; https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md#L680-L706.
  - E13: lean vectors; coverage; the mutation evidence at https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#L390-L394, :1062-1070, :1094-1112 and :1209-1220.
  - Plain-text citations in tools/check_schemas.py:83 and :221, and in spec comments, become finding IDs.
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md: E12 and E13 (H12). reference-model/CONFORMANCE.md:63-67 is retargeted to its live destination, not to C0 (sequence, step c).
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md: E7.
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3/closeout.md: E8, extended. reference-model/examples/s1/README.md:44-46 is retargeted to closeout-results#how-to-run-the-open-checks.
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md:
  - E14 (§2, the frozen S1 set).
  - E36 (§3, verbatim to conformance/s1-safety-cases.md).
  - E13 (D08, :136-155).
  - A verifier check that §4-§7 are carried by conformance/README, increment-2/decisions.md and the code. Anything not carried goes to the register or to Known to be open.
- https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/s1 (9 YAML files): no extraction. Remove the loader test s1_drafts.rs, or point it at reference-model/examples/s1. Repoint conformance/fixtures/s1/README.md:7.
- **Later, after the S1 rerun on the simplified language:** increment-4/coordinator-run/ (17 files) and fresh-author-run/ (57 files). Each run's counted project files and scripts stay, as 007's setup/ does, because v0 must account for failed runs. Do E9 now.

**C. Ruling and batch records: about 173k words**
- **Choices extracted (E35), then removed:**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G1.md to https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G7.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-core.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-docs.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md.
  - ruling-42/: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-conventions.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-core.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-docs.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-editorial.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-integration.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-language.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-machines.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-rename-a.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-rename-b.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-secrets.md. The old-to-new names are in spec/definitions "Words the promises do not use".
  - ruling-43/: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-core.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-docs.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-integration.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-interface.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-providers.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-43/log-secrets.md. MX-21 moves to ruling-log.md.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md and [log-promises.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-promises.md) (also E20's spot-check of the 22 "left undone" items).
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-entry-points.md, [log-older-text.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-older-text.md), [log-gaps.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/log-gaps.md).
- **No extraction:**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/check.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/handoff-check.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/log-spec.md. Repoint spec/core/README to ruling-38/README.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/APPLY.md (X1-X5 are in ruling row 37 and the README's Outcome).
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-entry-points.md and [reader-older-text.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/docs-audit/reader-older-text.md).
- **Extract first:**
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md: E16 and E19 (rules 3-5 and 8 go to "Writing promises").
  - ruling-45/log-roadmap.md: E19.
- **Condensed in place, not removed:**
  - **secrets/README.md (E18).** Only sections that are applied or superseded go; the reasoning stays at the same path, so spec/README:550 and secrets.yaml:8-9 stay valid.
  - ruling-41/open-decisions.md (E17: "For the owner" kept verbatim).
  - The READMEs of rulings 38, 42 and 43.
  - core-and-schemas/README.md.
  - vocabulary-audit.
  - multi-machine (E20, revised keep list).
  - docs-audit/README.md.
  - principles-review §5 (optional).
  - The 13 older records: conventions, schemas-and-interface, vocabulary, naming, placement, instantiation, normative-surface, relationships, operations, declarations, graphs-and-schemas, packages-and-start, consolidation (E20).

**D. Language-shape and simplification papers: about 239k words now**
- **Remove now:**
  - https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research (whole, 13 files): E21, E11, E22, E9 and E36's to-do ([map-constraints §1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-constraints.md#1-guarantees-that-must-survive)).
  - short-form:
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N1-normative-sugar.md (none).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N2-new-core.md (E21).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N3-project-document.md (E22).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md (none).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/inventory-build.md (E21).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/inventory-constructs.md (none).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/inventory-prior-art.md (E22, now all thirteen points and the five conditions).
    - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/inventory-scenarios.md (E23).
- **After the brief carries their provenance quotes and listed points, and the admin checks it (WO-037):** https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/design-P1-sixth-family.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/design-P2-scope.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/design-P3-hierarchy.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-authority.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-ownership.md (E23).
- **Later, in the PR that records the Projects rulings:** https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/synthesis.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/critic.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/design-P4-minimal.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-use-cases.md and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-prior-art.md. They are live inputs to an open ruling.
- **Later, after the spec batches for rulings 49-56 (E37 first):**
  - short-form/synthesis.md and critic.md go.
  - The three enrollment files become one record of about 2k words.
  - sign-in is condensed; its dated client facts go to docs/guides/accounts.md.

**E. docs/research and docs/archive: about 108k words**
- **docs/research/:**
  - AUTHORIZATION.md: E2 (A66's Source cell) and E25.
  - CLOUD_AUTHORITY.md, CONDUITS.md, ESTABLISHMENT.md, OBSERVABILITY.md, PI_CONNECTORS.md, PI_RUNTIME.md, RESOURCES.md and ECOSYSTEM_ORGANIZATION.md: E25 (RESOURCES also gets the A50 pointer in E2).
  - CONFIGURATION.md (:115-145, :220-238), CONSTRUCTION.md (:82-95) and RUNTIME_ESTABLISHMENT.md (:169-186): E25 "Prior art still applied".
  - DISTRIBUTED_RUNTIME.md and RUST_RUNTIME.md: no extraction.
  - language-protocol-foundations/ (whole, 6 files): FORMATS.md:53-70 goes to E25.
  - vocabulary/README.md, PRECEDENTS.md and SOURCES.md: E25 (ruling 57 lists ruling 36 as amended). **SIMPLIFICATION.md stays as written.**
- **docs/archive/:**
  - collaborator-proposals/ (whole, 4 files): E25 (attribution, plus REVIEW.md's "What needs adaptation" and "Additional roadmap candidates", verbatim) and the CONTRIBUTING license paragraph (E19). ROADMAP:84 points to C0.
  - migrations/ (whole, 3 files), together with tools/check_doc_migration.py, the CONTRIBUTING block at :176-186 and docs/README's tools row.
  - reviews/ (whole, 4 files): only after E1's "Rulings before the plan audit" is merged. REPOSITORY §5 and §6 go to E2's waiting question and to E25.
- **Condensed:** research/README.md, archive/plan-audit-history.md (E26; Multi-system options moves verbatim to E1), docs/LANGUAGE_DEFECTS.md (E27) and archive/README.md (E25).

**F. experiments/: 942 files, about 148k words.** Details are in the experiments section.
- 005: results.json, results-rerun.json, results.txt, results-rerun.txt.
- 007: evidence-2026-11-09/, except prompts.txt.
- 008:
  - src/, tests/, Cargo.toml, Cargo.lock, rust-toolchain.toml, .gitignore.
  - The raw files of evidence-2026-11-09/ (819 files) and evidence-2026-11-09-later-turn/ (37 files), except each directory's README.md.
  - **HARNESS.md stays.**
- 009:
  - src/, tests/, Cargo.*, rust-toolchain.toml, .gitignore.
  - The raw files of evidence-2026-11-09/, except its README.md.
  - **HARNESS.md stays.**
  - The "source" field of packages/agent-claude-code/tests/fixtures/stale-init.json becomes a C0 link.
- Later: 004's code, once A40's model runs in reference-model.

**G. docs/work-orders/: about 24k words**
- 001 to 028 (.md, 28 files): E30, including each order's decisions. E10 corrects the review-list citations.
- 001's schema.
- Renamed, not removed:
  - 004's schema becomes report.schema.json.
  - 005's becomes results.schema.json.
  - 002's becomes harness.schema.json.
  - 003's becomes study.schema.json.
- 029 goes once its record row exists.

**H. Sections removed from live files (no file removed)**
- AUDIT: rows 1-37, the amendments and the prefixes go to E1, verbatim. The rest goes to git. The convergence rule, the LD table, the role table and the labelling preamble stay.
- ROADMAP: "Open in the order", plus long state cells.
- docs/README: "Maintaining the design".
- INDEPENDENT_BUILDS: "State on 2026-11-06" moves verbatim to the ruling-39 record (E31).
- AGENTS.md: the PR-history paragraph.
- spec/: change narrative only; removed-ID registers and "Standards followed" sentences stay (E34).
- FIRST_BUILD.md: nothing removed ("What increment 3 needs" stays; E32).

## extractions
Each item gives source, then destination. The verifier checks each against its source at C0. Copy, do not paraphrase, where the source is an owner's or a coordinator's decision text, an option put to the owner, or a method.

**E1. docs/proposals/ruling-log.md (new)**
- AUDIT.md:545-555: the preamble, including the labelling sentence (ruling 39, point 15).
- AUDIT.md:556-592: ruling rows 1-37, verbatim.
- Under row 5: plan-audit-history.md:153-185, "Multi-system options", verbatim.
- A section "Rulings before the plan audit", verbatim, each with a one-line "superseded by" note where the spec or a later ruling changed it:
  - From docs/archive/reviews/AUTHORING-2026-11-04.md: :77-95 (M1-M3), :96-119 (owner decisions before public distribution) and :790-798 (explicitly later scope).
  - IMPLEMENTATION.md :19-108 (I1-I7).
  - CONCEPTUAL.md :24-102 (C1-C3 and the review outcomes).
- AUDIT.md:612-697: the later-amendments tables, including the six coordinator changes ruling 38 accepted. Add rows for rulings 45-56 from each record's "earlier rulings it changes". For example, ruling 55 A6 amends rulings 8 and 18 and the wording of 31, 35 and 37, while ruling 19 is kept.
- AUDIT.md:698-721: M22, M41 with the PLC-023/LIF-036 choice, M69 and the V2-15 reversal. Add M13, M47, M59, M66 and M107. All are recorded as amendments accepted by ruling 57.
- AUDIT.md:33-75: the ID prefixes. Add rows: R1-/R2-/R3-, 3A-3D and 4A-4D point to the C0 round directories; D/R (ruling-41 log-integration) to C0; P to ruling-41/README#planners-choices.

**E2. docs/AUDIT.md**
- **Keep verbatim:**
  - Purpose; the preservation tests and the two convention additions.
  - The layered claim with LD1-LD5 (:755-775).
  - The convergence rule and the definition of unsafe (:165-186).
  - The role table (:396-404).
  - The ruling-log preamble.
- **Who decides what:** as in the target structure. It names ruling 57's cutoff and its exceptions: review items 15-19, and log-increment-4 until increment 4 closes and its decisions reach the joint review.
- **Waiting for the owner:**
  - A20 and the Projects rulings:
    - Language-shape decisions 5 (derived defaults) and 6 (the skeleton).
    - [inventory-ownership point 10](../projects/brief.md#ownership-and-lifecycle-constraints): does a referenced Pool block retirement?
    - A request that the owner confirm his Projects direction in his own words. The papers quote him differently, and the approvals-of-widenings direction is not his (see E23 Provenance).
  - The S1 rerun's counting rules (E11).
  - Public name and namespace: PR 14, ruling 47 reserved, A75 and A76.
  - The joint review of items 15-19.
  - How v0 is released: REPOSITORY-2026-11-02 §5 (one coordinated release, separate version domains, macOS archives first, no marketplace, artifacts tested outside the workspace), with the C0 link.
- **Known to be open:**
  - The known-open list at :204-209, :222-224, :254-266 and :287-316.
  - Round-6 items: 1 is open and decided with the S1 edit-cycle measurement; 3 is open (INS-010's comparison of destinations); 4 is open (no checker rule for SEC-005); 2 is settled by 2-02; 5 is covered by e05-escaping.
  - From mcp-recheck:241-261:
    - MCP issue 3003: whether an isError result must conform to outputSchema, which is IFC-025's own reading.
    - The tasks extension at "Beta, to Core", where breaking changes are allowed (IFC-016).
  - First-build 4-12 and 4-17, linked to log-increment-4.
  - Round-5 README:93-124 in one line: 22 interface promises changed and 2 added, not rechecked against the protocol; 101 promises changed and 3 added, not read cold.
  - The S1 safety set:
    - 74 manifest cases carry no safety_rows and 38 carry an empty list.
    - Safety vectors beyond S1's rows are undefined, and the spec has no safety field (INDEPENDENT_BUILDS G5).
    - The adjacent open boundaries (plan.md:586-592): the -32602 assumption; IFC-046 wire resends kept apart from repeats; the filtered-replay variants of 6-1-02, which need Conduit-aware fixtures.
  - The closeout:
    - The user-settings sentinel check (an `owner_sentinel` MCP server and an output-style sentinel) was required but neither run nor listed as not shown.
    - Claude Code's user skills were not tried.
  - From log-increment-1:
    - :680-686: saphyr-parser reads a `---` line inside a root block scalar as content; the fault is to be reported upstream.
    - :695-706: patternProperties and the additionalProperties beside it are left to the jsonschema crate's engine, which is not ECMA-262.
  - packages/agent-codex/README.md:94 says the real-client check is pending, but order 027 found the six collaboration.* tools still listed. 4-06 is open.
  - The Projects papers' spec gaps:
    - [inventory-authority "Unverified"](../projects/brief.md#unverified-authority-gaps) (:177-185): how a bound judges reconcile.v1 through access; that registering another core#management.v1 or core#access.v1 Resource is not forbidden; which Conduit carries ACC-033 row 1.
    - [inventory-ownership:256](../projects/brief.md#package-removal-gap): PKG-006 and NAM-011 speak of removing a package, but no operation removes one.
  - multi-machine §6's caveat "No wider survey of spec/ was made", against A65's done-when.
  - LANGUAGE_DEFECTS:41 says runtime rows are "recorded and tested", but reference-model/CONFORMANCE.md cites no L-ID.
  - The 30 type-layer defects (a pointer).
  - Text not yet read cold.
- **Action items:**
  - Keep each done-when.
  - A63: the candidates.
  - A64: plain servers stay outside the gate (IFC-007, IFC-041).
  - A65: pointer to multi-machine#in-v1-for-a65, plus the §6 caveat.
  - A67: three features (forced removal; a restart verb only if D09 shows replacement too disruptive; a project-owned address).
  - A14: D19's state projected over a week, which bears on OPS-008.
  - A66's Source cell, from AUTHORIZATION §2/§4 and CLOUD_AUTHORITY §9:
    - An agent's access must not become its launcher's full rights.
    - A skipped required restriction is never satisfied.
    - Revocation needs an admission and consistency contract.
    - Administering a grant is distinct from using it.
    - Cedar, OPA and Biscuit are options behind a contract.
  - A50's Source: RESOURCES:148-250 at C0.
  - A44's Source: conformance/README "Evidence of vector strength so far" (E13).
  - A68's done-when gains [map-round6 §1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-round6.md#1-bottom-line) (E9).
  - New A79: lean vectors.
- **Scope guard:** from plan-audit-history, next to Purpose.

**E3. docs/guides/cold-readers.md**
- Link AUDIT's convergence rule and its definition of unsafe; do not move them.
- New section "The protocol":
  - What to watch: mcp-recheck:241-261, SEP-2145, and clients that hide protocol errors.
  - The claims checked without a finding, with their sources: reader-6:568-624, against 2026-07-28. Use them when the IFC-005 baseline moves.
- Deduplicate where findings go.

**E4. reaudit/round-6/DECISIONS.md, "For the owner" (anchor kept)**
- log-d06-d12:46-56: the alternative to choice 2, which makes the engine the last step of TPL-004's order and keeps a template's override.
- log-d06-d12:64-71: why a reference that resolves to nothing stays invalid, not conflict (Kubernetes' 404), and that changing it would change OPS-029.
- log-d06-d12:75-78: inspect.v1 reports conflict on an ID mismatch. expectedId was later adopted by ruling 53.
- log-conventions:56-68:
  - OPS-024: a new attempt answered with a reason leaves the outcome and the kept outputs unchanged.
  - OPS-034: a no-filter repeat whose outputs are no longer kept reports conflict, though re-dispatch would be safe.
- log-core:72-77: NAM-028 now rejects a stale #name under replace.v1.
- log-promises:51-70:
  - CTR-012's inputs are the root properties only; a branch that declares properties is not rejected.
  - SUB-009's equality goes through NAM-009, including the project digest.
- log-interface:52-61 (the IFC-033 bundle):
  - Rewrite $ref rather than embed $id.
  - Copy keys take the form address@version#name.
  - Replace in place only when no keyword sits beside the $ref.
  - The allOf form is fixed.
  - No tool is presented in either TPL-013 case.
- log-interface:84-93 (costs):
  - 46 copies for register.v1 and 40 for replace.v1, which clients' composition limits may hit.
  - A default under the presented allOf is outside SCH-008.

**E5. reaudit records condensed in place**
- README.md:
  - Keep :39-124 (conclusions, handling, decision tables, with the rows rulings 38 and 41 made moot marked).
  - Keep :158-240 (follow-up decisions and the editors' readings).
  - Keep **:241-284**: the follow-up pass with its "Why" column (ACC-030/ACC-006 self-widening; a Pool template declares no Conduits) and the five disagreements corrected.
  - Replace :1-37 with an index of rounds 1-6 that absorbs two lines each from the round-2, round-3 and round-4 READMEs.
  - Drop :125-157.
- round-5/README: keep the status, the links and :93-124.
- round-5/log-fixes: keep :128-152 and :197-313.
- round-6/log-integration: keep :95-123 with the E2 status, and :153-242.

**E6. first-build/README.md: index plus settlements register**
- One row per finding: ID, decision, promises, vector, rejected alternative and why.
- Increment 1 (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-1.md):
  - 1-01: SUB-005 for every schema rejection; per-keyword annotations would change every schema.
  - 1-04 and 1-17: ECMA-262 kept; I-Regexp would force rewrites without lookahead; ruling 48 point 3 restricts tokens instead.
  - 1-06: dot-names are skipped, taking only the dot part of Terraform's rule (the case was macOS tar's ._ files).
  - 1-07: a JSON non-map is one document (SCH-013).
  - 1-09: PyYAML kept; the tool is not the reference.
  - 1-10: wholeness on the binary64 value, with ±(2^53−1).
  - 1-11: SUB-011, not SUB-005, because a document beyond a limit is not malformed.
  - 1-12: two I-JSON rules; noncharacters allowed.
  - 1-13: ABNF case-insensitivity.
  - 1-14: SCH-015 for a $ref to a non-Schema.
  - 1-18.
  - Two open rows: the saphyr `---` fault to report upstream (:680-686) and patternProperties left to the jsonschema crate (:695-706).
- Increment 2 (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md):
  - 2-01: two BOMs rejected (YAML 1.2.2 prefix), applied in the vector only.
  - 2-03: different unconditional defaults rejected under SCH-009; equal ones count as one.
  - 2-02 and 2-04 to 2-08.
- Increment 3 (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-3.md#L11-L24): 3-01 to 3-07, including 3-02 (model required, empty default) and 3-06 (code_mode_host on).
- D19 (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#L1010-L1014):
  - Records-purge bytes fell from 9,187 to 4,495.
  - The unknown write's entry stayed at 373,365.
  - The invocation array grew from 499,935 to 622,636.
  - Carry its caveat verbatim: "measurements of this vector, not a capacity projection".
- Noted for later:
  - Replace the token-helper note with experiment 008's answer: Claude does not re-run the helper after a 401; Codex re-runs it but fails initialization with "Auth required". This bears on PRV-022.
  - Confirm the missing-plain-tool note against IFC-002 (unavailable), then drop it.

**E7. increment-2/decisions.md under 8a, about 300 words, from https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/check-plain-errors.md**
- The conclusion (:7-18).
- The tags and commits checked (:36-42 and :230-233):
  - TS SDK v2.4.0 951fe560.
  - Python v2.3.0 2118f14f.
  - rmcp-v3.9.0 5596a8a0.
  - servers v2.3.0 36380287.
  - **reference-servers v2.2.0, 194cc86e3ae6400958555bd4116849d822d69263**, which holds filesystem 2.1.0, the version S1 pins.
  - Revisions 2024-11-05 to 2026-07-28.
- The filesystem 2.1.0/2.2.0 result (:247-252).
- Why IFC-010's options A and C were rejected (:271-288), verbatim.
- IFC-046 option B, not adopted (:297): a no-effect guarantee for Conduit-aware implementations, dropped by 2-07 because it is a feature for the owner.

**E8. increment-3/closeout-results: new section "How to run the open checks", from closeout.md**
- :25-35: the evidence rules.
  - Never keep tokens or headers.
  - Keep the engine's lists, the native observations and the model reports apart.
  - not_reported and session_init_unverified count as unshown.
- **:37-54, the sentinel method, verbatim:**
  - One sentinel at a time.
  - A sentinel shown to work in a baseline run outside the engine, or it establishes nothing.
  - Baseline turns authorized and counted apart.
  - The rows for Claude instructions, Codex rules (--ignore-rules, project_doc_max_bytes=0), hooks, and user settings (the `owner_sentinel` MCP server and an output-style sentinel).
- :55-70: the method for each check:
  - Claude account services false, with a connector baseline and a service-side receipt.
  - Codex apps false.
  - Claude's outgoing peers SendMessage, ListAgents and RemoteTrigger.
  - Refusal of incoming cross-session messages.
  - Opt-in per Agent, in separate project copies.
  - Review of Codex's --disable list for every reach combination.
- :88: rotating a real client's token needs an authorized harness.
- **:163-164:** how to show one conversation per Agent.
- **:177-199:** the routes-around-the-engine method (ACC-027), which feeds A7.
- A closing line inside this new section, not an edit to "What is not shown": "Not run: the user-settings sentinel; Claude Code's user skills were not tried."

**E9. first-build/log-increment-4.md and A68**
- From the fresh-author report.json "unclear", the uncertainties no finding covers:
  - file.write.v1 has no create-if-absent.
  - No operation shows a Resource's package version.
  - Negative permission checks need another route.
- Into 4-12, from t6-recovery-notes:10: "a failed turn poisons its conversation".
- [map-round6 §1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-round6.md#1-bottom-line), into A68's done-when in AUDIT: none of round 6's applied simplifications (D06, D07, D11, D12, D14, D18, D21) saves a line of S1, and ruling 41's cuts 3a, 10 and 17 moved cost into T4, T5/T6 and T7.
- [map-constraints §1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-constraints.md#1-guarantees-that-must-survive) goes to E13 and E2 (safety tagging), not to the log.
- The owner's bar, quoted only in coordinator-run:28, goes to FIRST_BUILD (E32).

**E10. to-review-with-the-owner.md (corrected mapping)**
- Item 3's cap now exists: CONFORMANCE.md:30, 64 MiB and 7 days.
- Item 11: "work order 024" becomes "WORK_ORDERS record, 024"; 3-03 stays.
- Item 12: the same; 3-05 stays.
- Item 16: "work order 027's record" becomes "finding 4-06; WORK_ORDERS record, 027".
- **Item 17** has no finding. "work order 028" becomes "work order 028, decision 5, quoted in WORK_ORDERS record, 028", plus the C0 permalink to 028:45-49.
- Item 19: "work order 027" becomes "finding 4-04 (TPL-018, OPS-022); WORK_ORDERS record, 027".
- Items 18 (4-17) and 15 (4-06) are unchanged.

**E11. increment-4/measurement-plan.md: addendum "Options for the rerun's counting rules"**
- [simplification synthesis:404-416](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/synthesis.md#L404-L416) (decision 11):
  - The primary reading is file lines plus command lines that carry chosen values.
  - A line removed and restored in one task counts once.
  - Strict and JSON readings are reported beside it.
  - Type statements and the history convention count as provider work.
- short-form critic gap 1: block style gives T1 32 lines, idiomatic block about 38, which fails, so init and render should use flow style.
- short-form critic gap 10.
- [simplification critic gap 3](../first-build/increment-4/measurement-plan.md#options-for-the-reruns-counting-rules): which run decides the bar.
- [simplification critic gap 18](../first-build/increment-4/measurement-plan.md#options-for-the-reruns-counting-rules): separate installs make T1 18.
- AUDIT's waiting row points here.

**E12. reference-model/README.md: "The core's boundary"**
- From https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-3-handoff.md#L9-L175, rebuilt from the current code, not copied:
  - Input kinds, including BuiltinRunStopped.
  - Requests, intents and completions.
  - The identity materialization.
  - The setup-report shapes.
  - The shell's duties.
- Alternatively, doc comments on conduit_core's types.

**E13. conformance/README.md "Planned work", plus AUDIT A79 and A44, and the ROADMAP citation**
- The lean-vector proposal (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#L1137-L1175):
  - The measurements: 8.2 MB in 1,269 files; 361 cases at 6.67 MB; 224 snapshot recipes; 207 repeated register inputs; 129 whole-array expectations.
  - The five steps.
  - conformance/ is now 12 MB and about 414k words.
- Coverage work (:1128-1135, H12):
  - Port the loader-only rows.
  - Semantic boundary pairs.
  - The "before T5" release-compatibility reason at manifest:5168, though T5 ran.
  - The items that need shell evidence.
- plan.md:136-155: the D08 variant of nested Resource replacement.
- Safety tagging: tag the 74 untagged cases and review the 38 empty lists against s1-safety-cases.md.
- Port the scenarios of the 16 "vector pending" and 29 DL-mapped runtime rows of LANGUAGE_DEFECTS.
- **"Evidence of vector strength so far"**, verbatim:
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md#L390-L394: two resolver mutations, each failing all three S1 variants.
  - :1062-1070: nine targeted mutations, each caught.
  - :1209-1220: the 75-run record-omission fault injection, each omission caught at /records.
  - :1094-1112: increment 2's done-when table with its limits.
  - This is the only evidence toward A44 and G6.

**E14. conformance/s1-promises.yaml (frozen)**
- The 308 IDs of plan.md §2.
- Point lib.rs:145-147 and :206-260, manifest.yaml:6883 and README:399 at it.

**E15. tools/check_schemas.py: new steps**
- From https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#L136-L151: exactly the seven promise fields, the layer and level values, and unique IDs across spec/promises, spec/conventions and spec/interface.
- **No live ID matches a retired ID** named in any head comment's removed-ID register or in spec/README "Promise files that rulings removed".
- Every `safety_rows` value in conformance/manifest.yaml names a row of conformance/s1-safety-cases.md.
- If current text fails, report it; do not fix it.

**E16. ruling-41/README.md: "Planner's choices" (anchor #planners-choices)**
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md#planners-choices: P1-P20, each with its reason and alternative.
- Notes: P2 was changed by the owner, so that no removed ID can be reused. P1's names were replaced by N1-N20 (ruling 42).

**E17. ruling-41/open-decisions.md, condensed in place**
- **Keep "For the owner" (:128-436) verbatim**, anchors kept. It holds the D5, D6, D8, D10, D11, D12, D17, D20 and R5 option essays that ruling-42/README cites by letter.
- Keep rows 1-44 with their precedents, "Settled together", and the candidate list (A67).
- Drop "Left for round 5" (:437-end) only after the verifier finds each item settled. Any that is not goes to Known to be open.

**E18. secrets/README.md, condensed in place (same path)**
- Drop only:
  - Contents (:266-283).
  - "[What changed since the first draft](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#what-changed-since-the-first-draft)" (:284-416).
  - "[Existing promises and documents that would change](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#existing-promises-and-documents-that-would-change)" (:2899-2946).
  - The "Fits." paragraphs inside S1-S21.
  - "[Promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)" (:2418-2898), but only after the verifier maps each drafted SEC-n to spec/promises/secrets.yaml or to principles-review §4's dropped list. Any draft sentence carried by neither (for example "DL12 would add ...") goes to Known to be open.
- Keep verbatim:
  - The head.
  - "Decisions for the owner".
  - The problem and what the owner had ruled.
  - What existing systems do.
  - The one-page proposal.
  - S1-S21's questions, options, precedents, recommendations and "What it cannot do".
  - "Left out of the first version" (:2947-3015).
  - "Where the recommendation is least certain" (:3016-end).
- spec/README:550, secrets.yaml:8-9, principles-review §4 and experiment 005's citations stay valid.

**E19. CONTRIBUTING.md**
- ruling-45/log-roadmap:135-139: four pairs of facts held in two places, kept in step by hand: A68 against step 3; A63 against A22 and A73; the experiments index against the 001-005 rows; P1 against "In parallel".
- **"Writing promises"**, verbatim where general:
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#L11-L33, rules 1, 2, 3, 6 and 7: the seven fields; folded `>-` text indented four spaces; lines under 97 characters; IDs never reused; a new ID takes the next number never used, after checking the head's register; no promise moves between files; exactly one of refused, invalid, conflict and unavailable; cited IDs must exist.
  - https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md#L64-L83, rules 3, 4, 5 and 8: the tombstone form, with the ruling and "where it went / nothing replaces it"; content moves by a new ID; a rewrite keeps what it does not name.
  - The head-comment format is now the removed-ID register (E34).
- vocabulary:42-48:
  - One word for one idea, and one idea for one word.
  - Plain technical words for authoring and operation; flow imagery only around Conduits.
  - Type and profile words are settled with their type or profile.
  - "Kept distinct" stays in the record.
- consolidation:94-100: a proposed shape is checked against the CS/CC list before it is offered.
- docs-audit DA8, DA10, DA11 and DA12.
- The license paragraph: PR #1 (2111b0d), PR #2 (c480e96), the merges 71d6348 and 4b3db58, and PR #5 (2890ef7).
- New rules:
  - The removal procedure.
  - A spec head comment holds what the file holds, its standing, reading notes, the standards it follows, open issues and its removed IDs (spec/README "The form of a promise"). It carries no change narrative.
  - Raw run records stay in git once their results are merged.

**E20. Records condensed in place by deleting sections only**
- core-and-schemas/README: keep (b), (c) and Outcome (C1-C56 and N1-N42 with reasons). Spot-check the "left undone" items of [log-documents](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-documents.md#left-undone-and-why) and [log-promises](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/core-and-schemas/log-promises.md#left-undone-and-why); any unsettled go to E2.
- vocabulary-audit: keep N1-N20 and "Considered and kept".
- **multi-machine:**
  - Drop Contents (:126-141).
  - Drop §7 (:930-978) and §9 "In v0" (:993-1013) only for statements the verifier finds in spec/.
  - Keep, with anchors: "In brief, for the owner" (:42-125), §1-§6 (§6 holds ruling 40's 3f and the open survey caveat), §8, §9 "In v1", §10, §11 (H1-H10), §12, and Sources (the link definitions the kept sections use).
- docs-audit/README: keep the DA table. GAPS.md: add a note that ruling 41 settled GP3.
- instantiation: keep the known costs of closed input lists (:113-121).
- naming: keep "Versions across connections" and "Costs".
- schemas-and-interface: keep the I1 amendment.
- relationships: keep RD1-RD8.
- graphs-and-schemas: keep the SD rows and the standing instruction.
- Every record keeps its "Amended by" list and the owner's words, and may gain one line "Accepted by ruling 57" (E24).

**E21. simplification/README.md**
- **"Rejected, and why": [synthesis:423-487](../simplification/README.md#rejected-and-why) copied verbatim**, so the worker does not select. It covers:
  - B (apply in the engine and restart in place).
  - C (revise.v1, swap.v1) and D (apply.v1, operation groups), each with its "Kept" items.
  - **Design E's four rejected parts (:474-479).**
  - A (equal-document resubmission, ownerConduits on pool.v1 at 124 vectors, name reuse after retirement, template-input defaults).
  - Continuous reconciliation, automatic apply, prune by default, version ranges and aliases, the ACC-033 root exception, a default Pool at first start, a spec-level lock file, JSON-first authoring, and in-place upgrade in v0.
- The eight load-bearing guarantees and the incidental list ([map-mechanisms §1](../simplification/README.md#the-load-bearing-guarantees-and-incidental-costs)).
- v1-candidate reductions with their measured costs:
  - From [design-N2:864-900](../simplification/README.md#what-the-v1-candidate-reductions-lose): what each reduction loses.
  - From [inventory-build points 9-11](../simplification/README.md#measured-costs-of-core-reductions): removing declared Conduits invalidates 126 of 367 vectors; register-replacement.v1 also serves Pool entries and D09 rotation; replace.v1 is about 120 lines and 15 vectors; the three together invalidate 141 of 367; a form that is only short touches 252 of 367 plus 114 examples.
  - From [synthesis:417-421](../simplification/README.md#later-core-candidates): LIF-009 should build from the held definition.

**E22. short-form/synthesis.md: new closing section "Carried from the papers"**
- The text of [PRJ-011](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N3-project-document.md#L187), from [design-N3:187](../language-shape/short-form/synthesis.md#the-package-template-constraint).
- **All thirteen points of [short-form inventory-prior-art](../language-shape/short-form/synthesis.md#prior-art-key-points-and-five-conditions) and its "five conditions" (:45-56)**, including:
  - Never hash content into generated names.
  - Shorthands fail closed and write the full list of operations.
  - Identity fields are immutable.
  - The long form stays legal and no round-trip is required (the ALFA lesson).
  - Every rule needs A/B vectors.
  - Point 6 (stored defaults conflict with INS-013).
  - Point 13 (hazards: SUB-008, SUB-005, CON-016, OBJ-003, NAM-027, MAN-001).
- The 18 golden plan safety cases ([simplification synthesis:528-546](../language-shape/short-form/synthesis.md#golden-plan-safety-cases)), for WO-C.
- [simplification critic](../language-shape/short-form/synthesis.md#critic-constraints-on-turns-portability-costs-and-prior-art) gaps:
  - 11: conduit turn, --json, `as: <agent key>`.
  - 12: relative values, undecided since ruling 52 point 3.
  - 13: no Rust YAML editor that keeps comments; lock-file fallback.
  - 19: prior art.
- Head notes: tombstone option (a) is superseded by ruling 55 A7; decision 3 is open.
- critic.md stays unchanged until the batch.

**E23. docs/proposals/projects/brief.md (new)**
- **Provenance**, verbatim with source lines:
  - Each relayed quote of the owner's Projects words: https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-authority.md:165, [design-P2:187-189](../projects/brief.md#provenance-scope-design), [design-P1:121](../projects/brief.md#provenance-sixth-family-design) and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-use-cases.md:176.
  - A note that the papers disagree; owner-direction/2026-11-10-projects.md holds the later correction.
  - The statement, from https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/synthesis.md:91, :758, :820, :940 (risk 4) and https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project/inventory-use-cases.md:431, that the approvals-of-widenings direction "comes from the computed task text. It is not in the owner's relayed words".
- **Every [synthesis decision, 1-20](../projects/brief.md#earlier-decisions-1-to-21-and-their-alternatives), with its options and why each rejected one lost** (:715-765). This includes:
  - 2(b): Pool would mean two things, and retire-with-resources.v1 would retire Agents.
  - 2(c): every bound naming system would cover every Project.
  - 4: enclosure against a derived root Conduit. The owner's direction is 4(c); 4(b) flips 13-14 vector steps.
  - 6(b), 9(b), 12b, 14c, 15, 17, 18b, 19 and 20.
- ["Where it departs, and why" (:82-94)](../projects/brief.md#earlier-departures-and-reasons) and the [P1-P4 comparison (:696-711)](../projects/brief.md#p1-to-p4-comparison).
- Root-equivalence: [design-P4:702-713](../projects/brief.md#root-equivalence-and-home-granting-counterexamples), [synthesis fix 1 (:23-27) and risk 2](../projects/brief.md#root-equivalence-and-prerequisites).
- Prerequisites P-a to P-d ([synthesis:823-829](../projects/brief.md#root-equivalence-and-prerequisites)).
- The home-granting trace [H] ([P4:715-728](../projects/brief.md#root-equivalence-and-home-granting-counterexamples)).
- The A(x) single-authorizer routing ([P4:172-210](../projects/brief.md#single-authorizer-routing-ax)) and open questions 3, 5, 6, 7, 10, 11, 12, 13 and 15.
- [project/critic](../projects/brief.md#critic-constraints) gaps 1-6, 10-12, 16-19, 21, 22, 24, 25 and 27, as constraints.
- [Risks 12 and 14 (:962, :966)](../projects/brief.md#earlier-risks-isolation-provenance-store-break-and-deferred-use-cases).
- R1-R7, the R3 table and UC1-UC10 ([inventory-use-cases:38-379](../projects/brief.md#requirements-r1-to-r7-and-use-cases-uc1-to-uc10)).
- The 15 lessons with sources, and the pattern table ([project inventory-prior-art:5-20](../projects/brief.md#fifteen-prior-art-lessons-and-sources), :232-243).
- [inventory-ownership](../projects/brief.md#ownership-and-lifecycle-constraints) points 3, 5, 7, 9 and 10 (the open owner choice, also in E2), and the :256 gap.
- [inventory-authority](../projects/brief.md#system-pool-authority):
  - Point 15: a system Pool named in a bound is near-root.
  - B5: co-admins can close each other's Conduits through ACC-012.
  - B9 (:145): decoy names, which expectedId only partly fixes.
  - "Unverified" (:177-185), also in E2.
- [short-form inventory-scenarios](../projects/brief.md#worker-structure-and-upgrade-costs) points 10-12:
  - An Agent cannot be made together with its own Resource.
  - An admin-consumed grant cannot be declared inside a worker template (CON-002).
  - IN5 and IN2 force two templates.
  - An upgrade means ten sign-ins.

**E24. One-line notes (the only edits allowed to surviving text besides link targets)**
- "Superseded by ruling NN":
  - enrollment/README decision 5 and its short code: ruling 56.
  - verification-methods decision 5: ruling 56 #5.
  - networking-paradigms decision 5's "outcome", now "answer" (ruling 56 #8).
  - language-shape/README: "What would happen next" step 4 (ruling 55) and section 2.
- "Accepted by ruling 57 (point 2)", where a kept record still says its decisions can be struck: reaudit/round-5/DECISIONS.md:4, ruling-41/open-decisions.md:16, increment-2/decisions.md:6, ruling-43/README.md:104, core-and-schemas/README.md:17 and :444. Never on review items 15-19 or log-increment-4.
- experiments/009 README point 3: "observed under `--setting-sources project`, which finding 3-04 later showed loads an ancestor CLAUDE.md".

**E25. docs/archive/README.md**
- Rows, each with what it was and "source material for":
  - CONDUITS:795-833 and the evaluation cases: the history/messaging contract (L151-L154, L166, A10).
  - RESOURCES:148-250: A50.
  - ESTABLISHMENT:333-563 and :589-600: the directory and worktree types (L85, L86, L92, L96) and the fault cases.
  - PI_RUNTIME §§3, 4, 6, 7: the Agent contract (L77, L79, L81, L87, L94).
  - PI_CONNECTORS: "Authentication is an account contract" and "Session history".
  - OBSERVABILITY §3: an observation provider.
  - AUTHORIZATION and CLOUD_AUTHORITY: A66.
  - REPOSITORY §5/§6 and ECOSYSTEM's five conclusions.
  - The first-build logs: A44, G6, A79 and the register.
  - vocabulary/README, PRECEDENTS and SOURCES: the vocabulary decisions; kept as evidence by ruling 36 and amended by ruling 57.
  - Experiment 001 (de6566a).
  - After A27 is reconciled: the Pi pins 0441483 and 04025a6.
- FORMATS:53-70, as one paragraph:
  - YAML 1.2, JSON and JSON Schema 2020-12 were chosen because MCP and OpenAPI 3.2 build on 2020-12.
  - JSON alone was the runner-up.
  - CUE was the strongest alternative, to reconsider only if authoring exposes constraint or generation behavior the model cannot represent.
  - Dhall needs another evaluator; Smithy and WIT need a mapping; a new notation carries the largest burden.
- **"Not adopted from the collaborator's proposals, and why"**: REVIEW.md "What needs adaptation" and "Additional roadmap candidates", verbatim. It covers a local root Agent, a VM per environment, Sealmesh, a fixed JSON-RPC catalog, and every request naming its actor.
- **"Prior art still applied"**, one line each with a C0 permalink to the exact lines:
  - CONFIGURATION:115-145: no generic deep-merge of permission lists.
  - CONFIGURATION:220-238: labels and selectors not adopted.
  - CONSTRUCTION:82-95: Terraform's "removing a declaration is not permission to delete".
  - RUNTIME_ESTABLISHMENT:169-186: the failure cases to retain.
- Attribution: PR #1, #2 and #5 with their commits.

**E26. docs/archive/plan-audit-history.md**
- Keep:
  - The completed and superseded A rows.
  - Q1-Q12 and D1-D8.
  - E1-E11.
  - The six dropped behaviors, with the GP3 paragraph trimmed.
  - Deferrable scope (A18) and Reference repositories (A27).
- Multi-system options moves verbatim to E1, and links to it repoint to ruling-log.md.
- Drop the defect closure plan, research gaps and known inconsistencies.
- Repoint AUDIT:533, 543, 807 and 825, LANGUAGE_DEFECTS:24 and placement:302.

**E27. docs/LANGUAGE_DEFECTS.md**
- Keep in full:
  - The 30 open type-layer rows.
  - The 16 runtime rows "ruled ...; vector pending", for example L102-L106 and L155/L156, which AUDIT:111 cites.
- Every other row becomes a crosswalk line: L-ID | decision (ruled, DL, closed) | promise or DL item | one-line scenario copied from the row's first sentence. This covers the 74 ruled type-layer rows, the 29 implementation-defined rows (keeping the L-to-DL map, for example L53-L66 to DL4 and L32-L42 to DL6), and the 23 closed rows.
- Drop the sweep rounds only.
- Fix "None has a vector" against the manifest. Re-triaging the 30 is the coordinator's follow-up.

**E28. experiments 007, 008 and 009**
- Kept as the method as run, with links into src/ and raw files becoming C0 permalinks:
  - **008/HARNESS.md and 009/HARNESS.md.** They hold the stub as built (:60-84), its answers (:85-134), the flags actually used (008 :135-172; 009 :105-134), and the later-turn mechanics (:173-226).
  - **The evidence READMEs:** 008 evidence-2026-11-09, 008 later-turn, and 009 evidence-2026-11-09. They hold what was trimmed and sanitized (009 :47-58).
- 007: **evidence-2026-11-09/prompts.txt** (the exact check prompts) stays.
- 008 README: fix the stale "await admin review" lines (4, 142, 150).

**E29. experiments/README.md**
- "Evidence for v0" (A75) links:
  - Experiments 004 (A40), 005 (A22's dated observations), 007, 008, 009 and 010.
  - The first build's closeout-results, the increment-4 reports and the measurement plan.
- The raw-records rule.
- The 001 row moves to the archive.

**E30. docs/implementation/WORK_ORDERS.md "The record"; packages/agent-codex/README.md**
- One row each for 001-029:
  - Worker minutes and rounds.
  - Verifier minutes and must/should/nit counts.
  - What the admin changed.
  - Tests or vectors at merge.
  - Findings.
  - **The order's own decisions**, one line each in the order's own bolded sentence, with its reason where given.
  - **Any observation in its Record that no finding or package README carries, verbatim.**
  - State that admin time was not recorded.
- Required:
  - 028's decision 5 (:45-49), verbatim, including that vectors keep reading S1's snapshots as fixtures.
  - 027's Record :81-85, verbatim: with multi_agent, multi_agent_v2, multi_agent_v2_dynamic_tools, collaboration_modes, in_app_local_automation and agents.max_depth=0 disabled, Codex 0.167.0 still listed its six collaboration.* tools; `sleep_tool` controls `clock.sleep`; without `web_search="disabled"` Codex lists `web__run`.
- packages/agent-codex/README.md:94 gains that observation as a live correction, and the 4-06 follow-up is noted.
- Lessons:
  - 019 lost its scratch space.
  - 028 hit the output limit; the rule "never print a whole file" moves here.
  - 021 and 022 conflicted when run in parallel.
  - 024 misread its npm grant.
  - The admin's own decisions get verified (020 against DL13; 009's licence note).
  - Verifiers found must-level faults in most build orders and none in the results orders.
  - From 012: check each finding against the promise that decides it.
- A77's done-when becomes "each order's row".

**E31. INDEPENDENT_BUILDS.md; ruling-39/README.md**
- For each condition, the state now and what would meet it:
  - G8: Q3, Q7, D3, D5, D8 and the probes.
  - G9: DL12 and DL13 leave an implementation nothing engine-specific; the interface is rechecked against the protocol; the tests are written from spec/.
  - G10: the build base, and a time-boxed pilot.
- "State on 2026-11-06" moves **verbatim, row by row**, into ruling-39/README.md as "The assessment point 8 confirmed".

**E32. FIRST_BUILD.md**
- "What increment 3 needs" (:54-98) stays unchanged. closeout-results:5, increment-2/decisions.md:22 and experiments/009 README:8 measure against it.
- Add the owner's bar, quoted with its source (coordinator-run:28).

**E33. ROADMAP.md**
- Step 3's state shrinks to two lines.
- A77 and A71's states are corrected.
- A79 is cited.
- The short-form spike is registered with its pass conditions.

**E34. spec/ (point 7)**
- spec/README "History" becomes "Where the reasons are", one line each:
  - core-and-schemas.
  - round-5 and round-6 DECISIONS.
  - open-decisions and the planner's choices.
  - principles-review.
  - The secrets proposal.
  - vocabulary-audit.
  - multi-machine.
  - The first-build register.
  - ruling-log.md.
- In each YAML head, keep:
  - The sentences describing current content (for example implementation.yaml's layer exceptions).
  - Open issues.
  - "Standards followed" (providers.yaml, secrets.yaml).
  - **A removed-ID register**, "Removed: OLD → where it went (ruling NN)" or "OLD: nothing replaces it (ruling NN)". It covers every promise and DL ID that the C0 head names and that is no longer live (74 today; 72 of them appear nowhere else), and keeps "not reused" statements.
- Add one standing line. Drop the change narrative.
- Any dropped sentence that states a behavior no promise holds is reported as a spec gap, not deleted.

**E35. "Choices that set behavior" (new section in the README of each batch whose editor logs leave)**
- Sources:
  - reaudit log-group-a/b/c and log-followups-ab/c, round-5 log-fix-F1..F7, and round-6 log-specdocs and log-tracker go to reaudit/README.md (round 6 to round-6/README).
  - ruling-41 https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G1.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G2.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G3.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G4.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G5.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G6.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-G7.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-core.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-docs.md, https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/log-integration.md go to ruling-41/README.
  - The ruling-42 logs (about 107 choices) go to ruling-42/README.
  - The ruling-43 logs (about 63) go to ruling-43/README.
  - core-and-schemas and docs-audit logs go to their READMEs.
- What qualifies: every choice that names a rejected alternative, a candidate for later, or a reading of a promise the promise does not state. Each is one line: choice, alternative, why, log ID, copied verbatim where it fits.
- Example (https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-42/log-integration.md#L139-L150):
  - A stronger PRV-021, in which every provider stops a run that comes back (as kubelet and Nomad do), is left as a candidate for later research.
  - A PRV-015 reading was rejected because it would let a provider place an instance on the engine's machine.
- The worker reports a count per log; the verifier samples 20%.

**E36. conformance/s1-safety-cases.md (new)**
- https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md §3 (:515-594) verbatim:
  - The six E05 cases.
  - The 23 safety traces (bootstrap, S1-register-set, S1-duplicate, S1-archive-escape, S1-no-fetch, S1-target-type, S1-plain-filter, ..., S1-purge, S1-records, E08-broker, E11-replace, E12-name-reuse), each with its inputs, expected decision and requiring promises.
  - The adjacent open boundaries (:586-592).
- The manifest's header and conformance/README define `safety_rows` by this file. It is the list draft ruling 47's "every safety vector passes" and A68's done-when refer to.

**E37. Extractions for the later triggers (registered now, done in the PR that removes the source)**
- short-form/synthesis.md "WHY NOT THE OTHERS NOW" (:88-92) and the reasons for F1-F6 go to ruling-55's spec-batch record.
- enrollment/networking-paradigms.md decisions 6, 7, 9 and 10 (macaroons/UCAN, tailnet sharing, A2A/ANP/NANDA rejected or deferred) go to the merged enrollment record.
- The five remaining [project papers](../projects/brief.md#carried-from-the-earlier-project-papers) go to the Projects rulings' record, checked against brief.md.
- The increment-4 runs: their counted project files and scripts stay; reports are already kept.

## experiments
**Does experiments/ need refactoring?** Not of its code or layout. It needs removal of superseded material and a documentary tidy.
- The two harness crates (008, 009) are superseded by reference-model/crates/shell and packages/agent-*; work orders 021-023 reused their code.
- They also compile spec/names.yaml in through include_str!, which set_names.py does not reach, so they would go silently stale under PR 14's renames.
- Ruling 9 already allows "Old experiments are rewritten or removed as replacements land". No AGENTS.md rule protects experiments/.
- No renumbering and no moves. Each experiment keeps the layout experiments/NNN-name/{README.md, results-YYYY-MM-DD.md, HARNESS.md where a harness ran, evidence README where raw records were kept, code while still needed}.
- The finalizer's change: **the method as run stays with the results.** Removing code and raw dumps is safe only if the flags, stub behaviour, prompts and sanitization statements that interpret the results stay. Today's flags in packages/agent-* differ from the runs': 3-04 replaced `--setting-sources project`, 3-05 added thirteen disabled Codex features and project_doc_max_bytes=0, and 3-06 turned code_mode_host back on.

**Per experiment**
- **001:** already gone. Its index row moves to the archive table (last version de6566a). 002 and 003 are already there.
- **004 (Stateright; A40 and G5):**
  - Keep the README. Its tool verdict (bounded safety only, about 55x state growth with a third caller, weak liveness, UDP-only actors, version 0.31.0 unmaintained, 46 crates, TLA+/Quint/Kani not compared) feeds A40.
  - Keep the 463-line model until A40's model runs in reference-model's tests. Then remove the code with an archive row, and A40's record absorbs the README.
  - Listed in "Evidence for v0" under A40.
- **005 (keychain probe; A22, A63, A73):**
  - Keep the README, with anchors #safety and #what-it-means-for-a-macos-keychain-provider-package. Its citations of the secrets proposal's S5 and "[promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)" stay valid, because that file is condensed in place. If E18 drops "[Promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)", that one link becomes a C0 permalink.
  - Keep the probe code, because A22's daemon case is open.
  - Remove results.json and results-rerun.json (195 KB of raw ACLs and timings, all tabulated in README:129-262). Remove results.txt and results-rerun.txt, which are byte-identical. The README's 8 links become C0 links.
  - The next run writes dated output instead of overwriting.
  - Listed in "Evidence for v0" as A22's dated observations.
- **007 (A70 baseline; the bar for A68 and for the comparison draft ruling 47 asks for):**
  - Keep the README, results-2026-11-09.md (unedited apart from link targets; anchor #how-each-measure-was-counted), setup/ (the counted artifact), and **evidence-2026-11-09/prompts.txt**, the exact check prompts that results:69 only summarizes.
  - Remove the other 32 files of evidence-2026-11-09/. The results restate every cited message and count, and README:71 and results:146 already state what was trimmed.
  - Repoint 46 links to C0, except the prompts link.
- **008 (A71, real clients on the presented server):**
  - Keep:
    - The README, with its stale "await admin review" lines (4, 142, 150) fixed and anchors #the-stub, #limits-stated-before-the-run and the follow-up anchor kept.
    - **HARNESS.md** (about 3k words): the stub as built, its answers, the flags run, and the later-turn mechanics. Its links into src/ become C0.
    - results-2026-11-09.md and results-2026-11-09-later-turn.md.
    - **evidence-2026-11-09/README.md and evidence-2026-11-09-later-turn/README.md**: what was kept, trimmed and sanitized. Their links to raw files become C0.
  - Repoint the 313 and 49 result links to C0.
  - Remove src/, tests/, Cargo.*, rust-toolchain.toml, .gitignore, and the raw files of both evidence directories (819 + 37 files minus the two READMEs).
  - Optional and not recommended now: replacing the per-run quotes with a link.
- **009 (A4, native launch):**
  - Keep the README (with E24's note on point 3 and finding 3-04), **HARNESS.md** (the flags run, :105-134), results-2026-11-09.md, and **evidence-2026-11-09/README.md** (trimming and sanitization, :47-58).
  - Repoint 67 links to C0.
  - Remove src/, tests/, Cargo.*, rust-toolchain.toml, .gitignore and the raw files of evidence-2026-11-09/. Do it in the same change as 008, because 009 depends on 008's crate by path.
  - The provenance field in packages/agent-claude-code/tests/fixtures/stale-init.json becomes the C0 link.
  - Today's flags live in packages/agent-* and CONFORMANCE.md:169-175. The flags as run stay in HARNESS.md.
- **010 (A8, subscription terms):**
  - Keep.
  - Fix README:6, which cites the unmerged ruling 47 as in force.
  - Before v0, re-read the terms, OpenAI's whole.
- **The first build's real-engine runs** (increment-3 closeout-results, the two increment-4 reports and the measurement plan) are the evidence v0 most needs.
  - They stay under docs/proposals/first-build/. Moving them would break many links.
  - experiments/README.md indexes them under "Evidence for v0" (A75).
  - closeout-results gains "How to run the open checks" (E8), so the open real-client checks stay re-runnable.
  - The raw run directories (coordinator-run/ 17 files, fresh-author-run/ 57 files) stay until S1 is rerun on the simplified language. At that point the raw records go with C0 links, but **each run's counted project files and scripts stay** (the 332- and 646-line failures v0 must account for), as 007's setup/ does. The uncertainties in fresh-author report.json move into log-increment-4 now (E9).
- **experiments/README.md:**
  - Fix line 4 ("no experiment has run a Conduit engine") and line 28 (008 "awaits review").
  - Cut each "Current evidence" cell to state, date and link.
  - Add the v0 evidence index (004, 005, 007-010 and the first build's runs) and the rule: "results are self-contained; a harness's method and flags as run, and each evidence directory's README, stay with them; raw run records are committed with their results and leave the tree in the next condensation, the results linking their last version."
  - Update README.md:38 and docs/README:130 ("Rust evaluations and harnesses").
- **Still to register:** the short-form spike that ruling 55 requires (short-form critic gap 8). Register it in the ROADMAP with its pass conditions before it runs (E33). N4's scratch run was never committed and left out `as`. Its report uses results.schema.json, which is kept (G).
- **Local, not in git:** run `cargo clean` in 004 and 005. It frees 249 MB of target/ and keeps plain `grep -r` out of it.
- **Checks:**
  - check_roadmap rule 4 still holds: every remaining NNN directory is listed and linked.
  - Run `cargo test --release` in 004 and 005 only if they change. They do not.
  - check_docs verifies each C0 permalink and anchor.
- **Result:** 980 files become 38; about 194k words become about 46k; 3.05 MB becomes about 0.36 MB.

## sequence
**Step 0: admin, before any order**
1. Confirm work order 029 is merged (5956da9) and has its record. It edits docs/README, AGENTS, spec/README and docs/work-orders/README, and names 004's schema.
2. Put ruling 57 to the owner, as listed under rulings. Record docs/proposals/ruling-57/README.md in the canonical form, including the earlier rulings it changes. Add its AUDIT row, then merge.
3. Take **C0** = that merge commit, as a full SHA. Every "last version" link in this pass uses C0, as ruling 39 used e271d1c.
   - Freeze: no path on the removal list may change after C0 except in the order that removes it.
   - Each verifier runs `git diff --quiet C0 HEAD -- <removed paths>` before the deletion.
   - Every link written in this pass to a file on the removal list is written as the C0 permalink from the start, to avoid double edits.
4. Write each order with its own extraction list (E-numbers), removal list and the skeptic items it carries.
   - The verify.md standing order gains six checks:
     - Each extraction is present and faithful to its C0 source.
     - No surviving sentence is reworded, except link targets and E24's notes.
     - Every link into a removed path is either a C0 permalink, the new location of moved text, or a live destination approved by the admin.
     - No live instruction points at a frozen file.
     - For spec orders, every retired ID is still named.
     - Word counts before and after.
5. Local only: `cargo clean` in experiments/004 and 005.

**Step 1: WO-030, tools and code dependencies.** It removes nothing, so it may start before the ruling.
- check_schemas.py gains E15: the promise form, retired IDs not reused, and safety_rows names.
- check_docs.py verifies permalinks of the form `github.com/shrav-k/conduit/(blob|tree)/<sha>/<path>#anchor`:
  - The path is checked with `git cat-file -e sha:path`.
  - The anchor is checked against the headings of `git show sha:path`.
  - Outside a Git tree with history, it skips with a notice.
  - Today external URLs are skipped (check_docs.py:165), so a mistyped permalink would pass silently.
- A path-mention check: repo-relative paths named in tools/*.py, conformance/manifest.yaml, reference-model/ sources and (after WO-032) spec/ YAML comments must exist.
- E14 (the frozen S1 set; update lib.rs, manifest.yaml:6883 and conformance/README:399) and E36 (conformance/s1-safety-cases.md, with the manifest's safety_rows meaning pointed at it).
- Remove or retarget s1_drafts.rs.
- Checks: all six AGENTS checks, plus `cargo clippy --all-targets -- -D warnings`.

**Step 2: WO-031, rules and the read-first documents**
- AGENTS.md and CONTRIBUTING.md: the amended rules (points 1-7), E19 including "Writing promises", and the PR-history paragraph cut.
- docs/AUDIT.md condensed (E2). It keeps verbatim the convergence rule, the LD table, the role table and the labelling preamble, with stub headings for every linked anchor.
- docs/proposals/ruling-log.md (E1), including "Rulings before the plan audit", ruling 5's options and amendments 45-56.
- docs/guides/cold-readers.md (E3).
- ROADMAP state cells (E33) and A79:
  - Add A79's row under AUDIT P1 and cite it in the order of the work (check_roadmap rule 3).
  - Keep a "Done since ruling 39" table so that each A-item still has exactly one row across AUDIT, ROADMAP and plan-audit-history (rule 1).
  - Every state cell begins with an allowed word (rule 6).
- E24's "Accepted by ruling 57" lines.
- docs/README.md as a map listing only the records that remain.

**Step 3: WO-032, spec/ history (point 7a).** It touches spec/ only, so it may run in parallel with WO-031: merge one, rebase the other, rerun check_docs.
- E34, across the Markdown files and about 149 YAML head comments.
- Gates:
  - For every YAML file, `yaml.safe_load` before == after. JSON is untouched. Promise text is unchanged, compared by script.
  - **Removed-ID gate:** the IDs named in C0 head comments that are not live IDs ⊆ the IDs in the new removed-ID registers ∪ spec/README "Promise files that rulings removed". The same for DL IDs.
  - "Standards followed" sentences are present.
- Run check_schemas (now with the reuse check), set_names and `cargo test --release`. The shell and vectors read spec/core and two spec examples by path. If any digest or fixture changes, stop and report; do not regenerate.
- It must merge before any spec-batch order for rulings 49-56 starts.

**Steps 4 to 11: removal orders, one PR each, sequential** (the lesson of orders 021/022). Each order follows the same procedure:
- **(a) Extraction commit.**
- **(b) Deletion commit.**
- **(c) Repoint,** with a script in the worker's scratch space (not committed), in three classes:
  1. A link to text that this pass moved verbatim (for example Multi-system options, plan.md §3, the 2026-11-06 assessment) goes to its new location.
  2. **A link whose sentence says where future work goes** ("is logged in", "assigns the next", "supplies the sentinels") is listed for the admin and retargeted to its live destination, not to C0. Known cases:
     - conformance/README.md:117-118 → FIRST_BUILD#findings or the open increment's log.
     - reference-model/CONFORMANCE.md:63-67 → FIRST_BUILD's increments or the ROADMAP.
     - reference-model/examples/s1/README.md:44-46 → closeout-results#how-to-run-the-open-checks.
  3. Every other Markdown link that resolves to a removed path or a deleted section becomes `https://github.com/shrav-k/conduit/blob/<C0>/<path>#<anchor>` (`/tree/` for directories). A plain-path mention in a comment becomes a finding ID or the C0 permalink. This includes links from kept records, docs/archive and docs/research (allowed by point 1).
- **(d) Archive rows** in docs/archive/README.md: Path | What it was | Source material for | Last version.
- **(e) Checks:**
  - The six AGENTS checks; check_docs now verifies the permalinks and anchors.
  - check_roadmap rules 1, 3, 4 and 6.
  - `cargo test --release` whenever spec/, conformance/ or reference-model/ changed.
  - `git grep -n -F '<removed path>'` returns only C0 permalinks and archive rows.
  - A words-before-and-after report.

The orders:
- **4. WO-033, reaudit** (removal list A; E4, E5, and E35 for the reaudit and round-5/6 logs).
- **5. WO-034, ruling and batch records** (C; E16, E17, E18, E20, and E35 for the ruling 41/42/43, core-and-schemas and docs-audit logs).
  - The core-and-schemas spot-check and E18's SEC-draft mapping come first.
  - E31's verbatim move of the 2026-11-06 assessment into ruling-39/README is done here too.
- **6. WO-035, first build** (B; E6-E10, E12, E13).
  - Code dependencies and E36 are already done by WO-030.
  - E10 uses the corrected mapping, and item 17 quotes 028's decision 5.
- **7. WO-036, the language-shape and simplification papers that go now** (D; E9, E11, E21 with :423-487 verbatim, E22 with all of [short-form inventory-prior-art](../language-shape/short-form/synthesis.md#prior-art-key-points-and-five-conditions), E24).
- **8. WO-037, the Projects brief** (E23, with Provenance and decisions 1-20).
  - The admin checks the brief against the inventory's items C1-C8, the ruling conflicts (15, 30 OL2, 44 point 2, 49, 50), and each quote's source line.
  - A second commit removes [design-P1, P2, P3, inventory-authority and inventory-ownership](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project).
  - The five synthesis-level papers stay until the Projects rulings (later trigger).
- **9. WO-038, research and archive** (E; E25-E27).
  - Remove reviews/ only after confirming that E1's "Rulings before the plan audit" is merged.
  - Keep vocabulary/SIMPLIFICATION.md.
  - Remove check_doc_migration.py together with its CONTRIBUTING block and its docs/README row.
  - Never remove plan-audit-history.md (check_roadmap reads it). Keep its anchors #completed-and-superseded-action-items, #findings, #standing-probes-and-comparisons, #behaviors-not-carried-into-the-specification, #deferrable-scope and #reference-repositories.
- **10. WO-039, experiments** (F; E28, E29). Keep HARNESS.md files, the evidence READMEs and 007's prompts.txt.
- **11. WO-040, work orders and the implementation documents** (G; E30-E32).
  - Rename the four reusable schemas, and repoint verify.md and the open orders.
  - Add the Codex README correction.
  - Repoint the experiment, CONFORMANCE, first-build and critic links to orders.

**Step 12: WO-041, close**
- The final docs/README map and research/README.
- The archive table as one sorted table of about 45 rows, plus its two verbatim sections.
- Counts in AGENTS, spec/README and docs/README.
- A words report by area.

**Step 13: admin and a cold reader**
- A Codex reader with no context gets the condensed tree and the full extraction list (E1-E37), plus every skeptic item (decisions 1-10, rationale 1-12, evidence 1-10, links 1-12). It confirms each is findable at its destination, or that its file was kept.
- A grep audit of promise IDs, retired IDs and superseded terms.
- The admin reports to the owner and brings to-review-with-the-owner.md up to date.
- The coordinator separately closes or restates A27: removing the Pi-citing packets meets its second half only.

**Later triggers (each with its E37 extraction in the same PR)**
- After the S1 rerun: remove the increment-4 raw runs, keeping their counted files.
- After the Projects rulings: remove the five remaining project papers.
- After the spec batches for rulings 49-56:
  - Remove the short-form synthesis and critic (reasons to ruling 55's batch record).
  - Merge the enrollment files into one record (networking-paradigms decisions 6, 7, 9 and 10 carried).
  - Condense sign-in.
  - Rewrite OVERVIEW.
- After A40: remove 004's code.
- **Standing rule:** each round, increment or order is condensed in the PR that closes it. Logs become register rows plus "Choices that set behavior"; raw evidence goes to git, keeping the method as run; so the tree does not grow back.

## rulings
One ruling, **ruling 57**, answered point by point as ruling 39 was.
- Points 1-3 gate nearly everything. Point 7 gates WO-032. Points 4-6 and 8-10 gate their own areas.
- WO-030 needs no ruling.
- Its record lists the earlier rulings it changes:
  - Ruling 39, point 11 ("The rule stands for every other edit"), replaced by point 1, and point 6 (per-record notes), extended by E24.
  - Ruling 36, as far as it kept the vocabulary precedents and sources in the tree. The reviewer's SIMPLIFICATION.md stays as written.
  - Ruling 46, point 4 (first-build logs).
  - Ruling 48, as far as it set the work-order files as the record.
  - AUDIT's convention that ruling rows are not rewritten (AUDIT:39, :82-84).
- It changes neither ruling 38 point 4 nor ruling 39 point 15, nor rulings 13-14: the convergence rule, the labelling sentence and the LD table stay in AUDIT verbatim.

**1. The rule on "records of their time"**
- Where it is written: AGENTS.md:110-112; CONTRIBUTING.md:56 and :99; docs/README; the archive's "preserve attribution, source revisions and dissent".
- **Option (a): replace it with a removal rule.**
  - Any file under docs/proposals, docs/research, docs/archive, docs/work-orders or experiments may leave the tree, or be condensed by deleting sections, once everything relevant in it is in spec/ or a live document. "Relevant" means: a ruling and its reasons; what was put to the owner, with the options; an open item; the reason an alternative was rejected; v0 evidence and the method as run; or instructions.
  - A file that spec/ or a ruling record names as its reasoning is condensed in place, never removed. The secrets proposal is one.
  - Each removal gets an archive row pinned to its last commit, and links into it are repointed.
  - Surviving text is not reworded, except:
    - Link targets.
    - One-line "superseded by ruling NN" notes.
    - One-line "Accepted by ruling 57" notes.
    - A live instruction retargeted to where the work now goes.
  - Never removed: the owner's verbatim words, the decision tables of ruling records, and the option essays a record cites. Dissent in the archive is kept in the archive index.
- Option (b): a one-time leave for this pass, like ruling 39, with the rule unchanged afterwards.
- Option (c): keep the rule. Only proposals and experiments get condensed, which loses about 105k words of savings. Links from research and archive files then pin some proposal files in place.
- **Recommend (a)**, with the standing condense-at-close rule.

**2. Decisions the owner can still strike**
- Where they are logged:
  - Round 5's M01-M111 and its log-fixes choices.
  - Round 6's post-ruling details and editor choices.
  - The editor choices of rulings 41, 42 and 43, and ruling 43's verification.
  - Ruling 45's log-roadmap.
  - First-build fixes 1-01 to 3-07.
  - Work-order decisions 001-029.
- **Option (a): accept, as ruling 39 did, every coordinator decision, editor choice and admin order decision logged before ruling 57, with two exceptions.**
  - The exceptions: items 15-19 of to-review-with-the-owner.md, and the decisions of the open increment 4 (log-increment-4). Both stay strikeable until the joint review.
  - The decisions that changed an earlier ruling become recorded amendments in ruling-log.md: M13, M22, M41, M47, M59, M66, M69, M107 and the PLC-023/LIF-036 choice.
  - Each choice that names a rejected alternative or a later candidate is carried into its batch's "Choices that set behavior" (E35) before its log goes, so the reasons stay in the tree.
  - Kept records that say their decisions can be struck gain "Accepted by ruling 57".
- Option (b): keep them strikeable and carry each one, about 300 items, into the review list before its log goes.
- Option (c): keep the logs, which forfeits about 150k words.
- **Recommend (a).** It matches the owner's acceptance "for now" on 2026-11-09 and his confirmation of items 1-14.

**3. The ruling log and the form of ruling records**
- **Option (a):**
  - AUDIT keeps the labelling preamble and one line per ruling (number, date, the gist in 25 words or fewer, record, amended by).
  - These move verbatim to docs/proposals/ruling-log.md:
    - Rows 1-37.
    - Ruling 5's options.
    - The pre-audit rulings (M1-M3, I1-I7, C1-C3, the license decisions and the later-scope list).
    - The later amendments through ruling 56.
    - The ID prefixes.
  - The long text of rows 38-56 goes to git, since each record holds the owner's words (checked by script).
  - This lifts AUDIT's "rows are not rewritten".
  - The canonical form of a ruling record:
    - A status line.
    - What was put, with the options and why the others were not recommended, or a link to where that is kept.
    - The owner's answer, verbatim.
    - A points table (recommendation, answer).
    - The earlier rulings it changes.
    - "After the ruling".
    - For a batch, "Choices that set behavior".
    - No work orders, edit lists or sibling logs.
  - Older batch records (rulings 7-38) are condensed by deleting sections only, at the same paths.
- Option (b): move all 56 long rows verbatim, which adds 3.4k words of duplication.
- Option (c): leave the log in AUDIT, which keeps the read-first path about 13k words longer.
- **Recommend (a).**

**4. First-build logs (amends ruling 46, point 4, and AGENTS "Where things go")**
- **Option (a):** a closed increment's log becomes:
  - Rows of the settlements register in first-build/README.md (finding, decision, promises, vector, rejected alternative and why).
  - Its method and measurements in conformance/README "Planned work" or closeout-results.
  - Its safety cases in conformance.
  - The open increment keeps its log until it closes.
- Option (b): keep logs 1-3.
- **Recommend (a).**

**5. Work orders (amends ruling 48's record; A77)**
- **Option (a):**
  - A merged order's file leaves the tree once its row is in WORK_ORDERS.md "The record". The row carries the order's own decisions and any Record observation not carried elsewhere.
  - A77's done-when reads "each order's row". Admin time was never recorded, and the row says so.
  - The four reusable report schemas stay under purpose names.
- Option (b): keep the orders.
- **Recommend (a).**

**6. Raw evidence and v0 (rulings 9 and 45; draft ruling 47 in PR 14)**
- **Option (a):**
  - Dated results files are the v0 evidence. With them stay the method as run (HARNESS.md, the flags, the prompts), each evidence directory's README, and the counted artifacts, including those of failed runs.
  - Raw run records are committed with them, then leave the tree with links to their last version.
  - The first build's increment-4 raw runs stay until the S1 rerun.
- Option (b): keep raw records in the tree until v0 is published.
- **Recommend (a).** Confirm that draft ruling 47's "reported with their dated observations" and "account for missing runs and failures" are read this way.

**7. History inside spec/ (comments only)**
- The history is about 47k words in 149 YAML head comments, with 834 paths into the proposals, plus about 6.6k words of history paragraphs in spec/README, core/README, schemas/README, definitions and schema-profile. Two of those files are read first.
- **Option (a):** condense each to what the file holds, its standing, reading notes, the standards it follows, open issues, **its removed-ID register** (spec/README:290 requires one; 72 retired IDs are named nowhere else), and one pointer to "Where the reasons are".
  - Parsed data must be identical, checked by script.
  - No promise text changes.
  - Every retired ID stays named, checked by script.
  - Any dropped sentence stating a behavior that no promise holds is reported as a gap.
  - New rule: spec files carry no change narrative.
- Option (b): leave the history, and add one line saying paths to removed records resolve at C0. The paths go stale silently, and about 54k words of history stay in the binding text.
- Option (c): unchanged.
- **Recommend (a).**

**8. INDEPENDENT_BUILDS' assessment of 2026-11-06**
- The owner confirmed it under ruling 39, point 8. It now says reference-model/ and conformance/ do not exist.
- **Option (a):** replace it with "state now and what would meet it", one line per condition, marked as the coordinator's until G11. The confirmed assessment moves verbatim, row by row, into ruling 39's record.
- Option (b): keep it, with a stale note.
- **Recommend (a).**

**9. Inputs to the Projects rulings**
- **Option (a), revised:**
  - Write the Projects brief now, as the first step of the investigation. It opens with Provenance: the owner's relayed words with their sources, the note that the papers disagree, and that the approvals-of-widenings direction is not his.
  - The owner is asked to confirm his Projects direction in his own words.
  - Remove [design-P1, P2, P3 and the two authority/ownership inventories](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project) after the admin checks the brief.
  - Remove the five synthesis-level papers ([project synthesis, critic, design-P4, use cases, prior art](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project)) in the PR that records the Projects rulings.
- Option (b): keep all ten papers until the rulings.
- **Recommend (a).**

**10. What the index of removed files keeps**
- **Option (a):**
  - One row per removed directory or group of sibling files (about 45 rows), with the columns Path | What it was | Source material for | Last version. "Source material for" names the A-items, G-conditions, L-defects and type contracts it feeds.
  - Two verbatim sections: "Not adopted from the collaborator's proposals, and why" and "Prior art still applied".
  - One pinned commit (C0) per pass.
- Option (b): one row per file (about 400 rows).
- Option (c): one line, "everything removed on <date> is at C0". This loses the pointers to source material, the attribution and the dissent.
- **Recommend (a).**

**Coordinator's calls, not the owner's**
- The choice of C0.
- Closing or restating A27, whose second half the removals meet.
- Whether A11, A16, A25 and A35 are closable: A4 is done, and "vector pending" may be stale. Condensing must not change those statuses itself.
- Re-triaging the 30 type-layer defects, separately.
- Which links count as live instructions to retarget (step c, class 2). The admin approves the list in each order.

## savings
The figures are from the auditors' word counts, checked against `wc -w` by area, and revised for what the finalizer keeps:
- The option essays of open-decisions: 3.0k.
- The secrets proposal, condensed in place: about 29k of its 39.4k.
- Five project papers until the Projects rulings: 43.3k.
- The reaudit follow-up pass: 0.6k.
- The multi-machine sections: about 1k.
- "Choices that set behavior": about 8k added.
- The Projects brief, grown to about 6k.
- ruling-log.md, grown to about 11k.
- AUDIT, grown to about 7.3k.
- FIRST_BUILD's needs: 0.5k.
- HARNESS.md files, evidence READMEs and prompts.txt: 7.8k.
- The removed-ID registers: about 4k.
- LANGUAGE_DEFECTS crosswalk and kept rows: about 2.5k.
- The research and archive notes: about 1k.

**By area**

| Area | Before | Leaves the tree | After |
|---|---|---|---|
| reaudit | 255k | about 219k | about 36k |
| first-build | 68k | about 44k now (8.8k more after the S1 rerun) | about 24k (about 15k later) |
| Ruling and batch records (rulings 38-56, older batches, principles-review, secrets, multi-machine, docs-audit, core-and-schemas) | about 286k | about 173k | about 113k |
| language-shape, simplification, enrollment, sign-in, owner-direction | about 312k | about 239k now | about 73k now; about 27k after the Projects rulings; about 18k after the spec batches |
| **docs/proposals, all** | **918k** | **about 675k (74%)** | **about 246k now; about 190k after the later triggers** |
| docs/research, docs/archive, LANGUAGE_DEFECTS, OVERVIEW | about 165k | about 108k | about 57k |
| AUDIT, ROADMAP, docs/README, implementation/, work-orders | about 68k | about 36k net | about 32k, including ruling-log.md (about 11k) |
| experiments | 194k (980 files, 3.05 MB) | about 148k (942 files, about 2.7 MB) | about 46k (38 files, about 0.36 MB) |
| **Historical and tracking text** | **about 1,345k** | **about 967k (72%)** | **about 381k now; about 325k after the later triggers** |
| spec/ history (point 7) | about 54k of spec's 219k | about 41k | spec about 178k; promises and data unchanged; every retired ID still named |
| **Total now** | | **about 1.0M words** | |

Conformance (about 414k words, 12 MB) is not cut here. It gains about 2.5k (s1-safety-cases.md, s1-promises.yaml, "Planned work"). A79 tracks the lean-vector plan for it.

**Read-first size**

AGENTS.md (loaded into every session through CLAUDE.md), spec/README.md, spec/definitions.md, docs/AUDIT.md and ROADMAP.md:

| File | Before | After |
|---|---|---|
| AGENTS.md | 2.1k | about 1.6k |
| spec/README.md | 8.3k | about 6.8k |
| spec/definitions.md | 26.9k | about 25.8k |
| docs/AUDIT.md | 23.6k | about 7.3k |
| ROADMAP.md | 5.2k | about 4.3k |
| **Total** | **about 66k** | **about 46k (−31%)** |

- Leaving aside the binding glossary, the rest falls from 39k to about 20k (−49%).
- Without point 7 the total is about 48.5k.
- AUDIT is about 1.7k larger than in the first plan, because it keeps verbatim the convergence rule (ruling 38 point 4), the LD1-LD5 table (rulings 13 and 14), the role table and the labelling preamble, and Known to be open grows to about 900 words.

**Search noise**
- Promise IDs:
  - ACC-033: 33 files in reaudit become 4.
  - PRV-007: 121 mentions in 23 log files become none.
  - 380 promise-ID mentions in the work orders and about 10,700 in reaudit mostly go.
  - 834 proposal paths in spec/ YAML comments go. Retired IDs stay, in one register line per file.
- Raw evidence today accounts for 829 of the 902 lines matching files.read.v1, and 177 of the 260 matching CONDUIT_IDENTITY_DIR. Nearly all go.
- Superseded "accepted" text leaves the tree: policies, graphs, runtimes, composite Conduits, location facts, and the layered package's short names. What remains of it in the secrets proposal and the kept option essays is reasoning the spec or a ruling record names, not live direction.
- Stale statements stop competing with current state. Examples: "conformance/ does not exist", "index stops at 019", "rulings 1 to 46, 48 and 49", and "the real-client check is pending".

**What the finalizer's changes cost:** about 105k words fewer leave the tree than in the first plan. About 43k of those leave later, with the Projects rulings. In exchange:
- No owner ruling, option put to the owner, rejected alternative, open gap, method as run, safety definition or retired ID is left only in git.
