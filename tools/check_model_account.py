#!/usr/bin/env python3
"""Check G5's nonbinding account against the promises, catalogue and evidence.

With --history, also verify the stored digests against the commits the models read.
Only the script entry point may retry under /usr/bin/python3 when PyYAML is absent.
"""

import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

try:
    import yaml
except ImportError:
    yaml = None


SPEC_DIRS = ("spec/promises", "spec/conventions", "spec/interface")
ACCOUNT = "experiments/012-model-checking/account.yaml"
CATALOGUE = "docs/implementation/SAFETY_CATALOGUE.md"
MANIFEST = "conformance/manifest.yaml"
MODEL_DIR = "experiments/012-model-checking/src"
CLASSES = ("safety", "not safety", "uncatalogued")
RUN_STANDINGS = frozenset(("searched", "restated", "holds, not graded"))
FINDING = re.compile(r"(?:contradiction|gap|ambiguity|property over-broad): MC[0-9]+[a-z]?-[0-9]{3}")
OTHER_STANDINGS = frozenset(("text changed, not rerun", "static", "not modelled", "left to MC9"))
KEY = r"[A-Za-z_][A-Za-z0-9_]*'*"  # a key may be primed, as in N'
MODEL = re.compile(rf"([A-Za-z_][A-Za-z0-9_]*)::{KEY}(?:, {KEY})*")


def model_run(standing):
    return isinstance(standing, str) and (standing in RUN_STANDINGS or FINDING.fullmatch(standing) is not None)


def digest(promise):
    """Hash parsed fields, so YAML folding and mapping order do not affect the result."""
    fields = {key: promise.get(key) for key in ("level", "observable", "then", "when")}
    text = json.dumps(fields, sort_keys=True, ensure_ascii=False, separators=(",", ":"))
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def promise_files(paths):
    return sorted(path for path in paths
                  if path.suffix in (".yaml", ".yml") and path.parent.as_posix() in SPEC_DIRS)


def promises_in(data):
    if isinstance(data, list):
        return [item for item in data if isinstance(item, dict) and "id" in item]
    return []


def catalogue_classes(text):
    classes = {}
    section = None
    for line in text.splitlines():
        if line.startswith("## "):
            section = {"## Safety promises": "safety", "## The others": "not safety"}.get(line.strip())
        match = re.match(r"^\| ([A-Z]+-[0-9]+) \|", line)
        if section and match:
            classes[match[1]] = section
    return classes


def historical_promises(root, read):
    """Read each commit once; an ID may have moved between the three directories."""
    def git(*args):
        result = subprocess.run(["git", *args], cwd=root, capture_output=True, text=True, encoding="utf-8")
        if result.returncode:
            raise ValueError("Git could not read the recorded commit")
        return result.stdout

    paths = git("ls-tree", "-r", "--name-only", read, "--", *SPEC_DIRS).splitlines()
    promises = {}
    for path in promise_files(Path(name) for name in paths):
        data = yaml.safe_load(git("show", f"{read}:{path.as_posix()}"))
        for promise in promises_in(data):
            promises[promise["id"]] = promise
    return promises


def main(root, argv):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--history", action="store_true", help="verify digests at their recorded commits")
    args = parser.parse_args(argv)
    if yaml is None:
        print("PyYAML is needed. NOTHING WAS CHECKED.")
        return 2

    root = Path(root).resolve()
    errors = []

    def error(identifier, message):
        errors.append(f"{identifier}: {message}")

    def load(path):
        try:
            return yaml.safe_load((root / path).read_text(encoding="utf-8"))
        except (OSError, UnicodeError, yaml.YAMLError):
            error(str(path), "could not read YAML")
            return None

    promises = {}
    paths = (path.relative_to(root) for directory in SPEC_DIRS for path in (root / directory).glob("*"))
    for path in promise_files(paths):
        for promise in promises_in(load(path)):
            if promise.get("layer") == "convention":
                promises[promise["id"]] = promise
    try:
        classes = catalogue_classes((root / CATALOGUE).read_text(encoding="utf-8"))
    except (OSError, UnicodeError):
        error(CATALOGUE, "could not read catalogue")
        classes = {}

    manifest = load(MANIFEST)
    cases = manifest.get("cases", []) if isinstance(manifest, dict) else []
    covered = {identifier for case in cases for identifier in case.get("promises", [])}
    account = load(ACCOUNT)
    if not isinstance(account, list):
        error(ACCOUNT, "expected a list of rows")
        account = []

    rows = []
    seen = set()
    history = {}

    def outside_exists(path):
        if not isinstance(path, str) or not path or Path(path).is_absolute():
            return False
        target = (root / path).resolve()
        return target.is_relative_to(root) and target.exists()

    for index, row in enumerate(account, 1):
        if not isinstance(row, dict) or not isinstance(row.get("id"), str):
            error(f"{ACCOUNT} row {index}", "expected a mapping with a string id")
            continue
        rows.append(row)
        identifier = row["id"]
        if identifier in seen:
            error(identifier, "second row for this ID")
        seen.add(identifier)
        if identifier not in promises:
            error(identifier, "not a convention-layer promise in spec/")
        expected_class = classes.get(identifier, "uncatalogued")
        if row.get("class") != expected_class:
            error(identifier, f"class must be {expected_class!r} according to the catalogue")

        required = {"id", "class"}
        allowed = set(required)
        standing = row.get("standing")
        if row.get("class") == "safety":
            required.update(("standing", "note"))
            allowed.update(required)
            if model_run(standing) or standing == "text changed, not rerun":
                required.update(("model", "read", "digest"))
                allowed.update(required)
                if standing == "text changed, not rerun":
                    required.add("was")
                    allowed.add("was")
            elif isinstance(standing, str) and standing in OTHER_STANDINGS:
                allowed.add("model")
                if standing == "static":
                    allowed.add("outside")
            else:
                error(identifier, f"unknown standing {standing!r}")

        for key in sorted(required - row.keys()):
            error(identifier, f"missing required key {key}")
        for key in sorted(row.keys() - allowed, key=str):
            error(identifier, f"key {key} is not allowed for this class and standing")
        for key in ("note", "model", "read", "digest"):
            if key in row and (not isinstance(row[key], str) or not row[key]):
                error(identifier, f"{key} must be a nonempty string")
        if "was" in row and not model_run(row["was"]):
            error(identifier, "was must be a model-run standing")
        if "model" in row:
            match = MODEL.fullmatch(row["model"]) if isinstance(row["model"], str) else None
            if not match:
                error(identifier, "model must be module::key, with further keys after commas")
            elif not (root / MODEL_DIR / (match[1] + ".rs")).is_file():
                error(identifier, f"model module {match[1]} has no file")
        if "outside" in row:
            if not isinstance(row["outside"], list):
                error(identifier, "outside must be a list of repository paths")
            else:
                for path in row["outside"]:
                    if not outside_exists(path):
                        error(identifier, f"outside path {path!r} does not exist in the repository")
        if "digest" in row and (not isinstance(row["digest"], str)
                                or not re.fullmatch(r"[0-9a-f]{64}", row["digest"])):
            error(identifier, "digest must be a lowercase SHA-256 hexadecimal string")
        if model_run(standing) and identifier in promises and row.get("digest") != digest(promises[identifier]):
            error(identifier, "promise text differs from digest; rerun the model or mark the row text changed, not rerun")

        read = row.get("read")
        if args.history and isinstance(read, str) and read:
            if read not in history:
                try:
                    history[read] = historical_promises(root, read)
                except (OSError, ValueError, yaml.YAMLError):
                    history[read] = None
            old = history[read]
            if old is None:
                error(identifier, "could not read promise history at read")
            elif identifier not in old:
                error(identifier, "promise absent at read")
            elif row.get("digest") != digest(old[identifier]):
                error(identifier, "digest differs from promise text at read")

    for identifier in sorted(promises.keys() - seen):
        error(identifier, "convention-layer promise has no account row")

    counts = Counter(row.get("class") for row in rows if isinstance(row.get("class"), str))
    print(f"Rows: {len(rows)} (" + ", ".join(f"{key} {counts[key]}" for key in CLASSES) + ")")
    print("Uncatalogued IDs: " + ", ".join(sorted(row["id"] for row in rows if row.get("class") == "uncatalogued")))
    standings = Counter(row["standing"] for row in rows if isinstance(row.get("standing"), str))
    print("Standings:")
    for standing, count in sorted(standings.items()):
        print(f"  {standing}: {count}")
    changed = Counter(row["was"] for row in rows if row.get("standing") == "text changed, not rerun"
                      and isinstance(row.get("was"), str))
    print("Text changed, not rerun by was:")
    for standing, count in sorted(changed.items()):
        print(f"  {standing}: {count}")
    unsupported = [row for row in rows if row.get("standing") == "static" and row["id"] not in covered
                   and not (isinstance(row.get("outside"), list) and any(outside_exists(p) for p in row["outside"]))]
    print(f"Static rows with no manifest case and no outside evidence: {len(unsupported)}")
    for message in errors:
        print(message)
    print(f"Errors: {len(errors)}")
    return 1 if errors else 0


if __name__ == "__main__":
    fallback = "/usr/bin/python3"
    if (yaml is None and os.environ.get("CONDUIT_CHECK_MODEL_ACCOUNT_REEXEC") != "1"
            and os.path.exists(fallback)
            and os.path.realpath(sys.executable) != os.path.realpath(fallback)):
        os.environ["CONDUIT_CHECK_MODEL_ACCOUNT_REEXEC"] = "1"
        os.execv(fallback, [fallback, str(Path(__file__).resolve()), *sys.argv[1:]])
    sys.exit(main(Path(__file__).resolve().parent.parent, sys.argv[1:]))
