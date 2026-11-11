# Ruling 41 — log of group G7 (secrets)

Written 2026-11-06 by the editor of group G7, under the [work order](PLAN.md#g7-secrets) and
the rules of [CONSOLIDATION.md](../../../CONTRIBUTING.md#writing-promises). G7's one
file is the new [spec/promises/secrets.yaml](../../../spec/promises/secrets.yaml). No other
file under `spec/` was edited. The names `secret-reveal.v1`, `reveal-secret.v1` and
`secret-reference.v1` are the review's, kept as the work order says; the vocabulary audit
that [ruling 41](README.md) orders renames them.

## Counts

| File | Before | Removed | Rewritten | Added | After |
| --- | --- | --- | --- | --- | --- |
| `spec/promises/secrets.yaml` (new) | 0 | 0 | 0 | 20 | 20 |

By layer: `language` 5 (SEC-001, SEC-002, SEC-003, SEC-005, SEC-009), `convention` 13
(SEC-004, SEC-007, SEC-008, SEC-010 to SEC-018, SEC-020), `interface` 2 (SEC-006, SEC-019).
By level: `must` 13, `must not` 7.

The validation command of https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md passes: 425 promises at the last run, 20 of them in
this file; the rest are in files other editors were changing at the same time. Every line of
the file is at most 96 characters. `python3 tools/set_names.py --check` reports nothing in this
file (its errors at the time were in files of G3, G4 and G5, which were mid-edit). Every
promise has exactly the seven fields, folded `>-` text indented four spaces, and each of the
four promises that describe a request that is not dispatched names one reason (SEC-007 and
SEC-008 refused; SEC-012 and SEC-016 unavailable). `python3 tools/check_docs.py` reports no
error.

## Promises added

| ID | Layer, level | The rule its `then` states | Decisions | From the proposal's draft |
| --- | --- | --- | --- | --- |
| SEC-001 | language, must | `secret-reference.v1` governs a place only as its own schema, as `items`, or beside `{type: null}` alone under `anyOf` or `oneOf`; any other use rejects the schema, and any value there other than a reference to a Resource rejects the document. | S4.1, 2b, 20 | SEC-1 |
| SEC-002 | language, must | A secret reference that resolves to a Resource whose type does not offer `secret-reveal.v1` rejects the document. | S4.1, 19 | SEC-2 |
| SEC-003 | language, must not | A diagnostic about a value at such a place holds the value or any part of it. | S4.1 | SEC-3 |
| SEC-004 | convention, must not | A record holds a value at such a place, of a request's inputs, in any form but the resolved form. | S4.1 | SEC-22 |
| SEC-005 | language, must | A Type that offers `secret-reveal.v1` lists no control; one that lists one is rejected. | S4.2, 19 | none |
| SEC-006 | interface, must | An implementation of `reveal-secret.v1` takes the value only from the store its secret Resource's locator names, never from the configuration or another store, and answers failed with no value where it cannot read it. | S4.3, 1b | SEC-12 |
| SEC-007 | convention, must | A request for `reveal-secret.v1` that an instance sends, on its own behalf or onward, is refused, whatever Conduit lists it, the root Interface included. | 4b | none |
| SEC-008 | convention, must | A request to open a Conduit that lists `reveal-secret.v1` is dispatched only where the root Interface sent it on its own behalf and the consumer is a Resource or an Agent; otherwise refused. | S4.4 | SEC-7 |
| SEC-009 | language, must | A Conduit that lists `reveal-secret.v1` and whose consumer is an Interface is not instantiated. | S4.4 | SEC-6 |
| SEC-010 | convention, must | Before each run, the engine reads each secret that the item's `environment` or `arguments` points to, through the instance's own open Conduit to it, and passes the values in the run's `secrets` in place of the resolved form. | 4a, 2c, 25d, 13b, S4.5 | SEC-16 |
| SEC-011 | convention, must | A read belongs to the setup or the retirement within which the run or the connection is asked for, has its caller, and is decided by the instance's own Conduit and its filters alone, carrying none of that invocation's restrictions. | 4c | SEC-16 |
| SEC-012 | convention, must | Where a read does not succeed, no run is asked for and no connection made, and a request that was to be dispatched to the implementation reports unavailable. | S4.5 | SEC-16 |
| SEC-013 | convention, must | Before connecting to a `remote` item within a setup, the engine reads each secret its `headers` points to, sends each only as its header to the item's URL, and holds it only for that connection. | 25c | none |
| SEC-014 | convention, must | A value passed for a run is fixed for that run's life: no further read, a provider's restart reuses it, closing the Conduit recalls nothing. | S4.5, 2c | none |
| SEC-015 | convention, must not | The record of a read holds the value or any part of it. | S4.6 | SEC-8 |
| SEC-016 | convention, must | A read whose record cannot be kept is not dispatched, and reports unavailable. | S4.6 | SEC-19 |
| SEC-017 | convention, must not | The engine keeps a value it read to stand in for a later read. | S4.7 | SEC-10 |
| SEC-018 | convention, must not | The engine puts a value it read anywhere but the `secrets` of its run or the header of SEC-013. | S4.7 | SEC-10 |
| SEC-019 | interface, must not | An implementation that carries Conduit's keys puts a value it was given into a result, a diagnostic, or the `arguments` or `_meta` of a request to the engine. | S4.8 | SEC-11 |
| SEC-020 | convention, must not | The engine serves `reveal-secret.v1` itself, or passes to a run a value taken from its own environment. | 13a | none |

The head comment says what the file is; rulings 40 and 41 with the paths of their records, the
principles review and the secrets proposal; the standards followed; that the IDs are new, with
the map from the proposal's drafts that the work order gives; that the names are not final;
that no store is built into the engine; the limit of the guarantee, as a note pointing to
`spec/README.md`; what moves into each store package's contract; the ruling 41 line in the form
of rule 7 ("added SEC-001 to SEC-020; it removed and changed none"); and "Draft: no vector
exists yet."

## Choices made

Where the work order leaves wording open, these are the readings taken. None adds a rule that
the review or the work order does not state.

| # | Promise | Choice | Why |
| --- | --- | --- | --- |
| C1 | SEC-001 | "Under an applicator beside any branch other than `{type: null}`" is read as: `secret-reference.v1` may stand as the schema of a place itself (so under `properties`, `patternProperties`, `additionalProperties` or `prefixItems`), as `items`, or as a branch of `anyOf` or `oneOf` whose every other branch is `{type: null}`. Under `allOf`, `not`, `if`, `then`, `else`, `dependentSchemas` or `contains`, or beside any other branch, the schema is rejected. | The review's table says that once decision 20 admits `if`, `not` and the other applicators, "the rule covers them too". `contains` is listed because a list under it may hold literals beside one reference. |
| C2 | SEC-001 | A bare name is rejected at such a place, as a literal is; a reference to a Pool's entry and an entry reference are accepted. | The work order's export: "a reference schema of the kind Resource, no bare name, accepting the resolved form"; the draft SEC-1. |
| C3 | SEC-004 | The `when` covers any record of a request whose kept inputs hold a value at such a place, not only the inputs of an operation whose `inputSchema` names the schema (the draft SEC-22's scope). | The work order states the rule without that limit, and S4.1 says no record holds a literal. The core package's types keep no inputs, so the case arises only for a type whose `records` keeps inputs. See D5. |
| C4 | SEC-006 | The observable says a plain implementation of `reveal-secret.v1` learns its locator through its item's `environment` or `arguments` (MAN-010), since IFC-007 sends it no keys. | It states what follows from 1b and 25d, and adds no rule. |
| C5 | SEC-007 | Written as a request that "an instance sends: an actor on its own behalf, or any instance as an onward request", so that a Resource's onward request for the operation is refused too. | 4b: "Only the engine's read under 4a is dispatched." The work order's words "an actor's request ... or as an onward request" read the same where an actor's request means one whose caller is an actor. |
| C6 | SEC-008 | "The actor is the root Interface" is written "the root Interface sent it on its own behalf": an onward request that serves the root Interface is refused, and the part of a setup that opens such a declared Conduit is the root Interface's request only where it sent the setup (ACC-005, POOL-015). | S4.4 ("a Conduit that only the root Interface opens") and the draft SEC-7's observable. |
| C7 | SEC-009 | Kept the work order's "is not instantiated", in CON-021's form, and not the draft SEC-6's "the template is rejected". | The work order fixes the rule. |
| C8 | SEC-010 | The read goes through exactly one open Conduit of the instance to the secret Resource that lists `reveal-secret.v1`; with none, or more than one, it reports refused. | The draft SEC-16's observable, which the review's table keeps under the proposal's line 8a; with two, choosing one would bypass the other's filters. |
| C9 | SEC-010 | The value read is passed in `secrets` "in place of the resolved form that the configuration holds at that place"; a place that holds null under `{type: null}` reads nothing. | PRV-007's "the values of SEC-010 apart in `secrets`"; null is not a reference under OBJ-021. See D4. |
| C10 | SEC-011 | Where the instance is set up as a part of a Pool's setup (POOL-013), the read belongs to that setup. | 4c: the read belongs to the request that asked for the run. |
| C11 | SEC-012 | The part of the setup or the retirement that would ask for the run or make the connection reports unavailable, and no later part is begun. | PRV-007 has the part report unavailable where the runtime provider is not ready; TPL-012 begins a part only after the ones before it succeeded. |
| C12 | SEC-013 | "Sends the value only to that item's URL" is taken to exclude an HTTP redirect to another URL. | A redirect is a request to another URL. |
| C13 | SEC-015 | The record names, as REC-002's "instance that acted for the caller", the instance whose Conduit the read went through. | REC-002's fields, with the caller of SEC-011. |
| C14 | SEC-018 | The observable says the record of the provider call to `run.v1` holds no value of `secrets`, whatever the runtime provider's type states under `records`. | PRV-005 records every provider call, and S4.7 puts the value in no record. |
| C15 | SEC-020 | "Gives a run any variable of its own environment" is written as what the engine passes to a run in `environment`, `arguments` or `secrets`, matching PRV-007's observable; what else a process receives is the provider's. | The review's table, line 18: "what else a run receives is the provider's (2c, 13b)". See D3. |
| C16 | All | "Store" means the system outside the engine that a locator names (a keychain, a vault, a file); the object is always "secret Resource"; the plan's term "secret store" is not used in the promises. | See the terms below: the plan's rows for secret store and secret Resource name one object twice. |

## Terms, for the definitions

Used as section 1 of the work order defines them: secret Resource (SEC-002, SEC-005), secret
reference (SEC-001, SEC-002), read (SEC-010 to SEC-018), run, runtime provider, provider call
(SEC-010, SEC-018), part (SEC-012), retire hook (SEC-011), caller (SEC-011). For the docs
editor and the vocabulary audit:

- **secret store.** The work order defines it as a provider whose type offers
  `secret-reveal.v1`, which is also its definition of secret Resource. The promises use "store"
  for the system outside the engine that a secret Resource's locator names, and "secret
  Resource" for the object. Suggested rows: "store (of a secret): the system outside the
  engine that holds a secret's value, which a secret Resource's locator names, such as a
  keychain, a vault or a file; each kind is its own Type (SEC-005, SEC-006)", and either drop
  "secret store" or define it as that kind of Type.
- **locator.** Not in the work order's terms. Used in SEC-005, SEC-006 and the head comment:
  the configuration of a secret Resource, which names where its store holds the value; its form
  is the `schema` of the secret Resource's type (19, S4.2).
- **connection** (to a `remote` item). Used in SEC-011 to SEC-013, SEC-017 and SEC-018 as
  IFC-041 uses it: the engine's connection to a `remote` item to serve one instance, made as a
  part of the instance's setup.
- **a pointer leads to a place.** How the promises say that an item's `environment`,
  `arguments` or `headers` binds a variable, an argument or a header to a place in the
  instance's configuration (MAN-010); the review and the work order say "binds".

## For the integrator

G7 removed no promise, so nothing anywhere cites a removed ID of this file.

IDs that `secrets.yaml` cites in other groups' files, which must exist after their work, with
what its observables say of them:

| Group | IDs | What `secrets.yaml` relies on |
| --- | --- | --- |
| G2 | PRV-001, PRV-005, PRV-007, PRV-010, PRV-011, POOL-013, POOL-015 | `providers.yaml` did not exist when this log was written. PRV-001: a provider Type is of the family Resource and offers no other provider capability. PRV-005: a provider call is not a request and is recorded. PRV-007: the run is asked for with `secrets`, and the part reports unavailable where the runtime provider is not ready. PRV-010: a provider's restart makes no read. PRV-011: the provider delivers each value as the variable or argument that the item binds and states where it keeps it. |
| G1 | TPL-012, TPL-018 | As written at the time: parts in order, reads beside them; a retirement asks for the run to serve its retire hook. |
| G3 | ACC-003, ACC-005, ACC-011, ACC-017, ACC-032, ACC-039, CON-021 | ACC-003's exception for `reveal-secret.v1`; ACC-039 permits a read that belongs to the retirement. |
| G4 | OPS-022, REC-001, REC-002 | REC-002's fields for the record of a read; REC-001 records each read. The work order has REC-001 cite SEC-015 and REC-002 cite SEC-015 and SEC-004. |
| G5 | IFC-006, IFC-012, IFC-018, IFC-025, IFC-031, IFC-041, DL10 | IFC-006 sends the configuration with every request; IFC-025's diagnostic names a place by its JSON Pointer; DL10 keeps "behavior when storage is exhausted". |
| G6 | MAN-010, MAN-012, OBJ-007, OBJ-008, OBJ-009, OBJ-021 | MAN-010 gives `environment`, `arguments` and `headers`; MAN-012's observable that `engine` implements no operation of `secret-reveal.v1` (the work order's text, cited by SEC-020). |

Citations of SEC IDs in other files at the time of writing, checked against `secrets.yaml`:
ACC-001, ACC-003, ACC-005, ACC-006, ACC-017, ACC-019 and ACC-039 in `access.yaml`; IFC-006,
IFC-012, IFC-018 and IFC-041 in `implementation.yaml`; OBJ-021 in `objects.yaml`; TPL-012 and
TPL-018 in `templates.yaml`. Each agrees with what the cited promise states. Two wordings differ
slightly and may be aligned:

- ACC-001 and ACC-003 say "an actor's request for `reveal-secret.v1`, on its own behalf or as an
  onward request"; SEC-007 says a request that "an instance sends". The rule is the same where
  "an actor's request" means one whose caller is an actor (C5).
- ACC-005 says a declared Conduit that lists `reveal-secret.v1` is opened "only where the
  instance that sent the request to set up is the root Interface"; SEC-008 says "where the root
  Interface sent it on its own behalf". They differ only where the root Interface sends the
  setup as an onward request serving another caller, which SEC-008 refuses (C6).

For the core-and-schemas editor: `secret-reference.v1` as SEC-001 describes it (the kind
Resource, no bare name, a reference to a Pool's entry and an entry reference, the resolved
form); `reveal-secret.v1` with no inputs, the output `value` a string, and no `effect`, which
SEC-005's observable states; `secret-reveal.v1` listing it alone. The checker rule
`secret-branch` and the example `SEC-001-secret-reference-beside-a-string` should reject the
schema under every applicator that C1 lists, not only beside a string. The key of `run.v1`'s
`secrets` is D1.

For the docs editor: the head comment points to a section "What the guarantee for secrets
covers" in `spec/README.md`, which the work order gives the docs editor; the terms above.

## Still needs a decision

| # | Question | Where it bears | Options |
| --- | --- | --- | --- |
| D1 | The keys of `run.v1`'s `secrets`. A name in an item's `environment` and one in its `arguments` may be equal, so keys by name can collide. | SEC-010, PRV-007, PRV-011, `run.v1` (core) | Keyed by the JSON Pointer of the place, which the item's bindings map to names; or two maps, one for variables and one for arguments; or keyed by name, with a rule that no name is in both. SEC-010 holds under any of them. |
| D2 | A connection to a `remote` item that ends outside a setup: a dropped session, or the engine restarted. Connecting again needs a read that belongs to no invocation and has no caller, which 4c excludes. | SEC-011, SEC-013, IFC-041, TPL-012 | Requests to the instance report unavailable until a further setup connects again; or the engine connects again with a read that belongs to the setup that first connected, recorded as such. |
| D3 | Whether the engine's own runner gives a process variables of the engine's own environment beyond what the item binds, such as `PATH`, `HOME` or the login session that a keychain store needs. The review leaves what else a run receives to the provider and removes the engine's environment as a source of secrets (13a); for the built-in runner the provider is the engine, so its contract is the runtime profile. | SEC-020, PRV-003, DL12 | An item of the implementation-defined list for the built-in runner; or a promise that it passes none, or a named set. |
| D4 | What a run receives for a place that `environment` or `arguments` points to and that holds null under `{type: null}`: no variable, or an empty one. MAN-017 requires the place, not that it holds a reference. | SEC-010, PRV-007, MAN-017 | The variable or argument is left out; or the item may not point to a place whose schema admits null. |
| D5 | SEC-004 allows only the resolved form, so a reference that is not resolved when recorded, as in a template document that a request hands over, is kept in no form. | SEC-004 | Keep as written, which is safe; or allow such a reference to be kept as written, since a reference is never the secret itself. |
