#!/usr/bin/env python3
"""Offline checks for the repository's Markdown navigation (Python standard library only).

    python3 tools/check_docs.py
    python3 tools/check_docs.py --walk
    python3 tools/check_docs.py --work-orders

It checks, in every Markdown file, that each local link leads to a path that exists, that each
anchor is a heading or an `id` of the file it points into, that each reference link is defined
once, and that each code fence is closed. Repository GitHub permalinks are checked against
local Git objects; other external links are not followed. Literal repository path mentions
in tools, the conformance manifest and engine sources are checked too.

Each work order's first line gives its ID and index entry. The check validates those
lines and rejects table rows in the work-order README that link to the order files.
With `--work-orders`, it prints the Markdown index in file-name order or reports invalid
first lines. It can be combined with `--walk`, and the index is the same either way.

It finds the Markdown files in one of two ways, and its first line of output says which:

  - In a Git working tree whose top is this repository, it asks Git for the files that are
    tracked, and for those that are untracked and not ignored.
  - Anywhere else, it walks the directory. That is the case when the `git` command is not
    found, and when the directory is not a Git working tree, as after the repository was
    downloaded as an archive. It skips `.git/`, `target/` and what a `.gitignore` file plainly
    names: a line with no wildcard and no `!`. `--walk` asks for this way in a Git working
    tree too.

The check's last line is "Errors: N", and either mode exits 1 when it finds errors.
"""

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import quote, unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
SKIPPED = (".git", "target")           # never walked, whatever .gitignore says
WORK_ORDERS = Path("docs/work-orders")
ORDER_HEADING = re.compile(r"# Work order ([0-9][^\s:]*)(?: \([^()]+\))?: ([^|]+)")
INLINE_LINK = re.compile(r'!?\[[^\]\n]*\]\((?P<target><[^>]+>|[^\s)]+)(?:\s+"[^"\n]*")?\)')
TABLE_REFERENCE_TOKEN = re.compile(
    r"\\[^\w\s]|(?P<ticks>`+)(?!`).*?(?<!`)(?P=ticks)(?!`)|" + INLINE_LINK.pattern
    + r"|!?\[(?P<text>[^\]]+)\](?:\[(?P<label>[^\]]*)\])?"
)
TABLE_BLOCK_START = re.compile(
    r"^(?: {4}|\t| {0,3}(?:#{1,6}(?:\s|$)|>|(?:[-+*]|\d{1,9}[.)])\s|\[[^\]]+\]:))")


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


class Permalinks:
    """Check immutable links offline; absent history is a notice, not a broken path."""

    def __init__(self):
        self.commits = {}
        self.paths = {}
        self.headings = {}
        self.line_counts = {}
        self.checked = 0

    def git(self, *args):
        return subprocess.run(["git", *args], cwd=ROOT, stdout=subprocess.PIPE,
                              stderr=subprocess.DEVNULL, check=False)

    def check(self, parsed):
        if parsed.scheme != "https" or parsed.netloc != "github.com":
            return None
        match = re.fullmatch(r"/shrav-k/conduit/(?:blob|tree)/([0-9a-fA-F]{7,40})/(.+)",
                             parsed.path)
        if not match:
            return None
        sha, name = match[1], unquote(match[2]).rstrip("/")
        if sha not in self.commits:
            try:
                self.commits[sha] = self.git("cat-file", "-e", sha + "^{commit}").returncode == 0
            except FileNotFoundError:
                self.commits[sha] = False
            if not self.commits[sha]:
                print(f"Notice: skipped repository permalinks at {sha}: local Git history "
                      "is unavailable (or this is outside a Git tree).")
        if not self.commits[sha]:
            return None
        self.checked += 1
        key = f"{sha}:{name}"
        if key not in self.paths:
            self.paths[key] = self.git("cat-file", "-e", key).returncode == 0
        if not self.paths[key]:
            return "missing permalink path"
        if parsed.fragment:
            fragment = unquote(parsed.fragment)
            lines = re.fullmatch(r"L([0-9]+)(?:-L([0-9]+))?", fragment)
            if lines:
                if key not in self.line_counts:
                    is_file = self.git("cat-file", "-t", key).stdout.strip() == b"blob"
                    self.line_counts[key] = (len(self.git("show", key).stdout.splitlines())
                                             if is_file else 0)
                start, end = int(lines[1]), int(lines[2] or lines[1])
                if not 1 <= start <= end <= self.line_counts[key]:
                    return "missing permalink anchor"
                return None
            if key not in self.headings:
                result = self.git("show", key)
                self.headings[key] = anchors(result.stdout.decode("utf-8", errors="replace"))
            if fragment not in self.headings[key]:
                return "missing permalink anchor"
        return None


# These protocol methods resemble paths, and check_decisions.py refuses the retired
# directory's return. Concrete descendants of that directory still need to exist.
# The runner and promise_standing.py refuse the retired expected-failures file's return.
PATH_EXCEPTIONS = {
    "tools/call": "JSON-RPC method",
    "tools/list": "JSON-RPC method",
    "docs/proposals": "retired path",
    "docs/proposals/": "retired path",
    "conformance/expected-failures.yaml": "retired path",
}
PATH_MENTION = re.compile(
    r"(?<![\w/.:{\-])(?:(?:docs|spec|conformance|engine|packages|experiments|tools)/"
    r"[A-Za-z0-9_.{}*?\[\]/-]+|(?:AGENTS|CONTRIBUTING|README|ROADMAP)\.md)(?![\w.-])")


def path_mentions():
    """Check literal root-relative paths, including comments, without interpreting code.

    Dynamic format strings are not concrete paths. Bare names relative to a source file,
    JSON pointers, URLs, build output and runtime-generated paths are outside this scan.
    """
    sources = set((ROOT / "tools").glob("*.py"))
    sources.add(ROOT / "conformance/manifest.yaml")
    sources.update((ROOT / "conformance").rglob("index.yaml"))
    for current, directories, names in os.walk(ROOT / "engine"):
        directories[:] = [name for name in directories if name not in SKIPPED]
        sources.update(Path(current) / name for name in names
                       if Path(name).suffix in {".rs", ".toml"})
    errors, count, templates, exceptions = [], 0, set(), set()
    for source in sorted(sources):
        for line, text in enumerate(source.read_text(encoding="utf-8").splitlines(), 1):
            # Case paths in the coverage index are relative to conformance/, rather than
            # to the repository root. Decode JSON string escapes without needing PyYAML.
            if source == ROOT / "conformance/manifest.yaml" or source.name == "index.yaml":
                case = re.match(r'^\s*"case":\s*("(?:[^"\\]|\\.)*")', text)
                if case:
                    count += 1
                    name = json.loads(case[1])
                    if not (ROOT / "conformance" / name).is_file():
                        errors.append(f"{source.relative_to(ROOT)}:{line}: "
                                      f"missing case path {name}")
            for match in PATH_MENTION.finditer(text):
                name = match[0].rstrip(".")
                if "{" in name or "}" in name or "NNN-name" in name:
                    templates.add(name)
                    continue
                if name in PATH_EXCEPTIONS:
                    exceptions.add(name)
                    continue
                count += 1
                exists = (any(ROOT.glob(name)) if any(c in name for c in "*?[")
                          else (ROOT / name).exists())
                if not exists:
                    errors.append(f"{source.relative_to(ROOT)}:{line}: missing mentioned path {name}")
    print(f"Checked {count} repository path mentions in {len(sources)} source files.")
    protocol = sum(PATH_EXCEPTIONS[name] == "JSON-RPC method" for name in exceptions)
    print(f"Notice: skipped {len(templates)} dynamic/example path patterns; "
          f"allow-listed protocol method names: {protocol}, "
          f"retired paths: {len(exceptions) - protocol}.")
    return errors


def work_orders():
    return sorted(path for path in (ROOT / WORK_ORDERS).glob("[0-9]*.md") if path.is_file())


def work_order_entries(files):
    entries, errors, seen = [], [], {}
    for path in files:
        with path.open(encoding="utf-8") as stream:
            heading = ORDER_HEADING.fullmatch(stream.readline().rstrip("\n"))
        label = path.relative_to(ROOT).as_posix()
        if not heading or not heading[2].strip():
            errors.append(f"{label}:1: expected a work order heading with a nonempty title and no pipe")
            continue
        ident, what = heading.groups()
        if not path.name.startswith(ident + "-"):
            errors.append(f"{label}:1: work order ID {ident} must match the file name")
        elif ident in seen:
            errors.append(f"{label}:1: duplicate work order ID {ident}, first used in {seen[ident]}")
        else:
            seen[ident] = label
            entries.append((path.name, ident, what))
    return entries, errors


def print_work_orders():
    entries, errors = work_order_entries(work_orders())
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print("| Order | What |")
    print("| --- | --- |")
    for name, ident, what in entries:
        label = re.sub(r"([\\\[\]|])", r"\\\1", ident)
        print(f"| [{label}]({quote(name)}) | {what} |")
    return 0


def table_cells(line):
    line = line.strip()
    pipes = [match.start() for match in re.finditer(r"\\.|\|", line) if match[0] == "|"]
    bounds = [-1, *pipes, len(line)]
    cells = [line[start + 1:end].strip() for start, end in zip(bounds, bounds[1:])]
    if pipes and pipes[0] == 0:
        cells.pop(0)
    if pipes and pipes[-1] == len(line) - 1:
        cells.pop()
    return cells, bool(pipes)


def table_lines(body):
    """Identify header and body rows of tables with a matching separator."""
    header, header_has_pipe = [], False
    in_table = False
    for number, line in enumerate(body.splitlines(), 1):
        if not line.strip() or TABLE_BLOCK_START.match(line):
            in_table = False
            header, header_has_pipe = [], False
            continue
        if in_table:
            yield number
            continue
        cells, has_pipe = table_cells(line)
        if ((header_has_pipe or has_pipe) and len(header) == len(cells) and cells
                and all(re.fullmatch(r":?-+:?", cell) for cell in cells)):
            yield number - 1
            in_table = True
        header, header_has_pipe = cells, has_pipe


def table_link_targets(row, definitions):
    """Find inline links and resolve full, collapsed and shortcut references."""
    cells, _ = table_cells(row)
    for cell in cells:
        for match in TABLE_REFERENCE_TOKEN.finditer(cell):
            if match["target"] is not None:
                yield match["target"]
            elif match["text"] is not None:
                ident = " ".join((match["label"] or match["text"]).lower().split())
                if ident in definitions:
                    yield definitions[ident]


def local_destination(path, parsed):
    if parsed.path.startswith("/"):
        return (ROOT / unquote(parsed.path).lstrip("/")).resolve()
    return (path.parent / unquote(parsed.path)).resolve() if parsed.path else path.resolve()


def check(walk=False):
    found, why = (None, "--walk was given") if walk else from_git()
    if found is None:
        found = by_walking()
        print(f"Files: found by walking the directory, because {why}. Skipped: "
              f"{', '.join(name + '/' for name in SKIPPED)} and what .gitignore plainly names.")
    else:
        print("Files: listed by Git (tracked, and untracked but not ignored).")
    files = sorted(path for path in found if path.is_file())
    order_files = work_orders()
    _, errors = work_order_entries(order_files)
    order_files = {path.resolve() for path in order_files}
    count, cache = 0, {}
    permalinks = Permalinks()
    for path in files:
        body, fence = prose(path.read_text(encoding="utf-8"))
        label = path.relative_to(ROOT).as_posix()
        if fence:
            errors.append(f"{label}:{fence[2]}: unclosed code fence")
        order_rows = set(table_lines(body)) if path == ROOT / WORK_ORDERS / "README.md" else set()
        linked_orders = []
        definitions = {}
        targets = []
        target_spans = set()
        for match in re.finditer(r"^ {0,3}\[([^\]\n]+)\]:\s*(<[^>]+>|\S+)", body, re.M):
            ident = " ".join(match[1].lower().split())
            if ident in definitions:
                errors.append(f"{label}: duplicate reference definition [{ident}]")
            definitions[ident] = match[2]
            targets.append((match[2], body[:match.start()].count("\n") + 1))
            target_spans.add(match.span(2))
        for match in re.finditer(r"!?\[([^\]\n]+)\]\[([^\]\n]*)\]", body):
            ident = " ".join((match[2] or match[1]).lower().split())
            if ident not in definitions:
                errors.append(f"{label}: undefined reference [{ident}]")
        for match in INLINE_LINK.finditer(body):
            targets.append((match[1], body[:match.start()].count("\n") + 1))
            target_spans.add(match.span(1))
        for match in re.finditer(r"<https?://[^<>\s]+>", body):
            if match.span() not in target_spans:
                targets.append((match[0], body[:match.start()].count("\n") + 1))
        for raw, line in targets:
            target = raw.strip("<>")
            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc:
                error = permalinks.check(parsed)
                if error:
                    errors.append(f"{label}:{line}: {error} {target}")
                continue
            count += 1
            destination = local_destination(path, parsed)
            if not destination.exists():
                errors.append(f"{label}:{line}: missing path {target}")
            elif parsed.fragment and destination.suffix == ".md":
                if destination not in cache:
                    cache[destination] = anchors(destination.read_text(encoding="utf-8"))
                if unquote(parsed.fragment) not in cache[destination]:
                    errors.append(f"{label}:{line}: missing anchor {target}")
        if order_rows:
            for line, row in enumerate(body.splitlines(), 1):
                if line in order_rows:
                    for raw in table_link_targets(row, definitions):
                        parsed = urlsplit(raw.strip("<>"))
                        if not (parsed.scheme or parsed.netloc) and local_destination(path, parsed) in order_files:
                            linked_orders.append(line)
        if linked_orders:
            errors.append(f"{label}:{min(linked_orders)}: generate work order rows with --work-orders")
    print(f"Checked {len(files)} Markdown files and {count} local links (offline).")
    print(f"Checked {permalinks.checked} repository permalinks against local Git history.")
    errors.extend(path_mentions())
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return bool(errors)


def main(argv):
    if set(argv) - {"--walk", "--work-orders"}:
        print("usage: python3 tools/check_docs.py [--walk] [--work-orders]", file=sys.stderr)
        return 1
    if "--work-orders" in argv:
        return print_work_orders()
    return int(check(walk="--walk" in argv))


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
