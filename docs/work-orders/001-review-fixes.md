# Work order 001: the review's fixes, and the tokens of a pattern

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `001-review-fixes.schema.json` beside this file.

## What and why

After increment 1 of the first build (the strict loader, crate `conduit-loader` in
`reference-model/crates/loader/`) and the baseline for S1 (A70), a review read both against the
specification. It found faults in the coordinator's own settlements of increment 1's findings,
and the owner ruled on the tokens a `pattern` may use (ruling 48, point 3). This order applies
those decisions to the promises, the loader and `tools/check_schemas.py`, with vectors, and
logs them.

Read first: `docs/proposals/first-build/log-increment-1.md` (all of it),
`docs/proposals/first-build/README.md`, `docs/proposals/ruling-46/README.md` (point 4),
`docs/proposals/ruling-48/README.md`, `spec/README.md`, `spec/schema-profile.md`, and the
promises named below in `spec/promises/`, `spec/conventions/`, `spec/interface/` and
`spec/implementation-defined.yaml`. Match the form of a promise and the plain style of the text
around each change; write in the terms of `spec/definitions.md`.

## The decisions to apply

Each is the coordinator's decision under ruling 46, point 4, unless it says ruling 48. Do not
reopen them.

**A. A rejection by a schema cites SUB-005 everywhere (completes finding 1-01).** SUB-005 now
says that a document or manifest a Schema contract rejects is rejected under SUB-005. At least
nine promises still name another rule for a case that a schema decides: CON-020, POOL-009,
POOL-012, MAN-012, MAN-007, CTR-008, OBJ-015, OBJ-019 and TPL-001. Search all of `spec/` for
every other "rejected under X" (and like wording) where a Schema contract makes the rejection,
and list each in your report. Reword each so the requirement stays and the rule it is reported
under is SUB-005, for instance "is rejected by `declared-conduit.v1`, under SUB-005"; keep any
rule that a schema does not decide. Extend SUB-005 so that a document of a package rejected by
a schema when the package is installed is reported under SUB-005 too, with the request
reporting invalid under PKG-023, consistently with MAN-011. Add to SUB-005's observable that
the diagnostic carries the failing keyword's location as JSON Schema 2020-12's output format
names it (`keywordLocation`, and `absoluteKeywordLocation` where known) in the `properties` of
its SARIF result; check `diagnostic.v1` under `spec/`, and if it does not allow that, allow it
there in SARIF's own form. Have the loader's `Diagnostic` carry that location for a schema
rejection, with a test. In the log, say that the thesis "the core cites a promise for every
decision" is met for the rules a schema encodes through SUB-005 and the keyword's location.

**B. A byte order mark in JSON is accepted and ignored (corrects finding 1-05).** RFC 8259,
section 8.1, lets a parser ignore one, and files saved on some systems begin with one. Change
the profile's Encoding row so that a file in either syntax may begin with one, and so that
UTF-8 is stated as the profile's own rule, citing I-JSON as its precedent, which removes the
clash with the Value model row's "Two of its rules, and no other". Change SUB-005's wording if
it repeats the old rule. The loader and `check_schemas.py` skip a leading byte order mark in a
JSON file. Add tests for both.

**C. SUB-011 gets a floor and a wider reach (corrects finding 1-11).** Every engine accepts
values nested at least 64 levels deep; the loader's limit stays 128. Say it in SUB-011 or in
DL10, and keep every example and vector within it. Limits on a file's size, on the number of
documents or files in a submission, and on a package's archive also fall under SUB-011; at
installation the request reports invalid under PKG-023 with that diagnostic. Narrow "a limit
that the conformance document does not state is not applied" so that it does not reach the
exhaustion of storage, which DL10 leaves to the engine.

**D. A pattern keeps to JSON Schema's interoperable tokens (ruling 48, point 3; settles
1-04).** Amend SCH-016: a `pattern`, and a key of `patternProperties`, uses only the tokens
that JSON Schema 2020-12, core, section 6.4, asks schema authors to keep to, quoted from the
standard, plus a backslash before a character that has a meaning in a pattern, so that it
stands for itself. It is still read as ECMA-262 in Unicode mode. A pattern that uses anything
else is rejected as finding 1-03 settled. Then:

- Rewrite every pattern in the specification's own schemas (under `spec/schemas/`, `spec/core/`
  and anywhere else under `spec/`) that uses anything else, so that it keeps to the tokens.
  The review found lookaheads in `address.v1`, `qualified-reference.v1`, `destination.v1` and
  three places in `templates.yaml`; rewrite each with `allOf` and `not` and a second pattern
  where that says the same. It found `[^\s]` in `server-package.v1` and `server-remote.v1`;
  write out ECMA-262's white space and line terminators in the class instead, exactly. Search
  for every other `\d`, `\w`, `\s`, `\b`, `.`, lookaround, back-reference, named group or
  `\p{..}`.
- No verdict may change. Add a test to the loader that matches each old pattern and its
  rewrite, both with `regress`, against every string value in the examples and against
  generated strings (a few thousand per pattern, including the edge cases of each), and
  requires the same answer.
- The loader checks that an authored pattern keeps to the tokens, replacing its checks of
  ECMA-262's later editions; keep `regress` for matching. `check_schemas.py`'s rule
  `ecma-pattern` checks the same token list, by a small tokenizer, and keeps `ecma()` for the
  meaning of `$` and of any class.
- Vectors: keep `invalid-by-rule/SCH-016-pattern-that-is-not-ecma-262.yaml`, and add
  `invalid-by-rule/SCH-016-pattern-outside-the-interoperable-tokens.yaml`, whose pattern is
  valid ECMA-262 but uses a lookahead. Both the loader and the tool reject it.
- In the log, 1-04 is settled by ruling 48, point 3.

**E. The note on Codex's token is wrong.** In `docs/proposals/first-build/README.md`, "Noted
for later increments" says that Codex takes an MCP server's bearer token only from a variable,
so increment 3 needs an adapter. Codex 0.167.0 also has `mcp_servers.<id>.http_headers_helper`,
a local command that prints a JSON object of headers, as Claude Code has `headersHelper`. Both
can read `token` from `CONDUIT_IDENTITY_DIR` by a one-line helper, and no adapter is counted in
advance. Say what is still to confirm, by A4 or A71: whether Codex runs the helper again after
a 401, and whether its "locally connected" servers include an engine at a loopback address. Add
one more note for increment 3: when a `plain` server's `tools/list` lacks a tool that its
item's `tools` map names, whether the request reports `failed` or `unavailable` is to be
settled there, with a vector.

**F. What an Agent reaches by its own means is outside the documents.** In `spec/README.md`,
where it says what is deliberately outside the specification, add: an Agent's own means, its
built-in tools, its shell and sandbox, its network, and the services of the account it signs
in with, are outside the documents; inspecting an instance shows its reach through Conduit
only; confining an Agent is a runtime provider's, under PRV-006, and every restriction in this
version is reported as mediated, under ACC-027. One or two sentences.

**G. What a pin holds.** In `spec/README.md`'s limits, or in MAN-010's observable if it fits
better there, state that the hash of a package file does not pin what its registry resolves
for the package's dependencies, and that a `command` pins nothing.

**H. The log.** In `docs/proposals/first-build/log-increment-1.md`, add a section "After the
review of 2026-11-09", with one finding per item A to D and G, numbered from 1-14 on, each with
its class, its promises, what changed, its vector, and "The owner can strike it" (D: "Settled
by ruling 48, point 3"). Update the table, the summary and the README's table of logs. Keep
every count in `spec/README.md`, `spec/schemas/README.md`, `docs/AUDIT.md` and
`docs/implementation/INDEPENDENT_BUILDS.md` right, as `check_schemas.py` reports them.

## Scope

You may change: anything under `spec/`, `tools/check_schemas.py`, anything under
`reference-model/`, `docs/proposals/first-build/`, and only the counts and the lines about
promises in `docs/AUDIT.md` and `docs/implementation/INDEPENDENT_BUILDS.md`. Change nothing
else; report what else needs changing.

## Done when

Every decision above is applied, or your report says why one could not be. These pass, from
the repository root:

```sh
python3 tools/check_docs.py
python3 tools/set_names.py --check
python3 tools/check_schemas.py
python3 tools/check_roadmap.py
git diff --check
(cd reference-model && cargo test --release)
(cd reference-model && cargo clippy --all-targets -- -D warnings)
(cd reference-model && cargo fmt --check)
```

`git diff --check` sees only tracked files: check the files you create for trailing white space
yourself. Do not stage anything.
