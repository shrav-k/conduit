# Work order 290: both Agent packages' READMEs mark `.v1` and `.v2` compatibility only, since only a login Resource signs in, and name the release that removes them (DEC-078, its first part)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors` is
empty: this order changes no case. It changes README text only: run no cargo command, build and
release nothing, and use no network. Never print a whole large file or diff: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Run no
`conduit`, no real `codex` or `claude`, no dogfood harness and not `tools/gate.py`; download and
install nothing, and sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`.
Every file you write is public, your report included: no absolute local path, user, machine or
account name.

## What and why

**The owner's answer.**
[DEC-078](../decisions/log.md#dec-078-2026-11-22-only-login-resources-sign-in-trimmed-after-the-sitting):
only a Resource whose Type states `signIn` is signed in; an Agent's login place refers to one; no
Agent Type states `signIn`. Both Agent packages mark their per-Agent Types as compatibility now,
and the spec's per-Agent branch and the old wrappers are removed in the first package release
after the second-account sitting
([DEC-068](../decisions/log.md#dec-068-2026-11-22-the-second-account-sitting-runs-by-hand-tonight)).
DEC-078's status names two README orders; this order carries both READMEs in one turn, since each
gains one paragraph and one line (the coordinator's choice).

**Today**, read on `main` at `52a29f1a`, both packages at 0.2.5:

- In each `contracts.yaml`, `.v2` is the only Agent Type that states `signIn` (`codex.v2`,
  `claude-code.v2`), implemented by the item `wrapper-v2` and, for Codex, `wrapper-v2-path`.
- `.v1` (`codex.v1`, `claude-code.v1`, item `wrapper`) states neither `signIn` nor
  `loginResource`: its client uses the subscription login in the engine user's default home, as
  each README's paragraph beginning `The runner supplies` says.
- `.v3` and later (`codex.v3`, `codex.v4`, `claude-code.v3` to `claude-code.v5`) state
  `loginResource: {place: /login}`; their `login` refers to a `codex-login.v1` or
  `account-home.v1` Resource, whose Type states `signIn` (CTR-013). Stage 3 of experiment 028
  runs `.v4`.
- Neither README marks a Type. Each says, under "Moving to 0.2.3", how a `.v2` Agent moves to
  `.v3`.
- The archive holds `bin/`, `conduit.yaml` and `contracts.yaml` (`src/bin/package.rs`), not the
  README, so no digest changes and no release is needed.

**Which Types.** The question behind DEC-078, in the coordinator's design notes, named `.v1` and
`.v3`. Read against the contracts: `.v1` breaks the rule (no login place) and is marked; `.v2`
breaks it (`signIn` on an Agent) and is marked; `.v3` keeps it (its login place refers to a login
Resource) and is not marked. This is the coordinator's reading, open to the owner's strike.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once and never copied to an agent: one login Resource per account and
machine, not a login per Agent nor the engine user's own; 4, nothing assumes a vendor's agent: both
packages carry the same mark. None at risk: no Type, item, promise or archive changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Kubernetes' deprecated API migration guide names, for each deprecated
API version, the version to use instead and the release that stops serving it; Rust's
`#[deprecated]` attribute carries a note that names the replacement. Not taken: a warning at
registration (an engine order; DEC-078 asks the packages to mark now); a change to the Types'
descriptions (they are in the archive, so a release, which DEC-078 puts after the sitting);
removing the Types now (the sitting comes first); marking `.v3` (it keeps the rule).

**When it runs.** On `main` from `52a29f1a`, before or after the sitting. Through Conduit's
dogfood: one Codex worker turn (aim for under 30 minutes), then the Codex verifier and the Claude
reader at once, blind (DEC-071: it changes READMEs), one fix turn for both, then the gate
(DEC-033). The words are the coordinator's, open to the owner's strike. No CHANGES line: nothing
under `spec/` changes.

**Engine tests.** None. The order has no case and no regression, and nothing that the core's
tests or the vectors test read changes. Run no cargo command, and never the whole engine suite.

**Stop rule.** Stop, change nothing further, and report only where two promises contradict each
other, a step needs a change outside the scope, or a file outside Scope must change. Where this
order's wording contradicts a promise it leaves unchanged, the promise governs: follow it and list
it under `own_choices`. A choice no decision governs is yours: list it under `own_choices` too.

## The decisions

1. **The paragraph.** In each README, insert the lines below exactly, byte for byte, as a
   paragraph of their own directly before the line `Run these commands in this directory:`,
   with one blank line after them; the blank line before them is already there. The links
   resolve from the README's directory.

For `packages/agent-codex/README.md`:

```markdown
**Compatibility only.** Under
[DEC-078](../../docs/decisions/log.md#dec-078-2026-11-22-only-login-resources-sign-in-trimmed-after-the-sitting),
only a login Resource signs in, and an Agent's login place refers to one. `codex.v1` and
`codex.v2` break that rule and are kept for compatibility only. `codex.v1` has no login place:
its client uses the subscription login in the engine user's default home, and Conduit neither
signs that login in nor checks it. `codex.v2` states `signIn` on the Agent, and each of its
Agents signs in to its own home. For a new Agent, write `codex.v3` or `codex.v4`: its `login`
refers to a [`codex-login.v1` Resource](#the-login-resource), which root signs in for all its
Agents. [Moving to 0.2.3](#moving-to-023) says how to move a `.v2` Agent to `.v3`. The first
release of this package after the second-account sitting
([DEC-068](../../docs/decisions/log.md#dec-068-2026-11-22-the-second-account-sitting-runs-by-hand-tonight))
removes `codex.v1`, `codex.v2` and their items `wrapper`, `wrapper-v2` and `wrapper-v2-path`.
Until then, both Types work as this README describes.
```

For `packages/agent-claude-code/README.md`:

```markdown
**Compatibility only.** Under
[DEC-078](../../docs/decisions/log.md#dec-078-2026-11-22-only-login-resources-sign-in-trimmed-after-the-sitting),
only a login Resource signs in, and an Agent's login place refers to one. `claude-code.v1` and
`claude-code.v2` break that rule and are kept for compatibility only. `claude-code.v1` has no
login place: its client uses the subscription login in the engine user's default home, and
Conduit neither signs that login in nor checks it. `claude-code.v2` states `signIn` on the
Agent, and each of its Agents signs in to its own home. For a new Agent, write
`claude-code.v3`, `claude-code.v4` or `claude-code.v5`: its `login` refers to an
[`account-home.v1` Resource](#the-account-home-resource), which root signs in for all its
Agents. [Moving to 0.2.3](#moving-to-023) says how to move a `.v2` Agent to `.v3`. The first
release of this package after the second-account sitting
([DEC-068](../../docs/decisions/log.md#dec-068-2026-11-22-the-second-account-sitting-runs-by-hand-tonight))
removes `claude-code.v1`, `claude-code.v2` and their items `wrapper` and `wrapper-v2`.
Until then, both Types work as this README describes.
```

2. **The `.v2` section.** In each README, directly after the heading `## The .v2 Agent` and the
   blank line under it, before the line that begins `The package also exports`, insert one line
   and one blank line. The line is, in the Codex README:

   ```text
   `codex.v2` is kept for compatibility only, as [the top of this README](#agent-codex) says.
   ```

   and in the Claude Code README:

   ```text
   `claude-code.v2` is kept for compatibility only, as [the top of this README](#agent-claude-code) says.
   ```

3. **Nothing else.** Change and delete no other line of either README, and no other file. A
   README line that the paragraph seems to contradict stays: report it.

## What to read

Each README from its top to `Run these commands in this directory:`, its sections "The .v2
Agent", "The login Resource" or "The account home Resource", and "Moving to 0.2.3"; the Agent
Types in each `contracts.yaml`; DEC-068 and DEC-078 in `docs/decisions/log.md`, by range with
`sed -n`.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 52a29f1a HEAD; echo "main $?"
test -f docs/work-orders/290-agent-sign-in-types-marked-compatibility.md; echo "order $?"
c=packages/agent-codex/README.md; d=packages/agent-claude-code/README.md
t() { awk 'function out(){ if (f) print n, (s ? "signIn" : (l ? "login" : "none")) } /^---$/{out(); n=f=s=l=""} /^  name: /{n=$2} /^  family: Agent$/{f=1} /^  signIn:/{s=1} /^  loginResource:/{l=1} END{out()}' "packages/agent-$1/contracts.yaml" | paste -sd' ' -; }
t codex; t claude-code
grep -h '^version: ' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
for f in "$c" "$d"; do grep -ci 'compatibility only' "$f"; done
for f in "$c" "$d"; do grep -c '^Run these commands in this directory:$' "$f"; done
for f in "$c" "$d"; do sed -n '/^## The .v2 Agent$/{n;n;p;q;}' "$f" | cut -c1-24; done
grep -c '^## DEC-078, 2026-11-22, only login Resources sign in, trimmed after the sitting$' docs/decisions/log.md
git status --short | wc -l | tr -d ' '
```

They print `main 0`; `order 0`; `codex.v1 none codex.v2 signIn codex.v3 login codex.v4 login`;
`claude-code.v1 none claude-code.v2 signIn claude-code.v3 login claude-code.v4 login claude-code.v5 login`;
`version: 0.2.5` twice; `0` twice; `1` twice; `The package also exports` twice; `1`; and `0`.
Another list of Types means a release changed them: stop and report it.

## Scope

Change `packages/agent-codex/README.md` and `packages/agent-claude-code/README.md`, by adding
the lines decisions 1 and 2 give and nothing else. Change nothing else: not `contracts.yaml`,
`conduit.yaml`, `src/`, `tests/`, a `Cargo.toml` or lockfile, `spec/`, `engine/`,
`conformance/`, `docs/` (this order's text and the decision log among it), `experiments/` or
`ROADMAP.md`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise today and passes on a correct result. The second
group are guards, which pass today and must still pass.

```sh
c=packages/agent-codex/README.md; d=packages/agent-claude-code/README.md
o=docs/work-orders/290-agent-sign-in-types-marked-compatibility.md
b() { awk -v k="$1" '/^```markdown$/{n++; f=(n==k); next} /^```$/{f=0} f' "$o"; }
p() { awk '/^\*\*Compatibility only\.\*\* /{f=1} f&&/^$/{exit} f' "$1"; }
diff <(p "$c") <(b 1) >/dev/null; echo "codex $?"  # codex 0; today codex 1
diff <(p "$d") <(b 2) >/dev/null; echo "claude-code $?"  # claude-code 0; today claude-code 1
for f in "$c" "$d"; do grep -m1 -E '^(\*\*Compatibility only\.\*\* |Run these commands in this directory:$)' "$f" | grep -c '^\*\*'; done  # 1 twice; today 0 twice
for f in "$c" "$d"; do grep -B1 '^\*\*Compatibility only\.\*\* ' "$f" | head -1 | wc -c | tr -d ' '; done  # 1 twice; today 0 twice
for f in "$c" "$d"; do grep -B2 '^Run these commands in this directory:$' "$f" | head -1 | grep -c 'README describes\.$'; done  # 1 twice; today 0 twice
for f in "$c" "$d"; do sed -n '/^## The .v2 Agent$/{n;n;p;q;}' "$f" | grep -c '^`[a-z-]*\.v2` is kept for compatibility only, as \[the top of this README\](#agent-[a-z-]*) says\.$'; done  # 1 twice; today 0 twice
for f in "$c" "$d"; do sed -n '/^## The .v2 Agent$/{n;n;n;n;p;q;}' "$f" | cut -c1-24; done  # The package also exports twice; today other lines
git diff --numstat -- "$c" "$d" | awk '{print $1, $2, $3}'  # 17 0 packages/agent-claude-code/README.md, then 16 0 packages/agent-codex/README.md; today nothing
```

```sh
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/set_names.py --check | grep -o 'Errors: [0-9]*'  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^packages/agent-(codex|claude-code)/README\.md$' | wc -l | tr -d ' '  # 0
```

Report, per README, the lines added and where; every check's last line; and each README line
that the paragraph seems to contradict.

Stage nothing. The verifier's and the reader's turn follows; then the gate and the merge.

## What follows

Named here and not in this order:

- **After the sitting** (DEC-078): one order per package removes `.v1`, `.v2` and their items in
  its first release after the sitting. The spec order that removes the per-Agent branch of
  PND-005, PND-010 and PND-011 and CTR-013's `signIn` on an Agent Type, with CHANGES lines citing
  DEC-078, lands with those releases, not before: CTR-013 rejects at install a contract that
  breaks its rules, and the 0.2.5 contracts state `signIn` on `.v2`.
- **Found while writing:** `engine/examples/s1/templates.yaml` names `codex.v1` and
  `claude-code.v1`, and experiment 028's stage-2 Fleet names `codex.v2` and `claude-code.v2`;
  the trim's orders decide what each becomes.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (14:23 to 14:26) added the owner's DEC-078 notice to both Agent packages'
READMEs: only a login Resource signs in; the `.v1` and `.v2` Types are compatibility only, and the
first release after the second-account sitting removes them and their items. The inserts match
this text byte for byte; the document checks pass.

**Two reads** (the Codex verifier 14:27 to 14:30, the Claude reader 14:31 to 14:36), both passing
with no must. The reader's shoulds, applied by the coordinator in a few sentences: the notice now
tells a `.v1` user how to move off (replace the Agent with a `.v3` or later one whose `login`
names a signed-in login Resource), and that an Agent already registered keeps the release it
resolved until it is replaced. The gate's run on the squash is the coordinator's, at the merge.
