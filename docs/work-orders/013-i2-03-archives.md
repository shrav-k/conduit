# Work order 013 (I2-03, second part): installing a package from its archive

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`.
This order has network access, to fetch crates; use it for nothing else.

## What and why

The second part of I2-03 of the
[study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders),
which [decision 2](../proposals/first-build/increment-2/decisions.md) set apart: until now the
core has resolved packages from snapshots, which prove no installation. This order installs a
package from the bytes of its archive, so that the build may claim installation.

Read: PKG-004, PKG-022, PKG-023, PKG-026, MAN-013 and what they cite; the study's section 4 on
"preinstalled snapshots" and "install archive inputs"; DL7 in `spec/implementation-defined.yaml`;
and the core as order 012 left it.

## The decisions to apply

1. **The archive is an input**: its bytes, its expected hash and the completion of fetching
   them are given to the core; the core fetches nothing.
2. **Installing** verifies the archive's digest (PKG-026), unpacks it as PKG-022 and MAN-013
   say, in the form DL7's choice in the profile names, checks that its manifest states the
   address and the version that were asked for, reads its documents as the loader reads a
   project (MAN-011, dot-names skipped), and reports the outcome as PKG-023 says. A package
   installed this way enters the catalog as installed, not as a snapshot.
3. **Vectors with real archives**: small gzip-compressed tar archives kept under `conformance/`,
   made reproducibly and with their hashes computed: S1's filesystem package installed and then
   resolved as order 009's snapshot was; a wrong digest; a manifest naming another address or
   version; an instance document in an archive (MAN-011); a `._` entry that is skipped; and an
   entry that would unpack outside its directory, rejected. Every decision cites its promise.
4. **No third party's bytes in the repository.** Remove
   `conformance/fixtures/s1/artifacts/server-filesystem-2.1.0.tgz`, the published package of the
   filesystem server, which order 009 kept as a fixture: it carries no licence text. The three
   cases that hash it (`s1`, `s1-digest-mismatch` and `s1-missing` under
   `conformance/language/packages/`) take its SHA-256 as recorded data instead, with the fixture
   README saying how it was computed and from what (`npm pack` of the published version) and
   that the core never fetches it. Every archive the installation vectors use is one this
   repository makes.
5. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope.

## Done when

The archive vectors pass and S1's filesystem package resolves after installation exactly as
from its snapshot; nothing is fetched; every decision cites a promise; and the checks of order
004's done-when pass.
