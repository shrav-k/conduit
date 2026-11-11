# Work order 069 (K3): the CLI reference for plan and apply

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This is a paper order: no code, no build and no engine run. It
changes no vector, so its `vectors` list stays empty, it has no declared file, and it does not run
`tools/vector_delta.py`. It has no network access, and needs none. Never print a whole large file
or fixture: use `head`, `tail`, `grep`, `sed -n` or a count. Never read or print a credential.

## What and why

This is coordinator task K3 of spec batch 1 ([the plan](../proposals/spec-batch-1/README.md),
line 412): "the CLI reference for plan and apply, with golden cases that do not bind", due before
order 052a. Ruling 53, point 4, accepted an explicit plan and apply made of today's requests.
Ruling 55, A4, made the command the approval, with named flags counted when used. Ruling 59, A3,
added the cascades. `spec/README.md:348-364` states all of this as limits, not as promises.
Nothing yet says what the commands print, record and send, which act needs which flag, or how a
plan reads what exists. This order writes that once, so that each later order builds from one
reference:

- orders 052a-1 and 052a-2, which build the plan's order 052a (plan and apply);
- order 052b (stable-name replacement).

**Where it goes.** The reference binds nothing:

- the specification alone binds, and `docs/implementation/` does not (AGENTS.md, "What holds");
- `spec/README.md:111-112` says that "the guide to a tool's commands belongs to that tool";
- the plan, line 412, says the golden cases "do not bind";
- the synthesis's decision 10 puts the plan rules "into the CLI's reference", and a normative plan
  comes only "if a second planner must agree"
  (`docs/proposals/language-shape/short-form/synthesis.md:394-397`).

So it goes in `docs/implementation/plan-and-apply/`, and `spec/` does not change. What ships with
the CLI to the rerun's fresh author (ruling 53, point 9) is order 053's decision (the plan, line
657), not this order's. `docs/implementation/plan-and-apply/README.md` holds a placeholder now,
and its row in `docs/README.md` is already in place.

**Size: M.** About 0.5k lines of reference and 9 cases written by hand to exact text. The
synthesis's safety cases, the cascade case and the counted flags that S1 does not use are a later
order's (K3b), once the owner has answered the points this order marks for the owner (step 3).
This order lists those safety cases in a table (step 6), and writes none of them as a case.

Read first:

- the plan: conflict C8 (lines 286-300), C13 (line 322), K3 (line 412), the fixture agents
  (line 320), orders 051a, 052a, 052b and 053 (lines 626-658), and "Gaps" (line 681); the
  additions area's interplay note, `docs/proposals/spec-batch-1/areas/additions.md:370`;
- the rulings:
  - [53](../proposals/ruling-53/README.md), points 3, 4, 5 and 9 (lines 19-25);
  - [55](../proposals/ruling-55/README.md), A3, A4 and A7 (lines 18-22);
  - [59](../proposals/ruling-59/README.md), A3 and A7 (lines 22 and 26);
  - [61](../proposals/ruling-61/README.md), B11 (line 27);
  - [63](../proposals/ruling-63/README.md), points 1 to 3 (lines 16-29);
  - [68](../proposals/ruling-68/README.md), point 3, R1 (lines 41-43);
  - [71](../proposals/ruling-71/README.md), points 3 and 4 (lines 34-44);
- `spec/README.md`: the conventions (line 62), the author's path (lines 104-118) and "What the
  short form's meaning covers" (lines 336-364); `spec/promises/sections.yaml`, its head comment and
  SCT-001 to SCT-016 (SCT-010 is reserved and unused: cite it nowhere);
- the promises the requests rest on:
  - lifecycle: LIF-009, LIF-013, LIF-017, LIF-020, LIF-021, LIF-024, LIF-026, LIF-035, LIF-037,
    LIF-040, LIF-041;
  - access: ACC-024, ACC-029, ACC-033, ACC-053, ACC-054, ACC-057;
  - operations and records: OPS-008 to OPS-014, OPS-034, OPS-036, REC-002;
  - set-up and sign-in: TPL-012, PND-005, PND-011;
  - the rest: CON-008, FLEET-002, FLEET-005, IFC-012, IFC-023, INS-020, NAM-009, NAM-010,
    OBJ-002, PKG-031, REV-007, SCT-006, SCT-009, SEC-003, SEC-004, SEC-008, SUB-010;
  - the operations in `spec/core/operations.yaml`, above all `replace.v1` (line 176),
    `unregister.v1` (line 229), `retire.v1` (line 421) and `register-replacement.v1` (line 520),
    and which Conduits list them in `spec/core/capabilities.yaml` and `spec/core/templates.yaml`;
  - `io.github.shrav-k.conduit/pendingAction` in `spec/interface/implementation.yaml:728-733`;
- the design behind it, which does not bind:
  - `docs/proposals/language-shape/short-form/synthesis.md`: F6 (lines 58-66), decisions 10 and
    11 (lines 394-403), "Golden plan safety cases" (lines 590-612), and the critic's points 11 to
    13 (lines 614-631);
  - `docs/proposals/simplification/README.md`, point 4 and the critic's gaps (lines 63-104);
- S1:
  - the tasks, `docs/implementation/FIRST_BUILD.md:130-136`;
  - experiment 011's command ledger, `experiments/011-short-form-spike/commands/t1.txt` to
    `t7.txt`, and its counts, `experiments/011-short-form-spike/results-2026-11-10.md:95-104`;
  - the flow-style example `spec/schemas/examples/valid/sectioned-project/`;
  - the fixtures `conformance/fixtures/s1/packages/`, with digests in
    `conformance/fixtures/install-archives/digests.json` and `conformance/fixtures/s1/README.md`;
- today's commands, to read only: `engine/crates/shell/src/help.rs`,
  `engine/crates/shell/src/main.rs:18-125` and `engine/README.md:707-725`.

## What to do

1. **Write the reference** in `docs/implementation/plan-and-apply/README.md`, in place of the
   placeholder. Under its title, its first line is "Not binding. The specification in spec/ alone
   binds." It covers `conduit plan` and `conduit apply`, and the printed form of the cascades. It
   does not cover `install`, `render`, `init`, `sign-in`, `answer`, `pending`, `list` or the
   `fleet` commands, beyond what a plan reads or prints of them. For each command, give:
   - its usage line, one example and a one-sentence description, in the form of `help.rs`'s
     table (`engine/crates/shell/src/help.rs:1-64`);
   - what it reads, what it prints, what it records, what it sends and in which order, and its
     exit statuses.

   Write it so that a builder can build from it and an author can read it. Cite the promise or
   ruling behind each rule, and cite only promise IDs that exist in `spec/`. Link no file under
   `docs/work-orders/`: an order's file leaves the tree once it merges.

2. **Write in what is already decided**, each with its source:
   - **Tooling made of existing requests.** Plan and apply send only operations the core package
     has (`spec/core/operations.yaml`), with framed documents on the wire (SCT-001, SCT-003). They
     add no operation, promise or engine behavior (ruling 53, point 4; `spec/README.md:348`;
     `spec/promises/sections.yaml:6`; SCT-015). Like today's author commands, both are requests of
     `conduit serve`, made as the root Interface (`engine/README.md:715-716`).
   - **The command is the approval.** `conduit apply` computes the plan, prints it, records it and
     sends it, with no prompt. Nothing runs automatically or later (ruling 55, A4;
     `spec/README.md:348-349`). `conduit plan` prints the same plan, sends only the reads
     `list.v1` and `inspect.v1`, and writes nothing (ruling 53, point 4: "an explicit plan and
     apply").
   - **Bare commands.** Neither command needs a value: the project directory defaults to the
     current one, so a bare `conduit apply` carries no value the author chose (ruling 63, point 1).
   - **The counted flags, and no others.** `--reopen`, `--recreate`, `--prune`, `--replace` and
     `--adopt` (ruling 55, A4), and the cascade flags `--with-resources` and `--with-agents`
     (ruling 59, A3; `spec/README.md:350-353`). Each use is printed in the plan and recorded with
     it. `--withhold` is named as batch 2's (ruling 61, B11) and not specified. Today's `--store`
     and `--url` (`main.rs:36` and `:57`) are not new; whether plan and apply take them is step 3's
     point (c). Add no other flag or option: until the rerun, a CLI flag that could move S1's
     counts is the owner's (ruling 71, point 4). Report any you think the commands need.
   - **Narrowing and widening.** Quote `spec/README.md:354-357` as it stands: "Narrowing never
     waits. A plan prints and counts every widening, including one caused by moving a package pin
     or changing a reusable template's operations (PKG-031)", and "the plan still checks the
     operations that actually resolve" (ruling 55, A4). Every narrowing step comes before every
     widening step. Then, after any step, the access as a whole lies within the access before the
     apply or within the access the file asks for (safety case 8). The source is the proposal that
     ruling 53, point 4, accepted: "It narrows before it widens"
     (`docs/proposals/simplification/README.md:63-66`). The same proposal made a replacement
     "before the old is retired"; ruling 55, A3, reversed that, so break before make holds. List
     this ordering under `own_choices` as your reading.
   - **What exists is read, not assumed.** The plan reads the objects, their inventories and the
     tombstones root may see, through `list.v1` and `inspect.v1` (OPS-036, LIF-013, LIF-040). It
     compares them with the translation (SCT-003, SCT-004) and with what this working copy last
     applied, and prints each difference. Every request it sends that accepts `expectedId` carries
     it, and every `open.v1` and `close.v1` does (LIF-037; the plan, line 645).
   - **Root acts on its own behalf.** Every request is root's own, in SEC-008's form. Only root
     opens or registers a Conduit that root consumes (ACC-054), and only root registers an Agent
     (ACC-057). Each request goes through the Conduit that ACC-033 requires, and is named as the
     tool root calls (IFC-012). Then:
     - Root's first-start Conduits (`management`, `access` and `system`) and its FLEET-005
       defaults are never closed by request (ACC-024).
     - A default goes only when its Pool or Fleet is retired (CON-008; FLEET-005's observable).
     - Root may close its other Conduits on its own behalf, such as `N--operator` (SCT-009) or a
       member of `conduits` whose consumer is root (ACC-024, as ruling 59, A3, amended it).
       Reopening one follows the tombstone rules below.

     Synthesis F6's last bullet, "Root-consumed Conduits are re-pointed only by retiring their
     target" (`synthesis.md:66`), predates ruling 59, A3: do not apply it.
   - **Stable names, break before make.** A replacement keeps the name (ruling 55, A3; SCT-004;
     `spec/README.md:359-361`; the plan, line 650; `additions.md:370`). It runs in this order:
     1. Close the old Conduit, or retire the old instance.
     2. Go on only once the old object is removed. A closed Conduit is removed at once. Another
        instance is removed, with no further request, once its shutdown is confirmed, which a
        succeeded `retire.v1` does (LIF-024, LIF-035; `retire.v1`'s description,
        `spec/core/operations.yaml:421`). A retired instance keeps its name until it is removed
        (`register-replacement.v1`'s description, line 520). OBJ-002 rejects a document whose kind
        and name match an object that exists. `unregister.v1` removes a retired instance whose
        shutdown is not confirmed (LIF-017).
     3. Lift, with no flag, each tombstone that this same apply made (LIF-041, ACC-053).
     4. Register the new object under the same name.
     5. Open it or set it up.

     More rules:
     - An inline template, such as `N--t` (SCT-006), cannot be registered again under its name
       (OBJ-002). It is changed with `replace.v1` and removed with `unregister.v1` (ACC-033,
       row 2). Instances made from it keep what they resolved (INS-020). Say what T5's moved pin
       and T6's new directory each need of `shared--t` and of `shared`.
     - A nested instance, that is a declared Conduit or a Pool entry's Resource, is replaced with
       `register-replacement.v1` (LIF-009). It gets a newly generated name and lifts no tombstone
       (its description, line 520). Say what the plan prints for one.
     - A tombstone made outside this apply is lifted only under `--reopen`, for a Conduit, or
       `--recreate`, for another instance (`additions.md:370`; synthesis F6).
     - The plan prints every bound that stops covering a replaced Resource (ruling 59, A7;
       ACC-029) and every Conduit a retirement closes (CON-008).
     - An owner change is printed as a replacement, never a move (FLEET-002).
   - **Cascades.** `--with-resources` and `--with-agents` are flags of plan and apply
     (`spec/README.md:350-353`), and a cascade is printed in the plan (ruling 59, A3). Each is one
     command, printed and counted under its one flag:
     1. register a one-time Conduit from root, from the core template `retire-pool` or
        `retire-fleet` (`spec/core/templates.yaml:239` and `:278`; the plan's C8, line 300);
     2. open it;
     3. call `retire-with-resources.v1` or `retire-with-agents.v1` (LIF-021);
     4. close the Conduit;
     5. root lifts its own tombstone (the plan's C13, line 322).

     Write the printed form of these steps once. Order 051a's commands (the plan, line 627) are
     not this order's.
   - **The plan is recorded.** Its text is kept beside the project's files. Its digest is part of
     the idempotency key of every request apply sends after printing the plan
     (`io.github.shrav-k.conduit/idempotencyKey`; IFC-023, OPS-008), so every record links to the
     plan (REC-002). This is the plan's default (line 647), which lets K3 decide otherwise. Keep
     it unless you find a fault, and report the fault.

3. **Settle what the rulings leave open.** Write each rule with its reason and source, and list it
   in your report under `own_choices`. There are two kinds of point.

   **For the owner (ruling 71, point 4), through the admin.** A choice that could move S1's
   author counts, primary or strict (ruling 63, point 1), is not yours to settle. That covers
   CLI flags and defaults. For each such point:
   - write the default reading given below, which for (a) to (c) is the reading in experiment
     011's command ledger (`commands/t1.txt` to `t7.txt`), the reading the counts the owner saw
     assumed;
   - mark it "for the owner" in the reference;
   - in your report, give each alternative with its command lines for T1 to T7, primary and
     strict. List the person's actions at the panel apart (ruling 68, R1).

   The owner's points:
   - (a) **What each counted flag permits, exactly.** Two cases matter most. The first is
     whether closing a Conduit the file no longer names needs `--prune`, as the ledger's T7 has
     it, or is a narrowing that never waits. The second is whether changing a Conduit's operations
     needs `--replace` when it narrows, as when it widens, as the ledger's T4 has it. Ruling 53,
     point 4, says "never pruning or reopening without a flag", and `spec/README.md:354` says
     "Narrowing never waits": set out how the two meet.
   - (b) **How a flag names what it permits.** The ledger writes `--replace shared`,
     `--replace agent-a--project`, `--prune agent-a--project` and `--reopen agent-a/project`.
     Today's commands write `KIND/NAME` (`help.rs:17-36`). Choose one form as the default. Say
     whether a flag may permit an act without naming its object; the default is that every flag
     names its object, as the ledger's do.
   - (c) **How the commands reach the engine with no value on the command line.** Today every
     command requires `--store DIR` (`main.rs:123`), which is a value the author chose. The
     ledger's commands carry none. Set out the options, with a recommendation.
   - (d) **What apply does when a set-up raises a sign-in** (PND-005). A set-up does not wait for
     the person: it raises the action and returns, naming it in `pendingAction` (TPL-012's
     observable, `spec/promises/templates.yaml:228-229`; PND-011; `implementation.yaml:728-733`).
     The default: apply goes on to its next step, prints each raised action as the person's
     action at the panel (ruling 68, R1), and answers none. The alternative: apply stops at that
     step and the author runs `conduit apply` again after each sign-in. The fixture agents 0.2.0
     state no `signIn` (the plan, line 320), so no case here raises one. Give T1's lines under
     each reading for the real Agent packages of order 050. Safety case 17's "stops as pending"
     is the set-up's action, not apply's.
   - (e) **`conduit plan` itself.** Ruling 53, point 4, accepted "an explicit plan and apply". No
     S1 task requires running `conduit plan`. List it for the owner with that note.

   **Yours to settle.** List each under `own_choices`. If you find that one could move an S1
   count, treat it as the owner's, as above:
   - whether the narrowing parts of a plan are sent when another part lacks its flag;
   - what apply does when a request does not succeed (refused, invalid, conflict, unavailable,
     failed or unknown). It stops, prints what ran and what did not, and never sends a request
     again on its own (`spec/README.md:62`, the seventh convention). Say what the next plan
     shows, and refuses, while an outcome is unknown (OPS-012 to OPS-014);
   - what the next plan offers when an old instance's `retire.v1` did not succeed, so that it
     stays retired under its name (LIF-024): retire it again (LIF-026), or remove it with
     `unregister.v1`, which asserts that the shutdown is settled (LIF-017). Say whether that
     needs a flag;
   - a value that a control changed (REV-007). It is shown as a difference and never reverted;
     no flag reverts it;
   - an object that the translation names and this working copy did not register: what `--adopt`
     takes over, and that nothing else takes it over;
   - what apply does when the file drops a Pool or Fleet that still owns members. The
     recommended default: the plan refuses unless `--with-resources` or `--with-agents` is given,
     and then prints the cascade's steps in the one form of step 2. No S1 task retires a Pool or
     Fleet;
   - what is kept beside the file, and where: the plan's text under its digest, and what each
     request returned, so that the next plan knows what this working copy applied. Paths are
     relative to the project directory. Nothing kept holds an absolute path, a store path, a
     credential, or a value at a place a secret reference governs. SEC-003 and SEC-004 say this of
     diagnostics and records only: cite them as the analogy they are. Say whether the author
     commits what is kept, and that whoever edits it and runs apply can widen access;
   - what a plan with nothing to do prints, and every exit status;
   - what a repeat of an identical plan does, under OPS-008, OPS-009 and OPS-034.

4. **The printed form.** Plain text, one line per request, in the order apply sends them. Each
   line names:
   - the step's number;
   - the tool root calls;
   - the object, by kind, name and expected ID;
   - what the step does: one of `adds`, `narrows`, `widens`, `replaces`, `removes` and
     `neither`. Define each. Say which a set-up takes, since a set-up opens the instance's
     declared Conduits (TPL-012);
   - the flag that permits it, if any;
   - the promise it rests on.

   An object that this apply registers has no ID when the plan is printed. Its line names it by
   kind and name and, in place of the ID, the step that registers it, written `(step N)`. A later
   line that names it does the same. Its request carries the ID that step's result returned.
   Say whether reads that apply sends between steps print as lines.

   After the steps come the bounds and Conduits a step affects (step 2), and each counted flag
   used, once. The text holds nothing that varies between two runs on the same inputs, such as a
   time or an absolute path, so that its digest repeats. Apply prints each request's result after
   the plan; those results are not part of the recorded text.

5. **The golden cases for S1**, under `docs/implementation/plan-and-apply/cases/`: one YAML file
   per case, named `NN-name.yaml`. Each holds these keys:
   - `id`, `shows`, and `sources` (rulings, promises and the plan's lines);
   - `area`: one of `plan`, `grants`, `replacement`, `cascade` and `stops`, so that the admin can
     divide 052a-1, 052a-2 and 052b by it (`cascade` is for K3b);
   - `command`;
   - `files`: the author's files after the change, in flow style (ruling 63, point 2), as literal
     blocks;
   - `applied`: what this working copy kept from its last apply, or null;
   - `engine`: what the plan reads, the installed package releases included, in one compact form
     that the README defines;
   - `printed`: the exact text;
   - `recorded`: the exact record of the plan, and its digest;
   - `requests`: every request apply sends after printing the plan, in order, each with these
     keys: `tool`; `arguments` (with `expectedId`); `idempotencyKey`; `flag`; and `result`, which
     holds its outcome and what the next step needs of it, such as the names and IDs a
     `register.v1` returns in `objects`. A case that stops before sending has an empty list;
   - `kept`: what this working copy keeps after the apply, in the README's form;
   - `exit`: the exit status.

   Write these nine cases, as one sequence:
   - T1, from an engine that has only its first-start objects;
   - T2;
   - T3, as a plan with nothing to do;
   - T4;
   - T5;
   - T6;
   - T7's revocation;
   - T7's restoration;
   - T7's restoration without the flag it needs, in area `stops`.

   Each case's `applied` is the `kept` of the case before it, and its `engine` is the state that
   case left. The case without its flag starts from the same state as T7's restoration. Take
   each command from step 3's default readings.

   The inputs:
   - Start from the sectioned example, trimmed to each task, with real pins:
     - filesystem 0.1.1, then 0.2.1 at T5, with its `read` and `read-write` templates (PKG-031;
       ruling 63, point 3);
     - agents 0.2.0, with `claude-code.v2` and `codex.v2`;
     - history 0.1.0;
     - the digests that `conformance/fixtures/install-archives/digests.json` and
       `conformance/fixtures/s1/README.md` record.
   - History 0.1.0 pins filesystem 0.1.0 (`sha256:200de7…`,
     `conformance/fixtures/s1/packages/history/conduit.yaml:5-8`), while the cases pin
     filesystem 0.1.1. Show both releases installed in `engine`. If the specification does not
     let one project use both, log a finding with the IDs.
   - No program produces this output yet, so write every expected value from the reference's
     rules.
   - Use IDs in the form the conformance vectors use, such as
     `00000000-0000-4000-8000-000000000118`.
   - Use placeholder paths and accounts, as the sectioned example does (`/lab/shared`,
     `agent-a@example.org`).
   - Use an illustrative pin, marked as one, only where a case needs a package release the
     fixtures do not have, as the sectioned example marks its own (`s1.yaml`, line 1).

   Compute each digest with a tool, and say which in your report. Quote the names that
   `spec/names.yaml` assigns as they stand, and say in the README that `tools/set_names.py` does
   not rewrite them under `docs/`.

6. **The safety cases, as a table.** In the README, list the synthesis's golden plan safety cases
   1 to 18 (`synthesis.md:590-612`), adapted to batch 1: no `y/N`, no `--yes` and no `--take`.
   For each, give the rule of the reference that decides it and the outcome it gives, or why it is
   left out:
   - 12 needs a second node, which enrollment (order 049a) brings after the rerun;
   - 14 and 15 are the translator's (SCT-013, SCT-014);
   - 16 is the engine's inspection (LIF-013, ACC-033).

   Case 13 now concerns a FLEET-005 default or a first-start Conduit (step 2). Case 17's outcome
   follows point (d) of step 3. Say in the README that a later order writes the rest as golden
   cases, with one cascade and each counted flag that S1 does not use.

7. **Findings.** Where the reference needs what the specification does not give, such as an
   input, a disclosure or a promise, do not invent it. Name it in the reference as an open point,
   and log it in your report under `findings`, with the IDs. Among them: a request that carries
   documents rejects a relative `fetch` or program value (SUB-010; `replace.v1`'s description;
   `engine/README.md:711-713`), so a project that uses one cannot be applied as written. S1 uses
   none, so log it and do not settle it.

## Scope

Change:

- `docs/implementation/plan-and-apply/README.md`, replacing its placeholder;
- `docs/implementation/plan-and-apply/cases/`, new.

Do not change:

- `docs/README.md`: the reference's row is in place;
- `spec/`: report any defect or gap as a finding;
- `engine/`, `conformance/`, `packages/`, `experiments/` or `tools/`;
- `IN_FORCE.md`, `ROADMAP.md`, the ledger, the plan, a ruling's record or any other record.

Report anything else that needs changing.

## Done when

Every item of steps 1 to 7 is written, or its absence is reported with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. With the interpreter that has PyYAML, this prints the number of cases,
`ok;` and `areas missing: none`, and exits 0:

```bash
python3 -c 'import glob, yaml; keys = {"id", "area", "shows", "sources", "command", "files", "applied", "engine", "printed", "recorded", "requests", "kept", "exit"}; rkeys = {"tool", "arguments", "idempotencyKey", "flag", "result"}; areas = {"plan", "grants", "replacement", "cascade", "stops"}; paths = sorted(glob.glob("docs/implementation/plan-and-apply/cases/*.yaml")); cases = {p: yaml.safe_load(open(p)) for p in paths}; bad = [p for p, c in cases.items() if not isinstance(c, dict) or not keys <= set(c) or c["area"] not in areas or not isinstance(c["requests"], list) or any(not isinstance(r, dict) or not rkeys <= set(r) for r in c["requests"])]; missing = sorted({"plan", "grants", "replacement", "stops"} - {c["area"] for c in cases.values() if isinstance(c, dict) and "area" in c}); print(len(paths), "cases;", bad or "ok;", "areas missing:", missing or "none"); raise SystemExit(bool(bad or missing) or not paths)'
```

This prints `1`:

```bash
grep -m1 -v -e '^#' -e '^$' docs/implementation/plan-and-apply/README.md | grep -c '^Not binding\. The specification in spec/ alone binds\.'
```

Each of these prints nothing. The first shows that every promise ID the reference and its cases
cite exists in `spec/`. (The brackets in the third keep this order free of the paths it looks
for.)

```bash
for id in $(grep -rhoE '\b[A-Z]{3,5}-[0-9]{3}\b' docs/implementation/plan-and-apply | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
grep -rn -E '[[:space:]]+$' docs/implementation/plan-and-apply
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/' docs/implementation/plan-and-apply
grep -rn 'work-orders/' docs/implementation/plan-and-apply
```

The whitespace check is needed because `git diff --check` does not see files that are not
tracked.

`git status --porcelain` prints these two lines and no other; report any other line and why:

```text
 M docs/implementation/plan-and-apply/README.md
?? docs/implementation/plan-and-apply/cases/
```

If `tools/check_private_paths.py` exists in your tree, it passes too.

Report:

- each check's last line;
- every choice under `own_choices`. For each point for the owner, give each alternative with its
  command lines for T1 to T7, primary and strict, with the panel's actions apart;
- the cases, by area, and the tool that computed each digest;
- every finding.

Do not stage anything.

## Record

Run on 2026-11-11, beside several orders on a loaded machine: the worker in 24 minutes, then a fresh
verifier and three more rounds. The reference for `conduit plan` and `conduit apply` and nine golden
cases for S1's tasks T1 to T7 landed in `docs/implementation/plan-and-apply/`; they do not bind.

- Round 2 fixed two must findings: how the reference reads the state it needs (package resolution
  and the contract comparisons), and the recovery after a retirement that does not succeed.
- Round 3 fixed a must and a should: cascade recovery now works from the observed state and keeps
  CON-008 and LIF-021, and the install commands carry their archive digests.
- Round 4 fixed seven golden requests that expected no diagnostics although their manifests pin
  unused packages (NAM-008). The final verifier found no fault.
- The defaults follow experiment 011's ledger. Under them, S1's commands count 12 primary and 12
  strict across T1 to T7. The worker reported each reading reserved for the owner (ruling 71, point
  4) with its counts: which flags permit what, how a flag names its object, how commands reach the
  engine, what apply does when a set-up raises a sign-in, and `conduit plan` itself. Order 073 (PB)
  counts them with the authored files.
- Findings for the admin: WO069-O1 (relative fetch and program values cannot be applied as
  written), O2 (no expected revision on open or set-up), O3 and O4 (the temporary cascade Conduit
  against ruling 55's narrowing-first order).
- Checks: the documentation, rulings, names, schemas and roadmap checks and `git diff --check`
  passed; the Python unit tests could not run in the verifier's read-only sandbox.
