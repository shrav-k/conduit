# Work order 138 (A63-RK): a runtime's own reason for a failed staging or start

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token or a private key, even one that a test made.

## What and why

The runtime-provider kit, `packages/runtime-provider-kit/` (orders 122, 124, 126, 129 and 130),
answers a backend's failed staging with the fixed text "staging failed", and a failed start with
"start did not complete" and what became of the tree (`src/server.rs`, `run.v1`, near line 616,
and `start_failure_policy`, near line 229). The backend's error type, `Unavailable`
(`src/backend.rs`, near line 9), carries nothing, on purpose: a backend's own error text can hold
credentials. So a caller learns that a run failed and never why, though the runtime knows: a
program its `PATH` cannot resolve, a variable whose name the runtime cannot pass, a machine whose
setup lacks a setting. Ruling 52 asks that every refusal be as explicit as possible for an agent
caller. Order 137's SSH runtime has to write such reasons to its own standard error for now.

This order lets a runtime give the kit a reason it wrote itself, and the kit put it in the answer,
checked against the run's sensitive values as the kit already checks a gate's URL. Read the kit's
README and `src/server.rs` first.

## The decisions it applies

1. **A reason on the error.** A backend can attach a reason to the error it returns, for example
   `Unavailable::because(text)`; the error with no reason still means what `Unavailable` means now.
   Every existing use must compile and behave unchanged: as a value (`Err(Unavailable)`,
   `map_err(|_| Unavailable)`, `ok_or(Unavailable)`), as a type (`Result<T, Unavailable>` and the
   kit's `Result<T>`), and in comparisons. A form that does this: a braced struct with a private
   field holding the optional reason, and a constant `Unavailable` of that type with no reason,
   allowed past `non_upper_case_globals`; a tuple struct cannot, since its constructor takes the
   name. Use the error in comparisons only, never as a pattern. Write its `Debug` by hand, printing
   exactly `Unavailable` with or without a reason, so that no `unwrap` or `{:?}` ever shows one.
   The proof is `packages/provider-local-process/` (order 127), the kit's one runtime on `main`: it
   builds and passes its tests with no change to any of its files, its `Cargo.lock` included. So
   keep the kit's version at 0.1.0 and add no dependency to it. If that crate cannot build
   unchanged, report why rather than change it.
2. **What a reason is,** stated in the README and on the type: text the runtime writes itself,
   naming what failed (a setting, a variable's name, a program, a machine's hostname) and never a
   value; never an operating-system, library or remote tool's error text passed through.
3. **What the kit does with it,** in one function, before any answer holds it, in this order:
   - **Withhold.** The sensitive values are the non-empty values of the run's identity variables,
     identity files, secret variables and secret package arguments: the set `sensitive_url`
     checks a gate's URL against (`src/server.rs`, near line 796). Both checks use one helper over
     that set. The whole reason is withheld, and a fixed text put in its place saying that the
     runtime's reason was withheld because it contained a value supplied to the run, where the
     reason as given contains any of those values, or where the reason made one line (below)
     contains any of them made one line the same way. A fragment of a value is not caught; the
     README says so. The run's ordinary environment values and the item's arguments are not
     secrets by their schema, and short ones would withhold most reasons: they do not count, and
     the README says so too.
   - **Then clean and cut.** Make it one line, replacing each control character and U+2028 and
     U+2029 with a space; cut it to at most a stated length on a character boundary (your choice;
     report it). A reason empty or blank after this counts as no reason.
4. **Where it reaches,** and only there, as text content, never under Conduit's `_meta` reason key
   (which the README rules out for diagnostics):
   - `run.v1`'s answer when staging fails: "staging failed: " and the reason;
   - when the backend's `start` returns an error with a reason (`src/server.rs`, near line 648):
     "start did not complete: " and the reason, then "; tree stopped" or "; stop could not be
     confirmed and persisted", as `start_failure_policy` decides. `failed_start` and
     `start_failure_policy` take an optional reason; the other callers of `failed_start` (near
     lines 637, 653, 662, 670 and 675) pass none.

   The outcome of each answer does not change. With no reason, every answer's text stays exactly
   as it is now. A reason that `Gate::open`, the state store, `stop`, `state` or `inventory`
   attaches is dropped. The kit writes a reason to the ledger, the state store, a record or a log
   of its own nowhere, and holds it only until the answer is built; like the URL, the answer that
   carries it can reach rmcp's debug log (README, "Runtime interfaces"), which is why it passes the
   check in decision 3.
5. **The text it replaces.** Rewrite what says the error carries nothing: the README's sentence
   that backend errors are the payload-free `Unavailable`, so that their text "cannot become an
   answer" (near lines 33 to 36), the comment on `Unavailable` (`src/backend.rs`, near line 9),
   and the comments near lines 227, 228 and 348 of `src/server.rs`.
6. **Tests,** with the fake backend in `tests/support` (add the hooks it needs there):
   - a staging failure and a start failure each with a reason, in both start outcomes, the answer
     carrying it where decision 4 puts it;
   - for each of the four sensitive sources in turn, a reason holding that source's whole value
     as the fake gives it, and one holding a multi-line identity file's content after the reason's
     newlines are made spaces, each answer giving the withheld text and holding no marker; compare
     with `assert!(a == b)`, never `assert_eq!`, so that a failing test prints no marker;
   - a reason with a newline, other control characters, U+2028 and U+2029; an overlong reason,
     with multi-byte characters at the cut; a blank reason;
   - each failure without a reason, with today's exact text, and every other caller of
     `failed_start` unchanged;
   - `Debug` of an error with a reason printing `Unavailable`; and the ledger and state store
     holding no reason after each.

   Keep every test of orders 122, 124, 126, 129 and 130 passing.
7. **What this order does not do:** change any runtime (orders 127 and 137's runtimes take up
   reasons in their own orders), give reasons a path from `Gate::open`, the store, `inventory`,
   `stop` or `state`, or change anything outside the kit.

## Scope

Change only `packages/runtime-provider-kit/`.

## Done when

In `packages/runtime-provider-kit/`: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`. In
`packages/provider-local-process/`, changing nothing there:
`cargo clippy --all-targets -- -D warnings` and `cargo test --release`; then, from the repository
root, `git status --porcelain packages/provider-local-process` prints nothing. From the repository
root also: `python3 tools/check_docs.py` and `python3 tools/set_names.py --check`. Run these with
`bash`. Each prints nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep -E "target/" | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report the length bound and the error type's form as your own
choices. Do not stage anything.

## Record

Run on 2026-11-12 on the Mac, from base a005dab: the worker (647 s) and a fresh verifier (281 s),
then a second worker round (278 s) and a fresh verifier (201 s), which found nothing to change.
Round one's should: both multi-line regressions still passed with the one-line comparison removed,
since another identity file held a whole value; round two made each fixture need that comparison
and showed that each fails without it.

The result: `Unavailable` is a braced struct with a private optional reason, with a reasonless
constant of the same name and a hand-written `Debug` that prints only `Unavailable`; a backend
attaches a reason with `Unavailable::because`. The kit checks the whole reason, as given and made
one line, against the run's identity variables and files and its secret variables and package
arguments, withholding it entirely on a match, then makes it one line and cuts it to 512 Unicode
scalar values; a blank reason counts as none. It reaches only `run.v1`'s answers for a failed
staging and a failed backend `start`, as text. Order 127's `provider-local-process` builds and
passes its tests unchanged, its `Cargo.lock` included. The lane coordinator's review found nothing
to change.

The worker's own choices: the bound of 512, the withheld text "runtime's reason was withheld
because it contained a value supplied to the run", and the tests in a submodule of the runtime
suite.
