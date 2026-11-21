# 029 — A84: replay the plan-and-apply reference

Can plan and apply for S1's T1 to T7 be built from the existing requests? [A84 in the roadmap](../../ROADMAP.md#experiments-and-probes) tests the first stage of that question by replaying the nine [plan-and-apply cases](../../docs/implementation/plan-and-apply/README.md#case-format-and-sequence).

Start one engine on a temporary store. Install the scheduled S1 archives and send the cases' 60 requests as root through its loopback MCP endpoint, in chain order. Read the resulting state back through the same endpoint.

The pass is that all 60 requests end as stated and each case leaves the objects, open Conduits and tombstones in its `kept` through an ID map. Record any mismatch against the case or its deciding promise.

For each case record the number of requests sent, the number matched, and the first mismatch with its promise. Take these measures on the first replay after refreshing the fixtures and on the last replay.

The filesystem provider's responses are scripted. The first replay used the registration-only
Agent fixture. After that replay stopped, the owner amended the fixture constraint. The
continuation uses the pinned S1 Agent fixture with a selected implementation and supplies the
workspace's scripted client executables at test time. The engine runs on one machine at commit
`fd741539`, with the replay harness and cases from work order 241.
