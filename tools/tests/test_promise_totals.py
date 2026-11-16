#!/usr/bin/env python3
"""Regression checks for the promise totals stated in spec/README.md."""

import contextlib
import io
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import check_schemas as schemas


class PromiseTotalsChecks(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        for path, promises in (
            ("promises/a.yaml", (("A-001", "language"), ("A-002", "convention"))),
            ("conventions/b.yaml", (("B-001", "convention"),)),
            ("interface/c.yaml", (("C-001", "interface"),)),
        ):
            file = self.root / "spec" / path
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_text(schemas.yaml.safe_dump([
                {"id": id_, "layer": layer, "level": "must", "when": "always",
                 "then": "holds", "observable": "visible", "vectors": []}
                for id_, layer in promises
            ]), encoding="utf-8")
        self.rows = (
            "| [promises/a.yaml](promises/a.yaml) | A | "
            "Draft; 2 promises; vectors test slices of 1 promises |\n",
            "| [conventions/b.yaml](conventions/b.yaml) | B | "
            "Draft; 1 promise; no vectors yet |\n",
            "| [interface/c.yaml](interface/c.yaml) | C | Draft; 1 promise |\n",
        )
        self.sentence = (
            "Every area has been written as promises, 4 in all: 1 of the layer `language`,\n"
            "2 of the layer `convention` and 1 of the layer `interface`.\n"
        )
        self.readme = (
            "## Contents\n\n| Path | Part | State |\n| --- | --- | --- |\n"
            + "".join(self.rows) + "\n" + self.sentence
            + "\n### Promise files that rulings removed\n\n"
            "| File | IDs it held | Removed by |\n| --- | --- | --- |\n"
            "\n## The form of a promise\n"
        )

    def run_checks(self, readme=None):
        (self.root / "spec/README.md").write_text(
            self.readme if readme is None else readme, encoding="utf-8")
        errors = []
        with patch.object(schemas, "ROOT", self.root), patch.object(schemas, "errors", errors), \
                contextlib.redirect_stdout(io.StringIO()) as output:
            counts = schemas.check_promises()
            schemas.check_promise_totals(counts)
        return counts, errors, output.getvalue()

    def test_totals_that_agree_pass(self):
        counts, errors, output = self.run_checks()
        self.assertEqual(errors, [])
        self.assertEqual(counts, (
            {"promises/a.yaml": 2, "conventions/b.yaml": 1, "interface/c.yaml": 1},
            {"language": 1, "convention": 2, "interface": 1},
        ))
        self.assertIn(
            "8. Compared the promise totals that spec/README.md states with the promise files: "
            "4 promises in 3 files, 1 of the layer `language`, 2 of the layer `convention` and "
            "1 of the layer `interface`; 0 errors.", output.splitlines())

    def test_each_number_wrong_by_one_fails(self):
        changes = [
            ("4 in all:", "5 in all:",
             "spec/README.md: states 5 promises in all, and the promise files hold 4"),
        ]
        for layer, count in (("language", 1), ("convention", 2), ("interface", 1)):
            changes.append((
                f"{count} of the layer `{layer}`", f"{count + 1} of the layer `{layer}`",
                f"spec/README.md: states {count + 1} promises of the layer `{layer}`, "
                f"and the promise files hold {count}",
            ))
        for row, path, count in zip(self.rows, (
                "promises/a.yaml", "conventions/b.yaml", "interface/c.yaml"), (2, 1, 1)):
            changes.append((
                row, row.replace(f"Draft; {count} promise", f"Draft; {count + 1} promise"),
                f"spec/README.md: the Contents table gives {path} {count + 1} promises, "
                f"and the file holds {count}",
            ))
        for old, new, error in changes:
            with self.subTest(stated=new):
                _, errors, _ = self.run_checks(self.readme.replace(old, new))
                self.assertEqual(errors, [error])

    def test_rows_and_sentence_must_be_there(self):
        sentence_error = (
            'spec/README.md: under Contents, the sentence of the promise totals '
            '("N in all: N of the layer `language`, N of the layer `convention` and '
            'N of the layer `interface`") appears 0 times; it must appear once'
        )
        changes = (
            (self.rows[1], "",
             "spec/README.md: the Contents table has 0 rows for conventions/b.yaml; "
             "it must have one, stating the file's number of promises"),
            (self.rows[0], self.rows[0]
             + "| [promises/gone.yaml](promises/gone.yaml) | Gone | Draft; 3 promises |\n",
             "spec/README.md: the Contents table has a row for promises/gone.yaml, which is "
             "not a promise file under spec/promises/, spec/conventions/ or spec/interface/"),
            (self.rows[0], self.rows[0] * 2,
             "spec/README.md: the Contents table has 2 rows for promises/a.yaml; "
             "it must have one, stating the file's number of promises"),
            (self.rows[0], self.rows[0].replace("Draft; 2 promises;", "Draft;"),
             "spec/README.md: the Contents row for promises/a.yaml states its number of "
             'promises 0 times; it must state it once, as "N promises"'),
            (self.sentence, "", sentence_error),
            ("## Contents\n", "",
             'spec/README.md: has no line "## Contents", so the promise totals it states '
             'were not compared'),
        )
        for old, new, error in changes:
            with self.subTest(error=error):
                _, errors, _ = self.run_checks(self.readme.replace(old, new))
                self.assertEqual(errors, [error])

    def test_empty_final_state_cell_has_no_promise_count(self):
        for row in (
            "|[promises/a.yaml](promises/a.yaml)|2 promises||\n",
            "[promises/a.yaml](promises/a.yaml)|2 promises||\n",
            "| [promises/a.yaml](promises/a.yaml) | 2 promises | |\n",
            "[promises/a.yaml](promises/a.yaml)|2 promises| |\n",
        ):
            with self.subTest(row=row):
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [
                    "spec/README.md: the Contents row for promises/a.yaml states its number "
                    'of promises 0 times; it must state it once, as "N promises"',
                ])

    def test_balanced_brackets_in_link_label(self):
        for label in ("promises/[a].yaml", "promises/[[a]].yaml"):
            with self.subTest(label=label):
                row = self.rows[0].replace("[promises/a.yaml]", f"[{label}]")
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [])

    def test_balanced_parentheses_in_unknown_link_target(self):
        for path in ("promises/gone(copy).yaml", "promises/gone(copy(2)).yaml"):
            with self.subTest(path=path):
                row = f"| [gone]({path}) | Gone | Draft; 3 promises |\n"
                _, errors, _ = self.run_checks(
                    self.readme.replace(self.rows[0], self.rows[0] + row))
                self.assertEqual(errors, [
                    f"spec/README.md: the Contents table has a row for {path}, which is not "
                    "a promise file under spec/promises/, spec/conventions/ or spec/interface/",
                ])

    def test_link_destination_in_angle_brackets_or_with_title(self):
        for form in ("<{}>", '{} "Title"', "<{}> 'Title'", " {} (Title) ", '{} "a (b"',
                     "{} 'x)'"):
            with self.subTest(form=form):
                row = self.rows[0].replace(
                    "(promises/a.yaml)", "(" + form.format("promises/a.yaml") + ")")
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [])
                row += ("| [gone](" + form.format("promises/gone.yaml")
                        + ") | Gone | Draft; 3 promises |\n")
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [
                    "spec/README.md: the Contents table has a row for promises/gone.yaml, which "
                    "is not a promise file under spec/promises/, spec/conventions/ or "
                    "spec/interface/",
                ])
        path = "promises/gone(.yaml"
        with self.subTest(form="<{}>", path=path):
            row = self.rows[0] + f"| [gone](<{path}>) | Gone | Draft; 3 promises |\n"
            _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
            self.assertEqual(errors, [
                f"spec/README.md: the Contents table has a row for {path}, which is not "
                "a promise file under spec/promises/, spec/conventions/ or spec/interface/",
            ])

    def test_escaped_pipe_in_link_label(self):
        row = self.rows[0].replace("[promises/a.yaml]", "[promises/a.yaml\\|A]")
        _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
        self.assertEqual(errors, [])
        row += "| [Gone\\|old](promises/gone.yaml) | Gone | Draft; 3 promises |\n"
        _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
        self.assertEqual(errors, [
            "spec/README.md: the Contents table has a row for promises/gone.yaml, which is not "
            "a promise file under spec/promises/, spec/conventions/ or spec/interface/",
        ])

    def test_not_run_after_promise_errors(self):
        file = self.root / "spec/conventions/b.yaml"
        file.write_text(file.read_text(encoding="utf-8").replace("convention", "wrong"),
                        encoding="utf-8")
        counts, errors, output = self.run_checks()
        self.assertIsNone(counts)
        self.assertEqual(errors, [
            "spec/conventions/b.yaml promise 1 (B-001): invalid layer 'wrong'",
        ])
        self.assertIn(
            "8. NOT RUN: step 7 found errors in the promise files, so the promise totals "
            "that spec/README.md states were not compared.", output.splitlines())


if __name__ == "__main__":
    unittest.main()
