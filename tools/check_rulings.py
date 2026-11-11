#!/usr/bin/env python3
"""Check ruling 67's ledger and record form offline, using only the standard library."""

from collections import Counter
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
HEADER = "| # | Date | Ruling | The owner's words | Status |"
STATUS = re.compile(
    r"(?:(?:in force|done|withdrawn)(?: \([^()]+\))?"
    r"|directive \(A[0-9]+\b[^()]*\)|open \([^()]+\)"
    r"|amended by [1-9][0-9]* \([^()]+\)"
    r"|replaced by [1-9][0-9]*)"
)
DATE = re.compile(
    r"\b[12][0-9]{3}[-/][01][0-9][-/][0-3][0-9]\b"
    r"|\b(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|"
    r"Jul(?:y)?|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)"
    r"\.?\s+[0-3]?[0-9],?\s+[12][0-9]{3}\b"
    r"|\b[0-3]?[0-9]\s+(?:January|February|March|April|May|June|July|August|"
    r"September|October|November|December)\s+[12][0-9]{3}\b",
    re.I,
)


def valid_status(cell):
    """Split semicolons only outside point notes; notes are free text, not statuses."""
    parts = re.split(r";\s*(?![^()]*\))", cell)
    return bool(parts) and all(STATUS.fullmatch(part) for part in parts)


def ledger_rows(text):
    """Read the one current ledger table; historical appendix tables are not ledger rows."""
    lines = text.splitlines()
    if lines.count(HEADER) != 1:
        return [], ["ledger must have exactly one current table"]
    start = lines.index(HEADER)
    errors, rows = [], []
    if start + 1 >= len(lines) or not re.fullmatch(r"\|(?:\s*:?-+:?\s*\|){5}", lines[start + 1]):
        errors.append("ledger table needs a five-column separator")
    for line in lines[start + 2:]:
        if not line.startswith("|"):
            break
        cells = [cell.strip() for cell in re.split(r"(?<!\\)\|", line)[1:-1]]
        if len(cells) != 5 or not cells[0].isdigit():
            errors.append("malformed ledger row: " + line)
            continue
        rows.append((int(cells[0]), cells))
    return rows, errors


def check(root=ROOT):
    errors = []
    records = {}
    for path in (root / "docs/proposals").glob("ruling-*/README.md"):
        match = re.fullmatch(r"ruling-([0-9]+)", path.parent.name)
        if match:
            records[int(match[1])] = path
    if not records:
        return ["no ruling-NN/README.md records found"]
    # Include even a newly created directory whose README has not yet been written.
    highest = max(int(path.name[7:]) for path in (root / "docs/proposals").glob("ruling-*")
                  if path.is_dir() and re.fullmatch(r"ruling-[0-9]+", path.name))
    ledger = root / "docs/proposals/ruling-log.md"
    if not ledger.is_file():
        return ["missing docs/proposals/ruling-log.md"]
    rows, parse_errors = ledger_rows(ledger.read_text(encoding="utf-8"))
    errors.extend(parse_errors)
    counts = Counter(number for number, _ in rows)
    for number in range(1, highest + 1):
        if counts[number] != 1:
            errors.append(f"ruling {number}: expected one ledger row, found {counts[number]}")
    for number, cells in rows:
        if not 1 <= number <= highest:
            errors.append(f"ruling {number}: outside 1 to {highest}")
        if not valid_status(cells[4]):
            errors.append(f"ruling {number}: invalid status: {cells[4]}")
    statuses = {number: cells[4] for number, cells in rows}
    for number, path in sorted(records.items()):
        paragraphs = re.split(r"\n\s*\n", path.read_text(encoding="utf-8").strip())
        if (len(paragraphs) < 2 or not paragraphs[0].startswith("# ")
                or not paragraphs[1].startswith("Status: ")
                or "\n" in paragraphs[1]
                or ". Holds now: " not in paragraphs[1]
                or not paragraphs[1].endswith(". This record says why.")):
            errors.append(f"ruling {number}: first paragraph under title must be its status line")
        elif number in statuses and not paragraphs[1].startswith(
                f"Status: {statuses[number]} ("):
            errors.append(f"ruling {number}: record status differs from ledger")
    in_force = root / "IN_FORCE.md"
    if not in_force.is_file():
        errors.append("missing IN_FORCE.md")
    else:
        decisions = False
        for number, line in enumerate(in_force.read_text(encoding="utf-8").splitlines(), 1):
            if line.startswith("## "):
                decisions = True
            if decisions and DATE.search(line):
                errors.append(f"IN_FORCE.md:{number}: date in decisions")
    return errors


def main():
    errors = check()
    for error in errors:
        print(error, file=sys.stderr)
    print(f"Errors: {len(errors)}")
    return int(bool(errors))


if __name__ == "__main__":
    sys.exit(main())
