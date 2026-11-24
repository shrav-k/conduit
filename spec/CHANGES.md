# Changes to the specification

Status: draft, not yet in force; [the decisions' README](../docs/decisions/README.md) says what
holds until it is. This file adds no behavior: [`spec/`](README.md) binds alone.

The form follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Each change to `spec/`
adds one line under `Unreleased`, of the form `- Kind: the promise IDs it changes (decision).`
A prose change names a `spec/README.md#anchor` or a definitions term instead of IDs. Kind is
Added, Changed, Deprecated, Removed, Fixed or Security, and the decision is `DEC-NNN` in
[the log](../docs/decisions/log.md) or "ruling N" for an archived ruling. For example:
`- Removed: IFC-011, handle.v1 (ruling 78 pt 16).` A release moves the `Unreleased` lines under
its own heading.

Decisions before this file are rulings 1 to 92 and the log's DEC-001 to DEC-032. The rulings leave
the tree at their freeze and are then cited by links pinned to their last commit (DEC-031); until
then they are under `docs/proposals/`.

## [Unreleased]

- Changed: `inspect.v1`'s `packages` lists each package an object reaches through a contract of its submitted project, so a moved pin of a package that only such a contract names shows in that package's item, not only as a moved submission digest, and an instance of a template that reaches such a package needs that release installed (order 267's finding; NAM-009, PKG-007; ruling 52).
- Added: `set-up.v1`'s output `objects`: for a setup that succeeds, each nested instance of its first part, a declared Conduit or an entry's Resource and that Resource's declared Conduits, with its lifecycle `state` and `revision`, so a client learns what became of each with no separate inspection, and a repeat of one whose outputs the engine no longer keeps reports conflict under OPS-034. A request whose target holds a core release from before this change gives no `objects` until that object is replaced onto a release whose contract declares them, since its held contract declares no outputs (order 247's finding; ruling 52).
- Added: `inspect.v1`'s `packages`: the submitted project's item lists `documents`, the kind, name and digest of each contract document of that submission, the digest taken by SUB-008's canonical form, so a moved submission digest says which contract changed; no content is shown (order 245's finding; ruling 52).
- Added: `inspect.v1`'s status member `settings`: under the JSON Pointer of each setting that a control whose outcome is unknown addressed (REV-009), `state` `unknown`, the control and the invocation, and no value, kept after reconciliation (REV-010) until a later control settles it (order 245's finding; ruling 52).

- Added: `register.v1`'s `objects` and `register-replacement.v1`'s `object`: each item gives `revision`, the object's revision as the request leaves it, which `inspect.v1` then shows, so a client keeps it with no separate inspection (order 247's finding; ruling 52).
- Added: `inspect.v1`'s optional output `implementations`: for a template or an instance, other than a Conduit, a Conduit template or a first-start object, the implementation selected for each operation (TPL-004), the package or submission that holds it, and the item as the object holds it under NAM-009, a relative program resolved (order 245's finding; ruling 52).
- Changed: PND-010, PND-011 and `sign-in.v1`'s description: an account check that answers after DL18 ended the sign-in's wait changes nothing but another account's hold, and a late expected account's diagnostic names a new sign-in.v1 (DEC-043).
- Changed: PKG-028's and NAM-012's observables and `spec/README.md#an-author-of-documents`: the reference CLI may install an archive's missing dependencies first, each by its own request, from a cache or mirror copy it picks, and pins only that archive, exactly (DEC-048).
- Changed: SCT-002 (DEC-007).
- Added: `spec/README.md#what-the-short-forms-meaning-covers`, ruling 76 points 2 to 5: each flag names its object, commands use the current engine context, apply goes on past a sign-in and answers none, and `conduit plan` is never required (DEC-008).
- Fixed: `spec/README.md#what-the-short-forms-meaning-covers`, a cascade's one-time Conduit is closed and removed by its target's retirement, and apply sends no close (DEC-008).
- Changed: `spec/README.md#an-author-of-documents`, its sectioned package example, which now pins no S1 package and defines none of S1's templates (DEC-007, DEC-039, ruling 78 pt 2).
- Added: REC-007, a request whose access decision or dispatch cannot be recorded reports unavailable, as does one found undispatched at a restart; a dispatched one whose end cannot be recorded ends unknown, unless its implementation answered with a reason and no outcome, which stands; each record is kept late; records held for a late keeping are lost if the engine stops first, and recovery ends what they leave as IMP-007 does (DEC-044).
- Changed: REC-001, a request's record that cannot be kept is kept late, under REC-007 (DEC-044).
- Changed: SEC-016, REC-007 for a read of a secret: unavailable at once, and recorded late (DEC-044).
- Changed: OPS-023, unavailable under REC-007 comes after ACC-037's and ahead of refused (DEC-044).
- Changed: IMP-007, a request found undispatched at a restart ends unavailable under REC-007 (DEC-044).
- Changed: DL10 leaves storage exhaustion beyond REC-007 to each engine (DEC-044).
- Changed: SEC-024, an IPv4 address is RFC 3986's IPv4address, `[::1]` is compared as an IPv6 address, and its restatements in `spec/schema-profile.md` and `spec/core/README.md` become citations (DEC-044).
- Added: IFC-051, a result, plain or not, may carry one usage member in `_meta`, which the record of the invocation's end keeps unchanged, or drops with a warning naming IFC-051 where it is malformed; it changes nothing else; a secret read's usage member is ignored without a warning under SEC-015 (DEC-047).
- Added: `usage.v1`, the core schema of IFC-051's member, and `record.v1`'s `usage` and `usageDropped` (DEC-047).
- Changed: REC-002, except for a read of a secret, the record of an invocation's end holds its usage member or the warning that dropped it, under IFC-051, whatever `records` states (DEC-047).
- Changed: CTR-013, ACC-033 and `type-spec.v1`, an Agent's login place may refer to a Pool-owned login Resource, for every plan, its Pool authorizing it (DEC-042).
- Changed: PRV-029 and MAN-017, a login Resource's directory reaches its Agents' runs; an Agent's hooks never log it out (DEC-042).
- Changed: PND-005, PND-009 and PND-010, an Agent's lapse raises its login Resource's action and one call at a time until root lifts it; only the Resource's own hold holds its Agents (DEC-042, DEC-046).
- Changed: PRV-020 and LIF-021, a login Resource with an Agent reports conflict at retirement (DEC-042).
- Changed: `login Resource`, `notice`, `state directory` and the limit at spec/README.md#pending-actions-sign-in-and-the-network on a Resource's login (DEC-042).
- Changed: IFC-016, a URL request outside sign-in.v1 from an Agent of a login Resource raises PND-005 for that Resource, the form in which a plain turn reports a lapsed login (DEC-042, DEC-046).
- Changed: MAN-010 and MAN-013, a relative-path item may state `platforms`, `<os>/<arch>` pairs, each pair's program in the archive, or the package is rejected (DEC-049).
- Changed: PLC-007, such an item is placed only on a node whose os and arch are a listed pair (DEC-049).
- Changed: PRV-007, another runtime provider runs a package's such item only through `run.v2`, given the program and the kept archive's descriptor; any other relative-path item still runs on the engine's machine alone (DEC-049).
- Changed: PRV-019, for a new `run.v2` start the provider checks the archive's byte length and then its SHA-256 on the machine before each unpacking, and on a mismatch runs nothing and fails, naming both lengths or both digests (DEC-049).
- Added: `run.v2`, which a runtime provider implements by its type's `requires` (PRV-001), and `platforms` in `implementation-item.v1` (DEC-049).
- Changed: MAN-009 and PKG-019 admit `platforms` and keep the platform-pair requirement without `nodeSelector`; PRV-028 and IFC-017 apply the run URL and its certificate check to `run.v2`; REC-002 names the instance in the `run.v2` record's `subject` (DEC-049).
- Added: `value.v1`, the core Type of a value Resource, one installation value that may be tied to a node and that root alone registers (ACC-057); the core template `value`; and `value-entries.v1`, a document's `values` (DEC-076).
- Changed: OBJ-013, OBJ-014, INS-003, INS-004 and INS-005, an instance document or a nested instantiation may give an input under `values` as `{kind: Resource, name}`, never under `arguments` too, and it is then supplied as an argument is (DEC-076).
- Changed: OBJ-009, a `values` entry resolves to a value Resource not retired, tied to no node or the instance's, at no reference place and passing the input's schema, or the document is rejected, with no fallback to a default and no value in the diagnostic (DEC-076).
- Changed: INS-013 and `inspect.v1`, the instance keeps a source map from each place a value filled to its value Resource, shown under `values` (DEC-076).
- Added: IFC-052, a result may carry the usage limits its client saw in `_meta`, kept in the end record or dropped with a warning; the consumer's result does not carry it (DEC-077).
- Added: IFC-053, the status of an Agent or a login Resource shows the newest of each window that its calls, or its Agents', reported, `unknown` after the reset; the engine sums and acts on none (DEC-077).
- Added: `limit-report.v1` and `limit-status.v1`, the forms of IFC-052 and IFC-053, `record.v1`'s `limits` and `limitsDropped`, and the status's `limits` (DEC-077).
- Changed: IFC-051, a usage-limit ending's limits are IFC-052's member, no part of usage (DEC-077).
- Changed: REC-002, the end record also holds the limits member or its warning, whatever `records` states (DEC-077).
- Changed: OBS-002, IFC-053's `limits` is the one observation not reported as a condition (DEC-077).
- Added: SCT-017, a documents file's `pool: X` makes Pool X from the new core template `pool` and supplies X as the owner of each Resource instance of that file that states none, sectioned or framed; a stated owner is kept (DEC-081).
- Changed: SCT-001, SCT-003, SCT-004, SCT-006, SCT-013, SCT-014, MAN-011, SUB-010 and SCH-018 name `pool` or SCT-017 beside `fleet` or SCT-011, `sectioned.v1` accepts `pool`, and `sectioned-resource.v1` no longer requires `owner` (DEC-081).
- Changed: ACC-056, the cause for ACC-057 names a value Resource beside an Agent (DEC-076).
- Added: ACC-058's `request-access.v1`, `withdraw-access-request.v1` and the template `access-request`, by which an Agent asks root for access (DEC-064, DEC-079, DEC-084).
- Added: ACC-058, an Agent asks through its own Conduit to `access`; an ask for 23 operations, `install.v1`, `management` or `access` is refused with a fixed message, raising nothing (DEC-064, DEC-079, DEC-085).
- Changed: ACC-022, ACC-029, ACC-030, ACC-033, ACC-056 and SCT-008, an asking Conduit is no right to open and needs no bound inline (DEC-064, DEC-079).
- Changed: IFC-050 permits a promise to fix a message, as ACC-058 does for access-request refusals (DEC-064, DEC-085).
- Changed: ACC-058 makes the asking Agent the resolved subject and keeps the named target in evidence; root answers and the declined result wait for order 299 (DEC-064, DEC-084).
- Changed: PKG-020, a plain item reached by `command` or `package` may list in `attributed` the operations whose calls carry their invocation and caller, plain in every other way (DEC-067).
- Changed: IFC-007 allows invocation and caller on a plain call whose operation its item's `attributed` lists. IFC-037 requires discovery for IFC-006 requests and hooks and PRV-005 provider calls, so an attributed call needs no discovery. IFC-004 requires request metadata support for attributed calls and reserves extension negotiation for calls that use the extension. IFC-047 exempts all calls governed by IFC-007, whose errors IFC-010 handles (DEC-067).
- Changed: MAN-009 and MAN-010 admit `attributed`, the list of PKG-020 (DEC-067).
- Added: `attributed` in `implementation-item.v1`, only on a plain item reached by `command` or `package` (DEC-067).
- Changed: OPS-036 and `list.v1`, instance items give owner, placement, revision and Running status from inspection state without adding authority (DEC-114).
- Changed: PRV-026, once the engine restarts it runs again, without waiting for a request, each Resource that is not retired whose run on its own runner ended with it, its secrets read for the caller of the setup that last asked for that run; an Agent's run waits for a further setup (DEC-115).
- Changed: OBS-004, PRV-002, PRV-021, SEC-010, SEC-011, SEC-012, SEC-014, SEC-017, REC-002, ACC-019, IFC-017, DL16, DL17 and `get-secret.v1` say a Resource's run where PRV-026 made a provider's again (DEC-115).
- Changed: CTR-013, IFC-049 and NAM-024 name no Agent product in their observables, IFC-049 scoping by `carried: false` instead, and what each promises is unchanged (DEC-105).
- Changed: `spec/README.md#pending-actions-sign-in-and-the-network` names no Agent product, scoping by `carried: false` instead, and the Agent packages' READMEs hold the rest (DEC-105).
- Changed: plan and apply's duties and flags leave `spec/README.md#what-the-short-forms-meaning-covers`, and two commands' sentences leave the author path, for the CLI's guide, `engine/CLI.md` (DEC-105).
- Changed: `spec/README.md#what-this-is` opens the page and leads with the six families; the proof of identity follows the layers, and the status follows under its own heading (DEC-105).
- Changed: REC-002, IFC-051 and `record.v1`, the end record of a call that starts model work on an Agent names its Fleet in `fleet` and, where its Type states one, its login Resource in `loginResource`, and the engine adds no usage up (DEC-117).
- Changed: IFC-052 and `limit-report.v1`, a report may carry `paidUsage`, with no amount, a null `usedPercent`, `windowsOmitted` and window IDs with digits and hyphens (DEC-094, DEC-096).
- Changed: IFC-053, OBS-002, `limit-status.v1` and `inspect.v1`'s description, a status keeps in `signals` each usage signal's newest value other than null (DEC-094, DEC-096).
- Changed: IFC-052 and `limit-report.v1`, a report may carry `paidUsage`, with explicit client-field meanings and omission rules and no amount, a null `usedPercent`, `windowsOmitted` and window IDs with digits and hyphens (DEC-094, DEC-096).
- Changed: IFC-053, OBS-002, `limit-status.v1` and `inspect.v1`'s description, a status keeps in `signals` each usage signal's newest value other than null; a newer window's null percentage replaces its earlier number (DEC-094, DEC-096).
