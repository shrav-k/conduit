# Work order 076 (P2b): in the `.v2` Agent Types, the folder is the grant

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access, to fetch
crates; use it for nothing else. Never print a whole large file: use `head`, `tail`, `grep` or a
count. Never run `env`, `printenv`, `set` or `export -p`, and never print an environment variable's
value.

## What and why

Your tree is order 050a's branch, not `main`: order 050a added `codex.v2` and `claude-code.v2`
beside the `.v1` Types, with an input `ownTools` (default true). [Ruling 75](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-75/README.md),
point B1, replaces that switch before 0.2.0 publishes: **an Agent has its client's own tools
(shell, code execution, file edits, network) exactly when its template names the directory it
works in.** An Agent whose `directory` is null has exactly `.v1`'s tools and sandbox, as
`ownTools: false` gave. There is no switch. Point B2 adds that limits on this reach (hosts, a
read-only directory) will live on a confining runtime provider chosen with `runtime:`, never as
Agent inputs. Read 050a's order (`docs/work-orders/050a-agents-v2-own-tools.md`), ruling 70, ruling
74 and ruling 75 first.

## What to do

1. **The `.v2` schemas, both packages.** Remove `ownTools` from the properties and from `required`.
   Keep `directory` (`[string, 'null']`, default null) and every other `.v2` input as 050a wrote
   them. Do not touch the `.v1` documents: they stay byte for byte.
2. **How the wrapper tells `.v2`.** Today it tests for `AGENT_OWN_TOOLS`. Use the variable of
   `turnDeadlineSeconds` instead, which every `.v2` Agent has and no `.v1` Agent has. Own tools are
   on exactly when the directory's variable is present: PRV-007 gives no variable for a null value.
   Keep 050a's checks on the directory: absolute, an existing directory, canonicalized.
3. **Hosted search.** 050a turned on Codex's `web_search` and Claude Code's WebSearch only when
   `accountServices` and `ownTools` were both true. Make it `accountServices` true and a directory
   named.
4. **Contract text and READMEs.** Replace every mention of `ownTools`:
   - the Agent's own tools are on where the template names its directory;
   - they are not confined in v0: they run as the engine's user, with the client's sandbox and
     approval prompts off;
   - limits will come from a confining runtime provider chosen with `runtime:` (ruling 75, B2).

   Remove the sentence that calls the switch temporary.
5. **Tests.** Change the scripted-client tests that set `ownTools` to set or omit the directory.
   Keep a test that a `.v2` Agent with no directory launches with exactly `.v1`'s client
   arguments and tools, and one that a directory turns them on. Keep 050a's other tests passing.
6. **Step 11 of order 050a** (the manifest checked with the engine): run it again with the new
   schemas, and report its result.

## Scope

Change:

- `packages/agent-codex/` and `packages/agent-claude-code/`: their `.v2` contract documents,
  `src/`, tests and READMEs.

Do not change:

- the `.v1` documents;
- `engine/`, `spec/` or `conformance/`;
- any record.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`
- `git diff --check`

In each of `packages/agent-codex/` and `packages/agent-claude-code/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`.

Also:

- the engine check of order 050a's step 11;
- `grep -rn "ownTools" packages/` prints nothing.

Report the totals passed and failed. Do not stage anything.

## Record

Run on 2026-11-11 on the Mac, from order 050a's checkpoint `bb75e294` rather than from `main`, as
the order asks: one round, a Codex worker (1083 s) and a fresh Codex verifier (266 s), which found
no must or should finding and listed no finding at all. It did not rerun the Rust tests or the
engine registrations in its read-only sandbox, and it noted that the worker's two Step 11 failures
remain unresolved. The round is checkpointed on `wo/076-agents-v2-folder-grant` (`228b5dea`, on top
of `bb75e294`) and is not merged. Because the branch carries 050a's tree, its diff against `main`
also holds 050a's work (18 files in all, 10 of them changed by this order), and this order's own
file is not on it: it reached `main` in #162.

In both packages the `.v2` Types (`codex.v2`, `claude-code.v2`) no longer have `ownTools`: it is
gone from their properties, from `required` and, as `AGENT_OWN_TOOLS`, from `conduit.yaml`. The
`.v1` documents stay byte for byte. The wrapper recognizes a `.v2` Agent by
`AGENT_TURN_DEADLINE_SECONDS`, and turns on the client's own tools exactly when the directory's
variable is present. With a directory, Codex runs with `danger-full-access` and its shell, unified
exec and image-viewing features, and Claude Code runs with `bypassPermissions` and Bash, Read,
Edit, Write, Glob, Grep and WebFetch. The directory must still be absolute and an existing
directory, and it is now canonicalized. Hosted search (Codex's `web_search`, Claude Code's
WebSearch) needs `accountServices` true and a named directory. The contracts and READMEs say that
the tools are on where the template names its directory, that nothing confines them in v0, and
that limits will come from a confining runtime provider chosen with `runtime:` (ruling 75, B2). The
sentence that called the switch temporary is gone. With `directory` null the tools and sandbox
match `.v1`'s. The worker applied this decision by its "nearest sound reading": Codex still passes
050a's `.v2` `--cd` and MCP timeout arguments there, and its regression test allows for them. The
worker's own choices: an internal `v2` flag in place of `Config::own_tools`, and new tests for
symlink canonicalization, a path that is not a directory, and an empty deadline.

The worker's checks: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check` and
`git diff --check` pass. `cargo fmt --check` is clean in both packages. `cargo clippy` failed at
first (exit 101) because sccache could not write in the sandbox, and passed with `RUSTC_WRAPPER`
unset. With it unset, `cargo test --release` passes: 22 passed and 0 failed, by the worker's count.
`grep -rn "ownTools" packages/` prints nothing. 050a's step 11, rerun with the new schemas, gave 2
passed and 2 failed. In each package the valid templates are accepted with no diagnostic, but the
template-only registrations with `turnDeadlineSeconds: 604801` are also accepted with no diagnostic.
The worker added supplemental instance checks. After it relaxed its own evaluator to allow
informational NAM-008 notes, they gave 4 passed and 0 failed: the valid instances are accepted, and
the oversized deadline is rejected with INS-014, naming `turnDeadlineSeconds` and `maximum`.

The worker's finding, and what it could not do, kept for later:

- 076-01 (ambiguity; INS-014, MAN-017, SCH-009): step 11 expects the template-only submissions with
  `turnDeadlineSeconds=604801` to be rejected, and both were accepted without diagnostics. INS-014
  describes validation at instantiation, so the coordinator must reconcile the check's expectation
  with template validation. The worker left the engine unchanged.
- It could not read ruling 75's record, which is not on this branch, so it applied B1 and B2 from
  the order's text. The record and the matching IN_FORCE and status updates need integration
  outside this order's scope.

### The last verifier's findings

- None: the report's `findings` list is empty. Its verdict, verbatim: "No additional faults found.
  Documentation, names, formatting, whitespace and .v1 preservation checks passed. Rust tests and
  engine registrations were not rerun in this read-only sandbox. The worker’s two reported Step 11
  failures remain unresolved."

### Next

The handover says the packages of 050a, 076 and 078 stay unmerged until order 050, because NAM-024
forbids adding a default once `.v2` is published. They merge with 050 as one 0.2.0, and until then
Conduit's own Agents (A78) take their packages from this branch. Finding 076-01, step 11's
expectation against INS-014, is still open for the coordinator.
