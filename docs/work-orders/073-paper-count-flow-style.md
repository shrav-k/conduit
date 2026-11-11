# Work order 073 (PB): a paper count of S1 in flow style, after the bar's rule was fixed

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; its `vectors` list stays empty. This order has no network access,
and needs none: it is paper, with no code, no build and no engine run. Never print a whole large
file or fixture: use `head`, `tail`, `grep` or a count.

## What and why

On 2026-11-11 ruling 71, point 3, fixed the bar's rule for each task
([IN_FORCE.md, topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement), the
bullet on the author measurements). In each run, Conduit must take fewer lines than the baseline
on most of S1's tasks, and on no task more than one and a half times the baseline's count. This
order counts, on paper, what S1's tasks T1 to T7 cost an author in the short form as `spec/` now
states it (order 037):

- written in the documented flow style (ruling 63, point 2), with block style beside;
- counted by ruling 63, point 1, and ruling 68 R1;
- with the commands in the form of the CLI reference that order 069 (coordinator task K3) wrote;
- compared with the baseline A70.

Experiment 011 counted 55 flow lines against the baseline's 64, but before order 037. It used
local templates, and `.v1` Agent Types with no `account:`. It counted pins and bare
`conduit apply` commands as lines, and it counted T7's removal and restoration twice.

**Authority.** The coordinator decided this count under the plan that the owner accepted with
ruling 71. That plan runs the rerun's lane in this order: K3's reference, then the bar's rule
(ruling 71, point 3), then this count. Experiment 011 ran the same way, under ruling 55 and order
031. The register lists X2, "an author's effort, counted on paper", as dropped by ruling 45
(`ROADMAP.md:83`). That drop does not bar this count:

- X2 was to be a measure of an author's effort, and the first build now takes that measure on
  every run (A68);
- this count is design input to the flow style and the CLI, as 011's paper totals were. It is
  never the bar's measure.

The count informs only the design of the flow style and of the CLI (K3's reference, and orders
052a and 052b). It never informs the bar. It is not an author run under ruling 45, and its
forecast of ruling 71's rule is not a result. It runs after ruling 71 fixed the bar's rule, and
after order 069 merged. Its results say so, and the admin's Record will too. It is a follow-up
to experiment 011 and serves A79's row of the register. Its directory is
`experiments/023-paper-count/`. It is sized S: about two hours of paper.

Line numbers below are at `e94c057`.

Read:

- **The tasks**, [the first build](../implementation/FIRST_BUILD.md#the-tasks), and its
  [measures](../implementation/FIRST_BUILD.md#measures), above all "Inspection effort".
- **The baseline**, `experiments/007-s1-baseline/results-2026-11-09.md`:
  - lines written per task at line 19: 33, 16, 0, 8, 3, 2, 2;
  - how it counted, lines 117-147, above all lines 122-124;
  - each task's section, from line 149;
  - the audit's corrections, from line 597.
- **Experiment 011**: its counting, `experiments/011-short-form-spike/README.md` lines 21-44; its
  `states/` and `commands/`; its counts, `results-2026-11-10.md` lines 86-134.
- **The rules**:
  - [ruling 55](../proposals/ruling-55/README.md), A2 to A5;
  - [ruling 63](../proposals/ruling-63/README.md), point 1;
  - [ruling 68](../proposals/ruling-68/README.md), point 3, R1;
  - [ruling 71](../proposals/ruling-71/README.md), points 3 and 4;
  - IN_FORCE.md, topic 13: the bullet on the author measurements, and the bullets under "Each
    run measures what an author must do";
  - the measurement plan's section "The S1 rerun",
    `docs/proposals/first-build/increment-4/measurement-plan.md` lines 69-82, and what it does
    not count, lines 38-40;
  - R1's question, [spec batch 1's plan](../proposals/spec-batch-1/README.md) lines 122-129.
- **The CLI**:
  - K3's reference, `docs/implementation/plan-and-apply/README.md`, and its golden cases in
    `docs/implementation/plan-and-apply/cases/` (order 069). It covers `conduit plan` and
    `conduit apply`, and marks some points "for the admin";
  - today's commands, to read only: `engine/crates/shell/src/help.rs` (lines 1-64 and 84) and
    `engine/crates/shell/src/main.rs:123`;
  - `spec/README.md` lines 115-118: `conduit init` writes a flow-style skeleton, and
    `conduit install` writes the pins.
- **The short form**:
  - SCT-001 to SCT-016 in `spec/promises/sections.yaml` (SCT-010 is reserved);
  - `spec/schemas/sectioned.yaml`;
  - the example `spec/schemas/examples/valid/sectioned-project/`, which is S1 at T2;
  - `spec/README.md`, its section "What the short form's meaning covers" (lines 336-364): plan
    and apply, the counted flags and stable-name replacement.
- **The promises the states rest on**:
  - packages: PKG-028, PKG-031, NAM-006, NAM-014, NAM-024;
  - defaults: SCH-009 for configuration, TPL-004 for implementations;
  - the short form's owners and root's Conduits: SCT-006 (which applies OBJ-013), SCT-009,
    SCT-011, FLEET-005, ACC-033;
  - sign-in: PND-010.
- **The fixtures**, under `conformance/fixtures/`:
  - `s1/packages/filesystem-0.1.1/` and `s1/packages/filesystem-0.2.1/`: the npm server 2.1.0
    and 2.2.0, each exporting the templates `read` and `read-write`;
  - `s1/packages/agents-0.2.0/`: `claude-code.v2`, `codex.v2` and the template `turn`;
  - `s1/packages/history/`: `file-history.v1`. Its manifest pins `example.org/s1/filesystem`
    0.1.0 (`s1/packages/history/conduit.yaml:5-8`), the package in `s1/packages/filesystem/`;
  - the archives:
    - `install-archives/filesystem-0.1.1.tar.gz`, `install-archives/filesystem-0.2.1.tar.gz`
      and `install-archives/agents-0.2.0.tar.gz`, with their digests in
      `install-archives/digests.json`;
    - `s1/archives/history.tar.gz`, and `s1/archives/filesystem.tar.gz` (filesystem 0.1.0,
      history's dependency), with their digests in `s1/README.md` lines 22-23.
- **The Agent packages that order 050 is to ship**: the plan's order 050 (lines 620-624) and
  `docs/proposals/spec-batch-1/areas/refusals-and-identity.md:171`, which places them in
  `packages/agent-claude-code` and `packages/agent-codex`. Their shipped `.v1` Types state an
  implementation default for `turn.v1`: `packages/agent-claude-code/contracts.yaml:23-24` and
  `packages/agent-codex/contracts.yaml:23-24`.

## What to do

1. **Check your tree, then pre-register.**
   - If `docs/implementation/plan-and-apply/README.md` is not in your tree, stop and report it.
     This order runs after order 069 has merged.
   - `experiments/023-paper-count/README.md` holds a placeholder, committed with this order, and
     [experiments/README.md](../../experiments/README.md) holds its row. If either is missing, or
     another directory under `experiments/` has a name beginning `023-`, stop and report it: the
     admin assigns the number.
   - Before you write any state, replace the placeholder README. It opens "Not binding", as
     experiment 011's does, and names A79 and this order. It states:
     - what this count must show;
     - the counting of step 5, marked as fixed by this order before execution;
     - the limits;
     - how to reproduce each count with the commands of step 5.

2. **The states.** Write S1's author files after T1, T2, T4, T5, T6, T7's revocation and T7, in
   `states/flow/<stage>/` and `states/block/<stage>/`. The stages are `t1`, `t2`, `t4`, `t5`,
   `t6`, `t7-revoked` and `t7`. Each holds the manifest `conduit.yaml` and the sectioned
   document `s1.yaml`, as in experiment 011. T3 changes no file.
   - **What each state means.** S1's intended access, as in the baseline:
     - T1: agent A (`claude-code.v2`) reads the directory; agent B (`codex.v2`) reads and
       writes it;
     - T2: both read and post to the message history;
     - T4: A reads and writes the directory, and B only reads it;
     - T5: the project's filesystem package moves from 0.1.1 to 0.2.1, whose server is npm 2.2.0
       rather than 2.1.0; the project's pin of the history package does not change;
     - T6: the directory's root moves from `/lab/shared` to `/lab/shared-2`;
     - T7: A's Conduit to the directory is removed, then restored. `t7` equals `t6` byte for
       byte.
   - **The packages.**
     - T1 installs filesystem 0.1.1 and agents 0.2.0.
     - T2 installs history 0.1.0. History's manifest pins `example.org/s1/filesystem` 0.1.0, with
       digest `sha256:200de7c1…` (the archive `s1/archives/filesystem.tar.gz`). Under PKG-028 a
       package is not installed unless each package its manifest lists is already installed,
       each "by a request of its own". So T2 first installs `s1/archives/filesystem.tar.gz`, as
       its own command, and counts it. Experiment 011 never met this, because its project pinned
       filesystem 0.1.0 itself.
     - The project's manifest keeps one pin for `example.org/s1/filesystem`, at 0.1.1: NAM-006
       lets a project use an address at exactly one version, and NAM-014 resolves history's
       reference through history's own manifest. Write no project pin for 0.1.0.
     - Whether `conduit install` would write a project pin for a dependency, or refuse to, is
       open. Report it as a finding, with PKG-028, NAM-006 and NAM-014.
     - T5 installs filesystem 0.2.1. Version 0.1.0 stays installed for history.
   - **The Agent Types.** The fixture's `claude-code.v2` and `codex.v2` offer
     `#conversation.v1`, whose operation is `turn.v1`, but state no implementation and no
     default for it (`s1/packages/agents-0.2.0/contracts.json`), and the engine does not
     implement it. Under TPL-004, a template of either Type is therefore rejected, and so is
     every Agent of S1 in the fixture's terms. Do not drop any state for this, and do not add an
     `implementation:` line to any Agent. Write the states as if against order 050's `.v2` Types,
     which are taken to state an implementation default for `turn.v1`, as the shipped `.v1` Types
     do. Keep the fixture's addresses and pins. Record this as a limit, and as a finding with
     TPL-004.
   - **Only what `spec/` and the fixtures give.** Never invent syntax. Use:
     - the filesystem package's `read` and `read-write` templates (PKG-031);
     - `fleet:` for the Agents' owner (SCT-011), and a Pool that owns the Resources (SCT-006,
       which applies OBJ-013);
     - the Conduits root needs to run turns on the Agents, such as an `operator` made from the
       agents package's `turn` template (SCT-009, ACC-033).

     The example's `manages` keys (`s1.yaml:15`) and its `console` Interface (`s1.yaml:23`) are
     not part of S1. Leave them out unless a task needs them, and give the reason if you add
     one. Apart from the Agent Types above, where `spec/` cannot express a task, report a finding
     (gap or ambiguity) with its promises, and leave that state out.
   - **The fewest lines a competent author writes**, in the layout below.
     - Where a configuration default is what S1 needs, leave the value to it (SCH-009). The
       `.v2` Types' defaults for `accountServices`, `peerMessaging`, `model` and `apiKey` are
       the ones S1 relies on. The `.v2` Types carry them by ruling 55, A5, and NAM-024. The
       example writes these values out (`s1.yaml:11` and `18`); the states need not.
     - List each default relied on.
     - The Conduits' local names are `project`, which IN_FORCE.md, topic 13, keeps for comparison
       with the baseline (ruling 60), and `history`, as the example names it (`s1.yaml:14`).
   - **Values.**
     - `account:` values are placeholders at `example.org`, never a real account.
     - Paths are the literal values `/lab/shared`, `/lab/shared-2` and `/lab/history`.
     - Write each pin as `conduit install` would. Take each digest from
       `install-archives/digests.json`, or, for the two archives under `s1/archives/`, from
       `s1/README.md` lines 22-23, and check it with `shasum -a 256`.
     - No comments and no blank lines, so that every line of a state counts.
   - **Flow style** is the layout of `spec/schemas/examples/valid/sectioned-project/`:
     - each section key on its own line;
     - a Pool, Resource or Interface definition on one line;
     - an Agent's name on its own line, then each of its keys on its own line with a flow value,
       and each `consumes` item on its own line;
     - in the manifest, `apiVersion`, then `packages:`, then one flow-map pin per line.

     Where the example shows no layout for something you need, use the layout of the nearest
     thing it shows, and list the choice.
   - **Block style** is experiment 011's (its README, line 32): one map key per line and one list
     element per line. Each block state holds the same values as its flow state.
   - Write every state and ledger before you count any. After counting, change a state only to fix
     an error that a check of step 4 found, and list each fix.

3. **The ledgers.** None of their lines is executed: they describe a CLI that is not yet built,
   as experiment 011's ledgers do. For each task:
   - `commands/tN.txt` lists, one per line, the commands an author types;
   - `panel/tN.txt` lists the actions a person takes at the engine's panel, as ruling 68 R1 names
     them;
   - a ledger with nothing to list holds one comment line that says so. Comment lines are not
     counted.

   The commands' form:
   - **Plan and apply** take the form of K3's reference. Where it marks a point "for the admin",
     write its first alternative in the ledger, and count every alternative in step 5. T7 runs
     apply after its revocation and again after its restoration.
   - **The commands the reference does not cover** (`init`, `install` and a request sent alone)
     take today's forms in `help.rs`, without `--store`. List each as an assumption in the
     results:
     - `conduit init`;
     - `conduit install <archive path>#<SHA-256>` (`help.rs:25`), one per archive installed,
       history's dependency included;
     - a request sent alone as `conduit <operation> <name>`.

   Also:
   - **Panel:** one sign-in per Agent at T1. The `.v2` Types that order 050 ships state a sign-in
     (spec batch 1's plan, lines 123 and 621). The fixture's Types do not, and under PND-010 a
     sign-in to an Agent whose Type states none reports invalid. The results state that as a
     limit.
   - **Not in a ledger:** a check, that is a turn requested to show a task's state or a tool list
     (the measurement plan, lines 38-40).
   - **T3:** `commands/t3.txt` lists the commands, if any, needed to answer T3 from what is
     declared. The results list the files to read. Both are T3's inspection effort, a measure of
     its own in the first build's measures (the baseline's line 25). Step 5 says how they count.
   - **Routes:** where `spec/` allows T7, or another task, by requests alone (such as `close.v1`
     and `open.v1`, with no file edited), write that route's ledger as `commands/tN-requests.txt`
     and count it. The edit-then-apply route is the primary one, because it keeps the files a
     true record of the intended access, which T3 answers from.

4. **Check the states.**
   - **Schema.** Copy `spec/` and `tools/` to a scratch directory outside the repository, and
     call it `<scratch>` in everything you write. Copy each state directory to
     `<scratch>/spec/schemas/examples/valid/pb-<style>-<stage>/`, and run
     `python3 tools/check_schemas.py` in `<scratch>`. This checks `sectioned.v1` only, with the
     strict loader, which rejects a duplicate key (SCH-012). The translation waits for orders 043a
     and 043b.
   - **References, by hand.** In the results, a table of each Type, template and package address
     the states use, with the fixture file and the version that exports it at the state's pin.
   - **Styles.** Each block state parses to the same value as its flow state. Compare them with a
     PyYAML one-liner that you run and do not keep, and give it and its result in the results.
     PyYAML's `safe_load` keeps the last of two duplicate keys, so compare only states that passed
     the schema check.
   - **Against K3's cases.** For each task, compare each flow state with the files of K3's golden
     case for the same task, in `docs/implementation/plan-and-apply/cases/`. List each difference,
     with the lines it changes. Do not change K3's cases. Report a difference that looks like an
     error, in either, as a finding.

5. **Count.** These rules are fixed here, before you count.
   - **Lines in a file.** Compare each file before and after the task with
     `git diff --no-index --minimal -U0 OLD NEW`. It exits 1 when the files differ and 0 when they
     are equal; any other status is an error. At T1, OLD is an empty file. Each hunk costs the
     larger of its removed and added lines. That is experiment 011's rule, and the baseline's
     "each non-blank line added, changed or removed" (lines 122-124).
   - **Pins apart.** A hunk can mix pins with authored lines, so split each manifest before
     diffing. In `<scratch>`, make two copies of OLD and of NEW:
     - the authored copy, with the `packages:` line and every line of a pin deleted;
     - the pin copy, with only those lines.

     The authored copies give the file lines of the primary count. The pin copies give the lines
     `conduit install` writes. Keep the copies out of the repository.
   - **Once per task.** A line changed more than once in a task counts once. T7's restoration
     returns the lines its revocation changed, so T7 counts those lines once (ruling 63, point 1,
     third rule).
   - **The primary count** is the bar's measure (ruling 71, point 3; IN_FORCE.md, topic 13). It
     is:
     - the file lines of the authored copies;
     - plus each command that carries a value the author chose: a path, a name, a package, a
       digest, or a named flag;
     - with every line holding `account:` counted as authored (ruling 68 R1);
     - with the lines of root's Conduits counted, and also listed apart;
     - with the actions at the panel listed apart, and not counted.
   - **Readings of the rules.** Where a rule allows two readings, the primary count takes the one
     that counts more, and the results give the other beside it, with the tasks it changes.
     Settle none: order 053's measurement plan does. Two exceptions take the reading that counts
     less, on purpose:
     - **T3 counts 0.** What it takes to answer T3 is the measure "inspection effort", not lines
       (the first build's measures), and the baseline's T3 is 0 lines with its inspection listed
       apart. The other reading counts each command in `commands/t3.txt` that carries a value the
       author chose, such as an Agent's name (IN_FORCE.md, topic 13). At T3, any count of 1 or
       more is much more than the baseline's 0.
     - **The `packages:` key** is counted as written by `conduit install` with the first pin
       (`spec/README.md:117`; ruling 63, point 1). The other reading counts it as authored, one
       line at T1.
   - **The strict count** (ruling 63, point 1, fourth rule; ruling 68 R1) is the primary count,
     plus every other command, a bare `conduit init` or `conduit apply` included, plus each command
     in `commands/t3.txt`, plus every action at the panel.
   - **The block count** applies the primary count's rules to the block states.
   - **Listed apart, per task:**
     - the lines `conduit install` writes;
     - any digest the author copies by hand, such as one typed into `conduit install`: counted
       where its line or command counts, and listed apart (IN_FORCE.md, topic 13);
     - the lines of root's Conduits;
     - the actions at the panel;
     - T3's inspection effort.
   - **Readings the rules leave open.** At least these, each counted both ways:
     - each point that K3's reference marks "for the admin": every alternative it gives, with its
       command lines for T1 to T7;
     - **`--store`.** Today every command, `init` included, requires `--store DIR`
       (`main.rs:123`; `help.rs:5` and `84`), a value the author chose. Unless K3's reference
       says how the commands reach the engine, the primary count takes this reading: every
       command counts, a bare `conduit init` or `conduit apply` included. The other reading
       drops it;
     - **the install digest.** Today `install` takes `ARCHIVE#SHA256` (`help.rs:25`). With it,
       each install carries a digest copied by hand, listed apart; the command counts once
       either way, for its path. The other reading has install compute the digest;
     - root's Conduits' lines, counted or left out;
     - the lines `conduit init` writes, such as `apiVersion`. K3's reference does not cover
       `init`, so the primary count counts them as authored, and the other reading leaves them
       to `init`;
     - a command whose only chosen value is a flag that names no object, if the reference allows
       one.

     Add any other you meet.
   - **Assumptions about what later orders ship.** These are not readings of a rule: the states
     fix them, and the results give the other count beside.
     - One agents package, the fixture's `example.org/s1/agents`. Order 050 may ship two,
       `packages/agent-claude-code` and `packages/agent-codex`: that adds one install at T1, and
       one pin.
     - The `.v2` Types' implementation default for `turn.v1` (step 2).

6. **The results**, `experiments/023-paper-count/results-YYYY-MM-DD.md`, dated the day you run
   it. It opens "Not binding". Then it says, in these words or plainer, that:
   - it ran on that date, after ruling 71 fixed the bar's rule for each task on 2026-11-11, and
     after K3's reference merged;
   - it informs only the design of the flow style and the CLI, and never the bar;
   - it is a paper count by one author who knows the design, not an author run under ruling 45,
     and no engine, client or provider ran.

   Then these sections:
   - **The counts.** One table, a row per task and a total. Its columns: flow primary, flow
     strict, block, the baseline, one and a half times the baseline, and the lines listed apart.
   - **Where each line goes.** Per task, each counted line or command, with:
     - its file or ledger, and its text or key path;
     - its class: definition, grant, root's Conduit, configuration, owner or Fleet, package
       install, command or flag;
     - the promise or ruling it rests on.
   - **The forecast**, labelled a forecast on paper and not a result. It applies ruling 71,
     point 3, to the flow primary count:
     - per task: fewer, equal, more, or much more (more than one and a half times the baseline);
     - whether it is fewer on at least four of the seven tasks, and much more on none. This order
       reads "most" as more than half of the seven tasks; T3, at 0 against 0, cannot be fewer;
     - task by task, whether any of these changes the forecast: the strict count, the block
       count, the request route of step 3, each reading and each assumption of step 5;
     - that the bar needs both runs, the coordinator's and the fresh Codex author's, and this is
       neither.
   - **Against experiment 011.** Per task, 011's flow total and this count, and each cause of the
     difference with its lines: pins, package templates, the `.v2` Types and `account:`, root's
     Conduits, the layout, T7 counted once, commands with no chosen value, and history's
     dependency.
   - **Against K3's cases**, from step 4.
   - **For the design.** What the counts show about the flow style and the CLI, each point with
     the lines it touches: for instance, the lines that a flag's form, `--store`, a default or the
     layout decides. Give each style's longest line, and its number of lines over 100 characters.
     These are observations, not proposals. Until the S1 rerun, any change that could move S1's
     counts (CLI flags, short-form keys, templates and defaults) is the owner's (ruling 71,
     point 4).
   - **Findings** of the specification met while writing the states, each with its promises,
     the two of step 2 among them. Report them in your report's `findings` too.
   - **Limits**:
     - no engine ran, and the translation is checked against the schema only;
     - the fixture's `.v2` Types state no implementation for `turn.v1`, so under TPL-004 no Agent
       of S1 registers against them today, and they state no sign-in (PND-010). The states
       assume order 050's Types;
     - the CLI is prospective: K3's reference for plan and apply, and today's forms, without
       `--store`, for the rest;
     - a paper count by an author who knows the design is a floor.
   - **Checks**: each check's command, exit status and last line.

7. **The register.** ROADMAP.md's rules for an experiment are its section
   [Proposing an experiment](../../ROADMAP.md#proposing-an-experiment) (lines 86-103).
   - In `ROADMAP.md`, change A79's row (line 73) only:
     - add to its State, after the text there, the follow-up count, its date and a link to its
       results, described as design input and not the bar;
     - add "work order 073" to its Source.
   - In `experiments/README.md`, change only the third column of 023's row: the date it ran, on
     paper, and a link to its results, as 011's row has. Do not add it to "Evidence for v0".

## Scope

Change:

- `experiments/023-paper-count/`: replace its placeholder README, and add its results, `states/`,
  `commands/` and `panel/`;
- `ROADMAP.md`: A79's row only;
- `experiments/README.md`: 023's row only.

Do not change:

- `spec/`, `engine/`, `conformance/`, `packages/` or `tools/`;
- `docs/implementation/plan-and-apply/` (order 069's);
- experiments 007 and 011, or any other experiment;
- IN_FORCE.md, the ledger, any ruling's record, `docs/AUDIT.md`,
  `docs/implementation/FIRST_BUILD.md`, or the measurement plan (order 053 writes the rerun's);
- any other row of the register or of the index.

Write no code. Change no vector, so write no declaration. Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD`, which ends
  `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`
- `git diff --check`
- `python3 tools/check_private_paths.py`, if your tree has it

And these show what is stated:

- in `<scratch>`, `python3 tools/check_schemas.py` ends `Errors: 0`, and reports 14 more
  sectioned example projects schema-checked than it reports in your tree (2 at `e94c057`);
- each flow state and its block state compare equal;
- `cmp` finds `t7` equal to `t6`, file by file, in each style;
- `grep -rnE '^[[:space:]]*$' experiments/023-paper-count/states` prints nothing (no blank line);
- `grep -rnE '(^|[[:space:]])#' experiments/023-paper-count/states` prints nothing (no comment);
- `grep -rn -E '[[:space:]]+$' experiments/023-paper-count` prints nothing, since
  `git diff --check` does not see files that are not staged;
- `grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.[c]odex/' experiments/023-paper-count ROADMAP.md experiments/README.md`
  prints nothing. (The brackets keep this order itself free of the paths it looks for.)
- `git status --porcelain -- spec engine conformance packages tools docs IN_FORCE.md experiments/007-s1-baseline experiments/011-short-form-spike`
  prints nothing.

No Rust changes, so run no cargo command. In your report's summary, give the flow primary, flow
strict and block counts per task. Do not stage anything.

## Record

Run on 2026-11-11, after order 069 merged and after ruling 71 fixed the bar's rule: the worker in
11 minutes, then a fresh verifier and a second round; the final verifier found no fault. The
paper count is experiment 023. It informs only the design of the flow style and the CLI, never the
bar. In flow style, under ruling 63 and ruling 68's R1, with order 069's defaults, S1's tasks T1 to
T7 count 23, 6, 0, 3, 2, 2 and 3 lines (39 in all), against the baseline's 33, 16, 0, 8, 3, 2 and 2.
As a forecast only, that is fewer on four tasks and much more on none; T7 sits exactly at one and a
half times its baseline. The strict reading forecasts the same; block style forecasts a failure
(T7 much more). The results give the counts under each of the readings order 069 reserved for the
owner.
