# Schema and syntax profile

Draft. Plan-audit rulings 23, 24, 32 and 33 of 2026-11-05.

Graphs and schemas are not what distinguishes Conduit. This part of the specification invents
nothing: it selects existing standards and narrows them. Where a standard defines a meaning,
that meaning applies unchanged. The promises that bind are in
[promises/schemas.yaml](promises/schemas.yaml) and, where a line below cites one, in the promise
it names; this page lists what they refer to.

## Standards relied on

| Subject | Standard | How it is used |
| --- | --- | --- |
| Document syntax | YAML 1.2.2, core schema | The authoring syntax, narrowed as listed below. |
| Equivalent syntax | JSON, RFC 8259 | A JSON document means the same as the equivalent YAML document (SCH-013). A JSON file holds one document or one array of documents (SCH-018). |
| Value model | I-JSON, RFC 7493 | No duplicate keys, and whole numbers within the range it recommends. |
| Schemas | JSON Schema 2020-12, core and validation | The meaning of every permitted keyword, the definition of integer, and the definition of equality. |
| `pattern` | ECMA-262 regular expressions | The dialect of a `pattern`, as JSON Schema recommends (SCH-016). |
| Pointers into values | JSON Pointer, RFC 6901 | The syntax of a pointer in a value reference, of an input's destination and of a control's `changes`, in the string syntax of section 3 and not the URI fragment form of section 6 (OBJ-016). The `-` token is not accepted in a destination. |
| `date-time`, `date` | RFC 3339, section 5.6 | Format grammar. |
| `duration` | RFC 3339, appendix A | Format grammar. |
| `uri` | RFC 3986 | Format grammar. |
| `uuid` | RFC 9562 | Format grammar. |
| Package versions | Semantic Versioning 2.0.0 | The grammar of a version (PKG-001). Two versions are the same when neither precedes the other, so build metadata is set aside; the digest decides whether two archives are the same package (PKG-003, PKG-021). |
| Digests | SHA-256, FIPS 180-4 | The digest of a package's archive, written as `sha256:` followed by the hash in lowercase hexadecimal (PKG-004). |
| Package archives | tar, as POSIX (IEEE Std 1003.1) defines it, compressed with gzip, RFC 1952 | A package's archive is a gzip-compressed tar file, and the digest is taken over the compressed file as distributed (PKG-022). |
| Host part of an address | Host names, RFC 1123, section 2.1 | The host part of a package's address: a DNS name written in lowercase, with labels of letters, digits and hyphens, at least one dot and no port (NAM-001). |
| The implementation interface | Model Context Protocol, revision 2026-07-28 | The wire between an engine, its implementations and its consumers, and the baseline revision of this version of the specification (IFC-002). |

Two rules follow an existing practice rather than a standard. Applying `default` values when a
template is instantiated, and refusing a `default` placed inside a combiner, both follow the way
Kubernetes treats defaults in structural schemas. So does a `default` on a property of an object
inside `items`: it is accepted and applies to each item that is present, while a `default` on
`items` itself is not accepted (SCH-008, SCH-009). Treating a step's use of another step's
output as a dependency follows Terraform, and writing the relation as "depends on" follows
Docker Compose and Terraform.

## YAML features not accepted

Duplicate keys, anchors and aliases, merge keys, and tags. Keys are strings. A number that has
no JSON form is not accepted either: an infinity or a not-a-number, such as `.inf` or `.nan`,
and an integer written in hexadecimal or octal, such as `0x1F` or `0o17` (SCH-017).

## Permitted schema keywords

`type`, `enum`, `const`, `properties`, `required`, `additionalProperties`, `items`, `minItems`,
`maxItems`, `uniqueItems`, `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`,
`minLength`, `maxLength`, `pattern`, `format`, `default`, `title`, `description`, `examples`,
`$ref`, `$defs`, `allOf`, `anyOf`, `oneOf`.

Every other keyword of JSON Schema 2020-12 is outside the profile, including `if`, `then`,
`else`, `not`, `multipleOf`, `patternProperties`, `dependentSchemas`, `unevaluatedProperties`,
`unevaluatedItems`, `$dynamicRef`, `$id`, `$anchor` and `$schema`.

The value of `$ref` is one of two things: `#` followed by a JSON Pointer into the same schema,
such as `#/$defs/name`, or a qualified reference to a Schema contract that a package exports
(SCH-015). A Schema contract is the fifth kind of contract, and its `spec` is a schema under
this profile (CTR-008).

## Enforced formats

`date-time`, `date`, `duration`, `uri`, `uuid`. For each of them the grammar cited in the table
above is the one that is checked (SCH-004).

## Schemas this profile does not govern

A schema published by an existing Model Context Protocol server keeps the rules of its own
dialect (SCH-014).
