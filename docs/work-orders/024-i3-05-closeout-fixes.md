# Work order 024 (I3-05): what the first run with the real clients found

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests
that bind a loopback port; use it for nothing else. You cannot run `claude` or `codex` as
clients here: the admin reruns the closeout with them afterwards.

## What and why

On 2026-11-10 the admin began increment 3's
[closeout](../proposals/first-build/increment-3/closeout.md) with the real Claude Code 2.1.318
and Codex 0.167.0, on a fresh store serving the live S1 project. The engine did its part: each
Agent's engine list was exact, A's read of the shared file went through the engine with its
records, no token and no turn's text reached a record, and stopping the server left no
process. Three things failed, each the admin's to settle under ruling 46, point 4, and each
settled below.

1. **Claude Code was shown `{}` for a file it read.** The engine's reply held the file's text
   in `content` and, as IFC-025 and IFC-033 require of an operation that declares no outputs,
   an empty map in `structuredContent` under an `outputSchema` describing a map with no
   properties. Claude Code gave its model the structured result and not the text, so the model
   reported the read returned nothing. The Model Context Protocol makes `outputSchema`
   optional and has a client that sees one prefer `structuredContent`; the specification's
   choice of an empty `outputSchema` for an operation with no outputs therefore hides the
   content of every such operation, the reads of a `plain` server first among them, from the
   most used client. Finding 3-03.
2. **Claude Code loaded a `CLAUDE.md` from an ancestor of its private working directory.** The
   wrapper passes `--setting-sources project`, under which Claude Code discovers project
   instruction files. A sentinel planted there reached the model, after a baseline run showed
   it would. `--setting-sources ""` excluded it in a direct trial, as did `--safe-mode`.
   Finding 3-04.
3. **Codex went around the engine.** Asked to read and write the shared directory "using your
   tools", Codex read the file with its own shell, in its read-only sandbox, made no request
   of the engine, and said it could not write. It also loaded an `AGENTS.md` from an ancestor
   directory, which `--ignore-rules` does not stop. In a direct trial, Codex with its
   features `shell_tool`, `unified_exec`, `code_mode_host`, `browser_use`,
   `browser_use_external`, `in_app_browser`, `computer_use`, `multi_agent`, `hooks`,
   `plugins`, `view_image`, `goals` and `image_generation` disabled, and with
   `project_doc_max_bytes=0`, answered that it could not read a file outside the engine,
   naming its disabled execution host, and loaded no `AGENTS.md`. Finding 3-05.

## The decisions to apply

1. **Finding 3-03, in the specification.** An operation that declares no outputs is presented
   as a tool with **no** `outputSchema`, and a result of it carries **no** `structuredContent`;
   its `content` is as IFC-025 gives it, a `plain` result's content unchanged. An operation
   that declares outputs keeps its `outputSchema`, and its results their `structuredContent`,
   as now. Change IFC-025 and IFC-033 (and any other promise that says an operation with no
   outputs has an empty map, such as the passage in `spec/conventions/operations.yaml` on a
   repeat), giving the reason in each promise's `observable`: the protocol makes the schema
   optional, a client that sees one may prefer the structured result, and an empty map would
   hide the content. Change the core so it presents and answers that way, and every vector
   that expects the old form; add one vector that fails before the change and passes after,
   for a `plain` read whose text is the result's only content. Log 3-03 in the log of
   increment 3 as settled, with the spec's change.
2. **Finding 3-04, Claude Code's wrapper**: `--setting-sources ""` in place of `project`.
   Nothing else the wrapper relies on may come from a setting source: the engine's server,
   the header helper and the allow rule must reach Claude Code through `--mcp-config` and
   `--settings` alone. Test the arguments.
3. **Finding 3-05, Codex's wrapper**: disable every feature named in finding 3-05, by
   `--disable`, and set `-c project_doc_max_bytes=0`. `accountServices` true enables `apps`
   alone, as now; nothing in this order turns the others back on. Write in each package's
   README the client's own tools that stay on (for Codex, those the trial still listed, such
   as `apply_patch` in its read-only sandbox, and its request for user input), as routes the
   closeout must report. Test the arguments.
4. **The Agent's own machine tools are off by default.** Ruling 48's point 1 turned off the
   Agent's own account services and messaging. The admin settles, under ruling 46, point 4,
   that the same holds for a client's own tools that reach the machine, a shell, code
   execution, a browser, another Agent, hooks and plugins, which Claude Code's wrapper already
   does with `--tools ""` and Codex's now does by decision 3. Say so in each Type's description
   and README, and in `CONFORMANCE.md`.
5. **Pins and versions.** Bump each package's version to 0.1.1, rebuild its pinned copy under
   `conformance/fixtures/agents/`, and update the live project and the vector that registers
   it.
6. **The closeout plan**: update `docs/proposals/first-build/increment-3/closeout.md` for
   decisions 1 to 4: a sentinel in an ancestor directory and its baseline; Codex asked to read
   a file outside the engine; the read's text reaching the model.

## Scope

Create and change: `spec/` for decision 1 only, `reference-model/`, `conformance/`,
`packages/agent-claude-code/`, `packages/agent-codex/`,
`docs/proposals/first-build/log-increment-3.md` and
`docs/proposals/first-build/increment-3/closeout.md`. Report anything else that needs changing.

## Done when

The core presents no `outputSchema` and answers no `structuredContent` for an operation with no
outputs, by spec and vector; Claude Code's wrapper loads no setting source; Codex's wrapper
disables the features of finding 3-05 and loads no project document; the pins and the closeout
plan are current. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass; in `reference-model/` and each
package directory, `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` pass, and in `reference-model/`, `CONDUIT_TEST_NPM=1 cargo test --release`.
Check the files you create for trailing white space yourself; do not stage anything.

## Record

Run on 2026-11-10: the worker in 32 minutes, then a fresh verifier in 5, which found nothing
to fix. The worker logged findings 3-03 to 3-05 as settled. It did not run the suite with
`CONDUIT_TEST_NPM=1`, reading this order's network grant as not covering npm; the admin ran
it. The admin also rewrapped the spec paragraphs the worker had left with long lines, without
changing their content (each YAML file loads to the same value). The admin reran every
check: 136 Rust tests in `reference-model/` with `CONDUIT_TEST_NPM=1` and six in each package
pass, and clippy, fmt and the five repository checks are clean. The admin reruns the closeout
with the real clients next.
