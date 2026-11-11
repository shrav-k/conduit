Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

severity major

The overall shape holds: a normative notation, defined by translation into the unchanged kernel, is a defensible answer. The recommendation is not ready for the owner yet. The comparison credits the fixes F1-F6 to N4 alone. One of the four "buys" is wrong. Three ruling conflicts are labelled "confirm" when they are amendments. F3 changes 4 existing vectors that were not counted. The tombstone design changes how access is decided, and that was not analysed. The pass on the bar depends on writing each grant as one 200-400 character line, and that sensitivity is not reported. The S1 arithmetic is correct.

All paths below are under <home>

RECOUNT (verified)
- Baseline: 33+16+0+8+3+2+2 = 64. Source: experiments/007-s1-baseline/results-2026-11-09.md. The better T5 baseline is 2.
- Recommended S1 file:
  - T1: 19 file lines plus 1 install line (three archives in one command) = 20.
  - T2: 4 Resource lines plus 2 grant lines = 6, plus 1 install line = 7.
  - T4 = 2, T5 = 1, T6 = 1.
  - T7: 1 line, removed and then restored, counts once under the audit rule. Adding the --reopen command gives 2.
  - Total 33, and every column's sum checks.
- Other counts confirmed: 24 documents (12 templates, 12 instances); 455 promises (440 plus 15 POOL); 367 case.yaml files.
- The bar arithmetic and the failure under the strict reading are confirmed.

Small errors in the counts:
- The "5 extra lines" over the layered package are 2 operator lines, 2 owner lines, and +1 from the 3-line Pool block replacing the layered file's 2 lines (`conduit:` and `project:`).
- T5 wins by 2 lines against A70 and by 1 against the better baseline. It is not "1 line each".
- The "72 lines" said to be prevented by [SCT-010](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md#L140) include the turn Conduits, which authors still write. The Pool part was 35 lines (coordinator run, item 4).
- The Bar row compares columns on different bases. Every other column keeps T7 at 1 (no --reopen). Applied uniformly, F6's --reopen rule makes every column "better on 5, same on T3 and T7".
- Two sensitivities are omitted:
  - Three separate installs, as both real authors ran them, add 2 lines to T1 (critic gap 18).
  - Block style (MAJOR, see gap 1 below).

GAPS, MOST SEVERE FIRST

1. MAJOR: the bar depends on writing each grant as one long line.
   - Recount of the S1 file in other styles:
     - Block maps with operations as flow lists: T1 32 (baseline 33), T2 13, T7 5 (baseline 2).
     - Fully idiomatic block YAML: T1 about 38, T2 about 20, T7 about 8. The bar fails.
   - The fully qualified operator and grant lines (200-400 characters) carry the margin. The layered form is less exposed.
   - The first synthesis reported this sensitivity; the recommendation drops it.
   - Do:
     - Report a sensitivity table.
     - Have `conduit init`'s skeleton and `render` use flow style.
     - Fix the style counting rule before the rerun (ruling 45).

2. MAJOR: the comparison is biased against the layered package.
   - F1 (break-before-make under the key's own name), F2, F3, F4 and F6 all apply equally to the layered route.
   - F1 alone removes the layered column's "cannot follow renamed successors" in the S2-S10 row.
   - The safety row gives "every major item fixed" to N4 only.
   - Do:
     - Compare layered with F1-F6 against normative with F1-F6.
     - The real differences then reduce to: spec and vector cost, A34, binding translation vectors for thesis 2, and where the format reference lives.
     - Derived defaults (decision 3) are a separate choice that applies to both routes.

3. MAJOR: "No CLI mirror of about 27 promises; later core changes cost about 1.0-1.1x" is wrong.
   - The 27 promises in https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/design-E-layered.md, line 748, are mostly the planner's: ACC-022, ACC-024, ACC-033, LIF-007, LIF-017, LIF-027, LIF-035, LIF-037, OBJ-011, INS-020, CON-031, NAM-017, NAM-023, REC-002, PKG-005 and PKG-006.
   - Change stays tooling (decision 10), so the planner still mirrors them. F6 adds more: OBJ-010 cascades, control pointers and ACC-024.
   - Only the compile-side promises move into spec/: INS-025, NAM-027, IFC-012, IFC-014, SUB-003, SUB-005 and OBJ-018.
   - sectioned.yaml also restates every family's keys, which is a new mirror (the recommendation's own risk 5).
   - Do: re-estimate the multiplier (plausibly about 1.15-1.3x) or drop the claim.

4. MAJOR: rulings are labelled "confirm" where they need amending.
   - OBJ-006: "a reference that gives only a name in a field that accepts more than one kind is rejected".
   - Ruling 19 / IN10: the name alone only where the field accepts exactly one kind.
   - definitions.md, line 99: `consumer` and `target` are written with the kind.
   - The REFERENCES rule does exactly what these forbid.
   - Ruling 18 / V2 / IN8: a document with no `from` is a template. An inline definition has no `from` and makes an instance.
   - Ruling 8 / IN3: "one instantiation shape everywhere".
   - Ruling 52: the bare `to: shared` drops the kind of a connection's target, while the recommendation cites ruling 52 to justify its 5 extra lines.
   - Do: list rulings 8, 18 and 19 as amendments for the owner, or keep `to: {kind: Resource, name: shared}`. That costs 0 lines in flow style; it only widens lines.

5. MEDIUM-MAJOR: tombstones change how access is decided, and this is not analysed.
   - Under F2, a mark keyed by (consumer, target, capability), which only root or the actor that made it may supersede, becomes a deny layer above bounds.
   - S2's E07: an Agent within its bound opens an unfiltered Conduit, which ACC-030 allows. This would be refused if root had once closed any Conduit with the same (consumer, target, capability).
   - ACC-001's "decided anew for each request" gains history.
   - Option (a), root-only supersession, conflicts with preservation test 5: "every management action is available to an agent under a grant".
   - Option (c), a supersede operation of `access` that a bound can withhold, fits test 5.
   - Do:
     - Analyse the marks against ACC-029, ACC-030, E07 and test 5.
     - Put (a) against (c) to the owner on that basis.
     - Add vectors: E07 with a mark present, and an Agent's own supersession.

6. MEDIUM: F3 changes at least 4 existing vectors, and the count misses them.
   - conformance/conventions/access/collision-secret, collision-control-template, collision-control-type and collision-missing-contract each register a declared Conduit `c` and a standalone Conduit `c` for one consumer and expect acceptance.
   - IFC-014's observable deliberately allows this. F3 makes each a rejection.
   - "0-7 changed, by tombstones" becomes 4-11.
   - Do: say that F3 reverses a deliberate rule, and recount the vectors.

7. MEDIUM: thesis 2 is overclaimed.
   - "The store equals the translation after every completed apply, with three exceptions" leaves out:
     - generated names of nested instances (OBJ-003, DL15), with new names after each register-replacement (LIF-009);
     - root-consumed Conduits that cannot be narrowed (ACC-024), so a narrowed file is unreachable (the recommendation's own risk 7);
     - templates of deleted lines, unless pruned;
     - objects held retired while their shutdown is unknown;
     - the marks themselves.
   - Under decision 9(a), the engine accepts the form only when handed a directory: register.v1 takes `document.v1` items only (spec/core/operations.yaml, lines 253-260).
   - Agents, which by the purpose "operate the system through the same contract a human uses", must still author canonical documents.
   - Do:
     - List every exception.
     - State plainly in the ANSWER that the wire and Agents stay canonical.
     - Weigh adding 9(b) to the batch (about 1-2 promises).

8. MEDIUM: the N4 run evidence is overstated.
   - Today's schema rejects `as` (instances.yaml sets additionalProperties false), and `claude-code.v2` and `codex.v2` do not exist (packages/*/contracts.yaml).
   - So the 24 documents that "today's loader and core accepted" were not the ones shown.
   - Only registration ran. The new part, break-before-make with marks (T4-T7), is untested.
   - The N1-N4 working papers and the skeptics' scans are not in the repository.
   - Do:
     - Record the run under experiments/ with dated results.
     - Land the papers beside research/.
     - Register the spike in the ROADMAP register, with pass conditions stated before it runs (AGENTS.md, ruling 45).

9. MEDIUM: the plan misassigns 4-12.
   - "Order 028: 4-12 to 4-16" is wrong. docs/work-orders/028-i4-03-commands.md, line 20, covers 4-13 to 4-16.
   - 4-12, the turn deadline, is open in log-increment-4.md. It drove the fresh author's 112 T5 lines and 128 T7 lines.
   - Do: decide it, or give it its own order, before the rerun.

10. MEDIUM: the rerun may not test the form.
    - Both notations stay legal, and spec/ is dominated by framed examples. The last fresh author read spec/ and wrote canonical JSON.
    - Critic gap 3's discovery half is not answered: how an author finds Types, operations and config keys.
    - The bar is "work", but only lines are projected. Not projected:
      - requests: T4 about 8; T5 and T6 about 10 each under break-before-make;
      - windows without access;
      - concepts an author meets;
      - duplicated values;
      - lines naming a contract address.
    - Do:
      - State in the experiment plan that the documented form is the sectioned YAML.
      - Put the authoring reading path first.
      - Add `conduit describe`, or have install print the Types.
      - Project the other measures.

11. MEDIUM: change under failure is unspecified.
    - Break-before-make widens the damage of an apply that fails partway (E13's "restoring of access failing partway", S4): both Agents stay without access until the next apply.
    - F6 has no resume, no idempotency keys (OPS-008) and no apply lock (critic gap 15).
    - In S3 (E15), a name is held while a shutdown is unknown. That is the partition case, so reuse needs declare-lost.v1 (root only) or unregister: a recovery step that needs a person.
    - Do: add resume, keys and a lock to F6 and WO-C, with golden cases for E13 and E15.

12. MEDIUM: a staged alternative is missing.
    - Build N4's notation now as the CLI's form with golden cases, rerun, then promote it to spec/. The promotion costs about the same 24-26 touches.
    - The cost of deferring N2-style reductions past the S2-S9 fan-out and the independent builds is not given. That cost is the "critical moment" argument.
    - Do: cost both, and give a decision deadline (before step 5 and build-base).

13. LOW-MEDIUM: rulings 47, 49 and 52 are misstated.
    - Ruling 47 is readable (`gh pr view 14`): a draft v0 publication gate. It requires:
      - S1-S10 on the reference engine with their vectors;
      - S1 and S2 with real Agents;
      - the author bar, readable by either counting set;
      - an outside package-author path in any language (S7), which the templates-only sectioned rule touches.
      - Under ruling 35, a namespace move rewrites every fully qualified name in authored files.
    - Ruling 49 is accepted and pending in spec/ (spec/README.md, lines 19-22), not unwritten. Its declared account needs a place in the grammar.
    - Ruling 52's two-tier refusals (4-08) share disclosure rules with the inventory and marks. Write them in the same batch, so that they are "written once".

14. LOW-MEDIUM: F4's reasoning is wrong, though the consequence stands.
    - claude-code.v1 and codex.v1 already list the reach flags under `required` with no default.
    - Today's NAM-024 ("does not ... change anything the earlier one held") already forces `.v2`; F4 is about added properties.
    - Under F4, 4-02's sign-in choice ("subscription by default") would need a `.v3` unless it is folded into `.v2` in WO-D.

15. LOW-MEDIUM: F6's "same apply" rule for --reopen costs T7 its margin.
    - "This working copy's own recorded apply" would keep T7 at 1 line, while marks made by others still need the flag.
    - State the choice and its line cost.

16. LOW-MEDIUM: F2 is not fully costed.
    - Supersession needs inputs on open, register and register-replacement.
    - Purge needs to extend purge.v1 and REC-002.
    - CON-008 needs a cross-note, and is not in the list.
    - The shell store must persist marks; check that this is additive ("no store break").

17. LOW: preservation test 8 ("no rule assumes exactly one owner").
    - `operator` and [SCT-010](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/short-form/design-N4-minimal-normative.md#L140) hard-wire root into the form. Give a v1 path, such as naming an Interface.

18. LOW: vocabulary.
    - The grammar itself writes GRANT and "grant", which definitions.md, line 369, forbids for access.
    - "Definition" (definitions.md, line 333) excludes configuration, so "sectioned definition" collides.
    - "Document" (line 64) requires `kind`.
    - Ruling 29: in normative text, write "recreate", not "reopen".
    - Fix all of these before any spec text.

19. LOW: points to specify.
    - Whether `description` and `annotations` go on the `--t` template, the instance, or both. Ruling 53's tool descriptions depend on it.
    - Whether framed documents may reference generated names. If yes, the naming rule becomes API.
    - "Other projects can name `shared` stably" holds only for new documents. Existing holders keep the old ID (OBJ-010).

What checks out:
- All five accepted additions are placed: `as` in OBJ-013, IFC-012 and IFC-014; expectedId in LIF-037; the inventory in LIF-013, inspect.v1 and ACC-033; tombstones in LIF-038 with cross-notes; descriptions in IFC-033 and OBJ-018.
- Ruling 25 is respected: operation and Type names are fully qualified.
- Rulings 27, 40 and 48 are respected, and so is ruling 4's scope guard, since canonical documents stay legal.
- The scan of examples and vectors finds no map without `kind` that holds a section key.
- The code estimate is plausible against the code. The loader is 2.9k lines, so the translator adds 25-40%; the core is 10.6k lines and the shell 7.6k.