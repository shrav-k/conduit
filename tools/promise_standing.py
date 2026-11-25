#!/usr/bin/env python3
"""Generate or check the first build's recorded evidence for each promise."""

import argparse
from collections import Counter
import difflib
import os
from pathlib import Path
import re
import sys

try:
    import yaml
except ImportError:
    yaml = None


SPEC_DIRS = ("spec/promises", "spec/conventions", "spec/interface")
CONFORMANCE = "conformance"
EXPECTED_FAILURES = "conformance/expected-failures"
MANIFEST = "conformance/manifest.yaml"
SOURCE_DIRS = ("engine/crates/loader/src", "engine/crates/core/src", "engine/crates/shell/src")
TABLE = "engine/STANDING.md"
CRATES = ("loader", "core", "shell")
LAYERS = ("language", "convention", "interface")
STANDINGS = ("case passes", "expected failure", "reason, no case", "no case, no reason")
UNREADABLE = object()
KIND_NAMES = {dict: "a mapping", list: "a list", str: "a string"}

INTRO = """# Each promise's standing in the first build

Not binding. The [specification](../spec/README.md) alone binds. This file is generated: do not
edit it by hand. `python3 tools/promise_standing.py --write` writes it from the promise files
under `spec/`, the coverage index under `conformance/` (its `index.yaml` pieces and the S1 rows
of its `manifest.yaml`), [the expected failures](../conformance/README.md#expected-failures) and
the sources of the crates `conduit-loader`, `conduit-core` and `conduit-shell`. CI runs
`python3 tools/promise_standing.py --check`, which fails when this file differs from what those
inputs give.

It says what evidence the tree holds for each promise, not whether the first build keeps the
promise. A case shows only the slice of a promise that its steps assert, and a reason is not a
pass ([the S1 inventory](../conformance/README.md#s1-whole-and-promise-inventory-i2-09)). A
source file that names a promise may keep it only in part, and code that names no promise may
still keep one. The first build states its choices and limits in
[its conformance choices](CONFORMANCE.md).

## The columns

- **ID**, **Layer** and **Level**: the promise's own. The rows are in byte order of ID.
- **Cases**: how many entries of the coverage index list the promise among their `promises`.
- **Expected failures**: how many of those cases a fragment under
  `conformance/expected-failures/` lists.
- **S1**: `cases` where the promise's row in the S1 inventory (`s1_coverage` in
  `conformance/manifest.yaml`) names cases, `reason` where it gives a reason instead, and
  `outside` where the promise has no row there.
- **Engine source**: which of the crates `loader`, `core` and `shell` have a `.rs` file under
  their `src/` directory that names the promise's ID, in code, a string, a comment or a unit
  test alike; `none` where no such file does. The conformance runner's crate and the crates'
  `tests/` and `examples/` directories are not read.
- **Standing**, from Cases, Expected failures and S1 alone:
  - `case passes`: at least one case lists the promise and is not an expected failure. The
    runner fails a run in which a case fails that the expected failures do not list, and CI
    runs every case on each pull request and each push to `main`.
  - `expected failure`: cases list the promise, and the expected failures list each of them.
  - `reason, no case`: no case lists the promise, and its S1 row gives a reason.
  - `no case, no reason`: no case lists the promise, and no reason is recorded.

## Totals

Each number counts promises. The last row counts those whose Engine source is not `none`.
"""


def yaml_kind(value):
    if value is None:
        return "empty or null"
    for kind, name in ((bool, "a boolean"), ((int, float), "a number"), (str, "a string"),
                       (list, "a list"), (dict, "a mapping")):
        if isinstance(value, kind):
            return name
    return f"a {type(value).__name__}"


def render(rows):
    lines = [INTRO.rstrip(), "", "| Promises | language | convention | interface | all |",
             "| --- | ---: | ---: | ---: | ---: |"]
    for category in (*STANDINGS, "all", "named in engine source"):
        counts = Counter(row["layer"] for row in rows
                         if category == "all" or row["standing"] == category
                         or (category == "named in engine source" and row["source"] != "none"))
        values = [counts[layer] for layer in LAYERS]
        lines.append(f"| {category} | " + " | ".join(str(n) for n in (*values, sum(values))) + " |")
    lines.extend(("", "## The promises", "",
                  "| ID | Layer | Level | Cases | Expected failures | S1 | Engine source | Standing |",
                  "| --- | --- | --- | ---: | ---: | --- | --- | --- |"))
    for row in rows:
        lines.append("| " + " | ".join(str(row[key]) for key in
                     ("id", "layer", "level", "cases", "failures", "s1", "source", "standing")) + " |")
    return "\n".join(lines) + "\n"


def main(root, argv):
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument("--write", action="store_true", help="write the generated table")
    modes.add_argument("--check", action="store_true", help="compare without writing")
    args = parser.parse_args(argv)
    if yaml is None:
        print("PyYAML is needed. NOTHING WAS READ OR WRITTEN.")
        return 2

    root = Path(root)
    errors = []

    def error(path, message):
        errors.append(f"{path}: {message}")

    def load(path, missing=None):
        try:
            return yaml.safe_load((root / path).read_text(encoding="utf-8"))
        except FileNotFoundError:
            error(path, missing or "not found; provide the YAML input")
        except (OSError, UnicodeError):
            error(path, "could not read UTF-8 YAML; provide a readable UTF-8 YAML file")
        except yaml.YAMLError as exc:
            mark, problem = getattr(exc, "problem_mark", None), getattr(exc, "problem", None)
            if mark is not None:
                detail = f" ({' '.join(str(problem).split())})" if problem else ""
                error(path, f"could not parse YAML at line {mark.line + 1}, column {mark.column + 1}"
                      f"{detail}; correct the YAML there")
            else:
                error(path, f"could not parse YAML ({' '.join(str(exc).split())}); provide valid YAML")
        return UNREADABLE

    def kind(path, value, expected, location, repair):
        if value is UNREADABLE:
            return False
        if not isinstance(value, expected):
            error(path, f"{location} is {yaml_kind(value)}, not {KIND_NAMES[expected]}; {repair}")
            return False
        return True

    def field(path, row, key, expected, location, wanted):
        if key not in row:
            error(path, f"{location} has no {key}; add {key}, {wanted}")
            return False
        return kind(path, row[key], expected, f"{location}'s {key}", f"make {key} {wanted}")

    def entries(path, data, key, wanted):
        if (kind(path, data, dict, "document", f"make it a mapping whose {key} is {wanted}")
                and field(path, data, key, list, "document", wanted)):
            return data[key]
        return []

    promises, origins = {}, {}
    promise_paths = sorted(path.relative_to(root) for directory in SPEC_DIRS
                           for path in (root / directory).glob("*.yaml") if path.is_file())
    for path in promise_paths:
        data = load(path)
        if not kind(path, data, list, "document",
                    "make it a list of promises, each a mapping with an id, a layer and a level"):
            continue
        for position, item in enumerate(data, 1):
            location = f"promise {position}"
            if not kind(path, item, dict, location,
                        "make each promise a mapping with an id, a layer and a level"):
                continue
            valid = [field(path, item, key, str, location, wanted) for key, wanted in (
                ("id", "a string naming the promise"),
                ("layer", "a string: language, convention or interface"),
                ("level", "a string naming the promise's level"))]
            if not all(valid):
                continue
            if item["layer"] not in LAYERS:
                error(path, f"{location}'s layer is {item['layer']!r}, which is not a layer; "
                      "make layer language, convention or interface")
                continue
            identifier = item["id"]
            if identifier in promises:
                error(path, f"promise ID {identifier} also occurs in {origins[identifier]}; give each promise a unique ID")
                continue
            promises[identifier] = {key: item[key] for key in ("id", "layer", "level")}
            origins[identifier] = path
    if not promises:
        error("spec", "no promise file holds a promise, so there is nothing to tabulate")

    cases = []
    index_paths = []
    for directory, children, files in os.walk(root / CONFORMANCE):
        if Path(directory) == root / CONFORMANCE:
            children[:] = [name for name in children if name != "expected-failures"]
        if "index.yaml" in files:
            index_paths.append((Path(directory) / "index.yaml").relative_to(root))
    index_paths.sort()
    for path in index_paths:
        for position, item in enumerate(entries(path, load(path), "cases", "a list of case entries"), 1):
            location = f"case {position}"
            if not kind(path, item, dict, location, "make each case a mapping with an id and a promises list"):
                continue
            valid = [field(path, item, "id", str, location, "a string naming the case"),
                     field(path, item, "promises", list, location, "a list of promise IDs")]
            if not all(valid):
                continue
            valid_tags = [kind(path, tag, str, f"{location}'s promise {n}", "write each promise ID as a string")
                          for n, tag in enumerate(item["promises"], 1)]
            if all(valid_tags):
                cases.append(item)
    if not cases:
        error(CONFORMANCE, "no index.yaml with cases found, so no case was counted")

    retired = "conformance/expected-failures.yaml"
    if os.path.lexists(root / retired):
        error(retired, "retired; move its entries to a fragment under "
              "conformance/expected-failures/ named for the order (DEC-190)")
    expected, expected_origins = set(), {}
    expected_count, fragment_count = 0, 0
    try:
        fragments = sorted((root / EXPECTED_FAILURES).iterdir(), key=lambda path: os.fsencode(path.name))
    except FileNotFoundError:
        error(EXPECTED_FAILURES + "/", "not found; the conformance runner requires it (DEC-190)")
        fragments = []
    except OSError:
        error(EXPECTED_FAILURES + "/",
              "could not read directory; restore a readable directory with its README.md")
        fragments = []
    for fragment in fragments:
        name = fragment.name
        if name == "README.md" or name.startswith("."):
            continue
        path = fragment.relative_to(root).as_posix()
        if (not re.fullmatch(r"[a-z0-9][a-z0-9.-]*\.yaml", name)
                or name in ("index.yaml", "case.yaml") or fragment.is_symlink() or not fragment.is_file()):
            error(path, "not a fragment; a fragment is a regular file directly in "
                  "conformance/expected-failures/, named as its order's file with .yaml for .md (DEC-190)")
            continue
        fragment_count += 1
        expected_data = entries(path, load(path), "cases",
                                "a list of entries, each a mapping with a case's id")
        expected_count += len(expected_data)
        for position, item in enumerate(expected_data, 1):
            location = f"case {position}"
            if (kind(path, item, dict, location, "make each entry a mapping with a case's id")
                    and field(path, item, "id", str, location, "a string naming the case")):
                identifier = item["id"]
                if identifier in expected_origins:
                    first_path, first_position = expected_origins[identifier]
                    error(path, f"{location} ({identifier}) is listed again, after case "
                          f"{first_position} of {first_path}; keep only one entry for this ID")
                else:
                    expected_origins[identifier] = (path, position)
                    expected.add(identifier)

    s1_data = entries(MANIFEST, load(MANIFEST, "not found, so no S1 row was read"), "s1_coverage",
                      "a list of S1 rows")
    s1 = {}
    for position, item in enumerate(s1_data, 1):
        location = f"S1 row {position}"
        if not kind(MANIFEST, item, dict, location,
                    "make each S1 row a mapping with a promise, a vectors list and, if vectors is empty, a reason"):
            continue
        valid = [field(MANIFEST, item, "promise", str, location, "a string naming the promise"),
                 field(MANIFEST, item, "vectors", list, location, "a list of case IDs")]
        if "reason" in item:
            valid.append(field(MANIFEST, item, "reason", str, location, "a string giving the reason"))
        if not all(valid):
            continue
        if not item["vectors"] and not item.get("reason", "").strip():
            error(MANIFEST, f"{location} ({item['promise']}) has neither cases nor a reason; provide nonempty vectors or a reason")
            continue
        s1.setdefault(item["promise"], "cases" if item["vectors"] else "reason")

    sources = {crate: [] for crate in CRATES}
    source_count = 0
    for crate, directory in zip(CRATES, SOURCE_DIRS):
        if not (root / directory).is_dir():
            error(directory, "not found, so no source of the crate was read")
            continue
        for path in sorted((root / directory).rglob("*.rs")):
            if not path.is_file():
                continue
            source_count += 1
            try:
                sources[crate].append(path.read_text(encoding="utf-8"))
            except (OSError, UnicodeError):
                error(path.relative_to(root), "could not read UTF-8 source; provide a readable UTF-8 source file")

    if errors:
        for message in errors:
            print(message)
        print(f"{TABLE} was not {'written' if args.write else 'compared'}.")
        print(f"Errors: {len(errors)}")
        return 1

    case_counts = Counter(tag for case in cases for tag in set(case["promises"]))
    failure_counts = Counter(tag for case in cases if case["id"] in expected for tag in set(case["promises"]))
    rows = []
    for identifier in sorted(promises, key=lambda value: value.encode("utf-8")):
        count, failures = case_counts[identifier], failure_counts[identifier]
        inventory = s1.get(identifier, "outside")
        pattern = re.compile(r"(?<![A-Za-z0-9])" + re.escape(identifier) + r"(?![0-9])")
        source = ", ".join(crate for crate in CRATES if any(pattern.search(text) for text in sources[crate])) or "none"
        standing = (STANDINGS[0] if count > failures else STANDINGS[1] if count >= 1
                    else STANDINGS[2] if inventory == "reason" else STANDINGS[3])
        rows.append(dict(promises[identifier], cases=count, failures=failures,
                         s1=inventory, source=source, standing=standing))
    standings = Counter(row["standing"] for row in rows)
    print(f"Read {len(rows)} promises in {len(promise_paths)} files under spec/, {len(cases)} cases in "
          f"{len(index_paths)} index.yaml files under conformance/, {expected_count} entries in "
          f"{fragment_count} fragments under {EXPECTED_FAILURES}/, {len(s1_data)} S1 rows of {MANIFEST} "
          f"and {source_count} source files "
          "of the crates loader, core and shell.")
    print("Standings: " + "; ".join(f"{name} {standings[name]}" for name in STANDINGS) + ".")
    print(f"Named in the engine's source: {sum(row['source'] != 'none' for row in rows)} of {len(rows)}.")
    generated = render(rows).encode("utf-8")
    target = root / TABLE
    if args.write:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(generated)
        print(f"Wrote {TABLE}; promise rows: {len(rows)}.")
    else:
        try:
            committed = target.read_bytes()
        except FileNotFoundError:
            error(TABLE, "not found; run python3 tools/promise_standing.py --write and commit it")
            print(errors[-1])
        except OSError:
            error(TABLE, "could not read table; provide a readable file and run python3 tools/promise_standing.py --write and commit it")
            print(errors[-1])
        else:
            if committed != generated:
                error(TABLE, "differs from what its inputs give; run python3 tools/promise_standing.py --write and commit it")
                print(errors[-1])
                difference = list(difflib.unified_diff(
                    committed.decode("utf-8", errors="replace").splitlines(), generated.decode("utf-8").splitlines(),
                    fromfile=f"{TABLE} (committed)", tofile=f"{TABLE} (generated)", n=0, lineterm=""))
                if not difference:
                    print(f"{TABLE}: every line matches the generated table; the bytes differ only in line "
                          "ends or the final newline (the table is written with LF line ends and exactly one "
                          "final newline)")
                for line in difference[:40]:
                    print(line)
                if len(difference) > 40:
                    print(f"({len(difference) - 40} more lines of difference not shown)")
            else:
                print(f"{TABLE} is current; promise rows: {len(rows)}.")
    print(f"Errors: {len(errors)}")
    return 1 if errors else 0


if __name__ == "__main__":
    fallback = "/usr/bin/python3"
    if (yaml is None and os.environ.get("CONDUIT_PROMISE_STANDING_REEXEC") != "1"
            and os.path.exists(fallback)
            and os.path.realpath(sys.executable) != os.path.realpath(fallback)):
        os.environ["CONDUIT_PROMISE_STANDING_REEXEC"] = "1"
        os.execv(fallback, [fallback, str(Path(__file__).resolve()), *sys.argv[1:]])
    sys.exit(main(Path(__file__).resolve().parent.parent, sys.argv[1:]))
