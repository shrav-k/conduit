# Naming — accepted

Status: ruling 25: in force; ruling 27: in force; amended by 41 (item 21, NE) (ruled by the owner on 2026-11-05). Holds now: [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 6. Conduits and access](../../../IN_FORCE.md#6-conduits-and-access); [NAM-001 to NAM-028](../../../spec/promises/naming.yaml). This record says why.

Written as promises in the specification:
[naming](../../../spec/promises/naming.yaml), NAM-001 to NAM-017 as first written; later
rulings added more, and the comment at the head of that file lists them. This replaces decision
PD1 of the [packages batch](../packages-and-start/README.md). The evidence is the
[naming research](../../research/naming/README.md).

This is the record of a ruling. The specification in [`spec/`](../../../spec/README.md) governs
where they differ. Amended by:

- ruling 28: every selection of a type, a capability, an operation, a requirement or an
  implementation is a qualified reference; the core package is built in, needs no manifest item
  and cannot be replaced; an instance writes its values under `arguments`;
- ruling 32: where the two ends of a Conduit hold different versions, the target's contract
  governs; under one contract name a later release may only add optional things; `#name` refers
  to a document of the same project or package; a project that is not published may hold
  contracts;
- ruling 33: the manifest key that says where a package is fetched from is `fetch`, so that
  `source` is the key of a value reference only; a breaking release taking a new address is a
  `should`; the host part of an address is a lowercase DNS name with no port; and the
  specification's own names come from the repository, which replaces `conduit.dev`;
- ruling 37: the manifest's file and keys, among them that a manifest carries `apiVersion`; no
  two documents of one project share a name; it changed NAM-001, NAM-006, NAM-009, NAM-012,
  NAM-018, NAM-019 and NAM-022 and added NAM-027.

Promise IDs are as first written; the comment at the head of each spec file says where a removed
ID went.

## The problem is wider than one decision

The repository names things in three unrelated ways:

| What is named | How it is written today | Example from the fixtures |
| --- | --- | --- |
| Registered objects | Kind and name | `{kind: Resource, name: project-directory}` |
| Things a package exports | A local alias declared in the document, plus an export name | `plugins: {fixture: {package: fixture.core, release: 0.1.0}}`, then `{plugin: fixture, name: route.establish}` |
| Types, capabilities and requirements | A group, name and version | `{group: filesystem, name: local-directory, version: v0alpha1}` |

The goal is one coherent scheme, taken from existing practice, with nothing invented.

## What the research settles

- **Do not use a flat package namespace.** Terraform, Cargo and Ansible each had to retrofit
  namespaces.
- **Do not import a package's exports into the local namespace.** Ansible now steers authors
  away from its search path, and Compose's `include` has to report conflicts.
- **Without a registry, identity anchored in something the author owns keeps names unique.** Go,
  Kubernetes groups, OCI and the MCP registry all do this.
- **A constraint or pin in a project file, with hashes beside it, is the usual way to fix
  versions.** Go, Cargo, npm, OpenTofu and Helm all do this.
- **Contract identity is separate from the identity of what ships it** in Kubernetes and
  Backstage. In Go the same effect comes from the import path, which changes only on a breaking
  major version.
- **`/` cannot appear in an MCP tool name.** The current revision allows only letters, digits,
  `_`, `-` and `.`. An export name that may be exposed as a tool has to stay inside that set.

## Owner direction

| Point | Direction |
| --- | --- |
| Package identity | Address-like, as in Go. Conduit is defined by flexibility, and people will have entirely custom package setups. |
| References to a package's exports | Fully qualified. No per-document aliases. |
| Where versions are pinned | Once per project, as Go and Cargo do. The complication for instance documents is to be discussed. |
| Contracts with an identity independent of the package release | Open to it. |
| Syntax of a reference | Clean possibilities to be discussed. |

## What this consolidates

With an address for every package and fully qualified references, the three schemes become two:

| What is named | How it is written |
| --- | --- |
| A local object | Its kind and name, or the name alone where the field fixes the kind |
| Anything a package exports: a template, an implementation, or a contract such as a type, capability or requirement | The package's address and the export's name |

A reference never carries a release version; that lives in the project's file. So a reference to
a contract is already independent of the package release, which is the property Kubernetes gets
from its group, version and kind, and Go gets from its import path.

## Decisions

The owner accepted every recommendation.

| ID | Decision | Options | Accepted | Existing practice |
| --- | --- | --- | --- | --- |
| NA | How a fully qualified reference is written. | (1) `address#export`, as in `github.com/acme/conduit-history#shared-reviewer`. (2) A structured form, `{package: github.com/acme/conduit-history, name: shared-reviewer}`. (3) `address:export`. (4) The export as a further path segment, `github.com/acme/conduit-history/shared-reviewer`. | (1). It is the shape of a URL with a fragment, and of the `$ref` values already used in schemas. `#` never appears in a name or an address, and YAML reads it without quotes because a comment needs a space before the `#`. Option (4) cannot say where the address ends. | URL fragments; JSON Schema `$ref`; Nix flake references |
| NB | What file holds the versions. | (1) Every project, meaning the directory of documents that is submitted together, has one manifest listing each package it uses with one exact version and its digest. A package is the same thing published, and its manifest also lists its exports. (2) A manifest for the pins and a separate generated lock file for the digests. | (1). Pins are already exact, so a second file would only repeat them. | Go's `go.mod` and `go.sum`; Cargo's `Cargo.toml` and `Cargo.lock` |
| NC | How a contract carries its own compatibility version. | (1) In its export name, as in `conduit.dev/files#local-directory.v1`. A breaking change is a new name. (2) In the address, as Go does with a `/v2` suffix. (3) Keep a separate group, name and version form for contracts, as today. | (1). It needs no second scheme and no rule about addresses. Option (3) keeps two schemes. | Kubernetes `v1`, `v1beta1`; Go's major-version suffix |
| ND | What characters an export name may use. | (1) Those of an object name, plus `.` between segments. (2) Exactly the object-name rule. | (1). It keeps names such as `directory.establish`, allows `.v1`, and stays within what an MCP tool name permits. | MCP tool names |
| NE | Whether a package's address is also where it is fetched from. | (1) The address is the identity. By default it is also the source; a project's manifest may give a different source for an address, such as a local directory. (2) The address is always the source. | (1). It is what makes entirely custom setups possible without changing any document. | Go's `replace` directive; OpenTofu module sources |
| NF | What installing a package does to the registry. | (1) Nothing. (2) It registers the package's templates as local objects. | (1). | npm `exports`; Bazel's direct-dependency rule |

## The complication with a project-level file, and how each case resolves

| Case | Resolution |
| --- | --- |
| An instance document has no place to state a version. | It does not need one. Its `from` gives the address and export; the version comes from the manifest of the project it is submitted with. |
| A single document is submitted on its own. | It may refer only to local objects. A reference to a package with no manifest to pin it is rejected. |
| Two documents in one project want different versions of one package. | Not expressible. A project uses one version of a package, as a Go module does. |
| A package the project uses has its own dependencies. | The package's documents resolve through the package's own manifest, not the project's. This is already accepted: a package resolves its references in its own scope. |
| A submission was ruled to contain only templates and instances. | It may also contain one manifest. Promise SUB-005 changes accordingly. |
| A template was ruled to name an exact version for each dependency. | The exact version moves to the manifest. Promise PKG-002 changes accordingly; nothing else in the packages promises does. |

## Versions across connections

Raised by the owner after the direction above: Conduit is about connections, the two ends of a
Conduit may come from different versions of a package, and instances created at different times
run different versions. Compatibility has to be as wide as the requirements allow. The owner
asked whether packages need to be recorded directly in templates and in instances. The owner accepted the answers below (ruling 27).

What already holds:

- **An instance records its packages.** Promise PKG-005 has every instance hold the version and
  digest of each package it uses, fixed at creation, and PKG-008 keeps them from ever changing.
- **A connection is checked against contracts, never against package versions.** Opening a
  Conduit checks that the target offers the capability the Conduit names and supports its
  restrictions. Both are contracts.

What is missing or wrong:

| Gap | Proposed answer |
| --- | --- |
| "One version of a package per project" was offered as a limit. Read as a limit on what can run together, it would be a serious problem. | It limits only what one manifest can say. It places no limit on what is registered or running. |
| A template registered last month must keep meaning what it meant then, whatever the manifest says now. Nothing records its versions. | A template records the address, version and digest of each package it uses when it is registered. The manifest is where an author writes them; the registered template is where they live. Two templates registered at different times, or from different projects, can pin different versions of one package. |
| Nothing says several versions of a package can be installed together, though PKG-008 needs it. | State it: any number of versions of a package can be installed at once. |
| Nothing says how a contract evolves, which is what decides whether old and new ends can talk. | A contract version changes only compatibly. A breaking change is a new contract, which decision NC writes as a new name such as `.v2`. A type lists every contract version it offers, so a target can offer `.v1` and `.v2` together and serve consumers of both. |
| One project that needs two versions of a package at once. | Open; see the options below. |

Options for two versions inside one project:

| Option | How | Cost |
| --- | --- | --- |
| 1. A breaking version has a new address | As Go does with a `/v2` suffix. The two are then different packages and one manifest can list both. Non-breaking versions do not need to coexist. | Authors must change the address on a breaking release. |
| 2. A document may state its own version | The manifest gives the default; a template or instance may override it for itself. | Versions are written in two places. |
| 3. Versions are written in every document | No manifest. Each template and instance lists its packages and versions. | Verbose, and the opposite of the Go and Cargo style the owner preferred. |
| 4. Split the work into several projects | Each project is a directory with its own manifest. | None in the language; it is a matter of layout. |

Accepted: options 1 and 4 together. Neither adds a second place to write a version.

## What it would look like

The heading is as first written. The shapes were accepted with the decisions (ruling 27).

> Later rulings changed the shapes in the two examples below.
>
> - A manifest's `source` is now `fetch` (ruling 33; NAM-012). A manifest also carries
>   `apiVersion`, and its file name is fixed as `conduit.yaml` (ruling 37; MAN-001, MAN-003).
> - An instance writes its values under `arguments`, not `inputs` (rulings 28 and 29; OBJ-013).
> - `conduit.dev/files` stands for some package's address. `conduit.dev` is not a name of the
>   specification: its names are derived from the repository (ruling 33;
>   [`spec/names.yaml`](../../../spec/names.yaml)), and its own examples use `example.org`.
>
> The current worked example is
> [`spec/schemas/examples/valid/`](../../../spec/schemas/examples/valid/).

The manifest, with an illustrative file name of `conduit.yaml`:

```yaml
packages:
  github.com/acme/conduit-history:
    version: 1.4.0
    digest: sha256:0f3a...
  conduit.dev/files:
    version: 0.3.0
    digest: sha256:9b1c...
    source: ../files
```

Documents in the same project:

```yaml
kind: Agent
metadata: {name: reviewer}
from: github.com/acme/conduit-history#shared-reviewer
placement: laptop
inputs: {account: agent-account-a}
---
kind: Resource
metadata: {name: project-directory}
spec:
  type: conduit.dev/files#local-directory.v1
  setup:
    steps:
      establish:
        operation: conduit.dev/files#directory.establish
```

## Costs

- References are long. That is the price of having no aliases.
- A document cannot be read fully in isolation: the version of a package it uses is in the
  manifest beside it.
- A contract's version is part of its name, so two versions of a contract are, to the language,
  two different exports that happen to be related.

## Rules that would follow

The heading is as first written. These rules were accepted with the decisions (ruling 27) and
are written among the naming promises.

| ID | Rule |
| --- | --- |
| NR1 | An address begins with a host name that contains a dot, followed by path segments of lowercase letters, digits, `-`, `_` and `.`. |
| NR2 | A reference to an address that the project's manifest does not list is rejected. |
| NR3 | A package listed in the manifest that no document uses is reported and does not prevent registration. |
| NR4 | When an instance is created, each address it uses is resolved through its project's manifest to a version and digest, which the instance holds. This is promise PKG-005. |
| NR5 | A local name never contains `#` or `/`, so a reference is local exactly when it contains neither. |
