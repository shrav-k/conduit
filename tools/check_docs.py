#!/usr/bin/env python3
"""Offline checks for the repository's Markdown navigation (Python standard library only).

    python3 tools/check_docs.py
    python3 tools/check_docs.py --walk

It checks, in every Markdown file, that each local link leads to a path that exists, that each
anchor is a heading or an `id` of the file it points into, that each reference link is defined
once, and that each code fence is closed. It does not follow links to other sites.

It finds the Markdown files in one of two ways, and its first line of output says which:

  - In a Git working tree whose top is this repository, it asks Git for the files that are
    tracked, and for those that are untracked and not ignored.
  - Anywhere else, it walks the directory. That is the case when the `git` command is not
    found, and when the directory is not a Git working tree, as after the repository was
    downloaded as an archive. It skips `.git/`, `target/` and what a `.gitignore` file plainly
    names: a line with no wildcard and no `!`. `--walk` asks for this way in a Git working
    tree too.

The last line of output is "Errors: N", and the exit status is 1 when N is not 0.
"""

import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
SKIPPED = (".git", "target")           # never walked, whatever .gitignore says


def from_git():
    """The Markdown files Git knows of, or None with the reason Git could not be asked."""
    def git(*arguments):
        return subprocess.run(["git", *arguments], cwd=ROOT, stdout=subprocess.PIPE,
                              stderr=subprocess.DEVNULL, check=True).stdout.decode()
    try:
        top = git("rev-parse", "--show-toplevel").strip()
        if Path(top).resolve() != ROOT:
            return None, "this directory is not the top of a Git working tree"
        names = git("ls-files", "--cached", "--others", "--exclude-standard", "-z").split("\0")
    except FileNotFoundError:
        return None, "the `git` command was not found"
    except subprocess.CalledProcessError:
        return None, "this directory is not a Git working tree"
    return {ROOT / name for name in names if name.endswith(".md")}, None


def plainly_named(directory):
    """What the .gitignore of one directory plainly names, as (path, directories only, anchored).

    A line with a wildcard or a leading `!` is not read: Git's pattern rules are not
    reimplemented here.
    """
    try:
        lines = (directory / ".gitignore").read_text(encoding="utf-8").splitlines()
    except OSError:
        return []
    named = []
    for line in lines:
        line = line.strip()
        if not line or line[0] in "#!" or any(mark in line for mark in "*?[\\"):
            continue
        only_directories = line.endswith("/")
        line = line.rstrip("/")
        anchored = "/" in line
        if line.strip("/"):
            named.append((directory / line.strip("/"), only_directories, anchored))
    return named


def by_walking():
    """Every Markdown file under ROOT, apart from what is skipped."""
    files, named = set(), []

    def skipped(path, is_directory):
        for target, only_directories, anchored in named:
            if only_directories and not is_directory:
                continue
            if anchored:
                if path == target:
                    return True
            elif path.name == target.name and target.parent in path.parents:
                return True
        return False

    for current, directories, names in os.walk(ROOT):
        current = Path(current)
        named.extend(plainly_named(current))
        directories[:] = sorted(name for name in directories
                                if name not in SKIPPED and not skipped(current / name, True))
        files.update(current / name for name in names
                     if name.endswith(".md") and not skipped(current / name, False))
    return files


def prose(text):
    """Blank fenced code while preserving line numbers; return any unclosed fence."""
    fence = None
    lines = []
    for number, line in enumerate(text.splitlines(), 1):
        match = re.match(r"^ {0,3}(`{3,}|~{3,})(.*)$", line)
        if fence:
            if (match and match[1][0] == fence[0] and len(match[1]) >= fence[1]
                    and not match[2].strip()):
                fence = None
            lines.append("")
        elif match:
            fence = (match[1][0], len(match[1]), number)
            lines.append("")
        else:
            lines.append(line)
    return "\n".join(lines), fence


def anchors(text):
    body, _ = prose(text)
    result = set(re.findall(r'<(?:a|h[1-6])\b[^>]*\bid=["\']([^"\']+)', body))
    counts = {}
    for match in re.finditer(r"^ {0,3}#{1,6}\s+(.+?)\s*#*\s*$", body, re.M):
        title = re.sub(r"\[([^\]]+)\]\([^)]*\)", r"\1", match[1])
        title = re.sub(r"<[^>]*>|[`*_]", "", title).lower()
        key = re.sub(r"[^\w\- ]", "", title).replace(" ", "-")
        n = counts.get(key, 0)
        counts[key] = n + 1
        result.add(key + (f"-{n}" if n else ""))
    return result


def check(walk=False):
    found, why = (None, "--walk was given") if walk else from_git()
    if found is None:
        found = by_walking()
        print(f"Files: found by walking the directory, because {why}. Skipped: "
              f"{', '.join(name + '/' for name in SKIPPED)} and what .gitignore plainly names.")
    else:
        print("Files: listed by Git (tracked, and untracked but not ignored).")
    files = sorted(path for path in found if path.is_file())
    errors, count, cache = [], 0, {}
    for path in files:
        body, fence = prose(path.read_text(encoding="utf-8"))
        label = path.relative_to(ROOT).as_posix()
        if fence:
            errors.append(f"{label}:{fence[2]}: unclosed code fence")
        definitions = {}
        targets = []
        for match in re.finditer(r"^ {0,3}\[([^\]\n]+)\]:\s*(<[^>]+>|\S+)", body, re.M):
            ident = " ".join(match[1].lower().split())
            if ident in definitions:
                errors.append(f"{label}: duplicate reference definition [{ident}]")
            definitions[ident] = match[2]
            targets.append((match[2], body[:match.start()].count("\n") + 1))
        for match in re.finditer(r"!?\[([^\]\n]+)\]\[([^\]\n]*)\]", body):
            ident = " ".join((match[2] or match[1]).lower().split())
            if ident not in definitions:
                errors.append(f"{label}: undefined reference [{ident}]")
        for match in re.finditer(r'!?\[[^\]\n]*\]\((<[^>]+>|[^\s)]+)(?:\s+"[^"\n]*")?\)', body):
            targets.append((match[1], body[:match.start()].count("\n") + 1))
        for raw, line in targets:
            target = raw.strip("<>")
            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc:
                continue
            count += 1
            destination = ((ROOT / unquote(parsed.path).lstrip("/")) if parsed.path.startswith("/")
                           else (path.parent / unquote(parsed.path))) if parsed.path else path
            destination = destination.resolve()
            if not destination.exists():
                errors.append(f"{label}:{line}: missing path {target}")
            elif parsed.fragment and destination.suffix == ".md":
                if destination not in cache:
                    cache[destination] = anchors(destination.read_text(encoding="utf-8"))
                if unquote(parsed.fragment) not in cache[destination]:
                    errors.append(f"{label}:{line}: missing anchor {target}")
    print(f"Checked {len(files)} Markdown files and {count} local links (offline).")
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return bool(errors)


if __name__ == "__main__":
    if set(sys.argv[1:]) - {"--walk"}:
        sys.exit("usage: python3 tools/check_docs.py [--walk]")
    sys.exit(check(walk="--walk" in sys.argv[1:]))
