- Added: IFC-054's warning: each attempt that PND-016 dispatches on a report whose
  reason is `warning` or `cannot-tell` carries to its caller, once it ends, a warning
  diagnostic in `io.github.shrav-k.conduit/admission`, with `admission-hold.v1` and
  `dispatched` true, and the record of its dispatch holds it in `admissionWarning`;
  nothing of it reaches the called Agent's implementation (DEC-098, DEC-099, DEC-120).
- Added: CTR-015 refuses the open of a Conduit whose consumer is an Agent and that lists
  its target's admission control, whoever opens it, and says that nothing of `admission`
  reaches a call dispatched to an Agent's implementation (DEC-119, DEC-120).
- Added: ACC-058's rows for `admission-hook.v1` and a login's admission control: an
  Agent's ask for either is refused, raising nothing; the second is matched on the Types
  the engine holds, which its result and ACC-056's tier 1 may then depend on (DEC-079,
  DEC-119).
- Changed: PND-016, ACC-006, ACC-056, REC-002, `admission-hold.v1` (`dispatched` true
  for `warning` and `cannot-tell` alone), `diagnostic.v1`, `record.v1` and the
  definitions' `login Resource` and `Conduit's keys`, for the warning and the refusal
  (DEC-099, DEC-119).
- Fixed: CTR-015's two valid examples name their package `example.org/acme/admission`, a path
  of two parts, which `tools/set_names.py --check` cannot read as one of Conduit's `_meta` keys
  written with another prefix (order 318's merge; ruling 34).
