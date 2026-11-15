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
# One backslash or more, as escaped text (JSON, TOML, string literals, JSON inside JSON) writes it.
WINDOWS_HOME_PATH = re.compile(rb"[A-Za-z]:\\+Users\\+([A-Za-z0-9._-]+)", re.IGNORECASE)
# Match complete DNS labels and decimal octets, rather than fragments of longer values.
DNS_LABEL = rb"[A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?"
IP_OCTET = rb"(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[0-9]{1,2})"
# Conventional stand-ins for a user in tests and examples, which name nobody.
PLACEHOLDER_USERS = {b"test", b"user", b"username", b"example", b"you"}
PATTERNS = (
    ("home directory", HOME_PATH),
    ("session scratch directory", re.compile(rb"/private/" rb"tmp/|/tmp/" rb"claude")),
    ("Codex authentication file", re.compile(re.escape(b".codex/" + b"auth.json"))),
    ("tailnet name", re.compile(
        # The lookbehind starts a match only where a run of name characters begins, which keeps
        # the search linear on long runs such as encoded blobs.
        rb"(?<![A-Za-z0-9])(?:" + DNS_LABEL + rb"\.)+ts\.net(?![A-Za-z0-9-])", re.IGNORECASE)),
    ("tailnet address", re.compile(
        rb"(?<![0-9.])100\.(?:6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\."
        + IP_OCTET + rb"\." + IP_OCTET + rb"(?![0-9]|\.[0-9])")),
    ("SSH key fingerprint", re.compile(rb"SHA256:([A-Za-z0-9+/]{43})(?![A-Za-z0-9+/])")),
    ("Windows home directory", WINDOWS_HOME_PATH),
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
                if (category == "Windows home directory"
                        and match.group(1).lower() in PLACEHOLDER_USERS):
                    continue
                if (category == "SSH key fingerprint"
                        and re.fullmatch(rb"[A-Fa-f0-9]+", match.group(1))):
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
