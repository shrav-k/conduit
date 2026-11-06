# Re-audit, reader 1: duplicates and contradictions

Cold reader's report of 2026-11-05, kept verbatim. The reader saw only `spec/`. Findings are
not rulings; what was done about each is in the [re-audit record](README.md).

```text
Scope read: every file under spec/ in full (README.md, definitions.md, schema-profile.md, implementation-defined.yaml, 13 promises files, 7 conventions files, interface/implementation.yaml; 312 promises, matching the README counts). Nothing outside spec/ was read. No file was edited.

1. DUPLICATION (merge candidates)

1.1 ACC-011, CON-014, IFC-013, LIF-015 (also ACC-019, IMP-006). All state that a request through a closed Conduit is refused: "After the close every request through the Conduit reports refused" (ACC-011); "A request through the closed Conduit made after the close reports refused" (CON-014); "a call to one of them reports refused" (IFC-013); "A consumer's request reports refused, because the Conduits to the instance are closed" (LIF-015). Fix: ACC-011 survives as the single statement. CON-014 keeps only "the consumer is not retired or interrupted", IFC-013 keeps only the tool list, and LIF-015 keeps only the conflict half.

1.2 LIF-002, INS-016, SUB-006, CON-002, CON-012, POOL-002. Six promises say a new instance is in the state registered and nothing is set up: "Its state is registered" (LIF-002); "Each is in the state registered. Nothing is set up or opened" (INS-016); "every new instance is in the state registered, no setup has run" (SUB-006); "each such Conduit is in the state registered" (CON-002); "created in the state registered" (CON-012); "in the state registered ... None has been set up" (POOL-002). Fix: LIF-002 survives for the state and INS-016 for "no call, nothing opened"; delete the state clause from the other four and delete SUB-006's second sentence.

1.3 OBS-003, LIF-010, PLC-010 (and PLC-009). Three promises say that what cannot be observed is reported unknown and the lifecycle state is kept. PLC-010's "it is not given another placement" also repeats PLC-009. Fix: merge into OBS-003 (unknown, with the time of the last observation); keep LIF-010 only for "lifecycle state stays as last recorded"; delete PLC-010.

1.4 LIF-008, CON-010, LIF-009, POOL-010, POOL-003. CON-010 is LIF-008 restated for a declared Conduit ("must not ... The Conduit is opened again, or another Conduit is opened in its place"). POOL-010 is LIF-009 restated for an entry ("A new Resource is instantiated from the entry ... with a new ID, in the state registered"). Fix: LIF-008 and LIF-009 survive; fold CON-010's "or anything else happens to its consumer" into LIF-008; delete POOL-010; move POOL-003 (the not-retired case, conflict) beside LIF-009.

1.5 CON-015 against LIF-001, LIF-003, LIF-005, LIF-006 and the definitions of open and close. Every clause of CON-015 exists elsewhere: "Opening is the Conduit's setup and leaves it ready" (LIF-003: "For a Conduit, setup is being opened"); "Closing is its retirement" (LIF-005); "A request to open a closed Conduit is not dispatched and reports conflict" (LIF-006: "A request to set up a retired instance ... reports conflict"); "no state in which a Conduit is suspended" (LIF-001). Fix: delete CON-015.

1.6 ACC-001, ACC-003, ACC-004, ACC-008 (corollaries ACC-016, ACC-023, PLC-012). ACC-001 has "dispatched only if an open Conduit ... lists the operation", ACC-003 "permits exactly the operations it lists", ACC-008 "permitted only if the Conduit lists the operation and every restriction that applies permits it", and ACC-004 "permits nothing" is ACC-003 with an empty list. Fix: ACC-008 survives as the one rule (listed and every restriction permits); ACC-001 keeps "no call, record exists"; fold ACC-004 into ACC-003. ACC-023 and PLC-012 only restate ACC-001 for a new Interface and a shared host.

1.7 ACC-006, ACC-007, ACC-005. ACC-007 ("not all within the bound ... reports refused") is the negative of ACC-006 ("dispatched only if ... bound covers the new Conduit's target and every operation it lists"). ACC-005 is the same rule at a setup step. Fix: merge ACC-007 into ACC-006, keeping "not opened for the part that was within the bound"; ACC-005 keeps only "setup's opener is the actor running setup".

1.8 GRA-003, ACC-005, PLC-013, POOL-013, LIF-008. The sentence "Setup fails if the declared Conduit is required, and reports it as not opened if it is optional" appears verbatim in ACC-005 and PLC-013, and in other words in POOL-013 and LIF-008. All follow from GRA-003 plus GRA-022. Fix: state it once in GRA-003 and delete it from the four observables.

1.9 GRA-022, CON-004, POOL-013, TPL-012, definitions (setup graph). "Opening each declared Conduit is a step of that setup" (CON-004), "Setting up the Resource of each entry is a step of the Pool's setup" (POOL-013) and "Each declared Conduit and each entry is a step of that graph" (GRA-022) are the same statement. Fix: GRA-022 survives. CON-004 keeps only "already open is left as it is; no further Conduit is created"; POOL-013 keeps only ordering and "already ready is left".

1.10 OBJ-002, OBJ-011, SUB-007, FST-004, LIF-007 (observable). OBJ-002 already rejects any document whose "kind and name match an object that already exists, whether ... a template or an instance". OBJ-011 and SUB-007 are its instance and template cases. FST-004 is the same for first-start objects, which LIF-022 says can never be unregistered. Fix: OBJ-002 survives with "a submission only creates"; delete FST-004; reduce OBJ-011 and SUB-007 to the rename and replace operations (see 5.1).

1.11 TPL-003, NAM-018, PKG-010, PKG-011. NAM-018 says "written under the key `implementation`" and TPL-003 says "in the one `implementation` map". PKG-010 ("two implementations are selected for the same operation or runtime") cannot occur in a map keyed by operation, except as a duplicate key already rejected by SCH-012. PKG-011 restates NAM-014 and NAM-018 for a type default and omits the operation-contract default of CTR-002. Fix: TPL-003 survives; delete PKG-010; fold PKG-011's install-time rejection into TPL-004 or CTR-004.

1.12 NAM-006 with PKG-002; NAM-009, NAM-010, NAM-011 with PKG-005, PKG-008. PKG-002 ("version range, or with no version ... rejected") is inside NAM-006. PKG-005 says the instance holds the version and digest "resolved at creation", while NAM-010 says references resolve "through the packages the template recorded, not through any manifest in force at the time of creation". Fix: delete PKG-002; reword PKG-005 to "the instance holds the versions and digests its template recorded" or merge it into NAM-010; fold PKG-008 into NAM-011.

1.13 PLC-007 with PKG-015. The text is identical except "type" against "selected implementation". Fix: one promise, "the instance's type or a selected implementation".

1.14 PLC-005 with POOL-009. POOL-009's first sentence ("an entry names a location ... rejected") is a case of PLC-005 ("A template is registered that names a location"). Fix: POOL-009 keeps only the positive rule about the input destination. Whether that destination is mandatory or the PLC-002/003 default may apply needs a ruling.

1.15 GRA-013 with GRA-019; GRA-016 and GRA-011 with GRA-020; GRA-018 with GRA-017; GRA-021 with TPL-008. Both GRA-013 and GRA-019 say "exactly one of" the four binding forms. GRA-016 ("under `source`") and GRA-011 ("A pointer applies to an input, to configuration and to a step's output") are restated by GRA-020 ("All but `runtime` and `instance` may add a `pointer`"). "A setup or retire graph with `outputs` is rejected" (GRA-021) matches "has no outputs" (TPL-008). Fix: GRA-019 and GRA-020 survive. GRA-013 keeps only its rejection examples or is deleted; GRA-016 keeps only "`from` is rejected"; GRA-011 keeps only "pointer into a handle from a step output".

1.16 IMP-003 with CTR-002; IMP-005 with IMP-004; IMP-002 with OBS-006 and OBS-005; IMP-007 with OPS-012 and CTR-002. CTR-002's observable ("One without `startsModelWork: true` does not start model work") is IMP-003. IMP-005 is IMP-004 applied to setup. "inspected or observed" appears in both IMP-002 and OBS-006. "is not sent again after an unknown outcome" (CTR-002) is OPS-012. Fix: keep IMP-003, IMP-004 and OPS-012; strip the CTR-002 observable to shape only; reduce IMP-005 to "the setup operation is never marked"; drop the model-work clause from OBS-006 and OBS-005.

1.17 ACC-020, ACC-021 with REC-001, OPS-005. "A record of it is kept" for an access decision is the first item of REC-001. "The caller is told the reason" is OPS-005 ("The response and the record carry one of those four reasons"). Fix: delete ACC-020 and ACC-021, or reduce them to "the reason for an access decision is refused or unavailable".

1.18 ACC-025 with ACC-033. Both authorize setting up a Resource "against the Pool that owns" it. Fix: merge into ACC-033; only "create" is unique to ACC-025 (see 2.5).

1.19 FST-002 with ACC-022; LIF-022 with ACC-024. The root's two open Conduits are stated in both FST-002 and ACC-022. LIF-022's "either of `root`'s two Conduits" is covered by ACC-024 ("a Conduit whose consumer is the root Interface"). Fix: ACC-022 keeps only "no bound"; drop the Conduit clause from LIF-022.

1.20 INS-019 with INS-006, INS-007, INS-008, SCH-009; INS-015 with OBJ-013; SCH-011 with INS-005; INS-006 with GRA-019. INS-019 is the summary of the three before it. INS-015 ("accepted without the template declaring it") is OBJ-013's shape rule. "No value is converted" (INS-005) is SCH-011. "no part of a string is substituted" appears in INS-006 and GRA-019. Fix: keep INS-019 and reduce INS-007 and INS-008 to it, or the reverse; delete INS-015; drop the repeated sentences.

1.21 OPS-006 with IFC-009; OPS-007 with IFC-009 and IFC-010 ("no answer, is unknown"). Fix: keep both, with OPS-006 pointing at IFC-009 and IFC-010 (see 2.3).

1.22 CTR-006, TPL-010, the definition of control, OPS-020. "It is defined by a Type contract" (CTR-006), "A template cannot define a control" (TPL-010) and "A control is defined by a type" (definitions) are one statement. Fix: keep CTR-006; drop the sentence from TPL-010.

1.23 TPL-009, LIF-016, LIF-014. "Nothing else is done" (TPL-009) and "must not ... Anything is stopped or released other than what the instance itself holds" (LIF-016) overlap. "The shutdown is confirmed when the graph and every stop succeeded" (TPL-009) overlaps "Shutdown is confirmed when the retire operation succeeded" (LIF-014). Fix: one promise for what retiring does (TPL-009, moved to lifecycle) and one for when shutdown is confirmed (LIF-014). See 2.9.

1.24 Minor repeats. OBJ-010 and LIF-019 both say a reference to a removed object reports it "as missing". CON-018's observable and CTR-005's observable both reject a requirement applied where `appliesTo` does not list. NAM-007's last sentence and NAM-020 both say the core package needs no manifest. Fix: keep the second of each pair.

2. CONTRADICTION

2.1 OPS-008, OPS-009, OPS-013, OPS-014 against REC-001, REC-003, ACC-002. "Record" means two different things. In operations it is one mutable entry per identity: "The target keeps one record for that identity", "in progress, or its outcome", "The outcome becomes succeeded or failed ... The record states the new outcome". In records it is an append-only entry per event: separate records for "an access decision; an invocation is dispatched; an invocation ends", and "must not ... removed or altered by anything other than a purge". ACC-002 also requires "The record of the decision precedes any record of the implementation being invoked". Needs a ruling: (A) records are append-only events, and OPS-008 to OPS-014 are reworded to speak of the invocation's state; or (B) a per-identity entry is a second defined thing with its own name.

2.2 REC-001 against OPS-005, OPS-019 and the definition of record. REC-001 closes its list with "Nothing else is required to be recorded", and the list has no request that ends invalid or conflict. OPS-005 says of every request not dispatched "The response and the record carry one of those four reasons", and OPS-019 says a management request "leaves a record". Fix: add "a request is not dispatched" to REC-001's list, or needs a ruling that invalid and conflict leave no record, with OPS-005 reworded.

2.3 OPS-006, OPS-007 against IFC-007, IFC-010. OPS-006 says "Every result an implementation returns across the implementation interface carries one of the three outcome values". OPS-007 says a result that "states no outcome" is unknown and "The record never states succeeded or failed unless the implementation's result stated it". IFC-007 forbids Conduit's keys to a plain implementation, and IFC-010 records "A completed result that is not an error" as succeeded. Fix: scope OPS-006 and OPS-007 to implementations that carry Conduit's keys and cite IFC-010 for plain ones.

2.4 OPS-009 against OPS-012; OPS-005 against OPS-009; IMP-007. A repeat with the same identity and contents, after an unknown outcome, on an operation that deduplicates, is "not dispatched again" under OPS-009 and "dispatched" under OPS-012. Needs a ruling: (A) a new attempt is that repeat, and OPS-009 gains "except under OPS-012"; or (B) a new attempt is a distinct explicit request. Separately, OPS-005 says a request not dispatched "has no outcome" and one of four reasons, yet OPS-009's repeat is not dispatched and answers "in progress, or its outcome". Fix: say a repeat is not a new request. IMP-007's "Each operation identity is dispatched once per request for it" contradicts OPS-009's "The implementation receives one call for that identity"; reword to "at most once, however many requests".

2.5 Definition of `management`, ACC-025, ACC-033, POOL-002, SUB-002, LIF-009, CON-010. Which target authorizes creating, unregistering and replacing a Resource is stated three ways.
  - `management` is "The Resource whose operations register, replace, rename and unregister objects", but ACC-025 authorizes creating a Resource "against the Pool that owns, or will own" it, and ACC-033 authorizes unregistering it against "the Pool that owns it".
  - A Pool's entries are created with the Pool (POOL-002), and a Pool and its Resources may be submitted together (SUB-002). No Conduit to a Pool that does not yet exist can be open, so ACC-025 cannot be satisfied.
  - ACC-033 lists a Conduit under "inspect, set up, retire or unregister ... `management`" and then says "Opening and closing a Conduit are authorized against `access`". By the definitions, setting up a Conduit is opening it.
  - CON-010 says a replacement Conduit is requested by "an actor that holds the right to open it", while LIF-009 says "authorized as creating that instance would be".
  Needs a ruling: (A) registering and unregistering are always `management`, and a Pool authorizes only set up, retire and inspect of its Resources; or (B) the Pool authorizes creation, with a stated exception for entries and joint submissions. Either way reword ACC-033 to "inspect or unregister a Conduit".

2.6 LIF-022 against LIF-017, LIF-020, PLC-011, and the definition of refused. Three situations get two reasons: unregistering `local` while instances are placed on it (PLC-011 "reports conflict"; LIF-022 "reports refused, whoever makes it"); retiring `system` while it owns Resources (LIF-020 conflict; LIF-022 refused); unregistering any ready first-start object (LIF-017 conflict; LIF-022 refused). LIF-022 and ACC-024 also use refused where the definition ("No open Conduit lists the operation, or a restriction forbids it") does not apply to the root Interface. No promise orders the four reasons; DL9 covers controls only. Needs a ruling: a precedence among reasons, or LIF-022 reworded to one reason with the others excepted.

2.7 PKG-009 against TPL-004. PKG-009 rejects a template when no implementation is selected "by the template or by the type's stated default". TPL-004 accepts "or else the default the operation's own contract states". Fix: delete PKG-009; TPL-004 covers it.

2.8 GRA-010 against TPL-008 and GRA-020. GRA-010 rejects configuration in any "graph that implements an operation", excepts only the setup graph, and says such a graph "may refer only to the operation's declared inputs, to its own steps' outputs and to handles it was given". TPL-008 says "A `retire` graph may refer to the instance's configuration". GRA-020 allows the sources `runtime` and `instance` in every graph. Fix: reword GRA-010 to "`config` is accepted only in a setup or retire graph" and delete its "may refer only" sentence.

2.9 TPL-009 against CON-008, LIF-016, LIF-014. TPL-009 says "Nothing else is done" on retirement; CON-008 and LIF-016 close "the Conduits of which the retired instance was an end". LIF-014 says "Retiring an instance for which no setup was ever dispatched is confirmed at once", while TPL-009 says the "steps of its `retire` graph run, if it has one". LIF-014 confirms on "the retire operation succeeded" and TPL-009 on "the graph and every stop succeeded", and no promise says the stops are part of the retire operation. Fix: reword TPL-009 to "apart from closing its Conduits under CON-008". Needs a ruling on whether a never-set-up instance runs its retire graph.

2.10 REV-007 against REV-008. REV-007 says "A control succeeds ... exactly those settings are stored". REV-008 says "Nothing is stored" when the confirmed settings fail the schema, and does not state the outcome. Needs a ruling: (A) the outcome is unknown, as OPS-007 gives for an output that fails validation, so REV-008 merges into REV-009; or (B) the outcome is succeeded with nothing stored, and REV-007 gains the exception.

2.11 LIF-020 against LIF-021, through LIF-005. LIF-021 leaves the Pool ready "If any Resource's retirement failed or has an unknown outcome". By LIF-005 those Resources are already in the state retired, so LIF-020's guard ("owns a Resource that is not retired") no longer blocks a plain retire of the same Pool. Needs a ruling: the guard is the state retired, or shutdown confirmed, in both.

2.12 OBJ-012 against OBJ-017. OBJ-012: "A template document with any other top-level key is rejected" (other than kind, `metadata`, `spec`). OBJ-017 requires `apiVersion`. Fix: add `apiVersion` to OBJ-012, OBJ-013 and the definitions of template and instance.

2.13 OBJ-001, the definition of family, and SUB-005 against CTR-001 and NAM-004.
  - OBJ-001 rejects any document name not made "only of lowercase letters, digits and hyphens". A contract is "a document with ... `metadata`" whose "name is its export name", such as `local-directory.v1`, which contains a dot.
  - The definition says kind is "the same idea" as family (five), while CTR-001 adds the kinds `Type`, `Capability`, `Operation`, `Requirement`.
  - SUB-005 rejects a submission holding anything "that is not a template, an instance or the project's one manifest", yet a package is "a project that has been published" and holds contracts.
  Fix: scope OBJ-001 to templates and instances and reword the family definition. Needs a ruling on whether a project containing contract documents can be submitted.

2.14 PLC-002, PLC-003, PLC-004 against PLC-008. A location is a Resource instance. A location document that gives no placement gets "that location" (PLC-002) or is "not created" (PLC-004), but PLC-008 says "it is its own placement". Fix: add "other than a location" to PLC-002 to PLC-004.

2.15 CTR-004 against PLC-014. "A type with no requirements on placement writes an empty map" against "an empty list of requirements on placement is accepted". Fix: delete PLC-014 (CTR-004 covers it) or align the word.

2.16 CON-001, CON-002, CON-004, INS-016 against CON-003, CON-016. CON-001: "A Conduit template is instantiated. The resulting Conduit has ... exactly one capability. A declaration that supplies more or fewer ... is rejected". A composite template has `components` and no `capability`, and yields several Conduits with "no instance represents the composition itself". "one Conduit per declaration" (CON-002), "the same number of Conduits" (CON-004) and "the instance and its nested instantiations" (INS-016) all fail for a composite. Fix: reword to "each resulting Conduit" and "one Conduit per declaration, or per component of a composite".

2.17 ACC-013 against the README conformance rule, with CON-021 and PKG-019. README: an engine "that receives a declaration it cannot honor rejects it at registration". ACC-013 accepts a Conduit whose filter the target's implementation does not support, and only at open "reports unavailable ... The Conduit remains registered". PKG-019 says support for a filter "is answered from the manifest, without starting it", and CON-021 rejects the parallel capability case at instantiation. Needs a ruling: reject at instantiation like CON-021, or keep ACC-013 and qualify the README sentence.

2.18 CON-011 against CON-014 and ACC-015. After a close the consumer "reports the capability that Conduit carried as unavailable" (CON-011), yet its requests report refused (CON-014). In ACC-015 a capability that "is unavailable" makes requests "report unavailable". Fix: reword CON-011 to a word that is not one of the four reasons.

2.19 ACC-012 against the definition of right to open, and against ACC-024. ACC-012 dispatches a close when the bound "covers that Conduit's target". The definition bounds "the targets and operations of the Conduits its consumer may open and close". Needs a ruling: target only, or target and operations. ACC-012's "It is dispatched if" also lacks the exception of ACC-024.

2.20 ACC-009 against ACC-017. ACC-009 lists what limits a request ("its own Conduit's operations and filters ... any policy on the consumer and ... on the target") and its observable says it "succeeds ... unless a policy on the consumer or on the target forbids it". ACC-017 adds, for onward requests, "the filters on the Conduit the caller came in through and any policy on the caller". Fix: add "and, for an onward request, the restrictions carried under ACC-017" to ACC-009.

2.21 IFC-001 against PKG-020; IFC-006 and OPS-019 against OPS-011. IFC-001 names the tool "by the operation's export name" for every `command` or `url` implementation, while PKG-020 gives a plain one a `tools` map to another name. IFC-006 says `_meta` "holds ... `conduit.dev/operationIdentity`" and OPS-019 says a management request "carries an operation identity", while OPS-011 defines "A request ... carrying no operation identity". Fix: except plain in IFC-001. Needs a ruling on whether the identity is optional, and what IFC-006 carries when it is absent.

2.22 NAM-017 against ACC-006, PLC-013; NAM-018 against TPL-003. NAM-017: "Whether the Conduit opens depends only on the capability ... being offered ... and on its restrictions being supported". Opening also depends on the right to open (ACC-006) and the shared-host check (PLC-013). NAM-018: "The selection is a qualified reference"; TPL-003 allows "a graph written in place". Fix: reword NAM-017's observable to "Package versions are not compared" alone; add the graph case to NAM-018.

2.23 INS-012 against POOL-009, through GRA-022. INS-012 rejects an input whose destination is "inside its setup graph". GRA-022 makes each entry "a step of that graph". POOL-009 requires that "An entry's placement is the destination of one of the Pool template's inputs". Fix: reword INS-012 to name the keys (`setup`, `retire`, `runtimes`, `controls`, `implementation`, `type`).

2.24 ACC-001 against OPS-016. ACC-001 dispatches a consumer's request "only if an open Conduit ... lists the operation". OPS-016 answers the original caller "even if the caller's Conduit was closed". Needs a ruling: retrieving a result is not an operation, or ACC-001 gains the exception.

2.25 OBS-004 against OBS-005. OBS-004: "must not ... Anything acts to remove the difference without an explicit request". OBS-005 allows "An implementation restores its own runtime, as its type contract allows". Fix: except OBS-005 in OBS-004, or see 3.1.

3. STALE WORDING

3.1 PKG-009, PKG-010, the definition of implementation, OBS-005. "operation or runtime" (PKG-009, PKG-010) and "behind an operation or a runtime" (definitions) treat a runtime as having its own implementation; TPL-006 makes it two operations. OBS-005's "An implementation restores its own runtime" predates the handle held by the instance (TPL-007). It is unclear whether restoring invokes `start` again, which IMP-007 forbids without a request. Fix: delete "or runtime". Needs a ruling on what restoring means.

3.2 SUB-004. "or would change the same object" assumes a submission can change objects; SUB-007 says "A submission only creates". Fix: delete the clause.

3.3 INS-003, INS-015, OBJ-014. "an argument that is neither an input ... nor a standard argument" (INS-003) treats standard arguments as written under `arguments`, which OBJ-013 rejects. OBJ-014 says a nested instantiation has "standard arguments", but POOL-012 forbids `owner`, PLC-005 and POOL-009 forbid a location, and CON-020 forbids a consumer everywhere and a target in a component. Only a declared Conduit's target remains. Fix: reword both.

3.4 PLC-013, ACC-005. Both start "Setup reaches the step that opens ..." and cover only declared Conduits. CON-012 to CON-014 added Conduits opened on their own, and ACC-013 already says "A Conduit is being opened". Fix: generalize PLC-013's "when". The reason an explicit open then reports needs a ruling.

3.5 SCH-002, SCH-003, SCH-008. "rejected when it is accepted into the registry" uses an undefined word and contradicts itself. A schema arrives by install (in a contract) or by registration (in a template). Fix: "rejected when its package is installed or its template registered".

3.6 PKG-003, PKG-005, PKG-007, PKG-008. "A dependency names a version", "depends on packages" and "upgraded" are lockfile-era words. The manifest now lists "each package the project uses" at one exact version, and NAM-011 has no upgrade. Fix: reword to "manifest entry".

3.7 REC-003, FST-003, IMP-002, DL3, DL4. "after every Interface has detached", "an export of it" (export is a package term), "any service login", "a message or a notice is delivered to an Agent", "Waiting for a person", and "Whether a request fails at once" (failed is an outcome of a dispatched request) are not defined or used anywhere else. Fix: remove or define them.

3.8 LIF-007, LIF-020, PLC-011 against ACC-005, ACC-006. "registered" is both a lifecycle state and "not unregistered": "It remains registered, in the state retired" (LIF-007); "a Resource that is still registered" (LIF-020); "The Conduit remains registered" (ACC-005, meaning the state). Fix: use "is not unregistered" or "exists" for the second sense.

3.9 README. "It has two layers" against three `layer` values; "an Agent's lifecycle ... belongs to that type's contract" against LIF-001's one lifecycle; "An implementation claims conformance" where the definitions say engine; "as the owner rules" although the definitions say owner "is not used for a person" (also in schema-profile.md). Fix: reword.

3.10 definitions.md. The row for optional lists "Inputs, steps, declared Conduits and outputs", while OBJ-015 adds "a Pool's entry, a runtime". The `management` row lacks set up, retire and inspect (ACC-033, OPS-019). "grant" is defined and used by no promise. Fix: align the rows.

3.11 PLC-011. "Only the declare-lost operation ... ends a location" uses "ends", which is not a defined verb, and leaves open whether an ordinary retire of a location with instances is dispatched. Fix: reword to "retires".

3.12 NAM-018, NAM-004. NAM-018 keeps `driver` and `handler` alive as named rejections that OBJ-019 already covers. NAM-004 accepts the example `directory.establish`, which reads as an operation name and would fail CTR-001's version-segment rule. Fix: drop both examples.

4. WRONG FILE OR LAYER

4.1 IFC-014 (interface). "The Conduit is not created ... Resolving returns a diagnostic" is a rule on declarations, so a Language-level engine would not apply it. Move to promises/conduits.yaml as `language`.

4.2 GRA-002, GRA-003 (language). "must not ... The step runs" and "Setup has succeeded only if ..." are execution invariants, checked by "No record shows ...". GRA-003 cites LIF-004, a convention. Change to `convention`.

4.3 POOL-003, POOL-010 (language). Both are requests by "An actor" that report conflict or create on request; their general form LIF-009 is `convention`. Change to `convention` and move beside LIF-009.

4.4 CON-004 (language). "Setup runs for an instance ..." is behaviour at setup. Change to `convention`.

4.5 NAM-016, NAM-017 (language). "can be opened to it" and "The Conduit is refused" describe opening. Either restate them against instantiation (CON-021) or change to `convention`.

4.6 PLC-007, PKG-015, PLC-016 (language). PLC-007 and PKG-015 decide on "the facts reported by the instance's location", which is observed, not declared. Needs a ruling: location facts are declared (stay `language`) or observed (`convention`, with a rule for an unreachable location). PLC-016 is a control invocation that "reports invalid"; change to `convention`.

4.7 FST-002, FST-003 (language). "each open, and every first-start object is ready" and "no implementation ... has received a call" are run-time state. Change to `convention`; FST-001, FST-004 and FST-005 stay `language`.

4.8 IMP-003, IMP-004, IMP-005 (language, in conventions/implicit.yaml); LIF-002, LIF-007, LIF-012 (language, in conventions/lifecycle.yaml). A Language-level reader following README's reading order meets them late. Move IMP-003 to contracts.yaml and IMP-004 and IMP-005 to graphs.yaml. LIF-012's observable ("not dispatched and reports invalid") is a convention.

4.9 PKG-016 (convention, in packages.yaml). It is an obligation on an implementation's behaviour across instances; move to interface/implementation.yaml beside IFC-017.

4.10 TPL-009 (convention, in templates.yaml). Move to conventions/lifecycle.yaml with LIF-014 and LIF-016 (see 1.23).

5. SPLITS

5.1 OBJ-011, SUB-007. Each rejects a document and also defines an operation: "a rename operation that names the instance's current revision and changes only its name"; "a separate replace operation that names the template's current revision". Split the rename and replace operations into their own promises.

5.2 LIF-014. Four obligations: the three-valued status; when it is confirmed; "Retiring an instance for which no setup was ever dispatched is confirmed at once"; "can be retired again".

5.3 PLC-011. Unregistering a location with instances reports conflict; and separately what declare lost does ("they are retired, and the record states ...").

5.4 POOL-013. Entries are steps; the ordering; "already ready is left as it is"; the required and optional result.

5.5 TPL-010. The shape of `controls`; and availability ("A control the map does not list is not available ... no `controls` map makes every control ... available").

5.6 CTR-002, CTR-003, CTR-005. Each mixes a shape with a rule in the observable: defaults for absent marks; rejection of a reserved operation; rejection outside `appliesTo`; rejection of a cyclic composition.

5.7 IFC-011. Four obligations: opaque string; engine passes it unchanged; "Holding one permits nothing"; "the implementation checks it on every call".

5.8 IFC-012. The tool set equals the operations of open Conduits; and the tool naming rule.

5.9 ACC-006, ACC-017, LIF-020. ACC-006: open is an operation of `access`; needs a right; "Holding access to the target does not confer that right". ACC-017: the onward request carries identity; its effects are limited. LIF-020: a retire guard and an unregister guard.

5.10 OBJ-019, GRA-019, POOL-001, PLC-008, OBS-006, IMP-006, REC-004, LIF-017. Each joins two independent obligations: undefined key and camelCase; binding forms and no substitution; one owner and never changed; location shape and reports facts; three effects of inspecting; arranged by a marked operation and each start checked; root only and a purge record; unregister condition and "names the object's current revision".

JUDGEMENT

I count about 70 findings. Roughly 25 are real contradictions or collisions an implementer would have to guess at (section 2, plus 3.1 and 4.1, 4.2, 4.6). About 24 are duplication groups that are safe today but will drift (section 1). The remaining 20 or so are cosmetic: stale words, README and definition rows, and splits. Most real problems come from earlier batches not being rewritten after ruling 28 to 31 changes: packages against templates, graphs against the retire graph, operations against the plain interface, access against the lifecycle and Pool changes.

The three that matter most:
(1) 2.1 and 2.2: "record" is a mutable per-identity entry in operations.yaml and an append-only event log with a closed list in records.yaml, so neither convention can be tested as written.
(2) 2.5: whether creating, unregistering or replacing a Resource is authorized against `management` or its Pool, with the impossible case of a Pool's own entries and the open/set-up clash inside ACC-033. This is the product's core claim.
(3) 2.3 with 2.4: OPS-006 and OPS-007 flatly contradict the plain-server table of IFC-010, and OPS-009 and OPS-012 give opposite answers for a repeat after an unknown outcome.

Close behind is 2.6: no precedence among refused, invalid, conflict and unavailable, with LIF-022 colliding with three conflict promises.
```
