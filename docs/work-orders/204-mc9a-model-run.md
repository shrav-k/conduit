# Work order 204: MC9a's model built and run, pending actions and sign-in as registered

**This order runs on the second machine, the Linux node (WSL Ubuntu 24.04, x86_64, 24 CPUs).**
Nothing in the model needs Linux: the coordinator places it there so that the Mac keeps the
sign-in chain's engine builds, and its bounds below are that node's. It changes no engine code.

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since this order adds and changes no case. It has
no network access and needs none: the crate's toolchain and its one direct dependency must already
be in the node's caches (decision 1), and if they are not, stop and report it. Run one cargo
command at a time. Never print a whole large file or log: `experiments/012-model-checking/README.md`
has 7,638 lines, so read it with `grep -n`, `sed -n`, `head`, `tail` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Do not run `claude`, `codex` or any client, and sign in
nowhere.

## What and why

The full review of 2026-11-14 puts a sign-in model ahead of the sign-in build: order 24 of
[its orders](../proposals/full-review/orders.md), row 2 of
[FR-446](../proposals/full-review/findings/G1a-model-checking-register.md#fr-446) as its verifier
corrected it. [Order 201](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/201-mc9a-pending-actions-sign-in-registered.md) registered that model,
MC9a, in [experiment 012's README](../../experiments/012-model-checking/README.md), section
"MC9a — pending actions and sign-in as written: pre-registration": its texts and their commits,
two parts (sign-in, and actions), their scripts, bounds, configurations, eleven properties, 42
reach predicates, 23 mutants, the expected verdicts fixed before execution, and the findings
MC9a-001 to MC9a-003. Its Record says "MC9a remains unrun". This order writes the model, as
`src/pending.rs` in Rust on the existing harness like MC1 to MC7b (orders 104 to 115), runs it at
the registered configurations and bounds, and records each result beside the registered
prediction, never in its place.

Two orders built since take a stand that waits on this run:
[048a-3](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/048a3-build-sign-in-hold.md) builds MC9a-003's reading, that a successful `sign-in.v1`
lifts the hold whatever its action's state, and says "MC9a-003 stays reported, not settled, until
MC9a runs"; [048a-5b](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/048a5b-build-sign-in-account-first.md) follows MC9a-002's reading, that
another account found after the instance's retirement has been dispatched raises no action. This
run reports what the model shows for both (decision 11), and settles neither: the owner does.

**Who decided what.** Every decision below is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): that MC9a runs as registered, by the
registration's own rule, with its dated amendment of 2026-11-19 (decision 2); the module's names,
tests and output lines; the Linux node's bounds; the policy on differences, corrections and a
truncated run; the form of the README's record; and leaving the model-checking account, `G5.md`
and `ROADMAP.md` to the coordinator at the merge (decision 12). Apply them as written and report
them under `decisions` as the coordinator's.

**Its size, said in the open (FR-264).** This order is large for one Codex run and one
verifier: two protocols, 42 reach predicates, 23 mutants and 42 full runs, with their first
traces and the README's record. The coordinator expects `src/pending.rs` at about 2,000 to
3,000 lines, the size of MC7b's `records.rs` (2,081 lines), not the 400 to 600 lines the full
review's orders sized it at. The coordinator keeps it one order because the two parts share one
registration, one table of expected verdicts, one set of configurations and one dated record,
and that record must come from the same recorded runs of both parts; split by part, the joint
record would be written from runs at two bases. The verifier reads the two modules as two parts.
If the result proves too large to verify, the coordinator writes a finishing order, as MC6 and
MC7b had (orders 114 and 115).

**Read first:** `AGENTS.md`; this order; in `experiments/012-model-checking/README.md`, lines 1 to
60 (the harness and its rules), the MC9a section whole (from the line that begins `## MC9a` to
the end, about 660 lines, read in parts of at most 200), and MC7b's dated results (`grep -n '^###
MC7b dated results'`, then about 90 lines) for the form of a record; `src/harness.rs`;
`src/records.rs`, lines 1 to 80 and 1861 to 2081, for a module's form and its tests; and the
Records of orders [113](113-mc7b-records-and-packages.md#record) and
[160](160-mc2-rerun-077.md#record). You need not read `spec/`: the registration quotes every slice
it reads, and decision 2 checks the quotations against the present text.

## The decisions it applies

### Before any code

1. **The base and the node.** From the repository root of your worktree, run these before
   anything else. Each prints what its comment says. If one prints anything else, change nothing
   and report which: the coordinator dispatches this order only from a base on which each prints
   that.

   ```sh
   git status --porcelain | grep -c .                                       # 0
   git merge-base --is-ancestor 1e38ff92 HEAD; echo $?                      # 0
   git diff --quiet 75944df4 HEAD -- experiments/012-model-checking/src experiments/012-model-checking/Cargo.toml experiments/012-model-checking/Cargo.lock; echo $?   # 0
   grep -c '^## MC9a' experiments/012-model-checking/README.md              # 1
   grep -c '^### MC9a dated results' experiments/012-model-checking/README.md   # 0
   uname -m                                                                 # x86_64
   nproc                                                                    # 24
   ```

   `1e38ff92` is order 048a-2's merge; `75944df4` is order 201's, after which nothing in the
   experiment's `src/` or its Cargo files has changed, and its README only by the MC9a section's
   amendment of 2026-11-19 (decision 2), appended after its last line. This prints what its
   comment says:

   ```sh
   /usr/bin/python3 -I - <<'EOF'
   import hashlib, subprocess
   path = "experiments/012-model-checking/README.md"
   old = subprocess.run(["git", "show", f"75944df4:{path}"], capture_output=True, text=True,
                        check=True).stdout
   now = open(path, encoding="utf-8").read()
   print(now.startswith(old), hashlib.sha256(now[len(old):].encode()).hexdigest()[:16])
   EOF
   # prints: True 6a5f922dce80315f
   ```

   Then, from `experiments/012-model-checking/`:

   ```sh
   rustup toolchain list | grep -c '^1\.97\.1-x86_64-unknown-linux-gnu'    # 1
   rustup component list --toolchain 1.97.1 --installed | grep -c -e '^rustfmt' -e '^clippy'   # 2
   cargo fetch --locked --offline; echo $?                                  # 0
   ```

   Write `git rev-parse --short=8 HEAD` down as BASE; decision 11 records it.

2. **MC9a runs as registered, with its amendment of 2026-11-19.** The registration's rule is: "A
   promise whose text changes after these commits makes its row 'text changed, not rerun' until
   MC9a is registered again against the new text (D29)." It reads `spec/` at `f72c2bc3`, orders
   047, 047b and 047c as written at `f72c2bc3`, and order 048a-2 at `f414e438`, "as PND-010 and
   PND-005 will read once it merges". These texts it reads changed after those commits, each
   named here with the coordinator's judgement:

   - `1e38ff92`, order 048a-2's merge: PND-005's `observable` and PND-010's `then` in
     `spec/promises/pending.yaml`, by its decisions 1 and 2, and `sign-in.v1`'s description in
     `spec/core/operations.yaml`, by its decision 3. Those are the two promises the registration
     reads "as they will read once it merges"; check (a) below shows their text is `f72c2bc3`'s
     with those decisions applied word for word, so neither changed against the text MC9a
     registered. `spec/core/operations.yaml` is not among the files MC9a reads, and its new
     sentence says what decision 2's says. Order 048a-2's file gained only its Record (check (c)):
     the amendments accepted at its merge, R1 (the preflight's gate to root or an Interface on
     its own behalf through the Agent's owning Fleet), R3 (a `conflict` under LIF-037 for a stale
     `expectedId`) and R4 (no raise for a retired instance), are the engine's, not a promise's
     text. No script request carries an `expectedId`, and the model is no engine.
   - `6c2c274c`, order 047b's merge: its file gained only its Record, with a DL19 amendment about
     the engine's retained repeat outputs. No promise text.
   - `e4ceb4c8`, order 047c's merge: its decision 6 now resolves the instance by name at a
     redispatch as well, for a reused name, and its decision 10's case steps changed. No
     promise text, and MC9a's actions script never removes B or reuses its name.
   - `280a7fcc` and `3c761dd9`: order 047d written and built. MC9a's PND-007 row calls it "not
     yet written"; PND-007 is not modelled, and its text did not change.
   - `b1cc554e`, [ruling 84](../proposals/ruling-84/README.md): the full review's
     `experiments.md` gained a note in the last cell of row A22, the keychain unattended. MC9a
     reads nothing from that row: from that file it reads only the section "Experiment 012: the
     runs, each registered before it runs", which did not change. By the registration's rule the
     MC9a section was amended for it on 2026-11-19, before any code or run, in its last section,
     "MC9a amendment — 2026-11-19, before any run", which quotes the note and says that the
     registration otherwise stands as written. Check (b) shows the file is `f72c2bc3`'s with
     that note added and nothing else.
   - `553450aa`, order 206's merge: CTR-013's `then` in `spec/promises/contracts.yaml`, the
     sign-in login's schema in `spec/schemas/contracts.yaml`, three invalid examples, and the
     counts and prose of `spec/README.md` and `spec/schemas/README.md`: a redirect path holds
     neither `?` nor `#`, and a page holds no user information. MC9a reads none of these files,
     and the URL and IFC-049's checks are outside it, as its limits say.
   - No other file under `spec/` changed since `f72c2bc3`, nor anything else in the full
     review's `experiments.md` (check (b)), so no other text MC9a reads changed, among them
     those whose promises the registration uses (REV-014, REC-004, IMP-007, CON-024 and `scope`
     in `spec/definitions.md`); and every quotation in the registration and its amendment is
     still found in the present text (check (d)).

   So no promise MC9a reads has text other than the text it registered against, and MC9a runs as
   registered, with the dated amendment of 2026-11-19 and no other. The registration's
   descriptive words "written and not yet built" and "not yet written" stay as they are, since
   its tables are immutable; the results say what the base holds (decision 11). From the
   repository root, each prints what its comment says:

   ```sh
   # (a) PND-005, PND-010 and sign-in.v1's description are f72c2bc3's with 048a-2's decisions 1-3
   /usr/bin/python3 -I - <<'EOF'
   import re, subprocess
   def show(rev, path):
       return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True,
                             check=True).stdout
   fence = "`" * 3
   order = show("f414e438", "docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md")
   blocks = re.findall(fence + r"text\n(.*?)\n   " + fence, order, re.S)
   def dedent(b):
       return "\n".join(l[3:] if l.startswith("   ") else l for l in b.split("\n")) + "\n"
   pairs = {"spec/promises/pending.yaml": [(0, 1), (2, 3)], "spec/core/operations.yaml": [(4, 5)]}
   for path, reps in pairs.items():
       text = show("f72c2bc3", path)
       for old, new in reps:
           old, new = dedent(blocks[old]), dedent(blocks[new])
           assert text.count(old) == 1, path
           text = text.replace(old, new)
       print(path, "as registered" if text == show("HEAD", path) else "changed")
   EOF
   # prints: spec/promises/pending.yaml as registered
   #         spec/core/operations.yaml as registered
   # (b) every other file under spec/ unchanged since f72c2bc3, but order 206's, since 553450aa
   git diff --quiet f72c2bc3 HEAD -- spec ':!spec/promises/pending.yaml' ':!spec/core/operations.yaml' ':!spec/README.md' ':!spec/promises/contracts.yaml' ':!spec/schemas' && git diff --quiet 553450aa HEAD -- spec/README.md spec/promises/contracts.yaml spec/schemas; echo $?   # 0
   #     and experiments.md is f72c2bc3's with ruling 84's note in row A22 and nothing else
   /usr/bin/python3 -I - <<'EOF'
   import subprocess
   path = "docs/proposals/full-review/experiments.md"
   def show(rev):
       return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True, text=True,
                             check=True).stdout
   note = (". Ruled on 2026-11-18: [ruling 84](../ruling-84/README.md), point 1. No dedicated"
           " user is planned; by the coordinator's reading, A22 is narrowed: the dedicated user"
           " over SSH, and the case of where a Claude login lands, are withdrawn; the launchd"
           " session keeps its daemon case, with no graphical login, without the dedicated user;"
           " your own session's case stays; both are optional, and A22 gates no Agent's becoming"
           " a dogfood slot, nor batch 2")
   now = show("HEAD")
   same = now.count(note) == 1 and now.replace(note, "") == show("f72c2bc3")
   print(path, "as registered, with ruling 84's note" if same else "changed")
   EOF
   # prints: docs/proposals/full-review/experiments.md as registered, with ruling 84's note
   # (c) order 048a-2's file lost no line since f414e438
   git diff f414e438 HEAD -- docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md | grep -c '^-[^-]'   # 0
   # (d) each quotation in the registration and its amendment is found: 36, none missing
   /usr/bin/python3 -I - <<'EOF'
   import re
   paths = ["spec/promises/pending.yaml", "spec/conventions/operations.yaml",
            "spec/interface/implementation.yaml", "docs/proposals/full-review/experiments.md",
            "docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md"]
   norm = lambda s: " ".join(s.split())
   corpus = norm(" ".join(open(p, encoding="utf-8").read() for p in paths))
   text = open("experiments/012-model-checking/README.md", encoding="utf-8").read()
   quotes = re.findall("“([^”]*)”", text[text.index("## MC9a "):])
   print(len(quotes), len([q for q in quotes if norm(q) not in corpus]))
   EOF
   # prints: 36 0
   ```

   If any of them prints otherwise, a text MC9a reads has changed since the coordinator wrote this
   order: write no code, change nothing, and report the output. By the registration's rule MC9a
   must then be registered again first, as a dated amendment of its section before any code, and
   that is a separate order of the coordinator's.

### The model

3. **Where it lives.** One new file, `experiments/012-model-checking/src/pending.rs`, beginning
   `//! MC9a reference models: pending actions and sign-in as written.`, and one line
   `pub mod pending;` in `src/lib.rs`, between `pub mod operations;` and `pub mod placement;`.
   It imports only `crate::harness`, `stateright` and `std`: no engine code, and no other model.
   It holds two public modules, `sign_in` and `actions`, one for each registered part, and one
   `#[cfg(test)] mod tests`. Change no other file under `src/`, and not the harness: it already
   gives what the configurations need (`fault_allowed`, `gate_remaining_faults`,
   `fault_enabled`, `timer_enabled`).

4. **What it models is the registration, word for word.** Build each part as the README's MC9a
   section states it: "MC9a nodes, scripts and bounds" (the three nodes Q, E and H, what E saves
   and what is volatile, the ledger and what it records, each stage of each script, the clock's
   one expiry, the bounds), "MC9a configurations and fault windows", "MC9a property text and
   preconditions" (each property's predicate), "MC9a reach predicates" and "MC9a mutants". Each
   mutant changes one rule of the correct model. Safety properties and the protocol's reach
   predicates read only the ledger (`State::ledger`), recomputing what the text requires from the
   stimuli and earlier entries there; only `quiescent` reads queues and node data. Where the code
   needs a detail the section leaves open, take the reading that keeps every registered sentence
   true, and report it under `own_choices` with the sentence it reads. Never change a registered
   sentence, table, bound, property or mutant, and never write a rule the section does not state
   in order to avoid a refutation (decision 9).

5. **Names, as the tests and the record use them.** In each of `sign_in` and `actions`:
   - a protocol type, `SignIn` and `Actions`, with a field `variant: Variant`;
   - `pub enum Configuration` with `pub const ALL`, in this order: for `sign_in` `Base`, `LossQ`,
     `ReorderQ`, `CopyQ`, `Crash`, `Wait`; for `actions` the same without `Wait`;
   - `pub enum Variant` with `Correct` and the registered mutants, and `pub const BROKEN` listing
     the mutants in the registered table's order: for `sign_in` `NoConflict`, `Unavailable`,
     `CopyAsNew`, `ConflictForever`, `WrongAccountKept`, `UndoInitial`, `ReturnTwice`,
     `ReplayState`, `SavedReturn`, `LeakDiagnostic`, `ExpiryLifts`, `RetainHold`, `VolatileHold`,
     `RaiseTwice`; for `actions` `WrongScope`, `AgentSees`, `VolatileScope`, `SkipRaisedAt`,
     `AnswerTwice`, `AnswerExpired`, `VolatileAnswers`, `ForgetOnPurge`, `RecomputeDismissed`;
   - a node type that implements `Default` with every saved and volatile field empty, the
     part's empty node value;
   - `Variant::configuration(self) -> Configuration`, the configuration of its registered row,
     called only on the variants of `BROKEN` and panicking on `Correct`, which has no single
     registered row, and `Variant::target(self) -> &'static str`, its own property: `MC9a X10`
     for NoConflict, Unavailable, CopyAsNew and ConflictForever; `MC9a K11` for WrongAccountKept and UndoInitial;
     `MC9a R13` for ReturnTwice, ReplayState and SavedReturn; `MC9a V14` for LeakDiagnostic;
     `MC9a H05` for ExpiryLifts, RetainHold and VolatileHold; `MC9a P01` for RaiseTwice; `MC9a S02`
     for WrongScope, AgentSees and VolatileScope; `MC9a A09` for SkipRaisedAt; `MC9a A03` for
     AnswerTwice, AnswerExpired and VolatileAnswers; `MC9a D12` for ForgetOnPurge; `MC9a U06` for
     RecomputeDismissed (the column "One step, to a vector" of the registration's last table);
   - `pub const SAFETY`, in the registered safety order: for `sign_in` `MC9a X10`, `MC9a K11`,
     `MC9a R13`, `MC9a V14`, `MC9a H05`, `MC9a P01`, whose promises are `["PND-010"]`,
     `["PND-011"]`, `["PND-013"]`, `["PND-014"]`, `["PND-005"]` and `["PND-001", "PND-005"]`; for
     `actions` `MC9a S02`, `MC9a A09`, `MC9a A03`, `MC9a D12`, `MC9a U06`, whose promises are
     `["PND-002", "PND-008"]`, `["PND-009"]`, `["PND-003"]`, `["PND-012"]` and `["PND-006"]`;
   - `pub const REACH`, each reach predicate's name in source order, `MC9a ` followed by its
     registered name: 27 for `sign_in` (`MC9a setup-raises` to `MC9a undo-interrupted`) and 15 for
     `actions` (`MC9a setup-action` to `MC9a quiescent`). Each witness of `sign_in` names the
     promises `["PND-010", "PND-011", "PND-013", "PND-014", "PND-005", "PND-001"]`, and each of
     `actions` `["PND-002", "PND-008", "PND-009", "PND-003", "PND-012", "PND-006"]`;
   - `pub fn model(v: Variant, c: Configuration) -> Harness<…>`;
   - in `sign_in`, one method `fn refuses_overlap(&self) -> bool`, the only place that decides
     whether PND-010's conflict applies, returning `self.variant != Variant::NoConflict`; in
     `actions`, one method `fn tests_raised_at(&self) -> bool`, the only place that decides
     whether `answer.v1`'s raised-at test applies, returning
     `self.variant != Variant::SkipRaisedAt`. Decision 13's controls edit these two.

6. **Configurations and bounds**, as registered. Nodes, in this order and these names:
   `Q/requests`, `E/engine`, `H/instance`. Each `Config` has `name` `MC9a sign-in <Configuration>`
   or `MC9a actions <Configuration>`, `messages_per_link` 8, `unique_state_ceiling` 400,000,
   `timeout` 15 seconds, and `bounds`
   `"MC9a: three nodes Q, E and H; at most 8 messages per directed link; finite scripts and one-use local choices; see the README's MC9a pre-registration"`.
   Budgets are zero except: `LossQ`, `Loss` 1; `ReorderQ`, `Reorder` 1; `CopyQ`, `Duplication` 1;
   `Crash`, `Crash` 1 and `Recovery` 1; `Wait`, `Timer` 1. `gate_remaining_faults` is true, and
   `fault_allowed` admits loss and duplication only on Q to E and E to Q, reordering only on Q to
   E among a stage's bracketed requests (through `fault_enabled`), crash and recovery only of E,
   and the timer only of E, enabled only for DL18's end as the `Wait` row states it. The clock's
   expiries, the person's choices, a failed sign-out and a lapse are local actions, never harness
   faults. Each zero budget has this reason: reorder "reordering isolated in ReorderQ"; loss "loss
   isolated in LossQ"; duplication "duplication isolated in CopyQ"; cut "a cut and its repair are
   MC4's"; heal "no cut to heal"; crash "a crash of E isolated in Crash; a crash of H is not
   modelled"; recovery "no crash to recover from"; lost machine "a lost node is MC4's"; timer, in
   `sign_in`, "DL18's end isolated in Wait; these slices give no other timer", and in `actions`,
   "no timer in this part: DL18's end is sign-in's".

7. **The registered tables, in code.** A function in each module's tests,
   `registered(v, c) -> (Vec<&'static str>, Vec<&'static str>)`, gives the safety properties the
   registration expects refuted and the protocol witnesses it expects not reached, copied from
   "MC9a expected verdicts, fixed before execution": a Correct row expects nothing refuted and
   exactly its configuration's reach; a mutant's row expects its "Expected safety" and its
   configuration's reach with the listed changes. The eight mutants whose configuration is not
   Base (CopyAsNew, ConflictForever, ReplayState, SavedReturn, VolatileHold, VolatileScope,
   AnswerTwice, VolatileAnswers) run in Base too, expecting their "Expected in Base" column and
   Base's reach with the same changes; each of those changes names a witness Base's row lacks, so
   that is Base's row. Every enabled harness witness is expected reached in every row. That makes
   42 full runs: 25 of sign-in (6 Correct, 14 mutants, 5 more in Base) and 17 of actions (5, 9 and
   3).

8. **The tests**, in `pending.rs`'s `mod tests`, named exactly. Under `--nocapture
   --test-threads=1` libtest prints `test <name> ... ` with no newline before each test, so each
   test's first output lands on that line. Each test below therefore prints a header line first,
   so that every later line it prints starts a line: `MC9a table Sign-in` for `sign_in_table`,
   `MC9a table Actions` for `actions_table`, and `MC9a test <test name>` for each other test.
   The anchored greps and the record check of Done when rely on it.
   - `sign_in_table` and `actions_table`: after the header, every Correct configuration in
     `Configuration::ALL` order, then each mutant in `BROKEN` order in its configuration, then,
     for the eight, in Base. Each runs `check_all` and prints one line, then asserts:

     ```text
     MC9a row <Part>|<Variant>/<Configuration>|<unique states>|<depth>|<seconds, 3 decimals>|<safety>|<reach>
     ```

     with `<Part>` `Sign-in` or `Actions`, the variant and configuration by their `Debug` names,
     `<safety>` one `H` or `F` for each name of `SAFETY` in order, and `<reach>` one `+` (reached)
     or `-` for each name of `REACH` in order. The assertion compares the refuted and missing sets,
     each sorted, with `registered(v, c)` (or with the row's recorded difference, decision 9), and
     also asserts no harness witness missing; its message is
     `MC9a table mismatch: <Part> <Variant>/<Configuration>`.
   - `sign_in_first_refutations` and `actions_first_refutations`: for each mutant in `BROKEN`
     order, in its own configuration, print `MC9a trace <Part>|<Variant>/<Configuration>|<target>`,
     call `assert_refuted` for `v.target()` (one thread, breadth-first; it prints the trace), then
     replay the trace from the initial state, set every node's data to its node type's
     `Default` (decision 5) and every link to empty, assert that the target's condition is still
     false (the message "safety must read only the ledger"), and print
     `MC9a first <Part>|<Variant>/<Configuration>|<target>|<n> actions`. When a mutant's
     `RECORDED_DIFFERENCES` entry for its own part and configuration (decision 9) lacks its
     target, call nothing and print `MC9a first <Part>|<Variant>/<Configuration>|<target>|not refuted`
     instead; those lines count among the 23.
   - `difference_traces`: for each `RECORDED_DIFFERENCES` entry, sign-in's entries first, it
     builds the model of the entry's part, by the `<Part>` of its label, in that row's variant
     and configuration: for each property refuted against the table, print
     `MC9a trace <Part>|<Variant>/<Configuration>|<property>`, call `assert_refuted` (it prints
     the first trace) and print
     `MC9a difference trace <Part>|<Variant>/<Configuration>|<property>|<n> actions`; for each
     witness reached against the table, the same with `assert_reached`; for a witness missed
     against the table, call nothing and print
     `MC9a difference trace <Part>|<Variant>/<Configuration>|<witness>|not reached`, and for a
     property registered refuted and not refuted, the same line ending `|not refuted`; neither
     has a trace. With no entry, it prints only its header.
   - `finding_witnesses`: in the Correct sign-in model, for `MC9a restart-overlap` in `Crash`,
     `MC9a retire-during-sign-in` in `Base` and `MC9a expiry-then-signed-in` in `Base`, in that
     order, print `MC9a trace Sign-in|Correct/<Configuration>|<witness>`, call `assert_reached`
     (it prints the first witness trace), and print
     `MC9a witness Sign-in|Correct/<Configuration>|<witness>|<n> actions`. Where that witness is
     a recorded difference (decision 9) that the row did not reach, call nothing and print
     `MC9a witness Sign-in|Correct/<Configuration>|<witness>|not reached` instead.
   - `configuration_and_property_inventory`: for every configuration of both parts, the
     configuration validates, its ceiling is 400,000, its timeout 15 seconds, its links hold at
     most 8 messages, `gate_remaining_faults` is true, and the properties are exactly `SAFETY`
     and `REACH` (6 and 27 for sign-in, 5 and 15 for actions).

9. **Differences, corrections and a truncated run.** The recorded runs are decision 10's
   `plain` and `counts` runs, the counts run the last full run of the tables; the README is
   written from the plain and counts logs alone. The plain run's `test result` lines and each
   log's last line (`plain: exit ...` and `counts: exit ...`) are the only sources of section
   1's totals, and every other result comes from the counts log.
   - **A correction** is a change to the code, made before the recorded runs, where a full run
     made while building showed the code not doing what a registered sentence says. Make it, and
     list it in decision 11's corrections section: the date, the row, the verdicts observed, the
     sentence the code missed, and the change.
   - **A difference** is an observed row that differs from the registration with the code doing
     what the section says. Do not change the code for it. In the module's tests, a constant
     `RECORDED_DIFFERENCES: &[(&str, &[&str], &[&str])]` holds, for each, the row's label
     `<Part>|<Variant>/<Configuration>`, with `<Part>` `Sign-in` or `Actions` as in the output
     lines, the refuted properties and the missing witnesses observed. Each table test applies,
     and checks for a matching row, only its own part's entries: it asserts those for that row
     in place of the registered ones, prints `MC9a difference <Part>|<Variant>/<Configuration>`
     when it does, and fails if an entry of its own part matches no row. Class each difference
     as D33 asks: a refutation of a Correct row is a finding against the promises
     (contradiction, gap or ambiguity, numbered from MC9a-004) or an over-broad property; a mutant that refutes its target only beside or not at all, or a witness
     reached or missed against the table, is said as such, with its cause. Each gets its first
     single-thread traces from the test `difference_traces` (decision 8), and a missed witness
     or a target not refuted gets none.
   - **A truncated run** (its ceiling or its 15 seconds reached; the harness fails it) is this
     order's stop condition. Change no bound: a bound changes only by a dated revision registered
     before the next run, as the registration says. Keep the code as it is, write no results, and
     report the run, its configuration and the line the harness printed.

### Running it on the Linux node

10. **Threads, jobs and memory.** Full runs use two checker threads, as registered:
    `CONDUIT_MC_THREADS=2` (the harness caps the value at 2); first refutations and witnesses
    use one, as the harness fixes. Cargo builds with `CARGO_BUILD_JOBS=4`. The ordinary test run
    uses 4 test threads and the counts run 1; the harness serializes model checks behind one lock
    in either. Every test run is capped at 12 GiB of address space (`RLIMIT_AS`, below WSL's 15 GB
    on this node, per [the second-machine guide](../guides/second-machine.md)), and its peak
    resident memory must stay below 8 GiB (8,192 MiB). Run nothing else heavy on the node while
    it runs ([the load rule](../implementation/METHOD_AS_RUN.md#load)). Build before each capped
    run, uncapped, with `CARGO_BUILD_JOBS=4 cargo test --release --no-run`. Write this script to
    `experiments/012-model-checking/target/mc9a/run.py` (git ignores `target/`), and run every
    test through it, from `experiments/012-model-checking/`:

    ```python
    import os, resource, subprocess, sys, time
    mode, extra = sys.argv[1], sys.argv[2:]
    args = {"plain": ["--", "--test-threads=4"],
            "counts": ["--", "--nocapture", "--test-threads=1"],
            "control": extra + ["--", "--test-threads=1"]}[mode]
    cap = 12 << 30
    def limit():
        resource.setrlimit(resource.RLIMIT_AS, (cap, cap))
    env = dict(os.environ, CONDUIT_MC_THREADS="2", CARGO_BUILD_JOBS="4")
    log = "target/mc9a/" + "-".join([mode] + extra) + ".log"
    start = time.monotonic()
    with open(log, "w") as out:
        code = subprocess.run(["cargo", "test", "--release"] + args, stdout=out,
                              stderr=subprocess.STDOUT, env=env, preexec_fn=limit).returncode
    wall = time.monotonic() - start
    peak = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss // 1024
    summary = f"{mode}: exit {code}, {wall:.1f} s wall, peak {peak} MiB"
    print(summary)
    with open(log, "a") as out:
        out.write(summary + "\n")
    sys.exit(1 if code or peak >= 8192 else 0)
    ```

    `/usr/bin/python3 -I target/mc9a/run.py plain` is the ordinary run and
    `/usr/bin/python3 -I target/mc9a/run.py counts` the counts run; these two are the recorded
    runs, and each appends its summary line to its own log. If a capped run fails for
    memory (an allocation failure in its log) or prints a peak of 8,192 MiB or more, stop and
    report it, as for a truncated run.

### The record

11. **The README**, `experiments/012-model-checking/README.md`, gains these and nothing else:
    - On line 3, replace the last words `MC9a registered 2026-11-18 under work order 201, not run.`
      with `MC9a registered 2026-11-18 under work order 201 and run DATE under work order 204.`,
      DATE being `date -u +%F` on the day of the recorded runs.
    - After the file's last line, "stands as written.", the end of the amendment, one blank line
      and these six sections, in this order, each line of prose at most 100 characters and each
      link whole on one line. The registration above them does not change.
      1. `### MC9a dated results — DATE`. A first paragraph: "Run DATE under work order 204 on
         the Linux node (WSL Ubuntu 24.04, x86_64, 24 CPUs), from `main` `BASE`." Then: "At
         `BASE` the texts MC9a reads are those registered: PND-005 and PND-010 as order 048a-2's
         decisions 1 and 2 wrote them at `1e38ff92`, experiments.md with ruling 84's note under
         the amendment of 2026-11-19, order 206's files as at `553450aa`, the other files
         unchanged since `f72c2bc3`, and the 36 quotations found; orders 047b, 047c, 047d and
         048a-2 are built." Then the
         totals from the two runs: the `test result` lines' counts of the plain log, each run's
         wall seconds and peak MiB as its log's last line holds them, written `<w> s wall` and
         `peak <p> MiB`, and "two checker threads for a full
         run, one for a first refutation". Then **Sign-in.** and **Actions.**, each a table with
         the header `| Variant/configuration | Unique states | Depth | Seconds | Safety | Reach |`
         and the alignment row `| --- | ---: | ---: | ---: | --- | --- |`, one row for each
         `MC9a row` line of the counts log, in the log's order:
         `| <Variant>/<Configuration> | <unique> | <depth> | <seconds> | <safety> | <reach> |`,
         every value copied from that line. A sentence says that safety and reach strings follow
         the registered orders, `+` reached and `-` not, and that harness witnesses, all reached,
         are left out of the reach strings.
      2. `### MC9a differences from the registration`. Either the one line
         `None: every full run matched the registration.`, or a table with the header
         `| Row | Registered | Observed | Class | Cause |`, one row for each `MC9a difference`
         line of the log, its first cell `<Part> <Variant>/<Configuration>`. The table comes
         first, all its rows. Then, for each row in the table's order and each of its
         `MC9a difference trace` lines, a line
         `**<Part> <Variant>/<Configuration>, <property or witness>:**` followed by the trace
         printed before it in a `text` block, or by the words `not reached` or `not refuted`,
         with no trace.
      3. `### MC9a first single-thread refutations`. A table with the header
         `| Variant/configuration | Target | Actions |`, one row for each `MC9a first` line, in
         the log's order: `| <Variant>/<Configuration> | <target> | <n> |`, `<n>` the line's
         count or `not refuted`. A sentence says that each refuted target was replayed with
         every node and queue erased and stayed false, and that the traces are in the run's
         output and not copied here.
      4. `### MC9a findings, as run`. One paragraph each for MC9a-001, MC9a-002 and MC9a-003, in
         that order, beginning `**MC9a-00N, as run.**`, and one for each new finding of
         decision 9, which cites its `MC9a difference trace` lines and the traces of section 2.
         Each of MC9a-001 to MC9a-003 says whether its witness was reached, in one sentence
         copied from the `MC9a witness` line: the witness's name in backticks, a comma, a space,
         then
         `Correct/<Configuration>: first witness trace, <n> actions.` with `<n>` the log's
         count, or `Correct/<Configuration>: not reached.`; then what the trace shows in one or
         two sentences; then the first witness trace in a `text` block, where there is one; then
         "Reported, not settled." MC9a-002's also
         says whether any Correct row refuted X10 or P01; MC9a-003's says RetainHold's H05 verdict
         in Base, from the table, and that orders 048a-3 and 048a-5b build its reading as the
         coordinator's stand, open to the owner's strike.
      5. `### MC9a: what holds, and what is one step, as run`. The registered table's three
         columns, `| Property | One step, to a vector | Holds, with a fault |`, filled from the
         observed rows. Copy each registered cell word for word when the mutants it names
         behaved as registered in the observed rows. Otherwise list the observed mutants (a
         mutant that refuted its target in Base is "one step"; one that refuted it only with a
         fault on is "holds, with a fault" where every Correct row held) and end the cell with
         "(not as registered; see the differences)".
      6. `### MC9a corrections before the recorded runs`. Either `None.` or a table with the
         header `| Date | Row | Observed | Sentence missed | Change |`, one row for each
         correction of decision 9.

    No line of the record states a verdict, a count or a trace that the plain and counts logs do
    not hold:
    a registered expectation appears there only as such, in a cell headed "Registered".

12. **What this order does not do.** It changes no engine code, no `spec/`, no case, no harness,
    no other model, and not `Cargo.toml`, `Cargo.lock`, `account.yaml`, `G5.md`, `ROADMAP.md`,
    `IN_FORCE.md` or any order. The thirteen PND rows of `account.yaml` stay `left to MC9`: the
    coordinator grades them from this record when it merges, as order 201's decision 6 says, and
    names for each "one step" clause its vector or the order that writes it. Settle no finding.

13. **The controls.** The tables must fail when a registered rule is taken out of the Correct
    model. Before each control, copy `src/pending.rs` to `target/mc9a/pending.rs.orig`; after it,
    restore with `cp target/mc9a/pending.rs.orig src/pending.rs` (not `mv`, which keeps an older
    modification time) and check that `cmp target/mc9a/pending.rs.orig src/pending.rs` prints
    nothing.
    - Make `refuses_overlap` return `false`. Build, then run
      `/usr/bin/python3 -I target/mc9a/run.py control sign_in_table`: it exits 1, and
      `grep -c 'MC9a table mismatch: Sign-in Correct/Base' target/mc9a/control-sign_in_table.log`
      prints 1. Restore.
    - Make `tests_raised_at` return `false`. Build, then run
      `/usr/bin/python3 -I target/mc9a/run.py control actions_table`: it exits 1, and
      `grep -c 'MC9a table mismatch: Actions Correct/Base' target/mc9a/control-actions_table.log`
      prints 1. Restore.

    Run the controls before the recorded runs, so that their logs are the last written.

## Scope

Change only `experiments/012-model-checking/src/pending.rs` (new), the one line of
`experiments/012-model-checking/src/lib.rs` (decision 3) and
`experiments/012-model-checking/README.md` (decision 11). Report anything else that needs
changing.

## Done when

On the Linux node, from `experiments/012-model-checking/`, one at a time, each passes:

```sh
CARGO_BUILD_JOBS=4 cargo fmt --check
CARGO_BUILD_JOBS=4 cargo clippy --all-targets -- -D warnings
CARGO_BUILD_JOBS=4 cargo test --release --no-run
/usr/bin/python3 -I target/mc9a/run.py plain     # plain: exit 0, ... peak below 8192 MiB
/usr/bin/python3 -I target/mc9a/run.py counts    # counts: exit 0, ... peak below 8192 MiB
```

The plain and counts runs here are the recorded runs. Run them once, after the controls, then
write the README from their logs and do not rerun them. If a rerun is needed, rewrite the record
from the new logs.

Then each prints what its comment says:

```sh
grep -c '^test result: ok\. [0-9]* passed; 0 failed' target/mc9a/plain.log   # 2
grep -c '^MC9a row Sign-in|' target/mc9a/counts.log                          # 25
grep -c '^MC9a row Actions|' target/mc9a/counts.log                          # 17
grep -c '^MC9a first ' target/mc9a/counts.log                                # 23
grep -c '^MC9a witness Sign-in|' target/mc9a/counts.log                      # 3
grep -c 'safety must read only the ledger' target/mc9a/counts.log            # 0
grep -c '^pub mod pending;$' src/lib.rs                                      # 1
grep -E '^\s*(pub )?use ' src/pending.rs | grep -v -e 'crate::harness' -e 'stateright' -e 'std::' -e 'super::' | grep -c .   # 0
```

Decision 13's two controls each print as it says. Then, from the repository root, the record
matches the logs: this prints
`rows 42 42; firsts 23 23; witnesses 3 3; differences N N True; summaries 2 2`, N being the
number of differences (0 where there are none).

```sh
/usr/bin/python3 -I - <<'EOF'
import re
log = open("experiments/012-model-checking/target/mc9a/counts.log", encoding="utf-8").read()
text = open("experiments/012-model-checking/README.md", encoding="utf-8").read()
part = text[text.index("### MC9a dated results"):]
first = part[:part.index("### MC9a differences from the registration")]
rows = re.findall(r"^MC9a row (?:Sign-in|Actions)\|([^|]+)\|(\d+)\|(\d+)\|([0-9.]+)\|([HF]+)\|([+-]+)$", log, re.M)
firsts = [(a, b, c or d) for a, b, c, d in re.findall(
    r"^MC9a first (?:Sign-in|Actions)\|([^|]+)\|(MC9a [A-Z]\d\d)\|(?:(\d+) actions|(not refuted))$",
    log, re.M)]
wits = re.findall(r"^MC9a witness Sign-in\|([^|]+)\|(MC9a [a-z-]+)\|(\d+ actions|not reached)$", log, re.M)
diffs = re.findall(r"^MC9a difference (Sign-in|Actions)\|(\S+)$", log, re.M)
r = sum(part.count("| " + " | ".join(x) + " |\n") == 1 for x in rows)
f = sum(part.count(f"| {a} | {b} | {c} |\n") == 1 for a, b, c in firsts)
w = sum(part.count(f"`{b}`, {a}: " + ("not reached." if c == "not reached"
                   else f"first witness trace, {c}.")) == 1 for a, b, c in wits)
d = sum(part.count(f"| {a} {b} |") == 1 for a, b in diffs)
none = (len(diffs) == 0) == ("None: every full run matched the registration." in part)
sums = [re.fullmatch(m + r": exit 0, ([0-9.]+) s wall, peak (\d+) MiB", open(
    f"experiments/012-model-checking/target/mc9a/{m}.log", encoding="utf-8").read().splitlines()[-1])
    for m in ("plain", "counts")]
s = sum(bool(x and re.search(rf"(?<![0-9.]){re.escape(x[1])} s wall", first)
             and f"peak {x[2]} MiB" in first) for x in sums)
print(f"rows {len(rows)} {r}; firsts {len(firsts)} {f}; witnesses {len(wits)} {w}; "
      f"differences {len(diffs)} {d} {none}; summaries {len(sums)} {s}")
EOF
```

From the repository root, each passes, with the new file seen by the checks through an
intent-to-add that is undone after:

```sh
F=experiments/012-model-checking/README.md
git add -N experiments/012-model-checking/src/pending.rs
python3 tools/check_docs.py                      # last line: Errors: 0
python3 tools/check_roadmap.py                   # last line: Errors: 0
python3 tools/check_model_account.py             # last line: Errors: 0
python3 tools/check_private_paths.py             # last line: Errors: 0
git diff --check                                 # prints nothing
git reset -q experiments/012-model-checking/src/pending.rs
git status --short                               # the three files of Scope, and nothing else
git diff --numstat -- $F | awk '{print $2}'      # 1
git diff --numstat -- experiments/012-model-checking/src/lib.rs   # 1 0 and the path
sed -n 3p $F | grep -c '; MC9a registered 2026-11-18 under work order 201 and run 20[0-9-]* under work order 204\.$'   # 1
grep -c '^### MC9a' $F                           # 17
grep -c '^### MC9a dated results — 20[0-9-]*$' $F   # 1
git diff -U0 -- $F | grep '^+' | /usr/bin/python3 -c 'import sys; print(sum(chr(64) in l for l in sys.stdin))'   # 0
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking   # prints nothing
```

Do not stage anything. Report under `summary` the base, the date, the test totals, the wall times
and peaks, and for every full run its verdicts against the registration; under `findings` each
difference with its class and MC9a-001 to MC9a-003 as run; under `own_choices` every reading of
decision 4 and your state representation; and every check's last line. Report too, under
`summary`, the largest seconds and the largest unique-state count among the 42 rows, and flag
any row above two-thirds of either bound (10 seconds, 266,666 unique states), so the
coordinator can decide before the gate: the merge's local gate runs this crate's
`cargo test --release` again on the Mac with 4 test threads, where a run near the bound may be
truncated. The admin then has a fresh
Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2), and the merge waits on the local gate
([ruling 80](../proposals/ruling-80/README.md), point 10). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-19, from about 03:30 to 04:09 UTC, on the Linux node from `main` `7eed5a4d` (MC9a's
dated amendment of 2026-11-19), in two rounds: the worker (1263 s) and a fresh Codex verifier
(327 s), then the worker again (376 s) and a second fresh Codex verifier (400 s). Decision 9's stop
condition held in both rounds, so no recorded run was made and the README is unchanged; the order is
not done.

In round 1 the worker wrote `src/pending.rs` and the one line of `src/lib.rs`. Its first exploratory
row, Sign-in Correct/Base, reached the registered 400,000-state ceiling: the harness printed `run
truncated: Some("unique state ceiling")`, at 8.7 s wall and a peak of 1652 MiB. It stopped there,
kept the code and wrote nothing in the README. Its own choices: the scripts as ordered stages with
sent and replied masks; a wire request key apart from its invocation attempt; symbolic accounts and
one program per invocation in the home, where H chooses the page's completion, a sign-out failure
and a lapse; an initial and a final poll for the finite polling script; and ordered ledgers from
which the oracles rebuild their state. The first verifier found eleven musts and one should: an
exploratory test that always panicked; `cargo fmt --check` failing (171 hunks); 14 local imports
against the order's grep; the first poll, and with it DL18, possible before the person completed the
page; the overlap switch still blocking dispatch during an open retirement; RetainHold keeping
`SignInUndone=False` as well as the hold; a forgotten dismissal exposing an action past its expiry;
K11 accepting a failed ending with the sign-out result outstanding; R13 checking `already-returned`
only when a refusal was present; S02 and U06 accepting a list result whose pending list was omitted;
A03 not checking that an expiry dispatches nothing; and, the should, the restart-overlap witness
dropping a program when it took the return.

Round 2 changed only `src/pending.rs` and addressed the twelve. The actions control printed one
`MC9a table mismatch: Actions Correct/Base` line and exited 1 (460 states, 0.008 s). The sign-in
control, with overlap refusal off, truncated at its first row, Sign-in Correct/Base (9.8 s wall,
peak 2314 MiB), before its mismatch assertion. Both restores passed `cmp`. The worker stopped before
the plain and counts runs. The second verifier found nine musts: K11 crediting the sign-out
diagnostic to the newest attempt of the key, which under CopyAsNew is the duplicate; the first poll
enabled before Q sends the retry; `already-returned` while the first return is only buffered
(ReplayState); K11 accepting a success that answers the pending action with something other than
signed-in; H05 not requiring the held turn's diagnostics; A03's window opening only at `Expired`;
R13 and V14 accepting a PND-013 diagnostic on a rejected retry; `SignInUndone=False` not cleared by
a check that reports no account; and the report omitting round 1's exploratory truncation.

A first fresh read-only Opus reader then read the order (ruling 79, point 2) at the branch's head.
On the Mac it ran `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` (exit 0 each),
the order's module and import greps (1 and 0), the private-path grep over the experiment (nothing)
and the commits' identities (the work-order identity, no trailers); in a scratch clone reset to
`7eed5a4d` with `pending.rs` intent-to-add, `check_docs`, `check_roadmap`, `check_model_account` and
`check_private_paths` (Errors: 0 each) and `git diff --check` (nothing), and the record's greps (no
README change, 12 MC9a headings, no dated results). On the node it compared the node's `run.py` with
the order's (identical) and read the three control logs; in a scratch copy outside the repository,
through the order's `run.py`, it ran `sign_in_table` (truncated at Correct/Base, 9.9 s, peak
2113 MiB), the same with the ceiling at the harness's maximum of 1,000,000 and 120 s (still
truncated, 26.2 s, peak 4919 MiB), `actions_table` (17 rows as registered, largest 42918 states) and
the plain run (exit 101, 148.9 s, peak 2442 MiB; 66 passed, 3 failed: `sign_in_table`,
`sign_in_first_refutations` at UndoInitial/Base and `finding_witnesses` at retire-during-sign-in,
all truncations). It found six musts and two shoulds:

- R204-1: the sign-in part truncates in Correct/Base, so the plain and counts runs cannot pass, and
  decision 9's stop applies to the committed code; even the harness's maximum ceiling does not fit
  the space.
- R204-2: decision 11's record is absent, which is right under decision 9's stop.
- R204-3: the sign-in control prints no `MC9a table mismatch: Sign-in Correct/Base` line, since it
  truncates first.
- R204-4: the first poll is enabled once the person completes the page, before Q sends the retry,
  against stage 4's order and PND-010's "Each retry polls account-hook.v1".
- R204-5: E refuses a retry `already-returned` while the first return is only buffered, where
  R13(b)'s rule is that no return has been passed on.
- R204-6: H05 never requires the PND-005 diagnostic on a held turn, nor PND-011's where both holds
  apply; it only rejects a PND-005 diagnostic that names an account.
- R204-7: the second verifier's other findings stand in the code: `SignInUndone=False` cleared only
  on the expected account; the sign-out diagnostic credited through the key's newest attempt; a
  success accepted without a signed-in answer for the pending action; a PND-013 diagnostic accepted
  on a rejected retry. A03's window, it found, covers the step as written.
- R204-8: round 2 never ran the committed Correct sign-in model alone, and the report gives no
  Correct/Base size or its cause.

The fixer rebased the branch onto `origin/main` at `9e2a1410` (order 048a-5) with the work-order
identity, without conflict, and took each finding:

- R204-1: not applied, since no change within this order can make it pass: decision 9 forbids
  changing a bound and the order forbids changing the harness. The coordinator chooses between a
  dated revision of MC9a's section registered before the next run and a finishing order that removes
  branching the registration does not state. After the fixes below, `sign_in_table` alone, on the
  Mac, still truncated at Correct/Base (5.4 s, peak 2338 MiB).
- R204-2: not applied: as the reader's fix says, no result is written until R204-1 is resolved and
  the controls, plain and counts runs pass.
- R204-3: not applied: the sign-in control is rerun once R204-1 is resolved.
- R204-4: `Local::Poll(InitialPoll)` is enabled once E has passed the invocation's return on, in
  place of the program's choice. The poll can still run before H applies the return, so the
  registered empty observation stays.
- R204-5: E refuses `already-returned` only when a return has been passed on. The fixer's reading of
  a further accepted retry while a return is buffered, which only ReplayState's replayed copy can
  make: E passes the buffered return on first, in the same step, then refuses the retry
  `already-returned` with PND-013's diagnostic, so at most one return is passed on and each refusal
  follows a pass.
- R204-6: H05 scans the held turn's emissions up to its result: it requires a PND-005 diagnostic for
  that turn without an account, rejects one that carries an account, and where sign-out failure also
  holds the instance requires PND-011's. The turn's PND-011 diagnostic now reads `PND-011:
  SignInUndone=False`, naming its condition.
- R204-7: the model clears `SignInUndone=False` on any incorporated account-hook result of the
  expected account or none, in setup's check, sign-in's first check or either poll, as PND-011 says,
  and not on success alone; the oracles' history clears it on the same results and no longer on a
  successful ending, so WrongAccountKept, whose success on another account clears nothing, does not
  drift from the model. K11 credits the sign-out diagnostic to the attempt whose failed sign-out
  result precedes it in the ledger, and records the action pending when the expected account's
  result is incorporated, requiring a signed-in answer for it from that attempt. R13 rejects a
  PND-013 diagnostic unless the ledger's event before it is an `already-returned` refusal for the
  same key. A03 is unchanged, as the reader found.
- R204-8: not applied: it asks the finishing order's worker to report Correct/Base's count at a
  diagnostic ceiling and the choices that drive it, on the node.

Since the sign-in table truncates, no sign-in mutant row ran, and the changed predicates were not
tested against the registered table. The fixer then ran, from `experiments/012-model-checking/` on
the Mac: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` (exit 0 each), `cargo
test --release --no-run` (built), the order's module and import greps (1 and 0), and `cargo test
--release -- --test-threads=4` with `CONDUIT_MC_THREADS=2`, skipping the three truncating tests (66
passed, 0 failed, 59.8 s, peak 2319 MiB); and from the root `python3 tools/check_docs.py`, `python3
tools/check_roadmap.py`, `python3 tools/check_model_account.py` and `python3
tools/check_private_paths.py` (Errors: 0 each) and `git diff --check` (clean).

A second fresh read-only Opus reader then read the branch at the fixer's head. On the Mac it read
the fixes against the diff and ran `cargo fmt --check` and `cargo clippy --all-targets -- -D
warnings` (exit 0 each), the module and import greps (1 and 0), the private-path grep (nothing),
the commits' identities (the work-order identity, no trailers), and, in a scratch clone reset to
`9e2a1410` with `pending.rs` intent-to-add, the four repository checks (Errors: 0 each), `git diff
--check` (nothing) and the record's greps (no README change, 12 MC9a headings, no dated results).
It checked this Record against the run's log and the reports (timings, finding counts and control
figures agree). On the node, in a scratch copy outside the repository with sources identical to the
head, it ran `sign_in_table` (truncated at Correct/Base, 9.7 s, peak 2038 MiB) and the plain run
(exit 101, 139.9 s, peak 2294 MiB; 66 passed, 3 failed, the same three truncations), and a
scratch-only probe that caught each truncation and went on (568.6 s, peak 2259 MiB). The probe found
every completed mutant first refutation refuting its registered target, Correct refuting none of
K11, R13 and H05 within 400,000 states in its six configurations, and Correct/Base reaching
return-passed (27 actions), already-returned (33), signed-in (33) and held (9). Three more runs
truncate at the ceiling, at round 2's `pending.rs` as at the head: RetainHold/Base's H05 first
refutation (MC9a-003's row), UndoInitial/Base's K11 first refutation and Correct/Base's undo-failed
witness. It found the four applied fixes correct, no regression from them, and three musts:

- R204-1: still stands: sign-in truncates at Correct/Base, and beyond it the UndoInitial/Base K11
  and RetainHold/Base H05 first refutations and the Correct/Base undo-failed witness do not fit
  400,000 states either. A dated revision or a finishing order must make every sign-in full row
  fit the 400,000-state, 15 s bound, then rerun from the controls on, as decision 13 says.
- R204-2: still stands: decision 11's record is absent, which is right while R204-1 holds.
- R204-3: still stands: the sign-in control cannot print its mismatch line while Correct/Base
  truncates before its assertion.

The second fixer rebased the branch onto `origin/main` at `6ce1f97b` (order 048a-6 refreshed) with
the work-order identity, without conflict. It applied none of the three: no change within this order
can fix R204-1, since decision 9 forbids changing a bound and the order forbids changing the
harness, and R204-2 and R204-3 wait on it, as the reader's fixes say. From
`experiments/012-model-checking/` on the Mac it ran `cargo fmt --check`, `cargo clippy --all-targets
-- -D warnings` and `cargo test --release --no-run` (exit 0 each), the module and import greps (1
and 0), the private-path grep (nothing) and `sign_in_table` alone (truncated at Correct/Base,
5.4 s, peak 2339 MiB); and from the root `python3 tools/check_docs.py` and `python3
tools/check_private_paths.py` (Errors: 0 each) and `git diff --check` (clean). MC9a-002 and
MC9a-003 are not run and not settled. The next step, a dated revision of MC9a or a finishing order,
and the gate's run on any squash are the coordinator's.
