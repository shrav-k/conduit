# Ruling 41: log of group G6, the language

Written by the editor of group G6 on 2026-11-06, applying section 3, "G6 language", of the
[work order](PLAN.md) to `spec/promises/objects.yaml`, `spec/promises/contracts.yaml`,
`spec/promises/schemas.yaml`, `spec/promises/naming.yaml`, `spec/promises/packages.yaml`,
`spec/promises/manifest.yaml`, `spec/promises/submissions.yaml`,
`spec/promises/instantiation.yaml` and `spec/schema-profile.md`. No other file was changed. The
names that the review uses, such as `secret-reveal.v1` and `reveal-secret.v1`, are kept as they
are; the vocabulary audit that [ruling 41](README.md) orders renames them.

## Counts

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `objects.yaml` | 28 | 4 | 10 | 0 | 24 |
| `contracts.yaml` | 11 | 2 | 4 | 1 | 10 |
| `schemas.yaml` | 21 | 0 | 3 | 0 | 21 |
| `naming.yaml` | 28 | 0 | 10 (1 beyond the plan) | 0 | 28 |
| `packages.yaml` | 22 | 2 | 8 (1 beyond the plan) | 1 | 21 |
| `manifest.yaml` | 14 | 1 | 9 | 3 | 16 |
| `submissions.yaml` | 10 | 0 | 4 | 0 | 10 |
| `instantiation.yaml` | 24 | 1 | 3 | 0 | 23 |
| All | 158 | 10 | 51 (49 planned, 2 beyond) | 5 | 153 |

The counts after match those the plan gives `spec/README.md`. The validation of
[CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) passes over the whole specification
(426 promises). Every promise of the eight files has exactly the seven fields, folded `>-` text
and no line longer than 96 characters, and every ID that one of them cites exists.

## Promises removed

| ID | Decision | Where its content went |
| --- | --- | --- |
| OBJ-005 | 10 | Nothing replaces it: no object is renamed, so its ID never changes on a rename. OBJ-011 states that a name is fixed. |
| OBJ-026 | 10 | Nothing replaces it: `rename.v1` goes. |
| OBJ-027 | 10 | Nothing replaces it. |
| OBJ-028 | 10 | Nothing replaces it. |
| CTR-010 | 7 | Nothing replaces it: no requirement has constituents. Several requirements are applied side by side, as CTR-005 now says. |
| CTR-011 | 7 | Nothing replaces it: with no constituents, `bound.v1` can be hidden nowhere. |
| PKG-013 | 9 | Nothing replaces it: a package exports only its own documents (MAN-007). |
| PKG-014 | 9 | Nothing replaces it. |
| MAN-008 | 9 | Nothing replaces it: MAN-007 lists names only. |
| INS-022 | 12 | Nothing replaces it: a registration is rejected while a package its manifest lists is not installed (PKG-006), so no check is left to instantiation. |

## Promises rewritten

| ID | Decision | What changed |
| --- | --- | --- |
| OBJ-003 | 5, 30 | Observable: the sentence on component Conduits goes. |
| OBJ-010 | 10 | `when`: the name now belongs to another object, or to none, because the object it resolved to no longer exists (no rename). Observable: "whatever the objects are now named" goes. |
| OBJ-011 | 10 | `when`: an object exists. `then`: no document and no operation changes its name, fixed once made; no document rewrites an instance's definition. Observable: the OBJ-026 pointer goes; another name means another object. |
| OBJ-013 | 31b, 8, 8d | `then`: the standard arguments add `runtime` for an Agent and a Resource (PRV-006); `placement` left out is chosen under PLC-002; the location sentence goes. Observable: `runtime` under `arguments`, or on a Pool, is rejected. |
| OBJ-014 | 5, 31b | The component goes; an entry that carries `owner`, `placement` or `runtime` is rejected. |
| OBJ-015 | 16a, 1, 2 | `when`: a template's input, a Requirement's input, a declared Conduit or an entry. `then`: an Operation contract's input or output is optional where `required` does not list it. Observable: steps, graph outputs and runtimes go. |
| OBJ-016 | 1, 25d | The value reference goes; the pointers of an item's `environment`, `arguments` and `headers` join, rooted at the configuration. |
| OBJ-019 | 1, 3a, 7, 25, 8a | Exemptions: the value binding, `steps`, `outputs`, the `object` binding and the constituent go; the inside of `package` and `remote`, the keys of `environment`, `arguments` and `headers`, and the labels of a label selector join. Observable: `runtimes`, `setup` and `retire` replace the policy example. |
| OBJ-021 | 1, 15, S4.1 | Observable: the value reference and `settings` go; `config` under IFC-006 joins; `secret-reference.v1` is a reference schema under the narrower rule of SEC-001. |
| OBJ-022 | 1, 2, 5, 7, 8a, 20 | `when`: runtimes, components, a Type's `placement`, constituents and steps go. Observable: `propertyNames` states the rule; label keys are not names. |
| CTR-002 | 16a, 16b | `inputSchema` and `outputSchema` replace `inputs` and `outputs`; a handle is a property whose schema is `handle.v1`; the three hints join. |
| CTR-004 | 2, 8a, 20, P12 | `placement` is an optional label selector; `runtimes` and `restoresRuntimes` go; PRV-001 and `type-spec.v1` are cited. |
| CTR-005 | 3a, 7, 8a, 14, C20, P12 | A Requirement holds `inputs` only; it is applied only to a Conduit; a core requirement is applied by the engine and never sent, any other is a filter. `appliesTo`, `narrows`, `obliges`, `constituents` and placement go. |
| CTR-006 | 15, 16a | `inputSchema` requires `revision`; no output `settings`; REV-007 and REV-015 are cited. |
| SCH-008 | 20 | A `default` is also rejected under `not`, `if`, `then`, `else` and `dependentSchemas`. |
| SCH-009 | 20, 3a, 1 | Observable: the eight keywords; the policy and graph clauses go. |
| SCH-019 | 16a | Observable: an Operation input's `default` is its property's `default` in `inputSchema`. |
| NAM-008 | 12 | Beyond the plan: `when` covers only a package that is installed; a listed package that is not installed is PKG-006. Without this NAM-008 would accept what PKG-006 now rejects. |
| NAM-009 | 11, P11 | A submitted project's item keeps `address`, `version` where stated, and the SUB-008 digest. |
| NAM-012 | 21, 12 | `then`: `https` and `file` on every engine, other schemes by a package-source provider (PRV-017), a cache or a mirror, no credential. Observable: no `fetch` and an unaccepted scheme report invalid (PKG-023); a missing package rejects the registration (PKG-006). |
| NAM-018 | 1, 2, 7 | The graph written in place, the constituent, the step's `operation` and a runtime's `start` and `stop` go. |
| NAM-019 | 3a, 8a | `when`: applied to a Conduit only; the policy sentence goes. |
| NAM-020 | 8d | The example is `github.com/shrav-k/conduit/core#interface.v1`. |
| NAM-021 | 25, 11 | `url` and `graph` become `package` and `remote`. Also, beyond the plan's line: the sentence "`address` and `version` ... change nothing ... the one address that such a manifest may not state" is replaced, since decision 11 and MAN-016 make it untrue. |
| NAM-022 | 11 | A submitted project's contracts are `address#name` outside its documents; the replacing document reaches "the contracts and implementations of the submitted project" that the template holds. |
| NAM-026 | 1 | Observable: the step sentence goes. |
| NAM-028 | 9, 12, 1 | `when` and `then` drop the re-export and the manifest; observable drops the re-export example, the step and the deferral, and cites PKG-006 and PKG-028. |
| PKG-005 | 11 | Observable: a submitted project is listed by address, version where stated and digest. |
| PKG-006 | 12 | A registration whose manifest lists a package not installed at that version and digest is rejected, with a diagnostic naming each; INS-022 goes. |
| PKG-017 | 11 | Beyond the plan: its observable said that a manifest states both or neither of `address` and `version`, which MAN-004 no longer says. |
| PKG-019 | 25, 8a, 1, 7 | Exactly one of `command`, `package` and `remote`; `placement` is a label selector; `digest`, `environment`, `arguments`, `headers`; graphs and compositions go. |
| PKG-020 | 25, 1 | Plain is reached by `command`, `package` or `remote`; the graph clauses go; a plain server learns its configuration only through `environment`, `arguments` or `headers`. |
| PKG-023 | 21, 9 | `when` adds no `fetch` for a package not installed, an unaccepted scheme, and user information in `fetch`; PKG-014 and CTR-011 leave the list, PKG-028 joins it. |
| PKG-024 | 21 | Observable: DL7 → PRV-017. |
| PKG-028 | 12 | The installation is rejected, with a diagnostic naming each missing package; it reports invalid under PKG-023. |
| MAN-001 | C14 | `conduit.yaml`, `conduit.yml` or `conduit.json`. |
| MAN-002 | C14 | More than one of the three names is rejected. |
| MAN-004 | 11 | `version` only beside `address`; a submitted project states `address` where MAN-015 requires it; MAN-016 joins the forbidden addresses. |
| MAN-006 | 21, 20 | Checked by `uri-reference`; a relative one resolves to `file:`; no user information; DL7 → NAM-012 and PRV-017. |
| MAN-007 | 9 | Each item is the name of a document of the package. |
| MAN-009 | 25, 1 | Exactly one of `command`, `package` and `remote`; the optional keys add `digest`, `environment`, `arguments` and `headers`. |
| MAN-010 | 25, 8a, P15 | `package` (server.json 2025-12-11) with `digest`; `remote` limited to `streamable-http`; `environment`, `arguments`, `headers`; `placement` a label selector; `url`, `graph`, GRA-014 and DL12 go. |
| MAN-011 | C14 | Documents end in `.yaml`, `.yml` or `.json`. |
| MAN-012 | 25, C20 | `engine` states none of `command`, `package` and `remote` and no `supports`; it applies the core requirements itself; it implements no provider operation and not `reveal-secret.v1`. |
| SUB-005 | C14 | `.yaml`, `.yml` or `.json`. |
| SUB-008 | 11, 12, P11 | `#name` inside the project, `address#name` elsewhere; the digest identifies each submission's contracts, computed as before; the missing-package and `<digest>#<name>` sentences go. |
| SUB-009 | 11 | "the unpublished package" → the contracts and implementations of the submitted project that the template holds. |
| SUB-010 | C14, 12 | A missing package rejects the registration (PKG-006); the three manifest names; "unpublished package" goes. |
| INS-012 | 5, 31b, P14 | The composite clause goes; an entry's `runtime` joins; `/components/...`, `runtimes`, `setup` and `retire` leave the examples. |
| INS-016 | 5 | The composite sentence goes. |
| INS-025 | 9 | The PKG-013 sentence goes. |

`spec/schema-profile.md` was changed as the plan lists: the head paragraph names ruling 41;
"Graphs and schemas" became "Schemas"; the keyword list is the applicator vocabulary and the
validation vocabulary less `multipleOf`, with the excluded keywords listed; `uri-reference` is a
format; the `default` paragraphs name the conditional keywords; the `fetch` row names `https:`
and `file:` (RFC 8089); rows were added for a `remote` URL, server.json, SARIF, CloudEvents and
`metav1.Condition`; the step sentence became one on the order of a Pool's entries.

## Promises added

| ID | Layer, level | Decision | Rule |
| --- | --- | --- | --- |
| CTR-012 | language, must | 16a | An Operation contract whose `inputSchema` or `outputSchema` has at its root a keyword other than `type`, `properties`, `required` and `additionalProperties`, or no `type: object`, is rejected. |
| PKG-029 | convention, must | 11, P10 | A request to install a package at the address that a submitted project's manifest states is not dispatched and reports conflict. |
| MAN-015 | language, must | 11 | A submitted project that holds a contract or lists an implementation, with no manifest or no `address` in it, is rejected. |
| MAN-016 | language, must | 11 | A submitted project whose manifest states the address of an installed package is rejected. |
| MAN-017 | language, must | 25d | A template that selects an item whose `environment`, `arguments` or `headers` points at a place its type's schema does not require, with every member on the way required, is rejected. |

## Choices made where the plan left wording open

1. OBJ-010: "or to none" is kept, now explained by "because the object it resolved to no longer
   exists", and "whatever the objects are now named" is cut, since names no longer change.
2. OBJ-011: the observable says that an object is known by another name only where another
   object is made under it, as decision 10's text says, citing Kubernetes `metadata.name`.
3. OBJ-013: the example "`runtime` on a Pool" is added, matching the planned example
   `OBJ-013-runtime-on-a-pool`.
4. OBJ-015: the observable says that a property of `inputSchema` that states a `default` may be
   left out even where `required` lists it, because OPS-026 fills it before the request is
   checked against `inputSchema`; and that `optional: true` inside a schema is outside the
   profile (SCH-002).
5. OBJ-019: the exemption "a label that keys a member of a label selector under `placement`" is
   added. The plan does not list it, but labels are not names (OBJ-022), so without it OBJ-019
   would reject every label selector. `setup` and `retire` join `runtimes` in the example, as
   TPL-001 has a template that writes them rejected under OBJ-019.
6. CTR-002: an empty property name is still rejected ("any non-empty string", as the plan
   says), and `inputs` and `outputs` are named as keys that are now rejected.
7. CTR-005: `schema` is named among the removed keys, since a requirement on placement, which
   stated one, goes; the two core requirements are named with their promises.
8. CTR-012: `type: object` is required at the root, not only allowed, because the Model Context
   Protocol's tool definition requires it. The plan's "anything but `type: object`, ..." was read
   this way.
9. NAM-012: `when` drops "that differs from the item's address", since an address is no longer a
   place to fetch from (decision 21). `then` says every engine "accepts" `https` and `file`, and
   points at PRV-017 for which of the engine and a provider fetches, because PRV-017 lets a
   provider claim `https` too. A credential is "user information", the part of a URI where
   RFC 3986 places one; MAN-006 and PKG-023 use the same words.
10. NAM-022 and SUB-009: the plan's "the project contracts that the template being replaced
    holds" is written "the contracts and implementations of the submitted project that the
    template being replaced holds", in the plan's term "the contracts of a submitted project",
    since `#name` reaches implementations too.
11. PKG-006 and PKG-028: "not installed at the version and digest that the manifest gives", as
    decision 12 words it. NAM-008 was narrowed to match (see the table).
12. PKG-019: "A `package` item also states `digest`" and a pointer to PRV-012 are added, as
    MAN-010 requires the digest.
13. PKG-023: a `fetch` whose scheme nobody accepts reports invalid with no qualifier, as a
    relative one does; a request for a package already installed needs no `fetch`, since
    PKG-025 and PKG-027 fetch nothing.
14. MAN-010: which keys go with which way of reaching an item (`environment` with `command` or
    `package`, `arguments` with `package`, `headers` with `remote`, `digest` with `package`) is
    stated in its observable, as PKG-019's plan line gives it. For a `package` item,
    `environment` names only variables the Package declares (P15); for a `command` item, any.
15. MAN-012: "may state `placement`" is kept for `engine`, since the plan does not drop it and the
    core manifest writes `placement: {}`.
16. MAN-015: a project with contracts and no manifest at all is rejected too.
17. MAN-017: a type that states no `schema` requires nothing, so any pointer is rejected.
18. SUB-008: the digest's computation is kept word for word, including "left out when the
    project has none", although MAN-015 now makes that case unreachable for a project with
    contracts, so that the computation does not change.
19. `spec/schema-profile.md`: the whole keyword list stays in the first paragraph of its section,
    because `tools/check_schemas.py` reads only that paragraph; `$anchor`'s exclusion gives the
    review's reason. The Digests, Canonical form and Pointers rows were reworded too, since
    "unpublished package" and "value reference" are terms that go. In `manifest.yaml`'s head
    comment, the list of practices drops the re-export and adds `.yml` and server.json.

Terms used for the definitions: "label selector", "package-source provider", "the contracts of
a submitted project", `address#name`, "user information" (RFC 3986's term, for what `fetch` may
not hold), and "the digest that identifies a submission's contracts", which replaces "the
digest of an unpublished package".

## For the integrator: what cites a removed ID

No promise outside these files cites any of the ten removed IDs any longer. The citations that
remain are in files of the core-and-schemas editor and the docs editor:

| Removed | Still cited by |
| --- | --- |
| OBJ-005 | `spec/core/results.yaml`, `spec/core/README.md`, `spec/schemas/values.yaml`, `spec/definitions.md` |
| OBJ-026 | `spec/core/operations.yaml`, `spec/core/README.md` (3), `spec/definitions.md` (2), `tools/check_schemas.py` (3) |
| OBJ-027, OBJ-028 | `spec/definitions.md` |
| CTR-010 | `spec/core/requirements.yaml`, `spec/core/README.md`, `spec/schemas/contracts.yaml`, `spec/schemas/README.md` (3), the two `CTR-010-*` examples in `spec/schemas/examples/invalid/`, `spec/schemas/examples/valid/files-package/contracts.yaml`, `spec/definitions.md` (4), `tools/check_schemas.py` (2) |
| CTR-011 | `spec/core/requirements.yaml` (2), `spec/core/README.md` (2), `spec/schemas/contracts.yaml`, `spec/schemas/README.md`, `spec/definitions.md`, `tools/check_schemas.py` (3) |
| PKG-013 | `spec/core/README.md`, `spec/definitions.md` (2) |
| PKG-014 | `spec/schemas/manifest.yaml`, `spec/schemas/README.md` |
| MAN-008 | `spec/schemas/manifest.yaml`, `spec/schemas/examples/valid/files-package/conduit.yaml`, `spec/definitions.md` (2) |
| INS-022 | none outside the head comment of `spec/promises/instantiation.yaml` |

`tools/check_schemas.py` now reads 41 keywords and 6 formats from `spec/schema-profile.md`, and
reports that `schema.v1` does not list exactly those (it lacks `contains`, `dependentRequired`,
`dependentSchemas`, `else`, `if`, `maxContains`, `maxProperties`, `minContains`,
`minProperties`, `not`, `patternProperties`, `prefixItems`, `propertyNames`, `then` and the
format `uri-reference`). That is section 4's change to `schema.v1`, for the core-and-schemas
editor. Its other errors at the time of writing were rules named for removed promises.

## Still needs a decision

1. Replacing a template while a package is missing. PKG-006 now rejects a registration whose
   manifest lists a package that is not installed, and INS-022, which caught the other cases at
   instantiation, is gone. A request for `replace.v1` is not a registration, and the manifest it
   may hand over (SUB-009) is covered by no promise in that case. Decision 12 reads as covering
   it too, but neither the review nor the plan says so. Left as it is.
2. How long a submitted project's address is held. PKG-029 refuses to install a package at the
   address of "a submitted project", and MAN-016 rejects a project at an installed package's
   address. Whether that address counts for as long as any template or instance holds the
   project's contracts under NAM-009 and PKG-005, or for ever once submitted, is not said. Nor is
   whether two different projects may state one address, which would make their contracts share
   `address#name` on the wire and in bounds, as two versions of one package do.
3. Values that are not strings. MAN-010 and MAN-017 bind a variable, an argument or a header to a
   place in the configuration, and PRV-007 and IFC-041 pass the value. What a number, a map or a
   list at that place becomes (rejected unless the place's schema is a string, or written as
   JSON) is not said.
4. Positional arguments of a server.json Package. `arguments` maps "the name of an argument that
   the Package declares", but in server.json only a named argument has `name`; a positional one
   has `valueHint`. Which key names a positional argument is not said.
