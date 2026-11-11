# Short form translation on S1 (A79)

Not binding. The [specification](../../spec/README.md) alone binds. Work order 031,
under [ruling 55](../../docs/proposals/ruling-55/README.md), authorized this spike.
The [roadmap](../../ROADMAP.md#experiments-and-probes) registers it.

## What this must show

Fixed before execution on 2026-11-10: translate S1 after T1, T2, T4, T5, T6 and T7,
in flow and block YAML, into canonical documents accepted by today's strict loader
and pure core on a fresh first-start state. Also check T7's revoked intermediate
state, equal translations between styles, intended access lists, and stable names
across T5 and T6. Hash the retained package archives against the manifest pins.
No live client, provider, directory or network service is run.

The translator follows the [grammar's translation table](../../docs/proposals/language-shape/short-form/synthesis.md#grammar)
with ruling 55's amendments: kind-bearing references, reusable Conduit templates,
fully qualified operations, optional frame, and the manifest's version.
The core's canonical documents remain unchanged.

## Counting fixed before execution

Count non-blank authored file lines, comments excluded, with document separators
included. Compare successive files by a shortest line edit script: a contiguous
replacement costs the larger of removed and added lines; a pure addition or removal
costs its length. Thus changing one line costs one, and removing then restoring the
same grant at T7 costs two edits. T3 changes no files and costs zero; it is not a
claim that reach inspection was performed. T7 counts both transitions.

Every state contains the author's manifest and sectioned document. Flow style keeps
the synthesis's section/key layout and uses flow values for configuration, grants
and pins. Block style writes one map key per line and one list element per line.
The read and write templates are authored once at T1 and reused. These files count,
including copied digest lines. Report digest-bearing edits separately.

Count each proposed command line carrying an author-chosen value apart from file
edits, using the per-task `commands.txt` ledger. Those lines describe a prospective
planner interface, are **not executed**, and do not establish its syntax or cost in
a real run. They include package installation and named replacement/prune/reopen
approvals. Harness commands (translation, checking, counting) are excluded.
Show file-only and file-plus-command totals, baseline 33/16/0/8/3/2/2 and
coordinator 154/73/0/2/62/29/12. The coordinator figures sum to 332.
One `fleet: s1` line would add one at T1 in either style; this spike emits none.

## Scope and limits

This is a translation spike, not a reference implementation or new promises.
It expands all rows of the table: reusable templates, inline and referenced
definitions, consumes, operators, the Pool operator default, standalone Conduits
and metadata. It uses the existing strict YAML reader and checks closed shorthand
fields and generated names, then delegates canonical schemas and resolution to
the loader and core. It does not implement source maps, SCT diagnostic attribution,
package authoring restrictions, mixed framed/sectioned projects, JSON array inputs,
cross-file shorthand name validation, a planner, tombstones or Fleet.

Use the registration-only Agent types shipped in
[the S1 fixtures](../../conformance/fixtures/s1/README.md), with explicit
`accountServices: false` and `peerMessaging: false`. They offer no conversation
capability; no Agent operator can therefore be authored against them. The proposed
v2 types and runnable wrappers in the synthesis are not these fixtures.
The Pool's default root operator is included. History remains pinned to filesystem
0.1.0 through its own package manifest after the project's T5 upgrade to 0.2.0.

T5 changes the project manifest, not a Type's spelling: translation alone cannot
identify an implementation upgrade. The check must compare resolved implementation
selections too. Stable-name checks compare document identities and bodies and
separately follow the shared Resource's selected implementation/configuration.
They do not execute a retirement, rebind held references, or measure downtime.
The resolved comparison excludes the objects' held resolver contexts and package
pin lists; these contain fresh-registration context changes beyond the replacement.
It compares every other stored field. The exact canonical document comparison has
no such exclusion.
Fresh registration re-resolves every reference; a real update must preserve pins
held by unchanged objects and recreate incoming Conduits after retirement.
A paper count is not a run or evidence that the owner's authoring bar is met
([ruling 45](../../docs/proposals/ruling-45/README.md)).

## Reproduce

From this directory:

```sh
cargo run --release
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The binary checks all states, prints diagnostics on failure, reports totals, and
prints the counts. It exits 1 when exact translations fail. Tests assert the
observed rejection as a regression, so passing tests do not mean accepted states.
It accepts no network input. The library exposes `translate`
for rendering canonical JSON documents. Static author files live in `states/`.
The [dated results](results-2026-11-10.md) record the outcome: every exact translation
is rejected because today's canonical Conduit schema has no `as` field. A separate
`without_as` diagnostic probe removes only that field from submitted documents,
then uses the same loader and core. Its registration, access-list and resolved
replacement checks do not establish the exact translation's semantics: the
current kernel would use the full Conduit name as the tool prefix.
The four checked archive snapshots, including filesystem 0.2.0, are supplied as
preinstalled snapshots for this offline experiment. No installation is claimed.
