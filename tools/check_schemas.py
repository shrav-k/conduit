#!/usr/bin/env python3
"""Check the JSON Schemas of documents, the core package and the example documents.

    python3 tools/check_schemas.py

What it checks, and reports under these numbers:

  1. Every YAML file under spec/core/ and spec/schemas/ is strict YAML as SCH-012, SCH-017,
     SCH-006 and SCH-021 require: no duplicate key, anchor, alias, merge key, tag or key that
     is not a string; no number without a JSON form; no whole number outside the I-JSON range;
     and no other number outside the range of IEEE 754 binary64.
  2. Every Schema contract under spec/schemas/ and spec/core/ uses only the keywords and the
     formats that spec/schema-profile.md permits, places `default` only where SCH-008 accepts
     it, which is never inside `allOf`, `anyOf`, `oneOf`, `not`, `if`, `then`, `else` or
     `dependentSchemas`, and uses `$ref` only in the three forms of SCH-015, each of which
     resolves: `#` and a JSON Pointer into the same schema, a qualified reference to a Schema
     contract, and `#name` for a Schema contract of the same package. `schema.v1`, the
     profile written as a schema, lists exactly the profile's keywords and formats. A schema
     written inside another document is checked through `schema.v1` in step 3, and a `$ref`
     in it that is not a pointer is resolved in step 5.
  3. Every document under spec/core/, spec/schemas/ and spec/schemas/examples/valid/ is valid
     against `document.v1`, and every manifest (a file named conduit.yaml) against
     `manifest.v1`, with a JSON Schema 2020-12 validator and format checking on.
  4. Every document under spec/schemas/examples/invalid/ is rejected by a schema, or is
     rejected when its file is read, and its file is named for a promise that exists.
  5. The core package agrees with itself: every export in its manifest is a document of the
     package, every document is exported or is listed in PRIVATE below, no name is used
     twice, names follow NAM-004, CTR-001 and OBJ-001, every reference into the core package
     names an export of the kind the place expects, every control has a `changes` that maps
     inputs it declares to places its type's schema describes, its `inputSchema` requires
     `revision`, and it declares no output `settings` (CTR-006),
     the operations of REV-012 take no `revision`, `replace.v1` returns one (SUB-009), no
     implementation has the name of a document of its project or package (NAM-027), and the
     tables of spec/core/README.md and spec/schemas/README.md list exactly the files and the
     exports there are, each export with its kind and with promises that exist. The same
     reference check is made on every `#name`, a `$ref` and a `from` among them, inside each
     example directory, and every promise ID cited under spec/core/, spec/schemas/ and in
     spec/promises/manifest.yaml is the ID of a promise that exists. A directory under
     examples/valid/ whose manifest states `address` and `version` is a package (PKG-017),
     whose exports the others may refer to; one that holds a contract or lists an
     implementation and states no `address` breaks MAN-015.
  6. Rules that no schema can express and that this tool applies itself, each named in RULES
     below with the promise it comes from. The core package and every directory under
     examples/valid/ break none of them. Every file under examples/invalid-by-rule/ is one
     set of documents, each of them valid against the schemas, that the rule its file is
     named for rejects. A file there with `-manifest-` in its name holds a manifest, which is
     valid against `manifest.v1` and which a rule on manifests rejects. The rule for SCH-019
     validates a `default` against its schema, so it needs jsonschema; without it the rule is
     not applied, and the tool says so. The rules that follow a template's selections into
     an implementation item read the items from the manifests of the core package and of the
     directories under examples/valid/, and from the set's own manifest where it has one.
     examples/valid/first-start/ writes out the documents that first start makes, which no
     submission may hold (FST-006), so the rule for FST-006 is not applied to it.

What it does not check: anything that needs an engine. It does not resolve references to
packages that are not in this repository, instantiate a template, or evaluate any of the
cross-document rules that spec/schemas/README.md lists as checked by the engine, other than
the few of step 6. A rule of step 6 is applied as far as the documents in front of the tool
show, which each rule's comment states; it is not the engine's check.

It needs PyYAML and jsonschema. If the interpreter that runs it lacks either, it runs itself
again under /usr/bin/python3. If PyYAML is still missing it stops and says that nothing was
checked. If only jsonschema is missing it runs steps 1, 2, 5 and 6, says in capitals that no
document was validated against a schema, and exits with status 3. It never reports a check it
did not run. `pattern` is evaluated by Python's `re`,
not by an ECMA-262 engine (SCH-016); the patterns in these schemas use only syntax the two
share.
"""
import copy
import decimal
import math
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
VERSIONED = re.compile(r"^([a-z][a-z0-9-]*\.)+v[1-9][0-9]*$")
LOCAL_REF = re.compile(r"^#([a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)*)$")

# Documents of the core package that it deliberately does not export. There is none: every
# document of the core package is exported. A name listed here must be a document of the
# package and must not be among its exports.
PRIVATE = frozenset()

# The operations that REV-012 says name no revision and advance none.
NO_REVISION = ("set-up.v1", "retire.v1", "retire-with-resources.v1", "open.v1", "close.v1")

# The operations that return the revision of the object they changed (SUB-009).
RETURNS_REVISION = ("replace.v1",)

# The provider capabilities of the core package (PRV-001), and the three whose operations no
# Conduit lists, because the engine alone invokes them by provider calls (PRV-004, PRV-005).
PROVIDER_CAPABILITIES = ("runtime-provider.v1", "machine-provider.v1", "package-source.v1",
                         "secret-reveal.v1")
CALLED_CAPABILITIES = ("runtime-provider.v1", "machine-provider.v1", "package-source.v1")
CALLED_OPERATIONS = ("run.v1", "stop.v1", "inspect-run.v1", "list-machines.v1", "place.v1",
                     "fetch-package.v1")

# The core templates from which first start alone makes instances (FST-001, FST-006), and the
# directory under examples/valid/ that writes those instances out to show what first start
# makes. That directory is a picture of the engine's own work and not a submission, so the
# rule for FST-006 is not applied to it; every other rule is.
FIRST_START_TEMPLATES = ("system", "location", "management", "access", "root")
FIRST_START_DIRECTORY = "first-start"

# The prefixes of the promise files that a ruling removed, whose IDs no file this tool covers
# may cite: GRA, of spec/promises/graphs.yaml, whose promises ruling 41 removed.
REMOVED_PREFIXES = ("GRA",)

# Where a schema may hold the core package's `secret-reference.v1` (SEC-001): as the schema
# of a place, which `properties`, `additionalProperties` and a member of `$defs` give, or as
# `items`. Under `anyOf` or `oneOf` only beside branches that are `{type: null}`. Under any
# other applicator it may not stand.
SECRET_PLACES = ("properties", "additionalProperties", "items", "$defs")

# The rules of step 6, each with the promise it comes from. A file under
# examples/invalid-by-rule/ is named for one of these promises.
RULES = {
    "one-name": "NAM-027",
    "bound-once": "CON-028",
    "bound-target": "CON-028",
    "reserved-mark": "CTR-009",
    "entry-reference": "POOL-005",
    "operations-filled": "CON-016",
    "ref": "SCH-015",
    "default-valid": "SCH-019",
    "implementation-selected": "TPL-004",
    "names-nothing": "NAM-028",
    "core-address": "NAM-021",
    "first-start-from": "FST-006",
    "location-type": "PLC-028",
    "one-run": "TPL-019",
    "label-conflict": "PLC-025",
    "item-place-required": "MAN-017",
    "project-address": "MAN-015",
    "secret-branch": "SEC-001",
    "provider-family": "PRV-001",
    "provider-conduit": "PRV-004",
}
# The rules that validate a value against a schema, and so are applied only with jsonschema.
NEED_VALIDATOR = frozenset({"default-valid"})

errors = []


def fail(message):
    errors.append(message)


def rel(path):
    return pathlib.Path(path).relative_to(ROOT).as_posix()


# --- 1. Strict YAML: the YAML 1.2 core schema, narrowed as SCH-012 and SCH-017 require.

class NotStrict(Exception):
    pass


class StrictLoader(yaml.SafeLoader):
    yaml_implicit_resolvers = {}

    def construct_mapping(self, node, deep=False):
        seen = set()
        for key_node, _ in node.value:
            if key_node.tag != "tag:yaml.org,2002:str":
                line = key_node.start_mark.line + 1
                raise NotStrict(f"line {line}: a key that is not a string")
            key = key_node.value
            if key == "<<" and key_node.style is None:
                raise NotStrict(f"line {key_node.start_mark.line + 1}: a merge key")
            if key in seen:
                raise NotStrict(f"line {key_node.start_mark.line + 1}: duplicate key {key!r}")
            seen.add(key)
        return super().construct_mapping(node, deep)


def _int(loader, node):
    text = node.value
    if re.match(r"^0[ox]", text):
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} has no JSON form (SCH-017)")
    value = int(text)
    if abs(value) > SAFE_INT:
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} is outside the range of SCH-006")
    return value


def _float(loader, node):
    text = node.value
    if re.search(r"inf|nan", text, re.I):
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} has no JSON form (SCH-017)")
    try:
        number = decimal.Decimal(text)
        whole = number == number.to_integral_value()
    except decimal.DecimalException:
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} cannot be read as a number")
    if whole and abs(number) > SAFE_INT:
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} is outside the range of SCH-006")
    value = float(text)
    if math.isinf(value):
        raise NotStrict(f"line {node.start_mark.line + 1}: {text} is outside the range of IEEE 754 "
                      f"binary64 (SCH-021)")
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
    """Return the documents of one file, or raise NotStrict."""
    text = pathlib.Path(path).read_text()
    try:
        for event in yaml.parse(text, Loader=yaml.SafeLoader):
            line = event.start_mark.line + 1
            if isinstance(event, yaml.AliasEvent):
                raise NotStrict(f"line {line}: an alias")
            if getattr(event, "anchor", None):
                raise NotStrict(f"line {line}: an anchor")
            if getattr(event, "tag", None) is not None:
                raise NotStrict(f"line {line}: a tag")
        return [d for d in yaml.load_all(text, Loader=StrictLoader)]
    except yaml.YAMLError as error:
        raise NotStrict(str(error).replace("\n", " "))


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

    return (first_paragraph("Schema keywords in the profile"),
            first_paragraph("Formats that are checked"))


# --- 2. Schemas: a walk that knows which members of a schema are schemas.

# The keywords whose value is one schema, a map of schemas or a list of schemas, and which
# give no default to what they hold: a `default` inside them is rejected (SCH-008).
CONDITIONAL = ("not", "if", "then", "else")
BRANCHES = ("allOf", "anyOf", "oneOf")
ONE_SCHEMA = ("additionalProperties", "items", "contains", "propertyNames")
MAP_OF_SCHEMAS = ("properties", "$defs", "patternProperties", "dependentSchemas")


def subschemas(schema):
    """(keyword, token, sub) for each schema directly inside `schema`, where token is the name
    or the index that reaches it under the keyword, or None for a keyword that holds one."""
    for keyword in MAP_OF_SCHEMAS:
        members = schema.get(keyword)
        for name, sub in (members.items() if isinstance(members, dict) else ()):
            yield keyword, name, sub
    for keyword in ONE_SCHEMA + CONDITIONAL:
        if keyword in schema:
            yield keyword, None, schema[keyword]
    for keyword in BRANCHES + ("prefixItems",):
        branches = schema.get(keyword)
        for index, sub in enumerate(branches if isinstance(branches, list) else ()):
            yield keyword, index, sub


def walk(schema, where, visit, place="other"):
    """Visit every schema object. `place` is other, property or combined (SCH-008): a member
    of `properties` is a property, anything inside one of the eight keywords of SCH-008 is
    combined, and anything else is other."""
    if isinstance(schema, bool):
        return
    if not isinstance(schema, dict):
        fail(f"{where}: a schema is a map or a boolean")
        return
    visit(schema, where, place)
    inside = place == "combined"
    for keyword in MAP_OF_SCHEMAS:
        if keyword in schema and not isinstance(schema[keyword], dict):
            fail(f"{where}/{keyword}: not a map")
    for keyword, token, sub in subschemas(schema):
        at = f"{where}/{keyword}" + ("" if token is None else f"/{token}")
        if inside or keyword in BRANCHES + CONDITIONAL + ("dependentSchemas",):
            below = "combined"
        else:
            below = "property" if keyword == "properties" else "other"
        walk(sub, at, visit, below)


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


def is_pointer(ref):
    """True for `#` and a JSON Pointer into the same schema, the first form of SCH-015."""
    return ref == "#" or ref.startswith("#/")


def check_profile(label, root, keywords, formats, core, schemas):
    """`schemas` holds the Schema contracts of the package the schema belongs to, which is the
    core package: a qualified reference to it and `#name` resolve among them."""
    def visit(schema, where, place):
        for keyword in schema:
            if keyword not in keywords:
                fail(f"{where}: keyword `{keyword}` is outside the profile (SCH-002)")
        if "format" in schema and schema["format"] not in formats:
            fail(f"{where}: format `{schema['format']}` is outside the profile (SCH-003)")
        if "default" in schema and place != "property":
            fail(f"{where}: `default` is not on a property reached through `properties`, or is "
                 f"inside `allOf`, `anyOf` or `oneOf` (SCH-008)")
        ref = schema.get("$ref")
        if ref is None:
            return
        if not isinstance(ref, str):
            fail(f"{where}: `$ref` is not a string")
        elif is_pointer(ref):
            if not pointer(root, ref)[0]:
                fail(f"{where}: `$ref` {ref} does not resolve in the same schema (SCH-015)")
        elif LOCAL_REF.match(ref):
            if ref[1:] not in schemas:
                fail(f"{where}: `$ref` {ref} is not a Schema contract of the same package "
                     f"(SCH-015)")
        elif ref.startswith(core + "#"):
            if ref.split("#", 1)[1] not in schemas:
                fail(f"{where}: `$ref` {ref} is not a Schema contract of the core package "
                     f"(SCH-015)")
        else:
            fail(f"{where}: `$ref` {ref} is not `#` and a JSON Pointer, `#name`, or a "
                 f"qualified reference this check can resolve (SCH-015)")
    walk(root, label, visit)


def bundle(schemas, core):
    """One JSON Schema that holds every Schema contract under `$defs`, with each `$ref`
    rewritten to point inside it. A qualified reference, and `#name` within the package,
    stands for the `spec` of the contract it names (SCH-015), which is what the rewriting
    does."""
    defs = {}
    for name, spec in schemas.items():
        spec = copy.deepcopy(spec)

        def visit(schema, where, place, name=name):
            ref = schema.get("$ref")
            if isinstance(ref, str):
                if is_pointer(ref):
                    schema["$ref"] = f"#/$defs/{name}{ref[1:]}"
                elif LOCAL_REF.match(ref):
                    schema["$ref"] = "#/$defs/" + ref[1:]
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
    document was most plainly meant for: the one whose `kind` it has, and then the one it
    breaks in the fewest places."""
    while error.context:
        branches = {}
        for sub in error.context:
            branches.setdefault(sub.relative_schema_path[0], []).append(sub)

        def score(subs):
            other = any(s.validator in ("const", "enum")
                        and list(s.relative_path) == ["kind"] for s in subs)
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
    ("Operation", ("spec", "implementation"), "implementation"),
    ("template", ("spec", "type"), "Type"),
    ("template", ("spec", "capability"), "Capability"),
    ("template", ("spec", "operations", "*"), "Operation"),
    ("template", ("spec", "requirements", "*", "requirement"), "Requirement"),
    ("manifest", ("implementations", "*", "implements", "*"), "Operation"),
    ("manifest", ("implementations", "*", "supports", "*"), "Requirement"),
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


def check_references(role, doc, packages, local_kinds, counts):
    """Return what is wrong with the references of one document, as (rule, message) pairs.
    `packages` maps an address to what that package exports, name -> kind, and `local_kinds`
    does the same for the project or package the document belongs to, which `#name` reaches
    (NAM-022). The rule is "ref" for a `$ref` (SCH-015), "names-nothing" for a selection, or a
    `from` written as a qualified reference, that names nothing in a package this tool has or
    in the document's own project or package (NAM-028), and None for any other fault. A `from`
    written `#name` is not such a selection: in a submitted project it resolves among the
    templates that exist (NAM-022)."""
    found = []

    def one(ref, want, where, rule=None, absent="names-nothing"):
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
            found.append((rule or absent, f"{where}: {ref} names nothing"))
        elif got != want:
            found.append((rule, f"{where}: {ref} names something of the kind {got}, and the kind "
                                f"{want} is expected"))

    def refs(value):
        """Every `$ref` that is not a pointer into its own schema names a Schema contract."""
        if isinstance(value, dict):
            ref = value.get("$ref")
            if isinstance(ref, str) and not is_pointer(ref):
                one(ref, "Schema", "$ref", "ref")
            for item in value.values():
                refs(item)
        elif isinstance(value, list):
            for item in value:
                refs(item)

    refs(doc)

    def template(ref, want, where):
        """A `from` that is a qualified reference or `#name` names a template of the kind the
        place makes. A bare name or `{kind, name}` names a template that exists, which only
        an engine can look up, and is not checked here."""
        if isinstance(ref, str) and "#" in ref:
            one(ref, want, where, absent=None if ref.startswith("#") else "names-nothing")

    if role == "instance":
        template(doc.get("from"), doc.get("kind"), "from")
    if role == "template":
        spec = doc.get("spec") if isinstance(doc.get("spec"), dict) else {}
        for key, want in (("conduits", "Conduit"), ("entries", "Resource")):
            members = spec.get(key) if isinstance(spec.get(key), dict) else {}
            for name, member in members.items():
                if isinstance(member, dict):
                    template(member.get("from"), want, f"spec/{key}/{name}/from")

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
    return found


def role_of(doc, is_manifest):
    if is_manifest:
        return "manifest"
    kind = doc.get("kind")
    if kind in FAMILIES:
        return "instance" if "from" in doc else "template"
    return kind


def named(doc):
    metadata = doc.get("metadata")
    return metadata.get("name") if isinstance(metadata, dict) else None


def kinds_in(files):
    """name -> kind for everything in these files that `#name` reaches: every template, every
    contract and every implementation. An instance is not reached by `#name`. Also returns
    the names that an implementation shares with a document or with another implementation;
    two documents that share a name are found by the rule "one-name" of step 6."""
    out, twice = {}, []
    for path, docs in files:
        for doc in docs:
            if not isinstance(doc, dict):
                continue
            if path.name == "conduit.yaml":
                for name in (doc.get("implementations") or {}):
                    if name in out:
                        twice.append(name)
                    out[name] = "implementation"
            elif "from" not in doc:
                name = named(doc)
                if out.get(name) == "implementation":
                    twice.append(name)
                out[name] = doc.get("kind")
    return out, twice


def contents(files):
    """name -> document, for every template and contract in these files."""
    return {named(doc): doc for path, docs in files for doc in docs
            if isinstance(doc, dict) and path.name != "conduit.yaml" and "from" not in doc}


def members_of(body, key):
    """The properties of an Operation contract's `inputSchema` or `outputSchema`, and the
    names its `required` lists."""
    schema = body.get(key) if isinstance(body.get(key), dict) else {}
    found = schema.get("properties") if isinstance(schema.get("properties"), dict) else {}
    listed = schema.get("required") if isinstance(schema.get("required"), list) else []
    return found, listed


def check_controls(label, docs, package_docs, address):
    """For every control that a Type in `docs` lists and whose contract is in this repository:
    it is an Operation with `changes`, a map in which each key is the name of an input the
    control declares and each value is a pointer to a place the type's `schema` describes
    through `properties`; its `inputSchema` lists `revision` under `required`, and it declares
    no output `settings` (CTR-006). Other inputs may be listed under `required` too: a control
    may leave them out all the same (REV-006)."""
    count = 0
    for doc in docs:
        if not isinstance(doc, dict) or doc.get("kind") != "Type":
            continue
        spec = doc.get("spec") or {}
        for ref in spec.get("controls") or []:
            if not isinstance(ref, str):
                continue
            where, _, name = ref.partition("#")
            control = package_docs.get(where or address, {}).get(name)
            if control is None or control.get("kind") != "Operation":
                continue                   # not in this repository, or reported by step 5
            count += 1
            at = f"{label}: {named(doc)} lists the control {ref}, which"
            body = control.get("spec") or {}
            inputs, required = members_of(body, "inputSchema")
            outputs, _ = members_of(body, "outputSchema")
            changes = body.get("changes")
            if not isinstance(changes, dict) or not changes:
                fail(f"{at} has no `changes` that maps an input to the pointer it sets "
                     f"(CTR-006)")
                changes = {}
            for source, place in changes.items():
                if source not in inputs:
                    fail(f"{at} gives a pointer in `changes` to {source}, which is not an "
                         f"input it declares (CTR-006)")
                if not isinstance(place, str):
                    continue               # not a pointer: reported by step 3
                schema = spec.get("schema")
                for token in place.split("/")[1:]:
                    token = token.replace("~1", "/").replace("~0", "~")
                    schema = (schema.get("properties") or {}).get(token) \
                        if isinstance(schema, dict) else None
                    if schema is None:
                        fail(f"{at} changes {place}, a place the type's schema does not "
                             f"describe (CTR-006)")
                        break
            if "revision" not in inputs or "revision" not in required:
                fail(f"{at} does not require the input `revision` in its `inputSchema` "
                     f"(CTR-006)")
            if "settings" in outputs:
                fail(f"{at} declares the output `settings`, which a control does not "
                     f"(CTR-006)")
    return count


def check_readmes(core_dir, schema_dir, core_kinds, groups, loaded):
    """The tables of the two READMEs list exactly the files and the exports there are."""
    readme = core_dir / "README.md"
    if not readme.exists():
        fail("spec/core/README.md is missing")
        return
    text = readme.read_text()
    section = re.search(r"(?ms)^## Every export\n(.*?)(?=^## |\Z)", text)
    rows = re.findall(r"(?m)^\| `([^`]+)` \| ([^|]+) \| (.+) \|$",
                      section.group(1) if section else "")
    listed = {name for name, _, _ in rows}
    for name in sorted(set(core_kinds) - listed):
        fail(f"spec/core/README.md: the table omits {name}")
    for name in sorted(listed - set(core_kinds)):
        fail(f"spec/core/README.md: the table lists {name}, which the package does not hold")
    for name, cell, requires in rows:
        kind = core_kinds.get(name)
        want = None if kind is None else ("Implementation" if kind == "implementation"
                                           else f"{kind} template" if kind in FAMILIES else kind)
        if want and not cell.strip().startswith(want):
            fail(f"spec/core/README.md: the table gives {name} the kind `{cell.strip()}`, and "
                 f"its kind is {want}")
        if not re.search(r"[A-Z]+-[0-9]{3}", requires) and "none" not in requires:
            fail(f"spec/core/README.md: the table names no promise for {name}")
    for path in groups["core"]:
        if f"[{path.name}]({path.name})" not in text:
            fail(f"spec/core/README.md: the table of files omits {path.name}")
    for name in re.findall(r"(?m)^\| \[([^\]]+\.yaml)\]\(", text):
        if not (core_dir / name).exists():
            fail(f"spec/core/README.md: the table of files lists {name}, which does not exist")

    readme = schema_dir / "README.md"
    if not readme.exists():
        fail("spec/schemas/README.md is missing")
        return
    section = re.search(r"(?ms)^## Files\n(.*?)(?=^## |\Z)", readme.read_text())
    rows = dict(re.findall(r"(?m)^\| \[([^\]]+\.yaml)\]\([^)]*\) \| ([^|]+) \|",
                           section.group(1) if section else ""))
    for path in groups["schemas"]:
        holds = {named(d) for d in loaded.get(path, []) if isinstance(d, dict)}
        if path.name not in rows:
            fail(f"spec/schemas/README.md: the table of files omits {path.name}")
        elif set(re.findall(r"`([^`]+)`", rows[path.name])) != holds:
            fail(f"spec/schemas/README.md: the row of {path.name} does not list exactly the "
                 f"Schema contracts the file holds")
    for name in sorted(set(rows) - {p.name for p in groups["schemas"]}):
        fail(f"spec/schemas/README.md: the table of files lists {name}, which does not exist")


# What the tool says of an implementation whose name a document of its project or package has.
CLASH = "NAM-027: `#name` refers to one document or one implementation"


# --- 6. Rules that no schema can express, applied as far as the documents show.

ACCESS = {"kind": "Resource", "name": "access"}

# Where a document of each role holds a schema, and where it declares an input (SCH-019).
SCHEMA_PLACES = (
    ("Schema", ("spec",)),
    ("Type", ("spec", "schema")),
    ("Operation", ("spec", "inputSchema")),
    ("Operation", ("spec", "outputSchema")),
    ("Requirement", ("spec", "inputs", "*", "schema")),
    ("template", ("spec", "inputs", "*", "schema")),
    ("template", ("spec", "controls", "*", "inputs", "*", "schema")),
)
# An Operation contract's input states its `default` on its property in `inputSchema`, which
# SCHEMA_PLACES reaches; a template's input and a Requirement's state it beside `schema`.
INPUT_PLACES = (
    ("Requirement", ("spec", "inputs", "*")),
    ("template", ("spec", "inputs", "*")),
)


def places(value, path, at=""):
    """Like `reach`, and also says where each value was found."""
    if not path:
        yield at, value
    elif path[0] == "*":
        if isinstance(value, dict):
            for name, item in value.items():
                yield from places(item, path[1:], f"{at}/{name}")
    elif isinstance(value, dict) and path[0] in value:
        yield from places(value[path[0]], path[1:], f"{at}/{path[0]}")


def defaults_in(schema, tokens=()):
    """Every schema object inside `schema` that states the keyword `default`, with the tokens
    that lead to it. Only places that hold a schema are entered, so a property that is named
    `default`, and a value under `const`, `enum`, `default` or `examples`, are not mistaken
    for the keyword."""
    if not isinstance(schema, dict):
        return
    if "default" in schema:
        yield tokens, schema
    for keyword, token, sub in subschemas(schema):
        step = (keyword,) if token is None else (keyword, token)
        yield from defaults_in(sub, tokens + step)


class Defaults:
    """Validates a `default` against the schema it must satisfy (SCH-019). It holds the Schema
    contracts of every project and package in this repository, so that a `$ref` can be
    followed: `#` and a pointer inside the same schema, `#name` inside the same project or
    package, and a qualified reference to a package it holds. Where a `$ref` leads anywhere
    else the default is not validated, and is counted as not checked. A default that is an
    object is validated after the defaults of its own properties are filled in, as SCH-010
    applies them. It needs jsonschema."""

    def __init__(self):
        self.defs, self.needs, self.by_address, self.pending = {}, {}, {}, []
        self.count = self.checked = self.skipped = 0

    def add(self, address, docs):
        """Take in the Schema contracts of one project or package, and return its scope, which
        says what `#name` reaches in it. Call `close` before validating anything."""
        scope = {}
        for doc in docs:
            if isinstance(doc, dict) and doc.get("kind") == "Schema" and named(doc):
                self.count += 1
                scope[named(doc)] = f"s{self.count}"
                self.pending.append((scope, scope[named(doc)], doc.get("spec")))
        if address:
            self.by_address[address] = scope
        return scope

    def close(self):
        for scope, key, spec in self.pending:
            self.defs[key], self.needs[key] = self.rewrite(spec, scope, key)
        self.pending = []

    def rewrite(self, schema, scope, root):
        """A copy of `schema` in which every `$ref` points into one `$defs`, and the keys of
        the Schema contracts it refers to, or None where a `$ref` cannot be followed."""
        schema = copy.deepcopy(schema)
        needs, followed = set(), [True]

        def visit(node, where, place):
            ref = node.get("$ref")
            if not isinstance(ref, str):
                return
            if is_pointer(ref):
                node["$ref"] = f"#/$defs/{root}{ref[1:]}"
                return
            local = LOCAL_REF.match(ref)
            address, _, name = ref.partition("#")
            key = (scope.get(local.group(1)) if local
                   else self.by_address.get(address, {}).get(name))
            if key is None:
                followed[0] = False
            else:
                node["$ref"] = "#/$defs/" + key
                needs.add(key)
        before = len(errors)
        walk(schema, root, visit)
        del errors[before:]            # the shape of a schema is reported by steps 2 and 3
        return schema, (needs if followed[0] else None)

    def follows(self, needs):
        seen, todo = set(), list(needs or ())
        while todo:
            key = todo.pop()
            if key not in seen:
                seen.add(key)
                if self.needs.get(key) is None:
                    return False
                todo.extend(self.needs[key])
        return needs is not None

    @staticmethod
    def stands_for(schema, top):
        """The schemas that `schema` stands for: itself and, through `$ref`, what it refers
        to inside `top`."""
        out, seen = [], set()
        while isinstance(schema, dict):
            out.append(schema)
            ref = schema.get("$ref")
            if not isinstance(ref, str) or ref in seen:
                break
            seen.add(ref)
            found, schema = pointer(top, ref)
            if not found:
                break
        return out

    def filled(self, value, schema, top, depth=0):
        """`value` with the defaults of its own properties filled in, as SCH-009 and SCH-010
        apply them: a property that is absent from an object that is present takes the
        `default` its schema states, each object that is an item of a list takes the defaults
        of its own properties, and no object is made in order to hold defaults. The object
        that holds a property may be written directly, reached through `$ref`, or be the
        schema that `items` gives; nothing is filled in through `allOf`, `anyOf` or `oneOf`."""
        if depth > 32:
            return value
        parts = self.stands_for(schema, top)
        if isinstance(value, dict):
            value = dict(value)
            for part in parts:
                members = part.get("properties")
                for name, sub in (members.items() if isinstance(members, dict) else ()):
                    if name not in value:
                        if not isinstance(sub, dict) or "default" not in sub:
                            continue
                        value[name] = copy.deepcopy(sub["default"])
                    value[name] = self.filled(value[name], sub, top, depth + 1)
        elif isinstance(value, list):
            for part in parts:
                if isinstance(part.get("items"), dict):
                    return [self.filled(item, part["items"], top, depth + 1) for item in value]
        return value

    def fails(self, scope, root, tokens, value):
        """Why `value` does not satisfy the schema that `tokens` lead to inside `root`, once
        the defaults of its own properties are filled in. None where it does, and where a
        `$ref` cannot be followed, which is counted."""
        rewritten, needs = self.rewrite(root, scope, "root")
        if not self.follows(needs):
            self.skipped += 1
            return None
        self.checked += 1
        part = rewritten
        for token in tokens:
            part = part[token]
        top = {k: v for k, v in part.items() if k != "$defs"} if isinstance(part, dict) \
            else {"allOf": [part]}
        top["$schema"] = "https://json-schema.org/draft/2020-12/schema"
        top["$defs"] = dict(self.defs, root=rewritten)
        value = self.filled(value, top, top)
        found = list(jsonschema.Draft202012Validator(
            top, format_checker=jsonschema.Draft202012Validator.FORMAT_CHECKER).iter_errors(value))
        return explain(found[0]) if found else None

    def faults(self, doc, scope):
        """What SCH-019 finds in one document, as messages."""
        out, role, name = [], role_of(doc, False), named(doc)
        for kind, path in SCHEMA_PLACES:
            if kind != role:
                continue
            for at, root in places(doc, path):
                for tokens, schema in defaults_in(root):
                    why = self.fails(scope, root, tokens, schema["default"])
                    if why:
                        where = at + "".join(f"/{t}" for t in tokens)
                        out.append(f"{name}: the `default` at {where} does not satisfy the "
                                   f"schema that states it: {why}")
        for kind, path in INPUT_PLACES:
            if kind != role:
                continue
            for at, member in places(doc, path):
                if isinstance(member, dict) and "default" in member and "schema" in member:
                    why = self.fails(scope, member["schema"], (), member["default"])
                    if why:
                        out.append(f"{name}: the `default` of the input at {at} does not "
                                   f"satisfy the input's schema: {why}")
        return out


def template_of(ref, kind, templates, package_docs):
    """The template that a `from` names, where this tool has it: a template of the same set,
    by its name where the kind is fixed, by `{kind, name}` or by `#name`; or a template of a
    package in this repository, by a qualified reference. None where the tool does not have
    it, and then no rule is applied through it."""
    if isinstance(ref, dict):
        found = templates.get((ref.get("kind"), ref.get("name")))
    elif not isinstance(ref, str):
        found = None
    elif ref.startswith("#"):
        found = next((d for (_, name), d in templates.items() if name == ref[1:]), None)
    elif "#" in ref:
        address, _, name = ref.partition("#")
        found = package_docs.get(address, {}).get(name)
    else:
        found = templates.get((kind, ref))
    if isinstance(found, dict) and found.get("kind") in FAMILIES and "from" not in found:
        return found
    return None


def required_place(schema, pointer_text):
    """Whether the schema requires the place that a JSON Pointer names, with every member on
    the way to it required (MAN-017): True, False, or None where a `$ref` on the way leaves
    this tool unable to tell. A type that states no schema requires nothing."""
    if not isinstance(pointer_text, str) or pointer_text == "":
        return True
    node = schema
    tokens = [t.replace("~1", "/").replace("~0", "~") for t in pointer_text.split("/")[1:]]
    for index, token in enumerate(tokens):
        if not isinstance(node, dict):
            return False
        if "$ref" in node and "required" not in node and "properties" not in node:
            return None
        if token not in (node.get("required") or []):
            return False
        node = (node.get("properties") or {}).get(token)
        if node is None and index < len(tokens) - 1:
            return False
    return True


def secret_faults(schema, secret):
    """Each place where a schema puts `secret-reference.v1` where SEC-001 does not let it
    stand: under an applicator other than those of SECRET_PLACES, or under `anyOf` or `oneOf`
    beside a branch that is not `{type: null}`. A `$ref` to a definition that holds it is not
    followed."""
    out = []

    def visit(node, at, allowed):
        if not isinstance(node, dict):
            return
        if node.get("$ref") in secret and not allowed:
            out.append(at or "/")
        for keyword, token, sub in subschemas(node):
            here = f"{at}/{keyword}" + ("" if token is None else f"/{token}")
            if keyword in BRANCHES[1:]:
                others = [b for i, b in enumerate(node[keyword]) if i != token]
                ok = allowed and all(b == {"type": "null"} for b in others)
            else:
                ok = allowed and keyword in SECRET_PLACES
            visit(sub, here, ok)
    visit(schema, "", True)
    return out


def apply_rules(docs, is_core, core, package_docs, defaults=None, scope=None, items=None,
                first_start=False):
    """Return (rule, message) for each rule of RULES that one set of documents breaks. The set
    is the documents of one project or package, or of one file of examples/invalid-by-rule/.
    Manifests are left out, apart from their implementation items: `items` maps an address,
    or None for the set's own project or package, to the items of its manifest. Each rule
    says how far it goes. `defaults` is the Defaults that the rule of NEED_VALIDATOR uses,
    with the `scope` of this set, or None without jsonschema, and then that rule is not
    applied. `first_start` is true for the documents that first start makes, which are not a
    submission, and the rule for FST-006 is then not applied."""
    found = []
    items = items or {}
    docs = [d for d in docs if isinstance(d, dict)]
    templates = {(d.get("kind"), named(d)): d for d in docs
                 if d.get("kind") in FAMILIES and "from" not in d}
    instances = [d for d in docs if d.get("kind") in FAMILIES and "from" in d]
    bound = {core + "#bound.v1"} | ({"#bound.v1"} if is_core else set())

    def what(doc):
        kind = doc.get("kind")
        if kind in FAMILIES:
            return f"{kind} instance" if "from" in doc else f"{kind} template"
        return f"{kind} contract"

    # one-name (NAM-027): no two documents of the set share a name, whatever their kinds.
    seen = {}
    for doc in docs:
        name = named(doc)
        if name is None:
            continue
        if name in seen:
            found.append(("one-name", f"two documents are named {name}: a {seen[name]} and a "
                                      f"{what(doc)}"))
        seen.setdefault(name, what(doc))

    def bounds(template):
        """The items of `requirements` by which a Conduit template applies `bound.v1`."""
        spec = template.get("spec") or {}
        return [name for name, item in (spec.get("requirements") or {}).items()
                if isinstance(item, dict) and item.get("requirement") in bound]

    # bound-once (CON-028): one Conduit template applies `bound.v1` at most once.
    for (kind, name), template in templates.items():
        if kind == "Conduit":
            applied = bounds(template)
            if len(applied) > 1:
                found.append(("bound-once", f"the Conduit template {name} applies `bound.v1` "
                                            f"{len(applied)} times, as "
                                            f"{' and '.join(applied)}"))

    # bound-target (CON-028): a Conduit whose template applies `bound.v1` has `access` as its
    # target. Applied where a document writes the target in place, as a Conduit instance
    # document and a declared Conduit may, and the tool has the template. The target is
    # `access` when it is written `{kind: Resource, name: access}`.
    def aims(label, ref, target):
        template = template_of(ref, "Conduit", templates, package_docs)
        if template is not None and bounds(template) and target != ACCESS:
            found.append(("bound-target", f"{label} is made from the template "
                                          f"{named(template)}, which applies `bound.v1`, and "
                                          f"its target is not the Resource `access`"))
    for doc in instances:
        if doc.get("kind") == "Conduit":
            aims(f"the Conduit {named(doc) or 'with no name'}", doc.get("from"),
                 doc.get("target"))
    for (kind, name), template in templates.items():
        for local, declared in ((template.get("spec") or {}).get("conduits") or {}).items():
            if isinstance(declared, dict) and "target" in declared:
                aims(f"the declared Conduit {local} of the template {name}",
                     declared.get("from"), declared["target"])

    # reserved-mark (CTR-009): outside the core package no Operation contract is marked.
    if not is_core:
        for doc in docs:
            if doc.get("kind") == "Operation" \
                    and (doc.get("spec") or {}).get("reservedToRoot") is True:
                found.append(("reserved-mark", f"the Operation contract {named(doc)} is marked "
                                               f"`reservedToRoot: true` and is not a contract "
                                               f"of the core package"))

    # first-start-from (FST-006): no instance document is made from a core template from
    # which first start alone makes instances, `location` among them. Applied where `from` is
    # a qualified reference into the core package: the core package's templates are never
    # registered (NAM-013), so no other form of `from` names one.
    if not first_start:
        for doc in instances:
            ref = doc.get("from")
            if isinstance(ref, str) and ref.partition("#")[0] == core \
                    and ref.partition("#")[2] in FIRST_START_TEMPLATES:
                found.append(("first-start-from",
                              f"the {doc.get('kind')} {named(doc) or 'with no name'} is made "
                              f"from {ref}, from which first start alone makes instances"))

    # location-type (PLC-028): no template but the core package's `location` has the type
    # `location.v1`, so that no actor registers a location through a template of its own.
    # Applied where `type` is the core package's qualified reference, or `#location.v1` inside
    # the core package.
    location = {core + "#location.v1"} | ({"#location.v1"} if is_core else set())
    for (kind, name), template in templates.items():
        if (template.get("spec") or {}).get("type") in location \
                and not (is_core and kind == "Resource" and name == "location"):
            found.append(("location-type",
                          f"the {kind} template {name} has the type location.v1, which only "
                          f"the core package's template `location` has"))

    # entry-reference (POOL-005): `{entry: name}` is written only in an entry of a Pool
    # template. Applied to the places of a document that are never inside an entry and always
    # hold a reference: the `placement`, `runtime`, `consumer` and `target` of an instance
    # document and the `target` of a declared Conduit. A value inside `arguments` or `config`
    # is not looked at, because only the schema that governs it says whether it is a
    # reference.
    def entry(value):
        return isinstance(value, dict) and "entry" in value and "kind" not in value
    for doc in instances:
        for key in ("placement", "runtime", "consumer", "target"):
            if entry(doc.get(key)):
                found.append(("entry-reference",
                              f"the {doc.get('kind')} {named(doc) or 'with no name'} writes "
                              f"its `{key}` as `{{entry: {doc[key]['entry']}}}`, outside an "
                              f"entry of a Pool template"))
    for (kind, name), template in templates.items():
        for local, declared in ((template.get("spec") or {}).get("conduits") or {}).items():
            if isinstance(declared, dict) and entry(declared.get("target")):
                found.append(("entry-reference",
                              f"the declared Conduit {local} of the template {name} writes "
                              f"its `target` as `{{entry: {declared['target']['entry']}}}`, "
                              f"outside an entry of a Pool template"))

    # operations-filled (CON-016): a Conduit template that writes no `operations` has an
    # input whose destination is `/operations`.
    for (kind, name), template in templates.items():
        spec = template.get("spec") or {}
        if kind == "Conduit" and "capability" in spec and "operations" not in spec:
            inputs = spec.get("inputs") if isinstance(spec.get("inputs"), dict) else {}
            if not any(isinstance(i, dict) and "/operations" in (i.get("to") or [])
                       for i in inputs.values()):
                found.append(("operations-filled",
                              f"the Conduit template {name} writes no `operations`, and no "
                              f"input's destination is `/operations`"))

    # The rules below follow a selection from one document into another, so they name a
    # contract by the project or package that holds it and its name. `#name` is in the project
    # or package of the document that writes it (NAM-022): this set, or the package the
    # document was found in. A qualified reference names its package.
    here = core if is_core else None
    own = {named(d): d for d in docs if "from" not in d}

    def canon(ref, home):
        if not isinstance(ref, str) or "#" not in ref:
            return None
        address, _, name = ref.partition("#")
        return (address or home, name)

    def lookup(key, kind):
        """The contract of that kind which `key` names, where this tool has it."""
        if key is None:
            return None
        home, name = key
        held = own.get(name) if home == here else package_docs.get(home, {}).get(name)
        return held if isinstance(held, dict) and held.get("kind") == kind else None

    def item_of(key):
        """The implementation item that `key` names, where this tool has the manifest."""
        if key is None:
            return None
        home, name = key
        held = (items.get(None) if home == here else items.get(home)) or {}
        found_item = held.get(name)
        return found_item if isinstance(found_item, dict) else None

    def members(value):
        return value.items() if isinstance(value, dict) else ()

    def provider(key):
        return key is not None and key[0] == core and key[1] in PROVIDER_CAPABILITIES

    # The operations each template of the set must have an implementation for, under TPL-004,
    # with the implementation it has for each: the key of the template's `implementation`
    # map, else the default of its type, else the default of the operation's own contract.
    # Applied where the tool has the type's contract; a capability or an operation whose
    # contract it does not have is not followed.
    selected = {}
    for (kind, name), template in templates.items():
        spec = template.get("spec") if isinstance(template.get("spec"), dict) else {}
        type_key = canon(spec.get("type"), here)
        type_doc = lookup(type_key, "Type") if kind != "Conduit" else None
        if type_doc is None:
            continue
        body = type_doc.get("spec") if isinstance(type_doc.get("spec"), dict) else {}
        needed = {}

        def need(ref, home, why):
            key = canon(ref, home)
            if key is not None:
                needed.setdefault(key, why)

        for ref in body.get("requires") if isinstance(body.get("requires"), list) else ():
            need(ref, type_key[0], "its type requires the operation")
        for ref in body.get("controls") if isinstance(body.get("controls"), list) else ():
            need(ref, type_key[0], "its type lists the operation as a control")
        for offered in body.get("offers") if isinstance(body.get("offers"), list) else ():
            offered_key = canon(offered, type_key[0])
            capability = lookup(offered_key, "Capability")
            listed = (capability.get("spec") or {}).get("operations") if capability else None
            for ref in listed if isinstance(listed, list) else ():
                need(ref, offered_key[0], f"the capability {offered_key[1]}, which its type "
                                          f"offers, includes the operation")
        for key, value in members(spec.get("implementation")):
            need(key, here, "the template selects an implementation for it")
        chosen = {canon(k, here): canon(v, here)
                  for k, v in members(spec.get("implementation"))}
        by_type = {canon(k, type_key[0]): canon(v, type_key[0])
                   for k, v in members(body.get("implementation"))}
        picks = {}
        for key, why in needed.items():
            operation = lookup(key, "Operation")
            if key in chosen:
                picks[key] = chosen[key]
            elif key in by_type:
                picks[key] = by_type[key]
            elif operation is not None and "implementation" in (operation.get("spec") or {}):
                picks[key] = canon(operation["spec"]["implementation"], key[0])
            elif operation is not None:
                found.append(("implementation-selected",
                              f"the template {name} selects no implementation for {key[1]}, "
                              f"and neither its type {named(type_doc)} nor the operation's "
                              f"contract states a default; {why}"))
        selected[name] = (type_doc, picks)

    for name, (type_doc, picks) in selected.items():
        body = type_doc.get("spec") if isinstance(type_doc.get("spec"), dict) else {}
        chosen_items = {}
        for key in picks.values():
            item = item_of(key)
            if item is not None:
                chosen_items[key] = item

        # one-run (TPL-019): the selected implementations include at most one item reached by
        # `command` or `package`. Applied where the tool has the manifests of the items.
        run = sorted(f"{k[0] + '#' if k[0] else '#'}{k[1]}" for k, item in chosen_items.items()
                     if "command" in item or "package" in item)
        if len(run) > 1:
            found.append(("one-run", f"the template {name} selects {' and '.join(run)}, two "
                                     f"items that are run, and one run serves one instance"))

        # label-conflict (PLC-025): no selected item's `placement` names a label that its
        # type's `placement` names with another value.
        labels = body.get("placement") if isinstance(body.get("placement"), dict) else {}
        for key, item in chosen_items.items():
            for label, value in members(item.get("placement")):
                if label in labels and labels[label] != value:
                    found.append(("label-conflict",
                                  f"the template {name} selects {key[1]}, whose `placement` "
                                  f"gives the label {label} the value {value}, and its type "
                                  f"{named(type_doc)} gives it {labels[label]}"))

        # item-place-required (MAN-017): each pointer of a selected item's `environment`,
        # `arguments` or `headers` names a place that the type's schema requires, with every
        # member on the way required. A `$ref` on the way is not followed.
        for key, item in chosen_items.items():
            for binding in ("environment", "arguments", "headers"):
                for var, place in members(item.get(binding)):
                    if required_place(body.get("schema"), place) is False:
                        found.append(("item-place-required",
                                      f"the template {name} selects {key[1]}, whose "
                                      f"`{binding}` gives {var} the place {place}, which the "
                                      f"schema of its type {named(type_doc)} does not "
                                      f"require"))

    # secret-branch (SEC-001): `secret-reference.v1` stands only as the schema of a place, as
    # `items`, or under `anyOf` or `oneOf` beside `{type: null}`. Applied to every schema that
    # a document of the set writes.
    secret = {core + "#secret-reference.v1"} | ({"#secret-reference.v1"} if is_core else set())
    for doc in docs:
        role = role_of(doc, False)
        for kind, path in SCHEMA_PLACES:
            if kind != role:
                continue
            for at, schema in places(doc, path):
                for where in secret_faults(schema, secret):
                    found.append(("secret-branch",
                                  f"{named(doc)}: the schema at {at} puts "
                                  f"`secret-reference.v1` at {where}, under an applicator or "
                                  f"beside a branch where it may not stand"))

    # provider-family (PRV-001): a Type that offers a provider capability is of the family
    # Resource and offers no other provider capability.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Type" else None
        if not isinstance(spec, dict):
            continue
        offers = spec.get("offers") if isinstance(spec.get("offers"), list) else []
        offered = sorted({canon(o, here)[1] for o in offers if provider(canon(o, here))})
        if offered and spec.get("family") != "Resource":
            found.append(("provider-family",
                          f"the Type contract {named(doc)} offers {offered[0]} and is of the "
                          f"family {spec.get('family')}, not Resource"))
        if len(offered) > 1:
            found.append(("provider-family",
                          f"the Type contract {named(doc)} offers {' and '.join(offered)}, "
                          f"more than one provider capability"))

    # provider-conduit (PRV-004): no Conduit template's capability is one whose operations the
    # engine alone invokes, and none lists one of those operations.
    for (kind, name), template in templates.items():
        spec = template.get("spec") if isinstance(template.get("spec"), dict) else {}
        if kind != "Conduit":
            continue
        capability = canon(spec.get("capability"), here)
        if capability is not None and capability[0] == core \
                and capability[1] in CALLED_CAPABILITIES:
            found.append(("provider-conduit",
                          f"the Conduit template {name} has the capability {capability[1]}, "
                          f"whose operations the engine alone invokes"))
        for ref in spec.get("operations") if isinstance(spec.get("operations"), list) else ():
            key = canon(ref, here)
            if key is not None and key[0] == core and key[1] in CALLED_OPERATIONS:
                found.append(("provider-conduit",
                              f"the Conduit template {name} lists {key[1]}, which the engine "
                              f"alone invokes"))

    # default-valid (SCH-019): a `default` satisfies the schema that states it, and an input's
    # `default` the input's own `schema`. Applied where every `$ref` of the schema can be
    # followed inside this repository; the others are counted and not validated.
    if defaults is not None:
        for doc in docs:
            for message in defaults.faults(doc, scope or {}):
                found.append(("default-valid", message))
    return found


def manifest_rules(manifest, core, holds_contracts=False):
    """Return (rule, message) for each rule on manifests that one manifest breaks. There are
    two. core-address (NAM-021): a manifest neither lists the core package under `packages`
    nor states its address as its own. project-address (MAN-015): a manifest that lists an
    implementation, or that belongs to a project that holds a contract, states `address`.
    The core package's own manifest, which is built into the engine, is not given to this
    function."""
    found = []
    if not isinstance(manifest, dict):
        return found
    if manifest.get("address") == core:
        found.append(("core-address", f"the manifest states the core package's address, "
                                      f"{core}, as its own"))
    if isinstance(manifest.get("packages"), dict) and core in manifest["packages"]:
        found.append(("core-address", f"the manifest lists the core package, {core}, under "
                                      f"`packages`"))
    if "address" not in manifest and (manifest.get("implementations") or holds_contracts):
        why = "lists an implementation" if manifest.get("implementations") \
            else "belongs to a project that holds a contract"
        found.append(("project-address", f"the manifest {why} and states no `address`"))
    return found


def main():
    from importlib import metadata
    validating = jsonschema is not None
    print(f"Python {sys.version.split()[0]} at {sys.executable}; PyYAML {yaml.__version__}; "
          f"jsonschema {metadata.version('jsonschema') if validating else 'NOT IMPORTABLE'}")
    named_values = names()
    core = named_values["corePackage"]
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
    by_rule_dir = schema_dir / "examples" / "invalid-by-rule"

    promises = set()
    for path in (ROOT / "spec").glob("*/*.yaml"):
        promises.update(re.findall(r"(?m)^- id: ([A-Z]+-[0-9]+)$", path.read_text()))
    for rule, promise in RULES.items():
        if promise not in promises:
            fail(f"tools/check_schemas.py: the rule {rule} is named for {promise}, which is "
                 f"not a promise that exists")

    # 1. Strict YAML.
    loaded = {}
    groups = {
        "schemas": sorted(schema_dir.glob("*.yaml")),
        "core": sorted(core_dir.glob("*.yaml")),
        "valid": sorted(valid_dir.rglob("*.yaml")),
        "invalid": sorted(invalid_dir.rglob("*.yaml")),
        "by-rule": sorted(by_rule_dir.rglob("*.yaml")),
    }
    for group, paths in groups.items():
        for path in paths:
            try:
                loaded[path] = load(path)
            except NotStrict as problem:
                if group == "invalid":
                    loaded[path] = problem
                else:
                    fail(f"{rel(path)}: not strict YAML: {problem}")
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
                if isinstance(doc, dict) and doc.get("apiVersion") != named_values["apiVersion"]:
                    fail(f"{rel(path)} document {index + 1}: apiVersion is not the one "
                         f"spec/names.yaml states")
    if validating:
        print(f"3. Validated {checked} documents under spec/schemas/, spec/core/ and "
              f"examples/valid/.")

    # 4. Invalid documents.
    rejected = 0
    for path in groups["invalid"]:
        match = re.match(r"^([A-Z]+-[0-9]+)-", path.name)
        if not match or match.group(1) not in promises:
            fail(f"{rel(path)}: not named for a promise that exists")
        if not validating:
            continue
        docs = loaded[path]
        if isinstance(docs, NotStrict):
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
              f"rejected by a schema, or rejected when read.")

    # 5. The core package and the references that can be checked here.
    core_files = [(p, loaded.get(p, [])) for g in ("schemas", "core") for p in groups[g]]
    core_kinds, twice = kinds_in(core_files)
    for name in twice:
        fail(f"core package: an implementation and something else are both named {name} "
             f"({CLASH})")
    manifest = next((d[0] for p, d in core_files if p.name == "conduit.yaml" and d), {})
    exports = list(manifest.get("exports") or [])
    documents = {n for n, k in core_kinds.items() if k != "implementation"}
    for name in sorted(n for n in exports if exports.count(n) > 1):
        fail(f"spec/core/conduit.yaml: exports {name} twice (PKG-012)")
    for name in sorted(set(exports) - documents):
        fail(f"spec/core/conduit.yaml: exports {name}, which is not a document of the package "
             f"(MAN-007)")
    for name in sorted(documents - set(exports) - PRIVATE):
        fail(f"spec/core/conduit.yaml: does not export the document {name}, and PRIVATE in "
             f"this tool does not list it as deliberately private")
    for name in sorted(PRIVATE):
        if name not in documents or name in exports:
            fail(f"tools/check_schemas.py: PRIVATE lists {name}, which is not a document of "
                 f"the core package that it does not export")
    for name, kind in core_kinds.items():
        if not EXPORT.match(name or "") or len(name) > 63:
            fail(f"core package: {name} is not an export name (NAM-004)")
        elif kind in CONTRACTS and not VERSIONED.match(name):
            fail(f"core package: the contract {name} does not have a segment and then a "
                 f"version segment (CTR-001)")
        elif kind in FAMILIES and not NAME.match(name):
            fail(f"core package: the template {name} breaks OBJ-001")
    check_readmes(core_dir, schema_dir, core_kinds, groups, loaded)

    # Each directory under examples/valid/ is one project. One whose manifest states both
    # `address` and `version` is a package (PKG-017), and what it exports can be referred to
    # from the others. The documents of any directory whose manifest states `address` are
    # reached by that address, which is how its own `#name` selections are followed.
    packages = {core: core_kinds}
    package_docs = {core: contents(core_files)}
    items = {core: manifest.get("implementations") or {}}
    by_dir = {}
    for path in groups["valid"]:
        by_dir.setdefault(path.parent, []).append((path, loaded.get(path, [])))
    local, private, own_items = {}, 0, {}
    for directory, files in by_dir.items():
        local[directory], twice = kinds_in(files)
        own_items[directory] = {}
        for name in twice:
            fail(f"{rel(directory)}: an implementation and something else are both named "
                 f"{name} ({CLASH})")
        held = [d for p, ds in files if p.name != "conduit.yaml" for d in ds]
        has_contract = any(isinstance(d, dict) and d.get("kind") in CONTRACTS for d in held)
        manifests = [ds[0] for p, ds in files
                     if p.name == "conduit.yaml" and ds and isinstance(ds[0], dict)]
        if has_contract and not manifests:
            fail(f"{rel(directory)}: holds a contract and has no manifest, so it states no "
                 f"`address` (MAN-015)")
        for doc in manifests:
            address = doc.get("address")
            listed = set(doc.get("exports") or [])
            own_items[directory] = doc.get("implementations") or {}
            if address:
                package_docs[address] = contents(files)
                items[address] = own_items[directory]
            if address and doc.get("version"):
                packages[address] = {n: k for n, k in local[directory].items()
                                     if n in listed or k == "implementation"}
                private += sum(1 for n, k in local[directory].items()
                              if k != "implementation" and n not in listed)
                for name in sorted(n for n in listed if n not in local[directory]):
                    fail(f"{rel(directory)}/conduit.yaml: exports {name}, which is not a "
                         f"document of the package (MAN-007)")

    # The operations of REV-012, and the controls.
    operations = package_docs[core]
    for name in NO_REVISION:
        inputs, _ = members_of((operations.get(name) or {}).get("spec") or {}, "inputSchema")
        if operations.get(name, {}).get("kind") != "Operation":
            fail(f"core package: {name} is not an Operation contract")
        elif "revision" in inputs:
            fail(f"core package: {name} takes `revision`, and REV-012 says it names none")
    for name in RETURNS_REVISION:
        outputs, listed = members_of((operations.get(name) or {}).get("spec") or {},
                                     "outputSchema")
        if "revision" not in outputs or "revision" not in listed:
            fail(f"core package: {name} does not declare the output `revision` under "
                 f"`required`, which SUB-009 has it return")
    controls = check_controls("core package", [d for _, ds in core_files for d in ds],
                              package_docs, core)
    for directory, files in by_dir.items():
        address = next((d[0].get("address") for p, d in files
                        if p.name == "conduit.yaml" and d and isinstance(d[0], dict)), None)
        if address:
            controls += check_controls(rel(directory), [d for _, ds in files for d in ds],
                                       package_docs, address)

    counts = {"checked": 0, "skipped": 0}
    for path, docs in core_files:
        for doc in docs:
            if isinstance(doc, dict):
                role = role_of(doc, path.name == "conduit.yaml")
                for _, message in check_references(role, doc, packages, core_kinds, counts):
                    fail(f"{rel(path)}: {message}")
    for directory, files in by_dir.items():
        for path, docs in files:
            for doc in docs:
                if isinstance(doc, dict):
                    role = role_of(doc, path.name == "conduit.yaml")
                    for _, message in check_references(role, doc, packages, local[directory],
                                                       counts):
                        fail(f"{rel(path)}: {message}")
    used = set()
    for path in list(loaded):
        used.update(re.findall(re.escape(core) + r"#([a-z0-9.-]*[a-z0-9])", path.read_text()))
    for name in sorted(used - set(core_kinds)):
        fail(f"a file refers to {core}#{name}, which the core package does not hold")

    # Every promise cited by the files this tool covers exists. The prefix of a promise file
    # that a ruling removed is looked for too, so that no citation of one of its IDs remains.
    prefixes = "|".join(sorted({p.split("-")[0] for p in promises} | set(REMOVED_PREFIXES)))
    cited_in = [p for p in list(core_dir.rglob("*")) + list(schema_dir.rglob("*"))
                if p.is_file() and p.suffix in (".yaml", ".md")]
    cited_in.append(ROOT / "spec" / "promises" / "manifest.yaml")
    cited = 0
    for path in cited_in:
        text = path.read_text()
        if path.parent.name == "promises":
            # A promise file's head comment names the IDs that rulings removed from it, as
            # each must be named there; only its promises are read for citations.
            text = text[text.find("\n- id:"):]
        ids = set(re.findall(rf"\b((?:{prefixes})-[0-9]{{3}})\b", path.name + "\n" + text))
        cited += len(ids)
        for promise in sorted(ids - promises):
            fail(f"{rel(path)}: cites {promise}, which is not a promise that exists")
    print(f"5. Core package: {len(exports)} documents exported, {len(PRIVATE)} documents "
          f"deliberately private and {len(core_kinds) - len(documents)} implementation, which "
          f"is also an export. Checked {counts['checked']} references for the kind they name; "
          f"{counts['skipped']} name packages that are not in this repository and were not "
          f"checked. Checked {controls} controls, the {len(NO_REVISION)} operations that "
          f"take no `revision`, the {len(RETURNS_REVISION)} that returns one, the tables of "
          f"the two READMEs, and {cited} citations of promises in {len(cited_in)} files. The "
          f"example packages keep {private} documents private.")

    # 6. Rules that no schema can express.
    checker = Defaults() if validating else None
    core_docs = [d for p, ds in core_files if p.name != "conduit.yaml" for d in ds]
    sets = [("the core package", core_docs, True,
             checker.add(core, core_docs) if checker else None)]
    for directory, files in by_dir.items():
        docs = [d for p, ds in files if p.name != "conduit.yaml" for d in ds]
        address = next((d[0].get("address") for p, d in files
                        if p.name == "conduit.yaml" and d and isinstance(d[0], dict)), None)
        sets.append((rel(directory), docs, False,
                     checker.add(address, docs) if checker else None))
    if checker:
        checker.close()
    for (label, docs, is_core, scope), directory in zip(sets, [None] + list(by_dir)):
        own = dict(items)
        own[None] = own_items[directory] if directory else items[core]
        first_start = directory is not None and directory.name == FIRST_START_DIRECTORY
        for rule, message in apply_rules(docs, is_core, core, package_docs, checker, scope,
                                         own, first_start):
            fail(f"{label}: {message} ({RULES[rule]})")
    for directory, files in by_dir.items():
        holds_contract = any(isinstance(d, dict) and d.get("kind") in CONTRACTS
                             for p, ds in files if p.name != "conduit.yaml" for d in ds)
        for path, docs in files:
            if path.name == "conduit.yaml":
                for doc in docs:
                    for rule, message in manifest_rules(doc, core, holds_contract):
                        fail(f"{rel(path)}: {message} ({RULES[rule]})")
    in_place = (checker.checked, checker.skipped) if checker else (0, 0)
    turned_away, valid_first, not_run = 0, 0, 0
    for path in groups["by-rule"]:
        match = re.match(r"^([A-Z]+-[0-9]+)-", path.name)
        promise = match.group(1) if match else None
        if promise not in promises or promise not in RULES.values():
            fail(f"{rel(path)}: not named for a promise that a rule of this tool comes from")
            continue
        docs = loaded.get(path)
        if not isinstance(docs, list):
            continue                       # not strict YAML: already reported in step 1
        if checker is None and all(rule in NEED_VALIDATOR
                                   for rule, named_for in RULES.items() if named_for == promise):
            not_run += 1
            print(f"   NOT RUN {path.name}: the rule for {promise} needs jsonschema")
            continue
        is_manifest = "-manifest-" in path.name
        if validating:
            for index, doc in enumerate(docs):
                why = validate(bundled, doc, is_manifest)
                if why:
                    fail(f"{rel(path)} document {index + 1}: not valid against the schemas, "
                         f"so no rule is what rejects it: {why}")
                else:
                    valid_first += 1
        if is_manifest:
            found = [fault for doc in docs for fault in manifest_rules(doc, core)]
        else:
            scope = checker.add(None, docs) if checker else None
            if checker:
                checker.close()
            found = apply_rules(docs, False, core, package_docs, checker, scope, items)
            kinds, _ = kinds_in([(path, docs)])
            for doc in docs:
                if isinstance(doc, dict):
                    found += check_references(role_of(doc, False), doc, packages, kinds,
                                              {"checked": 0, "skipped": 0})
        mine = [message for rule, message in found if rule and RULES[rule] == promise]
        if not mine:
            fail(f"{rel(path)}: no rule of this tool for {promise} rejects it, and it must be "
                 f"rejected")
        else:
            turned_away += 1
            print(f"   rejected {path.name}: by this tool's rule for {promise}: {mine[0]}")
    shown = (f"each of their {valid_first} documents is valid against the schemas"
             if validating else "THEIR DOCUMENTS WERE NOT VALIDATED AGAINST A SCHEMA")
    applied = len(RULES) if checker else len(RULES) - len(NEED_VALIDATOR)
    print(f"6. Applied {applied} rules of this tool, which come from "
          f"{len({p for r, p in RULES.items() if checker or r not in NEED_VALIDATOR})} "
          f"promises, to the core package and "
          f"{len(by_dir)} example directories. {turned_away} sets of documents in "
          f"{len(groups['by-rule'])} files under examples/invalid-by-rule/ were each rejected "
          f"by the rule they are named for; {shown}.")
    if checker:
        print(f"   SCH-019: validated {in_place[0]} defaults of the core package and the valid "
              f"examples against their schemas; {in_place[1]} were not validated, because a "
              f"`$ref` in the schema leads outside this repository.")
    else:
        print(f"   SCH-019: NOT APPLIED. No `default` was validated against its schema, and "
              f"{not_run} files under examples/invalid-by-rule/ were not tried, because "
              f"jsonschema is not importable.")
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
