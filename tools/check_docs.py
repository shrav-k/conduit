#!/usr/bin/env python3
"""Offline checks for the repository's Markdown navigation (Python standard library only)."""

from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]


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


def check():
    names = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT
    ).decode().split("\0")
    files = sorted({ROOT / name for name in names if name.endswith(".md") and (ROOT / name).is_file()})
    errors, count, cache = [], 0, {}
    for path in files:
        body, fence = prose(path.read_text())
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
                    cache[destination] = anchors(destination.read_text())
                if unquote(parsed.fragment) not in cache[destination]:
                    errors.append(f"{label}:{line}: missing anchor {target}")
    print(f"Checked {len(files)} Markdown files and {count} local links (offline).")
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return bool(errors)


if __name__ == "__main__":
    sys.exit(check())
