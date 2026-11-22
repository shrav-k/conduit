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
