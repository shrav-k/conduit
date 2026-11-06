# Follow-up decisions — log of group C

Written 2026-11-05 by the editor of group C's files, under the rules of
[CONSOLIDATION.md](CONSOLIDATION.md) and the follow-up decisions F1 to F26 of the
[re-audit record](README.md#follow-up-decisions--accepted) (plan-audit ruling 33). The files are
`promises/objects.yaml`, `promises/instantiation.yaml`, `promises/submissions.yaml`,
`promises/templates.yaml`, `promises/graphs.yaml`, `promises/contracts.yaml`,
`promises/schemas.yaml`, `promises/naming.yaml`, `promises/packages.yaml` and
`schema-profile.md`. No other file was edited. The renaming ruled under F3 was already in place
and was not redone.

"Open item n" means item n of "Left undone, and why" in the [first log](log-group-c.md).

## Counts

| File | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| `promises/objects.yaml` | 22 | 6 | 0 | 1 | 23 |
| `promises/instantiation.yaml` | 21 | 5 | 0 | 1 | 22 |
| `promises/submissions.yaml` | 8 | 1 | 0 | 0 | 8 |
| `promises/templates.yaml` | 12 | 1 | 0 | 0 | 12 |
| `promises/graphs.yaml` | 22 | 9 | 0 | 0 | 22 |
| `promises/contracts.yaml` | 7 | 2 | 0 | 1 | 8 |
| `promises/schemas.yaml` | 17 | 5 | 0 | 1 | 18 |
| `promises/naming.yaml` | 26 | 7 | 0 | 0 | 26 |
| `promises/packages.yaml` | 15 | 1 | 0 | 1 | 16 |
| Total | 150 | 37 | 0 | 5 | 155 |

New IDs: OBJ-023, INS-023, CTR-008, SCH-018, PKG-022. No ID was removed or reused.

The validation command of CONSOLIDATION.md passes. It counted 346 promises across `spec/` at
the last run, 155 of them in these nine files; the rest belong to files another editor was
changing at the same time. `python3 tools/check_docs.py` and `python3 tools/set_names.py
--check` report no error. Every promise ID cited in these ten files exists. Every line of the
nine YAML files is under 97 characters; two lines of `naming.yaml` that the renaming had left
longer were re-wrapped.

## objects.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| OBJ-003 | The dot-replacement rule is gone, because it could never apply: a template's name follows OBJ-001 and holds no dot, exported or not. The promise now covers an instance document with no name and the nested instance of a declared Conduit or of a Pool's entry. A component Conduit is no longer named here; the text defers to the naming of a composite's components in `conduits.yaml`. | Open item 2; F25 |
| OBJ-013 | A Conduit's `consumer` is a reference to an Interface, Agent or Resource instance, and its `target` to an Interface, Agent, Resource or Pool instance. The observable notes that each accepts more than one kind and so is written with its kind under OBJ-006. | F20 |
| OBJ-014 | An entry with no placement from a Pool input follows "PLC-002 to PLC-004, as for any instance", where it cited only PLC-002 and PLC-003. | F15 |
| OBJ-020 | Ten kinds: the five families and the five kinds of contract, `Schema` among them. | F21 |
| OBJ-021 | The core package's reference schemas are Schema contracts that package exports. | F21 |
| OBJ-022 | The observable states that the names of an Operation contract's inputs and outputs are outside the rule (CTR-002), and that the keys of a step's `arguments`, of a graph's `outputs` and of the `inputs` a template gives a control are those names. | F17 |
| OBJ-023 | Added, layer `convention`. A registration whose document is rejected, for any cause other than the clash of OBJ-002, is not dispatched and reports invalid. OBJ-002 keeps conflict, and OPS-023 gives the order. | W4; open item 12 |

## instantiation.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| INS-001 | `Schema` joins the kinds that are contracts. | F21 |
| INS-010 | "Registration returns a diagnostic" became "Resolving returns a diagnostic". | F22 |
| INS-011 | The same. | F22 |
| INS-012 | Forbidden destinations now also include a Conduit template's `capability`, and the `from` and the `optional` of any nested instantiation. Accepted destinations now also include, in a Conduit template, `operations` and a place inside a component's `arguments`. "A destination inside" became "is, or is inside". "Registration returns" became "Resolving returns". | F18, F22 |
| INS-022 | "Every check that registration makes" became "every check that resolving makes", and the diagnostic is the one resolving the template would have returned. | F22 |
| INS-023 | Added. A Conduit whose `operations`, as an input's argument or default gives it, holds an operation the capability does not include is rejected when the Conduit is instantiated. | F18 |

## submissions.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| SUB-008 | The engine keeps the contracts as an unpublished package identified by a digest of its documents. A template registered by the submission keeps that digest under NAM-009, so the contracts stay available to it and to its instances. | F24; open item 15 |

## templates.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| TPL-005 | "Registration returns a diagnostic" became "Resolving returns a diagnostic". | F22 |

## graphs.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| GRA-004 | "Registration returns a diagnostic" became "Resolving returns a diagnostic". | F22 |
| GRA-005 | The same. | F22 |
| GRA-006 | The same. | F22 |
| GRA-007 | The same. | F22 |
| GRA-008 | The same. | F22 |
| GRA-010 | The same. | F22 |
| GRA-011 | The same. | F22 |
| GRA-017 | Only a step that has `operation` may have `conduit`. A step that has `runtime` and carries `conduit` is rejected. | F19; open item 4 |
| GRA-024 | A step that carries `conduit` depends on the step GRA-022 gives that declared Conduit, exactly as if the dependency had been stated. | F19; open item 4 |

## contracts.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| CTR-001 | `Schema` is a fifth kind of contract. The header comment says five. | F21 |
| CTR-002 | The name of an input or output may be any non-empty string. An empty name is rejected, and `maxResults` and `max_results` are given as accepted. | F17; open item 1 |
| CTR-008 | Added. A Schema contract's `spec` is a schema under the schema profile, and the schema is the whole of `spec`. | F21; open item 8 |

## schemas.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| SCH-002 | "Installation or registration returns a diagnostic" became "Installation or resolving returns a diagnostic", the form NAM-021 already had. | F22 |
| SCH-003 | The same. | F22 |
| SCH-008 | A `default` on a property of an object inside `items` is accepted. A `default` on `items` itself, and one inside `allOf`, `anyOf` or `oneOf`, stay rejected. | F23; open item 9 |
| SCH-009 | The observable states that an item of a list is an object that is present, so each item takes the defaults of its own properties, and that no item is added to a list. | F23 |
| SCH-015 | A `$ref` that is a qualified reference resolves to a Schema contract that a package exports and stands for that contract's `spec`. | F21 |
| SCH-018 | Added. A JSON file holds one document, or one array each of whose items is a document. | F26; open item 10 |

## naming.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| NAM-001 | The host part of an address is a lowercase DNS name with a dot and no port, with the grammar RFC 1123 gives a host name. | F26; open item 10 |
| NAM-004 | An export's name is at most 63 characters, counted over the whole name. | F5 |
| NAM-009 | A template that refers to a contract of its own unpublished project keeps the digest of the unpublished package of SUB-008, as it keeps the version and digest of any package. | F24 |
| NAM-012 | The manifest key is `fetch`. "Source" is no longer used for where a package comes from. | F22; open item 7 |
| NAM-020 | "The schemas the core package exports" became "the Schema contracts the core package exports". One line re-wrapped. | F21 |
| NAM-021 | "Gives a source for it" became "gives `fetch` for it". One line re-wrapped. | F22 |
| NAM-025 | Level became `should`. | Open item 11 |

## packages.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| PKG-005 | An unpublished package that the template holds under NAM-009 is listed by its digest in the same way. | F24 |
| PKG-022 | Added. A package whose archive is not a gzip-compressed tar file is rejected when it is installed. The digest of PKG-004 is taken over the compressed file as distributed. | F26; open item 10 |

## schema-profile.md

The JSON row cites SCH-013 and states that a JSON file holds one document or one array of
documents (SCH-018). Two rows were added to the table of standards: package archives (tar as
POSIX, IEEE Std 1003.1, defines it, compressed with gzip, RFC 1952; PKG-022) and the host part
of an address (RFC 1123, section 2.1; NAM-001). The paragraph on Kubernetes defaulting states
decision F23 and cites SCH-008 and SCH-009. The paragraph on `$ref` says a qualified reference
is to a Schema contract, the fifth kind of contract (SCH-015, CTR-008). Ruling 33 was added to
the line of rulings.

## The five confirmed readings

Nothing in these files contradicts them, and nothing was changed for them. PKG-019 and PKG-020
speak of `supports` in general terms; the reading that only a filter needs it is CON-022's.

## Choices the coordinator should check

1. **OBJ-023 has the layer `convention`.** It states how a request ends, as TPL-011, LIF-012
   and REV-011 do. OBJ-002, which states conflict for the same operation, has the layer
   `language` and was left as it is.
2. **OBJ-013 states a consequence of F20.** Since `consumer` and `target` each accept more than
   one kind, OBJ-006 already requires each to be written as `{kind, name}`; a bare name such as
   `consumer: root` is rejected. The observable now says so. OBJ-013 states the kinds that are
   accepted and leaves the rejection of any other kind to `conduits.yaml`.
3. **INS-012 says "is, or is inside"** where it said "inside". `capability` is a single value,
   so "inside `capability`" would have forbidden nothing, and the same was already true of
   `type`.
4. **F18's check is a new promise, INS-023,** and not part of INS-012. CON-017 makes the check
   on a list the template writes; INS-023 makes it on the list an argument or a default gives.
5. **F19's dependency is in GRA-024,** not a new promise. Its observable speaks of the setup
   graph only, because GRA-022 gives a declared Conduit a step only there. See item 4 below.
6. **SCH-015 also rejects a qualified reference to an export that is not a Schema contract,**
   in the way it rejects any other form of `$ref`. The ruling says such a reference "resolves
   to an exported `Schema`"; the rejection is my reading of what follows when it does not.
7. **PKG-022 is written in the form of PKG-001:** the package is rejected when it is installed.
   The ruling states only what the archive is.
8. **PKG-005 and OBJ-021 were touched lightly** so that F24 and F21 read the same in every
   place that lists what an instance holds or names the reference schemas.
9. **F22 was applied as a change of subject only.** Every "Registration returns a diagnostic"
   became "Resolving returns a diagnostic". "Installation returns a diagnostic" was left, since
   the ruling does not speak of installing. Sentences about what exists afterwards, such as
   OBJ-008's and PKG-006's "the template exists after registration", already used
   "registration" for the stored effect and are unchanged.

## Left undone, and why

1. **How the digest of an unpublished package is computed** (F24). PKG-004 defines a digest
   over an archive file, and an unpublished project has none. Which documents are hashed, and
   in what form, is not ruled. SUB-008 and NAM-009 say only "a digest of its documents". Two
   engines would report different digests for the same project. Needs a decision.
2. **What the value of `fetch` is** (F22): a URL, a path or another address. Left with the
   manifest's item keys to decision G1's deliverable.
3. **Whether `$ref` may use the `#name` form of NAM-022** to reach a Schema contract of the
   same project or package (F21). SCH-015 allows `#` followed by a JSON Pointer, or a qualified
   reference, and NAM-018's list of selections does not include a schema. A project's own
   Schema contract can then be reached by a `$ref` only once the project is published. Needs a
   decision.
4. **`conduit` on a step of a graph other than setup** (F19). GRA-022 makes a declared Conduit
   a step of the setup graph only. In a graph that implements another operation, or in a
   `retire` graph, there is no step to depend on. The text states the dependency as ruled and
   an observable for the setup graph; elsewhere the request is decided under ACC-018 as the
   Conduit then stands. Confirm that reading, or rule otherwise.
5. **Destinations that INS-012 still neither forbids nor accepts** (F18): an item of
   `operations`, such as `/operations/0`; a whole nested instantiation, such as
   `/conduits/checkout`, which contains its `from`; the whole of `config` or of an `arguments`
   map; `/requirements/<name>/requirement`; and anything under `/inputs`. The ruling lists
   places that may and may not be reached, and does not say the accepted list is closed.
6. **GRA-024's check that a step's operation is one its declared Conduit lists** can no longer
   always be made when the template is registered, since F18 lets an input fill a Conduit
   template's `operations`. INS-022 makes the checks again at instantiation, which appears to
   cover it. Left as it is.
7. **Which reason the installation of a rejected package reports.** Decision W4 speaks of a
   rejected document and its registration. Decision F14 makes installing an operation
   authorized against `management`, so it ends in one of the seven ways, but nothing says
   invalid. OBJ-023 covers registration only.
8. **"Registration returns a diagnostic" remains in files of the other groups:** POOL-005,
   POOL-011, CON-020, PLC-005, IMP-004 and IMP-005. Under F22 each would read "Resolving
   returns a diagnostic".
9. **A component Conduit's name and the 63-character limit** (F25). OBJ-003 shortens the
   generated name to 63 characters before a hyphen and the component's name are added. Whether
   the result may be longer is for the promise in `conduits.yaml`.
10. **Details of the host grammar** (F26). Citing RFC 1123 brings its limits on length and its
    rule that a label neither starts nor ends with a hyphen. A trailing dot, a host that is a
    dotted number such as `127.0.0.1`, and internationalized names are not addressed.
11. **What a package's archive holds** (F26): the layout inside the tar file and where the
    manifest lies. Left with the manifest's file name to decision G1's deliverable.
12. **Still open from the first log and not in this brief:** item 13 (key names left to
    decision G1), item 14 beyond what F25 settles, item 17 (promises not moved, splits not
    applied) and item 18 ("Owner rulings" in the header comments).

## For `definitions.md` and `README.md`

Rows that need to follow: family and kind (ten kinds); contract (five kinds, `Schema` among
them); export (a Schema contract); core package (its reference schemas are Schema contracts);
manifest (`fetch`); source (the key of a value reference only); address (the host grammar);
digest and project (the unpublished package and its digest); document (a JSON file holds one
document or one array of documents); reference (the kinds `consumer` and `target` accept);
name (an Operation's input and output names are any non-empty string; an export name is at
most 63 characters); and new rows for resolving and registration (F22). The row for "rejected"
can cite OBJ-023 and OBJ-002.

`spec/README.md` says "The four kinds of contract" against `contracts.yaml`. Promise counts:
objects 23, instantiation 22, submissions 8, templates 12, graphs 22, contracts 8, schemas 18,
naming 26, packages 16.
