# Work order 228: the decision framework's gate checks: `tools/check_decisions.py`, run by the gate and by CI

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network and builds no crate: it is Python and Git alone. Never print a whole large file, diff,
log or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Do not run `conduit`, `codex`, `claude`, the dogfood harness or
`tools/gate.py`, and sign in nowhere. Every file you write is public: write no absolute local
path, no user or machine name and no private repository name.

## What and why

**What.** The owner adopted the decision framework on 2026-11-20
([DEC-001](../decisions/log.md#dec-001-2026-11-20-the-decision-framework)): `spec/` binds alone,
with one `spec/CHANGES.md` line per change; `HOW-WE-WORK.md` holds the rules of the work;
[the log](../decisions/log.md) holds the owner's decisions, append-only. Change 1, those files as
drafts, merged in #456. This is change 2: the checks that keep them, as one gate row. Later
changes: 1b, AGENTS.md "What holds" names the new homes and the drafts end; 3, `docs/proposals/`
leaves the tree; 5, `HOW-WE-WORK.md` is written and AGENTS.md cut to 150 lines; 6, `IN_FORCE.md`
and `docs/AUDIT.md` leave. The owner's words: the not-done ratchet -> "Fails (Rec.)" (DEC-001);
the 136 promises -> "Sort and ratchet (Rec.)"
([DEC-026](../decisions/log.md#dec-026-2026-11-20-proof-cases-experiments-measurement));
retroactive -> "Yes, via a ratchet (Rec.)", dormancy -> "4 weeks (Rec.)" (answers of the same
sitting, in no log entry yet); two process rules -> "As you propose (Rec.)", AGENTS.md at most
150 lines, gate-checked
([DEC-030](../decisions/log.md#dec-030-2026-11-20-how-an-order-runs-and-merges)); archive
approach -> "Git is the archive (Rec.)"
([DEC-031](../decisions/log.md#dec-031-2026-11-20-records-the-archive-and-the-documents)). So no
hash list of archived files is built: `tools/check_docs.py` already checks each permalink's
commit, path and anchor.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests);
[ruling 92](../proposals/ruling-92/README.md), point 1): none served and none at risk. The order
changes no promise, engine code, package or case. It keeps the record that ties each change to
`spec/` to an owner's decision, and stops a promise losing its case unnoticed.

**Should this exist** (point 3). Yes, as existing practice: a pull-request rule that a change to
some paths adds a changelog line (Danger's first example; Keep a Changelog's `Unreleased`), and a
ratchet that lets a debt count fall but never rise (Rust's `tidy`; Kubernetes' rule of no new
feature without conformance tests). Danger and such actions run on GitHub; merges wait on the
local, offline gate ([ruling 80](../proposals/ruling-80/README.md)), and the Python checkers in
`tools/` are the named exception to Rust
([DEC-025](../decisions/log.md#dec-025-2026-11-20-the-engine-is-rust-no-private-code)). It adds one
tool and one gate row.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under 2 hours), a Codex verifier and a Claude reader, who run only what settles a doubt, and the
gate, which runs everything
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.
The names, line forms, the split between failing and reporting, and the no-case ratchet's home
here rather than in `promise_standing.py` are the coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where: the
real tree gives an `error:` line (report it; do not change the file it names, and do not weaken
the rule); a correct result needs a change outside the scope; `test_gate.py` needs more than
decision 8 names; or anything asks for a crate or the network. A choice no decision governs is
yours: list it under `own_choices`.

## The decisions

1. **The tool.** `tools/check_decisions.py BEFORE AFTER`: BEFORE a commit, AFTER a commit or
   `WORKTREE`, read as `tools/vector_delta.py` reads them (WORKTREE includes untracked files that
   `.gitignore` does not name). It needs Python 3 and Git alone, no PyYAML, and runs under
   Python 3.9 (the Mac's `/usr/bin/python3`, which the gate may run under) and 3.12 (CI's).
   `--today YYYY-MM-DD` sets the date decision 6 reads (default: today in UTC).
   - Each finding is one line beginning `error: ` (it fails) or `report: ` (it does not), naming
     the file, and the line where there is one. The last line is `Errors: N`.
   - Exit 0 with no error, 1 with any, 2 for bad arguments or a revision that does not resolve.
   - Where BEFORE lacks a file a rule compares, that rule compares nothing and says so in a
     `report:` line, except 3.4, 3.5, 4.2 and 5, which say what they do.
   - A KB is 1,024 bytes. A promise ID is a token like `ACC-001` (two to five capitals, a hyphen,
     three digits), other than `DEC-NNN`. A decision reference is a `DEC-NNN` that has an entry
     in AFTER's log, or "ruling N" with 1 ≤ N ≤ 92. Parentheses that "cite decisions" hold at
     least one, and every `DEC-NNN` and "ruling N" in them is one.
2. **In force or a draft.** The framework is in force when none of `HOW-WE-WORK.md`,
   `spec/CHANGES.md`, `docs/decisions/README.md` and `docs/decisions/log.md` holds, in AFTER, a
   line beginning `Status: draft, not yet in force`; change 1b drops all four. Some with it and
   some without is an error. The tool prints `Framework: in force.`, or a line beginning
   `Framework: not in force` that names the rules that report only.
3. **These fail now**, whatever the drafts say:
   1. *CHANGES lines.* Each line added to a list in `spec/CHANGES.md` (an item with its indented
      continuation lines, joined) reads `- Kind: … (…).`, Kind one of Added, Changed,
      Deprecated, Removed, Fixed and Security, and its last parentheses cite decisions. Each
      promise ID it names is a row of AFTER's `engine/STANDING.md`; in a `Removed:` line, a row
      of BEFORE's and not of AFTER's. A line with no ID names `spec/README.md#A` or
      `spec/definitions.md#A`, A an anchor that `anchors()` in `tools/check_docs.py` finds in that
      file (import it, do not copy it), or, in backticks, a term of `spec/definitions.md`: a
      first-column cell of one of its tables, or one comma-separated part of one.
   2. *Append-only.* An entry is a `## DEC-NNN, YYYY-MM-DD, title` heading and what follows it
      up to the next `## ` heading, trailing blank lines dropped. Each of BEFORE's entries is in
      AFTER, in the same order, byte for byte but for its Status paragraph (the line beginning
      `Status:` and the lines after it up to a blank line). A changed Status paragraph was one
      beginning `Status: accepted, not done`, begins `Status: done`, and names a pull request as
      `#N` ([the log's rule](../decisions/README.md#the-log)).
   3. *Numbering.* Every `## ` heading of the log is an entry heading; the numbers run from 001
      by one, with no gap or repeat; no entry is dated before the entry above it.
   4. *Gone stays gone.* Where BEFORE has no `docs/proposals/`, no `IN_FORCE.md` or no
      `docs/AUDIT.md`, AFTER has none either. This arms itself as changes 3 and 6 remove them.
   5. *New orders.* A file added directly in `docs/work-orders/` (not in a subdirectory) whose
      name begins with a digit and ends `.md` holds `**Preservation tests` and `**Should this
      exist` (ruling 92, points 1 and 3, in force now; orders 217 to 226 all hold both).
   6. *The ratchets* of decision 6.

   Why now: each reads only what a change adds, or a count against BEFORE. A bad line or an
   edited entry let through now is never checked again, and none of these can fail on a tree
   `main` holds today. The owner chose that the not-done ratchet fails.
4. **These report until the framework is in force, then fail:**
   1. *Coupling.* A change under `spec/`, other than to `spec/CHANGES.md`, adds at least one line
      under `## [Unreleased]`. A change to `.md` files under `spec/` that differs only in
      Markdown link targets (inline `](…)` and reference definitions) needs none.
   2. *No third copy.* No file is added under `docs/proposals/`, and the count of `IN_FORCE.md`'s
      bullet lines (a line whose first non-space characters are `- ` or `* `) does not rise.
      Bullets, not physical lines: order 216 rewrapped lines.
   3. *HOW-WE-WORK form.* Each list item under a `## N. ` heading, with its continuation lines,
      ends with `(…) label`, the parentheses citing decisions. The label is exactly one of
      `` `by review` ``, `` `checked: tools/PATH` `` (PATH a file in AFTER) and
      `` `refused: NAME` `` (NAME a `name:` of `spec/core/operations.yaml`, a `management.`
      prefix allowed). The item holds no date (`YYYY-MM-DD`), no order number
      (`order` or `orders`, then three digits) and no `*Coordinator's*` mark (the mark, not the
      word).

   Why: each needs a home that is a draft today.
   [AGENTS.md "Where things go"](../../AGENTS.md#where-things-go) still writes a ruling's record
   under `docs/proposals/` and its lines into `IN_FORCE.md`; `spec/CHANGES.md` says it is a
   draft; and `HOW-WE-WORK.md` is change 5's to write (its two lines under section 3 cite no
   decision today).
5. **Caps.** `AGENTS.md` at most 150 lines; `HOW-WE-WORK.md` at most 12 KB; each log entry at
   most 1.5 KB. A cap fails where BEFORE is within it or lacks the file or entry, and reports
   where BEFORE is over it too. So `HOW-WE-WORK.md` (5,101 bytes) and the entries (802 bytes at
   most) fail now. `AGENTS.md` has 222 lines, and its cap reports until change 5 moves its
   "Checks" section (99 lines) to CONTRIBUTING.md; it then fails from the first gate whose
   BEFORE is within 150 lines, with no change to the tool. A ratchet on its count is not built:
   change 1b may add lines to it.
6. **The ratchets.** Both are computed for BEFORE and AFTER, at the same `--today`; a rise
   fails, and the change that lowers a count lowers the ceiling, with no number stored.
   - *No case.* `No-case ratchet: base B, head H.`, the `all` column of the
     `no case, no reason` row of `engine/STANDING.md`'s totals (124 at `59cf7ce9`; 136 when the
     owner answered). DEC-026 lands it in `tools/promise_standing.py`; it reads that tool's
     output, which `--check` keeps true, and sits here because only this tool gets BEFORE.
   - *Not done.* An entry is a language decision when a line of `spec/CHANGES.md`, under any
     heading, holds its `DEC-NNN` in its last parentheses. It is done when every promise ID of
     those lines outside `Removed:` lines is `case passes` in that tree's `engine/STANDING.md`;
     a line with no ID is done by its text ("Done by spec text alone (Rec.)",
     [DEC-002](../decisions/log.md#dec-002-2026-11-20-what-conduit-is-for)). The count is the
     language decisions not done whose date is more than 28 days before `--today`:
     `Not-done ratchet: base B, head H, entries dated before D.` Time raises both counts alike
     and fails nothing; a change fails when it ties an old decision to a promise with no passing
     case, or takes a case from one. Its base and head are 0 today: `spec/CHANGES.md` has no
     line.
7. **Reported, never failing.**
   - A new order (3.5's files) over 20 KB: ruling 92 point 1 calls it "a sign".
   - Each dormant entry: more than 28 days old and not done, a language decision by 6, any other
     by a Status line that does not begin `Status: done`; an entry with no Status line is
     skipped. The owner keeps, defers or strikes it.
   - `By review: N of M HOW-WE-WORK rule lines.`
   - `Ratio: ` and the lines of tracked files in AFTER under `spec/` and `engine/`, to those under
     `docs/work-orders/` and `docs/proposals/`, and their quotient to two places (HOW-WE-WORK
     rule 5).
8. **Wiring.**
   - `tools/gate.py`: a row `decisions` right after `vector changes`, fed the merge base as that
     row is, `[sys.executable, 'tools/check_decisions.py', merge_base, 'WORKTREE']`, printed
     `python3 tools/check_decisions.py <merge base> WORKTREE`, required, with that row's reason
     and 15-minute limit; not in `PYTHON_GATES`.
   - `tools/tests/test_gate.py`: `test_python_gates_follow_the_docs_job`'s list gains
     `('decisions', 'python3 tools/check_decisions.py ' + self.base + ' WORKTREE')` after
     `vector changes`, with its slice one longer, and each count the new row changes follows
     ("12 passed", the 11 calls, the 12 `WOULD RUN` lines and the "other gates would run"
     counts, each one more); the assertion that vector_delta's call is last reads `calls[-2]`.
     No other test changes meaning.
   - `.github/workflows/checks.yml`: the docs job's last step, after vector_delta's, named and run
     as `python3 tools/check_decisions.py HEAD^1 HEAD`, with
     `if: ${{ !cancelled() && github.event_name == 'pull_request' }}` and a one-line comment. The
     test above compares it with the gate's row.
9. **The tests**, `tools/tests/test_decisions.py`, in the form of `test_vector_delta.py`. Each
   builds a small Git repository in a temporary directory holding the files the rules read (a
   few lines each, with a synthetic `engine/STANDING.md`), commits BEFORE with
   `-c user.name=` and `-c user.email=` of `Conduit work order <work-orders@conduit.invalid>`,
   changes the tree, and calls `main` with BEFORE and `WORKTREE` (one test with a commit as
   AFTER).
   - Each rule has a tree that breaks it and one that keeps it.
   - Each rule of decision 4 is shown reporting with the draft lines and failing without them.
   - Caps: `AGENTS.md` at 151 lines fails over a BEFORE of 150, and reports over one of 222.
   - Not done: `--today` makes one entry 28 days old and another 29; only the second counts.
   - Test names begin `test_coupling_`, `test_changes_line_`, `test_append_only_`,
     `test_numbering_`, `test_gone_`, `test_third_copy_`, `test_caps_`, `test_form_`,
     `test_orders_`, `test_no_case_`, `test_not_done_` or `test_in_force_`, by rule; each prefix
     is used.

Not built here: the `disputed` mark of DEC-026, which needs a home under `conformance/` that the
runner and `promise_standing.py` read; the placement check of change 6 (`--placement`).

## What this leaves

Report this list, corrected by what you built: the `disputed` mark; `HOW-WE-WORK.md`'s labels for
the rules now checked, which change 5 writes as `checked: tools/check_decisions.py`; change 6's
placement map; `tools/check_rulings.py`, which leaves with change 3; change 1b gives
HOW-WE-WORK section 3's two items their decision (DEC-030, DEC-033), or its gate fails 4.3; a
log entry for the retroactive and dormancy answers, since 3.2 freezes DEC-026. The `AGENTS.md`
cap and the gone-stays-gone rule arm themselves; no order is needed.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 59cf7ce9 HEAD; echo "main $?"
test -e tools/check_decisions.py; echo "tool $?"
grep -c check_decisions tools/gate.py tools/tests/test_gate.py .github/workflows/checks.yml
grep -c "\['gates'\]\[2:13\]" tools/tests/test_gate.py
grep -c '^Status: draft, not yet in force' HOW-WE-WORK.md spec/CHANGES.md docs/decisions/README.md docs/decisions/log.md
grep -c '^## DEC-033, 2026-11-20, ' docs/decisions/log.md
grep -cE '^- (Added|Changed|Deprecated|Removed|Fixed|Security): ' spec/CHANGES.md
grep -cE '^\| no case, no reason \| [0-9]+ \| [0-9]+ \| [0-9]+ \| [0-9]+ \|$' engine/STANDING.md
test "$(wc -l < AGENTS.md)" -gt 150; echo "agents $?"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform aarch64-apple-darwin >/dev/null); echo "offline $?"
git status --short | wc -l | tr -d ' '
```

They print:
- `main 0`, `tool 1`, then three lines ending `:0`;
- `1`, then four lines ending `:1`;
- `1`, `0`, `1`, `agents 0`, `offline 0` and `0`.

This order builds no crate; `offline` printing otherwise means a crate the gate's discovery
reads is missing from the cache: stop and report, and fetch nothing.

## Scope

Create `tools/check_decisions.py` and `tools/tests/test_decisions.py`. Change `tools/gate.py`
(the row of decision 8 alone), `tools/tests/test_gate.py` (as decision 8 says) and the docs job
of `.github/workflows/checks.yml` (one step). Change nothing else: not `spec/`, `engine/`,
`conformance/`, `packages/`, `experiments/` or `docs/`, not `AGENTS.md`, `HOW-WE-WORK.md`,
`IN_FORCE.md`, `ROADMAP.md` or `CONTRIBUTING.md`, and no other tool. Put anything else that needs
changing in your report.

## Done when

Each line marked "fails today" prints otherwise on `main` at `59cf7ce9`; all pass after. Run
`git add -N` on the two new files before these, and `git reset -q` on them after; stage nothing.

```sh
test -f tools/check_decisions.py && test -f tools/tests/test_decisions.py; echo "files $?"  # files 0; fails today (files 1)
python3 tools/check_decisions.py HEAD WORKTREE >/dev/null; echo "exit $?"  # exit 0; fails today (exit 2)
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; fails today (prints nothing)
/usr/bin/python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0, under Python 3.9; fails today (prints nothing)
python3 tools/check_decisions.py HEAD WORKTREE | grep -c '^Framework: not in force'  # 1; fails today (0)
python3 tools/check_decisions.py HEAD WORKTREE | grep -cE '^(No-case|Not-done) ratchet: base [0-9]+, head [0-9]+'  # 2; fails today (0)
python3 tools/check_decisions.py HEAD WORKTREE | grep '^report: ' | grep -c 'AGENTS.md'  # 1 or more; fails today (0)
python3 tools/check_decisions.py HEAD WORKTREE | grep -c '^By review: [0-9]* of [0-9]* HOW-WE-WORK rule lines\.$'  # 1; fails today (0)
python3 -m unittest discover -s tools/tests -p test_decisions.py 2>&1 | tail -3 | grep -cE '^Ran [1-9]|^OK$'  # 2; fails today (0 or 1)
python3 -m unittest discover -s tools/tests -p test_decisions.py -v 2>&1 | grep -c ' \.\.\. ok$'  # 24 or more; fails today (0)
grep -oE 'def test_(coupling|changes_line|append_only|numbering|gone|third_copy|caps|form|orders|no_case|not_done|in_force)_' tools/tests/test_decisions.py | sort -u | wc -l | tr -d ' '  # 12; fails today (0)
grep -l check_decisions tools/gate.py tools/tests/test_gate.py .github/workflows/checks.yml | wc -l | tr -d ' '  # 3; fails today (0)
grep -cF "('decisions', 'python3 tools/check_decisions.py ' + self.base + ' WORKTREE')" tools/tests/test_gate.py  # 1; fails today (0)
grep -cF 'run: python3 tools/check_decisions.py HEAD^1 HEAD' .github/workflows/checks.yml  # 1; fails today (0)
```

These pass, as on `main`; run the whole tool suite once, last:

```sh
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD -- spec engine conformance packages experiments docs AGENTS.md HOW-WE-WORK.md IN_FORCE.md ROADMAP.md CONTRIBUTING.md | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- tools | grep -vxE 'tools/(check_decisions|gate)\.py|tools/tests/test_(decisions|gate)\.py' | wc -l | tr -d ' '  # 0
/usr/bin/python3 -m unittest discover -s tools/tests  # OK, test_gate.py among them
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives:
- per decision, what you built and the tests that show it;
- the real tree's `report:` lines, counted by rule, and its `Framework:`, ratchet, `By review:`
  and `Ratio:` lines;
- the corrected list of what this leaves;
- every choice you made yourself, under `own_choices`;
- every check's last line.

The verifier's turn follows; the coordinator then runs `tools/gate.py`, has a fresh, read-only
Claude reader read the diff ([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4),
and merges.

## Record

Not yet run.
