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

Thirteen findings. Three are defects of a tool: two are fixed here, with vectors, and the third
is stated in the tool. Ten are for the owner, because each fix chooses between readings that the
text leaves open, or narrows what is accepted, or cannot yet be shown by a vector that the checks
run; for each, the loader's reading is stated. Three verifiers read the increment before it was
merged, and findings 1-08 to 1-13 come from their reports, as do the corrections to the
readings of 1-03, 1-04 and 1-06.

| # | Finding | Class | Promises | Settled |
| --- | --- | --- | --- | --- |
| 1-01 | A rejection by a schema names no one promise | Gap | OPS-005, IFC-025, OBJ-023, SUB-005 | For the owner |
| 1-02 | `check_schemas.py` matched `$` and `\s` otherwise than ECMA-262 | Defect of a tool | SCH-016, OBJ-001 | Fixed, with a vector |
| 1-03 | When a schema whose pattern is not ECMA-262 is rejected | Gap | SCH-016, SCH-002, SCH-003 | For the owner |
| 1-04 | ECMA-262 patterns are needlessly hard outside JavaScript | Needless complexity | SCH-016, INS-014 | For the owner |
| 1-05 | The encoding of a file, and a file that is not YAML or JSON | Gap | SUB-005, SCH-013 | For the owner |
| 1-06 | Which names in a directory are read | Gap | MAN-011, MAN-001, SUB-010, PKG-022 | For the owner |
| 1-07 | A JSON file whose value is not a map or a list of maps | Ambiguity | SCH-018, SCH-013, SUB-005 | For the owner |
| 1-08 | `check_schemas.py` read numbers and two formats otherwise than the promises | Defect of a tool | SCH-020, SCH-021, SCH-004, SCH-019 | Fixed, with vectors |
| 1-09 | `check_schemas.py` reads YAML 1.1 | Defect of a tool | SCH-012, SUB-005 | Stated in the tool; its parser for the owner |
| 1-10 | Whether a number is whole, by its decimal or by its binary64 value | Ambiguity | SCH-005, SCH-006, SCH-020, SCH-021 | For the owner |
| 1-11 | A rejection for a limit of an engine names no promise | Gap | OPS-005, IFC-025, SUB-005, DL10 | For the owner |
| 1-12 | I-JSON's rules on strings | Ambiguity | SCH-013, SCH-012 | For the owner |
| 1-13 | The case of the letters of a duration | Ambiguity | SCH-004 | For the owner |

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

**Fix, for the owner.** Either (a) say in SUB-005's observable that a document or a manifest
that a schema rejects is rejected under SUB-005, its diagnostic's location naming the place and
its message the Schema contract, which is the loader's reading; or (b) let each Schema contract
carry the promises it encodes where an engine can read them, keyword by keyword where one
contract encodes several, so that a diagnostic names the specific promise. Recommended, and not
ruled: (a) for this version, since it changes no schema and every engine can follow it.

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

**Fix, for the owner.** Add to SCH-016 that the document is rejected when its package is
installed or when it is submitted, as SCH-002 says. The fix is plain, but no vector that the
checks run can show it yet. No rule of `check_schemas.py` checks a pattern that an author
writes, and `schema.v1` cannot, since it checks only that a pattern is a string. A vector would
be a set under `examples/invalid-by-rule/` whose pattern both engines reject, such as `[z-a]` or
`a(`, with a new rule of the tool's step 6; that rule is a change to the tool, which waits until
the owner settles when such a schema is rejected. Otherwise the vector belongs with the
conformance vectors (A39), which the core runs.

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

**Fix, for the owner.** (a) Keep ECMA-262, and have the core match `patternProperties` itself,
or give the crate a way to take another engine, upstream. (b) Narrow the profile's patterns to
I-Regexp, RFC 9485, the subset of regular expressions meant to be matched alike by every engine;
it has no lookaround, so the specification's own patterns would have to be rewritten without it,
for instance with a second `pattern` under `not`, where that can say what the lookahead says.
(c) Drop `patternProperties` from the profile. Recommended, and not ruled: (a) for this
version. (b) is worth weighing before the independent builds, which SCH-016 binds as it binds
this one.

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
is its one rejection. `check_schemas.py` reads a file in the default encoding of the machine,
UTF-8 on those it has run on, and stops with an exception rather than a diagnostic on bytes that
are not; nor does it read YAML as YAML 1.2.2 does (1-09).

**Fix, for the owner.** State in the profile that a file is UTF-8, as I-JSON requires, and
whether a byte order mark is allowed in each syntax; and in SUB-005's observable, that a file
that is not UTF-8, or not YAML or JSON as its ending says, holds no document and is rejected
under SUB-005. It narrows what a YAML reader would accept, and no ruling settled it. A vector
needs `check_schemas.py` to report bytes that are not UTF-8 rather than stop.

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

**What the loader does.** It reads as the text is written: every name with one of the three
endings, compared exactly, so `Conduit.yaml` is a file of documents and `notes.YAML` is not
read; a name that begins with a dot is read; a link is followed as the file system follows it,
and one that leads to a directory is not entered. An entry with one of the endings that cannot
be read, a link that leads nowhere among them, or that is not a regular file, is rejected under
SUB-010, "cannot be read", and never skipped, so that a link named `conduit.yaml` that leads
nowhere cannot leave a project read as one with no manifest. The rest of the directory is still
read, and every fault in it reported.

**Fix, for the owner.** (a) Keep MAN-011 as written, and say so of the four cases. (b) Skip a
name that begins with a dot, as Terraform does, say that the names are compared exactly, and
say that an entry MAN-011 names that cannot be read is rejected. Recommended, and not ruled:
(b), since it keeps the files that tools leave behind out of a project and changes no project
that an author wrote.

## 1-07. A JSON file whose value is not a map or a list of maps

**Class:** ambiguity. **Promises:** SCH-018, SCH-013, SUB-005, MAN-001.

SCH-018 says that a JSON file holds one document, or one array each of whose items is a
document. A file that holds a string, a number or `null`, and an array one of whose items is not
a map, are each read two ways. One rejects the file when it is read, under SCH-018. The other
reads the value as one document, or the item as a document, which the schemas then reject under
SUB-005, as they reject a YAML document that is not a map; SCH-013 asks that the JSON form and
the YAML form give the same diagnostics, which only this reading does. The two name different
promises for one file. And `check_schemas.py` reads only names that end in `.yaml`, so no
example in `.yml` or `.json` is checked, and no vector for SCH-018, or for a JSON manifest, can
be written yet.

**What the loader does.** The second reading: an array is read as its items, anything else as
one document, and every document is judged by the schemas. A JSON manifest that holds an array
is rejected under MAN-001, as MAN-001 says.

**Fix, for the owner.** Say which in SCH-018's observable; the loader's reading keeps SCH-013. A
vector needs `check_schemas.py` to read `.yml` and `.json` files.

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

**Fix, stated in the tool.** The tool's header lists these differences. Reading YAML with a
reader of YAML 1.2 would remove them, but adds a dependency to the checks that every
contributor runs, which is the owner's to choose.

## 1-10. Whether a number is whole, by its decimal or by its binary64 value

**Class:** ambiguity. **Promises:** SCH-005, SCH-006, SCH-020, SCH-021.

SCH-006 rejects "a number with no fractional part" outside the range of ±(2^53 − 1). SCH-020
reads "a number that is not an integer, which is one that has a fractional part" as the nearest
binary64 value, and "that value is what a schema checks". Neither says whether the fractional
part is that of the decimal written or that of the value read. The two differ where the decimal
has a fraction and its nearest binary64 value is whole. `9007199254740993.5` is read as
9007199254740994, which lies outside SCH-006's range, and which `type: integer` accepts, since
JSON Schema counts a number whose fractional part is zero as an integer. Judged on the decimal
written, it is not whole, SCH-006 does not apply, and the document is accepted with an integer
outside the range that SCH-006 keeps. Judged on the value read, it is whole and is rejected.
`0.99999999999999999999`, read as 1, and `1e-400`, read as 0, are of the same kind, inside the
range. SCH-021's observable settles one case only: `1e400` is under SCH-006, as the decimal
reading has it.

**What the loader does.** The decimal reading, as `check_schemas.py` has it: `9007199254740993.5`
is the binary64 value 9007199254740994, accepted, and valid for `type: integer`.

**Fix, for the owner.** (a) Say in SCH-020 that a number whose binary64 value is whole is held
to SCH-006's range too, so that no integer outside it is read. (b) Say that whether a number is
whole, and so its range, is judged on the decimal written alone, and that a value read from a
fraction may be whole. Recommended, and not ruled: (a), since it keeps what SCH-006 is for,
that every integer a document holds is exact, and rejects only a number written with more
digits than binary64 holds.

## 1-11. A rejection for a limit of an engine names no promise

**Class:** gap. **Promises:** OPS-005, IFC-025, SUB-005; DL10 of the implementation-defined
list.

DL10 leaves sizes and counts to each engine, and the loader takes 128 levels of nesting, past
which it rejects a document, so that no input exhausts its stack or the validator's. An engine
that rejects a document reports invalid, with an error diagnostic whose `ruleId` is the ID of
the promise that decided it (OPS-005, IFC-025), and `diagnostic.v1` holds that ID to the
pattern `^[A-Z]+-[0-9]{3}$`. No promise says that an engine may reject a document beyond its
limits, so no ID fits: `DL10` is an item of the implementation-defined list, not a promise, and
does not match the pattern.

**What the loader does.** It cites `DL10`, which its diagnostics can carry because they are not
yet in the shape of `diagnostic.v1`.

**Fix, for the owner.** (a) Add a promise that an engine rejects a document beyond the limits
it states under DL10, with a diagnostic that names the limit, and cite it. (b) Say that such a
rejection cites SUB-005. Recommended, and not ruled: (a), since a document beyond a limit is not
malformed, and another engine with larger limits accepts it.

## 1-12. I-JSON's rules on strings

**Class:** ambiguity. **Promises:** SCH-013, SCH-012; the profile's row for I-JSON.

The profile names I-JSON, RFC 7493, as the value model, and says that "where a standard defines
a meaning, that meaning applies unchanged", but its row names two of I-JSON's rules only: no
duplicate keys, and whole numbers within the range it recommends. I-JSON, section 2.1, also says
that a name or a string must not hold a code point that is a surrogate or a noncharacter, such
as U+FDD0 or U+FFFF. Read with the general sentence, the profile adopts that rule too; read by
its row, only the two named apply. YAML adds a case of its own: an escape `\u` writes one code
point, and YAML 1.2.2 does not say whether two escapes may write a surrogate pair, as JSON's
do.

**What the loader does.** It applies the two rules that the row names. A noncharacter in a
string is read, in JSON and in a quoted YAML scalar alike. A surrogate that is not paired is
read in neither: `serde_json` rejects the escape `\ud800`, and `saphyr-parser` rejects it in
YAML, each as not JSON or not YAML (SUB-005). `saphyr-parser` rejects a pair of escaped
surrogates too, which JSON reads as one character; YAML writes that character as `\U0001F600`.

**Fix, for the owner.** (a) Adopt I-JSON's rule on strings: reject a noncharacter, under
SCH-012 or a promise of its own. (b) Say that only the two rules the row names apply, that a
surrogate that is not paired is not text in either syntax, and whether a YAML escape may write
a surrogate pair. Recommended, and not ruled: (b), since a noncharacter does no harm in a
configuration, and the readers of both syntaxes accept it.

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

**Fix, for the owner.** (a) Say in the profile that the letters of the grammars it cites match
either case, as ABNF has it, which is the loader's reading. (b) Narrow `duration` to capitals,
as ISO 8601 writes them and as the libraries check them. Recommended, and not ruled: (a), since
the profile invents nothing, and the grammar it cites already says so.

## Read, and found decided

These were checked and need no change: the text decides them, though a builder could miss it.

- **The grammar of `uuid`.** SCH-004 has the grammar that the profile cites checked, the ABNF of
  RFC 9562, section 4. It fixes neither the version nor the variant, so the Nil UUID, the Max
  UUID and a UUID whose variant RFC 9562 reserves are each valid, and a braced form, a
  `urn:uuid:` form and one without hyphens, which some libraries parse, are not.
- **A number written large.** Whether `1e400` has a fractional part is read from the decimal it
  writes: SCH-021's observable puts it under SCH-006, which only that reading gives. Where the
  decimal has a fraction and the value read is whole, the text does not decide (1-10).
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
  eleventh, of 2020. The loader rejects the syntax of later editions that its engine accepts
  (1-04).
- **A merge key** is a plain `<<` used as a key: YAML 1.1's merge type resolves a plain scalar
  alone, so a quoted `"<<"` is a string key, as `check_schemas.py` also has it.
- **An empty file and an empty document.** A YAML file with no document holds none, and is
  accepted; a `---` with nothing after it is an empty document, which the schemas reject.

## What increment 1 did not do

It does not apply the rules of `check_schemas.py`'s steps 5 and 6, which need several documents
or the core package's contracts; they are ported with the core, apart. It does not resolve a
relative `fetch` or program against the directory (MAN-006, MAN-010, SUB-010), and it registers
nothing. Its diagnostics are not yet in the shape of `diagnostic.v1`: they have no `level`, and
a manifest is a document like any other. Each comes with the core, in increment 2.
