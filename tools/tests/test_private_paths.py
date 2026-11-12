"""Exercise the private-path gate on disposable Git trees, without exposing values."""

from contextlib import redirect_stdout
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import check_private_paths as private_paths


class PrivatePathChecks(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)
        self.environment = patch.dict(os.environ, {"CONDUIT_PRIVATE_NAMES": ""})
        self.environment.start()
        self.addCleanup(self.environment.stop)

    def track(self, path, data):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(data)
        subprocess.run(["git", "add", "--", path], cwd=self.root, check=True)

    def run_check(self):
        output = io.StringIO()
        with redirect_stdout(output):
            status = private_paths.main(self.root)
        return status, output.getvalue()

    def test_clean_tree_is_green_and_untracked_file_is_ignored(self):
        self.track("README.md", b"A clean tree.\n")
        (self.root / "scratch").write_bytes(b"/Users/" + b"synthetic/private")
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_planted_paths_are_red_even_in_binary_files(self):
        paths = [b"/Users/" + b"synthetic/private", b"/home/" + b"synthetic",
                 b"/private/" + b"tmp/session/file", b"/tmp/" + b"claude-session/file",
                 b".codex/" + b"auth.json"]
        for index, path in enumerate(paths):
            with self.subTest(index=index):
                self.track("planted.bin", b"\0first line\n" + path)
                status, output = self.run_check()
                self.assertEqual(status, 1)
                self.assertIn("planted.bin:2:", output)
                self.assertNotIn(path.decode(), output)

    def test_planted_name_from_variable_is_red_and_literal(self):
        self.track("names.txt", b"first line\nSynthetic.Name+one\n")
        with patch.dict(os.environ, {"CONDUIT_PRIVATE_NAMES": " ,absent, Synthetic.Name+one, "}):
            status, output = self.run_check()
        self.assertEqual(status, 1)
        self.assertIn("names.txt:2: private name from CONDUIT_PRIVATE_NAMES", output)
        self.assertNotIn("Synthetic.Name+one", output)
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_working_tree_edit_is_checked_without_staging_it(self):
        self.track("edited.txt", b"clean\n")
        (self.root / "edited.txt").write_bytes(b"/home/" + b"synthetic/work")
        self.assertEqual(self.run_check()[0], 1)

    def test_only_the_listed_fixture_user_is_allowed(self):
        fake_home = b"/Users/" + b"u"
        self.track(private_paths.FIXTURE, fake_home + b"/x/exp\n" + fake_home)
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))
        with patch.dict(os.environ, {"CONDUIT_PRIVATE_NAMES": "exp"}):
            self.assertEqual(self.run_check()[0], 1)
        self.track(private_paths.FIXTURE, b"/Users/" + b"other/x")
        self.assertEqual(self.run_check()[0], 1)
        self.track(private_paths.FIXTURE, b"/home/" + b"u/x")
        self.assertEqual(self.run_check()[0], 1)
        self.track(private_paths.FIXTURE, b"clean")
        self.track("elsewhere.rs", fake_home + b"/x")
        self.assertEqual(self.run_check()[0], 1)

    def test_search_patterns_placeholders_and_documentation_are_green(self):
        self.track("order.md", b"grep -nE '/Users/" + b"[A-Za-z]|/home/" + b"[a-z]'\n"
                   b"Codex keeps its login in ~/.codex/" + b"auth.json.\n")
        self.track("tests.rs", b'("HOME", "/home/' + b'test")\n')
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))
        self.track("config.toml", b"path = \"~/.codex/" + b"auth.json\"\n")
        self.assertEqual(self.run_check()[0], 1)

    def test_empty_git_tree_is_green(self):
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_symlink_target_is_checked_without_following_it(self):
        target = "/home/" + "synthetic/absent"
        (self.root / "link").symlink_to(target)
        subprocess.run(["git", "add", "--", "link"], cwd=self.root, check=True)
        status, output = self.run_check()
        self.assertEqual(status, 1)
        self.assertIn("link:1: home directory", output)
        self.assertNotIn("cannot read", output)

    def test_missing_tracked_file_is_an_error(self):
        self.track("missing", b"clean")
        (self.root / "missing").unlink()
        self.assertEqual(self.run_check(), (1, "missing: cannot read tracked file\nErrors: 1\n"))


if __name__ == "__main__":
    unittest.main()
