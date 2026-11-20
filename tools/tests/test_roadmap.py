"""Exercise the roadmap's dated results check on small disposable trees."""

from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_roadmap as roadmap


class RoadmapChecks(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name).resolve()
        for name, value in (("ROOT", self.root), ("EXPERIMENTS", self.root / "experiments")):
            patched = patch.object(roadmap, name, value)
            patched.start()
            self.addCleanup(patched.stop)

    def write(self, path, text=""):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")

    def run_rows(self, rows):
        self.write("ROADMAP.md", "## Experiments and probes\n\n"
                   "| Item | Question it settles | State | Record |\n"
                   "| --- | --- | --- | --- |\n" + "\n".join(rows) + "\n")
        register = [table for table in roadmap.tables(self.root / "ROADMAP.md")
                    if table["h2"] == roadmap.REGISTER]
        return roadmap.done_without_results(register)

    def failure(self, line, item, linked):
        return (f"ROADMAP.md:{line}: {item} is done, and its State cell links no dated results: "
                "a file experiments/NNN-name/results-YYYY-MM-DD.md, or a heading of a file in "
                "experiments/NNN-name/ whose anchor ends in results-YYYY-MM-DD. "
                f"It links {linked}.")

    def test_done_row_linking_a_dated_results_file_passes(self):
        first = "experiments/007-sample/results-2026-11-09.md"
        second = "experiments/008-sample/results-2026-11-09-later-turn.md"
        self.write(first)
        self.write(second)
        self.assertEqual(self.run_rows([
            f"| A70 | Sample | done: [results]({first}) | |",
            f"| A71 | Sample | done: [results]({second}) | |",
        ]), (2, []))

    def test_done_row_linking_a_dated_results_heading_passes(self):
        path = "experiments/012-sample/README.md"
        self.write(path, "## MC1 dated results, 2026-11-11\n")
        self.assertEqual(self.run_rows([
            f"| Experiment 012 | Sample | done: [results]({path}#mc1-dated-results-2026-11-11) | |",
        ]), (1, []))

    def test_pinned_results_require_a_full_commit_sha(self):
        base = "https://github.com/shrav-k/conduit/blob/"
        commit = "0123456789abcdef0123456789abcdef01234567"
        for path in ("experiments/007-sample/results-2026-11-09.md",
                     "experiments/007-sample/README.md#probe-results-2026-11-09"):
            for revision, passes in ((commit, True), (commit.upper(), True),
                                     (commit[:8], False), ("main", False)):
                with self.subTest(path=path, revision=revision):
                    link = f"{base}{revision}/{path}"
                    errors = [] if passes else [
                        self.failure(5, "A70", f"{link} (not a local link)")
                    ]
                    self.assertEqual(self.run_rows([
                        f"| A70 | Sample | done: [results]({link}) | |",
                    ]), (1, errors))

    def test_done_row_without_dated_results_fails(self):
        readme = "experiments/012-sample/README.md"
        record = "docs/work-orders/104-sample.md#record"
        results = "experiments/007-sample/results-2026-11-09.md"
        self.write(readme)
        self.write("docs/work-orders/104-sample.md", "## Record\n")
        self.write(results)
        self.assertEqual(self.run_rows([
            f"| **[Experiment 012]({readme})** | Sample | done: [Record]({record}) | |",
            f"| A70 | Sample | done: `[results]({results})` <!-- [results]({results}) --> | |",
            f"| A71 | Sample | done | [results]({results}) |",
        ]), (3, [
            self.failure(5, "Experiment 012", f"{record} (not in a directory experiments/NNN-name/)"),
            self.failure(6, "A70", "nothing"),
            self.failure(7, "A71", "nothing"),
        ]))

    def test_each_link_that_does_not_count_names_its_reason(self):
        targets = [
            ("https://example.com/results-2026-11-09.md", "not a local link"),
            ("experiments/README.md", "not in a directory experiments/NNN-name/"),
            ("experiments/007-sample/evidence-2026-11-09/results-2026-11-09.md",
             "not in a directory experiments/NNN-name/"),
            ("experiments/007-sample/results-2026-11-08.md", "no such file"),
            ("experiments/007-sample/results-2026-13-40.md", "not a calendar date"),
            ("experiments/012-sample/README.md#results-2026-11-05", "no heading with this anchor"),
            ("experiments/007-sample/README.md",
             "neither its file name nor its anchor is dated results"),
        ]
        for target, reason in targets:
            if reason not in ("not a local link", "no such file"):
                self.write(target.split("#")[0])
        links = " ".join(f"[results]({target})" for target, _ in targets)
        expected = "; ".join(f"{target} ({reason})" for target, reason in targets)
        self.assertEqual(self.run_rows([
            f"| Experiment 012 | Sample | done: {links} | |",
        ]), (1, [self.failure(5, "Experiment 012", expected)]))

    def test_dated_file_outside_a_numbered_experiment_directory_fails(self):
        targets = ["experiments/sample/results-2026-11-09.md",
                   "docs/007-sample/results-2026-11-09.md"]
        for target in targets:
            self.write(target)
        links = " ".join(f"[results]({target})" for target in targets)
        expected = "; ".join(f"{target} (not in a directory experiments/NNN-name/)"
                             for target in targets)
        self.assertEqual(self.run_rows([f"| A70 | Sample | done: {links} | |"]),
                         (1, [self.failure(5, "A70", expected)]))

    def test_rows_not_done_are_not_checked(self):
        states = ["in progress: stage 0", "open", "superseded: closed by ruling 41",
                  "dropped by ruling 45", "removed: on 2026-11-05", "not adopted: candidates"]
        self.assertEqual(self.run_rows([
            f"| A{number} | Sample | {state} | |" for number, state in enumerate(states, 1)
        ]), (0, []))


if __name__ == "__main__":
    unittest.main()
