#!/usr/bin/env python3
"""Report authored vector changes without running an engine or rewriting a case."""

import argparse
import fnmatch
import hashlib
import json
import re
import posixpath
from pathlib import Path
import subprocess
import sys


MISSING = object()
KINDS = {"added", "removed", "renamed", "flip", "expectation", "input",
         "fixture", "metadata", "error"}
FLIPS = {("expect", "accepted"), ("expect", "stateUnchanged"),
         *(('expect', 'reply', key) for key in ("reason", "outcome", "accepted", "tool"))}
HELP = """Compare case.yaml, case.yml and case.json files under conformance/ by stable id.
Steps are paired by array index (displayed from 1); a moved case keeps its id.
Changed ids are removals/additions. JSON is tried first; YAML requires PyYAML.
All declared fixture aliases are compared as bytes, including paths outside
conformance/ within the repository. Formatting-only case changes are ignored.

Output: one JSON object per change, then a summary. Case-field paths are JSON
Pointers (zero-based step indices); fixture paths are repository-relative.
Missing before/after fields mean absence, distinct from JSON null. Fixture
values contain byte counts and SHA-256 hashes. Errors cannot be declared.
Exit 0: no flips, undeclared changes or errors; 1: any of these; 2: bad arguments,
revisions or declaration file. A declared flip STILL exits 1 for review.

--declared accepts a YAML/JSON list of shapes, for example:
  - name: intended refusal change
    kind: flip
    cases: [s1-*]
    paths: ['/steps/*/expect/reply/reason']
    before: refused
    after: conflict
  - name: registration gains a key
    kind: expectation
    cases: [s1-register-*]
    paths: ['/steps/*/expect/reply/objects/*/owner']
    operation: add
    after: example-fleet

name, kind and nonempty cases are required; paths defaults to ['*'].
kind is added, removed, renamed, flip, expectation, input, fixture or metadata.
cases globs match case ids; paths globs match report paths (case-sensitive,
fnmatch syntax, '*' also matches '/'). Optional operation is add/remove/replace;
optional before/after require exact JSON values. Omitted values are unconstrained.
For exact absence use operation: add/remove. First matching shape names the
declaration. Unknown shape keys fail, so a typo cannot broaden a declaration.
"""


class DeltaError(Exception):
    """A source or declaration could not be read reliably."""


def git(root, *args, data=None):
    result = subprocess.run(["git", "-C", str(root), *args], input=data,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if result.returncode:
        raise DeltaError(result.stderr.decode("utf-8", "replace").strip())
    return result.stdout


def structured(data):
    """Prefer JSON: some valid JSON control characters are rejected by PyYAML."""
    text = data.decode("utf-8")
    try:
        value = json.loads(text)
    except ValueError:
        try:
            import yaml
        except ImportError as exc:
            raise DeltaError("not JSON; reading YAML requires PyYAML in this interpreter") from exc
        try:
            value = yaml.load(text, Loader=_core12_loader(yaml))
        except yaml.YAMLError as exc:
            raise DeltaError(str(exc)) from exc
    # Only JSON data can have stable JSON paths and exact declaration values.
    try:
        return json.loads(json.dumps(value, allow_nan=False))
    except (TypeError, ValueError) as exc:
        raise DeltaError("data is not representable as JSON: " + str(exc)) from exc



_CORE12 = None


def _core12_loader(yaml):
    """A safe loader with YAML 1.2's core schema, as the engine's reader and check_schemas.py
    resolve scalars: only true and false are booleans, so `no` and `off` stay strings."""
    global _CORE12
    if _CORE12 is None:
        class Core12Loader(yaml.SafeLoader):
            yaml_implicit_resolvers = {}

        for tag, pattern, first in (
            ("null", r"^(?:~|null|Null|NULL|)$", ["~", "n", "N", ""]),
            ("bool", r"^(?:true|True|TRUE|false|False|FALSE)$", list("tTfF")),
            ("int", r"^(?:[-+]?[0-9]+|0o[0-7]+|0x[0-9a-fA-F]+)$", list("-+0123456789")),
            ("float", r"^(?:[-+]?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)(?:[eE][-+]?[0-9]+)?"
                      r"|[-+]?\.(?:inf|Inf|INF)|\.(?:nan|NaN|NAN))$", list("-+0123456789.")),
        ):
            Core12Loader.add_implicit_resolver("tag:yaml.org,2002:" + tag, re.compile(pattern), first)

        def _int(loader, node):
            text = node.value
            if text.startswith("0o"):
                return int(text[2:], 8)
            if text.startswith("0x"):
                return int(text[2:], 16)
            return int(text, 10)

        Core12Loader.add_constructor("tag:yaml.org,2002:int", _int)
        _CORE12 = Core12Loader
    return _CORE12

class Snapshot:
    def __init__(self, root, revision, blobs):
        self.root = root
        self.revision = revision
        self.blobs = blobs
        self.cache = {}
        self.entries = {}
        if revision == "WORKTREE":
            names = git(root, "ls-files", "-z", "--cached", "--others",
                        "--exclude-standard", "--", "conformance/")
            self.paths = sorted({p.decode("utf-8") for p in names.split(b"\0") if p})
            # Tracked deletions are absent, while dangling symlinks remain reportable.
            self.paths = [p for p in self.paths
                          if (root / p).exists() or (root / p).is_symlink()]
        else:
            commit = git(root, "rev-parse", "--verify", "--end-of-options",
                         revision + "^{commit}").decode().strip()
            for entry in git(root, "ls-tree", "-rz", commit).split(b"\0"):
                if entry:
                    header, path = entry.split(b"\t", 1)
                    mode, kind, oid = header.decode().split()
                    if kind == "blob":
                        self.entries[path.decode("utf-8")] = (mode, oid)
            self.paths = sorted(self.entries)

    def prefetch(self, paths):
        if self.revision == "WORKTREE":
            return
        oids = sorted({self.entries[p][1] for p in paths if p in self.entries}
                      - self.blobs.keys())
        if not oids:
            return
        data = git(self.root, "cat-file", "--batch",
                   data=("\n".join(oids) + "\n").encode())
        offset = 0
        for oid in oids:
            end = data.index(b"\n", offset)
            header = data[offset:end].split()
            size = int(header[2])
            self.blobs[oid] = data[end + 1:end + 1 + size]
            offset = end + size + 2

    def read(self, path):
        if path not in self.cache:
            if self.revision == "WORKTREE":
                if (self.root / path).is_symlink():
                    raise DeltaError("symlinks are not supported as case or fixture files")
                self.cache[path] = (self.root / path).read_bytes()
            else:
                if path not in self.entries:
                    raise DeltaError("file not present in revision")
                mode, oid = self.entries[path]
                if mode == "120000":
                    raise DeltaError("symlinks are not supported as case or fixture files")
                self.prefetch([path])
                self.cache[path] = self.blobs[oid]
        return self.cache[path]


def pointer(parts):
    return "".join("/" + str(p).replace("~", "~0").replace("/", "~1") for p in parts)


def equal(before, after):
    if before is MISSING or after is MISSING:
        return before is after
    return json.dumps(before, sort_keys=True) == json.dumps(after, sort_keys=True)


def differences(before, after, parts=()):
    if equal(before, after):
        return
    # A tool's whole definition is a flip, even when a nested schema changes.
    if len(parts) >= 3 and parts[2:] in FLIPS:
        yield parts, before, after
        return
    if (isinstance(before, dict) or before is MISSING) and (
            isinstance(after, dict) or after is MISSING):
        left = {} if before is MISSING else before
        right = {} if after is MISSING else after
        if left or right:
            for key in sorted(left.keys() | right.keys()):
                yield from differences(left.get(key, MISSING), right.get(key, MISSING),
                                       parts + (key,))
            return
    if (isinstance(before, list) or before is MISSING) and (
            isinstance(after, list) or after is MISSING):
        left = [] if before is MISSING else before
        right = [] if after is MISSING else after
        if left or right:
            for i in range(max(len(left), len(right))):
                yield from differences(left[i] if i < len(left) else MISSING,
                                       right[i] if i < len(right) else MISSING, parts + (i,))
            return
    # Container/scalar replacements still expose sensitive descendants as flips.
    if parts[:1] == ("steps",) and 2 <= len(parts) <= 4 and (
            isinstance(before, dict) or isinstance(after, dict)):
        for p, b, a in differences(before if isinstance(before, dict) else MISSING,
                                   after if isinstance(after, dict) else MISSING, parts):
            if p[2:] in FLIPS:
                yield p, b, a
    yield parts, before, after


def change(kind, case, file, path, before=MISSING, after=MISSING, **extra):
    result = dict(kind=kind, case=case, file=file, path=path, **extra)
    result["operation"] = "add" if before is MISSING else "remove" if after is MISSING else "replace"
    if before is not MISSING:
        result["before"] = before
    if after is not MISSING:
        result["after"] = after
    return result


def error(snapshot, file, message, case="", **extra):
    return dict(kind="error", case=case, file=file, path=file,
                side=snapshot.revision, message=str(message), **extra)


def fixture_path(file, path):
    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(file), path))
    if resolved == ".." or resolved.startswith(("../", "/")):
        raise DeltaError("fixture path leaves the repository: " + path)
    return resolved


def load_cases(snapshot):
    paths = [p for p in snapshot.paths if p.startswith("conformance/") and
             posixpath.basename(p) in ("case.yaml", "case.yml", "case.json")]
    snapshot.prefetch(paths)
    cases, errors = {}, []
    for file in paths:
        try:
            case = structured(snapshot.read(file))
            if not isinstance(case, dict) or not isinstance(case.get("id"), str) or not case["id"]:
                raise DeltaError("case needs a nonempty string id")
            if not isinstance(case.get("steps"), list) or any(
                    not isinstance(s, dict) or not isinstance(s.get("input"), dict) or
                    not isinstance(s.get("expect"), dict) for s in case["steps"]):
                raise DeltaError("steps must be a list of objects with input and expect maps")
            if not isinstance(case.get("files", {}), dict) or any(
                    not isinstance(v, str) or not v for v in case.get("files", {}).values()):
                raise DeltaError("files must map aliases to nonempty string paths")
            if case["id"] in cases:
                raise DeltaError("duplicate case id: " + case["id"])
            fixtures = {alias: fixture_path(file, path)
                        for alias, path in case.get("files", {}).items()}
            cases[case["id"]] = (file, case, fixtures)
        except (DeltaError, OSError, UnicodeError, ValueError, RecursionError) as exc:
            errors.append(error(snapshot, file, exc))
    snapshot.prefetch(p for _, _, fixtures in cases.values() for p in fixtures.values())
    for case_id, (file, _, fixtures) in cases.items():
        for alias, path in fixtures.items():
            try:
                snapshot.read(path)
            except (DeltaError, OSError) as exc:
                errors.append(error(snapshot, file, exc, case_id, fixture=alias, path_read=path))
    return cases, errors


def digest(data):
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def compare(root, before="origin/main", after="WORKTREE"):
    blobs = {}
    left, right = Snapshot(root, before, blobs), Snapshot(root, after, blobs)
    old, errors = load_cases(left)
    new, new_errors = load_cases(right)
    changes = errors + new_errors
    for case_id in sorted(old.keys() | new.keys()):
        if case_id not in old:
            file = new[case_id][0]
            changes.append(change("added", case_id, file, "", after=file))
            continue
        if case_id not in new:
            file = old[case_id][0]
            changes.append(change("removed", case_id, file, "", before=file))
            continue
        old_file, old_case, old_fixtures = old[case_id]
        file, case, fixtures = new[case_id]
        if old_file != file:
            changes.append(change("renamed", case_id, file, "", old_file, file))
        for parts, b, a in differences(old_case, case):
            extra = {}
            if len(parts) >= 2 and parts[0] == "steps" and isinstance(parts[1], int):
                extra["step"] = parts[1] + 1
                if parts[2:] in FLIPS:
                    kind = "flip"
                elif len(parts) >= 3 and parts[2] == "expect":
                    kind = "expectation"
                else:
                    kind = "input"
            elif parts and parts[0] in ("files", "initial"):
                kind = "input"
            else:
                kind = "metadata"
            changes.append(change(kind, case_id, file, pointer(parts), b, a, **extra))
        for alias in sorted(old_fixtures.keys() | fixtures.keys()):
            b_path, a_path = old_fixtures.get(alias), fixtures.get(alias)
            try:
                b = left.read(b_path) if b_path is not None else MISSING
                a = right.read(a_path) if a_path is not None else MISSING
            except (DeltaError, OSError):
                continue  # Already reported by load_cases, including unchanged cases.
            if b != a:
                changes.append(change("fixture", case_id, file, a_path or b_path,
                                      digest(b) if b is not MISSING else MISSING,
                                      digest(a) if a is not MISSING else MISSING,
                                      fixture=alias, before_file=b_path, after_file=a_path))
    return changes


def declarations(path):
    shapes = structured(Path(path).read_bytes())
    if not isinstance(shapes, list):
        raise DeltaError("declarations must be a list")
    allowed = {"name", "kind", "cases", "paths", "operation", "before", "after"}
    for shape in shapes:
        if not isinstance(shape, dict) or shape.keys() - allowed:
            raise DeltaError("unknown declaration keys or non-map shape")
        if not isinstance(shape.get("name"), str) or not shape["name"]:
            raise DeltaError("each declaration needs a nonempty name")
        if not isinstance(shape.get("kind"), str) or shape["kind"] not in KINDS - {"error"}:
            raise DeltaError("unknown declaration kind")
        for key in ("cases", "paths"):
            value = shape.get(key, ["*"] if key == "paths" else None)
            if not isinstance(value, list) or not value or any(
                    not isinstance(v, str) for v in value):
                raise DeltaError(key + " must be a nonempty list of globs")
        if "operation" in shape and shape["operation"] not in ("add", "remove", "replace"):
            raise DeltaError("operation must be add, remove or replace")
    return shapes


def mark_declared(changes, shapes):
    for item in changes:
        item["declared"] = False
        for shape in shapes:
            if item["kind"] == "error" or item["kind"] != shape["kind"]:
                continue
            if not any(fnmatch.fnmatchcase(item["case"], p) for p in shape["cases"]):
                continue
            if not any(fnmatch.fnmatchcase(item["path"], p) for p in shape.get("paths", ["*"])):
                continue
            if any(key in shape and not equal(item.get(key, MISSING), shape[key])
                   for key in ("before", "after", "operation")):
                continue
            item["declared"] = True
            item["declaration"] = shape["name"]
            break


def main(argv=None):
    parser = argparse.ArgumentParser(description=HELP, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("before", nargs="?", default="origin/main", help="base Git revision (default: origin/main)")
    parser.add_argument("after", nargs="?", default="WORKTREE", help="new Git revision or WORKTREE (default: WORKTREE)")
    parser.add_argument("--declared", metavar="FILE", help="YAML/JSON declarations of intended changes")
    args = parser.parse_args(argv)
    try:
        root = Path(git(Path.cwd(), "rev-parse", "--show-toplevel").decode().strip())
        shapes = declarations(args.declared) if args.declared else []
        changes = compare(root, args.before, args.after)
    except (DeltaError, OSError, UnicodeError, ValueError, RecursionError) as exc:
        print("vector_delta: " + str(exc), file=sys.stderr)
        print("Vector delta: comparison failed.")
        return 2
    mark_declared(changes, shapes)
    for item in changes:
        print(json.dumps(item, sort_keys=True, ensure_ascii=True))
    flips = sum(c["kind"] == "flip" for c in changes)
    errors = sum(c["kind"] == "error" for c in changes)
    undeclared = sum(not c["declared"] for c in changes)
    print(f"Vector delta: {len(changes)} changes; {flips} flips; {undeclared} undeclared; {errors} errors.")
    return int(bool(flips or undeclared or errors))


if __name__ == "__main__":
    sys.exit(main())
