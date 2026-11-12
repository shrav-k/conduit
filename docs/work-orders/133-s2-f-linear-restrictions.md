# Work order 133 (S2-F): S2's restriction contracts, as a contracts-only package

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates;
use it for nothing else. Run one cargo command at a time, and never print a whole large file: use
`head`, `tail`, `grep`, `sed -n` or a count.

## What and why

S2 shares one service login between two Agents with different restrictions
([its pre-registration](../../experiments/013-s2-shared-login/README.md), accepted under ruling 75
D). Its O3 was answered: the restriction D05 qualifies is **explicit issue IDs with a deadline,
refused outside that scope**. No core contract supplies it (S2's G2): it belongs to a package's
contracts. [S2's plan](../../experiments/013-s2-shared-login/plan.md) makes it S2-F, a
contracts-only package that S2's two Linear implementations (S2-L1, the Conduit-aware one, and
S2-L2, the adapter) both pin. The engine's vectors write their own fixtures from S2's README,
independently of this package (the plan's FLT-D and SEC-D rows); where they differ, S2's README
wins and this package is fixed. This order writes it.

Read first:

- S2's pre-registration, above all "What it must show" (P3 and P4), "Measures" (D01 and D05),
  "Vectors expected" (V1, V2 and V5) and "Open for the owner" with its answers; and S2's plan, its
  rows S2-F, S2-L1 and S2-L2 and decision rules at the head of "The orders";
- CTR-002, CTR-004 and CTR-005 in `spec/promises/contracts.yaml` (Operation and Requirement
  contracts, `inputs`); CON-018 and CON-022 in `spec/promises/conduits.yaml`; IFC-019, IFC-020 and
  IFC-026 in `spec/interface/implementation.yaml` (applying a filter, onward calls, and the
  `reason` given in place of an outcome); OPS-009 and OPS-024 in
  `spec/conventions/operations.yaml` (a repeated read and a filter's deadline); ACC-029 and ACC-030
  in `spec/conventions/access.yaml` and `bound.v1` in `spec/core/requirements.yaml`, which is not a
  filter and which this package does not touch;
- a package's form: `packages/agent-codex/` (`conduit.yaml`, `contracts.yaml`) and
  `packages/provider-local-machine/`, whose test validates its YAML against `spec/schemas/`.

## The decisions it applies

1. **Where it lives.** `packages/scenario-s2-linear-restrictions/`: `conduit.yaml`,
   `contracts.yaml`, a README, and a small Rust crate (its own `Cargo.toml`, `Cargo.lock`,
   `rust-toolchain.toml` at 1.97.1, `.gitignore`) whose tests check the YAML and whose one binary,
   `package`, writes a deterministic archive and its digest as the other packages' do
   (`src/bin/package.rs`), so that S2-L1 and S2-L2 can pin it by digest (MAN-005) and install it by
   `ARCHIVE#SHA256`. It has no implementation. Its README says first that it serves scenario S2 and is not a
   package for general use (the admin's rule for scenario packages).
2. **The contracts.** `conduit.yaml`: address `example.org/conduit/scenario-s2-linear-restrictions`,
   version 0.1.0, exporting:
   - three Operations, named verb first as the core's are: `read-issue.v1`, `list-issues.v1` and
     `update-issue.v1`. An issue is named by its identifier in Linear's form: an upper-case team
     key, a hyphen and a number, as `ENG-123`; the same form in every input and in the filter, as a
     schema pattern, matched exactly.
     `read-issue.v1` and `list-issues.v1` have no `effect` and do not start model work, so that a
     repeated read is dispatched again (OPS-024). `list-issues.v1` takes a `limit` and no cursor,
     and returns no count and no cursor. `update-issue.v1` changes only an issue's `title` and
     `description`, never its team, parent or anything that moves it, has an `effect`, declares
     outputs (the issue as it stands after the change, so that a filtered keyed repeat reports a
     conflict, OPS-034), and has no `deduplicates`. Set each tool's `idempotentHint` and
     `openWorldHint` deliberately, and say why;
   - two Capabilities, since a Conduit has exactly one capability (CON-001) and
     an offered capability needs an implementation of each of its operations (TPL-004): one for
     reading (read and list) and one for writing (update), so that a read-only implementation can
     qualify;
   - the Requirement `issue-scope.v1`, a filter (CTR-005: it is not a core requirement, so the
     target's implementations apply it, IFC-019), with `inputs`: `issues`, the identifiers in scope
     (at least one), and `deadline`, a `date-time` with its offset (as the core's `expires` is).

   The operation that S2's D01 qualifies on an unchanged plain server is that server's own, with
   its own input names (PKG-020): S2-L2 declares it when it pins the server. These contracts are
   for the adapter and the Conduit-aware implementation.
3. **What the filter means**, written in the Requirement's description, as package behavior:
   - the filter is judged per call, from the filters sent with that call (IFC-018), so two
     Conduits with different arguments are judged apart;
   - a call to read or update an issue outside `issues` is forbidden, and that is judged from the
     call's arguments before any lookup, so a refusal never depends on whether the issue exists
     (IFC-026: never after any part is performed); an issue in scope that does not exist is a
     `failed` outcome, not a refusal;
   - a list returns only issues in `issues`, never more, whatever the service holds;
   - what an answer returns: an issue's identifier, title, description, state and updated time,
     and never its relations, parent, sub-issues or comments; mentions of other issues inside an
     issue's text are returned as they are, since they are content, not references;
   - a call that the implementation receives at or after `deadline`, by its own clock at the time
     it receives the call, is forbidden, a repeated read included (OPS-024: the read is dispatched
     again and the filter decides afresh). The deadline is judged once, at receipt: a call allowed
     then completes even if the deadline passes while it runs. Times are compared as instants,
     across offsets. A test harness may set the implementation's clock (S2-H);
   - the scope is judged on every call against the call's own arguments and the filter's current
     inputs, never against an earlier answer (authorization data that changed is seen at once);
   - an implementation that receives the filter carries it on any onward call it makes (IFC-020).
     Whether S2's implementations make one is S2-L1's and S2-L2's matter: report as a finding that
     D05's onward-call case is unexercised unless one of them does;
   - forbidden means the implementation performs nothing of the call and answers in IFC-026's
     form: `refused` under Conduit's `reason` key, the filter item under its `restriction` key, and
     `isError: true`, in place of an outcome. "Cannot apply" is only IFC-019's case, an argument for
     an input that the supported version does not declare, which answers `unavailable` the same
     way; any other ground is this package's own and is labelled so.

   Invent no core rule: everything here is the package's own contract, and the README says which
   promise each point rests on.
4. **The README** maps each of D05's restriction cases (changed authorization data, keyed repeat,
   output delivery, onward calls, revocation) to the clause above that decides it, or says that
   the case is decided by Conduit and not by this filter (revocation is the Conduit's close). It
   also states what the filter does not do: it is not a bound (ACC-029), and it confines nothing
   outside Conduit (ACC-027).
5. **Tests** (reuse the YAML test of `packages/provider-local-machine/tests/schema_drift.rs`):
   - `conduit.yaml` validates against the manifest schema and `contracts.yaml` against the document
     schema in `spec/schemas/`, which also applies the schema profile to every operation schema and
     requirement input; skipped, saying so, where `spec/` is absent (`tools/check_schemas.py` does
     not read `packages/`); and each schema compiles under JSON Schema Draft 2020-12;
   - every export names a document of the package (PKG-018), and names are unique;
   - sample arguments pass or fail as they should: an empty `issues`, an identifier in another form,
     a date-only `deadline`, and a `deadline` with no offset each fail;
   - read and list declare no `effect`, and update declares outputs;
   - the `package` binary's archive is the same, byte for byte, when built twice. This, with the
     schema tests, is what the plan calls admission.
6. **What this order does not do:** no implementation, no stub service, no vector, no change to any
   core contract, and no change outside this package.

## Scope

Change only `packages/scenario-s2-linear-restrictions/` (new).

## Done when

In `packages/scenario-s2-linear-restrictions/`: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`. From the repository root:
`python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with `bash`.
Each prints nothing when it passes:

```bash
D=packages/scenario-s2-linear-restrictions
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "lin_api_|linear\.app/" $D --exclude-dir=target --exclude-dir=dist
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target --exclude-dir=dist
```

Use synthetic identifiers only, with no real workspace or team name.

`target/` and `dist/` stay ignored and unstaged; run `cargo run --release --bin package` twice and
report the digest. Report each name and each schema field you chose as your own
choice, and anything `spec/` leaves open as a finding. Do not stage anything.
