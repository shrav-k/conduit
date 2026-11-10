# Live S1 project

This is S1's fixture project with independently packaged Claude Code and Codex Types. Both
reach inputs are false on each Agent template. `turn-a` and `turn-b` authorize the root
Interface to request explicit model turns; setup itself starts none.

The checked-in Agent digests pin the metadata-only archives in
[the contract fixtures](../../../conformance/fixtures/agents/agent-claude-code/README.md), so
[the registration vector](../../../conformance/language/registration/live-agents/case.yaml)
can run on any host. Those archives contain no executable. Before a live run:

1. In each package directory, run `cargo run --release --bin package`.
2. Copy this project to a fresh local directory. In its `conduit.yaml`, replace each Agent
   digest with `sha256:` followed by its host archive's printed hash. Keep the address and
   version unchanged. The engine never silently replaces a submitted digest.
3. In `instances.yaml`, set the shared/history roots to two existing directories, separate
   from the engine store and every Agent working directory.
4. Initialize a fresh store, register the copy with both printed package arguments, and serve:

```sh
reference-model/target/release/conduit init --store /tmp/conduit-live-store
reference-model/target/release/conduit register --store /tmp/conduit-live-store \
  --package /absolute/agent-claude-code.tar.gz#HOST_SHA256 \
  --package /absolute/agent-codex.tar.gz#HOST_SHA256 /absolute/live-project
reference-model/target/release/conduit serve --store /tmp/conduit-live-store
```

In another terminal, request turns explicitly:

```sh
reference-model/target/release/conduit tools --store /tmp/conduit-live-store --as agent-a
reference-model/target/release/conduit call --store /tmp/conduit-live-store \
  turn-a.turn.v1 '{"message":"List the project directory using its engine tool and report the result."}'
reference-model/target/release/conduit call --store /tmp/conduit-live-store \
  turn-b.turn.v1 '{"message":"List the project directory using its engine tool and report the result."}'
```

The executable `claude` or `codex` must be on the runner's permitted PATH. Both clients use
the owner's shared subscription login. These commands start real model work only at call;
work order 023 did not run them. The admin's
[closeout plan](../../../docs/proposals/first-build/increment-3/closeout.md) supplies the
sentinels, evidence and further checks needed before claiming increment 3 complete.
