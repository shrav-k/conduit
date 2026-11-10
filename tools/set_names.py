#!/usr/bin/env python3
"""Check or change the names the specification assigns.

The names (the core package's address, a document's `apiVersion`, the prefix of Conduit's
`_meta` keys, and, since ruling 43, the names of what the input `identity` of `run.v1` carries)
are defined once, in spec/names.yaml, and written out in full wherever the specification uses
them. This tool keeps the two in step.

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
          - a name that appears nowhere;
          - a variable whose name begins as those of `identity` do, with `CONDUIT_`, and that
            is neither of them nor an explicitly allowed build test switch.
        The check reads text and knows nothing of YAML. It takes a token of the form
        `<dotted name>/<key>` to be a `_meta` key, and an address whose last part is the
        core package's last part to be meant as the core package.

    python3 tools/set_names.py --domain example.org
    python3 tools/set_names.py --github owner/repo
    python3 tools/set_names.py --core-package A --api-version B --meta-prefix C
    python3 tools/set_names.py --identity certificateFile=cert.pem
        Show what would change. Add --write to rewrite every file under the given roots
        (default: spec, conformance, reference-model, packages) and spec/names.yaml itself. A variable
        of `identity` is rewritten wherever it stands as a whole word; a file of `identity`
        only where it is written in backticks, and not under spec/schemas/examples/, whose
        examples use a word such as `token` for other things. Each line in which a file name
        would change is shown, since a sentence may name another system's file of that name,
        such as Kubernetes' own `token`: read them before adding --write.

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
# The names of what the input `identity` of `run.v1` carries (ruling 43, Q1): two variables and
# the files that the run finds in the directory that the second names.
VARIABLE_KEYS = ("engineUrlVariable", "identityDirVariable")
# Round 5 of the re-audit (M86) renamed trustBundleFile to x509BundleFile: the trust bundle is
# that file and jwtBundleFile's.
FILE_KEYS = ("x509BundleFile", "jwtBundleFile", "engineIdFile", "tokenFile",
             "certificateFile", "keyFile")
IDENTITY_KEYS = VARIABLE_KEYS + FILE_KEYS
VARIABLE = re.compile(r"[A-Z][A-Z0-9]*_[A-Z0-9_]*[A-Z0-9]")
FILE_NAME = re.compile(r"[A-Za-z0-9_][A-Za-z0-9._-]*")
# A file name of `identity` may be a common word, such as `token`, which the examples also use
# for other things. It is counted and rewritten only where it is written in backticks, and not
# under this directory.
EXAMPLES = ROOT / "spec" / "schemas" / "examples"
DEFAULT_ROOTS = ("spec", "conformance", "reference-model", "packages")
TEXT_SUFFIXES = {".yaml", ".yml", ".md", ".json", ".rs", ".toml", ".txt"}
# The file that defines Conduit's `_meta` keys. The list of keys is read from it.
INTERFACE = ROOT / "spec" / "interface" / "implementation.yaml"
# Prefixes of `_meta` keys that are not Conduit's: the Model Context Protocol's own.
FOREIGN_META_PREFIXES = ("io.modelcontextprotocol/",)
# Build-only switches are not run identity variables. Keep this exception exact so
# other CONDUIT_ spellings, including identity typos, still fail the check.
BUILD_TEST_VARIABLES = {"CONDUIT_TEST_NPM"}
# A run of characters that can make up a name, a reference or a path.
TOKEN = re.compile(r"[A-Za-z0-9._/#:-]+")
DNS_NAME = re.compile(r"[a-z0-9]([a-z0-9-]*[a-z0-9])?(\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)+")
# A key, or the name of an extension, after the prefix: a word, or words joined by hyphens,
# as in the extension `implementation-interface`.
WORD = re.compile(r"[A-Za-z][A-Za-z0-9]*(?:-[A-Za-z0-9]+)*")
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
        elif key in KEYS + IDENTITY_KEYS:
            names[key] = value
    missing = [k for k in KEYS + IDENTITY_KEYS if not names.get(k)]
    if missing:
        sys.exit(f"{NAMES}: missing {', '.join(missing)}")
    return names, retired


def occurrence(key: str, name: str) -> re.Pattern:
    """Where a name of `identity` stands: a variable as a whole word, not inside a longer one;
    a file name only in backticks."""
    if key in VARIABLE_KEYS:
        return re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")
    return re.compile("`" + re.escape(name) + "`")


def counts_for(key: str, path: pathlib.Path) -> bool:
    return key in VARIABLE_KEYS or EXAMPLES not in path.parents


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


def read(p: pathlib.Path) -> str:
    """The text of a file as UTF-8. A byte that is not UTF-8, as in the example under
    spec/schemas/examples/invalid/ that shows such a file rejected (SUB-005), is kept as it is,
    so that a file written back holds the same bytes but for the names changed."""
    return p.read_text(encoding="utf-8", errors="surrogateescape")


def write_back(p: pathlib.Path, text: str) -> None:
    p.write_text(text, encoding="utf-8", errors="surrogateescape")


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
    found = re.findall(re.escape(prefix) + "(" + WORD.pattern + ")", INTERFACE.read_text())
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
    identity = {k: 0 for k in IDENTITY_KEYS}
    variables = {names[k] for k in VARIABLE_KEYS}
    stems = {v.split("_", 1)[0] + "_" for v in variables}
    for p in files(roots):
        if p == NAMES:
            continue
        text = read(p)
        rel = p.relative_to(ROOT)
        for k in seen:
            seen[k] += text.count(names[k])
        for k in identity:
            if counts_for(k, p):
                identity[k] += len(occurrence(k, names[k]).findall(text))
        for n, line in enumerate(text.splitlines(), 1):
            found = []
            for m in VARIABLE.finditer(line):
                token = m.group(0)
                if (any(token.startswith(s) for s in stems)
                        and token not in variables and token not in BUILD_TEST_VARIABLES):
                    found.append(f"variable {token!r} begins as those of `identity` do and is "
                                 f"neither {' nor '.join(sorted(variables))}")
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
    for k, count in list(seen.items()) + list(identity.items()):
        if count == 0:
            errors.append(f"spec/names.yaml: {k} = {names[k]!r} appears nowhere under {', '.join(roots)}")
    for e in errors:
        print(e)
    print(f"Names: corePackage x{seen['corePackage']}, apiVersion x{seen['apiVersion']}, "
          f"metaPrefix x{seen['metaPrefix']}; of `identity`, "
          + ", ".join(f"{names[k]} x{identity[k]}" for k in IDENTITY_KEYS)
          + f". Errors: {len(errors)}")
    return 1 if errors else 0


def change(new: dict[str, str], roots: list[str], write: bool) -> int:
    old, retired = read_names()
    # Longest first, so that the repository name does not rewrite part of a longer name.
    pairs = sorted(((old[k], new[k]) for k in KEYS if old[k] != new[k]), key=lambda p: -len(p[0]))
    renamed = [(old[k], new[k]) for k in IDENTITY_KEYS if old[k] != new[k]]
    if not pairs and not renamed:
        print("Nothing to change.")
        return 0
    for o, n in pairs + renamed:
        print(f"  {o}  ->  {n}")
    total = 0
    for p in files(roots):
        text = read(p)
        out = text
        if p == NAMES:
            continue
        hits = 0
        for o, n in pairs:
            hits += out.count(o)
            out = out.replace(o, n)
        for k in IDENTITY_KEYS:
            if old[k] != new[k] and counts_for(k, p):
                pattern = occurrence(k, old[k])
                if k in FILE_KEYS:
                    # A file name may also be another system's, such as Kubernetes' own
                    # `token`: each line is shown, to be read before --write.
                    for n, line in enumerate(out.splitlines(), 1):
                        if pattern.search(line):
                            print(f"    {p.relative_to(ROOT)}:{n}: {line.strip()}")
                replacement = new[k] if k in VARIABLE_KEYS else f"`{new[k]}`"
                out, count = pattern.subn(replacement, out)
                hits += count
        if out != text:
            total += hits
            print(f"{'rewrote' if write else 'would rewrite'} {p.relative_to(ROOT)} ({hits})")
            if write:
                write_back(p, out)
    if write:
        text = NAMES.read_text()
        for k in KEYS + IDENTITY_KEYS:
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
    ap.add_argument("--identity", action="append", default=[], metavar="KEY=VALUE",
                    help="a name of `identity`: " + ", ".join(IDENTITY_KEYS))
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--roots", nargs="*", default=list(DEFAULT_ROOTS))
    args = ap.parse_args()
    if args.check:
        return check(args.roots)
    old, _ = read_names()
    new = derive(args, old)
    for item in args.identity:
        key, _, value = item.partition("=")
        if key not in IDENTITY_KEYS:
            sys.exit(f"--identity: {key!r} is not one of {', '.join(IDENTITY_KEYS)}")
        shape = VARIABLE if key in VARIABLE_KEYS else FILE_NAME
        if not shape.fullmatch(value):
            what = "variable" if key in VARIABLE_KEYS else "file name"
            sys.exit(f"--identity: {value!r} is not a {what}")
        new[key] = value
    return change(new, args.roots, args.write)


if __name__ == "__main__":
    sys.exit(main())
