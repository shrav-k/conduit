"""Regression cases for the ledger's current rows, statuses and record/date guards."""

from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_rulings as rulings


class RulingChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.proposals = self.root / "docs/proposals"
        self.proposals.mkdir(parents=True)
        self.record = self.proposals / "ruling-3/README.md"
        self.record.parent.mkdir()
        self.record.write_text(
            "# Ruling 3\n\nStatus: in force (ruled by the owner on 2026-11-10). "
            "Holds now: [topic](../../../IN_FORCE.md#topic). This record says why.\n\nWhy.\n")
        self.ledger = self.proposals / "ruling-log.md"
        self.table = rulings.HEADER + "\n| --- | --- | --- | --- | --- |\n"
        self.rows = [f"| {n} | 2026-11-10 | Gist | Words | in force |\n" for n in range(1, 4)]
        self.ledger.write_text(self.table + "".join(self.rows))
        self.in_force = self.root / "IN_FORCE.md"
        self.in_force.write_text("# Decisions\n\n## Topic\n\n- A decision (3).\n")

    def test_complete_ledger_with_historical_rows(self):
        self.ledger.write_text(self.ledger.read_text() +
                              "\n## History\n\n| # | Ruling |\n| --- | --- |\n| 1 | Old row |\n")
        self.assertEqual(rulings.check(self.root), [])

    def test_missing_duplicate_and_out_of_range_rows(self):
        self.ledger.write_text(self.table + self.rows[0] * 2 + self.rows[2] +
                              "| 4 | 2026-11-10 | Gist | Words | done |\n")
        errors = "\n".join(rulings.check(self.root))
        self.assertIn("ruling 1: expected one ledger row, found 2", errors)
        self.assertIn("ruling 2: expected one ledger row, found 0", errors)
        self.assertIn("ruling 4: outside", errors)

    def test_new_directory_raises_highest_even_without_readme(self):
        (self.proposals / "ruling-4").mkdir()
        self.assertIn("ruling 4: expected one ledger row, found 0", rulings.check(self.root))

    def test_allowed_statuses_and_free_point_notes(self):
        for value in ("in force", "done (point 2)", "directive (A75)", "open (C6 to C12)",
                      "amended by 67 (point 2; the ledger)", "replaced by 16", "withdrawn",
                      "in force; done (point 1); amended by 66 (point 3)"):
            with self.subTest(value=value):
                self.assertTrue(rulings.valid_status(value))
        for value in ("answered by 65", "settled by 25", "done;", "in force, directive (A75)",
                      "directive", "directive (work order 033)", "open ()", "amended by 0 (point 1)", "amended by 67"):
            with self.subTest(value=value):
                self.assertFalse(rulings.valid_status(value))

    def test_bad_status_cell_is_reported(self):
        self.ledger.write_text((self.table + "".join(self.rows)).replace(
            "Words | in force", "Words | settled by 4", 1))
        self.assertTrue(any("ruling 1: invalid status" in e for e in rulings.check(self.root)))

    def test_status_must_be_first_paragraph_and_one_line(self):
        for text in ("# Ruling 3\n\nIntroduction.\n\nStatus: in force.\n",
                     "# Ruling 3\n\nStatus: in force.\nHolds now: topic. This record says why.\n"):
            with self.subTest(text=text):
                self.record.write_text(text)
                self.assertTrue(any("first paragraph" in e for e in rulings.check(self.root)))

    def test_dates_on_decision_and_continuation_lines(self):
        for date in ("2026-11-10", "2026/11/10", "November 10, 2026", "10 November 2026"):
            with self.subTest(date=date):
                self.in_force.write_text("# Decisions\n\n## Topic\n\n- A decision\n  since " + date)
                self.assertTrue(any("date in decisions" in e for e in rulings.check(self.root)))

    def test_record_status_must_agree_with_ledger(self):
        self.record.write_text(self.record.read_text().replace("Status: in force", "Status: done"))
        self.assertIn("ruling 3: record status differs from ledger", rulings.check(self.root))

    def test_historical_intro_date_and_version_are_not_decision_dates(self):
        self.in_force.write_text("# Decisions\n\nIntroduced 2026-11-10.\n\n## Topic\n\n- Version 0.1.0 (3).\n")
        self.assertEqual(rulings.check(self.root), [])


if __name__ == "__main__":
    unittest.main()
