#!/usr/bin/env python3
"""Check or change the names the specification assigns.

The names (the core package's address, a document's `apiVersion`, and the prefix of Conduit's
`_meta` keys) are defined once, in spec/names.yaml, and written out in full wherever the
specification uses them. This tool keeps the two in step.

    python3 tools/set_names.py --check
        Verify that spec/ agrees with spec/names.yaml. Exit 1 if not. It reports:
          - a retired name, anywhere;
          - one of Conduit's `_meta` keys written with another prefix. The keys are read from
            spec/interface/implementation.yaml, so a key added there is checked everywhere;
          - a value that looks like one of the three names and is not it: an `apiVersion`
            with another value; an address that ends as the core package's or the
            `apiVersion`'s does, or that begins with the repository, and is neither; and a
            key under the prefix, or a prefix close to it, that the specification does not
            define;
          - a name that appears nowhere.
        The check reads text and knows nothing of YAML. It takes a token of the form
        `<dotted name>/<key>` to be a `_meta` key, and an address whose last part is the
        core package's last part to be meant as the core package.

    python3 tools/set_names.py --domain example.org
    python3 tools/set_names.py --github owner/repo
    python3 tools/set_names.py --core-package A --api-version B --meta-prefix C
        Show what would change. Add --write to rewrite every file under the given roots
        (default: spec, conformance, reference-model) and spec/names.yaml itself.

No third-party modules are needed.
"""
from __future__ import annotations

import argparse
import difflib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
NAMES = ROOT / "spec" / "names.yaml"
KEYS = ("repository", "corePackage", "apiVersion", "metaPrefix")
DEFAULT_ROOTS = ("spec", "conformance", "reference-model")
TEXT_SUFFIXES = {".yaml", ".yml", ".md", ".json", ".rs", ".toml", ".txt"}
# The file that defines Conduit's `_meta` keys. The list of keys is read from it.
INTERFACE = ROOT / "spec" / "interface" / "implementation.yaml"
# Prefixes of `_meta` keys that are not Conduit's: the Model Context Protocol's own.
FOREIGN_META_PREFIXES = ("io.modelcontextprotocol/",)
# A run of characters that can make up a name, a reference or a path.
TOKEN = re.compile(r"[A-Za-z0-9._/#:-]+")
DNS_NAME = re.compile(r"[a-z0-9]([a-z0-9-]*[a-z0-9])?(\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+")
WORD = re.compile(r"[A-Za-z][A-Za-z0-9]*")
API_VERSION_VALUE = re.compile(r"\bapiVersion: *['\"]?([^\s,}`'\"]+)")
# How close a dotted prefix must be to the current one to count as a misspelling of it.
CLOSE = 0.8


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


def meta_keys(prefix: str) -> list[str]:
    """The names of Conduit's `_meta` keys: every word the interface file writes after the prefix."""
    if not INTERFACE.exists():
        sys.exit(f"{INTERFACE}: missing; the `_meta` keys are read from it")
    found = re.findall(re.escape(prefix) + r"([A-Za-z][A-Za-z0-9]*)", INTERFACE.read_text())
    keys = sorted(set(found))
    if not keys:
        sys.exit(f"{INTERFACE}: no `_meta` key with the prefix {prefix!r}")
    return keys


def look_alikes(token: str, names: dict[str, str], keys: list[str], in_interface: bool) -> list[str]:
    """Say why a token looks like one of the three names without being it."""
    core, api, prefix, repo = names["corePackage"], names["apiVersion"], names["metaPrefix"], names["repository"]
    if "://" in token:  # A URL is not a name.
        return []
    token = token.rstrip(".:")
    head = token.split("#", 1)[0]
    if not head or head in (core, api, repo, prefix):
        return []
    first, slash, rest = head.partition("/")
    if not slash or not DNS_NAME.fullmatch(first):
        return []
    # One dotted name, then one word or nothing: the shape of a `_meta` key.
    if "/" not in rest:
        if first + "/" == prefix:
            if rest not in keys:
                return [f"{head!r} is not a `_meta` key that {INTERFACE.relative_to(ROOT)} defines"]
            return []
        if first + "/" in FOREIGN_META_PREFIXES:
            return []
        if rest in keys:
            return [f"`_meta` key {head!r} does not use {prefix!r}"]
        if rest == "" or WORD.fullmatch(rest):
            if in_interface and rest:
                # In the interface file a dotted name and one word is a `_meta` key.
                return [f"{head!r} is read as a `_meta` key, and does not use {prefix!r}"]
            if difflib.SequenceMatcher(None, first + "/", prefix).ratio() >= CLOSE:
                return [f"{head!r} looks like the prefix {prefix!r} and differs from it"]
        if rest == "":
            return []
    # An address: a host part and a path.
    last = head.rsplit("/", 1)[1]
    if last == core.rsplit("/", 1)[1]:
        return [f"{head!r} looks like the core package's address {core!r} and differs from it"]
    if last == api.rsplit("/", 1)[1]:
        return [f"{head!r} looks like the `apiVersion` {api!r} and differs from it"]
    if head.startswith(repo + "/"):
        return [f"{head!r} begins with the repository {repo!r} and is neither {core!r} nor {api!r}"]
    return []


def check(roots: list[str]) -> int:
    names, retired = read_names()
    keys = meta_keys(names["metaPrefix"])
    errors = []
    seen = {k: 0 for k in ("corePackage", "apiVersion", "metaPrefix")}
    for p in files(roots):
        if p == NAMES:
            continue
        text = p.read_text()
        rel = p.relative_to(ROOT)
        for k in seen:
            seen[k] += text.count(names[k])
        for n, line in enumerate(text.splitlines(), 1):
            found = []
            for old in retired:
                if old in line:
                    found.append(f"retired name {old!r}")
            # The value of an `apiVersion` key, wherever a document is written out.
            for m in API_VERSION_VALUE.finditer(line):
                value = m.group(1).rstrip(".")
                if value != names["apiVersion"] and not value.startswith(("{", "<", "$")):
                    found.append(f"`apiVersion` is {value!r}, not {names['apiVersion']!r}")
            for m in TOKEN.finditer(line):
                found.extend(look_alikes(m.group(0), names, keys, p == INTERFACE))
            for message in dict.fromkeys(found):
                errors.append(f"{rel}:{n}: {message}")
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
