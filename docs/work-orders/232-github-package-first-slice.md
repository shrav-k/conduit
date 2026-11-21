# Work order 232: dogfood gap G-6, the `github` package's first slice: read-only, through GitHub's own MCP server

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order needs no
network: it reaches no host and fetches no crate; its only sockets are the loopback ports the
engine's own tests bind. Run one cargo command at a time. Never print a whole large file, case,
fixture or log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Every token here is a
synthetic marker a test makes and never prints. Do not run `conduit`
against a store outside a test, `codex`, `claude`, the dogfood harness, `gh`, `docker` or any
real `github-mcp-server`, and sign in nowhere.

## What and why

**The gap** (the dogfood-gaps stream, step G-6; [ruling 92](../proposals/ruling-92/README.md),
point 9; [DEC-027](../decisions/log.md#dec-027-2026-11-20-conduit-builds-conduit-directly-through-conduit)).
No Agent reaches GitHub through Conduit: the coordinator reads and merges pull requests with its
own `gh`, so no Agent gets a narrower view, and nothing records what was read.

**The owner's decisions** (2026-11-20; [DEC-021](../decisions/log.md#dec-021-2026-11-20-how-packages-are-organized),
"github + git"; [DEC-034](../decisions/log.md#dec-034-2026-11-20-the-github-packages-server-and-its-credentials)):
GitHub implementation -> "Wrap github-mcp-server (Recommended)"; token kind -> "Fine-grained
personal token". Order 229 is the `git` package (G-5).

**This slice** is reads: a Resource naming one repository, four read operations on its issues
and pull requests, and a standard `read` Conduit template. A Rust front runs GitHub's own
`github-mcp-server` read-only and gives it the token, a Pool-owned secret Resource the engine
reads for the front's run (orders
[218](218-s2-1-secret-read-file-store.md) and [224](224-s2-2-keychain-store.md)).

**The server is not on this Mac** (checked 2026-11-20): not on `PATH`, in Homebrew or the npm
cache; no Go; Docker not running. The tests use a stand-in. **Live use waits for the owner's
sitting**: installing the server, making and storing the token, one read of an issue and a pull
request. This order reaches no GitHub host.

**Preservation tests served** ([the tests](../AUDIT.md#preservation-tests); ruling 92 point 1):
3, owned once and never copied to an Agent (the Pool's token reaches the server alone); 2,
inspected and revoked (one repository, by a Conduit listing its operations); 7, attributable
(each read is a recorded request through a named Conduit).

**Should this exist** (point 3). The server, the token and the stores exist; Conduit adds only the
access
([DEC-003](../decisions/log.md#dec-003-2026-11-20-innovate-only-on-connection-and-access)). The
Rust front fills three gaps: it fixes `owner` and `repo`, which the server takes from each call,
from the configuration (as GitHub Actions scopes `GITHUB_TOKEN` to one repository); it keeps the
server's tool names, which change between releases, in one table, failing by name on a mismatch;
and it gives the token to the server alone.

Not taken (the coordinator's, open to the owner's strike): the server as a `plain` item, as
[the secrets proposal](../proposals/secrets/README.md) sketches (any repository; no filter,
PKG-020); one Resource for all repositories, narrowed by a filter (only the package's own check
would guard the others); GitHub's remote endpoint or image (unserved); `gh` (its stored login is
a fallback, against ruling 52).

**When it runs.** Through Conduit's dogfood: one Codex worker turn (aim under 2 hours), a
Codex verifier and a Claude reader ([ruling 79](../proposals/ruling-79/README.md)) who
run only what settles a doubt, and the gate
([DEC-033](../decisions/log.md#dec-033-2026-11-20-the-testing-audit)). By
[ruling 89](../proposals/ruling-89/README.md), each decision says what must hold; how is yours.
Where your base holds order 229's rows in `tools/gate.py`, keep them.

## The decisions

1. **The package.** `packages/github/`, in provider-file's form (its own `Cargo.lock`,
   `rust-toolchain.toml` at 1.97.1, `.gitignore`, `conduit.yaml`, `contracts.yaml`, `README.md`,
   `tests/package.rs`, `tests/schema_drift.rs`, and `src/bin/package.rs` building
   `dist/github.tar.gz` and its digest). Cargo package `conduit-github`, binary
   `bin/conduit-github`; address `example.org/conduit/github`, version 0.1.0. Start the lock from
   a copy of provider-file's: with rmcp's `client` and `transport-child-process` (its dev
   features) and tokio's `process` as normal dependencies, resolving offline adds no crate.
2. **The contracts.**
   - **Type `repository.v1`**, family Resource. Schema `{repository, token}`, both required, no
     other key: `repository` is `owner/name` (an owner of 1 to 39 of `[A-Za-z0-9-]`, a name of 1
     to 100 of `[A-Za-z0-9._-]`, not `.` or `..`); `token` is core's `secret-reference.v1`. It
     offers `#repository-reading.v1`, lists no control, and implements the four operations by
     one item `#front`: `command: [bin/conduit-github]`, `environment:
     {GITHUB_PERSONAL_ACCESS_TOKEN: /token}`, not `plain`, so every call carries Conduit's keys
     (IFC-006), supporting no requirement. `records: {inputs: true, outputs: false}`.
   - **Operations**, each with a closed schema, `idempotentHint: true`, `openWorldHint: true`,
     no `effect`, and a description saying it reads the Resource's repository alone:
     `issue.read.v1` `{number}` (required, an integer of at least 1); `issue.list.v1` `{limit}`
     (required, an integer 1 to 100: the first page, in the server's default state and order);
     `pull-request.read.v1` `{number}`; `pull-request.list.v1` `{limit}`. Each gives `{data}`, the
     JSON the server's tool answered, unchanged but for decision 4's replacement within its
     strings (object or array), the descriptions saying the shape is the server's; Conduit's own
     projection is a later slice, beside `data`.
   - **Capability** `repository-reading.v1`, listing the four operations; one Conduit template,
     `read`, naming `#repository-reading.v1` and listing all four (PKG-031; one capability per
     Conduit, CON-017), exported as the S1 fixture `release-adds-optional` exports its templates.
3. **The front and the server.**
   - At its first call the front starts one server: `github-mcp-server` found on its own
     `PATH`, with exactly `stdio --read-only --toolsets issues,pull_requests`, its environment
     cleared but `PATH`, the front's `HOME` and `GITHUB_PERSONAL_ACCESS_TOKEN`.
   - After `initialize` it lists the server's tools: each tool of the table must be listed, its
     input schema's properties holding every argument the front sends, or every call fails
     naming the tool, the property and the server's reported version.

     | Operation | Tool | Arguments |
     | --- | --- | --- |
     | `issue.read.v1` | `issue_read` | `method: "get"`, `owner`, `repo`, `issue_number` |
     | `issue.list.v1` | `list_issues` | `owner`, `repo`, `perPage` |
     | `pull-request.read.v1` | `pull_request_read` | `method: "get"`, `owner`, `repo`, `pullNumber` |
     | `pull-request.list.v1` | `list_pull_requests` | `owner`, `repo`, `perPage` |

     The table, in one place in `src`, is the coordinator's reading of the server's
     documentation.
   - `owner` and `repo` come from the `repository` of each call's configuration (IFC-006), never
     from its arguments; an argument outside the operation's schema fails before the server is
     asked; the front calls no tool outside the table.
   - The server stops with the front (its stdin closing, or a signal): none outlives it.
4. **The token.** The front's only source is `GITHUB_PERSONAL_ACCESS_TOKEN` in its run's
   environment, which the engine fills by reading the Pool's secret Resource (SEC-010). No
   fallback: never `GH_TOKEN`, `GITHUB_TOKEN`, `gh` or its files, or any file. Absent or empty,
   every call fails saying so, and no server starts. The token goes into the server's
   environment alone: no argument, result, `_meta`, message, stderr line or file holds it, and
   in any text the front passes on from the server each occurrence of its bytes becomes `[token]`.
5. **Failures**, each outcome failed with a fixed message naming the cause and no partial
   `data`: no `github-mcp-server` on `PATH`; the server not started, or exited (the next call
   starts a new one); a tool or argument of the table missing; an `isError` answer (its text, at
   most 2 KiB, token replaced); an answer that is not one text item of JSON, or over 8 MiB; a
   configuration the schema refuses (no server starts).
6. **The README**, in provider-file's form with its D15 inventory, states: the configuration; the
   Pool owning the token's Resource and each repository's, each with a declared Conduit to the
   token listing `get-secret.v1`; the owner's steps (the server from GitHub's releases on serve's
   `PATH`, unpinned by the digest, MAN-010; a fine-grained token for the chosen repositories,
   read-only Issues, Pull requests and Metadata, with an expiry, in the dedicated keychain or a
   0600 file); the table, untested against a real server; and that no Conduit gives an Agent the
   token, but an unconfined v0 Agent's own tools can reach its store
   ([DEC-018](../decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2)).

## The tests

**The stand-in**, a test-only program `github-mcp-stand-in`, never packed; a test copies it
into a temporary directory as `github-mcp-server`. It serves the
table's four tools over stdio with those input schemas, answers each call with fixed JSON naming
the `owner`, `repo` and number it was given, and writes what it saw under its `HOME`: its
arguments, its environment's names, the token's SHA-256, each call's tool and arguments, and its
process ID. A mode file there makes it leave out a tool, answer `isError` with text holding the
token, answer success JSON holding the token, answer text that is not JSON or over 8 MiB, or exit
during a call. It opens no socket.

Each test canonicalizes its temporary directory, has no other `github-mcp-server` on its `PATH`,
prints no marker (compare digests), and bounds each call at 5 seconds.

**Package tests**, `packages/github/tests/mcp.rs`, drive `bin/conduit-github` over stdio with
its environment cleared but `PATH`, `HOME` (a temporary directory), the token variable, and
decoys `GH_TOKEN`, `GITHUB_TOKEN` and `EXTRA` holding other markers, and `_meta` as the engine
sends it, as provider-file's `tests/mcp.rs` does. At least, by these names:
- `reads_go_to_the_configured_repository_alone`: each operation gives the stand-in's `data`
  unchanged; the stand-in saw exactly decision 3's arguments, and each tool and its arguments
  with `owner` and `repo` from the configuration. Arguments adding `repository`, `owner` or
  `repo`, and a configuration that is not an object, lacks or adds a key, or holds a bad
  repository, each fail with no call recorded.
- `the_token_reaches_the_server_and_nothing_else`: the stand-in's environment names are exactly
  `PATH`, `HOME` and `GITHUB_PERSONAL_ACCESS_TOKEN`, its digest the marker's; the marker is in
  no result or stderr of the front, the `isError` mode's message and the success-with-token
  mode's `data` holding `[token]`; with the variable absent, then empty, the decoys still set,
  each call fails and no stand-in starts.
- `server_failures_fail_explicitly`: no server on `PATH`; a missing tool, named with the
  stand-in's version; not JSON; over 8 MiB; an exit during a call, after which the next call
  succeeds with a new process ID.
- `the_server_stops_with_the_front`: with the front's stdin closed, the stand-in is gone within
  5 seconds.

The file keeps provider-file's discovery, extension and filter refusal tests.
`tests/schema_drift.rs` checks the contracts against `spec/` as provider-file's does, and the
exports and the `read` template's operations; `tests/package.rs` checks the archive, with no
stand-in in it.

**Engine test**, `engine/crates/shell/tests/github.rs`, one test, `github_through_the_engine`.
It packs provider-file and `packages/github` as `tests/secrets.rs` packs provider-file, builds
the stand-in, and drives the CLI as `tests/serve.rs` does, serve's `HOME` a temporary directory
and its `PATH` the stand-in's. Its project, after the `secret-reads` one: a Pool owns `token`, a
`file-secret.v1` at a 0600 file outside the store holding a marker, and `allowed`, a
`repository.v1` for `octo/allowed` whose `token` is the entry reference to `token`, with a
declared Conduit to it listing `get-secret.v1`; root has `allowed-read`, from
`example.org/conduit/github#read`, and `allowed-issues`, naming
`example.org/conduit/github#repository-reading.v1` and listing `issue.read.v1` alone. It shows:
- the Pool's setup succeeds, and the stand-in's token digest is the marker's;
- `allowed-read.issue.read.v1 {"number": 1}` and `allowed-read.pull-request.list.v1 {"limit": 5}`
  succeed with the stand-in's `data`, the stand-in having seen `owner` `octo` and `repo` `allowed`;
- a call adding `"repository": "octo/other"` fails, and the stand-in saw no call for it;
- `allowed-issues` presents no `pull-request.read.v1`: `conduit tools` lists none, and a call is
  refused and reaches no server;
- the marker is in no byte under the store or of any `conduit` output (as `secrets.rs` scans);
- after serve stops, no stand-in process is left.

**The gate.** `tools/gate.py` gains the `RUST_READS` row
`('packages/github', 'spec/', 'tests/schema_drift.rs', '"../../spec'),` and `'packages/github/'`
in `ENGINE_PREFIXES` and `ENGINE_WHY`, since the engine test reads it; `tools/tests/test_gate.py`
gains `('packages/github/x', set(), True)` beside provider-file's case.

## What this leaves

Report this list, corrected by what you built: the owner's sitting (the server installed, the
token stored, the table checked against that version, a live read of each kind); the dogfood
reading through it (the coordinator's); Conduit's own projection of results; writes, checks and
search (whose query names any repository); the `git` package's push using the same secret (the
owner's "Working copy references the secret", DEC-034); the server pinned in the archive, or
its image or remote forms; an Agent confined away from the token's store (DEC-018).

## Stop rule

Stop, change nothing further, and report the gap and its options, without choosing, where: a
crate is not in the cache; a correct result needs a change under `spec/`, or to an engine
source, a case, an engine `Cargo.toml` or `engine/Cargo.lock`; the engine does not read the
token for the front's run, or names it otherwise; a project cannot take the `read` template by
`from:`; or a test would run a real `github-mcp-server` or reach the network.

A choice that no promise governs is yours: list it under `own_choices`.

## What to read

`packages/provider-file`; `engine/crates/shell/tests/secrets.rs`, `serve.rs` and the
`secret-reads` project; order 229, for the package's form; IFC-006, IFC-019, IFC-026, PKG-020,
PKG-031 and SEC-010.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 9d5517ba HEAD; echo "main $?"
test -e packages/github; echo "package $?"
test -e engine/crates/shell/tests/github.rs; echo "test $?"
test -f engine/crates/shell/tests/secrets.rs; echo "secrets $?"
grep -c 'packages/github' tools/gate.py tools/tests/test_gate.py
host=$(rustc -vV | sed -n 's/^host: //p'); d=$(mktemp -d) && cp packages/provider-file/Cargo.toml packages/provider-file/Cargo.lock packages/provider-file/rust-toolchain.toml "$d"/ && mkdir "$d/src" && echo 'fn main() {}' > "$d/src/main.rs" && sed -i.bak 's/features = \["server", "transport-io"\]/features = ["server", "transport-io", "client", "transport-child-process"]/' "$d/Cargo.toml" && grep -q '"transport-io", "client"' "$d/Cargo.toml" && (cd "$d" && cargo metadata --offline --locked --format-version 1 --filter-platform "$host" >/dev/null 2>&1); echo "offline $?"; rm -rf "$d"
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$(rustc -vV | sed -n 's/^host: //p')" >/dev/null); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `package 1`, `test 1`, `secrets 0`, two lines ending `:0`, `offline 0`,
`engine 0` and `0`. An `offline` or `engine` line that prints otherwise means a crate is not in
the cache: stop.

## Scope

Create `packages/github/` and `engine/crates/shell/tests/github.rs`; change `tools/gate.py` and
`tools/tests/test_gate.py` as above, and `engine/STANDING.md` only by
`promise_standing.py --write` if its check asks. Change nothing else: not `spec/`,
`conformance/`, `experiments/`, `docs/`, `.github/`, `IN_FORCE.md`, `ROADMAP.md`, another
package, an engine source, `engine/CONFORMANCE.md`, a `Cargo.toml` outside `packages/github/`, or
`engine/Cargo.lock`. Put anything else that needs changing in your report.

## Done when

Each line marked "fails today" prints otherwise on `main` at `9d5517ba`; all pass after.

```sh
(cd packages/github && cargo metadata --offline --locked --format-version 1 >/dev/null); echo "offline $?"  # offline 0; fails today (1)
(cd packages/github && cargo fmt --check)  # passes; fails today
(cd packages/github && cargo clippy --all-targets -- -D warnings)  # passes; fails today
(cd packages/github && cargo clippy --all-targets --target x86_64-unknown-linux-gnu -- -D warnings)  # passes, as CI's ubuntu job builds it; fails today
(cd packages/github && cargo test --release)  # passes; fails today
(cd packages/github && cargo test --release --test mcp -- --list 2>/dev/null | grep -cE '^(reads_go_to_the_configured_repository_alone|the_token_reaches_the_server_and_nothing_else|server_failures_fail_explicitly|the_server_stops_with_the_front): test$')  # 4; fails today (prints nothing)
(cd packages/github && grep -rhoE '"(issue_write|create_pull_request|update_pull_request|merge_pull_request|add_issue_comment|push_files|create_or_update_file|delete_file|search_[a-z_]+)"' src | wc -l | tr -d ' ')  # 0; fails today (prints nothing)
(cd packages/github && grep -rlE 'GH_TOKEN|GITHUB_TOKEN|hosts\.yml|\.config/gh|"gh"' src | wc -l | tr -d ' ')  # 0; fails today (prints nothing)
(cd engine && cargo test --release -p conduit-shell --test github)  # 1 passed; fails today (no such test target)
grep -c "('packages/github', 'spec/'" tools/gate.py  # 1; fails today (0)
(cd tools && PYTHONDONTWRITEBYTECODE=1 python3 -c "import gate as g; print('packages/github/' in g.ENGINE_PREFIXES, 'packages/github/' in g.ENGINE_WHY)")  # True True; fails today (False False)
grep -c "'packages/github/x'" tools/tests/test_gate.py  # 1; fails today (0)
```

Then run `git add -N packages/github engine/crates/shell/tests/github.rs` so that the checks see
the new files. The checks below pass, as on `main`. Run the whole suite once, last, then
`git reset -q`, and stage nothing:

```sh
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --all-targets -- -D warnings)
python3 tools/check_docs.py  # Errors: 0
python3 tools/set_names.py --check  # passes
python3 tools/check_schemas.py  # Errors: 0
python3 tools/promise_standing.py --check  # Errors: 0
python3 tools/check_private_paths.py  # Errors: 0
python3 -m unittest discover -s tools/tests  # OK
git diff --check  # prints nothing
git diff --name-only HEAD | grep -vE '^(packages/github/|engine/crates/shell/tests/github\.rs$|tools/gate\.py$|tools/tests/test_gate\.py$|engine/STANDING\.md$)' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # the whole suite
```

Your report gives, per decision, what you built and its tests; each failure's message; the
stand-in's record of one call (no digest); how the server's environment is built and how it
stops; `own_choices`; the corrected list of what this leaves; every check's last line.

## Record

Run on 2026-11-20 through Conduit itself, by the dogfood's own Agents, in Pacific time.
Worker-1's first turn (19:50 to 20:09) built the package, and its eight tests passed. It stopped
at `conduit submit`, which failed on a schema reference that order 227's SCH-015 fix then
resolved. The second turn (20:35 to 21:00), on main with 227, finished the order. The engine
integration covered installation, registration, Pool setup, delivery of the token through the
secret path, and both reads, and the one whole engine run passed.

`turn-verifier-1.turn.v1` (21:00 to 21:05) found three musts:
- a configured name could carry the token into an upstream argument;
- an echoed restriction could carry it into a reply;
- SIGHUP could end the front without stopping its server.

The third turn (21:06 to 21:13) fixed all three, each with a regression that failed first. 11
package tests pass.

`turn-reader-1.turn.v1` (21:14 to 21:16), the Claude reader, accepted it with no findings.

The real `github-mcp-server` is not on this Mac, so live use waits for the owner's sitting:
- install a GitHub release of the server onto serve's PATH;
- check the tool table against that release;
- store a scoped, fine-grained token in the Pool's secret;
- make one issue read and one pull-request read.

The gate's run on the squash is the coordinator's, at the merge.
