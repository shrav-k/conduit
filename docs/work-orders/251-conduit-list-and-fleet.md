# Work order 251: `conduit list`, `conduit fleet list` and `conduit fleet inspect`, ruling 60's spelling, after the pin

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports its tests bind. Every crate it builds is in the local cache already; if a build asks for one
that is not, stop and report, and fetch nothing. Run one cargo command at a time. Never print a
whole large file, diff, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential, even one a test made. Do not run `conduit` outside
a test, `codex`, `claude` or the dogfood harness, and sign in nowhere. Every file you write is
public: no absolute local path, no user or machine name and no private repository name.

## What and why

**The gap.** [Ruling 60](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-60/README.md)
spells the Fleet family's commands `conduit fleet ...`. Batch 1's order 051a was to build them with
`conduit list`
([the plan](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/README.md#the-work-orders);
[the Fleet area](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/spec-batch-1/areas/fleet.md),
section I), and was never written. On `main` the person reads `list.v1` only through
`conduit call management.list.v1`, one raw result, and no command shows a Fleet's Agents:
inspection holds no members (`inventory.v1`), and a Fleet's own `list.v1` is reached only by
`call`. IN_FORCE topic 9 still leaves the spelling to order 051a.

**This order is 051a, reduced.** It is the last of the S1 simplification chain (orders 240 to 251,
the owner's "Hold it; write the chain now" of 2026-11-21). What it no longer holds:
- *The cascades.* `spec/README.md`, "What the short form's meaning covers", now names them as
  plan and apply's flags `--with-resources` and `--with-agents`, which order 249 builds, so there
  is no `fleet retire --with-agents` and no `retire --with-resources` command.
- *Done already.* Inspection's scope and administrators (order 042, printed by `conduit inspect`),
  and the SPIFFE engine ID and token `sub` (`engine/crates/shell/src/identity.rs`).

**Off the rerun path.** Neither the baseline's tasks (experiment 007) nor K3's S1 schedule uses
`list` or `fleet`. It merges only after the rerun's pin is recorded: order 246's pack holds
`conduit --help` and each command's `--help` as at the pin, and this order edits `main.rs` and
`help.rs`, which orders 243 and 245 to 249 change.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
as existing practice: `docker network ls` and `docker network inspect NET`, whose inspection
lists the containers attached, and `kubectl get` and `kubectl describe`. One noun, two verbs,
output an agent can parse. `conduit fleet list` is `conduit list --kind Fleet` by one code path
and prints the same bytes, so the spelling the owner ruled adds no second format. Not taken:
- a `fleet retire` command, since spec/README makes the cascade a flag;
- `conduit pool list|inspect`, which no ruling names, and which `conduit list --kind Pool` and
  `conduit inspect` cover;
- a table or text format, since every command prints JSON lines (as `inspect` and `pending` do,
  `engine/README.md`);
- a count line, which OPS-036 keeps out of results.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves 2
in part, since the person sees each Fleet's Agents and the tombstones beside the Conduits that
inspection shows. None is at risk. Both commands send only `list.v1` and `inspect.v1` as root,
which `conduit call` reaches today. Listing grants no inspection (OPS-036), and nothing is
written. No promise, case or operation changes.

**When it runs.** After order 243 has merged (the start check says) and after the pin is
recorded (the coordinator checks), and once the owner has answered the chain's question 2 of
2026-11-21 (051a after the pin, its cascades as apply's flags) and its DEC-004 log entry is in.
"What it no longer holds" is that question's recommended answer; the coordinator amends it before
dispatch if the answer differs. Through Conduit's dogfood: one Codex worker turn (4-hour limit;
aim for under 1.5 hours; the whole suite once, at the end), a Codex verifier and a Claude reader,
who run only what settles a doubt, and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
each decision says what must hold; how is yours. The line keys, the test names and the fixture
are the coordinator's, open to the owner's strike.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a start-check line prints otherwise;
- a correct result needs a change outside the scope: the core, the loader, `conformance/`,
  `packages/`, or the behavior of order 243's context function (adding the two commands to its
  callers is in scope);
- root's Fleet tool is not `NAME.list.v1` for a Fleet that FLEET-005's default serves: report the
  tool the engine presents, and do not read it from the store's files;
- a crate is missing from the cache, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **`conduit list [--kind KIND]`** (OPS-036, ACC-033 row 15). Through `serve`, as root, send
   `management.list.v1` with `{"kind": KIND}` where `--kind` is given and `{}` otherwise. The
   engine checks KIND against `list.v1`'s input schema; the command does not. Standard output is
   one JSON line per item, in the order the result gives them:
   - `{"object": ITEM}` for each item of `objects`;
   - then `{"package": ITEM}` for each of `packages`;
   - then `{"tombstone": ITEM}` for each of `tombstones`.

   Each ITEM is as the result gives it, and nothing else goes to standard output: no `pending`
   (that is `conduit pending`'s) and no count. Where the three are empty, one stderr line says that
   root may list no such item, and the command exits 0. Where the result does not succeed, it is
   printed as one line, as `call` prints it; one stderr line names `management.list.v1`, and the
   command exits 1. `serve` must hold the store, as for `pending`; `list` never opens the store
   itself.
2. **`conduit fleet list`.** The request and the lines of `conduit list --kind Fleet`, by the same
   code. It takes no other argument. With a kind, the engine still lists every installed package,
   so `fleet list` prints those package lines too.
3. **`conduit fleet inspect NAME`.**
   - First `management.inspect.v1` with `{"object":{"kind":"Fleet","name":NAME}}` (ACC-033 row 7).
     Print the row that `conduit inspect` prints for that Fleet, by the function it uses, from
     the same snapshot it reads. Only `result._meta` and the result's `records` differ from that
     row, per request.
   - Where that succeeded and the Fleet is not retired, send `NAME.list.v1 {}`. That is root's
     default Conduit to the Fleet, whose local name FLEET-005 fixes as the Fleet's name; IFC-012
     forms the tool's name. Print one `{"agent": ITEM}` line per item of `objects`; through a
     Fleet, OPS-036 lists only its Agents. The tool is not looked up in the store's files, as
     order 245 decides for `plan`'s routes.
   - Where the inspection shows the Fleet retired, send no list. ACC-033 authorizes no request
     against a retired Fleet, and CON-008 has closed root's default. One stderr line says the Fleet
     is retired and that its remaining Agents are listed through `management`, by
     `conduit list --kind Agent`. The command exits 0.
   - Where the inspection or the list does not succeed, print its result as one line. One stderr
     line names the tool and `Fleet/NAME`, and the command exits 1. After a failed inspection, no
     list is sent.
   - The two reads are not atomic with concurrent changes, as `conduit inspect` says of its own.
4. **Arguments and the engine context.** `list`, `fleet list` and `fleet inspect` take
   `--store DIR`. Without it they use the context through order 243's one function, as its
   decision 3's commands do, with its stderr line and its status 2 for a stale context. `--kind` is
   `list`'s alone and is given at most once. `conduit fleet` exits 1 and sends nothing where its
   subcommand is missing or unknown, `retire` among them, or where its operands are wrong. Its one
   stderr line names `list` and `inspect NAME`, and for `retire` adds that a Fleet's cascade is the
   flag `--with-agents` that spec/README.md names for plan and apply.
5. **Help and the engine README.** `help.rs` gains a `list` row and a `fleet` row (`list | inspect
   NAME`), each with an example, and the usage line names both. `engine/README.md` gains a
   paragraph after `conduit pending`'s with decisions 1 to 4 in a few lines. In `tests/cli.rs`,
   `help_describes_every_command_without_store_access` gains the verbs `list` and `fleet`.
6. **IN_FORCE topic 9.** In the **In spec/** line on the family and kind, the sentence on the CLI
   spelling `conduit fleet ...` (on `main` it leaves the spelling to order 051a; order 250 makes it
   name order 251) becomes: "The CLI spelling: `conduit fleet list` and `conduit fleet inspect
   NAME`, landed by order 251; a Fleet's cascade is the plan-and-apply flag `--with-agents` in
   spec/README.md." No other line changes, and no bullet is added.
7. **The tests**, in a new `engine/crates/shell/tests/list.rs`, against the binary. Each test has
   its own temporary store, port and `CONDUIT_CONFIG_DIR` (move the helpers it shares with
   `tests/context.rs` or `tests/serve.rs` into `tests/common/mod.rs`, or copy them; either is in
   scope). The fixture is a project like `serve_failed_setups`' in `tests/serve.rs`, without its
   setup hook: a Pool `workspace` with Resources `kept` and `spare`, a Fleet `crew` with Agent
   `worker`, and a Fleet `empty` with no Agent. Install
   `conformance/fixtures/install-archives/filesystem-0.1.1.tar.gz` (its digest is in
   `digests.json`), then retire `Resource/spare` and `Fleet/empty`, each by
   `conduit retire` (LIF-040's tombstones). Use another archive of that directory that the engine
   accepts offline only if that one fails, and name it. Each test compares with `conduit call`,
   sent with nothing changing in between. The tests, at least these and by these names:
   - `list_prints_each_item_root_may_list`. `list`, with no option and with `--kind Agent`,
     prints exactly the lines decision 1 forms from `call management.list.v1` with the same
     input. With no option that is at least one object, one package and one tombstone line.
     `--kind Agent` prints `worker` and no Fleet line, and still prints the package line; no line
     holds `pending`. `--kind Nope` exits 1, printing the result, with stderr naming
     `management.list.v1`.
   - `fleet_list_and_inspect_show_a_fleet_and_its_agents`. `fleet list` prints the same bytes as
     `list --kind Fleet`. For `fleet inspect crew`, the first line equals `conduit inspect`'s
     row for `Fleet/crew` as JSON once `result._meta` and `result.structuredContent.records` are
     removed from both, since each request has its own invocation and record. Then comes
     `{"agent": ITEM}` for each object of `call crew.list.v1 {}`: `worker`, then nothing else; it
     exits 0. For `fleet inspect empty`, its `result.structuredContent.status.state` is
     `retired`, no `agent` line follows, the stderr line of decision 3 is printed, and it exits 0.
     `fleet inspect nobody` exits 1 and names `Fleet/nobody`. `fleet`, `fleet retire crew` and
     `fleet inspect` with no name each exit 1 with decision 4's line, and leave `conduit records`
     with no new line.
   - `list_and_fleet_use_the_engine_context`. With `serve` recording the context, bare `list` and
     `fleet list` print the same stdout as with `--store`, plus order 243's stderr line. Once that
     `serve` has ended, bare `list` exits 2 with order 243's line.

## What to read

- OPS-036 in `spec/conventions/operations.yaml`; ACC-033 rows 6A, 7 and 15 in
  `spec/conventions/access.yaml`; FLEET-005 in `spec/promises/fleets.yaml`; CON-008 and LIF-040;
  `list.v1` and `inspect.v1` in `spec/core/operations.yaml`; `spec/README.md`, "Fleets, discovery
  and tombstones".
- Order 243's text and its context function, as merged.
- In `engine/crates/shell/`: `src/main.rs` (the arguments and the group sent to `author::run`),
  `src/help.rs`, `src/author.rs` (`run`, `list_pending`, `inspection_row`, `print_inspection_row`,
  `ensure_success`), `tests/serve.rs` (`serve_failed_setups`, `start_server`), `tests/cli.rs` (the
  help test) and `tests/context.rs` (order 243's helpers).

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor cc3605a3 HEAD; echo "main $?"
test -e engine/crates/shell/tests/context.rs; echo "243 $?"
cat engine/crates/shell/src/*.rs | grep -c 'CONDUIT_CONFIG_DIR'
grep -cE '"(list|fleet)",' engine/crates/shell/src/help.rs
test -e engine/crates/shell/tests/list.rs; echo "list $?"
grep -cF '`conduit fleet ...`' IN_FORCE.md
test -e "$HOME/.config/conduit"; echo "home $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, then `243 0` and a count of `1` or more once order 243 has merged (`243 1`
and `0` on `main` at `cc3605a3`: stop, since this order needs it). Then `0`, `list 1`, `1`,
`home 1`, `engine 0` and `0`. `engine` printing otherwise means a crate is missing from the cache:
stop and report, and fetch nothing. `home 1`: a test that forgets its `CONDUIT_CONFIG_DIR` would
write your home, and the last check looks again.

## Scope

Change `engine/crates/shell/src/` (order 243's context function only to name the two
commands), `engine/crates/shell/tests/` (the new `list.rs`, `cli.rs` for decision 5's two verbs,
and the shared helpers of decision 7), `engine/README.md` for decision 5, `IN_FORCE.md` for
decision 6 alone, and `engine/STANDING.md` only by `promise_standing.py --write` if its check asks.
Change nothing else: not `spec/`, `conformance/`, `packages/`, `docs/`, `engine/CLI.md` (order
246's reference, in the rerun's pack), the loader, the core, `Cargo.toml` or `Cargo.lock`. Put
anything else that needs changing in your report.

## Done when

Write decision 7's tests first and run them on the unchanged code: report each failing line. Then
change the code until all pass. Each line in the first block prints otherwise on `main` at
`cc3605a3` with order 243 merged ("fails today"), and passes after.

```sh
t() { (cd engine && cargo test --release -p conduit-shell --test list "$1" 2>&1 | grep -c '^test result: ok\. 1 passed'); }
t list_prints_each_item_root_may_list                  # 1; fails today (0: no such target)
t fleet_list_and_inspect_show_a_fleet_and_its_agents   # 1; fails today (0)
t list_and_fleet_use_the_engine_context                # 1; fails today (0)
grep -cE '^\s+"(list|fleet)",$' engine/crates/shell/tests/cli.rs   # 2; fails today (0)
grep -c 'conduit fleet inspect' engine/README.md                   # 1 or more; fails today (0)
grep -c 'landed by order 251' IN_FORCE.md                          # 1; fails today (0)
```

These pass, as on `main`; run the whole suite once, last, then look at your home again:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy -p conduit-shell --all-targets -- -D warnings)
(cd engine && cargo test --release -p conduit-shell --test cli help_describes_every_command_without_store_access)  # 1 passed
git add -N engine                                   # so the checks see the new file
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1   # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py                               # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py                      # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check                 # Errors: 0
git diff --check                                    # prints nothing
git diff --name-only HEAD | grep -vE '^engine/crates/shell/(src|tests)/|^engine/(README|STANDING)\.md$|^IN_FORCE\.md$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite; once, last
test -e "$HOME/.config/conduit"; echo "home $?"     # home 1: no test wrote your context
git reset -q                                        # stage nothing
```

Your report gives, per decision, what changed; each new test's failing line, then its passing
line; the archive the fixture installed; every choice you made yourself, under `own_choices`; and
every check's last line. Stage nothing. The verifier's turn follows; the coordinator then runs
`tools/gate.py`, has a fresh, read-only Claude reader read the diff
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md);
ruling 92 point 4), and merges.

## What follows

Named here and not in this order:

- *A retired Fleet's remaining Agents.* `list.v1`'s items carry no owner, so after its Fleet
  retires, `management` lists them among every Agent. Listing them by their old Fleet would need
  the owner in the item, a spec change for the owner to rule on if it is wanted.
- *The CLI reference.* Order 246's `engine/CLI.md` names neither command, so that the pinned
  rerun's pack stays as measured. They can be added after the rerun.
- *Pools.* `conduit pool ...`, only if the owner asks for it.
- *Packages under a kind.* Whether OPS-036's `kind` narrows installed packages: a spec question
  for the owner.

## Record

Not yet run.
