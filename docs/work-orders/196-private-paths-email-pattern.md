# Work order 196: an email pattern for the private-path check

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access and needs none.
It runs no cargo command. Never print a whole large file or log. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a key,
a token or any credential. Never print, write down or pass in a command's arguments an email
address that decision 1's survey finds outside the reserved domains, nor any part of one: not
the part before its `@`, and not its domain, which can itself be private, a machine's own name
or a person's domain. Of what the survey finds, print and report only the reserved domains with
their counts, the totals, and the path and line number of each address outside them.

## What and why

`tools/check_private_paths.py` fails when a Git-tracked file holds a private detail, and names
the file, the line and the kind of detail, never the value. This order has it fail too on an
email address outside the domains reserved for examples, so that an account's address, written in
its common form, fails the check even while no private names are set. It narrows that gap and
does not close it: the forms decision 2 leaves out, and every private name that is not an
address, are still found only through the names.

The checker has seven built-in patterns: the three of [order 068](068-test-gate-and-ci.md) and the
four that [order 151](151-private-names-and-secret-scan.md) added for the full review's
[FR-534](../proposals/full-review/findings/G6-outside-the-tree.md#fr-534). Anything else it finds
only among the names that `CONDUIT_PRIVATE_NAMES` lists. With that variable unset or empty it
checks no name, and its output does not say so (lines 46 to 47 and 78 to 81 at `d4c8cf8b`). Under
[ruling 80](../proposals/ruling-80/README.md), point 10, CI checks no private name and the variable
stays unset in GitHub; the local gate before each merge is to check the names once it holds them,
and the coordinator's runs so far have used none (ruling 80, "Facts recorded with it"). So an
account's email address in a tracked file passes every check today: no pattern finds it, and the
secret scan looks for keys and tokens. Such an address can come from a client's status output,
which can name the signed-in account (experiment 027's probe records only whether and in which
field it does, never the value, and its tests use addresses at `example.org`), or from the
dogfood store's records, whose export the full review's decision D56 commits, scrubbed, once this
checker and the secret scan are in ([its decisions](../proposals/full-review/decisions.md)).

The pattern does not replace the names. A user, host or machine name that is not an address is
still found only through the list. How the names are held and passed is not this order's: order
193a's gate is to pass them to this check, and how they are held is still open (ruling 80, its
facts). Nor does this order decide anything about Conduit itself: which account a sign-in, a
record or a message may name belongs to the sign-in design, not to a check of this repository's
files.

On `main` at `d4c8cf8b` the coordinator ran decision 1's survey. It found 200 addresses in 42
files, all in reserved domains: 186 at `example.org`, 5 at `conduit.invalid` (the neutral commit
identity, quoted in the method as run and in order 173), 5 at `example.com` and 4 at
`example.invalid`. None is outside them, so the list of allowed addresses starts empty. An entry
would be, for example, a vendor's no-reply address in a commit trailer quoted in a document; no
document quotes one.

Read first: `tools/check_private_paths.py` and `tools/tests/test_private_paths.py`, whole (both are
short); AGENTS.md's "Checks"; and order 151's decisions 1 and 2, whose form this order follows.

## The decisions it applies

These are the coordinator's (rulings 37, 38 and 46, point 4), logged here and open to the owner's
strike, except where another source is named. Line numbers are those of `main` at `d4c8cf8b`; if
a line has moved, find it by its words.

1. **A survey first, and a stop on any address outside the reserved domains.** Before you change
   any file, write this script, unchanged, to `target/wo196/survey.py` (Git ignores `target/`),
   and run it from the repository root with `python3 target/wo196/survey.py`:

   ```python
   """Count the email addresses in Git-tracked files; print no address and no other domain."""
   import collections
   import os
   import re
   import subprocess

   LABEL = rb"[A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?"
   EMAIL = re.compile(rb"(?<![A-Za-z0-9._%+-])[A-Za-z0-9._%+-]+@((?:" + LABEL
                      + rb"\.)+[A-Za-z]{2,63})(?![A-Za-z0-9-])")
   SECOND_LEVEL = (b"example.com", b"example.org", b"example.net")
   TOP_LEVEL = (b"invalid", b"test", b"example", b"localhost")


   def reserved(domain):
       if domain.rsplit(b".", 1)[-1] in TOP_LEVEL:
           return True
       return any(domain == name or domain.endswith(b"." + name) for name in SECOND_LEVEL)


   paths = subprocess.run(["git", "ls-files", "-z"], check=True, stdout=subprocess.PIPE).stdout
   counts, files, hits = collections.Counter(), set(), []
   for raw in filter(None, paths.split(b"\0")):
       path = os.fsdecode(raw)
       data = os.fsencode(os.readlink(path)) if os.path.islink(path) else open(path, "rb").read()
       for match in EMAIL.finditer(data):
           domain = match.group(1).lower()
           files.add(path)
           if reserved(domain):
               counts[domain] += 1
           else:
               line = data.count(b"\n", 0, match.start()) + 1
               hits.append(f"{path}:{line}")
   for domain, count in sorted(counts.items(), key=lambda item: (-item[1], item[0])):
       print(f"{count:6d} {domain.decode()}")
   for hit in hits:
       print(hit)
   print(f"Addresses: {sum(counts.values()) + len(hits)} in {len(files)} files;"
         f" outside the reserved domains: {len(hits)}")
   ```

   It prints each reserved domain it found with its count, then `PATH:LINE` for each address
   outside the reserved domains, never its domain, then a last line with the totals. On `main` at
   `d4c8cf8b` it printed:

   ```text
      186 example.org
        5 conduit.invalid
        5 example.com
        4 example.invalid
   Addresses: 200 in 42 files; outside the reserved domains: 0
   ```

   Your tree may count more, this order's own file and any later commit among them. Report the
   whole output: it holds no address and no domain outside the reserved ones. If its last line
   ends `outside the reserved domains: 0`, go on. Otherwise stop. Change no file, and report each
   hit as the survey printed it, with the commit that last changed that line, which
   `git blame --porcelain -L LINE,LINE -- PATH | head -n 1 | cut -d ' ' -f 1` prints. Do not open,
   print or search for the line's text, do not edit it, and do not add it to the allowed list.
   Whether it is a person's address, a role address of a vendor or a service, or text that only
   looks like an address is for the coordinator to read and decide. A person's address on `main`
   is a private detail already in the tree: the coordinator decides what follows, and puts to the
   owner anything that would change Git's history. A role address may become an entry under
   decision 4, and a false match a change to decision 2, in this order's next round.

2. **The pattern.** In `tools/check_private_paths.py`, after `FIXTURE` (line 38), add:

   ```python
   # An email address in its common form: a local part of letters, digits and . _ % + -, then @,
   # then DNS labels, the last of letters only. The lookbehind starts a match only where a run of
   # local-part characters begins, which keeps the search linear on long runs.
   EMAIL = re.compile(
       rb"(?<![A-Za-z0-9._%+-])[A-Za-z0-9._%+-]+@((?:" + DNS_LABEL + rb"\.)+[A-Za-z]{2,63})"
       rb"(?![A-Za-z0-9-])")
   ```

   It is the survey's pattern. It reads every tracked file, binary files and Markdown included,
   and each address it finds is one error. It looks for the common form only. A quoted local
   part, a domain written as an IP address or in letters outside ASCII, and `%40` written for `@`
   are not looked for, though part of one may match. `name@machine`, whose domain has no dot, is
   not an address, and only the names find it. Add no other form, and change none of the seven
   patterns or their exceptions.

3. **The reserved domains** (RFC 2606 and RFC 6761). After the pattern, add:

   ```python
   # Domains reserved for documentation and tests (RFC 2606, RFC 6761), which name nobody.
   RESERVED_DOMAINS = (b"example.com", b"example.org", b"example.net")
   RESERVED_TOP_LEVEL = (b"invalid", b"test", b"example", b"localhost")
   ```

   and, after the constants of decisions 4 and 5:

   ```python
   def reserved(domain):
       """Whether a domain, in any case, is reserved for examples, and so names nobody."""
       domain = domain.lower()
       if domain.rsplit(b".", 1)[-1] in RESERVED_TOP_LEVEL:
           return True
       return any(domain == name or domain.endswith(b"." + name) for name in RESERVED_DOMAINS)
   ```

   So `mail.example.com` and `HOST.TEST` are reserved, and `example.com.synthetic`,
   `synthetic-example.com`, `host.invalid.synthetic` and `host.local` are not. `.local` is left
   out on purpose: it ends a machine's own name on its network, and such a name is a private host
   name.

4. **The allowed list.** After the reserved domains, add:

   ```python
   # Addresses outside those domains that a tracked file may hold: each key an address as bytes in
   # lower case, each value why it names no private person or account. Only a work order adds one.
   ALLOWED_ADDRESSES = {}
   ```

   It starts empty (What and why). Its keys are bytes, as the file's data is. A match whose whole
   text, `match.group()` in lower case, is a key is not an error. Add no entry yourself: an entry
   is the coordinator's, made by an order that gives its reason.

5. **The message** ([ruling 52](../proposals/ruling-52/README.md), point 2: a diagnostic says what
   happened and why, in full, unless saying it would disclose what the caller may not learn).
   After the allowed list, add:

   ```python
   EMAIL_ERROR = (
       "email address outside the domains reserved for examples and not in ALLOWED_ADDRESSES in"
       " tools/check_private_paths.py; a real address can name a person or an account, so use one"
       " at example.com, example.org or example.net, or under .invalid, .test, .example or"
       " .localhost (the address is not printed)")
   ```

   In `check`, find the addresses in a loop of their own, after the loop over `PATTERNS` (lines
   60 to 77) and before the loop over the names (lines 78 to 81), so that a file's errors come in
   that order. Each address that decisions 3 and 4 do not exempt adds the error
   `f"{path}:{line}: {EMAIL_ERROR}"`, its line counted as the other patterns count theirs. The
   error names the file and the line, says why the address fails and what to write instead, and
   never holds the address, its local part or its domain. The other errors, the last line
   `Errors: N` and the exit statuses do not change.

6. **The docstrings.** The module docstring's first line (line 2) becomes
   `"""Check Git-tracked files for private paths, email addresses and CONDUIT_PRIVATE_NAMES.`,
   and its other lines stay. `check`'s docstring (line 42) names the new exemptions, the reserved
   domains and `ALLOWED_ADDRESSES`, beside the fixture's user; its words are yours.

7. **Six tests** in `tools/tests/test_private_paths.py`, after
   `test_a_long_run_of_name_characters_is_checked_quickly`, in the style of the tests above them:
   `track`, `run_check` and `subTest`. Every value is synthetic. Write each address outside the
   reserved domains split at its `@` into two literals, as the tests already split paths
   (`b"someone@" + b"mail.synthetic"`), so that the test file holds none and the tree's own check
   stays green. A value in a reserved domain, or text that is no address, may be written whole.
   Every existing test passes unchanged.
   - `test_email_addresses_outside_the_reserved_domains_are_red`: each value is tracked alone as
     `planted.bin`, after `b"\0first line\n"`; the run exits 1, its output is exactly
     `f"planted.bin:2: {private_paths.EMAIL_ERROR}\nErrors: 1\n"`, and the value's text is not in
     it. The values: `b"someone@" + b"mail.synthetic"`; `b"First.Last+tag@" + b"Mail.Synthetic"`
     (capitals, a dot and a plus); `b"a_b%c-d@" + b"sub.mail.synthetic"` (the local part's other
     characters, and three labels); `b"someone@" + b"example.com.synthetic"`;
     `b"someone@" + b"synthetic-example.com"`; `b"someone@" + b"host.invalid.synthetic"`;
     `b"someone@" + b"host.local"`; `b"https://someone@" + b"git.synthetic/repository"` (user
     information in a URL); and `b"<someone@" + b"mail.synthetic>."`.
   - `test_each_address_is_reported_at_its_own_line`: one tracked file, `two.md`, holds two
     addresses outside the reserved domains on its first line and one on its third. The output is
     exactly `two.md:1:`, `two.md:1:` and `two.md:3:`, each followed by a space and `EMAIL_ERROR`,
     a line each, then `Errors: 3`.
   - `test_reserved_domains_and_text_that_is_no_address_are_green`: one tracked file holds, a line
     each, `b"someone@example.com"`, `b"Someone@EXAMPLE.ORG"`, `b"someone@example.net"`,
     `b"someone@mail.example.com"`, `b"a@b.example.org"`, `b"x@host.test"`, `b"x@site.example"`,
     `b"x@app.localhost"`, `b"x@HOST.INVALID"`, `b"x@a.b.test"` and `b"someone@example.com."`;
     and text that is no address: `b"@AGENTS.md"` (CLAUDE.md's first line),
     `b"    @contextlib.contextmanager"`, `b"actions/cache/save@v6"`, `b"name@1.2.3"`,
     `b"{}@example.org"`, `b"someone@"`, `b"a@.synthetic"` and `b"x@a.b2c"`. The run gives
     `(0, "Errors: 0\n")`.
   - `test_an_allowed_address_is_green_in_any_case_and_its_neighbours_are_not`: with
     `ALLOWED_ADDRESSES` patched (`patch.dict`) to hold `b"role@" + b"vendor.synthetic"` with a
     reason, a tracked file holding `b"From: Role@" + b"Vendor.Synthetic\n"` gives
     `(0, "Errors: 0\n")`. Holding `b"From: other@" + b"vendor.synthetic\n"` instead, the run
     exits 1, and so it does holding `b"From: role@" + b"vendor.synthetic.other\n"`.
   - `test_every_allowed_address_is_one_address_in_lower_case_with_a_reason`: for each entry of
     the real `ALLOWED_ADDRESSES`, `EMAIL.fullmatch` matches its key, the key equals its own lower
     case, `reserved` is false for the key's domain, and the reason is a string with text in it.
     With the list empty it checks nothing; it holds each later entry to decision 4.
   - `test_long_runs_around_an_at_sign_are_checked_quickly`: four runs, each tracked alone as
     `blob.txt`: `b"a" * 100_000 + b"@" + b"a" * 100_000`, `b"a+" * 100_000 + b"@"`,
     `b"x@" + b"a" * 200_000` and `b"a@" * 100_000`. Each gives `(0, "Errors: 0\n")` in less than
     10 s, the bound of `test_a_long_run_of_name_characters_is_checked_quickly`. No run holds a
     dot or a hyphen: decision 9 says why.

8. **AGENTS.md.** Line 129, which begins `python3 tools/check_private_paths.py`, becomes:

   ```text
   python3 tools/check_private_paths.py # tracked files contain no private paths, email addresses outside reserved domains, or configured names
   ```

   Line 155, on what the check needs and how the names are given, does not change.

9. **What stays as it is.**
   - The names: how `CONDUIT_PRIVATE_NAMES` is read (lines 46 to 47), the loop over the names,
     AGENTS.md's line 155 and `.github/workflows/checks.yml`. Supplying the names is for orders 193a
     and 193b and the coordinator's gate; this order touches none of it.
   - The seven patterns, their exceptions and their tests. The tailnet-name pattern is slow on a
     long run of short labels joined by dots or hyphens: on `main` at `d4c8cf8b`, 20,000
     repetitions of `a.` take about 10 s, and the time grows with the square of the length. That
     is known and is not this order's, so decision 7's runs hold neither a dot nor a hyphen, and
     that pattern stays as it is.
   - `CONTRIBUTING.md`, whose settings section counts the checker's built-in patterns, and the
     tracker's row for D11 in `docs/AUDIT.md`, which says the coordinator's runs used "only the
     seven patterns", are the admin's to bring into line in the same pull request (Done when). Do
     not edit them.

## Scope

Change only these files:

- `tools/check_private_paths.py`: decisions 2 to 6;
- `tools/tests/test_private_paths.py`: the six tests of decision 7;
- `AGENTS.md`: line 129 (decision 8).

Change nothing else: not `CONTRIBUTING.md` or `docs/AUDIT.md`, nothing under `.github/`, no other
tool, and no file that holds an address the survey reports.

## Done when

Run this order on the Mac, from the repository root, and keep scratch files under
`target/wo196/`. Decision 1's survey comes first. After your change, run these one at a time; each
passes:

```sh
python3 target/wo196/survey.py | tail -n 1      # ends: outside the reserved domains: 0
python3 tools/check_private_paths.py            # Errors: 0
/usr/bin/python3 tools/check_private_paths.py   # Errors: 0
python3 -m unittest discover -s tools/tests -p test_private_paths.py           # OK
/usr/bin/python3 -m unittest discover -s tools/tests -p test_private_paths.py  # OK
/usr/bin/python3 -m unittest discover -s tools/tests                           # OK
python3 -m unittest discover -s tools/tests -p test_private_paths.py -v 2>&1 | grep -c -E '^test_(email_addresses_outside_the_reserved_domains_are_red|each_address_is_reported_at_its_own_line|reserved_domains_and_text_that_is_no_address_are_green|an_allowed_address_is_green_in_any_case_and_its_neighbours_are_not|every_allowed_address_is_one_address_in_lower_case_with_a_reason|long_runs_around_an_at_sign_are_checked_quickly) .* ok$'   # prints 6
grep -c -F 'email addresses outside reserved domains, or configured names' AGENTS.md   # prints 1
python3 tools/check_docs.py                     # Errors: 0
git diff --check
git diff --numstat -- AGENTS.md                 # one line: 1 1 AGENTS.md
git status --short
```

`/usr/bin/python3` is the Mac's Python 3.9, the oldest that runs these tools; `python3` may be
newer. In the coordinator's trials of these tests, the test file ran in 6 to 23 s with either,
most of it in the existing test of every second octet, which starts Git 256 times.
`git status --short` lists exactly `AGENTS.md`, `tools/check_private_paths.py` and
`tools/tests/test_private_paths.py`, each changed.

The checks must also be able to fail. First run the new tests against `main`'s checker:

```sh
mkdir -p target/wo196/main/tests
git show HEAD:tools/check_private_paths.py > target/wo196/main/check_private_paths.py
cp tools/tests/test_private_paths.py target/wo196/main/tests/
python3 -m unittest discover -s target/wo196/main/tests -v 2>&1 | grep -E '^(FAIL|ERROR): ' | sed -E 's/^(FAIL|ERROR): //; s/ \(.*//' | sort -u
```

It prints exactly these four names, since `main`'s checker finds no address and has no allowed
list:

```text
test_an_allowed_address_is_green_in_any_case_and_its_neighbours_are_not
test_each_address_is_reported_at_its_own_line
test_email_addresses_outside_the_reserved_domains_are_red
test_every_allowed_address_is_one_address_in_lower_case_with_a_reason
```

The other two pass there. To show that each of them, and the allowed list's test, can fail, make
each of these three changes one at a time: copy `tools/check_private_paths.py` to
`target/wo196/check.orig`, make the change, run the named test alone with
`python3 -m unittest discover -s tools/tests -p test_private_paths.py -k NAME 2>&1 | tail -n 1`,
see a line that begins `FAILED`, and copy `target/wo196/check.orig` back. After each,
`cmp tools/check_private_paths.py target/wo196/check.orig` prints nothing.

- `reserved` returns `False` for every domain:
  `test_reserved_domains_and_text_that_is_no_address_are_green` fails.
- The skip for `ALLOWED_ADDRESSES` is taken out:
  `test_an_allowed_address_is_green_in_any_case_and_its_neighbours_are_not` fails.
- The lookbehind, `(?<![A-Za-z0-9._%+-])`, is taken out of `EMAIL`:
  `test_long_runs_around_an_at_sign_are_checked_quickly` fails. This run takes some minutes:
  without the lookbehind, three of its runs take over a minute each, and in the coordinator's
  trials the whole run took 4 to 8 minutes and ended `FAILED (failures=3)`. Give the command at
  least 15 minutes before you call it stuck.

Do not stage anything. In your report, give decision 1's survey output in full (it holds only
reserved domains, counts, paths and lines), mark each choice you made yourself as your own, give
each change above with the `FAILED` line it gave, and give the last line of every check.

After the report, the admin brings `CONTRIBUTING.md` into line in the same pull request. In its
section "Settings this repository relies on", lines 304 and 305, from "checker's seven built-in
patterns" to "and Windows home paths. The", become these three lines; line 306, which begins
"variable is meant to list", does not change:

```text
  checker's eight built-in patterns: home-directory paths, session scratch paths, Codex login files,
  tailnet DNS names, tailnet IPv4 addresses, SSH key fingerprints, Windows home paths and email
  addresses outside the domains reserved for examples. The
```

On line 318, "only the seven patterns" becomes "only the built-in patterns", and so do the same
words in `docs/AUDIT.md`, in the row for D11 (line 178). Where order 193b has merged first, its
texts E2 and G2 have replaced both sentences and neither holds those words: change nothing
there. The admin then runs `python3 tools/check_docs.py`, `python3 tools/check_private_paths.py`
and `git diff --check`. Before it merges, a fresh Codex verifier and a fresh read-only Opus
reader read the result ([ruling 79](../proposals/ruling-79/README.md), point 2). It merges on a
pass of the local gate, the coordinator's interim gate or `tools/gate.py` once order 193b has
merged, and CI's run is a second opinion that the merge does not wait for
([ruling 80](../proposals/ruling-80/README.md), point 10).

## Record

Run on 2026-11-16 on the Mac from `main` `55d2b85a`: the worker (657 s) and a fresh Codex
verifier (156 s), which found no must or should in one round; its report holds a pass and no
detail. Decision 1's survey, run first, found 221 addresses in 45 files, none outside the reserved
domains, and after the change 232 in 46. `tools/check_private_paths.py` now looks for an email
address in its common form in every tracked file, after the seven patterns and before the names.
Each address outside the domains reserved for examples, and not in `ALLOWED_ADDRESSES`, which
starts empty, is one error. The error names the file and the line, says why the address fails and
what to write instead, and never holds the address. Six tests in
`tools/tests/test_private_paths.py` cover the addresses that fail, one error for each address at
its own line, the reserved domains and text that is no address, the allowed list in any case and
its neighbours, the form of each allowed entry, and long runs around an at sign. AGENTS.md's line
for the check names the addresses. The names, the seven patterns and CI are as they were.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran the checker
(Errors: 0 under `python3` 3.9.6 and `/usr/bin/python3`), the test file (19 tests, OK, under
Python 3.13.2 and `/usr/bin/python3`), the whole `tools/tests` suite (108 tests, OK), the grep for
the six new tests (6), `check_docs`, `check_rulings` and `check_roadmap` (Errors: 0 each) and
`git diff --check origin/main...HEAD`, which was clean. It overlaid the three changed files on
`origin/main` at `a771b88b`, and the same checks found no error there. On copies outside the tree,
the new tests failed against `main`'s checker in exactly the order's four names, and each of the
order's three changes made its named test fail; so did comparing allowed addresses without lower
case, and checking a domain's suffix without its dot. The whole-tree check took 3.36 s, against
3.06 s with `main`'s checker. It found one must and two shoulds:

- The must: the admin's edits that "Done when" puts in the same pull request were not made, and
  the branch was two commits behind `origin/main` (ruling 81 and order 192), one of which moved the
  D11 row of `docs/AUDIT.md` down a line. The fixer made the edits on the branch as it stands.
  `CONTRIBUTING.md`'s settings section counts eight built-in patterns and names email addresses
  outside the domains reserved for examples, in the order's three lines, and it and the tracker's
  D11 row (line 179 on `a771b88b`) say "only the built-in patterns". The fixer was not to rebase the
  branch; the admin then rebased it onto `a771b88b`, without conflict.
- No test held `EMAIL`'s lookahead, `(?![A-Za-z0-9-])`: without it the 19 tests still passed,
  since `x@a.b2c`, whose last label is one letter, is no match with or without it. At the
  coordinator's direction, which adds to decision 7's list,
  `test_reserved_domains_and_text_that_is_no_address_are_green` now also holds `x@a.bc2` and
  `x@a.bc-d`, which only the lookahead keeps from matching. On a copy outside the tree with the
  lookahead taken out, that test now fails. The two values are on a line of their own, which keeps
  the file's width (the fixer's choice).
- `check`'s docstring named three exemptions as if they were all of them. It now names, as the
  reader wrote it, the placeholder users, the hex-only fingerprints, the Codex login path in
  Markdown, the fixture's user u, the reserved email domains and `ALLOWED_ADDRESSES`.

The reader also noted a limit of the prescribed pattern, not a defect of this change: an address
joined straight onto a reserved address by a local-part character is missed, since the search
resumes inside the run, where the lookbehind starts no match. It fits "narrows, does not close";
covering it is for a later order, if the coordinator wants it.

The fixer then ran, from the worktree's root under a cleared environment, the checker (Errors: 0
under `python3` and `/usr/bin/python3`, both 3.9.6), the test file (19 tests, OK, under each), the
grep for the six new tests (6), the whole `tools/tests` suite under `/usr/bin/python3` (108 tests,
OK), `check_docs`, `check_rulings` and `check_roadmap` (Errors: 0 each), and `git diff --check`,
before the commit and against `origin/main...HEAD` after it, which was clean.
