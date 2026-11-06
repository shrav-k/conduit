# Naming and reference patterns in existing systems

> Not binding. This research is evidence for the naming rulings, 25 and 27 of the [plan audit](../../AUDIT.md#ruling-log). What it calls the naming proposal is now the accepted [naming record](../../proposals/naming/README.md), and the rules in force are in [`spec/promises/naming.yaml`](../../../spec/promises/naming.yaml).

Retrieved 2026-11-05 from the primary documentation listed at the end, for
[plan-audit action A55](../../AUDIT.md#p0--make-the-language-concrete-and-complete). This is
evidence for the [naming proposal](../../proposals/naming/README.md); it recommends nothing.

Limits: most quotations were extracted through a summarizing fetch tool, so wording should be
re-checked against the source before it is quoted anywhere binding. Items that could not be
verified are listed under [Not verified](#not-verified). No pages were pinned to a commit.

## Comparison

| System | Identity, with an example | How something inside is referenced | Local alias | Where the version is pinned | Two packages exporting one name |
| --- | --- | --- | --- | --- | --- |
| OpenTofu and Terraform | Provider `namespace/type` with an optional host in front, e.g. `hashicorp/aws`. Registry module `namespace/name/provider`. | `module.<name>.<output>` through the local label; `//` selects a subdirectory of a source. | Yes: the key in `required_providers` and the label of a `module` block. | A constraint in the configuration; the exact provider version and checksums in `.terraform.lock.hcl`. The lock file covers providers only. | Local names are unique per module; compound local names are recommended for two providers of one type. |
| Kubernetes | `apiVersion: batch/v1` (group and version) plus `kind`. A custom resource definition is named `<plural>.<group>`, e.g. `crontabs.stable.example.com`. | By API group, resource type, namespace and name; reference fields are named `{field}Ref`. | No. | In the `apiVersion` of each object. | The group disambiguates. |
| Helm | Chart `name` plus a `repository` URL. | A parent sets a subchart's values under a key named after the subchart. | Yes: the `alias` field of a dependency. | `version` per dependency in `Chart.yaml`; a generated lock file fixes exact versions. | An alias lets one chart appear several times. |
| OCI and Docker | A name, then an optional `:tag`, then an optional `@digest`. The name may begin with a domain. | Not applicable. | No. | Inline, as a tag or a digest. | The registry host and path. |
| Docker Compose | None; file paths. `include` also accepts OCI artifacts and Git repositories. | A bare service name; `extends` takes a file and a service. | No. | None. | `include` merges into one flat model and reports a conflict. |
| Go modules | The module path, e.g. `example.com/mod/v2`; its first element must contain a dot. | An import path, then `package.Identifier`. | Yes, per file: `import m "lib/math"`. | `go.mod` requirements; `go.sum` holds SHA-256 hashes. | Paths are globally unique; a new major version changes the path. |
| Cargo | A flat package name. | Rust paths with `::`. | Yes: `bar = { package = "foo", ... }`. | A requirement in `Cargo.toml`; exact versions in `Cargo.lock`. | Renaming, to depend on two versions or on same-named crates from two registries. |
| npm | `@scope/name`. | Subpaths listed in `exports`; anything not exported is unreachable. | Yes: `"alias": "npm:pkg@1.0.0"`, for the project only. | A range in `package.json`; the exact tree and integrity hashes in `package-lock.json`. | Scopes; aliases allow versions side by side. |
| GitHub Actions | `owner/repo@ref` or `owner/repo/path@ref`. | A path segment in `uses`. | No. | Inline in every `uses`; a commit SHA is described as the safest. | The owner and repository. |
| Ansible | `namespace.collection`; content is `namespace.collection.name`. | The fully qualified name, or a short name found through a `collections:` search path. | No; only the search path. | `requirements.yml`, with ranges and a default of latest. | The fully qualified name. With search paths, a module from an earlier collection can be used unintentionally. |
| Backstage | An optional `kind:` prefix, an optional `namespace/` prefix, then the name. | The same string. A structured `{kind, namespace, name}` form exists but is discouraged in protocols. | No; omitted parts default by context. | Not applicable. | Unique per kind within a namespace. |
| Bazel | `@repo//package:target`. | A label. | Yes: `bazel_dep(repo_name=...)`. | `bazel_dep(version=...)`, with a lock file. | Names are per repository; only direct dependencies are visible. |
| Model Context Protocol, revision 2026-07-28 | A tool name of 1 to 128 characters from letters, digits, `_`, `-` and `.`, unique within a server. A registry server name such as `io.github.my-username/weather`. | `tools/call` with the bare name. | On the client: Claude Code uses `mcp__<server>__<tool>`. | The protocol revision travels with each request. | The specification says a client aggregating servers should prefix tool names with a server identifier. |

## Contract identity and the identity of what ships it

Kubernetes identifies a contract by group, version and kind, independently of the software that
serves it. Versioning is "at the API level rather than at the resource or field level"; groups
evolve independently and are owned through DNS; one custom resource definition serves several
versions at once, with conversion. API versions and software versions are described as
"indirectly related".

Backstage copies this split. The Model Context Protocol has a protocol revision that is separate
from any server's own version. Go is a partial case: the import path is the compatibility
identity and changes only on a breaking major version. OpenTofu, Helm, Cargo, npm, GitHub
Actions, Ansible and Bazel tie what they export to the package release.

## Lessons

1. **A dependency has three parts in most systems:** a global source identity, a local name and
   a version. OpenTofu, Cargo, npm, Helm and Bazel all have this shape, and the local name is
   always scoped to the declaring module or project.
2. **Flat global namespaces get retrofitted.** Terraform 0.13 added namespaces and needed a
   migration for old state. Cargo's RFC 3243 adds namespaces because a crate named
   `projectname-foo` is not necessarily maintained by that project. Ansible 2.10 needed redirect
   routing and warns of duplicate module names.
3. **Implicit search paths are discouraged after the fact.** Ansible now prefers the fully
   qualified name over the `collections` keyword. Compose's `include` merges into one flat model
   and has to report conflicts.
4. **Separator collisions are a documented cost.** Cargo dropped `/` for namespaces because
   slashes are ambiguous in feature specifiers. A proposal to allow `/` in MCP tool names was not
   adopted; the current revision allows only `_`, `-` and `.`. Claude Code's `__` delimiter is
   ambiguous for a server whose name contains it.
5. **Mutable references are the recurring pitfall.** GitHub: a tag can be moved or deleted, so
   pin to a SHA. Docker: no tag means `latest`. OpenTofu: unlocked modules take the newest match.
6. **A constraint in the manifest and an exact version with a hash in a lock file** is the usual
   split (Go, Cargo, npm, OpenTofu, Helm). Inline pinning in every reference is used where there
   is no project manifest (GitHub Actions, OCI).
7. **Identity anchored in DNS or an owner gives uniqueness without a registry.** Go's first path
   element must contain a dot; Kubernetes groups are DNS subdomains; OCI names begin with a
   domain; the MCP registry requires proof of ownership of a namespace such as `io.github.<user>`.
8. **Explicit export lists encapsulate.** With npm's `exports`, everything not listed is
   unreachable. Bazel exposes only direct dependencies.

## Separators in use

| Separator | Used for |
| --- | --- |
| `/` | OpenTofu host, namespace and type; Kubernetes group and version; OCI paths; Go paths; npm scope and subpaths; GitHub Actions owner, repository and path; Backstage namespace and name; Bazel package path; MCP registry namespace and name |
| `.` | Kubernetes custom resource names and DNS groups; Ansible fully qualified names; Go `package.Identifier`; OpenTofu `module.name.output`; Helm values; permitted inside MCP tool names and Backstage names |
| `:` | OCI tags and digest algorithm; Backstage kind prefix; Bazel target; npm `npm:` protocol |
| `@` | OCI digest; GitHub Actions ref; npm scope prefix and `name@version`; Bazel repository |
| `#` | npm internal imports and Git commit references |
| `::` | Rust paths; the proposed crates.io namespaces |
| `__` | Claude Code's MCP tool prefix |

## Not verified

- Helm: the lock file's name and contents, and whether an aliased subchart's values are keyed by
  the alias.
- Kubernetes: that a custom resource definition's name must be a valid DNS subdomain.
- Compose: one page says a conflict in `include` is an error and another says it is a warning.
- OCI: the grammar is from the `distribution/reference` source, not the distribution
  specification.
- Bazel: the lock file's name. Nix was not covered.
- MCP: no character rules for server names were found in the specification, and no formal
  grammar for registry names.
- No primary-source statement of regret was found for npm's unscoped namespace.

## Sources

| Subject | URL |
| --- | --- |
| OpenTofu provider requirements | <https://opentofu.org/docs/language/providers/requirements/> |
| OpenTofu dependency lock file | <https://opentofu.org/docs/language/files/dependency-lock/> |
| OpenTofu module syntax and sources | <https://opentofu.org/docs/language/modules/syntax/>, <https://opentofu.org/docs/language/modules/sources/> |
| Terraform 0.13 upgrade guide | <https://developer.hashicorp.com/terraform/language/v1.1.x/upgrade-guides/0-13> |
| Kubernetes names, API, API usage | <https://kubernetes.io/docs/concepts/overview/working-with-objects/names/>, <https://kubernetes.io/docs/concepts/overview/kubernetes-api/>, <https://kubernetes.io/docs/reference/using-api/> |
| Kubernetes custom resource definitions and versioning | <https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/>, <https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/> |
| Kubernetes API conventions | <https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md> |
| Helm charts and dependencies | <https://helm.sh/docs/topics/charts/>, <https://helm.sh/docs/helm/helm_dependency/>, <https://helm.sh/docs/helm/helm_dependency_update/> |
| OCI reference grammar | <https://github.com/distribution/reference/blob/main/reference.go> |
| Docker pull and tagging | <https://docs.docker.com/reference/cli/docker/image/pull/>, <https://docs.docker.com/get-started/docker-concepts/building-images/build-tag-and-publish-an-image/> |
| Docker Compose include and services | <https://docs.docker.com/reference/compose-file/include/>, <https://docs.docker.com/compose/how-tos/multiple-compose-files/include/>, <https://docs.docker.com/reference/compose-file/services/> |
| Go modules and specification | <https://go.dev/ref/mod>, <https://go.dev/ref/spec> |
| Cargo dependencies, manifest and lock file | <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html>, <https://doc.rust-lang.org/cargo/reference/manifest.html>, <https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html> |
| Rust RFC 3243, packages as optional namespaces | <https://rust-lang.github.io/rfcs/3243-packages-as-optional-namespaces.html> |
| npm scopes, package.json, lock file, install | <https://docs.npmjs.com/cli/v11/using-npm/scope>, <https://docs.npmjs.com/cli/v11/configuring-npm/package-json>, <https://docs.npmjs.com/cli/v11/configuring-npm/package-lock-json>, <https://docs.npmjs.com/cli/v11/commands/npm-install> |
| Node.js packages | <https://nodejs.org/api/packages.html> |
| GitHub Actions workflow syntax and secure use | <https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax>, <https://docs.github.com/en/actions/reference/security/secure-use> |
| Ansible collections | <https://docs.ansible.com/ansible/latest/collections_guide/collections_using_playbooks.html>, <https://docs.ansible.com/ansible/latest/porting_guides/porting_guide_2.10.html> |
| Backstage references and descriptor format | <https://backstage.io/docs/features/software-catalog/references/>, <https://backstage.io/docs/features/software-catalog/descriptor-format/> |
| Bazel labels and external dependencies | <https://bazel.build/concepts/labels>, <https://bazel.build/external/overview> |
| MCP versioning, tools, tool-name proposal, registry | <https://modelcontextprotocol.io/specification/versioning>, <https://modelcontextprotocol.io/specification/2026-07-28/server/tools>, <https://modelcontextprotocol.io/seps/986-specify-format-for-tool-names.md>, <https://github.com/modelcontextprotocol/registry/blob/main/docs/modelcontextprotocol-io/quickstart.mdx> |
| Claude Code MCP and permissions | <https://code.claude.com/docs/en/mcp>, <https://code.claude.com/docs/en/permissions> |
