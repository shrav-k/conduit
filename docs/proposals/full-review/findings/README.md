# The full review's findings: index

Not binding. Part of [the full review](../README.md) of 2026-11-14, at commit `2eb8f258`. Each row is one finding or item of a reader or an analyst, with the verdict of the adversarial verifier that tested it; the full text is in the part the ID links to. Where a verifier corrected a finding, the correction governs. The second column is the name the reviewers' own notes use.


## L01-access

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-001](L01-access.md#fr-001) | L01/F1 | ambiguity | unsafe | confirmed, high | coordinator | ACC-029, ACC-030, ACC-006, ACC-012, ACC-053 | A Pool named in a bound's `targets`: what it covers, and whether a bound naming it in `pools` is within it |
| [FR-002](L01-access.md#fr-002) | L01/F2 | contradiction |  | confirmed, medium | coordinator | ACC-056, ACC-054, SEC-008, ACC-024, ACC-029, ACC-039, ACC-034, IFC-050, OPS-023 | ACC-056 requires the specific refusing rule at tier 1 and also that tier 1 not depend on the named object's existence, owner or state |
| [FR-003](L01-access.md#fr-003) | L01/F3 | contradiction |  | weakened, medium | coordinator | OPS-029, ACC-006, ACC-012, ACC-053, ACC-056, OPS-036, LIF-013 | A bounded holder of `open.v1` or `close.v1` learns from the reason alone whether any Conduit name exists |
| [FR-004](L01-access.md#fr-004) | L01/F4 | ambiguity |  | confirmed, medium | coordinator | ACC-056, ACC-001, ACC-011, ACC-006, ACC-033, IFC-012, IFC-013, IFC-050, OPS-009, OPS-032, OPS-034, CTR-013, REC-001 | Which promise is "the refusing promise" is not determined for common refusals |
| [FR-005](L01-access.md#fr-005) | L01/F5 | ambiguity |  | confirmed, medium | owner | ACC-016, ACC-017, ACC-034, ACC-039, IMP-008, IFC-020, IFC-021, REC-002 | Whether a request an Agent makes during a turn serves that turn is left to the Agent's implementation |
| [FR-006](L01-access.md#fr-006) | L01/F6 | defect |  | confirmed, medium | coordinator | IMP-001, IMP-002, IMP-003, REC-001, TPL-017, PRV-010 | IMP-001 and IMP-002 are checked against a record that no promise requires, and the cases that matter are an Agent package's |
| [FR-007](L01-access.md#fr-007) | L01/F7 | contradiction |  | weakened, low | coordinator | OBS-004, PLC-024, PRV-010, PRV-026, LIF-011, OPS-014 | OBS-004 forbids anything acting on an observed difference, and three promises have something act |
| [FR-008](L01-access.md#fr-008) | L01/F8 | needless-complexity |  | confirmed, low | coordinator | ACC-018, ACC-015, ACC-017, ACC-033, ACC-057, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003, IFC-018, IFC-019 | Access promises decide three cases that their own cited promises make unreachable |
| [FR-009](L01-access.md#fr-009) | L01/F9 | needless-complexity |  | confirmed, medium | coordinator | ACC-030, ACC-008, ACC-011, ACC-015, ACC-018, ACC-019, ACC-039, IMP-007, OPS-009, OPS-024, OPS-034, CON-008 | Observables restate other promises and carry commentary, and the copies have begun to differ |

## L02-lifecycle

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-010](L02-lifecycle.md#fr-010) | L02/F1 | defect | unsafe | confirmed, high | owner | REV-010, REV-013, REV-009, ACC-055, ACC-033, OPS-031 | Reconciling an unknown control as having taken effect stores its old values now, on the reconciler's word, without the control's Conduit and over later changes |
| [FR-011](L02-lifecycle.md#fr-011) | L02/F2 | defect | unsafe | confirmed, high | coordinator | LIF-028, LIF-041, ACC-006, ACC-055, IMP-007 | Recording an unknown open as having occurred opens the Conduit with none of the checks an open makes at that time, a matching tombstone included |
| [FR-012](L02-lifecycle.md#fr-012) | L02/F3 | gap | unsafe | weakened, medium | coordinator | REV-001, REV-002, REV-005, LIF-037, LIF-040, SUB-009, PLC-017, PLC-026 | A revision is unique only within one object, and replace.v1 and declare-lost.v1 take no expectedId, so a stale request can pass against a later object of the same name |
| [FR-013](L02-lifecycle.md#fr-013) | L02/F4 | contradiction |  | confirmed, medium | coordinator | LIF-019, LIF-035, PRV-029 | LIF-019 forbids erasing backing files when an instance is removed; LIF-035 and PRV-029 erase its state directory; LIF-019's two terms are defined nowhere |
| [FR-014](L02-lifecycle.md#fr-014) | L02/F5 | defect |  | confirmed, medium | coordinator | LIF-036, LIF-017, IFC-050, IFC-025 | An unregister succeeds without saying that its last stop failed, ended unknown or was never sent |
| [FR-015](L02-lifecycle.md#fr-015) | L02/F6 | gap |  | weakened, medium | coordinator | LIF-018, LIF-028, LIF-035, LIF-036, LIF-017 | Two requests assert that a shutdown is settled; only unregistering asks the runtime provider for a last stop |
| [FR-016](L02-lifecycle.md#fr-016) | L02/F7 | defect |  | confirmed, medium | coordinator | REV-014, REV-012, LIF-013, IFC-050, OPS-027 | REV-014's conflict need not name the invocation in the way, nothing shows a setup, retirement or change in progress, and only its requester can end it |
| [FR-017](L02-lifecycle.md#fr-017) | L02/F8 | gap |  | weakened, medium | owner | LIF-013, LIF-028, LIF-004, OPS-022, OPS-013, PND-006 | A setup whose hook fails after its Conduits opened ends unknown, stays in `unreconciled` after a later setup succeeds, and has no true answer to reconcile |
| [FR-018](L02-lifecycle.md#fr-018) | L02/F9 | contradiction |  | weakened, medium | coordinator | LIF-001, ACC-011, LIF-040, LIF-041, ACC-053 | The stated way to narrow access, close and then open a narrower Conduit, reports conflict under LIF-041 until a lift |
| [FR-019](L02-lifecycle.md#fr-019) | L02/F10 | needless-complexity |  | weakened, medium | coordinator | LIF-013, LIF-009, LIF-036, REV-006, REV-010, REV-014 | LIF-013 states the output contract of inspect.v1 in its observable; LIF-009, LIF-036, REV-010 and REV-014 are built the same way, and the core package states the same rules again |

## L03-operations

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-020](L03-operations.md#fr-020) | L03/F1 | contradiction |  | confirmed, high | coordinator | OPS-009, OPS-023, OPS-029, OPS-030, OPS-034, REV-002, REV-014, ACC-012, ACC-033, LIF-037 | A repeat that is answered without a dispatch has no place in the order of checks, so the state its own invocation changed turns it away |
| [FR-021](L03-operations.md#fr-021) | L03/F2 | needless-complexity |  | confirmed, medium | owner | OPS-007, OPS-012, OPS-014, OPS-024, OPS-027, OPS-034, IFC-010, IFC-046, LIF-013 | An operation that states no effect still ends unknown when no result arrives, and then waits for a reconciliation that has nothing to check |
| [FR-022](L03-operations.md#fr-022) | L03/F3 | contradiction | unsafe | confirmed, medium | coordinator | OPS-014, OPS-018, OPS-024, PRV-009, PRV-010, PRV-025 | A report of `stopped` fences an attempt although a copy of its dispatch can still be waiting before the run |
| [FR-023](L03-operations.md#fr-023) | L03/F4 | gap | unsafe | weakened, medium | coordinator | OPS-013, OPS-014, OPS-030, OPS-008, OPS-024, REC-001, REC-004, LIF-013, DL2 | The actor who must reconcile is not shown what the engine knows: a late answer, whether the attempt is fenced, and after a purge even the operation |
| [FR-024](L03-operations.md#fr-024) | L03/F5 | gap |  | confirmed, medium | owner | OPS-027, OPS-014, OPS-021, OPS-022, OPS-030 | Passing a cancellation on over Streamable HTTP closes the only channel that could fence the attempt |
| [FR-025](L03-operations.md#fr-025) | L03/F6 | contradiction |  | confirmed, medium | coordinator | OPS-027, OPS-013, OPS-014 | OPS-027 says both that a cancelled invocation ends when the cancellation is sent and that a later answer can make it failed |
| [FR-026](L03-operations.md#fr-026) | L03/F7 | ambiguity |  | confirmed, medium | coordinator | OPS-022, CON-004, POOL-013, LIF-004, LIF-013 | OPS-022 gives a further setup that fails two outcomes: failed by the parts it dispatched, unknown by a part counted from before |
| [FR-027](L03-operations.md#fr-027) | L03/F8 | gap | unsafe | weakened, medium | owner | REC-001, REC-002, LIF-013, CTR-004, ACC-033 | Inputs and outputs that a target's type keeps in records are returned to whoever may inspect the caller |
| [FR-028](L03-operations.md#fr-028) | L03/F9 | gap |  | confirmed, medium | coordinator | REC-001, REC-002, OPS-014, LIF-013, LIF-005 | A record of a management verb or of a change has no stated member for the object it acted on, and the record of a reconciliation none for what OPS-014 says it states |
| [FR-029](L03-operations.md#fr-029) | L03/F10 | gap |  | weakened, medium | owner | REC-001, REC-002, REC-004, IFC-032, DL8, DL10 | Every readable request that establishes no instance must leave a record that no portable read returns, and REC-001 has no conforming behavior once a record cannot be written |
| [FR-030](L03-operations.md#fr-030) | L03/F11 | needless-complexity |  | weakened, medium | coordinator | REC-001, REC-003, REC-006, LIF-013, OPS-036 | The only portable read of records returns every record that names the object, each time, with no selection |
| [FR-031](L03-operations.md#fr-031) | L03/F12 | ambiguity |  | weakened, low | coordinator | OPS-005, OPS-007, OPS-025, IFC-009, IFC-026 | A result that gives a reason outside IFC-026's form is read neither as that reason nor as unknown by any promise |
| [FR-032](L03-operations.md#fr-032) | L03/F13 | needless-complexity |  | confirmed, medium | coordinator | OPS-005, OPS-006, OPS-007, OPS-009, OPS-014, OPS-024, OPS-034, REC-002, IFC-009, IFC-010 | The obligations of this slice sit in `observable` fields hundreds of words long, restated across promises and layers, and the restatements already differ |

## L04-interface

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-033](L04-interface.md#fr-033) | L04/F1 | contradiction | unsafe | confirmed, high | coordinator | IFC-046, IFC-003, IFC-010, IFC-007, OPS-018, IMP-007 | IFC-046 resends an effectful call after a revision error, without the safety condition it applies to -32020 |
| [FR-034](L04-interface.md#fr-034) | L04/F2 | contradiction |  | weakened, medium | coordinator | IFC-033, IFC-012, NAM-026, TPL-013, ACC-001, ACC-003, ACC-056 | An operation that the Conduit lists and the target cannot serve reports refused, unavailable or invalid, depending on which promise is read |
| [FR-035](L04-interface.md#fr-035) | L04/F3 | gap |  | confirmed, medium | coordinator | IFC-050, IFC-025, IFC-009, PND-010, PND-013 | The cause that a Conduit-aware implementation gives for a failed or unknown outcome has no required path to the caller |
| [FR-036](L04-interface.md#fr-036) | L04/F4 | gap |  | confirmed, medium | coordinator | IFC-010, IFC-017, IFC-041, OPS-007, PRV-025, SEC-013 | No promise decides what a call reports when no JSON-RPC answer comes back: a refused connection, a failed certificate, an HTTP status or a redirect |
| [FR-037](L04-interface.md#fr-037) | L04/F5 | ambiguity |  | confirmed, medium | coordinator | IFC-001, IFC-037, IFC-046, IFC-010, IFC-017, PKG-020 | Whether the engine reads an implementation's tools/list over stdio, and whether it reads it again, is open, so a missing tool is unavailable under one runtime provider and unknown under another |
| [FR-038](L04-interface.md#fr-038) | L04/F6 | contradiction |  | confirmed, medium | coordinator | IFC-012, IFC-013, IFC-050, ACC-001, ACC-011, ACC-056, REC-001 | IFC-012 and IFC-013 name different deciding promises for a call to a closed Conduit's tool, and the ruleId would then disclose what IFC-012 says tier 1 hides |
| [FR-039](L04-interface.md#fr-039) | L04/F7 | defect |  | confirmed, medium | coordinator | IFC-049, PND-010, PND-013, IFC-022, SEC-013 | IFC-049 binds requestState to a "session", a term no definition gives and that the baseline transport is said not to have |
| [FR-040](L04-interface.md#fr-040) | L04/F8 | ambiguity |  | confirmed, low | coordinator | IFC-049, IFC-036, PND-010, OPS-005, REC-001 | A sign-in.v1 request without the URL capability is answered -32021, and no promise says which reason it reports or whether it is recorded |
| [FR-041](L04-interface.md#fr-041) | L04/F9 | gap |  | confirmed, medium | coordinator | IFC-023, IFC-020, IFC-036, IFC-049, PND-013, OPS-011 | A consumer's request with a Conduit key that is not defined for it, or with a malformed value such as an empty idempotency key, is decided by no promise |
| [FR-042](L04-interface.md#fr-042) | L04/F10 | defect |  | confirmed, low | coordinator | IFC-033, CTR-012, SUB-005, PKG-023 | A rule that rejects documents sits in the observable of an interface promise, so the Language level does not include it |
| [FR-043](L04-interface.md#fr-043) | L04/F11 | needless-complexity |  | confirmed, medium | owner | IFC-010, IFC-016, IFC-045, OPS-007, OPS-012, OPS-013, PND-005 | An operation that has no effect is given the outcome unknown, which puts reads on the reconciliation list and blocks their repeats |
| [FR-044](L04-interface.md#fr-044) | L04/F12 | defect |  | weakened, medium | coordinator | IFC-001, IFC-003, IFC-011, IFC-016, IFC-023, IFC-025, IFC-033, IFC-041, IFC-049 | Most of the file's obligations, several wire keys and whole rules live in `observable` fields, so what binds, at which level and for whom cannot be told |
| [FR-045](L04-interface.md#fr-045) | L04/F13 | needless-complexity |  | confirmed, low | owner | IFC-011, CTR-002, IFC-033, IFC-010 | The handle concept asks nothing of the engine that a string does not, and no shipped contract uses it |
| [FR-046](L04-interface.md#fr-046) | L04/F14 | gap |  | confirmed, low | coordinator | IFC-001, IFC-025, IFC-038, OPS-027 | Whether the engine passes an implementation's progress and logging notifications on to the consumer that asked is undecided |

## L05-providers

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-047](L05-providers.md#fr-047) | L05/F1 | contradiction | unsafe | confirmed, high | coordinator | PRV-027, PRV-008, PRV-030, PLC-026, PLC-027, ENR-006, LIF-024, OPS-014 | A stop or inspection for an instance on a retired node names its machine by hostname alone, and another machine may by then hold that hostname |
| [FR-048](L05-providers.md#fr-048) | L05/F2 | gap | unsafe | weakened, medium | coordinator | ENR-005, ENR-006, ENR-007 | Nothing bounds the tries at the typed fingerprint, whose alphabet the provider chooses |
| [FR-049](L05-providers.md#fr-049) | L05/F3 | defect |  | confirmed, medium | coordinator | PRV-015, PLC-002, PLC-007, PLC-023, PRV-003, ENR-010 | An instance that can only be placed on local is still put to the registered machine provider, which need not know the engine's machine |
| [FR-050](L05-providers.md#fr-050) | L05/F4 | defect |  | confirmed, medium | coordinator | PRV-015, ENR-010, ENR-012, PLC-026, PLC-007 | place.v1 obliges the provider to choose by facts that only the engine holds, and its 'none' carries no reason |
| [FR-051](L05-providers.md#fr-051) | L05/F5 | gap |  | weakened, medium | coordinator | PLC-023, ENR-004, PRV-014, PRV-015, ENR-008 | No promise decides a machine reported pending that a node already represents |
| [FR-052](L05-providers.md#fr-052) | L05/F6 | gap |  | weakened, medium | coordinator | ENR-007, ENR-004, ENR-006, ENR-008, LIF-028, OPS-014, OPS-012 | An admission whose accept call ends unknown cannot be finished, and a machine on the provider's network without a node is shown nowhere |
| [FR-053](L05-providers.md#fr-053) | L05/F7 | contradiction |  | confirmed, medium | owner | PRV-013, PLC-026, PLC-023, ENR-006, PLC-011, PLC-017, PRV-018 | Replacing the machine provider strands every other node for good; the text says 'until its machine is enrolled again' |
| [FR-054](L05-providers.md#fr-054) | L05/F8 | gap |  | confirmed, medium | coordinator | PRV-011, PRV-022, PRV-029, PRV-021, PRV-008, LIF-035, LIF-036 | A runtime provider is never told that an instance is gone, and nothing bounds how long it keeps the run's secrets, identity and state |
| [FR-055](L05-providers.md#fr-055) | L05/F9 | ambiguity |  | confirmed, low | coordinator | PRV-029, CTR-004, PRV-007, PRV-019 | A Type's need of a state directory has no key, and the text does not say who fails the setup |
| [FR-056](L05-providers.md#fr-056) | L05/F10 | defect |  | confirmed, low | coordinator | ENR-005, PRV-002, PRV-013, PLC-018 | Layers: ENR-005 is marked language, and PRV-002 and PRV-013 decide a document's validity in the convention layer |
| [FR-057](L05-providers.md#fr-057) | L05/F11 | defect |  | weakened, low | coordinator | PRV-007, PRV-015, PRV-014, ENR-001, ENR-002, ENR-006, PRV-021 | Promises in this slice state several obligations for two parties, and rules that bind stand in `observable` |
| [FR-058](L05-providers.md#fr-058) | L05/F12 | ambiguity |  | confirmed, medium | coordinator | ENR-008, OPS-003, OPS-004, OPS-022 | refuse-machine.v1 has no stated outcome when the enrollment closes and a removal does not succeed |
| [FR-059](L05-providers.md#fr-059) | L05/F13 | ambiguity |  | confirmed, low | coordinator | PLC-026, LIF-041, OBJ-002, ENR-006, ENR-007 | The name a new node takes ignores tombstones and templates |
| [FR-060](L05-providers.md#fr-060) | L05/F14 | gap |  | weakened, low | coordinator | PRV-026, PRV-005, PRV-002, PRV-010, IFC-017, OBS-004 | A provider's own server that ends on its own, while the engine runs on, is covered by no promise |
| [FR-061](L05-providers.md#fr-061) | L05/F15 | needless-complexity |  | weakened, low | owner | ENR-002, ENR-006, ENR-010, PRV-014, ENR-001 | The enrollment's labels go through the provider and back, though the engine holds them and fixes them itself |

## L06-secrets-pending

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-062](L06-secrets-pending.md#fr-062) | L06/F1 | ambiguity | unsafe | confirmed, high | coordinator | PND-005, PND-003, PND-002, PND-010, PND-011 | Whether a sign-in action's expiry lifts the hold on model work is undecided, and one reading lets an Agent work under an unchecked account |
| [FR-063](L06-secrets-pending.md#fr-063) | L06/F2 | gap | unsafe | confirmed, high | coordinator | PND-011, PND-005, PND-010, CTR-013 | The wrong-account hold covers only model work, so a Resource that keeps a shared login goes on serving under the wrong account |
| [FR-064](L06-secrets-pending.md#fr-064) | L06/F3 | gap | unsafe | weakened, medium | owner | PND-004, PND-008, IFC-049, ENR-003, ENR-004, ENR-005 | "The engine's own panel" is the only place for codes, fingerprints and secret-bearing join actions, and no promise says what it is or who may see it |
| [FR-065](L06-secrets-pending.md#fr-065) | L06/F4 | contradiction | unsafe | weakened, medium | owner | SEC-013, SEC-023, ACC-011, CON-008 | Closing a secret's Conduit, or retiring the secret Resource, does not stop the engine's own use of the value on a lasting connection until the engine happens to restart |
| [FR-066](L06-secrets-pending.md#fr-066) | L06/F5 | gap |  | confirmed, medium | coordinator | PND-005, PND-004, IFC-049, CTR-013 | A device login's user code has no path from the sign-in hook to the engine |
| [FR-067](L06-secrets-pending.md#fr-067) | L06/F6 | gap |  | confirmed, medium | coordinator | PND-005, IFC-016 | PND-005's third trigger, a contract that "reports a missing or lapsed login", has no form an engine can observe |
| [FR-068](L06-secrets-pending.md#fr-068) | L06/F7 | ambiguity |  | confirmed, medium | coordinator | PND-010, PND-011 | sign-in.v1 that starts from another account cannot tell that account from a new wrong sign-in |
| [FR-069](L06-secrets-pending.md#fr-069) | L06/F8 | contradiction |  | weakened, low | coordinator | SEC-018, SEC-014, SEC-017, PRV-010, PRV-003 | On the engine's own runner, a restart with the run's values cannot hold together with the rule that the process is the one place the value is kept |
| [FR-070](L06-secrets-pending.md#fr-070) | L06/F9 | defect |  | weakened, low | coordinator | SEC-024, SEC-021 | SEC-024 puts a rule that the documents alone answer in the convention layer, so the Language level accepts a secret bound for a non-loopback http URL |
| [FR-071](L06-secrets-pending.md#fr-071) | L06/F10 | defect |  | weakened, low | coordinator | SEC-009, SEC-010, SEC-013, SEC-021, SEC-008, PND-009, PND-010, TPL-019 | Three kinds of document or request that can never be honored are accepted and fail only later, at setup or at the next read |
| [FR-072](L06-secrets-pending.md#fr-072) | L06/F11 | needless-complexity |  | weakened, low | owner | SEC-014, PRV-029, LIF-035, LIF-041, ACC-057 | Since batch 1, rotating a revoked credential by replacing the instance also destroys an Agent's home and login and needs root and a lift |
| [FR-073](L06-secrets-pending.md#fr-073) | L06/F12 | gap |  | confirmed, low | owner | FST-002, FST-005, FLEET-005, NAM-009 | No promise decides what the first-start objects and root's three Conduits hold after the engine is replaced by one with another version of the core package |
| [FR-074](L06-secrets-pending.md#fr-074) | L06/F13 | ambiguity |  | confirmed, low | coordinator | PND-002, PND-007, PND-009 | PND-002 and PND-007 give two different sets of Interfaces at which a notice is raised |
| [FR-075](L06-secrets-pending.md#fr-075) | L06/F14 | ambiguity |  | confirmed, low | coordinator | PND-010, IFC-049, IFC-036, REC-001 | A sign-in.v1 request without the URL capability has no stated ending and no stated moment at which it is checked |
| [FR-076](L06-secrets-pending.md#fr-076) | L06/F15 | defect |  | weakened, low | coordinator | PND-013, PND-001, PND-004, PND-010 | PND-013 holds the duties of three parties in one convention promise, and those of the Interface's client fall under no conformance level |
| [FR-077](L06-secrets-pending.md#fr-077) | L06/F16 | ambiguity |  | confirmed, medium | coordinator | SEC-001, OBJ-021, MAN-017 | SEC-001 does not say whether additionalProperties, patternProperties and prefixItems are among the applicators under which secret-reference.v1 is rejected |
| [FR-078](L06-secrets-pending.md#fr-078) | L06/F17 | needless-complexity |  | weakened, low | owner | SEC-001, SEC-010, SEC-013, MAN-017, SEC-007 | A secret reference is accepted in places from which nothing can ever read it, with no diagnostic |
| [FR-079](L06-secrets-pending.md#fr-079) | L06/F18 | ambiguity |  | confirmed, low | coordinator | PND-003, PND-009, SEC-011 | "Anything touching a secret remains root's" names no test, and can be read against SEC-011 |

## L07-objects-naming

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-080](L07-objects-naming.md#fr-080) | L07/F1 | gap | unsafe | confirmed, high | coordinator | SUB-010, IFC-022, IFC-032, ACC-033, ACC-057 | A directory registration is attributed to an actor the engine names, with no promise that the one handing it over proves to be that actor |
| [FR-081](L07-objects-naming.md#fr-081) | L07/F2 | gap |  | confirmed, medium | owner | NAM-009, NAM-010, NAM-011, NAM-020, NAM-023, PKG-005 | No promise says what objects that keep an earlier core package version resolve through once the engine's built-in core package has another version |
| [FR-082](L07-objects-naming.md#fr-082) | L07/F3 | ambiguity |  | confirmed, high | coordinator | OBJ-021, SEC-001, SCH-008, SCH-009 | Which schema keywords carry a reference schema, or a default, to a place is stated only for properties, items and anyOf/oneOf; the checker and the engine each chose differently |
| [FR-083](L07-objects-naming.md#fr-083) | L07/F4 | ambiguity |  | confirmed, medium | coordinator | NAM-024, NAM-025, PKG-023 | NAM-024's install-time rejection does not say which differences it tests; the text reads as a full compatibility diff and the first build compares defaults only |
| [FR-084](L07-objects-naming.md#fr-084) | L07/F5 | gap |  | confirmed, medium | coordinator | SCH-001, SCH-010, SCH-015, SCH-019, OBJ-021, SUB-011 | No promise rejects a schema whose evaluation or default filling never ends |
| [FR-085](L07-objects-naming.md#fr-085) | L07/F6 | gap |  | confirmed, medium | coordinator | SUB-005, SCH-006, SCH-007, SCH-012, SCH-020, SCH-021, OPS-009, OPS-010, OPS-033 | The value model stops at files and documents: a duplicate key or an out-of-range number in a request's arguments is decided by no promise |
| [FR-086](L07-objects-naming.md#fr-086) | L07/F7 | ambiguity |  | confirmed, medium | coordinator | SUB-009, OBJ-018, INS-024, REV-003, REV-004 | 'Equal to the template as stored' has no defined stored form, so whether a replace advances the revision differs between builders |
| [FR-087](L07-objects-naming.md#fr-087) | L07/F8 | gap |  | confirmed, low | coordinator | OBJ-003, LIF-041, SUB-004, SUB-001, NAM-027 | A generated name need only differ from existing objects, not from tombstones or from names in the same submission |
| [FR-088](L07-objects-naming.md#fr-088) | L07/F9 | defect |  | weakened, low | coordinator | SCH-009, SUB-005, NAM-012, NAM-022, NAM-024, OBJ-009, SUB-010 | Rules and rejections stand only in observables, and several promises hold many obligations, against the form the specification sets for itself |
| [FR-089](L07-objects-naming.md#fr-089) | L07/F10 | contradiction |  | known, low | coordinator | SUB-005, OBJ-001, OBJ-012, OBJ-018, OBJ-020, IFC-050 | Every schema rejection carries ruleId SUB-005, while the object promises say the diagnostic names the rule broken |
| [FR-090](L07-objects-naming.md#fr-090) | L07/F11 | needless-complexity |  | weakened, low | owner | NAM-022, INS-025, NAM-028, OBJ-008 | The exception that makes `#name` as `from` mean the bare name in a submitted project does not deliver what it is for |

## L08-templates-conduits

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-091](L08-templates-conduits.md#fr-091) | L08/F1 | ambiguity | unsafe | weakened, low | coordinator | FLEET-003, ACC-033, POOL-004, CON-024 | "Live existing" owner is undefined in FLEET-003 and ACC-033 rows 1 and 1A; one sound reading drops the owner's Conduit |
| [FR-092](L08-templates-conduits.md#fr-092) | L08/F2 | contradiction |  | confirmed, medium | coordinator | FLEET-006, FLEET-005, OBJ-023, OBJ-025, OPS-023, IFC-014 | A Pool and a Fleet of one name: FLEET-006 says conflict; OBJ-023, OBJ-025, FLEET-005 and OPS-023 give invalid |
| [FR-093](L08-templates-conduits.md#fr-093) | L08/F3 | defect |  | weakened, medium | owner | CON-004, LIF-008, LIF-009, TPL-012, OPS-022, PRV-007, PRV-026, CON-008 | Once the target of a required declared Conduit is retired, its consumer can never complete another setup, so its run can never be asked for again |
| [FR-094](L08-templates-conduits.md#fr-094) | L08/F4 | gap |  | confirmed, medium | coordinator | CON-004, POOL-013, OPS-022, IFC-025, IFC-050, LIF-004 | A setup that succeeds past a failed optional part has nowhere to report it |
| [FR-095](L08-templates-conduits.md#fr-095) | L08/F5 | ambiguity |  | confirmed, medium | coordinator | TPL-012, CON-004, POOL-013, OPS-022, LIF-027, LIF-021 | After one part of a setup fails, whether the other parts at its position are still begun is not decided |
| [FR-096](L08-templates-conduits.md#fr-096) | L08/F6 | gap |  | weakened, low | coordinator | CON-016, CON-017, INS-008, INS-012, INS-023, IFC-014 | A Conduit's `operations` can resolve to absent, or list one operation twice, and no promise decides either |
| [FR-097](L08-templates-conduits.md#fr-097) | L08/F7 | contradiction |  | confirmed, low | coordinator | INS-016, INS-017, POOL-016, PLC-002, PRV-015, PRV-026 | INS-016 and INS-017 state instantiation as pure, while POOL-016 and PRV-015 place by a provider call within it |
| [FR-098](L08-templates-conduits.md#fr-098) | L08/F8 | defect |  | confirmed, low | coordinator | FLEET-005, INS-016, SUB-006, IFC-014 | FLEET-005 is layered `convention` but decides which objects a registration makes and when it is rejected, and states five things |
| [FR-099](L08-templates-conduits.md#fr-099) | L08/F9 | defect |  | confirmed, low | coordinator | CON-031, CON-008, LIF-035, OPS-029, OPS-023 | CON-031's `when` and `then` give conflict for a retired target; its own observable and CON-008 give invalid |

## L09-packages

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-100](L09-packages.md#fr-100) | L09/F1 | gap | unsafe | confirmed, high | owner | PKG-026, PKG-023, PKG-024, NAM-012, PRV-017, PRV-003, MAN-013, MAN-014, IFC-050 | install.v1 reads any place its caller names, and PKG-026 tells the caller what it found |
| [FR-101](L09-packages.md#fr-101) | L09/F2 | gap |  | confirmed, medium | owner | CTR-013, PND-013, NAM-011, PKG-017, SUB-008 | CTR-013's root confirmation is keyed by an address that anyone who may install can claim |
| [FR-102](L09-packages.md#fr-102) | L09/F3 | contradiction |  | confirmed, medium | coordinator | SCT-011, OBJ-013, FLEET-002, SUB-005, SUB-010, SCT-003 | SCT-011 supplies an owner to framed Agents, which OBJ-013, FLEET-002 and instance.v1 reject without one |
| [FR-103](L09-packages.md#fr-103) | L09/F4 | defect |  | confirmed, medium | coordinator | CTR-013, PKG-027, PKG-023, PKG-025, OPS-023 | CTR-013 holds a convention with three reasons and a root-only input in the `observable` of a language promise, and clashes with PKG-027 |
| [FR-104](L09-packages.md#fr-104) | L09/F5 | gap |  | confirmed, medium | coordinator | MAN-014, MAN-013, PKG-022, PKG-004, SUB-011 | Which bytes of an archive are the manifest and the documents is left to each engine |
| [FR-105](L09-packages.md#fr-105) | L09/F6 | gap |  | confirmed, low | coordinator | CTR-013, CTR-014, CTR-006, PND-005, PND-011, PND-013, PRV-016 | A control may change the `signIn` account place; CTR-014 forbids the same for `toolsDirectory` |
| [FR-106](L09-packages.md#fr-106) | L09/F7 | gap |  | known, medium | coordinator | PKG-032, PKG-024, PRV-017, PRV-003 | Nothing bounds the engine's own fetch, and PKG-032 makes every later install of that version wait on it |
| [FR-107](L09-packages.md#fr-107) | L09/F8 | ambiguity |  | confirmed, medium | coordinator | CTR-006, REV-007, REV-015, MAN-017 | CTR-006 does not say which places a type's schema "describes" |
| [FR-108](L09-packages.md#fr-108) | L09/F9 | ambiguity |  | weakened, low | coordinator | PKG-016 | PKG-016's observable forbids two instances that share an outside store, and no test can tell |
| [FR-109](L09-packages.md#fr-109) | L09/F10 | ambiguity |  | confirmed, low | coordinator | PKG-029, PKG-025, PKG-026, OPS-023, PRV-017 | PKG-029 does not say whether anything is fetched before it reports conflict |
| [FR-110](L09-packages.md#fr-110) | L09/F11 | gap |  | known, low | coordinator | SCT-014, SCT-011, SCT-013, NAM-027, SCT-015 | SCT-014 promises authored places across files, and a diagnostic can name only a JSON Pointer |
| [FR-111](L09-packages.md#fr-111) | L09/F12 | needless-complexity |  | weakened, low | coordinator | PKG-012, MAN-007, NAM-027, SCH-012, SUB-005 | PKG-012 is never the deciding rule |
| [FR-112](L09-packages.md#fr-112) | L09/F13 | needless-complexity |  | weakened, low | owner | MAN-016, MAN-015, PKG-029, MAN-004, SUB-008, NAM-009, NAM-022, CTR-013 | Contracts at an address have two regimes, and three promises exist only to keep them apart |

## L10-core-operations

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-113](L10-core-operations.md#fr-113) | L10/F1 | ambiguity |  | weakened, low | coordinator | PND-010, OBJ-006, OBJ-002, IFC-033 | sign-in.v1 accepts a bare name for `instance` although the field accepts two kinds |
| [FR-114](L10-core-operations.md#fr-114) | L10/F2 | ambiguity |  | weakened, low | coordinator | OPS-036, OBJ-020, SUB-008 | list.v1's schema admits the five contract kinds, but a contract is not an object and has no ID |
| [FR-115](L10-core-operations.md#fr-115) | L10/F3 | gap |  | confirmed, medium | owner | LIF-013, REC-001, REC-002, REC-006, OPS-036, PRV-009 | inspect.v1 and list.v1 return everything, every time: no input bounds records, and no result may be continued |
| [FR-116](L10-core-operations.md#fr-116) | L10/F4 | defect |  | confirmed, low | coordinator | TPL-012, PND-011, LIF-037, CON-024, IFC-033 | set-up.v1, setup-hook.v1 and the core README state the setup's parts without the account check that TPL-012 requires |
| [FR-117](L10-core-operations.md#fr-117) | L10/F5 | needless-complexity |  | confirmed, medium | coordinator | IFC-033, OBJ-018, FLEET-005 | Core operation descriptions are promise restatements for builders, yet IFC-033 serves them as every consumer's tool descriptions |
| [FR-118](L10-core-operations.md#fr-118) | L10/F6 | needless-complexity |  | known, low | owner | CTR-013, PND-010, PND-011, PND-013, IFC-049 | Signing a client in is an OAuth relay inside the core: four contracts and clauses in four management verbs |
| [FR-119](L10-core-operations.md#fr-119) | L10/F7 | gap |  | weakened, medium | owner | NAM-020, NAM-021, NAM-023, INS-020, ACC-024, LIF-033, FST-001, IFC-033 | No promise decides what an engine with stored state does when its built-in core package changes |
| [FR-120](L10-core-operations.md#fr-120) | L10/F8 | gap |  | weakened, low | coordinator | LIF-037, PLC-017, PLC-026, LIF-040, OPS-011, LIF-006 | declare-lost.v1 has no `expectedId`, though node names are reused by design |

## L11-core-rest

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-121](L11-core-rest.md#fr-121) | L11/F1 | ambiguity | unsafe | confirmed, high | coordinator | ACC-029, ACC-030, CON-028 | A Pool named in a bound's `targets`: what it covers, and whether a bound naming it in `pools` is within it |
| [FR-122](L11-core-rest.md#fr-122) | L11/F2 | ambiguity | unsafe | confirmed, medium | coordinator | SUB-010, DL7, IFC-022, IFC-032, ACC-033, ACC-057 | The actor for whom a handed-over directory is registered is the engine's choice, with no rule that it be established |
| [FR-123](L11-core-rest.md#fr-123) | L11/F3 | gap | unsafe | confirmed, medium | owner | PRV-029, MAN-018, DL16, LIF-035, PND-005 | A state directory under any runtime provider but the engine's own runner has no fixed name, privacy or removal |
| [FR-124](L11-core-rest.md#fr-124) | L11/F4 | defect |  | confirmed, medium | coordinator | DL17, DL13, DL18, ACC-036, OPS-008, IMP-007, IFC-049 | The implementation-defined list holds obligations that no promise states |
| [FR-125](L11-core-rest.md#fr-125) | L11/F5 | ambiguity |  | confirmed, medium | coordinator | OBJ-021, OBJ-009, OBJ-010, SEC-001 | Which schema keywords make a reference schema govern a place |
| [FR-126](L11-core-rest.md#fr-126) | L11/F6 | ambiguity |  | confirmed, medium | coordinator | ACC-056, IFC-050, OPS-023, DL9 | Which rule is the deciding rule when several refuse |
| [FR-127](L11-core-rest.md#fr-127) | L11/F7 | ambiguity |  | confirmed, low | coordinator | ACC-056, IFC-050 | What a refusal's `cause` holds, and that it may be empty |
| [FR-128](L11-core-rest.md#fr-128) | L11/F8 | gap |  | confirmed, low | coordinator | OBJ-008, NAM-008, CTR-004, TPL-004, IFC-025, OBJ-023 | A diagnostic that does not reject has no fixed level or ruleId |
| [FR-129](L11-core-rest.md#fr-129) | L11/F9 | gap |  | confirmed, low | coordinator | REC-001, REC-002, ENR-007, PND-008, PND-012 | record.v1 names no member for what the promises say an enrollment or answer record holds |
| [FR-130](L11-core-rest.md#fr-130) | L11/F10 | gap |  | confirmed, medium | owner | NAM-009, PKG-005, NAM-020, NAM-021, IFC-033, FLEET-005 | No promise says what the core package's version means once the engine's built-in core differs from the one stored objects hold |
| [FR-131](L11-core-rest.md#fr-131) | L11/F11 | contradiction |  | weakened, low | coordinator | ACC-033, TPL-004, CTR-004, OPS-019 | Core package descriptions and head comments still state the rules as they stood before Fleets, sign-in and answers |

## L12-schemas

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-132](L12-schemas.md#fr-132) | L12/F1 | gap |  | confirmed, medium | coordinator | SUB-005, OBJ-020, INS-001, SCT-014 | No promise says which schema and which failing keyword a schema rejection reports when a union fails; the rule lives only in code, and vectors pin its result |
| [FR-133](L12-schemas.md#fr-133) | L12/F2 | ambiguity |  | confirmed, medium | coordinator | OBJ-013, FLEET-002, OBJ-006, SCT-006, SCT-011 | The form of `owner`: two promises say a bare name, the framed schema and a valid example accept `{kind, name}`, and the sectioned schema rejects it |
| [FR-134](L12-schemas.md#fr-134) | L12/F3 | ambiguity |  | weakened, low | coordinator | SCH-008, SCH-009, SCH-010, CTR-008 | `schema.v1` accepts a `default` on a property under `contains`, `propertyNames`, `prefixItems`, `additionalProperties` and `patternProperties`; SCH-008 names three routes and SCH-009 does not say which are filled |
| [FR-135](L12-schemas.md#fr-135) | L12/F4 | contradiction |  | weakened, medium | coordinator | SUB-005, OBJ-012, OBJ-013, OBJ-019, INS-012, CON-016, MAN-009, IFC-025 | A schema rejection is owed two different diagnostics: the encoded promise's ("naming the key") and SUB-005's (place, contract, keyword); only library prose bridges them |
| [FR-136](L12-schemas.md#fr-136) | L12/F5 | needless-complexity |  | confirmed, low | coordinator | SCT-005, SCT-006, SCT-013, INS-012, TPL-001 | The schema set repeats itself by hand: 19 copies of one name pattern, 13 of the reserved names, each template body written up to three times, and no check that the copies agree |
| [FR-137](L12-schemas.md#fr-137) | L12/F6 | defect |  | confirmed, low | coordinator | SCH-009, IFC-033, FLEET-001, FLEET-006, OBJ-020, MAN-010 | The schemas README has drifted from the files it describes, and two by-rule examples do not say what they show |
| [FR-138](L12-schemas.md#fr-138) | L12/F7 | gap |  | weakened, low | coordinator | OBJ-014, OBJ-015, INS-021, CTR-002, CTR-004, CTR-005, CTR-013, MAN-010, SCT-012, SCT-013, INS-012 | Rules that only a schema states, or that a schema states more narrowly than its promise |
| [FR-139](L12-schemas.md#fr-139) | L12/F8 | needless-complexity |  | weakened, medium | owner | SUB-005, OBJ-012, OBJ-013, OBJ-014, OBJ-018, OBJ-019, INS-021, CTR-001, MAN-004, MAN-009 | The shape of a document is stated three times and linked only by comments; the schema's internal layout is already observable, so it is the real source |

## L13-framing

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-140](L13-framing.md#fr-140) | L13/F1 | ambiguity | unsafe | confirmed, high | coordinator | MAN-014, PND-002, IFC-025, IFC-006, SCH-004, MAN-010 | The form of a promise does not say whether `observable` binds, and rules rest on it alone |
| [FR-141](L13-framing.md#fr-141) | L13/F2 | gap | unsafe | confirmed, high | owner | PND-004, PND-002, PND-008, ENR-003, ENR-004, ENR-005, IFC-049 | "The engine's own panel" carries secret-bearing evidence and is nowhere defined; the nearest definition contradicts it |
| [FR-142](L13-framing.md#fr-142) | L13/F3 | ambiguity |  | confirmed, medium | owner | LIF-007, LIF-012, FST-001, FST-002, ENR-005, PND-004, PRV-024, ENR-003, FLEET-005, SUB-006, PLC-007, NAM-016, CON-021, PRV-011, SEC-019, OBJ-023, IFC-014 | No test in spec/ assigns a promise to a layer, the three statements of the layers differ, and half of a ten-promise trial disagrees with the recorded layer |
| [FR-143](L13-framing.md#fr-143) | L13/F4 | ambiguity |  | confirmed, medium | coordinator | PRV-022, PRV-011, PRV-019, NAM-024, NAM-025, PKG-031, PKG-016, SEC-006 | Conformance levels turn on whom a promise binds, which no field records; no level covers a package's author |
| [FR-144](L13-framing.md#fr-144) | L13/F5 | ambiguity |  | weakened, low | coordinator | IFC-014, IFC-012, SUB-006, INS-016, FLEET-005, PLC-007, PLC-008, ENR-010, OBJ-013, PLC-002 | The Language level is not self-contained: language promises lean on convention and interface promises |
| [FR-145](L13-framing.md#fr-145) | L13/F6 | defect |  | confirmed, medium | owner | PRV-010, PRV-019, PRV-021, PND-010, PND-011, PND-013, ENR-001, ENR-006, TPL-012, ACC-033 | The README's account of the convention layer no longer matches it: 245 promises, 113 of them outside the seven conventions' files |
| [FR-146](L13-framing.md#fr-146) | L13/F7 | contradiction |  | weakened, medium | owner | PKG-031, SCT-003, SCT-015, CTR-013 | "Limits of this version" holds ruled behavior that no promise states, and IN_FORCE counts it as written in spec/ |
| [FR-147](L13-framing.md#fr-147) | L13/F8 | contradiction |  | confirmed, low | coordinator | INS-016, PRV-015, PLC-002, PRV-026, SUB-006 | "Register: nothing runs" and INS-016's observable deny the provider call that registration makes to place an instance |
| [FR-148](L13-framing.md#fr-148) | L13/F9 | defect |  | confirmed, low | coordinator | PRV-029, CTR-013, PND-005, PND-010, PND-011 | A reader does not meet the limits on logins, sign-in and enrollment before relying on them |
| [FR-149](L13-framing.md#fr-149) | L13/F10 | defect |  | confirmed, low | coordinator | IFC-006, IFC-025, IFC-049, PND-002, PND-003, OPS-036, ACC-055, PRV-029 | Definitions rows that contradict the promises or each other |
| [FR-150](L13-framing.md#fr-150) | L13/F11 | defect |  | confirmed, low | coordinator | PND-009, PRV-005, CTR-014, CTR-004, PRV-018, ENR-004, ACC-056, LIF-021, ACC-033, ENR-007 | Rule 4 is not kept: terms of art without a row, and "words the promises do not use" that the promises use |
| [FR-151](L13-framing.md#fr-151) | L13/F12 | defect |  | weakened, low | coordinator | ACC-032, ACC-057, SEC-008, ACC-024, LIF-022, CTR-014, FLEET-005 | The README names two exceptions to the preservation tests; the promises make more |
| [FR-152](L13-framing.md#fr-152) | L13/F13 | defect |  | confirmed, low | coordinator | CTR-013, SCH-009 | Coverage is kept in three places that disagree, and three README sentences are stale |

## X01-catalogue-layers

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-153](X01-catalogue-layers.md#fr-153) | X01/F1 | defect |  | confirmed, medium | coordinator | ACC-056, ENR-001, ENR-006, ACC-029, ACC-053, REC-004, PRV-029 | The observable field carries obligations, and the form does not say whether it binds |
| [FR-154](X01-catalogue-layers.md#fr-154) | X01/F2 | gap |  | weakened, low | coordinator | PRV-015, PRV-019, PRV-025, ENR-002, IFC-001, OPS-002, NAM-024, PKG-031, OBJ-024 | No field says which party a promise binds, although the conformance levels are defined by it |
| [FR-155](X01-catalogue-layers.md#fr-155) | X01/F3 | defect |  | confirmed, low | coordinator | ACC-033, IFC-025, IFC-033, IFC-041, IFC-049, IFC-017, PRV-007, PRV-019, PRV-022, PRV-025, PRV-014, PRV-015, PND-010, PND-013, ENR-006, ENR-001, OPS-008, OPS-014, LIF-021, CON-008 | 'One obligation per promise' fails for about a quarter of the catalogue; the twenty clearest, with splits |
| [FR-156](X01-catalogue-layers.md#fr-156) | X01/F4 | defect |  | weakened, low | coordinator | IMP-001, IMP-003, IFC-005, IFC-029, OBJ-024, NAM-025, PKG-016 | Seven promises have an observable that names nothing a test could see |
| [FR-157](X01-catalogue-layers.md#fr-157) | X01/F5 | defect |  | confirmed, low | coordinator | CTR-013, SCH-009 | Vector coverage is recorded in five places that disagree, and 153 untested promises have no recorded reason |
| [FR-158](X01-catalogue-layers.md#fr-158) | X01/F6 | ambiguity |  | confirmed, medium | coordinator | ACC-056, ACC-012, ACC-024, LIF-022, LIF-034, REC-004, ACC-032, ACC-001, ACC-028 | Which promise ID a refusal carries is undetermined where several promises state the same refusal |
| [FR-159](X01-catalogue-layers.md#fr-159) | X01/F7 | defect |  | weakened, low | coordinator | ENR-005, PRV-002, PRV-013, SEC-024 | Four promises carry a layer that LD1's test does not give them, so the Language level does not decide document validity |
| [FR-160](X01-catalogue-layers.md#fr-160) | X01/F8 | needless-complexity |  | weakened, medium | owner | TPL-012, PRV-007, PRV-022, SEC-013, ENR-006, PND-013, PLC-017, PKG-032, SUB-009 | The convention layer no longer matches its own definition: 245 promises, 104 of them outside the seven conventions or step by step |
| [FR-161](X01-catalogue-layers.md#fr-161) | X01/F9 | needless-complexity |  | weakened, low | owner | PND-005, PND-006, PND-010, PND-011, PND-013 | Group: pending actions and sign-in (12 promises) put a login protocol in the convention layer |
| [FR-162](X01-catalogue-layers.md#fr-162) | X01/F10 | needless-complexity |  | weakened, low | owner | ENR-001, ENR-002, ENR-004, ENR-006, ENR-007, ENR-008, ENR-009, ENR-012, PLC-026 | Group: machine enrollment (10 promises) specifies a join protocol step by step that ruling 40 gives to the machine provider |
| [FR-163](X01-catalogue-layers.md#fr-163) | X01/F11 | needless-complexity |  | weakened, low | coordinator | PRV-011, PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-023, PRV-029, PRV-030, OBJ-018 | Group: provider contracts and core operations (26 promises) are written two to four times, and the copies already differ |
| [FR-164](X01-catalogue-layers.md#fr-164) | X01/F12 | needless-complexity |  | weakened, low | owner | TPL-009, TPL-012, TPL-018, CON-004, POOL-013, PLC-027, ACC-039, SEC-011, SEC-013 | Group: the setup and retirement procedure (16 promises) is step by step, and starting a stopped run to serve its retire hook costs six promises |
| [FR-165](X01-catalogue-layers.md#fr-165) | X01/F13 | needless-complexity |  | weakened, low | coordinator | ACC-023, ACC-028, FLEET-004, PLC-012, ENR-011, PRV-018, LIF-022, ACC-024 | Group: eight promises restate another promise's refusal and add no obligation |
| [FR-166](X01-catalogue-layers.md#fr-166) | X01/F14 | gap |  | weakened, medium | coordinator | PRV-029, MAN-018, CTR-004, PRV-007 | No promise decides the state directory under a runtime provider other than the engine's own runner |

## X02-inforce-vs-spec

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-167](X02-inforce-vs-spec.md#fr-167) | X02/F1 | defect |  | confirmed, low | coordinator | PLC-026, PLC-008, ENR-004, ENR-007 | An In spec/ line says nodes are registered from the machine provider's reports; the cited promises say a report never makes a node |
| [FR-168](X02-inforce-vs-spec.md#fr-168) | X02/F2 | defect |  | weakened, low | coordinator | ENR-001, ENR-006 | The headless-admission line contradicts itself, and carries an unmarked coordinator's reading that differs from the owner's words |
| [FR-169](X02-inforce-vs-spec.md#fr-169) | X02/F3 | defect |  | confirmed, low | coordinator | ACC-056, IFC-050, OPS-023 | The refusals line restates ruling 52 in words ACC-056 no longer says |
| [FR-170](X02-inforce-vs-spec.md#fr-170) | X02/F4 | defect |  | confirmed, medium | coordinator | FLEET-004, ACC-029, ACC-030 | The spec-pending lines for ruling 61 B2 and B7 are weaker than the owner's words: a Fleet's holdings are not limited to two kinds, and a grant to the Fleet itself is not limited to Pool administration |
| [FR-171](X02-inforce-vs-spec.md#fr-171) | X02/F5 | defect |  | confirmed, low | coordinator | REC-001, REC-004 | The metrics spec-pending line omits ruled parts of ruling 68 point 1 that ruling 69 did not withdraw |
| [FR-172](X02-inforce-vs-spec.md#fr-172) | X02/F6 | gap |  | weakened, low | coordinator | CTR-013, ACC-027, MAN-017 | The owner's acceptance of finding 4-02 (an API key as an alternative sign-in in the .v2 Agent Types) is in no ruling record, ledger row, IN_FORCE line or promise |
| [FR-173](X02-inforce-vs-spec.md#fr-173) | X02/F7 | defect |  | confirmed, low | coordinator | OBJ-013, FLEET-002, FLEET-003, PND-002, PND-010 | Spec-pending markers lag merged orders: two "Spec pending, batch 1" headers hold only In spec/ children, and three qualifiers name orders that have merged |
| [FR-174](X02-inforce-vs-spec.md#fr-174) | X02/F8 | defect |  | confirmed, low | coordinator | TPL-018, OPS-022 | A line still marked "Coordinator's, not yet accepted" holds a decision the owner reviewed and spec/ holds; line 311 still calls every later decision strikeable |
| [FR-175](X02-inforce-vs-spec.md#fr-175) | X02/F9 | contradiction |  | weakened, low | coordinator |  | IN_FORCE states ruling 75 D4's reading as settled by the owner, while the waiting list it incorporates and four scenario plans still hold counted runs for the owner to confirm it |
| [FR-176](X02-inforce-vs-spec.md#fr-176) | X02/F10 | defect |  | confirmed, low | coordinator | PRV-025 | The line on work orders on the Windows computer keeps ruling 76 point 6's permission and drops its limit |
| [FR-177](X02-inforce-vs-spec.md#fr-177) | X02/F11 | defect |  | weakened, low | coordinator | ACC-027, PRV-006 | Three further lines drop a clause of their ruling: the dogfood guards (74), the first confining provider (75 B2) and the bar's two sets of counting rules (47) |
| [FR-178](X02-inforce-vs-spec.md#fr-178) | X02/F12 | needless-complexity |  | confirmed, medium | coordinator | PND-001, PND-004, PND-010, PND-013, IFC-049, CTR-013, CTR-014, SCT-012, ACC-056 | About 25 lines restate what spec/ holds and 21 carry the state of the work, against ruling 67's one pointer; the restatements are where the drift is |
| [FR-179](X02-inforce-vs-spec.md#fr-179) | X02/F13 | defect |  | confirmed, low | coordinator | PND-010, ACC-023 | Topic 1 lists sign-in and acting for a Fleet as powers of root alone in v0; rulings 66 and 61 and PND-010 say otherwise |

## X03-model-checking

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-180](X03-model-checking.md#fr-180) | X03/F1 | contradiction | unsafe | confirmed, high | coordinator | OPS-014, OPS-018, OPS-024, OPS-030, PRV-010, PRV-025, PRV-028 | The fence's run-end event is not the owner's fence where a provider restarts the run: a dispatch sent before the end can reach the later run |
| [FR-181](X03-model-checking.md#fr-181) | X03/F2 | gap | unsafe | confirmed, high | coordinator | OPS-012, OPS-014, OPS-024, OPS-003, OPS-030, CTR-002 | A new attempt that ends failed makes the invocation failed while an earlier attempt is still in flight |
| [FR-182](X03-model-checking.md#fr-182) | X03/F3 | ambiguity | unsafe | weakened, medium | coordinator | PRV-030, PRV-010, PRV-021, LIF-036 | After a held stop is carried out, may the provider's restart policy start the run again? |
| [FR-183](X03-model-checking.md#fr-183) | X03/F4 | gap |  | confirmed, medium | coordinator | OPS-008, OPS-012, OPS-014, OPS-018, OPS-021, OPS-024, OPS-028, OPS-030, LIF-028, REV-010 | The Fenced control that holds is not the rule order 077 wrote, so the fence-dependent rows have no model evidence for the text |
| [FR-184](X03-model-checking.md#fr-184) | X03/F5 | gap |  | confirmed, medium | coordinator | PRV-007, PRV-010, PRV-014, PRV-019, PRV-021, PRV-030, LIF-036, LIF-009, PLC-017, LIF-025 | MC4's correct provider now breaks PRV-019, its P1 is four clauses in one refuted property, and MC4-009's closure rests on one trace walked on paper |
| [FR-185](X03-model-checking.md#fr-185) | X03/F6 | gap |  | confirmed, medium | coordinator | PRV-019, PRV-007, PRV-011, PRV-022, PRV-028, SEC-010, SEC-014 | `run.v1` answers with an existing run without saying so: secrets the engine read for the call reach no run, and nothing tells the actor |
| [FR-186](X03-model-checking.md#fr-186) | X03/F7 | gap |  | weakened, low | coordinator | LIF-019, LIF-024, OPS-022, LIF-028, OPS-009, OPS-034, REV-010, PRV-011, PRV-019, PRV-022, PRV-028, PKG-032, REC-003, REC-006, CON-013 | The reruns that orders 077, 081 and 082 made necessary for MC6, MC7a, MC7b and MC3 are in no queue, and eight holds rows had their `when` or `then` changed after their model ran |
| [FR-187](X03-model-checking.md#fr-187) | X03/F8 | gap |  | confirmed, medium | coordinator | OPS-030, ACC-008, ACC-014, ACC-015, ACC-027, ACC-038, ACC-044, ACC-056, CON-011, FLEET-005, POOL-016 | The 11 exclusions judged: OPS-030's reason is stale and it now carries the fence's refusal; four have neither a model nor a vector; ACC-015 is one variant away from modelled |
| [FR-188](X03-model-checking.md#fr-188) | X03/F9 | gap |  | confirmed, medium | coordinator | PLC-012, PLC-020, PRV-012, SEC-004, SEC-024, LIF-022, ACC-023, ACC-028, OPS-020, REV-006, REV-015, PND-001, PND-002, PND-003, PND-005, PND-006, PND-007, PND-009, PND-010, PND-011, PND-012 | Eleven of the 53 static rows and all ten MC9 rows have no vector, and no order is assigned to write them |
| [FR-189](X03-model-checking.md#fr-189) | X03/F10 | gap |  | confirmed, medium | coordinator | PRV-030, PKG-032, PND-008, PND-013, ENR-001, ENR-002, ENR-004, ENR-006, ENR-007, ENR-008, ENR-009, ENR-010, ENR-011, ENR-012 | Fourteen convention-layer promises are outside the catalogue that G5 counts from, and enrollment, which MC4 called needed before S3, has no model assigned |
| [FR-190](X03-model-checking.md#fr-190) | X03/F11 | defect |  | weakened, low | owner | OPS-011, OPS-018, OPS-024 | v0's condition and the roadmap say more than nine separate models can show: 79 of the 114 holds rows were never checked under a partition |
| [FR-191](X03-model-checking.md#fr-191) | X03/F12 | defect |  | known, low | coordinator | PRV-019, PRV-010, OPS-030, OPS-014, PND-008 | G5.md's stale lines: 41 say that the three orders have not run, and the rows for the promises those orders rewrote misdescribe them |

## X04-stated-limits

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-192](X04-stated-limits.md#fr-192) | X04/F1 | ambiguity | unsafe | confirmed, high | coordinator | PND-011, PND-005, PND-010, CTR-013 | A failed undo of a wrong account holds only model work, so a Resource goes on serving under the wrong login |
| [FR-193](X04-stated-limits.md#fr-193) | X04/F2 | gap | unsafe | confirmed, high | owner | PND-004, PND-008, ENR-003, ENR-004, IFC-049, PND-002 | The engine's own panel, on which the sign-in and enrollment limits rest, is defined by no promise |
| [FR-194](X04-stated-limits.md#fr-194) | X04/F3 | defect |  | weakened, medium | owner | ACC-027, IFC-022, PRV-029, LIF-013, CTR-014, PRV-006 | No result tells the user that a consumer is inside the engine's domain of trust; its restrictions are still reported as mediated |
| [FR-195](X04-stated-limits.md#fr-195) | X04/F4 | gap |  | confirmed, medium | coordinator | OPS-014, OPS-030, OPS-021, OPS-022, OPS-028, TPL-017, LIF-028 | The fence's second limit names a way on that a setup hook does not have |
| [FR-196](X04-stated-limits.md#fr-196) | X04/F5 | gap |  | confirmed, medium | coordinator | IFC-022, IFC-032, IFC-042, PRV-022, LIF-034, FST-007 | After the stated exposure nothing ends root's credential or replaces the signing keys |
| [FR-197](X04-stated-limits.md#fr-197) | X04/F6 | gap |  | weakened, medium | coordinator | PRV-029, CTR-013, IFC-049, PND-010, SEC-010, LIF-013 | Ruling 75 C1's placement rule is checked and reported by nothing, and the first user's one machine cannot meet it |
| [FR-198](X04-stated-limits.md#fr-198) | X04/F7 | gap |  | confirmed, medium | coordinator | LIF-036, LIF-017, LIF-009, PRV-021, IFC-025, IFC-050 | Unregistering reports success and nothing else when its last stop failed or was never sent |
| [FR-199](X04-stated-limits.md#fr-199) | X04/F8 | contradiction |  | weakened, low | owner | SCT-003, SCT-015, PKG-031, CTR-014, ACC-029 | The limits section states duties of plan and apply while saying it holds no promise, and a promise leans on them |
| [FR-200](X04-stated-limits.md#fr-200) | X04/F9 | defect |  | confirmed, low | coordinator | CTR-014, LIF-013, ACC-027 | An Agent Type that declares nothing about its own tools gets the mildest report |
| [FR-201](X04-stated-limits.md#fr-201) | X04/F10 | defect |  | weakened, low | coordinator | ACC-032, ACC-057, SEC-008, ACC-054, PND-009, PND-010, FLEET-005 | The limits name two exceptions to the preservation tests; the promises make more to test 5 |

## X05-short-form

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-202](X05-short-form.md#fr-202) | X05/F1 | contradiction |  | confirmed, medium | coordinator | SCT-011, OBJ-013, FLEET-002, SCT-003, SUB-010 | A file's `fleet` key is said to supply the owner of framed Agents, which a framed Agent must itself state |
| [FR-203](X05-short-form.md#fr-203) | X05/F2 | gap |  | weakened, low | coordinator | SCT-006, SCT-007, SCT-014, SUB-005 | No promise says which form a failed sectioned definition is reported against |
| [FR-204](X05-short-form.md#fr-204) | X05/F3 | needless-complexity |  | weakened, low | owner | SCT-012, SCT-007, SCT-013, SCT-016 | `manages` writes nothing that `consumes` with the core scope templates does not |
| [FR-205](X05-short-form.md#fr-205) | X05/F4 | gap |  | known, low | coordinator | SCT-014, SCT-011, SCT-013, NAM-027, SUB-004 | SCT-014's authored locations cannot tell two files, or two documents of one file, apart |

## X06-authority-access

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-206](X06-authority-access.md#fr-206) | X06/F1 | gap |  | confirmed, high | coordinator | LIF-040, ACC-053, ACC-030, ACC-029, ACC-055, OPS-008 | Lift, close and reconcile decisions after a removal rest on facts no promise keeps |
| [FR-207](X06-authority-access.md#fr-207) | X06/F2 | contradiction |  | confirmed, medium | coordinator | ACC-033, ACC-055, LIF-028, LIF-005, LIF-024, LIF-035, IMP-007 | Reconciling a close: row 12 sends it to `management`, four other texts send it to `access` |
| [FR-208](X06-authority-access.md#fr-208) | X06/F3 | ambiguity | unsafe | weakened, low | coordinator | ACC-055, OPS-008, FLEET-002 | ACC-055 does not say how a kept sender is matched, or what "owns" means once the sender is removed |
| [FR-209](X06-authority-access.md#fr-209) | X06/F4 | contradiction |  | confirmed, medium | coordinator | ACC-056, OPS-029, ACC-012, ACC-024, ACC-029, ACC-030, ACC-053 | Tier 1 cannot be independent of the named object: the reason and the refusing rule disclose it to a bounded holder |
| [FR-210](X06-authority-access.md#fr-210) | X06/F5 | gap |  | confirmed, medium | coordinator | ACC-056, OPS-023, IFC-050, ACC-012, ACC-024, ACC-029, ACC-033, ACC-057 | No promise orders the access rules when more than one refuses, yet the ruleId is observable |
| [FR-211](X06-authority-access.md#fr-211) | X06/F6 | ambiguity |  | weakened, low | coordinator | ACC-056, ACC-033, ACC-001, IFC-050 | Who receives tier 2 is undecided: "may inspect the authorizing target under ACC-033" has two readings and no meaning for an ordinary operation |
| [FR-212](X06-authority-access.md#fr-212) | X06/F7 | needless-complexity |  | confirmed, low | coordinator | ACC-033, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003 | The retired-owner fallbacks of ACC-033 decide a state no request reaches |
| [FR-213](X06-authority-access.md#fr-213) | X06/F8 | gap |  | confirmed, low | coordinator | LIF-032, ACC-033, OPS-029, OPS-023 | ACC-033 has no row for set-up or retire that names a Conduit or a template |
| [FR-214](X06-authority-access.md#fr-214) | X06/F9 | ambiguity | unsafe | weakened, medium | coordinator | ACC-055, LIF-028, ACC-054, SEC-008, IMP-007 | A reconciliation that makes a Conduit open is not held to the root-only conditions on opening it |
| [FR-215](X06-authority-access.md#fr-215) | X06/F10 | gap |  | confirmed, low | coordinator | LIF-040, ACC-053, LIF-035, CON-008 | A tombstone lifted before its instance is removed: what later closures name as cause is the build's choice |
| [FR-216](X06-authority-access.md#fr-216) | X06/F11 | ambiguity |  | weakened, low | coordinator | ACC-033, FLEET-005, IFC-012, FLEET-003 | Row 1 needs two Conduits and a request travels through one: the owner's register tools can only be refused |

## X07-retries-fence

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-217](X07-retries-fence.md#fr-217) | X07/F1 | contradiction | unsafe | confirmed, high | coordinator | OPS-014, OPS-013, OPS-018, OPS-024, OPS-030 | The fence orders the record after the fence, not the check; a late answer fences whatever it says |
| [FR-218](X07-retries-fence.md#fr-218) | X07/F2 | gap | unsafe | confirmed, medium | coordinator | OPS-014, PRV-025, PRV-012, IFC-010 | An answer fences even where the implementation did not give it |
| [FR-219](X07-retries-fence.md#fr-219) | X07/F3 | ambiguity | unsafe | confirmed, medium | coordinator | OPS-014, PRV-009, PRV-010, PRV-003 | The run's end as a fence: processes that outlive the run's process, and a report 'made after dispatch' |
| [FR-220](X07-retries-fence.md#fr-220) | X07/F4 | contradiction | unsafe | confirmed, medium | owner | OPS-012, OPS-024, OPS-003, OPS-014, OPS-018 | Failed stated by a later attempt is recorded while an earlier attempt is still in flight |
| [FR-221](X07-retries-fence.md#fr-221) | X07/F5 | defect | unsafe | confirmed, high | coordinator | CTR-002, OPS-008, IFC-006, OPS-012, OPS-028 | CTR-002 has an implementation deduplicate by the key alone; OPS-008's scope cannot be rebuilt from IFC-006 |
| [FR-222](X07-retries-fence.md#fr-222) | X07/F6 | gap |  | confirmed, medium | owner | CTR-002, OPS-012, OPS-018, OPS-002, OPS-003, OPS-007 | No promise says what a deduplicating implementation answers to a call it has already been sent |
| [FR-223](X07-retries-fence.md#fr-223) | X07/F7 | gap | unsafe | weakened, medium | owner | OPS-014, OPS-018, IFC-020, ACC-019, IFC-028 | The fence ignores an onward request that the engine itself dispatched for the invocation |
| [FR-224](X07-retries-fence.md#fr-224) | X07/F8 | contradiction |  | weakened, medium | coordinator | OPS-014, OPS-008, DL2, LIF-028, OPS-022, PRV-005 | The engine's duties for the fence sit in one observable and conflict with DL2 and OPS-008 |
| [FR-225](X07-retries-fence.md#fr-225) | X07/F9 | gap |  | confirmed, medium | coordinator | OPS-014, OPS-030, OPS-021, OPS-022, OPS-027, OPS-028 | Never fenced is the usual case for a remote server, and a hook has no 'new request' |
| [FR-226](X07-retries-fence.md#fr-226) | X07/F10 | gap |  | confirmed, medium | coordinator | OPS-030, OPS-014, IFC-050, IFC-033, LIF-013 | The limits and the way on are not said where an actor or an Agent meets them |

## X08-one-run

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-227](X08-one-run.md#fr-227) | X08/F1 | contradiction | unsafe | weakened, medium | coordinator | PRV-021, PRV-019, PRV-007 | The owed-stop sentence says "fails" where PRV-019 says unknown, in three places |
| [FR-228](X08-one-run.md#fr-228) | X08/F2 | defect |  | confirmed, medium | coordinator | PRV-021, PRV-020, TPL-018, TPL-009, OPS-021, LIF-035, PRV-029 | PRV-021's observable says a retired provider's server never runs again; a retire hook runs it again and keeps its state directory |
| [FR-229](X08-one-run.md#fr-229) | X08/F3 | ambiguity |  | weakened, low | coordinator | PRV-021, PRV-030, PRV-010 | PRV-021's `then`: must every provider keep an uncarried stop durably, or only state what it does? |
| [FR-230](X08-one-run.md#fr-230) | X08/F4 | needless-complexity |  | confirmed, medium | coordinator | PRV-021, PRV-029, PRV-030, TPL-018 | SSH-R's operator-set durable directory and external drainer duplicate what the retire hook and systemd already give |
| [FR-231](X08-one-run.md#fr-231) | X08/F5 | ambiguity | unsafe | confirmed, high | coordinator | PRV-030, PRV-008, LIF-024, PRV-003 | A run has no stated extent: a stop can succeed while processes the run started go on |
| [FR-232](X08-one-run.md#fr-232) | X08/F6 | gap |  | confirmed, medium | coordinator | LIF-036, LIF-009, LIF-017, PRV-021, IFC-050 | MC4-002's overlap is stated in the limits and told to no caller when it happens |
| [FR-233](X08-one-run.md#fr-233) | X08/F7 | gap |  | confirmed, medium | coordinator | PRV-007, PRV-009, PRV-019, IFC-017 | A run reported `running` for which the engine holds no `url` can never be reached |
| [FR-234](X08-one-run.md#fr-234) | X08/F8 | gap |  | confirmed, medium | coordinator | PRV-029, LIF-035, LIF-036, PRV-008 | No call tells a runtime provider that an instance was removed, so a state directory it supplies is never removed |
| [FR-235](X08-one-run.md#fr-235) | X08/F9 | gap |  | weakened, low | coordinator | PRV-019, PRV-021, PRV-030 | Nothing outside the kit's fake-backend tests can test PRV-019, PRV-030 or the owed-stop rule |

## X09-records-loss

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-236](X09-records-loss.md#fr-236) | X09/F1 | contradiction | unsafe | weakened, medium | coordinator | REC-001, SEC-016, OPS-005, OPS-008, OPS-018, IMP-007, LIF-040, ACC-044, REC-004, PND-012, ACC-037, DL10 | No promise-keeping answer exists when the store cannot be written, and DL10 and SEC-016 hand the choice to each engine |
| [FR-237](X09-records-loss.md#fr-237) | X09/F2 | gap | unsafe | confirmed, high | owner | OPS-018, OPS-008, ACC-011, LIF-040, LIF-041, ACC-044, IFC-022, PND-011, REC-003, REC-004, ACC-037, DL13 | The limit on outside loss covers records only: a rolled-back store reopens closed Conduits and repeats effects, and the state promises stay unconditional |
| [FR-238](X09-records-loss.md#fr-238) | X09/F3 | ambiguity |  | confirmed, medium | coordinator | REC-003, REC-006, DL8 | REC-003's line between the engine's acts and outside faults leaves power loss, the engine's chosen store and its own restore tool undecided |
| [FR-239](X09-records-loss.md#fr-239) | X09/F4 | ambiguity |  | weakened, medium | coordinator | REC-004, REC-006, LIF-013 | Whether a purge leaves a gap: the definition says no, the portable vectors say yes under REC-006, and a reader below management cannot see that a purge happened |
| [FR-240](X09-records-loss.md#fr-240) | X09/F5 | gap | unsafe | weakened, medium | owner | LIF-013, REC-002, CTR-004, ACC-033, ACC-056, DL8 | A record is returned whole to whoever may inspect any one object it names, so a caller's inspector reads what a target returned; no reader rule exists outside inspect.v1 |
| [FR-241](X09-records-loss.md#fr-241) | X09/F6 | gap |  | confirmed, medium | coordinator | REC-002, REC-003, LIF-040, ACC-053, LIF-019 | A record names its objects by ID alone, and the only link from an ID to a name is a tombstone that a routine lift removes |
| [FR-242](X09-records-loss.md#fr-242) | X09/F7 | contradiction |  | confirmed, medium | coordinator | REC-001, REC-002, REC-004, ACC-053, OPS-014, CTR-004, LIF-013 | A management verb's record has no place for the object it acted on, and three promises require record content that REC-002 says is not kept |
| [FR-243](X09-records-loss.md#fr-243) | X09/F8 | gap | unsafe | known, medium | coordinator | PKG-029, MAN-016, CTR-013, PKG-032, OPS-023, DL17 | Races across addresses: the rule that closes them exists only in DL17, for an engine of several parts, and an install's window is its whole fetch |
| [FR-244](X09-records-loss.md#fr-244) | X09/F9 | gap |  | weakened, low | owner | OPS-008, OPS-018, OPS-034, REC-004, ACC-011, DL10 | Invocation state is kept for every invocation with no end and no purge, so the store grows without bound and a purge does not remove who did what |

## X10-queued-and-kept

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-245](X10-queued-and-kept.md#fr-245) | X10/F1 | contradiction |  | confirmed, medium | coordinator | PRV-021, PRV-019, PRV-007, PRV-030, OPS-022 | PRV-021's owed-stop sentence says failed where PRV-019 requires unknown for the start's own instance |
| [FR-246](X10-queued-and-kept.md#fr-246) | X10/F2 | contradiction |  | weakened, medium | coordinator | PRV-015, PRV-014, PLC-026, PLC-008, ENR-002, ENR-004 | PRV-014 and PRV-015 cannot be kept through the list-machines.v1 and place.v1 contracts; the staged fix does not say how `local` is named |
| [FR-247](X10-queued-and-kept.md#fr-247) | X10/F3 | contradiction |  | weakened, low | coordinator | PRV-027, LIF-036, ENR-010, PLC-008, PLC-026 | Three passages say a node's status holds the labels "last reported"; ENR-010 fixes them at admission |
| [FR-248](X10-queued-and-kept.md#fr-248) | X10/F4 | gap |  | confirmed, medium | coordinator | ENR-006, ENR-007, PLC-026, PRV-015, ENR-004, PLC-023, ENR-012 | Nothing keeps one machine to one live node: a second admission in flight, and a machine id admitted again under a new hostname |
| [FR-249](X10-queued-and-kept.md#fr-249) | X10/F5 | gap |  | confirmed, medium | coordinator | OPS-008, LIF-040, ACC-044, REC-001, SEC-016, OPS-013 | What a request reports when the engine cannot keep its state is left to each engine (DL10); only a read of a secret is decided |
| [FR-250](X10-queued-and-kept.md#fr-250) | X10/F6 | gap | unsafe | weakened, medium | coordinator | CTR-013, PKG-032, PND-013, OBJ-002, PRV-013, LIF-041 | Races across addresses: the rule that would decide them is stated only in DL17, for an engine of several parts, outside the promise form |
| [FR-251](X10-queued-and-kept.md#fr-251) | X10/F7 | gap |  | confirmed, medium | owner | OPS-020, REV-007, REV-011, CTR-006, SCT-015, INS-013 | spec/ names no request for an edited configuration value: apply may call the control, show a difference and do nothing, or replace the instance |
| [FR-252](X10-queued-and-kept.md#fr-252) | X10/F8 | defect |  | confirmed, low | coordinator | PRV-014, PRV-003, ENR-010, PLC-007 | The engine's first-start report passes Rust architecture names through where PRV-014 asks for Go's, and ENR-010 then fixes the wrong label for life |
| [FR-253](X10-queued-and-kept.md#fr-253) | X10/F9 | gap | unsafe | confirmed, high | coordinator | CTR-002, OPS-008, OPS-012, OPS-018, OPS-024, IFC-006 | An implementation that deduplicates cannot reproduce the scope of an idempotency key: it is sent the key and never the sending instance |
| [FR-254](X10-queued-and-kept.md#fr-254) | X10/F10 | gap |  | confirmed, medium | coordinator | PRV-015, PRV-014, PRV-019, PRV-013, PLC-007, PLC-023 | A registered machine provider that omits the engine's machine, or names it by the OS hostname under DL5, makes every default-placed local instance unregistrable |
| [FR-255](X10-queued-and-kept.md#fr-255) | X10/F11 | gap |  | weakened, low | coordinator | OPS-014, OPS-003, OPS-004, OPS-013, REV-010, OPS-021 | Reconciliation has no true answer for an effect that occurred in part, and spec/ does not say so |
| [FR-256](X10-queued-and-kept.md#fr-256) | X10/F12 | gap | unsafe | weakened, low | owner | REC-003, REC-004, REC-006, OPS-008, OPS-018, LIF-040, CON-008 | Restoring an older copy of the store is classed outside the engine for records only; the same copy reopens closed Conduits and forgets keyed invocations |
| [FR-257](X10-queued-and-kept.md#fr-257) | X10/F13 | ambiguity |  | weakened, low | coordinator | REC-004, REC-006, REC-003, LIF-013 | Whether reading records after a purge says anything, and whether a lost record may be written again, are undecided |
| [FR-258](X10-queued-and-kept.md#fr-258) | X10/F14 | gap |  | confirmed, low | coordinator | PKG-032, PKG-024, PRV-017, PRV-005 | No bound on a fetch: one install whose fetch never ends holds every later install of that address and version |
| [FR-259](X10-queued-and-kept.md#fr-259) | X10/F15 | needless-complexity |  | confirmed, low | owner | ACC-033, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003, PND-009 | ACC-033's retired-owner branches decide a state that no sequence of requests reaches |
| [FR-260](X10-queued-and-kept.md#fr-260) | X10/F16 | gap |  | confirmed, low | coordinator | LIF-037, REV-012, REV-011, OPS-020 | `expectedId` pins which object a set-up or an open names, not the configuration that the plan showed |
| [FR-261](X10-queued-and-kept.md#fr-261) | X10/F17 | defect |  | confirmed, low | coordinator | CON-008, LIF-035, LIF-021, OPS-029, LIF-040 | The cascade sequence in spec/README.md tells a tool to close a Conduit that CON-008 has already closed and removed |
| [FR-262](X10-queued-and-kept.md#fr-262) | X10/F18 | gap |  | weakened, low | coordinator | LIF-011, OBS-002, PRV-009, LIF-013 | Finding 4-17 (no form for an ended condition) is decided by the paradigm LIF-011 already adopts, which the text does not state: one condition per type |
| [FR-263](X10-queued-and-kept.md#fr-263) | X10/F19 | ambiguity |  | confirmed, low | owner | IMP-001, IMP-003, CTR-004, ACC-057, CTR-014, FLEET-002 | A Type of family Resource may offer an operation marked `startsModelWork: true`, though model work is defined as an Agent's and the Agent rules key on family |

## P01-work-orders-lanes

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-264](P01-work-orders-lanes.md#fr-264) | P01 I1 | assessment | partly | none | Rounds do not converge because a fresh verifier is a new sample of the diff; "a fresh verifier found nothing" is not a test of being done |
| [FR-265](P01-work-orders-lanes.md#fr-265) | P01 I2 | assessment | partly | none | The lane more than doubled merged orders a day, but engine merges halved, its output is not yet integrated, and three incidents reached main |
| [FR-266](P01-work-orders-lanes.md#fr-266) | P01 I3 | question-for-owner | partly | owner | What should the lane do when it restarts? |
| [FR-267](P01-work-orders-lanes.md#fr-267) | P01 I4 | fact | partly | none | The table: 125 orders, their rounds, musts and outcomes |
| [FR-268](P01-work-orders-lanes.md#fr-268) | P01 I5 | assessment | partly | none | The six suspected causes, ranked by what the Records show |
| [FR-269](P01-work-orders-lanes.md#fr-269) | P01 I6 | recommendation | partly | coordinator | A size limit, set on scope and checked on the round-1 diff |
| [FR-270](P01-work-orders-lanes.md#fr-270) | P01 I7 | recommendation | partly | coordinator | No raw third round: after a second pass with musts the coordinator reads, then chooses one of three; every coordinator fix gets a fresh read |
| [FR-271](P01-work-orders-lanes.md#fr-271) | P01 I8 | recommendation | partly | coordinator | Make CI the gate: required checks, engine shards that a package crate cannot skip, a macOS leg, repeated runs for timing tests, and no local squash |
| [FR-272](P01-work-orders-lanes.md#fr-272) | P01 I9 | experiment-change | partly | coordinator | A77: give it a pass condition and a record a script can read, including the coordinator's own spend |
| [FR-273](P01-work-orders-lanes.md#fr-273) | P01 I10 | assessment | supported | none | A reviewer before dispatch: the Records show no effect on rounds, and never say what it found |
| [FR-274](P01-work-orders-lanes.md#fr-274) | P01 I11 | recommendation | partly | coordinator | Take the conflicts out: land 071 next, check the totals by tool, keep stacks one deep, and write the promise before the provider code |
| [FR-275](P01-work-orders-lanes.md#fr-275) | P01 I12 | experiment-change | partly | coordinator | A78: run small, paired orders through Conduit, so that its friction is separated from an order's size |
| [FR-276](P01-work-orders-lanes.md#fr-276) | P01 I13 | fact | supported | coordinator | Pipeline documents that are stale or understate the Records |

## P02-s1-bar

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-277](P02-s1-bar.md#fr-277) | P02 I1 | question-for-owner | partly | owner | S1's bar: keep the line count, amend it, or replace it |
| [FR-278](P02-s1-bar.md#fr-278) | P02 I2 | assessment | partly | none | The line count is a useful ceiling and an unfair gate |
| [FR-279](P02-s1-bar.md#fr-279) | P02 I3 | fact | partly | none | What the bar measures, on which tasks, counted how, and by whom |
| [FR-280](P02-s1-bar.md#fr-280) | P02 I4 | fact | partly | none | The baseline does not deliver what Conduit does; on round 6's own measures the 332-line run had already won |
| [FR-281](P02-s1-bar.md#fr-281) | P02 I5 | assessment | partly | none | The margin has been fitted to S1's seven tasks, and the fresh author is handed the answer |
| [FR-282](P02-s1-bar.md#fr-282) | P02 I6 | fact | partly | none | Neither increment-4 author struggled with lines; both struggled with concepts, errors and repair |
| [FR-283](P02-s1-bar.md#fr-283) | P02 I7 | fact | partly | none | What waits on the bar, and what the bar waits on |
| [FR-284](P02-s1-bar.md#fr-284) | P02 I8 | experiment-change | partly | coordinator | The S1 rerun: what it must also report, fixed in order 053's plan |
| [FR-285](P02-s1-bar.md#fr-285) | P02 I9 | experiment-change | partly | coordinator | Take S1 out of the specification's example, and add three held-out tasks |
| [FR-286](P02-s1-bar.md#fr-286) | P02 I10 | experiment-change | partly | coordinator | A70: a second run of the baseline, by a fresh Codex author |
| [FR-287](P02-s1-bar.md#fr-287) | P02 I11 | recommendation | partly | owner | Let three things stop waiting on the judgment |
| [FR-288](P02-s1-bar.md#fr-288) | P02 I12 | recommendation | partly | coordinator | Five small fixes the coordinator can make now |
| [FR-289](P02-s1-bar.md#fr-289) | P02 I13 | assessment | supported | none | What is working |

## P03-plan-apply

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-290](P03-plan-apply.md#fr-290) | P03 I1 | assessment | supported | none | "Tooling made of existing requests" is the right layer, but the requests have no update, so apply can only hide a retire-and-recreate, and nothing has yet shown it works |
| [FR-291](P03-plan-apply.md#fr-291) | P03 I2 | fact | partly | none | The eight edits: what apply does, who decides it, and whether it is total |
| [FR-292](P03-plan-apply.md#fr-292) | P03 I3 | question-for-owner | partly | owner | What may a change to a running Agent or Resource cost? (finding 4-01) |
| [FR-293](P03-plan-apply.md#fr-293) | P03 I4 | recommendation | partly | coordinator | The least design under which "edit the project, then apply" always works or refuses with a reason |
| [FR-294](P03-plan-apply.md#fr-294) | P03 I5 | assessment | supported | coordinator | O7 blocks building apply, not the design: keep the archive on the author's side, keyed by its digest |
| [FR-295](P03-plan-apply.md#fr-295) | P03 I6 | recommendation | partly | coordinator | Apply should call the control a Type offers, for the author's own edit to a setting of a ready instance; otherwise refuse or replace |
| [FR-296](P03-plan-apply.md#fr-296) | P03 I7 | experiment-change | supported | coordinator | S4's T1 repair by control cannot be dispatched: the instance whose setup failed is not ready |
| [FR-297](P03-plan-apply.md#fr-297) | P03 I8 | assessment | partly | coordinator | Engine-side state: safety's state is already in the engine; the ledger beside the file is Terraform's state file, and a mark on the object would replace most of it |
| [FR-298](P03-plan-apply.md#fr-298) | P03 I9 | assessment | supported | none | Which paradigm fits: Terraform's workflow, Compose's labels and surviving volume, and Kubernetes's update only if the owner adds one |
| [FR-299](P03-plan-apply.md#fr-299) | P03 I10 | experiment-change | supported | coordinator | Before 052a is written: send the nine cases' requests to the engine, then make a convergence test the planner's done-when |
| [FR-300](P03-plan-apply.md#fr-300) | P03 I11 | experiment-change | partly | coordinator | The S1 rerun should record what replacement costs, and one probe should upgrade a signed-in Agent's package |
| [FR-301](P03-plan-apply.md#fr-301) | P03 I12 | question-for-owner | supported | owner | After the rerun: should an approval name the plan instead of each object? |
| [FR-302](P03-plan-apply.md#fr-302) | P03 I13 | recommendation | supported | coordinator | Small corrections to the reference and to one sentence of spec/README.md, before 052a |

## P04-providers-built

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-303](P04-providers-built.md#fr-303) | P04 I1 | fact | supported | none | The engine reaches no packaged provider, and no provider has passed a contract case |
| [FR-304](P04-providers-built.md#fr-304) | P04 I2 | assessment | partly | none | Verdict: three contracts are thin and plausible; the runtime contract is built in the costly form and gets none of that form's benefit |
| [FR-305](P04-providers-built.md#fr-305) | P04 I3 | question-for-owner | partly | owner | Where should the gate before a run on another machine live? |
| [FR-306](P04-providers-built.md#fr-306) | P04 I4 | recommendation | partly | coordinator | Delete SSH-R's stop-drainer: PRV-021 does not need it, and its own code relies on systemd for the guarantee |
| [FR-307](P04-providers-built.md#fr-307) | P04 I5 | recommendation | partly | coordinator | Put the built-ins behind the same suite and freeze the packaged doubles (reverse decision (b)) |
| [FR-308](P04-providers-built.md#fr-308) | P04 I6 | recommendation | partly | coordinator | Stop writing providers until one runs end to end and the suites have cases |
| [FR-309](P04-providers-built.md#fr-309) | P04 I7 | recommendation | supported | coordinator | Gap: the runtime contract has no removal, so a provider keeps an instance's state directory for ever |
| [FR-310](P04-providers-built.md#fr-310) | P04 I8 | fact | supported | none | Where the cost went: 85% of provider source, 15 of 20 orders and 22 of 25 musts are the runtime contract |
| [FR-311](P04-providers-built.md#fr-311) | P04 I9 | fact | partly | none | Every place the engine special-cases a provider or does a provider's job itself |
| [FR-312](P04-providers-built.md#fr-312) | P04 I10 | assessment | supported | none | Ruling 40's test, contract by contract: which existing tools, and is a thin adapter possible |
| [FR-313](P04-providers-built.md#fr-313) | P04 I11 | experiment-change | partly | coordinator | Experiment 022: cases before providers, and the built-in as one of each pair |
| [FR-314](P04-providers-built.md#fr-314) | P04 I12 | experiment-change | supported | coordinator | A74 (experiment 006): prove SSH-R's mechanisms on the real second machine before any more SSH-R code |
| [FR-315](P04-providers-built.md#fr-315) | P04 I13 | experiment-change | partly | owner | S10: fix which first configuration is swapped, and count three behaviors that differ across a swap today |
| [FR-316](P04-providers-built.md#fr-316) | P04 I14 | question-for-owner | partly | owner | Are runtime providers that take the kit by path "self-contained"? |
| [FR-317](P04-providers-built.md#fr-317) | P04 I15 | fact | supported | coordinator | Small defects for the coordinator, none needing a ruling |

## P05-second-machine

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-318](P05-second-machine.md#fr-318) | P05 I1 | assessment | partly | none | A fair second machine for placement and a total cut; not for a lost machine, identity between runs, or enrollment as a check |
| [FR-319](P05-second-machine.md#fr-319) | P05 I2 | question-for-owner | partly | owner | S3-OF: how S3 cuts and heals the network and loses the machine |
| [FR-320](P05-second-machine.md#fr-320) | P05 I3 | experiment-change | partly | owner | S3: add a loss that returns alive, and require a live unit for the owed stop |
| [FR-321](P05-second-machine.md#fr-321) | P05 I4 | fact | supported | none | The second machine has produced no experimental result, and its code ran ahead of its probe |
| [FR-322](P05-second-machine.md#fr-322) | P05 I5 | experiment-change | partly | owner | A74: register it again around what was built, and run it before any more SSH or enrollment code |
| [FR-323](P05-second-machine.md#fr-323) | P05 I6 | fact | partly | none | Adding this machine took the owner 26 actions, against the 4 that enrollment aims for |
| [FR-324](P05-second-machine.md#fr-324) | P05 I7 | assessment | partly | none | The SSH provider is sound for this computer and moves away from ruling 50's aim |
| [FR-325](P05-second-machine.md#fr-325) | P05 I8 | question-for-owner | partly | owner | Which join does v0 prove: SSH only, a Tailscale provider as well, or a smaller contract |
| [FR-326](P05-second-machine.md#fr-326) | P05 I9 | recommendation | partly | coordinator | What would make adding a machine one command |
| [FR-327](P05-second-machine.md#fr-327) | P05 I10 | question-for-owner | partly | owner | May enrollment and faults be developed against a disposable Linux node the coordinator controls |
| [FR-328](P05-second-machine.md#fr-328) | P05 I11 | experiment-change | partly | owner | S3: count what adding the machine really cost |
| [FR-329](P05-second-machine.md#fr-329) | P05 I12 | assessment | partly | coordinator | One run user: what S3 cannot claim on the second machine, and an unplanned conflict with Docker |
| [FR-330](P05-second-machine.md#fr-330) | P05 I13 | recommendation | partly | coordinator | Small fixes the coordinator can make now |

## P06-dogfooding

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-331](P06-dogfooding.md#fr-331) | P06 I1 | assessment | supported | none | Stage 1 tested how a turn is carried, not what Conduit is for, and was weaker than the plain runner on four counts |
| [FR-332](P06-dogfooding.md#fr-332) | P06 I2 | assessment | partly | coordinator | The fence of OPS-014 is recorded on evidence the smoke test showed false: a turn's processes outlive the answer and the stop (draft row 4-23) |
| [FR-333](P06-dogfooding.md#fr-333) | P06 I3 | question-for-owner | partly | owner | Order of risk: the dedicated macOS user now, and the confining provider before any coordinator Agent? |
| [FR-334](P06-dogfooding.md#fr-334) | P06 I4 | question-for-owner | partly | owner | What should A78's orders 4 and 5 test, now that the folder is a fixed input and not a grant? |
| [FR-335](P06-dogfooding.md#fr-335) | P06 I5 | assessment | partly | coordinator | A turn of hours as one blocking call is fragile now and will not compose into an Agent driving Agents (draft row 4-26) |
| [FR-336](P06-dogfooding.md#fr-336) | P06 I6 | recommendation | partly | coordinator | What to build next for dogfooding, and what to stop |
| [FR-337](P06-dogfooding.md#fr-337) | P06 I7 | experiment-change | partly | coordinator | A78's register row: stages, a pass condition per order, counted measures, and three probes |
| [FR-338](P06-dogfooding.md#fr-338) | P06 I8 | assessment | partly | coordinator | The network loss: three faults of the Agent package, none of convention 7 (draft row 4-21) |
| [FR-339](P06-dogfooding.md#fr-339) | P06 I9 | assessment | supported | coordinator | A fresh conversation costs an engine restart because the turn's contract has no conversation (draft row 4-24) |
| [FR-340](P06-dogfooding.md#fr-340) | P06 I10 | assessment | partly | coordinator | The write inside `.git`: a stated limit, with guards that trust what the Agent can write (draft row 4-22) |
| [FR-341](P06-dogfooding.md#fr-341) | P06 I11 | assessment | supported | coordinator | The guard that tripped on GitHub's refs: a harness check that reported "cannot tell" as a breach (draft row 4-25) |
| [FR-342](P06-dogfooding.md#fr-342) | P06 I12 | fact | supported | coordinator | A78's evidence and its means are outside the repository; the reviewed tree cannot reproduce the dogfood |
| [FR-343](P06-dogfooding.md#fr-343) | P06 I13 | assessment | partly | none | What is working: the frictions landed in the layers the layered claim assigns them to |

## P07-engine-build

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-344](P07-engine-build.md#fr-344) | P07 I1 | fact | supported | none | What of v0 is not built: none of the ten named capabilities is on main |
| [FR-345](P07-engine-build.md#fr-345) | P07 I2 | assessment | partly | coordinator | The largest risk: the engine is its own oracle, so its evidence cannot show a promise unmet and no second engine could use it |
| [FR-346](P07-engine-build.md#fr-346) | P07 I3 | assessment | partly | coordinator | Unsafe candidate: a fence is recorded when a kill signal is sent, not when the run has ended |
| [FR-347](P07-engine-build.md#fr-347) | P07 I4 | fact | supported | none | Performance cliffs that remain after order 070 |
| [FR-348](P07-engine-build.md#fr-348) | P07 I5 | question-for-owner | partly | owner | Does the engine get a short consolidation before batch 1's remaining core orders are written on top of it? |
| [FR-349](P07-engine-build.md#fr-349) | P07 I6 | question-for-owner | partly | owner | Which capabilities must the engine hold before v0 is brought for publication? |
| [FR-350](P07-engine-build.md#fr-350) | P07 I7 | recommendation | partly | coordinator | From the next core order on, write vectors at the public boundary and assert the one deciding promise |
| [FR-351](P07-engine-build.md#fr-351) | P07 I8 | recommendation | partly | coordinator | Store format version 2: an existing transactional store, files by digest, and only OPS-008's fields kept for an ended invocation |
| [FR-352](P07-engine-build.md#fr-352) | P07 I9 | assessment | partly | coordinator | About 11,000 lines of engine work wait on four branches, the last two core merges had no CI engine run, and the check that guards lazy publication is manual |
| [FR-353](P07-engine-build.md#fr-353) | P07 I10 | fact | partly | coordinator | Where the core decides with no promise cited, or cites one that does not say what it does |
| [FR-354](P07-engine-build.md#fr-354) | P07 I11 | fact | supported | coordinator | What CONFORMANCE.md admits as unmet or partly met |
| [FR-355](P07-engine-build.md#fr-355) | P07 I12 | fact | supported | none | The engine is still a strict loader, a pure core and a runnable shell; the core is pure by grep and nothing enforces it |
| [FR-356](P07-engine-build.md#fr-356) | P07 I13 | fact | supported | none | The largest files, and the code an independent builder would find hardest to compare against |
| [FR-357](P07-engine-build.md#fr-357) | P07 I14 | experiment-change | partly | coordinator | New probe: run the specification's by-rule examples through the core (the cheapest falsifier in this area) |
| [FR-358](P07-engine-build.md#fr-358) | P07 I15 | experiment-change | partly | coordinator | A78 (Conduit runs its own orders): add engine measures and a pass condition on them |
| [FR-359](P07-engine-build.md#fr-359) | P07 I16 | experiment-change | partly | coordinator | Experiment 022 and S10: a provider suite passes only when the engine, not just the harness, is the caller |

## P08-progress

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-360](P08-progress.md#fr-360) | P08 I1 | assessment | partly | coordinator | The step everything waits on is starved: on the last full day one merged order of 26 was on the rerun path |
| [FR-361](P08-progress.md#fr-361) | P08 I2 | question-for-owner | partly | owner | What must v0's gate hold, and what comes first after the bar? |
| [FR-362](P08-progress.md#fr-362) | P08 I3 | question-for-owner | partly | owner | Should the S1 rerun wait for sign-in through Conduit? |
| [FR-363](P08-progress.md#fr-363) | P08 I4 | recommendation | partly | coordinator | Until the pin, run one lane: the rerun path |
| [FR-364](P08-progress.md#fr-364) | P08 I5 | fact | supported | none | The critical path to the S1 rerun: 18 unwritten orders in two chains, about a third of the planned build done |
| [FR-365](P08-progress.md#fr-365) | P08 I6 | fact | supported | none | From the pin to v0's gate: ruling 47 is in force, one of its six rows is met, and 41 engine orders stand between |
| [FR-366](P08-progress.md#fr-366) | P08 I7 | assessment | partly | owner | The bar passes on paper with no margin, and both runs must pass |
| [FR-367](P08-progress.md#fr-367) | P08 I8 | assessment | partly | owner | After the bar failed, the 'simplification' added: 30 rulings and 64 promises in three days, none removed |
| [FR-368](P08-progress.md#fr-368) | P08 I9 | assessment | partly | coordinator | Work done ahead of need: the lanes hold more Rust than the engine, for parts the engine cannot yet call |
| [FR-369](P08-progress.md#fr-369) | P08 I10 | fact | supported | none | The owner's time still to come: 13 owner steps, 35 counted runs, 11 to 12 hours stated and more unsized |
| [FR-370](P08-progress.md#fr-370) | P08 I11 | fact | supported | none | Status against each step of the roadmap |
| [FR-371](P08-progress.md#fr-371) | P08 I12 | fact | supported | none | Timeline from git: fourteen days, four of them building |
| [FR-372](P08-progress.md#fr-372) | P08 I13 | experiment-change | partly | coordinator | S1 rerun: report the margins and what the baseline cannot do, and fix beforehand how each outcome is read |
| [FR-373](P08-progress.md#fr-373) | P08 I14 | experiment-change | partly | owner | A74: observe on the real pair with the parts now built, not with engine-free stand-ins |
| [FR-374](P08-progress.md#fr-374) | P08 I15 | experiment-change | partly | coordinator | A78: measure whether Conduit speeds the work, log its frictions, and keep it off the serial chain for now |
| [FR-375](P08-progress.md#fr-375) | P08 I16 | experiment-change | partly | owner | S3, S10 and batch 2, if the owner takes I2 (b) |
| [FR-376](P08-progress.md#fr-376) | P08 I17 | recommendation | supported | coordinator | Finish 071 now; leave 049a1, 080 and 144 parked until the pin |
| [FR-377](P08-progress.md#fr-377) | P08 I18 | fact | supported | coordinator | State lines that are stale at this commit, one of them an owner question already answered |

## E01-s2-s3-s4

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-378](E01-s2-s3-s4.md#fr-378) | E01 I1 | assessment | partly | none | S2 as registered cannot fail on what it is for |
| [FR-379](E01-s2-s3-s4.md#fr-379) | E01 I2 | experiment-change | partly | owner | S2: a baseline, an oracle, a plain row that means something, one real-client run |
| [FR-380](E01-s2-s3-s4.md#fr-380) | E01 I3 | assessment | partly | none | S3: half the fault matrix cannot fail, and the owed-stop condition passes with a provider that stops nothing |
| [FR-381](E01-s2-s3-s4.md#fr-381) | E01 I4 | experiment-change | partly | owner | S3: mechanics with scripted clients and a cut made on the Mac, then one owner session |
| [FR-382](E01-s2-s3-s4.md#fr-382) | E01 I5 | assessment | supported | none | S4: the best designed of the three, but its client axis is simulated and its one real replay cannot fail |
| [FR-383](E01-s2-s3-s4.md#fr-383) | E01 I6 | experiment-change | partly | owner | S4: a twelve-row oracle, the fresh author as the test, the lost response measured on both real clients now; run it first |
| [FR-384](E01-s2-s3-s4.md#fr-384) | E01 I7 | question-for-owner | partly | owner | Amend the pre-registrations of S2, S3 and S4 before anything runs? |
| [FR-385](E01-s2-s3-s4.md#fr-385) | E01 I8 | question-for-owner | partly | owner | S3-OF: a fourth option, the cut made on the Mac |
| [FR-386](E01-s2-s3-s4.md#fr-386) | E01 I9 | question-for-owner | partly | owner | S2's plain server: Linear's own, signed in through the Interface |
| [FR-387](E01-s2-s3-s4.md#fr-387) | E01 I10 | recommendation | partly | coordinator | Three results the lane can produce now, ahead of more scenario servers |
| [FR-388](E01-s2-s3-s4.md#fr-388) | E01 I11 | fact | supported | coordinator | The owner's steps: about 11.5 timed hours as registered, about 5 as proposed; one waiting item is already answered |
| [FR-389](E01-s2-s3-s4.md#fr-389) | E01 I12 | fact | supported | none | What each scenario still needs, and whether what the lane built matches the pre-registration |
| [FR-390](E01-s2-s3-s4.md#fr-390) | E01 I13 | fact | supported | coordinator | The READMEs have not changed since acceptance; the plans carry the decisions that change how they read, and two README passages are now wrong |
| [FR-391](E01-s2-s3-s4.md#fr-391) | E01 I14 | fact | partly | coordinator | Findings in the lane's Records that no log holds |
| [FR-392](E01-s2-s3-s4.md#fr-392) | E01 I15 | assessment | supported | none | What is working, and should be kept |

## E02-s5-s6-s7

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-393](E02-s5-s6-s7.md#fr-393) | E02 I1 | assessment | partly | owner | S7 as designed cannot show that an outsider can write a package from spec/ alone |
| [FR-394](E02-s5-s6-s7.md#fr-394) | E02 I2 | experiment-change | partly | owner | S7: split the authoring test from the real-Agent run, and make help a failure |
| [FR-395](E02-s5-s6-s7.md#fr-395) | E02 I3 | question-for-owner | partly | owner | Question 1: what S7 must prove, and when |
| [FR-396](E02-s5-s6-s7.md#fr-396) | E02 I4 | assessment | partly | owner | S6: the right claim and the best measure, one condition that can pass while false, and the longest queue |
| [FR-397](E02-s5-s6-s7.md#fr-397) | E02 I5 | experiment-change | partly | owner | S6: run its core first, add the runaway-child branch, and cut what tests only the engine's own choices |
| [FR-398](E02-s5-s6-s7.md#fr-398) | E02 I6 | assessment | partly | owner | S5: mostly conformance worked by hand, and its one design question is not asked |
| [FR-399](E02-s5-s6-s7.md#fr-399) | E02 I7 | experiment-change | partly | owner | S5: three author tasks, the rest as vectors, and one branch for an implementation that does not obey |
| [FR-400](E02-s5-s6-s7.md#fr-400) | E02 I8 | question-for-owner | partly | owner | Question 2: one fresh author on the tasks that need a person, for S5 and S6 |
| [FR-401](E02-s5-s6-s7.md#fr-401) | E02 I9 | recommendation | partly | coordinator | The pre-registration form asks for observations, not claims: add the claim and the result that refutes it |
| [FR-402](E02-s5-s6-s7.md#fr-402) | E02 I10 | recommendation | partly | coordinator | Write the scenario vectors now; tag them when the safety field exists |
| [FR-403](E02-s5-s6-s7.md#fr-403) | E02 I11 | fact | supported | none | Nothing specific to S5, S6 or S7 is built, ordered or dispatched |
| [FR-404](E02-s5-s6-s7.md#fr-404) | E02 I12 | fact | supported | coordinator | The accepted texts did not change, but the promises under them did, and several gates are stale |
| [FR-405](E02-s5-s6-s7.md#fr-405) | E02 I13 | fact | supported | none | The owner's steps: none in S5's and S6's runs, at least six in S7, and no minutes stated for any |

## E03-s8-s9-s10

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-406](E03-s8-s9-s10.md#fr-406) | E03 I1 | assessment | supported | owner | S10's runtime swap already fails the 'no edit to any other document' test on paper |
| [FR-407](E03-s8-s9-s10.md#fr-407) | E03 I2 | question-for-owner | partly | owner | What may change when a provider is swapped, and which providers does S10 start from? |
| [FR-408](E03-s8-s9-s10.md#fr-408) | E03 I3 | fact | partly | coordinator | S9's pre-registration was rewritten to match the engine where the engine departs from OBS-004: `serve` sets up every instance at each start |
| [FR-409](E03-s8-s9-s10.md#fr-409) | E03 I4 | question-for-owner | partly | owner | When the engine starts again, may it set instances up by itself? |
| [FR-410](E03-s8-s9-s10.md#fr-410) | E03 I5 | experiment-change | partly | owner | S10 in three stages: a paper swap count now, the contract suites next, fewer replays last |
| [FR-411](E03-s8-s9-s10.md#fr-411) | E03 I6 | experiment-change | partly | owner | S9 reduced to the claim only it tests: what happens across an engine restart |
| [FR-412](E03-s8-s9-s10.md#fr-412) | E03 I7 | assessment | supported | coordinator | S8 mostly tests two adapters and an oracle that the lane itself will write |
| [FR-413](E03-s8-s9-s10.md#fr-413) | E03 I8 | experiment-change | partly | owner | S8 as a contract suite on two search implementations, with scripted clients |
| [FR-414](E03-s8-s9-s10.md#fr-414) | E03 I9 | question-for-owner | partly | owner | May S8, and S10's single-provider replays, run on scripted clients? |
| [FR-415](E03-s8-s9-s10.md#fr-415) | E03 I10 | experiment-change | partly | coordinator | Experiment 022: no provider has passed a contract case; give the suites a pass condition and run the runtime pair that exists |
| [FR-416](E03-s8-s9-s10.md#fr-416) | E03 I11 | assessment | supported | none | S10 as accepted: claims, pass conditions, measures, clients, owner's steps, what is not built |
| [FR-417](E03-s8-s9-s10.md#fr-417) | E03 I12 | assessment | supported | none | S9 as accepted: claims, pass conditions, measures, clients, owner's steps, what is not built |
| [FR-418](E03-s8-s9-s10.md#fr-418) | E03 I13 | assessment | supported | none | S8 as accepted: claims, pass conditions, measures, clients, owner's steps, what is not built |
| [FR-419](E03-s8-s9-s10.md#fr-419) | E03 I14 | fact | supported | coordinator | Stale or wrong statements in my area, for the coordinator to correct |
| [FR-420](E03-s8-s9-s10.md#fr-420) | E03 I15 | assessment | supported | coordinator | How these pre-registrations were made, and what in them is working |

## E04-signin-isolation

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-421](E04-signin-isolation.md#fr-421) | E04 I1 | experiment-change | partly | owner | A5 becomes the test the second accounts are kept for: sign-in through the Interface and each Agent's own subscription, in four stages |
| [FR-422](E04-signin-isolation.md#fr-422) | E04 I2 | assessment | supported | coordinator | Under one operating-system user the test cannot show that an Agent's login is safe from another Agent, and must say so in every result |
| [FR-423](E04-signin-isolation.md#fr-423) | E04 I3 | fact | partly | none | The claims of 'agent subscription isolation', each with its promise or 'no promise' |
| [FR-424](E04-signin-isolation.md#fr-424) | E04 I4 | question-for-owner | partly | owner | What is A5, and may the second accounts do Conduit's own work once they are signed in through it? |
| [FR-425](E04-signin-isolation.md#fr-425) | E04 I5 | question-for-owner | partly | owner | Should the test get one arm where isolation can actually hold: a Linux user per Agent on the Windows computer? |
| [FR-426](E04-signin-isolation.md#fr-426) | E04 I6 | recommendation | partly | coordinator | Build a tracer to one real sign-in first: about two thousand lines, not the serial five to six thousand |
| [FR-427](E04-signin-isolation.md#fr-427) | E04 I7 | experiment-change | partly | coordinator | A5, stage 0: probe the two facts about the real clients that decide the design, before order 050 is written |
| [FR-428](E04-signin-isolation.md#fr-428) | E04 I8 | experiment-change | partly | coordinator | A5: the nine robustness cases, each with its registered outcome |
| [FR-429](E04-signin-isolation.md#fr-429) | E04 I9 | experiment-change | partly | coordinator | A81: add a replay by hand before any return code is written, then the two routes through Conduit |
| [FR-430](E04-signin-isolation.md#fr-430) | E04 I10 | experiment-change | partly | coordinator | The other register rows: A82 reworded, A7 fed, S2's table replaced, and A83 if the owner chooses the user-per-Agent arm |
| [FR-431](E04-signin-isolation.md#fr-431) | E04 I11 | fact | supported | coordinator | No promise or operation revokes one Agent's sign-in, and a home is removed with its Agent only on the engine's own runner |
| [FR-432](E04-signin-isolation.md#fr-432) | E04 I12 | fact | supported | none | The chain the brief names is not the whole chain: 046 and 047 come first, and three of the nine cases need work outside it |
| [FR-433](E04-signin-isolation.md#fr-433) | E04 I13 | fact | supported | coordinator | A gap: nothing decides two sign-ins of one instance at once when its logins state no fixed port |
| [FR-434](E04-signin-isolation.md#fr-434) | E04 I14 | fact | partly | coordinator | A defect: SEC-020 is cited for 'the engine holds no token' and does not say it |
| [FR-435](E04-signin-isolation.md#fr-435) | E04 I15 | recommendation | supported | coordinator | Read the vendors' terms again for the case this test creates, before the soak |

## E05-falsifier

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-436](E05-falsifier.md#fr-436) | E05 I1 | experiment-change | partly | owner | Chosen falsifier: fresh readers predict held-out verdicts from spec/ alone (A34, narrowed). Proposed register row A83, experiment 026 |
| [FR-437](E05-falsifier.md#fr-437) | E05 I2 | question-for-owner | partly | owner | Run the blind-verdict test now, before more of the specification is written? |
| [FR-438](E05-falsifier.md#fr-438) | E05 I3 | assessment | partly | none | The five candidates weighed: (b) is the fastest and the only one whose failure would be decisive |
| [FR-439](E05-falsifier.md#fr-439) | E05 I4 | fact | partly | none | The claim's central clause has never been measured, and its one test is last in the order and does not gate v0 |
| [FR-440](E05-falsifier.md#fr-440) | E05 I5 | fact | partly | none | "A few shared conventions, as invariants, not step by step" is contradicted by the catalogue |
| [FR-441](E05-falsifier.md#fr-441) | E05 I6 | fact | supported | coordinator | Obligations are 22,622 words; the `observable` fields are 69,486, and they carry binding text |
| [FR-442](E05-falsifier.md#fr-442) | E05 I7 | recommendation | partly | coordinator | Candidate (c) has been answered by model checking; re-check the changed text first, and treat S3's real run as a demonstration |
| [FR-443](E05-falsifier.md#fr-443) | E05 I8 | fact | partly | none | Candidate (d): the engine's code names no provider; the special cases are in the specification and in the built-ins |
| [FR-444](E05-falsifier.md#fr-444) | E05 I9 | experiment-change | partly | owner | S10 (ROADMAP.md row S10, experiment 021): add a paper edit ledger now, and swap the pair that passes the suite |
| [FR-445](E05-falsifier.md#fr-445) | E05 I10 | fact | supported | none | Any vector-based result is bounded: 292 of 519 promises are cited by a case |

## G1a-model-checking-register

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-446](G1a-model-checking-register.md#fr-446) | G1a I1 | recommendation | partly | coordinator | Run the MC2 rerun and a sign-in model before the pin, MC4 and enrollment beside it, the re-holds after it: each rerun's claim, pass condition, cost and place |
| [FR-447](G1a-model-checking-register.md#fr-447) | G1a I2 | assessment | partly | coordinator | 'Holds in a bounded projection' does not support E05 I7: the partition candidate is not answered, and the one partition model points the other way |
| [FR-448](G1a-model-checking-register.md#fr-448) | G1a I3 | fact | partly | none | The recount: G5's 217 is the convention-layer safety column of a catalogue that is 17 promises short; by the catalogue's own test the set is 231, and 34 of the 217 rows changed text after their model ran |
| [FR-449](G1a-model-checking-register.md#fr-449) | G1a I4 | recommendation | supported | coordinator | Bind each model to the text it ran against: a checked account, spec orders that name their model, and a vector or a stated limit for every finding |
| [FR-450](G1a-model-checking-register.md#fr-450) | G1a I5 | experiment-change | partly | coordinator | Register row 'Experiment 012' (ROADMAP.md:75): reopened, with the models inside its question and its done-when |
| [FR-451](G1a-model-checking-register.md#fr-451) | G1a I6 | experiment-change | partly | coordinator | A40's row (docs/AUDIT.md:466; cited at ROADMAP.md:31 and :129): narrowed to promises whose safety depends on the order of events, with a done-when judged on the text at v0's tag |
| [FR-452](G1a-model-checking-register.md#fr-452) | G1a I7 | recommendation | partly | coordinator | A44: mutation testing is the right measure only when keyed by promise and run on the vectors alone; first pass on the pinned tag, then in each order's diff, the full run before the cold read |
| [FR-453](G1a-model-checking-register.md#fr-453) | G1a I8 | fact | supported | none | The readiness gate at 2eb8f258: G2, G5, G6 and G7 are unmet, and no assessment of them newer than 2026-11-06 exists |
| [FR-454](G1a-model-checking-register.md#fr-454) | G1a I9 | question-for-owner | partly | owner | What must v0's model-checking row show? |
| [FR-455](G1a-model-checking-register.md#fr-455) | G1a I10 | assessment | partly | coordinator | What the method has earned and where it is the wrong tool: keep model checking for order-dependent rules, stop counting rows, and use an interleaving tester for the races in provider code |

## G1b-model-properties

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-456](G1b-model-properties.md#fr-456) | G1b I1 | assessment | partly | coordinator | 24 of the 33 holds rows credited to gates.rs, secrets.rs, records.rs and revisions.rs rest on a property the search cannot reach, and should no longer be called holds |
| [FR-457](G1b-model-properties.md#fr-457) | G1b I2 | assessment | partly | coordinator | What the rerun orders must change: register each row against its slice, make findings properties, and stop checking values the model copied from its own expectation |
| [FR-458](G1b-model-properties.md#fr-458) | G1b I3 | fact | supported | none | records.rs (MC7b): none of its 14 properties relates two steps; all 14 rows cannot fail in the correct model, and four also state less than the slice |
| [FR-459](G1b-model-properties.md#fr-459) | G1b I4 | fact | partly | none | gates.rs (MC5a): one of its nine rows has a searched property (PRV-028); the other eight cannot fail in the correct model |
| [FR-460](G1b-model-properties.md#fr-460) | G1b I5 | fact | supported | none | secrets.rs (MC5b): four of 13 rows have a searched property, two restate a guard, seven cannot fail in the correct model |
| [FR-461](G1b-model-properties.md#fr-461) | G1b I6 | fact | supported | none | revisions.rs (MC2 revisions): REV-012's property states less than its slice; REV-002 restates a guard; REV-011 cannot fail. retry.rs, harness.rs and mc2.rs credit no promise |
| [FR-462](G1b-model-properties.md#fr-462) | G1b I7 | fact | supported | none | Static rows with a vector: of 42, 26 have a step that would fail if the catalogue's slice were broken, 3 partly, 13 not |
| [FR-463](G1b-model-properties.md#fr-463) | G1b I8 | assessment | supported | coordinator | The manifest's promise names do not say what a vector tests: SEC-008 is named four times and tested nowhere; the safety field must rest on steps that fail |
| [FR-464](G1b-model-properties.md#fr-464) | G1b I9 | fact | partly | none | MC3-001 and MC3-002 still stand at this commit, unchanged; CON-013 has no evidence of any kind, and ACC-034's own slice is held by the onward property |
| [FR-465](G1b-model-properties.md#fr-465) | G1b I10 | experiment-change | partly | coordinator | MC7b's rerun: the install race as a safety property under PKG-032, records with a write that a crash can interrupt, and the observation rows dropped |
| [FR-466](G1b-model-properties.md#fr-466) | G1b I11 | experiment-change | partly | coordinator | MC5a and MC5b reruns: keep the searched properties, add order 081's second `run.v1`, real filters and the closing of I's Conduit; move the constant rows to experiment 022 and to vectors |
| [FR-467](G1b-model-properties.md#fr-467) | G1b I12 | experiment-change | partly | coordinator | MC2 revisions and MC3: a stored-once property for REV-012; CON-013 becomes a vector; ACC-034 stands on the onward property |
| [FR-468](G1b-model-properties.md#fr-468) | G1b I13 | experiment-change | partly | coordinator | Safety vectors: credit a static row only through a step that fails; write the 13 uncovered rows, SEC-008 first |
| [FR-469](G1b-model-properties.md#fr-469) | G1b I14 | recommendation | partly | coordinator | Before any model runs again: grade every holds row on paper, and keep model checking for races |

## G2-size-brake

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-470](G2-size-brake.md#fr-470) | G2 I1 | assessment | partly | none | Nothing in the tree can stop the specification growing: adding takes one accepted recommendation, removing takes a ruling per promise after S3 to S9 and both builds |
| [FR-471](G2-size-brake.md#fr-471) | G2 I2 | recommendation | partly | coordinator | Adopt A24 now, worded as an entry rule on spec/: a promise merges with a check that cites it, and no order writes promises alone |
| [FR-472](G2-size-brake.md#fr-472) | G2 I3 | question-for-owner | partly | owner | Should v0's list of features be closed at what is ruled today? |
| [FR-473](G2-size-brake.md#fr-473) | G2 I4 | fact | partly | none | The three brakes tested against what was added to spec/ from 2026-11-06 to 2eb8f258 |
| [FR-474](G2-size-brake.md#fr-474) | G2 I5 | fact | supported | none | The candidate list A18 would present: 97 promises that no vector and no accepted scenario cites (67 language, 18 convention, 12 interface) |
| [FR-475](G2-size-brake.md#fr-475) | G2 I6 | assessment | partly | coordinator | A18 as written cannot be the brake: its test lists validity rules and the purpose's own negative rules, misses the growth, and its done-when points at a dead list |
| [FR-476](G2-size-brake.md#fr-476) | G2 I7 | fact | partly | coordinator | The shortcut table, filled from the ledger: eleven rows, five with no trigger for replacement |
| [FR-477](G2-size-brake.md#fr-477) | G2 I8 | fact | partly | none | Stable numbers at this commit and at the commits where rounds ended |
| [FR-478](G2-size-brake.md#fr-478) | G2 I9 | recommendation | partly | coordinator | Print a size line with every change to spec/ and every question that adds behavior; hold one number, report the rest, and do not cap the ones the fixes must raise |
| [FR-479](G2-size-brake.md#fr-479) | G2 I10 | experiment-change | partly | coordinator | S1's rerun, S2 to S10 and A14: each dated result scores the promises its plan lists, so that A18's list comes from runs and not from plans |

## G3-native-probes

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-480](G3-native-probes.md#fr-480) | G3 I1 | recommendation | partly | coordinator | Write v0's security claim now: a draft assembled from what is already established |
| [FR-481](G3-native-probes.md#fr-481) | G3 I2 | experiment-change | partly | coordinator | A7 restated: three regimes, a fixed list of attempts, and exactly ten rows left to probe |
| [FR-482](G3-native-probes.md#fr-482) | G3 I3 | question-for-owner | partly | owner | May the drafted claim stand as v0's security claim, and may D5, A20 and the native probes stop holding the readiness gate? |
| [FR-483](G3-native-probes.md#fr-483) | G3 I4 | assessment | partly | none | A10: automatic wake must not enter v0, and no probe result could move it in |
| [FR-484](G3-native-probes.md#fr-484) | G3 I5 | experiment-change | partly | coordinator | A10 restated: no model work between turns, and the prompts an exchange needs under three drivers |
| [FR-485](G3-native-probes.md#fr-485) | G3 I6 | experiment-change | partly | coordinator | A6 restated: one unanswered fact, whether a real client re-sends after a lost response, runnable now on a restored stub |
| [FR-486](G3-native-probes.md#fr-486) | G3 I7 | experiment-change | supported | coordinator | A22 restated: the keychain in the sessions the engine will really run in, at the head of ruling 75 B3's setup |
| [FR-487](G3-native-probes.md#fr-487) | G3 I8 | experiment-change | supported | coordinator | A73 restated: a desk check now, the run only if a 1Password store is ordered; off the road to v0 |
| [FR-488](G3-native-probes.md#fr-488) | G3 I9 | fact | supported | none | What can run now with no engine change and no second account |
| [FR-489](G3-native-probes.md#fr-489) | G3 I10 | fact | partly | coordinator | Statements about these probes that are stale or wrong at 2eb8f258 |
| [FR-490](G3-native-probes.md#fr-490) | G3 I11 | recommendation | partly | owner | Give the native probes an owner and one short round, beside the rerun path and before order 050 is written |

## G4-who-verifies

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-491](G4-who-verifies.md#fr-491) | G4 I1 | assessment | partly | none | Changing the verifier's model closes one of three gaps; the Records put most escapes in the other two |
| [FR-492](G4-who-verifies.md#fr-492) | G4 I2 | recommendation | partly | coordinator | The verification design by kind and size of order: who reads, on which model, what runs, how many passes, and what it costs |
| [FR-493](G4-who-verifies.md#fr-493) | G4 I3 | recommendation | partly | coordinator | What CI must gate in place of a reader, and which escape each gate would have stopped |
| [FR-494](G4-who-verifies.md#fr-494) | G4 I4 | recommendation | partly | coordinator | The coordinator's own text gets an independent read: every hand fix before merge, every specification settlement before dispatch |
| [FR-495](G4-who-verifies.md#fr-495) | G4 I5 | fact | partly | none | Faults by who found them, from the Records and git log |
| [FR-496](G4-who-verifies.md#fr-496) | G4 I6 | fact | partly | none | A clean verifier, then a fault: 14 of 52 orders, and what would have caught each |
| [FR-497](G4-who-verifies.md#fr-497) | G4 I7 | fact | supported | none | Verifier musts that were wrong: none is recorded, and the Records could not show one |
| [FR-498](G4-who-verifies.md#fr-498) | G4 I8 | experiment-change | partly | coordinator | A77: both verifiers read the same diff for the next ten orders, with the decision rule fixed first |
| [FR-499](G4-who-verifies.md#fr-499) | G4 I9 | experiment-change | partly | coordinator | A78: in the dogfood the verifier becomes another client on another model, with its reach declared |
| [FR-500](G4-who-verifies.md#fr-500) | G4 I10 | recommendation | partly | coordinator | Record the owner's preference, change the six lines that say the opposite, and state what it does not move |
| [FR-501](G4-who-verifies.md#fr-501) | G4 I11 | fact | supported | coordinator | What the tree cannot show about verification |

## G5-round6-evidence

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-502](G5-round6-evidence.md#fr-502) | G5 I1 | assessment | partly | coordinator | No round-6 case or measure needs to leave its scenario; what the first pass wants to save sits in run counts, the owner's hand and unfixed counting rules |
| [FR-503](G5-round6-evidence.md#fr-503) | G5 I2 | fact | partly | none | The net of the first pass on the 31 rows: 9 untouched, 10 kept, 12 moved or dropped; four contradictions and two compound drops |
| [FR-504](G5-round6-evidence.md#fr-504) | G5 I3 | fact | partly | none | Ruling 45, measure by measure, for S2 to S7: of 23 measures, 6 can be counted by a third party as written, 15 in part, and D20 (twice) not at all |
| [FR-505](G5-round6-evidence.md#fr-505) | G5 I4 | recommendation | partly | coordinator | Before any harness order is written, add one dated, labelled amendment to each pre-registration that fixes only what a count is missing |
| [FR-506](G5-round6-evidence.md#fr-506) | G5 I5 | experiment-change | partly | coordinator | D20: one fixed list of attempts, kept in S2 and S3 where ruling 44 placed it, and shared with A5 and A7 |
| [FR-507](G5-round6-evidence.md#fr-507) | G5 I6 | experiment-change | partly | coordinator | E07 and E08: ruling 44 point 2 asked S1 and S2 to measure whether a broker is good enough, and no measure exists |
| [FR-508](G5-round6-evidence.md#fr-508) | G5 I7 | assessment | partly | coordinator | S2: the proposed changes in three classes |
| [FR-509](G5-round6-evidence.md#fr-509) | G5 I8 | assessment | partly | coordinator | S3: the proposed changes in three classes |
| [FR-510](G5-round6-evidence.md#fr-510) | G5 I9 | assessment | partly | coordinator | S4: the proposed changes in three classes |
| [FR-511](G5-round6-evidence.md#fr-511) | G5 I10 | assessment | partly | coordinator | S5: the proposed changes in three classes |
| [FR-512](G5-round6-evidence.md#fr-512) | G5 I11 | assessment | supported | coordinator | S6: the proposed changes in three classes |
| [FR-513](G5-round6-evidence.md#fr-513) | G5 I12 | assessment | partly | coordinator | S7: the proposed changes in three classes |
| [FR-514](G5-round6-evidence.md#fr-514) | G5 I13 | assessment | supported | coordinator | S8: the proposed changes in three classes |
| [FR-515](G5-round6-evidence.md#fr-515) | G5 I14 | assessment | partly | coordinator | S9: the proposed changes in three classes, and one accepted number that the specification overrides |
| [FR-516](G5-round6-evidence.md#fr-516) | G5 I15 | assessment | partly | coordinator | S10: the proposed changes in three classes |
| [FR-517](G5-round6-evidence.md#fr-517) | G5 I16 | fact | supported | none | Table, S1's rows: E05, E08, E11, E12, D08, D19 |
| [FR-518](G5-round6-evidence.md#fr-518) | G5 I17 | fact | partly | none | Table, S2's rows: E02, E07, E13, E17, D01, D05, D09, D20 |
| [FR-519](G5-round6-evidence.md#fr-519) | G5 I18 | fact | partly | none | Table, S3's rows: E15 and D20 under a partition |
| [FR-520](G5-round6-evidence.md#fr-520) | G5 I19 | fact | supported | none | Table, S4's rows: E01, E09, E18, E19, E20, D02, D10 |
| [FR-521](G5-round6-evidence.md#fr-521) | G5 I20 | fact | supported | none | Table, S5's rows: E03, E04, E10, E16 and D10 |
| [FR-522](G5-round6-evidence.md#fr-522) | G5 I21 | fact | partly | none | Table, S7's row: D01 |
| [FR-523](G5-round6-evidence.md#fr-523) | G5 I22 | fact | partly | none | Table, S10's rows: E06, E14, D15, D16 and D09 on the swapped store |
| [FR-524](G5-round6-evidence.md#fr-524) | G5 I23 | fact | partly | none | Table, D22 on every run; S6, S8 and S9 hold no other round-6 row |

## G6-outside-the-tree

| ID | Name | Kind | Verdict | Decides | Item |
| --- | --- | --- | --- | --- | --- |
| [FR-525](G6-outside-the-tree.md#fr-525) | G6 I1 | fact | partly | none | The inventory: fourteen things a third coordinator or an outside reader cannot reproduce from the tree, ranked by what a loss would cost |
| [FR-526](G6-outside-the-tree.md#fr-526) | G6 I2 | recommendation | partly | coordinator | One work order: the means and the evidence of the work enter the tree |
| [FR-527](G6-outside-the-tree.md#fr-527) | G6 I3 | fact | supported | coordinator | The runners: 13 lines of prose, one command and one name are all the tree holds, and the three descriptions disagree |
| [FR-528](G6-outside-the-tree.md#fr-528) | G6 I4 | fact | partly | coordinator | A78: the plan has no file, the engine's own records of order 042 are in a store due to be wiped, and an existing command could export them |
| [FR-529](G6-outside-the-tree.md#fr-529) | G6 I5 | fact | partly | coordinator | Reports of workers and verifiers were never committed, so the rule that raw records leave the tree with a link has nothing to link to; 071 and 080 resume from files that are outside |
| [FR-530](G6-outside-the-tree.md#fr-530) | G6 I6 | question-for-owner | partly | owner | Were the keys that an agent printed on 2026-11-11 replaced? |
| [FR-531](G6-outside-the-tree.md#fr-531) | G6 I7 | experiment-change | partly | coordinator | A78: the evidence of an order run through Conduit is the engine's own records, committed, and a fresh reader must rebuild the run from them |
| [FR-532](G6-outside-the-tree.md#fr-532) | G6 I8 | fact | partly | coordinator | The eleven '(log)' marks: two are already recorded elsewhere, one is checkable from the code, and none needs a private detail |
| [FR-533](G6-outside-the-tree.md#fr-533) | G6 I9 | fact | partly | coordinator | Checks and CI: results are cited by pull request and by sentence, never by run; one merge had no CI engine run and the differential run is in no workflow |
| [FR-534](G6-outside-the-tree.md#fr-534) | G6 I10 | fact | supported | coordinator | The private-path checker checks three kinds of path; names, addresses and fingerprints are checked only if a variable outside the tree lists them |
| [FR-535](G6-outside-the-tree.md#fr-535) | G6 I11 | fact | supported | coordinator | This review's first pass is itself in session scratch space, and 17 of its 40 files would fail the path check as they are |
| [FR-536](G6-outside-the-tree.md#fr-536) | G6 I12 | assessment | partly | coordinator | The fault is a repeated one, so the fix is a check at merge and not another rule |
| [FR-537](G6-outside-the-tree.md#fr-537) | G6 I13 | assessment | supported | none | What is working: model checking, the baseline, the lane's Records and the second-machine guide can be reproduced or checked from the tree and its history |
| [FR-538](G6-outside-the-tree.md#fr-538) | G6 I14 | fact | supported | coordinator | Smaller items: a link in spec/README.md that only a pull-request ref keeps alive, numbers whose sources were in scratch space, and the second machine's unrecorded state |

## GL1-protocol

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-539](GL1-protocol.md#fr-539) | GL1/F1 | gap |  | weakened, medium | owner | IFC-002, IFC-003, IFC-004, IFC-010, IFC-036, IFC-037, IFC-046, IFC-049, IFC-038, OPS-027, IFC-001, DL12 | Revisions before the baseline are admitted for plain servers and for consumers, but the promises are written only for the baseline's mechanisms |
| [FR-540](GL1-protocol.md#fr-540) | GL1/F2 | contradiction |  | known, medium | coordinator | OPS-027, OPS-014, OPS-030, IFC-001 | The only confirmation that fences a cancelled attempt is a response the protocol forbids the implementation to send, on stdio as on Streamable HTTP |
| [FR-541](GL1-protocol.md#fr-541) | GL1/F3 | contradiction |  | weakened, medium | coordinator | IFC-049, IFC-001, PND-010, PND-011, PND-013, CTR-013 | IFC-049 relays the service's own authorization URL, so nothing meets the protocol's rule that the person who completes a URL action is the one who started it |
| [FR-542](GL1-protocol.md#fr-542) | GL1/F4 | defect |  | known, low | coordinator | IFC-025, IFC-033, IFC-050, ACC-056 | An error result's `diagnostics` sit in `structuredContent`, which the released text requires to conform to `outputSchema` with no exception for errors |
| [FR-543](GL1-protocol.md#fr-543) | GL1/F5 | ambiguity |  | refuted, low | coordinator | OPS-014, IFC-010, OPS-007, OPS-024 | OPS-014 lets "a JSON-RPC error" fence an attempt, and the protocol gives no way to tell an error the server sent from one raised on the engine's own side |
| [FR-544](GL1-protocol.md#fr-544) | GL1/F6 | ambiguity |  | weakened, low | coordinator | IFC-037, IFC-036, IFC-038, IFC-008, IFC-023 | A settings object with no `revision` is the initial revision by the protocol's convention, and no promise says whether that is `v0` |
| [FR-545](GL1-protocol.md#fr-545) | GL1/F7 | gap |  | confirmed, low | coordinator | IFC-039, IFC-040, IFC-007, IFC-030, IFC-031, IFC-001 | Whether the engine sends a consumer's `baggage` and `tracestate` on to the implementation is undecided, and they are free text outside `arguments` |

## GL2-vectors

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-546](GL2-vectors.md#fr-546) | GL2/F1 | gap |  | confirmed, medium | coordinator | LIF-013, CTR-014, ACC-027 | No vector holds an Agent whose Type states `toolsDirectory`; the first build can never report `stated: true`, and all 494 vectors pass |
| [FR-547](GL2-vectors.md#fr-547) | GL2/F2 | contradiction |  | weakened, medium | coordinator | IFC-025, IFC-050, ACC-056 | 59 cases expect error results in a form IFC-025, IFC-050 and ACC-056 no longer allow: no diagnostic, or a text block of one bare word |
| [FR-548](GL2-vectors.md#fr-548) | GL2/F3 | contradiction | unsafe | known, high | coordinator | OPS-014, OPS-013, OPS-018, OPS-024, OPS-030 | `requests-restart-interrupted` expects the engine to ignore a late complete result and then accept that the effect did not occur |
| [FR-549](GL2-vectors.md#fr-549) | GL2/F4 | ambiguity | unsafe | known, medium | coordinator | OPS-014, ACC-029, LIF-036, LIF-013, SCH-009, SCH-010, SCH-019, IFC-033, PKG-018, SUB-005 | Twelve of the 60 sampled cases, and single steps of three more, expect what only an `observable` states, the events that fence an attempt among them |
| [FR-550](GL2-vectors.md#fr-550) | GL2/F5 | defect |  | weakened, low | coordinator | INS-024, PKG-012, OBJ-001, IFC-011, PRV-022, OPS-002, CTR-014, CON-011, LIF-036, OBJ-022 | Six promises' only citing case does not hold the promise's situation, four more are half tested, and 99 of 292 cited promises are named in no step |
| [FR-551](GL2-vectors.md#fr-551) | GL2/F6 | defect |  | weakened, low | coordinator | LIF-013, ACC-044, LIF-040, LIF-041 | The two vectors 042's Record names: `e4-inventory-reading` asserts one empty list, and `e4-node-readmit-no-tombstone` cites LIF-041 without any instantiation |
| [FR-552](GL2-vectors.md#fr-552) | GL2/F7 | defect |  | confirmed, low | coordinator | IFC-050, IFC-025, OPS-030, FST-006, OPS-029 | 45 steps in 24 cases compare a diagnostic's message or the text block with the first build's exact words |
| [FR-553](GL2-vectors.md#fr-553) | GL2/F8 | defect |  | weakened, low | coordinator | SUB-011, PKG-018, NAM-028, FLEET-006, REC-002 | Cases pin implementation-defined choices without naming the item: diagnostic order (DL9, absent from the profile), the depth limit (DL10), generated names (DL15) and record actions (DL8) |
| [FR-554](GL2-vectors.md#fr-554) | GL2/F9 | needless-complexity |  | known, low | coordinator | OPS-036, OPS-014, LIF-040, LIF-013 | 452 cases require the first build's own promise citations and 283 its state form; lists are compared in an order no promise states |
| [FR-555](GL2-vectors.md#fr-555) | GL2/F10 | ambiguity |  | refuted, low | coordinator | SCH-009, SCH-010, SCH-016, SCH-008 | `schema-applicator-defaults` decides L12/F3's open question in a vector: defaults are filled through `patternProperties` and `additionalProperties` |

## GL3-restart-mid-act

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-556](GL3-restart-mid-act.md#fr-556) | GL3/F1 | needless-complexity |  | weakened, medium | coordinator | IMP-007, OPS-004, LIF-004, LIF-005, LIF-024, LIF-028, LIF-036, PLC-017, LIF-013, OPS-014 | A restart gives every engine-only verb the outcome unknown, although a promise already fixes what the store holds for each |
| [FR-557](GL3-restart-mid-act.md#fr-557) | GL3/F2 | ambiguity | unsafe | weakened, medium | coordinator | OPS-004, IMP-007, PLC-017, SUB-006, FLEET-005, ACC-053, REC-004, CON-008, LIF-035, FST-001, DL2, DL6, DL17 | No promise says that an engine verb's stored effect is whole or absent after a restart |
| [FR-558](GL3-restart-mid-act.md#fr-558) | GL3/F3 | contradiction |  | confirmed, medium | owner | OPS-022, LIF-021, IMP-007, LIF-024, OPS-012, OPS-024, OPS-018, DL2 | For the verbs the engine carries out in parts, IMP-007 says unknown where OPS-022 and LIF-021 compute the outcome from the parts |
| [FR-559](GL3-restart-mid-act.md#fr-559) | GL3/F4 | gap |  | weakened, low | coordinator | OPS-012, CTR-002, OPS-008, OPS-029, OBJ-025, FLEET-005, IFC-014, REV-002, PND-003, OPS-030, LIF-029, ACC-053 | A keyed repeat of an unknown engine verb is turned away by what its own first attempt stored, for eight of the twelve verbs that declare `deduplicates` |
| [FR-560](GL3-restart-mid-act.md#fr-560) | GL3/F5 | ambiguity |  | confirmed, medium | coordinator | OPS-014, OPS-030, IMP-007, LIF-028, LIF-024 | Whether an engine-implemented invocation that a restart ended is fenced is said both ways in one observable |
| [FR-561](GL3-restart-mid-act.md#fr-561) | GL3/F6 | gap |  | weakened, low | coordinator | PRV-029, LIF-035, LIF-036, LIF-017, OPS-029 | Removing an instance's state directory is outside the store's change, and no promise orders it or finishes it after a restart |
| [FR-562](GL3-restart-mid-act.md#fr-562) | GL3/F7 | gap |  | weakened, low | coordinator | ENR-007, ENR-006, IMP-007, OPS-012, REC-001, ENR-011 | An admission cut after accept-machine.v1 answered leaves a machine accepted, no finished node and an open enrollment, though the engine holds the provider's answer |
| [FR-563](GL3-restart-mid-act.md#fr-563) | GL3/F8 | gap |  | weakened, low | coordinator | PRV-005, PRV-017, PKG-032, OPS-005, OPS-008, REC-001, IMP-007 | A request that a restart cuts before dispatch, during an install's fetch or a registration's place.v1, has no end: no reason and no record |

## GL4-second-core-resource

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-564](GL4-second-core-resource.md#fr-564) | GL4/F1 | gap |  | weakened, low | coordinator | TPL-004, TPL-003, FST-006, PLC-028, CTR-003, CTR-004, CON-021, CON-028, IFC-012, ACC-033, FST-001, OPS-019 | Nothing rejects a template outside the core whose management or access operations the engine implements: it registers, presents tools, and every call is refused |
| [FR-565](GL4-second-core-resource.md#fr-565) | GL4/F2 | defect |  | weakened, low | coordinator | FST-006, PLC-028, PLC-008, OBJ-014, INS-025, POOL-012 | FST-006's `when` names only an instance document, so a Pool's entry made from the core templates `access`, `management` or `node` falls outside it |

## GL5-stated-limits

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-566](GL5-stated-limits.md#fr-566) | GL5/F1 | defect |  | known, low | coordinator | ACC-029, ACC-030, ACC-006, ACC-033, ACC-054, SEC-008, ACC-032, PND-009, PND-010, ACC-015, ACC-016, ACC-017, IFC-021 | The filter limit's example assumes the bound holder can make a Conduit, but in v0 it can only open one already registered, and nothing offers a way round for filters |
| [FR-567](GL5-stated-limits.md#fr-567) | GL5/F2 | gap |  | weakened, low | coordinator | ACC-030, ACC-057, ACC-033, ACC-016, IFC-021, IFC-007, CON-028, CON-022, MAN-012, PRV-003 | The broker, the only way round the management limit, cannot register an Agent, and no scenario measures it |
| [FR-568](GL5-stated-limits.md#fr-568) | GL5/F3 | gap | unsafe | weakened, medium | coordinator | SEC-008, SEC-010, SEC-005, ACC-005, CON-004, CON-021, CON-025, POOL-013, POOL-015, ACC-033, ACC-030, ACC-057, IFC-025, LIF-013 | Root's setup opens the secret Conduits a template's author declared, and its result names none of them |
| [FR-569](GL5-stated-limits.md#fr-569) | GL5/F4 | defect |  | weakened, low | coordinator | LIF-035, ACC-011, ACC-006, LIF-041, ACC-053, ACC-033 | The short-form limit says a closed Conduit stays closed until reopened, but a closed Conduit no longer exists |

## GL7-in-force-rest

| ID | Name | Class | Unsafe | Verdict | Decides | Promises | Finding |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [FR-570](GL7-in-force-rest.md#fr-570) | GL7/F1 | defect |  | known, medium | coordinator | ACC-011, LIF-001, LIF-041, IFC-014, ACC-053 | IN_FORCE's narrowing sentence reverses the order in the ruling it cites: ruling 41 item 3a says open the narrower Conduit, then close the wider one |
| [FR-571](GL7-in-force-rest.md#fr-571) | GL7/F2 | gap |  | weakened, low | coordinator | INS-018, PLC-020, CTR-014 | Ruling 21's DD5 (a path in an instance is absolute) has no line in IN_FORCE, and INS-018 is weaker: a relative path registers and means a different directory on each engine |
| [FR-572](GL7-in-force-rest.md#fr-572) | GL7/F3 | gap |  | weakened, low | coordinator | OBJ-002, FLEET-006, SCT-013, FST-001 | Ruling 24's PR8 (no declared name may be a reserved first-start name) is held only per kind, for Pools and Fleets, and in the short form; two IN_FORCE lines cite it as in spec/ |
| [FR-573](GL7-in-force-rest.md#fr-573) | GL7/F4 | defect |  | confirmed, low | coordinator | SUB-006, INS-016, FLEET-005, IFC-014 | An In spec/ line says a submission only registers; SUB-006, INS-016 and FLEET-005 have it also make and open root's default Conduits, and the ledger rows of rulings 22 and 14 do not record the amendment |
| [FR-574](GL7-in-force-rest.md#fr-574) | GL7/F5 | defect |  | weakened, low | coordinator | SEC-007, SEC-008, SEC-010, SEC-019, ACC-003, POOL-001 | Topic 1 says the credential is never handed over and cites four promises that do not say it; SEC-008 and SEC-010 put a secret in an Agent's process at root's choice |
