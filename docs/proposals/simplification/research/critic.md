Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the simplification investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

severity major

Holds with fixes. I recounted the rewrite and got the same numbers: T1 15, T2 5, T4 2, T5 0, T6 1, T7 1 file lines, 20 lines in the final file. The savings arithmetic also reconciles with the coordinator's files (T1 152+2, T2 73, T5 62, T6 29, T7 12). So under the plan's own reading the paper pass is internally consistent. But the package leaves several gaps that matter. The worst is the confirmation design. Apply asks y/N, and `--yes` may never accept a widening. The rerun's fresh author is a non-interactive Codex session, so it cannot answer y/N, and T1, T2, T4 and T7's restore are all widenings. Any explicit approval that works without a terminal (a plan ID or digest) is a copied value, which ruling 52 point 6 counts as a line. Counted that way, T5 and T6 tie and T7 is one line more, and the package fails "most" against the better T5 baseline. Three more gaps are serious. (1) The revocation guard is keyed by the consumer, so replacing or recreating an Agent silently reopens grants that were revoked, and Agents bypass the advisory tombstones entirely (preservation test 5). (2) The rerun protocol under ruling 48 point 4 never says where the fresh author learns this format that is not in spec/, and the claim of about 8 concepts is not credible. (3) Writing short operation names contradicts ruling 25 ("fully qualified, with no aliases") and saves 0 lines. Medium gaps: the planner ignores controls, the language's existing route for changing a setting in place (S5); grants may target only Resources, and the escape hatch (framed documents placed beside the file) cannot follow names that change at each replacement; T5 gains a window without access that the coordinator's run did not have; and counting the Conduits the caller may not see discloses that they exist (LIF-037). None of these is fatal to the shape (layered authoring on canonical documents), but each needs a fix or an explicit owner decision before the package goes to the owner.

WHAT I VERIFIED
- Recount: running count.py on the files in synth/ gives T1 15, T2 5, T4 2, T5 0, T6 1, T7 1 (the same line removed, then restored), final file 20. The coordinator's files reconcile with its totals: T1 project/ 152 + t1fix conduit.yaml 2 = 154; T2 8+47+16+2 = 73; T5 5+57 = 62; T6 5+24 = 29; T7 5+7 = 12. The savings per item add up exactly (154-70-56-11-2 = 15; 73-41-19-8 = 5). The counts are credible under the stated reading.
- The engine checks the package relies on were confirmed:
  - ACC-024 refuses any close of a Conduit whose consumer is root, so "add-only" is correct.
  - LIF-037 does not cover open or close, so 5b is a real addition.
  - LIF-035 removes a Conduit, and frees its name, the moment it is closed.
  - SCH-009 fills a Type's defaults into config, so item 6 works mechanically.
  - install.v1 requires a digest, so trust on first use means the CLI must hash the archive itself.
  - The history Type's description already carries the posting convention, so 5e has something to surface.
  - `local_name` exists in reference-model/crates/core/src/access.rs.

GAPS, MOST SEVERE FIRST

1. MAJOR: confirmation does not work for the rerun's author and can flip the bar.
   - Apply asks y/N, and `--yes` "never accepts a widening". The fresh author (Codex, ruling 48 pt 4) runs commands without a terminal. T1, T2, T4 (A gains write) and T7's restore are all widenings, so it cannot finish them.
   - Piping `y` into apply would make the `--yes` rule meaningless. The honest non-interactive fix is a saved plan whose ID the author names to apply (Terraform's saved plan; B's "names the plan digest"). That ID is copied from a tool's output, which ruling 52 pt 6 counts as a line.
   - Recount with one value-bearing apply per change: T1 17, T2 7, T3 0, T4 3, T5 2, T6 2, T7 3. That is better on 4 of 7 against A70 (narrow), and 3 of 7 against the better T5 baseline (2), which FAILS "most". This is the same failure as the strict reading.
   - What to do: choose the approval mechanism now. Fix its counting rule under decision 11 before the rerun (ruling 45). Make the approval bound to the plan: any drift after the plan aborts it, not only narrowing drift. Or get the stage-2 one-step commands ruled in.

2. MAJOR (safety): the revocation guard is bypassed when the consumer is replaced, and is only advisory against Agents.
   - Tombstones are keyed by (consumer, local name) and kept by the engine.
   - Several routes create a new Agent instance, under a generated name or a new ID: `--replace`, `--recreate`, or recovery from a poisoned conversation, which is what the fresh author did at T7. None of them matches the old tombstone. A grant an operator revoked with `conduit close` reopens on the next apply without any flag.
   - Agents operate the system through canonical requests (preservation test 5), so they never see the CLI's check.
   - What to do:
     - Key the guard by (project, agent key, grant key), carried through annotations, and make every replacement plan list "reopens X, closed by Y at T".
     - Have open.v1's result tell the caller about a tombstone (ruling 52's explicitness).
     - Put option (ii), enforcement through register or open, to the owner as the multi-actor answer, rather than calling (i) sufficient.
     - Also decide retention and purge for tombstones (ruling 30 lets only root purge records) and whether they are exported.

3. MAJOR: the rerun protocol, discovery, and concepts.
   - Ruling 48 pt 4 gives the fresh author "only the specification and S1's tasks". The project file is not in spec/, and the last fresh author read spec/ and wrote canonical JSON.
   - Nothing states where the format is documented, or how an author finds Type addresses, export names and config keys such as `root`. That is the fresh author's "discovery" difficulty, and item 7 covers only `--help`.
   - "About 8 concepts" is not credible. The measurement plan counts engine commands and flags as concepts (init, install, apply, reach, render, close, --replace/--prune/--take/--reopen/--recreate/--yes). It also counts the spec terms the plan shows and that the author must understand to confirm it: Pool, Conduit, root Interface, template, capability, operation, Type, package, digest, expectedId, tombstone. A fair count is about 15-20, against 16 today and the baseline's 20.
   - What to do:
     - Write the authoring page into the engine page the fresh author is given, and state in decision 11 what that author receives.
     - Have `conduit install` print each Type, its capabilities, operations and config schema, or add `conduit describe`.
     - Have `conduit init` write a commented YAML skeleton. That also steers authors away from JSON, the reading that fails.
     - State which run decides the bar. A70 was written by an author who knew the tools, so the fair comparison is coordinator against A70, and the fresh run measures how easy the format is to learn.

4. MEDIUM: ruling 25 is contradicted for nothing.
   - Ruling 25: "references to a package's exports are fully qualified, with no aliases". Decision 8(a) resolves short names (`file.read.v1`) within an inferred capability, which is effectively an alias at the surface authors write.
   - The package itself counts this as 0 lines saved. Its "no ruling amended" list leaves out 25, and its REJECTED list rejects aliases.
   - What to do: recommend 8(b), full addresses, unless the owner amends 25. Say that only line width is at stake; if the long lines push authors to block style, the block-style row of the sensitivity table still passes.

5. MEDIUM: controls are ignored.
   - The language already has a route for changing a setting in place: controls with revisions (REV-001..009, scenario S5).
   - The planner replaces any change to a Resource's config, and refuses any change to an Agent, make-before-break, instead of sending a control where the Type's `changes` covers the pointer.
   - A legitimate control applied outside the file shows up as config drift, which leads to a conflict or a replacement.
   - What to do: add a change class "config at control pointers -> control request with revision as the precondition". Treat revision changes made by controls as known, not as drift. The cost in S1 is 0, since the S1 Types declare no controls.

6. MEDIUM: scope guard (ruling 4). The project file cannot express what S2-S9 need.
   - `reach` targets only `<resource>`. Grants to an Agent (peer turn), to a Pool (an Agent as operator), to `access` with a bound (delegation, preservation test 5), and secret references (S2) have no syntax.
   - The escape hatch is framed documents passed through unchanged. Those cannot refer to compiled objects, because replacement renames them (shared -> shared--2 -> shared--3). The planner also cannot change them, so 4-01's costs come back.
   - What to do: let the planner resolve project keys inside passed-through documents and manage them, or widen `to:` to the other kinds now. Also state how references from other projects survive a replacement.

7. MEDIUM: windows without access are understated.
   - With `as`, IFC-014 forbids two Conduits for one consumer under the same local name unless one is closed, so the old and new grants cannot overlap.
   - T5 now has a window for both Agents at once. The coordinator's T5 had none. The table lists windows only for T4-T6 and is silent on this regression at T5.
   - "Seconds" has no evidence behind it. In the fresh run, a register took 17 s and opens took 9-14 s, about 40 s for A's window at T4. The coordinator saw about 30 s at T4 and about 80 s at T6.
   - What to do: list the T5 window. Set a latency target in item 7 (one session per apply). Measure the windows in the rerun, and say that swap.v1 may be needed before v0 if the windows stay long.

8. MEDIUM: disclosure.
   - 5c's "list only what the caller may inspect, and count the rest" reveals that objects exist. LIF-037 and ACC-033 avoid exactly that ("learns nothing of whether the object existed"), and ruling 52 excepts it.
   - Tombstones expose the caller, invocation and idempotency key to anyone who may inspect, and no disclosure rule covers them.
   - What to do: drop the count, or put it to the owner as a ruling-52 question. Show a tombstone's caller and idempotency key only to root and to the caller who closed it.
   - Also say that the plan and render outputs and the applied file are redacted like the diagnostics.

9. MEDIUM: thesis 2 and conformance.
   - The author's artifact is the project file. Its meaning, including security-relevant derived Conduits (the operator Conduit with set-up and retire, the turn Conduits), is fixed by an unspecified compiler with no vectors. The independent builds (A34) never exercise it.
   - Decision 2 should say plainly that thesis 2 is not tested for what authors write.
   - What to do: add golden compiler cases (file -> canonical documents) under conformance/, marked as not binding. Write the derived-default rules, the naming scheme, the change classes and the conflict semantics as a precise design note. That note is reading cost that is not budgeted today.

10. MEDIUM: the bar says "work", but the projection covers lines only.
   - Other measures are not projected:
     - engine requests: about 51 sent by the tool, against the coordinator's 35;
     - windows without access;
     - time, at today's 9-17 s per request;
     - concepts (gap 3);
     - files edited: 1, which favours the package and should be claimed.
   - Two projected measures are also miscounted:
     - lines naming a contract: 8, not 4, since the grant lines name operation contracts;
     - grants made by hand: T4 edits 2.

11. LOW-MEDIUM: the claim of removing the fresh author's 130 script lines is overstated.
   - Exit codes, retries, the `--store` flag and document generation are addressed. The check harness is not: turn.py exists to escape JSON for `<agent>--turn.turn.v1`.
   - Generated names (`agent-a--turn`) reach the author in every check.
   - What to do: give derived turn Conduits `as: <agent key>`, or add `conduit turn <agent> <message>`, and offer `--json` output for plan, apply and reach.
   - 4-12's fix is still unspecified. The fresh author's 128 lines at T7 came from it, so the T7 projection for a fresh author depends on that fix. Under ruling 49, replacing an Agent also loses its login, and the plan should say so.

12. LOW-MEDIUM: relative paths and portability.
   - Ruling 52 pt 3 left the question of relative values to this simplification, and the package is silent on it.
   - A committed project file with absolute `<lab>` paths, plus `conduit init` recording a store path, is tied to one machine. Ruling 51 implies recording an engine endpoint (a context), not a store.

13. LOW-MEDIUM: costs.
   - The cargo-add pattern relies on toml_edit. Rust has no mature YAML editor that keeps comments and layout, so "200-300 lines" for install is optimistic. Consider option 5(b), a separate lock file the tool writes in full.
   - 1,500-2,500 lines for the planner and executor looks low: about nine change classes, three-way diff, waits, resume, contract and tools-map diffs, pending sign-ins. The CLI as a whole would be about 50-80% of today's shell (6,869 lines).
   - Documentation of the layer is not budgeted.

14. LOW-MEDIUM: a cheaper sequencing was not offered.
   - Option: build the layer with `as` and expectedId only; read state through today's inspect.v1 and the committed applied file (one actor in S1); rerun; and only then decide 5c and 5d with evidence. That saves about 6-8 promises before ruling 45's evidence exists.
   - Monotonic names for Conduits are unnecessary, since LIF-035 frees a Conduit's name on close.

15. LOW: concurrency.
   - There is no apply lock (Terraform locks its state).
   - Two concurrent applies can choose the same `--N` name, so "never collide" overclaims. They fail safely under OBJ-002 or IFC-014, but leave state partly applied.
   - During the close -> register -> open window, a third party could register `as: project` for the same consumer. 5e only mitigates this.

16. LOW: naming and housekeeping.
   - The one-off templates of closed Conduits are never unregistered in the T4, T5 or T7 plans.
   - Template and instance names within one kind (OBJ-002) are unspecified.
   - The default Pool named after the project collides with any key of the same name (ruling 37: names are unique across kinds).

17. LOW: item 6 details.
   - Check that NAM-024 allows adding defaults under `claude-code.v1` and `codex.v1`; today `required` holds both flags with no default. If it does not, both Types need `.v2`.
   - The rewrite files pin 0.1.1 while the text says 0.1.2.
   - Ruling 48 pt 1 says a flag is turned on "by an input of its template"; state the project-file syntax for that.

18. LOW: counting choices that favour the package.
   - One `install` with three archives is counted as one line; both earlier authors ran three installs, which makes T1 18.
   - The paper count has no mistakes in it. The margins of 1-2 lines on T5-T7 are erased by one extra command.

19. LOW: prior art.
   - MCP's `instructions` field in the initialize result could carry target and Type descriptions once, instead of repeating them in every tool description as 5e does.
   - Kubernetes server-side apply field managers address ownership among several appliers.
   - Terraform's moved and import blocks cover key renames and adopting existing objects.
   - Tailscale ACL files and IAM policy versions replace a policy atomically, which supports bringing swap.v1 forward.

SEVERITY: major. Gaps 1-3 must be fixed before the owner decides; gap 1 alone can turn the projected pass into a fail.