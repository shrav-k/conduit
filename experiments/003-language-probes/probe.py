"""Offline research probes, not a Conduit parser or conformance implementation.

Run with the pinned dependencies in requirements.txt. No network or provider calls.
"""

import copy
import importlib.metadata
import json
import math
import platform

from jsonschema import Draft202012Validator, FormatChecker
from ruamel.yaml import YAML
from ruamel.yaml.constructor import DuplicateKeyError


results = []


def record(name, observed, expected):
    assert observed == expected, (name, observed, expected)
    results.append({"case": name, "observed": observed, "expected": expected})


def valid(schema, value, **options):
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, **options).is_valid(value)


schema = {
    "type": "object",
    "properties": {"limit": {"type": "integer", "default": 10}},
    "additionalProperties": False,
}
value = {}
before = copy.deepcopy(value)
record("default annotation does not insert a value", valid(schema, value) and value == before, True)
record("closed record rejects misspelled field", valid(schema, {"limti": 10}), False)
record("schema validation does not coerce a string", valid(schema, {"limit": "10"}), False)
record("JSON Schema integer accepts integral number 10.0", valid(schema, {"limit": 10.0}), True)
record("absent optional field is distinct from null", valid(schema, {"limit": None}), False)
record("empty array is not missing", valid({"type": "array", "minItems": 1}, []), False)
record("unknown semantic keyword is ignored by stock validator", valid({"type": "integer", "conduitMustBeEven": True}, 3), True)
record("readOnly annotation does not authorize a write", valid({"type": "integer", "readOnly": True}, 5), True)
record("format is not checked by default", valid({"type": "string", "format": "ipv4"}, "not-an-address"), True)
record("format can be checked explicitly", valid({"type": "string", "format": "ipv4"}, "not-an-address", format_checker=FormatChecker()), False)
closed_base = {"type": "object", "properties": {"path": {"type": "string"}}, "additionalProperties": False}
extension = {"type": "object", "properties": {"mode": {"type": "string"}}}
record("allOf does not reopen a closed base", valid({"allOf": [closed_base, extension]}, {"path": "/tmp/demo", "mode": "attach"}), False)
open_base = {"type": "object", "properties": {"path": {"type": "string"}}}
record("unevaluatedProperties closes the combined shape", valid({"allOf": [open_base, extension], "unevaluatedProperties": False}, {"path": "/tmp/demo", "mode": "attach"}), True)
record("combined shape still rejects unknown fields", valid({"allOf": [open_base, extension], "unevaluatedProperties": False}, {"path": "/tmp/demo", "typo": 1}), False)
record("allOf intersects bounds rather than overriding", valid({"allOf": [{"type": "integer", "maximum": 20}, {"type": "integer", "maximum": 10}]}, 15), False)

yaml = YAML(typ="safe", pure=True)
yaml.version = (1, 2)
record("YAML 1.2 yes is a string", type(yaml.load("choice: yes\n")["choice"]).__name__, "str")
record("safe YAML loader still accepts anchors and aliases", yaml.load("a: &x [1]\nb: *x\n"), {"a": [1], "b": [1]})
record("safe YAML loader still expands merge keys", yaml.load("base: &b {limit: 10}\nconfig: {<<: *b}\n")["config"], {"limit": 10})
record("safe YAML loader accepts a non-finite scalar", math.isinf(yaml.load("limit: .inf\n")["limit"]), True)
record("safe YAML loader recognizes timestamp beyond Core scalar set", type(yaml.load("day: 2026-11-02\n")["day"]).__name__, "date")
try:
    yaml.load("limit: 10\nlimit: 20\n")
except DuplicateKeyError:
    duplicate = "rejected"
else:
    duplicate = "accepted"
record("this loader rejects duplicate keys", duplicate, "rejected")

print(json.dumps({
    "scope": "Library behavior probes; no Conduit implementation or provider execution",
    "python": platform.python_version(),
    "dependencies": {name: importlib.metadata.version(name) for name in ("jsonschema", "ruamel.yaml")},
    "cases": results,
}, indent=2))
