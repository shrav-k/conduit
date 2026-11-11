# Metrics: what Fleets, Pools, Agents and Resources used, seen from an Interface

Not binding. The specification alone binds. A proposal for the owner, written on 2026-11-10 from an
investigation run as a workflow (two grounding reports, three designs, a skeptic for each, a
synthesis and a critic), condensed here under ruling 58, point 10.

## The question

The owner, on 2026-11-10: "Also, for interface, I realize we shouldn't just bubble up prompts, we
also need to bubble up metrics for each fleet and pool. Resources and agents should track their own
metrics and be able to expose them, and they should be able to be viewed from the interface on
demand. This would include things like token usage for models on an agent, or number of searches on
a vector db resource. How do we factor this into our language/approach?"

## What Conduit already knows

The engine records every request through a Conduit, with caller, consumer, target, operation,
outcome or reason, and time (REC-001, REC-002). Counting those records, with no help from any
implementation, already gives requests per Conduit, consumer and operation; refusals by reason;
durations (end record minus dispatch record); turns per Agent, since model work only ever starts
through an operation marked `startsModelWork` (IMP-001, IMP-002); and searches on a vector database
wherever one search is one call of an operation such as `query.v1`. Owners are fixed for life
(POOL-001; ruling 59, A1), so a past record's Fleet or Pool never changes.

What only the instance knows: tokens, searches inside one batched call, embedding tokens, and
numbers that are state rather than per call, such as an index's size. Evidence from experiment 008
corrects one assumption: Codex's `turn.completed.usage` is cumulative over the thread, not per turn
(in run C1 r1, [turn 1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/turn-1.jsonl) reports 37,468 input tokens and [turn 2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/experiments/008-real-client/evidence-2026-11-09-later-turn/codex-C1-later-cache-off-ttl-0-auth-valid-r1/turn-2.jsonl) reports 63,087), so summing raw values
overcounts by about half after two turns. Whether Claude Code's `result.usage` is per turn is not
yet known.

## The recommendation

Metrics are numbers about objects Conduit already has: no new family, object kind, wire or store.

- **Two sources, each labeled.** The engine counts what passes through Conduits, from its records.
  An Agent or Resource reports what only it knows with each result it returns: the operation's
  implementation declares the points it reports (OpenTelemetry names and units, attributes with
  closed values), the result carries that invocation's numbers in one `_meta` key, and the engine
  checks them against the declaration, takes whose they are from its own record, and keeps them in
  that invocation's end record. A bad report is dropped and the drop recorded; it never changes the
  outcome. The instance measures, as the owner asked, and the engine keeps and attributes, so a
  restart loses nothing and no instance can say whose numbers they are.
- **State numbers** (an index's size, memory) come from the instance's own metrics endpoint, which
  databases such as Qdrant already expose in Prometheus form, read by a provider through a Conduit
  when a package first needs them (ruling 40). Not in v0.
- **Reading.** One operation, `metrics.v1`, pulled on demand, routed like inspection: a Resource's
  numbers through its Pool, an Agent's through its Fleet. Asked of a Pool or a Fleet, it sums every
  member the container has had in the window, retired ones included, so a Fleet's weekly total is
  not short after a retirement. Root sees every Pool and Fleet through its standing Conduits; a
  scoped Interface sees what it manages (ruling 66). Pending actions stay pushed; metrics are
  pulled. Numbers decide nothing: no access, outcome, limit or dispatch.
- **Numbers without inspection.** A Conduit may list `metrics.v1` without `inspect.v1`. Such a
  reader gets the container's totals only: no breakdown by member, consumer, target, Conduit or
  caller, and no names, since those are the inventory ruling 59, A4 gives only to those who may
  inspect.

## Rulings to make

**M1. The approach.** (a) The recommendation above: the instance measures per call and reports
with its result; the engine counts the rest from records and keeps both; state numbers later
through a provider. (b) The owner's model taken literally: each instance keeps running counters and
exposes them, and the Interface pulls from each instance when it reads. This works, as Prometheus
does, but totals restart with the instance unless start times are tracked, each read reaches every
instance, and the instance says whose its numbers are. (c) Instances push OpenTelemetry to an
intake the engine serves, with its own store: about 3,000 to 4,000 lines and a second read path
that no Conduit governs. Recommended: (a).

**M2. Who may read.** (a) `metrics.v1` routed like inspection, with container totals for a reader
without `inspect.v1` and breakdowns only with it, as above. (b) `metrics.v1` only together with
`inspect.v1`: simpler, but no numbers-only reader, such as a billing view. Recommended: (a).

**M3. Every read is recorded.** Every request through a Conduit leaves a record (REC-001), so a
console that polls adds records, and only root's purge (REC-004) bounds them, which also deletes
token history. (a) Keep the rule, with a minimum interval between one caller's reads, an
implementation-defined value (DL10). (b) Record metrics reads more coarsely, one record per caller
per interval. Recommended: (a); it keeps "every request is recorded" without exception.

Stated as defaults, not put as rulings, unless the owner wants one: tokens only, no cost, in v0;
usage is not relayed to the Agent that made the call in v0; a consumer sees its own Conduits'
numbers in v1; thresholds that raise a pending action in v1; Conduit never turns on the clients'
own telemetry exporters, and reports, as ruling 52 asks, when a managed policy has turned them on;
the engine's metric names take a prefix from `spec/names.yaml`.

## When

After batch 2 (Fleet authority, after the S1 rerun), as its own small batch: definitions (usage,
measured by, rollup); about six promises in a new `spec/conventions/metrics.yaml`; amendments to
REC-002, REC-006 (a purge inside a window is reported as a gap), ACC-033 and the contract schema;
`metrics.v1` in the core package; about fourteen vectors; then about 1,000 lines in the engine.
Before it, and needing no ruling, a probe under `experiments/` settles whether Claude Code's usage
is per turn and includes sub-agents, and whether a failed Codex turn reports usage. Today both
Agent packages end a session on a failed turn, so that turn's tokens would be lost unless the
wrapper reads them first.
