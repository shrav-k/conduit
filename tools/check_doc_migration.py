#!/usr/bin/env python3
"""Audit the dated migration; not a constraint on future documentation changes."""

from pathlib import Path
import argparse
import hashlib
import json
import posixpath
import re
import subprocess
import sys
from urllib.parse import quote, unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = "docs/archive/migrations/2026-11-protocol-framing.json"


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def digest(value):
    return hashlib.sha256(value).hexdigest()


def rebase_links(text, source, destination, moves, paths):
    def target(raw):
        bracket = raw.startswith("<") and raw.endswith(">")
        parsed = urlsplit(raw[1:-1] if bracket else raw)
        if parsed.scheme or parsed.netloc or not parsed.path or parsed.path.startswith("/"):
            return raw
        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(source), unquote(parsed.path)))
        if resolved not in paths and not any(p.startswith(resolved.rstrip("/") + "/") for p in paths):
            return raw
        new = moves.get(resolved, resolved)
        value = quote(posixpath.relpath(new, posixpath.dirname(destination)), safe="/._-")
        if parsed.query:
            value += "?" + parsed.query
        if parsed.fragment:
            value += "#" + parsed.fragment
        return "<" + value + ">" if bracket else value

    text = re.sub(r"(!?\[[^\]\n]*\]\()(<[^>]+>|[^\s)]+)(\))",
                  lambda m: m[1] + target(m[2]) + m[3], text)
    return re.sub(r"(^\[[^\]\n]+\]:\s*)(<[^>]+>|\S+)",
                  lambda m: m[1] + target(m[2]), text, flags=re.M)


def check(working_tree):
    if working_tree:
        read = lambda path: (ROOT / path).read_bytes()
        revision = "working tree"
    else:
        commits = git("log", "--diff-filter=A", "--format=%H", "--", MANIFEST).decode().splitlines()
        if not commits:
            raise ValueError("Migration record is not committed; use --working-tree before committing.")
        revision = commits[-1]
        read = lambda path: git("show", f"{revision}:{path}")
    manifest = json.loads(read(MANIFEST))
    baseline, mechanical = manifest["baseline"], manifest["mechanicalCommit"]
    paths = set(git("ls-tree", "-r", "--name-only", baseline).decode().splitlines())
    records = manifest["files"]
    errors = []
    if {r["source"] for r in records} != {p for p in paths if p.endswith(".md")}:
        errors.append("Manifest does not account for every baseline Markdown file.")
    destinations = [r["destination"] for r in records]
    if len(set(destinations)) != len(destinations):
        errors.append("More than one source maps to the same destination.")
    for record in records:
        source, dest = record["source"], record["destination"]
        original = git("show", f"{baseline}:{source}").decode()
        expected = rebase_links(original, source, dest, manifest["moves"], paths)
        moved = git("show", f"{mechanical}:{dest}")
        if moved != expected.encode() or digest(moved) != record["rebasedSha256"]:
            errors.append(f"{source}: mechanical checkpoint differs beyond moves/link rebasing.")
        if dest in manifest["navigationRewritten"]:
            if digest(read(dest)) != manifest["navigationSha256"][dest]:
                errors.append(f"{dest}: rewritten navigation differs from reviewed migration copy.")
            continue
        for edit in manifest["editorialReplacements"].get(dest, []):
            if expected.count(edit["before"]) != 1:
                errors.append(f"{dest}: editorial correction does not match exactly once.")
            expected = expected.replace(edit["before"], edit["after"], 1)
        if read(dest) != expected.encode():
            errors.append(f"{dest}: content differs beyond recorded editorial corrections.")
    for path, blob in manifest["unchangedFixtures"].items():
        if read(path) != git("cat-file", "blob", blob):
            errors.append(f"{path}: experiment fixture changed.")
    print(f"Audited {revision}: {len(records)} source documents, {len(manifest['moves'])} moves, "
          f"{len(manifest['unchangedFixtures'])} unchanged YAML fixtures.")
    print("Mechanical text preservation and listed editorial changes checked; navigation rewrites identified.")
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return bool(errors)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--working-tree", action="store_true", help="Audit uncommitted migration edits.")
    args = parser.parse_args()
    try:
        sys.exit(check(args.working_tree))
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print(f"Audit unavailable: {error}. A checkout with full migration history is required.", file=sys.stderr)
        sys.exit(2)
