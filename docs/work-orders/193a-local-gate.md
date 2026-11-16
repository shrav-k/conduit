# Work order 193a: the local gate, with the Python checks, the commit identity and the secret scan

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It runs no cargo command. Run the gate one run at a time. Install nothing, gitleaks included.
Never print a whole large file or log. Never run `env`, `printenv`, `set` or `export -p`, never
print an environment variable's value, and never read or print a key, a token or any credential.
The only file of private names you make is the one Done when makes, holding planted names; never
print its content. Write no real user, host, machine or account name, and no email address
outside a reserved domain such as `example.invalid`, into a test.

## What and why

On 2026-11-16 the owner answered the step that
[order 191](191-log-marks-closed-settings-written.md) put to the owner for the full review's D11,
required status checks on `main`, with a question: "Do we really need CI or these checks at all
right now at this stage of v0?" Asked how CI should read `CONDUIT_PRIVATE_NAMES`, the owner
answered: "For now, I'd like to understand what can be run as local scripts before merge rather
than GitHub ci. Eventually we will have to move everything to ci, but for now I think it's best if
we design and run our own gates and formalize them into GitHub processes later". Offered the
choices, the owner chose to build `tools/gate.py`, to keep GitHub CI and not wait for it, and to
keep the Linux leg in GitHub CI, not blocking. [Ruling 80](../proposals/ruling-80/README.md),
point 10, records the answers: before each merge a local gate runs the checks on the
coordinator's machine and the merge waits for it; CI keeps running on every pull request as a
free second opinion and the Linux leg; merges no longer wait for it; and a run that fails after a
merge is fixed in a follow-up. Its "What follows" gives `tools/gate.py` to order 193.

Until the gate lands, the coordinator runs an interim local gate before each merge, a script
outside the tree (`CONTRIBUTING.md`, "Settings this repository relies on"). It checks no private
name: the coordinator's local runs have used none, only the checker's built-in patterns (ruling
80, "Facts recorded with it"), and CI checks none, since `CONDUIT_PRIVATE_NAMES` stays unset in
GitHub (point 10). What ran before a merge is recorded nowhere: no Record states a CI result, nor
a local one ([FR-495](../proposals/full-review/findings/G4-who-verifies.md#fr-495), its
verifier's second point). A branch pushed to this public repository is public at once. CI's
`secrets` job scans its commits only after the push, and GitHub's push protection stops only the
patterns it knows, so only a scan on the Mac before the push keeps a secret off GitHub.

The same holds for a commit's identity. The coordinator commits as
`Conduit work order <work-orders@conduit.invalid>`, as the squash in
[section 7 of the method as run](../implementation/METHOD_AS_RUN.md#7-merge-and-the-record) does,
because the Mac's default identity carries its user and host: ruling 80's facts record the
branches whose commits carried them, deleted or rewritten on 2026-11-16. A plain `git rebase`
re-commits with the machine's default identity as committer, and nothing checks a commit's
identity before the push.

The coordinator splits order 193 in two, so that each part can converge in two rounds. This part,
193a, adds `tools/gate.py` and its tests: one command, run from the repository root on the Mac
before each push and merge, that checks the identity of each commit the branch adds, runs the
Python checks of CI's `docs` job, with the private names read from a file the coordinator gives
and passed to `tools/check_private_paths.py` alone, and runs the pinned gitleaks on the branch's
commits. It says of each gate whether it passed, failed or did not run and why
([ruling 52](../proposals/ruling-52/README.md), point 2), ends PASS only when every required gate
passed, and writes a short record under `target/` that a pull request cites. Order 193b adds the
Rust directories and the engine, and switches the method as run and the documents to the gate;
until it lands, the gate says that it ran no Rust gate, and the interim gate stays the merge's.
The gate replaces no reader: a fresh Opus reader still verifies each order before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

What it leaves to others:

- The Rust directories, the engine, and the switch of the method as run, AGENTS.md,
  CONTRIBUTING.md, the handover and the tracker to the gate: order 193b. This order changes no
  document.
- The coordinator's private merge script switches to the gate outside the tree, where
  [D62](../proposals/full-review/decisions.md) keeps the runners, once order 193b lands.
- The pinned gitleaks: the coordinator installed release 8.24.2 on the Mac, outside the
  repository, on 2026-11-16. The gate looks for it on the `PATH` it is given and installs nothing.
- Where the private names are held: whether one of Conduit's secret stores can hold them is being
  investigated (ruling 80, its facts). The gate reads them from a file the coordinator names,
  whichever way they come to be held, and this order decides nothing about where that file comes
  from. CI's `CONDUIT_PRIVATE_NAMES` stays unset.
- Whether a check becomes required on `main`: ruling 80 makes none required for now, and
  `CONTRIBUTING.md`'s settings already say so (order 191). IN_FORCE.md, topic 15, already states
  the local gate (ruling 80); this order changes no line of it.
- Making the gate a CI job, or CI a required check: the owner's "formalize them into GitHub
  processes later".

Read first: AGENTS.md's Checks; `.github/workflows/checks.yml`, the jobs `secrets` and `docs`,
every step; sections 2 and 7 of the method as run; `tools/check_private_paths.py`;
`tools/check_docs.py`'s `path_mentions`; `tools/promise_standing.py` and
`tools/check_model_account.py` for the form of a tool, and the latter's script entry for its
restart under `/usr/bin/python3`; `tools/tests/test_vector_delta.py` and
`tools/tests/test_private_paths.py` for tests in temporary Git trees; and
`tools/tests/test_model_account.py` for a test of that restart.

## The decisions it applies

The gate is the owner's (ruling 80, point 10, answer 13). Its design below is the coordinator's
(rulings 37, 38 and 46, point 4), logged here and open to the owner's strike, except where another
source is named. Line numbers are those of `main` at `d4c8cf8b`. If a line has moved, find it by
its words. Each block below is indented to sit in its list item: write its lines without that
indent.

1. **One command.** Create `tools/gate.py`, using the standard library alone, so that it runs
   under `/usr/bin/python3` (Python 3.9) as well as a later `python3`. It reads
   `.github/workflows/checks.yml` as text and imports no YAML library. Its usage is
   `python3 tools/gate.py [--base REF] [--plan] [--private-names-file FILE]`:
   - `--base REF`, by default `origin/main`: what the branch is compared with.
   - `--plan`: decision 10.
   - `--private-names-file FILE`: decision 3.

   Any other argument is argparse's usage error, exit 2. Its `main(root, argv, run=...)` takes
   the repository root, the arguments and the runner of decision 8, so that tests can build a
   tree and replace every command. It finds `gitleaks` with `shutil.which`, which a test patches.
   Its paths are module constants. `check_docs.py` checks each literal repository path in
   `tools/*.py` (`path_mentions`), so each path the tool spells out must exist. It runs read-only
   Git commands, the checkers and gitleaks, and nothing else: it never fetches, pushes, commits,
   stages or calls GitHub. It prints no value it reads from the environment. It exits 0 when its
   result is PASS, 1 when it is FAIL, 2 when it could not start (decisions 2 and 3), and 130 when
   interrupted (decision 8). Its script entry calls `main` with the root that its own path gives,
   `Path(__file__).resolve().parents[1]`, as the other tools do.

   CI's `docs` job runs the checks under a Python with PyYAML and jsonschema (line 120), and
   several checks and tool tests need them. At its script entry, before `main`, when its
   interpreter cannot import `yaml` or `jsonschema` (`importlib.util.find_spec`),
   `CONDUIT_GATE_REEXEC` is not `1`, and `/usr/bin/python3` exists and is another interpreter by
   `os.path.realpath`, the gate sets `CONDUIT_GATE_REEXEC=1` and runs itself again under
   `/usr/bin/python3` with its own path and arguments (`os.execv`), as `check_model_account.py`
   does. On the Mac `python3` imports neither package, and
   `python3 -m unittest discover -s tools/tests` there exits 2 at once: `check_schemas.py`, which
   two tool tests import, starts itself again under `/usr/bin/python3` with unittest's
   `argv[0]`, which is not a file. Under `/usr/bin/python3` the tool tests pass.
2. **The range and the tree.** From the root, in this order:
   - A root that is not a Git working tree ends the run:
     `Gate: NOT RUN: the gate's root is not a Git working tree, so there is no branch to compare. Nothing was checked.`
   - The base is `git rev-parse --verify --quiet <REF>^{commit}`. If that fails, the gate prints
     `Gate: NOT RUN: the base <REF> does not resolve to a commit; fetch it or pass --base with a commit. Nothing was checked.`
   - The merge base is `git merge-base <base> HEAD`. If the two share no commit, the gate prints
     `Gate: NOT RUN: the base <REF> shares no commit with HEAD. Nothing was checked.`
   - Then decision 3's file, when one is named.

   Each of these exits 2, runs no gate and writes no record. Otherwise the branch contains the
   base when `git merge-base --is-ancestor <base> HEAD` succeeds; if not, it lacks the
   `git rev-list --count HEAD..<base>` commits of the base. These Git queries, and decision 4's,
   run directly, not through the runner of decision 8.
   - The changed paths are those that `git diff --name-only --no-renames -z <merge base>` gives
     (committed, staged and unstaged changes to tracked files, deletions among them; with
     `--no-renames` a moved file counts at its old and its new place) together with those that
     `git ls-files --others --exclude-standard -z` gives (untracked and not ignored), each once,
     in byte order. A change on the base after the merge base is not the branch's.
   - The uncommitted paths are the entries of
     `git status --porcelain -z --no-renames --untracked-files=all`, so that each path counts
     once and an untracked directory counts by its files; the untracked paths are those entries
     marked `??`. No entry makes the tree clean.

   Before any gate it prints these lines, with the values in place of the angle brackets
   (commits shortened to 12 characters):

   ```text
   Gate: base <REF> at <base>, merge base <merge base>, head <head> on <branch>; nothing is fetched.
   Tree: clean.
   Changed since the merge base: <C> paths.
   Private names: read from the file that --private-names-file names, and given to tools/check_private_paths.py alone; neither they nor the file's path are printed or recorded.
   Python: <version> imports PyYAML and jsonschema, as CI's docs job does.
   Environment: removed from every command: <names>, and <K> variables named for a credential.
   ```

   On a detached head the first line reads `head <head>, detached`. A tree that is not clean
   reads `Tree: <N> uncommitted paths. The gates read the working tree; secrets reads commits only.`
   A branch that lacks commits of the base adds, after the tree's line,
   `Base: the branch lacks <K> commits of <REF>. CI tests a pull request merged with them; the gate tests the branch alone.`
   Without `--private-names-file` the private names' line reads
   `Private names: no --private-names-file was given, so tools/check_private_paths.py checks its built-in patterns only.`
   and, when the caller's environment holds `CONDUIT_PRIVATE_NAMES`, goes on
   ` CONDUIT_PRIVATE_NAMES in the environment is not used.`
   An interpreter that still lacks a package, after decision 1's restart or without
   `/usr/bin/python3`, reads
   `Python: <version> cannot import <PyYAML, jsonschema, or PyYAML and jsonschema>; the checks that need them fail, or skip tests that CI runs.`
   `<version>` is `platform.python_version()`. `<names>` names, in byte order, those of
   decision 7's named variables that the caller's environment held, or reads `no named variable`;
   `<K>` counts decision 7's variables named for a credential.
3. **The private names come from a file the coordinator names.** With
   `--private-names-file FILE`, a path read from the directory the gate was started in, the gate
   checks the file before any gate, in this order, and each fault ends the run with its line,
   exit 2, no gate run and no record written:
   - `os.lstat` fails:
     `Gate: NOT RUN: the file that --private-names-file names does not exist or cannot be read. Nothing was checked.`
   - It is not a regular file by `stat.S_ISREG` on `os.lstat`, so that a symbolic link is refused:
     `Gate: NOT RUN: the file that --private-names-file names is not a regular file; a symbolic link, a directory and any other kind are refused. Nothing was checked.`
   - Its mode, `stat.S_IMODE`, is not exactly `0o600`, with `<MODE>` four octal digits such as
     `0644`:
     `Gate: NOT RUN: the file that --private-names-file names has mode <MODE>, not 0600; run chmod 600 on it. Nothing was checked.`
   - Its resolved path is the root or under it, by `Path.resolve()`:
     `Gate: NOT RUN: the file that --private-names-file names is inside the repository, where Git could commit it; keep it outside. Nothing was checked.`
   - It is opened with `os.O_RDONLY | os.O_NOFOLLOW` and read whole; a failure there gives the
     first line above. Text that does not decode as UTF-8 gives
     `Gate: NOT RUN: the file that --private-names-file names is not UTF-8 text. Nothing was checked.`
   - It holds no name:
     `Gate: NOT RUN: the file that --private-names-file names holds no name. Nothing was checked.`

   The names are the file's text split at each comma and each line end, each stripped of the
   space around it, the empty ones dropped, and each kept once, in the order of the file. The
   gate passes them, joined by commas, as `CONDUIT_PRIVATE_NAMES` in the environment of the
   `private paths` gate's command alone, the form `tools/check_private_paths.py` reads (lines 46
   and 47), and never on a command line. It never prints, logs or records a name, the file's
   content or the file's path; a message names the file only as "the file that
   --private-names-file names". Without the option no command gets the variable (decision 7).
4. **The identity of each commit.** The gate `identity` runs first.
   - When `HEAD` is the merge base it is NOT RUN and not required:
     `the branch adds no commit since the merge base`.
   - Otherwise it runs directly, as decision 2's queries do, and writes no log, since what it
     reads names people and machines:
     `git log --format=%H%x09%an%x09%ae%x09%cn%x09%ce <merge base>..HEAD`. These are the raw
     fields: `%aN`, `%aE`, `%cN` and `%cE` would apply a `.mailmap`, which a branch could add to
     make any identity read as the work-order one. Its reason is
     `the branch adds <N> commits, each to be by Conduit work order <work-orders@conduit.invalid>`.
   - A commit passes when its author name is `Conduit work order`, its author email
     `work-orders@conduit.invalid`, and its committer name and email the same, each compared
     exactly. The gate passes when every commit passes. Otherwise it fails, and its failure is
     `<F> of <N> commits are not by Conduit work order <work-orders@conduit.invalid>: <list>; the identities found are not printed; squash again with that identity, as the method as run's squash does`,
     where `<list>` gives each failing commit, in the order `git log` gives them, as its first 12
     characters and, in brackets, `author`, `committer` or `author and committer`, joined by
     commas. A nonzero exit of `git log` fails it with `git log exited <E>`. It never prints or
     records a name or an email that it read.
   - It is required whenever it runs.
5. **The Python gates**, in the order of the `docs` job's steps (lines 122 to 163), each from the
   root with the interpreter that runs the gate (`sys.executable`, after decision 1's restart),
   recorded and printed as `python3`:

   | Gate | Command |
   | --- | --- |
   | `docs` | `python3 tools/check_docs.py` |
   | `names` | `python3 tools/set_names.py --check` |
   | `schemas` | `python3 tools/check_schemas.py` |
   | `roadmap` | `python3 tools/check_roadmap.py` |
   | `rulings` | `python3 tools/check_rulings.py` |
   | `model account` | `python3 tools/check_model_account.py --history` |
   | `promise standing` | `python3 tools/promise_standing.py --check` |
   | `private paths` | `python3 tools/check_private_paths.py` |
   | `tool tests` | `python3 -m unittest discover -s tools/tests` |
   | `whitespace` | `git diff --check <merge base>` |
   | `vector changes` | `python3 tools/vector_delta.py <merge base> WORKTREE --declared-changed` |

   `<merge base>` is its full name. Where the job compares `HEAD^1` with `HEAD` on a pull
   request, the gate compares the merge base with the working tree, so that uncommitted work is
   checked as the job would check it once committed. Each is required; exit 0 is PASS and any
   other status FAIL. Each runs whatever the one before it gave, as each of the job's steps runs
   after a failed one. The job first fetches the pull requests' refs (line 113); the gate fetches
   nothing.

   A gate that ran while part of its check did not run says which part. Its line, PASS or FAIL,
   ends `; not checked: <what>`, and its record's `partial` holds `<what>`:
   - `whitespace` and `private paths` read tracked files alone (`git diff` and `git ls-files`):
     `<U> untracked path(s)`, when decision 2 found untracked paths;
   - `docs`, when `check_docs.py` printed lines beginning
     `Notice: skipped repository permalinks at`:
     `the permalinks at <N> commit(s) this clone lacks, which CI fetches`;
   - `tool tests`, when unittest's last line is `OK (skipped=<N>)`:
     `<N> test(s) unittest skipped`;
   - `secrets`, when the tree is not clean:
     `<N> uncommitted path(s), since gitleaks reads commits alone`.
6. **The secrets gate**, `secrets`, after the Python gates.
   - The pin is the value of the line `scanner_version=<V>` of checks.yml (line 39). Without that
     line the gate is NOT RUN:
     `the pinned version could not be read from .github/workflows/checks.yml`.
   - With no `gitleaks` on the `PATH` it is NOT RUN:
     `gitleaks is not installed; the pin is <V> (.github/workflows/checks.yml), and the pull request's secrets job scans these commits after the push`.
   - When `gitleaks version` prints another version, after any leading `v`, it is NOT RUN:
     `gitleaks <found> is installed, not the pinned <V>`. When it exits nonzero, or runs past its
     timeout, it is NOT RUN:
     `gitleaks version did not answer (<exit E, or the timeout's words>); the pin is <V>`.
   - When `HEAD` is the merge base it is NOT RUN: `the branch adds no commit since the merge base`.
   - Otherwise it runs, from the root, as the `secrets` job runs it (lines 76 to 79):

     ```sh
     gitleaks git . --config .gitleaks.toml --redact --no-banner --log-opts="--diff-merges=first-parent <merge base>..HEAD" --report-format json --report-path <temporary>/findings.json
     ```

     where `<temporary>` is a directory the gate makes with `tempfile` and deletes once it has
     read the report. The scanner's own output goes there too, never to the gate's log, so the
     gate's line names no log and its record's `log` is null. The command is recorded with
     `<temporary>` as written here. A report with findings fails the gate,
     `<N> findings; values are not printed`, and the gate prints one line for each,
     `<RuleID> <File>:<StartLine> in <Commit, 12 characters>`, and never the `Secret`, `Match`
     or any other field. A nonzero exit with no finding fails it,
     `gitleaks exited <S> without reporting a finding: the scan did not complete`. Exit 0 with no
     finding passes.
   - It is required whenever it runs. A NOT RUN does not stop a PASS, but it is never counted as
     passed: the last line names it among the gates not run.
7. **The environment and the timeouts.** Every command the runner starts runs with the caller's
   environment, less these, which the gate removes:
   - the named variables `CONDUIT_TEST_NPM`, with which the engine's shell tests fetch from npm
     (the nightly job's), `SCR_AGENT_CODEX`, with which experiment 015's scripted client launches
     a real Codex wrapper, `RUSTUP_TOOLCHAIN`, which would override each directory's pin, and
     `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS` and `RUSTDOCFLAGS`, which CI does not set and which
     can change what clippy, rustc and rustdoc accept (all matter to the cargo commands that
     order 193b adds; the list is one, so they go from every command now); and
     `CONDUIT_PRIVATE_NAMES`, which the `private paths` gate alone gets, from decision 3's file;
   - every variable whose name holds `KEY`, `TOKEN`, `SECRET`, `PASSWORD` or `CREDENTIAL`, in
     capitals, as the runners' `SCRUB` matches them (section 2 of the method as run).

   Apart from that one, and the `CONDUIT_GATE_REEXEC=1` that decision 1's restart leaves in its
   own environment, the gate sets no variable in this order; order 193b sets cargo's. Each command
   has its CI job's timeout: 15 minutes for a Python gate (`docs`), and 10 for `gitleaks version`
   and the scan (`secrets`). A command past its timeout is ended with its process group
   (decision 8) and fails: `ended after <M> minutes, with its process group`.
8. **Running, the lines and the result.**
   - The gates run one at a time: `identity`, the Python gates, `secrets`, then one gate
     `rust and engine`, NOT RUN and not required, with no command:
     `this version of the gate runs no Rust directory and not the engine; order 193b adds them, and until then their checks run by hand, as the method as run says`.
   - The runner starts each command in a process group of its own (`start_new_session=True`),
     with standard input closed, and writes its standard output and error to its log,
     `target/gate/logs/<head>/<NN>-<name>.log`, where `<head>` is the head's full name, `<NN>` the
     gate's position from 01, and `<name>` the gate's name with each space or `/` written `-`;
     `gitleaks` writes to the temporary directory of decision 6 instead. On its timeout, or when
     the gate is interrupted, the runner ends the group: SIGTERM, then SIGKILL to what is left
     after 10 seconds. A process that left the group is not reached. Interrupted, the gate prints
     `Gate: interrupted during <name>; its process group was ended, and the record says incomplete.`
     and exits 130. The runner's interface is the worker's choice, reported as such: it must let a
     test's fake see each command's arguments, directory and environment, and give back an exit
     status or a timeout, and the output.
   - Before each gate it prints `running <name>: <command>[ in <dir>]` to standard error, so
     that a long run shows where it is. As each gate ends, it prints one of these lines to
     standard output, `<S>` in seconds to one decimal, and `; not checked: <what>` only as
     decision 5 says. `identity` and `secrets` have no log, so their FAIL line leaves out
     `; log <log>`:

     ```text
     PASS <name> (<S> s): <command>[ in <dir>]; because <reason>[; not checked: <what>]
     FAIL <name> (<S> s): <command>[ in <dir>]; because <reason>; <failure>; log <log>[; not checked: <what>]
     NOT RUN <name>: <command>[ in <dir>]; <why>
     ```

     `rust and engine` has no command: its line is `NOT RUN <name>: <why>`, and its record's
     `command` is null. `<failure>` is, for a command of the runner, `exit <E>` followed by
     `; its last line: <line>` when the output has a nonblank line, or decision 7's timeout words;
     for `identity` and `secrets`, decision 4's and decision 6's words. After a FAIL line it prints
     the log's last 20 lines to standard error, each indented four spaces; for `secrets`, the
     finding lines of decision 6 instead, and for `identity` nothing more. A Python gate's reason
     is `CI's docs job runs it on each pull request`; that of `secrets`,
     `gitleaks <V>, the pinned release, is installed`; that of `identity`, decision 4's.
   - Required are every Python gate, and `identity` and `secrets` when they run. The result is
     PASS when every required gate passed, and FAIL otherwise. The last line is one of these, with
     the names joined by commas, or `none`:

     ```text
     Gate: PASS. <P> passed; not run: <names>. Record: target/gate/<head>.json
     Gate: FAIL. Failed: <names>; required, not run: <names>; <P> passed. Record: target/gate/<head>.json
     ```

     When the tree is not clean, the line goes on
     ` The tree has <N> uncommitted paths: commit them and run the gate again before citing the record.`
     When the branch lacks commits of the base, it goes on
     ` The branch lacks <K> commits of <REF>: bring it up to date with the method as run's guarded squash, not a plain rebase, and run the gate again.`
9. **The record**, `target/gate/<head>.json`, with the head's full name. The gate writes it once
   the header is printed and again after each gate, each time to a temporary file in that
   directory that it then renames, so that a run that stops leaves a record whose `result` is
   `incomplete`. A later run for the same head replaces it, and that head's logs. It is one JSON
   object, indented two spaces, with these keys:
   - `format`: 1; `tool`: `tools/gate.py`; `started` and `finished`: UTC times in ISO 8601,
     `finished` null until the end; `result`: `PASS`, `FAIL` or `incomplete`;
   - `base`: `{"ref": <REF>, "commit": <full name>}`; `merge_base`, `head`: full names;
     `branch`: its name, or null when detached; `contains_base`: true or false, and
     `behind`: the count of decision 2;
   - `tree`: `{"clean": <bool>, "uncommitted_paths": <N>}`; `changed_paths`: their count;
   - `platform`: `platform.system()` and `platform.machine()` joined by a space, such as
     `Darwin arm64`; `python`: `platform.python_version()`; `python_packages`: true when the
     interpreter imports PyYAML and jsonschema;
   - `environment`: an object whose `set` maps each variable the gate sets for every command to
     its value (`{}` in this order; never `CONDUIT_PRIVATE_NAMES`), whose `removed` lists in byte
     order the named variables of decision 7 that the caller's environment held, whose
     `removed_credentials` counts the variables removed for their names, and whose
     `private_names_configured` is true when decision 3's file was given and read;
   - `gates`: one object per gate, in the order run, with `name`, `directory` (`.` for the root,
     or null for a gate with no command), `command` as printed, `required`, `result` (`PASS`,
     `FAIL` or `NOT RUN`), `reason` (why it ran, or null), `why` (why it failed or did not run, or
     null; for a FAIL, `exit <E>` or decision 7's timeout words for a command of the runner, and
     decision 4's or decision 6's words for `identity` or `secrets`, never a line of a command's
     output), `partial` (decision 5's `<what>`, or null), `exit` (or null), `seconds`
     (to one decimal, or null) and `log` (its path from the root, or null).

   Nothing else: no output of a command, no absolute path, no user, host or machine name, no
   name or email from a commit, no private name and no value read from the environment but the
   counts and names above. Git ignores `target/`. A pull request cites the record by copying its
   last line and its gates' names, commands and results.
10. **`--plan`** runs decisions 2 and 3, printing the header, then, instead of running a gate,
    prints `WOULD RUN <name>: <command>[ in <dir>]; because <reason>` for each that would run,
    and each NOT RUN line as a run gives it; for `secrets` it runs `gitleaks version` alone. It
    runs no check, writes no record and no log, and ends
    `Gate plan: <W> gates would run, <N> would not. Nothing was checked, and no record was written.`
    It exits 0, or 2 as decisions 2 and 3 say.
11. **Tests**, in a new `tools/tests/test_gate.py`. Each builds what it needs in a temporary
    directory, as `test_vector_delta.py` and `test_private_paths.py` do, with each commit made
    with an identity the test gives (`git -c user.name=... -c user.email=...`, or a committer
    passed in the child's environment). Each replaces the runner with a fake that records each
    command with its directory and environment and returns a scripted exit status and output,
    and patches `shutil.which` for `gitleaks`. The runner carries every gate's command and
    `gitleaks version`; decision 2's and decision 4's Git queries run directly, on the test's own
    repository. No test runs gitleaks or a checker, or reaches the network; item 10 runs a Python
    child of its own, and item 5 reads the repository. A test that changes `os.environ` does so
    with `mock.patch.dict`. Planted values are made up and use reserved domains such as
    `example.invalid`. No test has a docstring: with one, `unittest -v` prints the test's name
    and its result on different lines, and the count under Done when misses it. Each test fails
    when the behaviour it covers is taken out of `tools/gate.py`.
    1. `test_changed_paths_cover_commits_edits_renames_and_untracked`: a repository whose base
       commit holds `a/x.txt`, `b/y.txt` and a `.gitignore` naming `target/`. A branch from it
       moves `a/x.txt` to `c/x.txt` and commits; then the base branch gains `d/base-only.txt`.
       On the branch, `b/y.txt` is edited and not committed, `e/new.txt` is new and untracked,
       and `target/out.txt` is written. With the base branch as `--base`, the changed paths are
       exactly `a/x.txt`, `b/y.txt`, `c/x.txt` and `e/new.txt`; the tree has 2 uncommitted
       paths; and the branch lacks 1 commit of the base, as the header's `Base:` line and the
       last line say.
    2. `test_identity_names_each_commit_not_by_the_work_order_identity`: a base branch with one
       commit, and a branch from it that adds three. The first is by
       `Conduit work order <work-orders@conduit.invalid>` as author and committer, and adds a
       `.mailmap` that maps `Planted Person <planted@example.invalid>` to that identity. The
       second has that author and `Planted Person <planted@example.invalid>` as committer, as a
       plain rebase leaves one. The third has the planted author and the work-order committer.
       `identity` fails: its line says `2 of 3 commits`, names the second commit's first 12
       characters with `(committer)` and the third's with `(author)`, and does not name the
       first; the result is FAIL and `main` returns 1; and neither `Planted Person` nor
       `planted@example.invalid` appears in the output or the record. On a branch whose three
       commits are all by the work-order identity it passes, with decision 4's reason. With the
       head at the merge base it is NOT RUN with decision 4's words, and the last line names it
       among the gates not run.
    3. `test_private_names_reach_the_private_paths_gate_alone`: `--private-names-file` names a
       file of mode 0600, outside the test's repository, whose first line is
       `planted-alpha, planted-beta` and whose second is `planted-gamma`. The environment holds,
       each with a planted value, `CONDUIT_PRIVATE_NAMES`, `CONDUIT_TEST_NPM`, `SCR_AGENT_CODEX`,
       `RUSTUP_TOOLCHAIN`, `RUSTFLAGS` and `EXAMPLE_API_KEY`. The fake sees
       `CONDUIT_PRIVATE_NAMES` set to `planted-alpha,planted-beta,planted-gamma` in the
       environment of the `private paths` command and of no other; no command's environment
       holds any of the other five variables, nor the environment's own `CONDUIT_PRIVATE_NAMES`
       value. The header's private names line is decision 2's for a file. The record's
       `private_names_configured` is true, its `removed` is `CONDUIT_PRIVATE_NAMES`,
       `CONDUIT_TEST_NPM`, `RUSTFLAGS`, `RUSTUP_TOOLCHAIN` and `SCR_AGENT_CODEX`, in that order,
       and its `removed_credentials` is 1. No planted value, no name and not the file's path
       appears in the output, a log or the record.
    4. `test_a_run_that_cannot_start_checks_nothing`: `--base no-such-ref` returns 2 and prints
       decision 2's line. Then `--private-names-file` names, in turn: a file of mode 0644; a
       symbolic link to a file of mode 0600; a directory; a path that does not exist; a file of
       mode 0600 inside the test's repository; a file of mode 0600 holding only commas, spaces
       and line ends; and a file of mode 0600 holding bytes that are not UTF-8. Each returns 2
       and prints its own line of decision 3. In every case the fake is given no command, no
       record is written, and the output holds neither the file's path nor a planted name that
       a file holds. An unknown option raises `SystemExit` with code 2.
    5. `test_python_gates_follow_the_docs_job`: with the fake, the Python gates' names and
       commands are those of decision 5's table, in its order, with the merge base's full name.
       Then the test reads the repository's own `.github/workflows/checks.yml` with PyYAML, which
       it imports inside itself and is skipped without, and takes the `run` of each step of the
       `docs` job from the first whose `run` begins `python3 tools/`. Once CI's `HEAD^1 HEAD` and
       the gate's `<merge base>` (in `git diff --check`) or `<merge base> WORKTREE` (in
       `vector_delta.py`) are each written as one placeholder, the two lists are equal, in
       order; so a step added to the job, removed from it or moved in it fails the test. Last,
       the gate's script entry is run as `test_model_account.py` runs its own, with
       `runpy.run_path`, `sys.argv` set to its path, `--base` and `no-such-ref`,
       `importlib.util.find_spec` patched to find no `yaml`, `sys.executable` patched to another
       path, `os.path.exists` patched to true, `CONDUIT_GATE_REEXEC` set to `0` with
       `mock.patch.dict`, and `os.execv` patched to raise. It calls
       `os.execv("/usr/bin/python3", ["/usr/bin/python3", <its path>, "--base", "no-such-ref"])`
       once. With `CONDUIT_GATE_REEXEC` set to `1` it does not, and the run ends with
       `SystemExit` code 2 from the base that does not resolve, having started no command.
    6. `test_result_is_pass_only_when_every_required_gate_passed`: the branch adds one commit by
       the work-order identity, every command exits 0, and `shutil.which` finds no `gitleaks`:
       `identity` passes, the `secrets` line is NOT RUN with decision 6's words, the
       `rust and engine` line is decision 8's, the last line begins `Gate: PASS.` and names
       `secrets` and `rust and engine` among those not run, and `main` returns 0. Then `schemas`
       exits 1 with a last line of output: its FAIL line names `exit 1`, that line and its log;
       the fake still sees every later command; the last line begins
       `Gate: FAIL. Failed: schemas;`; and `main` returns 1. A command that the fake reports past
       its timeout fails with decision 7's words. With one untracked path, the `whitespace` and
       `private paths` lines end `; not checked: 1 untracked path(s)`; with the fake's output for
       `check_docs.py` holding two lines that begin `Notice: skipped repository permalinks at`,
       the `docs` line ends
       `; not checked: the permalinks at 2 commit(s) this clone lacks, which CI fetches`; with
       the fake's last line for `tool tests` `OK (skipped=3)`, its line ends
       `; not checked: 3 test(s) unittest skipped`; and each record's `partial` holds those
       words.
    7. `test_secrets_gate_runs_only_the_pinned_gitleaks_and_prints_no_value`: with a checks.yml
       pinning 8.24.2: no `gitleaks` gives NOT RUN naming 8.24.2; `gitleaks version` printing
       `v8.0.0` gives NOT RUN naming both versions; with `8.24.2` and the head at the merge
       base, NOT RUN for no commit. With `v8.24.2` and a commit on the branch: a report the fake
       writes, holding one finding whose `Secret` and `Match` are a planted value, with exit 1,
       fails the gate and prints `<RuleID> <File>:<StartLine> in <commit>`, and the planted value
       is in no output, log or record; exit 1 with no report fails it with the words "the scan
       did not complete"; exit 0 with an empty list passes, and its record's `log` is null. The
       command recorded holds `<temporary>/findings.json` and the range `<merge base>..HEAD`,
       and the temporary directory is gone after the run. With the tree not clean, its line
       ends with decision 5's words for `secrets`.
    8. `test_record_says_what_ran_and_holds_no_private_detail`: with `os.environ` holding only
       what the test sets, `PATH` and, with planted values, `CONDUIT_PRIVATE_NAMES` and
       `EXAMPLE_TOKEN`, and with a names file given as in item 3, a run writes
       `target/gate/<head>.json` with decision 9's keys: `result` `PASS`, each gate's name,
       command and result as printed, `private_names_configured` true and
       `removed_credentials` 1. Its text holds no planted value, no name from the file, not the
       file's path and not the temporary root's absolute path. While the fake runs the second
       gate, the record exists and its `result` is `incomplete`.
    9. `test_plan_lists_the_gates_and_runs_none`: with the branch adding one commit and
       `shutil.which` finding a `gitleaks`, `--plan` makes the fake see only `gitleaks version`;
       prints a `WOULD RUN` line for each gate that a run would hold, `identity` among them, and
       the same NOT RUN lines; ends with decision 10's last line; writes nothing under
       `target/gate/`; and returns 0.
    10. `test_a_command_past_its_timeout_is_ended_with_its_children`: the gate's own runner, not
        the fake, with a timeout of 1 second, runs `sys.executable` on a script that starts a
        child that sleeps 60 seconds, writes both process IDs to a file and sleeps 60 seconds
        itself. The runner reports the timeout within 15 seconds, and within 5 seconds after it
        neither process exists. The log holds what the script printed before it slept.

## Scope

- New: `tools/gate.py` and `tools/tests/test_gate.py`.

Change nothing else: no document, nothing under `.github/`, no other tool, no file under `spec/`,
`conformance/`, `engine/`, `packages/` or `experiments/`. The gate writes only under `target/`,
which Git ignores; Done when's file of planted names goes in the system's temporary directory and
is deleted.

## Done when

From the repository root, each passes:

```sh
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -v 2>&1 | grep -c -E '^test_(changed_paths_cover_commits_edits_renames_and_untracked|identity_names_each_commit_not_by_the_work_order_identity|private_names_reach_the_private_paths_gate_alone|a_run_that_cannot_start_checks_nothing|python_gates_follow_the_docs_job|result_is_pass_only_when_every_required_gate_passed|secrets_gate_runs_only_the_pinned_gitleaks_and_prints_no_value|record_says_what_ran_and_holds_no_private_detail|plan_lists_the_gates_and_runs_none|a_command_past_its_timeout_is_ended_with_its_children) .* ok$'   # prints 10
python3 tools/gate.py --plan | grep -c '^WOULD RUN '     # prints 11
python3 tools/gate.py --plan | grep '^NOT RUN ' | sed -E 's/:.*//'   # the three lines below
names=$(mktemp) && printf 'planted-name-one,planted-name-two\n' > "$names" && chmod 600 "$names" && python3 tools/gate.py --plan --private-names-file "$names" | grep -c '^Private names: read from the file'   # prints 1
chmod 644 "$names"; python3 tools/gate.py --plan --private-names-file "$names"; echo "exit $?"; rm -f "$names"   # decision 3's line for the mode, then exit 2
python3 tools/gate.py; echo "exit $?"                    # the last line below, then exit 0
python3 -c 'import json, subprocess; h = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip(); r = json.load(open("target/gate/" + h + ".json")); print(r["result"], r["tree"]["clean"], r["environment"]["private_names_configured"], [g["name"] for g in r["gates"] if g["result"] == "NOT RUN"])'   # PASS False False ['identity', 'secrets', 'rust and engine']
! grep -q -F "$(git rev-parse --show-toplevel)" target/gate/*.json && echo "no absolute path"
python3 tools/gate.py --base no-such-ref; echo "exit $?"   # decision 2's line, then exit 2
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
grep -n -E '[[:space:]]$' tools/gate.py tools/tests/test_gate.py   # prints nothing
git status --short
```

You commit nothing, so the head is the commit your tree started from, the branch adds no commit,
and the second `--plan` prints exactly:

```text
NOT RUN identity
NOT RUN secrets
NOT RUN rust and engine
```

The full run runs no cargo command. Its last line begins
`Gate: PASS. 11 passed; not run: identity, secrets, rust and engine.` and goes on with the
sentence on uncommitted paths; then comes `exit 0`. Its `secrets` line says that gitleaks is
not installed or that the branch adds no commit. Its `whitespace` and `private paths` lines end
`; not checked: 2 untracked path(s)`, the two new files. `git status --short` must list exactly two paths, both untracked:
`tools/gate.py` and `tools/tests/test_gate.py`. `git diff --check` does not read a file that is
not tracked, so the `grep` above checks the two new files for trailing whitespace.

Then show that each test can fail. For each of the 10, from the repository root, one at a time:
copy `tools/gate.py` to `target/gate.py.orig`, take out or change the one behaviour the test
covers (a line or two), run that test alone and see it fail, then copy `target/gate.py.orig`
back; `cmp tools/gate.py target/gate.py.orig` must then print nothing. Git ignores `target/`.

Do not stage anything. In your report, mark each choice you made yourself as your own, the
runner's interface among them; give the full output of the gate's run and of each `--plan` above
(they are short), for each test the change that made it fail, and the last line of every check.

After the report:

- `git diff --check` and `check_private_paths.py` read tracked files only. So the admin stages
  the new files, then runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
- Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
  ([ruling 79](../proposals/ruling-79/README.md), point 2).
- The admin runs the new gate on this order's squash, with the file of private names once they
  are held, beside the interim gate, which still gates this merge; the Record gives that run's
  last line, its `identity` result among its gates.
