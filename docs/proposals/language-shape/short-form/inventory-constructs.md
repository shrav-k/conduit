Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# The language layer as a completeness checklist: every construct, the promises that define it, what it protects, whether S1 uses it, and its class

## Key points
1. The language layer has exactly 203 promises. 197 are in spec/promises/; the other 6 are IMP-003, LIF-002, LIF-007, IFC-014, IFC-043 and IFC-044.
2. 167 of the 203 are in S1's touched set (conformance/manifest.yaml s1_coverage). The other 36 are the constructs S2 to S10 need: requirements, filters and bounds; controls; Pool entries; placement, selectors and shared-host; providers; secrets; and `remote` items. A short form tuned on S1 alone would silently drop them.
3. My class tally of the 203: about 118 essential to meaning, about 48 reuse machinery, about 30 notation and about 7 change machinery. Most change machinery is in the convention layer: SUB-009, INS-020, LIF-008, LIF-009, LIF-029, LIF-035, LIF-037, REV-001 to REV-015, TPL-011 and TPL-013.
4. Notation a short form can replace freely: the frame (`apiVersion` on every document, `kind`, `metadata`, `spec`, `---`), reference spellings, map-versus-list shapes and the manifest's file layout. The rules behind them carry safety and must be restated: version known per unit, closed keys with no part ignored, strict syntax, the name grammar with its 63+63 tool-name budget, and no fields the engine owns.
5. What any form must express:
   - the five families and their standard arguments (`owner`, `placement`, `runtime`, `consumer`, `target`);
   - the Conduit: one consumer, one target and one capability, an explicit fully qualified operations list and requirements;
   - a stable local name;
   - a Type with its `config`, and the selection of implementations and hooks;
   - fixed placement, runtime and owner;
   - secret references;
   - package pins held per template and per instance;
   - the operator (root).
6. Reuse machinery (templates, inputs and destinations, defaults, `from`, nested instantiations, templates exported by packages) is what "short form as the language" would change most. But declared Conduits and Pool entries also carry essential meaning that must be moved if templates go:
   - the stable local name, which sets tool names;
   - the order of setup's parts, and optional parts;
   - the order of entries' dependencies, and that a Pool is made with all its Resources or not at all;
   - replacement under LIF-009 from "the template as it stands now".
7. Canonical documents are also the wire form of management. `register.v1` takes `document.v1` values, `replace.v1` takes a template with its contracts and manifest, and `inspect.v1` returns `declared` in the resolved canonical form. Under preservation test 5, Agents manage through these, so a normative short form must define that wire as well, or keep the canonical form beside it.
8. SUB-008's digest of a submitted project is RFC 8785 over the canonical contract documents and the manifest, and NAM-009 and PKG-005 keep it. A form that changes how contracts are written must redefine it. Conformance vectors are files (SUB-010), so a normative form also needs a file and directory rule.
9. What S1's authors wrote:
   - a Pool (`pool.v1`, no entries);
   - Resource templates with one input to `/config/root`;
   - Agent templates with `config`, and in the coordinator's run one declared Conduit;
   - Conduit templates;
   - many standalone Conduits: turns, root to Pool, history and grants;
   - manifests with pins;
   - the requests `replace.v1`, `register-replacement.v1`, `install.v1`, `open`/`close`, `retire` and `unregister`.

   They wrote no contracts, requirements, controls, entries, placement, runtime, secrets or `remote` items.
10. S1's costs fall on these classes: about 100 re-declaration lines (change: no in-place change, pins per object, standalone names); 80 to 190 frame lines (notation); 72 lines of root's Conduits to its Pool and for turns (essential under G2, but derivable by a rule that makes real Conduits); 21 pin lines (an essential pin in a notational place); 17 lines of template plumbing (reuse); and 23 lines of access facts.
11. Rules that look like notation but carry safety:
    - OBJ-004 and OBJ-009/LIF-031: no forged fields or IDs;
    - OBJ-010: a reused name never redirects a stored reference;
    - OBJ-019: closed keys;
    - SEC-001: secrets only as references, and where they may stand;
    - MAN-017, MAN-018 and SEC-021: what bindings may reach;
    - IFC-043 and IFC-044: `remote` rules;
    - NAM-007 and PKG-006: nothing deferred;
    - ACC-003 (G1): a Conduit never grants the whole capability.
12. Contracts have a different audience. S1 projects write none; package and provider writers (S7, S10) write all five kinds. A short form for authors can leave contracts as they are, but must keep reaching them by fully qualified reference (rulings 25 and 53 #8) and keep the Type-level rules for providers, secrets and package sources.
13. Ruling 53's accepted additions are not yet in spec/ and need a home in whichever form wins:
    - `as`;
    - `expectedId` on `open` and `close` (LIF-037 already covers four verbs);
    - the inventory in `inspect.v1`;
    - tombstones enforced at open and at register, which change registration's meaning;
    - descriptive tool descriptions, which give `metadata.description` a wire meaning where OBJ-018 says it has none.

    Rulings 49 to 51 (an account field, pending actions at an Interface, peer mode) need room too.
14. The specific checks for any candidate form are the 15 items at the end of the findings.

## Findings
SCOPE AND METHOD
I only read the repository and changed nothing. I read these in full:
- spec/README.md and spec/definitions.md.
- Every language-layer promise in spec/promises/*.yaml. For providers.yaml and secrets.yaml I read the language-layer promises and the convention promises that reject documents (PRV-002, PRV-013, PRV-020, SEC-008, SEC-024).
- The language-layer promises held elsewhere: IMP-003, LIF-002, LIF-007, IFC-014, IFC-043 and IFC-044. I also read ACC-005, ACC-006, ACC-022, ACC-029, ACC-030, ACC-033, LIF-008, LIF-009, LIF-031, LIF-037, IFC-012 and IFC-033.
- spec/schemas/templates.yaml, instances.yaml, envelope.yaml, contracts.yaml, the start of manifest.yaml, and README.md's "What a schema cannot express".
- spec/core/templates.yaml and requirements.yaml, the export table in core/README.md, and the inputs of the core operations.

For S1, I read:
- the coordinator's run files (docs/proposals/first-build/increment-4/coordinator-run/*);
- the fresh author's run, counted by its keys;
- S1's shipped packages (conformance/fixtures/s1/packages/*, packages/agent-*/conduit.yaml and contracts.yaml);
- the S1-touched promise set in conformance/manifest.yaml `s1_coverage`, which comes from section 2 of https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md.

Promise files, by ID prefix:
- spec/promises/: OBJ objects, INS instantiation, TPL templates, CTR contracts, NAM naming, PKG packages, MAN manifest, SUB submissions, POOL pools, PLC placement, CON conduits, FST first-start, PRV providers, SEC secrets, SCH schemas.
- spec/conventions/: ACC, LIF, REV, IMP, OPS, OBS, REC.
- spec/interface/implementation.yaml: IFC.

Legend:
- **S1 use.**
  - W: an S1 author wrote it.
  - P: only S1's shipped packages use it.
  - X: the engine exercises it on S1's path, but no author writes it.
  - N: S1 does not use it.
- **Class.**
  - ESS: essential to what the documents mean.
  - NOT: incidental notation.
  - REU: reuse machinery.
  - CHG: change machinery.
  - "a/b" means primary a, secondary b.
- **Check** (on each entry): what any short form must still let an author state, or must define by a default written in spec/.

======================================================================
0. NUMBERS
- There are exactly 203 language-layer promises:
  - 197 in spec/promises/*.yaml;
  - 6 elsewhere: IMP-003, LIF-002, LIF-007, IFC-014, IFC-043, IFC-044.
- Per file: objects 22, schemas 21, instantiation 22, naming 24, manifest 17, packages 16, conduits 13, placement 11, contracts 10, submissions 10, pools 9, templates 8, secrets 7, providers 4, first-start 3.
- **167 of the 203 are in S1's touched set. 36 are not**, and they are exactly the constructs S2 to S10 need:
  - requirements: CON-006, CON-018, CON-028, CTR-005, NAM-019;
  - controls: CTR-006, TPL-010;
  - `remote` items and identity binding: IFC-043, IFC-044, PKG-030, MAN-018;
  - placement and selectors: PLC-005, PLC-006, PLC-007, PLC-013, PLC-018, PLC-025, PLC-028;
  - Pool entries: POOL-002, POOL-005, POOL-006, POOL-009, POOL-011, POOL-012, POOL-014;
  - providers: PRV-001, PRV-004, PRV-016;
  - secrets: SEC-001, SEC-002, SEC-003, SEC-005, SEC-009, SEC-021, SEC-022;
  - LIF-007, the one outlier: S1 retires instances, but the increment-2 list leaves it out.

  A short form tuned on S1 alone would lose these 36 without anyone noticing.
- My class tally of the 203 (a judgment; per-entry classes are below):

  | Class | Promises |
  | --- | --- |
  | ESS | about 118 |
  | REU | about 48 |
  | NOT | about 30 |
  | CHG | about 7 |

  - Per file:

    | File | ESS | NOT | REU | CHG |
    | --- | --- | --- | --- | --- |
    | objects | 9 | 7 | 5 | 1 |
    | instantiation | 5 | 1 | 16 | 0 |
    | templates | 6 | 1 | 0 | 1 |
    | contracts | 9 | 0 | 0 | 1 |
    | naming | 11 | 6 | 4 | 3 |
    | packages | 9 | 0 | 7 | 0 |
    | manifest | 5 | 6 | 6 | 0 |
    | submissions | 5 | 3 | 1 | 1 |
    | pools | 2 | 0 | 7 | 0 |
    | schemas | 15 | 6 | 0 | 0 |

    The rest are all ESS: placement 11, conduits 11 (plus REU 2), first-start 3, providers 4, secrets 7, and the other six 6.
  - Most change machinery is in the convention layer, not the language layer: SUB-009, INS-020, LIF-008, LIF-009, LIF-012, LIF-017, LIF-021, LIF-029, LIF-035, LIF-037, REV-001 to REV-015, TPL-011, TPL-013, POOL-003 and POOL-017.
- Where S1's lines went (docs/proposals/simplification/README.md:23-27, coordinator's 332 lines), with my classes:
  - 100 re-declaration lines, from no in-place change, pins per object and standalone names (CHG);
  - 80 frame lines (NOT), or 190 of 349 by map-constraints' count, which includes kind, metadata and name;
  - 72 lines of root's Conduits to its Pool and for turns (ESS under G2, but derivable by a rule);
  - 21 manifest and digest lines (an ESS pin, with an NOT location);
  - 17 lines of template plumbing (REU);
  - 23 lines of access facts (ESS).

======================================================================
A. DOCUMENTS, PROJECT AND SUBMISSION

**A1. Strict syntax.**
- What it does:
  - YAML, or the equivalent JSON (SCH-013).
  - Rejected: duplicate keys, anchors, aliases, merge keys, tags and keys that are not strings (SCH-012); `.inf`, `.nan`, hexadecimal and octal (SCH-017).
  - Numbers: an integer is integer however it is written (SCH-005); whole numbers within ±(2^53−1) (SCH-006); fractions read as binary64 (SCH-020), and out of range rejected (SCH-021).
  - A JSON file holds one document or an array of documents (SCH-018).
  - Files are UTF-8 with one optional BOM (SUB-005).
  - Limits are stated by the engine (SUB-011, DL10).
- Protects: one reading on every engine (G6), with no coercion (SCH-011).
- S1: W. Class: NOT/ESS.
- Check: a new syntax must restate every one of these rules.

**A2. `apiVersion` on every document and on the manifest.**
- Promises: OBJ-017, OBJ-024, MAN-003; the value comes from spec/names.yaml.
- Protects: which version's names apply, and loading two versions side by side; mixing one version's `apiVersion` with another version's core address is rejected.
- S1: W (42 lines). Class: NOT.
- What carries weight is only that the version is known for each unit. Ruling 35's wording may need amending to state it once per file (map-constraints row 35).

**A3. `kind`.**
- Ten kinds: the five families and the five contract kinds (OBJ-020). A family document without `from` is a template, and with `from` an instance (INS-001).
- S1: W. Class: ESS/NOT. The family is meaning; the key is notation.

**A4. `metadata`.**
- Keys (OBJ-018):
  - `name`: required in templates and contracts. An instance without one gets a generated name (OBJ-003, DL15).
  - `description` and `annotations`: no meaning to the language. Any other key is rejected.
- Caveat: IFC-033 already uses an Operation contract's `metadata.description` as the tool's description. Ruling 53 decision 3 (accepted) adds the target and its Type's description, so `description` gains a meaning on the wire.
- S1: W for names, P for descriptions. Class: ESS for the name, NOT for the rest.

**A5. The `spec` wrapper.**
- Templates and contracts put everything under `spec`; an instance has none (OBJ-012, CTR-001, INS-001).
- Class: NOT.

**A6. Closed keys, camelCase, and no part silently ignored.**
- Every key the spec does not define is rejected (OBJ-019). The open places are: inside `config`, `annotations` and `default` values; schema member names and patterns; inside `arguments`; the keys of named maps; label keys; the selection keys of `implementation`, `controls` and `tools` maps; inside server.json's `package` and `remote`; and binding keys.
- The engine never accepts a document and ignores part of it (spec/README.md "Conformance").
- S1: X. Class: ESS.
- Check: the closed key set and diagnostics at the author's own line (SUB-005 attribution, ruling 52).

**A7. Fields the engine owns.**
- `id`, `revision`, `status` and `packages` are rejected at the top level (OBJ-004).
- The resolved form `{kind,id}` is rejected in documents (OBJ-009) and in an actor's own requests (LIF-031).
- Protects: no forged state or IDs. S1: X. Class: ESS.

**A8. Names.**
- Grammar: `[a-z](?:[a-z0-9-])*`, at most 63 characters (OBJ-001).
- Unique per kind, templates and instances together (OBJ-002).
- Unique across kinds within a project or package, implementations included, because `#name` carries no kind (NAM-027).
- Member names of `inputs`, `conduits`, `entries` and `requirements` follow the same grammar, written as maps (OBJ-022).
- Names are fixed and never renamed (OBJ-011). A name is freed only on removal (LIF-035) and never redirects a stored reference (OBJ-010). Generated names: OBJ-003.
- The 63-character limit keeps tool names at 63+1+63 ≤ 128 (IFC-014).
- S1: W. Class: ESS/NOT.
- Check: derived names must still be OBJ-001-valid and unique per kind.

**A9. Project, submission and directory.**
- A project is documents plus at most one manifest (MAN-001, MAN-002).
- A directory: only top-level `.yaml`, `.yml` and `.json` files whose names do not begin with a dot; nothing below is read; links are followed; an entry that cannot be read is rejected, never skipped (MAN-011, SUB-010, DL7).
- Submission rules:
  - a set, whose order has no meaning (SUB-001);
  - references inside the submission resolve (SUB-002);
  - all or nothing (SUB-003);
  - two documents of one kind and name are rejected (SUB-004);
  - only templates, instances, contracts and the manifest are accepted, with schema rejections cited by JSON Pointer and `keywordLocation` (SUB-005);
  - registering only registers (SUB-006);
  - it never replaces (SUB-007, OBJ-002);
  - a rejection reports invalid (OBJ-023), and a name clash conflict (OBJ-025).
- Canonical documents are also the wire payload of management:
  - `register.v1` takes `documents` (`document.v1`) and `manifest` (spec/core/operations.yaml:221);
  - `replace.v1` takes `template`, `revision`, `contracts` and `manifest` (:279);
  - `inspect.v1` returns `declared` in the resolved canonical form (:361).

  An Agent that manages objects through a Conduit sends these values (preservation test 5, docs/AUDIT.md:486).
- Conformance vectors are files, which is why every engine takes a directory (SUB-010).
- Protects: G6 and G8, and atomicity.
- S1: W. Class: ESS/NOT/CHG. SUB-007 is the "no apply" rule of rulings 21 and 22.
- Check: a normative short form must define:
  - its file or directory rule;
  - the form `register.v1` and `replace.v1` take, or keep the canonical form as wire and storage beside it;
  - what `inspect.v1` shows.

======================================================================
B. REFERENCES, IDs AND RESOLUTION

**B1. IDs and the resolved form.**
- At instantiation every reference resolves to `{kind,id}` and is stored (OBJ-009). Stored references are never resolved again or redirected (OBJ-010).
- A request names objects by reference, never by ID (LIF-031).
- `expectedId` is optional on `set-up.v1`, `retire.v1`, `inspect.v1` and `unregister.v1` (LIF-037). Ruling 53 adds it to `open.v1` and `close.v1`, not yet in spec.
- Protects: G7, and keeps a reused name from undoing access.
- S1: X. T6 hit name reuse after a removal: mistake 3 at coordinator-run/log.txt:39.
- Class: ESS.

**B2. Reference forms.**
- `{kind, name}`, or the bare name where the field's schema fixes the kind (OBJ-006). An argument given as a bare name is written out with its kind (INS-006). `consumer` and `target` always state the kind (OBJ-013).
- A reference to a template where an instance is expected, or the reverse, is rejected (OBJ-007).
- A template's unresolved reference gives a diagnostic that does not reject (OBJ-008); an instance's rejects (OBJ-009).
- S1: W, for example `{kind: Resource, name: shared}`. Class: NOT/REU.

**B3. Scoping.**
- A local reference (no `#` and no `/`) resolves only among objects that exist and is never looked up in a package (NAM-003).
- A qualified reference is `address#export` (NAM-002): no version (NAM-005) and no alias (ruling 25; ruling 53 #8 keeps full qualification).
- `#name` stays inside its project or package (NAM-022), except `from` in a submitted project (INS-025).
- A selection that names nothing is rejected (NAM-028). A document never uses a qualified reference to its own project (NAM-022, NAM-007).
- A selection (type, capability, operation, requirement, implementation) is distinct from `from`, which names a template (NAM-018, INS-025).
- S1: W (45 address lines in the coordinator's files); P (`#name` inside packages).
- Class: ESS for scoping, NOT for spelling.
- Check: every short name still resolves to exactly one export at one pinned version.

**B4. Reference schemas decide what is a reference.**
- OBJ-021, with the 12 schemas of core/references.yaml, through any number of `$ref`s. Any other value is data and is never rewritten.
- S1: X (owner, consumer and target). Class: ESS.

**B5. Pool-entry references.**
- From outside the Pool's template, `{kind: Pool, name, entry}` reaches that entry's Resource that is not retired (POOL-014).
- Inside a Pool template's entries only, `{entry: name}` (POOL-005, POOL-006). It creates a dependency, and a cycle is rejected (POOL-011).
- S1: N. Class: REU/ESS.

======================================================================
C. FAMILIES, STANDARD ARGUMENTS AND OWNERSHIP

**C1. The five families.**

| Family | What the promises fix |
| --- | --- |
| Interface | An actor. No placement or runtime; its implementations are only the engine or `remote` (TPL-019). |
| Agent | An actor. Exactly one placement; may name a runtime. |
| Resource | Exactly one Pool owner and one placement; may name a runtime; acts only to serve an invocation (IFC-021). |
| Pool | Owns Resources. Never a consumer or caller, no placement, offers `pool-management.v1` (CTR-004, ACC-028). |
| Conduit | One consumer, one target and one capability; no type or placement (CON-001). |

- The families are listed in OBJ-013 and OBJ-020; PLC-006 gives Interfaces, Pools and Conduits no placement.
- S1: W for Pool, Resource, Agent and Conduit; X for Interface (`root` only). Class: ESS.

**C2. Standard arguments.**
- Top-level keys of an instance document, accepted whatever the template's inputs (OBJ-013):
  - `owner` (Resource, required);
  - `placement` and `runtime` (Agent and Resource);
  - `consumer` and `target` (Conduit, required).
- Written under `arguments`, given to the wrong family, or with an input's value at the top level: rejected.
- Templates give none of them: PLC-005 for placement and runtime, CON-020 for consumer and target. An entry gets placement and runtime only from Pool inputs (POOL-009).
- S1: W for owner, consumer and target; N for placement and runtime. Class: ESS.

**C3. Ownership.**
- Exactly one owner Pool, fixed (POOL-001). A standalone Resource joins an existing Pool that is not retired (POOL-004). A retired Pool gains nothing (POOL-004, POOL-017).
- Ownership permits nothing (ACC-028).
- Routing (ACC-033):
  - registering into a Pool needs Conduits to both `management` and the Pool that list `register.v1`;
  - setting up, retiring and inspecting a Resource goes through its Pool.
- So the operator needs Conduits to its own Pool. This was S1's mistakes 1 and 2 (log.txt:4 and :12), costing 72 lines, 22%.
- S1: W. Class: ESS (G2, ruling 29).
- Check: any default must be a real Conduit that can be inspected and recorded (G2).

**C4. Authored Interfaces.**
- `interface.v1` is the basic type. An Interface may be a consumer or a target, and is never the consumer of `get-secret.v1` (SEC-009).
- Pending, not yet promises: rulings 49 and 50 put sign-in prompts and pending actions at the root Interface.
- S1: X (`root` is the consumer of the turn and Pool Conduits); N for authored Interfaces. Class: ESS.

======================================================================
D. TEMPLATES AND INSTANTIATION (reuse machinery)

**D1. Template and instance.**
- `from` marks an instance (INS-001), and every instance has a template (INS-002; ruling 8 IN9, ruling 18 V2).
- `from` has four forms: bare name, `{kind,name}`, qualified (a package-exported template) and `#name` (INS-025).
- A template has no lifecycle (LIF-012), and can be replaced or unregistered while its instances live on (INS-020).
- Packages may export templates (INS-025, NAM-022, MAN-007), but S1's export none.
- S1: W. The coordinator ends with 16 templates and 19 instances (map-constraints §0); the fresh author writes 29 `from`.
- Class: REU.

**D2. Template `spec` keys by family.**
- TPL-001; spec/schemas/templates.yaml.
  - Interface, Agent and Resource: `type`, `inputs`, `config`, `implementation`, `controls`, `conduits`.
  - Pool: the same with `entries` and without `conduits`.
  - Conduit: `inputs`, `capability`, `operations`, `requirements` (CON-016).
- Rejected: `runtimes`, `setup`, `retire`.
- Class: NOT for the shape, ESS for the contents.

**D3. `type`.**
- Selects a Type contract of the same family; a Conduit has none (TPL-002).
- S1: W. Class: ESS.

**D4. `config`.**
- Checked against the Type's schema after filling (CTR-004, INS-014). Leaving it out means `{}` (INS-024).
- Schema defaults fill maps that are present, before validation and comparison (SCH-009, SCH-010).
- A relative path is never joined to anything (INS-018). A value tied to a node is interpreted only on that node (PLC-020).
- S1: W, for example `{accountServices:false, peerMessaging:false}`, with `model` filled by its default.
- Class: ESS.

**D5. Inputs and destinations.**
- An input (INS-021):
  - `schema`;
  - `to`: one or more JSON Pointers rooted at `spec`, with no `-` token (OBJ-016);
  - `default`: a literal that satisfies the schema (SCH-019);
  - `optional` (OBJ-015).
- The closed destination list (INS-012):
  - `config` or a place inside it;
  - a declared Conduit's `target`, or its `arguments` or a place inside them;
  - an entry's `placement`, `runtime`, or `arguments` or a place inside them;
  - a Conduit template's whole `operations`, and an applied requirement's `arguments`.

  Never `type`, `implementation`, `controls`, `capability`, `from`, `optional`, a whole nested instantiation, one item of `operations`, the requirement's selection or `inputs`.
- Destinations must not overlap (INS-010), and each needs its parent (INS-011). One input may have several destinations, as with `token` in spec/schemas/examples/valid/project/templates.yaml (to config and a Conduit's target).
- Argument rules:
  - an undeclared argument, a missing one, or one that fails its schema is rejected (INS-003, INS-004, INS-005);
  - the whole value is assigned (INS-006);
  - an absent input takes its default (INS-007), and an optional one with none is left unset (INS-008);
  - null is a value (INS-009);
  - precedence: argument, then the input's default, then the template's value, then the Type's default (INS-019);
  - `operations` filled by an input is checked against the capability (INS-023).
- Ruling 8 IN1 to IN7: a closed list; structure is never an input; no interpolation, expression or merge (IN5); the instance stores only the result (IN6).
- S1: W (root → `/config/root`; the fresh author wrote 2 inputs). Class: REU.
- Check: if templates go, IN5 and IN6 still hold.

**D6. `arguments`.**
- Written in instance documents, nested instantiations and applied requirements. Left out means present and empty (OBJ-013, OBJ-014, NAM-019, INS-011).
- S1: W. Class: REU/ESS.

**D7. What instantiating does.**
- The complete resolved definition is stored, with no layer of arguments (INS-013).
- Only the instance and its nested instances are made; nothing is set up (INS-016).
- Deterministic (INS-017).
- The new instance is registered (LIF-002) and exists until removed (LIF-007).
- It keeps what it resolved, its package pins among them (INS-020, PKG-005, NAM-010).
- S1: X. Class: ESS.

**D8. Declared Conduits (the `conduits` map).**
- Shape: `from`, `target`, `arguments`, `optional`. No `consumer` (CON-020), and no `kind`, `metadata` or `spec` (OBJ-014).
- One Conduit is made with each instance, which is its consumer (CON-002), under a generated name (OBJ-003).
- The map key is the local name, so tool names are `<key>.<export>` (IFC-012, IFC-014).
- Opening it is the first part of setup (CON-004, TPL-012), authorized by the Conduit to `access` of whoever sent the setup (ACC-005). An optional one does not fail the setup.
- Once closed, it is replaced only by `register-replacement.v1`, which keeps its local name (LIF-008, LIF-009, LIF-029).
- A Pool cannot declare Conduits (TPL-001).
- S1: W for the coordinator (`project`); N for the fresh author. Class: REU/ESS/CHG.
- Check: if templates are dropped, the stable local name and setup's opening order must move somewhere.

**D9. Pool entries (the `entries` map).**
- Shape: `from`, `arguments`, `optional` (OBJ-014, POOL-012). The owner is the Pool. No `owner`, `placement` or `runtime` key: those come only from Pool inputs that state no default (POOL-009, PLC-005).
- One Resource per entry, and no Pool unless every Resource is made (POOL-002).
- Dependencies through entry references; cycles rejected (POOL-005, POOL-006, POOL-011).
- Setup runs in dependency order, as parts authorized by the sender's Conduit to the Pool (POOL-013, POOL-015).
- An entry with no placement input is placed by the machine provider (POOL-016).
- An entry has one Resource that is not retired, replaced only by request (POOL-003, POOL-017, LIF-009).
- S1: N; all seven language promises are outside S1's set. Class: REU/ESS. S4 needs it, and probably S2 and S3.

**D10. Selecting implementations.**
- The `implementation` map: the key selects an operation that the core `engine` does not implement; the value selects an implementation (TPL-003).
- Otherwise the Type's default, or the engine for core operations, applies; with neither, the template is rejected (TPL-004).
- The item must say it implements the operation (TPL-005).
- An Agent or Resource has at most one item that is run, and an Interface or Pool none (TPL-019).
- Hooks are selected here: `setup-hook.v1` and `retire-hook.v1` (TPL-017, TPL-018). A default for a hook the Type does not require is unused, with a diagnostic that does not reject (TPL-004, CTR-004).
- S1: X/P. Every operation takes the Type's default. The history Type's defaults point into another package (NAM-014).
- Class: ESS/REU. S4 needs the hooks.

**D11. A template's `controls` map.**
- Keys select controls the Type lists, and each value may narrow inputs (TPL-010). Without the map, every control is available. A control left out is unavailable: invalid (TPL-013).
- Narrowing is enforced (TPL-011) and shown in the tool as `allOf` (IFC-033).
- S1: N; S1's Types have no controls (critic point 5). Class: CHG/REU. S5 needs it.

======================================================================
E. CONDUITS (connection and access, what Conduit itself invents)

**E1. One consumer, one target and one capability.**
- CON-001: ruling 41 removed composites, and round 6 says keep it so.
- The target's Type must offer the capability (CON-021). `.v1` and `.v2` are separate contracts (NAM-015, NAM-016), and package versions are never compared (NAM-017).
- S1: W. Class: ESS.

**E2. Consumer and target kinds.**
- Consumer: Interface, Agent or Resource. Target: those, or a Pool (OBJ-013, CON-027). Neither may be retired (CON-025).
- Targets with special meaning:
  - `management` (`object-management.v1`);
  - `access` (`access-control.v1`, the right to open);
  - a Pool (`pool-management.v1`);
  - a secret Resource (`get-secret.v1`: only root opens it, and the consumer is never an Interface, SEC-008 and SEC-009);
  - an Agent (`turn.v1`, `startsModelWork`, IMP-001 and IMP-003);
  - an Interface.
- Forbidden: the provider capabilities' operations (PRV-004); nodes, which offer nothing (PLC-005, PLC-008).
- S1: W for Agent→Resource, root→Agent turns and root→Pool. N for `management`, `access`, secrets, Agent→Agent and Agent→Pool.
- Critic point 6 (research/critic.md:57-60): the layered design's file could grant only to Resources.
- Class: ESS.

**E3. `operations`.**
- An explicit list of selections from the capability; it may be empty (CON-017), and an input may fill it (INS-023, CON-016).
- Fully qualified (NAM-002; ruling 25; ruling 53 #8).
- A permission is exactly the operations listed (ACC-003). Under G1 a Conduit never grants "the whole capability".
- Reserved operations are for root alone (ACC-032, CTR-003, CTR-009).
- A listed control the target lacks: no tool, and the request is invalid (TPL-013, IFC-012). An operation missing from the target's package version: no tool (NAM-026, IFC-033).
- S1: W (the 23 lines of access facts). Class: ESS.

**E4. `requirements`.**
- A map from name to `{requirement, arguments}`, with arguments checked like instantiation and defaults filled (CON-018, NAM-019, SCH-009).
- The core's two, `bound.v1` and `shared-host.v1`, are applied by the engine. Every other is a filter:
  - each implementation the target selected for a listed operation must `supports` it (CON-022);
  - it is sent and applied per call (IFC-018, IFC-019), carried onward (ACC-017, ACC-018), and limits only its own Conduit (ACC-009);
  - several apply side by side, never composed (CTR-005);
  - inspection reports it mediated (ACC-027) and names the implementation that applies it (ACC-038).
- S1: N. Class: ESS. S2 and S8 need it.

**E5. `bound.v1` (delegation).**
- Only on a Conduit to `access`, and at most once (CON-028).
- Its arguments (core/requirements.yaml):
  - `pools` and `targets`: reference lists;
  - `operations`: qualified names, compared as strings and never resolved (ACC-029).
- A Pool covers its present and future Resources. No bound means unlimited, as root's is (ACC-022). "Within" is ACC-030.
- A bound does not narrow filters, and `management` cannot be narrowed; the broker does that (spec/README.md:337-372).
- S1: N. Class: ESS. S2 (E07) and preservation test 5 need it.

**E6. `shared-host.v1`.**
- The ends' hostnames must be equal, decided once (PLC-013). An end with no placement, or one that runs nothing, is rejected (CON-006).
- S1: N. Class: ESS. S3 needs it.

**E7. Standalone Conduit.**
- An instance document with `from`, `consumer`, `target` and `arguments` (CON-012). The consumer need not declare it.
- Its local name is its own name (IFC-012), and a clash of tool names is rejected (IFC-014).
- Ruling 53 accepts `as`, a local name, not yet in spec.
- S1: W, the bulk of S1: turns, root→Pool, history and grants. T5 to T7 re-declared about 100 lines.
- Class: ESS/CHG.

**E8. Conduit template.**
- No consumer or target (CON-020). Provider operations are rejected (PRV-004), and so is a second bound (CON-028).
- S1: W. Class: REU/ESS.

**E9. What the documents fix about tools.**
- Name: local name + "." + export name (IFC-012), at most 128 characters and unique per consumer (IFC-014).
- Definition (IFC-033):
  - taken from the contract as the target holds it;
  - a control's narrowing appears as `allOf`;
  - `readOnlyHint` is set where the operation states no `effect`, is not a control and starts no model work;
  - the description comes from the contract.
- S1: X; tool names changed at T5 and T6. Class: ESS.

======================================================================
F. PLACEMENT AND RUNTIME

**F1. Placement.**
- One node, resolved and fixed (PLC-001, PLC-009). None for an Interface, Pool or Conduit (PLC-006), and templates give none (PLC-005).
- By default an instance is placed on `local` where nothing of it runs, and otherwise by the machine provider (PLC-002, PRV-015).
- Never a retired node (PLC-018). Only the engine makes nodes (PLC-008, PLC-026, PLC-028, FST-006). An instance depends on its node through placement alone (PLC-015).
- S1: X; everything is placed on `local` by default. Class: ESS. S3 needs it.

**F2. Label selectors.**
- `nodeSelector` (`labels.v1`) in a Type and in implementation items. They combine with `local`'s hostname where no runtime is named and an item runs (PLC-007). Conflicting values are rejected (PLC-025).
- S1: N. Class: ESS. S3 and S7 need it.

**F3. `runtime`.**
- A Resource whose Type offers `runtime-provider.v1`. It is fixed, and a retired one is rejected; with none given, the engine's own runner runs the instance (PRV-006).
- Runtime, machine and package-source providers are placed on `local` with no runtime (PRV-002). The machine and package-source providers are owned by `system`, at most one machine provider and one per scheme (PRV-013).
- A runtime provider is not retired while an instance names it (PRV-020).
- S1: N. Class: ESS. S3 and S10 need it.

======================================================================
G. CONTRACTS (written by package and provider writers; S1 projects write none)

**G1. Contract frame.**
- Five kinds. The name is an export name ending in `.vN` (CTR-001, NAM-004).
- A later release may only add (NAM-024); a breaking one takes a new address (NAM-025). `.v1` and `.v2` are separate contracts (NAM-015).
- S1: P. Class: ESS.

**G2. Type.**
- Keys (CTR-004): `family` (required), `schema`, `offers`, `requires` (hooks included), `implementation` (defaults), `controls`, `conditions`, `records` (`inputs` and `outputs`, for REC-002) and `nodeSelector`.
- Rules for special Types:
  - a Pool Type offers `pool-management.v1`;
  - a provider Type is a Resource offering exactly one provider capability (PRV-001);
  - a secret Type offers only `secret-provider.v1` and lists no control (SEC-022, SEC-005);
  - a package-source Type requires `schemes`, which no control may change (PRV-016).
- S1: P (`schema`, `offers`, `requires`, `implementation`, `records`).
- Ruling 48, and ruling 53 #7 (accepted): Agent Types' reach flags default to false, as new package versions.
- Class: ESS.

**G3. Capability.**
- An `operations` list (CTR-003), with no two operations of one export name (CTR-007). Reserved operations appear only in core capabilities.
- S1: P. Class: ESS.

**G4. Operation.**
- `inputSchema` and `outputSchema`, as MCP tool schemas with `type: object` at the root (CTR-002, CTR-012). A control's root holds only four keywords.
- Input defaults are filled (OPS-026); no output states a default; a handle is a property of `handle.v1` (IFC-011).
- Marks and keys:
  - `effect`: drives `readOnlyHint`, how a plain server's error maps (IFC-010) and whether a repeat is dispatched again (OPS-024);
  - `startsModelWork` (IMP-003, IMP-001);
  - `deduplicates` (OPS-012);
  - `reservedToRoot` (core only, CTR-009);
  - `changes` (controls);
  - the hints `idempotentHint`, `destructiveHint` and `openWorldHint`.
- No default implementation (CTR-002).
- S1: P (input and output schemas, `effect`, `startsModelWork`). Class: ESS.

**G5. Control.**
- An Operation with `changes`: an input name → a pointer into `config`. `revision` is required, there is no `settings` output, and no two pointers overlap (CTR-006).
- It applies exactly what it is sent or fails (REV-007). It is invalid where the configuration would fail the schema (REV-015). Only `revision` is required (REV-006).
- S1: N. Class: CHG. S5 needs it.

**G6. Requirement.**
- Only `inputs` (`schema`, `default`, `optional`) (CTR-005).
- S1: N. Class: ESS.

**G7. Schema contracts and the profile.**
- A Schema contract's whole `spec` is a schema (CTR-008).
- Rules:
  - the 2020-12 profile, its keywords and formats (SCH-001 to SCH-004, spec/schema-profile.md);
  - `$ref` takes three forms (SCH-015);
  - where a `default` may stand and when it is filled (SCH-008, SCH-009, SCH-010), and it must be valid (SCH-019);
  - no coercion (SCH-011); JSON Schema equality (SCH-007);
  - ECMA-262 interoperable patterns (SCH-016);
  - existing MCP servers' own schemas are not held to the profile (SCH-014);
  - the root `$defs` keys reserved by IFC-033.
- S1: P/W, for input schemas such as `{type: string, minLength: 1}`. Class: ESS, an adopted standard.

======================================================================
H. MANIFEST AND PACKAGES

**H1. The manifest file.**
- `conduit.yaml`, `conduit.yml` or `conduit.json`, one per project, with `apiVersion` and no `kind`, `metadata` or `spec`. Its keys are `packages`, `exports`, `implementations`, `address` and `version`, with `version` only beside `address` (MAN-001 to MAN-004).
- S1: W (7 manifests). Class: NOT.

**H2. Package pins.**
- The `packages` map, keyed by address (NAM-006, MAN-005, PKG-001, PKG-003, PKG-004, PKG-021):
  - `version`: exact SemVer;
  - `digest`: `sha256:` and the hash;
  - `fetch`: optional; a URI reference with no user information; relative only beside a directory (MAN-006, NAM-012).
- Before anything is resolved:
  - every listed package must already be installed, or the request is rejected (PKG-006);
  - an unused one gives a diagnostic that does not reject (NAM-008);
  - an address used but not listed is rejected (NAM-007);
  - the core is never listed (NAM-020, NAM-021).
- Afterwards:
  - a template keeps its pins (NAM-009), and its instances inherit them (PKG-005);
  - templates and instances resolve through what they hold (NAM-010);
  - versions are installed side by side (NAM-011);
  - a package resolves through its own manifest (NAM-014).
- S1: W (21 lines; T5's upgrade).
- Class: ESS for which contracts govern, NOT for where the pins are written. Ruling 53 #5 has `install` write them.

**H3. `exports`.**
- The names of the package's own documents (MAN-007, PKG-018). A package holds no instance document (MAN-011).
- S1: P. Class: REU.

**H4. Implementation items.**
- How an item is reached (MAN-009, MAN-010, PKG-019): `implements`, plus exactly one of:
  - `command`: a program and its arguments; a relative path runs only on the engine's own runner;
  - `package`: one server.json Package (2025-12-11), over `stdio` or `streamable-http`, pinned by `fileSha256` or an `oci` digest;
  - `remote`: a `streamable-http` Remote whose URL is written whole.
- Its other keys: `plain` with `tools`, which rules out `supports` and provider operations (PKG-020); `supports`; `nodeSelector`.
- Bindings: `environment`, `packageArguments` and `headers` point into `config`.
  - The place must be required and hold a string, number, boolean or null (MAN-017).
  - An input marked `isRequired` is never bound to a place that admits null (MAN-017).
  - An input marked `isSecret` is bound only to a secret reference (SEC-021).
  - `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` are never bound (MAN-018).
- `remote` rules:
  - an item that is not plain stands at an `https` URL (IFC-044) and names no `Authorization` header (IFC-043);
  - no `remote` item implements a provider operation (PKG-030);
  - SEC-024 is a convention.
- The core's `engine` item is reached by none of the three (MAN-012).
- S1: P. The npm stdio Package binds `packageArguments` root `/root`, plain with `tools`; the `command` item uses `environment`, plain with `tools`.
- Class: ESS. S2 and S7 need it.

**H5. A submitted project's contracts.**
- `address` is required where the project holds a contract or lists an implementation (MAN-015), and may not be an installed package's (MAN-016).
- Inside, its contracts are `#name`; outside, `address#name`.
- They are identified by SUB-008's digest: RFC 8785 over the contract documents, ordered by kind then name, plus the manifest as handed over.
- A template keeps the address and digest (NAM-009). No package may be installed at the address meanwhile (PKG-029).
- S1: N. Class: REU.
- Check: a normative form that changes how contracts are written must redefine this digest.

**H6. Archives and installation.**
- The archive: gzip tar, manifest at the root, no unsafe members, digest over the archive's bytes (PKG-022, MAN-013, MAN-014, PKG-026).
- The package: its manifest states its own address and version (PKG-017); its export names are unique (PKG-012).
- Installing: dependencies are installed first (PKG-028), and installing registers nothing (NAM-013). The requests are PKG-023 to PKG-027 and PKG-029.
- S1: P/X (`install` at T5). Class: REU (distribution).

======================================================================
I. SECRETS

**I1. Where a secret reference may stand.**
- `secret-reference.v1` governs a place only directly, as `items`, or beside `{type: null}`.
- The value is always a reference to a Resource, never a literal or a bare name (SEC-001), and that Resource's Type offers `secret-provider.v1` (SEC-002).
- Diagnostics never echo the value (SEC-003).
- S1: N. Class: ESS (G5).

**I2. Secret Resources and how they are read.**
- Locators are fixed (SEC-005, SEC-022).
- Pattern: the instance reads through its own Conduit to the secret Resource. Only root opens that Conduit (SEC-008, a convention), and an Interface never consumes it (SEC-009).
- Item bindings: SEC-021. The engine reads before a run or a connection (SEC-010, SEC-013).
- S1: N. Class: ESS. S2 and S10 need it.

======================================================================
J. PROVIDERS (language part)
- The language-layer promises are PRV-001, PRV-004, PRV-006 and PRV-016.
- Conventions that reject a document at registration: PRV-002, PRV-013 and PRV-020.
- The user picks a provider by registering a Resource of its Type.
- S1: N. The engine's own runner, machine provider and fetching serve (PRV-003).
- Class: ESS. S3 and S10 need it.

======================================================================
K. FIRST START AND THE CORE

**K1. First-start objects.**
- `system`, `local`, `management`, `access` and `root` (FST-001, FST-005).
- root's Conduits (core/templates.yaml):
  - `root-management`: every `object-management.v1` operation;
  - `root-access`: `open`, `close` and `reconcile`, with no bound (ACC-022).
- No instance document may use those templates (FST-006).
- The core: 5 Types, 7 Capabilities, 23 Operations, `bound.v1` and `shared-host.v1`, 12 reference schemas and 5 result schemas (spec/core/README.md:616+).
- S1: X; root is S1's operator. Class: ESS.
- Check: a short form needs a name for the operator. Core addresses stay fully qualified unless ruling 25 changes.

======================================================================
L. CHANGE MACHINERY (mostly the convention layer, tied to the documents)

**L1. Immutability.**
- OBJ-011, OBJ-002 and SUB-007 (rulings 21 and 22). Placement, runtime and owner never change (PLC-009, PRV-006, POOL-001).

**L2. Revisions.**
- Opaque; they advance only when something changes (REV-001 to REV-005). They are named in a change (REV-002, REV-011), and one change is applied at a time (REV-012, REV-014).

**L3. `replace.v1`.**
- One template, given with its project's contracts and manifest and checked against its revision. An equal one changes nothing. Old instances are frozen (SUB-009, INS-020).
- S1: W (T4).

**L4. `register-replacement.v1`.**
- Makes a new instance from the template as it stands now, keeping the local name (LIF-008, LIF-009, LIF-029, POOL-003, POOL-017).
- S1: W (T4).

**L5. Controls.**
- G5 and D11, plus REV-006, REV-007, REV-009, REV-010, REV-015, TPL-011 and TPL-013.
- S1: N.

**L6. Open, close, retire, unregister.**
- A closed Conduit is removed and its name freed (LIF-035), and is never reopened (LIF-006, ruling 29). Opening is idempotent (ACC-006).
- Unregister: LIF-012, LIF-017. `expectedId`: LIF-037. `retire-with-resources.v1`: LIF-021.
- S1: W (T4 to T7, mistake 3).

**L7. An upgrade means a new version and new objects.**
- NAM-011 and PKG-005: T5 cost 62 lines against 1 fact.

**L8. The parts of a setup, in order.**
- Declared Conduits or entries, then the run and `remote` connections, then the setup hook; some parts may be optional (TPL-012, CON-004, POOL-013, TPL-017, TPL-018).

======================================================================
M. NOT YET IN spec/, BUT ACCEPTED OR PENDING
- **Ruling 53, decision 3 (accepted):**
  - `as`;
  - `expectedId` on `open.v1` and `close.v1`;
  - an inventory in `inspect.v1` of the Conduits an object consumes and is the target of: only what the caller may see, with no counts;
  - tombstones enforced by the engine at open and at register, which change what registering means;
  - tool descriptions that name the target and its Type's description.
- **Ruling 53, other accepted points:** #5, pins written by `install`; #7, reach flags off by default; #8, fully qualified operation names.
- **Pending** (from map-constraints rows 49 to 51; I did not read their records):
  - ruling 49: the account each home or Resource is for, declared and checked, which needs a field in `config`;
  - ruling 50: pending actions at an Interface;
  - ruling 51: peer mode, and relationships across parties as two halves.

======================================================================
N. WHAT S2 TO S10 NEED (ROADMAP.md:134-142, 176-181)

| Scenario | Constructs it needs |
| --- | --- |
| S2 | Filters, Requirement contracts and `supports` (E4, G6, H4); a bound (E5, case E07); secrets (I1, I2); `remote` and headers (H4); onward requests |
| S3 | Placement, `nodeSelector`, `runtime`, machine provider and shared-host (F1 to F3, E6) |
| S4 | Setup parts and hooks (L8, D10); optional declared Conduits and entries (D8, D9); `deduplicates`; plain error mapping |
| S5 | Controls and template narrowing (G5, D11, L5); defaults (SCH-009, OPS-026) |
| S6 | Revocation and retirement in flight (CON-008, ACC-034, ACC-039) |
| S7 | Contracts, items, exports, `nodeSelector`, templates exported by packages (G, H3, H4, D1) |
| S8 | Filters on returned content (E4) |
| S9 | First start and repair (K1, L4, reconcile) |
| S10 | Provider Resources (J; PRV-006, PRV-013, PRV-016; SEC-005, SEC-022; NAM-012 schemes) |

======================================================================
THE CHECKLIST: what any short form must answer, condensed
1. **Kinds and names.** Does every object still get a family, a kind and a name valid under OBJ-001, OBJ-002 and OBJ-011? Is there a stated operator (root)?
2. **Closed keys and diagnostics.** Are the keys closed (OBJ-019), with diagnostics at the author's text (SUB-005, ruling 52)? Are the engine-owned fields and authored IDs still rejected (OBJ-004, OBJ-009, LIF-031)?
3. **Standard arguments.** `owner`, `placement`, `runtime`, `consumer`, `target`, each with its rules (OBJ-013, PLC-005, PRV-006).
4. **Every grant.**
   - Consumer: Interface, Agent or Resource.
   - Target: Interface, Agent, Resource or Pool, including `management`, `access`, a Pool, a secret Resource and an Agent.
   - Exactly one capability, and an explicit, fully qualified list of operations, which may be empty or filled by an input.
   - Requirements: filters with arguments, a bound with `pools`, `targets` and `operations`, and shared-host.
   - A stable local name (declared key, own name or `as`), and `optional`.
5. **Types and implementations.** `type` and `config` with defaults; implementation overrides and hooks; a subset of controls and their narrowing.
6. **Pools.** Entries with dependency references, `optional`, and placement or runtime through inputs; entry references from outside.
7. **Secrets.** Secret references only at governed places; the instance's own Conduit to a secret Resource; SEC-021 bindings.
8. **Packages.** Pins (exact version and digest per address) held per template and instance; templates exported by packages; a project's contracts with their address and SUB-008 digest.
9. **Contracts.** All five kinds and every key, for S7 and S10 writers.
10. **Reuse.** If templates are dropped or inlined, what replaces:
    - "the template as it stands now" in LIF-009;
    - INS-020's frozen instances;
    - templates exported by packages;
    - the order and optional parts of declared Conduits and entries?
11. **Change.** No in-place rewrite unless rulings 21 and 22 are amended; revision checks; `replace.v1`; `register-replacement.v1`; controls; `expectedId`; tombstones; names never redirected.
12. **Determinism.**
    - The same input gives the same objects (INS-017, SUB-001).
    - No interpolation (IN5), no joining of relative paths (INS-018) and no reading of the environment.
    - Nothing implicit (INS-016, SUB-006).
13. **The wire.** What `register.v1` and `replace.v1` take, and what `inspect.v1` returns, so that Agents can manage through Conduits (preservation test 5).
14. **Files.** A file and directory rule, so that conformance vectors stay files (SUB-010).
15. **The 36 promises outside S1's set** each have a counterpart.

## Unverified
- The class assignments are my judgment, with no rule behind them. Many constructs straddle classes; for example, declared Conduits are both reuse and essential.
- The tally (118/48/30/7) is approximate. Another reader could move about 10 to 20 promises, mostly between notation and essential and between reuse and essential.
- "S1 use" comes from four sources:
  - the coordinator's final files;
  - a key count of the fresh author's JSON (I did not read all 46 files);
  - S1's shipped packages;
  - the s1_coverage list, which comes from the increment-2 plan's inventory and is not a trace of the runs.

  LIF-007 is outside that list although S1 retires instances. The list is not a measured record of which promises the runs exercised.
- I read the language-layer promises in full. I did not read every convention or interface promise in full: providers, secrets, operations, records, observation and most of lifecycle, access and implementation.
- From core/operations.yaml I read only the inputs and outputs of nine core operations.
- From spec/schemas I did not read values.yaml, schema.yaml or document.yaml in full, nor all of manifest.yaml.
- I did not run any check: no `cargo test`, no `check_schemas.py`.
- What S2 to S10 need comes from ROADMAP.md's scenario table and round 6's evidence rows. No detailed scenario documents exist that I found.
- What rulings 49, 50 and 51 add (an account field, pending actions, peer mode) comes from map-constraints.md's summary; I did not read their records.
- Ruling 47 is reserved for pull request 14 and is unread. It might constrain the design.
- The line attributions of S1's costs are the simplification investigation's (README.md:23-27, map-constraints §0); I did not recount them.
- That ruling 53's tombstones change "registration's meaning" is my reading of "enforced by the engine at open and register". The design of tombstones is not written yet.