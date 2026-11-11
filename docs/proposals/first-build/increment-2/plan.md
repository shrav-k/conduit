# Increment 2: proposal for the core on S1

Not binding. Work order 003, 2026-11-09. **The worker's proposal, awaiting the admin's
decisions.** No recommendation below is an accepted decision. Only
[the specification](../../../../spec/README.md) binds. This study neither implements the
core nor claims that S1 has run. The one new Rust test checks the drafts with the loader.

The boundaries come from [the first build](../../../implementation/FIRST_BUILD.md),
[ruling 40](../../ruling-40/README.md), [ruling 46](../../ruling-46/README.md) and
[ruling 48](../../ruling-48/README.md). The [roadmap](../../../../ROADMAP.md#the-order-of-the-work)
puts this increment next; its [S1 row](../../../../ROADMAP.md#round-6s-evidence-and-measurements)
assigns E05, E08, E11 and E12 to S1. The baseline's
[setup and results](../../../../experiments/007-s1-baseline/results-2026-11-09.md) are evidence
about today's tools, not promises about Conduit. Names below use the current values in
[the names file](../../../../spec/names.yaml).

## 1. S1 as documents

### The project and its packages

The draft [project](s1/project/) contains two Agents, `agent-a` and `agent-b`; a Pool
`workspace`; and two Resources it owns, `shared` and `history`. The Resources are siblings,
not one directory exposed under two names. Each Agent declares `project` and `history` as
required Conduits. Instantiation makes all four Conduits, registered and unopened; setup opens
them. `agent-a` lists reads and directory listings on `shared`; `agent-b` also lists writes.
Both list reads, directory listings and writes on `history`. Neither Agent has management
access or a right to open. The configuration booleans in their types do not grant either.

| Directory | What it drafts | Dependencies and boundary |
| --- | --- | --- |
| [s1/filesystem/](s1/filesystem/) | Package `example.org/s1/filesystem`, version `0.1.0`: a Resource Type, a Capability, three Operations and one `plain` implementation item | The existing `@modelcontextprotocol/server-filesystem` `2.1.0` used by A70; one positional Package argument binds the instance's required string `/root` |
| [s1/history/](s1/history/) | Package `example.org/s1/history`, version `0.1.0`: Type `file-history.v1` | Reuses the filesystem package's capability, operations and implementation; another run, rooted at the history directory; no new history daemon |
| [s1/agents/](s1/agents/) | Package `example.org/s1/agents`, version `0.1.0`: Types `claude-code.v1` and `codex.v1` | Registration and resolution only: no offered operation, required implementation or selected run; two false configuration values reserve the intended disabled defaults for later wrappers |
| [s1/project/](s1/project/) | Nine templates and six named instances, including `root-workspace` | Uses the three packages; core references need no manifest item; four nested Conduits get generated names and IDs |

These addresses and versions are proposed draft names. All Conduit package digests and the
npm `fileSha256` are **illustrative placeholders**, marked in their manifests. No archive was
built or fetched. Before an installable fixture is made, compute real archive digests, replace
the dependency pins, and obtain the npm artifact hash. The loader checks their forms only.
A hash of the npm artifact does not pin its dependency graph (MAN-010). Alternatives are a
`command` that invokes an already installed executable, which pins nothing, or a `package`
item with the artifact hash. Recommend the Package for the eventual provider; use supplied
package snapshots with explicit illustrative pins for the core's first deterministic vectors.
Those snapshots must be labelled preinstalled fixtures, never successful archive installations.

The filesystem item maps exactly these operations; other tools of the unchanged server are
not exposed automatically:

| Operation | MCP tool | Contract in these drafts |
| --- | --- | --- |
| `file.read.v1` | `read_text_file` | Required string `path`; reads text; no effect |
| `directory.list.v1` | `list_directory` | Required string `path`; lists entries; no effect |
| `file.write.v1` | `write_file` | Required strings `path` and `content`; creates or replaces that file |

The Operations deliberately declare no outputs: A70's existing server returns text in MCP
`content`, and this study has not established a `structuredContent` contract for it. IFC-025
preserves the plain server's content; a successful presented result has `structuredContent: {}`.
This keeps the inputs and effects explicit without claiming text is a validated output.
The Type's `records` keeps inputs and no outputs. This can retain posted text in records;
S1 has no secret. Retention of plain content on a repeat needs the decision in section 6.
The item has no `supports`, as PKG-020 requires. Reads and writes are separated by operation
lists, not by a filter that the plain server cannot apply. The shell must keep its own MCP
Roots behavior from widening the configured root, a baseline limit to qualify in increment 3.

The history choice belongs to the admin, within ruling 40:

| Option | Benefit | Cost or limit |
| --- | --- | --- |
| A70's directory, a new file per message, using the same server | Existing infrastructure, no adapter, no shared read-modify-write | Posting discipline, unique filenames, authorship and ordering are client conventions; `write_file` can overwrite |
| An existing database or messaging tool behind its MCP contract | Could supply atomic append and server ordering | Choose and qualify a tool, write its actual contracts and count integration; not done by this study |
| One shared text file through the filesystem server | Fewest data files | A70's 2.1.0 has no append tool; read-modify-write loses concurrent posts |

**Recommend the first.** Use `<UTC time>-<agent>-<unique suffix>.md`, read in name order,
and instruct clients never to replace a post. A suffix reduces the same-second collision A70's
naming permits, but is not an enforcement guarantee. A70 also observed a fabricated timestamp;
lexical order is not authoritative chronology. The history Type's description states this
limit. Do not call it append-only or attribute an author from the filename. A real enforced
append contract would require a different existing tool or an explicitly counted adapter.

### Registration and setup trace

The following is a proposed trace for the core runner. Every request names its sending
instance, original caller, operation and authorizing Conduit; names learned from registration
are bound to symbolic IDs in a vector. Ordinary calls are made with fresh keys unless the
trace explicitly says repeat. `root` sends the management requests on its own behalf.

1. First start: five ready objects and root's two ready declared Conduits. Load the built-in
   core package without an archive digest. No Agent exists and no model work starts.
2. Supply the filesystem, history (after filesystem) and Agent packages as preinstalled
   snapshots for a resolving vector. A later installation vector must instead make three
   explicit `install.v1` requests through `management.install.v1` at root's server, filesystem before
   history. This project's registration performs no installation (SUB-010, PKG-006).
3. Hand over `s1/project/` as one submission for `root`: templates, instances and manifest
   together. The owning Pool is new in this submission, so ACC-033 row 2 authorizes registration
   against `management` alone. Result: 15 named objects, four nested Conduits, all instances
   registered. Contracts are package exports, not registered objects (NAM-013).
4. Open named Conduit `root-workspace` through `access.open.v1` at root's server, with input
   `{conduit: root-workspace}`. Root's unbounded Conduit to `access` authorizes it. Its tools
   include `root-workspace.set-up.v1` and `root-workspace.inspect.v1`.
5. Set up Pool `workspace` through `management.set-up.v1`, input
   `{instance: {kind: Pool, name: workspace}}`. This Pool has no entries. Its setup does not
   set up arbitrary Resources merely because it owns them.
6. Set up `shared` and `history`, separately, through `root-workspace.set-up.v1`, naming each
   Resource. Authorization is against `workspace`, ACC-033 row 4; the core asks its built-in
   runner for each filesystem run and receives simulated completions. Neither selects a hook.
7. Set up each Agent through `management.set-up.v1`, naming it in `instance`. Each declared
   Conduit's open is a part, sent by `root`, authorized through **root's** Conduit to `access`,
   not through an Agent's. In these drafts the Agents run nothing; their setups end after the
   opens. No model turn is implied.
8. From A, call `project.file.read.v1` and `project.directory.list.v1`; a write is refused.
   From B, also call `project.file.write.v1`. From either, call the corresponding `history.*`
   tools to post, list and read. In a core vector the external server replies are supplied
   values; in increment 3/4 they must be actual calls and content observations.

The test [s1_drafts.rs](../../../../engine/crates/loader/tests/s1_drafts.rs) reads all
four directories separately and asserts a manifest and documents were read, as well as loader
acceptance. It proves no package is installed, no reference resolves, no config meets its
Type, no declared Conduit is well typed, and no access is permitted. These all require the
core. The drafts select no filter, secret, remote server, external runtime, hook or control.
Their package argument binding still needs the core's MAN-017 check. Their references,
implementation selections, ownership, generated nested names, package pins and Agent defaults
also need the core.

### Increment 3 and the later change chapter

The registration-only Agent Types are intentional partial drafts, not runnable providers.
Increment 3 needs two actual MCP wrappers, one long-lived client session each, the isolation
flags and private working directories of the first build, per-client token-file helpers, one
server approval, and own account services and messaging off by default with explicit template
inputs to turn each on. An Agent's setup hook must not start model work. A separately offered
operation marked `startsModelWork: true` would need its own authorized Conduit. The false
booleans here alone implement none of that. Count the wrappers, client configuration and
provider contracts apart from the author's project, using the first build's counting rules.
A4 precedes increment 3; it is not another core feature to implement in this order.

D08 is a requirement for the later S1 change-and-repair chapter, not an automatic rollout:

| Task | Proposed explicit actions and permissions | What to preserve and measure |
| --- | --- | --- |
| T3 | Root inspects Agents and Conduits via `management`, Resources via `root-workspace`; shell separately inventories built-in/client/account routes | Declared reach is only mediated; records cannot establish confinement |
| T4 | Root closes each old `project` Conduit via `access`, registers a new standalone Conduit with the other operation list via `management`, opens it via `access` | Agent IDs and history Conduits stay; exact requests and intervals without directory access count; restoring a declared slot uses an explicit nested replacement |
| T5 | Explicit install of newer filesystem package, revision-checked replacement of the Resource template with its project, then explicit replacement of the live Resource and access | Existing instances keep old packages and definitions; editing a template alone upgrades no run; compare with an allow list and `--read-only`, not only A70's deny list |
| T6 | New Resource at new root; register/open new Conduits, close old ones and explicitly retire old Resource via its Pool | References to the old ID do not retarget; no automatic migration or rollback; backing files survive retirement |
| T7 | Root closes only A's project Conduit, checks B and history, explicitly registers its declared replacement then sets A up again | Replacement is registered first, then opened by root's access authority; no unrelated run restarted; ready B continues |

The drafts use top-level Resources to keep T1/T2 readable. To exercise D08's **nested
Resource replacement** specifically, a later variant must put `shared` in a Pool entry, retire
it explicitly and use `register-replacement.v1` on that entry before restoring access. That
variant must measure its extra structure; the top-level replacement trace is not evidence
for nested replacement. E11 and E12 below exercise frozen definitions and safe identity follow-up
already in the core. D22 counts files, lines, concepts, grants, adapters and human recovery,
plus the first build's remaining measures; an on-paper count of these drafts is no observed
author result. D19 also requires bytes of invocation state retained, separately from records.

## 2. The promises S1 touches

The inventory is for the paths above and their safety boundaries. `Yes` means the strict
loader already checks the stated syntax or shape. `Part` means it checks only that part; the
row states the core work. `No` means the loader supplies no such semantic check. A schema
rejection cites SUB-005, even when its encoded promise is listed here. No row is a claim of
whole-language conformance. The loader's [README](../../../../engine/README.md) and
[increment 1 log](../log-increment-1.md#what-increment-1-did-not-do) delimit its coverage.

### First start

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| FST-001 | Make exactly `system`, `local`, `management`, `access`, `root`, recognizing them by ID | No |
| FST-002 | Make root's two declared Conduits open and all five objects ready | No |
| FST-003 | Make no additional instance and call no other implementation | No |
| FST-005 | Keep qualified core-template provenance for all seven instances | No |
| FST-006 | Reject submitted instantiations of the five protected core templates | No |
| LIF-034 | Refuse retiring/unregistering first-start objects or declaring `local` lost | No |
| NAM-020 | Resolve core selections without a manifest package entry | No |
| PLC-008 | Make `local` a `node.v1` Resource owned by `system`, with itself as placement | No |
| PLC-026 | Associate canonical local hostname and reported labels with the unique node | No |
| PRV-014 | Take canonical hostname and required OS/architecture/hostname labels from machine reports | No |

### Registration and resolving

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| OBJ-001 | Keep explicit and generated object names within the ruled grammar | Yes for written names |
| OBJ-002 | Reject a kind/name already held by an object, including first start | No |
| OBJ-003 | Generate unique nested names and report each nested instance's source | No |
| OBJ-004 | Reject engine-owned top-level fields | Yes |
| OBJ-006 | Read references with fixed-kind shorthand only where its schema permits | Part: shape; resolve kinds in core |
| OBJ-007 | Reject a template where an instance is expected, and conversely | No |
| OBJ-008 | Register templates with unresolved object names, reporting them without blocking | No |
| OBJ-009 | Resolve every instance reference to kind/ID; reject authored resolved references | Part: shapes admit stored IDs; authorship and resolution are core |
| OBJ-010 | Never retarget an existing resolved reference after a name is reused | No |
| OBJ-011 | Keep instance definitions and all object names fixed | No |
| OBJ-012 | Read template envelope and `spec` | Yes |
| OBJ-013 | Read standard arguments; resolve owner, consumer and target | Part: shape only |
| OBJ-014 | Read nested declarations without metadata or consumer | Yes |
| OBJ-015 | Honor optional/defaulted inputs and required declarations | Part: marks only |
| OBJ-016 | Evaluate JSON Pointer destinations and item bindings with their fixed roots | Part: pointer grammar only |
| OBJ-017 | Use the current `apiVersion` | Yes |
| OBJ-018 | Preserve metadata while giving description/annotations no language effect | Part: shape only |
| OBJ-019 | Reject unknown structural keys while leaving config and data keys to authors | Yes for document shape |
| OBJ-020 | Accept only the ten document kinds | Yes |
| OBJ-021 | Resolve only places governed by reference schemas, including through `$ref` chains | No |
| OBJ-022 | Enforce named-map grammar for inputs and declared Conduits | Yes |
| OBJ-023 | Return invalid plus diagnostics for rejected registration except existing-name clash | No |
| OBJ-024 | Read the assigned names from the specification's names file | Part: current loader already reads them |
| OBJ-025 | Report existing-name clash as conflict with both objects named | No |
| INS-001 | Distinguish templates from instances by `from` | Yes |
| INS-002 | Reject `from` that does not resolve to a template of the same family | No |
| INS-003 | Reject an undeclared template argument | No |
| INS-004 | Reject an absent required template argument without a default | No |
| INS-005 | Validate supplied arguments against input schemas without coercion | No |
| INS-006 | Assign each whole supplied value to every destination; expand fixed-kind references | No |
| INS-007 | Apply an absent input's explicit default | No |
| INS-008 | Leave absent optional inputs' destinations as the template supplied them | No |
| INS-009 | Distinguish supplied null from absence | No |
| INS-010 | Reject equal or overlapping input destinations by pointer tokens | No |
| INS-011 | Require each destination's parent, including implicit empty config/arguments | No |
| INS-012 | Permit destinations only in the closed list, not implementation/type/from | Part: grammar; semantic destination checks remain |
| INS-013 | Store a complete resolved instance, without a separate arguments layer | No |
| INS-014 | Validate the resulting config against its Type, before and after reference resolution | No |
| INS-016 | Instantiate only the instance and its nested objects; run/open nothing | No |
| INS-017 | Make equal instantiations equal apart from generated names and IDs | No |
| INS-018 | Preserve a relative path argument as supplied; do not resolve it against a directory | No |
| INS-019 | Apply argument, input default, template value, Type default in that priority | No |
| INS-020 | Keep old instances frozen after template replacement | No |
| INS-021 | Read input schema, destination list and optional/default marks | Yes |
| INS-023 | Check a supplied operations list against the Conduit's capability | No |
| INS-024 | Treat missing config as an existing empty map | No |
| INS-025 | Resolve local, qualified and same-project `from` with the ruled meanings | Part: syntax only |
| TPL-001 | Read each family's allowed template keys | Yes |
| TPL-002 | Check the selected Type's family | Part: selection syntax only |
| TPL-003 | Reject explicit selection of an engine-implemented operation | No |
| TPL-004 | Select the engine by rule, otherwise the Type's default or explicit mapping | No |
| TPL-005 | Verify the selected item actually lists the operation in `implements` | No |
| TPL-019 | Permit at most one run item per Agent/Resource, none per Pool/Interface | No |
| CTR-001 | Read the five contract kinds and compatibility-versioned names | Yes |
| CTR-002 | Read actual operation schemas/effect/model-work marks; no Operation implementation default | Yes for shape |
| CTR-003 | Resolve capability members and reject non-core capabilities listing reserved operations | Part: shape only |
| CTR-004 | Read Type config schema, offers/requires/defaults and record-content choices | Part: shape only |
| CTR-007 | Reject a capability giving two operations the same export name | No |
| CTR-008 | Interpret a Schema contract's entire `spec` as its schema | Part: profile shape only |
| CTR-009 | Reject a non-core reserved-to-root Operation | No |
| CTR-012 | Preserve supported operation root keywords and keep declared inputs at root properties | Yes for shape; interpretation in core |
| SCH-001 | Validate authored schemas with formats asserted, including `patternProperties` behavior | Part: document validation exists; dynamic schemas need core validation |
| SCH-002 | Reject unsupported schema keywords | Yes |
| SCH-003 | Reject unsupported formats | Yes |
| SCH-004 | Assert the profile's exact format grammars | Yes; reuse these validators for config/requests |
| SCH-005 | Keep mathematically whole numbers as integers | Yes |
| SCH-006 | Reject integers outside the exact range, including rounded fractional spellings | Yes |
| SCH-007 | Use schema equality for contents/default comparison; ignore map order | Part: values exist; core comparisons remain |
| SCH-008 | Reject defaults outside permitted property positions, including conditional positions | Yes for placement |
| SCH-009 | Fill property defaults before validation/comparison, without creating absent parent maps | No |
| SCH-010 | Recursively fill defaults of supplied default maps | No |
| SCH-011 | Never coerce a value into a schema's type | Part: reading/validation; preserve in core |
| SCH-012 | Strict YAML, rejecting duplicate keys and unsupported YAML features | Yes |
| SCH-013 | Give equivalent JSON and YAML the same meaning | Yes |
| SCH-014 | Do not reject the unchanged MCP server's own schemas for being outside Conduit's profile | No; inspect them only at the shell edge |
| SCH-015 | Resolve authored `$ref` to Schema contracts or local pointers; check its referent kind | Part: syntax only |
| SCH-016 | Match allowed patterns as ECMA-262 in Unicode mode | Part: loader checks authored tokens and document patterns; core must reuse semantics dynamically |
| SCH-017 | Reject non-JSON number spellings | Yes |
| SCH-018 | Read JSON arrays as their documents and other values as one document | Yes |
| SCH-019 | Validate authored defaults after filling applicable property defaults | No |
| SCH-020 | Read fractions as binary64, keeping rounded whole values within range | Yes |
| SCH-021 | Reject fractional values beyond binary64 | Yes |
| SUB-001 | Resolve a set independently of file/document order, including forward references | Part: loader reading only |
| SUB-002 | Resolve references to objects made in the same submission | No |
| SUB-003 | Publish no objects if any document is rejected | Part: loader returns errors; core atomicity remains |
| SUB-004 | Reject duplicate kind/name in one submission with both documents named | No |
| SUB-005 | Cite SUB-005 for schema rejection, with instance pointer, contract and keyword location | Part: loader diagnostics need the SARIF adapter |
| SUB-006 | Register templates/instances and make project contracts available; perform no setup | No |
| SUB-007 | Never replace a template by ordinary registration | No |
| SUB-008 | Hash project contracts plus original manifest with the specified canonical ordering | No |
| SUB-009 | For E11, resolve a replacing template against its newly submitted project | No |
| SUB-010 | Accept a directory, preserve original manifest for hashing, install nothing | Part: file selection exists; resolving relative fetch/programs and authorization remain |
| SUB-011 | Reject only stated size/count/depth limits; include limit and value | Part: loader depth limit exists; core submission/archive limits remain |
| NAM-001 | Validate package addresses | Yes |
| NAM-002 | Require qualified names for another package's exports | Part: syntax only |
| NAM-003 | Treat a bare reference as local, never a package lookup | No |
| NAM-004 | Validate export names | Part: name grammar; actual exports remain |
| NAM-005 | Keep release versions out of references | Yes |
| NAM-006 | Use one exact version and digest per package address per manifest | Part: shape only |
| NAM-007 | Require manifest items for non-core external addresses | No |
| NAM-008 | Accept but report unused, satisfied package items | No |
| NAM-009 | Retain template package/project pins and selected project items | No |
| NAM-010 | Instantiate through the template's retained context | No |
| NAM-011 | Preserve old pins when a second package version is installed | No |
| NAM-012 | Treat fetch as an explicit installation source, never registration side effect | Part: URI syntax only |
| NAM-013 | Installing a package registers no exported templates or instances | No |
| NAM-014 | Resolve a package document through that package's own manifest | No |
| NAM-015 | Keep compatibility-versioned contract names distinct | No |
| NAM-017 | Do not refuse an open merely for different package releases at its ends | No |
| NAM-018 | Resolve selections against contracts and implementation items | Part: syntax only |
| NAM-020 | Resolve the built-in core with no manifest entry | No; shared with first start |
| NAM-021 | Protect the built-in core address from user manifests and installs | No |
| NAM-022 | Interpret `#name` in its own project/package; special-case submitted `from` | No |
| NAM-023 | Validate requests/results against the target's held version | No |
| NAM-024 | Accept only compatible additions under an existing contract export in later releases | No |
| NAM-025 | Recommend a new package address for incompatible releases (`should`); NAM-024's compatibility checks for an existing contract name remain mandatory | No |
| NAM-026 | Report unavailable when the target's older capability lacks a listed operation | No |
| NAM-027 | Reject duplicate names across document kinds or document/item in one project/package | No |
| NAM-028 | Reject selections of missing exports/local contracts rather than unresolved warnings | No |
| PKG-001 | Validate a package release version | Yes for grammar |
| PKG-003 | Compare exact release precedence, ignoring SemVer build metadata | No |
| PKG-004 | Verify archive SHA-256 over the distributed bytes for actual installs | Part: digest syntax only |
| PKG-005 | Retain instance package versions/digests from its template | No |
| PKG-006 | Reject missing or mismatched installed package pins without fetching | No |
| PKG-007 | Reject instantiation when a template's required package is absent | No |
| PKG-012 | Reject duplicate package exports | No |
| PKG-017 | Check installed package's own address/version manifest | Part: manifest shape only |
| PKG-018 | Restrict external selections to package exports | No |
| PKG-019 | Resolve implements/bindings of the selected filesystem implementation | Part: item shape only |
| PKG-020 | Require plain tool mapping, no supports, and no plain provider operations | Part: shape checks tools/no supports; semantic coverage remains |
| PKG-021 | Never substitute an installed digest for the requested one | No |
| PKG-022 | For actual installs, accept gzip tar and validate documents in its root | Part: loader can read extracted documents, not archives |
| PKG-023 | Return invalid plus correct diagnostics for rejected installs | No |
| PKG-024 | Report unavailable for an archive that cannot be fetched now | No |
| PKG-025 | Report conflict for an already-installed release with different digest; fetch nothing | No |
| PKG-026 | Reject an archive whose actual digest differs from the request, naming both digests | No |
| PKG-027 | Reinstall of the identical installed release succeeds without fetch/change | No |
| PKG-028 | Reject an installation missing its dependency packages | No |
| PKG-029 | Prevent installation at an address still held as a submitted project's context | No |
| MAN-001 | Read exactly one manifest document from one of its three names | Yes |
| MAN-002 | Reject multiple manifests | Yes |
| MAN-003 | Read manifest envelope without kind/metadata/spec | Yes |
| MAN-004 | Read the closed manifest key set | Yes |
| MAN-005 | Read exact version/digest and optional fetch per package | Yes for shape |
| MAN-006 | Resolve relative fetch against directory; reject it for values-only requests | Part: URI form only |
| MAN-007 | Check exports are names of the package's own documents | Part: list shape only |
| MAN-009 | Read one command/package/remote per implementation | Yes |
| MAN-010 | Check declared Package argument binding; keep artifact-pin limits explicit | Part: Package shape/hash form only |
| MAN-011 | Read only eligible root files; reject installed instance documents | Part: directory selection yes; installed-package restriction no |
| MAN-012 | Use the built-in engine item without a command, package, remote or supports | Part: core loader special case; selection semantics no |
| MAN-013 | For actual archive installs, find manifest/documents at archive root | No |
| MAN-014 | Reject archive absolute/parent paths and links escaping the archive | No |
| MAN-015 | Require an address for a submitted project with contracts/items | No |
| MAN-016 | Reject a submitted project claiming an installed package's address | No |
| MAN-017 | Check required scalar config places for item bindings when a template registers | No |
| POOL-001 | Resolve exactly one immutable Pool owner per Resource | Part: required owner shape only |
| POOL-004 | Permit Resources under a new/nonretired Pool, reject a retired owner | No |
| PLC-001 | Store one resolved placement per Agent/Resource | No |
| PLC-002 | Use `local` for runless Agents and built-in placement for the filesystem runs | No |
| PLC-009 | Never move an existing instance's placement | No |
| PLC-012 | Give no access merely for colocation | No |
| PLC-015 | Need no Conduit to a placement node | No |
| PLC-020 | Leave absolute paths to the implementation on its node | No |
| PRV-006 | Default to the engine's own runner and freeze any explicitly selected runtime | No |

The scalar shapes of `file.read.v1`, `directory.list.v1`, `file.write.v1` and the Types are
provider contracts proposed here, not additional promises. Package/archive verification can be
supplied as pure evidence at the core boundary; extraction and fetching are edge work. A test
with preinstalled snapshots does not cover the installation rows above. Rows for release
compatibility are required for T5/E11 before claiming those cases, even if the first T1/T2
trace uses only one release. Optional inputs, controls and Pool entries are absent in the
base drafts; their boundary and change vectors use small variants rather than more base files.

### Setup and opening

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| CON-001 | Store one consumer, target and capability per instantiated Conduit | Part: schema shape only |
| CON-002 | Instantiate each declared Conduit during Agent instantiation | No |
| CON-004 | Open declarations as setup parts; repeat setup preserves IDs; missing slot conflicts | No |
| CON-007 | Keep open Conduits on consumer crash/unreachability | No |
| CON-008 | Close/remove the affected Conduits on explicit close or retirement at its stated moment | No |
| CON-011 | Keep a ready consumer ready after a Conduit closes; report lost reach | No |
| CON-012 | Allow a new standalone Conduit without editing the consumer's definition | No |
| CON-013 | Open a new Conduit for a ready consumer without setting it up again | No |
| CON-014 | Close it without interrupting/retiring the consumer | No |
| CON-016 | Read a capability and explicit or input-supplied operation list | Part: shape; input destination semantics no |
| CON-017 | Verify every operation belongs to that capability, not all server tools | No |
| CON-020 | Reject consumer/target in Conduit templates | Yes |
| CON-021 | Reject a target whose Type does not offer the selected capability | No |
| CON-022 | Reject a filter on the plain filesystem target, even before it runs | No |
| CON-023 | Allow open while target is registered | No |
| CON-024 | Return unavailable for nonmanagement work until target is ready | No |
| CON-025 | Reject instantiation with a retired end | No |
| CON-027 | Check consumer/target family | Part: schema kind enumeration; referenced object's family no |
| CON-031 | Refuse to dispatch opening a registered Conduit with retired consumer: conflict | No |
| ACC-005 | Authorize declared opens with the setup sender's own Conduit to `access` | No |
| ACC-006 | Authorize opens with all target/operation rights; never partially open | No |
| TPL-008 | Represent one setup/retirement by one invocation of its management operation | No |
| TPL-009 | On later retirement stop the selected run, even after an unsuccessful hook | No |
| TPL-012 | Open required declarations before run/connect and then hook; stop new parts on cancellation/retirement | No |
| TPL-017 | If a variant selects a setup hook, dispatch it last through no Conduit | No |
| LIF-001 | Inspect instances as registered, ready or retired | No |
| LIF-002 | Leave every new instance registered | No |
| LIF-003 | Make it ready only after successful setup/open | No |
| LIF-004 | Keep state unchanged on failed/unknown/undispatched setup, reporting its end | No |
| LIF-005 | Enter retired at retirement dispatch | No |
| LIF-006 | Never return ready/retired to an earlier state | No |
| LIF-008 | Never silently recreate a closed declared Conduit | No |
| LIF-009 | Explicit nested replacement makes a new registered ID, opens nothing | No |
| LIF-012 | Give a template existence and revision, not lifecycle state | No |
| LIF-013 | Inspect declaration, status and related records separately | No |
| LIF-019 | Preserve records and backing data on object removal | No |
| LIF-024 | Confirm a Conduit's shutdown on close | No |
| LIF-027 | Do not roll back opened Conduits or successful parts after partial setup | No |
| LIF-029 | Reject replacing a still-live declared slot with conflict | No |
| LIF-031 | Reject authored resolved-ID references in actor requests | No |
| LIF-032 | Reject using `set-up.v1`/`retire.v1` for a template or Conduit | No |
| LIF-035 | Remove a closed Conduit immediately, keeping records | No |
| LIF-037 | Check optional expected ID before lifecycle actions in E12 | No |
| PRV-003 | Use built-in runner, local machine and fetching; declare supported Package registries | No |
| PRV-005 | Model provider calls separately from requests; keep their correlation and records | No |
| PRV-007 | Request filesystem run only if stopped; bind root; unavailable if run state unknown | No |
| PRV-008 | Ask the runner to stop an instance's run on retirement | No |
| PRV-009 | Translate run reports into Running conditions, separately from lifecycle | No |
| PRV-022 | Build identity input with the names assigned by the specification | No; crypto/file delivery belongs at shell boundary |
| PRV-024 | Keep identity values out of diagnostics, records and public results | No |
| REV-001 | Assign one revision per object | No |
| REV-002 | Check a revision before replacing a template in E11 | No |
| REV-003 | Advance revision for an actual stored definition/config change | No |
| REV-004 | Preserve revision on equal replacement or lifecycle-only change | No |
| REV-005 | Never reuse a prior revision value | No |
| REV-011 | Return invalid if required change revision is absent | No |
| REV-014 | Conflict immediately on another setup/retirement/change in progress | No |
| IMP-001 | Start model work only through a listed, explicitly marked operation | No |
| IMP-002 | Start no model work from registration, setup, opening or observation | No |
| IMP-003 | Treat an absent startsModelWork mark as false | Part: mark shape only |

### The access decision

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| ACC-001 | Refuse a request absent an open Conduit listing that operation | No |
| ACC-002 | Record the access decision before dispatch | No |
| ACC-003 | Permit exactly the listed operations; an empty list permits none | No |
| ACC-008 | Apply all applicable restrictions, never broaden the operation list | No |
| ACC-009 | Do not import restrictions from another Conduit | No |
| ACC-011 | End access on close, including repeats through the old tool | No |
| ACC-012 | Authorize close against access rights and protect root's Conduits | No |
| ACC-015 | Reject carried filters unsupported by plain/engine targets as unavailable | No |
| ACC-016 | Require a serving instance's own Conduit for an onward request | No |
| ACC-017 | Carry original caller and filters through a serving chain | No |
| ACC-019 | Refuse serving an invocation after it ended; arrange no later work | No |
| ACC-022 | Give root's first-start access Conduit no bound | No |
| ACC-024 | Never close a root-consumer Conduit at an actor's request | No |
| ACC-027 | Report restrictions as mediated, never enforced | No |
| ACC-029 | In boundary vectors, compare bound targets/Pools/operations, applying it in engine | No |
| ACC-030 | Keep delegated rights within bounds; management is not narrowed to one template | No |
| ACC-032 | Permit root-reserved operations only for root on its own behalf | No |
| ACC-033 | Use the ordered routing table, including the extra owning-Pool registration authority | No |
| ACC-034 | Deliver an in-flight result after close, but refuse subsequent onward calls | No |
| ACC-036 | Decide each request and repeat from current Conduits | No |
| ACC-037 | Report unavailable and inspect nothing else if authority/existence cannot be read | No |
| IFC-012 | Derive tools only from open Conduits; map local declaration name plus operation name | No |
| IFC-013 | Remove tools on close | No |
| IFC-014 | Reject tool-name collisions at instantiation, including unopened Conduits | No |
| IFC-020 | Derive original caller from the served invocation, not supplied trace metadata | No |
| IFC-021 | Refuse a nonactor making a request without a serving invocation | No |
| IFC-022 | Accept an established instance identity only while that ID exists | No; shell verifies proof |
| IFC-028 | Refuse a serving ID owned by another instance or no longer in progress | No |
| IFC-032 | Refuse a request that establishes no instance | No |
| IFC-033 | Build finite, self-contained tool schemas from target-held contracts; preserve hints | No; loader has a document-schema bundle, not this general publisher |
| IFC-038 | Produce tool-list changes on open/close; private lists with listChanged | No; core computes list changes, shell sends notifications |

### How a request ends

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| OPS-001 | End a dispatched invocation with exactly one outcome, subject to reason exception | No |
| OPS-002 | Use succeeded only for the effect the contract says occurred | No |
| OPS-003 | Use failed only when none of that effect occurred | No |
| OPS-004 | Use unknown for partial or unconfirmed effect | No |
| OPS-005 | Use one of four reasons and no outcome for an undispatched request | No |
| OPS-007 | Make missing/malformed Conduit-aware results or invalid successful outputs unknown | No |
| OPS-008 | Keep invocation state separately from records; key by caller/sender/target/key | No |
| OPS-009 | Return a successful invocation's retained result on a matching eligible repeat | No |
| OPS-010 | Conflict for same scope/key but different default-filled contents | No |
| OPS-011 | Give no-key requests new invocation identities even for equal contents | No |
| OPS-012 | Never redispatch unknown plain writes, even if a contract says deduplicates | No |
| OPS-013 | Leave unknown unknown until authorized reconciliation | No |
| OPS-014 | Reconcile only with an actor's evidence, through ACC-033 authority | No |
| OPS-018 | Preserve at-most-once effect under the promise's key/reorder/restart conditions | No |
| OPS-019 | Apply request/access/outcome conventions to management requests too | No |
| OPS-022 | Aggregate setup parts as succeeded, failed only with no effects, otherwise unknown | No |
| OPS-023 | Apply unreadable, authority, invalid, conflict, other-unavailable precedence | No |
| OPS-024 | Retry failed invocations only on a caller repeat; apply filtered-read case correctly | No |
| OPS-025 | Reconsider a former reason afresh; don't cache it as an invocation | No |
| OPS-026 | Fill operation defaults before validation, comparison and dispatch | No |
| OPS-027 | Forward cancellation and report unknown unless no effect is confirmed | No |
| OPS-028 | Derive stable per-instance hook keys if a hook is selected in a variant | No |
| OPS-029 | Report invalid for a referenced object/invocation that does not exist, after authority | No |
| OPS-030 | Conflict on reconciling an invocation that is not unknown | No |
| OPS-032 | Conflict on an in-progress repeat, returning the invocation ID | No |
| OPS-033 | Reject invalid operation inputs after default insertion, before dispatch | No |
| OPS-034 | Conflict on lost declared outputs of a succeeded invocation, returning its ID | No |
| IMP-007 | Never redispatch after end/restart without a caller repeat | No |
| IFC-001 | Produce the mapped plain tool call with operation arguments only | No; transport is shell work |
| IFC-007 | Send no Conduit keys or identity/config/revision to the plain server | No |
| IFC-010 | Interpret plain results/errors conservatively by its table | No |
| IFC-023 | Treat an optional nonempty metadata idempotency key as the key; none means new | No |
| IFC-025 | Produce mutually exclusive reason/outcome, isError, diagnostics and plain content | No |
| IFC-045 | Declare only the protocol capabilities the interface permits | No |
| IFC-046 | Model the one wire resend on the specified errors separately from a new attempt | No |

### Records

| ID | What the core must do for S1 | Loader |
| --- | --- | --- |
| REC-001 | Emit records for every decision, no-dispatch end, dispatch, end, provider call and object/state/revision change | No |
| REC-002 | Fill applicable caller/consumer/conduit/target/invocation/serving/key/time/end fields as record.v1 CloudEvents | Part: record schema compiled; emission/correlation no |
| REC-003 | Keep records immutable except for explicit purge | No |
| REC-004 | Permit purge only by root, and record the purge | No |
| REC-005 | Preserve order per source, with no invented cross-source order | No |
| REC-006 | Report unreadable/lost record ranges as gaps | No |

Run observations must not mutate definitions (OBS-001), must carry time/source (OBS-002),
and inspection cannot add access or effects (OBS-006). They are core responsibilities, not
loader checks. These are observation boundaries for the shell, not polling mechanisms to
invent for this increment. Provider delivery/identity proof and runtime environment rules
belong in increment 3's orders. Full controls, secrets, distributed providers, tasks and
retirement recovery need more promises than this S1 inventory, before a claim about them.

## 3. The safety cases S1 touches

These are **proposed vectors, not results**. Each starts from a fresh core unless its trace
says otherwise. Fixture mutation is preparation, never a covert management request. A request
is dispatched only after current access is decided. Each vector checks state, decisions,
external intents and records, including that a forbidden call produced no intent. The
[round 6 evidence table](../../reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage)
and its [decisions](../../reaudit/round-6/DECISIONS.md) supply E05/E08/E11/E12's provenance;
the promises below decide the expectations.

### E05 first: standalone schema definitions

Prepare tiny package/contract variants and a target holding them. Register and open a Conduit
to an Operation using each Schema, then ask the core for the tool definition. Validate each
sample twice, against the authored schema in its original contract context and against the
standalone wire definition **with no package resolver available**. Compare verdicts, not
library error wording or an arbitrary ordering of `$defs`. Require finite output and local
references that all resolve. Exercise both inputSchema and outputSchema.

| Vector | Inputs | Expected decision and observations | Requiring promises |
| --- | --- | --- | --- |
| E05-local | Schema `path.v1` has `$defs.part: {type: string, minLength: 1}` and refers to `#/$defs/part`; operation input `path` refers to that contract | Bundle keeps the pointer inside that contract's copy; `"x"` accepted, `""` and `3` rejected as path values; no network access | IFC-033, SCH-001, SCH-015 |
| E05-two-contexts | Two packages each export `path.v1`, each has local `$defs.part`, one constraining strings and one integers; a tool uses both | Two distinct copies with qualified address/version keys; a string on the first and integer on the second pass, swapped values fail; include two releases reached through different package manifests | IFC-033, NAM-014, NAM-023 |
| E05-recursive | Schema `node.v1` has required string `value` and optional `next` referring to itself via `#`; another pair of contracts refer mutually | Finite one-copy-per-contract bundle, self/mutual references close over the right copies; finite linked values pass, a nonstring `value` at depth two fails | IFC-033, SCH-015 |
| E05-siblings | A `$ref` to a string Schema sits beside `minLength: 2`; include a sibling constraint beside a reference that itself contains local pointers | Preserve both assertions; `"ab"` passes, `"a"` and a number fail; do not inline by dropping siblings | IFC-033, SCH-001 |
| E05-escaping | Contract copy key contains address slashes and `#`; also refer to a local property whose name contains `/` and `~` | Pointer tokens escape `~` and `/`, URI fragment escapes `#`; every target is unchanged after rewriting | IFC-033, SCH-015 |
| E05-core | Present the actual core `register.v1` and `replace.v1` tool definitions | Definitions are standalone despite document/schema recursion; representative valid inputs pass and a malformed document fails; no network resolver used | IFC-033, SUB-005 |

Add two adjacent publication checks: a handle becomes a wire string (IFC-011, IFC-033), and
a Type control narrowed to maximum 50 is published as `allOf` of the contract's maximum 100
and that narrowing, accepting 50 and rejecting 75 (IFC-033, TPL-011). A missing control is
omitted, and a call to its would-be tool is invalid under TPL-013, not a permission to invoke
it. These are small E05 variants, not controls added to the base S1 project. The known root
`$defs` collision in section 6 must be decided before its rejection/acceptance vector has a
normative expected answer.

### The remaining safety traces

`core#` below abbreviates the current core package address **in this table only**. It is not a
new reference syntax. Every actual vector uses the full address from the names file.

| Vector | Inputs | Expected decision and observations | Requiring promises |
| --- | --- | --- | --- |
| bootstrap | Empty state, assigned IDs, canonical machine report and first-start event | Exactly five ready objects and two open root Conduits; no other run/model intent; protected-name/retirement follow-ups conflict/refuse as appropriate | FST-001, FST-002, FST-003, FST-005, OBJ-025, LIF-034 |
| S1-register-set | Preinstalled draft package snapshots; all project files in several permutations, then a variant with an unresolved instance target | Accepted permutations make equivalent objects and nested-source mappings modulo generated IDs/names; invalid variant publishes none | SUB-001, SUB-002, SUB-003, OBJ-009, INS-016 |
| S1-duplicate | Same-submission duplicate kind/name; separate request reusing an existing kind/name; cross-kind name duplicate in one project | First invalid under SUB-004; existing-name clash conflict under OBJ-025 with OBJ-002 detail; cross-kind project duplicate invalid under NAM-027; no partial changes | SUB-004, OBJ-002, OBJ-025, NAM-027, SUB-003 |
| S1-archive-escape | Actual archive variants with absolute path, `..` segment and outside link, alongside a valid root manifest | Reject each unsafe archive under MAN-014 before exposing any member outside the extraction boundary; no registered objects | MAN-014, MAN-013, PKG-022 |
| S1-no-fetch | Valid registration whose manifest names an absent package, with a fetch URL that would work | Invalid; diagnostic PKG-006; no fetch/install intent; identical installed pin succeeds without implicit action | PKG-006, SUB-010, NAM-012 |
| S1-target-type | Change a declared Conduit's target to a Type offering no directory capability, then change its operations to one outside the capability | Reject instantiation under CON-021; operation-list variant rejected under CON-017; atomic submission | CON-021, CON-017, SUB-003 |
| S1-plain-filter | Add a non-core Requirement and apply it to a filesystem Conduit; separately carry it through an onward call to plain filesystem | Registration invalid under CON-022 in first case; carried-filter request unavailable under ACC-015 in second; no filtered call sent to plain | PKG-020, CON-022, ACC-015 |
| S1-pool-authority | Register Resource into existing `workspace` using an actor with register at management but none at workspace; compare with new Pool and its Resource in one submission | Existing-owner request refused, no diagnostic exposing the named Resource; same-submission Pool case authorized by management alone | ACC-033, OPS-023, IFC-025 |
| S1-setup-sender | Non-root actor can set up an Agent but cannot open one required declaration; one open succeeds before the refused part | Opening uses sender's access Conduit, never Agent's or original caller's; no run/hook begins; partial outer setup unknown, state registered, successful open preserved; no rollback | ACC-005, CON-004, TPL-012, OPS-022, LIF-004, LIF-027 |
| S1-setup-again | Complete base setup twice with fresh request keys; also open an already open Conduit | Same four nested IDs, no additional Conduits, no second run if reported running; successful no-op open; lifecycle does not advance revision | CON-004, ACC-006, PRV-007, REV-004 |
| S1-target-registered | Open A's directory Conduit before setting up shared; request read; then simulate successful Resource setup and read again | Open succeeds before target ready; read unavailable without dispatch, later read dispatched | CON-023, CON-024, LIF-003 |
| S1-exact-access | A and B make listed reads, A guesses write tool, B writes, both post/read history; add a new server tool not listed in contracts/Conduits | Only listed tools and calls; A's guessed write refused, no invocation; server upgrade adds no implicit access; both history paths independent | ACC-001, ACC-003, ACC-002, IFC-012, CON-017 |
| S1-close | Close only A's project Conduit, then A repeats an earlier successful read, B reads and A reads history; inspect/open/close removed Conduit by name | A's old project route refused despite cached invocation; B and history still work; removed-object management requests invalid; A stays ready | ACC-011, ACC-036, CON-014, LIF-035, OPS-029 |
| S1-close-in-flight | Dispatch B's write, close its Conduit, then deliver its successful result; try a subsequent onward request in that chain | Deliver original result and keep outcome; refuse new onward request; no compensation; records keep the removed Conduit's ID | ACC-034, ACC-017, REC-002, LIF-019 |
| S1-restore | Close A's declaration, set A up without replacing it, then explicitly register nested replacement, then set up | Missing slot conflicts; no implicit recreation; new slot has a new registered ID; root opens it only on later setup; B remains unchanged | LIF-008, LIF-009, LIF-029, CON-004, ACC-005 |
| S1-authority-unreadable | Identity exists but Conduit state cannot be read; give invalid arguments too; repeat with existence itself unreadable | Unavailable, no dispatch, examine/report no argument or object details; caller omitted when existence cannot be established | ACC-037, OPS-023, REC-002 |
| S1-reason-order | Requests simultaneously lack access and have invalid arguments; authorized request has invalid input and stale expected ID | Refused first, with no object diagnostics; authorized variant invalid before conflict; unreadable request invalid before access | OPS-023, OPS-033, IFC-025 |
| S1-plain-ends | Plain read/write completions: success, isError, no result, input_required, -32602 and -32021 | Success -> succeeded; ordinary error/-32021 -> failed for reads, unknown for writes; no result/input_required -> unknown; -32602 -> failed by the current table; preserve content and isError conventions | IFC-010, IFC-025, OPS-001, OPS-005 |
| S1-key-scope | Authorized effectful requests with same key/contents/caller/sender/target; independently change content, sender, original caller through one shared broker, or target, holding all other fields fixed; no-key and defaulted-equal input variants | Successful repeat no dispatch; changed content conflicts; different sender, caller and target cases each dispatch with distinct invocation IDs, as does each no-key request; defaults compare equal; in-progress repeat conflicts with invocation ID | OPS-008, OPS-009, OPS-010, OPS-011, OPS-026, OPS-032 |
| S1-unknown-write | Dispatch keyed plain write, lose result, repeat with same content/key, restart core from stored state and repeat | Unknown remains unknown, no new plain call; even deduplicates mark cannot authorize plain retry; no automatic retry after restart | OPS-012, OPS-013, OPS-018, IMP-007 |
| S1-reconcile | Unknown write, authorized root reconcile with what it checked, then repeat; compare unauthorized Agent and already-ended invocation | Evidence-based succeeded/failed, never guessed from a timeout; unauthorized refused; nonunknown conflict; no fabricated outputs after reconciliation | OPS-014, OPS-030, ACC-033, OPS-009 |
| S1-purge | Root purges records before a time, then repeats an old keyed write; compare non-root purge | Record purge recorded; invocation index survives, so effect not repeated; non-root refused | REC-004, REC-003, OPS-008, OPS-009, ACC-032 |
| S1-records | Run allowed/refused/invalid/conflict/unknown cases; include provider-run completion and lifecycle changes; simulate an unreadable range | Validate each applicable record against record.v1, causal IDs and per-source order; access before dispatch; no identity material; gaps explicit | REC-001, REC-002, REC-005, REC-006, ACC-002, PRV-024 |
| E08-broker | Variant Resource with a trusted approved-reviewer operation and its own management Conduit; caller has broker access only; feed simulated trusted onward registration | Direct management request refused; onward request uses broker's Conduit and original caller, not caller authority; filter carried to management unavailable; trusted adapter's approval check is separate provider evidence | ACC-030, ACC-016, ACC-017, ACC-015, IFC-020 |
| E11-replace | Submitted local Type/Operation/item and template; make old instance; replace template with current revision and edited contracts/manifest/item; make new instance; retry equal replacement and stale revision | New instance resolves new project digest/item; old keeps old ones; equal replace no revision advance, stale revision conflicts; omit handed-over local contract and replacement invalid | SUB-008, SUB-009, NAM-009, NAM-022, INS-020, PKG-005, REV-002, REV-004, NAM-028 |
| E12-name-reuse | Inspect Resource and capture ID; retire/remove it (or use never-setup variant); register new Resource with same name under a non-retired Pool; inspect/set-up with old expectedId using an actor with the applicable owning-Pool authority but no management access, and an actor lacking that Pool authority | Pool-authorized actor's stale expected-ID request conflicts with both IDs, changes nothing; resolved-ID-as-address invalid; actor lacking the applicable owning-Pool authority refused before ID details; new object not reached by old stored references/credentials | ACC-033, LIF-037, LIF-031, OBJ-010, IFC-022, OPS-023 |

The -32602 safety assumption is already recorded as open by the repository; the plain-ends
vector follows current IFC-010 rather than claiming that the protocol proves no partial
effect. The wire resend cases in IFC-046 likewise remain separate from an application repeat.
No study result closes those known questions. Filtered replay variants from round 6's finding
6-1-02 are adjacent boundaries: read-only success is redispatched on a caller repeat under
OPS-024; success with effect and outputs conflicts under OPS-034; success with effect and no
outputs returns its outcome under OPS-009, with no filter call. They require Conduit-aware
fixture implementations, since the S1 plain item supports no filters.

## 4. A proposed format for the vectors (A39)

Recommend file-based YAML cases whose documents remain ordinary Conduit files. The alternative
is Rust-only fixtures, which are convenient now but cannot drive independent builds without
reading this implementation. The portable case format and matcher are runner conventions,
not new Conduit keys or accepted behavior. The admin should settle them before implementations
copy them. No `conformance/` files are created by this study's restricted scope.

Proposed layout, to be created by later orders:

```text
conformance/
  README.md
  case.schema.json
  profiles/s1.yaml
  language/loader/<case>/case.yaml
  language/resolving/<case>/case.yaml
  interface/e05/<case>/case.yaml
  conventions/s1/<case>/case.yaml
  fixtures/s1/project/conduit.yaml
  fixtures/s1/project/templates.yaml
  fixtures/s1/project/instances.yaml
  fixtures/s1/packages/filesystem/conduit.yaml
  fixtures/s1/packages/filesystem/contracts.yaml
```

Each case directory can contain its own `project/`, `packages/` and raw input bytes, keeping
fixtures that differ from S1 next to their expectations. Shared fixtures are included by
relative case paths, resolved by the runner, never fetched. Package states distinguish
**preinstalled snapshots** (address/version/digest plus already-loaded documents) from
**install archive inputs** (actual compressed bytes, expected hash and a fetching completion).
Only the latter prove PKG-004/PKG-022/PKG-026. The core install contract
also rejects a manifest stating a different address or version; its invalid result follows
PKG-023, while PKG-026 specifically names a mismatch of digests. Hashes of submitted projects are computed from the
original manifest under SUB-008, not from expanded paths or runner fixture metadata.

A case contains a stable ID, layer(s), promise IDs, title, a stated implementation profile,
initial state recipe, files/directories, a sequence of inputs and assertions. The profile
fixes the first build's relevant DL choices, including depth 128, name generation, immediate
ACC-037 unavailability, output retention and one record source. Every case declares whether
it tests language, conventions or interface so a Language-level runner does not silently
pass convention cases by skipping them.

The runner input vocabulary should distinguish first-start, register-directory, request,
provider-report, implementation-result, cancellation, tick and inspect-state. A request names
its established sender identity, Conduit (or wire tool for publication cases), operation,
arguments and optional idempotencyKey/serving. These are runner fields; metadata is encoded
as IFC-023/IFC-020 require only at the interface boundary. No actor supplies stored IDs as
reference values in documents. Capture IDs from registration results by kind/name and nested
source, then bind symbols in expected state/records; a runner binding used as `expectedId`
is an actual UUID value, not a new reference form.

For example, this is runner pseudodata for a complete request after the S1 setup recipe,
not a Conduit document and not a specification of a new engine operation:

```yaml
id: s1-a-write-refused
layers: [convention, interface]
promises: [ACC-001, ACC-002, IFC-025, REC-001]
profile: s1
initial: {recipe: s1-ready}
steps:
  - request:
      sender: {capture: agent-a}
      tool: project.file.write.v1
      arguments: {path: /tmp/conduit-s1/shared/x.txt, content: x}
    expect:
      decisionRules: [ACC-001]
      reason: refused
      noOutcome: true
      noDispatch: true
      diagnostics: []
      stateUnchanged: true
      recordKinds: [access-decision, request-ended]
```

The actual interface call to that unpresented tool is a protocol error with reason in its
`data` under IFC-012, not a normal tool result; the runner's semantic expectation above
normalizes both paths. A wire case must also assert the actual error envelope. Every expected
result distinguishes an outcome from a reason, checks isError and invocation IDs where they
apply, and asserts no effect intent for no-dispatch cases. Internal decision citations remain
observable to the test even where IFC-025 forbids diagnostic details in a refused reply.

For diagnostics, match an exact `ruleId`, level where fixed, instance JSON Pointer and
identified document. Compare sets where DL9 leaves ordering open. Text assertions concern
facts (such as the missing input or the limit and its value), not complete library prose.
For a schema rejection require `ruleId: SUB-005`, the Schema contract in message.text and
`properties.keywordLocation`; also require absoluteKeywordLocation when known. A case pins the
root schema against which keywordLocation is evaluated; validate that following that location
reaches the failing keyword. A portable test must not require a private bundler's incidental
`$ref` traversal path. Explicit core rejections such as INS-014 or OPS-033 remain those rules;
they validate a config or argument against a Type/Operation schema, rather than rejecting a
document against a document Schema contract. Wrap loader diagnostics as diagnostic.v1 SARIF
results with logicalLocations, preserving source positions as additional physical locations.

`stateUnchanged` means the object-definition/lifecycle projection is unchanged; it does not
suppress the required new records for a refused request.

A schema-invalid fixture can express its expected diagnostic this way. The nested matcher
objects describe assertions, not values that an engine puts into a diagnostic:

```yaml
diagnostics:
  - ruleId: SUB-005
    level: error
    document: {file: project/instances.yaml, index: 1}
    instanceLocation: /metadata/name
    messageNamesContract: github.com/shrav-k/conduit/core#name.v1
    properties:
      keywordLocation:
        required: true
        pointsTo:
          contract: github.com/shrav-k/conduit/core#name.v1
          pointer: /pattern
      absoluteKeywordLocation: {requiredIfKnown: true}
```

State assertions use a partial structural projection for implementation-defined inspection
fields, but exact sets for required objects, Conduits, operation lists and required
inspection outputs. Records are validated as CloudEvents and compared in their source order,
with symbolic IDs/times, causal links and event facts; no invented global order across sources.
Assertions for tool definitions compare validation behavior and reference targets, while also
checking the fixed IFC-033 key/escaping rules. Test dynamic `patternProperties` and adjacent
`additionalProperties` explicitly: increment 1 leaves those on authored schemas to the core.

The runner loads bytes with conduit-loader, passes immutable values to the core, consumes only
the expected external intent, injects its scripted completion, and asserts both decisions and
new state after each step. An unexpected intent, missing assertion, unresolved symbol or
unsupported input fails the case, rather than skipping it. Run permutations for set semantics
and sequences for requests; the runner must not reorder requests. Output a case report with
pass/fail and failing expectation plus cited promise. Code coverage is not a substitute for
these assertions. A manifest of cases records which safety rows above each covers.

### Carry forward increment 1's settlements

The four original settlements whose coverage needs carrying from the loader tests are listed
explicitly; their current amended text governs. Keep existing tests while exporting file
fixtures. Do not report these as new findings.

| Settlement | Existing evidence to port | Portable expectation |
| --- | --- | --- |
| 1-01, completed by 1-14: schema rejection attribution | `every_invalid_example_is_rejected_by_the_step_its_name_gives` in tests/examples.rs; `every_fault_of_a_project_is_reported` and `a_schema_rejection_carries_the_keyword_location` in tests/project.rs | Schema-invalid document and manifest both cite SUB-005, with document pointer, Schema contract, keywordLocation and absoluteKeywordLocation where known; installation uses the same diagnostic |
| 1-11, completed by 1-16: engine limits | `values_nest_no_deeper_than_the_limit` in tests/reading.rs | Stated depth 128 accepts the boundary and rejects 129 under SUB-011 with that limit/value; 63 and 64 accepted; add file/submission/archive limits only with a declared value, no unstated size rejection |
| 1-12: string value model | `of_i_json_only_two_rules_apply_to_strings` in tests/reading.rs | Noncharacters accepted wherever YAML/JSON permit; unpaired surrogate rejected under SUB-005; JSON surrogate pair accepted; YAML two surrogate escapes rejected; single non-BMP escape accepted; malformed UTF-8 rejected |
| 1-13: case of format letters | `a_duration_matches_the_grammar_of_rfc_3339_appendix_a` and `a_uuid_matches_the_grammar_of_rfc_9562` in tests/formats.rs | Test actual Schema/config values using `p1d`, `PT1h` and mixed-case UUIDs; grammar accepts them, rejects malformed neighbors under the validation stage's rule |

Newer amendments also need file vectors: one leading BOM accepted/ignored in either syntax
(1-15, SUB-005/SCH-013), allowed and forbidden pattern tokens (1-17, SCH-016), and the
illustrated limit of a Package artifact pin (1-18, MAN-010). The last is a contract/document
check, not an execution guarantee. Existing examples for JSON scalars, numeric rounding and
unsupported patterns should be referenced rather than copied with divergent expectations.

## 5. A proposed shape of the core

Recommend two new crates beside `conduit-loader`: `conduit-core` and `conduit-conformance`.
The latter is the test runner and filesystem edge for vectors; it need not be a public API.
An alternative is a single core crate with integration-test-only vector support. Two crates
keep fixture I/O and test scripts out of the model's public state machine. A later
`conduit-shell` crate owns processes, HTTP, authentication, timers and durable storage.
All share the existing workspace, lockfile and pinned toolchain. No provider framework or
second engine is needed for S1.

Proposed pure interfaces, names not yet decided:

```text
resolve(snapshot, loaded_project, package_catalog, allocation_inputs)
  -> ResolvedSubmission | Diagnostics
step(state, input, context)
  -> Transition { state, decisions, records, external_intents, reply }
tools(state, established_consumer)
  -> ToolDefinitions | Decision
```

`resolve` evaluates document meaning without starting runs or making effects. Its snapshot
includes lifecycle/status facts needed for ownership, placement and references. Where a
registration needs machine placement, `step` emits the provider-call intent and resumes with
supplied placement evidence before applying the candidate objects. Resolving itself performs
no I/O. A rejected submission publishes no candidate object and starts no implementation effect;
any provider consultation needed to decide placement is still recorded.
`step`'s published state is one value, which a shell can persist with its records before acting
on any dispatch intent. Internal setup parts are invocations where the promises say so;
provider calls are distinct events, not counterfeit Conduits.

State holds an object store indexed by kind/name and ID, immutable instance definitions,
template revisions, held package/project contexts and items, declared-slot mappings, lifecycle
and observed run/machine state, pending management parts and an invocation index separate
from records. Keep caller, sending instance, effective authorization target, Conduit of latest
attempt, operation, default-filled contents digest, invocation state/outcome, hook key where
applicable, supplied control values when later supported, outputs with their retention, and
parent serving links. A purge of records never edits that index. D19 counts serialized bytes
of both stores and their growth per request, with plain content separately identified.

Every branch that accepts/rejects, changes state, dispatches or answers a repeat emits a
Decision with requiring promise IDs, inputs/facts consulted and affected IDs; external replies
follow IFC-025's narrower disclosure rules. Accepted provider-contract facts cite the contract
selection too. Implementation choices cite their DL item in a separate field; a DL identifier
is not put into diagnostic.ruleId. Exhausted allocations, unexpected completions and broken
runner recipes are harness/embedding errors, not invented Conduit reasons. An undecidable
language case stops the affected vector as a finding rather than silently choosing behavior.

Run intents name an opaque identity-material descriptor. The shell materializes the actual
PRV-022 identity files for the runner; the model checks required field names/presence without
putting credentials in its serialized Decisions or records.

All nondeterministic inputs enter explicitly:

| Value | Proposed source at the boundary |
| --- | --- |
| Time | Supplied event instant and elapsed/deadline events; no reads of wall/monotonic clocks inside core |
| Object, invocation and record IDs | Supplied unique UUID allocation stream or deterministic allocator seed plus stored counter; every allocation recorded in transition state |
| Generated name suffixes and revisions | Profile-selected deterministic policy, supplied allocation state; verify uniqueness and name/revision rules |
| Machine/run reports, archive bytes, path base | Loaded values and explicit provider/fetch completions; no ambient hostname, cwd, network or package lookup |
| Identity | Established instance ID, none or existence-unreadable; shell checks tokens/TLS; no credential bytes in core's public values/records |
| Protocol/server replies | Typed raw completion/error/no-result/task status as needed, tagged with the outstanding call/attempt |
| Ordering | One input at a time and deterministic ordering for otherwise unordered internal work; preserve causal record order |
| Limits/retention | Explicit immutable profile and expiry inputs; no unstated cap and no implicit effect retry |

Recommend DL2 contents digests as SHA-256 of RFC 8785 canonical operation and filled arguments,
while still checking semantic equality under SCH-007. Recommend five lowercase alphanumeric
suffix characters with collision retry for DL15, and a stored monotonic revision counter.
Recommend DL4 immediate unavailable, DL12 no optional inlining, DL8 one record source and an
explicit record export, and DL10 depth 128 with outputs retained throughout a vector unless
an explicit expiry step tests OPS-034. These are admin choices for the first build's
conformance document, not promises for every engine. DL6 first-start interruption/reentry,
DL7 archive dialect and directory entry point, DL13 identity proof and DL16 process behavior
must be documented by the appropriate orders; only the pure first-start event is needed now.

## 6. Where the text does not decide

These are proposed findings and decision points, awaiting the admin. No specification file
has changed. The admin may settle findings under ruling 46 point 4; a fix adding/removing a
feature or changing the purpose goes to the owner. The known open items below are labelled
as such, rather than claimed as new findings.

### F1. Plain content on a successful repeat

**Candidate ambiguity: OPS-009, OPS-034, IFC-025; DL10.** The draft read/list Operations have
no declared outputs because the server supplies MCP content. OPS-009 unconditionally answers
a successful no-output repeat with succeeded and an empty structuredContent. IFC-025 says
that, where an outcome came from a plain result, content holds that result's content unchanged.
The durable invocation list and DL10 specify retention of **outputs**, not plain content.
One reader retains and replays that content; another reads OPS-009's answer as the recorded
outcome only and returns no original content. The repeat is important here because content
is the actual file read, not optional decoration for S1.

Options: (a) explicitly retain/replay original plain content for successful no-filter repeats,
with a stated retention rule and answer after expiration; (b) explicitly answer such repeats
with only the recorded outcome; (c) require an adapter with declared outputs so existing
output-retention rules apply. **Recommend (a) for S1**, with the admin settling whether expired
content produces conflict or is omitted, and recording its cost under D19. It should never
redispatch a successful plain write to recover text. Until settled, replay-content expectations
remain pending; the invocation/effect safety expectations are already decided.

### F2. Root definition key collides with a required copy key

**Known open ambiguity: IFC-033, SCH-015.** Round 6's
[decision record](../../reaudit/round-6/DECISIONS.md#the-four-open-points) already identifies
an Operation's root `$defs` key equal to a generated contract-copy key. IFC-033 requires both
the fixed qualified/versioned copy key and the author's root keys to survive; there is no
stated collision procedure. E05 can construct this using a root key such as
`example.org/s1/filesystem@0.1.0#path.v1` and a reference needing that contract's copy.

Options: (a) reject such authored schemas with a specified promise and diagnostic;
(b) permit publication to rename the author's colliding key and rewrite every local pointer;
(c) change the fixed copy-key scheme for all publications. **Recommend (b)**: it keeps valid
contracts and their meaning, and confines the repair to publication. It needs an explicit
settlement because "members keep their keys" currently rules it out. Keep a pending vector;
do not silently overwrite either schema. This is a known issue needing an admin decision,
not a newly discovered unsafe result.

### F3. Record vocabulary and inspection shape

**Known format gap: REC-001, REC-002, LIF-013; DL8.** `record.v1` deliberately leaves event
`type`/`source` naming to the engine, and inspection has open declared/status maps. The core
cannot obtain a canonical full event taxonomy or inspection serialization from those
promises. The required facts and correlations are fixed, so this is largely a format choice
for A39, not absent access behavior.

Options: (a) portable required-field projections plus a documented first-build event
vocabulary; (b) freeze complete reference snapshots as the portable contract.
**Recommend (a)**. Admin defines a small event vocabulary, such as access-decision,
request-ended, invocation-dispatched, invocation-ended, provider-call and object-change, and
records gaps explicitly. Keep exact object sets and required fields; do not let projections
hide omissions. If an inspection must expose additional portable facts for repair, propose
those separately in the promises with a vector.

### Existing safety questions on plain errors

**Known open safety assumptions: IFC-010, IFC-046, OPS-003, OPS-018.** The tracker and round 6
already leave -32602 as proof of no effect and the pre-effect interpretation of -32020 for a
resend open. A70's successful calls do not answer them. Options are to follow the current
published table while keeping the questions open, or have the admin commission the focused
protocol check and settle them before their retry paths ship. **Recommend the focused check
before the shell's retry implementation**, retaining the current-table core vector meanwhile.
This study neither changes the table nor claims those checks ran.

### Decisions of scope and provider integration

These are not language defects; the specification deliberately leaves them elsewhere.

| Question | Options | Recommendation and decision holder |
| --- | --- | --- |
| What history semantics are needed for T2? | A70 file-per-message convention; an existing atomic append tool; one shared file | Admin: the first, with the stated overwrite/ordering limits; owner only if changing S1 to require a new feature |
| What packages/pins does the core consume before fetching exists? | Explicit preinstalled snapshots; complete archive pipeline in the first resolver slice | Admin: snapshots first, then actual archive vectors before claiming installation; replace illustrative hashes in runnable packages |
| How much Agent provider behavior belongs in increment 2? | Registration-only Types; build the real wrappers now | Admin: registration-only, as the order states; wrappers are increment 3 with every required flag/helper verified |
| Must a vector compare complete JSON or portable facts? | Exact snapshots; rule/fact/schema-semantic matching | Admin: portable facts, exact mandatory sets and failing keyword target; independent builds can choose their allowed representations |
| How is first-start retried and committed? | Recreate on every start; one transactional empty-state bootstrap and reload existing state | Admin, DL6: transactional bootstrap; never regenerate identities on an ordinary restart |
| How do calls get durable dispatch records? | Return state/records/intents and require persistence before execution; let dispatch race persistence | Admin, DL2: persist state and records first, then execute intent; prove restart does not blindly replay effects |
| How are defaults and patterns applied? | Reuse a general validator unchanged; add the profile's default traversal and ECMA-262 pattern handling | Admin: the latter, reusing crates while preserving the profile; no new schema dialect |

No contradiction requiring an owner feature ruling was established by this reading. This is
not a clean-read declaration: the core and vectors have not been built, and the pending
ambiguities are not answered by loader acceptance.

## 7. A proposed sequence of work orders

Each row is intended for one Codex run with a narrow file scope and explicit checks. These
are proposed order names, not issued work orders. The admin should assign actual numbers,
settle section 6 and the proposed formats, and create the increment's finding log before
issuing dependent implementation orders. A worker must stop the affected case at a newly
undecided promise rather than invent its answer.

| Name | Scope | Done when | Depends on |
| --- | --- | --- | --- |
| I2-01: vector contract and pure bootstrap | `conformance/` format/profile, core crate skeleton and deterministic first-start transition, elementary record/decision values; no shell | Bootstrap vector shows exactly five ready objects/two open Conduits, protected first-start cases, supplied time/IDs, no extra intent; every decision cites a promise; loader settlement fixtures introduced | Admin decisions on formats, DL6 and allocations |
| I2-02: authored schema evaluation and E05 | Core schema/default/bundling module, conformance runner schema assertions, E05 and profile variants; reuse loader validators | E05 local/two-context/recursive/sibling/escaping/core definitions standalone and verdict-equivalent; handle/control publication variants; dynamic patternProperties correct; root-key collision settlement has a vector | I2-01; admin settlement of F2 |
| I2-03: package and project contexts | Immutable catalog and manifest resolution, pure archive-validation evidence boundary, pin/export/name rules, submission digest; core tests/vectors | Filesystem/history/Agent snapshots validated and resolvable; missing/wrong pins reject without fetch; actual small gzip-tar fixtures verify digest/address/version and explicit install outcomes; core built-in cannot be replaced | I2-01, I2-02; admin archive/profile choice |
| I2-04: registration and instance resolution | Template/argument/reference/owner/placement/nested-slot resolution and atomic register; S1 fixtures under conformance | All draft project documents register in any order with 15 named and four nested objects; refs stored by ID; rejected variant publishes none; no setup/run/model effect; generated names reported | I2-02, I2-03 |
| I2-05: access and tool projection | Pure request authorization, ACC-033 routing, bounds, current-identity/existence inputs, exact tool list and no-dispatch replies | A/B rights, missing Pool authority, unreadable authority, bound limits, carried-filter-to-plain, guessed tool and protected-root cases pass; refused replies reveal no named-object details | I2-02, I2-04 |
| I2-06: invocation state and endings | Request normalization, outcomes/reasons, plain result table, idempotency index, caller-driven attempts, cancellation and reconciliation; no transport | Key-scope/content/default/in-progress/unknown-write vectors pass; unknown plain writes never redispatch; reason order and result envelope correct; F1 settlement represented; record purge cannot erase invocation state | I2-03, I2-05; admin settlement of F1 |
| I2-07: setup parts and Conduit lifecycle | Ordered setup interpreter, built-in run/report intents and scripted completions, open/close/removal/nested replacement; no process spawning | Base setup succeeds; sender authority, target-not-ready, partial setup/no rollback, same IDs on repeated setup, close-in-flight and explicit restore vectors pass; every part has correct invocation/caller links | I2-04, I2-05, I2-06 |
| I2-08: records, inspection and repair boundaries | Complete record emission and projections, retention/purge/gaps, expected-ID follow-up, revision-checked template replacement and narrow E08 broker fixture | REC cases and E08/E11/E12 pass; all applicable fields validate; access record precedes dispatch; old contexts/IDs preserved, no implicit effects; record and invocation bytes counted separately | I2-06, I2-07 |
| I2-09: integrated S1 and handoff | Run all increment vectors, finish coverage manifest, finding log, first-build conformance choices and shell interface notes | T1/T2 core trace registers/resolves/sets up, expected calls end/record correctly; all safety vectors pass or undecided cases are resolved with promise fix/vector before completion; no claim real Agents ran; increment 3 receives concrete intents/completions/tool contract and outstanding provider requirements | I2-01 through I2-08 |

Intermediate orders should emit applicable records from their first transitions; I2-08
completes and audits coverage rather than adding accountability after access has shipped.
If I2-03's archive cases do not fit one run, the admin should split archive verification into
its own order; preinstalled snapshots must not conceal that omitted coverage.

For every implementation order: the applicable repository Python checks and whitespace check,
Rust release tests, clippy with warnings denied and fmt; changed promise text requires its
vector and the first-build finding entry in the same change, under ruling 46. The worker's
scope must name those files explicitly. Each order reports its own choices, actual check
exit codes/last lines and anything it could not do. The admin commits/merges; workers do not.

Increment 2 is done only when the first build's stated done-when holds: S1 registers and
resolves on the core, every decision cites its promise, safety vectors pass, and undecided
cases have findings. Increment 3 then supplies actual process/MCP/identity behavior and actual
client tool lists, and increment 4 supplies the history run and author observations. The
D08 traces and D22/D19 measures above make that handoff reviewable without treating expected
results as observations.
