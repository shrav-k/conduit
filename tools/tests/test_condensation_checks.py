#!/usr/bin/env python3
"""Regression checks for the offline guards used by the condensation work orders."""

import contextlib
import copy
import io
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
from urllib.parse import urlsplit

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import check_docs as docs
import check_schemas as schemas


class CondensationChecks(unittest.TestCase):
    def test_removed_registers_exclude_destinations_and_later_additions(self):
        head = """# CON-005 was removed as a duplicate of PLC-006, and CON-009 was folded
# into LIF-007. Removed by consolidation pass 1: CON-010 was folded into LIF-008,
# and CON-015 into LIF-001, LIF-003.
# Ruling 41 removed TPL-006 and TPL-007 (decision 2: PRV-007), TPL-015
# (decision 2a: PRV-009) and TPL-016 (decision 1); changed TPL-001;
# and added TPL-017, and TPL-019, one item that is run.
# Removed IDs: OLD-001 to OLD-003, OLD-005 (folded into NEW-001).
# Ruling 41 removed PLC-003, with default placement (PLC-002), and PLC-021,
# with facts which PLC-026 replaces.
# Ruling 41 removed CON-029, on composite templates; CON-026, on renaming;
# and CON-030, on requirements; changed CON-001; and added CON-031.
# Removed: OLD-007 → NEW-002 (ruling 32).
# OLD-008: nothing replaces it (ruling 41).
"""
        self.assertEqual(schemas.retired_ids(head), {
            "CON-005", "CON-009", "CON-010", "CON-015", "TPL-006", "TPL-007",
            "TPL-015", "TPL-016", "OLD-001", "OLD-002", "OLD-003", "OLD-005",
            "PLC-003", "PLC-021", "CON-026", "CON-029", "CON-030", "OLD-007", "OLD-008",
        })

    def test_promise_errors_are_all_reported_across_layers_and_registers(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            record = {"id": "OLD-002", "layer": "language", "level": "must",
                      "when": "always", "then": "holds", "observable": "visible", "vectors": []}
            for name in ("promises", "conventions", "interface"):
                (root / "spec" / name).mkdir(parents=True)
            (root / "spec/README.md").write_text(
                "### Promise files that rulings removed\n"
                "| `old.yaml` | OLD-001 to OLD-003. Replaced by LIVE-001. | Ruling |\n"
                "## Next\n")
            (root / "spec/promises/a.yaml").write_text(schemas.yaml.safe_dump([record]))
            broken = dict(record, layer="wrong", level="wrong", extra=True)
            del broken["when"]
            (root / "spec/conventions/b.yaml").write_text(schemas.yaml.safe_dump([broken]))
            reused = dict(record, id="HEAD-001")
            (root / "spec/interface/c.yaml").write_text(
                "# Removed IDs: HEAD-001 (folded into LIVE-001).\n"
                + schemas.yaml.safe_dump([reused, copy.deepcopy(dict(record, id="LIVE-001"))]))
            errors = []
            with patch.object(schemas, "ROOT", root), patch.object(schemas, "errors", errors), \
                    contextlib.redirect_stdout(io.StringIO()):
                schemas.check_promises()
            self.assertEqual(len(errors), 6, errors)
            for expected in ("fields differ", "invalid layer", "invalid level", "duplicate ID",
                             "OLD-002): live ID is retired", "HEAD-001): live ID is retired"):
                self.assertTrue(any(expected in error for error in errors), (expected, errors))

    @contextlib.contextmanager
    def permalink_repository(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)

            def git(*args):
                return subprocess.run(["git", *args], cwd=root, check=True,
                                      stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout.decode()

            git("init", "--quiet")
            (root / "page.md").write_text("# Original heading\n# Original heading\n<a id='explicit'></a>\n")
            (root / "source").mkdir()
            (root / "source/code.rs").write_text("// line\n" * 215 + "// last line")
            git("add", "page.md", "source")
            git("-c", "user.name=Check", "-c", "user.email=check@example.invalid",
                "-c", "commit.gpgsign=false", "commit", "--quiet", "-m", "fixture")
            sha = git("rev-parse", "HEAD").strip()
            (root / "page.md").write_text("# Working tree heading\n")
            base = f"https://github.com/shrav-k/conduit/blob/{sha}/"
            yield root, base

    def test_permalinks_use_committed_content_and_skip_absent_history(self):
        with self.permalink_repository() as (root, base):
            with patch.object(docs, "ROOT", root):
                checker = docs.Permalinks()
                for anchor in ("original-heading", "original-heading-1", "explicit"):
                    self.assertIsNone(checker.check(urlsplit(base + "page.md#" + anchor)))
                self.assertEqual(checker.check(urlsplit(base + "absent.md")), "missing permalink path")
                self.assertEqual(checker.check(urlsplit(base + "page.md#working-tree-heading")),
                                 "missing permalink anchor")
                self.assertIsNone(checker.check(urlsplit(
                    base.replace("/blob/", "/tree/") + "page.md#original-heading")))
                with contextlib.redirect_stdout(io.StringIO()) as output:
                    self.assertIsNone(checker.check(urlsplit(
                        base.rsplit("/", 2)[0] + "/" + "0" * 40 + "/page.md")))
                self.assertIn("Notice: skipped", output.getvalue())
            with tempfile.TemporaryDirectory() as outside, patch.object(docs, "ROOT", Path(outside)):
                with contextlib.redirect_stdout(io.StringIO()) as output:
                    self.assertIsNone(docs.Permalinks().check(urlsplit(base + "page.md")))
                self.assertIn("Notice: skipped", output.getvalue())

    def test_permalink_line_anchors_use_committed_file_length(self):
        with self.permalink_repository() as (root, base), patch.object(docs, "ROOT", root):
            checker = docs.Permalinks()
            for anchor in ("L1", "L3", "L1-L3", "L3-L3"):
                with self.subTest(anchor=anchor):
                    self.assertIsNone(checker.check(urlsplit(base + "page.md#" + anchor)))
            for anchor in ("L0", "L4", "L1-L4", "L3-L1", "L0-L3"):
                with self.subTest(anchor=anchor):
                    self.assertEqual(checker.check(urlsplit(base + "page.md#" + anchor)),
                                     "missing permalink anchor")
            # A tree has no file lines, even though git show prints its entries.
            self.assertEqual(checker.check(urlsplit(base + "source#L1")), "missing permalink anchor")
            for anchor in ("L211", "L211-L216"):
                self.assertIsNone(checker.check(urlsplit(base + "source/code.rs#" + anchor)))
            self.assertEqual(checker.check(urlsplit(base + "source/code.rs#L211-L217")),
                             "missing permalink anchor")

    def test_markdown_autolinks_check_permalink_paths_and_anchors(self):
        with self.permalink_repository() as (root, base):
            page = root / "links.md"
            page.write_text(f"<{base}absent.md>\n<{base}page.md#missing-heading>\n"
                            f"<{base}page.md#original-heading>\n<{base}page.md#L1-L3>\n"
                            f"[inline](<{base}page.md>)\n[reference]: <{base}page.md>\n")
            with patch.object(docs, "ROOT", root), \
                    patch.object(docs, "from_git", return_value=({page}, None)), \
                    patch.object(docs, "path_mentions", return_value=[]), \
                    contextlib.redirect_stdout(io.StringIO()) as output, \
                    contextlib.redirect_stderr(io.StringIO()) as errors:
                self.assertTrue(docs.check())
            self.assertEqual(errors.getvalue().splitlines(), [
                f"links.md:1: missing permalink path {base}absent.md",
                f"links.md:2: missing permalink anchor {base}page.md#missing-heading",
            ])
            self.assertIn("Checked 6 repository permalinks", output.getvalue())
            self.assertTrue(output.getvalue().endswith("Errors: 2\n"))

    def test_path_mentions_report_literals_but_skip_templates_and_protocol(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in ("tools", "conformance", "engine", "spec"):
                (root / name).mkdir()
            (root / "spec/exists.yaml").touch()
            (root / "tools/check.py").write_text(
                '# spec/exists.yaml docs/missing.md spec/{dynamic}/file.yaml\n'
                '# https://example.org/docs/external.md #/spec/pointer\n')
            (root / "conformance/manifest.yaml").write_text(
                '{\n  "case": "missing-case.yaml"\n}')
            (root / "conformance/language").mkdir()
            (root / "conformance/language/index.yaml").write_text(
                '{\n  "cases": [\n    {\n      "case": "missing-piece-case.yaml"\n    }\n  ]\n}')
            (root / "engine/main.rs").write_text(
                '// conformance/missing.yaml tools/call tools/list\n')
            with patch.object(docs, "ROOT", root), contextlib.redirect_stdout(io.StringIO()):
                errors = docs.path_mentions()
            self.assertEqual(len(errors), 4, errors)
            self.assertTrue(any("missing case path missing-case.yaml" in e for e in errors))
            self.assertTrue(any("missing case path missing-piece-case.yaml" in e for e in errors))
            self.assertTrue(any("missing mentioned path docs/missing.md" in e for e in errors))
            self.assertTrue(any("missing mentioned path conformance/missing.yaml" in e for e in errors))


if __name__ == "__main__":
    unittest.main()
