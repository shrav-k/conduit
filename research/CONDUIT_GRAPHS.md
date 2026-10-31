# Inquiry: Conduits as graphs

Opened: 2026-10-31. Branch: `research/conduits-as-graphs`.
**Potential direction only. No new architecture ruling or implementation.**

## Preserved baseline

The cleaned design and prior research are committed and pushed on `main` at
[`c013f0a`](https://github.com/shrav-k/conduit/tree/c013f0a20c0c579f511aea84fdc6804b66417b96).
[SPEC.md](../SPEC.md) and [DESIGN.md](../DESIGN.md) still describe that accepted model.
[REVIEW.md](../REVIEW.md#next-discussion) records the exact discussion to resume if this inquiry
is set aside: plugin-owned lifecycle stages versus optional author-written typed composition,
using the filesystem examples before authentication and indexing.

The existing [establishment research](ESTABLISHMENT.md) belongs to that baseline. Its optional
stage-graph discussion did not make Conduits themselves the general establishment graph.

## Proposal to investigate

Keep Sessions, Pools and Resources as typed YAML templates and instances. Explore making
Conduits the graphs that turn those specifications into instances, coordinate authentication
and setup, and establish communication or other continuing flows. Reusable plugin operations
would supply the behavior; graph semantics would express dependencies and execution order.

This is broader than adding optional composition to a Resource template. Whether it improves
the model, and how it changes Conduit templates, instances and lifetimes, remains unresolved.

## Questions for the next discussion

- What does a graph own: execution progress, connection state, created instances, or some
  combination? Test this against exclusive Pool ownership and minimal bookkeeping.
- How does finite setup relate to an ongoing filtered flow, including the existing rule that
  the last consumer's departure ends a Conduit?
- Where does complete establishment intent live without duplicating configuration between
  a Session/Resource/Pool template and a graph? What remains plugin implementation code?
- Can the model explain ordering, outputs, authentication waits and recovery more clearly
  across the existing directory, worktree, subscription, indexing and password-manager cases?

Research and any proposed document revisions stay on this branch until reviewed. To return
to the prior discussion, switch to `main` and use its review checkpoint; no reconstruction or
reversal of the accepted design is needed.
