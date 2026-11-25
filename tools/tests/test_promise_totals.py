#!/usr/bin/env python3
"""Regression checks for computed promise totals and README counts."""

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
                 "then": "holds", "observable": "visible"}
                for id_, layer in promises
            ]), encoding="utf-8")
        self.rows = (
            "| [promises/a.yaml](promises/a.yaml) | A | "
            "Draft |\n",
            "| [conventions/b.yaml](conventions/b.yaml) | B | "
            "Draft |\n",
            "| [interface/c.yaml](interface/c.yaml) | C | Draft |\n",
        )
        self.readme = (
            "## Contents\n\n| Path | Part | State |\n| --- | --- | --- |\n"
            + "".join(self.rows)
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

    def test_stored_count_absent_prints_totals(self):
        counts, errors, output = self.run_checks()
        self.assertEqual(errors, [])
        self.assertEqual(counts, (
            {"promises/a.yaml": 2, "conventions/b.yaml": 1, "interface/c.yaml": 1},
            {"language": 1, "convention": 2, "interface": 1},
        ))
        self.assertIn(
            "8. Counted 4 promises in 3 files: 1 of the layer `language`, "
            "2 of the layer `convention` and 1 of the layer `interface`; "
            "spec/README.md's Contents has a row for each, and the three READMEs "
            "state no count; 0 errors.", output.splitlines())

    def test_stored_count_fails(self):
        for path, text, match in (
            ("spec/README.md", self.readme.replace("Draft", "Draft; 2 promises", 1), "2 promises"),
            ("spec/README.md", self.readme + "4 in all:\n", "4 in all"),
            ("spec/README.md", self.readme + "1 of the layer\n", "1 of the layer"),
            ("spec/core/README.md", "# Core\nThe 40 Operation contracts:\n", "40 Operation contracts"),
            ("spec/schemas/README.md", "# Examples\n| 77 files, each\n", "77 files"),
        ):
            with self.subTest(path=path, match=match):
                file = self.root / path
                if path != "spec/README.md":
                    file.parent.mkdir(parents=True, exist_ok=True)
                    file.write_text(text, encoding="utf-8")
                _, errors, _ = self.run_checks(text if path == "spec/README.md" else None)
                line = text.count("\n", 0, text.index(match)) + 1
                self.assertEqual(errors, [
                    f'{path}:{line}: states the count "{match}"; '
                    'the checkers give counts instead (DEC-177)',
                ])
                if path != "spec/README.md":
                    file.unlink()

    def test_stored_count_ignores_modes_and_names(self):
        _, errors, _ = self.run_checks(self.readme + "\n" + "\n".join((
            "mode-0600 files", "0700 directory with 0600 files", "each S1 promise's",
            "DEC-177 promises", "the 0.2.1 contracts",
        )))
        self.assertEqual(errors, [])

    def test_rows_must_be_there(self):
        changes = (
            (self.rows[1], "",
             "spec/README.md: the Contents table has 0 rows for conventions/b.yaml; "
             "it must have one"),
            (self.rows[0], self.rows[0]
             + "| [promises/gone.yaml](promises/gone.yaml) | Gone | Draft |\n",
             "spec/README.md: the Contents table has a row for promises/gone.yaml, which is "
             "not a promise file under spec/promises/, spec/conventions/ or spec/interface/"),
            (self.rows[0], self.rows[0] * 2,
             "spec/README.md: the Contents table has 2 rows for promises/a.yaml; "
             "it must have one"),
            ("## Contents\n", "",
             'spec/README.md: has no line "## Contents", so its promise-file rows '
             'were not checked'),
        )
        for old, new, error in changes:
            with self.subTest(error=error):
                _, errors, _ = self.run_checks(self.readme.replace(old, new))
                self.assertEqual(errors, [error])

    def test_balanced_brackets_in_link_label(self):
        for label in ("promises/[a].yaml", "promises/[[a]].yaml"):
            with self.subTest(label=label):
                row = self.rows[0].replace("[promises/a.yaml]", f"[{label}]")
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [])

    def test_balanced_parentheses_in_unknown_link_target(self):
        for path in ("promises/gone(copy).yaml", "promises/gone(copy(2)).yaml"):
            with self.subTest(path=path):
                row = f"| [gone]({path}) | Gone | Draft |\n"
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
                        + ") | Gone | Draft |\n")
                _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
                self.assertEqual(errors, [
                    "spec/README.md: the Contents table has a row for promises/gone.yaml, which "
                    "is not a promise file under spec/promises/, spec/conventions/ or "
                    "spec/interface/",
                ])
        path = "promises/gone(.yaml"
        with self.subTest(form="<{}>", path=path):
            row = self.rows[0] + f"| [gone](<{path}>) | Gone | Draft |\n"
            _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
            self.assertEqual(errors, [
                f"spec/README.md: the Contents table has a row for {path}, which is not "
                "a promise file under spec/promises/, spec/conventions/ or spec/interface/",
            ])

    def test_escaped_pipe_in_link_label(self):
        row = self.rows[0].replace("[promises/a.yaml]", "[promises/a.yaml\\|A]")
        _, errors, _ = self.run_checks(self.readme.replace(self.rows[0], row))
        self.assertEqual(errors, [])
        row += "| [Gone\\|old](promises/gone.yaml) | Gone | Draft |\n"
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
            "8. NOT RUN: step 7 found errors in the promise files, so promise totals, "
            "Contents rows and stored README counts were not checked.", output.splitlines())


if __name__ == "__main__":
    unittest.main()
