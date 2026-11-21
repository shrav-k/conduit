#!/usr/bin/env python3
"""Check the decision framework between two Git snapshots."""

import argparse
from collections import Counter
from dataclasses import dataclass
from datetime import date, datetime, timedelta, timezone
from pathlib import Path
import re
import stat
import sys
from urllib.parse import unquote

from check_docs import anchors
from vector_delta import DeltaError, Snapshot, git


WORK = "HOW-WE-WORK.md"
CHANGES = "spec/CHANGES.md"
DECISIONS = "docs/decisions/README.md"
LOG = "docs/decisions/log.md"
STANDING = "engine/STANDING.md"
DEFINITIONS = "spec/definitions.md"
OPERATIONS = "spec/core/operations.yaml"
WORK_ORDERS = "docs/work-orders/"
PROPOSALS = "docs/proposals"
DRAFT = "Status: draft, not yet in force"
UNRELEASED = "## [Unreleased]"
DECISION = re.compile(r"\bDEC-[0-9]{3}\b")
RULING = re.compile(r"\bruling (-?[0-9]+)\b")
PROMISE = re.compile(r"\b[A-Z]{2,5}-[0-9]{3}\b")
ENTRY = re.compile(r"## (DEC-[0-9]{3}), ([0-9]{4}-[0-9]{2}-[0-9]{2}), (\S.*)")
ITEM = re.compile(r"^( {0,3})([-+*]|[0-9]+[.)])[ \t]+(\S.*)")
FENCE = re.compile(r"^ {0,3}(\x60{3,}|~{3,})(.*)$")
CODE_SPAN = re.compile(r"(?<!\x60)(\x60+)(?!\x60).*?(?<!\x60)\1(?!\x60)", re.S)
COMMENT_OR_CODE = re.compile(r"<!--|" + CODE_SPAN.pattern, re.S)
CHANGE = re.compile(r"- (Added|Changed|Deprecated|Removed|Fixed|Security): (\S.*)\.$")
LABEL = re.compile(r" (\x60(?:by review|checked: tools/" r"[^\x60\s]+|refused: [^\x60\s]+)\x60)$")
ANCHOR = re.compile(r"\bspec/(?:README|definitions)\.md#[^\s)\x60>\],;]+")


class Tree(Snapshot):
    def __init__(self, root, revision, blobs):
        super().__init__(root, revision, blobs, pathspec=".", what="decision framework files")
        self.paths = set(self.paths)

    def read(self, path):
        try:
            if self.revision == "WORKTREE" and any(
                (self.root / parent).is_symlink() for parent in Path(path).parents
            ):
                raise self.symlink_refused()
            return super().read(path)
        except (DeltaError, OSError) as exc:
            raise DeltaError(f"{path}: could not read a regular snapshot file") from exc

    def text(self, path):
        if path not in self.paths:
            return ""
        try:
            return self.read(path).decode("utf-8")
        except UnicodeError as exc:
            raise DeltaError(f"{path}: expected UTF-8") from exc

    def tracked(self):
        if self.revision == "WORKTREE":
            paths = git(self.root, "ls-files", "-z", "--cached")
            return self.paths.intersection(p.decode("utf-8") for p in paths.split(b"\0"))
        return self.paths

    def mode(self, path):
        if self.revision != "WORKTREE":
            return self.entries[path][0]
        mode = (self.root / path).lstat().st_mode
        if stat.S_ISLNK(mode):
            return "120000"
        return "100755" if mode & stat.S_IXUSR else "100644"

    def regular_file(self, path):
        if path not in self.paths:
            return False
        if self.revision != "WORKTREE":
            return self.mode(path) in ("100644", "100755")
        relative = Path(path)
        return (not any((self.root / parent).is_symlink() for parent in relative.parents)
                and stat.S_ISREG((self.root / relative).lstat().st_mode))


def blank(text):
    return re.sub(r"[^\r\n]", " ", text)


def mask_comments(line, in_comment):
    if not in_comment and "<!--" not in line:
        return line, False
    parts = []
    offset = 0
    while True:
        if in_comment:
            end = line.find("-->", offset)
            if end == -1:
                return "".join(parts) + blank(line[offset:]), True
            parts.append(blank(line[offset:end + 3]))
            offset = end + 3
        else:
            opening = next((match for match in COMMENT_OR_CODE.finditer(line, offset)
                            if match[0] == "<!--"), None)
            if opening is None:
                return "".join(parts) + line[offset:], False
            start = opening.start()
            parts.append(line[offset:start])
            offset = start
        in_comment = not in_comment


def markdown_rows(text):
    """Yield visible lines, list depth, and whether a line starts a list item.

    Blanking preserves offsets into the original Markdown for link comparisons.
    Indentation is measured relative to the containing list item's text column.
    """
    parents = []
    fence = None
    in_comment = False
    for raw in text.splitlines(keepends=True):
        expanded = raw.expandtabs(4).rstrip("\r\n")
        indent = len(expanded) - len(expanded.lstrip(" "))
        if fence:
            column, marker = fence
            if column and expanded.strip() and indent < column:
                fence = None
            else:
                closing = FENCE.match(expanded[column:])
                if closing and closing[1].startswith(marker) and not closing[2].strip():
                    fence = None
                yield blank(raw), len(parents), False
                continue
        if not in_comment and expanded.strip():
            while parents and indent < parents[-1]:
                parents.pop()
            column = parents[-1] if parents else 0
            if indent >= column + 4:
                yield blank(raw), len(parents), False
                continue
            opening = FENCE.match(expanded[column:])
            if opening:
                fence = column, opening[1]
                yield blank(raw), len(parents), False
                continue
        visible, in_comment = mask_comments(raw, in_comment)
        depth = len(parents)
        marker = None
        if visible.strip():
            line = visible.expandtabs(4)
            indent = len(line) - len(line.lstrip(" "))
            while parents and indent < parents[-1]:
                parents.pop()
            depth = len(parents)
            column = parents[-1] if parents else 0
            marker = ITEM.match(line[column:])
            if marker:
                parents.append(column + marker.start(3))
        yield visible, depth, bool(marker)


@dataclass
class ListItem:
    line: int
    section: str
    text: str


def list_items(text):
    """Read Markdown items with indented continuation text and the current heading."""
    result = []
    section = ""
    active = []
    for line_number, (line, depth, marker) in enumerate(markdown_rows(text), 1):
        if not line.strip():
            continue
        del active[depth:]
        if marker:
            current = ListItem(line_number, section, line.strip())
            result.append(current)
            active.append(current)
        elif active:
            active[-1].text += " " + line.strip()
        elif re.match(r"^ {0,3}#{1,2} ", line):
            section = line.strip()
    return result


def added_items(before, after):
    remaining = Counter(item.text for item in before)
    for item in after:
        if remaining[item.text]:
            remaining[item.text] -= 1
        else:
            yield item


def citation(text):
    """Return the contents of the final parenthesized group, if present."""
    if not text.endswith(")"):
        return ""
    depth = 0
    for index in range(len(text) - 1, -1, -1):
        if text[index] == ")":
            depth += 1
        elif text[index] == "(":
            depth -= 1
            if not depth:
                return text[index + 1:-1]
    return ""


def valid_citation(group, entries):
    decisions = DECISION.findall(group)
    rulings = RULING.findall(group)
    return ((decisions or rulings) and all(ref in entries for ref in decisions)
            and all(1 <= int(ref) <= 92 for ref in rulings))


def promise_ids(text):
    return {ref for ref in PROMISE.findall(text) if not ref.startswith("DEC-")}


def term_cells(text):
    lines = [line for line, _, _ in markdown_rows(text)]
    terms = set()
    for index, line in enumerate(lines):
        cells = line.strip().strip("|").split("|")
        if len(cells) < 2 or not all(re.fullmatch(r":?-+:?", cell.strip()) for cell in cells):
            continue
        for row in lines[index + 1:]:
            if "|" not in row:
                break
            cell = row.strip().removeprefix("|").split("|", 1)[0].strip()
            terms.add(cell.strip("\x60"))
            terms.update(piece.strip().strip("\x60") for piece in cell.split(","))
    return terms


def change_subject(text, after):
    for target in ANCHOR.findall(text):
        path, anchor = target.split("#", 1)
        known = anchors(after.text(path))
        if unquote(anchor) in known or unquote(anchor.rstrip(".")) in known:
            return True
    terms = term_cells(after.text(DEFINITIONS))
    return any(match in terms for match in re.findall(r"\x60([^\x60]+)\x60", text))


# Keep the title and every other byte outside each destination significant.
REFERENCE = re.compile(r"^ {0,3}\[[^\]\n]+\]:[ \t]*(?:\n[ \t]*)?(<[^>\n]*>|\S+)", re.M)
INLINE = re.compile(r"(?<!\\)!?\[[^\]\n]*\]\([ \t]*")
INLINE_END = re.compile(r"(?:\s+(?:\"[^\"\n]*\"|'[^'\n]*'|\([^()\n]*\)))?\s*\)")


def inline_destination_end(text, start):
    if text.startswith("<", start):
        angle = re.match(r"<[^>\n]*>", text[start:])
        if not angle:
            return None
        end = start + angle.end()
    else:
        depth = 0
        end = start
        while end < len(text):
            char = text[end]
            if char.isspace() or (char == ")" and depth == 0):
                break
            if char == "\\" and end + 1 < len(text) and text[end + 1] in "()\\":
                end += 2
                continue
            depth += (char == "(") - (char == ")")
            end += 1
        if depth:
            return None
    return end if INLINE_END.match(text, end) else None


def link_targets_ignored(text):
    """Compare Markdown while ignoring only ordinary inline or reference destinations."""
    mask = "".join(line for line, _, _ in markdown_rows(text))
    mask = CODE_SPAN.sub(lambda m: blank(m[0]), mask)
    spans = {match.span(1) for match in REFERENCE.finditer(mask)}
    for link in INLINE.finditer(mask):
        start = link.end()
        end = inline_destination_end(mask, start)
        if end is not None:
            spans.add((start, end))
    # A link definition must not be treated as an inline link inside its own destination.
    end = 0
    chunks = []
    for start, stop in sorted(spans):
        if start >= end:
            chunks.extend((text[end:start], "<destination>"))
            end = stop
    chunks.append(text[end:])
    return "".join(chunks)


@dataclass
class Entry:
    line: int
    identifier: str
    dated: date
    before_status: tuple
    status: tuple
    after_status: tuple

    @property
    def size(self):
        return sum(len(line.encode("utf-8"))
                   for line in self.before_status + self.status + self.after_status)


def entries(text, bad_heading):
    lines = text.splitlines(keepends=True)
    headings = [i for i, line in enumerate(lines) if line.startswith("## ")]
    result = []
    previous = None
    for number, (start, end) in enumerate(zip(headings, headings[1:] + [len(lines)]), 1):
        heading = ENTRY.fullmatch(lines[start].rstrip("\r\n"))
        if not heading:
            bad_heading(start + 1, "expected a DEC-NNN entry heading")
            continue
        identifier, written, _ = heading.groups()
        try:
            dated = date.fromisoformat(written)
        except ValueError:
            bad_heading(start + 1, "expected a valid date")
            continue
        if identifier != f"DEC-{number:03d}":
            bad_heading(start + 1, f"expected DEC-{number:03d}, found {identifier}")
        if previous and dated < previous:
            bad_heading(start + 1, "entry date is earlier than the preceding entry")
        previous = dated
        while end > start + 1 and not lines[end - 1].strip():
            end -= 1
        status = next((i for i in range(start + 1, end) if lines[i].startswith("Status:")), end)
        stop = next((i for i in range(status, end) if not lines[i].strip()), end)
        # A terminating line ending does not make an additional blank line.
        content = tuple(lines[start:end])
        result.append(Entry(start + 1, identifier, dated, content[:status - start],
                            content[status - start:stop - start], content[stop - start:]))
    return result


def standing(tree):
    text = tree.text(STANDING)
    totals = re.findall(
        r"^\| no case, no reason \| [0-9]+ \| [0-9]+ \| [0-9]+ \| ([0-9]+) \|$",
        text, re.M,
    )
    promises = {}
    for line in text.splitlines():
        columns = line.split("|")
        if len(columns) >= 4 and PROMISE.fullmatch(columns[1].strip()):
            promises[columns[1].strip()] = columns[-2].strip()
    return promises, int(totals[0]) if len(totals) == 1 else None


def language_decisions(items, promises):
    done = {}
    for item in items:
        group = citation(item.text.removesuffix("."))
        ids = promise_ids(item.text)
        complete = item.text.startswith("- Removed: ") or all(
            promises.get(ref) == "case passes" for ref in ids)
        for identifier in DECISION.findall(group):
            done[identifier] = done.get(identifier, True) and complete
    return done


def lines(data):
    return data.count(b"\n") + bool(data and not data.endswith(b"\n"))


class Check:
    def __init__(self, before, after, today):
        self.before = before
        self.after = after
        self.cutoff = today - timedelta(days=28)
        self.errors = 0
        self.draft = False

    def finding(self, rule, path, message, line=None, fail=True):
        self.errors += fail
        location = path if line is None else f"{path}:{line}"
        print(f"{'error' if fail else 'report'}: {location}: {rule}: {message}")

    def has_base(self, rule, path):
        if path in self.before.paths:
            return True
        self.finding(rule, path, "no file in BEFORE, so no comparison", fail=False)
        return False

    def framework(self):
        paths = (WORK, CHANGES, DECISIONS, LOG)
        drafts = []
        for path in paths:
            if path not in self.after.paths:
                self.finding("framework", path, "required file missing in AFTER")
            elif any(line.startswith(DRAFT) for line in self.after.text(path).splitlines()):
                drafts.append(path)
        self.draft = bool(drafts)
        if self.draft and len(drafts) != len(paths):
            for path in paths:
                if path not in drafts and path in self.after.paths:
                    self.finding("framework", path, "draft status differs across framework files")
        if self.draft:
            print("Framework: not in force. Coupling, no third copy, and HOW-WE-WORK form report only.")
        else:
            print("Framework: in force.")

    def changes(self, old, new, base_promises, head_promises, decisions):
        if not self.has_base("CHANGES lines", CHANGES):
            return
        for item in added_items(old, new):
            kind = CHANGE.fullmatch(item.text)
            if not kind or not valid_citation(citation(item.text[:-1]), decisions):
                self.finding("CHANGES lines", CHANGES, "expected '- Kind: … (decisions).'", item.line)
                continue
            ids = promise_ids(item.text)
            if kind[1] == "Removed":
                if ids and self.has_base("Removed promises", STANDING):
                    bad = (ids - base_promises.keys()) | (ids & head_promises.keys())
                else:
                    bad = set()
            else:
                bad = ids - head_promises.keys()
            if bad:
                self.finding("CHANGES lines", CHANGES,
                             "unexpected promise standing for " + ", ".join(sorted(bad)), item.line)
            if not ids and not change_subject(item.text, self.after):
                self.finding("CHANGES lines", CHANGES,
                             "expected a spec anchor or a backticked definitions term", item.line)

    def append_only(self, base, head):
        if not self.has_base("append-only", LOG):
            return
        for index, old in enumerate(base):
            if index >= len(head) or old.identifier != head[index].identifier:
                self.finding("append-only", LOG, f"{old.identifier} is missing or moved")
                continue
            new = head[index]
            if (old.before_status, old.after_status) != (new.before_status, new.after_status):
                self.finding("append-only", LOG, f"{old.identifier} changed outside Status", new.line)
            if old.status != new.status and not (
                old.status and old.status[0].startswith("Status: accepted, not done")
                and new.status and new.status[0].startswith("Status: done")
                and re.search(r"(?<![\w#])#[1-9][0-9]*\b", "".join(new.status))
            ):
                self.finding("append-only", LOG,
                             f"{old.identifier} Status must go from accepted, not done to done with a pull request",
                             new.line)

    def paths(self):
        before, after = self.before.paths, self.after.paths
        added = after - before
        prefix = PROPOSALS + "/"
        proposals_added = sorted(path for path in added if path.startswith(prefix))
        if PROPOSALS not in before and not any(path.startswith(prefix) for path in before):
            for path in sorted(set(proposals_added) | (added & {PROPOSALS})):
                self.finding("gone stays gone", path, "docs/proposals/ was absent in BEFORE")
        for path in ("IN_FORCE.md", "docs/AUDIT.md"):
            if path in added:
                self.finding("gone stays gone", path, "absent in BEFORE")
        for path in sorted(added):
            if not path.startswith(WORK_ORDERS) or not re.fullmatch(
                r"[0-9][^/]*\.md", path.removeprefix(WORK_ORDERS)
            ):
                continue
            text = self.after.text(path)
            for section in ("Preservation tests", "Should this exist"):
                if f"**{section}" not in text:
                    self.finding("new orders", path, f"missing {section}")
            size = len(self.after.read(path))
            if size > 20 * 1024:
                self.finding("order size", path, f"{size} bytes exceeds 20 KB", fail=False)
        for path in proposals_added:
            self.finding("no third copy", path, "new proposal file", fail=not self.draft)
        if "IN_FORCE.md" in before:
            bullet = re.compile(r"^\s*[-*] ", re.M)
            base = len(bullet.findall(self.before.text("IN_FORCE.md")))
            head = len(bullet.findall(self.after.text("IN_FORCE.md")))
            if head > base:
                self.finding("no third copy", "IN_FORCE.md", f"bullet count rose from {base} to {head}",
                             fail=not self.draft)

    def coupling(self, old, new):
        if not self.has_base("coupling", CHANGES):
            return
        old = (item for item in old if item.section == UNRELEASED)
        new = (item for item in new if item.section == UNRELEASED)
        if next(added_items(old, new), None):
            return
        for path in sorted(self.before.paths | self.after.paths):
            if not path.startswith("spec/") or path == CHANGES:
                continue
            both = path in self.before.paths and path in self.after.paths
            if both and self.before.mode(path) == self.after.mode(path):
                if self.before.read(path) == self.after.read(path):
                    continue
                if path.endswith(".md") and (
                    link_targets_ignored(self.before.text(path))
                    == link_targets_ignored(self.after.text(path))
                ):
                    continue
            self.finding("coupling", path, "spec change needs a new Unreleased item", fail=not self.draft)

    def work_form(self, decisions):
        rules = [item for item in list_items(self.after.text(WORK)) if re.match(r"^## [0-9]+\. ", item.section)]
        review = 0
        operations = set(re.findall(r"^  name: ([^\s#]+)", self.after.text(OPERATIONS), re.M))
        for item in rules:
            label = LABEL.search(item.text)
            if label:
                review += label[1] == "`by review`"
            checked = label and label[1].startswith("`checked: ")
            refused = label and label[1].startswith("`refused: ")
            if not label or (checked and not self.after.regular_file(label[1][10:-1])) or (
                refused and label[1][10:-1].removeprefix("management.") not in operations
            ):
                self.finding("HOW-WE-WORK form", WORK, "expected a review, existing checker, or operation label",
                             item.line, fail=not self.draft)
            if not label or not valid_citation(citation(item.text[:label.start()]), decisions):
                self.finding("HOW-WE-WORK form", WORK, "expected decision citations before the label",
                             item.line, fail=not self.draft)
            if re.search(r"\b(?:[0-9]{4}-[0-9]{2}-[0-9]{2}|orders?\s+[0-9]{3}\b)", item.text, re.I) or (
                "*Coordinator's*" in item.text
            ):
                self.finding("HOW-WE-WORK form", WORK, "rule holds a date, order number, or coordinator mark",
                             item.line, fail=not self.draft)
        print(f"By review: {review} of {len(rules)} HOW-WE-WORK rule lines.")

    def caps(self, base_entries, head_entries):
        for path, measure, cap, unit in (("AGENTS.md", lines, 150, "lines"), (WORK, len, 12 * 1024, "bytes")):
            if path not in self.after.paths:
                continue
            size = measure(self.after.read(path))
            if size > cap:
                armed = path not in self.before.paths or measure(self.before.read(path)) <= cap
                self.finding("cap", path, f"{size} {unit} exceeds {cap}", fail=armed)
        base = {entry.identifier: entry.size for entry in base_entries}
        for entry in head_entries:
            if entry.size > 1536:
                self.finding("cap", LOG, f"{entry.identifier} is {entry.size} bytes, above 1536", entry.line,
                             fail=base.get(entry.identifier, 0) <= 1536)

    def ratchets(self, base_entries, head_entries, old, new, base_standing, head_standing):
        base_promises, base_count = base_standing
        head_promises, head_count = head_standing
        if self.has_base("no-case ratchet", STANDING):
            if STANDING not in self.after.paths:
                self.finding("no-case ratchet", STANDING, "required file missing in AFTER")
            for snapshot, total in ((self.before, base_count), (self.after, head_count)):
                if STANDING in snapshot.paths and total is None:
                    self.finding("no-case ratchet", STANDING,
                                 f"expected one no case, no reason totals row in {snapshot.revision}")
            if base_count is not None and head_count is not None:
                print(f"No-case ratchet: base {base_count}, head {head_count}.")
                if head_count > base_count:
                    self.finding("no-case ratchet", STANDING, "no case, no reason total rose")
        base = language_decisions(old, base_promises)
        head = language_decisions(new, head_promises)
        if all([self.has_base("not-done ratchet", path) for path in (CHANGES, LOG, STANDING)]):
            base_count = sum(e.dated < self.cutoff and base.get(e.identifier) is False for e in base_entries)
            head_count = sum(e.dated < self.cutoff and head.get(e.identifier) is False for e in head_entries)
            print(f"Not-done ratchet: base {base_count}, head {head_count}, entries dated before {self.cutoff}.")
            if head_count > base_count:
                self.finding("not-done ratchet", CHANGES, "old language decisions without passing cases rose")
        for entry in head_entries:
            if (entry.dated < self.cutoff and entry.status
                    and not head.get(entry.identifier, entry.status[0].startswith("Status: done"))):
                self.finding("dormant decision", LOG, f"{entry.identifier} is not done", entry.line, fail=False)

    def ratio(self):
        tracked = self.after.tracked()
        counts = []
        for prefixes in (("spec/", "engine/"), ("docs/work-orders/", "docs/proposals/")):
            paths = [p for p in sorted(tracked) if p.startswith(prefixes)]
            paths = [p for p in paths if self.after.mode(p) != "120000"]
            self.after.prefetch(paths)
            counts.append(sum(lines(self.after.read(p)) for p in paths))
        engine, orders = counts
        quotient = f"{engine / orders:.2f}" if orders else "undefined"
        print(f"Ratio: {engine} spec/ and engine/ lines to {orders} docs/work-orders/ and docs/proposals/ lines ({quotient}).")

    def run(self):
        self.framework()
        old = list_items(self.before.text(CHANGES))
        new = list_items(self.after.text(CHANGES))
        base_entries = entries(self.before.text(LOG), lambda *_: None)
        head_entries = entries(self.after.text(LOG),
                               lambda line, msg: self.finding("numbering", LOG, msg, line))
        decisions = {entry.identifier for entry in head_entries}
        base_standing, head_standing = standing(self.before), standing(self.after)
        self.changes(old, new, base_standing[0], head_standing[0], decisions)
        self.append_only(base_entries, head_entries)
        self.paths()
        self.coupling(old, new)
        self.work_form(decisions)
        self.caps(base_entries, head_entries)
        self.ratchets(base_entries, head_entries, old, new, base_standing, head_standing)
        self.ratio()


class Arguments(argparse.ArgumentParser):
    def error(self, message):
        raise ValueError(message)


def main(argv=None):
    parser = Arguments(description=__doc__)
    parser.add_argument("before", metavar="BEFORE", help="base commit")
    parser.add_argument("after", metavar="AFTER", help="head commit or WORKTREE")
    parser.add_argument("--today", metavar="YYYY-MM-DD", help="age comparison date (defaults to today in UTC)")
    check = None
    try:
        args = parser.parse_args(argv)
        if args.today is not None and not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}", args.today):
            raise ValueError("--today must be YYYY-MM-DD")
        today = date.fromisoformat(args.today) if args.today else datetime.now(timezone.utc).date()
        if args.before == "WORKTREE":
            raise ValueError("BEFORE must be a commit")
        root = Path(git(Path.cwd(), "rev-parse", "--show-toplevel").decode().strip())
        blobs = {}
        for name, revision in (("BEFORE", args.before), ("AFTER", args.after)):
            try:
                snapshot = Tree(root, revision, blobs)
            except DeltaError as exc:
                raise ValueError(f"{name} does not resolve to a readable snapshot") from exc
            if name == "BEFORE":
                before = snapshot
        check = Check(before, snapshot, today)
        check.run()
    except (DeltaError, ValueError, OSError, OverflowError) as exc:
        print(f"error: tools/check_decisions.py: {exc}")
        print(f"Errors: {1 + (check.errors if check else 0)}")
        return 2 if check is None else 1
    print(f"Errors: {check.errors}")
    return int(bool(check.errors))


if __name__ == "__main__":
    sys.exit(main())
