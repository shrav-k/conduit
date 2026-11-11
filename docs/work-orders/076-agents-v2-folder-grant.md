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
beside the `.v1` Types, with an input `ownTools` (default true). [Ruling 75](../proposals/ruling-75/README.md),
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
