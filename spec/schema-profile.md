# Schema and syntax profile

Draft. The owner accepted this profile as it stands at
[ruling 39](../docs/proposals/ruling-39/README.md), which accepted every logged decision of the
coordinator and choice of an editor. Plan-audit rulings 23, 24, 32, 33 and 37 of 2026-11-05
first accepted it. Decision DA3 of the [docs audit](../docs/proposals/docs-audit/GAPS.md), the
[decisions of the second round of the re-audit](../docs/proposals/reaudit/round-2/DECISIONS.md)
and the [decisions of its fourth round](../docs/proposals/reaudit/round-4/DECISIONS.md), which
are the coordinator's and which ruling 39 accepted, changed it since. The fourth round changed
what this page says of when a `default` is applied.
[Ruling 41](../docs/proposals/ruling-41/README.md), of 2026-11-06, which applies
[ruling 40](../docs/proposals/ruling-40/README.md) and accepts the
[principles review](../docs/proposals/principles-review/README.md), widened the profile to
JSON Schema 2020-12's applicator vocabulary and its validation vocabulary less `multipleOf`
(decision 20), added the format `uri-reference`, and added the standards of server.json,
SARIF, CloudEvents and Kubernetes conditions (decisions 21, 24 and 25).
[Ruling 42](../docs/proposals/ruling-42/README.md), of the same day, struck none of the
decisions that the coordinator settled on the questions the rewrite left open; two of them
changed this page, which now says how a value bound by an implementation item is converted to
a string, with RFC 8785 for a number, and how an argument of a Package is named. Its names, which the
[vocabulary audit](../docs/proposals/vocabulary-audit/README.md) proposed, then changed what
this page says of an item's keys, `packageArguments` in place of `arguments`, of a Package's
`fileSha256`, which pins it in place of the item's `digest`, and of a condition, whose
`origin` is `source` and which has no `observedGeneration`, as the
[log of those renames](../docs/proposals/ruling-42/log-rename-b.md) lists.
[Ruling 43](../docs/proposals/ruling-43/README.md), of the same day, on work across machines,
changed what this page says of an implementation item, which binds no variable by a name that
`identity` carries and, reached by `remote` and not plain, names no `Authorization` header,
and of a `remote` URL, to which a header that carries a secret is bound only where SEC-024 lets
it go, as the [log of its docs editor](../docs/proposals/ruling-43/log-docs.md) says. The
[integration of the fixes of round 5](../docs/proposals/reaudit/round-5/log-fixes.md) changed
what it says of a `remote` URL, which holds no variable and to which an item selected for
`get-secret.v1` is bound only where SEC-024 lets it stand, and of the place an item binds,
which is confined to a string, a number, a boolean or null. Its
[verification](../docs/proposals/reaudit/round-5/log-fixes.md#verification) said above that
ruling 42 struck none of the coordinator's decisions (V10).
[Ruling 44](../docs/proposals/ruling-44/README.md), of 2026-11-07, on round 6 of the re-audit,
changed what this page says of the root of an Operation contract's schemas, where the whole
profile may now be used beside `type: object`, a control's apart (D14); added what the schema of
a tool that the engine presents is made of (finding 6-1-04); and added how two pointers are
compared (finding 6-1-03), as the
[log of its docs editor](../docs/proposals/reaudit/round-6/log-specdocs.md) says. The
[verification of round 6](../docs/proposals/reaudit/round-6/log-integration.md#verification)
added that a copy's key holds a version only where the package or project states one (R-08).

Schemas are not what distinguishes Conduit. This part of the specification invents nothing: it
selects existing standards and narrows them. Where a standard defines a meaning, that meaning
applies unchanged. The promises that bind are in [promises/schemas.yaml](promises/schemas.yaml)
and, where a line below cites one, in the promise it names; this page lists what they refer
to. The profile is itself written as a schema, the
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
| Pointers into values | JSON Pointer, RFC 6901 | The syntax of an input's destination, of a value of a control's `changes` and of a value of an implementation item's `environment`, `packageArguments` or `headers`, in the string syntax of section 3 and not the URI fragment form of section 6 (OBJ-016). The `-` token is not accepted in a destination. No two pointers of a control's `changes` are equal or lie one inside the other (CTR-006), as no two destinations of a template's inputs do (INS-010). Two pointers are compared by their reference tokens, each with `~1` and `~0` read as `/` and `~`, as section 4 evaluates them: one lies inside another where the other's tokens begin its own, the empty pointer lies over every other, and `/a~1b`, one token, is not `/a/b` (CTR-006, PRV-016). |
| `date-time`, `date` | RFC 3339, section 5.6 | Format grammar. |
| `duration` | RFC 3339, appendix A | Format grammar. |
| `uri` | RFC 3986 | Format grammar. |
| `uri-reference` | RFC 3986, section 4.1 | Format grammar. `fetch` is checked by it (MAN-006). |
| `fetch` | RFC 3986; `file:` URLs, RFC 8089 | `fetch` in an item of a manifest's `packages` is a URI reference, with no user information. A relative one is resolved against the directory the manifest was read from to a `file:` URL, and is rejected where the manifest was handed over without a directory (MAN-006). Every engine fetches `https:` and `file:` URLs; another scheme is fetched by a package-source provider that claims it (NAM-012, PRV-017). |
| An implementation's `remote` URL | RFC 3986 | The `url` of a `remote` item is a URI, reached over the protocol's Streamable HTTP transport, so one that holds a variable of the Remote in braces is rejected, and no key binds such a variable (MAN-010). A template whose item binds a header to a secret, or is selected for `get-secret.v1`, is rejected unless that URL is `https`, or, on an engine that can neither restart on another machine nor run as several parts, `http` with the host `localhost`, an IPv4 address in `127.0.0.0/8` or `[::1]`, and on an engine that can do either, any item at such a host is rejected; the scheme and the host are compared without regard to case, as RFC 3986 has them, and no other host counts, a name that resolves to a loopback address among them (SEC-024). An item that is not plain is connected to only at an `https` URL (IFC-041). |
| Implementation items | The Model Context Protocol Registry's server.json, schema 2025-12-11 | A `package` item holds one server.json Package, pinned by its `fileSha256` or, for an `oci` image, by a digest in its `identifier`, and a `remote` item one server.json Remote whose `type` is `streamable-http`, each as that schema gives it (MAN-010). An argument of the Package's `packageArguments` is named by its `name`, or by its `valueHint` where it is positional, as server.json labels one, and binding one that states a fixed `value` is rejected; no key binds its `runtimeArguments` (MAN-010). A variable, an argument or a header holds a string, so the place it is bound to is confined to a string, a number, a boolean or null (MAN-017); a boolean is given as `true` or `false`, as server.json gives one, and a place that holds null gives nothing (PRV-007, IFC-041). A template is rejected whose item's `environment` binds a variable by a name that `identity` carries, which [names.yaml](names.yaml) fixes (MAN-018), and an item reached by `remote` that is not plain names no `Authorization` in its `headers`, in any case of its letters (IFC-043). |
| `uuid` | RFC 9562 | Format grammar. JSON Schema 2020-12 cites RFC 4122 for this format; the grammar cited here is the one that is checked (SCH-004). |
| Package versions | Semantic Versioning 2.0.0 | The grammar of a version (PKG-001). Two versions are the same when neither precedes the other, so build metadata is set aside; the digest decides whether two archives are the same package (PKG-003, PKG-021). |
| Digests | SHA-256, FIPS 180-4 | The digest of a package's archive, written as `sha256:` followed by the hash in lowercase hexadecimal (PKG-004). The digest that identifies the contracts of one submission of a project is written in the same way (SUB-008). A server.json Package states the hash of its package file in `fileSha256`, alone, in lowercase hexadecimal, and an `oci` image is pinned by a digest written `@sha256:` and the hash at the end of its `identifier` (MAN-010). |
| Canonical form of a value | JSON Canonicalization Scheme, RFC 8785 | The bytes that are hashed for the digest that identifies the contracts of one submission of a project: the canonical form of one map that holds the project's contract documents and its manifest (SUB-008). It writes equal values alike, so `1` and `1.0` give the same bytes. The contract documents are ordered by kind and then by name, each compared by the bytes of its UTF-8 string. A number that an implementation item binds to a variable, an argument or a header is given in the form that RFC 8785 writes it (PRV-007, IFC-041). |
| Package archives | tar, as POSIX (IEEE Std 1003.1) defines it, compressed with gzip, RFC 1952 | A package's archive is a gzip-compressed tar file, and the digest is taken over the compressed file as distributed (PKG-022). Which of the tar formats and extensions an engine reads is item DL7 of the [implementation-defined list](implementation-defined.yaml). |
| Host part of an address | Host names, RFC 1123, section 2.1 | The host part of a package's address: a DNS name written in lowercase, with labels of letters, digits and hyphens, each at most 63 characters, at least one dot, no port and no trailing dot. A host whose last label is all digits is not a host name (NAM-001). |
| Internationalized host names | IDNA, RFC 5890 | An internationalized name is written in its ASCII form, with `xn--` labels (NAM-001). |
| The implementation interface | Model Context Protocol, revision 2026-07-28 | The wire between an engine, its implementations and its consumers, and the baseline revision of this version of the specification (IFC-002). The protocol is adopted as it is: an engine and an implementation keep every requirement of the revision in use, and this specification repeats none of them (IFC-001). The protocol's tasks extension is outside the baseline revision, and may be used too (IFC-016). An Operation contract's `inputSchema` and `outputSchema` are written as the protocol's tool definition writes them (CTR-002, CTR-012). Its `inputSchema` holds `type: object` at its root and any keyword of JSON Schema 2020-12 beside it, composition keywords among them, and its `outputSchema` any schema; the protocol advises that the branches of a composition at the root assert only which properties are present. An Operation contract may use every keyword of the profile at the root of either, beside `type: object`, which it keeps for `outputSchema` too because its outputs are the members of `structuredContent`, keyed by name (IFC-001); a control's schemas hold at their root only `type`, `properties`, `required` and `additionalProperties` (CTR-012). |
| Diagnostics | SARIF 2.1.0, its `result` | The core schema `diagnostic.v1` is SARIF's `result` reduced to `ruleId`, `level`, `message.text` and the logical locations of `locations`, whose fully qualified name is a JSON Pointer; `ruleId` is the ID of a promise (IFC-025). |
| Records | CloudEvents 1.0 | The core schema `record.v1` is a CloudEvent whose `data` holds the fields of a record (REC-002). |
| Conditions | Kubernetes `metav1.Condition` | Inspection reports `conditions` in that shape, as the core schema `condition.v1` gives it, without the optional `observedGeneration`, with `lastHeartbeatTime` and `source` added as Conduit's own (LIF-011). |

Two rules follow an existing practice rather than a standard. Applying `default` values to a
value that is supplied, before it is validated, and rejecting a `default` placed inside `allOf`,
`anyOf`, `oneOf` or `not`, both follow the way Kubernetes treats defaults in structural schemas.
The first is one rule for every schema that governs a supplied value: the configuration of an
instance, and the argument of an input of a template, of an operation or of a requirement. The
filled value is what is validated, compared, stored and sent (SCH-009). So does a `default` on a
property of a map inside `items`: it is accepted and applies to each item that is present,
while a `default` on `items` itself is not accepted (SCH-008, SCH-009). The second extends to
the conditional keywords `if`, `then`, `else` and `dependentSchemas`, under which a `default`
would apply only where a condition holds (SCH-008). A `default` that is a map is checked
against its schema after the defaults of its own properties are filled in (SCH-010, SCH-019).
Ordering a Pool's entries by the entries they refer to, and writing that relation as "depends
on", follows Docker Compose and Terraform.

## YAML features not accepted

Duplicate keys, anchors and aliases, merge keys, and tags. Keys are strings. A number that has
no JSON form is not accepted either: an infinity or a not-a-number, such as `.inf` or `.nan`,
and an integer written in hexadecimal or octal, such as `0x1F` or `0o17` (SCH-017). Nor is a
number that is not an integer and is outside the range of IEEE 754 binary64 (SCH-021).

## Schema keywords in the profile

The applicator vocabulary of JSON Schema 2020-12, which is `allOf`, `anyOf`, `oneOf`, `not`,
`if`, `then`, `else`, `dependentSchemas`, `prefixItems`, `items`, `contains`, `properties`,
`patternProperties`, `additionalProperties` and `propertyNames`; its validation vocabulary but
for the one keyword on multiples, which is `type`, `enum`, `const`, `maximum`,
`exclusiveMaximum`, `minimum`, `exclusiveMinimum`, `maxLength`, `minLength`, `pattern`,
`maxItems`, `minItems`, `uniqueItems`, `maxContains`, `minContains`, `maxProperties`,
`minProperties`, `required` and `dependentRequired`; and `format`, `default`, `title`,
`description`, `examples`, `$ref` in the three forms below, and `$defs`.

Every other keyword of JSON Schema 2020-12 is outside the profile: `$id`, `$schema`, a `$ref`
to anything but the three forms below, `$anchor`, which would collide with `#name`,
`$dynamicRef`, `$dynamicAnchor`, `$vocabulary`, `$comment`, `unevaluatedProperties`,
`unevaluatedItems`, `multipleOf`, the content vocabulary (`contentEncoding`,
`contentMediaType` and `contentSchema`), `deprecated`, `readOnly` and `writeOnly`.

The value of `$ref` is one of three things: `#` followed by a JSON Pointer into the same
schema, such as `#/$defs/name`; a qualified reference to a Schema contract that a package
exports; or `#` followed by a name, for a Schema contract of the same project or package
(SCH-015, NAM-022). The three cannot be confused: a JSON Pointer is empty or starts with `/`,
and a name starts with a letter. A Schema contract is the fifth kind of contract, and its
`spec` is a schema under this profile (CTR-008).

A schema is a map or a boolean, as JSON Schema 2020-12 defines. Whether a `default` is
allowed is judged where it is written, in the schema that holds it (SCH-008), and none is
allowed inside `allOf`, `anyOf`, `oneOf`, `not`, `if`, `then`, `else` or `dependentSchemas`. A
`default` that is reached only through a `$ref` written inside one of those eight is accepted
where it is written and is not applied (SCH-009). A `default` satisfies the schema that states
it, and the `default` of an input satisfies the input's own `schema`; a document in which one
does not is rejected (SCH-019).

## Formats that are checked

`date-time`, `date`, `duration`, `uri`, `uri-reference`, `uuid`. For each of them the grammar
cited in the table above is the one that is checked (SCH-004).

## Schemas this profile does not govern

A schema of an existing Model Context Protocol server, one that was not written for this
specification, keeps the rules of its own dialect (SCH-014).

Nor does the profile, or CTR-012, govern the schemas of a tool that the engine presents: they
are the engine's, made for the wire from the contract's own, and the engine checks requests and
results against the contract (IFC-033). They can be read with no package at hand. Each holds,
under the `$defs` of its root, one copy of each Schema contract that the contract's schemas
reach, directly or through other contracts, keyed by the contract's qualified reference with `@`
and the version of the package or project that holds it after the address, where that package or
project states one, so that no two packages, and no two versions of one, share a key. Every
`$ref` in it is a JSON Pointer within it: one that named a contract points to that contract's
copy, and one that pointed into a schema points to the same place in the copy, with `~` and `/`
escaped as RFC 6901 escapes a reference token and `#` written `%23`, as a URI fragment needs.
The keywords written beside a `$ref` are kept, and a contract that refers to itself refers to
its own copy. A contract whose schema holds no `$ref` may be written in the place of a reference
to it beside which no keyword is written, instead of being copied. This follows the bundling of
JSON Schema 2020-12, section 9.3, except that a copy carries no `$id` and its pointers are
rewritten in its place. For a control, an input that the target's template narrows has the
schema `allOf` of the contract's and the template's (TPL-010, IFC-033).
