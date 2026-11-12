#!/usr/bin/env python3
"""Check Git-tracked files for private paths and CONDUIT_PRIVATE_NAMES.

Uses Python's standard library and Git. Reads working-tree bytes, including binary
files, and reports only the location and category, never the private value.
"""

import os
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
# A user name is made of name characters, so a search pattern such as /Users/[A-Za-z] is not one.
HOME_PATH = re.compile(rb"/(?:Users|home)/([A-Za-z0-9._-]+)")
# Conventional stand-ins for a user in tests and examples, which name nobody.
PLACEHOLDER_USERS = {b"test", b"user", b"username", b"example", b"you"}
PATTERNS = (
    ("home directory", HOME_PATH),
    ("session scratch directory", re.compile(rb"/private/" rb"tmp/|/tmp/" rb"claude")),
    ("Codex authentication file", re.compile(re.escape(b".codex/" + b"auth.json"))),
)
FIXTURE = "experiments/005-macos-keychain/src/lib.rs"


def check(root=ROOT):
    """Return location/category errors; the one exception is the fixture's user u."""
    tracked = subprocess.run(
        ["git", "ls-files", "-z"], cwd=root, check=True, stdout=subprocess.PIPE
    ).stdout.split(b"\0")
    names = [name.strip().encode("utf-8") for name in
             os.environ.get("CONDUIT_PRIVATE_NAMES", "").split(",") if name.strip()]
    errors = []
    for raw_path in tracked:
        if not raw_path:
            continue
        path = os.fsdecode(raw_path)
        try:
            target = root / path
            # Git tracks a symlink's target text, not the bytes it points to.
            data = os.fsencode(os.readlink(target)) if target.is_symlink() else target.read_bytes()
        except OSError:
            errors.append(f"{path}: cannot read tracked file")
            continue
        for category, pattern in PATTERNS:
            for match in pattern.finditer(data):
                if category == "home directory" and match.group(1) in PLACEHOLDER_USERS:
                    continue
                if (path == FIXTURE and category == "home directory"
                        and match.group() == b"/Users/" + b"u"):
                    continue
                # Documentation may say where Codex keeps its login; only other files are checked.
                if category == "Codex authentication file" and path.endswith(".md"):
                    continue
                line = data.count(b"\n", 0, match.start()) + 1
                errors.append(f"{path}:{line}: {category}")
        for name in names:
            for match in re.finditer(re.escape(name), data):
                line = data.count(b"\n", 0, match.start()) + 1
                errors.append(f"{path}:{line}: private name from CONDUIT_PRIVATE_NAMES")
    return errors


def main(root=ROOT):
    try:
        errors = check(root)
    except (OSError, subprocess.CalledProcessError):
        print("Could not list Git-tracked files; nothing was checked.")
        return 2
    for error in errors:
        print(error)
    print(f"Errors: {len(errors)}")
    return int(bool(errors))


if __name__ == "__main__":
    sys.exit(main())
