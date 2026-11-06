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
     it, and uses `$ref` only in the three forms of SCH-015, each of which resolves: `#` and a
     JSON Pointer into the same schema, a qualified reference to a Schema contract, and
     `#name` for a Schema contract of the same package. `schema.v1`, the profile written as a
     schema, lists exactly the profile's keywords and formats. A schema written inside
     another document is checked through `schema.v1` in step 3, and a `$ref` in it that is
     not a pointer is resolved in step 5.
  3. Every document under spec/core/, spec/schemas/ and spec/schemas/examples/valid/ is valid
     against `document.v1`, and every manifest (a file named conduit.yaml) against
     `manifest.v1`, with a JSON Schema 2020-12 validator and format checking on.
  4. Every document under spec/schemas/examples/invalid/ is rejected by a schema, or is
     rejected when its file is read, and its file is named for a promise that exists.
  5. The core package agrees with itself: every export in its manifest is a document of the
     package, every document is exported or is listed in PRIVATE below, no name is used
     twice, names follow NAM-004, CTR-001 and OBJ-001, every reference into the core package
     names an export of the kind the place expects, every control has a `changes` that maps
     inputs it declares to places its type's schema describes and declares the input
     `revision` and the output `settings` (CTR-006), every other input of a control is
     optional (REV-006, as the record settled it), the operations of REV-012 take no
     `revision`, no implementation has the name of a document of its project or package
     (NAM-027), and the
     tables of spec/core/README.md and spec/schemas/README.md list exactly the files and the
     exports there are, each export with its kind and with promises that exist. The same
     reference check is made on every `#name`, a `$ref` and a `from` among them, inside each
     example directory, and every promise ID cited under spec/core/, spec/schemas/ and in
     spec/promises/manifest.yaml is the ID of a promise that exists.
  6. Rules that no schema can express and that this tool applies itself, each named in RULES
     below with the promise it comes from. The core package and every directory under
     examples/valid/ break none of them. Every file under examples/invalid-by-rule/ is one
     set of documents, each of them valid against the schemas, that the rule its file is
     named for rejects. The rule for SCH-019 validates a `default` against its schema, so it
     needs jsonschema; without it the rule is not applied, and the tool says so.

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

# The rules of step 6, each with the promise it comes from. A file under
# examples/invalid-by-rule/ is named for one of these promises.
RULES = {
    "one-name": "NAM-027",
    "bound-once": "CON-028",
    "bound-target": "CON-028",
    "reserved-mark": "CTR-009",
    "location-placement": "PLC-008",
    "entry-reference": "POOL-005",
    "operations-filled": "CON-016",
    "ref": "SCH-015",
    "constituent-input": "CTR-010",
    "default-valid": "SCH-019",
    "pool-offers": "CTR-004",
    "runtime-once": "TPL-006",
    "applies-to": "CTR-005",
    "implementation-selected": "TPL-004",
    "narrows-on-conduit": "CON-030",
    "obliges-narrows": "CTR-005",
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

def walk(schema, where, visit, place="other"):
    """Visit every schema object. `place` is other, property or combined (SCH-008)."""
    if isinstance(schema, bool):
        return
    if not isinstance(schema, dict):
        fail(f"{where}: a schema is a map or a boolean")
        return
    visit(schema, where, place)
    inside = place == "combined"
    for keyword in ("properties", "$defs"):
        members = schema.get(keyword, {})
        if not isinstance(members, dict):
            fail(f"{where}/{keyword}: not a map")
            continue
        for name, sub in members.items():
            below = "combined" if inside else ("property" if keyword == "properties" else "other")
            walk(sub, f"{where}/{keyword}/{name}", visit, below)
    for keyword in ("additionalProperties", "items"):
        if keyword in schema:
            walk(schema[keyword], f"{where}/{keyword}", visit, "combined" if inside else "other")
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


def check_references(role, doc, packages, local_kinds, counts):
    """Return what is wrong with the references of one document, as (rule, message) pairs.
    `packages` maps an address to what that package exports, name -> kind, and `local_kinds`
    does the same for the project or package the document belongs to, which `#name` reaches
    (NAM-022). The rule is "ref" for a `$ref` (SCH-015) and None for any other reference."""
    found = []

    def one(ref, want, where, rule=None):
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
            found.append((rule, f"{where}: {ref} names nothing"))
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
            one(ref, want, where)

    if role == "instance":
        template(doc.get("from"), doc.get("kind"), "from")
    if role == "template":
        spec = doc.get("spec") if isinstance(doc.get("spec"), dict) else {}
        for key, want in (("conduits", "Conduit"), ("entries", "Resource"),
                          ("components", "Conduit")):
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


def check_controls(label, docs, package_docs, address):
    """For every control that a Type in `docs` lists and whose contract is in this repository:
    it is an Operation with `changes`, a map in which each key is the name of an input the
    control declares and each value is a pointer to a place the type's `schema` describes
    through `properties`, and it declares the input `revision` and the output `settings`
    (CTR-006). It also has the shape that item D13 of the record settled under REV-006:
    `revision` is not optional, and every other input is optional. That last part is a check
    of the documents in this repository against the record, not of a promise."""
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
            inputs = body.get("inputs") if isinstance(body.get("inputs"), dict) else {}
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
            if "revision" not in inputs:
                fail(f"{at} does not declare the input `revision` (CTR-006)")
            elif inputs["revision"].get("optional"):
                fail(f"{at} takes `revision` as an input that need not be supplied (REV-006)")
            for other, member in inputs.items():
                if other != "revision" and not member.get("optional"):
                    fail(f"{at} has the input {other}, which is not optional (REV-006, as "
                         f"item D13 of the record settled it)")
            if "settings" not in (body.get("outputs") or {}):
                fail(f"{at} does not declare the output `settings` (CTR-006)")
    return count


def contract_of(ref, kind, own, package_docs):
    """The contract of that kind which a selection names, where this tool has it: `#name` in
    `own`, the documents of the same project or package by name, or a qualified reference
    into a package in this repository. None where the tool does not have it."""
    if not isinstance(ref, str) or "#" not in ref:
        return None
    address, _, name = ref.partition("#")
    found = own.get(name) if not address else package_docs.get(address, {}).get(name)
    return found if isinstance(found, dict) and found.get("kind") == kind else None


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
    ("Operation", ("spec", "inputs", "*", "schema")),
    ("Operation", ("spec", "outputs", "*", "schema")),
    ("Requirement", ("spec", "inputs", "*", "schema")),
    ("Requirement", ("spec", "schema")),
    ("template", ("spec", "inputs", "*", "schema")),
    ("template", ("spec", "controls", "*", "inputs", "*", "schema")),
)
INPUT_PLACES = (
    ("Operation", ("spec", "inputs", "*")),
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
    for keyword in ("properties", "$defs"):
        members = schema.get(keyword)
        for name, sub in (members.items() if isinstance(members, dict) else ()):
            yield from defaults_in(sub, tokens + (keyword, name))
    for keyword in ("additionalProperties", "items"):
        if keyword in schema:
            yield from defaults_in(schema[keyword], tokens + (keyword,))
    for keyword in ("allOf", "anyOf", "oneOf"):
        branches = schema.get(keyword)
        for index, sub in enumerate(branches if isinstance(branches, list) else ()):
            yield from defaults_in(sub, tokens + (keyword, index))


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


def forwarded(binding):
    """The names of the inputs that the value references in one value binding name. A
    `literal` is data and is not entered."""
    if isinstance(binding, dict):
        if binding.get("source") == "input":
            yield binding.get("name")
        for item in (binding.get("object") or {}).values() if isinstance(
                binding.get("object"), dict) else ():
            yield from forwarded(item)
        for item in binding.get("list") if isinstance(binding.get("list"), list) else ():
            yield from forwarded(item)


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


def apply_rules(docs, is_core, core, package_docs, defaults=None, scope=None):
    """Return (rule, message) for each rule of RULES that one set of documents breaks. The set
    is the documents of one project or package, or of one file of examples/invalid-by-rule/.
    Manifests are left out. Each rule says how far it goes. `defaults` is the Defaults that
    the rule of NEED_VALIDATOR uses, with the `scope` of this set, or None without
    jsonschema, and then that rule is not applied."""
    found = []
    docs = [d for d in docs if isinstance(d, dict)]
    templates = {(d.get("kind"), named(d)): d for d in docs
                 if d.get("kind") in FAMILIES and "from" not in d}
    instances = [d for d in docs if d.get("kind") in FAMILIES and "from" in d]
    bound = {core + "#bound.v1"} | ({"#bound.v1"} if is_core else set())
    location = {core + "#location.v1"} | ({"#location.v1"} if is_core else set())

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

    def bounds(template, visited=()):
        """The items of `requirements` by which a Conduit template applies `bound.v1`: its own
        and, for a composite, those of the templates of its components that this tool has. A
        bound applied through the `constituents` of another requirement is not followed."""
        spec = template.get("spec") or {}
        items = [name for name, item in (spec.get("requirements") or {}).items()
                 if isinstance(item, dict) and item.get("requirement") in bound]
        for component in (spec.get("components") or {}).values():
            below = template_of(component.get("from"), "Conduit", templates, package_docs) \
                if isinstance(component, dict) else None
            if below is not None and id(below) not in visited:
                items += bounds(below, visited + (id(template),))
        return items

    # bound-once (CON-028): one Conduit template applies `bound.v1` at most once.
    for (kind, name), template in templates.items():
        if kind == "Conduit":
            items = [n for n, item in ((template.get("spec") or {}).get("requirements")
                                       or {}).items()
                     if isinstance(item, dict) and item.get("requirement") in bound]
            if len(items) > 1:
                found.append(("bound-once", f"the Conduit template {name} applies `bound.v1` "
                                            f"{len(items)} times, as {' and '.join(items)}"))

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

    # location-placement (PLC-008): a location's document states no placement. Applied where
    # the tool has the template, whose type says whether the instance is a location.
    for doc in instances:
        if doc.get("kind") == "Resource" and "placement" in doc:
            template = template_of(doc.get("from"), "Resource", templates, package_docs)
            if template is not None and (template.get("spec") or {}).get("type") in location:
                found.append(("location-placement",
                              f"the Resource {named(doc) or 'with no name'} is a location, "
                              f"made from the template {named(template)}, and its document "
                              f"states a placement"))

    # entry-reference (POOL-005): `{entry: name}` is written only in an entry of a Pool
    # template. Applied to the places of a document that are never inside an entry and always
    # hold a reference: the `placement`, `consumer` and `target` of an instance document and
    # the `target` of a declared Conduit. A value inside `arguments` or `config` is not
    # looked at, because only the schema that governs it says whether it is a reference.
    def entry(value):
        return isinstance(value, dict) and "entry" in value and "kind" not in value
    for doc in instances:
        for key in ("placement", "consumer", "target"):
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

    # operations-filled (CON-016): a Conduit template that is not composed and writes no
    # `operations` has an input whose destination is `/operations`.
    for (kind, name), template in templates.items():
        spec = template.get("spec") or {}
        if kind == "Conduit" and "capability" in spec and "operations" not in spec:
            inputs = spec.get("inputs") if isinstance(spec.get("inputs"), dict) else {}
            if not any(isinstance(i, dict) and "/operations" in (i.get("to") or [])
                       for i in inputs.values()):
                found.append(("operations-filled",
                              f"the Conduit template {name} writes no `operations`, and no "
                              f"input's destination is `/operations`"))

    # constituent-input (CTR-010): a value reference in a constituent's `arguments` names an
    # input that the composition itself declares. That its source is `input` is the schema's.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Requirement" else None
        if not isinstance(spec, dict) or not isinstance(spec.get("constituents"), dict):
            continue
        declared = set(spec["inputs"]) if isinstance(spec.get("inputs"), dict) else set()
        for local, constituent in spec["constituents"].items():
            arguments = constituent.get("arguments") if isinstance(constituent, dict) else None
            for argument, binding in (arguments.items() if isinstance(arguments, dict) else ()):
                for name in forwarded(binding):
                    if name not in declared:
                        found.append(("constituent-input",
                                      f"the constituent {local} of the Requirement contract "
                                      f"{named(doc)} gives `{argument}` the input {name}, "
                                      f"which the composition does not declare"))

    # pool-offers (CTR-004): a Type of the family Pool offers the core package's capability
    # `pool-management.v1`. The capability is recognized where `offers` names it in the core
    # package; one that reaches it through another package's re-export is not followed.
    pool_management = {core + "#pool-management.v1"} | \
        ({"#pool-management.v1"} if is_core else set())
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Type" else None
        if isinstance(spec, dict) and spec.get("family") == "Pool":
            offers = spec.get("offers") if isinstance(spec.get("offers"), list) else []
            if not any(isinstance(o, str) and o in pool_management for o in offers):
                found.append(("pool-offers", f"the Type contract {named(doc)} is of the family "
                                             f"Pool and does not offer {core}"
                                             f"#pool-management.v1"))

    # runtime-once (TPL-006): an instance has the runtimes of its type as well as those of its
    # template, so a template declares no runtime under a name its type declares. Applied
    # where the tool has the type's contract: in the same set, or in a package in this
    # repository.
    own = {named(d): d for d in docs if "from" not in d}
    for (kind, name), template in templates.items():
        spec = template.get("spec") if isinstance(template.get("spec"), dict) else {}
        declared = spec.get("runtimes") if isinstance(spec.get("runtimes"), dict) else {}
        type_doc = contract_of(spec.get("type"), "Type", own, package_docs)
        of_type = (type_doc.get("spec") or {}).get("runtimes") if type_doc else None
        for runtime in sorted(set(declared) & set(of_type if isinstance(of_type, dict) else ())):
            found.append(("runtime-once", f"the template {name} declares the runtime "
                                          f"{runtime}, which its type {named(type_doc)} "
                                          f"declares too"))

    # applies-to (CTR-005): a requirement is applied only where its `appliesTo` lists, and a
    # composition only where the `appliesTo` of every one of its constituents lists too.
    # Applied to the `requirements` of a Conduit template, which apply to a Conduit, and to
    # the `placement` of a Type contract, which applies to placement, where the tool has the
    # requirement's contract and, for a composition, as far as it has the constituents'.
    def may_not(ref, place, seen=()):
        """The requirements, the one `ref` selects and the constituents below it, whose
        `appliesTo` does not list `place`."""
        contract = contract_of(ref, "Requirement", own, package_docs)
        if contract is None or id(contract) in seen:
            return []
        body = contract.get("spec") if isinstance(contract.get("spec"), dict) else {}
        listed = body.get("appliesTo") if isinstance(body.get("appliesTo"), list) else []
        out = [] if place in listed else [named(contract)]
        parts = body.get("constituents") if isinstance(body.get("constituents"), dict) else {}
        for part in parts.values():
            if isinstance(part, dict):
                out += may_not(part.get("requirement"), place, seen + (id(contract),))
        return out
    applied = []
    for (kind, name), template in templates.items():
        spec = template.get("spec") if isinstance(template.get("spec"), dict) else {}
        if kind == "Conduit" and isinstance(spec.get("requirements"), dict):
            applied += [(f"the Conduit template {name}", item, "conduit")
                        for item in spec["requirements"].items()]
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Type" else None
        if isinstance(spec, dict) and isinstance(spec.get("placement"), dict):
            applied += [(f"the Type contract {named(doc)}", item, "placement")
                        for item in spec["placement"].items()]
    for where, (local, item), place in applied:
        ref = item.get("requirement") if isinstance(item, dict) else None
        for requirement in may_not(ref, place):
            found.append(("applies-to", f"{where} applies {ref} to "
                                        f"{'a Conduit' if place == 'conduit' else place}, as "
                                        f"{local}, and the `appliesTo` of {requirement} does "
                                        f"not list `{place}`"))

    # The three rules below follow a selection from one document into another, so they name a
    # contract by the project or package that holds it and its name. `#name` is in the project
    # or package of the document that writes it (NAM-022): this set, or the package the
    # document was found in. A qualified reference names its package.
    here = core if is_core else None

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

    def members(value):
        return value.items() if isinstance(value, dict) else ()

    # implementation-selected (TPL-004): every operation that a template's type requires, that
    # a capability the type offers includes, that the type lists as a control, or that a
    # runtime of the template or of its type, or a step of the template without `conduit`,
    # names has an implementation: a key of the template's `implementation` map, a default of
    # the type, or the default of the operation's own contract. Applied where the tool has the
    # type's contract, and for each operation whose contract it has; a capability whose
    # contract it does not have is not followed.
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
        for holder, home, whose in ((body, type_key[0], "its type"), (spec, here, "the template")):
            for runtime, ends in members(holder.get("runtimes")):
                for end in ("start", "stop"):
                    if isinstance(ends, dict):
                        need(ends.get(end), home, f"the runtime {runtime} of {whose} names "
                                                  f"the operation")
        graphs = [spec.get("setup"), spec.get("retire")]
        graphs += [value for _, value in members(spec.get("implementation"))]
        for graph in graphs:
            for step, held in members(graph.get("steps") if isinstance(graph, dict) else None):
                if isinstance(held, dict) and "conduit" not in held:
                    need(held.get("operation"), here, f"the step {step} names the operation")
        chosen = {canon(key, here) for key, _ in members(spec.get("implementation"))}
        chosen |= {canon(key, type_key[0]) for key, _ in members(body.get("implementation"))}
        for key, why in needed.items():
            operation = lookup(key, "Operation")
            if key in chosen or operation is None:
                continue
            if "implementation" not in (operation.get("spec") or {}):
                found.append(("implementation-selected",
                              f"the template {name} selects no implementation for {key[1]}, "
                              f"and neither its type {named(type_doc)} nor the operation's "
                              f"contract states a default; {why}"))

    def narrowing(key, seen=()):
        """Whether the requirement `key` names narrows requests: True, False, or None where
        this tool cannot tell because it lacks a contract. A composition narrows exactly where
        one of its constituents does (CTR-005). Also returns the names of the requirements,
        this one and those below it, that do not narrow and are not of the core package."""
        contract = lookup(key, "Requirement")
        if contract is None or id(contract) in seen:
            return None, []
        body = contract.get("spec") if isinstance(contract.get("spec"), dict) else {}
        parts = [part for _, part in members(body.get("constituents")) if isinstance(part, dict)]
        names = []
        if parts:
            states = []
            for part in parts:
                state, below = narrowing(canon(part.get("requirement"), key[0]),
                                         seen + (id(contract),))
                states.append(state)
                names += below
            narrows = True if True in states else None if None in states else False
        else:
            narrows = body.get("narrows") is True
        if narrows is False and key[0] != core:
            names.append(key[1])
        return narrows, names

    # narrows-on-conduit (CON-030): a requirement that does not narrow requests is applied to
    # a Conduit, itself or as a constituent, only where it is a contract of the core package.
    # Applied to the `requirements` of a Conduit template, as far as the tool has the
    # contracts.
    for (kind, name), template in templates.items():
        spec = template.get("spec") if isinstance(template.get("spec"), dict) else {}
        for local, item in members(spec.get("requirements")) if kind == "Conduit" else ():
            ref = item.get("requirement") if isinstance(item, dict) else None
            for requirement in narrowing(canon(ref, here))[1]:
                found.append(("narrows-on-conduit",
                              f"the Conduit template {name} applies {ref}, as {local}, and "
                              f"{requirement} does not narrow requests and is not a contract "
                              f"of the core package"))

    # obliges-narrows (CTR-005): a requirement that does not narrow requests states no
    # `obliges`. Applied where the tool can tell that the requirement does not narrow.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Requirement" else None
        if isinstance(spec, dict) and "obliges" in spec \
                and narrowing((here, named(doc)))[0] is False:
            found.append(("obliges-narrows", f"the Requirement contract {named(doc)} states "
                                             f"`obliges` and does not narrow requests"))

    # default-valid (SCH-019): a `default` satisfies the schema that states it, and an input's
    # `default` the input's own `schema`. Applied where every `$ref` of the schema can be
    # followed inside this repository; the others are counted and not validated.
    if defaults is not None:
        for doc in docs:
            for message in defaults.faults(doc, scope or {}):
                found.append(("default-valid", message))
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

    # Each directory under examples/valid/ is one project. One whose manifest states an
    # `address` is a package, and what it exports can be referred to from the others.
    packages = {core: core_kinds}
    package_docs = {core: contents(core_files)}
    by_dir = {}
    for path in groups["valid"]:
        by_dir.setdefault(path.parent, []).append((path, loaded.get(path, [])))
    local, private = {}, 0
    for directory, files in by_dir.items():
        local[directory], twice = kinds_in(files)
        for name in twice:
            fail(f"{rel(directory)}: an implementation and something else are both named "
                 f"{name} ({CLASH})")
        for path, docs in files:
            if path.name == "conduit.yaml" and docs and isinstance(docs[0], dict):
                address = docs[0].get("address")
                listed = set(docs[0].get("exports") or [])
                if address:
                    packages[address] = {n: k for n, k in local[directory].items()
                                         if n in listed or k == "implementation"}
                    package_docs[address] = contents(files)
                    private += sum(1 for n, k in local[directory].items()
                                   if k != "implementation" and n not in listed)
                    for name in sorted(n for n in listed if "#" not in n
                                       and n not in local[directory]):
                        fail(f"{rel(path)}: exports {name}, which is not a document of the "
                             f"package (MAN-007)")

    # The operations of REV-012, and the controls.
    operations = package_docs[core]
    for name in NO_REVISION:
        inputs = ((operations.get(name) or {}).get("spec") or {}).get("inputs") or {}
        if operations.get(name, {}).get("kind") != "Operation":
            fail(f"core package: {name} is not an Operation contract")
        elif "revision" in inputs:
            fail(f"core package: {name} takes `revision`, and REV-012 says it names none")
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

    # Every promise cited by the files this tool covers exists.
    prefixes = "|".join(sorted({p.split("-")[0] for p in promises}))
    cited_in = [p for p in list(core_dir.rglob("*")) + list(schema_dir.rglob("*"))
                if p.is_file() and p.suffix in (".yaml", ".md")]
    cited_in.append(ROOT / "spec" / "promises" / "manifest.yaml")
    cited = 0
    for path in cited_in:
        ids = set(re.findall(rf"\b((?:{prefixes})-[0-9]{{3}})\b",
                             path.name + "\n" + path.read_text()))
        cited += len(ids)
        for promise in sorted(ids - promises):
            fail(f"{rel(path)}: cites {promise}, which is not a promise that exists")
    print(f"5. Core package: {len(exports)} documents exported, {len(PRIVATE)} documents "
          f"deliberately private and {len(core_kinds) - len(documents)} implementation, which "
          f"is also an export. Checked {counts['checked']} references for the kind they name; "
          f"{counts['skipped']} name packages that are not in this repository and were not "
          f"checked. Checked {controls} controls, the {len(NO_REVISION)} operations that "
          f"take no `revision`, the tables of the two READMEs, and {cited} citations of "
          f"promises in {len(cited_in)} files. The example packages keep {private} documents "
          f"private.")

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
    for label, docs, is_core, scope in sets:
        for rule, message in apply_rules(docs, is_core, core, package_docs, checker, scope):
            fail(f"{label}: {message} ({RULES[rule]})")
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
        if validating:
            for index, doc in enumerate(docs):
                why = validate(bundled, doc, False)
                if why:
                    fail(f"{rel(path)} document {index + 1}: not valid against the schemas, "
                         f"so no rule is what rejects it: {why}")
                else:
                    valid_first += 1
        scope = checker.add(None, docs) if checker else None
        if checker:
            checker.close()
        found = apply_rules(docs, False, core, package_docs, checker, scope)
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
