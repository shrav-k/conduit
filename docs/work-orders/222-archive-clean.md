# Work order 222: the archive clean, groups A and D: finished orders, experiments and research leave the tree, and git keeps them

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network and builds nothing itself; `tools/gate.py` may run cargo offline, and fetches nothing.
Never print a whole large file, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `conduit` against a store,
`codex`, `claude` or the dogfood harness, and sign in nowhere. Every file you write is public:
write no absolute local path, no user or machine name and no private repository name.

## What and why

**What.** On 2026-11-20 the owner asked for "a clean of the archive", chose "Git is the archive
(Rec.)", and for its groups "A: now (Rec.)" and "D: finished experiments" (the coordinator's
notes, private). Group A is the finished work orders, `docs/implementation/`, `docs/research/`,
`docs/LANGUAGE_DEFECTS.md` and the archive's pages; group D, the finished experiments. Groups B
(`docs/proposals/`) and C (`IN_FORCE.md`, `docs/AUDIT.md`) come later.

**Why.** 147 of the 194 order files are finished and leave, and one, 213, is superseded: 4.6 MB
of the 15.6 MB tracked under `docs/` (ruling 92, point 5).

**Preservation tests** ([ruling 92](../proposals/ruling-92/README.md), point 1): none served and
none at risk. The order changes no promise, engine code, package or case.

**Should this exist** (point 3). Yes: removing a file and linking its last version is ruling 57's
practice ([Removed from the tree](../archive/README.md#removed-from-the-tree)). It adds no
mechanism; decision 6 widens one check.

**When it runs.** Through Conduit, under experiment 028's
[Stage 2's orders](../../experiments/028-a78-dogfood/README.md#stage-2s-orders). It changes
nothing under `experiments/028-a78-dogfood/`. Decision 8 covers branches open while it runs.

## The decisions it applies

1. **Git is the archive.** A file that leaves is deleted with `git rm -r -q`, never moved. BASE
   is the full SHA the start check prints. In every file that stays, each relative link, inline
   or a reference definition, whose target is a removed path or a section that decision 2
   removes becomes `https://github.com/shrav-k/conduit/blob/BASE/<path>` (`tree/` for a
   directory). Keep its anchor and its link text. A path written as plain text or in code, not as
   a link, stays as written. Rewrite with a one-off command of your own; add no script to the
   tree.

2. **The archive's index.** `docs/archive/README.md` keeps its title, its opening paragraph and
   its section "Removed from the tree" (both tables and the paragraph between them). Its other
   sections go. Append to the first table these rows, with BASE and its first 8 characters filled
   (*Coordinator's*, open to the owner's strike: one row per group, not per item):

   ```text
   | `docs/work-orders/`: order 222's leave list | Finished orders and declarations. | [Live orders](../work-orders/README.md). | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/work-orders) |
   | `experiments/{004, 008, 009, 010, 023}` | Five finished experiments. | [The register](../../ROADMAP.md#experiments-and-probes). | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/experiments) |
   | `docs/research/{SECRETS.md, MULTI_MACHINE.md, naming/, vocabulary/}` | Precedents of settled decisions. | The proposals they fed, and `spec/`. | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/research) |
   | `docs/archive/README.md`, its sections after this one | Format choice, the collaborator comparison and its adaptations, the roadmap candidates, prior art, the repository material and its conclusions to test, the release and website proposals, attribution. | [Two roadmap rows](../../ROADMAP.md#done-removed-dropped-or-not-adopted). | [BASE8](https://github.com/shrav-k/conduit/blob/BASE/docs/archive/README.md) |
   ```

   In the table's `vocabulary/` row, the text after "Vocabulary decisions;" becomes "the rest of
   `vocabulary/` left with order 222."

   *Coordinator's*, open to the owner's strike: `docs/archive/plan-audit-history.md` stays.
   `tools/check_roadmap.py` reads its 21 action-item rows, `engine/CONFORMANCE.md` links it, and
   open A27 needs its "Reference repositories". It goes with `docs/AUDIT.md`, in group C.

   *Coordinator's*, open to the owner's strike: two ideas in the removed sections are still live.
   Each becomes one row of ROADMAP.md's table "Done, removed, dropped or not adopted", after the
   row "Sealmesh; a local root Agent; a VM per environment":

   ```text
   | Release and delivery | One coordinated release first; contract, package, engine and storage-format versions kept apart; archives only for tested platforms; release artifacts tested outside the source tree | not adopted: a candidate for A76, decided at first publish | [Archived proposal](https://github.com/shrav-k/conduit/blob/BASE/docs/archive/README.md#5-release-and-delivery-proposal) |
   | Website | Routes by reader task (learn, author, reference, the engine, the project); a first example that defines a new capability; development docs told apart from released ones | not adopted: a candidate for A76's one website (ruling 47) | [Archived proposal](https://github.com/shrav-k/conduit/blob/BASE/docs/archive/README.md#6-website-and-documentation-experience) |
   ```

   The rest holds no open work: `spec/` applies the format choice; the collaborator material is
   history; the Sealmesh row holds the roadmap candidates; prior art gives reasons for choices
   made; ruling 47, A75 and A76 hold the repository material; CONTRIBUTING.md's "License and
   attribution" names the attribution's pull requests, and `plan-audit-history.md` the Pi pins.

3. **Finished work orders leave.** An order on the leave list leaves with its
   `docs/work-orders/<ID>-*.md` and its `<ID>-declared.yaml`. Each is paired with the squash
   commit on `main` that carries its work; for 050f, 050i, 050k, 050l and 078, built on the 050
   lanes, that is 050n's join. Order 213 is superseded: this order replaces its move into
   `docs/archive/work-orders/`. Order 212's parts that assume that move (step B, the index split,
   the archive of merged orders and its check, and the heading `## Additional roadmap candidates`)
   are rewritten at its refresh. Two declarations of leaving orders stay: `041a-declared.yaml`
   (`tools/migrate_041a.py` names it, and `check_docs.py` checks names in tools) and
   `209-declared.yaml` (orders 217 and 218 copy its form).

   ```sh
   mkdir -p target/wo222
   cat > target/wo222/leave.txt <<'EOF'
   037:9294c49d 038a:d08fb8bb 038c:719a8992 039:879f3063 040:b60112ed 041a:5ed438b2 041b:1091a71b
   044:be729584 045:99b641a3 046a:7814c7b8 046b:0e6343cf 046c:1ddb5678 047:3aafbd93
   047b:6c2c274c 047c:e4ceb4c8 047d:3c761dd9 048a1:facb4ff4 048a3:8a393e49 048a4:bba2f45e
   048a5:9e2a1410 048a5b:95b9fc3b 048a5d:148258e6 048a5d2:f2d972df
   048a5gh:246038e6 048a6:fde1c615 048b-relay1:8e205447 048b-relay2:9f7669e9 050b:649ed2af
   050f:b897d316 050g:2d209c09 050i:b897d316 050k:b897d316 050l:b897d316
   067:86c3b110 068:defee9ba 069:063e0ccd 069b:9729f211 071:2318ec59 072:969489cf 073:d5cb9b78
   074:6591a064 075:55cf7473 078:b897d316 081:35d2e45a 082:1be89f96 100:c2a1d063
   101:ea24a7d6 102:a0acc539 103:b46d470c 104:3bc816c5 105:1f302eca 107:511d606c 108:ebd723e4
   109:bd0717e1 110:1d5a48fd 111:211f7139 112:e1859597 114:211f7139 115:c9d3c4eb 116:f8adc17a
   118:61101e4c 119:63d644c0 120:1d633c55 121:1ecdf3d1 122:38c0b4f7 123:752b4830 124:38c0b4f7
   125:752b4830 126:c9b6240c 127:48c76a79 128:952e9f51 129:be3ab2fc 130:be3ab2fc 131:99247488
   132:a005dab0 133:0cc4fba3 134:1346ba24 135:9f55b1e6 137:6ce8d136 138:a8d2e947 139:0fee95d4
   140:6ce8d136 141:2689cbc5 143:5f7605ad 148:f266adfb 149:7e963b1a
   150:7e963b1a 151:e8a7bcd1 152:bd987017 154:d2b44944 155:4009d4ef 156:b1f6f8ca 157:d3140289
   158:e331d614 159:632881df 161:7346a280 163:42c64e24 164:1b01459f 165:ef980e64 166:b8efffbd
   167:68cd9f79 168:4a0a2739 169:2e4ddb9d 170:20dffe78 171a:41c620aa 171b:0db2951c 172a:121020ec
   172b:852a6d52 173:ab07fd47 174:ec926025 175:eac54d78 177:5f1553ea 178:dd1c2208 179:27e24db9
   180:2c0dc6bd 181:2f33a53e 182:7528a3c4 184:911961a2 185:f383edaf 186:0e53c233 187:0ed26306
   188:4a37c3ba 189:d4c8cf8b 190:b5780aa7 191:022b00cc 192:a771b88b 193a:f08a73ea 193b:2dc33a6b
   194:e7f86211 195:20e9b29d 196:ecabe372 197:787be003 198:3791fc80 199:c15c0fa1 200:281e35d8
   201:75944df4 202:506aa144 204b:ac53b20f 205:0052b91f 206:553450aa 207:0b5b1eba 209:92632057
   210:45026c21 211:fc1c54a6 213:superseded 214:a6dcd98b 215:d97fbe76 216:55d889bc
   EOF
   cat > target/wo222/stay.txt <<'EOF'
   049a1 051b3 079 080 117 144 147 203 204 205b 212 217 218 219 220 221 222
   038b 048a2 048a5ef 048b-join 050a 050d 050e 050h 050j 050m 050n 051b 051b2 077 113 136 142
   145 146 153 160 208 070 162 176 183 042 048a5c 050c 076
   EOF
   ```

   The stay list, by reason:
   - Open, in flight or held: 049a1, 079 and 080 (on branches; 221 carries 079 and 080); 051b3
     (held, ruling 88); 117 (it holds the cuts of the held 048a-7, 048a-8 and 050o); 144, 147,
     203, 205b, 212 and 217 to 222 (not yet run); 204 (its Record: "the order is not done").
   - Read by a staying order's checks or reading list: 038b (by 049a1); 048a2, 113 and 160 (204);
     048a5ef, 048b-join, 051b and 051b2 (051b3); 050a, 050d, 050e, 050h, 050j, 050m and 050n (203
     and 212); 077, 136 and 142 (144); 145 and 146 (147); 153 (217); 208 (205b).
   - Linked from `engine/CONFORMANCE.md` (162, 176, 183) or `conformance/README.md` (070), which
     this order does not change, or from `experiments/028-a78-dogfood/`, which a stage-2 order may
     not change (042, 048a5c, 050c, 076).

   Also staying: `README.md`, `verify.md`, `verify.schema.json`, the shared schema, the three
   `*-staged/` directories (`spec/` links them; their patches are not yet applied), `050j-a77.yaml`
   and `050k-a77.yaml` (A77's records; A77 is in progress), and each staying order's own
   declaration. `docs/work-orders/README.md` keeps the rows of the orders that stay and the
   unlinked rows of 048a7, 048a8 and 050o, and gains one line under "Orders still in the
   directory": "Work order 222 removed the other finished orders; git keeps them at
   [`BASE8`](https://github.com/shrav-k/conduit/tree/BASE/docs/work-orders)." Its lines from
   "Declarations stay with the orders that use them." to "removes the order and its own schema."
   become: "A merged order's file at its last commit is its Record (ruling 90, point 4).
   041a-declared.yaml and 209-declared.yaml stay for the tool and the orders that read them."

4. **`docs/implementation/`, `docs/research/` and `docs/LANGUAGE_DEFECTS.md`.** By ruling 57,
   these leave `docs/research/`, their content held elsewhere: `SECRETS.md` (ruling 41 and the
   SEC promises), `MULTI_MACHINE.md` (ruling 43), `naming/` (`docs/proposals/naming/` and the NAM
   promises) and `vocabulary/` (`docs/proposals/vocabulary/` and `spec/definitions.md`;
   CONTRIBUTING.md's "License and attribution" credits pull request 5).

   *Coordinator's*, open to the owner's strike: the rest stays until HOW-WE-WORK.md, at the root,
   or its last reader takes it: all of `implementation/` (AGENTS.md, CONTRIBUTING.md, scenarios
   013 to 021, `tools/check_model_account.py`, A63, A84 and the second implementation read it);
   in `research/`, `model-access-gateways.md` (A85) and `README.md` (its rows for removed files
   go); `LANGUAGE_DEFECTS.md` (its open type-contract rows feed G7 and A42).

   In `docs/README.md`, the row for `archive/` reads "Removed-file index and the plan audit's
   history.", and the rows for `plan-and-apply/` and `SAFETY_CATALOGUE.md` lose their last
   sentence, "Being written by work order …".

5. **Finished experiments leave.** 004 (A40; 012 replaced it), 008 (A71), 009 (A4), 010 (A8; its
   re-read before v0 is published is a new desk check) and 023 (PB), each done. No open order,
   scenario, harness or the readiness gate reads them; their register rows cite results.
   *Coordinator's*, open to the owner's strike: these stay, against the expectation:
   - 005: A22 is in progress, and `tools/check_private_paths.py` reads its `src/lib.rs`.
   - 007: A70 is done, but `engine/CONFORMANCE.md` links its results.
   - 011: A79 is done, but open order 049a1 runs its tests and may change it.
   - 025: A73's stage 2 is "later", and S10's plan puts its probe in this directory.
   - 026, 027 and 032: A83 and A5 are in progress; A85 has not run.

   012, 013 to 021, 022 and 028 stay. In `experiments/README.md` the five rows go; one line after
   the table says "Experiments 004, 008, 009, 010 and 023 left the tree under work order 222;
   the archive index links their last versions.", with "archive index" linking
   `../docs/archive/README.md#removed-from-the-tree`. Its "Evidence for v0" keeps its lines,
   their links pinned. "Exact prompts and counted artifacts stay as evidence." becomes "Exact
   prompts and counted artifacts stay while their experiment stays; then git keeps them."
   ROADMAP.md's register rows keep their states, their links pinned.

6. **One tool.** `tools/check_roadmap.py`, check 7: a done row's dated results may also be a
   link `https://github.com/shrav-k/conduit/blob/<40 hex>/experiments/NNN-name/<file>[#anchor]`,
   read by the same file-name and anchor rule. It does not read the file: `check_docs.py` checks
   the path and anchor at that commit. Say so in its docstring. Add a test to
   `tools/tests/test_roadmap.py` whose name holds `pinned_results`: such a link passes; the same
   link with a short SHA or a branch name fails.

7. **Not in this order.** In `docs/proposals/`, `IN_FORCE.md` and `docs/AUDIT.md`, links to
   removed files are pinned as decision 1 says, and nothing else changes; links into them are
   untouched.

8. **Branches in flight.** A branch rebased onto `main` after this merges pins its new links to
   removed files to this order's BASE; `check_docs.py` reports each as a missing path. A rebase
   conflict on a line this order rewrote keeps the branch's text, its links pinned; a branch that
   edits a removed file restores nothing and is reported to the coordinator.

## Stop rules

- Change nothing under `spec/`, `engine/`, `packages/`, `conformance/` or `.github/`. If a
  removal needs a change there, stop, change nothing further, and report it.
- If the start check disagrees with a fixed list, stop and report. Do not move a file from one
  list to the other.

## Before you change anything

From the repository root, run decision 3's block, then the lines below. Stop, change nothing and
report if any prints otherwise:

```sh
git merge-base --is-ancestor 14292f3c HEAD; echo "main $?"
git merge-base --is-ancestor HEAD origin/main; echo "pushed $?"
git rev-parse HEAD                                   # BASE
git status --short | wc -l | tr -d ' '
ls docs/work-orders | wc -l | tr -d ' '              # N
for p in $(cat target/wo222/leave.txt); do i=${p%%:*}; c=${p#*:}; f=$(ls docs/work-orders/$i-*.md); [ "$c" = superseded ] && continue; git merge-base --is-ancestor "$c" HEAD || echo "not on main $i"; awk '/^## Record/{r=1;next} r&&NF{print;exit}' "$f" | grep -qv '^Not yet run\.$' || echo "no record $i"; done; echo leave
for i in $(cat target/wo222/stay.txt); do ls docs/work-orders/$i-*.md >/dev/null 2>&1 || echo "missing $i"; done; echo stay
for p in $(cat target/wo222/leave.txt); do i=${p%%:*}; ls docs/work-orders/$i-*.md docs/work-orders/$i-declared.yaml 2>/dev/null; done | grep -v -e '/041a-declared.yaml$' -e '/209-declared.yaml$' > target/wo222/files.txt
ls -d experiments/004-* experiments/008-* experiments/009-* experiments/010-* experiments/023-* docs/research/SECRETS.md docs/research/MULTI_MACHINE.md docs/research/naming docs/research/vocabulary >> target/wo222/files.txt
wc -l < target/wo222/files.txt | tr -d ' '
for f in $(cat target/wo222/files.txt); do git grep -h -F "${f#docs/}" -- spec engine packages conformance; done | grep -vc 'github.com/shrav-k/conduit/'
for d in docs experiments; do git ls-files -z -- $d | xargs -0 cat | wc -c | tr -d ' '; done   # sizes
```

They print `main 0`, `pushed 0`, BASE, `0`, N (251 if no order was added since this text),
`leave`, `stay`, `194`, `0`, and two sizes (about 15.6 and 4.0 million bytes).

## Scope

Delete the files that decisions 3, 4 and 5 list. Change `docs/work-orders/README.md`,
`docs/archive/README.md`, `docs/research/README.md`, `docs/README.md`, `experiments/README.md`,
ROADMAP.md (two rows, and link targets), `tools/check_roadmap.py` and
`tools/tests/test_roadmap.py`, and link targets alone in any other Markdown file outside the five
directories of the stop rules. Create nothing in the tree. Anything else goes in your report.

## Done when

Each line marked "fails today" prints otherwise on BASE; all pass after.

```sh
BASE=<the start check's SHA>
python3 tools/check_docs.py | tail -1            # Errors: 0
python3 tools/check_roadmap.py | tail -1         # Errors: 0
python3 tools/check_private_paths.py | tail -1   # Errors: 0
python3 -m unittest discover -s tools/tests 2>&1 | tail -1          # OK
grep -c 'def test.*pinned_results' tools/tests/test_roadmap.py  # 1 or more; fails today (0)
python3 tools/gate.py | tail -1                  # begins "Gate: PASS."
git diff --check                                 # prints nothing
git diff --name-only HEAD -- spec engine packages conformance .github | wc -l | tr -d ' '  # 0
for f in $(cat target/wo222/files.txt); do test -e "$f" && echo "$f"; done | wc -l | tr -d ' '  # 0; fails today (194)
for i in $(cat target/wo222/stay.txt); do ls docs/work-orders/$i-*.md >/dev/null 2>&1 || echo "missing $i"; done; echo stay  # stay
ls docs/work-orders | wc -l | tr -d ' '          # N less 185; fails today
grep -c '^| \[' docs/work-orders/README.md       # 47, plus orders added since; fails today (180)
grep -c "tree/$BASE/docs/work-orders" docs/work-orders/README.md  # 1; fails today (0)
grep -c '^## ' docs/archive/README.md            # 1; fails today (11)
grep -c "/$BASE/" docs/archive/README.md         # 4 or more; fails today (0)
grep -c '^| \[' docs/research/README.md          # 1; fails today (5)
grep -cE '^\| \[0(04|08|09|10|23) ' experiments/README.md  # 0; fails today (5)
grep -cE '^\| (Release and delivery|Website) \|' ROADMAP.md  # 2; fails today (0)
u='github\.com/shrav-k/conduit/(blob|tree)/[0-9a-f]+/[^)#> ]+'
git diff HEAD | grep '^+' | grep -oE "$u" | sort -u > target/wo222/added.txt
git diff HEAD | grep '^-' | grep -oE "$u" | sort -u > target/wo222/gone.txt
comm -23 target/wo222/added.txt target/wo222/gone.txt > target/wo222/new.txt
grep -vc "/$BASE/" target/wo222/new.txt          # 0: every new pinned link names BASE
sed -E "s#.*/$BASE/##; s#/\$##" target/wo222/new.txt | while read p; do git cat-file -e "$BASE:$p" || echo "missing $p"; done; echo pinned  # pinned
wc -l < target/wo222/new.txt | tr -d ' '         # 1 or more; fails today (0)
for d in docs experiments; do git ls-files -z -- $d | xargs -0 cat | wc -c | tr -d ' '; done  # report both
```

`git rm` stages the deletions; stage nothing else. Report the files removed and links rewritten
per top directory, the sizes before and after, your own choices under `own_choices`, and every
check's last line.

## Record

Not yet run.
