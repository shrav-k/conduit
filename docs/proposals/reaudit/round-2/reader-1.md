# Re-audit round 2, reader 1: duplicates and contradictions

Read in full: every file under `spec/` (README, definitions, schema profile, names, the implementation-defined list, all 395 promises, `core/`, `schemas/` and all 73 example files) and the "Known before the round" list; nothing else in the repository was opened.

Findings are numbered within each heading, most severe first. "Needs a decision" marks a point where the text supports two behaviors and the owner has to choose.

## 1. Promises that contradict each other, or give different results for one situation

**1.1 ACC-030, ACC-029, ACC-022: a Conduit to `access` with no bound passes the "within" test as written.**
ACC-029 says "a Conduit to `access` that applies no `bound.v1` has no bound", and ACC-022 makes "no bound" unlimited ("every bound is within its own"). ACC-029 also says "A bound that names nothing covers nothing", so the two are opposites. ACC-030 defines the test only by what a bound names: "A new bound is within an old one when every Pool and every individual target it names is covered by the old one ... and every operation it names is one the old one names." A new Conduit that applies no `bound.v1` names nothing, so the test holds vacuously, and an actor with a narrow bound could open an unlimited right to open. The observable of ACC-030 says the opposite ("its bound covers nothing that the bound of the actor that opened it does not cover").
Fix: add to the `then` of ACC-030: "A Conduit that has no bound is within another only if that other has none."

**1.2 REV-012, REV-003, REV-004, OBJ-011, `rename.v1`: a rename names a revision that it does not advance.**
OBJ-011 says a rename "names the instance's current revision and changes only its name", with "the same ID and definition" afterwards. `rename.v1` says "Its ID, its definition and its revision are unchanged." REV-004 forbids an advance when "stored definition and configuration do not change". REV-012 says "Of several requests to change one object that name the same revision, at most one is stored: once it is, the revision has advanced under REV-003". Two renames that name the same revision are both stored under the first three and only one under REV-012.
Needs a decision: (A) a rename advances the revision, so REV-003, REV-004, the definition of revision and the description of `rename.v1` change; or (B) it does not, and REV-012 excepts a rename.
Related drift: OBJ-011 speaks only of "An instance is renamed", while `rename.v1` and ACC-033 rename "a template or an instance".

**1.3 LIF-024, LIF-017, LIF-018, CON-008, LIF-023: an open Conduit that is closed without a request can never be unregistered.**
CON-008 closes a Conduit "at once" when its target is retired or an end is declared lost, and LIF-023 calls these "retirements without a request". LIF-024 gives the shutdown as confirmed "when the retire operation succeeded" or for an instance never set up, unknown for two named cases, and "pending otherwise". A Conduit that was open and is closed by CON-008 has no retire operation, so it is pending. LIF-017 then never dispatches its unregistration, and LIF-018 offers only a reconciliation of "the retire operation", which does not exist. The exception for "its location was declared lost" does not reach a Conduit, which has no location. TPL-009 ("With no `retire` graph to run and no runtime held, the retire operation succeeds at once") supports the other reading only if a close under CON-008 is a retire operation.
Needs a decision: (A) the shutdown of a Conduit that CON-008 closes is confirmed at once; or (B) it is pending and some request confirms it. Then state it in LIF-024.

**1.4 ACC-033, OPS-014, LIF-018: which target authorizes reconciling a setup or a retirement.**
ACC-033 and OPS-014 say "the same target as setting up and retiring the instance the invocation was dispatched to". LIF-018 says "the same target as retiring the instance, as ACC-033 states". For a Resource R owned by Pool P the request is `retire.v1` through a Conduit to P with the input `instance: R`. By the definitions of target and dispatch the invocation was dispatched to P, so ACC-033 gives `management` (where retiring P is authorized); LIF-018 gives P. For an Agent the first reading gives the Pool `system` (which owns `management`), the second `management`.
Fix: reword ACC-033 and OPS-014 to "the instance that the invocation set up or retired, or else the instance it was dispatched to". The root cause is finding 2.1.

**1.5 OPS-015, OPS-008, IFC-024: the situation OPS-015 refuses cannot be told from "no such invocation".**
OPS-015 refuses "the retained result of an invocation it did not make". The retrieval tool names an invocation only by `operationIdentity` and `target` (IFC-024), and OPS-008 says "The same identity sent by another caller ... belongs to another invocation". So a caller can never address another caller's invocation; what it sends is an identity under which it has no invocation, and no promise gives the reason for that. This is close to the known point on an unregistered target, and is a separate case.
Needs a decision: (A) OPS-015 is reworded as the rule for an identity under which the caller has no invocation, reporting refused; or (B) another reason is chosen for that case and OPS-015 is removed as unobservable.

**1.6 NAM-007, NAM-020, NAM-021: NAM-007 rejects every reference to the core package.**
NAM-007 rejects when "A document refers to an address that the manifest of its project does not list, or is submitted with no manifest." NAM-021 forbids listing the core address and NAM-020 accepts a reference to it "with no manifest". NAM-007 states no exception; the valid example `first-start.yaml` is rejected by it as written.
Fix: reword the `when` of NAM-007 to "an address, other than the core package's, that ...".

**1.7 PLC-005: "names a location" is either empty or forbids what PLC-008 relies on.**
PLC-005 rejects "A template ... that names a location", with "a diagnostic naming the field that holds the location". No template can write a placement: TPL-001 gives no such key and OBJ-014 rejects `placement` on an entry. Read narrowly the promise has no case left except an input's `default`. Read as written it rejects a declared Conduit whose `target` is a location and a bound whose `targets` name one, although PLC-008 says `set-facts.v1` "is requested through an open Conduit to the location" and OBJ-008 lets a template refer to any object. POOL-009 cites PLC-005 for an entry that "itself names a location", which OBJ-014 already rejects on shape.
Needs a decision: (A) PLC-005 covers placement only, and is reworded to the one case that remains or merged into OBJ-014; or (B) no template may refer to a location at all, and PLC-008 says a Conduit to a location is always instantiated on its own.

**1.8 GRA-003, CON-004, POOL-013, TPL-007: only one of three "already done" steps is said to succeed.**
GRA-003 needs every step not marked optional to have "the outcome succeeded". TPL-007 says of a runtime already held that "the step is left as succeeded". CON-004 says of an open Conduit that the step "leaves it as it is", and POOL-013 that a ready Resource "is left as it is". Read with GRA-003, a second setup of an instance with an open declared Conduit cannot succeed.
Fix: reword CON-004 and POOL-013 to "and the step is left as succeeded", as TPL-007 has it.

**1.9 LIF-023, LIF-021: the list of retirements without a naming request omits one.**
LIF-023 says "The only retirements without a request naming the instance are those of a Conduit ... and of the instances on a location that is declared lost." Under LIF-021 `retire-with-resources.v1` has no input and names only the Pool that is its target, yet "A request to retire each Resource is dispatched".
Fix: add the Resources of LIF-021 to the list in LIF-023.

**1.10 OBJ-013 and the definition of instance, against PLC-002 to PLC-004 and PLC-008.**
OBJ-013 says an instance document "has `apiVersion`, `kind`, `from` and the standard arguments of its family as top-level keys"; the definition repeats it. `placement` is a standard argument, yet PLC-002 to PLC-004 accept a document that "gives no placement" and PLC-008 rejects a location's document that states one. `instance.v1` agrees with the PLC promises.
Fix: reword OBJ-013 and the definition to "`owner`, `consumer` and `target` are required; `placement` may be left out, and a location's document leaves it out".

**1.11 Addition to the known point on an input with a default.**
The known point names an Operation's input. The same collision with OBJ-015 ("Without that mark it is required") holds for a template's input (INS-004 rejects only an input "with no `default` and without marking it `optional: true`") and for a Requirement's input (NAM-019 checks "exactly as in instantiation"). The core package depends on it: the three inputs of `bound.v1` state `default: []` with no mark, and ACC-029 says each is "empty where it is left out".
Fix: whatever is decided for OBJ-015 has to cover all three kinds of input.

## 2. Contradictions with the definitions, the README, the schemas, the examples or the core package

**2.1 "The setup operation" and "the retire operation" of an instance have no contract.**
GRA-014 says "A setup graph implements its instance's setup operation"; TPL-008 says the same of `retire`; GRA-020 says these graphs "have no inputs"; IMP-005 and IMP-009 say the operation "is never marked"; OPS-022, LIF-018 and LIF-024 speak of "the retire operation". The definitions say an operation is "defined by an Operation contract". The core package has no such contract. It has `set-up.v1` and `retire.v1`, each with the input `instance`, implemented by `engine`, whose target is `management` or a Pool and not the instance. So either a setup is one invocation of `set-up.v1` (and a setup graph does have the input `instance`, against GRA-020), or it is a second invocation of an operation that no document defines (against GRA-014, "A graph that is attached to no operation ... is rejected").
Needs a decision: (A) one invocation, of `set-up.v1` or `retire.v1`, and "the setup operation of an instance" is defined as the part of it that the template's graph supplies; or (B) two invocations, with a contract for the second. Findings 1.3 and 1.4 follow from this.

**2.2 `register-replacement.v1` covers a component, which LIF-009 cannot serve when the composite was instantiated on its own.**
The contract's description lists "a declared Conduit, the Resource of a Pool's entry or a component of a composite Conduit". LIF-009 makes the new instance "from the nested instantiation as the instance it belongs to holds it in its resolved definition". CON-003 says of a composite that "no instance represents the composition itself", so for a composite instantiated by its own document nothing holds the component.
Needs a decision: (A) drop that case from the description; or (B) say in LIF-009 where the component is taken from.
Related: POOL-003 gives conflict for a request while the entry's Resource "is not retired", and no promise says the same for a declared Conduit or a component, though the README maps both promises to this one operation.

**2.3 CON-028 against the example `invalid-by-rule/CON-028-bound-applied-twice.yaml`.**
CON-028 acts when "A Conduit is instantiated" and says "The Conduit is not instantiated". The example is "One document", a template, "that must be rejected". The comment in `schemas/templates.yaml` ("that a Conduit template applies the bound of a right to open at most once") and the row in `schemas/README.md` ("Yes: a template that applies it twice") also treat it as a rule on the template.
Needs a decision: (A) a template that applies `bound.v1` twice is rejected when registered, and CON-028 says so; or (B) only the instantiation is refused, and the example gains an instance document.

**2.4 That the bound is a filter is stated only in comments.**
`bound.v1` states `narrows: true`. The comments of `core/requirements.yaml` and `core/conduit.yaml` and the example template `team-access` say "the bound is a filter", the first citing "the definitions of right to open and bound", which do not say it. If it is one, it is a restriction: IFC-018 lists it in `restrictions`, ACC-017 carries it on onward requests, ACC-027 reports it as mediated, and IFC-025 names it in `restriction` on a refusal. The definition of refused lists the two apart: "a restriction forbids it, a bound does not cover it".
Fix: add one sentence to ACC-029 and to the definition of bound saying that the bound is a filter, or set `narrows: false` and correct the comments.

**2.5 OPS-019 and `pool-management.v1` say every Pool offers it; CTR-004 does not require it.**
OPS-019 says "`pool-management.v1`, which every Pool offers", and the capability's description says "What every Pool offers". CTR-004 lets a Type of the family Pool state any `offers`, and no promise rejects one that omits this capability. ACC-033 and POOL-015 cannot be met for such a Pool, because CON-021 refuses a Conduit whose capability the target's type does not offer.
Needs a decision: (A) a Pool type that does not offer it is rejected, stated in CTR-004; or (B) the engine offers it for every Pool whatever the type says, stated in OPS-019.

**2.6 `apply-policy.v1` takes only a qualified reference; NAM-018 and IFC-018 allow `#name`.**
The input `requirement` is `qualified-reference.v1`. NAM-018 accepts both forms for "the `requirement` of an applied requirement", and IFC-018 lets a restriction, "every policy that applies" included, name "a requirement of an unpublished package" as `#` and its name. A policy of an unpublished project can be sent on the wire and cannot be applied. The `operations` of `bound.v1` have the same limit, so a Conduit that lists `#name` operations can be opened only by an actor with no bound.
Needs a decision: (A) policies and bounds reach published contracts only, stated in ACC-031 and ACC-029; or (B) the two inputs take another form.

**2.7 The core package says a request that ends with any reason carries diagnostics; OPS-023 says a refusal says nothing more.**
`core/README.md` and the header of `core/operations.yaml` say "a request that ends with a reason carries them too". OPS-023 says a request that is not permitted "says nothing else about the objects it names".
Fix: reword both to "that ends with the reason invalid or conflict".

**2.8 `core/README.md` and `core/types.yaml` cite the definitions for a fact they do not name.**
Both say "`host` and `localPaths` are the two facts the definitions name". `definitions.md` names `host` only; `localPaths` appears nowhere outside `core/` and the examples.
Fix: add `localPaths` to the row for location fact, or reword to "the fact the definitions name is `host`".

**2.9 The count of exports.**
`spec/README.md` and `core/README.md` say "98 exports and 1 implementation". PKG-019 says an implementation "has a name, which is an export", the definition of export includes it, and the table "Every export" has a row for `engine`.
Fix: say "98 documents exported and 1 implementation, which is also an export".

## 3. Promises that state the same obligation and have drifted

**3.1 ACC-019 and IMP-006: work that runs later.**
Both bind the work "to the Conduit through which it was arranged". ACC-019 covers any "Work that a caller arranged" and says that after the close "the work's requests report refused when it runs". IMP-006 says such work "was arranged by an operation that is marked as starting model work" and that "A start ... is not dispatched and reports refused". They differ on whether work arranged by an unmarked operation may run later at all, and on whether the start or the work's own requests are refused.
Needs a decision on those two points; then merge into IMP-006 and leave ACC-019 as the access half only.

**3.2 TPL-009, LIF-024, LIF-025: "never set up" against "no setup was ever dispatched".**
The `then` of TPL-009 and LIF-025 turn on whether "a setup of the instance was ever dispatched". The observable of TPL-009 says "An instance that was never set up", and LIF-024 says "an instance that was never set up". For an instance whose only setup was dispatched and failed, the two wordings give different answers on whether the `retire` graph runs and whether shutdown is confirmed at once.
Fix: use "for which no setup was ever dispatched" in all three.

**3.3 OBJ-014, POOL-012, POOL-009, POOL-016, CON-020: the shape of a nested instantiation.**
POOL-012 is a strict subset of OBJ-014 and has drifted: it rejects an entry with `owner` and is silent on `placement`, which OBJ-014 rejects. POOL-009 gives a third ground for the same rejection (PLC-005). OBJ-014 also restates POOL-009 and POOL-016 on how an entry is placed, and the part of CON-020 on a declared Conduit and a component repeats OBJ-014.
Fix: merge POOL-012 into OBJ-014; cut the placement sentence from OBJ-014 in favor of POOL-009 and POOL-016; reduce CON-020 to the Conduit template.

**3.4 LIF-006 and LIF-015: setting up a retired instance.**
The second sentence of LIF-006's observable ("A request to set up a retired instance ... is not dispatched and reports conflict") is a strict subset of LIF-015, and LIF-008 cites LIF-006 for it.
Fix: remove the sentence from LIF-006 and have LIF-008 cite LIF-015.

## 4. Wording that assumes an older model

**4.1 NAM-019: a policy applied by a document.**
"A document applies a requirement, as a filter, as a policy or as a requirement on placement." A policy "is never written in a template" (definitions), OBJ-019 rejects one written there, and ACC-031 applies it by a request. `applied-requirement.v1` already says "on a Conduit or on placement". The phrase "as a filter" also leaves out `shared-host.v1`, which is applied to a Conduit and is not a filter.
Fix: reword to "applies a requirement to a Conduit or to placement".

**4.2 CTR-002 to CTR-007: rejection only "when its package is installed".**
CTR-001, CTR-009, CTR-010, SCH-002 and SCH-019 say "when its package is installed or its project is submitted". CTR-002 ("when the contract's package is installed"), CTR-003, CTR-004, CTR-005, CTR-006 and CTR-007 ("Installation returns a diagnostic") name installation alone, as before SUB-008 gave a project contracts of its own. A contract of an unpublished project that breaks one of them has no stated moment of rejection.
Fix: use the phrase of CTR-001 in all six.

**4.3 "Entry" for an item of the implementation-defined list.**
The definitions say "entry" is used only for a Pool's entry and give "item" for the list. `implementation-defined.yaml` keys each item `entry:` and says "its choice for every entry", and the definition of purge says "as entry DL2".
Fix: rename the key and both phrases to "item".

**4.4 Two small leftovers.**
The comment of `invalid/SCH-015-ref-to-an-https-address.yaml` says "a `$ref` is `#` and a JSON Pointer, or a qualified reference", from before the third form. MAN-001 says a manifest "holds exactly one document", while the definitions say "A manifest is not a document".
Fix: add the third form to the comment; reword MAN-001 to "one YAML document".

**4.5 "The owner" as a person, outside the promises.**
The definitions say "No promise uses the word for a person", which holds. The header comment of every promise file ("Owner rulings of ..."), `core/README.md` and `schemas/README.md` ("accepted by the owner") use it so. Cosmetic.
Fix: none needed, or "the specification's owner" once in `spec/README.md`.

## 5. Promises whose layer is wrong

**5.1 OBJ-011 and SUB-007 (language) require management operations and revision checking.**
"It is done by a rename operation that names the instance's current revision" and "It is done by a replace operation that names the template's current revision". The operations are management verbs under OPS-019 and the revision rule is REV-002 and REV-011, all `convention`. As written, an engine at the Language level ("documents in, resolved objects or diagnostics out") must offer both.
Fix: keep in each only "Submitting a document never renames an instance" and "never replaces a template", and move the sentence on the operation to a promise of the layer `convention`.

**5.2 OBJ-002 (language) states a reason.**
"The document is rejected, and its registration reports conflict." Its sibling OBJ-023 is `convention` for that cause, as are PKG-023 to PKG-025.
Needs a decision: move the clause into OBJ-023, which would then name two reasons, or into a new promise of the layer `convention`.

**5.3 PLC-008 (language) states how a control is requested and what it returns.**
"Once the location is instantiated they are changed only by the control `set-facts.v1`", with its Conduit, inputs and result in the observable. PLC-016 says the same kind of thing for `set-default-location.v1` and is `convention`.
Fix: move those sentences of PLC-008 to a promise of the layer `convention`.

## 6. Citations for something the cited promise does not say

**6.1 OBJ-001 cites NAM-004 for a contract's name.** NAM-004 applies when "A package is installed" and governs an export's name. The rule for a contract, including one of an unpublished project, is CTR-001. Fix: cite CTR-001.

**6.2 TPL-004 cites NAM-014 for how a default implementation resolves.** NAM-014 is about a package's document that "refers to another package". A default written `#name`, as in every example, resolves under NAM-022. Fix: cite NAM-022 beside NAM-014.

**6.3 CON-028 and ACC-029 cite each other.** CON-028 says "A bound is carried only by a Conduit to `access`, as ACC-029 states"; ACC-029 says "CON-028 says where a bound may be applied" and does not state "only". Fix: drop "as ACC-029 states".

**6.4 Two rows of the definitions.** "set up" cites TPL-012, which covers only a template with no `setup` graph. "right to open" cites ACC-023, which uses the term and does not define it. Fix: cite GRA-014 and CON-004 for the first, and ACC-006 alone for the second.

## Clean

Checked and found consistent:

- **Counts.** 395 promises, the count each file's row gives, 14 implementation-defined items, 45 schema contracts in 9 files, and 55, 11 and 7 example files.
- **Retired IDs.** No promise cites a removed ID, and every gap in numbering is explained in its file's header.
- **Vocabulary.** No use of grant, issue or revoke, provision, driver, handler, Host, declaration, create, a name under `conduit.dev`, "not open" as a state, values under `inputs` on an instance (only as a rejected form), `source` for anything but a value reference, or "hold" for a Conduit or a right. "Registered" as a description is always the state.
- **Reasons.** Apart from the findings above, every promise that names a reason agrees with the table in the definitions and with the order of OPS-023, including the precedence in LIF-022 and the split of PKG-023 to PKG-025.
- **Outcomes and the wire.** OPS-001 to OPS-012, OPS-024, OPS-025, IFC-009, IFC-010 and IFC-025 to IFC-027 agree with each other and with the definitions, as do withheld content (OPS-017, ACC-034, IFC-024, IFC-025) and onward requests (ACC-016 to ACC-018, IFC-020, IFC-021, IFC-028, IMP-008).
- **Core operations.** Inputs, outputs, `effect`, `deduplicates`, `reservedToRoot` and capability membership match the promises that need them (LIF-013, REV-006, REV-007, REV-010, REV-011, ACC-031, ACC-032, ACC-033, REC-004, PLC-016, PLC-008, IFC-024), except as found above.
- **Core types, capabilities and templates.** They satisfy CTR-003, CTR-004, CTR-006, CTR-007, CON-016, CON-017, TPL-002 and FST-001 to FST-005; the manifest satisfies MAN-004, MAN-007 and MAN-012.
- **Schemas.** `schema.v1` matches the profile's keywords, formats and the three forms of `$ref`, and encodes SCH-008. The value patterns match OBJ-001, NAM-001, NAM-004, CTR-001, PKG-001, PKG-004 and OBJ-016. The `reaches` patterns match INS-012. `instance.v1`, the nested instantiations, `step.v1`, `value-reference.v1`, `operation-spec.v1`, `requirement-spec.v1` and `implementation-item.v1` match their promises.
- **Examples.** Every valid example is accepted by the promises, apart from the wording of NAM-007 in 1.6. Every invalid example is rejected by the promise it is named for, and every invalid-by-rule set by its rule, apart from 2.3.
- **Citations.** Every citation in the promises and the definitions was followed; the four in section 6 are the only ones that miss.

## Judgement

Thirty-six findings: about fifteen are real, in that two engines could act differently or a promise cannot be met as written (1.1 to 1.7, 2.1 to 2.6, 3.1, 3.2); the rest are wording, merges and citations that one editing pass would close. The three that matter most are 1.1, where the test for passing on a right to open is satisfied by a Conduit with no bound; 2.1, where the setup and retire operations of an instance have no contract, which is the root of 1.3 and 1.4; and 1.2, where a rename names a revision it does not advance. The round is not yet clean but it is close: vocabulary, reasons, the wire, the schemas and the examples hold together, and what remains is concentrated where the core package, written last, meets the lifecycle and access promises written before it.
