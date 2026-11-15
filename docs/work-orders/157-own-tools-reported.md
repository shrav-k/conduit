# Work order 157: an Agent's own tools reported as its Type declares them

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list in `vectors` the six cases this order adds. This order has network
access only because the engine's tests bind loopback ports; use it for nothing else, and reach no
other machine. Run one cargo command at a time, and never print a whole large file, fixture or
log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 15 of [its orders](../proposals/full-review/orders.md))
found that the first build can never report that an Agent's Type states its own tools, and that
no vector can tell
([FR-546](../proposals/full-review/findings/GL2-vectors.md#fr-546), with its verifier's
corrections). `type-spec.v1` makes `toolsDirectory` an object, `{directory: <pointer>}`
(`spec/schemas/contracts.yaml` lines 198 to 203). But `own_tools` in
`engine/crates/core/src/inventory.rs` reads the member itself as a string (line 320). No Type
that the loader accepts holds a string there, so `inspect.v1` answers `ownTools: {stated: false}`
for every Agent, and every `targetOwnTools` does the same (line 363), against LIF-013
(`spec/conventions/lifecycle.yaml` lines 266 to 290). All 494 cases pass because the fixtures
that state the member, under `conformance/language/loader/own-tools/`, are only loaded, never
registered or inspected. The engine does not check CTR-014's rules on the place either
(`spec/promises/contracts.yaml` lines 333 to 368): no source under `engine/` names CTR-014.

No package states the member today. On the branch of orders 050a and 076, the `.v2` Agent Types
of the Agent packages' 0.2.0 will state `toolsDirectory: {directory: /directory}` (order 075,
lines 55 to 58). From then on, for Agents whose own tools run unconfined, the report would say
the opposite of the truth. This order fixes the reading, checks CTR-014's three rules on the
place, and adds the six cases that would have caught both. This is the coordinator's fix F44
([decisions](../proposals/full-review/decisions.md)), a finding of the first build fixed with
vectors (ruling 46, point 4). It adds no behavior beyond what CTR-014 and LIF-013 already
promise. It must merge before those Types state the member.

## The decisions it applies

1. **The inventory reads the pointer inside the member.** In `own_tools`
   (`engine/crates/core/src/inventory.rs` line 320), the pointer is the string at
   `/spec/toolsDirectory/directory` in the Type `ty`, not the member `/spec/toolsDirectory`
   itself. Nothing else in the function changes, and `targetOwnTools` (line 363) is fixed through
   the same function. A Type that states no `toolsDirectory` still gives `{stated: false}`: a later
   order changes that form
   ([FR-200](../proposals/full-review/findings/X04-stated-limits.md#fr-200), fix F37). Add no
   runtime or domain-of-trust member either: ruling 78, point 14, leaves that to a later order.
2. **CTR-014's three rules are checked when packages and projects are resolved.** Put the check
   in `engine/crates/core/src/packages.rs`, in `Catalog::resolve`'s loop over each scope's
   documents, beside the FLEET-001 check (lines 586 to 599). That loop runs for a submitted
   project, for every package snapshot and for an archive being installed
   (`engine/crates/core/src/archives.rs` line 298), and never for the core package. Take each
   Type whose `spec` states `toolsDirectory`, and let P be its `directory`. Three rules apply:
   - (a) the Type's `schema` requires P and every member on the path to it, by the same walk as
     MAN-017's test of a bound place (`branches`, `binding` and `binding_parts`, lines 1089 to
     1253), which follows `$ref` and each branch of `anyOf` and `oneOf`, in the Type's own scope;
   - (b) in every branch, the place admits string, null or both, and nothing else (the mask of
     `binding_mask`, lines 1255 to 1288, holds no bit but null's and string's). This is narrower
     than MAN-017, which also admits a number and a boolean. MAN-017's exemption for a place that
     `secret-reference.v1` governs does not carry over: do not skip a part marked `secret`, as
     MAN-017 does at line 1242. That part, with its `$ref` removed, admits every type, so it
     breaks this rule, as the schema it stands for, which admits only objects, would;
   - (c) no control that the Type lists under `controls` has, under `changes`, P, a pointer
     inside P or a pointer over P. Find each control's contract in the Type's scope the way the
     loop above resolves every selection (`resolve_name`, line 756, and `indexes`). Compare
     pointers by decoded reference tokens (`~1`, then `~0`), as CTR-006 does. The empty pointer
     has no tokens and is over every pointer; `/dir` is neither over nor inside `/directory`.
     Do not reuse `registration::tokens` (`engine/crates/core/src/registration.rs` line 173) as
     it stands, which gives the empty pointer one empty token.

   Rules (a) and (b) are one walk. Where it fails, the Type gets one error diagnostic that says
   which rule failed. Rule (c) adds one error diagnostic for each control that reaches P. Each
   diagnostic's `ruleId` is `CTR-014`. Its message names the scope, the Type, P and the cause,
   and also the control where one is the cause. The resolution is then rejected, as for the
   loop's other errors (line 612). Share the walk with MAN-017 however you choose, but change
   nothing that MAN-017 or any other rule accepts or rejects: the
   `conformance/language/packages/binding-*` cases still pass unchanged. The closed shape and the
   family Agent stay the loader's checks (SUB-005). In the module's `tests` (line 1357), add a
   unit test of the pointer comparison. It covers an equal pointer, one inside, one over, the
   empty pointer, and three pointers that do not reach: `/dir` against `/directory`, `/tools/a`
   against `/tools/b`, and `/a~1b` against `/a`.
3. **Three inspection cases** go in a new directory, `conformance/conventions/inventory/own-tools/`,
   with each case in a subdirectory of its own (`string`, `null`, `holder`). Their one fixture
   sits beside them (`fixture.json`, `manifest.json`, `documents.json`), modelled on
   `conformance/conventions/inventory/e4/` and its case `inventory`. Change nothing of e4's,
   since every e4 case names its files. The fixture holds:
   - an Agent Type `tools-agent.v1` whose schema is `{type: object, properties: {directory:
     {type: [string, 'null'], default: null}}, required: [directory], additionalProperties:
     false}` and which states `toolsDirectory: {directory: /directory}`. It offers one capability
     of two operations that the fixture's plain server implements: `ping.v1`, and `work.v1`,
     marked `startsModelWork: true`;
   - a Fleet `group` from the core's `fleet` template, as in e4;
   - four Agent instances of that Type, each owned by `group` (FLEET-002) and each from a
     template whose `config` gives its value: `named`, whose configuration holds
     `directory: /work`; `empty`, which holds `""`; `cleared`, which holds null; and `defaulted`,
     which writes nothing there, so that SCH-009 fills in null;
   - an Interface, `holder`, that consumes two Conduits to `named`: one listing `work.v1`, and
     one listing `ping.v1` alone.

   Each case starts as e4's `inventory` does, from the same initial state, and registers the
   fixture as its step 1 does, with the same evidence. Root then inspects each Agent through
   `group.inspect.v1`, the default Conduit that FLEET-005 gives root to the Fleet. It inspects
   `holder` through `management.inspect.v1`, as e4's `inventory-scope` inspects `viewer`. An
   expected map is only a projection, so use `absent` to assert every field that LIF-013 leaves
   out. An expected array must match in length.
   - `inventory-own-tools-string` (promises LIF-013, CTR-014): `named` gives
     `{stated: true, pointer: /directory, directoryNamed: true, directory: /work, enforcement:
     neither}`, and `empty` gives the same with `directory: ""`.
   - `inventory-own-tools-null` (LIF-013, CTR-014): `cleared` and `defaulted` each give
     `{stated: true, pointer: /directory, directoryNamed: false}`, with `directory` and
     `enforcement` absent.
   - `inventory-own-tools-holder` (LIF-013): root first opens both of `holder`'s Conduits with
     `access.open.v1`, as e4's `inventory` opens its Conduits. In `holder`'s `consumes`, which
     holds exactly these two entries, the entry for the Conduit that lists `work.v1` carries
     `targetOwnTools` equal to `named`'s `ownTools` above. The entry for the Conduit that lists
     `ping.v1` alone carries none.

   The string and null cases have `layers: [convention, language]`, and the holder case has
   `[convention]`.
4. **Three rejection cases** go under `conformance/language/contracts/`. Each is one
   `resolve-project` step, modelled on `conformance/language/packages/binding-nullable-parent/`:
   a project of its own, no snapshot (`snapshots: []`), and `layers: [language]`. Each project
   holds one Agent Type that breaks one rule and is otherwise valid: with that fault removed, the
   same project resolves as accepted. Check this once for each, say how in your report, and
   commit nothing of it. Each step expects what that model's step expects, with `CTR-014` in
   place of `MAN-017` and with no message asserted: `accepted: false`, `reason: invalid`,
   exactly one diagnostic `{ruleId: CTR-014}`, `required_promises: [CTR-014]`, no records, no
   external intents, `stateUnchanged: true`, and `/reply/environment`, `/reply/digest` and
   `/reply/contexts` absent. Each case's promises are CTR-014 alone.
   - `contracts-own-tools-optional`, in directory `own-tools-optional`: the Type states
     `toolsDirectory: {directory: /directory}`, and its schema describes `directory` as
     `[string, 'null']` without requiring it.
   - `contracts-own-tools-not-string`, in `own-tools-not-string`: the place is required and its
     schema is `{type: [string, number]}`, which MAN-017's test alone would accept.
   - `contracts-own-tools-control`, in `own-tools-control`: the Type states
     `toolsDirectory: {directory: /tools/directory}`. Its schema requires `tools` and its
     `directory`, and gives the place as `[string, 'null']`. The Type lists one control, valid
     under CTR-006, whose `changes` holds `/tools`, a pointer over the place.
5. **The coverage index and the declaration.** Since order 071
   ([its file](071-conformance-manifest-pieces.md)), the coverage index is in pieces: a case's
   entry is in the `index.yaml` of the directory that holds the case's own directory, entries are
   sorted by `id` in byte order, and `conformance/manifest.yaml` holds no `cases`
   ([the rules](../../conformance/README.md#the-coverage-index-and-the-case-schema-in-pieces)).
   Give each new case one entry: its `id`, its `case` path relative to `conformance/`, its
   `promises` exactly as the case lists them, and `safety_rows: []`, in that order. The three
   inspection cases' entries make a new piece,
   `conformance/conventions/inventory/own-tools/index.yaml`, beside their fixture. The three
   rejection cases' entries go in `conformance/language/contracts/index.yaml`, before its one
   entry today, `contracts-sign-in-logins`. Add them as new work does: write the six entries in a
   `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag. The tool moves each
   entry into its piece, in `id` order and in the pieces' printed form, and leaves
   `manifest.yaml`, its `s1_coverage` included, byte for byte as it was. The cases use only input
   kinds and expectations that exist: add nothing under `conformance/case-schema/`, and leave
   `conformance/case.schema.json` unchanged. LIF-013 and CTR-014 keep `vectors: []`, and nothing
   under `spec/` changes. Write `docs/work-orders/157-declared.yaml` with one `added` declaration
   for each of the six cases, in the form of `docs/work-orders/081-declared.yaml`, and nothing
   more: no existing case changes.

## Scope

Change only `engine/crates/core/src/inventory.rs`, `engine/crates/core/src/packages.rs` and
`conformance/language/contracts/index.yaml`. Add `conformance/conventions/inventory/own-tools/`
(the three cases, their fixture and their piece),
`conformance/language/contracts/own-tools-optional/`,
`conformance/language/contracts/own-tools-not-string/`,
`conformance/language/contracts/own-tools-control/` and `docs/work-orders/157-declared.yaml`.
`conformance/manifest.yaml` holds the new entries only until the pieces tool moves them, and ends
unchanged. Change no existing case or fixture, neither `conformance/case.schema.json` nor anything
under `conformance/case-schema/`, nothing under `spec/`, and no other source.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. inventory-own-tools-
cargo run --release -p conduit-conformance -- .. contracts-own-tools-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The pieces tool prints `Nothing to move.` Each of the two runner commands ends
`3 cases passed`. Since order 071 the runner runs the cases on every thread the machine offers
unless `CONDUIT_VECTOR_THREADS` names a number, and at 24 threads it ran out of memory (order
071's Record); hence the 4.

Each set of cases must also be able to fail. Run the `inventory-own-tools-` command once with
only decision 1's change undone, and the `contracts-own-tools-` command once with only decision
2's check turned off. Each run must fail with the line `FAIL 0 cases passed; 3 failed`. Then
restore the change and run the command again.

From the repository root, each passes:

```bash
python3 tools/vector_delta.py HEAD --declared docs/work-orders/157-declared.yaml
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/check_docs.py
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema
git diff --check
```

`vector_delta.py` must end `Vector delta: 6 changes; 0 flips; 0 undeclared; 0 errors.` It reads
the declaration with PyYAML, so where `python3` lacks it, run it with `/usr/bin/python3`. The
`git status` command prints nothing. Do not stage anything. In your report, mark each choice you
made yourself as your own, and give the message of each rejection's diagnostic, the `FAIL` line of
each run that had to fail, and the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. On
the pull request, the admin shows the engine build, the four engine shards and the doctests
passing. A fresh Codex verifier and a fresh read-only Opus reader read the result before it
merges ([ruling 79](../proposals/ruling-79/README.md), point 2).
