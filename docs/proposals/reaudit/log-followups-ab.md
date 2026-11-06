# Follow-up decisions — log of groups A and B

Written 2026-11-05 by the editor of the files of groups A and B, under the rules of
[CONSOLIDATION.md](CONSOLIDATION.md) and the follow-up decisions F1 to F26 of the
[re-audit record](README.md#follow-up-decisions--accepted) (plan-audit ruling 33). The files are
`conventions/access.yaml`, `conventions/lifecycle.yaml`, `promises/conduits.yaml`,
`promises/pools.yaml`, `promises/placement.yaml`, `promises/first-start.yaml`,
`conventions/operations.yaml`, `conventions/revisions.yaml`, `conventions/records.yaml`,
`conventions/observation.yaml`, `conventions/implicit.yaml`, `interface/implementation.yaml` and
`implementation-defined.yaml`. No other file was edited. The renaming ruled under F3 was already
in place and was not redone; no new `_meta` key was added, and the one key newly used on the
implementation's side is the existing `io.github.shrav-k.conduit/reason`.

"First log A" and "first log B" are [log-group-a.md](log-group-a.md) and
[log-group-b.md](log-group-b.md). "Log C" is [log-followups-c.md](log-followups-c.md).

## Counts

| File | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| `conventions/access.yaml` | 28 | 9 | 0 | 0 | 28 |
| `conventions/lifecycle.yaml` | 26 | 1 | 0 | 0 | 26 |
| `promises/conduits.yaml` | 21 | 3 | 0 | 2 | 23 |
| `promises/pools.yaml` | 13 | 3 | 0 | 1 | 14 |
| `promises/placement.yaml` | 16 | 3 | 0 | 1 | 17 |
| `promises/first-start.yaml` | 4 | 0 | 0 | 0 | 4 |
| `conventions/operations.yaml` | 24 | 8 | 0 | 0 | 24 |
| `conventions/revisions.yaml` | 12 | 2 | 0 | 1 | 13 |
| `conventions/records.yaml` | 6 | 3 | 0 | 0 | 6 |
| `conventions/observation.yaml` | 6 | 1 | 0 | 0 | 6 |
| `conventions/implicit.yaml` | 8 | 5 | 0 | 1 | 9 |
| `interface/implementation.yaml` | 25 | 10 | 0 | 3 | 28 |
| Total | 189 | 48 | 0 | 9 | 198 |
| `implementation-defined.yaml` | 13 entries | 2 | 0 | 0 | 13 entries |

New IDs: CON-026, CON-027, POOL-016, PLC-019, REV-013, IMP-009, IFC-026, IFC-027, IFC-028. No ID
was removed or reused.

The validation command of CONSOLIDATION.md passes and counts 353 promises across `spec/`.
`implementation-defined.yaml` parses as a list of 13 entries, each with `id`, `entry` and
`document`. Every promise ID and list entry cited in the body of these thirteen files exists.
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check` report no error. Every
line of the thirteen files is under 97 characters: the renaming had left lines longer in
ACC-016, ACC-017, ACC-034, IFC-006, IFC-018, IFC-020 and IFC-023, which were re-wrapped with no
change of text and are not counted as changed. The header comment of each file that changed
names ruling 33; `first-start.yaml` was not changed.

## New IDs, for cross-references from other files

| ID | Layer | Holds |
| --- | --- | --- |
| CON-026 | convention | F5: a rename that would make two tool names of one consumer equal reports conflict |
| CON-027 | language | F20: a Conduit document that gives an end of a kind OBJ-013 does not accept is rejected |
| POOL-016 | language | F15: an entry whose placement no input supplies takes the default placement |
| PLC-019 | convention | F16: setting up an instance whose location is not ready reports unavailable |
| REV-013 | convention | F11: a reconciliation of a control whose settings fail the schema reports invalid |
| IMP-009 | language | F8: the retire operation is never marked as starting model work |
| IFC-026 | interface, `may` | F1: an implementation answers with a reason in place of an outcome |
| IFC-027 | interface | F1: such an answer ends the request with that reason and no outcome |
| IFC-028 | interface | F7: naming in `serving` an invocation the sender does not serve reports refused |

## What changed

### conventions/access.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| ACC-002 | "The engine's own access decision" precedes any call. A request that decision does not permit has no record of invocation. A restriction the implementation applies is applied after it, and its reason ends the request under IFC-027. | F1 |
| ACC-006 | The bound "covers the new Conduit's target, as ACC-029 states, and names every operation". An actor's request to open a Conduit to `access` of which it is itself the consumer is decided here like any other, with `access` as the target. | F13; first log A, reading 3 |
| ACC-008 | A request that a restriction forbids reports refused, has no outcome and nothing of it is performed. The engine decides whether the Conduit lists the operation before any call; the implementation applies the restrictions and answers refused under IFC-026. | F1 |
| ACC-014 | "Reports unavailable, has no outcome, and nothing of it is performed", in place of "is not dispatched". Where the engine finds it, no call is made; where the implementation finds it, it answers unavailable under IFC-026. | F1 |
| ACC-019 | Work arranged for later is bound to the Conduit through which it was arranged. Once that Conduit is closed its requests report refused from then on, whatever Conduits the caller is later the consumer of. | F9 |
| ACC-029 | Now the statement of what a bound is: it names Pools, individual targets and operations; naming a Pool covers the Pool itself and every Resource it owns, later ones included. "Exactly the Resources" is gone. | F13 |
| ACC-030 | "Within" is defined: every Pool and individual target the new bound names is covered by the old one, and every operation it names is one the old one names. A request to open also needs every operation of `access` the new Conduit lists to be one the actor's own Conduit to `access` lists. The observable says a new bound may name a Resource covered only through its Pool, and a Pool only if the old one names it. | F12, F13 |
| ACC-031 | "Is covered by that Conduit's bound, as ACC-029 states" in place of "is within", since "within" now relates two bounds. | F13 |
| ACC-033 | Retiring a Pool together with its Resources (LIF-021) is authorized against that Pool. Installing a package, and every operation reserved to the root Interface, are operations of `management` and authorized against it. | F14 |

### conventions/lifecycle.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| LIF-024 | The shutdown is unknown for an instance "for which a setup was dispatched and that was retired because its location was declared lost". The sentence no longer gives a second value for an instance that was never set up, which LIF-025 and PLC-017 report as confirmed. | Confirmed reading 3 |

### promises/conduits.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| CON-003 | Each component Conduit is named by the name given to, or generated for, the instance document or nested instantiation, then a hyphen, then the component's name. The observable gives an example, says the name is generated as OBJ-003 generates one, and applies the rule again where a component is itself a composite. | F25 |
| CON-017 | The observable points at INS-023 for a list of operations that an input supplies. | F18 (log C) |
| CON-020 | "Resolving returns a diagnostic" in place of "Registration returns". | F22 |
| CON-026 | Added. | F5 |
| CON-027 | Added. It cites OBJ-013 for the kinds each end accepts and states only the rejection. | F20 |

### promises/pools.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| POOL-005 | "Resolving returns a diagnostic". | F22 |
| POOL-009 | The observable points at POOL-016. | F15 |
| POOL-011 | "Resolving returns a diagnostic". | F22 |
| POOL-016 | Added. | F15 |

### promises/placement.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| PLC-005 | "Resolving returns a diagnostic". | F22 |
| PLC-011 | An ordinary request "to retire or to unregister" a location that has other instances placed on it reports conflict. "Other" was added because a location is its own placement (PLC-008). | F16 |
| PLC-018 | The observable says a location that is registered and not yet ready is accepted as a placement, and points at PLC-019. | F16 |
| PLC-019 | Added. | F16 |

### conventions/operations.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| OPS-001 | Excepts an invocation whose implementation answered with a reason in place of an outcome. | F1 |
| OPS-005 | Also covers a request that the implementation answers with a reason. "Reports why" in place of "reports why it was not dispatched". The observable separates the two cases: no call received, or nothing performed. | F1 |
| OPS-006 | The result states its outcome, or a reason in place of one where IFC-026 permits. | F1 |
| OPS-007 | "States neither an outcome nor a reason under IFC-026". | F1 |
| OPS-016 | The state is in the result of the tool of IFC-024. | Consumer's side; first log B, item 3 |
| OPS-017 | IFC-024 says where the tool's result carries the statement that content is withheld. | Consumer's side; first log B, item 3 |
| OPS-019 | "Install a package" joins the management verbs. | F14 |
| OPS-022 | A step that was answered with a reason in place of an outcome counts, like a failed one, as having had no effect. | F1 |

### conventions/revisions.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| REV-010 | The reconciliation names the instance's current revision. A stale revision reports conflict under REV-002, none reports invalid under REV-011, and settings that fail the schema are REV-013. | F11 |
| REV-012 | Reworded: no change to an instance is dispatched while another change to it has not ended. The observable keeps "of several requests that name the same revision at most one is stored" and says the others report conflict under REV-002. What happens to a request that arrives meanwhile is DL9. | F10; first log B, item 2 |
| REV-013 | Added. Split from REV-010 because rule 6 allows one reason in a promise. | F11 |

### conventions/records.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| REC-001 | "Its outcome, or its reason where it has no outcome under OPS-005", in place of "its outcome if it was dispatched". | F1 |
| REC-004 | A purge also keeps a digest of each invocation's contents. Two requests with the same contents give the same digest, so OPS-010 still holds after a purge. | F4 |
| REC-005 | "Origin" in place of "source" for where a record was made. | F22 |

### conventions/observation.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| OBS-002 | An observation carries "its origin, which is where it was made", in place of "the source that made it". | F22 |

### conventions/implicit.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| IMP-002 | "Retired" joins the events that start no model work. | F8 |
| IMP-004 | "Resolving returns a diagnostic". | F22 |
| IMP-005 | "Resolving such a template returns a diagnostic". | F22 |
| IMP-006 | The work is bound to the Conduit through which it was arranged. A start is permitted only if that Conduit is open, and once it is closed no start is ever dispatched again. | F9 |
| IMP-008 | "A step of a setup or retire graph". | F8 |
| IMP-009 | Added, layer `language`, in the form of IMP-005. | F8 |

### interface/implementation.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| IFC-008 | The refusal is a tool result whose `isError` is true. The result states the outcome failed under IFC-009. | F6; first log B, item 11 |
| IFC-009 | The result holds the outcome "unless it holds `reason` in place of an outcome under IFC-026". A result with neither key, or with a value not named for the key, is unknown. | F1 |
| IFC-010 | A result whose `resultType` is `input_required` is unknown. The JSON-RPC error for an unknown tool or a malformed request is the protocol's invalid-params error, code -32602. | F6; first log B, items 10 and 11 |
| IFC-012 | A tool is named by the Conduit's local name, then the name of each component, then the export name, joined by dots. The local name is the name the consumer's template gives a declared Conduit, or the own name of a Conduit instantiated on its own. Examples, and the 63-character limits of OBJ-001, OBJ-022 and NAM-004. | F5 |
| IFC-014 | Generalized: a Conduit one of whose tool names would be longer than 128 characters, or would equal the name of another tool of the same consumer, is not instantiated. Stays `language`. | F5 |
| IFC-019 | An implementation that is sent a restriction that forbids the request, or one it cannot apply, performs nothing and answers refused or unavailable under IFC-026. | F1 |
| IFC-021 | Covers only a request that names no invocation. One that names an invocation the sender does not serve is IFC-028. | F7 |
| IFC-022 | The engine establishes the instance for each request, from what that request presents and not from the connection. | F2 |
| IFC-024 | The `structuredContent` of the retrieval tool's result states the invocation's state as in progress, succeeded, failed or unknown, and states that content is withheld. | Consumer's side; first log B, item 3 |
| IFC-025 | "Has an outcome" and "ended with a reason" in place of "was dispatched" and "was not dispatched". For the retrieval tool the two keys say how the retrieval itself ended. | F1; consumer's side |
| IFC-026 to IFC-028 | Added. | F1, F7 |

### implementation-defined.yaml

| ID | What changed | Decision |
| --- | --- | --- |
| DL9 | "How many controls run at once on one instance" is removed. It keeps what happens to a request for a busy instance, defined as one with a change in progress under REV-012, and the order among failed checks that give the same reason. | F10 |
| DL13 | "What a request presents to show which instance its consumer is, and how the engine establishes the instance from it for each request." | F2 |

## Applied although not named in the brief

Decision F22 lands in these files too, as log C, item 8 points out, and no other editor owns
them. It was applied as a change of word only:

- "Registration returns a diagnostic" became "Resolving returns a diagnostic" in CON-020,
  POOL-005, POOL-011, PLC-005, IMP-004 and IMP-005. IMP-009 is written the same way.
- "Source" became "origin" in REC-005 and OBS-002, for where a record or an observation was
  made.

If the coordinator did not intend this, those eight promises are the whole of it.

## Readings made while writing, to confirm

1. **F1 is written as a `may` and a `must`.** IFC-026 (`may`) is the permission to give a
   reason in place of an outcome, with its two cases and two values. IFC-019 (`must`) says an
   implementation that is sent a restriction which forbids the request, or which it cannot
   apply, performs nothing and answers so, because ACC-008 and ACC-014 promise the caller
   refused and unavailable. IFC-027 is the engine's side.
2. **The local name of a Conduit instantiated on its own is its own name** (IFC-012), as the
   brief says. For a component Conduit of a composite that was instantiated on its own, the
   own name is the one F25 gives, so the tool name repeats the component:
   `review-read.read.directory.read`. The other reading, in which the local name is the name of
   the instance document (`review.read.directory.read`), would need the engine to keep that
   name after a rename. IFC-012's observable states the first reading in an example.
3. **"Another tool presented to the same consumer"** (IFC-014, CON-026) counts the tools of
   the consumer's other Conduits that are not closed, whether or not they are open yet, and
   the tool of IFC-024. A closed Conduit is never presented again, and a replacement for a
   closed declared Conduit must be able to take its local name.
4. **IFC-014 is narrower than before in one case.** It used to reject a Conduit instantiated
   on its own under the name of one of its consumer's declared Conduits, whatever they listed.
   It now rejects it only where the two would have a tool name in common, which is what F5
   states.
5. **F25 applied again for a composite inside a composite** (CON-003): the name formed for
   the inner composite's component takes the place of the given or generated name.
6. **F12 is applied to opening only.** Closing a Conduit to `access` of another consumer needs
   only that its bound is within the actor's own, as before.
7. **F11's "the schema"** is read as in REV-008: the stored configuration with the stated
   settings applied must satisfy its schema. The narrowing a template adds to a control's
   input (TPL-011) is not applied to a reconciliation.
8. **F11's revision applies to every reconciliation of a control,** also one that records
   that the effect did not occur.
9. **F10's "change"** is a request that REV-002 checks against a revision. REV-012 keeps, for
   any object, that at most one of several requests naming the same revision is stored.
10. **PLC-011 says "other instances".** A location is its own placement, so without the word
    no location could ever be retired by an ordinary request.
11. **PLC-019 excepts the location itself,** which is its own placement and would otherwise
    never become ready.
12. **POOL-016 covers an optional input that was not supplied,** as well as a template with
    no input for the entry's placement.
13. **IFC-008's result states the outcome failed under IFC-009,** since IFC-009 gives unknown
    to a result with no outcome key and IFC-008 promises failed.

## Left undone, and why

Needing a decision that is not in the record:

1. **An actor that opens a Conduit to `access` for itself** (ACC-006). The brief keeps this
   under ACC-006, which checks that the actor's bound covers `access` and names the operations
   the new Conduit lists. Nothing limits the bound of the new Conduit, since ACC-030 applies
   only where the consumer is another instance. An actor whose bound covers `access` could
   open itself a right with a wider bound. This looks unsafe.
2. **Whether the opener's bound must also cover `access` as a target when it passes a right
   on** (first log A, reading 3; reader 3, finding 7, first question). ACC-006 and ACC-012
   still say only that "ACC-030 states what the bound must cover".
3. **A result that holds both `outcome` and `reason`** (F1). IFC-009 gives unknown to a
   result with neither key or with a value not named for its key. A result with two valid
   keys is not covered.
4. **A repeat of a request that ended with a reason** (F1; first log B, item 4). OPS-009,
   OPS-012 and OPS-024 answer a repeat of an invocation that is in progress, succeeded, unknown
   or failed. A later request with the same caller, target, identity and contents as one that
   the implementation answered with a reason, or that the engine did not dispatch, is not
   covered. The texts avoid saying whether a request answered with a reason "was dispatched".
5. **How the implementation's answer names the restriction** (IFC-019, ACC-014). No field is
   named. Whether `isError` is true on such an answer is also not stated; IFC-025 states it
   for the engine's answer to a consumer only.
6. **"In progress" and "withheld" outside the retrieval tool.** How the answer to a repeat of
   an invocation still in progress says so (OPS-009; IFC-025 has only the three outcome
   values), and how the direct answer of ACC-034 says that content is withheld, are not
   decided. The names of the fields in the retrieval tool's `structuredContent` are left to
   the core package (decision G1).
7. **A component Conduit's name and the 63-character limit** (F25; log C, item 9). A given or
   generated name of up to 63 characters, a hyphen and a component's name of up to 63 can
   reach 127. OBJ-001 bounds only a name that a document gives. CON-003 states the rule as
   ruled and neither shortens nor rejects.
8. **A rename that would make a tool name longer than 128 characters.** F5 gives conflict for
   a collision only. It can arise only for a component Conduit of a composite, under reading 2.
9. **A Pool template that declares Conduits** (F20). TPL-001 lets a Pool's `spec` hold
   `conduits`, and a declared Conduit's consumer is the instance being instantiated, which
   would make a Pool a consumer. CON-027 rejects only an end that a document gives. TPL-001 is
   group C's.
10. **Whether an instance that is retired and not yet unregistered counts as "placed on" a
    location** for PLC-011. The text is as it was.
11. **Whether setting up or retiring an instance counts as a "change"** for REV-012's
    one-at-a-time rule. Neither names a revision, so under reading 9 it does not.
12. **The digest of an invocation's contents** (F4). The algorithm and the form that is
    hashed are not fixed. REC-004 requires only that the same contents give the same digest,
    which equality under SCH-007 makes non-trivial: `1` and `1.0` are equal.
13. **The code of the JSON-RPC error for a malformed request** (F6). IFC-010 names -32602 for
    both cases, as the brief says. The check recorded in the first log B confirmed that code
    for an unknown tool only.
14. **What the engine does next with a plain server's `input_required` result.** IFC-010
    gives the outcome unknown and says nothing more.

Still open from the first logs and not in this brief: first log B, item 12 (a request that
cannot be read gets a JSON-RPC error, so `reason` has nowhere to go) and items 16 to 19; and
the splits and moves that rules 3 and 4 of the first pass excluded. "Is registered" in the
`when` of a document rule, such as "a template is registered in which", was left as it is, as
log C left it.

## For `definitions.md` and `README.md`

Promise counts are now: access 28, lifecycle 26, conduits 23, pools 14, placement 17,
first-start 4, operations 24, revisions 13, records 6, observation 6, implicit 9, the
implementation interface 28, and 13 entries in the implementation-defined list.

Rows that need to follow:

- "How a request ends": a reason may also come from the implementation, which then has
  performed nothing (refused: a restriction forbids the request; unavailable: a restriction
  cannot be applied). "Was it dispatched?" no longer divides outcome from reason (F1).
- bound and right to open: a bound names Pools, individual targets and operations; naming a
  Pool covers the Pool itself and its Resources; "within" relates two bounds (F12, F13).
- `management`: also installing a package and the operations reserved to the root Interface
  (F14). root Interface: its reserved operations are operations of `management`.
- location: it cannot be retired or unregistered by an ordinary request while other instances
  are placed on it; an instance placed on a location that is not ready cannot be set up (F16).
- source: split into `source`, the key of a value reference, and origin, where a record or
  an observation was made (F22). observation and record follow.
- purge: keeps each invocation's operation identity, outcome and a digest of its contents (F4).
- declared Conduit, Conduit: the local name, and how a tool is named (F5); how a component
  Conduit is named (F25).
- control and revision: changes to one instance are applied one at a time (F10).
- model work: neither setup nor retirement starts it (F8).

Terms the new text uses that have no row: local name (of a Conduit), tool name, origin, busy
instance, "bound to the Conduit through which it was arranged", and "within" (of two bounds).
