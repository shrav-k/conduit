# Ruling 42: log of the secrets group

Date: 2026-11-06. This is the account of how the editor of the secrets group applied
[ruling 42](README.md) to its three files: parts (a) and (b) of the ruling, the coordinator's
decisions in the table [Settled by the coordinator](../ruling-41/open-decisions.md#settled-by-the-coordinator)
with the notes "Settled together" under it, and the owner's answers under
[For the owner](../ruling-41/open-decisions.md#for-the-owner), where they touch these files. The
rulings are the owner's. The wording is the editor's, and stands unless the coordinator or the
owner strikes it. The names are those in force before the renames of the vocabulary audit
(`secret-reveal.v1`, location, operation identity and so on): another stage applies the names N1
to N20 and the editorial fixes E1 to E23. Row 5 of the table, D5 and D20 are not applied: D6
decides row 5, and the owner left D5 and D20 for more research.

Files: [`spec/promises/secrets.yaml`](../../../spec/promises/secrets.yaml),
[`spec/promises/templates.yaml`](../../../spec/promises/templates.yaml) and
[`spec/promises/conduits.yaml`](../../../spec/promises/conduits.yaml). No other file was touched
but this log.

| File | Before | Changed | Added | Removed | After |
| --- | --- | --- | --- | --- | --- |
| `secrets.yaml` | 21 | 14 | 2 | 0 | 23 |
| `templates.yaml` | 14 | 4 | 0 | 0 | 14 |
| `conduits.yaml` | 22 | 1 | 0 | 0 | 22 |
| The group | 57 | 19 | 2 | 0 | 59 |

Each file's head comment has a line for ruling 42 that names what changed and points to this log.
No promise moved between files, and no ID was reused.

## Decisions applied

| Row or answer | Promises | What changed |
| --- | --- | --- |
| Row 3 (D3, R4) | TPL-009, TPL-018 | The stop does not wait on the retire hook, as Kubernetes kills a container whose PreStop hook failed. A hook that did not succeed keeps the shutdown from being confirmed, and a further retirement dispatches it again, as Terraform runs a failed destroy-time provisioner at the next apply, so it must not depend on the state of a run that an earlier retirement stopped |
| Row 7 (D8, part) | SEC-020 | Superseded by the owner's D8 (below). The row's pointer to a new item of the implementation-defined list was not written |
| Row 8 (D9) | SEC-014; head of `secrets.yaml` | Through Conduit alone a rotated value reaches a running instance only in another instance made in its place, whose retirement closes the Conduits of the one it replaces. Once a run has ended, a further setup asks for another with reads of its own (PRV-007, OBS-004). Rotation into a running instance is no longer among what the text leaves open |
| Row 9 (D12, part) | SEC-011, SEC-012, SEC-013, TPL-009, TPL-012, TPL-018, CON-008 | A retirement whose hook goes to a `remote` item with no connection that lasts connects within the retirement, with reads that belong to it. A connection lasts until a later setup's connection takes its place or the retirement ends; a failed HTTP request does not end it, since the baseline revision keeps no session. Where a later setup's read fails, the connection that lasts is kept, and closing the reading Conduit recalls nothing. A retired consumer's Conduit permits the read for such a connection |
| Row 11 (D13) | TPL-012 | No part of a setup is begun once the instance is retired. Parts already dispatched end as they end, the outcome is OPS-022's and the instance stays retired, as the kubelet starts no new container of a Pod that is terminating. A run that such a part started is stopped by a further retirement, or by the stop of LIF-036 (D11) |
| Row 12 (D14) | SEC-011, SEC-015 | The record of a read names the engine's ID for the read as its invocation, and the setup or the retirement it belongs to as the invocation it serves |
| Row 18 (R3) | CON-008 | The record of a closing names the invocation within which the Conduit was closed and its caller: the close, the target's retirement, `declare-lost.v1`, or the `retire.v1` or `reconcile.v1` that confirmed the consumer's shutdown; under PLC-024 it states the cause |
| Row 20 (R6) | TPL-017, TPL-018 | A template selects a hook, or takes the default that a type that requires the hook states, under TPL-004. The hooks' own contracts state none. A type's default hook that is an item that is run counts under TPL-019 |
| Row 23 (R10) | SEC-010 | Each value read is passed in the run's `secrets` under the name of each variable and argument whose pointer leads to its place, and delivered under that name (PRV-011) |
| Row 24 (R11) | SEC-010 | A place that holds null is read for nothing, and its variable or argument is left out (PRV-007) |
| Row 25 (R13) | SEC-004 | Where a reference did not resolve, the record holds no part of it, names the place and says that it did not resolve |
| Row 33 (R25) | TPL-012, TPL-009 | A cancelled setup begins no later part. A cancelled retirement dispatches no hook it has not dispatched, and still asks for the stop |
| Row 40 (G1) | TPL-012 | The run and the connections stand at one position in the order, none waits for another, and the hook waits for all of them |
| D6 | None | CON-008 already has a request to a retired target report refused, and a request that names a removed Conduit report invalid under OPS-029. The head of `conduits.yaml` says so |
| D8 (option c) | SEC-020 | The engine's own runner passes, of the engine's environment, only `HOME`, `LOGNAME`, `PATH`, `SHELL`, `TERM`, `USER` and the variables a list set on the runner names, under PRV-023, and what the item binds wins. A token exported in the engine's environment reaches a process only where that list names it |
| D10 | SEC-022 (new), SEC-005 | A Type that offers `secret-reveal.v1` offers no other capability, and is rejected when its package is installed or its project submitted. A store package that lists or writes secrets does so through another Type. SEC-005 points to SEC-022 |
| D11 | TPL-012 | The run that a part of a setup started after the instance was retired is stopped by a further retirement or by the stop of LIF-036 |
| D12 (option c) | SEC-023 (new), SEC-007, SEC-008, SEC-011, SEC-012, SEC-013, SEC-017, SEC-018, SEC-019 | Where a connection that lasts has lost its values, as after the engine restarts, the engine reads again for the caller of the setup that made the connection, decided by the instance's own Conduit and its filters as they stand, and keeps nothing across the restart. The promises on reads cite SEC-023 beside SEC-010 and SEC-013 |
| D17, R5 | None | Neither touches these files. The input `identity` of `run.v1`, the name coined for D17, appears in none of them |

The record agent's entries for these files were followed, with three exceptions where the table
or an owner's answer governs: SEC-020 points to PRV-023 (D8) rather than to an item of the
implementation-defined list (row 7); SEC-013 points to SEC-023 (D12) where the entry said that a
restart waits on the owner; and TPL-012 names LIF-036 (D11) where the entry left the run to a
further retirement alone.

## Choices beyond the record

1. SEC-022 rejects such a Type when its project is submitted too, as SEC-005 and PRV-001 do. The
   ruling names only installing its package.
2. SEC-022's observable says that the type may still require a setup hook or a retire hook,
   which is an operation it requires and not a capability it offers. This is from the text of
   the option ("Hooks still reach the store's implementation"), which the ruling's summary leaves
   out.
3. SEC-023 reads again when the engine is about to send an HTTP request on the connection, not
   at the restart. The ruling does not say when. This reads nothing for an instance that no one
   uses, and decides each read as things stand when it is made. IFC-041, as the interface group
   wrote it, agrees.
4. Where the read of SEC-023 does not succeed, the request that was to be sent reports
   unavailable and the connection lasts without values, so the next request reads again, and
   each read is recorded (SEC-012, SEC-023).
5. SEC-023 covers a connection that a setup made, as the ruling says. Its observable argues that
   no other lasts across a restart: a connection made within a retirement ends with it, and an
   invocation in progress at a restart has the outcome unknown (IMP-007).
6. The record of a read under SEC-023 names the setup that made the connection as the
   invocation it serves, although that setup may have ended. REC-002, as the records group wrote
   it, agrees.
7. SEC-012's `then` adds that the engine sends no HTTP request on a connection for which a read
   failed, so that it covers SEC-023.
8. SEC-017's observable adds that nothing the engine keeps across a restart holds a value read.
9. SEC-014 takes row 8's cost line in its own words: retiring the instance that a new one
   replaces closes the Conduits of which it is an end (CON-008).
10. CON-008's observable now says that the engine can read through a retired consumer's Conduit
    for a connection that its retirement makes, as well as for a run. Row 9 lists no edit to
    CON-008; this keeps it in step with ACC-039 and TPL-018.
11. CON-008 names `declare-lost.v1` where row 18 says "the declaration of loss".
12. TPL-009 states the cancelled retirement in its `then`, where the entry put it in the
    observable, because it says what the retirement does.
13. TPL-009 says a hook that did not succeed "keeps the shutdown from being confirmed", not that
    it leaves it pending, because OPS-022 can make the retirement unknown; and that a further
    retirement dispatches it again "as OPS-021 and OPS-028 allow", not under OPS-024 alone,
    because a hook of unknown outcome is OPS-021's.
14. TPL-018 says a hook dispatched again "may" run against a new run: where an earlier stop
    failed, the run may still be reported running, and PRV-007 then asks for none.
15. SEC-023's `when` says "as after the engine restarts, on its machine or on another" for the
    ruling's "restarts or fails over"; failover is not a term of the definitions.
16. The head comment of `secrets.yaml` keeps its summary of what `spec/README.md` left open at
    ruling 41, marked as of that time, and its line for ruling 42 says what changed in that list
    without quoting `spec/README.md`, which another group edits.

## New IDs

SEC-022 (D10) and SEC-023 (D12), both assigned in advance. No other ID was added.

## Open

For the owner or a reader:

- SEC-013 ends a connection "when the instance's retirement ends". An instance that the engine
  retires under PLC-017 or PLC-024 is retired by no invocation of `retire.v1` (row 11), so it is
  not stated when its setup's connection ends. Requests to it are refused under ACC-039 in the
  meantime, and a read under SEC-023 for it would be refused too, so no value is exposed, but the
  sentence has a gap.
- A read under SEC-023 that keeps failing, such as after the reading Conduit is closed, is made
  again before each request to the item until a further setup. Whether the engine should stop
  asking until then is not stated.
- D5 is not ruled. After a failover, the read of SEC-023 crosses from the store's server to the
  engine's new machine, and who may reach the store's server is still outside the guarantee.

For other groups, whose files these are:

- `spec/definitions.md`: the rows for the engine's read of a secret, connection, caller and
  request say that a read belongs to the setup or the retirement within which the run or the
  connection is asked for; they need SEC-023, the connection made within a retirement (row 9),
  and the row for secret Resource needs SEC-022 in place of "no other provider capability".
- `spec/README.md`: the list of what the guarantee for secrets leaves outside moves rotation to
  the stated limits (row 8), drops the other capabilities of a secret Resource's type (SEC-022),
  and changes the line on a run left on a machine reported gone (LIF-036, PRV-021).
