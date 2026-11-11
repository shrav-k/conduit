# Work order 078 (AGT): the Codex Agent item that finds its wrapper by name

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access, to fetch
crates; use it for nothing else. Never print a whole large file: use `head`, `tail`, `grep` or a
count. Run one cargo command at a time. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a credential. Never run the real
`codex` program, or any `agent-codex` that your PATH finds: the tests use the package's scripted
client and the binary that cargo builds.

## What and why

Your tree is cut from order 076's branch, `wo/076-agents-v2-folder-grant`, not from `main`. Order
050a added `codex.v2` and its item `wrapper-v2` to `packages/agent-codex`, and order 076 made the
directory the grant ([ruling 75](../proposals/ruling-75/README.md), point B1). This order's branch
is kept with theirs, not merged on its own: it merges with order 050, as version 0.2.0. If 076's
branch moves, the admin rebases this one.

An Agent placed on a machine other than the engine's is run by a runtime provider other than the
engine's own runner, which runs only on the machine the engine runs on (PRV-019,
`spec/promises/providers.yaml:544-546`). `wrapper-v2`'s program is `bin/agent-codex`
(`packages/agent-codex/conduit.yaml:22`). That is a relative path: a file of the archive, on the
engine's machine (MAN-010, `spec/promises/manifest.yaml:219-222`). The engine asks no runtime
provider but its own runner to run such an item, and that part of the setup ends failed (PRV-007,
`spec/promises/providers.yaml:218-221`; the core's check is
`engine/crates/core/src/lifecycle.rs:892-909`, over the list that
`engine/crates/core/src/packages.rs:619-626` makes). The archive's binary is also a host build,
not one for another operating system or architecture (`packages/agent-codex/README.md:32-33`).

A program that holds no `/` is a name, found as the one who starts it finds programs (MAN-010,
lines 225-227). So this order adds an item `wrapper-v2-path`: `wrapper-v2` with
`command: [agent-codex]`. A runtime provider that runs it on another machine finds `agent-codex`
there. A `command` pins nothing (MAN-010, line 218): no digest covers the program found on that
machine, and the README must say so. MAN-010 (lines 222-223) has a server meant to run on another
machine published as a pinned `package` item. This item is a shortcut that stands in for that
form, the one the coordinator's plan below chose, and it does not claim to be that form. The scope
guard ([IN_FORCE.md](../../IN_FORCE.md), line 30) has a shortcut logged with the general behavior
it stands in for: the README says so (step 5), and the admin logs it in
[the owner's review list](../proposals/first-build/to-review-with-the-owner.md).

This is item 1 (AGT) of the coordinator's plan for enrolling the Windows computer through Conduit
and running Agents on it. The owner let the coordinator adopt that plan's recommendations
([ruling 76, later answers](../proposals/ruling-76/README.md#later-answers)). Alone, this order
proves nothing: no runtime provider for another machine exists yet. Four of the plan's choices
are held for the owner: the machine provider; which operating-system user the runs on that
machine execute as; how TLS is terminated at the engine's address; and whether that machine's
Windows interop and drives are turned off. This order decides and assumes none of them, and
nothing you write may decide or assume any of them. Decision 5 says what the README may not name.

Read MAN-010, PRV-007 and PRV-019 at the lines above, PRV-006 (`spec/promises/providers.yaml:165`)
and PLC-020 (`spec/promises/placement.yaml:270`). In the package, read `conduit.yaml`,
`contracts.yaml` (its `codex.v2` document), `README.md`, `src/bin/package.rs` and
`tests/package.rs`. Line numbers are from your tree before your changes. Your branch changes
nothing under `spec/` or `engine/` from 50e7cc5, the `main` commit it starts from.

The decisions. Apply them, and do not reopen them:

1. **One new item, `wrapper-v2-path`.** It is `wrapper-v2` but for `command: [agent-codex]`: the
   same `implements`, `environment`, `plain` and `tools`. It has no `nodeSelector` and no other
   key.
2. **The Type's default stays `wrapper-v2`** (`contracts.yaml:114`). A template opts in by
   selecting the new item. A template that selects nothing still runs the archive's own binary.
3. **Codex only.** `agent-claude-code` gets no such item: the plan names the Codex Agent alone.
4. **No source change.** The wrapper already finds `codex` by name on its own PATH.
5. **The README names no runtime provider, machine provider, machine, operating-system user,
   network or TLS arrangement.** Those are the held choices above. A runtime provider's own
   contract states them.
6. **`codex.v2`'s description says where the own tools run.** Its description
   (`contracts.yaml:81-99`) says that they "run as the engine's operating-system user" and reach
   "the engine's store, root's client token" (lines 85-87), as
   [ruling 70](../proposals/ruling-70/README.md), point 2, has the Type's contract say. Under a
   runtime provider other than the engine's own runner, `wrapper-v2-path` runs on another machine
   (PRV-019), where that is not so. Step 2 limits that sentence to the engine's own runner and
   leaves the rest to the provider's contract, as decision 5 does for the README; it names no
   user. This is the time to change it: once 0.2.0 publishes, NAM-024
   (`spec/promises/naming.yaml:414-440`) forbids a later release to change the description, and
   a change would need a new contract, `codex.v3`.

## What to do

1. **The item.** Append exactly these lines to `packages/agent-codex/conduit.yaml`, after
   `wrapper-v2` (lines 20-31), at the end of the file:

   ```yaml
     wrapper-v2-path:
       implements: ['#turn.v1']
       command: [agent-codex]
       environment:
         AGENT_DIRECTORY: /directory
         AGENT_TURN_DEADLINE_SECONDS: /turnDeadlineSeconds
         AGENT_EFFORT: /effort
         AGENT_ACCOUNT_SERVICES: /accountServices
         AGENT_MODEL: /model
       plain: true
       tools:
         '#turn.v1': turn
   ```

   Each line is indented as `wrapper-v2`'s are: the item's name by two spaces. Change no existing
   line, `version` included: it stays 0.2.0.
2. **The Type's description** (decision 6). In `packages/agent-codex/contracts.yaml`, in
   `codex.v2`'s `description`, replace lines 85 and 86:

   ```yaml
       client's sandbox and approval prompts off. They run as the engine's operating-system user
       and nothing confines them in v0. They reach every file, process, network address, credential and
   ```

   with exactly these three lines:

   ```yaml
       client's sandbox and approval prompts off. On the engine's own runner, they run as the
       engine's operating-system user and nothing confines them in v0. They reach every file,
       process, network address, credential and
   ```

   Then replace line 90:

   ```yaml
       records it (ruling 70). The client's own browser, desktop-control, goal, image-generation,
   ```

   with exactly these three lines:

   ```yaml
       records it (ruling 70). Under another runtime provider, these tools run on the machine of
       the instance's node, as the operating-system user and with the limits that its contract
       states. The client's own browser, desktop-control, goal, image-generation,
   ```

   Each line is indented by four spaces, as the description's other lines are. The description
   is a folded scalar, so a short line changes nothing it says. Change no other line of
   `contracts.yaml`: `codex.v2`'s `implementation` stays `{'#turn.v1': '#wrapper-v2'}`
   (decision 2), and the file's other documents are unchanged.
3. **The wrapper, started by name.** Change no source. Read how the wrapper finds `codex`
   (`src/lib.rs:44`, spawned at line 453), and which path it gives Codex for its header helper:
   `std::env::current_exe()` at line 45, written into the helper's command at lines 155-158. Say
   in your report whether either depends on how the wrapper itself was started, on Linux and on
   macOS, and whether that can matter for a wrapper found by name. If something must change,
   report it as a finding and change nothing.
4. **A manifest test**, in a new file `packages/agent-codex/tests/manifest.rs`. Add no dependency.
   Read `conduit.yaml` as text from `env!("CARGO_MANIFEST_DIR")`, as `tests/package.rs` does.
   Split its `implementations:` map into items: an item starts at a line of two spaces, a name and
   `:`, and holds the more deeply indented lines that follow it. The test checks that:
   - the items are `wrapper`, `wrapper-v2` and `wrapper-v2-path`, in that order;
   - `wrapper-v2`'s command line is `    command: [bin/` + `agent_adapter::WRAPPER` + `]`, the
     archive member that `src/bin/package.rs` writes (line 43);
   - `wrapper-v2-path`'s lines are `wrapper-v2`'s, with only that line changed, to
     `    command: [` + `agent_adapter::WRAPPER` + `]`;
   - `agent_adapter::WRAPPER` holds no `/`, so that MAN-010 reads the program as a name;
   - `contracts.yaml` does not name `wrapper-v2-path`, so that no Type selects it by default.

   Each assertion's message names the item and the line that differs. Then show that the test can
   fail: change `wrapper-v2-path`'s command to `[bin/agent-codex]`, run
   `cargo test --release --test manifest`, and report its failure message. Restore the line and
   run it again.
5. **The README**, `packages/agent-codex/README.md`.
   - After line 122 ("The package also exports codex.v2, implemented by the separate `wrapper-v2`
     item."), add a sentence: a template may select `wrapper-v2-path` instead, as the section on
     it says.
   - Add a section `## The wrapper-v2-path item`, just before
     `## Turn boundaries and failures (both versions)` (line 185). It says, in this order:
     - **What it is:** `wrapper-v2` with `command: [agent-codex]`. It binds the same inputs to the
       same variables, so everything this README says of `.v2` holds for it. `codex.v2`'s default
       is still `wrapper-v2`.
     - **Why:** `bin/agent-codex` is a relative path, a file of the archive on the engine's
       machine (MAN-010), and the engine asks no runtime provider but its own runner to run such
       an item (PRV-007). The archive's binary is a host build.
     - **How it is found:** `agent-codex` holds no `/`, so it is a name, found as the one who
       starts it finds programs (MAN-010). Under a runtime provider other than the engine's own
       runner, that is on the machine of the instance's node (PRV-019), as that provider's
       contract states. The wrapper then finds `codex` on its own PATH, as it always does. On the
       engine's own runner, the item runs whatever `agent-codex` is found on the engine's
       machine, not the archive's: use `wrapper-v2` there.
     - **It pins nothing:** a `command` pins nothing (MAN-010). The archive's digest does not
       cover the program found on that machine. Whoever prepares the machine builds `agent-codex`
       there from the same source as the installed archive, and puts it and `codex` where that
       provider's runs find them. Then name the general form this item stands in for: MAN-010
       publishes a server meant to run on another machine as a pinned `package` item, in those
       words. Say that this item is a shortcut that stands in for one, and is not one.
     - **How to select it:** a template writes
       `implementation: {example.org/conduit/agent-codex#turn.v1: example.org/conduit/agent-codex#wrapper-v2-path}`
       beside its `type` and `config`. An instance of it names its runtime provider in `runtime`
       (PRV-006). An instance that names none runs on the engine's own runner, on the engine's
       machine (PRV-019).
     - **Its directory:** `directory` is then a path on that machine (PLC-020), and the wrapper
       checks it there.
     - **Its reach:** the paragraph on what nothing confines (lines 142-154) describes the
       engine's own runner. Under another runtime provider, the client and its own tools run on
       that machine, as the operating-system user that the provider runs them as, with that
       user's files and whatever client login that user has there. That provider's contract
       states what, if anything, confines them. This package copies no login.
   - Follow decision 5. Cite no line numbers in the README. Write no absolute local path. Keep
     the README's plain style.
6. **Check the manifest with the engine,** as order 050a's step 11 did
   (`docs/work-orders/050a-agents-v2-own-tools.md:412`). It checks what the package tests cannot:
   that the engine accepts a template that selects the item, under MAN-017's bindings.
   - In `engine/`, run `cargo build --release`. Change nothing there.
   - In `packages/agent-codex/`, run `cargo run --release --bin package`. It prints
     `ARCHIVE#SHA256`.
   - In a new temporary directory T outside the repository, run
     `engine/target/release/conduit init --store T/store`, and again with `--store T/store-v1`.
   - Write a project P1 as `engine/crates/shell/tests/agents.rs:150-151` writes one. Its
     `conduit.yaml` pins `example.org/conduit/agent-codex` at version 0.2.0, with the digest
     printed. It has two Agent templates:
     - `default-v2`: type `example.org/conduit/agent-codex#codex.v2`, with `config: {}`;
     - `path-v2`: the same type, with `config: {directory: T}` and the `implementation` of step
       5.
   - Run `engine/target/release/conduit register --store T/store --package ARCHIVE#SHA256 P1`.
   - Write a project P2, pinned the same way, with one Agent template `path-v1`: type
     `example.org/conduit/agent-codex#codex.v1`, with
     `config: {accountServices: false, peerMessaging: false}` and the same `implementation`.
     Register it the same way, in `T/store-v1`. It must be refused: `.v1`'s schema has no
     `directory`, `turnDeadlineSeconds` or `effort`, and the item binds all three (MAN-017).
   - A registration of templates starts no wrapper and no client. Report each reply's `accepted`
     and its diagnostics.

## Scope

Change:

- `packages/agent-codex/conduit.yaml`: step 1's lines only;
- `packages/agent-codex/contracts.yaml`: step 2's lines only;
- `packages/agent-codex/README.md`;
- `packages/agent-codex/tests/manifest.rs`, new.

Do not change:

- the rest of `packages/agent-codex/`: the rest of `contracts.yaml`, `src/`, `Cargo.toml`,
  `Cargo.lock` and the other tests;
- `packages/agent-claude-code/` (decision 3);
- `engine/`, `spec/`, `conformance/` or `docs/`. The copies under `conformance/fixtures/agents/`
  stay at 0.1.1: order 050 updates them;
- any record.

This order changes no promise and no vector, so it has no declaration file. Report anything else
that needs changing.

One engine test fails on this branch, and the admin expects it to:
`live_s1_scripted_agents_call_engine_with_distinct_run_identities`
(`engine/crates/shell/tests/agents.rs:101`) compares the packages' `conduit.yaml` and
`contracts.yaml` with the 0.1.1 copies. Do not run the engine's test suite.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`
- `git diff --check`

From the repository root, these give the result stated:

- `git status --short` prints four lines: `packages/agent-codex/README.md`,
  `packages/agent-codex/conduit.yaml` and `packages/agent-codex/contracts.yaml` modified, and
  `packages/agent-codex/tests/manifest.rs` new.
- `git diff -U0 -- packages/agent-codex/conduit.yaml | grep '^-' | grep -v '^--- '` prints
  nothing.
- `git diff -U0 -- packages/agent-codex/conduit.yaml | grep '^+' | grep -v '^+++ '` prints step
  1's twelve lines, each after a `+`, and nothing else.
- `git diff -U0 -- packages/agent-codex/contracts.yaml | grep '^-' | grep -v '^--- '` prints the
  three lines that step 2 replaces (lines 85, 86 and 90), each after a `-`, and nothing else.
- `git diff -U0 -- packages/agent-codex/contracts.yaml | grep '^+' | grep -v '^+++ '` prints
  step 2's six new lines, each after a `+`, and nothing else.
- For each of `wrapper-v2-path`, `pins nothing`, `pinned .package. item`, `MAN-010`, `PRV-019`
  and `PLC-020`, `grep -c -- 'TEXT' packages/agent-codex/README.md` prints a number above 0. The
  README holds none of them before your change.
- These print nothing:
  - `grep -niwE 'ssh|sshd|tailscale|tailnet|wsl|windows|conduit-run|tls|https|systemd|sudo' packages/agent-codex/README.md packages/agent-codex/contracts.yaml packages/agent-codex/tests/manifest.rs`
  - `grep -nE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex/README.md packages/agent-codex/conduit.yaml packages/agent-codex/contracts.yaml packages/agent-codex/tests/manifest.rs`

In `packages/agent-codex/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, with the new test among those that pass;
- step 4's run against the changed line fails, and passes once the line is restored;
- `cargo run --release --bin package`. Report the digest it prints; it is a host build's.

Step 6's registration of P1 is accepted, each reply with no diagnostic. P2's is not accepted, and
its diagnostics cite MAN-017 and name `wrapper-v2-path`. If P2 is refused for another reason,
report the reply whole.

Report:

- in `decisions`, each of the six decisions, as applied;
- under `findings`, step 3's answer, and anything else that needs changing;
- under `own_choices`, how the test splits the map, the README's wording where this order left it
  to you, and every other choice you made yourself;
- the totals passed and failed, and each check's last line.

Do not stage anything.
