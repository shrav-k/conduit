# Collaborator proposals against the current design

Reviewed: 2026-11-02. Two open PRs from `allocmemory` propose related directions. This document
compares their submitted text with the current implementation branch. It does not adopt, merge,
edit or post a review on either PR. Recommendations below remain for discussion.

| Source | Reviewed revision | Subject |
| --- | --- | --- |
| [PR #1 — docs(proposal): add roadmap proposal](https://github.com/shrav-k/conduit/pull/1) | `2111b0d4ca60080eb8de36a1728c6e56ffc69c71` | Protocol focus, Sealmesh, an autonomous local root Agent and per-environment VMs. |
| [PR #2 — How Conduit can become a protocol?](https://github.com/shrav-k/conduit/pull/2) | `c480e96b94db24e7080354161b948428478e14f8` | Schema, wire, lifecycle/error semantics and independent conformance. |

Both proposals explicitly identify themselves as discussion material against `main`. The current
branch contains later accepted authority, plugin, composition and persistence decisions. Compare
intent and behavior, rather than treating outdated terminology as disagreement with those rulings.

## What strengthens the current direction

- **Independent implementations and conformance.** Both PRs ask what another implementer needs
  to reproduce. This is the right test for the language-neutral contract and portable template goal.
  A reference implementation should not be the only source of protocol meaning.
- **Explicit boundaries.** Identify client/Interface, managed service, plugin and provider surfaces;
  describe identity, effects, versions and outcomes at each. A single diagram or schema is not enough.
- **Replaceable presentation.** Frontend layout and widgets are implementation choices. Preserve the
  Interface's accepted management/observation contract even when no particular UI is standardized.
- **Interoperability evidence.** Reuse the experiment scenarios as conformance tests, distinguishing
  parsing, behavior, independent-client/plugin interoperability and actual provider validation.

These align with accepted intent. Their exact conformance/release implications belong to C1 in the
[conceptual review](../REVIEW.md#c1--what-does-conformance-promise).

## What needs adaptation

| Proposal | Current contract / recommended treatment |
| --- | --- |
| Carry setup/runtime as opaque implementation-defined fields in the first protocol | Conflicts with portable plugin/template authoring. Keep public graph, operation, control and lifecycle meanings. A limited consumer may decline an unsupported execution surface; a template executor cannot claim portability by ignoring its semantics. |
| Exclude plugin ABI with all graph behavior | Separate physical ABI/process plumbing from public invocation and execution semantics. Native ABI details need not be standardized; MCP/profile contracts and declared behavior do. |
| Derive the protocol directly from what Experiment 001 needs | Use 001 as a scenario and 002 for composition/failure coverage. Draft fixture fields are not rulings, and neither fixture alone defines every required semantic boundary. |
| Start with a parallel fixed JSON-RPC method catalog and local socket wire | MCP is already the accepted public plugin/capability base. First map supported standard mechanisms and the necessary Conduit profile; evaluate any additional transport/method surface in the later authoring PR. No new wire is adopted here. |
| Use one short family lifecycle and a closed error list | Shared evidence is useful, but setup, runtime readiness, uncertain effects and domain outcomes cannot all be collapsed into one state. C3 will settle minimum guarantees; exact results stay extensible and typed. |
| Every request names its actor | A supplied name is attribution data until authenticated by the appropriate binding. Same-OS-user execution does not grant an Agent root management rights. C2 examines identity and restrictions across dependent calls. |
| Require two independent full implementations for the first release | Strong potential evidence, with a meaningful delivery cost. C1 should decide whether initial independent clients/plugins plus conformance vectors suffice, and when a second runtime becomes a gate. |
| No skills | Already no Conduit-owned skills framework. Preserve BYO native skills and explicit Agent activation through permitted Resources. |

The proposals' `v1`/`v2` labels, identifiers, paths and message names are suggestions, not accepted
release targets. Preserve the public behavior first; choose serialization together later.

## Additional roadmap candidates

**Sealmesh:** PR #1 proposes it as a messaging backend and describes its room/identity/retention
behavior. Those claims have not been independently revalidated in this pass. Evaluate the actual
implementation before choosing an adapter. It could fit as a Pool-owned communication Resource
if its declared guarantees match the selected retained-history capability. Adopting it must not
make rooms, GitHub organizations, tickets, Tailscale or remote deployment mandatory core concepts.
Conduit's accepted retain-until-explicit-purge contract is not superseded by a backend default;
capability/retention gaps must remain visible.

**Local root Agent:** this is distinct from the accepted root Interface and its deterministic
setup/recovery operations. A host-administration Agent could be an optional Agent with explicit
Resource access and delegated controls. It need not replace the Interface, own policy, or become a
mandatory model dependency for startup/repair. Model choice and local-model guarantees are later
questions only if that optional use case is selected.

**VM per environment:** a possible isolated-execution implementation, subject to actual host,
credential, filesystem and network tests. Making it the default would reopen the accepted native
default and compatibility tradeoff. Do not adopt that change merely by choosing a sandbox driver.
Remote scheduling and one-VM-per-account/process are not consequences of the five-family model.

## Integration plan

1. Discuss C1–C3 using the PRs' useful protocol/conformance challenge and the current accepted model.
2. Fold only agreed conceptual refinements into the canonical design topics. Preserve source links
   and acknowledge which recommendations remain optional or need validation.
3. In the later authoring PR, select schemas, MCP/profile mappings, complete package examples and
   conformance vectors together. Do not standardize the draft package by copying its field names.
4. Keep backend/VM/root-Agent experiments separate unless explicitly added to scope.

No collaborator messages, PR comments, approvals or merges were sent during this comparison.
