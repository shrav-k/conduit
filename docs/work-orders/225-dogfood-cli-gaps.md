# Work order 225: the dogfood's command-line gaps G-2 and G-3: arguments from a file, `--version`, `pending --agent`, `inspect --setup`, serve's ready line

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate, and the only sockets it opens are the loopback
ports that the engine's own tests bind. Every crate it builds is in the local cache already; if a
build asks for one that is not, stop and report, and fetch nothing. Run one cargo command at a
time. Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n`
or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run `conduit`
against a store outside a test, `codex`, `claude` or the dogfood harness, and sign in nowhere.

## What and why

**The gaps** (the dogfood-gaps stream, steps G-2 and G-3; [ruling 92](../proposals/ruling-92/README.md),
point 9; the owner approved the stream's list as written,
[DEC-027](../decisions/log.md#dec-027-2026-11-20-conduit-builds-conduit-directly-through-conduit)).
The dogfood runs Conduit's own work orders by `conduit call --store DIR TOOL ARGS-JSON` of root's
turn tools while `conduit serve` holds the store. It found five gaps in the command line:

- a turn's arguments, a JSON message of 20 to 40 KB, can be passed only inline on the command
  line, where `ps` shows them;
- nothing says which build is running: there is no `--version`, and no commit;
- `conduit pending` lists every action raised at root, with no way to ask for one Agent's;
- no command shows each instance's last setup outcome in one line: `inspect` prints each
  object's whole result, every record that names it included;
- `serve` prints nothing when it is ready, so the coordinator polls `conduit tools` until it
  answers.

G-2 is the first three; G-3 the last two.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
7, everything an Agent does through Conduit is attributable: a turn's result can be tied to the
build that served it, and an action to the Agent it waits on; and 2, every access can be
inspected: the setup view is root's `inspect.v1`, narrowed.

**Should this exist** (point 3). Each is an existing command-line paradigm:

- arguments from a file, `-` for standard input: `gh api --input FILE`, `kubectl apply -f -`;
- a version with its commit: `cargo --version` and `rustc --version`;
- a filter on a list: `gh pr list --author`, `kubectl get --field-selector`;
- a one-line view beside a full one: `kubectl get` beside `kubectl describe`;
- a line on standard output once serving: `kubectl proxy`'s "Starting to serve on".

Two routes are not taken (the coordinator's, open to the owner's strike):

- `jq` over the output of `pending` and `inspect`. It would make an unknown Agent's name read as
  "no action" (ruling 52), and the dogfood's harness is Rust (point 2).
- A new MCP operation for any of them. Each is the client's work over operations that exist, and
  an operation would change `spec/`, which is the owner's.

**When it runs.** An order of A78's stage 2, sent through Conduit under
[order 211's registration](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/211-a78-stage-2-registration.md): one Codex worker turn (a 4-hour
limit; aim for under 2 hours), a Codex verifier and a Claude reader, who run only what settles a
doubt, and the gate, which runs the whole suite
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). Order 219 changes the same
files (`main.rs`, `author.rs`, `help.rs`, `server.rs`, `tests/serve.rs`, `engine/README.md`);
where your base holds it, keep what it built, and change nothing it decided. The harness takes
these up afterwards, by the coordinator, not here.

## The decisions it applies

By [ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how the
shell does it is yours. The names and line forms are the coordinator's, open to the owner's strike.

1. **Arguments from a file.** `conduit call --store DIR TOOL --arguments-file PATH` reads the
   arguments from PATH, and `--arguments-file -` from standard input to its end. The JSON read
   goes where ARGUMENTS-JSON goes today, with the same checks and the same result. Inline
   ARGUMENTS-JSON is unchanged. Where both are given, PATH cannot be read, or its content is not
   valid JSON, the command sends nothing, prints nothing on stdout and exits nonzero. Its stderr
   names the cause and the path (`standard input` for `-`) and, for a parse error, its line and
   column, but no byte of the content. The option is `call`'s alone; for any other command it
   stays an unknown argument. The name follows the help's ARGUMENTS-JSON and the CLI's
   whole-word options.
2. **The version.** `conduit --version`, given alone, prints one line on stdout and exits 0. It
   opens no store and needs no `--store`. The line is `conduit VERSION (commit HASH)`:
   - VERSION is the `conduit-shell` crate's version (`0.1.0` today).
   - HASH is the full 40-character commit that `git rev-parse HEAD` named in the repository at
     the binary's build, or `unknown` where the build found no commit (a copy without Git, or no
     `git`). Neither case fails the build.
   - The build asks Git again when HEAD, or the branch it names (packed or not), moves, so a
     binary built after a commit names that commit. It does not ask on every build. Find these
     files with `git rev-parse --git-path`, since `.git` may be a file (a linked worktree), and
     name in `rerun-if-changed` only paths that exist: Cargo reruns a script every build for a
     missing one.
   - Changes not committed are not shown, and the help says so. A `-dirty` mark is not made: it
     could not be kept true without asking Git again on every change.

   Cargo's build script and `std::process::Command` need no new crate.
3. **Pending by Agent.** `conduit pending --store DIR --agent NAME` prints those of the lines
   `conduit pending` prints whose `subject` holds the Agent NAME (an entry with kind `Agent` and
   name NAME). Each line is byte for byte as unfiltered, in the same order.
   - The filter is the CLI's, over the one `management.list.v1` reply that `pending` reads today:
     nothing more is sent, and `list.v1`'s arguments do not change.
   - Whether NAME is an Agent instance is read from that reply's `objects`, not from the store
     file. Where it is not, the command prints nothing on stdout, exits nonzero, and its stderr
     names NAME and says that root sees no Agent of that name.
   - Where the Agent exists and no action names it, stdout is empty, stderr says so naming the
     Agent, and the exit is 0, as `pending` does today when none is raised.

   Without `--agent`, `pending` is unchanged, and so is `answer`.
4. **The setup view.** `conduit inspect --store DIR --setup` prints one JSON line per instance
   of kind Interface, Pool, Fleet, Resource or Agent, the kinds that serve's startup sets up. It
   prints none for a template or a Conduit, in the order `inspect` prints them. Each line is:

   ```json
   {"object":{"kind":"Agent","name":"a"},"state":"registered","setup":{"outcome":"failed","invocation":"ID","notSucceeded":[{"name":"run","outcome":"failed"}]}}
   ```

   - `state` and `setup` come from that instance's `inspect.v1` result,
     `structuredContent.status`; its `setup` is what LIF-004 has the status report (`settle` in
     `engine/crates/core/src/lifecycle.rs`).
   - `notSucceeded` holds `{name, outcome}` for each part of that setup whose outcome is not
     `succeeded`, and is empty when there is none. `"setup":null` where the status holds no setup.
   - Where an instance's inspection does not succeed, its line is
     `{"object":{...},"result":REPLY}`, and the command exits nonzero, as `inspect` does.
   - It runs where `inspect` runs: through serve while serve holds the store, and on the store
     otherwise. Both paths print through one function, so their lines are the same by
     construction. It inspects only the instances it prints.

   Without `--setup`, `inspect` is unchanged. A flag on `inspect`, not a new command, since it is
   a narrower view of the same requests.
5. **Serve's ready line.** Once its listener is bound, after the startup setups as today,
   `conduit serve` writes exactly one line to stdout and flushes it:
   `{"ready":{"url":URL,"store":DIR}}`.
   - URL is the engine address it listens at. DIR is the store directory as a canonical
     absolute path.
   - A request sent once the line is read is answered at its first attempt. The line is
     written whether or not every startup setup succeeded, since serve keeps serving for repair,
     as today.
   - Nothing else is written to serve's stdout. Where serve exits before listening (the store
     held, the port taken, a signal during setup), no line is written.
   - A failed write of the line, such as to a closed stdout, is reported on stderr and does not
     stop serve.

   JSON, as the CLI's other stdout lines are.
6. **The words.** `engine/crates/shell/src/help.rs`:
   - `call`'s arguments become `TOOL [ARGUMENTS-JSON | --arguments-file PATH|-]`, and its
     description says what `-` reads.
   - `pending` takes `[--agent NAME]` and `inspect` takes `[--setup]`, each with a clause in its
     description.
   - `serve`'s description names the ready line.
   - The general help names `conduit --version` and says uncommitted changes are not shown.

   `engine/README.md`:
   - The sentence in "Runnable Agent packages" that gives `conduit call --store DIR
     turn-a.turn.v1` gains the file form.
   - The paragraphs on `conduit inspect` and `conduit pending` in "Change a serving project"
     each gain one sentence for their flag. The paragraph beginning "These commands are requests
     of `conduit serve`" gains one on the ready line, giving its form.
   - The paragraph beginning "`conduit --help` and" names `conduit --version`.

   The README writes each option as the help does (`--arguments-file PATH`, `--agent NAME`,
   `--setup`, `conduit --version`) and the ready line's form, `{"ready":...}`.
   `engine/CONFORMANCE.md` does not change: no promise, case or DL row moves.
7. **The tests**, each running the real binary, with no network and no gating on
   `CONDUIT_TEST_NPM`. The first four go in `engine/crates/shell/tests/serve.rs`, with
   its `start_server` and helpers. The second and third may share one store, whose project has
   the shape of `pending_and_answer_list_and_dismiss_a_failed_setup`'s, adding a Fleet and an
   Agent whose setup hook is the same missing program, so that each fails its setup and has a
   setup action at root (`submit` stops at its first failed step: set the other up by `call`).
   - `call_reads_arguments_from_a_file_or_stdin`. After `init` and `serve`, `call
     management.inspect.v1` of root succeeds with root's ID in three forms: inline, from a file
     of at least 64 KiB (the same object padded with whitespace), and from `-` on standard input.
     Each of these exits nonzero with empty stdout and a stderr that names the path or standard
     input, with the count of `access-decision` records in `state.json` unchanged:
     - both forms at once;
     - a missing file;
     - a file of invalid JSON holding a marker the test makes, which appears in no output.
   - `pending_agent_lists_that_agents_actions_alone`.
     - `pending` lists both actions, and `pending --agent` with the Agent's name prints the
       Agent's line alone, byte for byte as in the full list.
     - With the Resource's name, or a name nothing has, `--agent` prints nothing on stdout,
       exits nonzero, and its stderr names it.
     - For an Agent with no action raised, it prints nothing and exits 0: the Agent after
       `answer` dismisses its action, or a second Agent whose setup succeeded.
   - `inspect_setup_prints_one_line_per_instance`. While serving, `inspect --setup` prints one
     line per instance of the five kinds, and none for a template or a Conduit.
     - The line of the Pool `workspace` holds `succeeded` with `notSucceeded` empty; those of
       the Resource `failing` and the failed Agent hold `failed` or `unknown` and name the
       failed part; the first-start instances' lines hold `"setup":null`.
     - Serve then stops: send SIGTERM and wait for exit 0, without `stop_server`, whose second
       half runs `inspect` on the store. After the stop, `inspect --setup` prints the same lines
       byte for byte.
   - `serve_prints_one_ready_line_once_it_answers`. Serve starts with stdout piped.
     - Its first line, read with a bound on the wait as the file's other waits bound a hang, is
       the ready line, with the store's engine address and the canonical store path.
     - `conduit tools --store` run at once exits 0, and its stderr holds no "retrying".
     - After SIGTERM and exit 0, stdout held that one line alone.
     - With the store's port already held by a listener the test binds, serve exits nonzero
       with empty stdout.

   In `engine/crates/shell/tests/cli.rs`:
   - `version_names_the_crate_version_and_the_commit`. `conduit --version`, alone and with no
     store, exits 0 with empty stderr. Its stdout is exactly `conduit VERSION (commit H)` and a
     newline, where VERSION is `env!("CARGO_PKG_VERSION")`. H is the output of `git rev-parse
     HEAD` run in the repository root at the test's time, or `unknown` where that command fails.

   On `main`, the first four fail because `--arguments-file`, `--agent` and `--setup`
   are unknown arguments and serve writes no line, and the fifth because `--version` is an
   unknown command. `help_describes_every_command_without_store_access` still passes.
8. **Stop and report, change nothing more,** where any of these holds:
   - a correct result needs a change under `engine/crates/core/` or `spec/`, to a case, to a
     `Cargo.toml` or to `engine/Cargo.lock`, or a crate that is not in the cache;
   - a choice needs `spec/` text;
   - `inspect.v1`'s result holds no `status.setup` for an instance whose setup ended;
   - `list.v1`'s reply does not let the CLI tell which actions name an Agent, or which objects
     are Agent instances.

   Report the gap and the options it leaves, and do not choose.

## What to read

- `engine/crates/shell/src/main.rs`: the argument loop, the `call` arm, `inspect`'s fallback on
  "store already has an active shell", and the `serve` arm.
- `engine/crates/shell/src/author.rs`: `run`, `list_pending`, `inspection_row` and its `inspect`
  arm.
- `engine/crates/shell/src/server.rs`: `serve` (where it binds) and `client`.
- `engine/crates/shell/src/help.rs`.
- `engine/crates/shell/tests/serve.rs`: `start_server`, `stop_server` and
  `pending_and_answer_list_and_dismiss_a_failed_setup`.
- `engine/crates/shell/tests/cli.rs`: `help_describes_every_command_without_store_access`.
- LIF-004 in `spec/conventions/lifecycle.yaml`; `list.v1` and `inspect.v1` in
  `spec/core/operations.yaml`; `pending-action.v1` in `spec/core/results.yaml`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor ed1c0477 HEAD; echo "main $?"
grep -c '"--version"' engine/crates/shell/src/main.rs
grep -c 'arguments-file' engine/crates/shell/src/main.rs engine/crates/shell/src/help.rs engine/README.md
grep -c 'const COMMANDS: \[(&str, &str, &str); 15\]' engine/crates/shell/src/help.rs
grep -c '"management.list.v1"' engine/crates/shell/src/author.rs
grep -c 'object\["status"\]\["setup"\]' engine/crates/core/src/lifecycle.rs
test -f engine/crates/shell/build.rs; echo "build $?"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform aarch64-apple-darwin >/dev/null); echo "offline $?"
git status --short | wc -l | tr -d ' '
```

They print:
- `main 0`, then `0`;
- three lines ending `:0`;
- `1`, `1`, `1`;
- `build 1`, `offline 0` and `0`.

## Scope

Change these:
- `engine/crates/shell/src/`: `main.rs`, `author.rs`, `help.rs`, `server.rs`, or a module of
  your own;
- `engine/crates/shell/build.rs`, new, if you take that route;
- `engine/crates/shell/tests/serve.rs` and `cli.rs`;
- `engine/README.md`;
- `engine/STANDING.md`, only by `promise_standing.py --write` if its check asks.

Change nothing under `engine/crates/core/`, `engine/crates/loader/`,
`engine/crates/conformance/`, `spec/`, `conformance/`, `packages/`, `tools/`, `docs/` or
`experiments/`, and not `engine/CONFORMANCE.md`, any `Cargo.toml` or `engine/Cargo.lock`. Put
anything else you find that needs changing in your report.

## Done when

Write the five tests of decision 7 first and run them on the unchanged engine: report each
failing line. Then change the shell until they pass. The binary lines use the release binary
the two test lines build.

```sh
(cd engine && cargo test --release -p conduit-shell --test serve -- call_reads_arguments_from_a_file_or_stdin pending_agent_lists_that_agents_actions_alone inspect_setup_prints_one_line_per_instance serve_prints_one_ready_line_once_it_answers)  # 4 passed; fails today (0 passed)
(cd engine && cargo test --release -p conduit-shell --test cli version_names_the_crate_version_and_the_commit)  # 1 passed; fails today (0 passed)
[ "$(engine/target/release/conduit --version)" = "conduit 0.1.0 (commit $(git rev-parse HEAD))" ]; echo "version $?"  # version 0; fails today (version 1)
engine/target/release/conduit --help | grep -c -e '--version'  # 1 or more; fails today (0)
engine/target/release/conduit call --help | grep -c -e '--arguments-file'  # 1 or more; fails today (0)
engine/target/release/conduit pending --help | grep -c -e '--agent NAME'  # 1 or more; fails today (0)
engine/target/release/conduit inspect --help | grep -c -e '--setup'  # 1 or more; fails today (0)
grep -o -e '--arguments-file' -e '--agent NAME' -e '--setup' -e 'conduit --version' -e '{"ready"' engine/README.md | sort -u | wc -l | tr -d ' '  # 5; fails today (0)
```

If you made a new file, run `git add -N` on it before the next block and `git reset -q` on it
after, so that the checks see it; stage nothing. These pass, as on `main`; run the whole suite
once, last:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_docs.py  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git diff --name-only HEAD -- engine/crates/core engine/crates/loader engine/crates/conformance spec conformance packages tools docs experiments engine/CONFORMANCE.md engine/Cargo.lock | wc -l | tr -d ' '  # 0
git diff --name-only HEAD -- '*Cargo.toml' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
```

Your report gives:
- each test's name with its failing line on the unchanged engine, then its passing line;
- the forms as built: the version line and how the build finds the commit and when it asks
  again, the stderr of each refusal of decisions 1 and 3, a line of `inspect --setup` for a
  failed setup, and the ready line;
- where `inspect --setup`'s two paths share their printing;
- every choice you made yourself, under `own_choices`;
- every check's last line.

The verifier's turn follows; the coordinator then runs `tools/gate.py`, has a
fresh, read-only Claude reader read the diff
([ruling 79](../proposals/ruling-79/README.md); ruling 92 point 4), and merges.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, in Pacific time. Worker-2's
first turn (14:48 to 15:12) built it: its five new tests failed on the unchanged shell for the
expected reasons and pass, and the one whole engine run passed 418. `turn-verifier-2.turn.v1`
(15:15 to 15:20) found two musts in the build's commit stamp: it missed a retargeted
intermediate symbolic ref and a reftable repository's moves. The second turn (15:21 to 15:25)
fixed both, each with a regression that failed first. `turn-reader-1.turn.v1` (15:30 to 15:34),
the Claude reader, accepted it with one nit: the README's ready-line example now shows the bytes
serve prints. The gate's run on the squash is the coordinator's, at the merge.
