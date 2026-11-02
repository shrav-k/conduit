# Repository, documentation and distribution review

Status: **repository boundary and common package envelope accepted; remaining proposals under review**, 2026-11-02.
One main repository under a Conduit GitHub organization and a common package envelope are accepted. The organization name/transfer,
directory layout, release scheme, license and package syntax remain unsettled. The completed
[conceptual review](../REVIEW.md) remains complete; this review precedes the authoring/conformance PR.

The documentation discussion establishes the language and public contracts as the primary product.
The runtime and a thin initial plugin/template set demonstrate it; richer supported packages may
move to ancillary repositories. Exact navigation and the website implementation remain proposals.

**Accepted repository boundary:** keep one main development repository under a Conduit GitHub
organization initially, with explicit public boundaries so the protocol/conformance kit, reference
runtime and extension packages remain independently consumable. This allows a contract, example,
conformance test and implementation change to be reviewed together. It does not require one process,
one installation containing everything or a common version for every package. Packages use a common
envelope with typed contents, including template-only collections. Exact publication artifacts,
package syntax and release mechanics remain open.

The [ecosystem comparison](../research/ECOSYSTEM_ORGANIZATION.md) covers Pi, MCP, OpenTelemetry,
Kubernetes, Compose, containerd, OCI, OpenTofu, Wasmtime/Component Model and uv. This proposal builds
on their different strengths rather than treating one project as Conduit's blueprint.

## 1. Current repository audit

At `4c54263`, the repository has 43 Markdown files, about 134,000 words and no runtime implementation.
Approximately 57,000 words are research and 41,000 are proposals. The detailed accepted design is
about 22,000 words. These are whitespace-based counts, not quality or completeness scores.

| Area | Assessment and proposed treatment |
| --- | --- |
| README, SPEC, DESIGN and seven design topics | Preserve the concise overview and one authoritative detailed home for each rule. These already separate the five families from the concepts beneath them. |
| REVIEW and implementation REVIEW | Completed reviews should stay marked complete. Route this discussion to this proposal; do not resurrect every historical question. Later shorten retrospective summaries when they merely repeat canonical rules. |
| Implementation proposals | Valuable reasoning, but mixed accepted boundaries and detailed candidates. The old ORGANIZATION tree is an earlier proposal, not a commitment to eight crates or a single built-in package. |
| Authoring proposals | Preserve the directory, MCP, history and graph examples for the syntax pass. Do not promote their aliases, filenames or API spellings merely by moving them into a public-looking directory. |
| Research | Keep source pins, failure examples and actual probes. Organize the index by question; consolidate repeated recommendations only after their useful evidence has a destination. Historical wording is not an active ruling queue. |
| Experiments | Keep outside docs, with their own instructions and evidence. 001 is draft configuration, 002 is an unexecuted runtime proof; neither belongs in a runnable quickstart yet. |
| Archive | Keep attributed collaborator proposals and dissent separate from current rules. Old alternatives do not need to be repeated in the primary reading path. |
| Contributor/release entry points | No tracked license, contribution guide, security policy or CI configuration was found. These need deliberate choices as implementation and external contributions begin. |

The existing checker passed 359 local links, Markdown fences/anchors and 111 pinned Pi source paths.
That does not validate all external sources or every semantic assertion. The structural review also
found stale references to an unfinished I3 review in the old organization proposal and a single-store
installation sketch that needs clearer service-local ownership; these are documentation corrections,
not new policy decisions.

The main improvement is a shorter reading path for each audience, not immediately moving all 43 files.

## 2. Organization and repository options

| Arrangement | Benefit | Cost / trigger |
| --- | --- | --- |
| **One org, one main repository initially — accepted** | A contract, example, conformance case and implementation change can be reviewed together. Small author teams share tooling. | Enforce public boundaries in tests; a shared checkout must not conceal private dependencies. |
| Protocol repository plus implementation repository now | Clear independent ownership and release surfaces from the beginning. | More cross-repository changes while both are unsettled. Prefer this if separate maintainers are already responsible for the protocol. |
| Many repositories for runtime, SDKs, plugins and site now | Independent permissions and release pipelines everywhere. | Coordination and duplicated setup before there are independent teams/products. Defer until a concrete boundary benefits. |

Suggested organization starts with the existing `conduit` repository, preserving its Git history and
PR attribution if transferred. An organization `.github` repository is optional for shared community
metadata. Do not create placeholder repositories for every family or every proposed crate.

An independently authored client/plugin may eventually have its own repository, even under another
owner. Its acceptance test must use published artifacts/public interfaces outside the reference
checkout. A second repository authored through private runtime imports would not meet that goal.

A later split is justified by independent maintainers, incompatible build/toolchain needs, a distinct
support cadence or an external-consumer boundary that is already proven. An org name or directory
name should not silently become a permanent protocol/type identity. Confirm names before publication.

## 3. Proposed source boundaries

This is a growth map, not a request to create empty directories. All names remain proposals.

```text
conduit/
  README.md
  CONTRIBUTING.md           # contributor paths, checks, review responsibilities
  LICENSE                  # project-owner decision
  Cargo.toml / Cargo.lock   # when Rust implementation begins
  protocol/
    schemas/               # language-neutral core representations
    conformance/           # public scenarios, fixtures and test adapters
  crates/                  # Rust runtime, service/client wiring and internal libraries
  packages/                # thin initial reference packages; richer packages may live elsewhere
  examples/                # small supported authoring journeys, once executable
  experiments/             # exploratory scenarios, failures and measured evidence
  docs/
    SPEC.md / DESIGN.md
    design/                # canonical prose until a deliberate reference reorganization
    guides/                # user/author/implementer guides as they become runnable
    proposals/             # pending changes, with status
    research/              # evidence and comparisons
    archive/               # superseded material and attributed dissent
  site/                    # navigation/theme/build configuration, when a site is built
  tools/                   # validation, release and contributor tooling
  .github/                 # review and CI configuration
```

`protocol/` holds machine-readable core artifacts and public behavior tests, not a second prose
specification or every extension's schemas. Extension-specific contracts stay with their packages.
The website renders canonical docs and package documentation from their source locations. An
eventual `docs/reference/` should replace the relevant design reference, never duplicate it.

`crates/` divides implementation responsibilities. Start with the smallest tested vertical slice;
extract more crates when an actual build, dependency or consumer boundary warrants it. The earlier
[crate table](implementation-v0/ORGANIZATION.md#2-crate-contracts-and-dependency-direction) is a useful
candidate allocation, not a requirement to create every crate. Separate management and execution
authority remains required even when both use shared Rust libraries.

Keep these dependency boundaries visible:

- Core schemas, conformance definitions and offline contract checking do not depend on executor SQL,
  UI code or provider SDKs. Validation starts no plugin code or provider work.
- Frontend clients use public management/observation contracts rather than querying executor databases.
- Authored packages use declared host/capability contracts, not private runtime imports. Trusted
  in-process built-ins retain the same declared semantics; implementation optimization is not a new API.
- Shared implementation utilities can be internal. A Cargo crate is not automatically a supported
  SDK, installable plugin, process or object family.

## 4. Packages and the standard distribution

There are three independently useful deliverables:

| Deliverable | Consumer | What it contains |
| --- | --- | --- |
| Protocol/conformance kit | Client, plugin and alternative-runtime authors | Versioned public definitions, behavioral requirements, fixtures and a usable test entry point. It must not require importing the reference runtime's private code. |
| Reference distribution | Users and experiment operators | Compatible CLI/Interface client, management/execution programs, and a small declared set of bundled implementations/templates. Several processes can be delivered together. |
| Extension package | Template/plugin authors and operators | Typed contributions under a common package envelope: plugins, templates or both. Plugin contributions can bind code, existing MCP services or composed operations; template-only packages require no implementation code of their own. |

SDK/helper libraries can become another published convenience as repeated authoring needs justify
them; a large SDK remains neither the initial priority nor a prerequisite for extension authors.

Recommended package source ownership: keep a contribution's contracts, implementation source or
binding, templates, examples and conformance tests together. A Rust package may contain a crate
participating in the main Cargo workspace. A declaration-only package needs none. Shared adapters
can be referenced without copying their implementation into every package. Source directory layout
and the packed installation format are distinct decisions.

**Accepted common envelope:** a Linear binding package can publish contracts and standard templates
while reusing an MCP adapter; a history package can ship an implementation and operation tests; a
template-only workflow package can compose their available contributions. All use the same outer
package contract for identity, version, typed contents and dependencies. Each contribution retains its
own validation and behavior. The [canonical model](../design/MODEL.md#authoring-concepts-beneath-the-object-families)
records this boundary. Exact fields, dependency references, archive layout and source-tree conventions
remain open. Automatic dependency resolution stays deferred, and installation establishes no instances,
provider authentication or grants.

Start with only the packages needed to prove authoring and Experiment 002. The main repository's
purpose is the protocol, conformance and reference implementation, not accumulating a comprehensive
integration library. Supported ancillary packages may move into separate repositories as their
maintenance warrants. Preserve simple external package authoring and public compatibility when they
move. Choose package boundaries by responsibility, reuse and dependencies; Resource type groups,
package names and implementation languages remain distinct.

## 5. Release and delivery proposal

Use one coordinated reference-distribution release initially. It records exactly which protocol
capabilities, package artifacts and platforms were tested together. Core Rust crates can follow that
release without promising stable public APIs for each crate.

Keep logical version domains separate: protocol/contract version, package implementation release and
artifact identity, reference application release, and internal storage-format compatibility. An app
patch should not require changing every capability contract's version. Conversely, two releases claiming
the same shape do not establish identical semantics. Exact version syntax still belongs in the authoring pass.

First distribution: prepared archives for validated macOS targets with a readable inventory and integrity
metadata, plus source build instructions. A Homebrew entry can wrap the same tested artifacts later.
Do not claim Linux/Windows validation from portable Rust code alone. No npm/Python runtime should be
required for the base Rust distribution merely because some extensions use those languages; each such
extension declares its actual prerequisites.

Keep v0's accepted explicit local package installation. A user can download a package release and install
it locally without a hosted Conduit marketplace. Future OCI, Git or other delivery mechanisms can carry
the same package contract. Neither a registry nor automatic cross-language dependency resolution is
needed to prove authoring. External MCP services keep separately reported compatibility; pinning the
Conduit adapter cannot freeze a remote service implementation.

Installing a release must preserve artifacts bound to retained instances. Package installation still
does not instantiate templates, authenticate services or grant access. Test built release artifacts
outside the source workspace so workspace paths cannot hide missing schemas, executables or dependencies.

## 6. Website and documentation experience

Lead with the protocol and original authoring. The earlier product-first navigation was too centered
on installing templates and composing existing integrations. The proposed replacement is:

| Route | First useful experience |
| --- | --- |
| Learn the protocol | Understand the five families, language, contracts and how their relationships express a system. |
| Author with the protocol | Define capabilities, operations and requirements; implement a small plugin; construct templates and package contributions. |
| Protocol reference and conformance | Find exact semantics, representations, MCP mappings, compatibility and observable guarantees. |
| Reference implementation | Execute and inspect the contracts using the Rust runtime and clients; distinguish those mechanisms from protocol requirements. |
| Project and ecosystem | Find source responsibilities, experiments, proposals, research, archived alternatives and supported ancillary packages. |

An introductory example should define a small new capability, supply its implementation, use it in
a Resource template, establish it through a Pool and connect an Agent/Interface through a Conduit.
Inspection and a failure case should demonstrate its guarantees. Reusing existing integrations is a
later application of this language. Defining an extension must not require reading executor SQL or
modifying core dispatch. These are documentation priorities, not a new requirement to implement every
capability before the first authoring example can be reviewed.

Separate tutorials/task guides from exhaustive reference and architecture explanation. Keep research
and dissent reachable under contribution/design context without presenting them as supported features.
Until implementation exists, the site should say protocol design/development and link reviewable examples;
it should not display an installation quickstart that cannot run.

Keep site configuration in the main repo initially; a separate deployment does not require a separate
content repository. Choose a static-site framework after the content paths and version strategy are clear.
Development docs and later released docs must be distinguishable. Package docs should identify the exact
release/contract they describe. Conformance status, platform support and experimental features should be
visible alongside the relevant instructions.

## 7. Joint review sequence

Repository and package rulings are complete. Documentation emphasis is clarified; exact navigation,
layout and release strategy remain to be settled alongside the transition:

1. **Repository ownership — accepted:** one organization/main repository with explicit public boundaries
   and independently consumable protocol/conformance, runtime and extension packages. Actual organization
   creation/transfer and the proposed directory names are separate from this ruling.
2. **Package envelope — accepted:** one common envelope with typed contents supports plugin contributions,
   template-only collections and combinations. Package syntax and the exact artifacts in the proposed
   protocol kit/reference distribution remain drafting and release-review work.
3. **Documentation — emphasis clarified:** the language/contracts and original authoring lead; the
   runtime and thin reference packages demonstrate the protocol. A single source set rendered as a
   website remains recommended; exact navigation, source locations and generator remain open.
4. **Release contract:** coordinated reference releases, separately identified contracts/artifacts,
   macOS-first prepared delivery and local extension installation? I recommend this limited first surface.

Project-owner decisions also needed before public code distribution: intended license/contribution terms,
organization/package/domain names and who may approve protocol changes and releases. The review found
no license to carry forward automatically. Do not invent a legal policy or transfer the repository as an
implementation detail.

After agreement, revise the existing organization proposal rather than maintaining competing layouts.
Promote one author journey into public conformance fixtures and runnable examples; use that journey to
settle package exports/references, contract versions, bindings and artifact selection together. Keep
Experiment 002's accepted runtime scope, adding clean-checkout/artifact installation checks to its planned
authoring evidence when the packaging decisions are accepted. No additional live service is required.

## 8. Preserving the design through the transition

The transition is a proposed work sequence, not approval of syntax or implementation choices. Keep
accepted behavior readable in the active documentation throughout. Git history protects the original
material, but readers should not need Git archaeology to discover a rule that still applies.

### Preserve a reviewable baseline

- `4c54263` on `main` preserves the completed conceptual review, research, experiments and attributed
  collaborator proposals before this organizational review.
- `a51c87b` on `docs/protocol-framing` preserves the subsequent ecosystem investigation, accepted
  single-repository/common-package rulings and the then-current organizational proposals. Their status
  markings distinguish accepted rules from suggestions. The present follow-up records protocol emphasis.
- Keep structural moves and substantive semantic changes in separate commits. Do not replace either
  baseline or treat a documentation branch as approval to begin the runtime.

Before removing or merging an existing section, account for its accepted obligations, qualifications,
examples, rationale and evidence. A temporary section-level migration checklist is sufficient; there
is no need to recreate the long decision ledger or assign an ID to every sentence. Preserve existing
decision references where they are still useful and repair links/anchors when moving their targets.

### Map the existing work by authority

These are destination responsibilities; exact directory names remain for the structural pass.

| Existing source | Destination responsibility | What must survive |
| --- | --- | --- |
| SPEC and DESIGN index | Protocol overview and reference navigation | Five families, extension concepts, project priorities and clear links to detailed accepted rules. |
| design/MODEL, CONFIGURATION and LIFECYCLE | General protocol reference | Typed configuration, identity, composition, authority, controls, persistence/observation boundaries and recovery semantics. Separate reference-runtime selections where appropriate without withdrawing their accepted implementation status. |
| design/AGENT, RESOURCES, POOLS and CONDUITS | Object and relationship contracts | Ownership, account/state boundaries, activation, shared authentication, unitary resources, participation, requirements/grants, dependent-call restrictions and communication/memory behavior. |
| Completed conceptual and implementation reviews | Closed review summaries linked to current contracts | Accepted rulings, limitations and the independent-client/plugin conformance milestone. They do not become parallel active queues. |
| Implementation architecture, authorization and organization proposals | Reference-implementation design | Accepted deployment/storage boundaries plus explicitly marked candidate mechanisms. Move mixed protocol syntax into the authoring proposal when needed; do not silently promote candidates. |
| Authoring, contracts and package walkthrough proposals | One active language/authoring review packet | Small-handler, filesystem, MCP, history, composite operation/requirement and template-only cases; competing syntax remains illustrative until ruled on. |
| Research | Dated evidence and design rationale | Primary links, source pins, comparisons, failed approaches, actual probes and their limits. Consolidate duplicate recommendations only after preserving unique evidence. |
| Experiments 001 and 002 | Exploratory fixtures and evidence outside docs | Accepted scope, draft examples, intended outcomes and actual validation records. Expected behavior never becomes a claimed successful test. |
| Collaborator comparison and archive | Attributed alternatives and dissent | Original source identity, authorship, accepted adaptations and unadopted arguments. |

### Three focused passes

**Pass 1 — protocol documentation and repository preparation.** Review the map, then move intact
sections first and edit navigation/explanation separately. Present the language, contracts and
authoring path before the reference application. Keep shared protocol semantics distinct from
accepted reference-runtime choices such as Rust, SQLite and the first process layout. Carry the
thin example packages with the implementation only as needed for proof. Introduce contributor and
validation instructions as they become usable; choose the license and organization details explicitly.
Do not scaffold a large crate tree or publish an unimplemented installation guide.

Exit evidence: every source section is retained, mapped to a destination, or deliberately archived
with its reason; accepted behavior has an active home; local links/anchors validate; no semantic
deletion is disguised as shortening prose. A repository-local documentation check should replace
reliance on temporary validation scripts when the tooling is introduced.

**Pass 2 — language, package and conformance review.** Use one complete original-authoring journey
as the backbone, with focused variants for a declaration-only MCP binding and a template-only package.
Bring the existing examples forward instead of starting a new independent schema draft. Review
connected questions in batches: package/contract identity and exports; template/reference scope and
defaults; graph/control bindings and results; MCP/profile mappings and conformance claims. Address
release compatibility with those examples; installer channel details need not determine language syntax.

For each batch, show the authored files, how names resolve, what instance configuration results,
what is observable and at least one meaningful rejection or uncertain outcome. Compare syntax against
the accepted contracts. A proposal that changes behavior names that change explicitly and returns it
for a ruling. Routine field spelling does not reopen the entire conceptual design.

Exit evidence: reviewed public representations and examples, a coherent package-to-instance journey,
and conformance cases tied to the rules they exercise. Publish schemas and tests alongside the
corresponding accepted reference text. Syntax validation alone cannot establish effect or recovery
guarantees; unresolved cases remain visible rather than receiving invented defaults.

**Pass 3 — smallest executable reference slice.** Implement the reviewed boundary using the accepted
Experiment 002 arrangement. Start with the authoring path through validation, an externally supplied
plugin, graph execution, managed instances/connections and Interface evidence. Extend that slice to
the accepted composition, failure and recovery cases; do not replace the proof with a broad plugin
library. Keep protocol conformance and runtime-specific tests distinguishable.

Exit evidence: a separately authored client/plugin uses public contracts without private checkout
imports; package installation works from prepared artifacts; required success/failure cases produce
inspectable results. The complete first milestone still includes management and two executors,
authority/outage enforcement and the agreed recovery proof. Smaller development increments do not
claim that milestone complete. Native account validation and the owned model loop retain their agreed
later sequence.

### Continue iterating without losing decisions

Keep one current proposal packet for the active language questions. After a ruling, amend its canonical
contract once and link the corresponding example/conformance case; keep the rationale or source link
nearby. Record intentional reversals explicitly. Do not force all implementation details into the
protocol, or keep duplicate editable versions of an accepted rule in research and review documents.
Code may reveal a counterexample: isolate it, explain which guarantee it affects, and resolve it
collaboratively. Naming changes, file moves and prototype convenience cannot silently change meaning.

The next substantial change should be the first pass only. The later passes describe the path to
implementation and are not authorization to settle unresolved syntax or start coding now.
