#!/usr/bin/env python3
"""Check the roadmap against the plan audit's action items (Python standard library only).

    python3 tools/check_roadmap.py

ROADMAP.md, at the root of the repository, holds the order of the work and one row for each
experiment and probe, in its register. docs/AUDIT.md, the plan audit, holds the rows of the
other action items, and docs/archive/plan-audit-history.md those that were done or superseded
before ruling 39 moved them there. An action item's row is a row of a Markdown table whose
first cell is `A` followed by digits, such as `A70`, or a link whose text is one. The checks:

  1. Each action item from A1 to the highest number has exactly one row across
     docs/AUDIT.md, ROADMAP.md and the history: no number is missing, and none has two rows.
  2. Each action item that docs/AUDIT.md or ROADMAP.md cites, as `A70` or as a range
     `A51 to A54`, has a row in one of the three.
  3. Each action item of ROADMAP.md has its row in the register. Each action item of
     docs/AUDIT.md or of the register whose status, or state, does not begin with the word
     done, superseded or dropped is cited in the Items column of the roadmap's order of the
     work.
  4. Each directory experiments/NNN-name/ is listed in experiments/README.md and linked from
     ROADMAP.md, and no two directories share a number. A number that ROADMAP.md holds for a
     directory not yet made, as `experiments/006-across-machines/`, is not used by a
     directory of another name.
  5. README.md, AGENTS.md, CONTRIBUTING.md, docs/README.md and experiments/README.md each link
     ROADMAP.md.
  6. Each State cell of the order of the work and of the register, its finished items
     included, begins with one of the words: done, now, next, later, open, in progress,
     dropped, superseded, removed, not adopted.

It reads text, and knows only the shape of a Markdown table: a table is a run of lines that
begin with `|`, its first line is the header, and a column is found by its name in the header.
Tables and links inside code fences, and links inside inline code or HTML comments, are not
counted. The order of the work is a table under the heading "The order of the work" with the
columns Items and State. The register is each table under "Experiments and probes" with the
columns Item and State, and one of them must also have the column "Question it settles". It
cannot tell whether a stated state is true: the coordinator keeps the states.

The last line of output is "Errors: N", and the exit status is 1 when N is not 0.
"""

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
ROADMAP = ROOT / "ROADMAP.md"
AUDIT = ROOT / "docs" / "AUDIT.md"
HISTORY = ROOT / "docs" / "archive" / "plan-audit-history.md"
EXPERIMENTS = ROOT / "experiments"
INDEX = EXPERIMENTS / "README.md"
ENTRY_POINTS = ("README.md", "AGENTS.md", "CONTRIBUTING.md", "docs/README.md",
                "experiments/README.md")
ORDER = "the order of the work"            # the heading of the order, in lower case
REGISTER = "experiments and probes"        # the heading of the register, in lower case
STATES = ("done", "now", "next", "later", "open", "in progress", "dropped", "superseded",
          "removed", "not adopted")
FINISHED = ("done", "superseded", "dropped")
STATE = re.compile(r"(?:%s)\b" % "|".join(re.escape(s) for s in STATES))
FINISHED_STATE = re.compile(r"(?:%s)\b" % "|".join(FINISHED))
ITEM = re.compile(r"A\d+")
ITEM_LINK = re.compile(r"\[(A\d+)\]\([^)\s]*\)")
CITED = re.compile(r"\bA(\d+)(?:\s+to\s+A(\d+))?\b")
DIRECTORY = re.compile(r"(\d{3})-[A-Za-z0-9._-]+")
HELD = re.compile(r"experiments/((\d{3})-[A-Za-z0-9._-]*[A-Za-z0-9])")
LINK = re.compile(r"!?\[[^\]\n]*\]\((<[^>]+>|[^\s)]+)(?:\s+\"[^\"\n]*\")?\)")


def cells(line):
    """The cells of a table row, split at each `|` that is outside backticks and not escaped."""
    out, cell, code, i = [], [], False, 0
    text = line.strip()
    while i < len(text):
        c = text[i]
        if c == "\\" and i + 1 < len(text) and text[i + 1] == "|":
            cell.append("|")
            i += 2
            continue
        if c == "`":
            code = not code
        if c == "|" and not code:
            out.append("".join(cell).strip())
            cell = []
        else:
            cell.append(c)
        i += 1
    out.append("".join(cell).strip())
    if out and out[0] == "":
        out = out[1:]
    if out and out[-1] == "":
        out = out[:-1]
    return out


def plain(cell):
    """A cell without the emphasis around it, in lower case."""
    return cell.strip().strip("*_").strip().lower()


def tables(path):
    """Each table of a Markdown file, as a dict: its headings, its header and its rows.

    A row is (line number, cells). Code fences are skipped.
    """
    found, fence, headings, block = [], None, {}, []

    def close():
        if len(block) >= 2:
            header = [plain(c) for c in cells(block[0][1])]
            rows = [(n, cells(text)) for n, text in block[2:]]
            found.append({"path": path, "line": block[0][0], "h2": headings.get(2, ""),
                          "h3": headings.get(3, ""), "header": header, "rows": rows})
        block.clear()

    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        mark = re.match(r"^ {0,3}(`{3,}|~{3,})", line)
        if fence:
            if mark and mark[1][0] == fence[0] and len(mark[1]) >= len(fence):
                fence = None
            continue
        if mark:
            close()
            fence = mark[1]
            continue
        if line.lstrip().startswith("|"):
            block.append((number, line))
            continue
        close()
        heading = re.match(r"^ {0,3}(#{1,6})\s+(.+?)\s*#*\s*$", line)
        if heading:
            level = len(heading[1])
            headings[level] = heading[2].strip().lower()
            for deeper in range(level + 1, 7):
                headings.pop(deeper, None)
    close()
    return found


def item_rows(found):
    """Each row whose first cell is an action item's ID, as (ID, table, line, cells).

    The ID may be written plain, with emphasis, or as the text of a link: `[A70](...)`.
    """
    rows = []
    for table in found:
        for number, row in table["rows"]:
            if not row:
                continue
            first = row[0].strip("*_ ")
            link = ITEM_LINK.fullmatch(first)
            if link:
                first = link[1]
            if ITEM.fullmatch(first):
                rows.append((first, table, number, row))
    return rows


def cited(text):
    """The action items a text cites, with each range `A51 to A54` expanded."""
    ids = set()
    for match in CITED.finditer(text):
        first = int(match[1])
        last = int(match[2]) if match[2] else first
        ids.update(f"A{n}" for n in range(first, max(first, last) + 1))
    return ids


def column(table, name):
    return table["header"].index(name) if name in table["header"] else None


def prose(path):
    """A Markdown file's text without its code fences, inline code and HTML comments."""
    kept, fence = [], None
    for line in path.read_text(encoding="utf-8").splitlines():
        mark = re.match(r"^ {0,3}(`{3,}|~{3,})", line)
        if fence:
            if mark and mark[1][0] == fence[0] and len(mark[1]) >= len(fence):
                fence = None
            kept.append("")
            continue
        if mark:
            fence = mark[1]
            kept.append("")
            continue
        kept.append(re.sub(r"(`+)(?:(?!\1).)+?\1", "", line))
    return re.sub(r"<!--.*?-->", "", "\n".join(kept), flags=re.S)


def links_to(path, target):
    """Whether a Markdown file has a local link to the path `target`, or into it."""
    for match in LINK.finditer(prose(path)):
        parsed = urlsplit(match[1].strip("<>"))
        if parsed.scheme or parsed.netloc or not parsed.path:
            continue
        destination = (path.parent / unquote(parsed.path)).resolve()
        if destination == target or target in destination.parents:
            return True
    return False


def label(path):
    return path.relative_to(ROOT).as_posix()


def check():
    errors = []
    missing = [label(p) for p in (ROADMAP, AUDIT, HISTORY, INDEX) if not p.is_file()]
    if missing:
        for name in missing:
            print(f"{name}: missing", file=sys.stderr)
        print("Nothing was checked.")
        print(f"Errors: {len(missing)}")
        return 1

    roadmap, audit, history = tables(ROADMAP), tables(AUDIT), tables(HISTORY)
    order = [t for t in roadmap if t["h2"] == ORDER
             and column(t, "items") is not None and column(t, "state") is not None]
    register = [t for t in roadmap if t["h2"] == REGISTER
                and column(t, "item") is not None and column(t, "state") is not None]
    if not order:
        errors.append(f"{label(ROADMAP)}: no table with the columns Items and State under "
                      f"\"The order of the work\"")
    if not any(column(t, "question it settles") is not None for t in register):
        errors.append(f"{label(ROADMAP)}: no table with the columns Item, Question it settles "
                      f"and State under \"Experiments and probes\"")

    # 1. One row for each action item, from A1 to the highest, across the three files.
    live = item_rows(audit) + item_rows(roadmap)
    finished_before = item_rows(history)
    done_before = {ident for ident, *_ in finished_before}
    seen = {}
    for ident, table, number, _ in live + finished_before:
        seen.setdefault(ident, []).append(f"{label(table['path'])}:{number}")
    for ident, places in sorted(seen.items(), key=lambda kv: int(kv[0][1:])):
        if len(places) > 1:
            errors.append(f"{ident}: {len(places)} rows, at {', '.join(places)}")
    highest = max((int(ident[1:]) for ident in seen), default=0)
    for n in range(1, highest + 1):
        if f"A{n}" not in seen:
            errors.append(f"A{n}: no row in {label(AUDIT)}, {label(ROADMAP)} or "
                          f"{label(HISTORY)}, though A{highest} has one")

    # 2. Each action item that the plan audit or the roadmap cites has a row somewhere.
    known = set(seen)
    roadmap_text = ROADMAP.read_text(encoding="utf-8")
    for path in (AUDIT, ROADMAP):
        for ident in sorted(cited(prose(path)) - known, key=lambda i: int(i[1:])):
            errors.append(f"{label(path)}: cites {ident}, which has no row in "
                          f"{label(AUDIT)}, {label(ROADMAP)} or {label(HISTORY)}")

    # 3. Each action item of the roadmap is in its register, and each one not yet finished
    #    is cited in the Items column of the order.
    in_order = set()
    for table in order:
        items = column(table, "items")
        for _, row in table["rows"]:
            if items < len(row):
                in_order |= cited(row[items])
    open_items = 0
    for ident, table, number, row in live:
        if table["path"] == ROADMAP and table not in register:
            errors.append(f"{label(ROADMAP)}:{number}: {ident} has a row outside the register, "
                          f"a table under \"Experiments and probes\" with the columns Item and "
                          f"State")
            continue
        where = column(table, "status")
        if where is None:
            where = column(table, "state")
        if where is None or where >= len(row):
            errors.append(f"{label(table['path'])}:{number}: {ident} has no status or state")
            continue
        status = plain(row[where])
        if FINISHED_STATE.match(status):
            continue
        open_items += 1
        if ident not in in_order:
            word = status.split(":")[0]
            errors.append(f"{label(table['path'])}:{number}: {ident} is {word!r}, and the "
                          f"Items column of the order of the work does not cite it")

    # 4. Experiment directories: listed in the index, linked from the roadmap, numbered once.
    directories = sorted(d for d in EXPERIMENTS.iterdir()
                         if d.is_dir() and DIRECTORY.fullmatch(d.name))
    numbers = {}
    for d in directories:
        numbers.setdefault(d.name[:3], []).append(d.name)
        if not links_to(INDEX, d):
            errors.append(f"{label(INDEX)}: does not link experiments/{d.name}/")
        if not links_to(ROADMAP, d):
            errors.append(f"{label(ROADMAP)}: does not link experiments/{d.name}/")
    for number, names in sorted(numbers.items()):
        if len(names) > 1:
            errors.append(f"experiments/: {len(names)} directories numbered {number}: "
                          f"{', '.join(names)}")
    held = {}
    for match in HELD.finditer(roadmap_text):
        held.setdefault(match[2], set()).add(match[1])
    for number, names in sorted(held.items()):
        if len(names) > 1:
            errors.append(f"{label(ROADMAP)}: number {number} is held for "
                          f"{', '.join(sorted(names))}")
        for existing in numbers.get(number, []):
            if existing not in names:
                errors.append(f"experiments/{existing}/: number {number} is held on "
                              f"{label(ROADMAP)} for {', '.join(sorted(names))}")

    # 5. Each entry point links the roadmap.
    for name in ENTRY_POINTS:
        path = ROOT / name
        if not path.is_file():
            errors.append(f"{name}: missing")
        elif not links_to(path, ROADMAP):
            errors.append(f"{name}: does not link ROADMAP.md")

    # 6. Each state begins with an allowed word.
    states = 0
    for table in order + register:
        where = column(table, "state")
        for number, row in table["rows"]:
            states += 1
            state = plain(row[where]) if where < len(row) else ""
            if not STATE.match(state):
                errors.append(f"{label(ROADMAP)}:{number}: the state {state[:40]!r} does not "
                              f"begin with one of {', '.join(STATES)}")

    in_audit = sum(1 for _, t, _, _ in live if t["path"] == AUDIT)
    print(f"Action items: {in_audit} rows in {label(AUDIT)}, {len(live) - in_audit} in "
          f"{label(ROADMAP)}, {len(done_before)} in {label(HISTORY)}; {open_items} not finished.")
    print(f"Order of the work: {sum(len(t['rows']) for t in order)} rows, citing "
          f"{len(in_order)} action items. Register: {sum(len(t['rows']) for t in register)} "
          f"rows. States checked: {states}.")
    print(f"Experiment directories: {len(directories)}. Entry points: {len(ENTRY_POINTS)}.")
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return 1 if errors else 0


if __name__ == "__main__":
    if sys.argv[1:]:
        sys.exit("usage: python3 tools/check_roadmap.py")
    sys.exit(check())
