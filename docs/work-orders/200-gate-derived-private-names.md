# Work order 200: the gate derives the private names from the machine it runs on

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
Run no `cargo build` and no `cargo test`; the gate's own discovery runs the offline cargo queries
that order 193b gave it, and nothing else of cargo runs. Run the gate one run at a time. Install
nothing. Never print a whole large file or log. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

This order handles private names. Never run `scutil`, `tailscale`, `hostname`, `whoami`, `id`,
`git config --global` or `finger` yourself, never read a sibling directory's Git configuration
yourself, and never print, log, copy or write anywhere a name that the new code derives on this
machine: the only commands that touch the real names are those of Done when, which print counts
and gate lines. Every value in a test is made up, as decision 9 gives it; no test or fixture holds
a value read from this machine. Write no real user, host, machine, repository or account name,
and no email address outside a reserved domain such as `example.invalid`, into any file.

## What and why

[Ruling 80](../proposals/ruling-80/README.md), point 10, has the local gate check the private
names before each merge, and order 193a built `tools/gate.py` to read them from a file that
`--private-names-file` names, of mode 0600 and outside the repository. Where the names were to be
held stayed open (ruling 80, "Facts recorded with it"): no file was ever made, so every merge so
far has run the `private paths` gate on the checker's built-in patterns alone. On 2026-11-17 the
coordinator asked the owner for that file and for leave to set the work-order identity in the
main checkout's local Git configuration. The owner answered: "1. Is this necessary if we do 2? 2.
Agreed 3. Agreed  I can do the live session whenever you inform me. I suggest we find a durable
way to proceed without a manually created private names file if possible, let me know if there
are solutions". The coordinator explained that the identity covers a commit's metadata and the
names check a file's content, and asked through the question tool how the gate should check the
names without a hand-made file. The owner chose "Gate derives them (Recommended)", whose
description read: "A small order adds --derive-private-names to tools/gate.py: names read from
this machine on each run (OS account, host names, git identity, sibling repos, tailnet peers),
kept in memory, never printed, with already-public names and short words dropped. Tested with
fake values; works on any machine."

This order is that small order. With `--derive-private-names` the gate reads the names from the
machine it runs on, at each run, with no file; drops those that are short or already public;
gives the rest to `tools/check_private_paths.py` alone, as it gives a file's names today; redacts
them from its output and logs; and prints, logs and records only how many it derived and from
which sources. The merge runs with it. The order also records the owner's answer in ruling 80's
later answers, where it settles what the ruling's facts left open, and brings the documents that
describe the gate up to date.

On 2026-11-17 the coordinator counted, on the Mac and without printing a name, what decision 3's
sources give against the tracked files of `main` at `5eb71667`: two of the sibling repositories'
names appear in tracked files, once each, both in
[ruling 68's record](../proposals/ruling-68/README.md), point 1's list of ruling 1's reference
clones, and no other derived name appears in any case. Decision 4's list of public words holds
those two for that reason.

What it leaves to others:

- `tools/check_private_paths.py` stays as it is: it matches each configured name exactly, as
  bytes, so decision 5 gives it each derived name's lower-case form beside the name. Other cases
  of a derived name go unchecked until the checker matches without regard to case, although the
  gate redacts its own output in every case (decision 8). CI sets no names, so a later order
  could make that change safely.
- CI: `CONDUIT_PRIVATE_NAMES` stays unset in GitHub (point 10), and no CI job derives names; the
  gate runs on the coordinator's machine.
- The coordinator's private merge script switches to the flag outside the tree, where
  [D62](../proposals/full-review/decisions.md) keeps the runners.
- The owner part of a sibling's remote URL (the account or organization before the repository's
  name) is not taken; this repository's own account is in its origin URL and would be dropped as
  public anyway. A name that no source gives can still be added with `--private-names-file`.
- A false positive on another machine, such as a peer whose host name is a word the documents
  use, fails the `private paths` gate at the line where the word stands; a later order adds the
  word to decision 4's list, with its reason. Only a work order changes that list.
- The owner's "3. Agreed" and the live session it offers change nothing here.

Read first: `tools/gate.py` and `tools/tests/test_gate.py` as orders 193a and 193b left them
(`redact_output`, `read_private_names`, `main`'s header, record and `private paths` gate, and the
tests' `FakeRunner`, `setUp`, `invoke` and `assert_private`); `tools/check_private_paths.py`;
decisions 2, 3, 9 and 11 of [order 193a](193a-local-gate.md); AGENTS.md's Checks;
`CONTRIBUTING.md`'s checks and "Settings this repository relies on"; the variables and section 7
of [the method as run](../implementation/METHOD_AS_RUN.md); IN_FORCE.md, topic 14; and ruling
80's "Facts recorded with it" and "Later answers".

## The decisions it applies

The flag and what it reads are the owner's choice (the option's description above). Its design
below is the coordinator's (rulings 37, 38 and 46, point 4), logged here and open to the owner's
strike. Line numbers are those of `main` at `5eb71667`. If a line has moved, find it by its words.
Orders 193a's and 193b's decisions stand except where this order changes them. Each block below is
indented to sit in its list item: write its lines without that indent.

1. **The option.** `main`'s parser gains `--derive-private-names` (`action='store_true'`), added
   right after `--private-names-file` (line 439), so that the usage reads
   `python3 tools/gate.py [--base REF] [--plan] [--private-names-file FILE] [--derive-private-names] [--all] [--jobs N] [--test-threads N]`.
   `main`'s signature becomes `main(root, argv, run=run_command, machine=None)`. Without the
   flag nothing of this order runs, `machine` is never used, and every line, record key and exit
   is as today but for decision 7's `private_names` key and decision 8's redaction.
2. **The seam.** A new class `Machine`, the only code that reads the machine. Its constructor
   takes the environment that `scrub_environment()` returns, and keeps a copy as
   `self.environment`. Its methods:
   - `account()`: imports `pwd` inside the method and returns
     `(entry.pw_name, entry.pw_gecos)` of `pwd.getpwuid(os.getuid())`. An `ImportError` raises
     `SourceSkipped('this system has no pwd module')`; a `KeyError` raises
     `SourceSkipped("this process's user ID has no account entry")`.
   - `host_name()`: returns `socket.gethostname()`.
   - `is_macos()`: returns `sys.platform == 'darwin'`.
   - `which(name)`: returns `shutil.which(name, path=self.environment.get('PATH', os.defpath))`.
   - `list_directory(path)`: returns `os.listdir(path)` sorted by `os.fsencode`.
   - `query(args, extra=None)`: runs `args` with `subprocess.run`, the environment
     `dict(self.environment, **(extra or {}))`, standard input from `subprocess.DEVNULL`, standard
     output captured, standard error to `subprocess.DEVNULL`, and a timeout of `DERIVE_TIMEOUT`,
     a new module constant of 10 seconds. It returns `(exit, stdout bytes)`: `(None, b'')` on
     `subprocess.TimeoutExpired`, and `(127, b'')` when the command cannot start (`OSError`).

   `SourceSkipped` is a new exception class whose one argument, its `why`, is decision 3's
   words. When the flag is given and `machine` is `None`, `main` makes
   `Machine(environment)` from the scrubbed environment, before the cargo variables are added
   to it. Tests pass their own (decision 9).
3. **The sources**, a new function `derive_private_names(root, machine)` that returns a
   `Derivation` (a dataclass with `kept`, `values`, `dropped`, `sources`, `origin_remote` and
   `unreadable_remotes`). It reads six sources, always in this order, each giving its candidates
   in the order given here. A source either is read, giving zero or more candidates, or is NOT
   RUN with exactly one of its whys below; one source's failure never stops another. Only the
   exceptions this decision lists are caught (`SourceSkipped`, an `OSError`, and what
   `json.loads` raises), each making its source NOT RUN, and the `ValueError` that
   `urllib.parse.urlsplit` raises, which makes no source NOT RUN (the repository name of a URL
   below, and decision 4's public text); any other exception propagates, so
   that `FakeMachine`'s `AssertionError` (decision 9) is never hidden as a NOT RUN. It never
   calls the network or GitHub: `scutil` and `git config`
   read local files and `tailscale status --json` asks the local daemon. It reads no credential:
   no command's standard error is kept, and no command's output is printed, logged or recorded.

   | Source | Candidates | NOT RUN, with its why |
   | --- | --- | --- |
   | `account` | `account()`'s short name; its full name, which is `pw_gecos` up to its first comma; then each whitespace-separated word of the full name | `account()` raised `SourceSkipped`: its why |
   | `host name` | `host_name()`'s value; then its first label, the text before its first `.`, when that differs | `host_name()` raised `OSError`: `socket.gethostname() failed` |
   | `macOS computer names` | the output of `scutil --get ComputerName`, then of `scutil --get LocalHostName`, each stripped, each only when its exit is 0 | `is_macos()` is false: `this is not macOS`; neither exit is 0: `scutil answered neither ComputerName nor LocalHostName` |
   | `git identity` | the output of `git config --global --get user.name`, stripped; then that of `git config --global --get user.email`, stripped, cut at its last at sign to keep the local part before it (the whole value when it holds none); each only when its exit is 0 | the first of these that applies, in this order: either exit is `None`: `git config --global did not answer within 10 seconds`; otherwise either exit is neither 0 nor 1, the first such in query order: `git config --global exited <E>`; both exits are 1: `git's global configuration sets neither user.name nor user.email` |
   | `sibling repositories` | the repositories in the directory that holds this one, below | below |
   | `tailnet peers` | from the JSON object that `<path> status --json` prints, where `<path>` is what `which('tailscale')` returned: for `Self`, then for each value of `Peer` in the object's order, its `HostName`, then the first label of its `DNSName` (the text before its first `.`); each only when it is a non-empty string | `which('tailscale')` is `None`: `the tailscale command is not on the PATH`; exit `None`: `tailscale status --json did not answer within 10 seconds`; any other exit but 0: `tailscale status --json exited <E>`; output that `json.loads` refuses or that is not an object: `tailscale status --json did not print a JSON object` |

   Each command's output is decoded as UTF-8 with `errors='replace'`. Every command of decision
   3 except two goes through `machine.query`, never through the gate's runner, and writes no
   log. The two are the common directory below and decision 4's origin URL: they run in the
   root through the gate's `git` helper, with `machine.environment` as their env. In
   `tailnet peers`, a `Self` or `Peer` that is missing, `null` or not an object gives no
   candidates, and the source is still read. A value of `Peer` that is not an object gives no
   candidates, and the source is still read.

   **The sibling repositories.**
   - This repository's common directory is
     `git --no-replace-objects rev-parse --path-format=absolute --git-common-dir`, run in the
     root with the gate's `git` helper, as decision 2 of order 193a runs its queries. When it
     fails, or its last component is not `.git`, the source is NOT RUN:
     `this repository has no main working tree, so its sibling directories are not known`.
     Otherwise the main working tree is its parent (a linked worktree, such as a worker's, so
     finds the main checkout), and the holder is the main working tree's parent.
   - `machine.list_directory(holder)` lists it; an `OSError` makes the source NOT RUN:
     `the directory that holds this repository could not be listed`.
   - Each entry, in that order, that is a directory and not a symbolic link
     (`os.path.isdir` and not `os.path.islink`) and holds a `.git` (`os.path.lexists`, a
     directory or a file) is a repository; this repository's own main working tree is among
     them. Its candidates are its directory name, then the repository name of each URL that
     `machine.query(['git', '--no-replace-objects', '-C', <entry path>, 'config', '--local', '--get-regexp', '^remote\\..*\\.url$'], extra={'GIT_CEILING_DIRECTORIES': <holder>})`
     prints, one per line, in the order printed, the URL being the text after the line's first
     space. Exit 1 means it has no remote. Any other exit but 0, or `None`, adds 1 to
     `unreadable_remotes` and gives its directory name alone.
   - The repository name of a URL, stripped: when it holds `://`, the path of
     `urllib.parse.urlsplit`, called inside `try: ... except ValueError`; a URL that it refuses
     gives no candidate and adds 1 to `unreadable_remotes`, the same as a remote that cannot be
     read, and the exception and its message are never printed, logged or recorded; otherwise, when it holds a `:` and the text before its first `:`
     holds no `/` (the form user, at sign, host, colon, path), the text after that `:`;
     otherwise, as when it holds no `:`, the whole, treated as a path. Then
     the part after its last `/` once trailing `/` are removed, less a final `.git`. The user
     and password of a URL are never kept.
4. **The filters.** First the candidates of all sources are split, each at every comma and
   line end (`re.split(r'[,\r\n]', ...)`, as `read_private_names` splits a file), each part
   stripped and the empty ones dropped. Then each is kept once, compared by `str.lower()`: the
   first that appears, in source order, stands for the rest, and is the one its source counts.
   Then each that stands is dropped, by the first that applies, and counted once under that
   reason:
   - `short`: it is shorter than 4 characters (`len`);
   - `origin`: its lower-case form is a substring of the lower-case public text of this
     repository's origin URL. The URL is `git --no-replace-objects config --local --get remote.origin.url`,
     run in the root with the gate's `git` helper; its public text is, when it holds `://`, the
     `hostname` of `urllib.parse.urlsplit` (or `''`) followed by its `path`, `urlsplit` called
     inside `try: ... except ValueError`; otherwise the text after the first at sign when an at
     sign comes before the first `:`, and else the whole URL. When that command fails, or
     `urlsplit` refuses the URL, there is no origin remote: `origin_remote` is false, decision
     6's line for no origin remote is printed, and this filter drops nothing. The exception and
     its message are never printed, logged or recorded. The URL and its public text are never
     printed, logged or recorded;
   - `public`: its lower-case form is a key of `PUBLIC_WORDS`, a new module constant, a dict
     whose keys are lower-case words that must appear in the repository and whose values say
     why. Write it with exactly these entries:

     | Key | Value |
     | --- | --- |
     | `agentwatch` | `ruling 68's record names it among ruling 1's reference clones` |
     | `localhost` | `the documents and examples use it, and phones and some machines report it as their host name` |
     | `pi-reference` | `ruling 68's record names it among ruling 1's reference clones` |

   The rest are kept. `Derivation.kept` lists them in order; `values` lists each kept name
   followed by its `str.lower()` form when that differs, each string once; `dropped` is
   `{'short': S, 'origin': O, 'public': P}`; and `sources` holds, for each of the six in order,
   `{'name': <source>, 'result': 'read' or 'NOT RUN', 'kept': <how many kept names it stands
   for>, 'why': <decision 3's why, or None>}`.
5. **The names given, and the file's precedence.** In `main`, after the base and the merge base
   resolve (order 193a's decision 2):
   - When `--private-names-file` is given, the file is read and checked first, exactly as today;
     a fault ends the run with its line and exit 2 before anything is derived, and the machine is
     not asked.
   - Then, with the flag, `derive_private_names(root, machine)` runs. When every one of the six
     sources is NOT RUN, the gate prints
     `Gate: NOT RUN: --derive-private-names read none of its sources on this machine. Nothing was checked.`
     and exits 2, runs no gate and writes no record.
   - The names given are the file's names, as read, in the file's order, and never filtered;
     then each of `Derivation.values` that is not already among them, compared exactly. The
     file's names so take precedence: none of decision 4's filters removes one, and they come
     first. A derived name equal, in any case, to one of the file's names still counts as
     derived.
   - When the names given are not empty, they are joined by commas into the one string that
     today's `private_names` holds: it alone is set as `CONDUIT_PRIVATE_NAMES` in the
     environment of the `private paths` gate's command (lines 671 and 672), and it is what every
     call of `redact_output` receives. When they are empty, `private_names` is `None`, as without
     either option, and no command gets the variable.
   - `--plan` derives too, and prints the same lines, as it reads the file today.
6. **The lines.** With the flag, the header's private names line (lines 501 to 509) is one of
   these, `<K>` the number of kept names, `<D>` = `<S>` + `<O>` + `<P>`, and `<read>` each source
   that was read, in order, as `<source> <its kept count>`, joined by `, `:
   - without a file:

     ```text
     Private names: <K> derived from this machine (<read>), and given to tools/check_private_paths.py alone; <D> dropped as public or short (<S> shorter than 4 characters, <O> in the origin remote's URL, <P> on the gate's list of public words); none is printed or recorded.
     ```

     When `<K>` is 0 it goes on ` So tools/check_private_paths.py checks its built-in patterns only.`
     When the caller's environment holds `CONDUIT_PRIVATE_NAMES`, it then goes on
     ` CONDUIT_PRIVATE_NAMES in the environment is not used.`, as order 193a's line does.
   - with a file:

     ```text
     Private names: read from the file that --private-names-file names, and <K> more derived from this machine (<read>), all given to tools/check_private_paths.py alone; <D> dropped as public or short (<S> shorter than 4 characters, <O> in the origin remote's URL, <P> on the gate's list of public words); neither the names nor the file's path are printed or recorded.
     ```

   Right after it come, in this order, only those that apply:
   - for each source NOT RUN, in source order, `Private names: NOT RUN <source>: <why>.`;
   - when `unreadable_remotes` is `<U>` > 0,
     `Private names: the remotes of <U> sibling repositories could not be read; their directory names alone were taken.`;
   - when there is no origin remote,
     `Private names: this repository has no origin remote, so no derived name was dropped as already public in its URL.`

   These lines are made from constants and counts, never from a derived value. Without the flag
   the private names line is order 193a's, unchanged.
7. **The record.** `environment` gains a key `private_names`:
   `{"file": <true when the file was read>, "derived": null}` without the flag, and with it
   `"derived"` is
   `{"kept": <K>, "dropped": {"short": <S>, "origin": <O>, "public": <P>}, "origin_remote": <bool>, "unreadable_sibling_remotes": <U>, "sources": <Derivation.sources>}`.
   `private_names_configured` stays, true when `CONDUIT_PRIVATE_NAMES` is given to the
   `private paths` gate (decision 5). Nothing else changes in the record: it never holds a
   derived name, a candidate, a URL, a directory name or a command's output, and `format` stays
   1.
8. **The redaction.** `redact_output` (lines 262 to 267) matches the configured names without
   regard to case: `re.sub(pattern, '[redacted]', text, flags=re.IGNORECASE)`, longest first as
   today. It applies to a file's names as to derived ones, everywhere it applies today: each
   command's output before its log is written, the branch and `--base` in the header and the
   record, each gate's line, record fields and finding lines, and the interrupted line.
9. **Tests**, in `tools/tests/test_gate.py`, added to `GateTests` with its harness. `invoke`
   gains a keyword `machine=None`, passed to `gate.main`. A new test class `FakeMachine`,
   a subclass of `gate.Machine` built on `os.environ` as `setUp` leaves it, takes scripted values
   for `account()` (a pair, or a `SourceSkipped` to raise), `host_name()` (a string, or `OSError`
   to raise), `is_macos()`, `which('tailscale')`, `list_directory` (the real one, or `OSError`
   to raise), and `query` for the commands of decision 3 by their arguments:
   `git config --global --get user.name` and `... user.email`, `scutil --get ComputerName` and
   `... LocalHostName`, and `<the tailscale path> status --json`, each an `(exit, bytes)` pair.
   It records every call. It passes any other `git` command, which holds no `--global`, to
   `gate.Machine.query`, so that the sibling queries read the test's own repositories, and it
   fails the test (`AssertionError`) on any other command, so that no test can reach a real
   source. A helper adds the origin remote to the test's repository; a helper makes a sibling
   repository in `self.parent`, the directory that holds the test's repository, with
   `git init -q` and `git remote add`. Every value is made up; an email is written in a test as
   the local part, an at sign and `example.invalid`. No test has a docstring. Each test fails
   when the behaviour it covers is taken out of `tools/gate.py`.
   1. `test_derived_names_come_from_each_source`: the origin remote is
      `https://example.invalid/planted-owner/repository.git`. In `self.parent`: a sibling
      `planted-sibling-one` whose remote `origin` is `https://` followed by
      `planted-user:planted-secret`, an at sign and
      `example.invalid/planted-owner/planted-remote-one.git`; a sibling `planted-sibling-two`
      whose remote `origin` is the user `git`, an at sign and
      `example.invalid:planted-owner/planted-remote-two.git`, and whose remote `upstream` is
      `https://example.invalid/planted-owner/planted-remote-three`; a directory
      `planted-plain-directory` without `.git`; and a symbolic link `planted-link` to
      `planted-sibling-one`. The machine gives the account `('planted-short', 'Fakefirst Fakelast,Room 1,,')`,
      the host name `planted-host.example.invalid`, macOS true with `Planted Computer` and
      `planted-local-host` (exit 0 each), git's `Planted Gitname` and an email of local part
      `planted-git` under `example.invalid` (exit 0 each), `tailscale` at
      `planted-bin/tailscale`, whose `status --json` (exit 0) is an object with `Self`
      `{"HostName": "planted-self", "DNSName": "planted-self-dns.example.invalid."}` and `Peer`
      `{"planted-key": {"HostName": "planted-peer", "DNSName": "planted-peer-dns.example.invalid."}}`.
      With `--derive-private-names`, the `private paths` command alone gets
      `CONDUIT_PRIVATE_NAMES`, and it is exactly
      `planted-short,Fakefirst Fakelast,fakefirst fakelast,Fakefirst,fakefirst,Fakelast,fakelast,planted-host.example.invalid,planted-host,Planted Computer,planted computer,planted-local-host,Planted Gitname,planted gitname,planted-git,planted-sibling-one,planted-remote-one,planted-sibling-two,planted-remote-two,planted-remote-three,planted-self,planted-self-dns,planted-peer,planted-peer-dns`.
      The header's line is exactly
      `Private names: 19 derived from this machine (account 4, host name 2, macOS computer names 2, git identity 2, sibling repositories 5, tailnet peers 4), and given to tools/check_private_paths.py alone; 1 dropped as public or short (0 shorter than 4 characters, 1 in the origin remote's URL, 0 on the gate's list of public words); none is printed or recorded.`
      and no `Private names: NOT RUN` line is printed. The machine was asked for
      `planted-bin/tailscale status --json`, and each sibling query had `GIT_CEILING_DIRECTORIES`
      equal to `self.parent`. The record's `private_names` is
      `{"file": false, "derived": {"kept": 19, ...}}` with decision 7's keys, every source
      `read`, and the kept counts of the header. `main` returns 0.
   2. `test_derived_names_drop_short_public_and_origin_words`: `gate.PUBLIC_WORDS`'s keys are
      exactly `agentwatch`, `localhost` and `pi-reference`, each value a non-empty string. The
      origin remote is `https://example.invalid/planted-origin-word/repository.git`; no sibling
      but the test's repository. The machine gives the account `('abc', '')`, the host name
      `Planted-Origin-Word`, macOS false, git's `AgentWatch` and an email of local part
      `LOCALHOST`, and `tailscale` whose `Self` has the `HostName` `planted-peer` and whose
      `Peer` object has one entry with the `HostName` `PLANTED-PEER` and the `DNSName`
      `Pi-Reference.example.invalid.`, and a second entry, `"planted-null": null`, which changes
      none of the expected names.
      `CONDUIT_PRIVATE_NAMES` is exactly `planted-peer`; the header's line is exactly
      `Private names: 1 derived from this machine (account 0, host name 0, git identity 0, sibling repositories 0, tailnet peers 1), and given to tools/check_private_paths.py alone; 6 dropped as public or short (1 shorter than 4 characters, 2 in the origin remote's URL, 3 on the gate's list of public words); none is printed or recorded.`
      followed by `Private names: NOT RUN macOS computer names: this is not macOS.` Then, with
      the origin remote removed, `CONDUIT_PRIVATE_NAMES` is exactly
      `Planted-Origin-Word,planted-origin-word,repository,planted-peer`, the header's line says
      `3 derived` with `(account 0, host name 1, git identity 0, sibling repositories 1, tailnet peers 1)`
      and `4 dropped as public or short (1 shorter than 4 characters, 0 in the origin remote's URL, 3 on the gate's list of public words)`,
      and decision 6's line for no origin remote follows the NOT RUN line.
   3. `test_derived_names_combine_with_a_names_file`: the origin remote as in test 1; a names
      file of mode 0600 in `self.parent` holding `xyz, Planted-Peer` and a line end. The machine
      gives the account `('planted-short', '')`, `OSError` for the host name, macOS false, exit
      1 for both of git's queries, and `tailscale` whose `Self` has the `HostName`
      `planted-peer` and no peer. With both options, `CONDUIT_PRIVATE_NAMES` is exactly
      `xyz,Planted-Peer,planted-short,planted-peer`; the header's line is exactly
      `Private names: read from the file that --private-names-file names, and 2 more derived from this machine (account 1, sibling repositories 0, tailnet peers 1), all given to tools/check_private_paths.py alone; 1 dropped as public or short (0 shorter than 4 characters, 1 in the origin remote's URL, 0 on the gate's list of public words); neither the names nor the file's path are printed or recorded.`
      followed by the NOT RUN lines of `host name`, `macOS computer names` and `git identity`,
      with decision 3's whys; the record's `private_names` has `file` true and `kept` 2, and
      `private_names_configured` is true; neither the file's path nor any name is in the output,
      a log or the record. Then delete the record, clear `self.fake.calls`, and use a new
      `FakeMachine` with the same values; with the file at mode 0644, the run exits 2 with order
      193a's line for the mode, the machine records no call, the fake runner gets no command and
      no record is written: `self.record_path()` does not exist after it.
   4. `test_a_missing_source_is_skipped_by_name`: a base machine gives the account
      `('planted-short', '')`, the host name `planted-host`, macOS false, exit 1 for both of
      git's queries, and no `tailscale`; its run prints the NOT RUN lines of
      `macOS computer names`, `git identity` and `tailnet peers` with decision 3's whys. Then,
      as subtests, each changing one thing of the base machine, the run prints exactly the line
      given, still ends `Gate: PASS.` and returns 0. Each subtest that creates a directory in
      `self.parent` removes it (`shutil.rmtree`) before the next subtest begins, so each
      subtest's run sees only the base machine and its own one change:
      - `host_name()` raises `OSError`: `Private names: NOT RUN host name: socket.gethostname() failed.`
      - macOS true, both `scutil` queries exit 1:
        `Private names: NOT RUN macOS computer names: scutil answered neither ComputerName nor LocalHostName.`
      - `git config --global --get user.name` exits 128:
        `Private names: NOT RUN git identity: git config --global exited 128.`
      - it exits `None`: `Private names: NOT RUN git identity: git config --global did not answer within 10 seconds.`
      - `tailscale` at `planted-bin/tailscale`, whose `status --json` exits 1 printing
        `planted-tailscale-output`: `Private names: NOT RUN tailnet peers: tailscale status --json exited 1.`,
        and `planted-tailscale-output` is in no output, log or record;
      - it exits `None`: `Private names: NOT RUN tailnet peers: tailscale status --json did not answer within 10 seconds.`
      - it exits 0 printing `[]`: `Private names: NOT RUN tailnet peers: tailscale status --json did not print a JSON object.`
      - `list_directory` raises `OSError`:
        `Private names: NOT RUN sibling repositories: the directory that holds this repository could not be listed.`
      - `account()` raises `SourceSkipped("this process's user ID has no account entry")`:
        `Private names: NOT RUN account: this process's user ID has no account entry.`
      - a directory `planted-broken` in `self.parent` whose `.git` is a file holding
        `gitdir: planted-nowhere`:
        `Private names: the remotes of 1 sibling repositories could not be read; their directory names alone were taken.`,
        and `planted-broken` is among the names given.
      - a sibling `planted-sibling-one` in `self.parent` whose remote `origin` has the URL
        `'https://planted-user:planted-secret'`, an at sign and
        `'planted\uff03host.example.invalid/planted-owner/planted-remote.git'` (U+FF03, the
        fullwidth number sign, written in the test as the escape `\uff03`), which
        `urllib.parse.urlsplit` refuses with a `ValueError`:
        `Private names: the remotes of 1 sibling repositories could not be read; their directory names alone were taken.`,
        and `planted-secret` is in no output, log or record.

      Last, a machine whose six sources all fail (the account's `SourceSkipped`, the host
      name's `OSError`, macOS false, git's exits 1, `list_directory`'s `OSError`, no
      `tailscale`) makes the run exit 2 with decision 5's line. Before that run, delete the
      record and clear `self.fake.calls`; after it, the fake runner gets no command and no record
      is written: `self.record_path()` does not exist.
   5. `test_derived_names_are_redacted_and_never_recorded`: on a branch
      `wo/planted-short-work`, a machine giving the account `('planted-short', 'Fakefirst Fakelast')`
      and the base machine of test 4 otherwise, and the fake runner's `check_docs.py` exiting 1
      with the output `PLANTED-SHORT and fakefirst FAKELAST here` and a line end. The run's
      first line holds `on wo/[redacted]-work`; the `docs` gate's FAIL line and its log hold
      `[redacted] and [redacted] here`; `CONDUIT_PRIVATE_NAMES` reaches the `private paths`
      command alone; and none of `planted-short`, `Fakefirst Fakelast`, `Fakefirst`, `Fakelast`
      and `planted-host`, nor any case of one (compared with `str.lower()` on both sides), is in
      standard output, standard error, any log or the record, whose `branch` is
      `wo/[redacted]-work`. Then the record is deleted, and `--plan` with the flag prints the
      same private names line, asks the machine again, and writes no record: the record file
      does not exist after it.
   6. `test_the_real_readers_take_planted_values`: the real `gate.Machine`, with no real value.
      With `pwd.getpwuid` patched to return an object whose `pw_name` is `planted-short` and
      `pw_gecos` is `Fakefirst Fakelast,,,`, `account()` returns that pair; patched to raise
      `KeyError`, it raises `SourceSkipped` with decision 2's why. With `socket.gethostname`
      patched to return `planted-host`, `host_name()` returns it. In a temporary directory, a
      directory `planted-bin` holds an executable `tailscale` script that prints
      `{"Self": {"HostName": "planted-self"}}` and a line end with `printf`, and a file `planted-gitconfig` holds a `[user]`
      section with the name `Planted Gitname`. A `Machine` whose environment is `PATH` set to
      `planted-bin`'s path, then the directory of the `git` that `shutil.which('git')` finds,
      `HOME` set to the temporary directory, `GIT_CONFIG_GLOBAL` to `planted-gitconfig`'s path
      and `GIT_CONFIG_NOSYSTEM` to `1`, finds `tailscale` in `planted-bin` while `os.environ`'s
      `PATH` does not hold it; its `query` of that path with `status --json` returns exit 0 and
      those bytes with the line end; its `query` of `git config --global --get user.name` returns
      exit 0 and `Planted Gitname` with a line end; with `gate.DERIVE_TIMEOUT` patched to 1 and
      the script's text changed to exactly `#!/bin/sh` and then `exec /bin/sleep 30`, each on a
      line of its own (`/bin/sleep` exists on macOS and, through usr-merge, on Linux, and `exec`
      leaves no orphan process), `query` returns `(None, b'')` within 15 seconds;
      and `query(['planted-no-such-command'])` returns `(127, b'')`. This test calls no other
      method and runs neither `scutil` nor `derive_private_names`.
10. **The documents.** Each text named here is given whole under "The texts of decision 10"
    below. Write it as given, and rewrap a join only where an existing line must meet it. Texts
    A, B1, B2 and B3 take the two-space indent of the item they join, as Text E's lines show
    theirs. Change nothing else in these files.
    - Text A, AGENTS.md, Checks, the item that begins
      `` - `gate.py` needs Python 3 and Git, and what each check it runs needs: the items above ``
      (lines 172 to 190): it replaces the sentence that begins `It reads the private` and ends
      `it does not use.` (lines 180 to 183). Keep the item's text before and after it.
    - Text B1, CONTRIBUTING.md, "What each check needs", the item that begins
      `` - `gate.py` needs Python 3 and Git `` (lines 234 to 237): it goes before the item's last
      sentence, the one that links AGENTS.md's Checks and ends `says the rest.`
    - Text B2, CONTRIBUTING.md, "What the checks cover", the item that begins
      `` - `gate.py` runs on the Mac. `` (lines 268 to 276): it goes after the sentence that ends
      `and gitleaks when the release that checks.yml pins is installed.`
    - Text B3, CONTRIBUTING.md, "Settings this repository relies on", the item that begins
      `` - **`CONDUIT_PRIVATE_NAMES` is not set** ``: it replaces the item's last sentences, from
      `Since then the only name check` to `can hold the names is open.` (lines 329 to 334).
    - Text C1, the method as run, "Variables": delete the row of `<NAMES_FILE>` (line 31). It
      has no replacement.
    - Text C2, the method as run, section 7: it replaces the three lines that begin
      `commands. Then, on the Mac, with the tree clean, run the gate` and end
      `names once they are held:` (lines 288 to 290); and the command below them,
      `python3 tools/gate.py --private-names-file <NAMES_FILE>` (line 292), becomes
      `python3 tools/gate.py --derive-private-names`.
    - Text C3, the same paragraph: it replaces `with the private names given to` and the next
      line's `` `tools/check_private_paths.py` alone; `` (lines 298 and 299).
    - Text C4, the next paragraph: it replaces
      `` `origin/main` and, once the names are held, that they were configured. `` (line 318).
    - Text D, docs/AUDIT.md, the row of D11 (line 183): it replaces
      `with the names from a file the coordinator names once they are held; where the names are to be held is open.`
    - Text E, IN_FORCE.md, topic 14: the line that begins
      `  - *Coordinator's* (80 point 10, applying answers 12 and 13` (line 352) says that the
      local gate "is to check the names before each merge once it holds them", which no longer
      holds: since this order the gate checks them at every merge. Text E's two lines replace
      that line. They hold no date, as `check_rulings.py` requires of IN_FORCE.md.
    - Text F, docs/proposals/ruling-80/README.md, "Later answers": after its last paragraph
      (line 353), a blank line and text F's three paragraphs. The owner's answer stands on one
      line, as given, its two spaces after "3. Agreed" kept.
    - The ledger, docs/proposals/ruling-log.md, ruling 80's row (line 88): only its owner's-words
      cell (the fourth) changes, from `In the record, with a later answer of 2026-11-16.` to
      `In the record, with later answers of 2026-11-16 and 2026-11-17.`, since under ruling 67
      the ledger says when the owner answered, as ruling 81's change did for the answer of
      2026-11-16.

### The texts of decision 10

Text A:

```markdown
With `--derive-private-names` it derives the private names from the machine it runs on,
at each run and with no file: the OS account's short and full names, the host names (on
macOS the computer and local host names too), git's global `user.name` and the local part
of its `user.email`, the names of the Git repositories beside this one and of their
remotes, and the host names of the tailnet's peers where `tailscale` is installed. It
drops a name shorter than 4 characters, one in the origin remote's URL and one on its list
of public words, and skips a source it cannot read, naming the source. With
`--private-names-file` it also reads names from that file, of mode 0600 and outside the
repository, and drops none of them. It gives the names to `check_private_paths.py` alone,
redacts them from its output and logs, and prints only how many it derived and from which
sources; the `CONDUIT_PRIVATE_NAMES` of its own environment it does not use.
```

Text B1:

```markdown
With `--derive-private-names` it also asks the machine for the private names, through
`scutil` on macOS and `tailscale` where it is installed, each answering from the machine
itself, and skips by name a source it cannot read.
```

Text B2:

```markdown
With `--derive-private-names`, which each merge uses, it derives the private names from the
machine at each run and gives them to `check_private_paths.py` alone, printing only how many
it derived and from which sources.
```

Text B3:

```markdown
Since then the only name check is the `private paths` gate of `tools/gate.py` before each
merge ([the method as run](docs/implementation/METHOD_AS_RUN.md#7-merge-and-the-record)). On
2026-11-17 the owner chose that the gate derive the names itself ([ruling
80](docs/proposals/ruling-80/README.md), its later answers): with `--derive-private-names`,
which the merge uses, it reads them from the machine it runs on at each run, keeps them in
memory, gives them to the checker alone and prints none. No file of names is kept, and
`--private-names-file` adds names that no source gives.
```

Text C2:

```markdown
commands. Then, on the Mac, with the tree clean, run the gate
([ruling 80](../proposals/ruling-80/README.md), point 10 and its later answers), which
derives the private names from the Mac at each run:
```

Text C3:

```markdown
with the private names it derives (the Mac's account, host and computer names, git's
global identity, the repositories beside this one and their remotes, and the tailnet's
peers, less those shorter than 4 characters, in the origin remote's URL or on its list of
public words) given to `tools/check_private_paths.py` alone and never printed;
```

Text C4:

```markdown
`origin/main` and that the private names were configured; the pull request copies the
gate's `Private names:` lines, which give counts and sources and no name.
```

Text D:

```markdown
with the names that the gate derives from the coordinator's machine at each run, as the owner chose on 2026-11-17 ([ruling 80](proposals/ruling-80/README.md), its later answers); no file of names is kept.
```

Text E:

```markdown
  - *Coordinator's* (80 point 10, applying answers 12 and 13 to order 191's question, against its own recommendation of a masked secret): CI does not check the private names that `CONDUIT_PRIVATE_NAMES` lists; it stays unset in GitHub, and the local gate checks the names before each merge.
  - The local gate derives the private names from the machine it runs on, at each run, and keeps them in memory alone; no file of names is kept (80 point 10 and its later answers).
```

Text F:

```markdown
2026-11-17: the coordinator asked the owner, in chat, three things: (1) a file of private
names, of mode 0600 and outside the repository, for the `--private-names-file` of
`tools/gate.py`; (2) whether the coordinator may set the neutral identity "Conduit work
order <work-orders@conduit.invalid>" in the main checkout's local Git configuration; (3)
optional strikes of the coordinator's calls; and it noted the live check to come. The owner
answered:
"1. Is this necessary if we do 2? 2. Agreed 3. Agreed  I can do the live session whenever you inform me. I suggest we find a durable way to proceed without a manually created private names file if possible, let me know if there are solutions".
The coordinator explained that the identity covers a commit's metadata and the names check
a file's content, proposed deriving the names on each run of the gate, and asked through
the session's question tool: "How should the merge gate check that private names (your
user name, machine names, private repo names) never land in the public repo, without a
hand-made names file?" Its options: "Gate derives them (Recommended)", described as "A
small order adds --derive-private-names to tools/gate.py: names read from this machine on
each run (OS account, host names, git identity, sibling repos, tailnet peers), kept in
memory, never printed, with already-public names and short words dropped. Tested with fake
values; works on any machine."; "Private script only"; and "Skip names check". The owner
chose "Gate derives them (Recommended)".

It settles where the private names are held, which the facts above left open: nowhere.
The gate derives them from the machine it runs on, at each run, and keeps them in memory
alone ([order 200](../../work-orders/200-gate-derived-private-names.md)). The owner makes
no file of names, and `--private-names-file` stays for a name that no source gives. That
the facts' investigation of a secret store for the names ends with it is the coordinator's
reading, open to the owner's strike. Point 10's choice that CI checks no private name
stands, and no decision of this ruling changes. The owner's "3. Agreed" and the live
session it offers change nothing here.

A fact recorded with it: on the owner's "2. Agreed", the coordinator set `user.name` to
"Conduit work order" and `user.email` to work-orders@conduit.invalid in the main
checkout's local Git configuration the same day; the global configuration was left as it
was. The method as run's squash still names the identity on its command line, and the
gate's `identity` gate still checks every commit a branch adds.
```

## Scope

- Changed: `tools/gate.py` and `tools/tests/test_gate.py` (decisions 1 to 9); `AGENTS.md`,
  `CONTRIBUTING.md`, `docs/implementation/METHOD_AS_RUN.md`, `docs/AUDIT.md`, `IN_FORCE.md` and
  `docs/proposals/ruling-80/README.md` (decision 10's texts, and nothing else in them); and
  `docs/proposals/ruling-log.md` (ruling 80's owner's-words cell alone, decision 10).

Change nothing else: not `tools/check_private_paths.py`, nothing under `.github/`, no other tool,
no file under `spec/`, `conformance/`, `engine/`, `packages/` or `experiments/`, no other ruling's
record and no other cell of the ledger. The gate writes only under `target/`, which Git ignores; so do the
commands below. Done when's file of planted names goes in the system's temporary directory and is
deleted.

## Done when

From the repository root, each passes:

```sh
mkdir -p target
/usr/bin/python3 -m unittest discover -s tools/tests     # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -v 2>&1 | grep -c -E '^test_(derived_names_come_from_each_source|derived_names_drop_short_public_and_origin_words|derived_names_combine_with_a_names_file|a_missing_source_is_skipped_by_name|derived_names_are_redacted_and_never_recorded|the_real_readers_take_planted_values) .* ok$'   # prints 6
/usr/bin/python3 -m unittest discover -s tools/tests -p test_gate.py -v 2>&1 | grep -c -E '^test_[a-z_]+ .* ok$'   # prints 21
python3 tools/gate.py --plan | grep -c '^Private names: no --private-names-file was given'   # prints 1
python3 tools/gate.py --plan --derive-private-names > target/derive-plan.txt; echo "exit $?"   # exit 0
grep -c -E '^Private names: [1-9][0-9]* derived from this machine \(account [0-9]+, host name [0-9]+' target/derive-plan.txt   # prints 1
grep -E '^Private names: NOT RUN ' target/derive-plan.txt | sed -E 's/:[^:]*$//'   # report these lines: source names only
names=$(mktemp) && printf 'planted-name-one\n' > "$names" && chmod 600 "$names" && python3 tools/gate.py --plan --derive-private-names --private-names-file "$names" | grep -c '^Private names: read from the file that --private-names-file names, and [0-9]* more derived from this machine'; rm -f "$names"   # prints 1
python3 tools/gate.py --derive-private-names > target/derive-run.txt 2>&1; echo "exit $?"   # exit 0
grep -c '^PASS private paths ' target/derive-run.txt   # prints 1
tail -n 1 target/derive-run.txt | cut -c1-15   # Gate: PASS. and the count
python3 -c 'import json, subprocess; h = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip(); r = json.load(open("target/gate/" + h + ".json")); e = r["environment"]; print(r["result"], e["private_names_configured"], e["private_names"]["file"], [s["name"] for s in e["private_names"]["derived"]["sources"]], [g["name"] for g in r["gates"] if g["result"] == "NOT RUN"])'   # PASS True False ['account', 'host name', 'macOS computer names', 'git identity', 'sibling repositories', 'tailnet peers'] ['identity', 'secrets', 'rust unreached', 'engine']
python3 -c 'import sys, pathlib, subprocess; sys.path.insert(0, "tools"); import gate; root = pathlib.Path.cwd().resolve(); h = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip(); d = gate.derive_private_names(root, gate.Machine(gate.scrub_environment()[0])); files = [root / "target/derive-plan.txt", root / "target/derive-run.txt", root / "target/gate" / (h + ".json"), *sorted((root / "target/gate/logs" / h).glob("*.log"))]; texts = [p.read_text(errors="replace").lower() for p in files]; print(len(d.kept) > 0, sum(any(n.lower() in t for t in texts) for n in d.kept))'   # True 0
! grep -q -F "$(git rev-parse --show-toplevel)" target/gate/*.json && echo "no absolute path"
grep -c -F 'python3 tools/gate.py --derive-private-names' docs/implementation/METHOD_AS_RUN.md   # prints 1
grep -c -F 'NAMES_FILE' docs/implementation/METHOD_AS_RUN.md   # prints 0
grep -c -F 'once the names are held' docs/implementation/METHOD_AS_RUN.md   # prints 0
grep -c -F 'with the private names it derives' docs/implementation/METHOD_AS_RUN.md   # prints 1
grep -c -F "gate's \`Private names:\` lines" docs/implementation/METHOD_AS_RUN.md   # prints 1
grep -c -F -- '--derive-private-names' AGENTS.md CONTRIBUTING.md   # AGENTS.md:1 and CONTRIBUTING.md:3
grep -c -F 'where the names are to be held is open' docs/AUDIT.md   # prints 0
grep -c -F 'once it holds them' IN_FORCE.md   # prints 0
grep -c -F '"1. Is this necessary if we do 2? 2. Agreed 3. Agreed  I can do the live session' docs/proposals/ruling-80/README.md   # prints 1
grep -c -F 'In the record, with later answers of 2026-11-16 and 2026-11-17.' docs/proposals/ruling-log.md   # prints 1
grep -c -F 'keeps them in memory alone; no file of names is kept (80 point 10 and its later answers)' IN_FORCE.md   # prints 1
grep -c -F 'A fact recorded with it: on the owner' docs/proposals/ruling-80/README.md   # prints 1
python3 tools/check_docs.py                              # Errors: 0
python3 tools/check_rulings.py                           # Errors: 0
python3 tools/check_roadmap.py                           # Errors: 0
python3 tools/check_private_paths.py                     # Errors: 0
git diff --check
git diff --quiet -- .github tools/check_private_paths.py && echo "unchanged"
git status --short
```

The line of NOT RUN sources may be empty; report it as printed, source names only, never a why's
value beyond decision 3's words. The full run with the flag runs no cargo build or test, since
this branch changes nothing a Rust directory or the engine reads. Its last line begins
`Gate: PASS.`, names `identity`, `secrets`, `rust unreached` and `engine` among the gates not run,
and goes on with the sentence on uncommitted paths; then comes `exit 0`. If its `private paths`
gate fails, report the paths and line numbers it printed and nothing else: do not open those
lines, and do not add a word to `PUBLIC_WORDS`; the coordinator decides. `git status --short`
must list exactly the nine changed files of Scope, each modified, none untracked.

Then show that each new test can fail. For each of the six, from the repository root, one at a
time: copy `tools/gate.py` to `target/gate.py.orig`, make the change named here, run that test
alone and see it fail, then copy `target/gate.py.orig` back; `cmp tools/gate.py target/gate.py.orig`
must then print nothing.

1. `test_derived_names_come_from_each_source`: take out the first label of a peer's `DNSName`.
2. `test_derived_names_drop_short_public_and_origin_words`: take out the `public` filter.
3. `test_derived_names_combine_with_a_names_file`: put the derived names before the file's.
4. `test_a_missing_source_is_skipped_by_name`: let `host_name()`'s `OSError` propagate.
5. `test_derived_names_are_redacted_and_never_recorded`: take `re.IGNORECASE` out of
   `redact_output`.
6. `test_the_real_readers_take_planted_values`: make `which` search the `PATH` of `os.environ`.

Do not stage anything. In your report, mark each choice you made yourself as your own; give the
output of the two `--plan` runs' private names lines, the full run's last line, the record's
summary and the leak check's line, for each test the change that made it fail, and the last line
of every check. Never give a name the machine derived.

After the report:

- `git diff --check` and `check_private_paths.py` read tracked files only, and this order adds
  no file, so the worker's runs of both cover its changes.
- Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
  ([ruling 79](../proposals/ruling-79/README.md), point 2), each under the same rules on private
  names as this order.
- The admin runs `python3 tools/gate.py --derive-private-names` on this order's squash, the first
  merge whose `private paths` gate checks derived names; the Record gives that run's last line
  and its `Private names:` lines, which hold counts and sources and no name.
