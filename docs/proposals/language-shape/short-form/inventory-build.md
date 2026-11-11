Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Where a normative short form would enter the reference model, and what it would cost

## Key points
1. Every submission passes through one place: crates/loader/src/project.rs load() (lines 95-121). The shell's register, submit and replace, the runner's register-project, resolve-project and load steps, and the core's own archive admission all go through it. The core only ever receives canonical JSON (core/src/packages.rs:16). Only register.v1 over MCP bypasses it, because it carries values.
2. A normative short form defined by desugaring belongs in that loader stage. Add a new module over Node, which keeps line and column positions, recognize the short form by a separately marked document validated against a new Schema contract in spec/schemas, and expand it before the canonical check. The core does not change.
3. Size of (a): about 700-1,200 lines of expander code, plus about 400-800 lines of glue (provenance and mapped-back diagnostics, shell render and replace by key, runner expand and equivalent modes), plus 500-900 lines of tests. Add a 150-300 line schema and 50-500 lines of Python in check_schemas.py. Spec: about 20-30 promises (12-17 new, 6-12 amended).
4. Under (a), 0 of the 367 vectors change, provided document.v1, envelope.v1 and manifest.v1 are untouched. That means a separate marked document, not sections inside conduit.yaml, which would also break SUB-008's manifest digest. New vectors: about 25-45. These include one exact-equivalence vector (S1 short form registers the same transition as a long-form S1 written with the derived names), modelled on registration-orders (conformance/src/lib.rs:598-631).
5. Rules that need package contracts cannot run in the loader: inferring the capability, and expanding turn Conduits from an Agent's Type. They would cost about 100-200 lines in registration.rs, reusing offered() at line 1241 and capability_includes. Recommended for v0: the author writes the capability and operations stay fully qualified (ruling 53, point 8). The expansion is then pure syntax.
6. For thesis 2: an expansion in the loader emits no core Decision. Each derived document must carry the promise ID of the rule that made it in its provenance, asserted by expand vectors. Otherwise the expansion has to become a core input.
7. Core diagnostics name /objects/<name> (registration.rs:50), not the author's line. A normative form needs a provenance map from derived names and pointers back to authored places, about 150-250 lines.
8. Frames-once-per-file and one-line grants (as long as a grant stays a real Conduit) can never be core changes. The core reads per-document kind and metadata.name everywhere. The only genuine model changes are inline definitions with no template object, and the removals.
9. Removing declared Conduits saves little code: they share the nested machinery with Pool entries, and the Conduit-specific setup branch is about 15 lines (lifecycle.rs:537-549). It still invalidates 126 of 367 vectors with regenerated expectations, because all 118 S1 cases shift IDs. It touches about 30-40 promises and 6 shell test files. It also moves TPL-012's guarantee (Conduits opened before the run starts) to clients.
10. register-replacement.v1 also serves Pool entries and D09's rotation by explicit replacement (S2, S10), so removing it fails the scope guard. replace.v1 is about 120 lines; removing it touches 15 vectors.
11. Removing declared Conduits, replace.v1 and register-replacement.v1 together invalidates 141 of 367 vectors (38%). Making the short form the only accepted form touches about 252 of 367 (69%), plus all 114 spec examples and check_schemas.py.
12. Counts of vectors that depend on document shapes: 280 have loader-consuming steps; 316 carry authored documents; 247 carry templates or instances; 118 share the 185-line S1 project fixture; 126 declare Conduits; 134 have standalone Conduits. Objects are compared by projection but arrays exactly, so an added field passes and a reordering or allocation shift fails.
13. Keeping both forms is inherent and cheap under (a): one core, plus equivalence vectors to stop drift. Under (b) the core carries two representations: about 600-1,200 more lines, 25-40 promises with none removed, and 40-70 more vectors.
14. Against the layered approach, the compiler code is the same in both routes. The normative route moves it into the loader and adds spec text, the Python checker, runner modes and 25-45 vectors. It needs decision 6 (derived defaults) settled first, because those defaults become part of the language.
15. Normative desugaring defines only what a file means when registered. Making change (T4-T7) normative as a (file, state) to plan function adds about 25-30 promises and about 50 plan vectors, per design E's cost verdict. That still involves no engine-side apply or restart.

## Findings
I read only. I built nothing, did not run cargo, and changed no file. I read reference-model/README.md, reference-model/CONFORMANCE.md, all four crates (loader, core, shell, conformance), conformance/README.md and conformance/manifest.yaml, and I classified all 367 case.yaml files with scratch scripts. All line references are into <home>

## 0. How documents reach the core today

There is a single point that every submission passes through.

- **The loader reads files into documents with positions.** `crates/loader/src/project.rs` `load()` (lines 95-121) does three things in order:
  1. `read_files` classifies each file by name (`FileRole`).
  2. Each file is parsed to `Node` trees. Every value has a line and column (`value.rs`, `Node{value, position}`).
  3. Each document is checked with `spec.check(document, Role::Document|Manifest)`.

  The result is `Project{directory, manifest, documents: Vec<Document>}`.
- **Schemas are bundled at run time from spec.** `schemas.rs` `Specification::from_documents` (lines 110-150) bundles every Schema contract found in `spec/schemas/` and `spec/core/`. It builds fixed validators for envelope, document, manifest, template, instance and contract. A schema rejection gets its place from `document.root.locate(pointer).position` (lines 208-235).
- **The boundary converts to JSON and drops positions.** The core's input is `conduit_core::packages::Project{manifest: Option<Value>, documents: Vec<Value>, directory}` (`core/src/packages.rs:16-21`). Every caller converts with `root.to_json()`:
  - the shell's `boundary::project` (`shell/src/boundary.rs:21-33`), used by `register` (`main.rs:265`), `submit` and `replace` (`author.rs:183-246`);
  - the runner's `packages::load` (`conformance/src/packages.rs:40-64`), used by register-project, resolve-project and registration-orders;
  - the runner's load and submit-files steps (`conformance/src/lib.rs:653-690`);
  - archive admission inside the core (`core/src/archives.rs:245`, `read_submitted_files`).
- **The core registers the JSON documents.** `registration.rs` `register()` (lines 1409-1458) and `prepare()` (lines 1461 onward) work as follows:
  - documents are sorted by `(kind, metadata.name, text)`, which fixes allocation order (lines 1470-1476);
  - SUB-004 and OBJ-002 are checked on (kind, name);
  - the catalog is resolved;
  - templates are documents without `from`; instances are expanded by `Builder.expand` (lines 271-582) through `template()`;
  - two nested slots are expanded by one loop (lines 566-579): `conduits`, which are declared Conduits, and `entries`, which are Pool entries.
- **Core diagnostics name objects, not authored places.** They are `/objects/<name>` (`registration.rs:50`).
- **register.v1 takes the same canonical documents.** `records.rs` `register_project` (lines 134-170) calls `prepare` with `arguments.documents`. Its inputSchema references `core#document.v1` and `manifest.v1` (`spec/core/operations.yaml:252-260`).

So `loader::project::load` is the only place every submission passes: the shell CLI, the runner and package archives. Only register.v1 over MCP bypasses it, because it carries values.

## 1. How many vectors depend on document shapes

All figures are case directories out of 367, from my scripts. The manifest has 367 cases covering 249 distinct promises; the spec has 455 promises.

| What the case does | Cases |
| --- | --- |
| Has at least one step that sends files through the loader (register-project, resolve-project, submit-files, load, registration-orders) | 280 |
| Submits authored documents of any kind (template, instance or contract) | 316 |
| Submits template documents | 240 |
| Submits instance documents | 211 |
| Submits template or instance documents | 247 |
| Uses the shared S1 project fixture (`conformance/fixtures/s1/project`: 3 files, 185 lines, 9 templates, 6 instances) | 118 |
| Submits declared Conduits (non-Pool templates with `spec.conduits`), including all 118 S1 cases | 126 |
| Submits standalone Conduit instances | 134 |
| Uses Pool entries | 6 |
| Calls register-replacement.v1 | 7 |
| Calls replace.v1 | 4 (10 calling either) |
| Mentions `#register-replacement.v1` anywhere | 85 |
| Mentions `#replace.v1` anywhere | 15 |

- **Step counts.** register-project appears in 213 steps; registration-orders in 2.
- **Why so many cases mention register-replacement.v1.** It is listed in the S1 `workspace-admin` template and in root-management's operation array, which the first-start vectors assert exactly.
- **How assertions behave.** Maps are compared by projection, so an added field passes. Arrays are compared exactly in cardinality and order: object lists, record lists and operation lists.
- **Why changing S1's structure is costly.** IDs come from evidence in sorted (kind, name) order. Any change to S1's templates or instances shifts the IDs. All 118 S1 cases then need regenerated expectations, not just edited inputs.

## (a) The short form defined by normative desugaring into today's documents

**Where the translation lives: a loader stage.**
- Put it in `project.rs::load`, between `read_files` and the canonical `spec.check`.
- New module `crates/loader/src/expand.rs`, built over `Node`.
- Recognize the short form by a positive marker in a separate document, for example `kind: Project`. Route it to a new `Role::ProjectFile`, validated against a new Schema contract (e.g. `project-file.v1`) in `spec/schemas/`. `from_documents` picks that contract up automatically; the change is about one line plus the Role variant.
- Expand into canonical `Document`s whose `Node`s carry the authored positions. Validate them exactly as today. Return a provenance map alongside: each derived (kind, name, pointer) maps to the authored file, document, pointer, position and the promise ID of the rule that made it.

**What then works with no change.** The core, `boundary::project`, `register`, `submit`, the runner's register-project, resolve-project and load steps, and archive admission all receive canonical documents unchanged.

**Two choices that carry cost.**
1. **register.v1.**
   - Keep it canonical-only: the operation is unchanged, and an Agent that registers through the operation sends canonical documents.
   - Or let it accept a `project` value: the core calls the same pure expander, since `conduit-core` already depends on `conduit-loader`. This costs about 20-40 lines in `records.rs` and `registration.rs`, a change to the inputSchema in `operations.yaml`, and a re-check of e05-core-register's samples.
2. **Rules that need package contracts must live in the core or be dropped for v0.** Examples are inferring the capability, and expanding the turn Conduit from an Agent's Type.
   - The loader has only the spec's own contracts.
   - The helpers exist in the core (`registration.rs:1241` `offered()`, `schemas.rs:919` `capability_includes`). But the target is resolved only after Conduit templates are built, so inference means patching the template after `resolve_instance`. That is medium work, about 100-200 lines in `registration.rs`.
   - Recommended: the author writes the capability (line width, not line count), and operations stay fully qualified (ruling 53, point 8). The expansion is then purely syntactic and the core does not change.

**Size of the code.**

| Part | Estimate |
| --- | --- |
| Expander | ~700-1,200 Rust lines: frame once; keyed sections to instance documents; `from` for reuse; an inline one-off definition becomes a template with a derived name plus an instance; a grant line becomes a standalone Conduit template plus an instance with `as`; names derived within the length grammar, with clashes checked over the expanded set; the default Pool and the operator's Conduit to it only if decision 6 makes them language |
| Loader integration (`project.rs`, `schemas.rs`, `rules.rs`) | ~80-150 |
| Provenance, and mapping core diagnostics (`/objects/<derived>`) back to the author's line, in the shell and the runner | ~150-250 |
| Shell (`render`; `replace` choosing by key; opening Agent-consumer standalone Conduits after `submit`, since setup opens only declared ones) | ~100-200 |
| Runner: new `expand` input kind (exact canonical documents plus provenance rule IDs) and an `equivalent` option on register-project | ~100-200, plus ~40-80 lines of `case.schema.json` |
| Tests | ~500-900 |
| `project-file.v1` schema | ~150-300 YAML lines; inline definitions `$ref` the existing template-spec schemas |
| Spec examples | +10-20 |
| `tools/check_schemas.py` (2,588 lines) | +50-100 Python to validate the schema only, or +300-500 to reimplement the expansion. The loader tests compare verdicts with it, so it must move in step |

The `equivalent` option copies the existing `registration-orders` exact-transition comparison (`conformance/src/lib.rs:598-631`). In total that is about 1,100-2,000 source lines and 0 lines in the core.

**Promises.**
- New: about 12-17 (form and marker; frame; sections; reuse with `from`; inline definition and derived template name; templates section; grant to standalone Conduit with `as`; resolving the grant target; name derivation and clashes; purity; meaning = expansion, all-or-nothing; diagnostics at the authored place; mixing with long-form documents; plus 2-3 more if derived defaults are accepted).
- Amended: about 6-12, among MAN-011, SUB-001, SUB-004, SUB-005, OBJ-003, OBJ-017, OBJ-018, INS-001, NAM-027, IFC-012 and register.v1's description.
- Total: about 20-30, against 50-80 for designs C and D. This covers only what a file means when registered.
- If changing a file (T4-T7) must also be normative, define plan as (file, state) to requests. Design E's cost verdict (§1C) puts that at about 25-30 more promises, about 50 plan vectors and 300-600 runner lines, bringing the total to about 45-60 with no engine-side apply or restart.

**Existing vectors that change: 0 of 367, under three conditions.**
1. The short form is a separate marked document, so `document.v1`, `envelope.v1` and `manifest.v1` do not change. The `loader-schema-attribution` vector pins keyword locations.
2. The short form is not placed inside `conduit.yaml`. If it were, manifest.v1 and SUB-008 would change: the digest covers "the manifest as it was handed over", so every grant edit would change a project's contract digest (`spec/promises/submissions.yaml:176-191`). The 14 MAN-011 vectors and manifest diagnostics would then be at risk.
3. The canonical form does not change.

No vector uses `kind: Project` today.

**New vectors: about 25-45.**
- Schema of the short form: 5-8
- Frame and apiVersion: 2
- Sections and `from`: 3-5
- Inline definitions with derived names and collisions, including a derived name clashing with an existing object under OBJ-002: 5-8
- Grants (consumer and target inside and outside the file, `as`, capability written out, requirements): 6-10
- SUB-001 order independence: 1-2
- SUB-008 digest unaffected by the file: 1-2
- Diagnostics mapped back: 3-5
- Short form in a package (templates only, instances rejected): 1-2
- Derived defaults, if accepted: 3-6
- Safety rows (derived Conduits visible, nothing opened implicitly): 2-4
- At least one equivalence vector: S1 in the short form registers to exactly the same transition as a long-form S1 written with the derived names. It cannot reuse the existing fixture: its authored template names (`reader-agent`, `directory-read`) drive generated nested names such as `history-post-read-a0000`.

For the equivalence to be exact, either keep provenance outside the documents, or compare modulo `metadata.annotations`. The core keeps metadata on objects (`registration.rs:540`).

**Thesis 2.** Expansion in the loader emits no core `Decision`. To keep "a promise for every decision" for what authors write, each derived document needs the citing rule ID in the provenance, asserted by the `expand` vectors. The alternative is to run the expansion as a pure core input that emits Decisions, but positions would have to be passed in.

## (b) Changing the core's object model instead

The listed changes split into two groups. Frames-once and one-line grants (as long as a grant stays a real Conduit) can never be core changes. Inline definitions without a template object and the removals are real model changes.

**Why frames and grant lines stay in the reader.** The core reads per-document `kind` and `metadata.name` everywhere: the prepare sort, SUB-004 and OBJ-002, `packages::index`, `selections_document`. Anything shorter must be normalized by the reader. If the long form were dropped as accepted input:
- the inputs of 247 cases (316 counting contracts) would be rewritten mechanically;
- all 114 spec examples would be rewritten, plus `check_schemas.py`;
- expectations would not change, as long as the objects are identical.

If package documents were rewritten as well, the 84 fixture archives get new SHA-256 values. The pins in every manifest then change, and so does every expected `packages[].digest` (all S1 cases), because pins are archive-byte digests under PKG-004.

**b1. Inline definitions with no template object ("templates only for reuse").**
- Document-level sites:
  - `registration.rs`: `template()` and `expand`, the template/instance loops at lines 1564 and 1588, `template_diagnostics`, and nested `replacement`;
  - `packages.rs`: lines 216, 577, 643 and 747;
  - `schemas.rs:723` `roots()`;
  - `archives.rs:253`, which prohibits instances in packages;
  - `access.rs`: lines 447 and 580;
  - `records.rs`: inspect `declared` and the replace template lookup;
  - the LIF-009 path in `lifecycle.rs`.
- About 40 stored-object sites tell templates by the `spec` key. They survive if inline instances are stored as today with a `from` marker.
- Size: about 250-500 core lines; the loader's `instance.v1` gains a definition branch.
- Promises: about 12-18 amended (INS-001, INS-002, INS-013, INS-020, INS-025, OBJ-003, NAM-009, NAM-010, NAM-011, PKG-005, LIF-009, TPL-001, SUB-009) and 2-4 new.
- Vectors: 0 invalidated if additive; about 15-25 new. One example flips: `spec/schemas/examples/invalid/INS-001-instance-with-spec.yaml`.
- Enforcing "templates only for reuse" by rewriting fixtures would regenerate all 118 S1 cases and most of the 84 registration cases.

**b2. Grants as one line.** This is b1 for Conduits plus `as`, which ruling 53 already accepts. `local_name` already exists in stored objects and tool naming (`access.rs:103-107`, `registration.rs:1905-1912`, `author.rs:103-107`). A grant that is not a Conduit object would break ruling 52 and is not costed.

**b3. Removing declared Conduits.**
- The code saved is small, because Pool entries share the machinery:
  - the setup "declarations" phase serves both slots (`lifecycle.rs:497-620`), and the Conduit-specific branch is about 15 lines (lines 537-549);
  - the slot loop in `registration.rs` is at lines 566-579;
  - first start derives root's two Conduits from the root template's declarations (`lib.rs:682-690`, and `spec/core/templates.yaml` lines 96-110);
  - the net reduction is about 100-250 lines, with about 300-600 lines touched across the core and the shell.
- Spec: 57 promises mention nesting or declarations. About 30-40 of them concern Conduits rather than Pool entries, including TPL-012, CON-004, LIF-008, LIF-009, IFC-012, IFC-014 and FST-002.
- Vectors: **126 invalidated**, with regenerated expectations because S1's IDs shift (26 are tagged TPL-012, including s1-whole's 58 steps). Six shell test files also depend on S1's declared Conduits: `tests/agents.rs`, `changes.rs`, `runner.rs`, `cli.rs`, `serve.rs` and `src/tests.rs`.
- Safety: TPL-012 promises that an instance's Conduits are opened before its run starts. Without declared Conduits that guarantee moves to clients. This touches S4 and S9.

**b4. Removing replace.v1.**
- About 120 lines in `records.rs` (lines 36-41, 84-133 and about 405-455), plus the root-management, object-management and core-type lists. The capability contract then needs a new version.
- About 6-10 promises (SUB-009, INS-020, REV-011, ACC-033 among them).
- 15 vectors touched: 4 calls, e05-core-replace, and the first-start vectors that assert root's operation array.

**b5. Removing register-replacement.v1.**
- About 135 lines (`registration.rs:1976-2061`, `lifecycle.rs:96-118` and 232-250, plus access routing). It also serves Pool entries: the slot becomes `entries` for a Pool, with a POOL-003 conflict.
- About 10-14 promises.
- 85 vectors touched, mostly by mechanically editing the S1 `workspace-admin` and root-management operation arrays, which shifts no IDs; 7 rewritten.
- It also carries entry replacement and D09's "rotation by explicit replacement" (S2, S10), so it fails the scope guard unless an alternative is shown.

**Totals.**
- b3 + b4 + b5 together: **141 of 367 (38%)**.
- A single form with all of the above: **about 252 of 367 (69%)**, plus 114 examples, a rework of `check_schemas.py`, the fixtures, and re-evidencing increments 2 and 3 (s1-whole and the shell tests).

## (c) The cost of accepting both forms

**Under (a), both forms are accepted by construction.** The long form is what the core consumes. The ongoing costs are:
- about 20-30 promises (+4-7%);
- loader growth of +40-70% on its 2,879 source lines;
- 25-45 vectors (+7-12%);
- a second implementation in Python;
- about 5-8 more concepts for implementers;
- every later change to template, instance or Conduit keys must decide its short-form surface and add 1-3 vectors.

Inline definitions that `$ref` the template-spec schemas give S2-S10's features (requirements, controls, entries, placement, runtime, hooks) automatically. Long-form documents beside the file cover anything else. The existing prepare SUB-004/OBJ-002 checks already catch clashes across the two forms, since expansion produces ordinary documents. The equivalence option guards drift cheaply.

**Under (b) with both kept.**
- The core carries two representations: inline and template-based instances, declared and standalone grants.
- About 600-1,200 more core lines; registration, packages, inspect and replacement each branch.
- About 25-40 promises added with none removed (the design C cost verdict found the same: about 16 concepts added, 0 removed, about 9% more words).
- About 40-70 vectors, because each affected promise is tested in both forms.

**Under (b) with only the new form.** The invalidation figures above apply: 141 to 252 vectors.

**Against the layered approach.**
- The compiler code is the same in both: design E's cost verdict puts it at about 800-1,200 lines.
- The normative route moves it from a shell crate into the loader. It adds the spec text, the Python checker, the runner modes and vectors (about 20-30 promises, 25-45 vectors), and it needs decision 6 settled, because derived defaults become language.
- It changes neither the core nor any existing vector. Neither does the layered route, beyond the 11-14 promises of the five accepted additions, which both routes need and which are not counted here.

## Unverified
- Every line count for new code is a projection from comparable existing code (registration.rs Builder at lines 271-582, project.rs at 239 lines, the runner's registration-orders at lines 598-631). Nothing was written or built, and cargo was not run.
- Vector counts come from my own scripts:
  - the scripts parsed each case's declared files and searched case directories for operation strings;
  - "mentions #register-replacement.v1" counts any appearance, which may include descriptions;
  - the count of 126 declared-Conduit cases relies on parsing referenced files, so documents embedded in other ways may be missed;
  - which cases actually need regenerated expectations, rather than edited inputs, was inferred from how IDs are allocated, not by running anything.
- Promise counts per change come from a regex over promise text (57 nesting, 17 replacement, 133 template, 17 frame). The sub-estimates (30-40 for declared Conduits, and the others) are judgments; I did not read each promise.
- Not checked: whether adding a `kind: Project` route would shift any loader keyword location; whether register.v1 accepting a `project` value would change e05-core-register's sample verdicts; how much of check_schemas.py's step 6 would need the expansion reimplemented in Python.
- Not verified against the S2-S10 scenario texts beyond the ROADMAP table: that S4 and S9 depend on TPL-012's declared-open part, and that S2 and S10's D09 needs register-replacement.v1.
- The idea that grants become standalone Conduits opened by the client after register, rather than declared Conduits opened by setup, is a design choice I assumed from the synthesis. That opens are allowed before the Agent is set up is design E's reading of CON-031 and lifecycle.rs:88-95, which I did not recheck.
- Archive and digest effects if package documents were re-authored were reasoned from the S1 fixture README (pins are archive-byte SHA-256 values), not run.