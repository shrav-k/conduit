"""Exercise the generated standing table on disposable trees."""

from contextlib import redirect_stderr, redirect_stdout
import io
from pathlib import Path
import sys
import tempfile
import unittest

try:
    import yaml
except ImportError:
    raise unittest.SkipTest("PyYAML is needed for the standing fixtures")

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import promise_standing as standing


class PromiseStandingChecks(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.promise_path = "spec/conventions/sample.yaml"
        self.index_path = "conformance/language/index.yaml"
        self.write_promises(1)
        self.write_cases([("a", ["TST-001"])])
        self.write_yaml(standing.MANIFEST, {"s1_coverage": []})
        self.write_yaml(standing.EXPECTED_FAILURES, {"cases": []})
        for directory in standing.SOURCE_DIRS:
            (self.root / directory).mkdir(parents=True)

    def write(self, path, text):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")

    def write_yaml(self, path, data):
        self.write(path, yaml.safe_dump(data, sort_keys=False))

    def write_promises(self, count):
        self.write_yaml(self.promise_path, [
            {"id": f"TST-{n:03}", "layer": "convention", "level": "must"}
            for n in range(1, count + 1)])

    def write_cases(self, cases):
        self.write_yaml(self.index_path, {"cases": [
            {"id": identifier, "promises": promises} for identifier, promises in cases]})

    def run_tool(self, *argv):
        output = io.StringIO()
        with redirect_stdout(output):
            status = standing.main(self.root, list(argv))
        return status, output.getvalue()

    def table(self):
        return (self.root / standing.TABLE).read_text(encoding="utf-8")

    def test_each_standing_follows_its_evidence(self):
        self.write_promises(5)
        # A tag given twice in one case, expected failure or not, counts once, and a promise's
        # first S1 row is its row.
        self.write_cases([("a", ["TST-001"]), ("b", ["TST-002", "TST-003", "TST-003"]),
                          ("c", ["TST-003", "TST-003"])])
        self.write_yaml(standing.EXPECTED_FAILURES, {"cases": [{"id": "b"}]})
        self.write_yaml(standing.MANIFEST, {"s1_coverage": [
            {"promise": "TST-001", "vectors": ["a"]},
            {"promise": "TST-004", "vectors": [], "reason": " No case yet. "},
            {"promise": "TST-004", "vectors": ["a"]}]})
        status, output = self.run_tool("--write")
        self.assertEqual(status, 0, output)
        table = self.table()
        rows = [line for line in table.splitlines() if line.startswith("| TST-")]
        self.assertEqual(rows, [
            "| TST-001 | convention | must | 1 | 0 | cases | none | case passes |",
            "| TST-002 | convention | must | 1 | 1 | outside | none | expected failure |",
            "| TST-003 | convention | must | 2 | 1 | outside | none | case passes |",
            "| TST-004 | convention | must | 0 | 0 | reason | none | reason, no case |",
            "| TST-005 | convention | must | 0 | 0 | outside | none | no case, no reason |"])
        for row in ("| case passes | 0 | 2 | 0 | 2 |", "| expected failure | 0 | 1 | 0 | 1 |",
                    "| reason, no case | 0 | 1 | 0 | 1 |", "| no case, no reason | 0 | 1 | 0 | 1 |",
                    "| all | 0 | 5 | 0 | 5 |", "| named in engine source | 0 | 0 | 0 | 0 |"):
            self.assertIn(row, table.splitlines())

    def test_engine_source_is_named_by_crate(self):
        self.write_promises(4)
        self.write("engine/crates/core/src/a.rs", "// TST-001\n")
        self.write("engine/crates/loader/src/deep/b.rs", 'const ID: &str = "TST-001";\n')
        self.write("engine/crates/core/tests/t.rs", "// TST-002\n")
        self.write("engine/crates/shell/tests/t.rs", "// TST-002\n")
        self.write("engine/crates/shell/examples/e.rs", "// TST-002\n")
        self.write("engine/crates/conformance/src/lib.rs", "// TST-002\n")
        self.write("engine/crates/shell/src/bin/main.rs", "// TST-003\n")
        self.write("engine/crates/core/src/b.rs", "// TST-0040 XTST-004\n")
        status, output = self.run_tool("--write")
        self.assertEqual(status, 0, output)
        rows = {line.split(" | ")[0]: line for line in self.table().splitlines() if line.startswith("| TST-")}
        for identifier, source in (("TST-001", "loader, core"), ("TST-002", "none"),
                                   ("TST-003", "shell"), ("TST-004", "none")):
            self.assertEqual(rows["| " + identifier].split(" | ")[6], source)
        self.assertIn("| named in engine source | 0 | 2 | 0 | 2 |", self.table())

    def test_one_row_per_promise_in_id_order(self):
        (self.root / self.promise_path).unlink()
        for directory, identifier, layer in zip(standing.SPEC_DIRS, ("ZZZ-001", "AAA-002", "AAA-001"),
                                                ("language", "convention", "interface")):
            self.write_yaml(directory + "/sample.yaml", [{"id": identifier, "layer": layer, "level": "must"}])
        status, output = self.run_tool("--write")
        self.assertEqual(status, 0, output)
        table = self.table()
        self.assertEqual([line.split(" | ")[0] for line in table.splitlines()
                          if line.startswith(("| AAA-", "| ZZZ-"))], ["| AAA-001", "| AAA-002", "| ZZZ-001"])
        self.assertIn("| all | 1 | 1 | 1 | 3 |", table)
        self.assertTrue(table.endswith("\n"))
        self.assertFalse(table.endswith("\n\n"))
        self.assertTrue(all(line == line.rstrip() for line in table.splitlines()))
        before = (self.root / standing.TABLE).read_bytes()
        self.assertEqual(self.run_tool("--write")[0], 0)
        self.assertEqual((self.root / standing.TABLE).read_bytes(), before)

    def test_check_passes_after_write_and_reports_a_stale_row(self):
        self.write_promises(30)
        self.write_cases([("a", [f"TST-{n:03}" for n in range(1, 31)])])
        self.assertEqual(self.run_tool("--write")[0], 0)
        before = (self.root / standing.TABLE).read_bytes()
        status, output = self.run_tool("--check")
        self.assertEqual(status, 0, output)
        self.assertEqual(output.splitlines()[-2:], ["engine/STANDING.md is current; promise rows: 30.", "Errors: 0"])
        self.write_yaml(standing.EXPECTED_FAILURES, {"cases": [{"id": "a"}]})
        status, output = self.run_tool("--check")
        self.assertEqual(status, 1, output)
        lines = output.splitlines()
        differs = "engine/STANDING.md: differs from what its inputs give; run python3 tools/promise_standing.py --write and commit it"
        self.assertIn(differs, lines)
        for prefix in ("-| TST-001 |", "+| TST-001 |"):
            self.assertTrue(any(line.startswith(prefix) for line in lines), output)
        # Two hunks with no context: the two changed totals (1 header, 5 lines) and the 30 rows
        # (1 header, 61 lines), after the two file labels: 68 lines, of which 40 are shown.
        start, end = lines.index(differs), lines.index("(28 more lines of difference not shown)")
        self.assertEqual(end - start - 1, 40, output)
        self.assertEqual(lines[start + 1:start + 3], ["--- engine/STANDING.md (committed)",
                                                      "+++ engine/STANDING.md (generated)"])
        self.assertTrue(all(line.startswith(("---", "+++", "@@", "-", "+")) for line in lines[start + 1:end]), output)
        self.assertEqual(lines[end + 1:], ["Errors: 1"])
        self.assertEqual((self.root / standing.TABLE).read_bytes(), before)
        self.write_yaml(standing.EXPECTED_FAILURES, {"cases": []})
        line_ends = "engine/STANDING.md: every line matches the generated table; the bytes differ only in line ends or the final newline (the table is written with LF line ends and exactly one final newline)"
        for changed in (before.replace(b"\n", b"\r\n"), before[:-1]):
            with self.subTest(changed=changed[-2:]):
                (self.root / standing.TABLE).write_bytes(changed)
                status, output = self.run_tool("--check")
                self.assertEqual(status, 1, output)
                lines = output.splitlines()
                self.assertEqual(lines[lines.index(differs) + 1:], [line_ends, "Errors: 1"])
                self.assertEqual((self.root / standing.TABLE).read_bytes(), changed)
        (self.root / standing.TABLE).unlink()
        status, output = self.run_tool("--check")
        self.assertEqual(status, 1, output)
        self.assertIn("engine/STANDING.md: not found; run python3 tools/promise_standing.py --write and commit it", output)

    def test_unreadable_input_names_its_file_and_writes_nothing(self):
        self.assertEqual(self.run_tool("--write")[0], 0)
        before = (self.root / standing.TABLE).read_bytes()
        no_s1_evidence = "S1 row 1 (TST-001) has neither cases nor a reason; provide nonempty vectors or a reason"
        faults = [(self.index_path, "cases: 3\n",
                   "document's cases is a number, not a list; make cases a list of case entries"),
                  (standing.EXPECTED_FAILURES, "[]\n",
                   "document is a list, not a mapping; make it a mapping whose cases is a list of "
                   "entries, each a mapping with a case's id"),
                  (self.promise_path, "[unclosed\n", None),
                  (standing.MANIFEST, "s1_coverage:\n- promise: TST-001\n  vectors: []\n", no_s1_evidence),
                  (standing.MANIFEST, "s1_coverage:\n- promise: TST-001\n  vectors: []\n  reason: '  '\n",
                   no_s1_evidence),
                  (standing.SOURCE_DIRS[2], None, "not found, so no source of the crate was read")]
        # Other malformed forms must fail with a file diagnostic, never a traceback.
        faults.extend([(self.index_path, "{}\n", "document has no cases; add cases, a list of case entries"),
                       (self.index_path, "cases: [3]\n",
                        "case 1 is a number, not a mapping; make each case a mapping with an id and a promises list"),
                       (self.index_path, "cases: [{id: a, promises: TST-001}]\n",
                        "case 1's promises is a string, not a list; make promises a list of promise IDs"),
                       (self.index_path, "cases: [{id: a, promises: [3]}]\n",
                        "case 1's promise 1 is a number, not a string; write each promise ID as a string"),
                       (self.promise_path, "- {id: TST-001, layer: wrong, level: must}\n",
                        "promise 1's layer is 'wrong', which is not a layer; make layer language, convention or interface"),
                       (self.promise_path, "- {id: TST-001, layer: convention}\n",
                        "promise 1 has no level; add level, a string naming the promise's level"),
                       (self.promise_path, "- {id: TST-001, layer: convention, level: must}\n" * 2,
                        f"promise ID TST-001 also occurs in {Path(self.promise_path)}; give each promise a unique ID"),
                       (standing.EXPECTED_FAILURES, None,
                        "not found; the conformance runner requires it (work order 163)"),
                       (standing.MANIFEST, None, "not found, so no S1 row was read")])
        for path, replacement, message in faults:
            with self.subTest(path=path, replacement=replacement):
                target = self.root / path
                original = target.read_bytes() if target.is_file() else None
                try:
                    if replacement is None:
                        target.rmdir() if target.is_dir() else target.unlink()
                    else:
                        self.write(path, replacement)
                    for mode, action in (("--write", "written"), ("--check", "compared")):
                        status, output = self.run_tool(mode)
                        self.assertEqual(status, 1, output)
                        self.assertTrue(any(line.startswith(path + ": ") for line in output.splitlines()), output)
                        if message is not None:
                            self.assertIn(f"{path}: {message}", output.splitlines())
                        self.assertIn(f"engine/STANDING.md was not {action}.", output.splitlines())
                        self.assertRegex(output.splitlines()[-1], r"^Errors: [1-9][0-9]*$")
                        self.assertEqual((self.root / standing.TABLE).read_bytes(), before)
                finally:
                    if original is None:
                        target.mkdir(parents=True, exist_ok=True)
                    else:
                        target.write_bytes(original)
        self.write(self.promise_path, "- id: TST-001\n  layer: [unclosed\n")
        status, output = self.run_tool("--write")
        self.assertEqual(status, 1, output)
        self.assertRegex(output, r"(?m)^spec/conventions/sample\.yaml: could not parse YAML at line [0-9]+, "
                                 r"column [0-9]+ \([^\n]+\); correct the YAML there$")
        self.write_promises(1)
        source = self.root / standing.SOURCE_DIRS[2] / "bad.rs"
        source.write_bytes(b"\xff")
        status, output = self.run_tool("--write")
        self.assertEqual(status, 1, output)
        self.assertIn(source.relative_to(self.root).as_posix() + ": ", output)
        self.assertEqual((self.root / standing.TABLE).read_bytes(), before)

    def test_one_mode_is_required(self):
        for argv in ([], ["--write", "--check"]):
            with self.subTest(argv=argv), redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as raised:
                    self.run_tool(*argv)
                self.assertEqual(raised.exception.code, 2)
                self.assertFalse((self.root / standing.TABLE).exists())


if __name__ == "__main__":
    unittest.main()
