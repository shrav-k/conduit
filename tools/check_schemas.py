#!/usr/bin/env python3
"""Check the proposed JSON Schemas, the core package and the example documents.

    python3 tools/check_schemas.py

What it checks, and reports under these numbers:

  1. Every YAML file under spec/core/ and spec/schemas/ is strict YAML as SCH-012, SCH-017 and
     SCH-006 require: no duplicate key, anchor, alias, merge key, tag or key that is not a
     string; no number without a JSON form; no whole number outside the I-JSON range.
  2. Every Schema contract under spec/schemas/ and spec/core/ uses only the keywords and the
     formats that spec/schema-profile.md permits, places `default` only where SCH-008 accepts
     it, and uses `$ref` only in the two forms of SCH-015, each of which resolves. `schema.v1`,
     the profile written as a schema, lists exactly the profile's keywords and formats. A
     schema written inside another document is checked through `schema.v1` in step 3, and a
     qualified `$ref` in it is resolved in step 5.
  3. Every document under spec/core/, spec/schemas/ and spec/schemas/examples/valid/ is valid
     against `document.v1`, and every manifest (a file named conduit.yaml) against
     `manifest.v1`, with a JSON Schema 2020-12 validator and format checking on.
  4. Every document under spec/schemas/examples/invalid/ is rejected, and its file is named
     for a promise that exists.
  5. The core package agrees with itself: its manifest exports exactly its documents, no name
     is used twice, names follow NAM-004 and OBJ-001, every reference into the core package
     names an export of the kind the place expects, and the table "Every export" of
     spec/core/README.md lists exactly what the package holds.
     The same reference check is made on `#name` references inside each example directory.

What it does not check: anything that needs an engine. It does not resolve references to
packages that are not in this repository, instantiate a template, or evaluate any of the
cross-document rules that spec/schemas/README.md lists as checked by the engine.

It needs PyYAML and jsonschema. If the interpreter that runs it lacks either, it runs itself
again under /usr/bin/python3. If PyYAML is still missing it stops and says that nothing was
checked. If only jsonschema is missing it runs steps 1, 2 and 5, says in capitals that no
document was validated against a schema, and exits with status 3. It never reports a check it
did not run. `pattern` is evaluated by Python's `re`,
not by an ECMA-262 engine (SCH-016); the patterns in these schemas use only syntax the two
share.
"""
import copy
import os
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
FALLBACK = "/usr/bin/python3"

try:
    import yaml
    import jsonschema
except ImportError as missing:
    if (os.environ.get("CONDUIT_CHECK_SCHEMAS_REEXEC") != "1" and os.path.exists(FALLBACK)
            and os.path.realpath(sys.executable) != os.path.realpath(FALLBACK)):
        os.environ["CONDUIT_CHECK_SCHEMAS_REEXEC"] = "1"
        os.execv(FALLBACK, [FALLBACK] + sys.argv)
    try:
        import yaml
    except ImportError:
        print(f"{sys.executable}: {missing}.")
        print("PyYAML is needed to read any file. NOTHING WAS CHECKED.")
        sys.exit(2)
    jsonschema = None                  # steps 3 and 4 are then not run, and the tool says so

FAMILIES = ("Interface", "Agent", "Resource", "Pool", "Conduit")
CONTRACTS = ("Type", "Capability", "Operation", "Requirement", "Schema")
SAFE_INT = 9007199254740991
NAME = re.compile(r"^[a-z][a-z0-9-]*$")
EXPORT = re.compile(r"^[a-z][a-z0-9-]*(\.[a-z][a-z0-9-]*)*$")
VERSIONED = re.compile(r"^([a-z][a-z0-9-]*\.)*v[1-9][0-9]*$")
LOCAL_REF = re.compile(r"^#([a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)*)$")

errors = []


def fail(message):
    errors.append(message)


def rel(path):
    return pathlib.Path(path).relative_to(ROOT).as_posix()


# --- 1. Strict YAML: the YAML 1.2 core schema, narrowed as SCH-012 and SCH-017 require.

class Refused(Exception):
    pass


class StrictLoader(yaml.SafeLoader):
    yaml_implicit_resolvers = {}

    def construct_mapping(self, node, deep=False):
        seen = set()
        for key_node, _ in node.value:
            if key_node.tag != "tag:yaml.org,2002:str":
                line = key_node.start_mark.line + 1
                raise Refused(f"line {line}: a key that is not a string")
            key = key_node.value
            if key == "<<" and key_node.style is None:
                raise Refused(f"line {key_node.start_mark.line + 1}: a merge key")
            if key in seen:
                raise Refused(f"line {key_node.start_mark.line + 1}: duplicate key {key!r}")
            seen.add(key)
        return super().construct_mapping(node, deep)


def _int(loader, node):
    text = node.value
    if re.match(r"^0[ox]", text):
        raise Refused(f"line {node.start_mark.line + 1}: {text} has no JSON form (SCH-017)")
    value = int(text)
    if abs(value) > SAFE_INT:
        raise Refused(f"line {node.start_mark.line + 1}: {text} is outside the range of SCH-006")
    return value


def _float(loader, node):
    text = node.value
    if re.search(r"inf|nan", text, re.I):
        raise Refused(f"line {node.start_mark.line + 1}: {text} has no JSON form (SCH-017)")
    value = float(text)
    if value == int(value) and abs(value) > SAFE_INT:
        raise Refused(f"line {node.start_mark.line + 1}: {text} is outside the range of SCH-006")
    return value


for _tag, _pattern, _first in (
    ("null", r"^(?:~|null|Null|NULL|)$", ["~", "n", "N", ""]),
    ("bool", r"^(?:true|True|TRUE|false|False|FALSE)$", list("tTfF")),
    ("int", r"^(?:[-+]?[0-9]+|0o[0-7]+|0x[0-9a-fA-F]+)$", list("-+0123456789")),
    ("float", r"^(?:[-+]?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)(?:[eE][-+]?[0-9]+)?"
              r"|[-+]?\.(?:inf|Inf|INF)|\.(?:nan|NaN|NAN))$", list("-+0123456789.")),
):
    StrictLoader.add_implicit_resolver("tag:yaml.org,2002:" + _tag, re.compile(_pattern), _first)
StrictLoader.add_constructor("tag:yaml.org,2002:int", _int)
StrictLoader.add_constructor("tag:yaml.org,2002:float", _float)


def load(path):
    """Return the documents of one file, or raise Refused."""
    text = pathlib.Path(path).read_text()
    try:
        for event in yaml.parse(text, Loader=yaml.SafeLoader):
            line = event.start_mark.line + 1
            if isinstance(event, yaml.AliasEvent):
                raise Refused(f"line {line}: an alias")
            if getattr(event, "anchor", None):
                raise Refused(f"line {line}: an anchor")
            if getattr(event, "tag", None) is not None:
                raise Refused(f"line {line}: a tag")
        return [d for d in yaml.load_all(text, Loader=StrictLoader)]
    except yaml.YAMLError as error:
        raise Refused(str(error).replace("\n", " "))


# --- The names and the profile, read from the specification itself.

def names():
    text = (ROOT / "spec" / "names.yaml").read_text()
    keys = ("corePackage", "apiVersion")
    return {k: re.search(rf"(?m)^{k}: *(\S+)", text).group(1) for k in keys}


def profile():
    text = (ROOT / "spec" / "schema-profile.md").read_text()

    def first_paragraph(title):
        match = re.search(rf"(?ms)^## {re.escape(title)}\n\n(.*?)\n\n", text)
        if not match:
            sys.exit(f"spec/schema-profile.md: no section {title!r}. NOTHING WAS CHECKED.")
        return set(re.findall(r"`([^`]+)`", match.group(1)))

    return first_paragraph("Permitted schema keywords"), first_paragraph("Enforced formats")


# --- 2. Schemas: a walk that knows which members of a schema are schemas.

def walk(schema, where, visit, place="plain"):
    """Visit every schema object. `place` is plain, property or combined (SCH-008)."""
    if isinstance(schema, bool):
        return
    if not isinstance(schema, dict):
        fail(f"{where}: a schema is an object or a boolean")
        return
    visit(schema, where, place)
    inside = place == "combined"
    for keyword in ("properties", "$defs"):
        members = schema.get(keyword, {})
        if not isinstance(members, dict):
            fail(f"{where}/{keyword}: not a map")
            continue
        for name, sub in members.items():
            below = "combined" if inside else ("property" if keyword == "properties" else "plain")
            walk(sub, f"{where}/{keyword}/{name}", visit, below)
    for keyword in ("additionalProperties", "items"):
        if keyword in schema:
            walk(schema[keyword], f"{where}/{keyword}", visit, "combined" if inside else "plain")
    for keyword in ("allOf", "anyOf", "oneOf"):
        for index, sub in enumerate(schema.get(keyword, [])):
            walk(sub, f"{where}/{keyword}/{index}", visit, "combined")


def pointer(root, text):
    """Resolve `#` and a JSON Pointer inside one schema; return (found, value)."""
    value = root
    for token in text[1:].split("/")[1:]:
        token = token.replace("~1", "/").replace("~0", "~")
        if isinstance(value, dict) and token in value:
            value = value[token]
        else:
            return False, None
    return True, value


def check_profile(label, root, keywords, formats, core, schemas):
    def visit(schema, where, place):
        for keyword in schema:
            if keyword not in keywords:
                fail(f"{where}: keyword `{keyword}` is outside the profile (SCH-002)")
        if "format" in schema and schema["format"] not in formats:
            fail(f"{where}: format `{schema['format']}` is outside the profile (SCH-003)")
        if "default" in schema and place != "property":
            fail(f"{where}: `default` is not on a property reached through `properties`, or is "
                 f"inside a combiner (SCH-008)")
        ref = schema.get("$ref")
        if ref is None:
            return
        if not isinstance(ref, str):
            fail(f"{where}: `$ref` is not a string")
        elif ref == "#" or ref.startswith("#/"):
            if not pointer(root, ref)[0]:
                fail(f"{where}: `$ref` {ref} does not resolve in the same schema (SCH-015)")
        elif ref.startswith(core + "#"):
            if ref.split("#", 1)[1] not in schemas:
                fail(f"{where}: `$ref` {ref} is not a Schema contract of the core package "
                     f"(SCH-015)")
        else:
            fail(f"{where}: `$ref` {ref} is neither `#` and a JSON Pointer nor a qualified "
                 f"reference this check can resolve (SCH-015)")
    walk(root, label, visit)


def bundle(schemas, core):
    """One JSON Schema that holds every Schema contract under `$defs`, with each `$ref`
    rewritten to point inside it. A qualified reference stands for the `spec` of the contract
    it names (SCH-015), which is what the rewriting does."""
    defs = {}
    for name, spec in schemas.items():
        spec = copy.deepcopy(spec)

        def visit(schema, where, place, name=name):
            ref = schema.get("$ref")
            if isinstance(ref, str):
                if ref == "#" or ref.startswith("#/"):
                    schema["$ref"] = f"#/$defs/{name}{ref[1:]}"
                elif ref.startswith(core + "#"):
                    schema["$ref"] = "#/$defs/" + ref.split("#", 1)[1]
        before = len(errors)
        walk(spec, name, visit)
        del errors[before:]            # already reported by check_profile
        defs[name] = spec
    return {"$schema": "https://json-schema.org/draft/2020-12/schema", "$defs": defs}


def validator(bundled, name):
    schema = dict(bundled)
    schema["$ref"] = "#/$defs/" + name
    return jsonschema.Draft202012Validator(
        schema, format_checker=jsonschema.Draft202012Validator.FORMAT_CHECKER)


def explain(error):
    """Say why a document is not valid. Where a schema offers alternatives, follow the one the
    document was most plainly meant for: the one whose `kind` or `source` it has, and then the
    one it breaks in the fewest places."""
    while error.context:
        branches = {}
        for sub in error.context:
            branches.setdefault(sub.relative_schema_path[0], []).append(sub)

        def score(subs):
            other = any(s.validator in ("const", "enum")
                        and list(s.relative_path) in (["kind"], ["source"]) for s in subs)
            wrong_type = any(s.validator == "type" and not s.relative_path for s in subs)
            return (other, wrong_type, len(subs), -max(len(s.absolute_path) for s in subs))
        subs = min(branches.values(), key=score)
        error = max(subs, key=lambda s: (bool(s.context), len(s.absolute_path)))
    at = "/" + "/".join(str(p) for p in error.absolute_path)
    message = error.message if len(error.message) <= 150 else error.message[:147] + "..."
    return f"at {at}: {message}"


def expected(doc, is_manifest):
    if is_manifest:
        return "manifest.v1"
    kind = doc.get("kind") if isinstance(doc, dict) else None
    if kind in CONTRACTS:
        return "contract.v1"
    if kind in FAMILIES:
        return "instance.v1" if "from" in doc else "template.v1"
    return "document.v1"


def validate(bundled, doc, is_manifest):
    """Return None if valid, else one line saying why not."""
    top = "manifest.v1" if is_manifest else "document.v1"
    if not is_manifest:
        outer = list(validator(bundled, "envelope.v1").iter_errors(doc))
        if outer:
            return explain(outer[0])
    found = sorted(validator(bundled, top).iter_errors(doc), key=lambda e: list(e.absolute_path))
    if not found:
        return None
    narrow = expected(doc, is_manifest)
    if narrow != top:
        closer = list(validator(bundled, narrow).iter_errors(doc))
        if closer:
            return explain(closer[0])
        return f"valid as {narrow} and not as {top}: " + explain(found[0])
    return explain(found[0])


# --- 5. References between documents that can be checked without an engine.

EXPECT = (
    # kind of the document, path to the value, kind the reference must name
    ("Capability", ("spec", "operations", "*"), "Operation"),
    ("Type", ("spec", "offers", "*"), "Capability"),
    ("Type", ("spec", "requires", "*"), "Operation"),
    ("Type", ("spec", "controls", "*"), "Operation"),
    ("Type", ("spec", "placement", "*", "requirement"), "Requirement"),
    ("Type", ("spec", "runtimes", "*", "start"), "Operation"),
    ("Type", ("spec", "runtimes", "*", "stop"), "Operation"),
    ("Operation", ("spec", "implementation"), "implementation"),
    ("Requirement", ("spec", "constituents", "*", "requirement"), "Requirement"),
    ("template", ("spec", "type"), "Type"),
    ("template", ("spec", "capability"), "Capability"),
    ("template", ("spec", "operations", "*"), "Operation"),
    ("template", ("spec", "requirements", "*", "requirement"), "Requirement"),
    ("template", ("spec", "runtimes", "*", "start"), "Operation"),
    ("template", ("spec", "runtimes", "*", "stop"), "Operation"),
    ("template", ("spec", "setup", "steps", "*", "operation"), "Operation"),
    ("template", ("spec", "retire", "steps", "*", "operation"), "Operation"),
    ("manifest", ("implementations", "*", "implements", "*"), "Operation"),
    ("manifest", ("implementations", "*", "supports", "*"), "Requirement"),
    ("manifest", ("implementations", "*", "placement", "*", "requirement"), "Requirement"),
)
EXPECT_KEYS = (
    # the keys of these maps are references too
    ("template", ("spec", "implementation"), "Operation"),
    ("template", ("spec", "controls"), "Operation"),
    ("Type", ("spec", "implementation"), "Operation"),
    ("manifest", ("implementations", "*", "tools"), "Operation"),
)


def reach(value, path):
    if not path:
        yield value
    elif path[0] == "*":
        for item in (value.values() if isinstance(value, dict) else value
                     if isinstance(value, list) else ()):
            yield from reach(item, path[1:])
    elif isinstance(value, dict) and path[0] in value:
        yield from reach(value[path[0]], path[1:])


def check_references(label, role, doc, packages, local_kinds, counts):
    """`packages` maps an address to what that package exports, name -> kind."""
    def one(ref, want, where):
        if not isinstance(ref, str):
            return
        local = LOCAL_REF.match(ref)
        address, _, export = ref.partition("#")
        if local:
            got = local_kinds.get(local.group(1))
        elif address in packages:
            got = packages[address].get(export)
        else:
            counts["skipped"] += 1
            return
        counts["checked"] += 1
        if got is None:
            fail(f"{label}: {where}: {ref} names nothing")
        elif got != want:
            fail(f"{label}: {where}: {ref} is a {got}, and a {want} is expected")

    def refs(value):
        """Every `$ref` that is not a pointer into its own schema names a Schema contract."""
        if isinstance(value, dict):
            ref = value.get("$ref")
            if isinstance(ref, str) and not (ref == "#" or ref.startswith("#/")):
                one(ref, "Schema", "$ref")
            for item in value.values():
                refs(item)
        elif isinstance(value, list):
            for item in value:
                refs(item)

    refs(doc)

    for kind, path, want in EXPECT:
        if kind == role:
            for ref in reach(doc, path):
                one(ref, want, "/".join(path))
    for kind, path, want in EXPECT_KEYS:
        if kind == role:
            for mapping in reach(doc, path):
                if isinstance(mapping, dict):
                    for key, value in mapping.items():
                        one(key, want, "/".join(path) + " key")
                        if path[-1] == "implementation" and isinstance(value, str):
                            one(value, "implementation", "/".join(path) + " value")


def role_of(doc, is_manifest):
    if is_manifest:
        return "manifest"
    kind = doc.get("kind")
    if kind in FAMILIES:
        return "instance" if "from" in doc else "template"
    return kind


def kinds_in(files):
    """name -> kind for every document and implementation in these files."""
    out, twice = {}, []
    for path, docs in files:
        for doc in docs:
            if not isinstance(doc, dict):
                continue
            if path.name == "conduit.yaml":
                entries = [(n, "implementation") for n in (doc.get("implementations") or {})]
            elif "from" in doc:
                continue                   # an instance is not reached by `#name`
            else:
                entries = [((doc.get("metadata") or {}).get("name"), doc.get("kind"))]
            for name, kind in entries:
                if name in out:
                    twice.append(name)
                out[name] = kind
    return out, twice


def main():
    from importlib import metadata
    validating = jsonschema is not None
    print(f"Python {sys.version.split()[0]} at {sys.executable}; PyYAML {yaml.__version__}; "
          f"jsonschema {metadata.version('jsonschema') if validating else 'NOT IMPORTABLE'}")
    named = names()
    core = named["corePackage"]
    keywords, formats = profile()
    if validating:
        checkers = jsonschema.Draft202012Validator.FORMAT_CHECKER.checkers
        unchecked = sorted(f for f in formats if f not in checkers)
        state = ("Format checkers available for all of them." if not unchecked
                 else "Format checkers MISSING for " + ", ".join(unchecked) + ".")
    else:
        state = "No validator, so no format is checked."
    print(f"Profile: {len(keywords)} keywords, {len(formats)} formats. {state}")

    schema_dir, core_dir = ROOT / "spec" / "schemas", ROOT / "spec" / "core"
    valid_dir = schema_dir / "examples" / "valid"
    invalid_dir = schema_dir / "examples" / "invalid"

    # 1. Strict YAML.
    loaded = {}
    groups = {
        "schemas": sorted(schema_dir.glob("*.yaml")),
        "core": sorted(core_dir.glob("*.yaml")),
        "valid": sorted(valid_dir.rglob("*.yaml")),
        "invalid": sorted(invalid_dir.rglob("*.yaml")),
    }
    for group, paths in groups.items():
        for path in paths:
            try:
                loaded[path] = load(path)
            except Refused as refused:
                if group == "invalid":
                    loaded[path] = refused
                else:
                    fail(f"{rel(path)}: not strict YAML: {refused}")
    count = sum(len(v) for v in loaded.values() if isinstance(v, list))
    print(f"1. Read {len(loaded)} files holding {count} documents as strict YAML.")

    # 2. The profile.
    schemas = {}
    for group in ("schemas", "core"):
        for path in groups[group]:
            for doc in loaded.get(path, []):
                if isinstance(doc, dict) and doc.get("kind") == "Schema":
                    schemas[doc["metadata"]["name"]] = doc["spec"]
    for name, spec in schemas.items():
        check_profile(f"{name}", spec, keywords, formats, core, schemas)
    # `schema.v1` is the profile written as a schema. It must list what the profile lists.
    meta = schemas.get("schema.v1", {}).get("$defs", {})
    for name, without in (("schema", {"default"}), ("property", set()), ("combined", {"default"})):
        try:
            have = set(meta[name]["anyOf"][1]["properties"])
        except (KeyError, IndexError, TypeError):
            have = set()
        if have != keywords - without:
            fail(f"schema.v1: `{name}` does not list exactly the profile's keywords: "
                 f"{sorted(have ^ (keywords - without))}")
    if set(meta.get("format", {}).get("enum", [])) != formats:
        fail("schema.v1: `format` does not list exactly the profile's formats")
    bundled = bundle(schemas, core)
    if validating:
        try:
            jsonschema.Draft202012Validator.check_schema(bundled)
        except jsonschema.SchemaError as error:
            fail(f"the schemas are not valid JSON Schema 2020-12: {error.message}")
    print(f"2. Checked {len(schemas)} Schema contracts against the profile, and every `$ref`.")
    if errors:
        return finish(validating)

    # 3. Valid documents.
    checked = 0
    if not validating:
        print("3. NOT RUN: jsonschema is not importable. NO DOCUMENT WAS VALIDATED AGAINST A "
              "SCHEMA.")
        print("4. NOT RUN: no invalid example was shown to be rejected.")
    for group in ("schemas", "core", "valid") if validating else ():
        for path in groups[group]:
            for index, doc in enumerate(loaded.get(path, [])):
                checked += 1
                why = validate(bundled, doc, path.name == "conduit.yaml")
                if why:
                    fail(f"{rel(path)} document {index + 1}: not valid: {why}")
                if isinstance(doc, dict) and doc.get("apiVersion") != named["apiVersion"]:
                    fail(f"{rel(path)} document {index + 1}: apiVersion is not the one "
                         f"spec/names.yaml states")
    if validating:
        print(f"3. Validated {checked} documents under spec/schemas/, spec/core/ and "
              f"examples/valid/.")

    # 4. Invalid documents.
    promises = set()
    for path in (ROOT / "spec").glob("*/*.yaml"):
        promises.update(re.findall(r"(?m)^- id: ([A-Z]+-[0-9]+)$", path.read_text()))
    rejected = 0
    for path in groups["invalid"]:
        match = re.match(r"^([A-Z]+-[0-9]+)-", path.name)
        if not match or match.group(1) not in promises:
            fail(f"{rel(path)}: not named for a promise that exists")
        if not validating:
            continue
        docs = loaded[path]
        if isinstance(docs, Refused):
            rejected += 1
            print(f"   rejected {path.name}: when read: {docs}")
            continue
        for doc in docs:
            why = validate(bundled, doc, "-manifest-" in path.name)
            if why is None:
                fail(f"{rel(path)}: accepted, and it must be rejected")
            else:
                rejected += 1
                print(f"   rejected {path.name}: {why}")
    if validating:
        print(f"4. {rejected} invalid documents in {len(groups['invalid'])} files were each "
              f"rejected.")

    # 5. The core package and the references that can be checked here.
    core_files = [(p, loaded[p]) for g in ("schemas", "core") for p in groups[g]]
    core_kinds, twice = kinds_in(core_files)
    for name in twice:
        fail(f"core package: the name {name} is used twice (PKG-012)")
    manifest = next((d[0] for p, d in core_files if p.name == "conduit.yaml"), {})
    exports = list(manifest.get("exports") or [])
    documents = {n for n, k in core_kinds.items() if k != "implementation"}
    for name in sorted(set(exports) - documents):
        fail(f"spec/core/conduit.yaml: exports {name}, which is not a document of the package")
    for name in sorted(documents - set(exports)):
        fail(f"spec/core/conduit.yaml: does not export the document {name}")
    for name, kind in core_kinds.items():
        if not EXPORT.match(name or "") or len(name) > 63:
            fail(f"core package: {name} is not an export name (NAM-004)")
        elif kind in CONTRACTS and not VERSIONED.match(name):
            fail(f"core package: the contract {name} has no version segment (CTR-001)")
        elif kind in FAMILIES and not NAME.match(name):
            fail(f"core package: the template {name} breaks OBJ-001")
    readme = core_dir / "README.md"
    if readme.exists():
        section = re.search(r"(?ms)^## Every export\n(.*?)(?=^## |\Z)", readme.read_text())
        listed = set(re.findall(r"(?m)^\| `([^`]+)` \|", section.group(1) if section else ""))
        for name in sorted(set(core_kinds) - listed):
            fail(f"spec/core/README.md: the table omits {name}")
        for name in sorted(listed - set(core_kinds)):
            fail(f"spec/core/README.md: the table lists {name}, which the package does not hold")
    else:
        fail("spec/core/README.md is missing")

    # Each directory under examples/valid/ is one project. One whose manifest states an
    # `address` is a package, and what it exports can be referred to from the others.
    packages = {core: core_kinds}
    by_dir = {}
    for path in groups["valid"]:
        by_dir.setdefault(path.parent, []).append((path, loaded.get(path, [])))
    local = {}
    for directory, files in by_dir.items():
        local[directory], twice = kinds_in(files)
        for name in twice:
            fail(f"{rel(directory)}: the name {name} is used twice")
        for path, docs in files:
            if path.name == "conduit.yaml" and docs and isinstance(docs[0], dict):
                address = docs[0].get("address")
                listed = set(docs[0].get("exports") or [])
                if address:
                    packages[address] = {n: k for n, k in local[directory].items()
                                         if n in listed or k == "implementation"}
                    for name in sorted(n for n in listed if "#" not in n
                                       and n not in local[directory]):
                        fail(f"{rel(path)}: exports {name}, which is not a document of the "
                             f"package")
    counts = {"checked": 0, "skipped": 0}
    for path, docs in core_files:
        for doc in docs:
            if isinstance(doc, dict):
                check_references(rel(path), role_of(doc, path.name == "conduit.yaml"), doc,
                                 packages, {}, counts)
    for directory, files in by_dir.items():
        for path, docs in files:
            for doc in docs:
                if isinstance(doc, dict):
                    check_references(rel(path), role_of(doc, path.name == "conduit.yaml"), doc,
                                     packages, local[directory], counts)
    used = set()
    for path in list(loaded):
        used.update(re.findall(re.escape(core) + r"#([a-z0-9.-]*[a-z0-9])", path.read_text()))
    for name in sorted(used - set(core_kinds)):
        fail(f"a file refers to {core}#{name}, which the core package does not hold")
    print(f"5. Core package: {len(exports)} exports and {len(core_kinds) - len(documents)} "
          f"implementation. Checked {counts['checked']} references for the kind they name; "
          f"{counts['skipped']} name packages that are not in this repository and were not "
          f"checked.")
    return finish(validating)


def finish(validating):
    for error in errors:
        print("ERROR " + error)
    if not validating:
        print(f"Errors: {len(errors)} in the steps that ran. PARTIAL: steps 3 and 4 were not "
              f"run.")
        return 1 if errors else 3
    print(f"Errors: {len(errors)}")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
