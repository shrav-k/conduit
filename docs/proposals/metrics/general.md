# Metrics as a general, customizable feature

Not binding. The specification alone binds. A proposal for the owner, written on 2026-11-11 for
[ruling 68](../ruling-68/README.md), point 6 (M4), after the owner asked for metrics to be a general
feature ("customize metrics to display or aggregate whatever we want, not just for usage, for all
kinds of metrics"). Made by a workflow: nine surveys (metric data models, query languages and
views, declared metrics, tenancy and access, events against stored metrics, AI tools, model usage by
account, other control planes, Conduit itself), four competing designs each checked through three
lenses, a synthesis, a completeness critic and a revision. The surveys' quoted sources are listed
under Sources; their raw notes are not kept in the tree. It may amend ruling 68's point 1 and
supersedes [the earlier proposal](README.md) where they differ.

Status: superseded by [ruling 69](../ruling-69/README.md), which kept a much smaller model (packages
emit, Fleets and Pools aggregate with attributes kept, Interfaces view); the surveys and sources
below remain the evidence.

## The answer

WHAT THE FEATURE IS
Metrics become one general feature. A person can pick any number Conduit knows, or is told, about its Agents, Resources and Conduits. At the moment of looking, the person chooses how to filter it, group it, aggregate it and compare it. Usage is not a special case. Token usage is one declared metric among many. Others are:
- requests by outcome and reason;
- turns;
- call durations;
- searches on a vector database;
- reports that failed;
- a subscription's share used in each limit window;
- later, state such as an index's size.

WHAT THIS ROUND ADDED TO THE RESEARCH
You said the research was not enough, and the critic agreed on four fronts. I checked each one, and every check changed the design.
- Usage metering and billing:
  - OpenMeter defines meters over CloudEvents, with a "subject" that says whose usage it is.
  - Stripe Billing Meters offer sum, count and last, and say "After you configure the meter, you can't make any changes aside from the display name."
  - The FinOps Foundation's FOCUS 1.4 names "Billing Account ID" and "Sub Account ID". It also says FOCUS 1.5 "is scoped to surface AI model identity and token consumption (input and output)".
- Query standards:
  - CloudEvents SQL v1.0.0 is filter-only and "doesn't support the handling of the data field", which is where Conduit's record fields live.
  - The CNCF query-language working group produced an evaluation rubric, not a standard.
  - Open Semantic Interchange, now Apache Ossie, is at 0.1.1 and does not execute queries.
  - Result: there is no standard structured metrics query. PromQL is the only query language existing tools speak.
- Unused access: AWS IAM Access Analyzer reports "Roles with no access activity within the specified usage window". That is a metric squarely inside Conduit's own domain, and the first design left it out.
- OpenTelemetry's resolved registry format, and a rerun of the cost benchmark: details in sources.

HOW IT WORKS
Conduit already writes an unchangeable record for every step of every call (REC-001). Those records become the only source of every per-call number. Nothing is aggregated or thrown away when it is written, so any breakdown someone thinks of later reaches back over all kept history.

Numbers come from three sources, and every result labels which source each number came from:
- The engine counts from its own records. The counters are defined the way OpenMeter defines meters over CloudEvents: a record type, a value path, an aggregation and group-by paths.
- An Agent or Resource reports what only it knows, as a list of points returned with each result. Examples: tokens per model, searches inside a batch, and the subscription's quota reading. The engine checks the form, stamps whose it is from its own record, and keeps it in that call's end record. A report that arrives after the engine stopped waiting is kept too, as a late record. It is never lost silently.
- Later, state numbers are read on demand inside a read.

Each metric is declared once as a Metric contract. The contract carries a pinned reference to the OpenTelemetry convention it adopts, a generated copy of that convention, and a few fields marked as Conduit's own.

HOW A PERSON CUSTOMIZES WHAT IS SHOWN AND AGGREGATED
1. Ask what exists. metrics.v1 has a describe form. It lists the measures, their kinds and units, which aggregations are allowed, how they split, and which dimensions this reader may use.
2. Ask a query. A query names:
   - measures, each with an aggregation (sum, count, min, max, avg, p50 to p99, or last for gauges);
   - dimensions to keep;
   - filters;
   - a UTC window and bucket;
   - order and a row limit.
   One read may carry several named queries, so a dashboard is still one read. Each query may be shifted back in time ("this week against last week").
3. Save the queries as a Perses dashboard, the open CNCF format. The file grants nothing. It is evaluated each time under the Conduits of whoever opens it.

"Aggregate whatever we want" is limited by three things only, and each refusal names its rule:
- what the reader may see;
- arithmetic that would give wrong numbers: summing a gauge, adding cache reads to the input they are part of, averaging averages, or mixing units;
- your rule that tokens are never summed across model or account unless the query says so by name.

Drawing belongs to the client. The CLI shows tables in the first metrics batch. A Perses plugin and a PromQL bridge for Grafana follow.

WHAT CONDUIT DEFINES AND WHAT EXISTING TOOLS DO
Conduit defines only its own ground:
- whose a number is: the engine stamps it, never the reporter;
- the extra fields of a declaration;
- who may read which numbers at which detail;
- the promise that every number equals an aggregation over the records in its window, stated in every result.

Everything else is existing practice:
- names from pinned OpenTelemetry conventions;
- engine counters in OpenMeter's meter shape;
- account levels in FOCUS's terms;
- aggregation rules from OpenTelemetry instrument kinds and Prometheus practice;
- query field names borrowed from Langfuse's Metrics API, with filter operators spelled as CloudEvents SQL spells them;
- dashboards from Perses, and Grafana through a PromQL bridge;
- long history through the existing CloudEvents export into DuckDB, ClickHouse or an OpenTelemetry Collector.

The query's meaning (the split rule, overflow, exact percentiles, the two grains) is Conduit's own, because no standard covers it. The design says so rather than calling the query a standard.

USAGE BY ACCOUNT AND MODEL
Tokens are OpenTelemetry's five usage counters: input, output, cache read, cache write and reasoning. They are separate metrics, so a cache read can never be added to the input it is part of.
- Model comes from the instance's report: the model that answered, or else the model requested.
- Account is stamped by the engine from the account each Agent's Type declares (the signIn pointer), as it stood when the call was sent. Its status is stated: confirmed by sign-in at a given time, unconfirmed, or undeclared. A Type may also declare the FOCUS billing account and sub-account it bills to, for example an organization and its workspace or project.
- Model, provider and account are split dimensions. So the plainest query, "input tokens for Fleet coding this week", already returns one row per account, provider and model.
- A total across models or accounts exists only if the query names them in `without`, and the result then lists what was summed.
- A call whose report gave no model gets its own row, marked "not reported". It is never shown as zero.

On subscription accounts the one number that compares across models is the vendor's share used per limit window. It is reported per call as a gauge, never summed, and labelled account-wide, because it also counts use outside Conduit.

WHY THIS DESIGN
- Every surveyed system splits declare, then query, then display. That includes OpenTelemetry, Prometheus, Istio, Kubernetes, Langfuse, Anthropic's and OpenAI's usage APIs, OpenMeter and Stripe.
- Metering systems already derive usage from CloudEvents at read time, per subject. Conduit's records are CloudEvents, unsampled and kept for audit.
- At Conduit's volume, deriving from records is exact and needs no new store. Nothing is created per combination of dimensions.
- The other designs were rejected for reasons that were checked:
  - An external Prometheus-compatible store: no numbers until one is installed, and its numbers cannot be checked by Conduit's tests. VictoriaMetrics deduplication keeps "a single raw sample with the biggest timestamp" per interval, which drops per-call deltas, and its "Delete API doesn't support the deletion of specific time ranges". Prometheus's OTLP receiver rewrites dotted names unless an experimental mode is set.
  - Istio-style overrides that drop data when it is written cannot be undone, and one holder narrows what every reader sees.
  - Views as Resources lend numbers, which is the leak documented for Datadog shared dashboards and Mimir federated rules.

WHAT IT CHANGES
- It amends ruling 68 point 1 in ten explicit places.
- It settles M4: yes, there are shared standards, adopted rather than decided. The core ships OpenTelemetry's five GenAI token counters, pinned. It adds one Conduit-named gauge for subscription quota, because OpenTelemetry has none. Vendor-specific metrics go under each package's own address and never merge with another vendor's.
- It needs a twelfth contract kind, Metric.

WHAT MUST COME FIRST, AND THE COST
1. An engine storage order. My rerun of the benchmark: at 100,000 synthetic end records, rewriting the whole state costs about 2.5 seconds and a group-by scan 0.57 seconds; at a million, about 40 seconds and 12 seconds. In a probe store, about 97% of each invocation entry is a copy of its held target.
2. The usage probe of both clients. It adds the quota event and turns that end late.
3. Then the metrics batch, after batch 2. My rough estimate for the recommended scope: about 18 new and 20 amended promises, 2,400 to 3,200 engine lines with tests, and 60 to 70 vectors, plus one scripted migration of the vectors that assert root's operation lists. A minimal option is about half that.

WHAT YOU DECIDE
Six rulings, each with a recommendation:
- R1: the general feature, and its ten amendments to 68 point 1.
- R2: M4, and where declarations live: a Metric contract kind.
- R3: the query form: structured with borrowed names, or PromQL.
- R4: what "account" means, and whether numbers-only readers see account names. I now recommend that they do, because otherwise the billing reader gets exactly the shared sum you called useless.
- R5: where saved views live.
- R6: what the first metrics batch includes, with every deferral named for your explicit yes, as the scope guard requires.
Everything else is stated as a default you may strike.

## Rulings for the owner

RULINGS FOR THE OWNER (six). Each gives the recommendation first, then the other options.

R1. The general feature, and how it amends ruling 68 point 1.

RECOMMENDED (a): records are the only source, and there is one governed read.
- Every number equals an aggregation over the records in its window.
- metrics.v1 takes a describe form and several named sub-queries (R3).
- The engine counts every kind of record, its counters written in OpenMeter's meter shape over CloudEvents.
- Agents and Resources report declared points with each result: per-call counters and histograms, and gauges such as subscription quota.
- Late answers and recovered usage are kept, never lost silently.
- State numbers come later, read inside a read.

Amendments to 68 point 1:
- A1. "The engine counts from its records" becomes "every number is an aggregation over records".
- A2. "Attributes with closed values" becomes "declared keys, closed or bounded-open values". Model names are an open set.
- A3. The one _meta key now holds a list of points, plus kept-apart recovered points. It is read from plain results and stripped before relay.
- A4. The engine also stamps container, Type, implementation, revision, duration, and the account with its status and FOCUS levels.
- A5. State numbers come through a read-only hook, not a provider.
- A6. "Totals only" becomes the grains G1 and G2.
- A7. "Sums every member" becomes "covers every member and never sums across a split key unless the query names it".
- A8. "No new object kind" falls (R2). "No new family, store or wire" holds.
- A9. "Tokens only in v0" becomes "every declared metric".
- A10. "One read" becomes "one read with describe and sub-queries".

Kept as they are: M3; each package answers for its own numbers; the engine checks form, not truth; numbers decide nothing; no cost in v0; no relay to the calling Agent.

Other options:
- (b) Narrow: only split the totals by account and model. This does not meet "whatever we want ... all kinds of metrics".
- (c) A store behind a provider, queried in PromQL. It reverses "no store". There are no numbers until one is installed. Its numbers cannot be tested by Conduit's vectors. VictoriaMetrics deduplication drops per-call deltas, and its delete API cannot remove a time range.
- (d) Istio-style write-time overrides. Dropping data cannot be undone, and one holder narrows what every reader sees.

R2. M4: shared standards, and where declarations live.

Your question: "Do we have to decide on shared standards for that the same we do for agents?" Answer: yes, but adopted, not decided.

RECOMMENDED (a): a twelfth contract kind, Metric.
- Its spec is:
  - a pin: OpenTelemetry registry, commit, schema_url and metric name;
  - a resolved copy generated from Weaver's output, never edited by hand;
  - a `conduit:` block of Conduit's own fields: source, valueType, split, subsetOf, stateOf, bounds, disclosure, origin and derivedFrom.
- The core ships:
  - OpenTelemetry's five GenAI token counters, pinned to semantic-conventions-genai e6658f14;
  - one Conduit-named gauge, conduit.account.quota.used, because OpenTelemetry has none;
  - the engine's counters.
- db.client.operation.duration waits until a package needs it.
- Vendor metrics go under each package's address and are never merged across vendors.
- Items list what they report, so Types need no new version.
- Re-pinning makes a new version, as Stripe meters cannot change once configured.
- This amends OBJ-020 (eleven kinds become twelve), INS-001 and the contracts schema.

Other options:
- (b) No new kind: declarations sit inline on manifest items, and the core sets are a table outside the contracts. OBJ-020 stays, but there is a second declaration format and no way for third parties to ship a set.
- (c) Schema contracts as carriers. This bends CTR-008.

R3. The query form.

The research is plain. No standard structured metrics query exists:
- CloudEvents SQL v1.0.0 filters only, and "doesn't support the handling of the data field", where Conduit's record fields live.
- The CNCF query-language working group produced an evaluation rubric, not a standard.
- Apache Ossie (formerly OSI) 0.1.1 is a semantic model that does not execute queries.
PromQL is the only language existing tools speak.

RECOMMENDED (a): a structured query whose field names are borrowed and whose semantics are Conduit's own, said as such.
- Field names come from Langfuse's documented Metrics API v2 request (pinned at commit 0cb170c7). It has the same parts as Anthropic's and OpenAI's usage APIs and MLflow's trace metrics.
- Filter operators are spelled as CloudEvents SQL spells them.
- Conduit's stated additions: `from`, `without`, named sub-queries with `offset`, and `describe`.
- The engine checks every named field against the reader's grain before evaluating. Answers are exact integers over per-call deltas.
- Agent callers get a schema'd input (ruling 52).
- PromQL arrives through a bridge package for Grafana and Perses. The bridge translates a declared subset into metrics.v1. A mapping is published for sum, count, min and max over counters.

Other options:
- (b) A declared PromQL subset as the governed input. The engine parses it with promql-parser 0.11.1, compatible with Prometheus v3.8, checks the tree, injects scope matchers, and evaluates over records.
  - For: existing tools speak it directly.
  - Against: the checker is critical to security; rate() and increase() must be refused over deltas (CloudWatch warns they are "incorrect ... without a warning" on deltas); agent callers get an opaque string.
- (c) PromQL evaluated by an external store. This is R1(c).

R4. "By account": what it means, and who sees account names.

These parts go with the recommendation whichever option is chosen:
- The account is the pair (the Type's package address, the sign-in account at the signIn.account pointer), stamped at dispatch.
- Its status is stated: confirmed (with the time of the last account-hook check), unconfirmed, declared, or undeclared.
- A Type may also declare FOCUS's levels:
  - the billing account ("an invoice-issuer-assigned identifier for a billing account");
  - the sub-account ("a grouping of resources or services, often used to manage access and/or cost").
  For example, an organization and its workspace or project. Usage can then later be matched with FOCUS cost data. FOCUS 1.5 is scoped to add AI model identity and token consumption.
- Batch 1's .v2 Agent Types must state signIn. Records are immutable, so calls made before that can never be split by account.
- No control may reach the signIn pointers (CTR-013).
- At G1, members with undeclared accounts merge into one row, and the result says so.

Who sees account names.

RECOMMENDED (a): account is a usage dimension, not inventory. A numbers-only reader (the `metrics` role, G1) may split by account, billing account and sub-account, as well as by provider and model.
- Why: M2's numbers-only reader exists for billing. Under (b) it gets exactly the shared sum you called "useless".
- Cost:
  - One configuration value, the declared account string, becomes visible beyond inspectors (a narrow amendment of LIF-013).
  - Where each Agent has its own account, the split shows per-member numbers without member names. This is stated in every result.
- Member, consumer, Conduit, caller, node, Types, packages and versions, operations and rules stay inventory (G2).

Other options:
- (b) Account is inventory (G2 only). A numbers-only reader must name conduit.account in `without`, and the result says it summed across accounts. This keeps 59 A4 and LIF-013 whole. Lending the split to one numbers-only Conduit cannot be a requirement, because a restriction only narrows (ACC-008). It would need a new grain mechanism later.
- (c) Stable pseudonyms per reader (HMAC). This brings key custody, the labels can be linked across reads, it is useless for paying a bill, and it is new infrastructure.

Two sub-questions, recommended as stated, unless you strike them:
- (i) Type-identifying values. Provider, model and vendor metric names are open at G1, because they say what was measured, not which member. Package@version lists and operations are inventory.
- (ii) Quota gauges. A reading is account-wide, so it also reflects other Fleets' and claude.ai's use of that account. Recommended: readable by anyone who may see that account for a container holding a member with it, labelled account-wide. The Agent's own client already shows it to whoever drives the turn. Alternative: root only.

R5. Where saved views live, and how they are drawn.

RECOMMENDED (a): Perses.
- A saved view is a Perses dashboard (CNCF). Its panels use a Conduit query-plugin kind whose spec is a metrics.v1 sub-query.
- Opening a view sends one read holding every panel.
- Views are files with the person or in a project, or are shipped by packages as non-binding defaults that are copied when edited.
- A view grants nothing. It is evaluated under the opener's own Conduits.
- The CLI renders tables in Part A, and the Perses plugin comes in Part B.

Other options:
- (b) Views inside Interface configuration now, through a core interface.v2 `views` member. Root writes them. A scoped Interface editing its own views needs a self-control that the spec lacks.
- (c) Views as Resources that lend numbers. This is the documented leak of Datadog shared dashboards and Mimir federated rules. Only root can create them in v0, and they depend on batch-2 forwarding.

R6. What the first metrics batch includes (the scope guard, A18: every deferral needs your explicit yes).

RECOMMENDED (a): Part A as specified, with each deferral named.

Part A includes:
- the Metric kind and core sets;
- reports (counters, histograms, gauges, late, recovered);
- stamping with account levels;
- metrics.v1 with describe, sub-queries, offset, sums, counts, min and max, avg and nearest-rank percentiles, last for gauges, the split rule, G1 and G2, several containers, overflow, watermarks and explicit results;
- opt-in roles, Agents symmetric, and root's defaults;
- deadline alignment;
- CLI tables;
- the Codex and Claude Code reporting with fixtures.

Deferrals, each for a yes or a strike:
1. Formulas (ratios, and + and - in one unit), Part B. Until then the client divides the returned sums.
2. The Perses plugin and the PromQL bridge, Part B. Charts in v0 are tables in the CLI.
3. Replayed engine state and `fill` for unused grants, Part B.
4. metrics-grain.v1 narrowing, Part B.
5. The observe-metrics hook for state from instance endpoints, such as an index's size. Later.
6. The consumer side. v1, already accepted.
7. Thresholds. v1, already accepted, under the rules in the model.
8. A record-store provider. Later; it amends ruling 40 point 2.
9. Cost through a price-table provider. Later; "no cost in v0" is already accepted.
10. Owner-set labels (team, cost centre). Later; needs an envelope-labels ruling.
11. Scores or feedback about past calls. Later; a separate access question.
12. Percentiles that merge across engines. Later.
13. Codex quota. After the probe; it needs the app-server protocol.
14. Views inside an Interface Type. Later.

Other options:
- (b) Minimal Part A: a structured query with sum, count, min and max over counters, the split rule, G2 and root only, one container, and CLI tables. Everything else follows. About half the size: roughly 1,200 to 1,600 engine lines and 10 new promises. This honours "We shouldn't overcomplicate this", but the numbers-only reader and quota wait.
- (c) Parts A and B together in one batch.

DEFAULTS (they stand unless struck):
1. Cross-split sums are allowed when named in `without`, and every such result lists summedAcross. A ban would be Conduit's own invention.
2. Always refused, each with a named rule:
   - sums of gauges;
   - a subset added to its parent;
   - mixed units;
   - averages of averages or ratios.
   Ratios aggregate the numerator and the denominator, then divide.
3. OpenTelemetry's requirement levels are kept. conduit.model is the response model, else the request model, and the result says which. A missing model gets its own structurally marked row.
4. Plain packages: the engine reads the one metrics key where the item declares `reports` (IFC-010 amended) and strips it before relay.
5. Report caps (DL10), for example 32 points and 4 KB. A failing report is dropped whole with its rule, values never echoed. Coverage is per report. An absent point of an `always` metric is zero; otherwise it is not reported. Reports on failed results are kept.
6. Deadlines: for model-work items, the engine's dispatch deadline exceeds the wrapper's per-turn deadline (today 120 s against 180 s, the reverse). A late answer is kept as a report-late record with the outcome unchanged. A wrapper may report an earlier turn's usage later, as recovered points kept apart. Unknown turns are stated, never zero.
7. Roles are opt-in. `metrics` is a new role. Role lists such as [view, metrics] are accepted. `view` and `manage` keep their meaning, so no existing holder widens. The generated name is N--metrics--T. Agents' own Fleet entries accept `metrics` too, and there is no default metrics Conduit for Agents.
8. Root's standing Conduits gain metrics.v1 by ruled amendment (FLEET-005, FST-002). The vector migration is scripted once, together with batch 2's if that is still open.
9. Only the target side counts in v0. A refusal is attributed only through a Conduit the caller named and holds. A container's reads exclude requests whose target is the container itself.
10. LIF-013 is amended: inspecting a caller or consumer shows an end record's account and points only to readers who may inspect its target.
11. Time is UTC, with half-open windows and granularity minute, hour, day, week or month. Offsets are whole minutes, hours, days or weeks. Values are integers except declared double gauges. Percentiles are nearest-rank. avg is the exact sum divided by the exact count, with both returned.
12. M3: the minimum interval per caller stays, held in engine state. One read carries every sub-query and describe. Outputs are not retained.
13. OPS-008 is amended for unkeyed read-only engine operations (a minimal index entry).
14. Results carry per-source watermarks, and a window past one is marked incomplete (REC-006).
15. History ends at a purge, with the gap stated, and long history goes through the export. The purge-time rollup alternative amends REC-004 and is not recommended.
16. Thresholds (v1):
    - evaluated only at relevant end records;
    - never triggered by reads, firings or the pending actions they raise;
    - an engine-state running total is allowed if it equals the records' aggregation;
    - only firings are recorded;
    - raised only where the receiver holds metrics covering every container read, which amends ruling 66 point 3.
17. SLOs and alerts on absence are outside Conduit: the bridge plus Alertmanager or Sloth/OpenSLO, recorded only as the bridge's reads.
18. Unchanged: no cost in v0; clients' exporters stay off; the metric prefix comes from names.yaml (metricPrefix, set by tools/set_names.py).

## The design

0. VERDICT, AND WHAT CHANGED IN THIS REVISION

Base design: "records as wide events". It keeps the accepted shape of ruling 68 point 1:
- no new store or wire;
- two labelled sources;
- the engine stamps whose a number is.

Rejected designs, and why:
- An external Prometheus-compatible store as the source of numbers:
  - no numbers until a store is installed and secured;
  - its answers are outside Conduit's vectors;
  - VictoriaMetrics deduplication "would leave a single raw sample with the biggest timestamp per each discrete interval", which drops per-call deltas;
  - its "Delete API doesn't support the deletion of specific time ranges", so a purge cannot be honoured;
  - Prometheus's OTLP receiver defaults to "UnderscoreEscapingWithSuffixes", and "NoTranslation" is "(EXPERIMENTAL)";
  - it needs a PromQL rewriter that is critical to security;
  - it reverses M1's "no new store".
  The Mimir range-deletion issue (#4968) and remote-write backfill limits are the skeptics' claims and were not re-read.
- Write-time overrides in Istio's style: they solve a cardinality cost Conduit does not have, they drop data irreversibly, and one holder narrows what every reader sees.
- Views as Resources: only root creates them in v0, they lend numbers, and they depend on batch-2 forwarding.

What this revision adds, after the critic's review and a further round of research:
- a describe form;
- several named queries per read, with time offsets;
- per-call gauges, including subscription quota;
- late and recovered reports;
- deadline alignment;
- account levels named after FOCUS;
- engine counters in OpenMeter's meter shape;
- engine-known state and unused grants (Part B);
- per-source watermarks;
- opt-in roles that widen no existing holder;
- Agents treated symmetrically;
- G1 leaks closed;
- the declaration format made honest: a pin plus a generated resolved copy, not "verbatim";
- coverage counted per report;
- thresholds that cannot trigger themselves;
- a scope ruling.

1. LAYERS: WHAT IS CONDUIT'S AND WHAT IS NOT

Conduit's own part (ruling 40) is:
- attribution;
- the extra fields of declarations;
- who reads which numbers at which grain;
- the equivalence promise;
- explicit results (ruling 52).

What is not Conduit's:
- How queries are evaluated and records stored. These stay the engine's choice behind the equivalence promise. The first build scans. Once records move to a segment log it may embed Apache DataFusion or DuckDB. A swappable record-store provider comes later.
- Long history and heavy analysis, through the existing export (DL8, CloudEvents JSON lines).
- Display: Interface clients, Perses, and Grafana through a bridge.
- Prices: none in v0.
- SLOs and alerts on absence. These are possible only outside Conduit, through the PromQL bridge plus an external evaluator such as Prometheus Alertmanager or Sloth/OpenSLO. That path is outside Conduit's governance, and is recorded only as the bridge's reads.

2. RECORDS ARE THE ONLY SOURCE (MET-001)

Promise: every number that metrics.v1 returns equals a stated aggregation over the records in its window (and, later, over readings taken within that read). An engine may index or cache only where the answer is identical. This is ClickStack's equivalence for materialized views.

Tests:
- Aggregation cases are written as data: records in, query in, rows out.
- Two harnesses read them: the core runner, and a reference evaluator over exported records. The evaluator is a Python checker under tools/, which is the one exception AGENTS.md allows.
- Any later record-store provider must pass both.

Consequences:
- There is no registry of derived series and there are no recording rules.
- A new breakdown reaches back over all kept history.
- Several sources merge exactly, because no running totals are pushed.

Engine prerequisites, as a storage order before the metrics batch. These are build-only changes under DL2, DL10 and DL20, plus one promise amendment.

(a) Index entries reference the held target by ID and revision instead of copying it.
- Measured 2026-11-11 in the probe store of 2026-11-10 (scratchpad/probe-store/state.json): each install.v1 invocation entry is about 129.6 KB, of which held_target is 126.3 KB (97%).
- The earlier "174 KB at 98%" for a read is not on file and is withdrawn.

(b) Records move to an append-only segment. REC-003 makes records immutable, which allows this. The core reads records through an interface rather than from a Vec in State.

(c) OPS-008 is amended so that an unkeyed request to a read-only, engine-implemented operation (inspect.v1, list.v1, metrics.v1) keeps a minimal index entry. This needs the owner's yes (see the defaults).

(d) metrics.v1 outputs are not retained under DL10.

(e) Evaluation runs outside the single writer lock:
- decide and record;
- evaluate over a snapshot as an intent;
- end.
A DL10 bound on the records scanned per read is refused by name when exceeded.

(f) The shell measures each attempt on a monotonic clock, from sending it to receiving the answer. It supplies the result as evidence, and the core stamps an integer durationMs in the end record.

Cost evidence: the skeptic's synthetic bench, rerun 2026-11-11, one run on this machine. Records are 3.9 KB each with 15 points; output is in scratchpad/bench-out/results-2026-11-11.txt.

| Records | serialize | parse | write+fsync | group-by scan |
|---|---|---|---|---|
| 100,000 | 0.91 s | 1.40 s | 0.15 s | 0.57 s |
| 1,000,000 | 18.0 s | 18.4 s | 2.7 s | 12.1 s |

The binding cost is the whole-state rewrite, not the scan, but the scan over a parsed JSON tree does not scale past about 10^6. Before M3's interval is fixed, the order measures one read under D19.

3. THE SOURCES (each labelled in every result: engine, reported, observed)

3a. Engine-counted (MET-002)

The core package declares one counter for each kind of thing REC-001 records. Each counter's derivation is written in OpenMeter's meter shape, which is the existing precedent for meters over CloudEvents:
- eventType: the record type;
- valueProperty: a JSONPath into data (absent means count);
- aggregation;
- groupBy: JSONPaths.
Each logical event counts once, never once per record.

The counters:
- conduit.requests, counter {request}. One per request, at its end, whether dispatched or not. Attributes: operation, outcome (succeeded, failed, unknown) or reason (refused, invalid, conflict, unavailable), model_work, rule, refusal_tier.
- conduit.invocations, counter {invocation}. One per dispatched attempt, at its first end. A later reconciliation changes no count. Turns are invocations with model_work=true.
- conduit.invocation.duration, histogram, ms, int. Taken from durationMs. Absent for attempts that end at the deadline or are reconciled; coverage counts those.
  - It is a Conduit name, because OpenTelemetry's mcp.client.operation.duration is in seconds and requires mcp.method.name. A bridge maps it on export.
- conduit.reports, counter {report}. One per invocation whose item declares reports. Attribute status: reported, not_reported, dropped (with the rule) or late.
- Disclosure class inventory (G2 or root): conduit.provider.calls, conduit.object.changes, conduit.metrics.reads.

3b. Reported per call (MET-004)

Format:
- With each result, the implementation puts one _meta key, io.github.shrav-k.conduit/metrics.
- It holds {points: [{name, value, attributes}], recovered?: [{points}]}.
- It is a list because one Claude Code turn runs several models (main loop, subagents, auxiliary calls), and one Qdrant batch runs several searches.
- The engine strips this key from the result it relays to the caller. This keeps 68's default that usage is not relayed to the calling Agent.

Kinds that may be reported:
- counter and histogram. Values are per-call deltas, int in v0, so sums are exact.
- gauge. A reading at the time of the end record. The value type is as declared: int or double.
  - Its series identity is declared by `stateOf`:
    - member: per member, such as context used;
    - account: per account, such as subscription quota, where two members on one account read the same meter.

Converting the client's running totals is the package's job, proven with fixtures from real runs:
- Codex exec's turn.completed.usage is the thread total, confirmed in source.
- Claude Code's modelUsage is a running total, restored on resume and zeroed on a crash.
A package that cannot know the previous total reports nothing for that turn and never guesses (OpenTelemetry's cumulative-to-delta initial-value case).

The engine checks form, never truth:
- the name is in the item's `reports`;
- attribute keys are declared;
- values are in their closed set, or within the declared maxLength under an engine-fixed character set;
- required attributes are present, at OpenTelemetry's requirement levels unchanged;
- counter values are positive integers where the pinned convention says "MUST be a known, strictly positive" contribution, and non-negative elsewhere;
- points and bytes are within DL10 caps (for example 32 points and 4 KB);
- no key is one the engine stamps. Reported attributes that a core Metric contract declares under the conduit prefix are allowed;
- no key collides with an engine key after OpenTelemetry-to-Prometheus name translation.

A failing report is dropped whole. The drop is recorded with its rule, and the record never echoes the dropped values. A drop never changes the outcome.

Coverage is per report (per invocation), not per measure:
- An item lists each reported metric with `always: true|false`.
- In a report whose status is reported, an absent point of an always metric means zero. This matches OpenTelemetry's strictly-positive rule, under which a turn with no cache reads sends no cache_read point.
- An absent point of a metric that is not always reported means not reported for that measure.

Failed results: their reports are kept. OpenTelemetry says instrumentation "SHOULD record" failed operations.

Late and recovered reports (MET-018):
- Deadlines are aligned. For an item whose operation starts model work, the engine's dispatch deadline is longer than the wrapper's per-turn deadline. DL10 gains a per-item value the manifest may declare. Today it is the reverse: 120 s dispatch against a 180 s wrapper deadline (engine/CONFORMANCE.md). So the wrapper answers first, with a failure and whatever usage its stream carried.
- A late answer is one that arrives after the engine's deadline for an attempt that ended unknown, while the run still holds the session. The engine keeps its validated report as a new record, report-late, referencing the invocation. The outcome is unchanged (OPS-013). The tokens count in the bucket where that record lands, and the result says so. This amends REC-001's list of records.
- A wrapper that later learns an earlier turn's usage reports it in a later report's `recovered` list, kept as recovered and never merged into that turn's own delta. Example: reading the client's local session log, as ccusage does.
- Otherwise coverage says "tokens of N turns unknown", never zero.

Plain implementations: both Agent packages are plain: true. Where the item declares `reports`, the engine reads this one key from plain results (an IFC-010 amendment). IFC-007 is unchanged.

3c. Observed state (later, not in the metrics batch)
- A read-only hook, observe-metrics.v1, is selected by the Type under TPL-004.
- It is dispatched only within a metrics.v1 read that names an observed metric, and only to members in the reader's reach.
- It changes nothing and starts no model work.
- Its answer is kept as a reading record. A member that does not answer is Unknown, with its last time (OBS-002, OBS-003).
- It may read the instance's own /metrics, which the gate keeps from outside scrapers (PRV-025).
- Kinds: updowncounter and gauge.

3d. Engine-known state and unused grants (Part B, G2 only)
Replayed from records the engine already writes, as kube-state-metrics derives from object state:
- conduit.conduits.open;
- conduit.instances by phase, at the times changes were recorded or observed;
- conduit.pending.outstanding;
- conduit.pending.answer_time (histogram, raise to answer).
`fill: [conduit.conduit | conduit.member]` seeds explicit zero rows from the objects that existed in the window. This shows unused grants, following AWS IAM Access Analyzer's unused access findings ("Roles with no access activity within the specified usage window"). Counts of objects are inventory, so G2 only.

4. DECLARATIONS: THE METRIC CONTRACT AND M4 (MET-003)

Metric is a twelfth contract kind. It is not a family. It amends OBJ-020, INS-001 and the contracts schema.

Its spec has three parts:
- (i) pin: registry repository, commit, schema_url and metric name;
- (ii) resolved: a copy of that metric and its attributes, generated by a tools/ script from Weaver's resolved form. The source YAML is definition/2, with ref and ref_group. Weaver's resolved form is resolved/2.0, with an attribute catalog. Generating that copy from Weaver's actual output is a probe and a precondition. The claim that OpenTelemetry's checker reads the document as it stands is withdrawn: Conduit's envelope wraps it.
- (iii) a `conduit:` block holding Conduit's only additions, each labelled as such:
  - source: engine, reported or observed;
  - valueType: int or double, from the convention's annotations.code_generation.metric_value_type;
  - derivedFrom: OpenMeter-shaped; core package only;
  - split;
  - subsetOf;
  - stateOf: gauges only;
  - bounds: maxLength only;
  - disclosure per attribute: open or inventory;
  - origin per attribute: instance or input. Input-origin attributes are refused where records.inputs is false.

Who declares what:
- The core package ships the engine counters and two shared sets.
  - The GenAI usage counters, generated from semantic-conventions-genai commit e6658f14 (schema_url gen-ai-dev/1.42.0-dev, Development, no release tag): input_tokens, output_tokens, cache_read.input_tokens, cache_write.input_tokens and reasoning.output_tokens. All int.
    - split = gen_ai.provider.name, conduit.model, conduit.account.
    - modality is not split, because OpenTelemetry's design note says summing across modality equals the known total.
    - Identity attributes are excluded: gen_ai.agent.*, gen_ai.conversation.id, server.address, user.*, enduser.*.
  - conduit.account.quota.used, a gauge in %, double, stateOf account. Attributes: conduit.quota.window and conduit.quota.scope, bounded strings set by the package, such as 5h, 7d, all or a model family. It is Conduit-named because OpenTelemetry has no quota metric. disclosure: account-wide, see section 7.
  - db.client.operation.duration is dropped from v0's core sets. It is a double in seconds and would duplicate conduit.invocation.duration for single-operation calls. It returns when a Resource package needs it.
- Vendor metrics are declared by their own package and named under the package address, for example example.org/conduit/qdrant/searches. They never merge with another package's metrics.
- Project-local Metric contracts keep `#name` (NAM-022). Only the project's own items report them, and they never merge.
- An implementation item lists `reports: [{metric, always}]`. Putting the list on the item, not the Type, means adding metrics never forces a new Type version (NAM-024).

Install-time checks:
- Two streams merge only when they come from the same Metric contract. Engine-counted and reported streams never merge.
- Engine names and stamped keys are reserved.
- Duplicate names on one item are refused.
- A package narrowing a shared set may only:
  - narrow a closed list;
  - lower maxLength. Narrowing by pattern is refused as undecidable;
  - add attributes under its own prefix that change neither identity nor split, such as Claude Code's query_source, or a cache-write TTL as OpenTelemetry's design note allows.
- conduit.account is always split for reported counters, whatever the package says. A package's `split` may name only conduit.account and its own attributes.
- Re-pinning creates a new contract version and never edits one in place, as Stripe meters cannot change after configuration. .v1 and .v2 points never merge.

5. DIMENSIONS AND STAMPING (MET-005)

Each point carries two separate maps, following OpenTelemetry's split between resource and data-point attributes, OpenMeter's subject, and Prometheus's honor_labels=false.

`whose`: stamped by the engine into the request-ended and invocation-ended records, labelled attested:
- member (the target) and container;
- consumer, consumerContainer, conduit, caller;
- node (the REC-005 source);
- operation, model_work;
- outcome, reason, rule, refusal_tier.
Only where the item declares reports:
- revision;
- implementation (package@version and item);
- type;
- account and its levels;
- the validated points.

Account:
- The sign-in account. The key is the pair (the Type's package address, the declared account string). Its value is taken at dispatch from the Type's signIn.account pointer in the revision dispatched (CTR-013).
- Status:
  - confirmed: account-hook matched at time T, and the configuration has not changed since;
  - unconfirmed: no check yet, or the configuration changed since the last check;
  - declared: an API-key configuration with no hook, stamped from a non-secret organization or project identifier the Type declares;
  - undeclared.
- FOCUS levels, optional. A Type's signIn may also point at the billing account and sub-account the configuration bills to. They are stamped as conduit.billing_account and conduit.sub_account with FOCUS 1.4's meanings:
  - BillingAccountId is "an invoice-issuer-assigned identifier for a billing account";
  - SubAccountId is "An ID assigned to a grouping of resources or services, often used to manage access and/or cost".
  Status declared. Examples (my reading, to be checked by the probe): Anthropic API is organization plus workspace, OpenAI API is organization plus project, ChatGPT Business is the workspace, Claude Max is the claude.ai account. These names align with FOCUS 1.5, which is "scoped to surface AI model identity and token consumption", so a later FOCUS export needs no renaming. They are re-checked when 1.5 is ratified.
- CTR-013 is amended so that no control's `changes` reaches the signIn pointers. Otherwise a control holder could relabel usage.
- Undeclared accounts:
  - At G2 with account kept, each member's undeclared account stays its own group, labelled with the member.
  - At G1, undeclared groups merge into one row, and the result says so.
  - Under `without`, everything merges, and the result says so.

`attributes`: supplied by the instance, declared keys only, labelled reported. Examples: gen_ai.provider.name, gen_ai.response.model, gen_ai.request.model, gen_ai.token.modality, conduit.quota.*, vendor keys.

Derived at read time:
- conduit.model is gen_ai.response.model, else gen_ai.request.model, else structurally absent. The result names the source.
- An absence is marked structurally ({value: null, notReported: true}), never as a string a reporter could forge.

Withheld from inspection (amends LIF-013): when inspect.v1 returns a record that names the inspected instance as caller or consumer, the record's stamped account and points are shown only to a reader who may also inspect the record's target.

6. THE READ: metrics.v1 (MET-006 to MET-009, MET-013, MET-014, MET-017)

The shape is honest about its origin. Field names are borrowed from Langfuse's documented Metrics API v2 request (GetMetricsV2Request; data model at commit 0cb170c7): metrics {measure, aggregation}, dimensions {field}, filters {column, operator, value, type}, timeDimension {granularity}, fromTimestamp, toTimestamp, orderBy, config.row_limit. Rows are keyed <aggregation>_<measure>. Anthropic's analytics API, OpenAI's usage API and MLflow QueryTraceMetrics have the same parts.

The semantics are Conduit's own, because no standard covers them:
- CloudEvents SQL v1.0.0 is a predicate language that "doesn't support the handling of the data field".
- The CNCF query-language working group produced an evaluation rubric, not a standard.
- Apache Ossie (formerly OSI) 0.1.1 is a semantic model that does not execute queries.
PromQL is the one language existing tools speak. It arrives through a bridge (R3).

Conduit's stated changes to Langfuse's shape:
- `from` replaces `view`. It names containers by reference, or `management` for root's whole-system read.
- `without` is added. It names split keys deliberately aggregated away, following Prometheus's rule-writing practice ("Always specify a without clause with the labels you are aggregating away").
- `queries` is a list of named sub-queries, each with its own from, metrics, dimensions, without, filters and timeDimension, and an optional `offset`. Up to a DL10 number, for example 10, like the several queries Datadog and CloudWatch widgets and Grafana panels hold. The window is shared at the top level.
  - offset is an ISO 8601 duration in whole minutes, hours, days or weeks (no months), and shifts that sub-query's window back. This follows PromQL's offset and NRQL's COMPARE WITH.
  - All sub-queries are evaluated as one read, one record and one grain check.
- `describe: true`, alone or with queries, returns the catalog for the named containers at the caller's grain:
  - measures, instrument, unit, valueType, source, split, subsetOf and allowed aggregations;
  - the dimensions usable at this grain;
  - the values of open dimensions seen in the window, capped, with an overflow marker.
  Values of dimensions the caller may not use are not listed (prom-label-proxy disables its label endpoints for this reason). At G1 the declarations list (package@version) is omitted.

Filters:
- The operators are spelled as CloudEvents SQL spells them, with its comparison semantics: =, !=, <, <=, >, >=, IN, NOT IN.
- LIKE is not offered in v0: a pattern over values could probe values the caller cannot list.
- Every named field, in any clause, is checked against grain before evaluation.

Default dimensions: if absent, the union of the measures' split keys. This is the semantics of OpenTelemetry's advisory Attributes parameter.

Split rule (MET-008): every split key of every measure must be kept, fixed by a one-value = filter, or named in `without`. Otherwise the query is invalid, and the diagnostic names the keys and the three ways to satisfy the rule. A query that uses `without` lists summedAcross in its result.

Aggregation by kind (MET-007), refused otherwise with a named rule:
- counter and histogram (per-call integer values): sum, count, min, max, avg, p50, p75, p90, p95, p99.
  - avg is the exact integer sum divided by the exact count in one binary64 division. sum and count are returned with it.
  - Percentiles are nearest-rank: the value at rank ceil(p/100*N), always an actual sample.
- gauge: last value per series (stateOf key plus attributes) per bucket. Then min, max or avg across series, evaluated over series sorted by label tuple so the result is deterministic. Never sum.
- Later, updowncounter: last per member per bucket, then sum, min, max or avg across members. Never summed over time.
- At G1, no request may reveal how many members a container has. Sum and avg across members together, and a count of series, are refused.
- Always refused:
  - mixing units;
  - a subset added to its parent;
  - an average of averages or ratios;
  - Langfuse's `histogram` aggregation and its `auto` granularity in v0.

Time (MET-009):
- UTC instants. The range [from, to) is half-open.
- granularity is minute, hour, day, week (ISO, Monday 00:00 UTC) or month (UTC calendar). The client resolves local periods to UTC.
- A call counts in the bucket of its end record. A late report counts in its own record's bucket.
- No extrapolation. A window total is an exact sum, unlike PromQL increase().

Several containers: each record counts once, and each row carries conduit.container when it is kept. Results return mergeable forms (sum, count, min, max) so a client never adds percentiles.

Limits (DL10): maximum window (for example 31 days), dimensions (5), sub-queries, points, records scanned, and the minimum interval per caller.

Overflow: groups beyond row_limit fold into one row marked otel.metric.overflow=true. It is never dropped and never double-counted. Which groups survive is decided once per read over the whole window by orderBy (default: the first measure, descending), with ties broken by the label tuple. The overflow row names the dimensions it spans.

Every result (metrics-result.v1) carries:
- the normalized query;
- covers: each container and its grain;
- the fixed statement "numbers from containers not in covers are never included; whether any exist is not said";
- the statement "calls through this engine only; a vendor account's own meter also counts use outside it";
- per measure: metric, instrument, unit, valueType, source and pin. At G2 it also lists declarations;
- per row: the bucket start, and each dimension marked attested, reported or derived;
- summedAcross;
- coverage:
  - reports: reported, not_reported with cause, dropped with rule, late, recovered;
  - attempts with no duration;
  - "tokens of N turns unknown";
  - gaps: purges per window (REC-006);
- watermarks: for each record source (node), the time through which its records are held. A window past any watermark is marked incomplete. This prepares for v1 local enforcement, where machines cut off from the rest write records that arrive late (ruling 40 3f-B).
- the overflow row;
- no count of anything withheld.

PromQL: a mapping is published only for sum, count, min and max over counters: sum by (D) (sum_over_time(m{F}[step])), with OpenTelemetry-to-Prometheus name translation. No general equivalence is claimed.

7. ACCESS AND DISCLOSURE (MET-010 to MET-012, MET-015, MET-016)

Routes (a new ACC-033 row):
- metrics.v1 is authorized for each container in every sub-query's `from` by an open Conduit from the caller to that container listing metrics.v1, routed like rows 6 and 6A.
- If any container is refused, the whole read is refused, naming each under ACC-056's tiers. No partial results and no silent emptying (Mimir: "No partial results will be saved").
- Through `management`, root alone reads:
  - retired containers;
  - records that name no container;
  - records whose target is a container, `management` or `access`;
  - inventory-class counters system-wide.
- The default is deny.

Which side, and attribution (MET-012):
- In v0 a call counts only in its target's container. The consumer side is v1, as accepted.
- A refused request is attributed to a container only where it named a Conduit the caller holds, open or closed. It is never attributed by resolving names in its inputs. This closes an oracle.
- Requests whose target is the container itself are excluded from that container's reads. Root reads them through management.

Grain per container (MET-011), checked over every field named anywhere and over the default dimensions, before evaluation:
- G2: metrics.v1 plus inspect.v1 on the container. Every dimension, with names.
- G1: metrics.v1 only.
  - May use: time; conduit.container (its own); model_work; outcome; reason; report status; conduit.model; gen_ai.provider.name; reported attributes marked open; vendor metric names. Also conduit.account and its FOCUS levels, under R4's recommendation.
  - May not use, each refused by name and never silently removed: member, consumer, consumerContainer, conduit, caller, node, revision, type, implementation and its declarations list, operation, rule, refusal_tier, reported attributes marked inventory.
- Account-scoped gauges (quota) are readable by any reader who may see that account for a container holding a member with it. Each one is labelled "account-wide: includes use outside this container and outside Conduit". R4 gives root-only as the alternative.

Roles, opt-in, widening nobody (amends SCT-012):
- A `manages` value may be one role or a list, such as {coding: [view, metrics]}.
- The new role `metrics` expands to Conduit N--metrics--T from the new core templates fleet-metrics or pool-metrics, which list metrics.v1 alone.
- `view` and `manage` mean exactly what they mean today, so documents already written gain nothing (ruling 66 point 6). This amends ruling 66 point 1's "manage or view".
- Agents are treated symmetrically. An Agent's own-Fleet entry accepts `metrics` too, and an Agent may be granted any metrics Conduit like any other Conduit (preservation test 5). There is no default metrics Conduit for Agents in v0. Ruling 68's default is kept: a call's result carries no usage back to the Agent that made it.
- Root's standing Conduits gain metrics.v1 by ruled amendment (FLEET-005, FST-002), and management lists it.

Later narrowing: metrics-grain.v1 (Part B). It is a core requirement the engine applies, as bound.v1 is, because a filter cannot apply to an engine-implemented operation (CON-022, MAN-012). It only narrows (ACC-008):
- metrics: an explicit list;
- dimensions: an explicit list;
- holdTo: field to values.
Lists are positive and closed, with no wildcard and no "[] means all". ACC-030's "within" compares grains too.

Recording (MET-015):
- Every read is a recorded request, and its record keeps the query.
- The minimum interval per caller is held in engine state, so a purge does not reset it.
- One read carries every sub-query, so a dashboard is one read.
- A view's read is its opener's read.
- Reads start no model work and change nothing.

Stated limits (MET-016):
- Totals hide names, not numbers.
- A container with one member shows that member's numbers.
- Differencing two windows isolates a member (Denning, Denning and Schwartz).
- Grouping reveals how many models or accounts exist.
- Where each member has its own account, the account split shows per-member numbers, without names.
- Conduit adds no suppression or noise.

Exports and bridges: a bridge holds its Interface's authority. Everyone using the bridge reads at that grain, recorded as that Interface. Once scraped, the data leaves Conduit's governance. Per-person Interfaces are recommended where accountability matters.

8. CUSTOMIZATION, SAVED VIEWS AND DISPLAY (R5)

Four layers:
1. Declarations set the defaults: split keys give the default grouping, and the kind gives the allowed aggregations.
2. describe.
3. Any ad hoc query.
4. The saved view.

A saved view is a Perses Dashboard (CNCF; kind/metadata/spec; panels separate from layouts):
- Panel queries use a query-plugin kind, ConduitMetricsQuery, whose spec is a sub-query without timestamps. The dashboard's duration supplies the window, and the client resolves it to UTC.
- Opening the dashboard sends one read holding every panel's sub-query.
- Conduit defines only that plugin's spec.
- Views are files kept by the person or in a project. Packages may ship them as non-binding defaults; editing one makes a copy.
- The CLI renders views as tables in Part A. A Perses plugin package comes in Part B.
- A view grants nothing. It is evaluated under the opener's current Conduits, never as its author and never as the system.

Grouping by labels the owner sets (team, cost centre) waits for a ruling on envelope labels. If adopted, the engine would stamp them from the object's document at dispatch.

9. WHERE COMPUTATION HAPPENS

- In the engine, at read time, over records, outside the writer lock, inside the read's invocation.
- No timers, subscriptions or background rollups (IN_FORCE topic 1, OBS-004).
- Reads are per engine, and peers hold their own records.
- Existing tools take the rest:
  - the export (DL8) into DuckDB read_json, ClickHouse, Vector or the OpenTelemetry Collector filelog receiver;
  - later, a PromQL bridge for one Interface. It translates a declared subset (promql-parser 0.11.1, which "declares compatible with prometheus v3.8") into metrics.v1, and exposes OpenMetrics cumulative totals with created timestamps;
  - later, a record-store provider (DataFusion, DuckDB or ClickHouse), held to the data vectors. It amends ruling 40 point 2's v0 list.

10. RETENTION (a default; the alternative is named)

- Records stay until root purges them.
- A window that touches a purge reports the gap, with its interval.
- Long history goes through the export.
- Kept points copy their instrument, unit and valueType, so old records never depend on an installed package.
- The alternative: a purge record that keeps per-day rollups at the G1 dimensions, as ClickHouse TTL GROUP BY and Datadog's 15 months do. It amends REC-004's "removes records and nothing else", and it is a stored aggregate whose dimensions are fixed once, which this design avoids elsewhere.

11. THRESHOLDS (v1; the batch must not foreclose them)

- A threshold names its holder.
- It is evaluated only when an end record its query can reach is written. Metrics reads, threshold firings and the pending actions they raise never trigger thresholds, so evaluation cannot recurse.
- The engine may keep a running total per threshold as its own state, as OPS-008 keeps state, provided the total equals the records' aggregation under MET-001. This is the place where pre-aggregation pays (the events survey, lesson f).
- Evaluations are part of the triggering invocation, as PND-007's notice is. Only firings are recorded, so there are not N records per call. A firing is rate-limited as the holder's read.
- Dimensions are checked against the holder's grain when the threshold is declared.
- A firing raises a pending action only at Interfaces holding metrics.v1 that covers every container the threshold read. This amends ruling 66 point 3, which raises at every Interface whose scope holds the subject. The amendment is labelled as such.
- An alert on absence is impossible inside Conduit. It lives outside, through the bridge (section 1).

12. RULING 68 POINT 1, AMENDED EXPLICITLY

- A1. "The engine counts ... from its records" becomes "every metrics.v1 number equals an aggregation over records". The engine's counters are core Metric contracts in OpenMeter's meter shape.
- A2. "Attributes with closed values" becomes "declared keys, closed or bounded-open values". The cardinality guard is the per-read row limit with OpenTelemetry's overflow row.
- A3. "One _meta key" becomes "one key holding a list of points: per-call deltas, and gauges with a declared stateOf; recovered points kept apart". It is read from plain results too, and stripped from what is relayed to the caller.
- A4. "Takes whose it is from its own record" is kept and extended. The engine also stamps container, type, implementation, revision, durationMs, and the account in force with its status and FOCUS levels. A reported key equal to a stamped key drops the report.
- A5. "State numbers ... through a provider" becomes "through a read-only hook the member's Type selects, inside a read". Still later than v0.
- A6. M2's "totals only, no breakdown, no names" becomes the grains G1 and G2. Totals-only stays expressible.
- A7. The proposal's "sums every member the container has had" becomes "covers every member, retired ones included, and never sums across a split key unless the query names it".
- A8. "No new family, store or wire" holds. "No new object kind" does not: Metric is a twelfth contract kind.
- A9. "Tokens only in v0" becomes every declared metric, plus the engine's counters, plus quota gauges.
- A10. "One metrics.v1 read" becomes "one metrics.v1 read carrying a describe form and several named sub-queries".

Unchanged:
- M3;
- each package answers for its own numbers;
- the engine checks form, not truth;
- numbers decide nothing;
- usage is not relayed to the calling Agent;
- the consumer side in v1;
- thresholds in v1 (section 11);
- client exporters stay off;
- the prefix comes from names.yaml.

13. SPEC IMPACT AND SIZE (rough, my estimates)

New:
- spec/conventions/metrics.yaml, MET-001 to MET-018. Later MET-019 to MET-023: hook, grain requirement, formulas, thresholds, replayed state and fill.
- About 12 terms in definitions.md.
- Core: the Metric kind schema; the engine Metric contracts; the generated sets and the quota gauge; metrics.v1; metrics-query.v1, metrics-result.v1 and metrics-catalog.v1; the fleet-metrics and pool-metrics templates.

Amended:
- OBJ-020, INS-001 and the contracts schema; a new CTR for Metric;
- MAN (item `reports`, `always`, `narrow`, and the per-item deadline);
- CTR-013;
- REC-001 (report-late), REC-002 and record.v1, REC-006 (per-window gaps and watermarks);
- LIF-013;
- IFC-010;
- ACC-033 and ACC-056;
- SCT-012 (role lists, the `metrics` role, Agents);
- FLEET-005 and FST-002;
- OPS-008;
- pool-management.v1 and fleet-management.v1;
- DL10 and DL20;
- names.yaml.

Engine, full Part A: about 2,400 to 3,200 lines with tests, beside the storage order.

| Part | Lines (about) |
|---|---|
| Stamping and account levels | 300 |
| Report validation, gauges, late and recovered | 500 |
| The Metric kind in the loader and core (five kind lists in four crates) | 250 |
| Query, split and grain checks | 650 |
| Evaluator and describe | 650 |
| Results and watermarks | 300 |
| Routing, roles and templates | 200 |
| Evaluation outside the lock | 300 |

Vectors: 60 to 70, mostly aggregation cases as data over one shared snapshot of about 250 records, plus one scripted migration.

Minimal Part A (R6(b)): about 1,200 to 1,600 lines, about 10 new promises and 12 amended, about 30 vectors.

14. REMAINING RISKS (accepted, and stated in results)

- Truth is unchecked; fixtures are the guard.
- Account totals cover only calls through this engine.
- Codex's model, quota and late usage depend on the probe.
- The pinned GenAI conventions are at Development status and will move. So will FOCUS 1.5.
- History ends at a purge unless exported.
- G1 leaks by small groups and by differencing.
- Bridges are broad credentials.
- Reads cost linear time until the storage order and a store land.
- The split marker is Conduit's own, and packages may mark it wrong. The engine always forces the account split.

## Example documents

# Illustrative only, not spec text.
# - core#x means github.com/shrav-k/conduit/core#x.
# - `conduit.` stands for the metric prefix a new names.yaml entry (metricPrefix) would fix.
# - model-a/b/c and the email addresses are placeholders.
# - 25619 is turn 2 minus turn 1 of experiment 008, run C1 r1 (63,087 - 37,468).
# - Other numbers are invented.

# =====================================================================================
# 1. CORE PACKAGE: Metric contracts (twelfth contract kind)
# =====================================================================================
apiVersion: github.com/shrav-k/conduit/v0
kind: Metric
metadata:
  name: requests.v1
  description: One per request, at its end, whether or not dispatched (REC-001).
spec:
  resolved:                          # Conduit-named: OpenTelemetry has no access-decision metric
    name: conduit.requests
    instrument: counter
    unit: '{request}'
    brief: Requests that ended, by outcome or reason.
    stability: development
    attributes:
    - {key: conduit.operation, type: string, requirement_level: required}
    - {key: conduit.outcome, type: {members: [succeeded, failed, unknown]}, requirement_level: conditionally_required}
    - {key: conduit.reason, type: {members: [refused, invalid, conflict, unavailable]}, requirement_level: conditionally_required}
    - {key: conduit.model_work, type: boolean, requirement_level: required}
    - {key: conduit.rule, type: string, requirement_level: conditionally_required}
    - {key: conduit.refusal_tier, type: int, requirement_level: conditionally_required}
  conduit:
    source: engine
    valueType: int
    derivedFrom:                     # OpenMeter's meter shape over CloudEvents records
      eventType: request-ended
      aggregation: COUNT             # no valueProperty: each event counts once
      groupBy:
        conduit.operation: $.operation
        conduit.outcome: $.outcome
        conduit.reason: $.reason
        conduit.rule: $.ruleId
        conduit.refusal_tier: $.refusalTier
      attribution: a refusal counts in a container only through a Conduit the caller named and holds
    disclosure: {conduit.operation: inventory, conduit.rule: inventory, conduit.refusal_tier: inventory}
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Metric
metadata:
  name: gen-ai-input-tokens.v1
  description: >-
    OpenTelemetry's gen_ai.client.inference.usage.input_tokens. Input tokens include cached
    ones; billable tokens are reported where a client gives both.
spec:
  pin:
    repository: https://github.com/open-telemetry/semantic-conventions-genai
    commit: e6658f14c9d9ea0760825f029cb82e7673f48fab
    schemaUrl: https://opentelemetry.io/schemas/gen-ai-dev/1.42.0-dev
    metric: gen_ai.client.inference.usage.input_tokens
  resolved:                          # GENERATED by tools/ from Weaver's resolved output; never edited by hand
    name: gen_ai.client.inference.usage.input_tokens
    instrument: counter
    unit: '{token}'
    brief: The number of input (prompt) tokens used, including cached tokens.
    stability: development
    note: Each measurement MUST be a known, strictly positive modality contribution. (abridged)
    attributes:
    - {key: gen_ai.operation.name, type: string, requirement_level: required}
    - {key: gen_ai.provider.name, type: string, requirement_level: required}
    - {key: gen_ai.token.modality, type: {members: [text, image, audio, unknown]}, requirement_level: required}
    - {key: gen_ai.request.model, type: string, requirement_level: {conditionally_required: If available.}}
    - {key: gen_ai.response.model, type: string, requirement_level: recommended}
  conduit:                           # Conduit's additions, and nothing else
    source: reported
    valueType: int                   # from annotations.code_generation.metric_value_type
    strictlyPositive: true           # set in the generator's mapping table from the note
    split: [gen_ai.provider.name, conduit.model, conduit.account]   # modality is additive, so it is not split
    bounds: {gen_ai.provider.name: {maxLength: 64}, gen_ai.request.model: {maxLength: 128}, gen_ai.response.model: {maxLength: 128}, gen_ai.operation.name: {maxLength: 64}}
    disclosure: {}                   # every reported attribute is open
# output_tokens has the same shape.
# cache_read.input_tokens and cache_write.input_tokens carry conduit.subsetOf: ...input_tokens.
# reasoning.output_tokens carries conduit.subsetOf: ...output_tokens.
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Metric
metadata:
  name: account-quota-used.v1
  description: >-
    Share used of a subscription's limit window, as the vendor client reports it. Conduit-named
    because OpenTelemetry has no quota metric.
spec:
  resolved:
    name: conduit.account.quota.used
    instrument: gauge
    unit: '%'
    brief: Percentage (0 to 100) of the account's limit window consumed, at the time of the call's end.
    stability: development
    attributes:
    - {key: conduit.quota.window, type: string, requirement_level: required}   # for example 5h, 7d
    - {key: conduit.quota.scope, type: string, requirement_level: required}    # for example all, or a model family
  conduit:
    source: reported
    valueType: double
    stateOf: account                 # last value per (account, window, scope); never summed, not even across members
    bounds: {conduit.quota.window: {maxLength: 16}, conduit.quota.scope: {maxLength: 32}}
    disclosure: {}                   # each result says: account-wide, includes use outside this container and outside Conduit

# =====================================================================================
# 2. CLAUDE CODE AGENT PACKAGE: several models in one turn, and a quota reading
# =====================================================================================
# Manifest item (abridged)
implementations:
  wrapper:
    implements: ['#turn.v1']
    plain: true                      # the engine reads the one metrics key from plain results (IFC-010 amended)
    deadline: 900s                   # NEW per-item value. The engine's dispatch deadline exceeds the wrapper's own (DL10).
    reports:                         # NEW. Listing it here forces no new Type version.
    - {metric: github.com/shrav-k/conduit/core#gen-ai-input-tokens.v1, always: true}
    - {metric: github.com/shrav-k/conduit/core#gen-ai-output-tokens.v1, always: true}
    - {metric: github.com/shrav-k/conduit/core#gen-ai-cache-read-input-tokens.v1, always: true}
    - {metric: github.com/shrav-k/conduit/core#gen-ai-cache-write-input-tokens.v1, always: true}
    - {metric: github.com/shrav-k/conduit/core#account-quota-used.v1, always: false}   # only when the stream carried rate_limit_event (probe)
    narrow: {gen_ai.provider.name: {values: [anthropic]}}
# The .v2 Type that batch 1 ships states signIn:
#   signIn: {account: /account, billingAccount: /organization}    # FOCUS level; optional
# ---- One turn's result ----
_meta:
  io.github.shrav-k.conduit/metrics:
    points:
    - {name: gen_ai.client.inference.usage.input_tokens, value: 98014,
       attributes: {gen_ai.provider.name: anthropic, gen_ai.operation.name: chat, gen_ai.token.modality: text, gen_ai.response.model: model-c, example.org/conduit/agent-claude-code/query_source: main}}
    - {name: gen_ai.client.inference.usage.input_tokens, value: 12550,
       attributes: {gen_ai.provider.name: anthropic, gen_ai.operation.name: chat, gen_ai.token.modality: text, gen_ai.response.model: model-b, example.org/conduit/agent-claude-code/query_source: subagent}}
    - {name: gen_ai.client.inference.usage.output_tokens, value: 4120,
       attributes: {gen_ai.provider.name: anthropic, gen_ai.operation.name: chat, gen_ai.token.modality: text, gen_ai.response.model: model-c, example.org/conduit/agent-claude-code/query_source: main}}
    - {name: conduit.account.quota.used, value: 37.0, attributes: {conduit.quota.window: 5h, conduit.quota.scope: all}}
    - {name: conduit.account.quota.used, value: 12.0, attributes: {conduit.quota.window: 7d, conduit.quota.scope: all}}
    # No cache_read point: none was read. The item marks cache_read `always`, so its absence counts as zero, not as not reported.
    # The package converted input to OpenTelemetry's meaning (input includes cache), as OpenTelemetry's Anthropic note requires.

# =====================================================================================
# 3. CODEX AGENT PACKAGE: a delta, a late turn, and recovered usage
# =====================================================================================
_meta:
  io.github.shrav-k.conduit/metrics:
    points:
    - {name: gen_ai.client.inference.usage.input_tokens, value: 25619,
       attributes: {gen_ai.provider.name: openai, gen_ai.operation.name: chat, gen_ai.token.modality: text, gen_ai.request.model: model-a}}
    recovered:                       # an earlier turn of this session whose answer never arrived; kept apart, labelled recovered
    - points:
      - {name: gen_ai.client.inference.usage.input_tokens, value: 40511,
         attributes: {gen_ai.provider.name: openai, gen_ai.operation.name: chat, gen_ai.token.modality: text, gen_ai.request.model: model-a}}
---
# The end record the engine keeps (record.v1 data; NEW members marked)
type: invocation-ended
source: node/studio
time: '2026-11-11T10:02:41.120Z'
data:
  caller: <root>
  consumer: <root>
  conduit: <root--turn--codex-1>
  target: <codex-1>
  operation: example.org/conduit/agent-codex#turn.v1
  invocation: <id>
  outcome: succeeded
  container: <Fleet coding>          # NEW, stamped at write time
  consumerContainer: null            # NEW
  durationMs: 41877                  # NEW, from the shell's monotonic clock
  revision: 3                        # NEW (this and below: only where reports are declared)
  implementation: {package: example.org/conduit/agent-codex, version: 0.2.0, item: wrapper}
  account:
    key: example.org/conduit/agent-codex
    value: owner@example.com
    status: confirmed
    confirmedAt: '2026-11-10T21:14:03Z'
    billingAccount: {value: null, status: undeclared}
  metrics:
    status: reported                 # reported | not_reported | dropped (with rule; never the values) | late
    points: [<validated; each with instrument, unit and valueType copied>]
    recovered: [<validated; labelled recovered>]
---
# A late answer: the engine had already ended the attempt unknown at its deadline.
type: report-late                    # NEW record kind (REC-001 amended). The outcome stays unknown (OPS-013).
data: {invocation: <id of the unknown turn>, container: <Fleet coding>, account: {...}, metrics: {status: late, points: [...]}}

# =====================================================================================
# 4. VECTOR DATABASE PACKAGE (hypothetical example.org/conduit/qdrant)
# =====================================================================================
apiVersion: github.com/shrav-k/conduit/v0
kind: Metric
metadata: {name: searches.v1}
spec:
  resolved:
    name: example.org/conduit/qdrant/searches   # vendor metric under the package address; never merged with others
    instrument: counter
    unit: '{search}'
    brief: Searches the call ran (the batch size for a batched call).
    stability: development
    attributes:
    - {key: example.org/conduit/qdrant/search.kind, type: {members: [nearest, hybrid, recommend]}, requirement_level: required}
    - {key: db.collection.name, type: string, requirement_level: recommended}
  conduit:
    source: reported
    valueType: int
    split: [conduit.account]
    bounds: {db.collection.name: {maxLength: 64}}
    disclosure: {db.collection.name: inventory}
    origin: {db.collection.name: input}         # refused if the Type's records.inputs is false
# Latency per call needs no declaration: conduit.invocation.duration covers it.

# =====================================================================================
# 5. WHO MAY READ: opt-in roles (existing view and manage meanings unchanged)
# =====================================================================================
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: fleet-metrics                # NEW core template; pool-metrics is alike
  description: Lists metrics.v1 alone. With inspect.v1 on the same Fleet its holder reads at G2, without it at G1.
spec:
  capability: github.com/shrav-k/conduit/core#fleet-management.v1
  operations: [github.com/shrav-k/conduit/core#metrics.v1]
---
fleet: coding
interfaces:
  alice:   {manages: {fleets: {coding: [view, metrics]}, pools: {vectors: [view, metrics]}}}   # alice--manages--coding and alice--metrics--coding: G2
  bob:     {manages: {fleets: {coding: view}}}       # unchanged meaning: list and inspect, no numbers
  billing: {manages: {fleets: {coding: metrics}}}    # billing--metrics--coding: G1
agents:
  planner: {manages: {fleets: {coding: [view, metrics]}}}   # an Agent's own Fleet accepts metrics too (symmetry)
# Root's standing Conduits gain metrics.v1 by ruled amendment (FLEET-005, FST-002).

# =====================================================================================
# 6. ONE READ: describe, plus three named sub-queries with a time comparison
# =====================================================================================
read:
  describe: true
  fromTimestamp: '2026-11-09T08:00:00Z'      # the client resolved "this week, America/Los_Angeles"
  toTimestamp: '2026-11-16T08:00:00Z'
  config: {row_limit: 100}
  queries:
  - id: tokens
    from: [{kind: Fleet, name: coding}]
    metrics:
    - {measure: gen_ai.client.inference.usage.input_tokens, aggregation: sum}
    - {measure: gen_ai.client.inference.usage.cache_read.input_tokens, aggregation: sum}
    - {measure: gen_ai.client.inference.usage.output_tokens, aggregation: sum}
    # dimensions omitted: the default is the split keys [conduit.account, gen_ai.provider.name, conduit.model]
    timeDimension: {granularity: day}
  - id: tokens-last-week
    from: [{kind: Fleet, name: coding}]
    metrics: [{measure: gen_ai.client.inference.usage.input_tokens, aggregation: sum}]
    offset: P7D                      # the same window shifted one week back (PromQL offset; NRQL COMPARE WITH)
  - id: quota
    from: [{kind: Fleet, name: coding}]
    metrics: [{measure: conduit.account.quota.used, aggregation: max}]   # max of last per bucket, per account and window
    dimensions: [{field: conduit.account}, {field: conduit.quota.window}]
    timeDimension: {granularity: hour}
---
# Pool vectors, G2: searches and p95 latency per Resource
- id: searches
  from: [{kind: Pool, name: vectors}]
  metrics:
  - {measure: example.org/conduit/qdrant/searches, aggregation: sum}
  - {measure: conduit.invocation.duration, aggregation: p95}   # nearest-rank over raw per-call values
  dimensions: [{field: conduit.member}]                        # inventory: G2 only
  without: [conduit.account]                                   # searches by any account, said explicitly
  filters: [{column: conduit.model_work, operator: '=', value: false, type: boolean}]
  timeDimension: {granularity: hour}
---
# Fleet coding, G2: Conduits never used this week (Part B: fill)
- id: unused-grants
  from: [{kind: Fleet, name: coding}]
  metrics: [{measure: conduit.requests, aggregation: sum}]
  dimensions: [{field: conduit.conduit}]
  fill: [conduit.conduit]            # seeds explicit zero rows for Conduits that existed in the window
  orderBy: [{field: sum_conduit.requests, direction: asc}]

# =====================================================================================
# 7. WHAT EACH READER SEES (with R4's recommendation (a))
# =====================================================================================
# The `tokens` sub-query, read by ROOT or ALICE (G2):
result:
  covers: [{container: {kind: Fleet, name: coding}, grain: G2}]
  statement: Numbers from containers not in covers are never included; whether any exist is not said. Calls through this engine only; a vendor account's own meter also counts use outside it.
  watermarks: [{source: node/studio, through: '2026-11-11T17:59:58Z'}]   # the window runs past it, so the result is marked incomplete
  complete: false
  measures:
  - {measure: gen_ai.client.inference.usage.input_tokens, instrument: counter, unit: '{token}', valueType: int, source: reported,
     pin: semantic-conventions-genai@e6658f14,
     declarations: [example.org/conduit/agent-codex@0.2.0, example.org/conduit/agent-claude-code@0.2.0]}   # G2 only
  rows:
  - {time: '2026-11-10T00:00:00Z',
     dimensions: {conduit.account: {value: 'example.org/conduit/agent-codex:owner@example.com', status: confirmed, from: attested},
                  gen_ai.provider.name: {value: openai, from: reported},
                  conduit.model: {value: model-a, from: derived, source: request.model}},
     sum_gen_ai.client.inference.usage.input_tokens: 803114}
  - {time: '2026-11-10T00:00:00Z',
     dimensions: {conduit.account: {value: 'example.org/conduit/agent-codex:owner@example.com', status: confirmed, from: attested},
                  gen_ai.provider.name: {value: openai, from: reported},
                  conduit.model: {value: null, notReported: true}},
     sum_gen_ai.client.inference.usage.input_tokens: 31220}     # its own row, never merged into model-a
  summedAcross: []
  coverage:
    reports: {reported: 212, not_reported: [{cause: ended unknown at the dispatch deadline, count: 1}], late: 1, recovered: 1, dropped: [{rule: MET-004, count: 1}]}
    unknownTurns: 1                  # "tokens of 1 turn unknown": never zero
    gaps: []
  overflow: null
---
# The same sub-query, read by BILLING (G1). It succeeds under R4(a):
result:
  covers: [{container: {kind: Fleet, name: coding}, grain: G1}]
  measures: [{measure: gen_ai.client.inference.usage.input_tokens, instrument: counter, unit: '{token}', source: reported, pin: semantic-conventions-genai@e6658f14}]   # no declarations list at G1
  rows: <same rows by account, provider and model; two members with undeclared accounts appear as ONE row {conduit.account: {value: null, status: undeclared, merged: true}}>
# Under R4(b), billing's query is refused:
#   invalid, MET-011: "conduit.account is an inventory dimension of Fleet coding ...;
#   name it in `without` to read the total across accounts."
---
# Refused for every reader, each with its rule:
# - dimensions [{field: conduit.account}] only
#     -> invalid, MET-008: "gen_ai.provider.name and conduit.model are split keys of
#        gen_ai.client.inference.usage.input_tokens; keep them, fix each with a one-value =
#        filter, or name them in without."
# - aggregation sum of conduit.account.quota.used
#     -> invalid, MET-007: "a gauge is non-additive; use last, then min, max or avg across series."
# - billing adding {field: conduit.member}
#     -> invalid, MET-011: "conduit.member is inventory; it needs inspect.v1 on coding."
# - alice naming Fleet research in any sub-query
#     -> the whole read is refused, ACC-001 tier 1. The answer is identical whether or not research exists.

# =====================================================================================
# 8. A SAVED VIEW: a Perses dashboard. Two panels, one read when opened.
# =====================================================================================
kind: Dashboard
metadata: {name: coding-usage, project: conduit}
spec:
  display: {name: Fleet coding usage}
  duration: 7d
  panels:
    tokens:
      kind: Panel
      spec:
        display: {name: Tokens per account and model, per day}
        plugin: {kind: Table, spec: {}}
        queries:
        - kind: TimeSeriesQuery
          spec:
            plugin:
              kind: ConduitMetricsQuery      # the only thing Conduit defines here
              spec:
                from: [{kind: Fleet, name: coding}]
                metrics: [{measure: gen_ai.client.inference.usage.input_tokens, aggregation: sum}, {measure: gen_ai.client.inference.usage.output_tokens, aggregation: sum}]
                timeDimension: {granularity: day}
    refusals:
      kind: Panel
      spec:
        display: {name: Refusals by reason}
        plugin: {kind: Table, spec: {}}
        queries:
        - kind: TimeSeriesQuery
          spec:
            plugin:
              kind: ConduitMetricsQuery
              spec:
                from: [{kind: Fleet, name: coding}]
                metrics: [{measure: conduit.requests, aggregation: sum}]
                filters: [{column: conduit.reason, operator: '=', value: refused, type: string}]
                dimensions: [{field: conduit.reason}]
  layouts:
  - kind: Grid
    spec:
      items:
      - {x: 0, y: 0, width: 24, height: 10, content: {$ref: '#/spec/panels/tokens'}}
      - {x: 0, y: 10, width: 24, height: 8, content: {$ref: '#/spec/panels/refusals'}}
# `conduit view coding-usage.yaml` sends ONE metrics.v1 read with both panels as sub-queries,
# under the opener's own Interface. The file grants nothing.

## Staging

NOW: WHAT BATCH 1, THE SPEC BATCH BEING WRITTEN, MUST NOT FORECLOSE

1. The .v2 Agent Types state signIn: {account: <pointer>}, with room for the optional billingAccount and subAccount pointers. Without it no account can be stamped, and records are immutable.
2. record.v1 `data` stays open to added members, and REC-002's list is not exhaustive. REC-001's list of record kinds is not closed against report-late. Members the metrics batch adds:
   - container, consumerContainer;
   - durationMs, revision, implementation;
   - account {key, value, status, confirmedAt, billingAccount, subAccount};
   - metrics {status, points, recovered}.
3. Names left unused:
   - the _meta key io.github.shrav-k.conduit/metrics;
   - the manifest item keys `reports`, `narrow` and `deadline`;
   - the Type and Operation keys `metrics` and `reports`;
   - room in names.yaml for metricPrefix.
4. IFC-007 and IFC-010 are not worded so that the engine may never read one declared key from a plain result, or strip it before relay.
5. The "no counts" rules of OPS-036 and LIF-013 stay scoped to discovery and inventory.
6. SCT-012: a role value can later become a list, and new role words can be added. An Agent's own-Fleet entry is not limited forever to manage and view. Expansions into more than one Conduit stay possible.
7. FLEET-005 and FST-002 are phrased so a ruled amendment can add metrics.v1.
8. CTR-005's list of core requirements is worded so metrics-grain.v1 is an addition.
9. CTR-013:
   - the expected account stays a recordable value;
   - each account-hook check's result and time are recordable for reference;
   - no control's `changes` may reach the signIn pointers;
   - a null account leaves room for a declared non-secret organization or project identifier.
10. OBJ-020 and the loaders do not hard-code the number of contract kinds beyond the schema. Today they appear in five lists in four crates.
11. Dispatch and end record times stay the engine's own, and per-source order stays as REC-005 has it. Nothing forbids the shell supplying durationMs as evidence, or DL10 holding a per-item dispatch deadline.
12. The pending-action kinds stay open to a threshold kind. ACC-056's tiers stay applicable to a read naming several containers.

BATCH 2
- Fleet authority, forwarding (61 B3) and 62 C10 must not grant metrics reads through forwarding, and must not define a detail mechanism that conflicts with G1 and G2.
- If batch 2 migrates root's default operation lists or the role templates, the metrics list change is folded into the same scripted migration. The last such migration, order 041a, touched 167 cases.
- Peer engines (62 C4) hold their own records, so metrics reads are per engine. Watermarks name each source.

ENGINE WORK BEFORE THE METRICS BATCH
This is a storage order: build only, under DL2, DL10 and DL20, plus the OPS-008 amendment.
1. Index entries reference the held target by ID and revision. In the 2026-11-10 probe store, held_target is 126.3 KB of each 129.6 KB install.v1 entry (97%).
2. Records go to an append-only segment file, and the core reads them through an interface.
3. Unkeyed, read-only engine requests keep a minimal index entry, and their outputs are not retained.
4. Evaluation of reads moves outside the writer lock, or DL10 bounds the records scanned, refused by name.
5. The shell measures each attempt on a monotonic clock and supplies durationMs.
6. DL10 gains a per-item dispatch deadline that a manifest may declare. The model-work default exceeds the wrappers' per-turn deadline.
7. Measure one read under D19. My bench rerun at 100,000 synthetic end records: about 2.5 s per whole-state rewrite and 0.57 s per scan. At a million: about 40 s and 12 s.

PROBES (before the metrics batch's promises are fixed; ruling 68's probe, widened)
- Codex:
  - Where the served model comes from: configuration, turn_context, or the app-server protocol.
  - Whether input_tokens includes cached tokens.
  - Whether a failed or timed-out turn's tokens appear in a later thread total or in the local session log (the recovery path).
  - Delta conversion with experiment 008 fixtures.
  - Whether RateLimitSnapshot is reachable through the app-server protocol.
- Claude Code, on the pinned version:
  - Whether `claude -p --output-format stream-json` carries modelUsage per model, including subagents, and rate_limit_event or rate_limits rows.
  - Spend restored on resume.
  - Usage zeroed on a crash.
  - What a turn past the wrapper deadline emits.
- Generator: run Weaver's resolve on the pinned GenAI registry and confirm the resolved fields: resolved/2.0 with its attribute catalog, metric_value_type, and requirement levels.
- FOCUS mapping: for each vendor the two packages reach (Anthropic API and Claude subscriptions; OpenAI API and ChatGPT plans), which identifier is the billing account and which the sub-account, and which the configuration or account-hook can supply.
- Later, for the observation hook: Qdrant's /metrics names.

THE METRICS BATCH, AFTER BATCH 2. PART A (R6(a))

Declarations:
- the Metric kind (pin, generated resolved copy, the `conduit:` block);
- core engine counters in OpenMeter's shape (requests, invocations, invocation.duration, reports, plus the inventory-class provider.calls, object.changes and metrics.reads);
- the five GenAI counters at e6658f14;
- conduit.account.quota.used;
- items' `reports` with `always`, `narrow` and `deadline`;
- install checks.

Reports:
- the _meta key, read from plain results and stripped before relay;
- validation and caps, with strictly-positive counters;
- gauges with stateOf;
- drop whole, with status;
- failed results;
- report-late records;
- recovered points;
- coverage per report.

Stamping on request-ended and invocation-ended records, including account status and FOCUS levels.

metrics.v1:
- describe;
- named sub-queries with offset;
- Langfuse-named fields with CESQL-spelled operators;
- sum, count, min, max, avg, nearest-rank p50 to p99, and last for gauges;
- the split rule and default dimensions;
- UTC granularities;
- several containers;
- DL10 limits and a deterministic overflow row;
- watermarks and explicit results with per-window gaps;
- G1 and G2 checked over every named field before evaluation, with R4's answer applied;
- refusal attribution only through held Conduits;
- self-records excluded;
- recorded, with a minimum interval per caller.

Access:
- the ACC-033 row and ACC-056 for several containers;
- LIF-013 withholding;
- the opt-in `metrics` role and role lists in SCT-012, Agents included;
- the fleet-metrics and pool-metrics templates;
- root's defaults.

Packages: the Codex and Claude Code wrappers report per-model deltas, quota where present, recovered usage, and failed or late turns, all with fixtures from real runs.

Client: `conduit view <file>` reads a Perses dashboard and renders tables. One read per opening.

Checks: a Python reference evaluator under tools/ recomputes every metrics vector from exported records.

Rough size: about 18 new and 20 amended promises; 2,400 to 3,200 engine lines; 60 to 70 vectors; one scripted migration.

PART B (the same batch if the budget holds, otherwise the next)
- Formulas: ratios, and + and - within one unit, aggregated first. Subset plus parent is refused.
- metrics-grain.v1, explicit and positive, narrowing only.
- Replayed engine state (open Conduits, instances by phase, pending outstanding, time to answer), and `fill` for unused grants, at G2.
- Packages: the Perses ConduitMetricsQuery plugin; the PromQL bridge for one Interface (a declared subset, and OpenMetrics cumulative totals with created timestamps, stated as a broad credential); a DuckDB recipe over export-records.

V1 AND LATER
- v1 (already accepted):
  - the consumer side, with counterpart dimensions gated on authority over the counterpart's container;
  - thresholds under the stated rules, which amend ruling 66 point 3.
- Later, each named in R6 for an explicit yes:
  - the observe-metrics.v1 hook (an index's size; updowncounters and gauges);
  - Codex quota after the probe;
  - a record-store provider (amends ruling 40 point 2);
  - cost through a price-table provider, labelled an estimate;
  - views in a core Interface Type;
  - owner-set labels;
  - scores or feedback;
  - percentiles that merge across engines;
  - a FOCUS export once FOCUS 1.5 lands;
  - a purge rollup only if default 15 is struck.

## Sources

CONDUIT REPOSITORY (read only, HEAD 5ed438b)
- /Users/shravankumaran/Projects/Conduit/conduit/docs/proposals/ruling-68/README.md
  - point 1: M1 to M3 and the defaults. Line 30's default is "usage not relayed to the Agent that made the call", not a ban on Agents reading.
  - point 6: M4, with the owner's words.
- /Users/shravankumaran/Projects/Conduit/conduit/docs/proposals/metrics/README.md: the accepted design. It says "it sums every member the container has had" and "totals only".
- /Users/shravankumaran/Projects/Conduit/conduit/docs/proposals/ruling-66/README.md:
  - point 1: "manage or view";
  - point 3: raised at every Interface whose scope contains the subject;
  - point 6: no Scope object that silently widens.
- /Users/shravankumaran/Projects/Conduit/conduit/IN_FORCE.md:
  - topic 1: scope guard A18; "We shouldn't overcomplicate this"; preservation test 5;
  - lines 148 to 149: metrics, in force and open.
- /Users/shravankumaran/Projects/Conduit/conduit/docs/AUDIT.md: the preservation tests (test 5, every management action available to an agent under a grant).
- /Users/shravankumaran/Projects/Conduit/conduit/spec/promises/sections.yaml: SCT-012 (one Conduit N--manages--T per member; roles manage and view).
- /Users/shravankumaran/Projects/Conduit/conduit/spec/conventions/operations.yaml: OPS-008, OPS-013, OPS-014 and OPS-030 (unknown stays unknown until reconciled).
- /Users/shravankumaran/Projects/Conduit/conduit/spec/conventions/records.yaml: REC-001 to REC-006 (REC-004: "A purge removes records and nothing else").
- /Users/shravankumaran/Projects/Conduit/conduit/spec/core/results.yaml: record.v1 "may add members".
- /Users/shravankumaran/Projects/Conduit/conduit/spec/conventions/access.yaml: ACC-008, ACC-033 and ACC-056.
- /Users/shravankumaran/Projects/Conduit/conduit/spec/promises/fleets.yaml: FLEET-005.
- /Users/shravankumaran/Projects/Conduit/conduit/spec/promises/first-start.yaml: FST-002.
- /Users/shravankumaran/Projects/Conduit/conduit/spec/promises/contracts.yaml: CTR-005, CTR-008 and CTR-013.
- /Users/shravankumaran/Projects/Conduit/conduit/spec/promises/objects.yaml: OBJ-020.
- /Users/shravankumaran/Projects/Conduit/conduit/engine/CONFORMANCE.md:
  - DL10: dispatch 120 s; "No key expires and invocation state is never compacted";
  - I3-04: the wrappers' 180-second per-turn deadline, which poisons the session.
- /Users/shravankumaran/Projects/Conduit/conduit/AGENTS.md: code is Rust; Python only for the checkers in tools/.

MEASUREMENTS AND SAVED SOURCES (scratchpad, this session)
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/bench-out/results-2026-11-11.txt: the skeptic's synthetic bench, rerun. Source: scratchpad/bench/src/main.rs.
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/probe-store/state.json: invocation entries of 129.6 KB, of which held_target is 126.3 KB.
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/otel-genai/repo/model/gen-ai/token-metrics.yaml:
  - file_format: definition/2;
  - "Each measurement MUST be a known, strictly positive modality contribution";
  - "Instrumentation SHOULD record contributions from successful and failed inference operations";
  - metric_value_type: int.
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/skeptic-cost/docs/vm.md:
  - deduplication would "leave a single raw sample with the biggest timestamp per each discrete interval";
  - "Delete API doesn't support the deletion of specific time ranges".
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/skeptic-cost/docs/prom-config.md: translation_strategy defaults to "UnderscoreEscapingWithSuffixes", and "NoTranslation" is "(EXPERIMENTAL)".
- /private/tmp/claude-501/-Users-shravankumaran-Projects-Conduit/c29254d6-89de-4b45-ae6e-51820c38ec5c/scratchpad/skeptic-cost/pp/promql-parser-0.11.1/README.md: "This crate declares compatible with prometheus v3.8".
- Not re-read: the Mimir range-deletion issue https://github.com/grafana/mimir/issues/4968, and remote-write backfill limits. Both are the skeptics' claims.

USAGE METERING AND BILLING (new this round)
- OpenMeter usage events. Quotes: "OpenMeter ingests events using the CloudEvents specification"; "`subject`: The event's subject (e.g. the customer ID)"; "Meter values are aggregated by subjects"; valueProperty JSONPath ("only basic scalar paths"); groupBy. https://openmeter.io/docs/metering/events/usage-events
- OpenMeter meter aggregations, from a search summary of its API enum and docs: SUM, COUNT, UNIQUE_COUNT, AVG, MIN, MAX and LATEST. https://openmeter.io/docs/getting-started/meters
- Stripe Billing Meters. Aggregation is Sum, Count or Last. Quotes: "After you configure the meter, you can't make any changes aside from the display name"; adjustments "within the last 24 hours". https://docs.stripe.com/billing/subscriptions/usage-based/meters/configure
- FinOps FOCUS 1.4 columns:
  - Billing Account ID: https://focus.finops.org/docs/specification/v1-4/columns/cost-and-usage/billing-account-id/
  - Sub Account ID: https://focus.finops.org/docs/specification/v1-4/columns/cost-and-usage/sub-account-id/
  - Consumed Quantity: https://focus.finops.org/docs/specification/v1-4/columns/cost-and-usage/consumed-quantity/
- FOCUS 1.4 announcement: ratified June 4, 2026; "FOCUS 1.5 is scoped to surface AI model identity and token consumption (input and output)". https://www.finops.org/insights/introducing-focus-1-4/
- FOCUS specification index: https://focus.finops.org/focus-specification/

QUERY STANDARDS CHECKED (new this round)
- CloudEvents SQL Expression Language, Version 1.0.0. Quotes: "doesn't support the handling of the data field"; operators NOT, AND, OR, XOR, =, !=, <>, <, <=, >, >=, LIKE, IN, EXISTS. https://github.com/cloudevents/spec/blob/main/cesql/spec.md
- CESQL v1 release (2024-06-13): https://cloudevents.io/blog/2024-07-15/
- CNCF TAG Observability Query Language Standardization working group (formed 2023; a draft Semantic Feature Rubric at KubeCon NA 2024; I found no published standard):
  - https://www.cncf.io/blog/2023/08/03/streamlining-observability-the-journey-towards-query-language-standardization/
  - https://thenewstack.io/works-commences-on-the-observability-query-language-standard/
- Open Semantic Interchange, now Apache Ossie. First specification January 2026; 0.1.1; "does not execute queries"; a metrics language is a later phase. From vendor posts, read through search summaries:
  - https://www.snowflake.com/de/blog/open-semantic-interchanges-specs-finalized/
  - https://www.getdbt.com/blog/the-osi-spec-updates
  - https://cwiki.apache.org/confluence/spaces/INCUBATOR/pages/430408796/OssieProposal

ACCESS GOVERNANCE (new this round)
- AWS IAM Access Analyzer unused access. Quotes: "Roles with no access activity within the specified usage window"; "Unused permissions – Service-level and action-level permissions that weren't used by a role within the specified usage window". https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-findings.html

OPENTELEMETRY
- Metrics data model: https://github.com/open-telemetry/opentelemetry-specification/blob/a03fa4e26b3982da42aa0010aaebcd9f694cb037/specification/metrics/data-model.md
- SDK Views, aggregations and the overflow series: https://github.com/open-telemetry/opentelemetry-specification/blob/a03fa4e26b3982da42aa0010aaebcd9f694cb037/specification/metrics/sdk.md
- Supplementary guidelines (additive and non-additive): https://github.com/open-telemetry/opentelemetry-specification/blob/a03fa4e26b3982da42aa0010aaebcd9f694cb037/specification/metrics/supplementary-guidelines.md
- Prometheus compatibility: https://github.com/open-telemetry/opentelemetry-specification/blob/a03fa4e26b3982da42aa0010aaebcd9f694cb037/specification/compatibility/prometheus_and_openmetrics.md
- GenAI token metrics (Development; the pin) and its design note:
  - https://github.com/open-telemetry/semantic-conventions-genai/blob/e6658f14c9d9ea0760825f029cb82e7673f48fab/docs/gen-ai/gen-ai-token-metrics.md
  - https://github.com/open-telemetry/semantic-conventions-genai/blob/e6658f14c9d9ea0760825f029cb82e7673f48fab/model/gen-ai/token-metrics.yaml
- Weaver and its resolved registry format (resolved/2.0 with an attribute catalog, from a search summary): https://github.com/open-telemetry/weaver and https://raw.githubusercontent.com/open-telemetry/weaver/main/schemas/semconv-schemas.md
- Collector count, sum and signal-to-metrics connectors: https://github.com/open-telemetry/opentelemetry-collector-contrib/tree/e822a54458e1127c4b8ef0217dde47a45ff9d333/connector

PROMETHEUS
- Naming: https://prometheus.io/docs/practices/naming/
- Rules ("Always specify a without clause"; numerator and denominator separately): https://prometheus.io/docs/practices/rules/
- Operators: https://prometheus.io/docs/prometheus/latest/querying/operators/
- Functions: https://prometheus.io/docs/prometheus/latest/querying/functions/
- prom-label-proxy (matcher injection; label endpoints off): https://github.com/prometheus-community/prom-label-proxy/blob/d176e112a58facb9656425e6b4c47ee210a99ae0/README.md

QUERY SHAPES AND DISPLAY
- Langfuse Metrics API v2, the borrowed field names: https://langfuse.com/docs/metrics/features/metrics-api
- Langfuse data model (commit 0cb170c7): https://github.com/langfuse/langfuse/blob/0cb170c7681aaefa16ba737f9e1c8d6cc1cd1984/packages/shared/src/features/query/dataModel.ts
- Anthropic Usage and Cost Analytics API: https://platform.claude.com/docs/en/manage-claude/usage-cost-analytics-api
- OpenAI Usage API: https://developers.openai.com/api/reference/resources/organization/subresources/usage/methods/get_completions
- MLflow QueryTraceMetrics: https://github.com/mlflow/mlflow/blob/aebec5c03a3cc8e97f2dd0ba30217ef946e1c218/mlflow/protos/service.proto
- Perses (CNCF) dashboards: https://perses.dev/ and https://github.com/perses/perses/blob/22762b0/docs/api/dashboard.md
- CloudWatch PromQL (the warning about deltas): https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-PromQL-Querying.html

TENANCY AND LEAKS
- Mimir federated rules: https://github.com/grafana/mimir/blob/9ae50e85015691389020eb0cd042b436b60b44f8/docs/sources/mimir/references/architecture/components/ruler/index.md
- Prometheus Operator enforcedNamespaceLabel: https://github.com/prometheus-operator/prometheus-operator/blob/b8b4b78476c060fe7cc42e099491336b578ab21e/pkg/apis/monitoring/v1/prometheus_types.go
- Datadog Data Access Control: https://docs.datadoghq.com/account_management/rbac/data_access/
- Statistical databases (Denning, Denning and Schwartz, as quoted): https://en.wikipedia.org/wiki/Statistical_database

EVENTS VERSUS STORED METRICS
- ClickStack materialized views (equivalence; about 10 TB a day): https://clickhouse.com/docs/clickstack/managing/materialized-views
- Apache DataFusion: https://datafusion.apache.org/
- DuckDB: https://duckdb.org/why_duckdb
- kube-state-metrics: https://github.com/kubernetes/kube-state-metrics/blob/5e7546610118ae5fc9cd20ad7b053bbfe6a21de7/README.md

CLIENTS' OWN USAGE DATA
- Codex exec events (turn.completed carries the thread total): https://github.com/openai/codex/blob/e58bf8f5/codex-rs/exec/src/event_processor_with_jsonl_output.rs
- Codex protocol (TokenUsage, RateLimitSnapshot): https://github.com/openai/codex/blob/e58bf8f5/codex-rs/protocol/src/protocol.rs
- Claude Code cost tracking (modelUsage; restored spend; zeroed on crash): https://code.claude.com/docs/en/agent-sdk/cost-tracking.md
- Claude Code status line and Agent SDK rate limits: https://code.claude.com/docs/en/statusline.md and https://code.claude.com/docs/en/agent-sdk/typescript.md
- ccusage (read-time recovery from local logs): https://github.com/ryoppippi/ccusage
