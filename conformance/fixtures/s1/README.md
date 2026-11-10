# S1 package fixtures

Not binding. These are preinstalled package snapshots and a submitted project's documents,
for [the context vector](../../language/packages/s1/case.yaml). That context vector does not install, register,
start or qualify an implementation. The [archive vector](../../language/archives/s1/case.yaml)
also uses these repository-authored archives as installation inputs, then resolves the same
project against the resulting installed catalog. The [study's drafts](../../../docs/proposals/first-build/increment-2/s1/)
remain their historical source; this order's scope permits promoting copies here, not deleting
those drafts.

The filesystem package exports its three Operations, Capability and Resource Type. The history
Type reuses that package's implementation for a separate directory, with one file per post;
its description states the overwrite, ordering and authorship limits. The Agent Types remain
registration-only, with no implementation or model work. The project has nine templates and
six instance documents. Its paths are supplied values, and no directory is created.

## Computed digests

Computed on 2026-11-09 from the retained bytes:

| File | SHA-256 |
| --- | --- |
| [filesystem.tar.gz](archives/filesystem.tar.gz) | `200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160` |
| [history.tar.gz](archives/history.tar.gz) | `e946746509490be49cbcc07d41ff5fed24a6e6c8b377bf423c77f79c02502f69` |
| [agents.tar.gz](archives/agents.tar.gz) | `3b73fe853164e58125d960da62795f748b88c06a6eefe86046bbef00bc08b44a` |

The Conduit archives hold exactly each package directory's two YAML files at the archive
root. They use USTAR, sorted file names, mode 0644, zero uid/gid/mtime, empty owner names,
and gzip level 9 with mtime zero and no filename. Their bytes are evidence for the supplied
snapshot identities. The runner reads the documents beside them and verifies their archive
hashes; it does not assert an archive installation.

The filesystem implementation's `fileSha256` is recorded data:
`65aba6b962efa7fdfdd51d70b48c06e7786c8bbc57292520dec21924836031df`. It is the SHA-256
of the published npm package `@modelcontextprotocol/server-filesystem` version `2.1.0`, the
`server-filesystem-2.1.0.tgz` archive that `npm pack @modelcontextprotocol/server-filesystem@2.1.0`
produces. The value was computed on 2026-11-09 from a locally cached copy of that published
archive, whose SHA-512 matched its cache content address and whose `package/package.json`
identified that exact package and version. No network was used for that computation.

Order 013 removed the retained npm tarball because it contains no licence text. The resolver
and registration fixtures compare the implementation pin with that recorded SHA-256, without
claiming to rehash bytes they no longer hold. The core never fetches it. As MAN-010 says, this
pins the artifact, not its dependency graph. Every archive used for installation contains only
material this repository makes; no provider is launched by these vectors.

The history manifest and project manifest pin the Conduit archive digests above. All of these
values are computed evidence, not illustrative placeholders. Fetching is supplied as a
completion input; it is never performed by the core or these fixtures.
