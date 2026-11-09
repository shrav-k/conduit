# S1 snapshots

Not binding. These are preinstalled package snapshots and a submitted project's documents,
for [the context vector](../../language/packages/s1/case.yaml). They do not install, register,
start or qualify an implementation. The [study's drafts](../../../docs/proposals/first-build/increment-2/s1/)
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
| [server-filesystem-2.1.0.tgz](artifacts/server-filesystem-2.1.0.tgz) | `65aba6b962efa7fdfdd51d70b48c06e7786c8bbc57292520dec21924836031df` |

The Conduit archives hold exactly each package directory's two YAML files at the archive
root. They use USTAR, sorted file names, mode 0644, zero uid/gid/mtime, empty owner names,
and gzip level 9 with mtime zero and no filename. Their bytes are evidence for the supplied
snapshot identities. The runner reads the documents beside them and verifies their archive
hashes; it does not assert an archive installation.

The npm artifact was already present in the local npm cache. Its SHA-512 matched the cache's
content address, and its `package/package.json` identified
`@modelcontextprotocol/server-filesystem` version `2.1.0`. No network was used to obtain it.
Its SHA-256 replaces the draft's illustrative `fileSha256`; the runner verifies the retained
artifact against that pin. As MAN-010 says, this pins the artifact, not its dependency graph.
No provider is launched by these vectors.

The history manifest and project manifest pin the Conduit archive digests above. All of these
values are computed evidence, not illustrative placeholders. Archive parsing, safety checks,
installation outcomes and fetching completions belong to the second part of I2-03.
