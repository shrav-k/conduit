# The first build: what it finds

Not binding. The specification in [spec/](../../../spec/README.md) alone binds. Where a log
here and the specification differ, the specification governs.

This directory holds what the first build finds in the language: one log per increment, as
[ruling 46](../ruling-46/README.md), point 4, has it. The increments and what each must show are
pre-registered in [the first build](../../implementation/FIRST_BUILD.md#increments), and the
code is in [reference-model/](../../../engine/README.md).

## What a finding is

Each row of a log is one finding, against the IDs of the promises it concerns, of one of four
classes:

- a **gap**: a case that no promise decides;
- an **ambiguity**: a promise that two readings would build differently;
- a **contradiction**: two promises that cannot both hold;
- **needless complexity**: a part of the language that the task did not need, or that cost more
  than it gave, among it what the profile asks that the tools of the ecosystem make needlessly
  hard.

A defect in a tool that checks the specification, such as `tools/check_schemas.py`, is logged
too, and marked as such: it is not a finding against the promises, but a reader who trusts the
tool is misled by it. Each row says what the build did, and the fix.

## Who settles a finding

Under ruling 46, point 4, the coordinator fixes a finding in the same pull request as the
increment that found it, in the promises and with a vector, and logs the fix as its own, as it
settles a cold reader's finding under rulings 37 and 38. The owner can strike any such fix. A
finding whose fix adds or removes a feature, or changes what Conduit is for, goes to the owner,
and the log records what the build assumed in the meantime.

A log marks each finding as fixed, with its vector, or as left for the owner, with the build's
reading. Increment 1 and its review have eighteen findings, all settled. Ruling 48, point 3,
settled the restriction on pattern tokens; the other settlements are the coordinator's under
ruling 46, point 4, and the owner can strike them. None adds or removes a feature or changes
what Conduit is for.

## Increments and settlements

Increments 1 to 3 are closed. Their findings are preserved below under ruling 58, point 3;
the original logs are in the archive’s C0 history. These are coordinator settlements under
ruling 46, point 4, strikeable by the owner; ruling 48, point 3, settled 1-04/1-17, and the
owner chose 3-07’s stated leak. The [admin’s decisions](increment-2/decisions.md) keep the
increment 2 choices. Increment 4 remains open in its [log](log-increment-4.md); its measurement
plan, dated reports and raw runs stay until the S1 rerun.

Evidence names below identify existing examples, loader/package tests or portable case IDs;
they are historical settlement evidence, not fresh test results or whole-promise proofs.

## Settlements register

| ID | Decision | Promises | Vector or evidence | Rejected alternative and why |
| --- | --- | --- | --- | --- |
| <a id="1-01"></a>1-01 | SUB-005 attributes every schema rejection, completed by 1-14 | OPS-005, IFC-025, OBJ-023, SUB-005 | `tests/examples.rs::every_invalid_example_is_rejected_by_the_step_its_name_gives`; `tests/project.rs::every_fault_of_a_project_is_reported` | Per-keyword promise annotations would change every schema for an answer SUB-005 already gives |
| <a id="1-02"></a>1-02 | Translate end anchors and whitespace to ECMA-262 in the Python checker | SCH-016, OBJ-001 | `spec/schemas/examples/invalid/OBJ-001-name-ending-in-a-line-feed.yaml` | Python re semantics admit a name ECMA-262 rejects |
| <a id="1-03"></a>1-03 | Reject an invalid authored pattern at installation/submission under SCH-016 | SCH-016, SCH-002, SCH-003 | `SCH-016-pattern-that-is-not-ecma-262.yaml`; `tests/examples.rs::check_rejects_the_invalid_by_rule_examples_of_sch_016_alone` | Deferring rejection until use leaves an invalid schema admitted |
| <a id="1-04"></a>1-04 | Keep ECMA-262 in Unicode mode; ruling 48 point 3 restricts its tokens (1-17) | SCH-016, INS-014 | 1-17 below; no separate vector added for the initial decision | I-Regexp would force rewrites without lookahead; no I-Regexp question remains open |
| <a id="1-05"></a>1-05 | UTF-8 is the profile rule; reject unreadable syntax under SUB-005; 1-15 allows one initial BOM | SUB-005, SCH-013 | `spec/schemas/examples/invalid/SUB-005-not-utf-8.yaml` | Accepting YAML UTF-16/32 would leave file encoding unspecified across engines |
| <a id="1-06"></a>1-06 | Skip dot-names; read exact lowercase suffixes, follow file links, reject unreadable selected entries | MAN-011, MAN-001, SUB-010, PKG-022 | `spec/schemas/examples/valid/project/.notes.yaml`; loader dot-name test | Reading every matching suffix admits macOS tar ._ files; only the dot part of Terraform’s rule is needed |
| <a id="1-07"></a>1-07 | A JSON non-map is one document, judged under SCH-013/SCH-018; a manifest array is MAN-001 invalid | SCH-018, SCH-013, SUB-005 | `spec/schemas/examples/invalid/SUB-005-json-value-that-is-not-a-map.json` | Skipping non-maps disagrees with the YAML document model |
| <a id="1-08"></a>1-08 | Check exact decimal numbers and grammar formats before taking binary64 values | SCH-020, SCH-021, SCH-004, SCH-019 | `SCH-021-fraction-just-above-binary64.yaml`; SCH-019 invalid uuid/duration examples | Python number rounding and built-in format checks accepted values the profile rejects |
| <a id="1-09"></a>1-09 | Keep PyYAML; list its YAML 1.1 differences; the tool is not the reference and examples may rely on none of those differences | SCH-012, SUB-005 | Checker header; no new vector or promise change | Replacing it adds a contributor dependency; the loader follows YAML 1.2.2 independently |
| <a id="1-10"></a>1-10 | Wholeness is on the binary64 value, held to ±(2^53−1) | SCH-005, SCH-006, SCH-020, SCH-021 | `SCH-006-fraction-read-as-an-integer-out-of-range.yaml`; `tests/reading.rs` and `number.rs` edges | Decimal-only wholeness lets a fractional decimal round to an out-of-range whole value |
| <a id="1-11"></a>1-11 | Add SUB-011 and state DL10 limits | OPS-005, IFC-025, SUB-005, DL10 | `tests/reading.rs::values_nest_no_deeper_than_the_limit` | SUB-005 would call a valid document malformed merely for exceeding an engine limit |
| <a id="1-12"></a>1-12 | Apply only I-JSON’s no-duplicate-key and whole-number-range rules; allow noncharacters; unpaired surrogates are not text, and YAML rejects a pair written as two escapes | SCH-013, SCH-012 | `tests/reading.rs::of_i_json_only_two_rules_apply_to_strings` | I-JSON’s full string restriction rejects harmless noncharacters allowed by YAML/JSON |
| <a id="1-13"></a>1-13 | ABNF letters match either case for uuid and duration | SCH-004 | `tests/formats.rs` duration/uuid grammar tests | A case-sensitive reading invents a restriction absent from the cited ABNF |
| <a id="1-14"></a>1-14 | SUB-005 throughout, with keywordLocation and known absoluteKeywordLocation; a $ref to a non-Schema still cites SCH-015 | SUB-005, PKG-023 and the schema-encoded rules | `tests/project.rs::a_schema_rejection_carries_the_keyword_location`; schemas absolute-location test | Schema-encoded promise attribution varies across engines; the instance pointer alone does not locate the failing keyword |
| <a id="1-15"></a>1-15 | Accept and ignore one initial BOM in either syntax; retain source positions | SUB-005, SCH-013; the profile | `spec/schemas/examples/valid/project/bom.json`; reading and checker profile regressions | Rejecting JSON’s BOM contradicts the chosen RFC 8259 permission; skipping two would exceed the profile |
| <a id="1-16"></a>1-16 | At least 64 levels; limits cover files, submissions and archives; loader depth remains 128 | SUB-011, PKG-023, DL10 | `tests/reading.rs::values_nest_no_deeper_than_the_limit` at 63/64/65/128 and beyond | An unrestricted engine limit has no portable floor; no file/archive runtime limit is claimed by the loader |
| <a id="1-17"></a>1-17 | Apply ruling 48 point 3’s interoperable ECMA-262 tokens, including patternProperties keys | SCH-016, INS-014 | SCH-016 invalid examples; `tests/formats.rs`; `tests/pattern_rewrites.rs::every_rewritten_pattern_keeps_its_verdict` | Unrestricted expressions burden non-JavaScript engines; I-Regexp is not adopted; rewrite comparisons changed no verdict |
| <a id="1-18"></a>1-18 | MAN-010: an artifact hash pins no registry-resolved dependencies; a command pins nothing | MAN-010 | `spec/schemas/examples/valid/files-package/conduit.yaml` comment (illustration only) | Treating the artifact hash as a dependency pin claims execution guarantees the loader cannot prove |
| <a id="2-01"></a>2-01 | Reject two initial YAML BOMs under SUB-005; YAML 1.2.2 has one optional prefix mark; vector-only application, no spec change | SCH-013, SUB-005; schema profile, Encoding | `loader-leading-bom` | Ignoring both exceeds the YAML prefix; a permitted mark before a later document remains accepted |
| <a id="2-02"></a>2-02 | Reserve qualified contract-copy root $defs keys under IFC-033, even if publication never reaches them | IFC-033 | `e05-root-key`; `IFC-033-reserved-root-defs.yaml` | Renaming and rewriting authored keys violates members-keep-their-keys; changing the copy scheme changes every publication |
| <a id="2-03"></a>2-03 | Different unconditional $ref/sibling/allOf defaults reject under SCH-009; equal defaults are one | SCH-009, SCH-015, SCH-019 | `schema-conflicting-defaults`; `schema-equal-defaults` | An embedding error or traversal-order choice leaves a document’s meaning undefined |
| <a id="2-04"></a>2-04 | Retain/replay plain content with outputs; expiry reports OPS-034 conflict | OPS-009, IFC-025, DL10 | `requests-plain-content` (requests/plain-content/case.yaml); declared-outputs | Outcome-only repeats lose the read; redispatching a success to recover content may repeat an effect; an adapter is unnecessary |
| <a id="2-05"></a>2-05 | A plain -32602 is failed only for no effect, no control, no model work; unknown otherwise | IFC-010, OPS-003, OPS-018 | `read-invalid-parameters`; `write-invalid-parameters` | The code alone proves no pre-effect rejection; a new implementation guarantee adds a qualification surface |
| <a id="2-06"></a>2-06 | After -32020 repair/send again only in that same safe category, once | IFC-046, OPS-018 | `repair-read-success`; `read-parameter-headers`; `unsafe-write-headers` | Unconditional recovery can repeat an effect; dropping safe recovery entirely loses protocol recovery |
| <a id="2-07"></a>2-07 | No effectful-plain safe-repeat declaration is added; narrow decision 8a | IFC-046, OPS-012 | `effectful-plain-headers`; `aware-headers-no-repair` | Such a declaration or a new Conduit-aware no-effect guarantee is an owner feature, not a coordinator gap fix |
| <a id="2-08"></a>2-08 | Management alone authorizes a missing object; Pool-only callers are refused without disclosure; OPS-029 reversed explicitly | ACC-033, OPS-023, OPS-029, LIF-037 | `records-expected-id` missing-object pairs with/without stale ID | Invalid through a Pool reveals whether a global name exists elsewhere; expectedId supplies no authority |
| <a id="3-01"></a>3-01 | JWT-SVID exp is iat plus 100 calendar years; reject expired tokens | DL13, IFC-022, PRV-022 | Shell identity tests | No exp conflicts with DL13 and JWT-SVID section 3.3 |
| <a id="3-02"></a>3-02 | Require model string with empty-string default; authors may omit it | MAN-017, SCH-009 | Agent Type/package validation | An optional bound field violates MAN-017’s required scalar rule |
| <a id="3-03"></a>3-03 | Omit outputSchema and successful structuredContent when no outputs are declared; preserve plain content | IFC-025, IFC-033, OPS-009 | Updated request repeat vectors and real Claude closeout | An empty schema/map makes the client prefer the empty structured result over the file text |
| <a id="3-04"></a>3-04 | Claude --setting-sources empty; engine settings/config supplied explicitly | ACC-027 | Closeout ancestor-instruction baseline and wrapped run | Project sources load ancestor CLAUDE.md outside the private directory |
| <a id="3-05"></a>3-05 | Disable machine features and project docs on first launch/resume; accountServices enables apps alone; narrowed by 3-06 and later 4-06 | ACC-027 | Closeout outside-file and instruction sentinels | ignore-rules alone left shell reads and ancestor AGENTS.md available |
| <a id="3-06"></a>3-06 | Keep code_mode_host on; shell and unified exec remain off | ACC-027, IMP-001 | Second closeout failure and direct host-on trial; final closeout | Disabling the host disables engine MCP tools too; host-on outside-file read returned unsupported import in exec |
| <a id="3-07"></a>3-07 | State Codex’s home-instruction leak; later own home/login per Agent when less cumbersome, with login as a credential Resource | ACC-027 | Owner-authorized home sentinel in wrapped and direct runs; closeout results | The flags tried do not exclude $CODEX_HOME/AGENTS.md; copying subscription login remains A5’s question; skills/memories untried |

## Open remainders of increment 1

| Item | State and boundary |
| --- | --- |
| saphyr document marker | A root block-scalar line beginning `---` is read as content by saphyr-parser although YAML 1.2.2 c-forbidden makes it a document marker. The loader rejects it under SUB-005; YAML would yield two schema-invalid documents (or MAN-001 for a manifest). Same verdict, different rule. Reporting the fault upstream remains open; this order has no network access. |
| patternProperties engine | The loader leaves patternProperties and adjacent additionalProperties to jsonschema’s non-ECMA-262 engine. No specification schema uses them. The core’s authored-schema matcher handles them itself: this is a loader boundary, not an open core matching decision. Port loader-only coverage under A39; no new behavior is chosen here. |

The other increment-1 remainders are carried: the core supplies cross-document checks,
relative manifest resolution, registration and diagnostic.v1-shaped results. UUIDs follow
RFC 9562 ABNF (no version/variant restriction); decimal overflow, request/document ordering,
ECMA-262 edition/token restrictions, plain versus quoted merge keys, and empty-file versus
empty-document reading were already decided, not open language questions.

## D19 measurement

**Measured D19 sample, 2026-11-09.** In `records-purge`, record storage fell from 9,187 to
4,495 serialized bytes. The keyed unknown write's invocation entry stayed at 373,365 bytes.
The entire invocation array grew from 499,935 to 622,636 bytes because the purge itself is
another retained invocation. These counts include the representation's held contexts; they
are measurements of this vector, not a capacity projection or a proposed compaction policy.

## Noted for later increments

- **Token helper after a 401 (PRV-022).** Experiment 008’s answer replaces the earlier question:
  Claude does not re-run the helper; Codex re-runs it but fails initialization with “Auth required”.
  See [experiment 008](../../../experiments/008-real-client/README.md).
- **Missing plain tool.** The plan named IFC-002, which fixes the baseline revision; the current missing-tool rule is IFC-001: an absent server tool makes
  the call unavailable. The old failed-versus-unavailable question is closed and dropped from
  future work; this note records its disposition.
