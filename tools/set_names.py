#!/usr/bin/env python3
"""Check or change the names the specification assigns.

The names (the core package's address, a document's `apiVersion`, and the prefix of Conduit's
`_meta` keys) are defined once, in spec/names.yaml, and written out in full wherever the
specification uses them. This tool keeps the two in step.

    python3 tools/set_names.py --check
        Verify that spec/ agrees with spec/names.yaml. Exit 1 if not.

    python3 tools/set_names.py --domain example.org
    python3 tools/set_names.py --github owner/repo
    python3 tools/set_names.py --core-package A --api-version B --meta-prefix C
        Show what would change. Add --write to rewrite every file under the given roots
        (default: spec, conformance, reference-model) and spec/names.yaml itself.

No third-party modules are needed.
"""
from __future__ import annotations

import argparse
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
NAMES = ROOT / "spec" / "names.yaml"
KEYS = ("repository", "corePackage", "apiVersion", "metaPrefix")
DEFAULT_ROOTS = ("spec", "conformance", "reference-model")
TEXT_SUFFIXES = {".yaml", ".yml", ".md", ".json", ".rs", ".toml", ".txt"}


def read_names() -> tuple[dict[str, str], list[str]]:
    names: dict[str, str] = {}
    retired: list[str] = []
    in_retired = False
    for raw in NAMES.read_text().splitlines():
        line = raw.split("#", 1)[0].rstrip() if raw.lstrip().startswith("#") else raw.rstrip()
        if not line.strip():
            continue
        if in_retired and line.lstrip().startswith("- "):
            retired.append(line.split("- ", 1)[1].strip())
            continue
        in_retired = False
        key, _, value = line.partition(":")
        key, value = key.strip(), value.strip()
        if key == "retired":
            in_retired = True
        elif key in KEYS:
            names[key] = value
    missing = [k for k in KEYS if not names.get(k)]
    if missing:
        sys.exit(f"{NAMES}: missing {', '.join(missing)}")
    return names, retired


def reverse_dns(host_and_path: str) -> str:
    host, _, path = host_and_path.partition("/")
    labels = list(reversed(host.split("."))) + [p for p in path.split("/") if p]
    return ".".join(labels) + "/"


def derive(args: argparse.Namespace, old: dict[str, str]) -> dict[str, str]:
    new = dict(old)
    version = old["apiVersion"].rsplit("/", 1)[1]
    base = None
    if args.domain:
        base = args.domain.strip("/")
        new["metaPrefix"] = reverse_dns(base)
    if args.github:
        owner, _, repo = args.github.strip("/").partition("/")
        if not owner or not repo:
            sys.exit("--github takes owner/repo")
        base = f"github.com/{owner}/{repo}"
        # The Model Context Protocol registry's namespace for publishers verified through GitHub.
        new["metaPrefix"] = f"io.github.{owner}.{repo}/"
    if base:
        new["repository"] = base
        new["corePackage"] = f"{base}/core"
        new["apiVersion"] = f"{base}/{version}"
    if args.core_package:
        new["corePackage"] = args.core_package
    if args.api_version:
        new["apiVersion"] = args.api_version
    if args.meta_prefix:
        new["metaPrefix"] = args.meta_prefix if args.meta_prefix.endswith("/") else args.meta_prefix + "/"
    return new


def files(roots: list[str]) -> list[pathlib.Path]:
    out = []
    for r in roots:
        base = ROOT / r
        if not base.exists():
            continue
        for p in sorted(base.rglob("*")):
            if p.is_file() and p.suffix in TEXT_SUFFIXES and "target" not in p.parts:
                out.append(p)
    return out


def check(roots: list[str]) -> int:
    names, retired = read_names()
    errors = []
    seen = {k: 0 for k in ("corePackage", "apiVersion", "metaPrefix")}
    # Anything that looks like one of Conduit's `_meta` keys or a core reference must use the
    # current names.
    meta_key = re.compile(r"([A-Za-z0-9.-]+/)(interfaceVersion|operationIdentity|restrictions|serving)\b")
    for p in files(roots):
        if p == NAMES:
            continue
        text = p.read_text()
        rel = p.relative_to(ROOT)
        for k in seen:
            seen[k] += text.count(names[k])
        for n, line in enumerate(text.splitlines(), 1):
            for old in retired:
                if old in line:
                    errors.append(f"{rel}:{n}: retired name {old!r}")
            for m in meta_key.finditer(line):
                if m.group(1) != names["metaPrefix"]:
                    errors.append(f"{rel}:{n}: `_meta` key {m.group(0)!r} does not use {names['metaPrefix']!r}")
    for k, count in seen.items():
        if count == 0:
            errors.append(f"spec/names.yaml: {k} = {names[k]!r} appears nowhere under {', '.join(roots)}")
    for e in errors:
        print(e)
    print(f"Names: corePackage x{seen['corePackage']}, apiVersion x{seen['apiVersion']}, "
          f"metaPrefix x{seen['metaPrefix']}. Errors: {len(errors)}")
    return 1 if errors else 0


def change(new: dict[str, str], roots: list[str], write: bool) -> int:
    old, retired = read_names()
    # Longest first, so that the repository name does not rewrite part of a longer name.
    pairs = sorted(((old[k], new[k]) for k in KEYS if old[k] != new[k]), key=lambda p: -len(p[0]))
    if not pairs:
        print("Nothing to change.")
        return 0
    for o, n in pairs:
        print(f"  {o}  ->  {n}")
    total = 0
    for p in files(roots):
        text = p.read_text()
        out = text
        if p == NAMES:
            continue
        hits = 0
        for o, n in pairs:
            hits += out.count(o)
            out = out.replace(o, n)
        if out != text:
            total += hits
            print(f"{'rewrote' if write else 'would rewrite'} {p.relative_to(ROOT)} ({hits})")
            if write:
                p.write_text(out)
    if write:
        text = NAMES.read_text()
        for k in KEYS:
            text = re.sub(rf"(?m)^{k}: .*$", f"{k}: {new[k]}", text)
        # Retire the old repository name and the old prefix; the longer names contain them.
        # A name that is current again is no longer retired.
        current = {new["repository"], new["metaPrefix"].rstrip("/")}
        keep = [r for r in retired if r not in current]
        for o in (old["repository"], old["metaPrefix"].rstrip("/")):
            if o not in current and o not in keep:
                keep.append(o)
        head = text.split("\nretired:", 1)[0].rstrip("\n")
        text = head + "\nretired:\n" + "".join(f"  - {r}\n" for r in keep)
        NAMES.write_text(text)
        print(f"Updated {NAMES.relative_to(ROOT)}. {total} occurrences rewritten.")
        return check(roots)
    print(f"{total} occurrences would be rewritten. Add --write to apply.")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--domain", help="a domain the project controls, such as example.org")
    ap.add_argument("--github", help="owner/repo on GitHub")
    ap.add_argument("--core-package")
    ap.add_argument("--api-version")
    ap.add_argument("--meta-prefix")
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--roots", nargs="*", default=list(DEFAULT_ROOTS))
    args = ap.parse_args()
    if args.check:
        return check(args.roots)
    old, _ = read_names()
    return change(derive(args, old), args.roots, args.write)


if __name__ == "__main__":
    sys.exit(main())
