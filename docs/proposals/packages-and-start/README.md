# Packages, first start and conformance — accepted

Status: the owner accepted everything here on 2026-11-05 (plan-audit ruling 24, action A54) except
decision PD1, on how a template in a package is referred to, which was held for more research.
PD1 has since been settled by the naming rulings (plan-audit rulings 25 and 27;
[naming record](../naming/README.md)), so nothing in this record is open. It is written as
promises in the specification: [packages](../../../spec/promises/packages.yaml),
[naming](../../../spec/promises/naming.yaml),
[first start](../../../spec/promises/first-start.yaml) and the conformance section of the
[specification's README](../../../spec/README.md#conformance). Spellings follow the
[ruled vocabulary](../vocabulary/README.md).

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- rulings 25 and 27: PD1 is settled. A reference to a package's export is the package's address,
  `#` and the export's name, and installing a package registers nothing. Exact versions are
  written once, in the project's manifest, and never in a template or a reference, which
  changes PR1 and PR4;
- rulings 28 and 29: what an implementation needs of a location is a requirement on placement,
  which changes PR6; the first-start objects are instances of templates that the core package
  exports;
- ruling 31: the `implementation` map is keyed by operation only, since a runtime is a pair of
  operations and a control is an operation;
- the re-audit that ruling 32 accepted: the second conformance level is called Engine, not
  Host, because "host" is the machine a location is on (a finding of its second reader, fixed
  without a ruling of its own); several promises of this batch were folded into others;
- ruling 33: the core package's address comes from the repository;
- ruling 37: the manifest's format, and changes to PKG-005, PKG-013, PKG-017 to PKG-020 and
  PKG-022.

Promise IDs are as first written; the comment at the head of each spec file says where a removed
ID went.

This is the fifth and last batch of fixes in the specification's two layers. The owner ruled
that the package system is not what differentiates Conduit, and instructed that graphs and
schemas rely on existing frameworks. This batch applies the same instruction to packages: each
rule names the existing practice it follows.

Defects covered, from the [register](../../LANGUAGE_DEFECTS.md): L27, L28, L29, L30, L35, L37,
L38, L44, L45, L46, L47 and L48.

## Decisions

PD2 to PD5 are accepted. PD1 was open when this was written, and the "Recommendation" column
below is as it stood then.

> Later rulings changed two rows below.
>
> - PD1 is settled by rulings 25 and 27: option (a) in substance, written as the package's
>   address, `#` and the export's name, such as
>   `github.com/acme/conduit-history#shared-reviewer` (NAM-002, NAM-013).
> - PD5: the second level is called Engine, and it is an engine that claims conformance. The
>   Engine level also asks for a runtime profile
>   ([Conformance](../../../spec/README.md#conformance)).

| ID | Decision | Options | Recommendation | Existing practice | Defects |
| --- | --- | --- | --- | --- | --- |
| PD1 | How a template in a package is referred to. | (a) By a qualified name, the package's name and the export's name, such as `history/shared-history`. Installing a package registers nothing. (b) Installing a package registers its templates under their own names, in the same namespace as local templates. | (a). Under (b) two packages that export the same name collide, and installing has an effect on the registry. | Qualified names in Terraform's registry, Go import paths and Kubernetes API groups | L27 |
| PD2 | How a package version is written and matched. | (a) Semantic Versioning 2.0.0, and a dependency names one exact version. No ranges and no "latest". (b) Opaque strings compared exactly. | (a). The format is the standard one; the exact pin keeps the accepted rule that nothing is substituted. | SemVer; a lockfile's exact pins | L35, L45 |
| PD3 | What a package's digest is computed over. | (a) The SHA-256 of the package's archive file, exactly as distributed. (b) A canonical listing of its files defined by Conduit. | (a). It needs no canonical form of our own, and any tool can check it. | OCI image layers; npm and Cargo checksums | L48 |
| PD4 | What exists after first start. | (a) A fixed set with fixed names: the Pool `system`, the location `local`, the two management Resources `management` and `access`, and the Interface `root`, with its Conduits to both. The names are reserved. (b) The same set with names chosen at first start. | (a). Declarations refer to these by name, so they must be the same everywhere. | Kubernetes's `default` and `kube-system` namespaces | L29 |
| PD5 | What an implementation may claim to conform to. | (a) Two levels, each naming the specification version. "Language": every language-layer promise. "Host": that, plus the shared conventions and the implementation interface. A declaration that uses something an implementation does not support is rejected at registration. (b) A free list of supported promises per implementation. | (a). Two levels match the two layers and what the independent builds cover. | Conformance levels in OCI and other standards | L46 |

## Rules

Accepted as a group, except the half of PR8 that concerns the separator in qualified names, which
waited on PD1.

> Later rulings changed four rules below.
>
> - PR1: a template names no version. The project's manifest names exactly one version of each
>   package, and a registered template keeps the versions and digests the manifest gave
>   (ruling 27; NAM-005, NAM-006, NAM-009).
> - PR4: a default is a qualified reference and carries no version (ruling 27; NAM-005,
>   TPL-004). Only operations have implementations selected (ruling 31; TPL-003).
> - PR6: there are no location capabilities. An implementation states its requirements on
>   placement in its manifest item (rulings 28 and 32; PKG-019, PLC-007).
> - PR8: the separator is settled. A local name contains neither `#` nor `/` (ruling 27;
>   NAM-003, OBJ-001).

| ID | Rule | Existing practice | Defects |
| --- | --- | --- | --- |
| PR1 | A template's dependencies name a package and an exact version. When an instance is created, each selection is resolved to that package's version and digest, which the instance then holds. | A lockfile | L28, L35 |
| PR2 | A template whose dependency is not installed is registered, with the missing dependency reported. An instance cannot be created from it until the dependency is installed. | The reference rule already accepted for templates | L38 |
| PR3 | Installing a package later never changes an existing instance. An instance uses the version and digest it holds, or reports that they are unavailable. | A lockfile | L38 |
| PR4 | Every operation, runtime and control that a type requires has exactly one implementation selected, either by the template or by the type's stated default. Two selections for one of them reject the template. A type's default names its package and version like any other selection. | — | L37 |
| PR5 | A re-export resolves to the package that originally exports it, and an instance records that package. A package whose re-exports form a cycle is rejected when it is installed. | Module re-exports | L44 |
| PR6 | An implementation states the location capabilities it needs, as a type does, and they are checked when an instance is created. | The placement rule already accepted for types | L30 |
| PR7 | An implementation's behavior for one instance does not depend on its having served another. Whether an engine runs one process for many instances or one each is its runtime profile. | Stateless request handlers | L47 |
| PR8 | A name declared in a document may not be one of the reserved first-start names, and may not contain the separator used in qualified names. | — | L29 |

## Where each is written

| Decision or rule | Promises |
| --- | --- |
| PD1 | Open when this was written; settled by the [naming rulings](../naming/README.md) |
| PD2 | PKG-001, PKG-002, PKG-003 |
| PD3 | PKG-004 |
| PD4 | FST-001, FST-002, FST-003 |
| PD5 | The conformance section of the specification's manifest |
| PR1 | PKG-005 |
| PR2 | PKG-006, PKG-007 |
| PR3 | PKG-008 |
| PR4 | PKG-009, PKG-010, PKG-011 |
| PR5 | PKG-013, PKG-014 |
| PR6 | PKG-015 |
| PR7 | PKG-016 |
| PR8 | FST-004 for reserved names; the separator waits on PD1 |
| Accepted earlier: export names are unique within a package | PKG-012 |

## Not in this batch

- Where a package's files must be present, how they are fetched and when they are removed: the
  runtime profile.
- Local development of a package that has not been published, which the earlier text calls edit
  mode: the runtime profile, except that a published package always has a version and a digest.
- What first start does when it is interrupted or repeated, and how repair works: the runtime
  profile.
