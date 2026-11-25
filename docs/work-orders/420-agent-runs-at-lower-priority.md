# Work order 420: runs at a lower scheduling priority, set by the runtime provider where each run starts (DEC-046, DEC-182)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty, since no case changes. This order needs no network, and every crate it builds is
cached: run cargo offline, and if a crate is missing, stop and report, and fetch nothing. Run one
cargo command at a time, and not `tools/gate.py` or the whole engine suite. Never print a whole
large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential, even one a test made. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere: every implementation a test meets is scripted. Open,
list or sweep no store but one a test makes in a temporary directory: never the dogfood's. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute
local path, user, machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-182](../decisions/log.md#dec-182-2026-11-24-everything-stays-in-v0-more-accounts-and-machines-come-first):
"Everything in v0, prioritize adding new accounts and machines to improve performance and
parallelism".
[DEC-046](../decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement):
"to maximize agents for the fleet", then "Yes, grow by measurement (Recommended)", which lands as
a pair added at a time while "the machine and the gate have room".
[DEC-171](../decisions/log.md#dec-171-2026-11-23-the-coordinator-hands-off-through-written-state-its-kits-rust-replacements-follow-the-guard):
the kit -> "Order Rust replacements first"; the renice below is such a stopgap.

**Today.** On one machine five pairs of Agents build and test at once. On 2026-11-24 the merge
gate's engine tests ran past their 90-minute limit twice, at load averages of 40 to 100 on 12
cores; the coordinator reniced by hand every process in the Agents' working copies to nice 10,
each after it had started. Conduit gives no run a priority: neither the engine's own runner
(DL16) nor `provider-local-process` (`stage`, `start`, `guard_main`) sets one, and no promise or
contract states one. Each of the dogfood's Agents has a login Resource (DEC-046), whose
directory only the engine's own runner passes (PRV-029), so they name no `runtime`. Their items
are relative programs, which `provider-local-process` does not run (PRV-007; it has no
`run.v2`). So this order lowers nothing in the dogfood by itself; What follows names the steps
that move the Agents. The gate is not Conduit's: it is untouched.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4,
nothing assumes a particular vendor's agent: the setting is a provider's, for any instance; 6,
nothing starts model work or repeats an effect implicitly: a priority starts, stops and restarts
nothing, and a refused run is not retried; 7, everything is attributable: nothing changes in a
record or reply but a refused run's failed outcome and reason. Decisions 3 and 4 keep them.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice, which decision 7's observable cites: a priority is set where a process
starts and inherited by what it starts, never changed from outside. PRV-019 makes it Conduit's:
the runtime provider starts the run. **Where it is declared**, the coordinator's choice: in the
runtime provider's configuration, as its contract states. An instance names its provider as a
Pod names its `runtimeClassName` (PRV-006), and PRV-006 already puts a provider's limits in
"that provider's contract and configuration". **Its cost:** the dogfood's Agents reach it only
once PRV-029 lets a runtime provider pass a login Resource's directory, as DEC-202's Windows
pair needs anyway, since the engine's own runner runs nothing on another node (PRV-019). A
setting on that runner would reach them now, but it has no configuration per instance: it would
lower all it starts, each provider's server (PRV-002) and each login's run among them, and serve
only the engine's machine, so DEC-182's new machines would still need their providers' setting.
Root's own work and the engine itself are unchanged. Not taken: a Type's run field (its author
knows neither the machine nor what else runs there); a Fleet's field or an instance's argument
in `run.v1` (it changes the language and every provider); renicing from outside (it races each
new process); `nice -n`'s relative form, where `Nice=` is absolute; `CPUWeight=` and macOS's
background QoS, each one system's; a negative nice, which needs privilege.

**When it runs.** On `main` at `46c740a9` or later, now (DEC-180). Through Conduit: one Codex
worker turn, the blind Codex verifier and the Claude reader at once (it changes `spec/` and a
README, DEC-189), one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision
says what must hold; how is yours. The coordinator's, open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): the place, `nice`
and its form, the refusal, the guard taking it, 0.2.0, every quoted text and the test names. It
fits one turn; were it not to, 420b takes the engine test and the fixture's `nice`, and 420a the
rest, the engine test's pin included. The coordinator's renice sweep skips this order's working
copy while its turns and checks run, since its tests assert exact nice values.

**Shared files.** Order 416, if it merges first, changes the waits in
`engine/crates/shell/tests/local_process.rs`; the later of the two rebases and keeps both.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), a file outside Scope
that must change, or a provider call from the engine whose configuration lacks the `nice` its
provider Resource was given (give the engine test's failing line). Where this order's wording
contradicts a promise it does not change, the promise governs: apply it and list it under
`own_choices`, with every choice no decision governs.

## The decisions

Read first PRV-005, PRV-006, PRV-019, PRV-030, NAM-024 and MAN-017; in
`packages/provider-local-process`, `configuration` (`src/lib.rs`), `stage` and `start`
(`src/backend.rs`), `guard_main` (`src/guard.rs`), `contracts.yaml` and the README; the kit's
README on reasons (`Unavailable::because`); and `Fixture::new` in
`engine/crates/shell/tests/local_process.rs`. Wrap each insertion at its file's width.

1. **The setting** (PRV-006, NAM-024). `local-process.v1`'s `schema` gains one property,
   `nice: {type: integer, minimum: 0, maximum: 19}`: optional, with no `default`, as NAM-024
   requires of a later release under the same name, and bound by no item, so MAN-017 does not
   reach it. It is the run's nice value, absolute as `Nice=` is: 0 the usual priority, 19 the
   lowest. Without it a run starts as today. No other Type, item or variable changes.
2. **Read from the call.** The provider reads `nice` from the configuration that each `run.v1`
   call carries (PRV-005; `CallContext::config`) when it stages the run, so a change reaches the
   next run started. The seven startup fields stay as they are. A `nice` that is not an integer
   from 0 to 19, as the Type's schema reads it (so `10.0` is 10), fails staging with the reason
   `nice must be an integer from 0 to 19`, and runs nothing.
3. **Set where the run starts, inherited.** The run's guard takes the nice value before it
   records its group and launches the program (`setpriority(PRIO_PROCESS, 0, nice)`), so the
   guard, the program and every process the program starts have it from the program's start.
   Nothing changes a process's priority after it starts. The provider's own server keeps its
   priority, and so does each run without `nice`. Stops and `maxLifetime` end the group as today.
4. **What it cannot give, it refuses** (PRV-019 as changed). Where `nice` is below the
   provider's own nice value, which the guard inherits, whatever the user's privilege, or where
   the call that sets it fails, no program starts: the guard records no group (its marker keeps
   `Unstarted`), the run ends failed, and the answer's reason is `nice is below the provider's
   own nice value` or `nice could not be set`, the provider's own words, never the system's
   error text. How the guard tells the provider is yours. `inspect-run.v1` then reports
   `stopped`.
5. **0.2.0.** `Cargo.toml`'s version and `Cargo.lock`'s own line, `conduit.yaml`'s `version`,
   the README's version and the engine test's pin, so that a store holding 0.1.0 is not refused
   this archive at the same version (PKG-025).
6. **The provider's text.**
   - In `contracts.yaml`'s description, after "Configuration changes take effect at the
     provider's next start.": "Optional nice (0 to 19, no default) is read from each run.v1
     call instead: the run's guard takes it before launching the program, so the program and
     every descendant start at that nice value; the provider's own server keeps its priority.
     A nice below the provider's own runs nothing and fails the run, naming nice."
   - In the README, "Values are read once" becomes "Bound values are read once", and in
     `src/lib.rs` the doc comment of `configuration` begins "Bound configuration": both cover
     only the seven bound fields.
   - In the README, "version 0.1.0" becomes "version 0.2.0", and this section comes before
     `## Processes, credentials and retained state`. The D15 table's line counts stay as they
     are (DEC-177: such counts are to be computed, not stored).

     ```text
     ## Scheduling priority

     `nice`, optional, with no default, is a run's nice value, a whole number from 0 (the usual) to 19 (the lowest), absolute as systemd's `Nice=` is. The provider reads it from each `run.v1` call, binding no startup variable, so a change reaches the next run. The run's guard sets it on itself before it launches the program, so every process of the run has it from its start; nothing changes it afterwards. The provider's own server keeps its priority, as does a run without `nice`. A value below the provider's own fails the run, whatever the user's privilege, naming `nice`, and no program starts. On Linux under autogroup, a nice value weighs a run only against the other processes of its session.
     ```
7. **The spec.**
   - PRV-019's `then`, after "the other strings being its arguments.", adds: "Where its contract
     states that its configuration sets the scheduling priority of its runs, and the
     configuration sets one, it gives the run that priority where it starts the run, before the
     item's program runs, so that every process of the run has it from the program's start, and
     it changes no process's priority afterwards. Where that priority is above its own, or it
     cannot give it, it runs nothing and the run ends failed, its diagnostic naming the setting."
   - Its `observable`, after "finds bytes altered since.", adds: "A scheduling priority is set
     where a process is started, and not from outside once it runs: POSIX's `setpriority` sets a
     process's nice value, which every process it starts inherits, systemd sets a unit's with
     `Nice=` and `CPUWeight=`, and Kubernetes a container's share of the processor from its CPU
     request. It weighs the run's processes against those the system weighs them with while they
     compete for the processor, and reserves and limits nothing. A configuration that sets none
     leaves each run at the provider's own priority. The engine's own runner states none, so
     what it runs, each provider's own server among it under PRV-002, keeps the engine's
     priority, and an instance runs at a lower one only by naming such a provider in `runtime`,
     under PRV-006; an Agent of a login Resource that does so fails its setup under PRV-029."
   - In `spec/definitions.md`'s term `runtime provider`, "and whether it confines what it runs
     (ACC-027)." becomes "whether it confines what it runs (ACC-027), and whether it gives its
     runs a scheduling priority that its configuration sets (PRV-019)."
   - `spec/CHANGES.md`, first under `## [Unreleased]`, on one line: "- Changed: PRV-019 and the
     `runtime provider` term: a runtime provider whose configuration sets its runs' scheduling
     priority gives it where each run starts, never one above its own, and never changes it
     afterwards, or runs nothing and ends the run failed, naming the setting; the engine's own
     runner sets none (DEC-046, DEC-182)." Where order 400 has merged (the start check's
     `fragments` line), the item goes instead in
     `spec/changes/420-agent-runs-at-lower-priority.md`, in the same words, and `CHANGES.md` is
     unchanged.

   PRV-019's row in 012's account already reads "text changed, not rerun", and its standing
   stays `no case, no reason`.
8. **Tests**, written first; run each on the otherwise unchanged code and report its line. A
   test reads a nice value with `getpriority(PRIO_PROCESS, pid)`, for a PID that a fixture wrote
   or the test spawned, and reads no process's environment. `base` is the test process's own
   nice value; a test that needs `base` below a bound asserts it, with a message.
   - In `packages/provider-local-process/tests/runtime.rs` (a helper that sends a configuration
     in a call's `_meta` is yours):
     - `nice_from_the_call_reaches_every_process_of_the_run` (decisions 1 to 3): with `n` the
       lesser of `base` + 5 and 19, and `base` below 19, a `run.v1` whose configuration holds
       `nice: n` succeeds; the program and its child (`pids.json`) and their group's leader,
       the guard, have `n`; the provider's server keeps `base`. After `stop.v1`, the test
       removes `pids.json`, and a `run.v1` with no `nice` gives the program and its child
       `base`. Today the first run's have `base`.
     - `nice_that_cannot_be_given_runs_nothing_and_names_nice` (decisions 2 and 4): with
       `base` at most 15, the provider starts at `base` + 2, set before its exec by the test. A
       `run.v1` with `nice` `base` + 1 ends failed, its text holding `nice is below the
       provider's own nice value`; no `starts` file appears in the instance's state directory,
       and `inspect-run.v1` reports `stopped`. Then `nice` 20 and `nice` `"10"` each end failed
       with `nice must be an integer from 0 to 19`, and nothing starts. Today the first run
       starts at `base` + 2.
   - In `packages/provider-local-process/tests/package.rs`,
     `nice_is_an_optional_unbound_property_without_a_default` (decisions 1 and 5): the Type's
     `nice` is `{type: integer, minimum: 0, maximum: 19}`, not in `required`, with no `default`;
     no item's `environment` names `/nice`; the manifest's version is `0.2.0`. Today `nice` is
     absent.
   - In `engine/crates/shell/tests/local_process.rs`, `Fixture::new` also takes the provider's
     `nice`, none for the existing tests, and its pin names 0.2.0.
     `local_process_runs_an_agent_at_its_providers_nice` (decisions 2 and 3, through the
     engine): with `n` the lesser of `base` + 3 and 19, and `base` below 19, the provider's
     configuration holds `nice: n`; `a` is set up and takes one turn, as in
     `local_process_runs_an_agent_turn`; the scripted server's `pid` has `n`, and the test's
     own process, the engine's, keeps `base`. Today the fixture is not admitted: the 0.1.0 Type
     refuses `nice`, and the 0.2.0 pin matches no installed archive.
   - An existing test changes only where it pins 0.1.0 or calls `Fixture::new`. Report each.

## Start check

From the repository root. Any line that prints otherwise means stop, change nothing and report it.

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
p=packages/provider-local-process
git status --short | wc -l | tr -d ' '  # 0
git merge-base --is-ancestor 46c740a9 HEAD; echo "base $?"  # base 0
grep -c -e '^## DEC-046, ' -e '^## DEC-182, ' docs/decisions/log.md  # 2
k $p/conduit.yaml 'version: 0.1.0' 'version: 0.2.0'  # 1 0
k $p/Cargo.toml 'version = "0.1.0"'  # 1
k $p/Cargo.lock 'name = "provider-local-process" version = "0.1.0"'  # 1
k $p/contracts.yaml 'maxLifetime: {type: integer, minimum: 1, maximum: 2147483647, default: 3600}' 'nice'  # 1 0
k $p/src/guard.rs 'pub lifetime_ms: u64,' 'let mut child = command.spawn()?;' 'let ready = pipe.write_all(b"R");'  # 1 1 1
k $p/src/backend.rs 'lifetime_ms: 0,' 'if byte == *b"R" {' 'Err(std::io::Error::other("guard not ready"))'  # 1 1 1
k $p/src/lib.rs '/// Configuration is captured at provider startup'  # 1
grep -rlE 'setpriority|getpriority|PRIO_PROCESS' $p/src $p/tests engine/crates/shell/tests/local_process.rs | wc -l | tr -d ' '  # 0
k $p/tests/runtime.rs 'names::key("config"): {}' 'fn ready(root: &Path, id: &str) -> Vec<i32>'  # 1 1
k $p/README.md 'version 0.1.0' '`nice`' 'Values are read once;'  # 1 0 1
k engine/crates/shell/tests/local_process.rs '"version":"0.1.0"' 'fn new(builtin: bool) -> Self {' 'fs::write(report.join("pid"), std::process::id().to_string())'  # 1 1 1
k spec/promises/providers.yaml 'the other strings being its arguments.' 'finds bytes altered since.' 'scheduling priority'  # 1 1 0
k spec/definitions.md 'and whether it confines what it runs (ACC-027).' 'scheduling priority'  # 1 0
grep -c '^| PRV-019 | convention | must | 0 | 0 | outside | none | no case, no reason |$' engine/STANDING.md  # 1
sed -n '/^- id: PRV-019$/,/^- id: PRV-021$/p' experiments/012-model-checking/account.yaml | grep -c '^  standing: text changed, not rerun$'  # 1
grep -rhoE 'fn (nice_[a-z_]+|local_process_runs_an_agent_at_[a-z_]+)\(' $p/tests engine/crates/shell/tests | wc -l | tr -d ' '  # 0
command -v cargo rustc | wc -l | tr -d ' '  # 2
test -f spec/changes/README.md; echo "fragments $?"  # fragments 1, or fragments 0 once order 400 has merged (decision 7)
```

## Scope

Change, as the decisions say: in `spec/`, PRV-019 in `promises/providers.yaml`, the term
`runtime provider` in `definitions.md`, and one item of `CHANGES.md` or, once order 400 has
merged, of the fragment `changes/420-agent-runs-at-lower-priority.md`; in
`packages/provider-local-process/`, `src/`, `tests/runtime.rs`, `tests/package.rs`,
`contracts.yaml`, `README.md` and the version lines of `Cargo.toml`, `Cargo.lock` and
`conduit.yaml`; in `engine/crates/shell/tests/local_process.rs`, the fixture, the pin and the
new test. Change an existing test only under decision 8. Change nothing else: not the engine's
sources, the kit, the Agent packages, `provider-ssh-runtime`, a case, `docs/` (DEC-046 and
DEC-182 keep their status), `experiments/`, `tools/` or `engine/STANDING.md`. Write outside the
tree only in temporary directories the tests make.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each new test by name, once, then the package's tests and the engine's
`local_process` tests once each, last.

```sh
export PYTHONDONTWRITEBYTECODE=1
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
p=packages/provider-local-process
ok='^test result: ok\. 1 passed'
for t in nice_from_the_call_reaches_every_process_of_the_run nice_that_cannot_be_given_runs_nothing_and_names_nice; do (cd $p && cargo test --offline --release --test runtime -- --exact "$t" 2>&1 | grep -c "$ok"); done  # 1 twice; 0 today
(cd $p && cargo test --offline --release --test package -- --exact nice_is_an_optional_unbound_property_without_a_default 2>&1 | grep -c "$ok")  # 1; 0 today
(cd engine && cargo test --offline --release -p conduit-shell --test local_process -- --exact local_process_runs_an_agent_at_its_providers_nice 2>&1 | grep -c "$ok")  # 1; 0 today
k $p/conduit.yaml 'version: 0.1.0' 'version: 0.2.0'  # 0 1; 1 0 today
k $p/Cargo.toml 'version = "0.2.0"'  # 1; 0 today
k $p/Cargo.lock 'name = "provider-local-process" version = "0.2.0"'  # 1; 0 today
k $p/contracts.yaml 'nice: {type: integer, minimum: 0, maximum: 19}' 'Optional nice (0 to 19, no default)'  # 1 1; 0 0 today
k $p/src/lib.rs '/// Bound configuration is captured'  # 1; 0 today
k $p/README.md 'version 0.1.0' 'version 0.2.0' '## Scheduling priority' 'Bound values are read once;'  # 0 1 1 1; 1 0 0 0 today
k engine/crates/shell/tests/local_process.rs '"version":"0.1.0"' '"version":"0.2.0"'  # 0 1; 1 0 today
k spec/promises/providers.yaml "it changes no process's priority afterwards" "The engine's own runner states none"  # 1 1; 0 0 today
k spec/definitions.md 'whether it gives its runs a scheduling priority that its configuration sets (PRV-019).'  # 1; 0 today
cat spec/CHANGES.md spec/changes/420-agent-runs-at-lower-priority.md 2>/dev/null | grep -c '^- Changed: PRV-019 and the `runtime provider` term:'  # 1; 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
p=packages/provider-local-process
(cd $p && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd $p && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "clippy $?"  # clippy 0
(cd engine && cargo fmt --all -- --check); echo "engine fmt $?"  # engine fmt 0
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings >/dev/null 2>&1); echo "engine clippy $?"  # engine clippy 0
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths check_schemas check_model_account; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(promises/providers\.yaml|definitions\.md|CHANGES\.md|changes/420-agent-runs-at-lower-priority\.md)|packages/provider-local-process/(Cargo\.(toml|lock)|conduit\.yaml|contracts\.yaml|README\.md|src/[a-z_]+\.rs|tests/(runtime|package)\.rs)|engine/crates/shell/tests/local_process\.rs)$' | wc -l | tr -d ' '  # 0
(cd $p && cargo test --offline --release 2>&1 | grep -E '^(test result|error)' | cut -d. -f1 | sort -u)  # test result: ok, the package's tests, once
(cd engine && cargo test --offline --release -p conduit-shell --test local_process 2>&1 | grep -c '^test result: ok\.')  # 1: the engine's local-process tests, once, last
```

`/usr/bin/python3` is the system's interpreter, which has PyYAML. Your report gives, per
decision, what changed and where; each new test's line on the unchanged code, then its passing
line; each existing test you changed, and why; your own choices; and every check's last line.
Stage nothing.

## What follows

- **The Agents onto it**, in three steps. PRV-029 changes: a runtime provider whose contract
  states that it passes a login Resource's directory does so, and the engine's `run_diagnostic`
  admits it (DEC-202's Windows pair needs the same). `provider-local-process` runs the Agent
  packages' programs by `run.v2` (PRV-007, PRV-019), with `platforms` in their items. Then the
  pairs are made again naming a local-process provider whose configuration sets `nice` (PRV-006:
  no operation changes `runtime`). The hand renice ends then, and not before.
- **The measure** (DEC-062): each gate run records the machine's load average, and the gate's
  engine time before and after the Agents move says whether nice is enough.
- **Linux nodes.** Where autogroup leaves nice too weak, a provider that sets `CPUWeight=`
  through systemd is that system's route.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Drafted and
checked by the coordinator's Workflow; built and read on the team seats (DEC-179, DEC-180). This
order answers the day's measured cap: with five pairs building at once the machine's load stood
at 40 to 100 on 12 cores, and the merge gate's engine tests twice ran past their limit, while the
coordinator reniced the Agents' processes by hand as a stopgap.

**Worker-4's turn**, on the Business seat, built the priority's declaration and the local-process
provider's application of it at spawn, with four tests that failed first.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, passed it with no fault. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, found it carrying out the decisions, with
three shoulds (the configuration section did not mention the priority; the refusal named no
remedy; the tests compared exact nice values that an outside renice could change) and three nits.
The coordinator's stopgap was stopped for this order's working copy during its fix turn. One fix
turn by worker-4 answered all six. The gate's run on the squash is the coordinator's, at the
merge.
