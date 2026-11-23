"""Exercise the private-path gate on disposable Git trees, without exposing values."""

from contextlib import redirect_stdout
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
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

    def test_contributors_entries_may_hold_an_address(self):
        address = b'listed@' + b'mail.synthetic'
        entry = b'Listed Synthetic <' + address + b'>\n'
        self.track('CONTRIBUTORS', b'# Header\n' + entry)
        with self.subTest(part='entry'):
            self.assertEqual(self.run_check(), (0, 'Errors: 0\n'))
        for data in (b'# ' + entry, entry.rstrip() + address + b'\n',
                     address + b' <safe@example.invalid>\n',
                     b' Listed <' + address + b'>\n',
                     b'Listed\tSynthetic <' + address + b'>\n'):
            with self.subTest(part='other position', data=data):
                self.track('CONTRIBUTORS', data)
                code, output = self.run_check()
                self.assertEqual(code, 1)
                self.assertNotIn(address.decode(), output)
        self.track('CONTRIBUTORS', entry)
        for path in ('other.txt', 'nested/CONTRIBUTORS'):
            with self.subTest(path=path):
                self.track(path, entry)
                self.assertEqual(self.run_check()[0], 1)
                self.track(path, b'clean\n')
        with patch.dict(os.environ, {'CONDUIT_PRIVATE_NAMES': 'Listed Synthetic'}):
            self.assertIn('private name from CONDUIT_PRIVATE_NAMES', self.run_check()[1])
        with patch.dict(os.environ, {'CONDUIT_PRIVATE_NAMES': 'mail.synthetic'}):
            self.assertIn('private name from CONDUIT_PRIVATE_NAMES', self.run_check()[1])
        target = self.root / 'CONTRIBUTORS'
        target.unlink()
        target.symlink_to(entry.decode().rstrip())
        subprocess.run(['git', 'add', '--', 'CONTRIBUTORS'], cwd=self.root, check=True)
        self.assertEqual(self.run_check()[0], 1)

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

    def test_new_patterns_report_only_location_and_category(self):
        cases = [
            (b"node.synthetic." + b"TS.NET", "tailnet name"),
            (b"100." + b"64.0.0", "tailnet address"),
            (b"100." + b"127.255.255", "tailnet address"),
            (b"100." + b"64.0.1.", "tailnet address"),
            (b"100." + b"100.100.100", "tailnet address"),
            (b"100." + b"99.200.1", "tailnet address"),
            (b"_host.synthetic." + b"ts.net_", "tailnet name"),
            (b"host.synthetic." + b"ts.net.log", "tailnet name"),
            (b" -host.synthetic." + b"ts.net", "tailnet name"),
            (b"SHA256:" + b"G" * 41 + b"+/", "SSH key fingerprint"),
            (b"C:\\Users\\" + b"synthetic\\file", "Windows home directory"),
            (b"z:\\users\\" + b"synthetic", "Windows home directory"),
            (b"C:\\\\Users\\\\" + b"synthetic", "Windows home directory"),
            (b"C:" + b"\\" * 4 + b"Users" + b"\\" * 4 + b"synthetic", "Windows home directory"),
        ]
        for index, (value, category) in enumerate(cases):
            with self.subTest(index=index):
                self.track("planted.bin", b"\0first line\n" + value)
                status, output = self.run_check()
                self.assertEqual(status, 1)
                self.assertEqual(output, f"planted.bin:2: {category}\nErrors: 1\n")
                self.assertNotIn(value.decode(), output)

    def test_digests_placeholders_and_non_tailnet_addresses_are_green(self):
        digest = b"abcdef0123456789" * 4
        values = [
            digest, b"sha256:" + digest, b"SHA256:" + digest,
            b"SHA256:" + b"a" * 43,
            b"SHA256:" + b"G" * 42, b"SHA256:" + b"G" * 44,
            b"100.128.0.1", b"10.0.0.1", b"100.63.255.255",
            b"100." + b"64.256.1", b"100." + b"64.0.999",
            b"1100." + b"64.0.1", b"100." + b"64.0.1.2",
            b"C:\\Users\\<name>", b"C:\\\\Users\\\\<name>", b".ts.net", b"x.ts.network", b"x.ts.net-y",
        ]
        values.extend(b"C:\\Users\\" + name + b"\\file"
                      for name in private_paths.PLACEHOLDER_USERS)
        values.append(b"C:\\Users\\USER\\file")
        self.track("examples.txt", b"\n".join(values))
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_every_second_octet_of_the_tailnet_range_and_none_outside_it(self):
        for second in range(256):
            with self.subTest(second=second):
                self.track("f.txt", b"100." + str(second).encode() + b".0.1")
                if 64 <= second <= 127:
                    self.assertEqual(self.run_check(), (1, "f.txt:1: tailnet address\nErrors: 1\n"))
                else:
                    self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_a_long_run_of_name_characters_is_checked_quickly(self):
        # With no tailnet name in it, a pattern that tries a match at every character of the run
        # takes minutes; one that starts only where a run begins takes well under a second.
        self.track("blob.txt", b"A" * 200_000)
        started = time.monotonic()
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))
        self.assertLess(time.monotonic() - started, 10)

    def test_email_addresses_outside_the_reserved_domains_are_red(self):
        values = [
            b"someone@" + b"mail.synthetic",
            b"First.Last+tag@" + b"Mail.Synthetic",
            b"a_b%c-d@" + b"sub.mail.synthetic",
            b"someone@" + b"example.com.synthetic",
            b"someone@" + b"synthetic-example.com",
            b"someone@" + b"host.invalid.synthetic",
            b"someone@" + b"host.local",
            b"https://someone@" + b"git.synthetic/repository",
            b"<someone@" + b"mail.synthetic>.",
        ]
        for index, value in enumerate(values):
            with self.subTest(index=index):
                self.track("planted.bin", b"\0first line\n" + value)
                status, output = self.run_check()
                self.assertEqual(status, 1)
                self.assertEqual(output, f"planted.bin:2: {private_paths.EMAIL_ERROR}\nErrors: 1\n")
                self.assertNotIn(value.decode(), output)

    def test_each_address_is_reported_at_its_own_line(self):
        self.track("two.md", b"first@" + b"mail.synthetic second@" + b"mail.synthetic\n\nthird@"
                   + b"mail.synthetic\n")
        self.assertEqual(self.run_check(), (
            1, f"two.md:1: {private_paths.EMAIL_ERROR}\ntwo.md:1: {private_paths.EMAIL_ERROR}\n"
            f"two.md:3: {private_paths.EMAIL_ERROR}\nErrors: 3\n"))

    def test_reserved_domains_and_text_that_is_no_address_are_green(self):
        values = [
            b"someone@example.com", b"Someone@EXAMPLE.ORG", b"someone@example.net",
            b"someone@mail.example.com", b"a@b.example.org", b"x@host.test", b"x@site.example",
            b"x@app.localhost", b"x@HOST.INVALID", b"x@a.b.test", b"someone@example.com.",
            b"@AGENTS.md", b"    @contextlib.contextmanager", b"actions/cache/save@v6",
            b"name@1.2.3", b"{}@example.org", b"someone@", b"a@.synthetic", b"x@a.b2c",
            b"x@a.bc2", b"x@a.bc-d",
        ]
        self.track("examples.txt", b"\n".join(values))
        self.assertEqual(self.run_check(), (0, "Errors: 0\n"))

    def test_an_allowed_address_is_green_in_any_case_and_its_neighbours_are_not(self):
        with patch.dict(private_paths.ALLOWED_ADDRESSES, {
                b"role@" + b"vendor.synthetic": "Synthetic vendor role for this test."}):
            self.track("from.txt", b"From: Role@" + b"Vendor.Synthetic\n")
            self.assertEqual(self.run_check(), (0, "Errors: 0\n"))
            values = [b"From: other@" + b"vendor.synthetic\n",
                      b"From: role@" + b"vendor.synthetic.other\n"]
            for index, value in enumerate(values):
                with self.subTest(index=index):
                    self.track("from.txt", value)
                    self.assertEqual(self.run_check()[0], 1)

    def test_every_allowed_address_is_one_address_in_lower_case_with_a_reason(self):
        for index, (address, reason) in enumerate(private_paths.ALLOWED_ADDRESSES.items()):
            with self.subTest(index=index):
                match = private_paths.EMAIL.fullmatch(address)
                self.assertIsNotNone(match)
                self.assertEqual(address, address.lower())
                self.assertFalse(private_paths.reserved(match.group(1)))
                self.assertIsInstance(reason, str)
                self.assertTrue(reason.strip())

    def test_long_runs_around_an_at_sign_are_checked_quickly(self):
        values = [b"a" * 100_000 + b"@" + b"a" * 100_000,
                  b"a+" * 100_000 + b"@", b"x@" + b"a" * 200_000, b"a@" * 100_000]
        for index, value in enumerate(values):
            with self.subTest(index=index):
                self.track("blob.txt", value)
                started = time.monotonic()
                self.assertEqual(self.run_check(), (0, "Errors: 0\n"))
                self.assertLess(time.monotonic() - started, 10)

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
