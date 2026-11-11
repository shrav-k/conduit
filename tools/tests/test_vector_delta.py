"""Small authored cases in temporary repositories; no engine output is used."""

import contextlib
import copy
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import vector_delta as delta


class VectorDeltaTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "-q")
        self.path = "conformance/small/case.yaml"
        self.case = {
            "id": "s1-small", "files": {"source": "source.json"},
            "initial": {"recipe": "empty"},
            "steps": [{"input": {"kind": "read", "file": "source"},
                       "expect": {"accepted": True, "stateUnchanged": True,
                                  "reply": {"accepted": True, "reason": "refused",
                                            "outcome": "failed", "tool": {"name": "read"}},
                                  "records": []}}]}
        self.write(self.path, self.case)
        self.write("conformance/small/source.json", {"original": True})
        self.commit()

    def git(self, *args):
        return subprocess.run(["git", "-C", str(self.root), *args], check=True,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE).stdout

    def commit(self):
        self.git("add", ".")
        self.git("-c", "user.name=Vector tests", "-c", "user.email=tests@example.invalid",
                 "-c", "commit.gpgsign=false", "commit", "-qm", "test snapshot")

    def write(self, path, value):
        file = self.root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(json.dumps(value), encoding="utf-8")

    def compare(self):
        return delta.compare(self.root, "HEAD", "WORKTREE")

    def run_main(self, *args):
        out = io.StringIO()
        err = io.StringIO()
        with mock.patch.object(Path, "cwd", return_value=self.root), \
                contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            status = delta.main(list(args))
        return status, out.getvalue(), err.getvalue()

    def test_unchanged_and_formatting_only(self):
        self.assertEqual(self.compare(), [])
        (self.root / self.path).write_text(json.dumps(self.case, indent=4))
        self.assertEqual(self.compare(), [])
        status, out, _ = self.run_main("HEAD")
        self.assertEqual(status, 0)
        self.assertEqual(out, "Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.\n")

    def test_added_untracked_case(self):
        new = copy.deepcopy(self.case)
        new["id"] = "added"
        new["files"] = {}
        self.write("conformance/new/case.json", new)
        changes = self.compare()
        self.assertEqual([(c["kind"], c["case"]) for c in changes], [("added", "added")])
        self.assertEqual(self.run_main("HEAD")[0], 1)

    def test_removed_case(self):
        (self.root / self.path).unlink()
        self.assertEqual(self.compare()[0]["kind"], "removed")

    def test_each_flip_kind_including_nested_tool(self):
        changes_to_make = [
            (("reply", "reason"), "conflict"),
            (("reply", "outcome"), "succeeded"),
            (("reply", "accepted"), False),
            (("accepted",), False),
            (("reply", "tool", "name"), "write"),
            (("stateUnchanged",), False),
        ]
        for keys, value in changes_to_make:
            with self.subTest(keys=keys):
                case = copy.deepcopy(self.case)
                target = case["steps"][0]["expect"]
                for key in keys[:-1]:
                    target = target[key]
                target[keys[-1]] = value
                self.write(self.path, case)
                changes = self.compare()
                self.assertEqual(len(changes), 1)
                item = changes[0]
                self.assertEqual(item["kind"], "flip")
                self.assertEqual(item["step"], 1)
                self.assertIn("before", item)
                self.assertIn("after", item)
                self.assertEqual(self.run_main("HEAD")[0], 1)

    def test_missing_is_distinct_from_null_and_false_from_zero(self):
        case = copy.deepcopy(self.case)
        del case["steps"][0]["expect"]["reply"]["reason"]
        case["steps"][0]["expect"]["accepted"] = 1
        self.write(self.path, case)
        changes = self.compare()
        self.assertEqual(len(changes), 2)
        removed = next(c for c in changes if c["operation"] == "remove")
        self.assertEqual(removed["kind"], "flip")
        self.assertNotIn("after", removed)
        delta.mark_declared(changes, [{"name": "null", "kind": "flip", "cases": ["*"], "after": None}])
        self.assertFalse(any(c["declared"] for c in changes))

    def test_parent_reply_replacement_exposes_flips(self):
        case = copy.deepcopy(self.case)
        case["steps"][0]["expect"]["reply"] = None
        self.write(self.path, case)
        changes = self.compare()
        self.assertEqual(sum(c["kind"] == "flip" for c in changes), 4)
        self.assertTrue(any(c["path"] == "/steps/0/expect/reply" for c in changes))

    def test_declared_and_undeclared_exact_shapes(self):
        case = copy.deepcopy(self.case)
        case["steps"][0]["expect"]["records"] = [{"owner": "fleet"}]
        self.write(self.path, case)
        shapes = [{"name": "add owner", "kind": "expectation", "cases": ["s1-*"],
                   "paths": ["/steps/*/expect/records/*/owner"], "operation": "add", "after": "fleet"}]
        self.write("declared.json", shapes)
        status, out, _ = self.run_main("HEAD", "--declared", str(self.root / "declared.json"))
        self.assertEqual(status, 0)
        self.assertTrue(json.loads(out.splitlines()[0])["declared"])
        self.assertEqual(self.run_main("HEAD")[0], 1)
        for key, value in [("cases", ["other-*"]), ("after", "wrong"),
                           ("operation", "replace"), ("paths", ["/wrong"])]:
            with self.subTest(key=key):
                wrong = copy.deepcopy(shapes)
                wrong[0][key] = value
                self.write("declared.json", wrong)
                self.assertEqual(self.run_main("HEAD", "--declared", str(self.root / "declared.json"))[0], 1)

    def test_declared_flip_still_fails(self):
        case = copy.deepcopy(self.case)
        case["steps"][0]["expect"]["reply"]["reason"] = "conflict"
        self.write(self.path, case)
        self.write("declared.json", [{"name": "intended", "kind": "flip", "cases": ["s1-*"],
                                      "before": "refused", "after": "conflict"}])
        status, out, _ = self.run_main("HEAD", "--declared", str(self.root / "declared.json"))
        self.assertEqual(status, 1)
        self.assertIn('"declared": true', out)
        self.assertIn("1 flips; 0 undeclared", out)

    def test_renamed_case_matched_by_id(self):
        new_path = "conformance/moved/case.yml"
        (self.root / new_path).parent.mkdir()
        (self.root / self.path).rename(self.root / new_path)
        (self.root / "conformance/small/source.json").rename(self.root / "conformance/moved/source.json")
        changes = self.compare()
        self.assertEqual(len(changes), 1)
        self.assertEqual(changes[0]["kind"], "renamed")
        self.assertEqual(changes[0]["before"], self.path)
        self.assertEqual(changes[0]["after"], new_path)

    def test_inputs_capture_initial_and_metadata_paths(self):
        case = copy.deepcopy(self.case)
        case["steps"][0]["input"]["syntax"] = "json"
        case["steps"][0]["capture"] = {"result": "/reply"}
        case["initial"]["recipe"] = "snapshot"
        case["title"] = "new title"
        self.write(self.path, case)
        changes = self.compare()
        self.assertEqual({c["path"] for c in changes}, {
            "/steps/0/input/syntax", "/steps/0/capture/result", "/initial/recipe", "/title"})
        self.assertEqual(sum(c["kind"] == "input" for c in changes), 3)

    def test_fixture_bytes_and_shared_fixture_outside_conformance(self):
        self.write("shared/source.json", {"old": True})
        self.case["files"]["source"] = "../../shared/source.json"
        self.write(self.path, self.case)
        self.commit()
        self.write("shared/source.json", {"new": True})
        changes = self.compare()
        self.assertEqual(len(changes), 1)
        self.assertEqual(changes[0]["kind"], "fixture")
        self.assertEqual(changes[0]["path"], "shared/source.json")
        self.assertNotEqual(changes[0]["before"]["sha256"], changes[0]["after"]["sha256"])

    def test_fixture_path_move_with_identical_bytes(self):
        (self.root / "conformance/small/source.json").rename(self.root / "conformance/small/moved.json")
        self.case["files"]["source"] = "moved.json"
        self.write(self.path, self.case)
        changes = self.compare()
        self.assertEqual(len(changes), 1)
        self.assertEqual(changes[0]["kind"], "input")
        self.assertEqual(changes[0]["path"], "/files/source")

    def test_unreadable_case_and_fixture_are_reported(self):
        original = Path.read_bytes
        for path in [self.path, "conformance/small/source.json"]:
            with self.subTest(path=path):
                def unreadable(file):
                    if file == self.root / path:
                        raise PermissionError("unreadable test file")
                    return original(file)
                with mock.patch.object(Path, "read_bytes", unreadable):
                    changes = self.compare()
                    self.assertTrue(any(c["kind"] == "error" and "unreadable" in c["message"]
                                        for c in changes))

    def test_missing_and_invalid_fixtures_or_cases(self):
        (self.root / "conformance/small/source.json").unlink()
        self.assertTrue(any(c["kind"] == "error" for c in self.compare()))
        self.write(self.path, {"id": "broken", "steps": 3})
        self.assertTrue(any(c["kind"] == "error" for c in self.compare()))
        (self.root / self.path).write_text('{"id":')
        self.assertEqual(self.run_main("HEAD")[0], 1)

    def test_two_revisions_ignore_working_tree(self):
        self.case["steps"][0]["input"]["syntax"] = "json"
        self.write(self.path, self.case)
        self.commit()
        (self.root / self.path).write_text("broken [")
        changes = delta.compare(self.root, "HEAD^", "HEAD")
        self.assertEqual(len(changes), 1)
        self.assertEqual(changes[0]["path"], "/steps/0/input/syntax")

    def test_yaml_and_json_control_characters(self):
        import yaml
        (self.root / self.path).write_text(yaml.safe_dump(self.case))
        self.assertEqual(self.compare(), [])
        self.case["title"] = "JSON permits an escaped control: \x01"
        self.write(self.path, self.case)
        self.assertEqual(self.compare()[0]["path"], "/title")
        (self.root / "declared.yaml").write_text(
            "- name: title\n  kind: metadata\n  cases: ['s1-*']\n  paths: ['/title']\n")
        self.assertEqual(self.run_main("HEAD", "--declared", str(self.root / "declared.yaml"))[0], 0)

    def test_yaml_core_schema_scalars(self):
        """Plain `no` and `off` are distinct strings under YAML 1.2's core schema, as the engine
        reads them; equal plain YAML and quoted JSON strings compare equal."""
        head = ("id: s1-small\nfiles: {source: source.json}\ninitial: {recipe: empty}\nsteps:\n"
                "  - input: {kind: read, file: source}\n"
                "    expect:\n      reply: {tool: {description: %s}}\n")
        (self.root / self.path).write_text(head % "no")
        self.commit()
        (self.root / self.path).write_text(head % "off")
        changes = self.compare()
        self.assertTrue(any(c.get("flip") or "/tool" in c.get("path", "") for c in changes),
                        changes)
        status = self.run_main("HEAD")[0]
        self.assertEqual(status, 1)
        (self.root / self.path).write_text(json.dumps({
            "id": "s1-small", "files": {"source": "source.json"},
            "initial": {"recipe": "empty"},
            "steps": [{"input": {"kind": "read", "file": "source"},
                       "expect": {"reply": {"tool": {"description": "no"}}}}]}))
        self.assertEqual(self.compare(), [])

    def test_invalid_declaration_and_revision_fail_cleanly(self):
        self.write("declared.json", [{"name": "typo", "kind": "input", "cases": ["*"], "path": "*"}])
        status, out, err = self.run_main("HEAD", "--declared", str(self.root / "declared.json"))
        self.assertEqual(status, 2)
        self.assertIn("unknown declaration keys", err)
        self.assertNotIn("Traceback", out + err)
        self.write("declared.json", [{"name": "bad kind", "kind": [], "cases": ["*"]}])
        self.assertEqual(self.run_main("HEAD", "--declared", str(self.root / "declared.json"))[0], 2)
        self.assertEqual(self.run_main("nonexistent-revision")[0], 2)

    def test_json_pointer_escaping_and_new_step(self):
        self.case["steps"].append({"input": {"kind": "read"}, "expect": {
            "accepted": True, "facts": {"a/b~c": 3}}})
        self.write(self.path, self.case)
        changes = self.compare()
        self.assertIn("/steps/1/expect/facts/a~1b~0c", [c["path"] for c in changes])
        flips = [c for c in changes if c["kind"] == "flip"]
        self.assertEqual(len(flips), 1)
        self.assertEqual(flips[0]["step"], 2)


if __name__ == "__main__":
    unittest.main()
