# Ruling 43: log of the core package and the schemas

Date: 2026-11-06. This is the account of how the editor of the core package and the schemas
applied [ruling 43](README.md) to its files: the
[multi-machine proposal](../multi-machine/README.md) as the ruling accepts it, with D5's second
part as option b and Q1 tightened, by its section
[9, "In v0"](../multi-machine/README.md#in-v0) and the detail of sections 1.2 to 1.5, 3, 4 and
5; and the items of the [log of ruling 42's integration](../ruling-42/log-integration.md) that
waited on it, where they touch these files. Section "In v1, for A65" is not applied: it is for
the tracker. The rulings are the owner's. The wording, and every choice logged below, is the
editor's, and stands unless the coordinator or the owner strikes it. Nothing was committed.

Files: everything under [`spec/core/`](../../../spec/core/README.md) and
[`spec/schemas/`](../../../spec/schemas/README.md), with the examples;
[`tools/check_schemas.py`](../../../tools/check_schemas.py); and
[`spec/names.yaml`](../../../spec/names.yaml) with
[`tools/set_names.py`](../../../tools/set_names.py), since the proposal fixes the names of
`identity` "with the other names that the specification assigns" (Q1). No other file was
touched but this log. Other groups edited the promise files, `spec/definitions.md` and
`spec/README.md` at the same time; this log names what in their files these files depend on.

## Checks

Run from the repository root at the end of this pass, with the other groups' files as they
stood then.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `447 promises ok` |
| `python3 tools/check_schemas.py` | `Errors: 0`. 106 files holding 250 documents read as strict YAML; 58 Schema contracts checked against the profile; 146 documents validated; 55 invalid documents each rejected; 102 exports and 1 implementation; 414 references checked for their kind; 992 citations of promises in 109 files, every one of a promise that exists; 24 rules from 23 promises applied, and the 28 sets under `examples/invalid-by-rule/` each rejected by the rule they are named for; 10 defaults validated |
| `python3 tools/set_names.py --check` | `Errors: 0`. `CONDUIT_URL` x12, `CONDUIT_IDENTITY_DIR` x9, `` `ca.crt` `` x7, `` `token` `` x8, `` `tls.crt` `` x7, `` `tls.key` `` x6 under `spec/` |
| `python3 tools/check_docs.py` | `Errors: 0` once this log exists; before it, the two links to it from the READMEs |
| `git diff --check` | Clean |

Two tests were run on a copy of `spec/` and `tools/` in the scratchpad, and the copy was then
removed: with `spec/names.yaml` changed by hand, both tools report the disagreement; and
`set_names.py --identity engineUrlVariable=CONDUIT_ENGINE --identity keyFile=tls.pem --write`
rewrote 18 occurrences, after which both tools reported no error.

## Counts

| | Before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| Exports of the core package | 102 | 8 | 0 | 0 | 102 |
| of which Operation contracts | 23 | 5 | 0 | 0 | 23 |
| of which Capability contracts | 7 | 2 | 0 | 0 | 7 |
| of which Schema contracts | 58 | 1 | 0 | 0 | 58 |
| Rules of `tools/check_schemas.py` | 22, from 21 promises | 0 | 2 | 0 | 24, from 23 promises |
| `examples/invalid/` | 54 files | 0 | 1 | 0 | 55 files |
| `examples/invalid-by-rule/` | 26 files | 0 | 2 | 0 | 28 files |
| `examples/valid/` | 7 files | 1 | 0 | 0 | 7 files |
| Names in `spec/names.yaml` | 4 | 0 | 6 | 0 | 10 |

These files hold no promise, so no promise was changed or added here, and no ID was assigned.
Four shapes changed: `run.v1`'s `identity`, which is now required and closed; the inputs of
`stop.v1` and `inspect-run.v1`, which take `machine`; `inspect-run.v1`'s outputs, which may hold
`url`; and `implementation-item.v1`, which rejects `Authorization` among the `headers` of an item
that is not `plain`. Each file changed has a line for ruling 43 in its head comment, or, for a
README, a paragraph on it; the new examples say what they show in theirs.

## What changed

| What | Promises | Why, by the proposal's section |
| --- | --- | --- |
| `run.v1`, schema: `identity` is required; `identity.environment` holds exactly `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR`, both required, and nothing else; `identity` holds `environment` and `files`, both required, and nothing else | PRV-007, PRV-022, DL13 | Q1 (5): the names of what `identity` carries are fixed once, and PRV-022 now has it carry "these names and no other". Section 6, D17 |
| `run.v1`, description: the fixed names; `files` keyed by their paths in the directory that `CONDUIT_IDENTITY_DIR` names, `ca.crt` always and `token`, `tls.crt`, `tls.key` as the forms of proof need them; the gate also serves `tls.crt`; the provider may give both variables as the run sees them | PRV-022, DL13 | Q1 (5); 1.4, a trust bundle beside the credential; section 6, D17 |
| `run.v1`, description: the credential names the instance by its ID and lasts as long as the instance, as the certificate the gate serves does, and nothing renews either | PRV-022, DL13 | 1.4, "as long as the instance, in v0"; X9 |
| `run.v1`, description: `secrets` and `identity` reach the run's machine only over a channel that authenticates it and keeps them confidential, named in the contract; the gate may use `identity` for its means and nothing else | PRV-011, PRV-022, PRV-025 | Section 9, row PRV-011, PRV-022; 1.4, "delivered in confidence" |
| `run.v1`, description: `url` stands behind the provider's gate, which admits only an enforcement point's proof, and the engine sends nothing there until the server's side has proved it serves the instance | PRV-025, IFC-017 | Section 3, the gate in substance; section 9, row IFC-017 |
| `run.v1`, description: a `package` whose transport the provider does not run ends failed; `stdio` only bridged behind the gate | PRV-012, MAN-010 | Section 4, "a gap D20 did not raise"; section 9, row PRV-012, MAN-010 |
| `run.v1`, description: neither `environment` nor `secrets.environment` holds a variable of `identity`, which no item that a template selects binds | MAN-018 | X3 and item 2 of [Needs a decision](../ruling-42/log-integration.md#needs-a-decision) |
| `stop.v1`, schema and description: the input `machine`, required, as `run.v1` gives it, the labels last reported where the node is retired | PRV-008, PRV-027 | X10 |
| `stop.v1`, description: a stop that the provider cannot carry out now is kept where neither a restart of the engine, on its machine or another, nor the provider's own retirement loses it, and the contract states where | PRV-020, PRV-021 | Section 6, D11; section 9, row PRV-021, PRV-020; item 5 of Needs a decision; X11's second part |
| `inspect-run.v1`, schema and description: the input `machine`, required; the optional output `url`, the URL at which the run is reached now; the engine reaches the latest one reported, once its side has proved it serves the instance | PRV-009, PRV-027, IFC-017 | X10; section 3, the engine's side; section 9, row `inspect-run.v1`, PRV-009 |
| `get-secret.v1`, description: the read again after a connection's values are lost is of each secret Resource that the setup read | SEC-023 | X11 |
| `get-secret.v1`, description: the reads before the engine runs again a provider whose run ended with the engine, for the caller of the setup that last asked for that run | PRV-026, SEC-011 | Q3 (5) |
| `get-secret.v1`, description: a secret Resource's server is reached only with the engine's proof, behind its runtime provider's gate or checking it itself | PRV-025, IFC-041 | Section 3; S2 |
| `register.v1`, description: a template is rejected where a selected item binds a variable of `identity`, or a selected `remote` item sends a header that carries a secret to an `http` URL whose host is not `localhost`, `127.0.0.0/8` or `::1`, or to any `http` URL on an engine that can restart on another machine or run as several parts | MAN-018, SEC-024, DL17 | X3; D5's second part, option b |
| `runtime-provider.v1`, description: the server of every run stands behind the provider's gate; the contract names its means and its bound on a silent engine | PRV-025 | Section 3; section 8, row runtime provider |
| `secret-provider.v1`, description: a secret Resource's server is reached only with the engine's proof; a store reached by `remote` that cannot check it is marked `plain` and sent no locator | PRV-025, IFC-041 | Section 3, option A; S2; section 8, row secret provider |
| `implementation-item.v1`, schema and description: an item that is not `plain` names no `Authorization`, in any case, in its `headers`; the description also says that MAN-018 and SEC-024 reject the template, not the item | IFC-043 | Section 3, the engine's side, the collision with `Authorization`, which the interface group settled as IFC-043 |
| The rules comment of `operations.yaml`: `get-secret.v1` is invoked by the reads of PRV-026 too | PRV-026 | Q3 |
| New rule `identity-name` (MAN-018), with the example `invalid-by-rule/MAN-018-item-binds-a-variable-of-identity.yaml` | MAN-018 | X3; the names it reads are those of `spec/names.yaml` |
| New rule `secret-header-https` (SEC-024), with the example `invalid-by-rule/SEC-024-secret-header-to-http.yaml` | SEC-024 | D5's second part, option b, its static part |
| New example `invalid/IFC-043-manifest-remote-item-names-authorization.yaml` | IFC-043 | As above |
| `check_schemas.py`: a set under `examples/invalid-by-rule/` may hold its project's manifest, a document with no `kind`, from which the rules read the items its templates select | — | The two new rules need an item that no valid example has, and no example under `invalid-by-rule/` could carry one before |
| `check_schemas.py`, step 5: `run.v1`'s `identity.environment` holds and requires exactly the variables of `spec/names.yaml`, and its description names each file there | Q1 | One place defines the names, and the tool checks that the contract agrees |
| `spec/names.yaml`: `engineUrlVariable`, `identityDirVariable`, `trustBundleFile`, `tokenFile`, `certificateFile`, `keyFile` | PRV-022, Q1 | Q1: "fixed once, with the other names that the specification assigns" |
| `set_names.py`: reads the six names; `--check` counts each and reports a variable that begins `CONDUIT_` and is neither; `--identity KEY=VALUE` changes one, a variable as a whole word, a file only in backticks outside `spec/schemas/examples/`, showing each line in which a file name would change | Q1 | As above |
| The example project's Pool template `workspace`: the input `runtime` and its destination `/entries/checkout/runtime` are gone; its comment says why | PRV-007, MAN-010 | D20, A: "The valid example's Pool template `workspace` changes under any answer" (section 4) |
| `spec/schemas/templates.yaml`, the comment on what the engine checks: MAN-018 and SEC-024 | MAN-018, SEC-024 | As above |
| `spec/core/README.md`: status; a paragraph on ruling 43; Providers (the names of `identity`, the channel, the credential's life, MAN-018, the gate, stdio on the engine's own runner, PRV-027, PRV-021, PRV-026); the secrets paragraph (SEC-023's references, SEC-024); Conduit's keys (IFC-017, IFC-041, IFC-043, PRV-025, IFC-042); the two tables of operations and the export table; "What this package does not settle", whose bullet on `identity` now leaves only what DL13 does | As named | The ruling, as above |
| `spec/schemas/README.md`: status; a paragraph on ruling 43; the open places row for `headers`; two rows of "What a schema cannot express"; the examples table; Checking | As named | As above |

## The items that waited on this ruling

From the [log of ruling 42's integration](../ruling-42/log-integration.md):

| Item | Settled as | Here |
| --- | --- | --- |
| X1, runs under one operating-system user | Q5: stated plainly in IFC-022 and `spec/README.md` | Nothing: neither is in these files. The description of `run.v1` says nothing that contradicts it |
| X3 and Needs a decision item 2, a name in both `identity` and what the item binds | Q1 fixes the names, so a template whose item binds one is rejected (MAN-018) | `register.v1`, `run.v1`, `implementation-item.v1`'s description, the rule `identity-name` and its example, the two READMEs, the schemas' `templates.yaml` comment |
| X7, the engine restarting on another machine | Q2: it keeps its hostname (DL17) | Nothing directly. `register.v1` and the README's secrets paragraph name the engine that can restart on another machine or run as several parts, as SEC-024 and DL17 do, and never "fail over", which the definitions list as a word not used |
| X9, no call brings a refreshed credential | In v0 a credential lasts as long as its instance; renewal is v1's (1.4) | `run.v1`: nothing renews the credential, or the certificate the gate serves, in this version; README, Providers |
| X10, `stop.v1` and `inspect-run.v1` give no machine | They take the same `machine` as `run.v1` (PRV-027) | Both schemas and descriptions; README tables |
| X11, which references SEC-023 reads | Those the setup read | `get-secret.v1`; README, the secrets paragraph |
| Needs a decision item 5, a stop held when a provider is retired | Kept where neither a move of the engine nor the provider's retirement loses it (PRV-020, PRV-021) | `stop.v1`; README, Providers |
| For the research: D5, both parts | Ruled | The gate in `run.v1`, `inspect-run.v1`, `get-secret.v1`, the two capabilities and the README; SEC-024's rule and example |
| For the research: D20 | Ruled, A | The example project's `workspace`; `run.v1` keeps row 17's guard |
| For the research: which engine address works from another machine | IFC-042, and `CONDUIT_URL` given as the run sees it | `run.v1`; README, Conduit's keys |
| For the research: a refreshed credential | X9 above | As X9 |
| For the research: SEC-023's read after a restart on another machine | SEC-024 | The rule `secret-header-https` |

## Coined names

The ruling leaves the names of what `identity` carries to the text. This editor first chose
`CONDUIT_ENGINE_URL` and spiffe-helper's file names (`svid.pem`, `svid_key.pem`,
`svid_bundle.pem`, `jwt_svid.token`, `jwt_bundle.json`), and then found that the providers, the
interface and the implementation-defined groups had already written the names below into
PRV-022 and DL13. To keep one set across files, these files and `spec/names.yaml` use theirs.
Their precedents are in the [providers group's log](log-providers.md#coined-names) and the
[interface group's log](log-interface.md#coined-names). For the owner to strike; one run of
`python3 tools/set_names.py --identity <key>=<value> --write` changes any of them everywhere.

| Name | Key in `spec/names.yaml` | What it is |
| --- | --- | --- |
| `CONDUIT_URL` | `engineUrlVariable` | The variable that holds the engine's address, as the run sees it |
| `CONDUIT_IDENTITY_DIR` | `identityDirVariable` | The variable that names the directory of the files of `identity`, as the run sees it |
| `ca.crt` | `trustBundleFile` | The trust bundle: the root certificates of DL13's mechanism |
| `token` | `tokenFile` | A token in the form of a JWT-SVID |
| `tls.crt`, `tls.key` | `certificateFile`, `keyFile` | A certificate in the form of an X509-SVID, and its private key |

Also coined here: the names of the two rules of `check_schemas.py`, `identity-name` and
`secret-header-https`, and of the six keys of `spec/names.yaml`. The terms gate, enforcement
point and trust bundle are used as the ruling and PRV-022 and PRV-025 use them; decision point
is not used in these files. None of them is defined here: `spec/definitions.md` is the
definitions group's.

## Choices beyond the record

1. **`identity` is required in `run.v1`, and closed.** PRV-007 and DL13 give every run
   `identity`, and PRV-022 lists its names "and no other", so the schema requires it and
   admits no other variable. The keys of `files` stay paths, as PRV-022 has them, so the schema
   cannot list the file names; `check_schemas.py` checks that the description names each.
2. **`url`, not `endpoint`,** for the new output of `inspect-run.v1`: N5 of ruling 42 renamed
   `run.v1`'s `endpoint` to `url`, and the providers and interface groups use `url` too.
3. **`machine` is required in `stop.v1` and `inspect-run.v1`,** in the same form as in
   `run.v1`, as PRV-027 has the engine always give it.
4. **IFC-043 in the schema.** IFC-043, the interface group's, rejects an item "wherever it is
   read, whichever engine reads it", so `implementation-item.v1` can state it, in its `else`
   branch for an item that is not `plain`, with a pattern that matches `Authorization` in any
   case. It is not among the items assigned to these files; if IFC-043 is struck, the schema's
   `propertyNames` under `else`, the example and the README rows go with it.
5. **MAN-018 and SEC-024 as rules on the template, not the manifest.** Each promise rejects the
   template, and an item that binds `CONDUIT_URL`, or a `remote` item at an `http` URL with a
   header bound to a secret, is valid alone, so neither is in `implementation-item.v1`, which
   would have rejected a package that no template uses that way. The rules look at the items
   that the tool has, as MAN-017's does.
6. **SEC-024's static part.** The rule rejects an `http` URL whose host is not `localhost`, an
   IPv4 address in `127.0.0.0/8` or the IPv6 address `::1`, compared without regard to case, as
   SEC-024 states; `127.1` and `foo.localhost` are not loopback hosts there. It does not look at
   a host written as a variable of the Remote (`{host}`), nor through a `$ref` to the place, and
   it does not reject a loopback host, which only an engine that can restart on another machine
   or run as several parts rejects.
7. **A by-rule set may hold its project's manifest.** The two rules need an item that no valid
   example has. Adding one to the valid examples would have shown an item that no template of
   theirs could use, so `check_schemas.py` now reads a document with no `kind` in a set under
   `examples/invalid-by-rule/` as the set's manifest, validates it against `manifest.v1`, and
   gives its items to the rules as the set's own, reached by `#name`.
8. **The example project's `workspace` loses its `runtime` input** rather than gaining an entry
   whose item is a pinned `package`: no item of the examples is a `package` that a Pool entry's
   template could select without more contracts, and D20's answer is that such an entry runs on
   the engine's own runner. The project no longer shows an input whose destination is
   `/entries/<name>/runtime`, which `pool-template-spec.v1` still allows.
9. **`spec/names.yaml` holds the names of `identity`,** and `set_names.py` changes them. A file
   name such as `token` is also a common word and Kubernetes' own file's name, so the tool
   counts and rewrites a file name only in backticks, never under `spec/schemas/examples/`,
   whose example project has an input `token`, and shows every line in which a file name would
   change before `--write`. The sentences of MAN-018 and PRV-022 that cite Kubernetes' own
   `token` and `ca.crt` would be rewritten too, and must be put back by hand after such a change.
10. **The description of `secret-provider.v1` says a store that cannot check the engine's proof
    is marked `plain` and sent no locator.** That follows IFC-041 and PRV-025, and means such a
    store learns its locator from no request: it can serve one fixed secret at most.

## Open, for the coordinator or the owner

1. **How a gate checks the engine's token.** Under the token form toward a run, the engine
   presents a JWT-SVID whose audience is the instance's ID, and the run's gate checks it with
   what `identity` holds (PRV-022, PRV-025). `identity` holds `ca.crt`, root certificates, and
   no key that verifies a token. Either DL13's mechanism signs tokens with a key whose
   certificate chains to `ca.crt` and the token carries it, as the JWS header `x5c` allows, or
   `identity` needs a fifth file, a JWK Set of the keys that sign tokens, as spiffe-helper
   writes `jwt_bundle.json`. Not decided here.
2. **A `remote` item that is not `plain`, at an `http` URL.** IFC-041 has the engine never
   connect to it, and the part report unavailable. It could be rejected when the manifest is
   read, by `implementation-item.v1`, as IFC-043's item is; no promise says so, so the schema
   does not.
3. **AGENTS.md** names three kinds of name in `spec/names.yaml`; it now holds the names of
   `identity` too, which `set_names.py --identity` changes. AGENTS.md is not this editor's.
4. **The definitions of gate, enforcement point and trust bundle** are the definitions group's.
   These files use the three words; if the definitions name them otherwise, the descriptions of
   `run.v1`, `inspect-run.v1`, `get-secret.v1`, the two capabilities and the READMEs follow.
