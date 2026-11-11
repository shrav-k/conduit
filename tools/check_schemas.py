#!/usr/bin/env python3
"""Check the JSON Schemas of documents, the core package and the example documents.

    python3 tools/check_schemas.py

What it checks, and reports under these numbers:

  1. Every file of documents under spec/core/ and spec/schemas/, which is one whose name ends
     in `.yaml`, `.yml` or `.json` and does not begin with a dot (MAN-011), is UTF-8 (SUB-005)
     and strict YAML, or strict JSON for a name that ends in `.json`, as SCH-012, SCH-017,
     SCH-006 and SCH-021 require: no duplicate key, anchor, alias, merge key, tag or key that
     is not a string; no number without a JSON form; no whole number outside the I-JSON range,
     nor a number with a fraction whose binary64 value is whole and outside it; and no other
     number outside the range of IEEE 754 binary64. A JSON file whose value is an array holds
     its items as documents, and any other holds its value as one document (SCH-018).
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
     against `document.v1`, and every manifest (a file named conduit.yaml, conduit.yml or
     conduit.json) against `manifest.v1`, with a JSON Schema 2020-12 validator and format
     checking on.
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
     implementation and states no `address` breaks MAN-015. The input `identity` of `run.v1`
     holds exactly the variables and the files, and its description names the files, that
     spec/names.yaml fixes (ruling 43, Q1).
  6. Rules that no schema can express and that this tool applies itself, each named in RULES
     below with the promise it comes from. The core package and every directory under
     examples/valid/ break none of them. Every file under examples/invalid-by-rule/ is one
     set of documents, each of them valid against the schemas, that the rule its file is
     named for rejects. A file there with `-manifest-` in its name holds a manifest, which is
     valid against `manifest.v1` and which a rule on manifests rejects. Any other file there
     may also hold the manifest of its set's project, a document with no `kind`, valid against
     `manifest.v1`, from which the rules read the items that the set's templates select as
     `#name`. The rules for SCH-009 and SCH-019 compare unconditional defaults and validate
     each `default` against its schema, so they need jsonschema; without it these rules are
     not applied, and the tool says so. The rules that follow a template's selections into
     an implementation item read the items from the manifests of the core package and of the
     directories under examples/valid/, and from the set's own manifest where it has one.
     The rule for MAN-018 reads the names of the variables of `identity` from
     spec/names.yaml, and compares them without regard to case. The rule for SCH-016 compiles
     each authored pattern against SCH-016's interoperable tokens, then compiles it with
     Python's `re` after `ecma` translates anchors and character classes (see `pattern_faults`).
     examples/valid/first-start/ writes out the documents that first start makes, which no
     submission may hold (FST-006), so the rule for FST-006 is not applied to it.
  7. Promises under spec/promises/, spec/conventions/ and spec/interface/ have exactly the
     seven fields of spec/README.md, allowed layers and levels, unique IDs, and no ID named
     as retired in a head comment or the README's removed-file register is reused.

What it does not check: anything that needs an engine. It does not resolve references to
packages that are not in this repository, instantiate a template, or evaluate any of the
cross-document rules that spec/schemas/README.md lists as checked by the engine, other than
the few of step 6. A rule of step 6 is applied as far as the documents in front of the tool
show, which each rule's comment states; it is not the engine's check.

It needs PyYAML and jsonschema. If the interpreter that runs it lacks either, it runs itself
again under /usr/bin/python3. If PyYAML is still missing it stops and says that nothing was
checked. If only jsonschema is missing it runs steps 1, 2, 5 and 6, says in capitals that no
document was validated against a schema, and exits with status 3. It never reports a check it
did not run.

Where it reads otherwise than the specification, and why no verdict here depends on it. The
strict loader in reference-model/ follows the specification in each, and the first build's log
of increment 1 (docs/proposals/first-build/log-increment-1.md) lists them. Where the two
differ, this tool is not the reference, and no example may rely on a difference listed here.

  - `pattern` uses SCH-016's interoperable tokens, checked by a tokenizer. Python's `re`
    evaluates them after `ecma` translates the end anchor and character classes to their
    ECMA-262 meanings. The strict loader matches with `regress` in Unicode mode.
  - The formats are Python's checkers, but for `uuid` and `duration`, which are checked here by
    the grammars that SCH-004 cites (see `format_checker`). Python's `date-time` and `date`
    reject a leap second and the year 0000, which the grammar of RFC 3339 allows; no value
    checked here holds either.
  - YAML is read by PyYAML, which reads YAML 1.1. It rejects a tab, NEL or a line separator in
    a plain scalar, a character outside YAML's printable set even inside quotes, a byte order
    mark before a later document and a key of more than 1024 characters in a flow mapping,
    each of which YAML 1.2.2 allows, and it reads a byte order mark inside a plain or a block
    scalar, which YAML 1.2.2 does not allow there. It also reads an escape of a surrogate,
    `\\ud800`, alone or as one of a pair written as two escapes, as that code point, where the
    profile has the file rejected (SUB-005): a surrogate that is not paired is not text, and a
    YAML escape writes one code point. No file here holds any of them. The specification and
    the loader follow YAML 1.2.2; this tool keeps PyYAML, so that the checks need no other
    package.
  - JSON is read by Python's `json`, which reads an escape of a surrogate that is not one of a
    pair, `\\ud800`, as that code point, where the profile rejects the file (SUB-005). No file
    here holds one.
"""
import copy
import decimal
import ipaddress
import json
import os
import pathlib
import re
import sys
import urllib.parse

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

# The endings of a file of documents, and the names of a manifest, compared exactly (MAN-001,
# MAN-011).
ENDINGS = (".yaml", ".yml", ".json")
MANIFEST_NAMES = ("conduit.yaml", "conduit.yml", "conduit.json")

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
                         "secret-provider.v1")
CALLED_CAPABILITIES = ("runtime-provider.v1", "machine-provider.v1", "package-source.v1")
CALLED_OPERATIONS = ("run.v1", "stop.v1", "inspect-run.v1", "list-machines.v1", "place.v1",
                     "fetch-package.v1")

# The core templates from which first start alone makes instances (FST-001, FST-006), and the
# directory under examples/valid/ that writes those instances out to show what first start
# makes. That directory is a picture of the engine's own work and not a submission, so the
# rule for FST-006 is not applied to it; every other rule is.
FIRST_START_TEMPLATES = ("system", "node", "management", "access", "root")
FIRST_START_DIRECTORY = "first-start"

# The prefixes of the promise files that a ruling removed, whose IDs no file this tool covers
# may cite: GRA, of the former graphs.yaml promise file, which ruling 41 removed.
REMOVED_PREFIXES = ("GRA",)

# Where a schema may hold the core package's `secret-reference.v1` (SEC-001): as the schema
# of a place, which `properties`, `additionalProperties` and a member of `$defs` give, or as
# `items`. Under `anyOf` or `oneOf` only beside branches that are `{type: null}`. Under any
# other applicator it may not stand.
SECRET_PLACES = ("properties", "additionalProperties", "items", "$defs")

# The rules of step 6, each with the promise it comes from. A file under
# examples/invalid-by-rule/ is named for one of these promises. Ruling 42 (2026-11-06,
# docs/proposals/ruling-42/log-core.md) added plain-provider and secret-only, made
# label-conflict compare the selected items with each other, and made item-place-required
# reject a place at which the schema admits an object or an array, or null for a required
# input. Ruling 42's names (docs/proposals/ruling-42/log-rename-a.md) renamed location-type
# to node-type, for the type `node.v1`, and made label-conflict read `nodeSelector`. Its
# other names (docs/proposals/ruling-42/log-rename-b.md) made secret-only read
# `secret-provider.v1`, and item-place-required read an item's `packageArguments`, matched
# to the Package's own `packageArguments` alone. Its editorial fix E17
# (docs/proposals/ruling-42/log-editorial.md) made secret-only's message say the type of a
# secret Resource, where it said a secret store's Type, and renamed its example to
# SEC-022-type-of-a-secret-resource-that-offers-another-capability.yaml; E12 made the docstring
# of bundle say that a reference references the `spec` of a contract, as SCH-015 now says.
# Ruling 43 (2026-11-06, docs/proposals/ruling-43/log-core.md) added identity-name (MAN-018)
# and secret-header-https (SEC-024), let a set under examples/invalid-by-rule/ hold its
# project's manifest, and checks that `run.v1` holds the names of `identity` that
# spec/names.yaml fixes. Its verification (docs/proposals/ruling-43/log-integration.md,
# "Verification") made identity-name compare names without regard to case, made
# secret-header-https reject an `http` URL whose host is a variable of the Remote, and checks
# that `run.v1` holds exactly the files of `identity`, and requires every one but the
# certificate and its key.
# Round 5 of the re-audit (M86) reads the key x509BundleFile of spec/names.yaml, which was
# trustBundleFile. Its integration (docs/proposals/reaudit/round-5/log-fixes.md) added
# remote-provider (PKG-030, M25), and made item-place-required reject a place whose schema does
# not confine its value to a string, a number, a boolean or null (MAN-017, M30).
# Round 6 of the re-audit, under ruling 44 (2026-11-07; log
# docs/proposals/reaudit/round-6/log-core.md), added changes-overlap (CTR-006, finding 6-1-03)
# and fixed-schemes (PRV-016, finding 6-1-05), which compare pointers as reference tokens, and
# had check_controls read a pointer's tokens with the same function.
# Ruling 44's D06, as revised after the ruling (docs/proposals/reaudit/round-6/log-d06-d12.md),
# made implementation-selected read no default from an Operation contract, which the schema
# now rejects (CTR-002), and take the engine for an operation that the core package's `engine`
# lists; and added engine-selected (TPL-003), for a template or a Type whose `implementation`
# map selects an implementation for such an operation.
# The coordinator's decision on finding 1-03 of the first build's log of increment 1
# (docs/proposals/first-build/log-increment-1.md) added ecma-pattern (SCH-016), for a pattern
# that a schema of a document writes and that is not a regular expression of ECMA-262.
RULES = {
    "one-name": "NAM-027",
    "bound-once": "CON-028",
    "bound-target": "CON-028",
    "reserved-mark": "CTR-009",
    "entry-reference": "POOL-005",
    "operations-filled": "CON-016",
    "ref": "SCH-015",
    "default-valid": "SCH-019",
    "conflicting-defaults": "SCH-009",
    "implementation-selected": "TPL-004",
    "engine-selected": "TPL-003",
    "names-nothing": "NAM-028",
    "core-address": "NAM-021",
    "first-start-from": "FST-006",
    "node-type": "PLC-028",
    "one-run": "TPL-019",
    "label-conflict": "PLC-025",
    "item-place-required": "MAN-017",
    "project-address": "MAN-015",
    "secret-branch": "SEC-001",
    "provider-family": "PRV-001",
    "provider-conduit": "PRV-004",
    "plain-provider": "PKG-020",
    "remote-provider": "PKG-030",
    "secret-only": "SEC-022",
    "identity-name": "MAN-018",
    "secret-header-https": "SEC-024",
    "changes-overlap": "CTR-006",
    "fixed-schemes": "PRV-016",
    "ecma-pattern": "SCH-016",
    "reserved-copy-key": "IFC-033",
}
# The rules that validate a value against a schema, and so are applied only with jsonschema.
NEED_VALIDATOR = frozenset({"default-valid", "conflicting-defaults"})

errors = []


def fail(message):
    errors.append(message)


def rel(path):
    return pathlib.Path(path).relative_to(ROOT).as_posix()


def is_manifest_file(path):
    return path.name in MANIFEST_NAMES


def files_of(directory, deep=False, endings=ENDINGS):
    """The files under `directory` with one of these endings, in the directory itself or, where
    `deep`, below it too, in order. A name that begins with a dot is not read, nor anything
    below a directory whose name does, as MAN-011 has it; so not `.notes.yaml`, nor the
    `._name` that `tar` on macOS stores beside a file."""
    found = directory.rglob("*") if deep else directory.glob("*")
    return sorted(p for p in found if p.is_file() and p.suffix in endings
                  and not any(part.startswith(".") for part in p.relative_to(directory).parts))


def text_of(path):
    """The text of a file that is searched, not read as documents: a byte that is not UTF-8,
    which step 1 rejects where it reads the file, is replaced."""
    return pathlib.Path(path).read_bytes().decode("utf-8", errors="replace")


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


def read_integer(text, at):
    """A number in the form of a YAML integer, or a JSON number with no fraction or exponent.
    `at` says where it is written, for a message."""
    if re.match(r"^0[ox]", text):
        raise NotStrict(f"{at}{text} has no JSON form (SCH-017)")
    value = int(text)
    if abs(value) > SAFE_INT:
        raise NotStrict(f"{at}{text} is outside the range of SCH-006")
    return value


def _int(loader, node):
    return read_integer(node.value, f"line {node.start_mark.line + 1}: ")


# The largest finite IEEE 754 binary64 value, exactly: an integer of 309 digits (SCH-021).
LARGEST_BINARY64 = int(sys.float_info.max)


def _decimal(text):
    """The number that a text in the form of a YAML float writes, exactly, whatever its
    exponent: `(significant, scale)`, for the value `significant` times ten to the power
    `scale`, where `significant` is its digits with no zero at either end, empty for zero."""
    whole, fraction, exponent = re.fullmatch(
        r"[-+]?([0-9]*)(?:\.([0-9]*))?(?:[eE]([-+]?[0-9]+))?", text).groups()
    fraction = fraction or ""
    digits = (whole + fraction).lstrip("0")
    significant = digits.rstrip("0")
    return significant, int(exponent or 0) - len(fraction) + len(digits) - len(significant)


def read_fraction(text, at):
    """A number in the form of a YAML float, or a JSON number with a fraction or an exponent.
    Whether it has a fractional part, and whether it is in range, are read from the decimal
    value that it writes, as SCH-005, SCH-006 and SCH-021 have them: `1.5e1` is a whole number,
    and a number with a fractional part a little above the largest binary64 value is rejected,
    although it would round down to that value. It is then read as the binary64 value nearest
    to it (SCH-020), which for `1e-400` is 0; and a value so read that is whole is held to the
    range of SCH-006 too, so that `9007199254740993.5`, read as 9007199254740994, is rejected.
    The first build's log of increment 1 records why it is read so. `at` says where it is
    written, for a message."""
    if re.search(r"inf|nan", text, re.I):
        raise NotStrict(f"{at}{text} has no JSON form (SCH-017)")
    significant, scale = _decimal(text)
    whole_digits = len(significant) + scale
    largest_digits = len(str(LARGEST_BINARY64))
    if not significant or scale >= 0:
        if whole_digits > 16 or int(significant or "0") * 10 ** scale > SAFE_INT:
            raise NotStrict(f"{at}{text} is outside the range of SCH-006")
    elif whole_digits > largest_digits or (
            whole_digits == largest_digits
            and int(significant[:whole_digits]) >= LARGEST_BINARY64):
        # The fraction is not zero, so the number is greater than the largest value exactly
        # when its whole part is at least that value.
        raise NotStrict(f"{at}{text} is outside the range of IEEE 754 binary64 (SCH-021)")
    value = float(text)
    if value.is_integer() and abs(value) > SAFE_INT:
        raise NotStrict(f"{at}{text} is read as the binary64 value {value:.0f}, which is whole "
                        f"and outside the range of SCH-006")
    return value


def _float(loader, node):
    return read_fraction(node.value, f"line {node.start_mark.line + 1}: ")


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
    """Return the documents of one file, or raise NotStrict. The file is read as bytes and
    decoded as UTF-8, the one encoding the profile allows, so that a file that is not UTF-8 is
    rejected when it is read, with a message and not an exception (SUB-005). A name that ends
    in `.json` is read as JSON, any other as YAML."""
    data = pathlib.Path(path).read_bytes()
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as error:
        line = data[:error.start].count(b"\n") + 1
        raise NotStrict(f"line {line}: the byte 0x{data[error.start]:02X} is not UTF-8, and a "
                        f"file that is not UTF-8 holds no document (SUB-005)")
    if pathlib.Path(path).suffix == ".json":
        return load_json(text)
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


def load_json(text):
    """The documents of a JSON text, read strictly, as SCH-013 has JSON mean what the equivalent
    YAML means: two members with one key are rejected (SCH-012), each number is read from its
    digits as a YAML number is, and `NaN` and `Infinity` are not JSON. One leading byte order
    mark is accepted and ignored.
    A value that is an array holds its items as documents, and any other value is one document,
    which the schemas then judge (SCH-018)."""
    def members(pairs):
        seen = set()
        for key, _ in pairs:
            if key in seen:
                raise NotStrict(f"duplicate key {key!r}")
            seen.add(key)
        return dict(pairs)

    def constant(name):
        raise NotStrict(f"not JSON: {name}")

    try:
        value = json.loads(text.removeprefix("\ufeff"), object_pairs_hook=members, parse_constant=constant,
                           parse_int=lambda t: read_integer(t, ""),
                           parse_float=lambda t: read_fraction(t, ""))
    except json.JSONDecodeError as error:
        raise NotStrict(f"not JSON: {error}")
    except RecursionError:
        raise NotStrict("not read: values nested too deep")
    return value if isinstance(value, list) else [value]


# --- The names and the profile, read from the specification itself.

def names():
    text = (ROOT / "spec" / "names.yaml").read_text()
    keys = ("corePackage", "apiVersion")
    return {k: re.search(rf"(?m)^{k}: *(\S+)", text).group(1) for k in keys}


# The names of what the input `identity` of `run.v1` carries, which spec/names.yaml fixes
# (ruling 43, Q1): its two variables, and its files.
IDENTITY_VARIABLE_KEYS = ("engineUrlVariable", "identityDirVariable")
IDENTITY_FILE_KEYS = ("x509BundleFile", "jwtBundleFile", "engineIdFile", "tokenFile",
                      "certificateFile", "keyFile")
# The files of `identity` that a run is not always given (PRV-022, DL13).
IDENTITY_OPTIONAL_FILE_KEYS = ("certificateFile", "keyFile")


def identity_names():
    text = (ROOT / "spec" / "names.yaml").read_text()

    def value(key):
        found = re.search(rf"(?m)^{key}: *(\S+)", text)
        if not found:
            sys.exit(f"spec/names.yaml: no {key}. NOTHING WAS CHECKED.")
        return found.group(1)
    return ([value(k) for k in IDENTITY_VARIABLE_KEYS], [value(k) for k in IDENTITY_FILE_KEYS],
            [value(k) for k in IDENTITY_FILE_KEYS if k not in IDENTITY_OPTIONAL_FILE_KEYS])


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


def is_place(text):
    """True for a JSON Pointer in its string form, which is empty or starts with `/`."""
    return isinstance(text, str) and (text == "" or text.startswith("/"))


def tokens(text):
    """The reference tokens of a JSON Pointer in its string form, each unescaped as RFC 6901
    says, `~1` to `/` and then `~0` to `~`. The empty pointer, the whole document, has none."""
    return [t.replace("~1", "/").replace("~0", "~") for t in text.split("/")[1:]]


def overlap(first, second):
    """Whether one of two pointers is the other or lies inside it, compared as their reference
    tokens: `/limit` and `/limits` do not overlap, nor do `/a~1b` and `/a/b`, and the empty
    pointer lies over every other (CTR-006, PRV-016)."""
    one, other = tokens(first), tokens(second)
    return one[:len(other)] == other or other[:len(one)] == one


def shown(text):
    """A pointer as a message names it: the empty pointer as `""`."""
    return text if text else '""'


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
    references the `spec` of the contract it names (SCH-015), which is what the rewriting
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


# What ECMA-262 matches with `\s` (SCH-016): its WhiteSpace and LineTerminator, among them
# U+FEFF, and not U+001C to U+001F or U+0085, which Python's `re` also matches with `\s`.
# U+2000 to U+200A is a range, as it stands inside a class.
ECMA_SPACE = ("\t\n\x0b\x0c\r\x20\xa0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u3000"
              "\ufeff")


def ecma(pattern):
    r"""Translate ECMA-262's end anchor and character classes for Python's `re`.
    Empty classes and literal `[` inside a class have different syntax in `re`; escaped
    punctuation keeps its literal meaning. No unsupported authored token reaches this function.
    `\s` is retained for comparing the historical schemas in regression checks."""
    out, in_class, at = [], False, 0
    while at < len(pattern):
        char = pattern[at]
        if char == "\\" and at + 1 < len(pattern):
            following = pattern[at + 1]
            if following == "s":
                out.append(ECMA_SPACE if in_class else f"[{ECMA_SPACE}]")
            else:
                out.append(char + following)
            at += 2
            continue
        if not in_class and pattern.startswith("[^]", at):
            out.append("[\\s\\S]")
            at += 3
            continue
        if not in_class and pattern.startswith("[]", at):
            out.append("(?!)")
            at += 2
            continue
        if in_class:
            out.append("\\" + char if char in "[&~|" else char)
            in_class = char != "]"
        else:
            in_class = char == "["
            out.append("\\Z" if char == "$" else char)
        at += 1
    return "".join(out)


def ecma_pattern(validator, pattern, instance, schema):
    """The keyword `pattern`, matched as ECMA-262 matches it as far as `ecma` translates it."""
    if validator.is_type(instance, "string") and not re.search(ecma(pattern), instance):
        yield jsonschema.ValidationError(f"{instance!r} does not match {pattern!r}")


# The grammars that SCH-004 cites for `uuid`, RFC 9562, section 4, and for `duration`, RFC 3339,
# appendix A. The quoted letters of ABNF match either case (RFC 5234, section 2.3), and a digit
# is ASCII.
UUID = re.compile(r"[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}")
DUR_TIME = r"[Tt](?:[0-9]+[Hh](?:[0-9]+[Mm](?:[0-9]+[Ss])?)?|[0-9]+[Mm](?:[0-9]+[Ss])?|[0-9]+[Ss])"
DURATION = re.compile(r"[Pp](?:(?:[0-9]+[Dd]|[0-9]+[Mm](?:[0-9]+[Dd])?"
                      r"|[0-9]+[Yy](?:[0-9]+[Mm](?:[0-9]+[Dd])?)?)(?:" + DUR_TIME + r")?"
                      r"|" + DUR_TIME + r"|[0-9]+[Ww])")


def format_checker():
    """Python's checkers for the formats of JSON Schema 2020-12, but `uuid` and `duration`,
    which are checked by the grammars that SCH-004 cites. Python's `uuid` takes what
    `uuid.UUID` parses, a sign, an underscore, a digit outside ASCII and white space among
    them, and its `duration` takes ISO 8601's, such as `P1Y3D` and `PT0.5S`, which the grammar
    of RFC 3339 does not. The first build's log of increment 1 records why."""
    checker = jsonschema.FormatChecker()
    checker.checkers = dict(jsonschema.Draft202012Validator.FORMAT_CHECKER.checkers)
    for name, grammar in (("uuid", UUID), ("duration", DURATION)):
        checker.checks(name)(
            lambda value, grammar=grammar: not isinstance(value, str)
            or grammar.fullmatch(value) is not None)
    return checker


def ecma_validator(schema):
    """A JSON Schema 2020-12 validator that asserts formats, by `format_checker`, and matches
    `pattern` by `ecma`."""
    kind = jsonschema.validators.extend(jsonschema.Draft202012Validator,
                                        {"pattern": ecma_pattern})
    return kind(schema, format_checker=format_checker())


def validator(bundled, name):
    schema = dict(bundled)
    schema["$ref"] = "#/$defs/" + name
    return ecma_validator(schema)


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
            if is_manifest_file(path):
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
            if isinstance(doc, dict) and not is_manifest_file(path) and "from" not in doc}


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
    through `properties`, followed token by token; its `inputSchema` lists `revision` under
    `required`, and it declares no output `settings` (CTR-006). Other inputs may be listed
    under `required` too: a control may leave them out all the same (REV-006). That no two
    pointers of `changes` overlap is the rule changes-overlap of step 6, which has examples."""
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
                for token in tokens(place):
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
    def stands_for(schema, top, all_of=False):
        """Unconditional schemas; `allOf` participates only in conflict detection.
        SCH-009 still excludes it from filling defaults."""
        out, seen, todo = [], set(), [schema]
        while todo:
            schema = todo.pop()
            if not isinstance(schema, dict):
                continue
            identity = json.dumps(schema, sort_keys=True, ensure_ascii=False)
            if identity in seen:
                continue
            seen.add(identity)
            out.append(schema)
            ref = schema.get("$ref")
            if isinstance(ref, str):
                found, sub = pointer(top, ref)
                if found:
                    todo.append(sub)
            if all_of:
                todo.extend(reversed(schema.get("allOf", [])))
        return out

    def conflicts_at(self, schemas, top, at, ancestry=frozenset()):
        parts = [part for schema in schemas for part in self.stands_for(schema, top, True)]
        identity = json.dumps(parts, sort_keys=True, ensure_ascii=False)
        if identity in ancestry:
            return []
        ancestry = ancestry | {identity}
        values, members, out = [], {}, []
        for part in parts:
            if "default" in part and not any(
                    ecma_validator({"const":v}).is_valid(part["default"]) for v in values):
                values.append(part["default"])
            for keyword in ("properties", "patternProperties"):
                for name, sub in part.get(keyword, {}).items():
                    token = name.replace("~", "~0").replace("/", "~1")
                    members.setdefault("/" + keyword + "/" + token, []).append(sub)
            for keyword in ("items", "additionalProperties"):
                if keyword in part:
                    members.setdefault("/" + keyword, []).append(part[keyword])
            for index, sub in enumerate(part.get("prefixItems", [])):
                members.setdefault(f"/prefixItems/{index}", []).append(sub)
        if len(values) > 1:
            values = sorted(json.dumps(v, sort_keys=True) for v in values)
            out.append(f"conflicting unconditional defaults at {at}: " + " and ".join(values))
        for path, subs in members.items():
            out.extend(self.conflicts_at(subs, top, at + path, ancestry))
        return out

    def conflicts(self, doc, scope):
        out, role = [], role_of(doc, False)
        for kind, path in SCHEMA_PLACES:
            if kind != role:
                continue
            for at, root in places(doc, path):
                rewritten, needs = self.rewrite(root, scope, "root")
                if not self.follows(needs):
                    continue
                top = {"$defs": dict(self.defs, root=rewritten)}
                def visit(schema, at):
                    out.extend(self.conflicts_at([schema], top, at))
                    for keyword, key, sub in subschemas(schema):
                        if isinstance(sub, dict):
                            token = str(key).replace("~", "~0").replace("/", "~1")
                            visit(sub, at + "/" + keyword + ("/" + token if key is not None else ""))
                if isinstance(rewritten, dict):
                    visit(rewritten, at)
        return list(dict.fromkeys(out))

    def filled(self, value, schema, top, depth=0):
        """`value` with the defaults of its own properties filled in, as SCH-009 and SCH-010
        apply them: a property that is absent from an object that is present takes the
        `default` its schema states, each object that is an item of a list takes the defaults
        of its own properties, and no object is made in order to hold defaults. The object
        that holds a property may be written directly, reached through `$ref`, or be the
        schema that `items` gives. Existing map members also take defaults through matching
        `patternProperties` or applicable `additionalProperties`. Nothing is filled in
        through `allOf`, `anyOf` or `oneOf`."""
        if depth > 32:
            return value
        parts = self.stands_for(schema, top)
        if isinstance(value, dict):
            value = dict(value)
            for part in parts:
                members = part.get("properties", {})
                for name, sub in members.items():
                    if name not in value and isinstance(sub, dict) and "default" in sub:
                        value[name] = copy.deepcopy(sub["default"])
                patterns = [(re.compile(ecma(pattern)), sub)
                            for pattern, sub in part.get("patternProperties", {}).items()]
                for name in value:
                    applicable = [members[name]] if name in members else []
                    applicable.extend(sub for pattern, sub in patterns if pattern.search(name))
                    if not applicable and "additionalProperties" in part:
                        applicable.append(part["additionalProperties"])
                    for sub in applicable:
                        value[name] = self.filled(value[name], sub, top, depth + 1)
        elif isinstance(value, list):
            value = list(value)
            for part in parts:
                prefix = part.get("prefixItems", [])
                for index, item in enumerate(value):
                    sub = prefix[index] if index < len(prefix) else part.get("items")
                    if sub is not None:
                        value[index] = self.filled(item, sub, top, depth + 1)
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
        found = list(ecma_validator(top).iter_errors(value))
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


def schema_at(schema, pointer_text):
    """The schema that governs the place a JSON Pointer names, following `properties`: None
    where a `$ref` on the way, or a member that `properties` does not state, leaves this tool
    unable to tell."""
    if not isinstance(pointer_text, str):
        return None
    node = schema
    tokens = [t.replace("~1", "/").replace("~0", "~") for t in pointer_text.split("/")[1:]]
    for token in tokens:
        if not isinstance(node, dict) or "$ref" in node:
            return None
        node = (node.get("properties") or {}).get(token)
    return node


def admits(node, kinds, secret):
    """Whether a schema admits a value of one of `kinds` ("object", "array", "null") by its
    `type`, its `enum` or its `const`, or by those of a branch of its `anyOf` or `oneOf`
    (MAN-017). False for a place that `secret-reference.v1` governs, where `kinds` holds no
    "null": SEC-001 lets such a place hold the reference."""
    of = {"object": dict, "array": list, "null": type(None)}
    if not isinstance(node, dict):
        return False
    branches = [b for k in BRANCHES[1:] for b in (node.get(k) or []) if isinstance(b, dict)]
    if "null" not in kinds and (node.get("$ref") in secret
                                or any(b.get("$ref") in secret for b in branches)):
        return False
    named_types = node.get("type")
    named_types = [named_types] if isinstance(named_types, str) else named_types or []
    values = list(node.get("enum") or []) + ([node["const"]] if "const" in node else [])
    if any(t in kinds for t in named_types) \
            or any(isinstance(v, tuple(of[k] for k in kinds)) for v in values):
        return True
    return any(admits(b, kinds, secret) for b in branches)


SCALAR_TYPES = ("string", "number", "integer", "boolean", "null")


def confined(node, secret):
    """Whether a schema confines a value to a string, a number, a boolean or null by its
    `type`, its `enum` or its `const`, in itself or in each branch of its `anyOf` or `oneOf`
    (MAN-017). True for a place that `secret-reference.v1` governs, which SEC-001 lets hold the
    reference, and for a `$ref`, which this tool does not follow."""
    if not isinstance(node, dict):
        return False
    if holds_secret(node, secret) or "$ref" in node:
        return True
    named_types = node.get("type")
    named_types = [named_types] if isinstance(named_types, str) else named_types or []
    if named_types and all(t in SCALAR_TYPES for t in named_types):
        return True
    values = list(node.get("enum") or []) + ([node["const"]] if "const" in node else [])
    if ("enum" in node or "const" in node) and not any(isinstance(v, (dict, list))
                                                       for v in values):
        return True
    branches = [b for k in BRANCHES[1:] for b in (node.get(k) or [])]
    return bool(branches) and all(confined(b, secret) for b in branches)


def holds_secret(node, secret):
    """Whether `secret-reference.v1` governs a place: the schema at the place is that `$ref`,
    or a branch of its `anyOf` or `oneOf` is (SEC-001)."""
    if not isinstance(node, dict):
        return False
    branches = [b for k in BRANCHES[1:] for b in (node.get(k) or []) if isinstance(b, dict)]
    return node.get("$ref") in secret or any(b.get("$ref") in secret for b in branches)


def secret_may_go(url):
    """Whether a header that carries a secret may go to a Remote's URL on every engine
    (SEC-024): True for an `https` URL and for an `http` URL whose host is `localhost`, an IPv4
    address in 127.0.0.0/8 or the IPv6 address ::1, which an engine that can restart on
    another machine or run as several parts still rejects; False for any other `http` URL; None
    where the URL cannot be read, and the tool cannot tell. A host written as a variable of the
    Remote is none of the loopback hosts, and such a URL is not a URI, under MAN-010. Scheme
    and host are compared without regard to case (RFC 3986)."""
    if not isinstance(url, str):
        return None
    try:
        parts = urllib.parse.urlsplit(url)
        host = parts.hostname
    except ValueError:
        return None
    scheme = parts.scheme.lower()
    if scheme == "https":
        return True
    if scheme != "http":
        return None
    if "{" in parts.netloc:
        return False
    if host is None:
        return None
    if host == "localhost":
        return True
    try:
        address = ipaddress.ip_address(host)
    except ValueError:
        return False
    if address.version == 4:
        return address in ipaddress.ip_network("127.0.0.0/8")
    return address == ipaddress.ip_address("::1")


def declared_input(item, binding, key):
    """The input of server.json that an item's binding names by `key`: a variable of the
    Package's `environmentVariables`, an argument of its `packageArguments`, by `name` where it
    is named and by `valueHint` where it is positional, or a header of the Remote's `headers`.
    No key binds the Package's `runtimeArguments` (MAN-010). None for a `command` item, which
    declares none, and where the key names no input or more than one."""
    package = item.get("package") if isinstance(item.get("package"), dict) else {}
    remote = item.get("remote") if isinstance(item.get("remote"), dict) else {}
    if binding == "environment":
        found = [v for v in package.get("environmentVariables") or []
                 if isinstance(v, dict) and v.get("name") == key]
    elif binding == "packageArguments":
        found = [a for a in package.get("packageArguments") or []
                 if isinstance(a, dict) and key == (a.get("name") if a.get("type") == "named"
                                                    else a.get("valueHint"))]
    else:
        found = [h for h in remote.get("headers") or []
                 if isinstance(h, dict) and h.get("name") == key]
    return found[0] if len(found) == 1 else None


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


def interoperable_tokens(pattern):
    """SCH-016's tokens, before compilation. Return why a token is outside the list, or None.
    Kept in step with the Rust loader's tokenizer in formats.rs."""
    at, can_quantify = 0, False
    while at < len(pattern):
        char = pattern[at]
        if char == "\\":
            at += 1
            if at == len(pattern) or pattern[at] not in "^$\\.*+?()[]{}|":
                return "only escaped pattern punctuation is allowed"
            can_quantify = True
        elif char == "[":
            at += 1
            if at < len(pattern) and pattern[at] == "^":
                at += 1
            while at < len(pattern) and pattern[at] != "]":
                if pattern[at] == "\\":
                    at += 1
                    if at == len(pattern) or pattern[at] not in "^$\\.*+?()[]{}|-":
                        return "only escaped pattern punctuation is allowed in a class"
                at += 1
            if at == len(pattern):
                return "an unclosed character class"
            can_quantify = True
        elif char == "(":
            if pattern[at + 1:at + 2] == "?":
                return "only simple grouping is allowed"
            can_quantify = False
        elif char in "|^$":
            can_quantify = False
        elif char in "*+?{":
            if not can_quantify:
                return "a quantifier without an atom"
            if char == "{":
                at += 1
                begin = at
                while at < len(pattern) and pattern[at] in "0123456789":
                    at += 1
                if at == begin:
                    return "a range quantifier needs a decimal count"
                if pattern[at:at + 1] == ",":
                    at += 1
                    while at < len(pattern) and pattern[at] in "0123456789":
                        at += 1
                if pattern[at:at + 1] != "}":
                    return "an unclosed range quantifier"
            if pattern[at + 1:at + 2] == "?":
                at += 1
            can_quantify = False
        elif char == ".":
            return "the wildcard is outside the interoperable tokens"
        elif char in "]}":
            return "an unmatched closing delimiter"
        else:
            can_quantify = True
        at += 1
    return None


def pattern_faults(schema):
    """Each authored pattern outside SCH-016, as (place, pattern, why). Tokenize the
    interoperable subset first, then compile its translated ECMA-262 meaning."""
    out = []

    def visit(node, at):
        if not isinstance(node, dict):
            return
        written = []
        if isinstance(node.get("pattern"), str):
            written.append((f"{at}/pattern", node["pattern"]))
        if isinstance(node.get("patternProperties"), dict):
            written += [(f"{at}/patternProperties/{key}", key)
                        for key in node["patternProperties"]]
        for where, pattern in written:
            why = interoperable_tokens(pattern)
            if why:
                out.append((where, pattern, why))
                continue
            try:
                re.compile(ecma(pattern))
            except re.error as error:
                out.append((where, pattern, str(error)))
            except (OverflowError, RecursionError):
                # Python's `re` cannot build it, as with a repeat count past its limit, though
                # it keeps to the tokens and ECMA-262 takes it: not a fault (work order 001).
                pass
        for keyword, token, sub in subschemas(node):
            visit(sub, f"{at}/{keyword}" + ("" if token is None else f"/{token}"))
    visit(schema, "")
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
    identity_variables = [v.upper() for v in identity_names()[0]]
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
    # which first start alone makes instances, `node` among them. Applied where `from` is
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

    # node-type (PLC-028): no template but the core package's `node` has the type `node.v1`,
    # so that no actor registers a node through a template of its own. Applied where `type`
    # is the core package's qualified reference, or `#node.v1` inside the core package.
    node_type = {core + "#node.v1"} | ({"#node.v1"} if is_core else set())
    for (kind, name), template in templates.items():
        if (template.get("spec") or {}).get("type") in node_type \
                and not (is_core and kind == "Resource" and name == "node"):
            found.append(("node-type",
                          f"the {kind} template {name} has the type node.v1, which only "
                          f"the core package's template `node` has"))

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

    secret = {core + "#secret-reference.v1"} | ({"#secret-reference.v1"} if is_core else set())

    # The operations that the core package's implementation `engine` implements (MAN-012): the
    # engine implements each with no selection (TPL-004), and no template or Type selects an
    # implementation for one (TPL-003).
    engine_item = (items.get(core) or {}).get("engine")
    engine_ops = {canon(ref, core) for ref in (engine_item or {}).get("implements") or ()
                  if isinstance(engine_item, dict)}
    engine_ops.discard(None)
    for doc in docs:
        kind = doc.get("kind")
        if kind not in FAMILIES and kind != "Type" or "from" in doc:
            continue
        body = doc.get("spec") if isinstance(doc.get("spec"), dict) else {}
        home = here if kind != "Type" or is_core else None
        for key, _ in members(body.get("implementation")):
            if canon(key, home) in engine_ops:
                found.append(("engine-selected",
                              f"the {what(doc) if kind != 'Type' else 'Type'} {named(doc)} "
                              f"selects an implementation for {key}, which the engine "
                              f"implements"))

    # The operations each template of the set must have an implementation for, under TPL-004,
    # with the implementation it has for each: the engine, for an operation that `engine`
    # implements, else the key of the template's `implementation` map, else the default of its
    # type. An Operation contract states no default (CTR-002). Applied where the tool
    # has the type's contract; a capability or an operation whose contract it does not have
    # is not followed.
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
            if key in engine_ops:
                picks[key] = (core, "engine")
            elif key in chosen:
                picks[key] = chosen[key]
            elif key in by_type:
                picks[key] = by_type[key]
            elif operation is not None:
                found.append(("implementation-selected",
                              f"the template {name} selects no implementation for {key[1]}, "
                              f"its type {named(type_doc)} states no default, and the engine "
                              f"does not implement it; {why}"))
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

        # label-conflict (PLC-025): no selected item's `nodeSelector` names a label that its
        # type's `nodeSelector`, or another selected item's, names with another value.
        labels = body.get("nodeSelector") if isinstance(body.get("nodeSelector"), dict) else {}
        for key, item in chosen_items.items():
            for label, value in members(item.get("nodeSelector")):
                if label in labels and labels[label] != value:
                    found.append(("label-conflict",
                                  f"the template {name} selects {key[1]}, whose `nodeSelector` "
                                  f"gives the label {label} the value {value}, and its type "
                                  f"{named(type_doc)} gives it {labels[label]}"))
        pairs = sorted(chosen_items.items(), key=lambda pair: (pair[0][0] or "", pair[0][1]))
        for index, (key, item) in enumerate(pairs):
            for other_key, other in pairs[index + 1:]:
                theirs = other.get("nodeSelector") \
                    if isinstance(other.get("nodeSelector"), dict) else {}
                for label, value in members(item.get("nodeSelector")):
                    if label in theirs and theirs[label] != value:
                        found.append(("label-conflict",
                                      f"the template {name} selects {key[1]}, whose "
                                      f"`nodeSelector` gives the label {label} the value "
                                      f"{value}, and {other_key[1]}, whose `nodeSelector` gives "
                                      f"it {theirs[label]}"))

        # item-place-required (MAN-017): each pointer of a selected item's `environment`,
        # `packageArguments` or `headers` names a place that the type's schema requires, with
        # every member on the way required; the schema at that place confines the value, by
        # `type`, `enum` or `const`, in itself or in each branch of its `anyOf` or `oneOf`, to a
        # string, a number, a boolean or null, unless `secret-reference.v1` governs it; and
        # where the Package or the Remote marks the input `isRequired: true`, it admits no
        # null. A `$ref` on the way, or at the place, is not followed.
        for key, item in chosen_items.items():
            for binding in ("environment", "packageArguments", "headers"):
                for var, place in members(item.get(binding)):
                    if required_place(body.get("schema"), place) is False:
                        found.append(("item-place-required",
                                      f"the template {name} selects {key[1]}, whose "
                                      f"`{binding}` gives {var} the place {place}, which the "
                                      f"schema of its type {named(type_doc)} does not "
                                      f"require"))
                        continue
                    at = schema_at(body.get("schema"), place)
                    if at is not None and not confined(at, secret):
                        found.append(("item-place-required",
                                      f"the template {name} selects {key[1]}, whose "
                                      f"`{binding}` gives {var} the place {place}, at which "
                                      f"the schema of its type {named(type_doc)} admits more "
                                      f"than a string, a number, a boolean or null"))
                    declared = declared_input(item, binding, var)
                    if isinstance(declared, dict) and declared.get("isRequired") is True \
                            and admits(at, ("null",), secret):
                        found.append(("item-place-required",
                                      f"the template {name} selects {key[1]}, which marks "
                                      f"{var} `isRequired: true`, and whose `{binding}` gives "
                                      f"it the place {place}, at which the schema of its type "
                                      f"{named(type_doc)} admits null"))

        # identity-name (MAN-018): no selected item's `environment` binds a variable that has
        # the name of one that `identity` carries, as spec/names.yaml fixes them.
        for key, item in chosen_items.items():
            for var, place in members(item.get("environment")):
                if isinstance(var, str) and var.upper() in identity_variables:
                    found.append(("identity-name",
                                  f"the template {name} selects {key[1]}, whose `environment` "
                                  f"binds {var}, a variable that `identity` carries, to the "
                                  f"place {place}"))

        # secret-header-https (SEC-024): a selected `remote` item whose `headers` gives a header
        # a place that `secret-reference.v1` governs, by the type's schema, has an `https` URL,
        # or an `http` URL whose host is a loopback address. The place is found as MAN-017's is,
        # and not through a `$ref`. Whether the engine can restart on another machine or run as
        # several parts, which makes it reject the second too, is not in the documents. A host
        # written as a variable of the Remote is not a loopback address.
        for key, item in chosen_items.items():
            remote = item.get("remote") if isinstance(item.get("remote"), dict) else None
            if remote is None:
                continue
            for header, place in members(item.get("headers")):
                if holds_secret(schema_at(body.get("schema"), place), secret) \
                        and secret_may_go(remote.get("url")) is False:
                    found.append(("secret-header-https",
                                  f"the template {name} selects {key[1]}, whose header "
                                  f"{header} carries the secret at {place}, and whose URL "
                                  f"{remote.get('url')} is `http` to a host that is not a "
                                  f"loopback address"))

    # secret-branch (SEC-001): `secret-reference.v1` stands only as the schema of a place, as
    # `items`, or under `anyOf` or `oneOf` beside `{type: null}`. Applied to every schema that
    # a document of the set writes.
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

    # IFC-033 reserves contract-copy keys only in each authored schema's root $defs.
    for doc in docs:
        role = role_of(doc, False)
        for kind, path in SCHEMA_PLACES:
            if kind != role:
                continue
            for at, schema in places(doc, path):
                defs = schema.get("$defs") if isinstance(schema, dict) else None
                for key in (defs if isinstance(defs, dict) else ()):
                    address, mark, name = key.partition("#")
                    address, version_mark, version = address.partition("@")
                    ref = address + "#" + name
                    shape = package_docs[core]["qualified-reference.v1"]["spec"]
                    version_shape = package_docs[core]["version.v1"]["spec"]
                    matches = all((re.search(ecma(part["pattern"]), ref) is not None)
                                  if "pattern" in part else
                                  (re.search(ecma(part["not"]["pattern"]), ref) is None)
                                  for part in shape["allOf"])
                    version_ok = not version_mark or re.search(ecma(version_shape["pattern"]), version)
                    if mark and matches and version_ok:
                        found.append(("reserved-copy-key",
                                      f"{named(doc)}: reserved contract-copy key {key!r} "
                                      f"at {at}/$defs/{key}"))

    # ecma-pattern (SCH-016): every `pattern`, and every key of `patternProperties`, in a
    # schema that a document of the set writes is a regular expression of ECMA-262 in Unicode
    # mode. Checked against the interoperable tokens, then compiled as `pattern_faults` says.
    for doc in docs:
        role = role_of(doc, False)
        for kind, path in SCHEMA_PLACES:
            if kind != role:
                continue
            for at, schema in places(doc, path):
                for where, pattern, why in pattern_faults(schema):
                    found.append(("ecma-pattern",
                                  f"{named(doc)}: the pattern {pattern!r} at {at}{where} is "
                                  f"not an interoperable pattern of SCH-016: {why}"))

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

    # secret-only (SEC-022): a Type that offers `secret-provider.v1` offers no other capability.
    # A store that lists or writes secrets does so through another Type.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Type" else None
        if not isinstance(spec, dict):
            continue
        offers = spec.get("offers") if isinstance(spec.get("offers"), list) else []
        offered = {canon(o, here) for o in offers} - {None}
        if (core, "secret-provider.v1") in offered and len(offered) > 1:
            others = sorted(k[1] for k in offered if k != (core, "secret-provider.v1"))
            found.append(("secret-only",
                          f"the Type contract {named(doc)} offers secret-provider.v1 and also "
                          f"{' and '.join(others)}, and the type of a secret Resource offers "
                          f"no other capability"))

    # changes-overlap (CTR-006): no two pointers of a control's `changes` are equal, and none
    # lies inside another, as INS-010 says of two destinations; the empty pointer lies over
    # every other. Pointers are compared as their reference tokens. Applied to every Operation
    # contract of the set that holds `changes`, whether or not a Type of the set lists it.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Operation" else None
        changes = spec.get("changes") if isinstance(spec, dict) else None
        pairs = sorted((source, place) for source, place in members(changes) if is_place(place))
        for index, (source, place) in enumerate(pairs):
            for other, there in pairs[index + 1:]:
                if overlap(place, there):
                    how = (f"the same pointer, {shown(place)}" if tokens(place) == tokens(there)
                           else f"the pointers {shown(place)} and {shown(there)}, one of which "
                                f"lies inside the other")
                    found.append(("changes-overlap",
                                  f"the control {named(doc)} gives its inputs {source} and "
                                  f"{other} {how}, in its `changes`"))

    # fixed-schemes (PRV-016): a Type that offers `package-source.v1` lists no control whose
    # `changes` holds `/schemes`, a pointer inside it or the empty pointer, which lies over
    # it, so that a package-source provider claims the schemes it was made with for its whole
    # life. Applied where the tool has the control's contract, comparing reference tokens.
    for doc in docs:
        spec = doc.get("spec") if doc.get("kind") == "Type" else None
        if not isinstance(spec, dict):
            continue
        offers = spec.get("offers") if isinstance(spec.get("offers"), list) else []
        if (core, "package-source.v1") not in {canon(o, here) for o in offers}:
            continue
        listed = spec.get("controls") if isinstance(spec.get("controls"), list) else []
        for ref in listed:
            control = lookup(canon(ref, here), "Operation")
            body = control.get("spec") if isinstance(control, dict) else None
            for source, place in members((body or {}).get("changes")):
                if is_place(place) and overlap(place, "/schemes"):
                    found.append(("fixed-schemes",
                                  f"the Type contract {named(doc)} offers package-source.v1 "
                                  f"and lists the control {ref}, whose input {source} sets "
                                  f"{shown(place)}, which reaches the claimed `/schemes`"))

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
    # conflicting-defaults (SCH-009) rejects different defaults at one unconditional place,
    # including `$ref` siblings and `allOf`, with the same repository-bound resolution.
    if defaults is not None:
        for doc in docs:
            for message in defaults.faults(doc, scope or {}):
                found.append(("default-valid", message))
            for message in defaults.conflicts(doc, scope or {}):
                found.append(("conflicting-defaults", message))
    return found


def manifest_rules(manifest, core, holds_contracts=False):
    """Return (rule, message) for each rule on manifests that one manifest breaks. There are
    four. core-address (NAM-021): a manifest neither lists the core package under `packages`
    nor states its address as its own. project-address (MAN-015): a manifest that lists an
    implementation, or that belongs to a project that holds a contract, states `address`.
    plain-provider (PKG-020): an item marked `plain: true` implements no operation of
    `runtime-provider.v1`, `machine-provider.v1` or `package-source.v1`, which the engine
    reaches only by provider calls, and a provider call carries Conduit's keys; matched by the
    operation's qualified reference in the core package. remote-provider (PKG-030): nor does
    an item reached by `remote`, since the engine's own runner runs every such provider. The
    core package's own manifest, which is built into the engine, is not given to this
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
    items = manifest.get("implementations")
    for name, item in (items.items() if isinstance(items, dict) else ()):
        if not isinstance(item, dict):
            continue
        for ref in item.get("implements") if isinstance(item.get("implements"), list) else ():
            if not (isinstance(ref, str) and ref.partition("#")[0] == core
                    and ref.partition("#")[2] in CALLED_OPERATIONS):
                continue
            if item.get("plain") is True:
                found.append(("plain-provider",
                              f"the plain implementation {name} implements {ref}, which the "
                              f"engine invokes only by a provider call, and a plain server "
                              f"reads none of Conduit's keys"))
            if "remote" in item:
                found.append(("remote-provider",
                              f"the implementation {name}, reached by `remote`, implements "
                              f"{ref}, whose provider the engine's own runner runs, and an "
                              f"item reached by `remote` is never run"))
    return found


def check_profile_regressions():
    """Vectors for the reader and the pattern checker, run with the ordinary checks."""
    for text, expected in [('\ufeff{}', [{}]), ('\ufeff[]', []),
                           ('\ufeff{"x": "\ufeff"}', [{"x": "\ufeff"}])]:
        if load_json(text) != expected:
            fail("JSON BOM regression: a leading mark is ignored; content is kept")
    try:
        load_json('\ufeff\ufeff{}')
    except NotStrict:
        pass
    else:
        fail("JSON BOM regression: two leading marks were accepted")
    accepted = ["", "😀", "[a-z]", "[^a-z]", "[]", "[^]", "[[]", "[a&&b]",
                "a+?", "a*?", "a??", "a{2}", "a{2,3}?", "a{2,}", "a{4294967296}", "^(a|b)$",
                r"\(\?i:a\)", r"[\-]", r"[\]]", r"\^\$\\\.\*\+\?\(\)\[\]\{\}\|"]
    rejected = ["(?:a)", "(?=a)a", "(?!b)a", "(?<=a)b", "(?<!a)b", "(?<a>x)|(?<b>y)",
                r"\d", r"\w", r"\s", r"\b", r"(a)\1", r"\p{Lu}", ".", r"\u0041",
                "a++", "a*+", "a{1}+", "a???", "[a-z", "a{,2}", "a{2,1}", "a{", "[z-a]",
                "a}", "a]", r"\-", r"\a", "(?P<x>a)"]
    for pattern in accepted + rejected:
        faults = pattern_faults({"pattern": pattern})
        keyed = pattern_faults({"patternProperties": {pattern: True}})
        if bool(faults) != (pattern in rejected) or bool(keyed) != bool(faults):
            fail(f"SCH-016 token regression: {pattern!r}: {faults}, key: {keyed}")
    for pattern, text, expected in [("[]", "a", False), ("[^]", "😀", True),
                                    ("[[]", "[", True), ("a$", "a\n", False)]:
        if bool(re.search(ecma(pattern), text)) != expected:
            fail(f"ECMA class/anchor regression: {pattern!r} on {text!r}")


PROMISE_ID = r"[A-Z]+-[0-9]{3}"
ID_RANGE = rf"{PROMISE_ID}(?:\s+(?:to|through|[-–])\s+{PROMISE_ID})?"
ID_LIST = rf"{ID_RANGE}(?:(?:\s*,\s*(?:and\s+)?|\s+and\s+){ID_RANGE})*"


def expanded_ids(text):
    """IDs and inclusive same-prefix ranges in a removed-ID register."""
    ids = set(re.findall(PROMISE_ID, text))
    for match in re.finditer(r"([A-Z]+)-([0-9]{3})\s+(?:to|through|[-–])\s+"
                             r"([A-Z]+)-([0-9]{3})", text):
        if match[1] == match[3]:
            ids.update(f"{match[1]}-{n:03d}" for n in range(int(match[2]), int(match[4]) + 1))
    return ids


def retired_ids(head):
    """Read explicit registers and the older removal/folding clauses in head comments.

    A destination of a fold or an ID mentioned in its explanation is not retired.
    """
    text = " ".join(line.lstrip("# ") for line in head.splitlines())
    # Parenthetical explanations may themselves cite live promises.
    while re.search(r"\([^()]*\)", text):
        text = re.sub(r"\([^()]*\)", "", text)
    result = set()
    patterns = [
        rf"\b(?:removed|retired)(?:[- ](?:promise[- ]?)?IDs)?\s*:\s*({ID_LIST})",
        rf"\bremoved\s+({ID_LIST})",
        rf"\b({PROMISE_ID})\s+(?:was|were)\s+(?:removed|folded)\b",
        rf"(?:;\s*(?:and\s+)?|,\s+and\s+)({ID_LIST})\s+into\b",
        rf"\bfolded\s+(?:in\s+)?({ID_LIST})",
        # The register form of work order 032: "OLD: nothing replaces it (ruling NN)."
        rf"\b({ID_LIST})\s*:\s*nothing replaces it\b",
    ]
    for pattern in patterns:
        for match in re.finditer(pattern, text, re.I):
            result.update(expanded_ids(match[1]))
    # Older removal sentences also continue after descriptions of individual IDs.
    for sentence in re.split(r"\.\s+", text):
        sentence = re.split(r";\s*(?:and\s+)?(?:changed|added)\b", sentence)[0]
        if re.search(rf"\bremoved\s+{PROMISE_ID}", sentence):
            for match in re.finditer(rf"(?:;\s*(?:and\s+)?|,\s+and\s+)({ID_LIST})\s*,", sentence):
                result.update(expanded_ids(match[1]))
    return result


def check_promises():
    fields = {"id", "layer", "level", "when", "then", "observable", "vectors"}
    seen, retired = {}, {}
    total, before = 0, len(errors)
    paths = sorted(path for directory in ("promises", "conventions", "interface")
                   for path in (ROOT / "spec" / directory).rglob("*.yaml"))
    for path in paths:
        text = path.read_text(encoding="utf-8")
        head = []
        for line in text.splitlines():
            if line.strip() and not line.lstrip().startswith("#"):
                break
            head.append(line)
        for id_ in retired_ids("\n".join(head)):
            retired.setdefault(id_, set()).add(rel(path))
        try:
            # Keep duplicate authored fields visible instead of silently losing one.
            records = load(path)
        except (OSError, NotStrict) as error:
            fail(f"{rel(path)}: cannot read promises: {error}")
            continue
        if len(records) != 1 or not isinstance(records[0], list):
            fail(f"{rel(path)}: promises must be one list")
            continue
        for index, promise in enumerate(records[0], 1):
            total += 1
            label = f"{rel(path)} promise {index}"
            if not isinstance(promise, dict):
                fail(f"{label}: must be a record with exactly seven fields")
                continue
            id_ = promise.get("id")
            label += f" ({id_})"
            if set(promise) != fields:
                fail(f"{label}: fields differ: missing {sorted(fields - set(promise))}; "
                     f"extra {sorted(set(promise) - fields)}")
            if promise.get("layer") not in ("language", "convention", "interface"):
                fail(f"{label}: invalid layer {promise.get('layer')!r}")
            if promise.get("level") not in ("must", "must not", "should", "may"):
                fail(f"{label}: invalid level {promise.get('level')!r}")
            if not isinstance(id_, str):
                fail(f"{label}: ID must be a string")
                continue
            if id_ in seen:
                fail(f"{label}: duplicate ID, first in {seen[id_]}")
            seen.setdefault(id_, label)
    readme = (ROOT / "spec/README.md").read_text(encoding="utf-8")
    section = re.search(r"^### Promise files that rulings removed\n(.*?)(?=^## |\Z)",
                        readme, re.M | re.S)
    if not section:
        fail("spec/README.md: removed promise files register missing")
    else:
        for line in section[1].splitlines():
            cells = line.split("|")
            if len(cells) >= 4 and re.search(PROMISE_ID, cells[2]):
                # The IDs column starts with the retired set; later prose cites live IDs.
                for id_ in expanded_ids(cells[2].split(".", 1)[0]):
                    retired.setdefault(id_, set()).add("spec/README.md removed-file register")
    for id_ in sorted(seen.keys() & retired.keys()):
        fail(f"{seen[id_]}: live ID is retired in {', '.join(sorted(retired[id_]))}")
    print(f"7. Checked {total} promises in {len(paths)} files: seven fields, layers, levels, "
          f"unique IDs and {len(retired)} retired IDs; {len(errors) - before} errors.")


def main():
    from importlib import metadata
    validating = jsonschema is not None
    print(f"Python {sys.version.split()[0]} at {sys.executable}; PyYAML {yaml.__version__}; "
          f"jsonschema {metadata.version('jsonschema') if validating else 'NOT IMPORTABLE'}")
    check_profile_regressions()
    named_values = names()
    core = named_values["corePackage"]
    keywords, formats = profile()
    if validating:
        checkers = format_checker().checkers
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

    # 1. Strict YAML and JSON, in UTF-8. A name that begins with a dot is not read (MAN-011).
    loaded = {}
    groups = {
        "schemas": files_of(schema_dir),
        "core": files_of(core_dir),
        "valid": files_of(valid_dir, deep=True),
        "invalid": files_of(invalid_dir, deep=True),
        "by-rule": files_of(by_rule_dir, deep=True),
    }
    for group, paths in groups.items():
        for path in paths:
            try:
                loaded[path] = load(path)
            except NotStrict as problem:
                if group == "invalid":
                    loaded[path] = problem
                else:
                    fail(f"{rel(path)}: not read strictly: {problem}")
    count = sum(len(v) for v in loaded.values() if isinstance(v, list))
    print(f"1. Read {len(loaded)} files holding {count} documents as UTF-8 and as strict YAML or "
          f"JSON.")

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
                why = validate(bundled, doc, is_manifest_file(path))
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
    manifest = next((d[0] for p, d in core_files if is_manifest_file(p) and d), {})
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
        held = [d for p, ds in files if not is_manifest_file(p) for d in ds]
        has_contract = any(isinstance(d, dict) and d.get("kind") in CONTRACTS for d in held)
        manifests = [ds[0] for p, ds in files
                     if is_manifest_file(p) and ds and isinstance(ds[0], dict)]
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
    # The input `identity` of `run.v1` holds exactly the variables that spec/names.yaml fixes,
    # each required, and the description of `run.v1` names each of the files there.
    variables, identity_files, required_files = identity_names()
    run_spec = (operations.get("run.v1") or {}).get("spec") or {}
    identity = ((run_spec.get("inputSchema") or {}).get("properties") or {}).get("identity")
    environment = ((identity or {}).get("properties") or {}).get("environment") or {}
    if set(environment.get("properties") or {}) != set(variables) \
            or set(environment.get("required") or []) != set(variables):
        fail(f"core package: run.v1's `identity.environment` does not hold and require exactly "
             f"{', '.join(variables)}, the variables spec/names.yaml fixes")
    files = ((identity or {}).get("properties") or {}).get("files") or {}
    if set(files.get("properties") or {}) != set(identity_files) \
            or set(files.get("required") or []) != set(required_files) \
            or files.get("additionalProperties") is not False:
        fail(f"core package: run.v1's `identity.files` does not hold exactly "
             f"{', '.join(identity_files)}, the files spec/names.yaml fixes, closed, and require "
             f"{', '.join(required_files)}")
    described = ((operations.get("run.v1") or {}).get("metadata") or {}).get("description", "")
    for name in identity_files:
        if f"`{name}`" not in described:
            fail(f"core package: run.v1's description does not name the file `{name}` of "
                 f"`identity`, which spec/names.yaml fixes")
    controls = check_controls("core package", [d for _, ds in core_files for d in ds],
                              package_docs, core)
    for directory, files in by_dir.items():
        address = next((d[0].get("address") for p, d in files
                        if is_manifest_file(p) and d and isinstance(d[0], dict)), None)
        if address:
            controls += check_controls(rel(directory), [d for _, ds in files for d in ds],
                                       package_docs, address)

    counts = {"checked": 0, "skipped": 0}
    for path, docs in core_files:
        for doc in docs:
            if isinstance(doc, dict):
                role = role_of(doc, is_manifest_file(path))
                for _, message in check_references(role, doc, packages, core_kinds, counts):
                    fail(f"{rel(path)}: {message}")
    for directory, files in by_dir.items():
        for path, docs in files:
            for doc in docs:
                if isinstance(doc, dict):
                    role = role_of(doc, is_manifest_file(path))
                    for _, message in check_references(role, doc, packages, local[directory],
                                                       counts):
                        fail(f"{rel(path)}: {message}")
    used = set()
    for path in list(loaded):
        used.update(re.findall(re.escape(core) + r"#([a-z0-9.-]*[a-z0-9])", text_of(path)))
    for name in sorted(used - set(core_kinds)):
        fail(f"a file refers to {core}#{name}, which the core package does not hold")

    # Every promise cited by the files this tool covers exists. The prefix of a promise file
    # that a ruling removed is looked for too, so that no citation of one of its IDs remains.
    prefixes = "|".join(sorted({p.split("-")[0] for p in promises} | set(REMOVED_PREFIXES)))
    cited_in = [p for d in (core_dir, schema_dir) for p in files_of(d, True, ENDINGS + (".md",))]
    cited_in.append(ROOT / "spec" / "promises" / "manifest.yaml")
    cited = 0
    for path in cited_in:
        text = text_of(path)
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
    core_docs = [d for p, ds in core_files if not is_manifest_file(p) for d in ds]
    sets = [("the core package", core_docs, True,
             checker.add(core, core_docs) if checker else None)]
    for directory, files in by_dir.items():
        docs = [d for p, ds in files if not is_manifest_file(p) for d in ds]
        address = next((d[0].get("address") for p, d in files
                        if is_manifest_file(p) and d and isinstance(d[0], dict)), None)
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
                             for p, ds in files if not is_manifest_file(p) for d in ds)
        for path, docs in files:
            if is_manifest_file(path):
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
            continue                       # not read strictly: already reported in step 1
        if checker is None and all(rule in NEED_VALIDATOR
                                   for rule, named_for in RULES.items() if named_for == promise):
            not_run += 1
            print(f"   NOT RUN {path.name}: the rule for {promise} needs jsonschema")
            continue
        is_manifest = "-manifest-" in path.name
        # A set may hold its project's manifest, a document with no `kind`, from which the
        # rules read the items that its templates select.
        own_manifests = [] if is_manifest else [d for d in docs
                                                 if isinstance(d, dict) and "kind" not in d]
        held = docs if is_manifest else [d for d in docs
                                         if not (isinstance(d, dict) and "kind" not in d)]
        if validating:
            for index, doc in enumerate(docs):
                why = validate(bundled, doc, is_manifest or not (isinstance(doc, dict)
                                                                 and "kind" in doc))
                if why:
                    fail(f"{rel(path)} document {index + 1}: not valid against the schemas, "
                         f"so no rule is what rejects it: {why}")
                else:
                    valid_first += 1
        if is_manifest:
            found = [fault for doc in docs for fault in manifest_rules(doc, core)]
        else:
            scope = checker.add(None, held) if checker else None
            if checker:
                checker.close()
            own = dict(items)
            own[None] = next((m.get("implementations") or {} for m in own_manifests), {})
            found = apply_rules(held, False, core, package_docs, checker, scope, own)
            kinds, _ = kinds_in([(pathlib.Path("conduit.yaml"), own_manifests), (path, held)])
            for doc in held:
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
    check_promises()
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
