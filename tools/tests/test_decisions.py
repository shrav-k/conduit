import contextlib
import io
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_decisions as decision


DRAFT = decision.DRAFT + ".\n"
UNRELEASED = "## [Unreleased]\n"
STATUS = "Status: accepted, not done (work remains)\n"
ORDER = "**Preservation tests**: none.\n\n**Should this exist**: yes.\n"
SAFE = "- Changed: engine text in spec/README.md#overview (DEC-001).\n"


class DecisionTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.git("init", "-q")
        self.write(".gitignore", "ignored/\n")
        self.write("AGENTS.md", "line\n" * 150)
        self.write(decision.WORK, DRAFT + "\n## 1. Work\n\n1. Keep evidence (DEC-001) `by review`\n")
        self.write(decision.CHANGES, DRAFT + "\n" + UNRELEASED)
        self.write(decision.DECISIONS, DRAFT)
        self.write(decision.LOG, DRAFT + "\n" + self.entry(1))
        self.write("spec/README.md", "# Overview\n\nEngine text.\n")
        self.write(decision.DEFINITIONS, "# Definitions\n\n| Term | Explanations |\n| --- | --- |\n"
                   "| object, document | An object or document. |\n")
        self.write(decision.OPERATIONS, "metadata:\n  name: inspect.v1\n")
        self.write("tools/check_stub.py", "# A fixture.\n")
        self.write("IN_FORCE.md", "# In force\n\n- A rule\n")
        self.write("docs/AUDIT.md", "# Audit\n")
        self.write("docs/proposals/ruling-1/README.md", "# A ruling\n")
        self.write("docs/work-orders/001-order.md", ORDER)
        self.standing()
        self.base = self.commit()

    def git(self, *args):
        return subprocess.run(["git", "-C", str(self.root), *args], check=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout

    def commit(self):
        self.git("add", "-A")
        self.git("-c", "user.name=Conduit work order", "-c", "user.email=work-orders@conduit.invalid",
                 "-c", "commit.gpgsign=false", "commit", "-qm", "test snapshot")
        return self.git("rev-parse", "HEAD").decode().strip()

    def write(self, path, contents):
        file = self.root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(contents, encoding="utf-8")

    def append(self, path, contents):
        self.write(path, (self.root / path).read_text(encoding="utf-8") + contents)

    def remove(self, *paths):
        for path in paths:
            (self.root / path).unlink()

    @staticmethod
    def entry(number, dated="2026-11-20", status=STATUS, body="Owner's words: test answer\n"):
        return f"## DEC-{number:03d}, {dated}, A decision\n\n{body}\n{status}\n"

    def standing(self, no_case=1, pass_a=True, keep_b=True):
        text = f"| no case, no reason | 0 | {no_case} | 0 | {no_case} |\n"
        text += "| ACC-001 | convention | must | 1 | 0 | cases | shell | "
        text += ("case passes" if pass_a else "expected failure") + " |\n"
        if keep_b:
            text += "| OPS-002 | convention | must | 0 | 0 | outside | none | no case, no reason |\n"
        self.write(decision.STANDING, text)

    def undraft(self, *paths):
        for path in paths or (decision.WORK, decision.CHANGES, decision.DECISIONS, decision.LOG):
            self.write(path, (self.root / path).read_text(encoding="utf-8").replace(DRAFT, "", 1))

    def run_main(self, result=0, *options, after="WORKTREE", before=None):
        output = io.StringIO()
        with mock.patch.object(Path, "cwd", return_value=self.root), contextlib.redirect_stdout(output):
            status = decision.main([before or self.base, after, "--today", "2026-12-19", *options])
        printed = output.getvalue()
        self.assertEqual(status, result, printed)
        self.assertRegex(printed.splitlines()[-1], r"^Errors: [0-9]+$")
        return printed

    def test_in_force_all_draft_or_all_live(self):
        self.assertIn("Framework: not in force", self.run_main())
        self.undraft()
        self.assertIn("Framework: in force.", self.run_main())

    def test_in_force_mixed_status_fails(self):
        self.undraft(decision.CHANGES)
        self.assertIn("error: spec/CHANGES.md: framework:", self.run_main(1))

    def test_in_force_bad_args_and_revisions_exit_two(self):
        for kwargs in ({"before": "WORKTREE"}, {"before": "no-such-revision"}, {"after": "no-such-revision"}):
            with self.subTest(kwargs=kwargs):
                self.assertIn("error:", self.run_main(2, **kwargs))
        self.assertIn("error:", self.run_main(2, "--today", "2026-02-30"))

    def test_coupling_reports_in_draft_and_fails_in_force(self):
        self.write("spec/sample.yaml", "changed: true\n")
        self.assertIn("report: spec/sample.yaml: coupling:", self.run_main())
        self.undraft()
        self.assertIn("error: spec/sample.yaml: coupling:", self.run_main(1))
        self.append(decision.CHANGES, SAFE)
        self.assertNotIn(": coupling:", self.run_main())

    def test_coupling_ignores_commented_unreleased_item_in_draft_and_live(self):
        self.write("spec/sample.yaml", "changed: true\n")
        self.append(decision.CHANGES, "\n<!--\n" + SAFE + "-->\n")
        self.assertIn("report: spec/sample.yaml: coupling:", self.run_main())
        self.undraft()
        self.assertIn("error: spec/sample.yaml: coupling:", self.run_main(1))

    def test_coupling_exempts_only_link_destinations(self):
        start = "# Overview\n\n[link](old(a(b)) \"title\") and [reference][ref].\n\n[ref]: old\n"
        changed = start.replace("old(a(b))", "<dir/new target>").replace("[ref]: old", "[ref]: new")
        self.write("spec/README.md", start)
        self.undraft()
        self.base = self.commit()
        self.write("spec/README.md", changed)
        self.assertNotIn(": coupling:", self.run_main())
        self.write("spec/README.md", changed.replace('"title"', '"new title"'))
        self.assertIn("error: spec/README.md: coupling:", self.run_main(1))

    def test_coupling_keeps_link_examples_in_code_significant(self):
        self.undraft()
        text = "# Overview\n\n`[sample](old)`\n"
        self.write("spec/README.md", text)
        self.base = self.commit()
        self.write("spec/README.md", text.replace("old", "new"))
        self.assertIn("error: spec/README.md: coupling:", self.run_main(1))

    def test_coupling_keeps_indented_code_link_bytes_significant(self):
        self.undraft()
        text = "# Overview\n\n    [example](old)\n"
        self.write("spec/README.md", text)
        self.base = self.commit()
        self.write("spec/README.md", text.replace("old", "new"))
        self.assertIn("error: spec/README.md: coupling:", self.run_main(1))

    def test_coupling_exempts_links_in_indented_list_continuations(self):
        self.undraft()
        text = "# Overview\n\n- A list item.\n    A [link](old) in the same item.\n"
        self.write("spec/README.md", text)
        self.base = self.commit()
        self.write("spec/README.md", text.replace("old", "new"))
        self.assertNotIn(": coupling:", self.run_main())

    def test_coupling_counts_a_mode_change(self):
        self.undraft()
        self.base = self.commit()
        (self.root / "spec/README.md").chmod(0o755)
        self.assertIn("error: spec/README.md: coupling:", self.run_main(1))

    def test_coupling_needs_unreleased_and_accepts_wrapped_item(self):
        self.undraft()
        self.base = self.commit()
        self.remove("spec/definitions.md")
        self.append(decision.CHANGES, "\n## [0.1.0]\n" + SAFE)
        self.assertIn("error: spec/definitions.md: coupling:", self.run_main(1))
        self.write(decision.CHANGES, UNRELEASED + "- Fixed: ACC-001\n  (DEC-001, ruling 92 pt 1).\n")
        self.assertNotIn(": coupling:", self.run_main())

    def test_coupling_commit_after_ignores_worktree(self):
        self.undraft()
        self.base = self.commit()
        self.append(decision.CHANGES, SAFE)
        head = self.commit()
        self.write("spec/README.md", "# Different heading\n")
        self.assertNotIn(": coupling:", self.run_main(after=head))
        self.assertIn("error: spec/CHANGES.md:", self.run_main(1))

    def test_changes_line_keeps_promises_anchor_and_terms(self):
        self.append(decision.CHANGES, "- Added: ACC-001 (DEC-001).\n"
                    "- Fixed: `<!--` in the `document` and `object, document` terms (ruling 1).\n" + SAFE)
        self.assertNotIn(": CHANGES lines:", self.run_main())

    def test_changes_line_does_not_accept_indented_code_as_definitions(self):
        self.append(decision.DEFINITIONS, "\n    | Term | Explanation |\n"
                    "    | --- | --- |\n    | simulated | An example. |\n")
        self.append(decision.CHANGES, "- Fixed: the `simulated` term (DEC-001).\n")
        self.assertIn("expected a spec anchor or a backticked definitions term", self.run_main(1))

    def test_changes_line_validates_a_nested_bullet_separately(self):
        self.append(decision.CHANGES, "- Added: ACC-001 (DEC-001).\n"
                    "    - Updated: ACC-001 (DEC-001).\n")
        output = self.run_main(1)
        self.assertIn("error: spec/CHANGES.md:5: CHANGES lines:", output)
        self.assertEqual(output.count(": CHANGES lines:"), 1)

    def test_changes_line_rejects_wrong_kind_citations_and_subjects(self):
        items = ["- Updated: ACC-001 (DEC-001).\n",
                 "- Changed: ACC-001 (DEC-001, DEC-999).\n",
                 "- Changed: ACC-001 (ruling 93, DEC-001).\n",
                 "- Changed: ACC-001 (ruling 0).\n",
                 "- Changed: ACC-001 (DEC-001, ruling -1).\n",
                 "- Changed: ABC-999 (DEC-001).\n",
                 "- Changed: spec/README.md#absent (DEC-001).\n",
                 "- Changed: `Term` (DEC-001).\n",
                 "+ Changed: ACC-001 (DEC-001).\n"]
        self.append(decision.CHANGES, "".join(items))
        self.assertEqual(self.run_main(1).count(": CHANGES lines:"), len(items))

    def test_changes_line_removed_must_have_left_the_standing(self):
        self.append(decision.CHANGES, "- Removed: OPS-002 (ruling 78 pt 17).\n")
        self.assertIn("unexpected promise standing for OPS-002", self.run_main(1))
        self.standing(no_case=0, keep_b=False)
        self.assertNotIn(": CHANGES lines:", self.run_main())

    def test_changes_line_reads_ids_only_in_standing_families(self):
        algorithms = "SHA-256, SHA-512, AES-256 and ISO-639"
        cases = (
            (f"{algorithms} in spec/README.md#overview", None),
            (algorithms, "expected a spec anchor or a backticked definitions term"),
            ("ACC-999", "unexpected promise standing for ACC-999"),
        )
        for subject, finding in cases:
            with self.subTest(subject=subject):
                self.write(decision.CHANGES, DRAFT + UNRELEASED
                           + f"- Changed: {subject} (DEC-001).\n")
                output = self.run_main(int(finding is not None))
                if finding is None:
                    self.assertNotIn(": CHANGES lines:", output)
                else:
                    self.assertIn(finding, output)
                if subject == algorithms:
                    self.assertNotIn("unexpected promise standing", output)

    def test_changes_line_takes_families_from_before_and_after(self):
        self.standing(no_case=0, keep_b=False)
        self.append(decision.STANDING,
                    "| ZZZ-001 | convention | must | 1 | 0 | cases | shell | case passes |\n")
        for kind, ref, valid in (
            ("Removed", "OPS-002", True),
            ("Removed", "OPS-999", False),
            ("Added", "ZZZ-001", True),
            ("Added", "ZZZ-002", False),
        ):
            with self.subTest(kind=kind, ref=ref):
                self.write(decision.CHANGES, DRAFT + UNRELEASED + f"- {kind}: {ref} (DEC-001).\n")
                output = self.run_main(int(not valid))
                if valid:
                    self.assertNotIn(": CHANGES lines:", output)
                else:
                    self.assertIn(f"unexpected promise standing for {ref}", output)

    def test_changes_line_checks_additions_but_not_released_movement(self):
        self.append(decision.CHANGES, "- Older wording.\n")
        self.base = self.commit()
        self.write(decision.CHANGES, DRAFT + UNRELEASED + "\n## [0.1.0]\n- Older wording.\n")
        self.assertNotIn(": CHANGES lines:", self.run_main())

    def test_append_only_allows_append_and_done_status_with_pr(self):
        before = DRAFT + self.entry(1, status=STATUS + "  Another line.\n")
        self.write(decision.LOG, before)
        self.base = self.commit()
        self.write(decision.LOG, DRAFT + self.entry(1, status="Status: done\n  Pull request #456\n") + self.entry(2))
        self.assertNotIn(": append-only:", self.run_main())

    def test_append_only_rejects_rewritten_bytes_and_bad_status(self):
        self.write(decision.LOG, DRAFT + self.entry(1, status="Status: done\n", body="Different words.\n"))
        output = self.run_main(1)
        self.assertIn("changed outside Status", output)
        self.assertIn("done with a pull request", output)

    def test_append_only_rejects_removed_entry(self):
        self.write(decision.LOG, DRAFT)
        self.assertIn("DEC-001 is missing or moved", self.run_main(1))

    def test_numbering_allows_consecutive_entries_and_same_dates(self):
        self.append(decision.LOG, self.entry(2) + self.entry(3, "2026-11-21"))
        self.assertNotIn(": numbering:", self.run_main())

    def test_numbering_rejects_non_entry_gap_repeat_and_backdating(self):
        self.append(decision.LOG, "## Direction\n" + self.entry(4, "2026-11-19")
                    + self.entry(4, "2026-11-19") + self.entry(5, "2026-02-30"))
        output = self.run_main(1)
        for fragment in ("expected a DEC-NNN entry heading", "expected DEC-003", "expected a valid date",
                         "entry date is earlier"):
            self.assertIn(fragment, output)
        self.assertEqual(output.count(": numbering:"), 4)

    def test_gone_allows_removal_and_remaining_absent(self):
        self.remove("docs/proposals/ruling-1/README.md", "IN_FORCE.md", "docs/AUDIT.md")
        self.assertNotIn(": gone stays gone:", self.run_main())
        self.base = self.commit()
        self.assertNotIn(": gone stays gone:", self.run_main())

    def test_gone_rejects_restoration_even_in_draft(self):
        paths = ("docs/proposals/ruling-1/README.md", "IN_FORCE.md", "docs/AUDIT.md")
        self.remove(*paths)
        self.base = self.commit()
        for path in paths:
            self.write(path, "# Restored\n")
        self.assertEqual(self.run_main(1).count(": gone stays gone:"), 3)

    def test_gone_rejects_a_symlink_at_the_retired_proposals_path(self):
        self.remove("docs/proposals/ruling-1/README.md")
        self.base = self.commit()
        proposals = self.root / "docs/proposals"
        (proposals / "ruling-1").rmdir()
        proposals.rmdir()
        proposals.symlink_to("decisions")
        for after in ("WORKTREE", self.commit()):
            with self.subTest(after=after):
                self.assertIn("error: docs/proposals: gone stays gone:", self.run_main(1, after=after))

    def test_third_copy_reports_in_draft_and_fails_in_force(self):
        self.write("docs/proposals/ruling-1/added.md", "# Added\n")
        self.append("IN_FORCE.md", " * Another bullet\n")
        self.assertEqual(self.run_main().count(": no third copy:"), 2)
        self.undraft()
        self.assertEqual(self.run_main(1).count(": no third copy:"), 2)

    def test_third_copy_allows_rewrap_and_existing_proposal_edit(self):
        self.undraft()
        self.base = self.commit()
        self.append("IN_FORCE.md", "  more words in the first bullet\n  on two lines\n")
        self.append("docs/proposals/ruling-1/README.md", "A revision.\n")
        self.assertNotIn(": no third copy:", self.run_main())

    def test_caps_agents_151_fails_after_150(self):
        self.append("AGENTS.md", "one\n")
        self.assertIn("error: AGENTS.md: cap: 151 lines exceeds 150", self.run_main(1))

    def test_caps_agents_151_reports_after_222(self):
        self.write("AGENTS.md", "line\n" * 222)
        self.base = self.commit()
        self.write("AGENTS.md", "line\n" * 151)
        self.assertIn("report: AGENTS.md: cap: 151 lines exceeds 150", self.run_main())

    def test_caps_bytes_and_log_entry_baselines(self):
        self.write(decision.WORK, DRAFT + "é" * 6200)
        self.append(decision.LOG, self.entry(2, body="é" * 780 + "\n"))
        output = self.run_main(1)
        self.assertIn("error: HOW-WE-WORK.md: cap:", output)
        self.assertIn("cap: DEC-002 is", output)
        self.base = self.commit()
        output = self.run_main()
        self.assertIn("report: HOW-WE-WORK.md: cap:", output)
        self.assertIn("report: docs/decisions/log.md:", output)

    def test_caps_new_files_are_armed_and_missing_bases_report(self):
        self.remove("AGENTS.md", decision.CHANGES, decision.LOG, decision.STANDING)
        self.base = self.commit()
        self.write("AGENTS.md", "new line\n" * 151)
        self.write(decision.CHANGES, DRAFT + UNRELEASED)
        self.write(decision.LOG, DRAFT + self.entry(1))
        self.standing()
        output = self.run_main(1)
        self.assertIn("error: AGENTS.md: cap:", output)
        self.assertIn("append-only: no file in BEFORE", output)
        self.assertIn("CHANGES lines: no file in BEFORE", output)
        self.assertIn("no-case ratchet: no file in BEFORE", output)

    def test_form_reports_in_draft_and_fails_in_force(self):
        self.append(decision.WORK, "\n- In order 224, on 2026-11-20. *Coordinator's* (DEC-999) `refused: no.v1`\n")
        output = self.run_main()
        self.assertEqual(sum(line.startswith("report: HOW-WE-WORK.md:") for line in output.splitlines()), 3)
        self.undraft()
        output = self.run_main(1)
        self.assertEqual(sum(line.startswith("error: HOW-WE-WORK.md:") for line in output.splitlines()), 3)

    def test_form_accepts_wrapping_labels_and_coordinator_word(self):
        self.undraft()
        self.append(decision.WORK, "2. Coordinator's check (ruling 4,\n"
                    "   DEC-001) `checked: tools/check_stub.py`\n"
                    "3. Enforced (DEC-001) `refused: management.inspect.v1`\n"
                    "\n## Notes\n\n- This is outside the numbered headings.\n")
        self.assertIn("By review: 1 of 3 HOW-WE-WORK rule lines.", self.run_main())
        self.remove("tools/check_stub.py")
        self.assertIn("expected a review, existing checker, or operation label", self.run_main(1))

    def test_form_parent_cannot_borrow_nested_citation_and_label(self):
        self.undraft()
        self.append(decision.WORK, "\n- Parent needs its own ending\n"
                    "    - Child has its own ending (DEC-001) `by review`\n")
        output = self.run_main(1)
        self.assertIn("error: HOW-WE-WORK.md:6: HOW-WE-WORK form: expected decision citations", output)
        self.assertNotIn("error: HOW-WE-WORK.md:7:", output)
        self.assertIn("By review: 2 of 3 HOW-WE-WORK rule lines.", output)

    def test_form_checked_label_refuses_a_dangling_symlink(self):
        self.undraft()
        self.append(decision.WORK, "2. Checked (DEC-001) `checked: tools/check_stub.py`\n")
        self.remove("tools/check_stub.py")
        (self.root / "tools/check_stub.py").symlink_to("missing.py")
        for after in ("WORKTREE", self.commit()):
            with self.subTest(after=after):
                self.assertIn("expected a review, existing checker, or operation label",
                              self.run_main(1, after=after))

    def test_orders_checks_untracked_orders_and_exempts_subdirectories(self):
        self.write("docs/work-orders/002-order.md", "**Should this exist**: yes.\n")
        self.write("docs/work-orders/nested/003-order.md", "# Notes\n")
        self.assertIn("new orders: missing Preservation tests", self.run_main(1))
        self.write("docs/work-orders/002-order.md", ORDER)
        self.assertNotIn(": new orders:", self.run_main())

    def test_orders_size_is_report_only_and_ratio_uses_tracked_files(self):
        path = "docs/work-orders/002-order.md"
        self.write(path, ORDER + "x" * (20 * 1024) + "\n")
        first = self.run_main()
        self.assertIn("report: docs/work-orders/002-order.md: order size:", first)
        line = lambda output: next(line for line in output.splitlines() if line.startswith("Ratio: "))
        self.write("ignored/docs/work-orders/001.md", "ignored\n")
        self.assertEqual(line(first), line(self.run_main()))
        head = self.commit()
        self.assertNotEqual(line(first), line(self.run_main(after=head)))
        self.assertRegex(line(self.run_main(after=head)), r"\([0-9]+\.[0-9]{2}\)\.$")

    def test_no_case_allows_decrease_and_reports_new_ceiling(self):
        self.standing(no_case=0)
        self.assertIn("No-case ratchet: base 1, head 0.", self.run_main())
        self.base = self.commit()
        self.standing(no_case=1)
        self.assertIn("No-case ratchet: base 0, head 1.", self.run_main(1))

    def test_no_case_reports_only_when_standing_is_missing_from_both_snapshots(self):
        self.remove(decision.STANDING)
        self.base = self.commit()
        output = self.run_main()
        self.assertIn("report: engine/STANDING.md: no-case ratchet: no file in BEFORE", output)
        self.assertNotIn("No-case ratchet: base", output)
        self.assertIn("By review: 1 of 1 HOW-WE-WORK rule lines.", output)
        self.write(decision.STANDING, "| ACC-001 | case passes |\n")
        self.assertNotIn("expected one no case, no reason totals row", self.run_main())

    def test_no_case_rejects_rise_and_bad_totals(self):
        self.standing(no_case=2)
        self.assertIn("no case, no reason total rose", self.run_main(1))
        self.write(decision.STANDING, "| ACC-001 | case passes |\n")
        self.assertIn("expected one no case, no reason totals row", self.run_main(1))

    def test_not_done_boundary_and_dormancy(self):
        self.append(decision.LOG, self.entry(2, "2026-11-21"))
        self.base = self.commit()
        self.append(decision.CHANGES, "- Changed: OPS-002 (DEC-001).\n- Added: OPS-002 (DEC-002).\n")
        output = self.run_main(1)
        self.assertIn("Not-done ratchet: base 0, head 1, entries dated before 2026-11-21.", output)
        self.assertIn("dormant decision: DEC-001 is not done", output)
        self.assertNotIn("dormant decision: DEC-002", output)
        self.base = self.commit()
        self.assertIn("Not-done ratchet: base 1, head 1", self.run_main())

    def test_not_done_reads_ids_only_in_standing_families(self):
        self.append(decision.CHANGES,
                    "- Changed: SHA-256 in spec/README.md#overview (DEC-001).\n")
        output = self.run_main()
        self.assertIn("Not-done ratchet: base 0, head 0", output)
        self.assertNotIn("dormant decision: DEC-001", output)

    def test_not_done_lost_case_fails_even_for_released_item(self):
        self.append(decision.CHANGES, "\n## [0.1.0]\n- Added: ACC-001 (DEC-001).\n")
        self.base = self.commit()
        self.standing(pass_a=False)
        self.assertIn("Not-done ratchet: base 0, head 1", self.run_main(1))

    def test_not_done_removal_prose_and_passing_case_count_as_done(self):
        self.append(decision.LOG, self.entry(2) + self.entry(3, status="") + self.entry(4))
        self.append(decision.CHANGES, SAFE + "- Removed: OPS-002 (DEC-002).\n")
        self.standing(no_case=0, keep_b=False)
        output = self.run_main()
        self.assertIn("Not-done ratchet: base 0, head 0", output)
        self.assertIn("dormant decision: DEC-004", output)
        self.assertNotIn("dormant decision: DEC-003", output)
        self.assertNotIn("dormant decision: DEC-002", output)

    def test_not_done_can_fall_and_notes_only_final_citations(self):
        self.append(decision.CHANGES, "- Added: OPS-002 (DEC-001).\n")
        self.base = self.commit()
        self.write(decision.CHANGES, DRAFT + UNRELEASED
                   + "- Changed: ACC-001, discussed above in DEC-102 (DEC-001).\n")
        self.assertIn("Not-done ratchet: base 1, head 0", self.run_main())


if __name__ == "__main__":
    unittest.main()
