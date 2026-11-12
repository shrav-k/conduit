# Enrollment contracts staged by 038b

The enrollment promises are in [enrollment.yaml](../../../spec/promises/enrollment.yaml).
C25 of the [batch plan](../../proposals/spec-batch-1/README.md#the-order-of-the-edits)
keeps contract edits that need a later build out of the active core. The six new Operation
contracts, their exports, evidence schema and descriptive node changes are live. Capability
bindings and changes to existing provider operations are exact patches here:

- [049a](049a.md), with [049a.patch](049a.patch): root verbs in object-management.v1,
  management.v1, root-management and engine implements.
- [049b](049b.md), with [049b.patch](049b.patch): machine-provider.v1, list-machines.v1,
  place.v1, the checker provider-operation list and the recorded 0.1.0 exception.
- [Vector impact](vector-impact.md): existing expectations to change, by zero-based step.

Apply [036b's 048a.patch](../036b-staged/048a.patch) first. Each patch here applies alone
on that baseline, and both apply together in either order. No shared hunk had to move to
049a. The patches are made against 038b's finished tree with that predecessor applied;
apply them from the repository root. No staged edit binds until applied under spec/.

The local .gitattributes sets `*.patch -whitespace`: a unified diff's empty context line
has one space, which must remain exact. Markdown has no trailing whitespace. The scratch
checks use fresh copies outside this repository, excluding build output; each sequence
checks and applies every patch, then checks schemas and documentation links. No vector,
engine code or live checker is changed by 038b.

Four related entries fall outside 038b's named promise scope and need coordinator follow-up:
PRV-027 in [providers.yaml](../../../spec/promises/providers.yaml) and LIF-036 in
[lifecycle.yaml](../../../spec/conventions/lifecycle.yaml) still say a node holds labels last
reported, whereas ENR-010 fixes them when made. DL18 in
[implementation-defined.yaml](../../../spec/implementation-defined.yaml) still gives every URL
wait an unknown ending; IFC-049 now explicitly makes enrollment the succeeded exception.
IFC-016 in [implementation.yaml](../../../spec/interface/implementation.yaml) still permits
only sign-in URL elicitation and ends any other URL elicitation unknown, even for root.
Its relay restriction must also permit the engine's secret-free enrollment URL flow under
ENR-003 and IFC-049, including successful completion on retry or at DL18, while preserving
rejection of unsolicited implementation elicitation. 038b leaves those four entries unchanged
and reports them for the coordinator to align.
