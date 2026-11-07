# Ruling 42: log of the other renames

Date: 2026-11-06. This is the account of the editor who applied the names that
[ruling 42](README.md) accepted from the [vocabulary audit](../vocabulary-audit/README.md) and
that the [first rename](log-rename-a.md) left: N1, N4 to N7 and N12 to N20, with the owner's
answers on N7 and N14, as each subsection of
[Names for the owner](../vocabulary-audit/README.md#names-for-the-owner) says what changes with
it. The names are the owner's. The wording that carries them out is the editor's, and each
choice listed under [Choices beyond the record](#choices-beyond-the-record) stands unless the
coordinator or the owner strikes it. Nothing was committed.

Not applied here: the editorial fixes E1 to E23, which another stage applies, and D5 and D20,
which the owner did not rule. Where an editorial fix concerns a sentence that a rename reached
(E3, E8, E17, E19), the rename was made and the fix left, as [What was kept](#what-was-kept)
says. The gap that `spec/README.md` states for D5 now names the run's `url` where it named its
`endpoint`, and says nothing else that is new; row 17's guard for D20, in PRV-007, MAN-010 and
`run.v1`, was not touched.

Scope: every file under `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`, and one line
of `tools/set_names.py` (see [the names check](#the-names-check)). No other file was changed but
this log. Each changed YAML file has a line at the end of its head comment naming ruling 42's
names, this log and what changed; the Markdown files say it in their head paragraphs
(`spec/README.md`, `spec/definitions.md`, `spec/core/README.md`, `spec/schemas/README.md`,
`spec/schema-profile.md`, `docs/OVERVIEW.md`) and the checker in the comment above `RULES`.
Two example files were added, and none was renamed: no example file's name carries one of these
names.

How it was done: the tree was copied aside first, and every occurrence of every old name was
read where it stands before it was changed. The changes were made by exact replacements, one
sentence at a time, of text that a search across line breaks had found, and each changed
paragraph was rewrapped only from the changed line until it met its old line breaks again. The
diff against the copy was then read whole. No other job changed these files meanwhile: the
copy and the working tree agreed when the work began.

## Checks

Run from the repository root after the renames.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | 439 promises ok |
| Every promise has exactly the seven fields, its text fields folded with `>-` | Yes |
| A YAML line over 96 characters under `spec/` | None |
| `python3 tools/check_docs.py` | 0 errors in 151 Markdown files, with this log written |
| `python3 tools/set_names.py --check` | 0 errors, after the change to its pattern below |
| `python3 tools/check_schemas.py` | 0 errors. 54 invalid documents rejected, the two new ones among them; the rules `secret-only` (SEC-022) and `item-place-required` (MAN-017) reject their examples under the new names |
| `git diff --check` | Clean |

No promise was added, removed or moved, and no ID changed. 69 promises in 13 files changed, each
only by these names, as the head comment of each file lists:

| File | Promises changed |
| --- | --- |
| `interface/implementation.yaml` | 13: IFC-006, IFC-012, IFC-016 to IFC-020, IFC-023, IFC-026, IFC-029, IFC-037, IFC-038, IFC-040 |
| `promises/secrets.yaml` | 12: SEC-002, SEC-005 to SEC-011, SEC-015, SEC-020 to SEC-022 |
| `conventions/operations.yaml` | 11: OPS-008 to OPS-012, OPS-018, OPS-019, OPS-021, OPS-024, OPS-025, OPS-028 |
| `promises/providers.yaml` | 9: PRV-001, PRV-003 to PRV-005, PRV-007, PRV-009 to PRV-011, PRV-018 |
| `conventions/access.yaml` | 7: ACC-001, ACC-003, ACC-005, ACC-006, ACC-009, ACC-017, ACC-018 |
| `promises/manifest.yaml` | 4: MAN-009, MAN-010, MAN-012, MAN-017 |
| `conventions/records.yaml` | 3: REC-002, REC-004, REC-005 |
| `conventions/observation.yaml` | 2: OBS-002, OBS-004 |
| `promises/objects.yaml`, `packages.yaml`, `templates.yaml` | 2 each: OBJ-016, OBJ-019; PKG-019, PKG-020; TPL-017, TPL-018 |
| `conventions/lifecycle.yaml`, `promises/contracts.yaml` | 1 each: LIF-011, CTR-002 |

Also changed: the head comments of `implementation-defined.yaml` and `promises/first-start.yaml`
(N18); in the core package, `capabilities.yaml`, `conduit.yaml`, `operations.yaml`,
`references.yaml`, `results.yaml` and the README; the schemas `manifest.yaml`, the comments of
`templates.yaml` and `contracts.yaml`, and the schemas README; `schema-profile.md`; the examples
`valid/project/conduit.yaml`, `contracts.yaml` and `templates.yaml`,
`valid/files-package/conduit.yaml` and `invalid-by-rule/SEC-022-...`; and the checker. The
counts below are of occurrences under `spec/`, in `tools/check_schemas.py` and in
`docs/OVERVIEW.md`, found across line breaks, leaving out the lines that this pass wrote to
record the renames.

## The names

### N1: `secret-reveal.v1` becomes `secret-provider.v1`, `reveal-secret.v1` becomes `get-secret.v1`

| What | Changed | Kept |
| --- | --- | --- |
| `secret-reveal.v1` | 39 | 7 |
| `reveal-secret.v1` | 61 | 4 |

Files: the promises SEC-002, SEC-005 to SEC-010, SEC-015, SEC-020, SEC-022, PRV-001, PRV-004,
ACC-001, ACC-003, ACC-005, ACC-006, IFC-012, MAN-012 and PKG-020; the core package's
`capabilities.yaml` (the capability, its operation and its line under "Promises encoded"),
`operations.yaml` (the contract, `open.v1` and the rules of its head), `conduit.yaml` (two
exports and a comment), `references.yaml` and the README (its export tables and prose);
`spec/README.md`, `spec/definitions.md` (eleven rows), `spec/schemas/README.md` (four rows), the
comments of `schemas/templates.yaml` and `schemas/contracts.yaml`; the example project's
`conduit.yaml`, `contracts.yaml` and `templates.yaml`, the example for SEC-022; and the
checker's `PROVIDER_CAPABILITIES` and its rule `secret-only` (code, comment and message).

What changed with it, as the subsection asks:

- The noun "the engine's read of a secret" stays, and so do secret Resource, store, locator,
  `secret-reference.v1` and the output `value`. The row provider call and PRV-005 still say
  that the read is a request and not a provider call; neither was touched.
- The row secret Resource adds that each secret Resource locates exactly one secret, so
  `get-secret.v1` takes no input, where the `GetSecret` of External Secrets Operator or Dapr,
  whose store serves many secrets, takes the name of one.
- The sentences that called the names not final now cite the owner's ruling: in
  `spec/README.md`, `spec/definitions.md`, the core README, and the head comments of
  `secrets.yaml` and `providers.yaml`.

### N4: `inspect-run.v1`'s state `stopped` stays

Nothing changed, as the owner accepted the recommendation to keep it.

### N5: `run.v1`'s output `endpoint` becomes `url`

12 occurrences changed in 5 files: `run.v1`'s description, its `outputSchema` and its
`required`; PRV-003 (two), PRV-007 (two), PRV-018, IFC-017 (two), the row run, and the gap that
`spec/README.md` states for D5. 4 kept, all in head comments and in the core README's paragraph
that records ruling 42's earlier pass. The engine now reaches every Streamable HTTP server at a
`url`, a run's as a Remote's. The words-not-used row says that the prose may still call it the
MCP endpoint.

### N6: the `arguments` of an implementation item and of `run.v1` become `packageArguments`

38 occurrences changed in 12 files, by the search below, and 3 kept in records of earlier
passes:

- The key of `implementation-item.v1` (`manifest.yaml`: the property, `dependentRequired`, the
  description, "Promises encoded" and "Not expressed here"), MAN-009, MAN-010, MAN-017, PKG-019,
  PKG-020, OBJ-016, OBJ-019, SEC-006, SEC-010, SEC-020 (two) and SEC-021; the rows
  implementation item, plain, run, the engine's read of a secret and value binding;
  `schema-profile.md` (two rows), `spec/schemas/README.md` (three rows); the example
  `files-package/conduit.yaml` (the key of `listing` and its comment); and the checker's
  `declared_input` and `item-place-required` (code and comments).
- `run.v1`'s input, and the member of its `secrets` that mirrors it (see the choices), in its
  description and its schema, and in PRV-007 (three).

What changed with it, as the subsection asks:

- MAN-010 states that a key is the `name` of a named argument, with its dashes, or the
  `valueHint` of a positional one, declared in the Package's own `packageArguments`; a key that
  names an argument of `runtimeArguments` is rejected, as one that names no argument is. Its
  observable adds that no key binds `runtimeArguments`, which go to the runtime command, such as
  `npx` or `docker`, and so are the runtime provider's, under PRV-012. The description of
  `implementation-item.v1`, `schema-profile.md` and the row implementation item say the same.
- The checker's `declared_input` no longer looks in `runtimeArguments`, so MAN-017's rule for an
  input marked `isRequired: true` reads the Package's `packageArguments` alone.
- The row argument loses its carve-out: `arguments` is now only what an instance document, a
  nested instantiation or an applied requirement supplies, and the arguments of a call.
- `headers` and `environment` stay. Every other `arguments` was read and kept: an instance's,
  a nested instantiation's, an applied requirement's (OBJ-013 and the rest, the schemas of
  instances and templates, the examples), a filter item's in the list of IFC-018, and those of
  a `tools/call` request (IFC-001, SEC-001, SEC-019), with "arguments" in plain prose.

### N7: the `digest` of a `package` item goes, and the Package's own `fileSha256` pins it

8 occurrences of the item's `digest` changed, none kept. With the owner's answer on N7, an
`oci` Package is pinned by a digest in its `identifier`, as server.json allows in place of the
tag; the MCP
Registry parses such an identifier with go-containerregistry, which reads
`registry/namespace/image@sha256:<hex>` and `image:tag@sha256:<hex>`. PRV-012 lists `npm`,
`pypi` and `oci` as examples of the registry types a runtime provider runs; only `oci` has no
single file to hash.

- `implementation-item.v1` loses the property `digest` and its two `dependentRequired` rows,
  and gains, under `dependentSchemas` for `package`, a rule on the Package: where its
  `registryType` is `oci`, its `identifier` ends in `@sha256:` and 64 lowercase hexadecimal
  digits; otherwise it states `fileSha256`. `server-package.v1` stays a transcription of
  server.json, where `fileSha256` is optional.
- MAN-009 no longer lists `digest`. MAN-010's `then` says that the Package is pinned, by its own
  `fileSha256`, the SHA-256 of the package file in lowercase hexadecimal, or, for `oci`, by a
  digest in its `identifier`; its observable rejects a Package that is pinned neither way, and
  says that the item states its artifact's hash once, in the Package, as server.json writes it.
  PKG-019 says that a `package` item's Package is pinned to one artifact, under MAN-010.
- The rows implementation item and digest, `schema-profile.md` (the rows implementation items
  and digests), `docs/OVERVIEW.md` and the description of the examples say the same.
- The example `listing` in `files-package/conduit.yaml` moved its hash from `digest:
  sha256:4c2e...` to `fileSha256: 4c2e...` in its Package. Two examples were added under
  `examples/invalid/`: `MAN-010-manifest-package-without-file-sha256.yaml`, an `npm` Package
  with no hash, and `MAN-010-manifest-oci-package-without-a-digest.yaml`, an `oci` Package named
  by a tag alone. The count in both READMEs is now 54.
- A package's `digest` under `packages` (PKG-004), the digest of a submission (SUB-008) and
  `digest.v1` are untouched.

### N12: `record.v1`'s `data.instance` becomes `data.consumer`

The member of `record.v1` and its description, REC-002 (`then` and observable, which now names
the member), SEC-015, and the schemas README row on `record.v1`, which lists `consumer` beside
`target`, since it is not one of Conduit's keys. "Acted for the caller" (3) and "reading
instance" (5) are gone: the engine reads a secret for an instance through that instance's own
Conduit, in the rows the engine's read of a secret and request, in the descriptions of
`secret-provider.v1` and `get-secret.v1`, and in the core README. The record names the consumer
where a Conduit was used, so a hook and a provider call, which go through none, name none.

### N13: `observedGeneration` leaves `condition.v1` and LIF-011

3 occurrences removed: the property of `condition.v1`, its description, and LIF-011's list of
fields. The description now says that the optional field is left out because no object of
Conduit has a generation, and the rows condition and the profile's row on conditions say
without it. `condition.v1` stays open, so a condition copied from Kubernetes that carries the
field still validates.

### N14: origin becomes source

21 occurrences changed in 7 files, 1 kept (HTTP's origin, in IFC-037). With the owner's answer
on N14, a condition's `source` takes a record's form: `condition.v1`'s `source` is
`{type: string, format: uri-reference, minLength: 1}`, a URI-reference that the engine names.

- The condition key in `condition.v1`, LIF-011, OBS-002 and PRV-009 ("a `source` that names the
  runtime provider"); the term in OBS-002, REC-002 and REC-005; the rows condition and
  observation; the profile's row on conditions.
- REC-005 says that a record's source is its CloudEvent's `source`, the context in which it was
  made, as CloudEvents' sequence extension orders events within one `source`, and, as the
  subsection asks the owner, that an engine that names one `source` for every record keeps all
  its records in one order.
- The row origin is the row source, which says that the term is the CloudEvents attribute and
  that `package-source.v1` is another thing. The row of words that went with graphs named
  `source`; it now names "`source` of a value reference" and says what source is now.

### N15: operation identity becomes idempotency key

78 occurrences changed in 12 files: 51 of "operation identity", 8 of `operationIdentity`, and
19 of the bare word identity where it meant the key (in OPS-008, OPS-011, OPS-018, OPS-021,
OPS-024, OPS-028, REC-004, three rows and the overview). The `_meta` key is
`io.github.shrav-k.conduit/idempotencyKey`, in IFC-006, IFC-023, OPS-028, PRV-005, the member of
`record.v1`, the row Conduit's keys and the core README. The convention is "idempotency keys and
outcomes", in `spec/README.md` (two), the first line of `conventions/operations.yaml` and the
overview. The row is the row idempotency key, which adds that the IETF's draft of the
`Idempotency-Key` header defines one so and names the `_meta` key. Kept: identity in its other
senses (a caller's identity in ACC-017, `identity` of `run.v1`, an object's ID, a package's
address, SPIFFE's identity documents) and the history paragraph of `spec/definitions.md`.
`deduplicates` stays; E3 relates it to `idempotentHint`.

### N16: the `_meta` key `io.github.shrav-k.conduit/restrictions` becomes `.../filters`

The key in 9 places (IFC-006 two, IFC-018, IFC-019, IFC-020, IFC-026, ACC-017, ACC-018 two),
written short as `filters` in PRV-005, the row Conduit's keys and the core README. "Filters" in
place of "restrictions" where only filters are carried or sent, as the subsection lists:
ACC-009's clause on what is carried, ACC-017 (five), ACC-018 (three), IFC-006, IFC-019 (five),
IFC-020 (two), IFC-026's `when` (two), IFC-040's observable, SEC-011 (two), TPL-017, the rows
onward request and hook, and the comments and READMEs that say what an implementation is sent
(`references.yaml`, the core README, the schemas README, the overview).

Kept: "restriction" in ACC-008, ACC-014, ACC-027 and the general case (ACC-009's first two
sentences, IFC-026's `then`, IFC-040's `then`, PRV-005's "no restriction applies to it", the
rows permitted and bound), and the result key `io.github.shrav-k.conduit/restriction`, which can
name a bound (IFC-025): IFC-019 now says that the implementation names the filter in that key.

### N17: the extension `io.github.shrav-k.conduit/interface` becomes `.../implementation-interface`

4 occurrences changed: IFC-037, IFC-038, the row Conduit's extension and the core README, with
IFC-029's derivation, "that prefix followed by `implementation-interface`". 1 kept: the head
comment of `implementation.yaml` that records ruling 41's change.

### N18: runtime profile becomes conformance document

9 changed in 5 files: `spec/README.md` (three), the row, which adds the POSIX precedent and that
it is not a document in the sense of `definitions.md`, since no engine reads it; the head
comments of `implementation-defined.yaml` and `first-start.yaml`; and the overview (two). None
kept under `spec/`.

### N19: trust domain becomes domain of trust

4 changed: the row, `spec/README.md`, PRV-011's observable and the head comment of
`secrets.yaml`. The row cites the AWS agent's phrase and says that it is not SPIFFE's trust
domain, which a mechanism of DL13 may use.

### N20: "its own setting for restarts" becomes restart policy

6 changed: PRV-010 (`then` and observable, which now cites Kubernetes' `restartPolicy` beside
`Restart=` and `restart:`), OBS-004's observable, the rows run and runtime, and the overview.
The row policy adds that the word stays only in restart policy, a runtime provider's own term.

### The words the promises do not use

`spec/definitions.md` lists the earlier words in thirteen new rows, after the five of the first
rename: `secret-reveal.v1` and `reveal-secret.v1`; `run.v1`'s `endpoint`; the `arguments` of an
item and of `run.v1`; the `digest` of a `package` item; the instance that acted for the caller,
the reading instance and a record's `instance`; `observedGeneration`; origin; operation identity
and `operationIdentity`; `io.github.shrav-k.conduit/restrictions`;
`io.github.shrav-k.conduit/interface`; runtime profile; trust domain; and its own setting for
restarts. Each says what replaced it and which name of the audit did.

### The names check

`tools/set_names.py --check` reads Conduit's `_meta` keys from `implementation.yaml` as the
words after the prefix, with a pattern that takes a letter and then letters and digits. That
pattern stops at a hyphen, so it read the extension
`io.github.shrav-k.conduit/implementation-interface` as the key `implementation`, and then
reported the extension's full name, in four places, as "not a `_meta` key that
spec/interface/implementation.yaml defines". The prefix is unchanged, as the task says, but the
name is not one word. The tool's pattern `WORD` now also takes such words joined by hyphens, as
the protocol's extension names are (`io.modelcontextprotocol/oauth-client-credentials`), and
`meta_keys` uses it. Nothing else of the tool changed, and the check reports 0 errors.

## What was kept

Every remaining occurrence of an old name under `spec/`, in the checker and in the overview,
found by searching for each across line breaks, and why it stays. The thirteen words-not-used
rows and the lines this pass wrote are left out.

| Where | Old word | Why |
| --- | --- | --- |
| Head comments: `capabilities.yaml` (ruling 41's and ruling 42's lines), `operations.yaml` of the core (two), `providers.yaml` (ruling 41's description of the file and ruling 42's line), `secrets.yaml` (ruling 42's line), `schemas/contracts.yaml`, `access.yaml` (the integration's line) | `secret-reveal.v1`, `reveal-secret.v1` | They record what an earlier pass did, in the words of its time |
| `secrets.yaml`, head: the sentence that called the names not final, now rewritten | `secret-reveal.v1`, `reveal-secret.v1` | It says what the names were and what ruling 42 made them |
| `spec/core/README.md`: ruling 41's paragraph (the names were not final) and ruling 42's (the descriptions that followed the answers) | `secret-reveal.v1`, `reveal-secret.v1`, `endpoint`, `arguments` | Records of those passes |
| Head comments of `implementation.yaml`, `providers.yaml` and the core `operations.yaml` | `endpoint` | Ruling 42's earlier lines |
| Head comments of the core `operations.yaml` and `schemas/manifest.yaml` | `arguments`, `digest` | Ruling 41's and ruling 42's earlier lines |
| `spec/schemas/README.md`, ruling 42's paragraph: "says how `arguments` names a positional argument" | `arguments` | The record of the earlier pass |
| Head comment of `implementation.yaml`: the key `restrictions` in a tool's definition (round 2), and the extension `io.github.shrav-k.conduit/interface` (ruling 41) | `restrictions`, `.../interface` | Records of those passes |
| `conventions/operations.yaml`, head: "made OPS-015 the rule for an identity" | identity | The record of round 2 |
| `spec/definitions.md`, the history paragraph: "of the scope of an operation identity" | operation identity | The history of the rows |
| IFC-037: "for its origin on HTTP" | origin | The web origin of HTTP, which the protocol's client keys its discovery on |
| ACC-008, ACC-014, ACC-027, ACC-009's first two sentences, IFC-026's `then`, IFC-040's `then`, PRV-005, the rows permitted and bound, the overview's section on access | restriction | The general case, which N16 keeps |
| Every `arguments` of an instance, a nested instantiation, an applied requirement, a filter item or a `tools/call` request | `arguments` | Not the item's key (see N6) |
| A package's `digest`, a submission's digest, `digest.v1`, `fetch-package.v1`'s input | digest | Not the item's key (see N7) |
| `inspect-run.v1`'s `stopped`, PRV-009, LIF-011 | `stopped` | N4 keeps it |

Left for the stage that applies the editorial fixes, though a rename reached the sentence:

- E8: TPL-017's observable now reads "Its request carries the filters"; E8 makes it "It carries
  the filters".
- E17: "secret store" stays in the core README, `capabilities.yaml`, `operations.yaml` (the
  section comment "The operation of a secret store"), the schemas README, the example project,
  the row secret store, the checker's message for SEC-022 and the name of its example
  `SEC-022-secret-store-type-that-offers-another-capability.yaml`.
- E3: the row `deduplicates` and CTR-002, which now say idempotency key, do not yet relate it to
  `idempotentHint`.
- E19: the row record still says "a request, a provider call or an event".
- The sentences in `spec/README.md`, `spec/definitions.md` and the overview that this pass wrote
  say that the editorial fixes are not yet applied; that stage changes them.

## Choices beyond the record

1. **`secrets.packageArguments`.** `run.v1`'s `secrets` holds the values read for the run under
   `environment` and `arguments`, the names of its two other inputs. N6 renames the input; the
   member of `secrets` follows it, so that a value in `secrets` stands under the name of the
   input it would otherwise be in. PRV-007 and the row run say so.
2. **The digest of an `oci` image.** The rule is written as a pattern on `identifier`,
   `@sha256:[a-f0-9]{64}$`: PKG-004's form of a digest, after the `@` that OCI references use,
   at the end of the identifier, as go-containerregistry reads the registry's examples. An `oci`
   Package that also states `fileSha256` is not rejected; nothing gives that hash a meaning.
3. **Where the rule stands.** It is in `implementation-item.v1`, under `dependentSchemas`, and
   not in `server-package.v1`, which stays a transcription of server.json.
4. **No promise says who checks the hash.** None said who checked the item's `digest` either.
   MAN-010 says that the item states the hash once, as server.json writes it; server.json has
   the client that downloads a package check it.
5. **Two new examples** under `examples/invalid/`, one for each way of pinning (see N7).
6. **"No key binds `runtimeArguments`"** is said in MAN-010's observable, the description of
   `implementation-item.v1`, the profile and the definitions, where the subsection asks only to
   leave them unbound, so that a reader of the item does not look for them.
7. **"With its dashes"** in MAN-010, which the subsection's wording asks for, for a named
   argument such as `--root`.
8. **N12 beyond the records.** "The reader's own Conduit" in the core README's export table
   became "the instance's own Conduit", as "reading instance" did. "The reading Conduit" in
   SEC-013 stays: it names a Conduit, not an instance.
9. **The condition's `source`.** PRV-009 says "a `source` that names the runtime provider",
   where it said "the runtime provider as its `origin`", since a `source` is now a URI-reference
   that the engine names.
10. **The row source** says that it is not `package-source.v1`; **the row domain of trust** that
    it is not SPIFFE's trust domain; **the row conformance document** that no engine reads it;
    **the row idempotency key** that the IETF draft defines one so. The first three follow the
    subsections; the last cites the draft that N15's subsection cites.
11. **PRV-010** cites Kubernetes' `restartPolicy` beside systemd and Compose, from N20's
    precedents.
12. **The names check** (see above): one pattern of `tools/set_names.py`, a file outside the
    scope this pass was given, changed so that its check could read the ruled name.
13. **The overview** says that a package is pinned by the SHA-256 of its file in `fileSha256`,
    or an OCI image by its digest, where it said "pinned by its digest".

## For the coordinator

Files that other jobs own still carry old names, as the task left them:

- `AGENTS.md`, line 22: "a runtime profile" (N18).
- `docs/AUDIT.md`: "runtime profile" 13 times, which N18's subsection says to keep in the words
  of rulings 13 and 14 and to add the new term to its reading list; `secret-reveal.v1` or
  `reveal-secret.v1` twice; "operation identity" 6 times.
- `docs/README.md` describes the record `proposals/operations/` as "operation identity,
  outcomes", which is that record's own title and may stay.

## Search for the old names after the renames

Each search covers `spec/`, `tools/check_schemas.py` and `docs/OVERVIEW.md`, across line
breaks. What it finds, apart from the words-not-used rows and the lines this pass wrote to
record the renames, is the table under [What was kept](#what-was-kept).

| Old name | Remaining | Where |
| --- | --- | --- |
| `secret-reveal.v1` | 7 | Head comments recording earlier passes (6) and the core README's ruling 42 paragraph (1); the rewritten sentences of the core README and of `secrets.yaml` that say what the names were also hold it |
| `reveal-secret.v1` | 4 | Head comments of the core `operations.yaml` (2) and `access.yaml`, the core README's ruling 42 paragraph; and the same two rewritten sentences |
| `endpoint` as `run.v1`'s output | 4 | Head comments (3) and the core README's ruling 42 paragraph |
| `arguments` as the key of an item or the input of `run.v1` | 3 | Head comments of the core `operations.yaml` and `schemas/manifest.yaml`, and the core README's ruling 42 paragraph |
| `digest` of a `package` item | 0 | |
| acted for the caller, reading instance, `data.instance` | 0 | |
| `observedGeneration` | 0 | |
| origin | 1 | HTTP's origin in IFC-037 |
| operation identity, `operationIdentity` | 1 | The history paragraph of `spec/definitions.md` |
| `io.github.shrav-k.conduit/restrictions` | 0 | The head comment of `implementation.yaml` names the key `restrictions` without the prefix, in round 2's record |
| `io.github.shrav-k.conduit/interface` | 1 | The head comment of `implementation.yaml`, ruling 41's line |
| runtime profile | 0 | |
| trust domain (of a secret) | 0 | The row domain of trust names SPIFFE's trust domain to set it apart |
| its own setting for restarts | 0 | |
