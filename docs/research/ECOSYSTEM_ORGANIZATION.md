# Repository, extension and distribution patterns

Research date: 2026-11-02. This compares project organization, publication and documentation;
it does not select dependencies or reopen the accepted object model. Recommendations are in the
[repository proposal](../archive/reviews/REPOSITORY-2026-11-02.md). None is adopted merely by appearing here.

## Method and limits

Reviewed Conduit's tracked Markdown inventory, document roles/headings, entry points, existing
organization proposal and selected cross-cutting contracts. Ran the existing documentation checker.
This is a structural review, not a fresh line-by-line semantic audit of every historical report or
revalidation of all its external sources. The baseline is commit `4c54263`.

For the external comparison, inspected primary repositories, selected manifests and published
documentation. GitHub source revisions below were resolved during this investigation; websites
are observations on the research date. No compared project was installed or benchmarked. The
older Pi investigations keep their original pins; this report does not replace their evidence.

## Comparison

Each lesson below is an inference for Conduit, not a claim that the source project recommends it.

| Project or ecosystem | Observed organization | Useful lesson for Conduit | Boundary not to copy automatically |
| --- | --- | --- | --- |
| Pi | One workspace contains the application and separately published libraries. Package manifests expose their own APIs; the release script checks lockstep package versions. | Develop related pieces together while making their consumable boundaries explicit. | A monorepo does not make every library independent of its dependencies. Neither Pi's runtime nor its lockstep version policy is a Conduit requirement. |
| MCP | Specification, schema and documentation share a repository; conformance tooling lives separately. | Public behavior and interoperability tests should be accessible without reading reference-runtime internals. | Separate repositories are an organizational choice, not proof of protocol independence. MCP's TypeScript-first schema generation is not our accepted language-neutral authority rule. |
| OpenTelemetry | Collector core, contributed components and release distributions have distinct source areas/repositories. Distribution manifests select components; a builder can assemble custom distributions. | Distinguish extension packages from the convenient collection shipped to users. | Do not require a Go-style static rebuild to install every Conduit extension, or start with a large contrib organization. |
| Kubernetes | The main repository stages libraries that are published to separate repositories; the staging source remains authoritative. | Source development boundaries and published package/repository boundaries need not coincide. | Publishing bots, staging mirrors and project governance at Kubernetes scale would add maintenance before we need it. |
| Docker Compose / Compose Specification | The application model/schema and Docker's implementation occupy separate repositories. | Give portable configuration an identity distinct from its first executor. | A separate spec repository is not required to establish that distinction now; Compose's deployment/defaulting semantics are not automatically ours. |
| containerd | Its scope document emphasizes independently useful components, shared primitives, replacement points and defaults. The source distinguishes APIs, clients, commands, plugins and internal implementation. | Let an integration or client use supported capabilities without requiring every frontend or subsystem. | Do not turn each Conduit family into a daemon, crate or independently deployed service. |
| OCI | Distribution has its own specification and conformance tooling, distinct from image and runtime specifications. | Separate what a package means, how it executes and how its bytes are delivered. | OCI hosting can be a later transport option; choosing it would not require containers as the plugin execution model. |
| OpenTofu | Provider discovery/download metadata is explicitly separate from the provider runtime protocol. The registry protocol carries release, platform, protocol and checksum information. | Package identity, compatibility and artifact selection deserve attention before field spellings. | Avoid adopting its registry address syntax, whole dependency resolver or provider-only executable model. |
| Wasmtime / Component Model | Wasmtime has CLI and library surfaces in a Cargo workspace. WIT describes component interfaces and imports/exports, while the runtime is a separate implementation concern. | Distinguish public contracts from internal Rust modules and a complete user-facing application. | WIT itself does not define behavioral guarantees; neither its syntax nor Wasm is required for our operation/recovery contracts. |
| uv | A multi-crate Rust workspace ships a user-facing tool through several installation channels. Its docs separate guides, concepts, reference and internals. | Internal modularity can coexist with a straightforward installation experience. | We do not need every installation channel or a separately supported public API for every crate. |

Primary evidence for the rows:

- **Pi:** [workspace manifest](https://github.com/earendil-works/pi/blob/642e30b68930a644050794011555a04ed9027eb4/package.json),
  [AI package](https://github.com/earendil-works/pi/blob/642e30b68930a644050794011555a04ed9027eb4/packages/ai/package.json),
  [release synchronization](https://github.com/earendil-works/pi/blob/642e30b68930a644050794011555a04ed9027eb4/scripts/sync-versions.js).
- **MCP:** [specification repository](https://github.com/modelcontextprotocol/modelcontextprotocol/tree/d0e246ea3c69eafaa2806bc4cd28025703ba2c2b),
  [conformance framework](https://github.com/modelcontextprotocol/conformance/tree/c37eec888e1c6ff140af79987a40008548b7cc5f).
- **OpenTelemetry:** [core](https://github.com/open-telemetry/opentelemetry-collector/tree/3e3833c1bc14e8997839e3d1d454f4650d20a7e7),
  [contrib](https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/3bafad9e2ae7ce6c8885aeab770cf1ff5acea04d),
  [distribution manifest](https://github.com/open-telemetry/opentelemetry-collector-releases/blob/3e02cb7515551f9b9ac2302dc8e2a15e7eaada81/distributions/otelcol/manifest.yaml),
  [custom distribution builder](https://opentelemetry.io/docs/collector/extend/ocb/).
- **Kubernetes:** [staging source and publication rules](https://github.com/kubernetes/kubernetes/blob/de06dce3c02d5731a0dc4161861b9c34b1444280/staging/README.md).
- **Compose:** [specification](https://github.com/compose-spec/compose-spec/tree/3a4a6995e4a971f6173dff9130cb3e38e6738651),
  [Docker implementation](https://github.com/docker/compose/tree/5bceecf985347fcf6dd23029ce440b502a0d4301).
- **containerd:** [scope](https://github.com/containerd/containerd/blob/8a9c72699b5a29daa94cbf1bfe270f1bd1b52d12/SCOPE.md),
  [source tree](https://github.com/containerd/containerd/tree/8a9c72699b5a29daa94cbf1bfe270f1bd1b52d12).
- **OCI:** [distribution specification and conformance](https://github.com/opencontainers/distribution-spec/tree/97274622c11112caa21efb8c52acca3c6b8fa7f1).
- **OpenTofu:** [registry protocol](https://opentofu.org/docs/v1.9/internals/provider-registry-protocol/),
  [plugin protocol](https://github.com/opentofu/opentofu/blob/b767a5aefc8998252858201189c2593c5aa9744d/docs/plugin-protocol/README.md).
- **Wasmtime:** [workspace and CLI](https://github.com/bytecodealliance/wasmtime/blob/4c1c3f3404e85d2838f3ebfdbf957d458139cb55/Cargo.toml),
  [library](https://github.com/bytecodealliance/wasmtime/blob/4c1c3f3404e85d2838f3ebfdbf957d458139cb55/crates/wasmtime/Cargo.toml),
  [WIT reference](https://component-model.bytecodealliance.org/design/wit.html).
- **uv:** [workspace](https://github.com/astral-sh/uv/blob/60f4e951a5a76ebce880de94fe5d950b5ae667cc/Cargo.toml),
  [installation and documentation navigation](https://docs.astral.sh/uv/getting-started/installation/).

## Websites and author journeys

Pi's current site offers entry routes for running, customizing and building on Pi, followed by
reference pages. Its source contains a documentation navigation file, and its package guide covers
distribution through existing npm/Git/local mechanisms. This makes authoring discoverable alongside
ordinary use. [Pi documentation](https://pi.dev/docs/latest) ·
[navigation source](https://github.com/earendil-works/pi/blob/642e30b68930a644050794011555a04ed9027eb4/packages/coding-agent/docs/docs.json) ·
[package guide](https://github.com/earendil-works/pi/blob/642e30b68930a644050794011555a04ed9027eb4/packages/coding-agent/docs/packages.md).

For Conduit, use those audience distinctions while adding a first-class protocol-implementer path.
A package author should reach a small complete example without first studying executor persistence.
An implementer needs behavioral contracts and conformance cases, not just a product tutorial.
Diátaxis provides a useful distinction between tutorials, task guides, reference and explanation;
it does not prescribe repository count or a website framework. [Diátaxis](https://diataxis.fr/).

Keep research available from contributor/design context, with dates and acceptance status. It should
not be interleaved with current reference pages as though every proposal were supported behavior.
Publish reviewed development documentation first; add release-specific views once real releases
exist. Repository prose and website prose should share sources rather than drift as editable copies.

## Conclusions to test, not rules already adopted

1. One source repository can produce several public artifacts. Repository, crate, package,
   executable, process and object-instance boundaries answer different questions.
2. Public protocol independence needs accessible contracts, explicit capability claims and tests
   against external clients/plugins. Merely moving the spec to another repository does not supply it.
3. A standard distribution can bundle a convenient working selection without making its contents
   mandatory for every compatible implementation.
4. Plugin runtime communication and package delivery are distinct protocols. MCP does not decide
   package identity, dependency resolution, releases or executable retention.
5. Website navigation should follow user and author tasks; the repository should follow authority,
   ownership and build/release boundaries. They need not have identical trees.

The [proposal](../archive/reviews/REPOSITORY-2026-11-02.md) applies these lessons and records the decisions still needed.
