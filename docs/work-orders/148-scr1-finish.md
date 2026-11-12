# Work order 148 (SCR1, finished): case-folded names, invalid-script reports and redaction

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else, and reach no real service. Start no
real Codex or Claude client and use no login. Run one cargo command at a time, and never print a
whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never print a key or a token, even a synthetic one that a test
made.

## What and why

[Order 142](142-scr1-scripted-client.md) built S4's scripted client,
`experiments/015-s4-partial-setup/scripted-client/`, in two rounds. Your working tree begins at its
last commit. Its final verifier left three musts; this order fixes them. Order 142's decisions all
still apply: read it, then the crate's `src/script.rs` and `src/record.rs`.

## The decisions it applies

1. **Names compared as the file system compares them.** A trace named `Held.release` collides with
   the barrier `held`'s release file on a case-insensitive file system, as macOS's usually is, and
   so could release the barrier without the harness (`src/script.rs`, near line 196). The script's
   names are ASCII, so compare the trace's file name against every barrier's handshake file names
   (`.reached`, `.release`, `.acknowledged`) with ASCII case folded, by file name only, whatever the
   directories (comparing whole paths misses aliases that `canonicalize` keeps apart), and refuse
   the script as invalid, naming the field, on any match. Test a mixed-case collision for each
   suffix.
2. **An invalid script still completes the turn.** Where a script is invalid, its `redact` list has
   not been checked, so a short entry, such as one equal to a protocol key, can corrupt the events
   the wrapper reads and the turn fails without the report (`src/script.rs`, near line 79). When
   reporting an invalid script, apply only the `redact` entries that meet the rule order 142 gives
   them (at least 8 characters); no token exists yet, since an invalid script never runs the helper.
   Test, through the wrapper's launch test where it runs and directly otherwise, short entries equal
   to protocol keys: the turn completes and its summary reads `invalid`.
3. **Redaction that keeps valid JSON.** A marker inside a number escapes the trace's redaction, and
   a second pass over the summary's serialized text can make it invalid JSON (`src/record.rs`, near
   line 45). Redact each JSON value before it is serialized: a string leaf by replacement, and a
   number whose text holds a marker or the token by replacing the whole number with the string
   `"[redacted]"`; object keys likewise. Never rewrite serialized text. So `emit` (`src/record.rs`,
   near lines 58 to 62) writes the stdout protocol lines unredacted, since their strings are fixed
   and the thread ID comes from the argv; the summary is redacted as a value before it is serialized
   (`src/main.rs`, near line 543), and the trace per value, as now. The summary stays valid JSON,
   and a well-formed `redact` entry equal to a protocol word, such as `turn.completed`, or holding
   JSON syntax, such as `null,"cause`, can no longer break the turn. Test markers inside numbers in
   a trace line and in the fields the summary copies (the test server may echo a numeric argument
   into `_meta`'s `restriction` for this), and that both stay valid JSON; and an invalid and a valid
   script each with `redact` holding `turn.completed`, and with `null,"cause`: each turn completes
   and its text parses.
4. **Nothing else changes.** Keep every test of order 142 passing.

## Scope

Change only `experiments/015-s4-partial-setup/scripted-client/`.

## Done when

First, in `packages/agent-codex/`, `cargo build --release --locked --bin agent-codex`. Then, in the
crate: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --release`
twice: once without `SCR_AGENT_CODEX`, and once with it set to that binary's absolute path, when
the launch test runs. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/check_private_paths.py` and `git diff --check`. Run these with `bash`. Each prints
nothing when it passes:

```bash
D=experiments/015-s4-partial-setup/scripted-client
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" \
  $D --exclude-dir=target
grep -rn -E 'env::var|\.codex|\.claude|CONDUIT_' $D/src
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
```

`target/` stays ignored and unstaged. Report each choice you make as your own. Do not stage
anything.
