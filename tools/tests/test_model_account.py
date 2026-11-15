"""Exercise the model account gate on small disposable trees."""

from contextlib import redirect_stdout
import builtins
import copy
import hashlib
import io
from pathlib import Path, PureWindowsPath
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

try:
    import yaml
except ImportError:
    raise unittest.SkipTest("PyYAML is needed for the model account fixtures")

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_model_account as model_account


class ModelAccountChecks(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for directory in model_account.SPEC_DIRS:
            (self.root / directory).mkdir(parents=True)
        self.promise_path = "spec/promises/sample.yaml"
        self.promise = {
            "id": "TST-001", "layer": "convention", "level": "must",
            "when": "A request arrives.", "then": "The effect occurs once.",
            "observable": "Inspect café.",
        }
        # Independent canonical bytes pin field selection, UTF-8 and JSON separators.
        canonical = ('{"level":"must","observable":"Inspect café.",'
                     '"then":"The effect occurs once.","when":"A request arrives."}')
        self.digest = hashlib.sha256(canonical.encode("utf-8")).hexdigest()
        self.row = {
            "id": "TST-001", "class": "safety", "standing": "holds, not graded",
            "model": "sample::P", "read": "recorded-commit", "digest": self.digest,
            "note": "A bounded slice.",
        }
        self.write_promises()
        self.write_account()
        self.write(model_account.CATALOGUE,
                   "## Safety promises\n\n| TST-001 | convention |\n\n## The others\n")
        self.write_yaml(model_account.MANIFEST, {"cases": [{"promises": ["TST-001"]}]})
        self.write(model_account.MODEL_DIR + "/sample.rs", "// Fixture module.\n")

    def write(self, path, text):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")

    def write_yaml(self, path, data):
        self.write(path, yaml.safe_dump(data, allow_unicode=True, sort_keys=False))

    def write_promises(self):
        self.write_yaml(self.promise_path, [self.promise])

    def write_account(self, rows=None):
        self.write_yaml(model_account.ACCOUNT, [self.row] if rows is None else rows)

    def run_check(self, *argv):
        output = io.StringIO()
        with redirect_stdout(output):
            status = model_account.main(self.root, list(argv))
        return status, output.getvalue()

    def assert_failure(self, fragment, identifier="TST-001", *argv):
        status, output = self.run_check(*argv)
        self.assertEqual(status, 1, output)
        self.assertTrue(any(identifier in line and fragment in line for line in output.splitlines()), output)
        self.assertRegex(output.splitlines()[-1], r"^Errors: [1-9][0-9]*$")

    def mark_changed(self):
        self.row["was"] = self.row["standing"]
        self.row["standing"] = "text changed, not rerun"

    def git(self, *args):
        return subprocess.run(["git", *args], cwd=self.root, check=True,
                              capture_output=True, text=True).stdout.strip()

    def commit(self):
        self.git("add", ".")
        self.git("-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
                 "-c", "commit.gpgsign=false", "commit", "-qm", "Fixture")
        return self.git("rev-parse", "HEAD")

    def two_commits(self):
        self.git("init", "-q")
        self.row["read"] = self.commit()
        # The history lookup must find the ID even when it moves directories and suffixes.
        (self.root / self.promise_path).unlink()
        self.promise_path = "spec/conventions/moved.yml"
        self.promise["then"] = "The effect occurs twice."
        self.write_promises()
        self.commit()
        self.mark_changed()
        self.write_account()

    def test_clean_tree_passes(self):
        status, output = self.run_check()
        self.assertEqual(status, 0, output)
        self.assertIn("Rows: 1 (safety 1, not safety 0, uncatalogued 0)", output)
        self.assertTrue(output.endswith("Errors: 0\n"))
        self.assertEqual(model_account.digest(self.promise), self.digest)

    def test_changed_then_requires_rerun_or_explicit_mark(self):
        self.promise["then"] = "The effect occurs twice."
        self.write_promises()
        self.assert_failure("rerun the model or mark the row text changed, not rerun")
        self.mark_changed()
        self.write_account()
        status, output = self.run_check()
        self.assertEqual(status, 0, output)
        self.assertIn("Text changed, not rerun by was:\n  holds, not graded: 1", output)

    def test_same_words_folded_differently_keep_digest(self):
        self.write(self.promise_path, """- id: TST-001
  layer: convention
  observable: Inspect café.
  then: >-
    The effect
    occurs once.
  when: >-
    A request
    arrives.
  level: must
""")
        self.assertEqual(self.run_check()[0], 0)

    def test_level_observable_and_when_also_detect_staleness(self):
        original = copy.deepcopy(self.promise)
        for field in ("level", "observable", "when"):
            with self.subTest(field=field):
                self.promise = dict(original, **{field: "Changed"})
                self.write_promises()
                self.assert_failure("promise text differs")

    def test_missing_row_fails(self):
        self.write_account([])
        self.assert_failure("has no account row")

    def test_unknown_promise_fails(self):
        self.write_account([self.row, {"id": "TST-999", "class": "uncatalogued"}])
        self.assert_failure("not a convention-layer promise", "TST-999")

    def test_duplicate_row_fails(self):
        self.write_account([self.row, self.row])
        self.assert_failure("second row")

    def test_contradicted_class_fails(self):
        self.row["class"] = "not safety"
        self.write_account()
        self.assert_failure("class must be 'safety'")

    def test_unknown_standing_fails(self):
        self.row["standing"] = "holds"
        self.write_account()
        self.assert_failure("unknown standing")

    def test_standing_keys_and_was_are_checked(self):
        original = dict(self.row)
        cases = [
            ({key: value for key, value in original.items() if key != "read"}, "missing required key read"),
            (dict(original, outside=[]), "key outside is not allowed"),
            (dict(original, was="static"), "was must be a model-run standing"),
            (dict(original, standing="text changed, not rerun"), "missing required key was"),
            (dict(original, standing="not modelled"), "key digest is not allowed"),
            ({"id": "TST-001", "class": "safety", "standing": "static"}, "missing required key note"),
        ]
        for row, fragment in cases:
            with self.subTest(fragment=fragment):
                self.write_account([row])
                self.assert_failure(fragment)

    def test_model_file_and_outside_evidence_are_checked(self):
        self.row["model"] = "missing::P, Q"
        self.write_account()
        self.assert_failure("model module missing has no file")
        self.row = {"id": "TST-001", "class": "safety", "standing": "static",
                    "model": "sample::P, Q", "note": "One step.", "outside": ["evidence.txt"]}
        self.write_account()
        self.assert_failure("outside path 'evidence.txt' does not exist")
        self.write("evidence.txt", "Fixture evidence.\n")
        self.assertEqual(self.run_check()[0], 0)
        # A rerun may register a primed property, such as N'.
        self.row["model"] = "sample::P'"
        self.write_account()
        self.assertEqual(self.run_check()[0], 0)
        # A prime may follow a key but not lead it.
        self.row["model"] = "sample::'P"
        self.write_account()
        self.assert_failure("model must be module::key")

    def test_static_evidence_counts_manifest_and_outside_separately(self):
        self.row = {"id": "TST-001", "class": "safety", "standing": "static", "note": "One step."}
        self.write_account()
        self.assertIn("no outside evidence: 0", self.run_check()[1])
        self.write_yaml(model_account.MANIFEST, {"cases": []})
        self.assertIn("no outside evidence: 1", self.run_check()[1])
        self.row["outside"] = ["evidence.txt"]
        self.write("evidence.txt", "Fixture evidence.\n")
        self.write_account()
        self.assertIn("no outside evidence: 0", self.run_check()[1])

    def test_other_classes_and_only_direct_convention_promises(self):
        self.write_yaml("spec/interface/more.yml", [
            {"id": "TST-002", "layer": "convention"}, {"id": "TST-003", "layer": "convention"},
            {"id": "TST-004", "layer": "interface"},
        ])
        self.write_yaml("spec/promises/nested/ignored.yaml", [{"id": "TST-005", "layer": "convention"}])
        self.write_yaml("spec/conventions/mapping.yaml", {"id": "TST-006", "layer": "convention"})
        self.write(model_account.CATALOGUE, "## Safety promises\n| TST-001 | convention |\n"
                   "## The others\n| TST-002 | convention |\n")
        self.write_account([self.row, {"id": "TST-002", "class": "not safety"},
                            {"id": "TST-003", "class": "uncatalogued"}])
        status, output = self.run_check()
        self.assertEqual(status, 0, output)
        self.assertIn("Rows: 3 (safety 1, not safety 1, uncatalogued 1)", output)
        self.assertIn("Uncatalogued IDs: TST-003", output)

    def test_history_accepts_digest_of_text_at_read(self):
        self.two_commits()
        status, output = self.run_check("--history")
        self.assertEqual(status, 0, output)
        self.assertTrue(output.endswith("Errors: 0\n"))

    def test_history_rejects_digest_of_later_text(self):
        self.two_commits()
        self.row["digest"] = model_account.digest(self.promise)
        self.write_account()
        self.assert_failure("digest differs from promise text at read", "TST-001", "--history")

    def test_history_rejects_promise_absent_at_read(self):
        self.git("init", "-q")
        self.write_yaml(self.promise_path, [])
        self.row["read"] = self.commit()
        self.write_promises()
        self.write_account()
        self.assert_failure("promise absent at read", "TST-001", "--history")

    def test_history_ignores_nested_promise_at_read(self):
        # Only files directly inside the three directories hold promises, at read as now.
        self.git("init", "-q")
        (self.root / self.promise_path).unlink()
        self.write_yaml("spec/promises/nested/x.yaml", [self.promise])
        self.row["read"] = self.commit()
        (self.root / "spec/promises/nested/x.yaml").unlink()
        self.write_promises()
        self.write_account()
        self.assert_failure("promise absent at read", "TST-001", "--history")

    def test_promise_files_reads_windows_paths_and_skips_nested(self):
        first = PureWindowsPath("spec\\promises\\x.yaml")
        nested = PureWindowsPath("spec\\promises\\nested\\y.yaml")
        self.assertEqual(model_account.promise_files([first, nested]), [first])

    def test_missing_pyyaml_in_main_checks_nothing_without_reexec(self):
        with patch.object(model_account, "yaml", None), patch.object(model_account.os, "execv") as reexec:
            status, output = self.run_check()
        self.assertEqual(status, 2)
        self.assertIn("NOTHING WAS CHECKED", output)
        reexec.assert_not_called()

    def test_missing_pyyaml_launcher_retries_only_once(self):
        script = Path(model_account.__file__).resolve()
        original_import = builtins.__import__

        def without_yaml(name, *args, **kwargs):
            if name == "yaml":
                raise ImportError("PyYAML deliberately unavailable")
            return original_import(name, *args, **kwargs)

        output = io.StringIO()
        with (patch.object(builtins, "__import__", side_effect=without_yaml),
              patch.object(sys, "executable", str(self.root / "launcher-python3")),
              patch.object(sys, "argv", [str(script), "--history"]),
              patch.object(model_account.os.path, "exists", return_value=True),
              patch.dict(model_account.os.environ, {"CONDUIT_CHECK_MODEL_ACCOUNT_REEXEC": "0"}),
              patch.object(model_account.os, "execv", side_effect=RuntimeError("fallback restart")) as reexec,
              redirect_stdout(output)):
            with self.assertRaisesRegex(RuntimeError, "fallback restart"):
                runpy.run_path(str(script), run_name="__main__")
            # The launcher's executable path still differs after the restart.
            with self.assertRaises(SystemExit) as stopped:
                runpy.run_path(str(script), run_name="__main__")
            self.assertEqual(stopped.exception.code, 2)
            reexec.assert_called_once_with("/usr/bin/python3", ["/usr/bin/python3", str(script), "--history"])
        self.assertEqual(output.getvalue(), "PyYAML is needed. NOTHING WAS CHECKED.\n")


if __name__ == "__main__":
    unittest.main()
