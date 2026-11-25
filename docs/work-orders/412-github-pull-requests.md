# Work order 412: the github package creates a pull request and squash-merges one only at its expected head, for root's write Conduit, with a second token (DEC-173)

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

This order reaches no GitHub host. Run no `gh`, `git push` or real `github-mcp-server`, and make,
store or read no GitHub token: every token here is a synthetic marker a test makes and never
prints, and every server a test meets is the stand-in.

## What and why

**The owner's words.** [DEC-173](../decisions/log.md#dec-173-2026-11-23-merging-through-conduit-uses-a-second-fine-grained-github-token):
which GitHub credential lets root's GitHub Conduit create and merge pull requests -> "A second
fine-grained token (Recommended)", landing in "a separate fine-grained token with pull-request
write on this repository only, held as a secret Resource, which the owner creates; the read-only
token stays as it is".
[DEC-034](../decisions/log.md#dec-034-2026-11-20-the-github-packages-server-and-its-credentials):
GitHub implementation -> "Wrap github-mcp-server (Recommended)"; token kind -> "Fine-grained
personal token".
[DEC-171](../decisions/log.md#dec-171-2026-11-23-the-coordinator-hands-off-through-written-state-its-kits-rust-replacements-follow-the-guard):
the kit -> "Order Rust replacements first";
[DEC-180](../decisions/log.md#dec-180-2026-11-24-parallel-tracks-on-the-accounts): "We should
also likely rewrite everything we can into the engine now using the new accounts we have access
to", which gives the kit's replacements to pairs 3 and 4 now. This order is the github package's
part of the merge's replacement; order 414 moves the queue onto it.

**Today.** The coordinator's merge kit opens each pull request and squash-merges it with the `gh`
CLI under the owner's own login, `--match-head-commit` binding the merge to the commit the gate
passed. That login reaches every repository the owner can, and Conduit records nothing of it.
The github package ([order 232](232-github-package-first-slice.md)) reads one repository through
GitHub's `github-mcp-server`, started `--read-only` with a read-only token, and writes nothing.

**The server's tools** (the coordinator's reading of `github-mcp-server`'s README and source on
2026-11-24, untested against a release; decision 4 lists what the writer sends). The four it needs
are in its `pull_requests` toolset, and `--read-only` drops the two writes. That toolset also holds
other writes: reviews, comments, `update_pull_request` and `update_pull_request_branch`. `--tools`
with no `--toolsets` registers only the tools it names (`ResolvedEnabledToolsets`: "--tools=X alone
registers only X"), and a name the server does not know fails its start. `create_pull_request`
answers `{id, url}`, `url` the pull request's page. `merge_pull_request` sends `expectedHeadSha` as
the REST merge's `sha`, so GitHub itself refuses a merge whose head has moved, and answers
`{sha, merged, message}`. `pull_request_read`'s `get` answers `state`, `merged`, `head.sha` and
`base.ref`, among others. A failed call answers `isError`, with text that does not always hold
GitHub's status.

**The token's permissions.** GitHub's table of fine-grained token permissions lists
`POST /repos/{owner}/{repo}/pulls` under Pull requests write and
`PUT /repos/{owner}/{repo}/pulls/{pull_number}/merge` under Contents write. So DEC-173's
"pull-request write" becomes Contents and Pull requests write: past its words, and the owner's
question at the sitting (What follows). Contents write also lets whoever holds the token push to
any branch the repository's rules allow, not only merge. The writer calls four tools alone, and
the token reaches the writer and its server alone, but an unconfined v0 Agent can reach the
token's store with its own tools (DEC-018).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 2, an
access can be inspected and revoked: one Conduit of root's to one Resource, and closing it ends
the writes; 3, owned once and never copied to an agent: the Pool's write token reaches the
writer and its server alone; 6, nothing repeats an effect implicitly: the front never retries a
write, and a write it cannot confirm is unknown, which the engine does not send again (OPS-012);
7, attributable: each create and merge is a recorded request through root's Conduit, with its
inputs and outputs. Decisions 1, 5 to 7 and 9 keep them.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes, by
existing practice. GitHub's REST merge takes `sha`, the commit the head must match, and
`gh pr merge --match-head-commit` sends the same check; GitLab's merge refuses unless its `sha`
matches the source branch's head. GitHub Actions' `permissions:` narrows a job's token to
`contents: write` and `pull-requests: write` on one repository, the two this order's token holds.
The server has both tools (DEC-034); Conduit adds only the access: the repository and base fixed by
configuration, a token for writes alone, a Conduit that root alone holds, and outcomes the queue can
act on ([DEC-010](../decisions/log.md#dec-010-2026-11-20-every-result-says-what-happened-and-why)).
Not taken: `gh` in the queue (the other option offered; its stored login is a fallback); the writes
on `repository.v1` (a Resource holds one token, and a reader's Conduit would then run a server that
can write); GitHub's REST API from the front (DEC-034 wraps the server, which already sends the
check); `--toolsets pull_requests` for the writer's server (it would offer every write of the
toolset, reviews and comments among them, to a process holding Contents write); a GitHub App's
installation token (What follows); `deduplicates` (the front keeps no memory of an invocation across
runs); GitHub's merge queue or auto-merge (they wait on required checks, while the local gate
decides here,
[DEC-186](../decisions/log.md#dec-186-2026-11-24-one-merge-queue-per-repository-with-blocking-ci-there)).

**When it runs.** On `main`, written at `6d3a1f14b2a9d3815c425e433ef5284a1ea6883d`, now (DEC-180):
it changes only the github package and its engine test. Through Conduit: one Codex worker turn, the
blind Codex verifier and the Claude reader at once (a README and refusals,
[DEC-189](../decisions/log.md#dec-189-2026-11-24-the-claude-reader-reads-text-facing-orders-only)),
one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each decision says what must hold;
how is yours. The coordinator's, open to the owner's strike
([DEC-029](../decisions/log.md#dec-029-2026-11-20-who-decides-and-who-does)): a Type of its own for
writes; `base` in its configuration; the reads before and after each write and the outcomes they
decide; the writer's server flags; the write token's permissions, Contents and Pull requests write,
past DEC-173's words; the names, the limits, 0.2.0 and the test names. It fits one turn; were it not
to, the engine test `pull_requests_through_roots_write_conduit` goes second as 412b, and the rest,
the engine test's version pin with it, first.

**Shared files.** Order 416, in flight, changes how `engine/crates/shell/tests/github.rs` bounds
its waits, and its rule covers this order: "Any engine order that adds a test with a fixed wait:
whichever merges later rebases and uses the helper." So the new engine test bounds its waits with
416's `wait_support` helper where 416 is on `main`, and otherwise with the file's `WAIT` and
`timeout`, which 416's rebase converts (decision 8; the Start check's `wait_support` line says
which). No order in flight changes `packages/github`. The later of two orders rebases and keeps
both.

**Stop rule.** Stop, change nothing further, and report only for a contradiction between promises
(give both), a step needing a change outside Scope (give its failing line), a file outside Scope
that must change, or a test that would reach the network or run a real `github-mcp-server`. Where
this order's wording contradicts a promise it does not change, the promise governs: apply it and
list it under `own_choices`, with every choice no decision governs.

## The decisions

Read first, in `packages/github`: `Operation`, `Front::dispatch` and `Front::call` in `src/lib.rs`,
`supervise_upstream` in `src/upstream_lifetime.rs`, `src/main.rs`, the stand-in, the helpers of
`tests/mcp.rs`, `tests/schema_drift.rs` and the README; `engine/crates/shell/tests/github.rs`;
then CTR-002, CTR-004, IFC-006, IFC-009, IFC-026, OPS-002 to OPS-004, OPS-012, PKG-025, PKG-031,
SEC-001, SEC-010, SEC-014, SEC-019 and SEC-020. Wrap each insertion at its file's width.

1. **The Type** `pull-requests.v1`, family Resource. Schema `{repository, base, token}`, all
   required, no other key: `repository` as `repository.v1` writes it; `base`, a branch (decision
   3); `token`, core's `secret-reference.v1` (SEC-001). It offers `#pull-request-writing.v1`,
   lists no control, requires nothing, implements both operations by item `#writer`, and keeps
   `records: {inputs: true, outputs: true}`: what was asked and what GitHub made (CTR-004).
   Capability `pull-request-writing.v1` lists `#pull-request.create.v1` and
   `#pull-request.merge.v1`; one Conduit template, `write`, names it and lists both (PKG-031).
   `repository.v1`, its capability, its four operations and `read` stay as they are.
2. **The item** `writer`: it implements the two operations, with
   `command: [bin/conduit-github, --write]` and
   `environment: {GITHUB_PERSONAL_ACCESS_TOKEN: /token}`, not `plain`, supporting no
   requirement. The binary with no argument stays the read front, unchanged; with `--write` it is
   the writer, which lists only the two writes and fails any other name as the read front fails
   one outside its table. The writer starts its server as the read front does (the supervisor,
   the token on the server's exec alone, the server's environment `PATH`, `HOME` and the token),
   with exactly these three arguments, the tool list one of them:
   `stdio --tools list_pull_requests,create_pull_request,pull_request_read,merge_pull_request`.
   No `--toolsets` and no `--read-only`, so the server registers those four tools alone. The
   read front's server keeps exactly `stdio --read-only --toolsets issues,pull_requests`. No call
   chooses the flags.
3. **The operations**, each with a closed schema, `openWorldHint: true`, and a description
   saying it acts on the Resource's repository and base alone. A branch, `base` or `head`, is 1
   to 255 of ASCII letters, digits, `.`, `_`, `/` and `-`; it does not begin with `-`, `/` or
   `.`, end with `/`, `.` or `.lock`, or hold `..`, `//`, `/.` or `.lock/`. Write the exclusions
   with `not`, as `repository.v1` excludes `.` and `..`: a pattern holds only JSON Schema's
   interoperable tokens (`spec/schema-profile.md`).
   - `pull-request.create.v1`. Inputs `{head, title, body}`, all required: `head`, a branch;
     `title`, 1 to 256 characters and no line break; `body`, at most 65,536 characters. Outputs
     `{number, url}`, both required: the new pull request's number, an integer of at least 1,
     and its page, a string whose pattern is anchored at `^https://github[.]com/`. `effect`:
     "Opens one pull request in the Resource's repository from the branch head into the
     Resource's base, with the given title and body." `idempotentHint: false`,
     `destructiveHint: false`.
   - `pull-request.merge.v1`. Inputs `{number, expectedHead, title, message}`, all required:
     `number`, an integer of at least 1; `expectedHead`, 40 lower-case hexadecimal digits;
     `title`, 1 to 1,024 characters and no line break; `message`, at most 65,536 characters.
     Output `{sha}`, required: the squash commit, 40 lower-case hexadecimal digits. `effect`:
     "Merges one open pull request of the Resource's repository into the Resource's base by
     squash, as one commit with the given title and message, only where the pull request's head
     is expectedHead." `idempotentHint: true`, `destructiveHint: false`.
   - No `deduplicates`: a repeat after unknown is answered unknown, never sent again (OPS-012).
4. **The writer's table**, in one place in `src`, checked once per server as the reads' table
   is: every tool listed, and each argument the front sends among its input properties, or every
   write fails naming the tool, the property and the server's version, and no tool is called.

   | Tool | Arguments the writer sends |
   | --- | --- |
   | `list_pull_requests` | `owner`, `repo`, `state: "open"`, `head: "OWNER:HEAD"`, `base` |
   | `create_pull_request` | `owner`, `repo`, `title`, `head`, `base`, `body` |
   | `pull_request_read` | `method: "get"`, `owner`, `repo`, `pullNumber` |
   | `merge_pull_request` | `owner`, `repo`, `pullNumber`, `merge_method: "squash"`, `commit_title`, `commit_message`, `expectedHeadSha` |

   `owner`, `repo` and `base` come from each call's configuration (IFC-006), never from its
   arguments. The writer calls no other tool, sends no other argument (no `draft`, `reviewers` or
   `maintainer_can_modify`) and retries no call.
5. **Create.**
   - (a) It lists the open pull requests from `head` into `base`. A list that fails, or is not a
     JSON array: failed, "could not list open pull requests from HEAD into BASE; nothing
     created". An entry: failed, "pull request #N from HEAD into BASE is already open; nothing
     created".
   - (b) It calls `create_pull_request`. An answer whose `url` is
     `https://github.com/OWNER/REPO/pull/N`, the configured repository compared without regard to
     ASCII case: succeeded, `{number: N, url}`.
   - (c) An `isError` answer: it lists again. None open: failed, with the server's text. One
     open, or a list that fails: unknown, naming #N where known.
   - (d) Any other answer, or none (a time-out, an exit, an answer over 8 MiB, or the front
     stopping after the call was sent): unknown.
6. **Merge.**
   - (a) It reads the pull request. A read that fails, or lacks `state`, `merged`, `head.sha` or
     `base.ref`: failed, "could not read pull request #N; nothing merged". Then failed, each
     message ending "nothing merged", where it is merged ("pull request #N is already merged"),
     its `state` is not `open` ("pull request #N is closed"), `base.ref` is not `base` ("pull
     request #N targets REF, not BASE"), or `head.sha` is not `expectedHead` ("pull request #N's
     head is SHA, not the expected EXPECTED").
   - (b) It calls `merge_pull_request`. An answer with `merged: true` and a `sha` of 40
     lower-case hexadecimal digits: succeeded, `{sha}`.
   - (c) An `isError` answer: it reads again. Not merged: failed, with the server's text, and
     "the head is now SHA" where the head moved. Merged, or a read that fails: unknown.
   - (d) Any other answer, or none: unknown.
7. **What every write keeps.** Succeeded only at 5(b) or 6(b) (OPS-002); failed only where no
   write tool was called, or the server answered `isError` and the read after it shows no trace
   of the write (OPS-003); unknown otherwise (OPS-004); `isError` as IFC-009 states. These are
   outcomes, not reasons: an implementation gives a reason only for a filter (IFC-026), and the
   writer keeps the reads' filter refusal. A stop before any write tool is called answers failed,
   as the reads' stop does. A stop after one is called answers unknown (OPS-004), or nothing
   where the session has already ended, and never failed. Each failed or unknown message names the
   pull request or head, the cause, and what was not done or is not known; an unknown one says
   to read the pull request through `pull-request.read.v1` or `pull-request.list.v1` (DEC-010).
   The server's text goes on at most 2 KiB, the token's bytes replaced by `[token]`, as the reads
   pass it. Arguments outside the schema, a configuration the schema refuses, a `head` equal to
   `base`, and any argument or configured name holding the token's bytes fail before a server
   starts. The token's only source is `GITHUB_PERSONAL_ACCESS_TOKEN` in the writer's run
   (SEC-010, SEC-020): no fallback; absent or empty, every write fails and no server starts. No
   result, message, `_meta` or standard-error line holds it (SEC-019). A rotated token reaches
   the writer at its next run (SEC-014). Only root's Conduit names the writer, by the project's
   choice alone: the engine refuses only a consumer with no open Conduit that lists the operation
   (ACC-001), only core's operations can be reserved to root (CTR-009), and the package adds no
   check of its own.
8. **Tests.** Write them first, run them on the otherwise unchanged package and report each
   failing line. No other test's name contains a new test's name. In `packages/github`, bound
   each wait as the file's tests bound theirs. In `engine/crates/shell/tests/github.rs`, bound
   each new wait with order 416's `wait_support` helper where the file holds `mod wait_support;`,
   and otherwise with the file's `WAIT` and `timeout`, which 416's rebase converts (Shared files).
   Print no marker.
   - **The stand-in.** Started without `--read-only`, it serves decision 4's tools with those
     properties, from a file of pull requests under its `HOME` that a test writes (number, head
     ref and SHA, base, state, merged), as GitHub would: create adds one and answers `{id, url}`,
     refusing one already open from that head into that base; merge refuses a head other than
     `expectedHeadSha` and answers `{sha, merged, message}`. Modes: the head moves after the
     first read; a write answers `isError` after its effect; a write answers `isError` without
     its effect, its text holding the token; create answers another repository's `url`; an exit
     during a write; a write held unanswered; `merge_pull_request` without `expectedHeadSha`. It
     records each process and call as today.
   - In `tests/mcp.rs`, with the reads' environment and decoys:
     - `pull_requests_are_created_and_merged_in_the_configured_repository` (decisions 2 to 6):
       create gives `{number, url}` and merge `{sha}`; the stand-in's arguments are decision 2's
       writer flags, and it saw exactly decision 4's calls, `owner`, `repo` and `base` from the
       configuration. Arguments adding `repository`, `owner`, `base` or `merge_method`, a `head`
       equal to `base`, and a configuration lacking `base` or holding a bad one each fail with no
       call recorded.
     - `a_merge_is_refused_unless_the_head_is_the_expected_commit` (decision 6): another head,
       a merged and a closed pull request, and another base each fail with their message, and
       the stand-in saw no `merge_pull_request`; with the head moved after the first read, the
       merge fails with "the head is now" and the new SHA, and the pull request stays open.
     - `writes_are_unknown_unless_their_effect_is_known` (decisions 5 to 7): an exit during
       each write, an `isError` after each effect, and another repository's `url` each give
       unknown naming the operation to read with; one already open gives failed naming it, and
       no `create_pull_request` was seen. With an `isError` and no effect, create gives failed
       with the server's text and "nothing created", and the stand-in saw the second
       `list_pull_requests`; merge gives failed with the server's text and no "the head is now"
       (the moved head is `a_merge_is_refused_unless_the_head_is_the_expected_commit`'s). With
       each write held, the writer then gets `TERM` as `the_server_stops_with_the_front` sends
       it, and answers unknown or nothing, never failed.
     - `the_writer_runs_its_own_server_and_keeps_its_token` (decisions 2 and 7): the writer's
       stand-in has the environment names `PATH`, `HOME` and `GITHUB_PERSONAL_ACCESS_TOKEN` and the
       marker's digest; no result or standard error holds the marker, and with an `isError` and no
       effect, its text holding the marker, each write's failed message holds `[token]` in its
       place; with the variable absent, then empty, each write fails and no stand-in starts. The
       writer lists only the two writes and fails `issue.read.v1`; the read front lists only the
       four reads and fails `pull-request.merge.v1`; neither call is recorded.
     - `a_missing_write_tool_or_property_fails_every_write` (decision 4): without
       `expectedHeadSha`, each write fails naming `merge_pull_request`, `expectedHeadSha` and the
       stand-in's version, and no tool was called.
   - In `tests/schema_drift.rs`, `pull_request_exports_and_tools_match_the_contracts` (decisions 1
     to 3): the item, Type, capability, operations and `write` are as decided, and the schemas
     the writer lists equal the contracts'. `repository_exports_and_tools_match_the_contracts`
     keeps the read front's.
   - In `engine/crates/shell/tests/github.rs`, `pull_requests_through_roots_write_conduit`, in
     `github_through_the_engine`'s form, its project its own or `project` extended: a second file
     secret, a 0600 file holding a second marker, and a `pull-requests.v1` entry for
     `octo/allowed`, base `main`, whose token is that secret's entry, with its declared Conduit
     listing `get-secret.v1`; root's Conduit `allowed-write` from
     `example.org/conduit/github#write`. The Pool's setup succeeds; create and merge through
     `allowed-write` succeed with the stand-in's outputs; the writer's stand-in has the second
     marker's digest and the writer's flags; the `request-ended` records of both writes hold
     their outputs; `conduit tools` lists no write for `allowed-read`; neither marker is in any
     byte under the store or of any output; no stand-in outlives serve.
   - An existing test changes only where it counts exports, contracts or tools this order adds,
     or pins the version (`github_through_the_engine`'s pin becomes 0.2.0). Report each.
9. **0.2.0 and the README.** `Cargo.toml`'s version, `Cargo.lock`'s own line and `conduit.yaml`'s
   `version` become 0.2.0, so that a store holding 0.1.0 is not refused this archive at the same
   version (PKG-025). No new crate or dependency. The README's title names the writes too, as
   `# Read one GitHub repository and create and merge its pull requests`, and its first sentence
   names version 0.2.0 and the `write` template. A section `## Create and merge pull requests`,
   before `## D15 inventory`, states: the configuration; that the Pool owns the write token's
   Resource and the `pull-requests.v1` Resource, which declares its own Conduit to the token
   listing `get-secret.v1`; that only root's Conduit from `#write` should name it, no Agent's,
   which the project declares and neither the engine nor the package enforces; the owner's steps:
   a second fine-grained personal token for this repository alone, Contents and Pull requests
   read and write, with an expiry, kept in the dedicated keychain or a 0600 file, the read token
   as it is (DEC-173); that GitHub's merge needs Contents write, which also lets whoever holds the
   token push to any branch the repository's rules allow, not only merge; the writer's flags,
   decision 2's, and that a release without `--tools` refuses the flag, so every write fails;
   decision 4's table, untested against a real server; decisions 5 to 7's outcomes, and that
   GitHub checks `expectedHead` again at the merge; and that an unconfined v0 Agent can reach the
   token's store with its own tools
   ([DEC-018](../decisions/log.md#dec-018-2026-11-20-what-an-agent-reaches-enforcement-in-v0-after-s2)).
   Rows (a) to (d) of its D15 inventory name the writer.

## Start check

From the repository root:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
git status --short | wc -l | tr -d ' '  # 0
git merge-base --is-ancestor 6d3a1f14 HEAD; echo "base $?"  # base 0
for n in 034 173; do sed -n "/^## DEC-$n, /,/^## DEC-/p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done | paste -sd' ' -  # 1 1
k packages/github/conduit.yaml 'version: 0.1.0' 'version: 0.2.0' 'command: [bin/conduit-github]' '--write'  # 1 0 1 0
k packages/github/Cargo.toml 'version = "0.1.0"'  # 1
k packages/github/Cargo.lock 'name = "conduit-github" version = "0.1.0"'  # 1
k packages/github/src/upstream_lifetime.rs '.args(["stdio", "--read-only", "--toolsets", "issues,pull_requests"])'  # 1
k packages/github/src/lib.rs 'pub const OPERATIONS: [Operation; 4] = [' 'pub const TOKEN_VARIABLE: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";'  # 1 1
k packages/github/contracts.yaml 'name: pull-requests.v1' 'name: pull-request-writing.v1' 'effect:' 'name: write'  # 0 0 0 0
k packages/github/README.md '`example.org/conduit/github`, version 0.1.0' '## Create and merge pull requests'  # 1 0
k engine/crates/shell/tests/github.rs 'GITHUB:{"version":"0.1.0"' 'async fn github_through_the_engine()'  # 1 1
k engine/crates/shell/tests/github.rs 'mod wait_support;' 'const WAIT: Duration'  # 0 1, or 1 0 with order 416 on main
(cd packages/github && grep -rhoE '"(create_pull_request|merge_pull_request|update_pull_request|update_pull_request_branch|pull_request_review_write|add_comment_to_pending_review|request_copilot_review|add_reply_to_pull_request_comment|push_files|create_or_update_file|delete_file|create_branch|issue_write|add_issue_comment|search_[a-z_]+)"' src | wc -l | tr -d ' ')  # 0
(cd packages/github && grep -rlE 'GH_TOKEN|GITHUB_TOKEN|hosts\.yml|\.config/gh|"gh"' src | wc -l | tr -d ' ')  # 0
grep -rlE 'fn (pull_requests_are_created_and_merged_in_the_configured_repository|a_merge_is_refused_unless_the_head_is_the_expected_commit|writes_are_unknown_unless_their_effect_is_known|the_writer_runs_its_own_server_and_keeps_its_token|a_missing_write_tool_or_property_fails_every_write|pull_request_exports_and_tools_match_the_contracts|pull_requests_through_roots_write_conduit)\(' packages/github engine/crates/shell | wc -l | tr -d ' '  # 0
grep -c "('packages/github', 'spec/'" tools/gate.py  # 1
command -v cargo >/dev/null; echo "cargo $?"  # cargo 0
```

Where a line prints otherwise, stop, change nothing and report it. An `upstream_lifetime.rs` line
other than `1` means the read front's server start moved since this order was written. The
`wait_support` line says which waits decision 8's engine test uses: `0 1`, the file's `WAIT` and
`timeout`; `1 0`, order 416's helper. Report which.

## Scope

Change, as the decisions say: in `packages/github`, `src/`, `tests/` (the stand-in included),
`contracts.yaml`, `conduit.yaml`, `README.md`, and the version lines of `Cargo.toml` and
`Cargo.lock`; and `engine/crates/shell/tests/github.rs`. Change an existing test only under
decision 8. Change nothing else: not `spec/`, `docs/` (DEC-173 stays not done), `tools/`, another
package, an engine source, `engine/CONFORMANCE.md` or `engine/Cargo.lock`; no promise changes, so
no changelog fragment. Write outside the tree only in temporary directories the tests make.
Report anything else that needs changing.

## Done when

The first group prints otherwise today and passes after; the second, guards, passes today and
after. Run each new test by name, once, then the package's tests once, last.

```sh
export PYTHONDONTWRITEBYTECODE=1
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | paste -sd' ' -; }
ok='^test result: ok\. 1 passed'
for t in pull_requests_are_created_and_merged_in_the_configured_repository a_merge_is_refused_unless_the_head_is_the_expected_commit writes_are_unknown_unless_their_effect_is_known the_writer_runs_its_own_server_and_keeps_its_token a_missing_write_tool_or_property_fails_every_write; do (cd packages/github && cargo test --offline --release --test mcp -- --exact "$t" 2>&1 | grep -c "$ok"); done  # 1 five times; 0 today
(cd packages/github && cargo test --offline --release --test schema_drift -- --exact pull_request_exports_and_tools_match_the_contracts 2>&1 | grep -c "$ok")  # 1; 0 today
(cd engine && cargo test --offline --release -p conduit-shell --test github -- --exact pull_requests_through_roots_write_conduit 2>&1 | grep -c "$ok")  # 1; 0 today
k packages/github/conduit.yaml 'version: 0.1.0' 'version: 0.2.0' '--write'  # 0 1 1; 1 0 0 today
k packages/github/Cargo.toml 'version = "0.2.0"'  # 1; 0 today
k packages/github/Cargo.lock 'name = "conduit-github" version = "0.2.0"'  # 1; 0 today
k packages/github/contracts.yaml 'name: pull-requests.v1' 'name: pull-request-writing.v1' 'name: write'  # 1 1 1; 0 0 0 today
k packages/github/README.md '`example.org/conduit/github`, version 0.2.0' '## Create and merge pull requests' 'stdio --tools list_pull_requests,create_pull_request,pull_request_read,merge_pull_request' | awk '{print $1, $2, ($3 > 0)}'  # 1 1 1; 0 0 0 today
k engine/crates/shell/tests/github.rs 'GITHUB:{"version":"0.1.0"' 'GITHUB:{"version":"0.2.0"' | awk '{print $1, ($2 > 0)}'  # 0 1; 1 0 today
```

```sh
export PYTHONDONTWRITEBYTECODE=1
ok='^test result: ok\. 1 passed'
(cd packages/github && grep -rhoE '"(update_pull_request|update_pull_request_branch|pull_request_review_write|add_comment_to_pending_review|request_copilot_review|add_reply_to_pull_request_comment|push_files|create_or_update_file|delete_file|create_branch|issue_write|add_issue_comment|search_[a-z_]+)"' src | wc -l | tr -d ' ')  # 0
(cd packages/github && grep -rlE 'GH_TOKEN|GITHUB_TOKEN|hosts\.yml|\.config/gh|"gh"' src | wc -l | tr -d ' ')  # 0
(cd engine && cargo test --offline --release -p conduit-shell --test github -- --exact github_through_the_engine 2>&1 | grep -c "$ok")  # 1: the reads, the read-only server's flags among them
(cd packages/github && cargo fmt --check); echo "fmt $?"  # fmt 0
(cd packages/github && cargo clippy --offline --all-targets -- -D warnings >/dev/null 2>&1); echo "clippy $?"  # clippy 0
(cd engine && cargo fmt --all -- --check); echo "engine fmt $?"  # engine fmt 0
(cd engine && cargo clippy --offline -p conduit-shell --all-targets -- -D warnings >/dev/null 2>&1); echo "engine clippy $?"  # engine clippy 0
n=$(git ls-files -o --exclude-standard -- packages/github engine); [ -z "$n" ] || git add -N -- $n
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
for t in check_docs check_private_paths check_schemas; do python3 tools/$t.py | tail -1; done  # Errors: 0, three times
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(packages/github/(Cargo\.(toml|lock)|conduit\.yaml|contracts\.yaml|README\.md|src/.+\.rs|tests/.+\.rs)|engine/crates/shell/tests/github\.rs)$' | wc -l | tr -d ' '  # 0
[ -z "$n" ] || git reset -q -- $n  # stage nothing
(cd packages/github && cargo test --offline --release 2>&1 | grep -E '^(test result|error)' | cut -d. -f1 | sort -u)  # test result: ok, the package's tests, once, last
```

`/usr/bin/python3` is the Mac's interpreter with PyYAML. Your report gives, per decision, what
changed and where; each new test's line on the unchanged package, then its passing line; the
stand-in's record of one create and one merge (no digest); each existing test you changed, and
why; your own choices; and every check's last line. Stage nothing.

## What follows

- **The owner's sitting**, before live use: make the second token as decision 9 says, store it,
  and register its secret Resource and the writer in the dogfood's project. The table and the
  `--tools` flag are checked against the installed server's version at the first live merge.
- **The owner's question at the sitting**, the write token's permissions: "The write token holds
  Contents and Pull requests write, since GitHub's merge needs Contents write, and so it can push
  to any branch the repository's rules allow." Options: accept it as is, the writer calling four
  tools alone; or accept it once a rule on `main` lets only pull-request merges update it. The
  answer is a new decision beside DEC-173.
- **Order 413** pushes the branch that a create names (DEC-034's push slice); which token its
  working copy references is its own decision.
- **Order 414** moves the queue onto root's `write` Conduit: it creates and merges through it,
  reads the pull request through `read` before anything else after an unknown, and `gh` leaves
  the tool.
- A GitHub App's installation token, short-lived and scoped to one repository's permissions, can
  later replace the personal token, as DEC-209 takes temporary credentials for AWS.
- DEC-173's status is the coordinator's once this order and 414 merge.

## Record

Run on 2026-11-24 through Conduit itself, in Pacific time, on `main` at its start. Drafted and
checked by the coordinator's Workflow; built and read on the team seats (DEC-179, DEC-180).

**The owner's ruling.** The draft found that merging a pull request needs Contents write as well
as Pull requests write, past DEC-173's words; the owner ruled DEC-212 before dispatch: Contents and
Pull requests write on this repository alone.

**Worker-4's turn**, on the Business seat, built the two operations, the writer's server limited to
four tools, and the stand-in's modes, raising the github package to 0.2.0; every test ran against
the stand-in, offline, with no real token.

**Two blind reads.** The Codex verifier, `root-verifier-4.turn.v1`, passed it with no finding. The
Claude reader, `root-reader-a.turn.v1`, on the Team seat, found decisions 1 to 9 applied, with
three shoulds and two nits on the README and the merge's results. One fix turn by worker-4 answered
all five. The gate's run on the squash is the coordinator's, at the merge.
