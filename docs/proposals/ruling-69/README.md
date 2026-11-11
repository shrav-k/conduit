# Ruling 69: metrics, emitted by packages, aggregated by Fleets and Pools, viewed from Interfaces

Status: in force (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 7 (spec pending, the metrics batch after batch 2). This record says why.

After [ruling 68](../ruling-68/README.md) left M4 open and the owner widened it to customizable
metrics in general, the coordinator put [a general design](../metrics/general.md) with six rulings:
a new Metric contract kind, an account stamped by the engine with its status and billing levels, a
structured query language with reader grains, and saved dashboards. The owner judged it too much:
"I feel we are creating many new concepts here, like metrics and account. Account specifically
should be something that's self reported by the package. Do you not think we're building too much
of our own infrastructure around this? My intention was to have each agent and resource emit
metrics according to their package, have fleets and pools aggregate them while preserving their
attributes, and have interfaces be able to view all metrics for the fleets and pools they have
visibility over. Is this not a cleaner model?" The coordinator agreed and restated the model in
three points; the owner: "Yes record it this way."

## The decisions

1. **Emit.** Each Agent and Resource emits the metrics its package declares: each metric's name,
   unit, kind (counter, histogram or gauge) and attribute keys, using OpenTelemetry's names where
   they exist. Every attribute is the package's own report, account and model included. The engine
   attaches exactly one thing, which object emitted the point, and through it that object's Fleet
   or Pool. The engine's own counts (requests by outcome and reason, refusals, durations, turns) are
   emitted the same way, under the core package, with attributes such as consumer and operation.
   Each package answers for its own numbers (ruling 68).
2. **Aggregate.** A Fleet's or a Pool's metrics are its members' points merged: points with the
   same name and identical attributes are summed, and attributes are never dropped, so different
   models or accounts stay apart without any further rule. Gauges are not summed; each member's
   latest value is shown. Token types are separate metrics, so a cached token is never added to the
   input it is part of.
3. **View.** An Interface reads the metrics of every Fleet and Pool it may inspect: root all of
   them, a scoped Interface those of its scope (ruling 66). Filtering, grouping and charts are the
   viewer's: tables in the CLI in v0, and export to existing tools for anything more. Conduit
   defines no query language and no saved views.

## What it amends

Ruling 68, point 1: M1's report and counting stand in this form; M2's reader with totals only is
withdrawn, since what may be viewed is what may be inspected; the one read becomes a read of a Fleet's
or a Pool's series. Ruling 68, point 6 (M4) is settled: the shared standard is OpenTelemetry's
names, adopted by each package where they exist, and nothing is summed across different names or
attributes. The [general proposal](../metrics/general.md) is superseded where it differs; its
surveys remain the evidence. Dropped from it: the Metric contract kind, the engine-stamped account
with its status and billing levels, the query language and its grains, and saved views.
