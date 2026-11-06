# Schema and syntax profile

Draft. The owner accepted this profile as it stood at plan-audit rulings 23, 24, 32, 33 and 37
of 2026-11-05. What decision DA3 of the [docs audit](../docs/proposals/docs-audit/GAPS.md), the
[decisions of the second round of the re-audit](../docs/proposals/reaudit/round-2/DECISIONS.md)
and the [decisions of its fourth round](../docs/proposals/reaudit/round-4/DECISIONS.md) changed
since rests on the coordinator's logged decisions under ruling 37, which the owner has not
ruled on. The fourth round changed what this page says of when a `default` is applied.

Graphs and schemas are not what distinguishes Conduit. This part of the specification invents
nothing: it selects existing standards and narrows them. Where a standard defines a meaning,
that meaning applies unchanged. The promises that bind are in
[promises/schemas.yaml](promises/schemas.yaml) and, where a line below cites one, in the promise
it names; this page lists what they refer to. The profile is itself written as a schema, the
Schema contract `schema.v1` in [schemas/schema.yaml](schemas/schema.yaml), and
`python3 tools/check_schemas.py` checks that the two agree.

## Standards relied on

| Subject | Standard | How it is used |
| --- | --- | --- |
| Document syntax | YAML 1.2.2, core schema | The authoring syntax, narrowed as listed below. |
| Equivalent syntax | JSON, RFC 8259 | A JSON document means the same as the equivalent YAML document (SCH-013). A JSON file holds one document or one array of documents (SCH-018). |
| Value model | I-JSON, RFC 7493 | No duplicate keys, and whole numbers within the range it recommends. |
| Numbers that are not integers | IEEE 754, binary64 | A number that has a fractional part is read as a binary64 value, which is what a schema checks and what equality compares (SCH-020). One whose magnitude is greater than the largest finite binary64 value is rejected (SCH-021). |
| Schemas | JSON Schema 2020-12, core and validation, with the format-assertion vocabulary | The meaning of every keyword of the profile, the definition of integer, and the definition of equality. A format the profile lists is asserted, not only annotated (SCH-001). |
| `pattern` | ECMA-262 regular expressions, in the edition that JSON Schema 2020-12 cites | The dialect of a `pattern`, as JSON Schema recommends. A pattern is matched in Unicode mode, which is the `u` flag (SCH-016). |
| Pointers into values | JSON Pointer, RFC 6901 | The syntax of a pointer in a value reference, of an input's destination and of a value of a control's `changes`, in the string syntax of section 3 and not the URI fragment form of section 6 (OBJ-016). The `-` token is not accepted in a destination. |
| `date-time`, `date` | RFC 3339, section 5.6 | Format grammar. |
| `duration` | RFC 3339, appendix A | Format grammar. |
| `uri` | RFC 3986 | Format grammar. |
| `fetch`, and an implementation's `url` | RFC 3986 | `fetch` in an item of a manifest's `packages` is a URI reference. A relative one is resolved against the directory the manifest was read from, and is rejected where the manifest was handed over without a directory (MAN-006). An implementation's `url` is a URI (MAN-010). |
| `uuid` | RFC 9562 | Format grammar. JSON Schema 2020-12 cites RFC 4122 for this format; the grammar cited here is the one that is checked (SCH-004). |
| Package versions | Semantic Versioning 2.0.0 | The grammar of a version (PKG-001). Two versions are the same when neither precedes the other, so build metadata is set aside; the digest decides whether two archives are the same package (PKG-003, PKG-021). |
| Digests | SHA-256, FIPS 180-4 | The digest of a package's archive, written as `sha256:` followed by the hash in lowercase hexadecimal (PKG-004). The digest of an unpublished package is written in the same way (SUB-008). |
| Canonical form of a value | JSON Canonicalization Scheme, RFC 8785 | The bytes that are hashed for the digest of an unpublished package: the canonical form of one map that holds the project's contract documents and its manifest (SUB-008). It writes equal values alike, so `1` and `1.0` give the same bytes. The contract documents are ordered by kind and then by name, each compared by the bytes of its UTF-8 string. |
| Package archives | tar, as POSIX (IEEE Std 1003.1) defines it, compressed with gzip, RFC 1952 | A package's archive is a gzip-compressed tar file, and the digest is taken over the compressed file as distributed (PKG-022). Which of the tar formats and extensions an engine reads is item DL7 of the [implementation-defined list](implementation-defined.yaml). |
| Host part of an address | Host names, RFC 1123, section 2.1 | The host part of a package's address: a DNS name written in lowercase, with labels of letters, digits and hyphens, each at most 63 characters, at least one dot, no port and no trailing dot. A host whose last label is all digits is not a host name (NAM-001). |
| Internationalized host names | IDNA, RFC 5890 | An internationalized name is written in its ASCII form, with `xn--` labels (NAM-001). |
| The implementation interface | Model Context Protocol, revision 2026-07-28 | The wire between an engine, its implementations and its consumers, and the baseline revision of this version of the specification (IFC-002). The protocol is adopted as it is: an engine and an implementation keep every requirement of the revision in use, and this specification repeats none of them (IFC-001). The protocol's tasks extension is outside the baseline revision, and may be used too (IFC-016). |

Two rules follow an existing practice rather than a standard. Applying `default` values to a
value that is supplied, before it is validated, and rejecting a `default` placed inside `allOf`,
`anyOf` or `oneOf`, both follow the way Kubernetes treats defaults in structural schemas. The
first is one rule for every schema that governs a supplied value: the configuration of an
instance, and the argument of an input of a template, of an operation or of a requirement. The
filled value is what is validated, compared, stored and sent (SCH-009). So does a `default` on a
property of a map inside `items`: it is accepted and applies to each item that is present,
while a `default` on `items` itself is not accepted (SCH-008, SCH-009). A `default` that is a
map is checked against its schema after the defaults of its own properties are filled in
(SCH-010, SCH-019). Treating a step's use of another step's output as a dependency follows
Terraform, and writing the relation as "depends on" follows Docker Compose and Terraform.

## YAML features not accepted

Duplicate keys, anchors and aliases, merge keys, and tags. Keys are strings. A number that has
no JSON form is not accepted either: an infinity or a not-a-number, such as `.inf` or `.nan`,
and an integer written in hexadecimal or octal, such as `0x1F` or `0o17` (SCH-017). Nor is a
number that is not an integer and is outside the range of IEEE 754 binary64 (SCH-021).

## Schema keywords in the profile

`type`, `enum`, `const`, `properties`, `required`, `additionalProperties`, `items`, `minItems`,
`maxItems`, `uniqueItems`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`,
`minLength`, `maxLength`, `pattern`, `format`, `default`, `title`, `description`, `examples`,
`$ref`, `$defs`, `allOf`, `anyOf`, `oneOf`.

Every other keyword of JSON Schema 2020-12 is outside the profile, including `if`, `then`,
`else`, `not`, `multipleOf`, `patternProperties`, `dependentSchemas`, `unevaluatedProperties`,
`unevaluatedItems`, `$dynamicRef`, `$id`, `$anchor` and `$schema`.

The value of `$ref` is one of three things: `#` followed by a JSON Pointer into the same
schema, such as `#/$defs/name`; a qualified reference to a Schema contract that a package
exports; or `#` followed by a name, for a Schema contract of the same project or package
(SCH-015, NAM-022). The three cannot be confused: a JSON Pointer is empty or starts with `/`,
and a name starts with a letter. A Schema contract is the fifth kind of contract, and its
`spec` is a schema under this profile (CTR-008).

A schema is a map or a boolean, as JSON Schema 2020-12 defines. Whether a `default` is
allowed is judged where it is written, in the schema that holds it (SCH-008). A `default` that
is reached only through a `$ref` written inside `allOf`, `anyOf` or `oneOf` is accepted where
it is written and is not applied (SCH-009). A `default` satisfies the schema that states it,
and the `default` of an input satisfies the input's own `schema`; a document in which one does
not is rejected (SCH-019).

## Formats that are checked

`date-time`, `date`, `duration`, `uri`, `uuid`. For each of them the grammar cited in the table
above is the one that is checked (SCH-004).

## Schemas this profile does not govern

A schema of an existing Model Context Protocol server, one that was not written for this
specification, keeps the rules of its own dialect (SCH-014).
