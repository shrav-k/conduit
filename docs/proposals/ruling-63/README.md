# Ruling 63: how the S1 rerun is counted, the documented style, and package templates

Status: in force (ruled by the owner on 2026-11-10). Holds now: [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 13. Proof, the build, experiments and measurement](../../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement) (spec pending except where a promise is cited). This record says why.

This is the record of a
ruling. Where it and the specification differ, the specification governs.

[Experiment 011](../../../experiments/011-short-form-spike/results-2026-11-10.md) translated S1's
states, written in ruling 55's short form, through today's loader and core, and counted 55 lines in
flow style and 97 in block style against the baseline's 64. The coordinator showed where T1's lines
went and that the bar turns on how lines are counted and written, and put three decisions, fixed
before the rerun as ruling 45 requires. The owner: "All as recommended".

## The decisions

1. **The counting rules for the S1 rerun**:
   - lines that `conduit install` writes (package pins) are written by a tool, not by the author,
     consistent with ruling 53's decision 5;
   - a command counts as a line only when it carries a value the author chose, such as a path or a
     name; a bare `conduit apply` does not;
   - a line removed and later restored within one task counts once, the baseline's own rule;
   - the block-style reading and the strict reading (every command a line) are reported beside the
     primary one.
2. **The documented style**: the specification's examples and the guides write grants and
   definitions in flow style, and `conduit init` writes a flow-style skeleton; block style stays
   legal.
3. **Packages ship the standard Conduit templates of their capabilities**, such as `read` and
   `read-write` for the filesystem package, which authors refer to with `from:`; the measurement plan
   counts them as provider work.

## What follows

The measurement plan for the rerun states these rules before it runs; the filesystem fixture
package gains its templates when the first spec batch is built.
