# Ruling 68: metrics, safety vectors, spec batch 1's questions, and four older rulings

Status: in force; open (point 6: M4) (ruled by the owner on 2026-11-11). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 7, 9, 13 and 15, and the promises the batches write. This record says why.

The coordinator put the questions that waited at the end of the night of 2026-11-10: the metrics
proposal, what a safety vector is, the three questions of spec batch 1, and four earlier rulings
whose status was unclear, each with a recommendation; and listed, as strikeable without an answer,
ruling 67 (decided under the owner's delegation), the readings spec batch 1 applies and one reading
of order 037. The owner: "I accept all recommendations, but each package should be responsible for
fixing its own metrics (codex fixing its token usage metric). Do we have to decide on shared
standards for that the same we do for agents?"

## The decisions

1. **Metrics** ([the proposal](../metrics/README.md)).
   - **M1, the approach, (a).** Metrics are numbers about objects Conduit already has: no new family,
     store or wire. The engine counts what passes through Conduits from its records; an Agent or
     Resource reports what only it knows with each result, in one `_meta` key, as its declaration
     states; the engine checks the report, takes whose it is from its own record, and keeps it in
     that call's end record. Numbers that are state, such as an index's size, come later from the
     instance's own metrics endpoint through a provider (ruling 40).
   - **M2, who reads.** One `metrics.v1`, routed like inspection; a holder of `metrics.v1` without
     `inspect.v1` sees the container's totals only, with no breakdown and no names.
   - **M3, records.** Every read is recorded, with a minimum interval between one caller's reads
     (implementation-defined).
   - **Each package answers for its own numbers** (the owner's words). Turning a client's raw
     numbers into what the declaration says is the package's work: the Codex package turns its
     client's running total per thread into what each turn used. The engine checks the form of a
     report, never its truth; a package shows its numbers are right with fixtures from real runs.
   - The stated defaults stand: tokens only and no cost in v0; usage not relayed to the Agent that
     made the call in v0; a consumer sees its own Conduits' numbers in v1; thresholds that raise a
     pending action in v1; Conduit never turns on a client's own telemetry and reports when a
     managed policy has; the engine's metric names take a prefix from `spec/names.yaml`. The metrics
     batch follows batch 2; a probe of Claude Code's and Codex's usage reports comes first.
2. **A safety vector** (for ruling 47's gate, "Every safety vector passes"; finding WO030-F1) is a
   vector whose failure would be an unsafe finding as the tracker defines "unsafe". Each safety
   vector carries a field naming the safety promise it tests, and each of S1's proposed safety traces
   (`conformance/s1-safety-cases.md`) has at least one such vector or is listed as missing. The
   manifest's group tags are not safety tags.
3. **Spec batch 1's questions** ([the plan](../spec-batch-1/README.md#for-the-owner)).
   - **R1, (b).** In the S1 rerun, the actions a person takes at the panel, such as a sign-in, are
     listed apart from the author's lines, as root's Conduits are (ruling 52, point 6), with the
     strict count reported beside; the `account:` lines count as authored lines.
   - **R2, yes.** Admitting a machine requires the fingerprint the person read on it, and the engine
     compares; a machine with no screen is admitted only with `--accept-weaker-check`, recorded.
   - **R3, (b).** Peer engines (ruling 62, C4) are written in batch 2, with appointment, keep and
     withhold. The Interface a peer admits counts as root for ACC-054 (C1), ACC-024 and ruling 59's
     "only root registers Agents", and not for the operations reserved to root, which stay with the
     peer's own root.
4. **Four earlier rulings.**
   - Ruling 5's option (b), separated components under a simulated network, then real processes,
     then two machines, is replaced by model checking of the safety promises (A40) and S3 on real
     machines (ruling 47).
   - Ruling 7's PL8, that moving an Agent to another machine may fork from a checkpoint, is
     type-contract material: an Agent Type may offer it, and the language promises nothing.
   - Ruling 46's fallback of two Claude Code instances, if one client could not reach HTTP with a
     token, is void for v0: ruling 47 needs both Claude Code and Codex.
   - Ruling 1's reference clones (agentwatch and pi-reference) are removed when convenient. They are
     on the owner's machine, and only the owner removes them.
5. **Not struck.** Ruling 67, decided under the owner's delegation, stands as the owner's; so do the
   readings spec batch 1 applies and the coordinator's reading in order 037 that ruling 55 A2's
   "fully qualified" means never an alias or a bare name, a project's own contracts keeping `#name`
   (NAM-022).
6. **Open: M4, shared names for metrics.** The owner asked whether shared standards are needed, as
   for Agents. The coordinator recommends yes, adopted rather than invented: the core package ships
   a few standard sets of usage points pointing at OpenTelemetry's semantic conventions (model token
   usage; database calls), pinned to a version, each with its meaning stated; a package declares the
   sets it reports; anything vendor-specific goes under the package's own name and is never summed
   with another's; Conduit names its own only where OpenTelemetry has none. The owner then gave the
   direction, the same morning: "We should differentiate model token usage by account and model
   type!! Otherwise it's useless, let's research attribution and aggregation. A shared token sum is
   useless since they're all different models on potentially different accounts". Usage is
   therefore never summed across models or accounts by default; how it is attributed and
   aggregated is being researched, and M4 waits for that.

## What it amends

Ruling 5 (option (b) replaced), ruling 7 (PL8 to type contracts), ruling 46 (point 2's fallback void
for v0), ruling 47 (the safety vectors defined), ruling 62 (C4 placed in batch 2) and ruling 1 (the
clones' end). It answers the metrics proposal and spec batch 1's R1 to R3, and confirms ruling 67.
