# Increment 1: the strict loader

Not binding. The log of what increment 1 of the first build found, under
[ruling 46](../ruling-46/README.md), point 4; [the README](README.md) says what a finding is and
who settles it. The specification governs where this log and it differ.

Increment 1 is the strict loader (A43), the crate `conduit-loader` in
[reference-model/](../../../reference-model/README.md), written on the branch `build/loader` from
`main` at `cdf3236`, on 2026-11-09. It reads a document in YAML or JSON as the schema profile
and SCH-005, SCH-006, SCH-012, SCH-013, SCH-017, SCH-018, SCH-020 and SCH-021 say, reads a
project's directory as MAN-001, MAN-002, MAN-011 and SUB-010 say, and validates each document
against the schemas under `spec/schemas/` and `spec/core/`, bundled as `tools/check_schemas.py`
bundles them, with formats asserted (SCH-001, SCH-004) and patterns matched as ECMA-262
(SCH-016). Its done-when, in [the first build](../../implementation/FIRST_BUILD.md#increments),
holds: every document under `examples/valid/` is accepted, each directory read as a project;
every file under `examples/invalid/` is rejected by the step that `check_schemas.py` rejects it
by; every document under `examples/invalid-by-rule/` passes the schemas; and the cases written
for SCH-004's grammars and SCH-016's patterns pass.

Eighteen findings, all settled: the first thirteen from the increment and its verifiers, and
five from the review of 2026-11-09. Three are defects of a tool, two fixed with vectors and one
stated in the tool; fifteen concern the promises. The coordinator settled the first findings
under ruling 46, point 4, and the review completed or corrected those settlements below.
Ruling 48, point 3, settles 1-04 and the review's 1-17. Every other settlement is the
coordinator's own, and the owner can strike it. None adds or removes a feature or changes what
Conduit is for. The original entries record what the increment found and did; the section
[after the review](#after-the-review-of-2026-11-09) gives the later changes and their vectors.
| # | Finding | Class | Promises | Settled |
| --- | --- | --- | --- | --- |
| 1-01 | A rejection by a schema names no one promise | Gap | OPS-005, IFC-025, OBJ-023, SUB-005 | Settled by the coordinator: SUB-005, with a vector |
| 1-02 | `check_schemas.py` matched `$` and `\s` otherwise than ECMA-262 | Defect of a tool | SCH-016, OBJ-001 | Fixed, with a vector |
| 1-03 | When a schema whose pattern is not ECMA-262 is rejected | Gap | SCH-016, SCH-002, SCH-003 | Settled by the coordinator: SCH-016, with a vector |
| 1-04 | ECMA-262 patterns are needlessly hard outside JavaScript | Needless complexity | SCH-016, INS-014 | Settled by ruling 48, point 3; applied in 1-17 |
| 1-05 | The encoding of a file, and a file that is not YAML or JSON | Gap | SUB-005, SCH-013 | Settled by the coordinator: the profile and SUB-005, with a vector |
| 1-06 | Which names in a directory are read | Gap | MAN-011, MAN-001, SUB-010, PKG-022 | Settled by the coordinator: MAN-011, SUB-010 and SUB-005, with a vector |
| 1-07 | A JSON file whose value is not a map or a list of maps | Ambiguity | SCH-018, SCH-013, SUB-005 | Settled by the coordinator: SCH-018, with a vector |
| 1-08 | `check_schemas.py` read numbers and two formats otherwise than the promises | Defect of a tool | SCH-020, SCH-021, SCH-004, SCH-019 | Fixed, with vectors |
| 1-09 | `check_schemas.py` reads YAML 1.1 | Defect of a tool | SCH-012, SUB-005 | Stated in the tool; its parser settled by the coordinator |
| 1-10 | Whether a number is whole, by its decimal or by its binary64 value | Ambiguity | SCH-005, SCH-006, SCH-020, SCH-021 | Settled by the coordinator: SCH-006 and SCH-020, with a vector |
| 1-11 | A rejection for a limit of an engine names no promise | Gap | OPS-005, IFC-025, SUB-005, DL10 | Settled by the coordinator: SUB-011 added, and DL10, with a vector |
| 1-12 | I-JSON's rules on strings | Ambiguity | SCH-013, SCH-012 | Settled by the coordinator: the profile, with a vector |
| 1-13 | The case of the letters of a duration | Ambiguity | SCH-004 | Settled by the coordinator: the profile, with a vector |
| 1-14 | Schema rejections still cite other promises and lack keyword locations | Contradiction | SUB-005, PKG-023 and the schema-encoded rules | Settled by the coordinator: attribution and diagnostic location, with vectors |
| 1-15 | JSON's byte order mark and the source of the UTF-8 rule | Contradiction | SUB-005, SCH-013; the profile | Settled by the coordinator: one leading mark in either syntax, with vectors |
| 1-16 | SUB-011 has no floor and leaves files, submissions and archives out | Gap | SUB-011, PKG-023, DL10 | Settled by the coordinator: floor and scope, with vectors |
| 1-17 | Patterns exceed JSON Schema's interoperable tokens | Needless complexity | SCH-016, INS-014 | Settled by ruling 48, point 3, with vectors |
| 1-18 | A package file's hash can be mistaken for a pin of its dependencies | Gap | MAN-010 | Settled by the coordinator: what a pin holds, with an example |

## 1-01. A rejection by a schema names no one promise

**Class:** gap. **Promises:** OPS-005, IFC-025, OBJ-023, SUB-005, and the promises that each
Schema contract encodes.

A registration that is rejected reports invalid with its diagnostics (OBJ-023), and among them
an error diagnostic whose `ruleId` is the ID of the promise that decided it (OPS-005, IFC-025).
Most documents are rejected by a schema. A Schema contract encodes several promises, and they
are listed only in a comment at the head of its file, "Promises encoded", contract by contract
and not keyword by keyword: `template-metadata.v1` encodes OBJ-018 and OBJ-001, so an engine
cannot tell from the schemas whether a template with no `name` breaks the one or the other.
`document.v1` joins them all. Two engines can name different promises for one document, and a
conformance vector that states the diagnostics it expects needs one answer.

**What the loader does.** It names SUB-005 for every rejection by a schema, since `document.v1`
encodes SUB-005 as "anything else is not a document", and says in the message which Schema
contract's keyword failed, with the place as a JSON Pointer and as a line and column: "not valid
against `name.v1`: "Console" does not match the pattern ...". It names SUB-005 for a manifest
too.

**Fix, made by the coordinator.** Option (a) of the two that the log put, the loader's reading.
SUB-005's observable now says that a document, or the manifest, that a Schema contract rejects
is rejected under SUB-005, whichever promise that contract encodes: the diagnostic names the
place in the document by a JSON Pointer, in a logical location, and its message names the
Schema contract. No schema changed, and every engine can follow it. Option (b), each Schema
contract carrying the promises it encodes keyword by keyword, would change every schema to give
an answer that (a) already gives. The vector is the loader's test
`every_invalid_example_is_rejected_by_the_step_its_name_gives`, in `tests/examples.rs`, which
asserts SUB-005 for every file under `examples/invalid/` that a schema rejects, beside
`every_fault_of_a_project_is_reported`, in `tests/project.rs`, which asserts the place and the
contract; the conformance vectors (A39) will carry it. Promise changed: SUB-005. The owner can
strike it.

**After the review.** The review completed this settlement in 1-14: schema-encoded rejections name SUB-005 throughout, including at installation, and diagnostics carry the failing keyword's location.

## 1-02. `check_schemas.py` matched `$` and `\s` otherwise than ECMA-262

**Class:** a defect of a tool. **Promises:** SCH-016; OBJ-001, through `name.v1`.

`tools/check_schemas.py` validates with Python's jsonschema, which matches `pattern` with
Python's `re`. Its header said that the patterns of the schemas use only syntax the two share.
They do, but two pieces of that syntax mean other things in `re`. `$` also matches before a line
feed at the end of the string, so a template named `"console\n"` passed `name.v1`, whose pattern
ends in `$`, though under SCH-016 it does not match. And `\s` matches U+001C to U+001F and
U+0085 and not U+FEFF, where ECMA-262 does the reverse, so the two patterns `^https?://[^\s]+$`
of `server-package.v1` and `server-remote.v1` gave other verdicts on those characters. The
promise is clear; the tool that every reader of the examples runs disagreed with it, and an
engine built to agree with the tool would be wrong.

**What the loader does.** It matches every `pattern` with an engine of ECMA-262's regular
expressions in Unicode mode, and rejects the name that ends in a line feed.

**Fix, made.** `check_schemas.py` gives `re` each pattern translated by a new function, `ecma`:
`$` outside a class becomes `\Z`, the end of the string, and `\s`, in a class or outside one,
becomes ECMA-262's own set of spaces, written with escapes. It does this for validation and for
the defaults of SCH-019. Its header says which syntax it translates, that no pattern here uses
`\d`, `\w` or `\b`, and that the one `.`, in the lookahead of `qualified-reference.v1`, stands
where the rest of the pattern admits only ASCII, so that what `.` matches otherwise cannot change
a verdict. The vector is
[OBJ-001-name-ending-in-a-line-feed.yaml](../../../spec/schemas/examples/invalid/OBJ-001-name-ending-in-a-line-feed.yaml)
under `spec/schemas/examples/invalid/`, named, as every file there is, for the promise the
document breaks, OBJ-001; its comment says that SCH-016 is why a pattern rejects it. The tool
and the loader both reject it by a schema. The counts of
[spec/README.md](../../../spec/README.md) and [spec/schemas/README.md](../../../spec/schemas/README.md)
include it, and the second says why the example was added. No promise changed.

## 1-03. When a schema whose pattern is not ECMA-262 is rejected

**Class:** gap. **Promises:** SCH-016, SCH-002, SCH-003.

SCH-016 says that a pattern is a regular expression of ECMA-262, and the table of
[what a schema cannot express](../../../spec/schemas/README.md#what-a-schema-cannot-express)
leaves the check to the engine. No promise says what happens to a schema whose pattern is not
one, such as `(?P<x>a)`, Python's named group, or `a{` and `\-`, which ECMA-262 allows without
the `u` flag and rejects with it: whether the document that holds it is rejected when its
package is installed or when it is submitted, as SCH-002 rejects a keyword outside the profile
and SCH-003 a format, or only when a value is checked against it. `schema.v1` checks only that
a pattern is a string.

**What the loader does.** It rejects the document when it is read, under SCH-016, for each
`pattern` and each key of `patternProperties` in the places where a document holds a schema,
which are the places that `SCHEMA_PLACES` in `check_schemas.py` lists, once the document has
passed the schemas. `Specification::check` gives this verdict beside the schemas', as reading a
project does; `Specification::validate` applies the schemas alone.

**Fix, made by the coordinator.** The loader's reading. SCH-016 now says that a key of
`patternProperties` is a pattern too, and its observable that a schema written in a contract or
a template whose pattern is not a regular expression of ECMA-262, in Unicode mode and in the
edition the profile names, makes the document that holds it rejected, when its package is
installed or when it is submitted, as SCH-002 rejects a keyword outside the profile, with a
diagnostic naming the pattern and where it appears. `check_schemas.py` has a new rule in its
step 6, `ecma-pattern`, which compiles with Python's `re` every `pattern`, and every key of
`patternProperties`, that a schema of a document writes at the places that `SCHEMA_PLACES`
lists, once `ecma` has translated it, and rejects one that `re` cannot compile. The two engines
read some patterns otherwise, so the rule catches only a pattern that both reject, and says so:
it misses one that `re` takes and ECMA-262 rejects, such as `(?P<x>a)` or `a{`, and it does not
judge one that holds syntax that ECMA-262 has and `re` lacks, such as `\p{Lu}`, a named group or
a lookbehind. The loader checks every pattern by ECMA-262 itself. The vector is
[SCH-016-pattern-that-is-not-ecma-262.yaml](../../../spec/schemas/examples/invalid-by-rule/SCH-016-pattern-that-is-not-ecma-262.yaml)
under `examples/invalid-by-rule/`: a Schema contract whose pattern is `[z-a]`, a class whose
range runs backwards, which both engines reject. It passes the schemas, as every set there does,
and the tool's rule rejects it. The loader's test
`check_rejects_the_invalid_by_rule_examples_of_sch_016_alone`, in `tests/examples.rs`, has
`Specification::check` reject it under SCH-016, and pass every other file there, whose rules come
with the core. The table of what a schema cannot express marks the rule, and the counts include
the example. Promise changed: SCH-016. The owner can strike it.

## 1-04. ECMA-262 patterns are needlessly hard outside JavaScript

**Class:** needless complexity. **Promises:** SCH-016; INS-014, under which the core validates
a configuration against its type's schema.

SCH-016 asks every engine for ECMA-262's meaning with the `u` flag, "whichever
regular-expression library the engine is built with". Neither of Rust's common engines,
`regex` and `fancy-regex`, is ECMA-262's, and the `jsonschema` crate translates a pattern into
their syntax as far as it can and passes a pattern with a lookaround to `fancy-regex`
untranslated. The specification's own patterns use lookahead, in `address.v1`,
`qualified-reference.v1`, `destination.v1` and the destinations of `templates.yaml`. Python's
`re` differs too (1-02). The loader replaces the crate's `pattern` with `regress`, an engine of
ECMA-262. Even that engine is not the edition the profile names: `regress` implements the
current edition, and accepts two pieces of syntax of the edition of 2025, a group that sets
flags, `(?i:a)`, and a name given to two groups, which the edition of 2020 that JSON Schema
2020-12 cites rejects, so the loader rejects them itself. The crate offers no way to replace the
engine with which it matches `patternProperties`, and `additionalProperties` beside it. No
schema of the specification uses `patternProperties`, so the verdicts of increment 1 are exact;
but the profile allows it, an author's schema may use it, and the core validates configurations
against authors' schemas from increment 2.

**What the loader does.** It matches `pattern` as ECMA-262 does, in the edition the profile
names, checks that every pattern an author writes is one of ECMA-262 (1-03), and leaves
`patternProperties` to the crate.

**Fix, made by the coordinator.** Option (a): ECMA-262 is kept, and the text is unchanged. The
core matches `patternProperties`, and the `additionalProperties` beside it, itself when it
validates the schemas that authors write, from increment 2 (below,
[what increment 1 did not do](#what-increment-1-did-not-do)). Option (b), narrowing the
profile's patterns to I-Regexp, would have the specification's own patterns rewritten without
lookahead, and option (c), dropping `patternProperties`, would take from the profile a keyword
of JSON Schema that an author may expect; this version needs neither. Left as a question for
before the independent builds, which SCH-016 binds as it binds this one, and not ruled: whether
to narrow the profile's patterns to I-Regexp, RFC 9485, the subset of regular expressions meant
to be matched alike by every engine. No promise changed, and no vector was added. The owner can
strike it.

**After the review.** Ruling 48, point 3, replaced this settlement: SCH-016 keeps ECMA-262 in Unicode mode and restricts its tokens to the interoperable list. Finding 1-17 applies it; no I-Regexp question remains open.

## 1-05. The encoding of a file, and a file that is not YAML or JSON

**Class:** gap. **Promises:** SUB-005, SCH-013; the profile's rows for YAML 1.2.2, JSON and
I-JSON.

The profile names YAML 1.2.2, which allows UTF-8, UTF-16 and UTF-32 and a byte order mark; JSON,
which RFC 8259 has in UTF-8 between systems, and whose reader may ignore a byte order mark or
reject it; and I-JSON, which requires UTF-8, but only for its value model. Nothing says which
encodings a file may be in. Nor does a promise name the rejection of a file that is not YAML at
all, or of a `.json` file that is not JSON; SUB-005 comes closest, "something that is not a
template, an instance, a contract or the project's one manifest".

**What the loader does.** It reads UTF-8 alone. It allows a byte order mark in YAML, as YAML
does, and rejects one in JSON, as `serde_json` does. A file that is not UTF-8, not YAML, or for
a `.json` file not JSON, is rejected under SUB-005, at the place where reading stopped, and that
is its one rejection. `check_schemas.py` read a file in the default encoding of the machine,
UTF-8 on those it has run on, and stopped with an exception rather than a diagnostic on bytes
that are not; nor does it read YAML as YAML 1.2.2 does (1-09).

**Fix, made by the coordinator.** The profile has a row for the encoding: every file is UTF-8,
as I-JSON requires; a YAML file may begin with a byte order mark, as YAML 1.2.2 allows, and a
JSON file may not, since RFC 8259 does not let one be added. SUB-005's observable says that a
file that is not UTF-8, or that is not YAML or JSON as the ending of its name says, holds no
document and is rejected under SUB-005, with a diagnostic naming the file. That narrows what a
reader of YAML alone would take, UTF-16 and UTF-32, which no example uses; it adds no feature
and removes none that Conduit is for. `check_schemas.py` now reads every file as bytes and
decodes it as UTF-8 itself, so that a file that is not UTF-8 is rejected when read, with its
line and the byte, and never stops the tool; where its step 5 searches a file for citations, it
replaces such a byte. `tools/set_names.py`, which reads every file under `spec/` as text, now
reads and writes such a byte unchanged. The vector is
[SUB-005-not-utf-8.yaml](../../../spec/schemas/examples/invalid/SUB-005-not-utf-8.yaml) under
`examples/invalid/`: an Interface template whose description holds the byte 0xE9, the letter é
in Latin-1, which is not UTF-8. The tool and the loader both reject it when they read it, and
the loader's fixture and the counts of the two READMEs include it. Promise changed: SUB-005;
and the profile. The owner can strike it.

**After the review.** The review corrected this settlement in 1-15: either syntax may begin with one byte order mark, which is ignored, and UTF-8 is the profile's own rule.

## 1-06. Which names in a directory are read

**Class:** gap. **Promises:** MAN-011, MAN-001, SUB-010; PKG-022 and MAN-013, for a package's
archive.

MAN-011 reads "the files whose names end in `.yaml`, `.yml` or `.json` and that lie in the
directory itself". It leaves four cases to the reader. A name that begins with a dot. The case
of the ending and of the manifest's name, `notes.YAML` or `Conduit.yaml`, which a file system
that ignores case also lets an author write. A link in the directory to a file elsewhere. And
an entry with such a name that is not a file that can be read: a link that leads nowhere, a
file without permission to read it, or something that is not a regular file. The manifest's
promises cite Terraform's reading of a module, and Terraform skips a name that begins with a
dot, that ends with `~`, or that begins and ends with `#`, which editors leave; kubectl, which
they also cite, reads by the ending alone. The case that matters: on macOS, `tar` stores the
extended attributes of a file as a separate `._` entry unless told not to, so the archive of a
package (PKG-022) made there and unpacked as MAN-013 says may hold `._templates.yaml`, which is
not text, and which MAN-011 then reads as a document and rejects.

**What the loader did.** It read as the text was written: every name with one of the three
endings, compared exactly, so `Conduit.yaml` is a file of documents and `notes.YAML` is not
read; a name that begins with a dot was read; a link is followed as the file system follows it,
and one that leads to a directory is not entered. An entry with one of the endings that cannot
be read, a link that leads nowhere among them, or that is not a regular file, is rejected under
SUB-010, "cannot be read", and never skipped, so that a link named `conduit.yaml` that leads
nowhere cannot leave a project read as one with no manifest. The rest of the directory is still
read, and every fault in it reported.

**Fix, made by the coordinator.** Option (b), which keeps the files that tools leave behind out
of a project and changes no project that an author wrote. MAN-011 now says that its documents
are the files whose names end in `.yaml`, `.yml` or `.json` and do not begin with a dot; that
names and endings are compared exactly, the manifest's three names and the three endings in
lower case, so `Conduit.yaml` is a file of documents and `notes.YAML` is not read; that a link
to a file is read as that file, and a link to a directory is not entered; and that an entry
with a name it reads that cannot be read as a regular file, a link that leads nowhere, a file
that the engine may not read or anything that is not a regular file, is rejected under SUB-010
and never skipped. It names the macOS case: `._templates.yaml` is no longer read, so the
archive of a package made there is not rejected for it. Only the dot is taken from Terraform's
rule: a name that ends in `~`, or that begins and ends with `#`, has none of the three endings,
and is not read anyway. SUB-010's observable says that such an entry is rejected under it, and
SUB-005's that a name that begins with a dot is not read. The loader now skips such a name,
in a directory and among files handed over without one (`FileRole::of`); the rest it already
did. `check_schemas.py` reads no such name in `spec/core/`, `spec/schemas/` or the trees of
examples, nor where it searches them for citations. The vector is
[.notes.yaml](../../../spec/schemas/examples/valid/project/.notes.yaml) in
`examples/valid/project/`, which holds a map that is not a document and would be rejected if it
were read: the tool and the loader both accept the project beside it, and the loader's test
`a_file_whose_name_begins_with_a_dot_is_not_read_in_a_valid_example`, in `tests/examples.rs`,
checks that it would be rejected if it were read. Promises changed: MAN-011, SUB-010 and
SUB-005. The owner can strike it.

## 1-07. A JSON file whose value is not a map or a list of maps

**Class:** ambiguity. **Promises:** SCH-018, SCH-013, SUB-005, MAN-001.

SCH-018 said that a JSON file holds one document, or one array each of whose items is a
document. A file that holds a string, a number or `null`, and an array one of whose items is not
a map, are each read two ways. One rejects the file when it is read, under SCH-018. The other
reads the value as one document, or the item as a document, which the schemas then reject under
SUB-005, as they reject a YAML document that is not a map; SCH-013 asks that the JSON form and
the YAML form give the same diagnostics, which only this reading does. The two name different
promises for one file. And `check_schemas.py` read only names that end in `.yaml`, so no
example in `.yml` or `.json` was checked, and no vector for SCH-018, or for a JSON manifest,
could be written.

**What the loader does.** The second reading: an array is read as its items, anything else as
one document, and every document is judged by the schemas. A JSON manifest that holds an array
is rejected under MAN-001, as MAN-001 says.

**Fix, made by the coordinator.** The loader's reading, which keeps SCH-013. SCH-018 now says
that a file whose value is an array holds the documents that are its items, and a file whose
value is anything else holds one document, that value; its observable, that each document is
then judged by the schemas, so a value that is not a map, the file's or an item's, is rejected
under SUB-005, as a YAML document that is not a map is, and that a JSON manifest that holds an
array is rejected under MAN-001, as MAN-001 already says. The coordinator gave the vector to the
checks now, rather than leave it to the conformance vectors (A39), since the change to
`check_schemas.py` is small: it now reads files whose names end in `.yml` and `.json` as well as
`.yaml`, takes the three names of a manifest, and reads a JSON file with Python's `json`,
strictly, with two members of one key rejected and every number read by the same exact decimal
path as a YAML number, as SCH-018 has it. The vector is
[SUB-005-json-value-that-is-not-a-map.json](../../../spec/schemas/examples/invalid/SUB-005-json-value-that-is-not-a-map.json)
under `examples/invalid/`, a JSON file whose value is a string, which says why the file is
there, since JSON holds no comment. The tool and the loader both read it as one document and
reject it by a schema, under SUB-005. Promise changed: SCH-018. The owner can strike it.

## 1-08. `check_schemas.py` read numbers and two formats otherwise than the promises

**Class:** a defect of a tool. **Promises:** SCH-020, SCH-021, SCH-004; SCH-019, through which
the tool checks a format.

Three places of the same kind as 1-02: the promise is clear, and the tool that gives the
examples their verdicts disagreed with it, though no example showed it.

- SCH-021 rejects a number that is not an integer and whose magnitude is greater than the
  largest finite binary64 value, which "is not rounded into the range". The tool read such a
  number with `float()`, which rounds one a little above the largest value down to it, so it
  accepted the largest value with `.5` after it.
- SCH-020 reads a number with a fractional part as the binary64 value nearest to it, which for
  `1e-99999999999999999999` is 0. The tool asked Python's `decimal` whether it was whole, and
  `decimal` cannot hold so large an exponent, so the tool rejected it as "cannot be read as a
  number", which names no promise.
- SCH-004 has each format checked by the grammar the profile cites. The tool used Python's
  checkers. Its `uuid` takes what `uuid.UUID` parses: a final hyphen, a sign, an underscore, a
  digit outside ASCII and white space before it, none of which RFC 9562's grammar has. Its
  `duration` takes ISO 8601's, such as `P1Y3D`, `PT1H30S`, `PT0.5S`, `-P1D` and `P1,5D`, which
  the grammar of RFC 3339, appendix A, does not. Its `date-time` and `date` reject a leap
  second, `23:59:60`, and the year 0000, which RFC 3339's grammar allows.

**What the loader does.** It reads every number from its digits (`number.rs`). It checks `uuid`
and `duration` by its own code, written from the cited grammars, and `date-time` and `date` by
the `jsonschema` crate's, which takes the leap second and the year 0000.

**Fix, made.** The tool reads a number from its digits too, exactly whatever its exponent,
judges SCH-006 and SCH-021 on that decimal value, and then takes the binary64 value nearest to
it. It checks `uuid` and `duration` by regular expressions written from the two grammars, the
letters in either case (1-13). Its header says what it still reads otherwise, Python's
`date-time` and `date` among them, which no value it checks holds. The vectors:
[SCH-021-fraction-just-above-binary64.yaml](../../../spec/schemas/examples/invalid/SCH-021-fraction-just-above-binary64.yaml)
under `examples/invalid/`, which the tool and the loader both reject when it is read, and
[SCH-019-default-that-is-not-a-uuid.yaml](../../../spec/schemas/examples/invalid-by-rule/SCH-019-default-that-is-not-a-uuid.yaml)
and
[SCH-019-default-that-is-not-a-duration.yaml](../../../spec/schemas/examples/invalid-by-rule/SCH-019-default-that-is-not-a-duration.yaml)
under `examples/invalid-by-rule/`, whose `default` the tool's rule for SCH-019 now rejects. The
tool accepted all three before. The counts of [spec/README.md](../../../spec/README.md) and
[spec/schemas/README.md](../../../spec/schemas/README.md) include them. No promise changed.

## 1-09. `check_schemas.py` reads YAML 1.1

**Class:** a defect of a tool. **Promises:** SCH-012, SUB-005; the profile's row for YAML 1.2.2.

The tool reads YAML with PyYAML, which reads YAML 1.1, where the profile names YAML 1.2.2. The
two differ on characters and line breaks. PyYAML rejects a tab in a plain scalar. It takes NEL,
U+0085, and the line separator, U+2028, for line breaks, where YAML 1.2.2 reads them as content.
It rejects DEL, the C1 control characters, U+FFFE and U+FFFF even inside a quoted scalar, which
YAML 1.2.2 allows there for JSON's sake (`nb-json`). It rejects a byte order mark before a later
document, which YAML 1.2.2 allows, and a key of more than 1024 characters in a flow mapping,
which YAML 1.2.2 does not limit. And it reads a byte order mark inside a plain or a block
scalar, which YAML 1.2.2 does not allow there. No example holds any of these, so the verdicts
that the loader's fixture records agree with the tool's; but the tool is not a reader of YAML
1.2.2, and the loader does not follow it in these.

**What the loader does.** It follows YAML 1.2.2 in each. Where its parser, `saphyr-parser`, does
not check what YAML 1.2.2 says, the loader checks it (below, "Read, and found decided").

**Fix, made by the coordinator.** The tool's header lists these differences. Reading YAML with a
reader of YAML 1.2 would remove them, but adds a dependency to the checks that every contributor
runs, which the log left to the owner. The coordinator settled it so: the specification and the
loader follow YAML 1.2.2; `check_schemas.py` keeps PyYAML, so that the checks need no other
package; the tool is not the reference where the two differ; and no example may rely on one of
the differences that the tool's header lists. The header now says so, and lists one difference
more, which 1-12 found: PyYAML reads an escape of a surrogate, alone or as one of a pair written
as two escapes, as that code point, where the profile now has the file rejected. Python's `json`,
which the tool has used since 1-07, reads an escape of a surrogate that is not one of a pair in
the same way, and the header lists it too. No example holds any of them. No promise changed. The
owner can strike it.

## 1-10. Whether a number is whole, by its decimal or by its binary64 value

**Class:** ambiguity. **Promises:** SCH-005, SCH-006, SCH-020, SCH-021.

SCH-006 rejected "a number with no fractional part" outside the range of ±(2^53 − 1). SCH-020
reads "a number that is not an integer, which is one that has a fractional part" as the nearest
binary64 value, and "that value is what a schema checks". Neither said whether the fractional
part is that of the decimal written or that of the value read. The two differ where the decimal
has a fraction and its nearest binary64 value is whole. `9007199254740993.5` is read as
9007199254740994, which lies outside SCH-006's range, and which `type: integer` accepts, since
JSON Schema counts a number whose fractional part is zero as an integer. Judged on the decimal
written, it is not whole, SCH-006 does not apply, and the document is accepted with an integer
outside the range that SCH-006 keeps. Judged on the value read, it is whole and is rejected.
`0.99999999999999999999`, read as 1, and `1e-400`, read as 0, are of the same kind, inside the
range. SCH-021's observable settles one case only: `1e400` is under SCH-006, as the decimal
reading has it.

**What the loader did.** The decimal reading, as `check_schemas.py` had it: `9007199254740993.5`
was the binary64 value 9007199254740994, accepted, and valid for `type: integer`.

**Fix, made by the coordinator.** Option (a), since it keeps what SCH-006 is for, that every
integer a document holds is exact, and rejects only a number written with more digits than
binary64 holds. SCH-006 now holds to its range a number that has a fractional part and is read
under SCH-020 as a binary64 value that has none, and its observable says that no integer outside
the range is read, however the number is written; SCH-020's observable says that such a value
is an integer too: `0.99999999999999999999` is read as 1, `1e-400` as 0, and
`9007199254740993.5`, read as 9007199254740994, is rejected under SCH-006. A number beyond
binary64 is still rejected under SCH-021, judged on its decimal, and `1e400` under SCH-006. The
loader (`number.rs`) now holds a binary64 value that is whole as an integer, rejected outside
the range, so that a number it holds as binary64 never has a fractional part of zero; and
`check_schemas.py` rejects such a value in the same way. Unit tests in `number.rs` and in
`tests/reading.rs` cover the edges, in YAML and in JSON: `9007199254740993.5`, and
`9007199254740991.5`, which rounds to 2^53, rejected under SCH-006; `9007199254740990.5`,
`0.99999999999999999999` and `1e-400` read as integers within the range; and `1e400` still under
SCH-006. The vector is
[SCH-006-fraction-read-as-an-integer-out-of-range.yaml](../../../spec/schemas/examples/invalid/SCH-006-fraction-read-as-an-integer-out-of-range.yaml)
under `examples/invalid/`, a Type whose `maximum` is `9007199254740993.5`, which the tool and the
loader both reject when they read it, and which the tool accepted before. Promises changed:
SCH-006 and SCH-020. The owner can strike it.

## 1-11. A rejection for a limit of an engine names no promise

**Class:** gap. **Promises:** OPS-005, IFC-025, SUB-005; DL10 of the implementation-defined
list.

DL10 leaves sizes and counts to each engine, and the loader takes 128 levels of nesting, past
which it rejects a document, so that no input exhausts its stack or the validator's. An engine
that rejects a document reports invalid, with an error diagnostic whose `ruleId` is the ID of
the promise that decided it (OPS-005, IFC-025), and `diagnostic.v1` holds that ID to the
pattern `^[A-Z]+-[0-9]{3}$`. No promise said that an engine may reject a document beyond its
limits, so no ID fitted: `DL10` is an item of the implementation-defined list, not a promise,
and does not match the pattern.

**What the loader did.** It cited `DL10`, which its diagnostics can carry because they are not
yet in the shape of `diagnostic.v1`.

**Fix, made by the coordinator.** Option (a), since a document beyond a limit is not malformed,
and another engine with larger limits accepts it, which option (b), citing SUB-005, would hide.
A new promise, SUB-011, of the layer `language`, says that a document that exceeds a limit that
the engine states under DL10, such as how deeply its values may nest, is rejected, with an error
diagnostic that names the limit, and that a document within every limit that the engine states
is not rejected for its size. DL10 names it. The loader cites SUB-011 in place of `DL10`
(`rules.rs`, `yaml.rs`, `json.rs`), with a message that names the limit, 128 levels of nesting,
and `tests/rules.rs` now checks that every rule the loader cites is a promise that `spec/`
defines, written as a diagnostic's `ruleId` is. The vector is the loader's test
`values_nest_no_deeper_than_the_limit`, in `tests/reading.rs`, which reads a document nested 128
levels deep and rejects one nested 129 levels deep under SUB-011, in YAML and in JSON, and far
deeper ones without exhausting the stack; a conformance vector for it states the limit it
assumes. The counts of [spec/README.md](../../../spec/README.md) and of the
[independent builds](../../implementation/INDEPENDENT_BUILDS.md) include the new promise.
Promises changed: SUB-011, added, and DL10. The owner can strike it.

**After the review.** The review completed this settlement in 1-16: every engine accepts nesting at least 64 levels deep; files, submissions and archives fall under SUB-011 too; exhaustion of storage stays with DL10.

## 1-12. I-JSON's rules on strings

**Class:** ambiguity. **Promises:** SCH-013, SCH-012; the profile's row for I-JSON.

The profile names I-JSON, RFC 7493, as the value model, and says that "where a standard defines
a meaning, that meaning applies unchanged", but its row named two of I-JSON's rules only: no
duplicate keys, and whole numbers within the range it recommends. I-JSON, section 2.1, also says
that a name or a string must not hold a code point that is a surrogate or a noncharacter, such
as U+FDD0 or U+FFFF. Read with the general sentence, the profile adopted that rule too; read by
its row, only the two named applied. YAML adds a case of its own: an escape `\u` writes one code
point, and YAML 1.2.2 does not say whether two escapes may write a surrogate pair, as JSON's
do.

**What the loader does.** It applies the two rules that the row names. A noncharacter in a
string is read, in JSON and in a quoted YAML scalar alike. A surrogate that is not paired is
read in neither: `serde_json` rejects the escape `\ud800`, and `saphyr-parser` rejects it in
YAML, each as not JSON or not YAML (SUB-005). `saphyr-parser` rejects a pair of escaped
surrogates too, which JSON reads as one character; YAML writes that character as `\U0001F600`.

**Fix, made by the coordinator.** Option (b), since a noncharacter does no harm in a
configuration, and the readers of both syntaxes accept it. The profile's row for I-JSON now says
that two of its rules apply and no other, no duplicate keys and whole numbers within the range;
that a noncharacter may stand in a string wherever YAML 1.2.2 or JSON lets it stand; that a
surrogate code point that is not one of a pair is not text, so a file whose string holds one is
not YAML or JSON and is rejected under SUB-005; and that in YAML an escape writes one code
point, so a surrogate pair written as two `\u` escapes is rejected, and the character is written
with one escape, `\U0001F600`. That is the loader's reading. `check_schemas.py` reads otherwise:
PyYAML reads an escaped surrogate, alone or in a pair, and Python's `json` an escaped surrogate
that is not paired, as that code point, and its header lists both (1-09); no example holds one.
The vector is the loader's test `of_i_json_only_two_rules_apply_to_strings`, in
`tests/reading.rs`: a noncharacter, written out or escaped, read in either syntax; an escaped
surrogate that is not paired, in a value or in a key, rejected under SUB-005 in either syntax; a
pair written as two escapes rejected in YAML and read as one character in JSON; and a surrogate
encoded in the file rejected as not UTF-8. No promise changed; the profile did. The owner can
strike it.

## 1-13. The case of the letters of a duration

**Class:** ambiguity. **Promises:** SCH-004; the profile's row for `duration`.

SCH-004 has each format checked by "the grammar cited", for `duration` the ABNF of RFC 3339,
appendix A. In ABNF a quoted letter matches either case (RFC 5234, section 2.3), so by that
grammar `p1d` and `PT1h` are durations; RFC 3339 says as much of the `T` and the `Z` of a
date-time (section 5.6), and the loader reads the grammar of `uuid` in the same way. But
ISO 8601, from which the grammar comes, writes the letters as capitals, and the libraries an
engine is likely to use take capitals only: the `jsonschema` crate and Python's `isoduration`
both reject `p1d`. An engine that checks the cited grammar and one that uses its library give
different verdicts.

**What the loader does.** It follows the cited grammar: the letters match either case, as for
`uuid`. It checks `duration` by its own code, in place of the crate's, and `check_schemas.py`
does the same (1-08).

**Fix, made by the coordinator.** Option (a), since the profile invents nothing, and the grammar
it cites already says so. The profile's section on the formats that are checked now says that a
letter that one of the grammars it cites writes in quotes matches either case, as ABNF has it,
so `p1d` and `PT1h` are durations, and a UUID may be written in capitals, in small letters or in
both. The loader and `check_schemas.py` already read so, and the checks of `date-time` of both
take a small `t` and `z`. The vector is the loader's tests in `tests/formats.rs`:
`a_duration_matches_the_grammar_of_rfc_3339_appendix_a`, with `p1d` and `PT1h`, and
`a_uuid_matches_the_grammar_of_rfc_9562`, with letters of either case, each checked against the
validator and against the grammar. No promise changed; the profile did. The owner can strike it.

## After the review of 2026-11-09

### 1-14. A rejection by a schema cites SUB-005 everywhere

**Class:** contradiction. **Promises:** SUB-005, PKG-023, CON-020, POOL-009, POOL-012,
MAN-012, MAN-007, CTR-008, OBJ-015, OBJ-019, TPL-001; also CTR-002, CTR-005, SCH-002,
SCH-003, SCH-015, IFC-041, IFC-043 and IFC-044.

The first settlement of 1-01 gave schema rejections SUB-005, while other promises still gave
the same rejection a different rule. Those attributions now name SUB-005 and the rejecting
Schema contract. The search beyond the order's initial list found the undefined keys of
CTR-002 and CTR-005, the unknown keyword and format of SCH-002 and SCH-003, the malformed
`$ref` of SCH-015, and the header and URL restrictions of IFC-041, IFC-043 and IFC-044. It
also separated MAN-007's duplicate export name, which `manifest.v1` rejects, from an export
that names no document, which needs MAN-007's semantic check. POOL-009 keeps PLC-005 for an
input with a placement or runtime destination that states a default. A reference that resolves
to something other than a Schema contract is not decided by a schema, and SCH-015 said it was
rejected under SCH-002, while `check_schemas.py` and the example
`SCH-015-ref-to-a-name-that-is-not-a-schema.yaml` name SCH-015. The coordinator settled it on
SCH-015, the promise that says what a `$ref` resolves to, so the text now agrees with the tool
and the example. Every rejection that a schema does not decide keeps its rule.

SUB-005 now also covers a schema rejection of a package's document or manifest during
installation, with the request reporting invalid under PKG-023, as MAN-011 says. Its
observable requires the keyword's `keywordLocation`, and `absoluteKeywordLocation` where
known, in the SARIF result's `properties`. `diagnostic.v1` already allows SARIF's other
members, including this property bag; its description now states that use. The loader's
`Diagnostic` carries both locations. The bundle has no absolute base URI, so its absolute
location is unknown; one is preserved when the validator supplies it. The core's thesis "the
core cites a promise for every decision" is met for the rules a schema encodes through SUB-005
and the keyword's location.

**Vectors:** `a_schema_rejection_carries_the_keyword_location` in `tests/project.rs` checks
both a document and a manifest, with the instance pointer kept apart from the keyword's path;
`a_known_absolute_keyword_location_is_kept` in `src/schemas.rs` checks a known absolute URI.
`every_invalid_example_is_rejected_by_the_step_its_name_gives` still requires SUB-005 for
every schema rejection. The custom-keyword errors of jsonschema 0.61 lose the reference
tracker; the loader reads the evaluation tree to retain the `$ref` traversals required by
JSON Schema 2020-12. **The owner can strike it.**

### 1-15. A byte order mark in JSON is accepted and ignored

**Class:** contradiction. **Promises:** SUB-005, SCH-013; the profile's Encoding and Value
model rows.

The Encoding row now allows one leading byte order mark in either syntax and ignores it,
which RFC 8259, section 8.1, permits a JSON parser to do. UTF-8 is the profile's own rule,
with I-JSON as its precedent, so the Value model row still adopts only its two stated rules.
Both the loader and `check_schemas.py` skip that one mark before reading JSON. A second mark
is not skipped; a mark inside a string remains content. The loader keeps source positions
relative to the original text, including the leading mark.

**Vectors:** [bom.json](../../../spec/schemas/examples/valid/project/bom.json), read by both
tools; `text_that_is_not_yaml_or_json_is_rejected` in `tests/reading.rs`, which now accepts one
mark and rejects two and checks a diagnostic after it; and `check_profile_regressions` in
`tools/check_schemas.py`, which checks one mark, two, an array and content inside a string.
**The owner can strike it.**

### 1-16. SUB-011 has a floor and covers files, submissions and archives

**Class:** gap. **Promises:** SUB-011, PKG-023, DL10.

Every engine accepts values nested at least 64 levels deep. This loader keeps its stated
limit of 128. SUB-011 also covers file sizes, counts of documents and files in a submission,
and package archives. At installation the request reports invalid under PKG-023 with the
SUB-011 diagnostic. The rule against unstated limits now applies to size, count and nesting
limits alone, leaving exhaustion of storage to DL10. DL10 lists the same scope and floor.

**Vector:** `values_nest_no_deeper_than_the_limit` in `tests/reading.rs` accepts 63, 64 and
65 levels in YAML and JSON and keeps the existing checks at 128 and beyond. All specification
examples remain within the floor. File-count and archive limits are not imposed by this
loader, and installation comes with a later increment; no runtime verdict on an archive is
claimed here. **The owner can strike it.**

### 1-17. A pattern uses only the interoperable tokens

**Class:** needless complexity. **Promises:** SCH-016, INS-014.

SCH-016 now names and quotes the token notation of JSON Schema 2020-12, core, section 6.4,
plus a backslash before pattern punctuation to give its literal meaning. It keeps ECMA-262 in
Unicode mode. A malformed pattern, or one outside those tokens, makes its document rejected
under SCH-016 when read, including at package installation, as 1-03 settled.

The schemas' eight patterns that used other tokens have been rewritten: `address.v1`,
`qualified-reference.v1`, `destination.v1`, the three destination schemas in `templates.yaml`,
and the transport URL of `server-package.v1` and the URL of `server-remote.v1`. Lookaheads now
use `allOf` and `not` with another pattern; the qualified reference's export length uses a
second positive pattern. The two URL classes write out exactly ECMA-262's white space and
line terminators, with YAML escapes decoded to individual Unicode characters. The search of
the rest of `spec/` found no other unsupported schema patterns, apart from the deliberately
invalid examples. The loader replaces its checks for later editions with a tokenizer and
keeps `regress` for matching. The tool's `ecma-pattern` rule tokenizes the same list, then
compiles it; `ecma()` keeps the end anchor and character classes at their ECMA-262 meanings.

**Vectors:** the existing
[SCH-016-pattern-that-is-not-ecma-262.yaml](../../../spec/schemas/examples/invalid-by-rule/SCH-016-pattern-that-is-not-ecma-262.yaml)
and the new
[SCH-016-pattern-outside-the-interoperable-tokens.yaml](../../../spec/schemas/examples/invalid-by-rule/SCH-016-pattern-outside-the-interoperable-tokens.yaml)
are rejected by both tools. `only_interoperable_tokens_enter_a_pattern` and
`a_document_whose_schema_holds_such_a_pattern_is_rejected` in `tests/formats.rs` check allowed
and forbidden tokens, and keys of `patternProperties`; `check_profile_regressions` checks the
tool's tokenizer and class translation too. `every_rewritten_pattern_keeps_its_verdict` in
`tests/pattern_rewrites.rs` compares each historical expression with its rewrite, both matched
by `regress`, on every string value read from the examples and over 10,000 strings per pattern
including generated strings and the boundaries of host labels, export lengths, destinations,
escapes, white space and line terminators. A file that is not readable has no document value to
match. The test uses `regress`'s PikeVM backend because a long host label makes the historical
qualified-reference pattern take excessive time by backtracking. No verdict changed in these
comparisons. This settles 1-04. **Settled by ruling 48, point 3.**

### 1-18. What a pin holds

**Class:** gap. **Promise:** MAN-010.

MAN-010's observable now says that the hash of a package file does not pin what its registry
resolves for that package's dependencies, and that a `command` pins nothing. No new field or
runtime behavior is introduced.

**Vector:** the existing
[files-package manifest](../../../spec/schemas/examples/valid/files-package/conduit.yaml)
keeps the `listing` Package's illustrative `fileSha256` beside `files-server`'s `command`,
and its comment states the distinction. The loader and tool still accept the manifest. The
loader neither fetches a registry's dependency graph nor executes a command, so this example
illustrates the stated limit and claims no execution result. **The owner can strike it.**

## Read, and found decided

These were checked and need no change: the text decides them, though a builder could miss it.

- **The grammar of `uuid`.** SCH-004 has the grammar that the profile cites checked, the ABNF of
  RFC 9562, section 4. It fixes neither the version nor the variant, so the Nil UUID, the Max
  UUID and a UUID whose variant RFC 9562 reserves are each valid, and a braced form, a
  `urn:uuid:` form and one without hyphens, which some libraries parse, are not.
- **A number written large.** Whether `1e400` has a fractional part is read from the decimal it
  writes: SCH-021's observable puts it under SCH-006, which only that reading gives. Where the
  decimal has a fraction and the value read is whole, SCH-006, as 1-10 settled it, holds that
  value to its range.
- **The order of documents, and every fault.** SUB-001 says that the same documents in any
  order, within a file or across files, give the same diagnostics. A reader that stops at the
  first fault of a file reports one fault in one order and another in the other, so the loader
  reads on past a fault in a value, a duplicate key, an anchor, a number out of range or a
  value nested too deep, and reports every one, each with its document and its place as a JSON
  Pointer, which are what the order changes. A file that is not YAML or JSON is rejected for
  that alone, wherever it stops. Its faults are named alike in the YAML and the JSON form of one
  document, as SCH-013 asks.
- **The characters of a YAML stream.** YAML 1.2.2, section 5.1, allows the printable characters
  only: a C0 control character other than tab, line feed and carriage return nowhere, and DEL,
  a C1 control character other than NEL, U+FFFE and U+FFFF only inside a quoted scalar, which
  takes any character but a C0 control for JSON's sake. `saphyr-parser` does not check this, so
  the loader does, and rejects such a text as not YAML (SUB-005). The JSON reader rejects a C0
  control in a string, as RFC 8259 does, so one string is read alike in either syntax.
- **A `%YAML` directive.** YAML 1.2.2, section 6.8.1, has a reader take a document marked 1.1 as
  1.2 and reject one of another major version, and section 6.8 has it ignore a reserved
  directive, such as `%YAMLX`. `saphyr-parser` does not check the version, so the loader does,
  for a directive named `YAML` exactly; `%YAML 01.2` is version 1.2.
- **A document marker inside a block scalar.** A line that begins with `---` or `...` marks a
  document and is never content (YAML 1.2.2, `c-forbidden`). `saphyr-parser` reads a `---` line
  into a block scalar at the root as content. The loader rejects such a text as not YAML
  (SUB-005) rather than read it otherwise than YAML does. YAML 1.2.2 reads it as two documents,
  each of which the schemas reject, or for a manifest as two documents, which MAN-001 rejects:
  the verdict is the same, and the rule named differs. The fault is to be reported to the
  parser's maintainers.
- **The edition of ECMA-262.** The profile names the edition that JSON Schema 2020-12 cites, the
  eleventh, of 2020. Since the review it first checks the interoperable token list (1-17),
  which excludes the later syntax too.
- **A merge key** is a plain `<<` used as a key: YAML 1.1's merge type resolves a plain scalar
  alone, so a quoted `"<<"` is a string key, as `check_schemas.py` also has it.
- **An empty file and an empty document.** A YAML file with no document holds none, and is
  accepted; a `---` with nothing after it is an empty document, which the schemas reject.

## What increment 1 did not do

It does not apply the rules of `check_schemas.py`'s steps 5 and 6, which need several documents
or the core package's contracts; they are ported with the core, apart. Of those rules, it
applies only SCH-016's to the patterns that a document's schemas write (1-03). It does not
resolve a relative `fetch` or program against the directory (MAN-006, MAN-010, SUB-010), and it
registers nothing. Its diagnostics are not yet in the shape of `diagnostic.v1`: they have no
`level`, and a manifest is a document like any other. It leaves `patternProperties`, and the
`additionalProperties` beside it, to the `jsonschema` crate's own engine, which is not
ECMA-262's: no schema of the specification uses `patternProperties`, so no verdict of increment
1 depends on it, and the core matches both itself when it validates the schemas that authors
write (1-04). Each comes with the core, in increment 2.
