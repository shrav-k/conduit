# Work order 151: private names and secrets checked before anything private is committed

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to download one
pinned release of an existing secret scanner and its checksums; use it for nothing else, and reach
no other machine. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never print a key, a token, a password or any value a scanner reports: a
finding is reported by its rule, its file and its line only.

## What and why

The full review of 2026-11-14 ([its record](../proposals/full-review/README.md), order 2 of
[its orders](../proposals/full-review/orders.md)) found that `tools/check_private_paths.py` has
three built-in patterns: a home-directory path, a session scratch path and the Codex login file
([FR-534](../proposals/full-review/findings/G6-outside-the-tree.md#fr-534)). Anything else is
checked only against the names in `CONDUIT_PRIVATE_NAMES`, a repository variable the tree cannot
see. A tailnet name, a tailnet address, a host key's fingerprint or a Windows home path in a
committed file would pass, and nothing scans for keys and tokens at all. Later orders of the review
commit redacted runners, reports and an export of the dogfood store's records, the kind of file
most likely to carry one. This order closes that before they run.

## The decisions it applies

1. **Four more built-in patterns** in `tools/check_private_paths.py`, each reported by location and
   category only, as the three existing ones are, never with the matched text:
   - a tailnet name: a DNS name ending `.ts.net` with at least one name character before it,
     matched without regard to case;
   - a tailnet address: an IPv4 address in the shared address space that Tailscale uses (first
     octet 100, second octet 64 to 127), written as four dotted decimal octets;
   - an SSH key fingerprint: `SHA256:` followed by exactly 43 base64 characters that are not all
     hexadecimal digits, so that a hexadecimal digest, such as the one in
     `conformance/language/archives/digest-format/install.json`, is never flagged;
   - a Windows home path: a drive letter, `:\Users\` and a name character, with the same
     placeholder names the home-directory pattern skips (`test`, `user`, `username`, `example`,
     `you`); a placeholder written in angle brackets, such as `C:\Users\<name>`, is not a name
     and is not flagged.
2. **Unit tests**, in `tools/tests/test_private_paths.py`, in the style of the existing ones: for
   each new pattern a tracked file that is flagged, with the reported line naming the file, the line
   and the category and not containing the matched text; and files that are not flagged: a
   64-character hexadecimal digest, `sha256:` followed by one, an address outside the range such as
   100.128.0.1 or 10.0.0.1, `C:\Users\<name>`, and the bare text `.ts.net`.
3. **A secret scan on every pull request and every push to `main`**, by an existing tool:
   [gitleaks](https://github.com/gitleaks/gitleaks), a pinned release. A new job `secrets` in
   `.github/workflows/checks.yml` downloads that release for Linux x64, verifies the archive against
   the release's checksums file before it runs anything, and scans:
   - on a pull request, the commits it adds (the checkout is a merge commit whose first parent is
     the base branch, as the docs job's `git diff --check` step notes), with `gitleaks git` and a
     log range over those commits;
   - on a push to `main`, the pushed commits;
   always with `--redact` and with no report uploaded or written outside the runner's temporary
   space. The job fails on any finding. The `rust` and `docs` gates are not changed; `secrets` is its
   own check.
4. **A configuration file, `.gitleaks.toml`, at the root**, that extends gitleaks' default rules
   and allowlists only what you find, by running the pinned gitleaks over the whole working tree
   (`gitleaks dir` or `detect --no-git`, `--redact`), to be a fake value made for a test or a
   fixture: each allowlisted path or pattern carries a comment saying what it is and why it is fake.
   Allowlist by path and rule where you can, never by a broad pattern. If you find anything that may
   be a real secret, stop: do not allowlist it, do not print it, and report its rule and path:line.
5. **One section in `CONTRIBUTING.md`**, "Settings this repository relies on", in a few sentences
   and without any value: `main` is protected (ruling 74 point 1); CI passes the repository variable
   `CONDUIT_PRIVATE_NAMES`, a comma-separated list of private names that is never written in the
   tree, to `tools/check_private_paths.py`; that checker's seven built-in patterns, named by kind;
   and the secret scan, with where its configuration lives.

## Scope

Change only `tools/check_private_paths.py`, `tools/tests/test_private_paths.py`,
`.github/workflows/checks.yml`, a new `.gitleaks.toml`, and `CONTRIBUTING.md`. Download the scanner
only to a temporary directory outside the repository, and leave nothing of it in the tree.

## Done when

From the repository root, each passes:

```bash
python3 -m unittest discover -s tools/tests
python3 tools/check_private_paths.py
python3 tools/check_docs.py
git diff --check
```

and the pinned gitleaks, run with `.gitleaks.toml` over the whole working tree with `--redact`,
reports no finding. Check the workflow's YAML by loading it with `/usr/bin/python3` and PyYAML. Do
not stage anything. Report each choice you make as your own, the gitleaks version and the checksum
you verified, every allowlist entry with its reason, and every check's last line.

## Record

Run on 2026-11-14 on the Mac from `main` `b1aacc08`: the worker (487 s), a fresh Codex verifier
(263 s) with two musts and a should, a second worker round (385 s) and a second verifier (282 s).
`tools/check_private_paths.py` gains four built-in patterns, each with tests: tailnet DNS names,
tailnet IPv4 addresses, SSH key fingerprints (a hexadecimal digest is never one) and Windows home
paths, with the placeholders the home-directory pattern skips. A new `secrets` job downloads
gitleaks 8.24.2 for Linux x64, checks it against the release's checksums before running it, and
scans the commits a pull request or push to `main` adds, merges against their first parent
included, with `--redact`, printing only rule and location. `.gitleaks.toml` extends the default
rules and allowlists two lines by path, rule and the whole line: planning prose in
`docs/implementation/PROVIDERS.md` and a synthetic key in a test of `packages/scenario-s2-linear`.
`CONTRIBUTING.md` names the settings the repository relies on, without values. The second round
anchored the allowlists to whole lines and added the merge diffs. The one finding left was the
admin's: this order's own text gave the tailnet range as dotted addresses, which the new pattern
flags; the admin reworded it. A fresh read-only Opus reader then verified the order (ruling 79,
point 2), with one must: a Windows home path written with escaped backslashes, as a JSON export
carries it, was not flagged. The admin finished the order: the pattern takes one or two
backslashes, the tailnet-name pattern no longer misses names beside `_`, `-` or a file suffix, the
tests cover more of the address range, the scan says when it did not complete, the archive's
digest is pinned beside the release's checksums, and CONTRIBUTING.md says that no check is
required on `main` by a setting. A second fresh Opus reader read the admin's fix and found no
must. Its shoulds were applied before merge: the tailnet-name pattern starts a match only where a
run of name characters begins, which keeps it linear on long runs (a test checks 200,000 of them);
a Windows home path is flagged however many times its backslashes are escaped; a test covers every
second octet in and out of the tailnet range; names beside `-` are tested; and CONTRIBUTING.md
says that the admin, not a setting, waits for the three checks.
