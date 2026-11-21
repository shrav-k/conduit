# Work order 240: DEC-008's wording in `spec/`: ruling 76 points 2 to 5 under "What the short form's meaning covers", and a cascade's one-time Conduit closed by its target's retirement

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`, with
`vectors` empty: no case changes. This order needs no network and changes text only. Run one
cargo command, the engine's whole suite, once at the end. Every crate it builds is in the local
cache already; if one is not, stop and report, and fetch nothing. Do not run `tools/gate.py`.
Never print a whole large file, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Run no `conduit` outside a test, no
`codex` or `claude`, and sign in nowhere. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

## What and why

**What.** DEC-008, plan and apply, is "accepted, not done (ruling 76's wording)"
([the log](../decisions/log.md#dec-008-2026-11-20-plan-and-apply)). Order 217 wrote
[ruling 76](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-76/README.md)'s
point 1 into `spec/README.md`, "What the short form's meaning covers" (#453). Points 2 to 5 are
not yet in `spec/`: they are in the ruling, in K3's rows B to E and in IN_FORCE.md topic 4's
*spec pending* line:

- 2: every flag names its object, by its generated name;
- 3: commands find the engine through a current engine context that `conduit serve` records,
  with `--store` as an override;
- 4: when a set-up raises a sign-in, apply goes on, prints it as the person's action at the
  panel, and answers none;
- 5: `conduit plan` is available and never required.

The same section lists "close the Conduit" among a cascade's requests. CON-008 already closes
and removes a Conduit whose target retires, so a later close would name a removed object and
be invalid (LIF-035). The plan-and-apply reference records this as its O4
([K3](../implementation/plan-and-apply/README.md)), and the full review's
[D86](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md#the-state-of-the-documents)
put the fix on the next specification order.

**Why first.** This is the first order of the S1 simplification chain, orders 240 to 251, which
the owner asked for on 2026-11-21 ("Hold it; write the chain now"). Orders 243 (the engine
context, `init` and `install`), 245 (`conduit plan`) and 247 to 249 (`conduit apply` and its
flags) build on this wording. Orders 242 and 244, the short-form translator, need only SCT,
which is already in `spec/`.

**Read against `main` at `cc3605a3`.** Where it differs from the chain's plan:

- `tools/check_decisions.py` lets a Status become "done" only if it names a pull request.
  Decision 5 cites #453, and the coordinator adds this order's number when the pull request
  opens.
- D86's other correction, the slice count in the Contents table, is already made.
- Each change gets its own line, so `spec/CHANGES.md` gets two lines, or three with decision 6.
- IN_FORCE topic 4's **In spec/** line says "the cascade flag", but `spec/` has two. Decision 4
  fixes it.
- Order 238's text has merged but its build has not, so this order adds its own index row.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): serves
2 and 6, and puts none at risk.

- Test 2: a cascade's temporary access is printed. The retirement it served closes it, and its
  tombstone is lifted.
- Test 6: apply answers no sign-in, plan is never a hidden prerequisite, and nothing runs
  automatically.

No promise, case, engine code or package changes.

**Should this exist** (ruling 92, point 3). Yes. It writes what the owner has already ruled into
the file that binds, and it removes a contradiction with CON-008. Each point is existing practice:

- kubectl's current context and `docker context use` record the engine once, and a flag on one
  command overrides it;
- `terraform apply` computes its own plan, so `terraform plan` is optional;
- `terraform apply -replace=ADDRESS` names the one object it replaces.

Not taken: a context-selection command; a saved plan that apply requires (K3 has neither); and
new flags or promises. DEC-008 says "Done by spec text alone".

**When it runs.** It runs through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim
for under an hour), then a Codex verifier and a Claude reader, who run only what settles a
doubt, then the gate. The set wording and the example operand are the coordinator's choices,
open to the owner's strike.

**The owner's item 3: the `sectioned-package` example.** It pins S1's own filesystem package at
S1's digest and defines S1's `read` and `read-write` templates. `spec/README.md` points authors
to it, and a fresh S1 author gets `spec/`, while IN_FORCE topic 13 says "The pack holds nothing of
S1's design". The coordinator recommended replacing it here, as order 217 replaced the S1
project example. The other option is to leave it out of the author's copy and record the
departure. Before dispatch, the coordinator writes the owner's answer on the next line.

Answer: replace it with a package that is not S1's (owner, 2026-11-21, DEC-039).

## The decisions

The coordinator applied decisions 1 to 5, and decision 6 with a sample package, in a scratch
copy: `check_decisions.py`, `check_docs.py` and `check_schemas.py` passed on the lines as written.

1. **The plan and apply paragraph (points 2 and 5, and O4).** In `spec/README.md`, "What the
   short form's meaning covers", replace the text from "Plan and apply are tooling composed of
   existing requests." through "close the Conduit, then root lifts its own tombstone." with
   this set wording, wrapped at the file's width:

   > Plan and apply are tooling composed of existing requests. The plan is printed, and apply
   > records it; nothing runs automatically. `conduit apply` computes and prints its own plan,
   > records it when it has a request to send, and then sends its requests; `conduit plan`
   > prints the same plan and sends and records nothing. Plan is available and never required.
   > In v0 the command is the approval. Every dangerous act needs a named flag, recorded with
   > the plan when used: `--reopen`, `--recreate`, `--prune`, `--replace`, `--adopt`, and the
   > cascade flags `--with-resources` and `--with-agents`. Each flag names one object by its
   > name, the generated name where the short form makes one (SCT-013), such as
   > `--reopen auditor--catalog`. Each cascade is one command under one flag, printed and
   > recorded with the plan: register a one-time Conduit to the Pool or Fleet, open it, and call
   > the cascade operation through it. The target's retirement closes and removes that Conduit
   > (CON-008, LIF-035), so apply sends no close; root then lifts the Conduit's tombstone
   > (LIF-040, ACC-053). A cascade that does not succeed stops apply as any failure does, and
   > its one-time Conduit may already be removed: a later apply under the same flag closes it
   > only if inspection finds that same Conduit, and lifts only the tombstone that retains its
   > ID.

   The text from "The `--withhold` flag belongs" on stays as it is. `auditor--catalog` is the
   Conduit that SCT-007 generates for the sectioned project's `auditor` and its `catalog`
   item.
2. **The engine context and sign-ins (points 3 and 4).** After the paragraph that ends "it
   prints the target's `ownTools` member.", add this paragraph:

   > A command that talks to a serving engine finds it through the person's current engine
   > context, as kubectl's and Docker's commands do: `conduit serve` records the engine it serves
   > as current, and `--store` on a command overrides the context for that command. When a set-up
   > that apply sends succeeds and its result names pending sign-in actions (IFC-025,
   > PND-005), apply goes on to its next step. It prints each as the person's action at the
   > panel, listed apart from authored lines, and answers none.
3. **`spec/CHANGES.md`.** Under `## [Unreleased]`, add these two lines:

   ```text
   - Added: `spec/README.md#what-the-short-forms-meaning-covers`, ruling 76 points 2 to 5: each flag names its object, commands use the current engine context, apply goes on past a sign-in and answers none, and `conduit plan` is never required (DEC-008).
   - Fixed: `spec/README.md#what-the-short-forms-meaning-covers`, a cascade's one-time Conduit is closed and removed by its target's retirement, and apply sends no close (DEC-008).
   ```
4. **IN_FORCE.md.**
   - Topic 4, the **In spec/** line on "What the short form's meaning covers": "the cascade
     flag," becomes "the cascade flags `--with-resources` and `--with-agents`,". After
     "Narrowing never needs a flag (76 point 1).", add "The flags' operands, the engine
     context, sign-ins during apply and `conduit plan` are in the same section (76 points 2 to
     5)."
   - Topic 4: remove the next line, which begins "*Spec pending, with a revision of order 069's
     reference and order 052a*".
   - Topic 9, the **In spec/** line on retirement conflicts and cascades: "The flag opening,
     using and closing a one-time cascade Conduit remains for CLI order 051a (59 A3)." becomes
     "The cascade flags and their one-time Conduit, which the target's retirement closes, are
     in spec/README.md, "What the short form's meaning covers" (59 A3)."
5. **DEC-008's Status** in `docs/decisions/log.md` becomes, with no other change to the log:
   `Status: done (ruling 76 point 1 by order 217, #453; points 2 to 5 by order 240)`.
6. **The package example, only where the line above reads `Answer: replace.`** Replace the two
   files of `spec/schemas/examples/valid/sectioned-package/` with a flow-style short-form
   package that is not S1's. Keep the directory's name, since `spec/README.md` and
   `spec/schemas/README.md` link to it, and its two file names, `conduit.yaml` and
   `templates.yaml`. The shape is yours, within these limits:
   - it keeps the address `example.org/sectioned-templates` and states a `version`;
   - it pins one package that is not under `example.org/s1/` and is not a filesystem or
     directory package, with a digest in the right form, as the sectioned project's is;
   - it defines and exports at least two reusable Conduit templates (SCT-005) over that
     package's capability, with qualified operation references (NAM-018), none of them named
     `read` or `read-write`, such as a ticket queue's `triage` and `resolve`;
   - no `config` holds an `apiKey`.

   Add this line under `## [Unreleased]`:

   ```text
   - Changed: `spec/README.md#an-author-of-documents`, its sectioned package example, which now pins no S1 package and defines none of S1's templates (DEC-007, ruling 78 pt 2).
   ```

   Where the line reads anything else, change nothing under `spec/schemas/` and say so.

## What to read

- `spec/README.md`, "An author of documents" and "What the short form's meaning covers".
- CON-008 (`spec/promises/conduits.yaml`); LIF-035 and LIF-040
  (`spec/conventions/lifecycle.yaml`); ACC-053 (`spec/conventions/access.yaml`); IFC-025
  (`spec/interface/implementation.yaml`); PND-005 (`spec/promises/pending.yaml`); and SCT-005,
  SCT-007 and SCT-013 (`spec/promises/sections.yaml`).
- Read only: K3's "Commands", its readings table (rows B to E), its cascade section and O4.
- `spec/CHANGES.md`'s head; `docs/decisions/README.md`, "The log"; IN_FORCE.md topics 4 and 9.
- For decision 6: both `sectioned-*` example directories and `spec/schemas/README.md`'s
  paragraph on them.

## Stop rule

Change only the files in Scope. Order 241 brings K3 up to date beside this order, and order 250
updates the markers. If the set wording contradicts a promise, or a check refuses a line
written as given, stop and change nothing further. Report the promise or the check's error, with
the words, and do not choose other words. Stop and report if the start check prints otherwise.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
grep -c '^close the Conduit, then root lifts its own tombstone\.' spec/README.md
grep -c -e 'engine context' -e 'never required' -e 'auditor--catalog' spec/README.md
grep -c '^  auditor: {' spec/schemas/examples/valid/sectioned-project/release-library.yaml
sed -n '/^## DEC-008/,/^## DEC-009/p' docs/decisions/log.md | grep '^Status:'
grep -c -e "Spec pending, with a revision of order 069" -e 'remains for CLI order 051a' IN_FORCE.md
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- '
grep -c '^Answer: replace\.$' docs/work-orders/240-dec-008-wording-in-spec.md
h=$(rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
```

The expected output, line by line:

- `main 0`, `0`, `1`, `0` and `1`;
- `Status: accepted, not done (ruling 76's wording)`;
- `2`;
- `0`, or a higher count if another order added a line first: report it and go on;
- `1` (apply decision 6) or `0` (skip it);
- `engine 0`. Any other value means a crate is missing from the cache: stop.

## Scope

Change `spec/README.md`, `spec/CHANGES.md`, `IN_FORCE.md` and `docs/decisions/log.md` as the
decisions say, and, under decision 6 alone, the files of
`spec/schemas/examples/valid/sectioned-package/`. Change nothing else. Put anything else that
needs changing in your report: K3, the shell's help, `engine/README.md`, HANDOVER, ROADMAP, and
topic 9's line that leaves `conduit fleet ...` to order 051a, which the chain moves to order 251.

## Done when

Each line in the first group prints otherwise on `main` at `cc3605a3` ("today") and passes
after:

```sh
j() { tr '\n' ' ' < spec/README.md | tr -s ' ' | grep -oF -- "$1" | wc -l | tr -d ' '; }
j 'close the Conduit, then root'  # 0; 1 today
j 'so apply sends no close'  # 1; 0 today
j 'Plan is available and never required.'  # 1; 0 today
j '`--reopen auditor--catalog`'  # 1; 0 today
j 'current engine context'  # 1; 0 today
j 'and answers none.'  # 1; 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '^- .*(DEC-008)\.$'  # 2; 0 today
grep -c -e "Spec pending, with a revision of order 069" -e 'remains for CLI order 051a' IN_FORCE.md  # 0; 2 today
grep -c 'in the same section (76 points 2 to 5)' IN_FORCE.md  # 1; 0 today
test "$(git show HEAD:IN_FORCE.md | grep -cE '^\s*[-*] ')" -eq "$(( $(grep -cE '^\s*[-*] ' IN_FORCE.md) + 1 ))"; echo "bullets $?"  # bullets 0; bullets 1 today
sed -n '/^## DEC-008/,/^## DEC-009/p' docs/decisions/log.md | grep -c '^Status: done (.*#453'  # 1; 0 today
```

Where decision 6 applies, these do too:

```sh
cat spec/schemas/examples/valid/sectioned-package/*.yaml | grep -cE 'example\.org/s1/|read-write|filesystem|directory'  # 0; 4 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '#an-author-of-documents'  # 1; 0 today
```

Where it does not, `git diff --name-only HEAD -- spec/schemas | wc -l | tr -d ' '` prints `0`.

Guards, which pass today and must still pass. Run the engine's suite once, last:

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_roadmap.py | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 -m unittest discover -s tools/tests 2>&1 | tail -1  # OK
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(spec/README\.md|spec/CHANGES\.md|IN_FORCE\.md|docs/decisions/log\.md|spec/schemas/examples/valid/sectioned-package/[^/]+\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes
```

Your report gives the following:

- the new paragraphs as written, and each IN_FORCE.md line before and after;
- whether decision 6 applied, and if it did, the new example's files;
- every check's last line;
- every choice you made yourself, under `own_choices`.

Stage nothing. The verifier's turn follows. The coordinator then runs `tools/gate.py`, has a
fresh, read-only Claude reader read the diff (ruling 79; ruling 92, point 4), adds this order's
pull request number to DEC-008's Status, and merges.

## What follows

These are not in this order:

- Order 241 updates K3's O4 and its rows B to E to cite this wording.
- Order 243 builds the context, and orders 245 and 247 to 249 build the commands and flags.
- Order 250 updates the IN_FORCE markers of topics 4, 9 and 13, and HANDOVER rows 9 and 12.
- If the owner keeps the example (the answer is not "replace"), the S1 rerun's author pack
  leaves it out and records the departure.

## Record

Run on 2026-11-21 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's turn** (14:20 to 14:25) did four things:
- wrote ruling 76's points 2 to 5 into `spec/README.md`;
- aligned the cascade sentence to CON-008, so retirement closes and removes the one-time
  Conduit, apply sends no close, and root lifts its tombstone;
- replaced the S1-derived sectioned-package example with a ticket-queue package;
- updated `CHANGES.md`, IN_FORCE.md and DEC-008's status.

The engine suite passed once: 498 passed, 0 failed, 9 ignored.

On the example, decision 6 was applied on the owner's answer in DEC-039 ("Replace with a non-S1
package"). The start check's literal `^Answer: replace\.$` did not match the answer line, which
quotes the owner.

**`turn-verifier-2.turn.v1`** (14:26 to 14:29) confirmed points 2 and 3, the cascade wording and
the example. It found two musts in the wording this order prescribed, which root fixed:
- a sign-in never stops apply, and a set-up that does not succeed stops it as any failure does;
- `conduit plan` sends none of its requests.

**The Claude reader** (14:30 to 14:33) approved with one should and three nits. Root applied:
- the should: `conduit plan` "records no plan", and its reads are recorded as any request is
  (REC-001);
- the CHANGES citation of DEC-039.

IN_FORCE topic 4's "printed and recorded" sentence is left for order 250, which updates that
topic.

At the rebase onto order 242, the gate's engine row failed once: 242's new loader test expected the old example's `read` and `read-write` templates. Root changed that one expectation to the new example's `resolve` and `triage`.

The gate's run on the squash is the coordinator's, at the merge.
