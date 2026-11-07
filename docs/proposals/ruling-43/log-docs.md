# Ruling 43: log of the docs editor

Date: 2026-11-06. This is the account of the docs editor's pass after the four groups and the
[integration](log-integration.md) applied [ruling 43](README.md), which accepts the
[proposal on work across machines](../multi-machine/README.md) with D5's second part as option b
and Q1 tightened. The ruling is the owner's. The edits below are the docs editor's, and each
stands unless the coordinator or the owner strikes it. No record was edited but this log, and
nothing was committed.

The files of this pass: `spec/definitions.md`, `spec/README.md`, `spec/schema-profile.md` and
`docs/OVERVIEW.md`. The integration had already written the first two up to the ruling. This
pass read them against the promises as they now stand, completed them, and brought the other
two up to the ruling. The section [In v1, for A65](../multi-machine/README.md#in-v1-for-a65) is
not applied: it is for the tracker.

## What changed

| File | Change | Ground |
| --- | --- | --- |
| `spec/definitions.md`, head paragraph | Names this pass, the row it added and the rows it changed | The rule for every changed file |
| `spec/definitions.md`, new row credential | What a sender presents to prove who it is: in the promises on who is who, what DL13 issues from one root of trust in its two forms; a run's names its instance by ID, lasts as long as the instance, and establishes none once the instance is removed; the engine's own may be short-lived; the word keeps its ordinary sense for a fetch (MAN-006) | Section 1.4 and Q1 of the proposal; DL13, PRV-022, IFC-022, IFC-032, PRV-025. The new promises use the word in that sense throughout, and no row said what it was |
| `spec/definitions.md`, row `identity` | `token` where the mechanism uses that form, and `tls.crt` and `tls.key` where it uses mutual TLS or the run's server stands behind a gate, as PRV-022 conditions each; the channel also keeps it confidential | PRV-022, PRV-011 |
| `spec/definitions.md`, row represent | An engine that can restart on another machine, or run as several parts, runs only on machines that take its hostname, so `local` represents one hostname | X7; Q2 (PLC-026, DL17) |
| `spec/definitions.md`, row access decision | An engine that runs as several parts reads the state as it stands for every decision | Q2 (DL17) |
| `spec/definitions.md`, row invocation | At most one part of the engine acts on any one invocation, as the store of its own state enforces | Q2 (DL17) |
| `spec/definitions.md`, row `serving` | A read for a provider's run made again under PRV-026 names, as the invocation it serves, the setup that last asked for that run | Q3 (REC-002, PRV-026); the integration had added PRV-026's read to the rows caller, request, later work and the engine's read of a secret, and not to this one |
| `spec/definitions.md`, row cancel | A connection that the engine sees drop cancels the invocation for the engine; whether and when the implementation learns of it is its transport's and, behind a gate, the gate's | Section 9, row OPS-027 (OPS-027, PRV-025) |
| `spec/definitions.md`, row provider | The servers of the runtime, machine and package-source providers run on the engine's own runner over stdio, end with the engine and are run again when first needed; a secret Resource is placed and run as any other Resource | Q3 (PRV-002, PRV-026), with the integration's narrowing of DL16 to the three kinds |
| `spec/definitions.md`, row secret Resource | Its server, where another runtime provider runs it or it is reached by `remote` and is not plain, stands behind a gate | D5, A (PRV-025, SEC-006) |
| `spec/README.md`, What the guarantee for secrets covers | The paragraph after the list, which said that D5 "this section stated as a gap until ruling 43, is ruled", is gone; its substance, that only an enforcement point reaches a secret Resource's server that the engine does not run itself and that the request and the value cross confidential and unaltered, is now a point of the list of its own, split from the point on an Agent's model | Section 9, row "`spec/README.md`": "The D5 gap goes" (PRV-025, IFC-017, SEC-006) |
| `spec/README.md`, Existing standards first | The proof of who is who takes the forms of SPIFFE's X509-SVID and JWT-SVID and the protocol's own authorization, and a gate is made of existing tools, as each provider's contract states | Q1; section 3, "Means a provider may use" (DL13, PRV-025) |
| `spec/schema-profile.md`, head | Names ruling 43 and what it changed on this page | The rule for every changed file |
| `spec/schema-profile.md`, row An implementation's `remote` URL | A template whose item binds a header to a secret is rejected unless the URL is `https`, or `http` with a loopback host on an engine that can neither restart on another machine nor run as several parts; the three hosts; scheme and host compared without regard to case; no other host counts; an item that is not plain is connected to only at `https` | D5's second part, b (SEC-024, IFC-041) |
| `spec/schema-profile.md`, row Implementation items | A template is rejected whose item's `environment` binds a variable by a name that `identity` carries, which `names.yaml` fixes; an item reached by `remote` that is not plain names no `Authorization` | X3; Q1 (MAN-018, IFC-043) |
| `docs/OVERVIEW.md`, head | Last read as ruling 43 left it; points to the new section | The record |
| `docs/OVERVIEW.md`, new section Work across machines | One engine decides, as the only decision point and enforcement point, and may be several parts or restart elsewhere under DL17; who a request comes from, by `identity` and the two forms of proof, the token at the engine's address from another machine, credentials as long as the instance; the gate, its means from existing tools and what does not count; what needs none of it: one machine with the engine's own runner over stdio, a `plain` server, any MCP client; what stays open: the engine's own user, packaged programs, a machine cut off, and a link to the shape of a later version | Section 9, "In v0"; the ruling's Q5 and the owner's question on friction for one machine and for the protocol |
| `docs/OVERVIEW.md`, Pools, ownership and placement | A machine is reported gone only where the provider knows it no longer exists | Section 9, row PRV-014 |
| `docs/OVERVIEW.md`, Conduits and access | A gate keeps other callers away and makes no restriction enforced | Section 9, row "ACC-027, definitions" |
| `docs/OVERVIEW.md`, One lifecycle | The provider keeps a stop it owes where neither a restart of the engine nor its own retirement loses it | Section 9, row "PRV-021, PRV-020" |
| `docs/OVERVIEW.md`, Operations, outcomes and reasons | Two exceptions to ACC-019, SEC-023's read and PRV-026's, where it said one | Q3 |
| `docs/OVERVIEW.md`, Providers | The runtime provider's run is behind a gate; the engine's own runner talks over stdio; the providers' servers run again after a restart; `identity` points to the new section, and "may refresh while the run lasts" is gone | Q1, Q3, D5 (PRV-002, PRV-022, PRV-026) |
| `docs/OVERVIEW.md`, Secrets | SEC-024's rule; PRV-026's reads; a secret Resource's server behind a gate | D5 and its second part; Q3 |

## What this pass checked

- Every row of `spec/definitions.md` that the integration added or changed, against the
  promises it cites: decision point, enforcement point, gate, trust bundle, engine, caller,
  mediated, enforced, request, implementation item, connection, plain, the engine's own runner,
  machine provider and fetching, the list set on the engine's own runner, runtime provider, run,
  `identity`, machine, the engine's read of a secret, domain of trust, names a version assigns,
  later work and fail over. They agree with PRV-002, PRV-003, PRV-011, PRV-014, PRV-021,
  PRV-022, PRV-023, PRV-025 to PRV-027, IFC-017, IFC-022, IFC-041 to IFC-043, ACC-027, SEC-011,
  SEC-023, SEC-024, MAN-018, PKG-020, DL13, DL16 and DL17 as they stand; the row `identity` was
  made exact where it simplified PRV-022's conditions.
- The other rows whose promises ruling 43 changed: the rows above are those that needed it. The
  rows record, provider call, unregister and remove already agree with REC-002, PRV-005 and
  LIF-036, since a stop's `machine` is in the row run.
- That every term of the new promises is defined: enforcement point, gate, trust bundle and
  credential have rows, the engine's several parts are in the row engine, and a restart of the
  engine is in the row fail over. Decision point has a row although no promise uses it, as the
  integration chose (its choice 1). The words of the standards the promises cite, such as
  X509-SVID, JWT-SVID, audience, canonical URI and protected resource metadata, keep the
  standards' meanings and have no rows, as other standards' words have none.
- The counts of `spec/README.md` against the validation command and a count of the files: 447
  promises, 201 `language`, 205 `convention` and 41 `interface`; 17 in `manifest.yaml`, 21 in
  `placement.yaml`, 27 in `providers.yaml`, 24 in `secrets.yaml`, 29 in `access.yaml`, 35 in
  `lifecycle.yaml`, 31 in `operations.yaml`, 5 in `observation.yaml`, 6 in `records.yaml`, 40
  in the interface file; 16 items of the implementation-defined list; 23 Operation contracts,
  102 exports; 55 files under `examples/invalid` and 28 under `examples/invalid-by-rule`. The
  integration's figures all hold, and this pass changed none.
- That the section on the guarantee for secrets states what section 9 asks: the run's machine
  and its gate in the domain of trust, for what that run serves; Q5 plainly; and no D5 gap.

## The items that waited on this ruling

From the [log of ruling 42's integration](../ruling-42/log-integration.md). The integration
settled each in the promises; this is where these four files show it.

| Item | In these files |
| --- | --- |
| X1, runs under one operating-system user can read each other's credential | `spec/README.md`, the point "No consumer can present as another", as the integration wrote it, beside IFC-022; `docs/OVERVIEW.md`, "What stays open" |
| X3 and item 2 of Needs a decision, a variable named in `identity` and bound by the item | `spec/definitions.md`, rows `identity` and implementation item; `spec/schema-profile.md`, row Implementation items (MAN-018) |
| X7, the engine keeps its hostname across a move | `spec/definitions.md`, row represent (this pass); `docs/OVERVIEW.md`, "One engine decides" |
| X9, a credential lasts as long as its instance, renewal is v1's | `spec/definitions.md`, rows `identity` and credential; `spec/README.md`, the point on what a run is given in `identity`; `docs/OVERVIEW.md`, which no longer says a credential may be refreshed |
| X10, `stop.v1` and `inspect-run.v1` take the run's machine | `spec/definitions.md`, rows runtime provider and run (PRV-027) |
| X11, which references SEC-023 reads | `spec/definitions.md`, rows connection and the engine's read of a secret; `spec/README.md`, the point on a connection whose values the engine lost |
| Item 5 of Needs a decision, a stop held when a provider is retired | `spec/definitions.md`, row runtime provider; `spec/README.md`, the last point of the guarantee for secrets; `docs/OVERVIEW.md`, One lifecycle (PRV-021, PRV-020) |

## Coined names

The names of what `identity` carries, `CONDUIT_URL`, `CONDUIT_IDENTITY_DIR`, `ca.crt`, `token`,
`tls.crt` and `tls.key`, and the terms decision point, enforcement point, gate and trust bundle,
were coined by the groups and the integration, which log their precedents
([coined names](log-integration.md#coined-names)); they are for the owner to strike. This pass
coined none. `spec/definitions.md` writes the names as `spec/names.yaml` does, and
`tools/set_names.py --check` counts them with no error. Credential is not coined: it is the
ordinary word, which the row gives the sense the promises use.

## Choices beyond the record

1. **The D5 paragraph after the list goes.** The integration kept a paragraph that said D5 was
   a gap of this section until ruling 43. Section 9 says that the gap goes; the status paragraph
   of `spec/README.md` already says that ruling 43 ruled D5, and the substance is now a point of
   the list, which states limits and what holds rather than history.
2. **A row for credential.** The promises give the word a precise sense (named by ID, as long as
   the instance, issued from one root), and use it in its ordinary sense for a fetch and a
   store; the row states the first and keeps the second.
3. **`docs/OVERVIEW.md` does not write the names of `identity`.** `tools/set_names.py --write`
   rewrites them only under `spec/`, so a change of a coined name would leave the overview
   wrong. It links `spec/names.yaml` instead.
4. **`docs/OVERVIEW.md` mentions a later version only by a link** to Q4 of the proposal, and
   says that none of it is part of this version, since ruling 43 accepted Q4 as a record for
   A65 and not as text.
5. **`spec/schema-profile.md` repeats SEC-024's three loopback hosts**, which are the secrets
   group's choice 3, so that a reader of the profile sees which URLs a template may bind a
   secret header to. If that choice is struck, the row changes with SEC-024.

## Depends on what is still to be decided

These items of the [integration's list](log-integration.md#needs-a-decision) would change these
files if they are decided otherwise than as the text now stands:

- Item 1, a secret Resource on the engine's own runner after a restart: under (b), the rows
  provider, secret Resource and the engine's read of a secret, the paragraph on later work in
  `spec/README.md`, and the Providers section of the overview.
- Item 3, how a gate checks the engine's token: under (a), the rows `identity` and trust bundle
  and the coined names.
- Item 5, a `remote` item that is not plain at an `http` URL: under (b), the row An
  implementation's `remote` URL of the profile, and the row implementation item.
- Item 6, IFC-043 or mutual TLS: if IFC-043 is struck, the same two rows.

## For other jobs

- `AGENTS.md`, "Checks": `spec/names.yaml` now also holds the names of what `identity` carries
  (the integration's own item).
- `docs/AUDIT.md`: A64 and A65, and the items of the integration's list.

## Checks

Run from the repository root at the end of this pass.

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](../reaudit/CONSOLIDATION.md#validation) | `447 promises ok` |
| `python3 tools/check_docs.py` | `Errors: 0` |
| `python3 tools/set_names.py --check` | `Errors: 0`; `CONDUIT_URL` x13, `CONDUIT_IDENTITY_DIR` x10, `` `ca.crt` `` x8, `` `token` `` x8, `` `tls.crt` `` x7, `` `tls.key` `` x6 |
| `python3 tools/check_schemas.py` | `Errors: 0` |
| `git diff --check` | Clean |
