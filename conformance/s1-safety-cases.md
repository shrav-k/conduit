# The safety traces proposed for S1 in increment 2


These are **proposed vectors, not results**. Each starts from a fresh core unless its trace
says otherwise. Fixture mutation is preparation, never a covert management request. A request
is dispatched only after current access is decided. Each vector checks state, decisions,
external intents and records, including that a forbidden call produced no intent. The
[round 6 evidence table](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage)
and its [decisions](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-6/DECISIONS.md) supply E05/E08/E11/E12's provenance;
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
