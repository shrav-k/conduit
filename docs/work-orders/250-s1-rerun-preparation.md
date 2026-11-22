# Work order 250: the S1 rerun's preparation: T1 to T7 in the short form on the release binary, the example and the author's README on plan and apply, the markers and the pin checklist

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`; leave
`vectors` empty. This order needs no network: its only sockets are its tests' loopback ports; if a
build asks for a crate not in the cache, stop and report. Run one cargo command at a time. Never
print a whole large file, diff, log or list. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential. Do not run `codex`, `claude`, `npm`, the dogfood harness or `tools/gate.py`, nor
`conduit` outside a test but for the `--help` lines under "Done when", and sign in nowhere. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

**The git rule.** On this repository run only `git merge-base`, `git log`, `git show`, `git grep`,
`git status`, `git diff`, `git add -N` and `git reset -q`.

## What and why

**The gap.** v0's author bar is S1's rerun against A70
([IN_FORCE.md, topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement)),
registered to measure the short form, plan and apply, `init`'s skeleton and `install`'s pins. On
2026-11-21 none was built; the owner held the rerun ("Hold it; write the chain now"), to go on a
new pin from `main`. Orders 240 to 249 and 263 build them. This last order, batch 1's 053
([the plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders)),
shows that [S1's tasks](../implementation/FIRST_BUILD.md#the-tasks) run as an author runs them,
and readies the repository for the pin. The earlier orders run
[K3's cases](../implementation/plan-and-apply/README.md#s1-commands-and-counts-under-ruling-76)
on the `example.org/s1/agents` fixture, which raises both sign-ins and exports `turn`. The rerun's
real Agent packages require `account`, state `signIn` and, since 0.2.2, export `turn` too.

**Corrections to the chain's plan, at `cc3605a3`:** the guide is `engine/CLI.md` (order 246); the
Agent contract fixtures hold no executable, so scripted Agents take the wrapper's place; the case
`live-agents` and `tests/agents.rs` need the framed `.v1` project (the wire rejects sectioned
values, SUB-005), so "agents.rs follows" means both keep the framed files, moved; K3's 18 safety
cases (the full review's D26) are in no chain order; decision 6 maps their tests; IN_FORCE topic 4
misses order 217's cut of SCT-012.

**The owner's item 1(c).** Root's turn Conduits inline, or from an Agent release exporting `turn`.
Answer: templates in 0.2.2 (DEC-040, order 253).

**The owner's item 2.** 051a's `conduit fleet` and `list` move after the pin, as order 251, its
cascades are 249's flags, and `render` is dropped (Rec.). Decisions 4 and 5 cite the log's entry
under DEC-004 where one exists, else mark both as the coordinator's.
Answer 2: agree.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves
2 (T7's close and `--reopen`), 4 (both vendors' Types), 6 (no turn but root's calls; apply answers
no sign-in; a stopped plan sends nothing) and 7 (each Agent's own identity). None at risk; no
promise or case changes. Rule 8: refusals and a close are exercised.

**Should this exist** (HOW-WE-WORK rule 3). Yes: an end-to-end test of the release build before a
tag (Kubernetes' e2e conformance, Terraform's acceptance tests) and a release checklist naming
artifacts by digest. Not taken: real clients; building the Agent packages; scoring S1 in a test.

**When it runs.** After orders 240 to 249 and 263, through Conduit's dogfood: one Codex worker
turn (4-hour limit), a Codex verifier, a Claude reader and the gate
([DEC-030](../decisions/log.md#dec-030-2026-11-20-how-an-order-runs-and-merges)). By ruling 89, each
decision says what must hold; how is yours. Names and forms are the coordinator's, open to the
owner's strike, as is this: if 259 or 264 merged first, that Agent is 0.2.3, still `.v2` with
`account`.

## The decisions

1. **The test**, `engine/crates/shell/tests/s1_short_form.rs`, one test
   `s1_short_form_t1_to_t7_through_init_install_and_apply`, not ignored. Under `--release`,
   `CARGO_BIN_EXE_conduit` is the release binary, and the test drives only it, as a person would.
   - *Isolation.* Every `conduit` it runs gets order 243's context and cache variables under its
     temporary directory, never the person's (`config.json` too). The store and `serve` on a
     loopback port are harness.
   - *Archives.* It writes scripted variants of `conformance/fixtures/`'s filesystem 0.1.1 and
     0.2.1 (`install-archives/`), filesystem 0.1.0 and history (`s1/archives/`), and both
     `agents/*/contracts.tar.gz`: each implementation item's npm `package` or `command` runs this
     test binary as a scripted server, history's dependency pin names the scripted 0.1.0, and
     every other byte is kept; its own mirror, in the cache's layout, holds the scripted 0.1.0
     by digest (as `s1/mirror/` holds `200de7c1…`). Reuse 241's and 247's scripted filesystem;
     `scripted_package` in `tests/agents.rs` shows the rewrite. First assert, as that test does,
     that each Agent fixture's YAML files equal the package's.
   - *The scripted Agent* serves the `wrapper-v2` item's tools with outputs its contracts accept.
     `account` answers an account the test controls; `turn` takes a message naming one engine
     tool and its arguments, calls it at `CONDUIT_URL` with the run's identity
     (`CONDUIT_IDENTITY_DIR`, as the wrapper's `header` reads it) and returns the reply.
   - *The tasks*, from an empty project directory, by K3's S1 schedule as 263 left it. T1: `init`,
     three `install`s (K3 lists one Agent archive, the fixture's `agents-0.2.1`; the report gives
     T1's primary and strict counts both ways, and K3's schedule stays: the owner's, ruling 71 pt
     4), then the authored file, `engine/examples/s1/s1.yaml` with its root path replaced, then
     `apply`. With no account set, apply prints one `panel action ID: sign-in required` line per
     Agent and exits 0; the test sets the account and runs `conduit sign-in Agent/agent-a` and
     `Agent/agent-b` with standard input closed (panel actions), each exiting 0; `conduit.yaml` then
     equals the example's (263's pin form) but for the rewritten digests. T2's one line installs
     filesystem 0.1.0 from the mirror, then history, adding history's pin alone (filesystem's stays
     0.1.1); T5's prints `pin moved up`. At T4, and at T7's restore, bare `conduit apply` first
     exits 2 with `requires=--replace agent-a--project`, or `--reopen`, and sends nothing (a check,
     not counted); the flagged apply then exits 0.
   - *Checks after each task:* `conduit plan` prints no step and exits 0; `conduit tools --as`
     lists exactly each Agent's tools; root's include `agent-a.turn.v1` and `agent-b.turn.v1`.
     Through root's turn calls each granted use works and each withheld one is refused: T1 A's
     write refused, B's accepted; T2 both post and read history; T4 the reverse; T7 A's `project`
     refused while its history and B's project work, then restored. `conduit inspect` shows
     `shared` on 0.2.1 at T5 and on its new root at T6. A stopped command leaves the record count
     unchanged. No printed line holds `not built`, `unbuilt`, `043a`, `052a` or `order 24`. The
     test prints, per task, its commands and the authored lines it changed.

2. **The example.** `engine/examples/s1` becomes S1 after T1 in the short form, three files.
   `conduit.yaml` is as `init` and 263's `install` write it, a pin a line (`ADDRESS: {version: V,
   digest: 'sha256:HEX'}`): filesystem 0.1.1 (`sha256:eaa13cfe…`) and both Agents at their
   contract fixtures' versions and digests, as today. `s1.yaml`, in flow style: `fleet: s1`; Pool
   `team`; Resource `shared` (`local-directory.v1`, root `/tmp/conduit-s1/shared`); `agent-a`
   (`claude-code.v2`) and `agent-b` (`codex.v2`), each with `config: {account: …}` alone, a
   `project` grant to `shared` from `example.org/s1/filesystem#read` or `#read-write`, and
   `operator: {from: ADDRESS#turn}`, ADDRESS its package's. `README.md`: what it is, that
   decision 1's test runs it, how a person runs it live (host Agent archives built offline),
   starting with `conduit init` in an empty directory and copying `s1.yaml` alone, since `install`
   writes each pin exactly from the archive given, and that it is not in the fresh author's pack.
   The framed files move byte for byte to `conformance/fixtures/s1/live-project/`, read there by
   `live-agents/case.yaml` and `tests/agents.rs`; `conformance/fixtures/s1/README.md` says which is
   which. If `tools/vector_delta.py` reports the moved inputs, declare them, kind `input`, in
   `docs/work-orders/250-declared.yaml`; it reports no flip.

3. **The author's texts.** `engine/README.md`'s live S1 paragraph, its walk-through under "The
   shell's store, runner and command line" and "Change a serving project" give the path: the short
   form, init, serve, install (T2's one `--mirror` line), apply, plan optional. They cite
   `engine/CLI.md` for commands, flags, statuses and kept state (rule 7); `register`, `submit` and
   `replace` stay, for either form (SCT-003), without a plan; "Runnable Agent packages" names
   `live-project/`. `conduit --help` names `engine/CLI.md` in one line. In `engine/CLI.md` change
   only a `not yet built` sentence left, a usage line that differs from the final `--help`, and S1
   content 246's check finds.

4. **IN_FORCE.md**, lines edited, none added. Topic 4: `*Spec pending, batch 1*:` becomes `Batch 1,
   all in spec/:`, and "(SCT-010 unused)" becomes "(SCT-010 unused; SCT-012 cut by order 217)".
   Topic 9: "The CLI spelling ... remains for order 051a." becomes "*Coordinator's*, open to the
   owner's strike: the CLI spelling `conduit fleet ...` is order 251's, after the pin." Topic 13:
   the pin line adds that on 2026-11-21 the owner held the rerun for a new pin from `main` after the
   S1 chain (the owner's words name 043a, 043b, 051a, 052a, 052b and 053), citing the log's entry
   where one exists; orders 240 to 250 and 263 build it, and 051a's rest as order 251 after the pin
   is *Coordinator's*; the fresh author's line names `engine/CLI.md` and the measurement plan's
   pack.

5. **The handover.** Row 9 says, from `git log` at your base, which of its orders merged, that the
   chain, orders 240 to 250, did 043a, 043b, 052a, 052b and 053, and that `render` and 043b's
   `manages` half were dropped; next, order 251, after the pin (the coordinator's, open to the
   owner's strike, as is `render`'s drop). Row 12: order 250 merged; next, the pin from `main` by
   the measurement plan's pin checklist, at or after the second-account sitting (the coordinator's
   reading), the rerun with the owner present, the owner's judgment.

6. **K3's README**, these sentences alone: "have no case file and no order" names instead 248's
   `tests/apply_replace.rs` and 249's prefixes `plan_apply_` plus `prune_`, `adopt_`,
   `with_resources_` or `with_agents_`; each of the 18 safety rows a test at your base shows ends
   "Test: `NAME`.", the others unchanged; the cascade pattern gains the `affects` lines 249's
   cascade test prints; "Which form ..., for order 250." becomes "By reference (DEC-040)."

7. **The pin checklist**, a new last subsection `### The pin checklist` of
   [The S1 rerun](../implementation/first-build/measurement-plan.md#the-s1-rerun), after 246's
   pack, with full digests: the pin, a commit of `main` at or after this merge, and its binary
   built offline, with its SHA-256 and `conduit --version`; the Agent archives at the example's
   pins, built in each Agent package by `CARGO_NET_OFFLINE=true cargo run --release --locked
   --offline --bin package` (the inner build has no `--offline`), their digests recorded and their
   YAML files equal to the contract fixtures'; for the pack, each archive the tasks install by
   path, version and digest: `install-archives/filesystem-0.1.1.tar.gz` at T1, history at T2 with
   `s1/mirror/` holding `200de7c1…`, `filesystem-0.2.1.tar.gz` (`sha256:f01c1a0a…`) at T5; npm's
   2.1.0 and 2.2.0 tarballs fetched or seeded, as harness; 243's context variable set per lab.

8. **T2's two filesystem versions.** Where history's filesystem 0.1.0 and the project's 0.1.1 do not
   both resolve at T2, stop as the stop rule says, citing NAM-011 and NAM-017: the fix, an engine
   order or a history 0.1.1 fixture that moves T2's count, is the owner's (ruling 71 pt 4).

## What to read

K3 and its cases; SCT-006, -007, -009, -011, -013; spec/README's "What the short form's meaning
covers"; PND-005, `sign-in.v1`, NAM-011, NAM-017; `tests/agents.rs` and 241's and 247's harness;
the shell's init, install, plan, apply and sign-in; the `.v2` contracts and the wrapper's `header`.

## Before you change anything

From the repository root; stop, change nothing and report if a line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
for n in $(seq 240 249) 263; do git log --oneline HEAD | grep -qE "Work order $n: .*built through Conduit" || echo "missing $n"; done | wc -l | tr -d ' '
grep -oE -- '--(reopen|recreate|prune|replace|adopt|with-resources|with-agents)' engine/crates/shell/src/help.rs | sort -u | wc -l | tr -d ' '
test -f engine/CLI.md; echo "guide $?"
ls engine/examples/s1 | LC_ALL=C sort | tr '\n' ' '; echo
grep -c 'engine/examples/s1/' conformance/language/registration/live-agents/case.yaml
test -e engine/crates/shell/tests/s1_short_form.rs; echo "new $?"
grep -c '^Answer: templates in 0\.2\.2 (DEC-040, order 253)\.$' docs/work-orders/250-s1-rerun-preparation.md
grep -cE '^Answer 2: (none yet|agree)\.$' docs/work-orders/250-s1-rerun-preparation.md
(cd engine && h=$(rustc -vV | sed -n 's/^host: //p') && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`; `0` once the builds of 240 to 249 and 263 have merged (`11` on
`cc3605a3`; the coordinator corrects the line if one merged under another subject); `7` after 248
and 249 (`0`); `guide 0` after 246 (`guide 1`); `README.md conduit.yaml instances.yaml
templates.yaml`, `3` and `new 1`; `1` (with `0`, stop: the answer changed);
`1` (with `0`, stop: a strike changes decisions 4 and 5); and `engine 0`, else a crate is missing:
stop.

## Scope

The files the decisions name, and `engine/crates/shell/tests/plan_apply_support/`. Nothing else:
not `spec/`, the core, the loader's source, the shell's source but `help.rs`, `packages/`, K3's
cases, the decision log, any `Cargo.toml` or `Cargo.lock`. Report anything else.

## Done when

Write decision 1's test first, run it on the unchanged tree and report its failing line. Each line
here prints otherwise on `cc3605a3` ("fails today") and passes after:

```sh
M=docs/implementation/first-build/measurement-plan.md; K=docs/implementation/plan-and-apply/README.md; H=docs/implementation/HANDOVER.md; G=engine/CLI.md
(cd engine && cargo test --release -p conduit-shell --test s1_short_form 2>&1 | grep -cE '^test s1_short_form_[a-z0-9_]+ \.\.\. ok$')  # 1; fails today (0)
ls engine/examples/s1 | LC_ALL=C sort | tr '\n' ' '; echo  # README.md conduit.yaml s1.yaml; fails today
cat engine/examples/s1/*.yaml | grep -c '^kind:'  # 0; fails today (20)
grep -oE 'agent-(claude-code#claude-code|codex#codex)\.v2|operator: \{' engine/examples/s1/s1.yaml | wc -l | tr -d ' '  # 4; fails today (0)
for f in conduit.yaml templates.yaml instances.yaml; do git show HEAD:engine/examples/s1/$f | cmp -s - conformance/fixtures/s1/live-project/$f || echo "moved $f"; done | wc -l | tr -d ' '  # 0; fails today (3)
cat conformance/language/registration/live-agents/case.yaml engine/crates/shell/tests/agents.rs | grep -c 'engine/examples/s1'  # 0; fails today (4)
(cd engine && cargo build --release -p conduit-shell --bin conduit 2>&1 | tail -1); engine/target/release/conduit --help | grep -c 'engine/CLI\.md'  # 1; fails today (0)
for c in init install plan apply; do u=$(engine/target/release/conduit $c --help 2>/dev/null | head -1); test -n "$u" && grep -qF -- "$u" $G 2>/dev/null || echo "usage $c"; done | wc -l | tr -d ' '  # 0; fails today (4)
git grep -nIiE 'not (yet )?built|043a|043b|052a|052b|order 051a|order 053' -- engine/README.md $G engine/examples/s1 engine/crates/loader engine/crates/shell/src/help.rs | wc -l | tr -d ' '  # 0; fails today (1)
grep -c 'conduit apply' engine/README.md  # 2 or more; fails today (0)
grep -c 'order 051a' IN_FORCE.md  # 0; fails today (2)
sed -n '/^## 4\./,/^## 5\./p' IN_FORCE.md | grep -c 'Spec pending'  # 0; fails today (2)
grep -c 'SCT-012 cut by order 217' IN_FORCE.md  # 1; fails today (0)
sed -n '/^## 13\./,/^## 14\./p' IN_FORCE.md | grep -cE 'engine/CLI\.md|orders 240 to 250'  # 2; fails today (0)
grep -cE '^\| 9 \|.*order 251|^\| 12 \|.*pin checklist' $H  # 2; fails today (0)
grep -c 'Test: `' $K  # 1 or more; fails today (0)
grep -cE 'no case file and no order|for order 250' $K  # 0; fails today (1)
sed -n '/^### The pin checklist$/,/^## /p' $M | grep -oE 'eaa13cfe|200de7c1|f01c1a0a|CARGO_NET_OFFLINE=true' | sort -u | wc -l | tr -d ' '  # 4; fails today (0)
```

Also, order 246's `clean` line, from its Done when, prints `clean 0` on `engine/CLI.md` (`clean 1`
today).

These pass, as on the base; run the engine's whole suite once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
git add -N engine conformance docs/work-orders  # so the checks see new files
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE $(test -f docs/work-orders/250-declared.yaml && echo --declared docs/work-orders/250-declared.yaml) >/dev/null; echo "delta $?"  # delta 0, no flip
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/set_names.py --check | tail -1  # ends Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(engine/(crates/shell/(tests/(s1_short_form\.rs|agents\.rs|common/|plan_apply_support/)|src/help\.rs)|examples/s1/|README\.md|CLI\.md)|conformance/(fixtures/s1/|language/registration/live-agents/case\.yaml)|IN_FORCE\.md|docs/implementation/(HANDOVER|first-build/measurement-plan|plan-and-apply/README)\.md|docs/work-orders/250-declared\.yaml)' | wc -l | tr -d ' '  # 0
git reset -q  # stage nothing
(cd engine && cargo test --release)  # every test passes
(cd engine && cargo run --release -p conduit-conformance)  # every case passes, live-agents among them
```

Your report gives per decision what changed; T1's apply output; per task, its commands and
authored lines beside K3's schedule; the helper you reused; and the safety rows with no test.

## Stop rule

Stop, change nothing further, and report the gap and the options, without choosing, where a
start-check line prints otherwise; a task fails or prints otherwise than K3 and these decisions
expect because of the shell or the engine (keep the test failing there, report the command, its
output and the K3 row or promise, and fix no shell or engine source, ruling 87); a scripted Agent
cannot serve a real contract unchanged; K3 and spec/README disagree on a step the test sends; or a
result needs a change outside the scope or the network. A choice no decision governs is yours: list
it under `own_choices`.

## What this leaves

The scored rerun on the pin, with the owner present; order 251 after the pin; K3's sections citing
the guide; the safety rows with no test, the owner's.

## Record

Not yet run.
