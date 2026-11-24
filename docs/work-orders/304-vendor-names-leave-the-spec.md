# Work order 304: vendor names leave the specification, and plan and apply's text goes to the CLI's guide (DEC-105)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
is empty, since this order adds no case. It needs no network and changes no Rust source; if a
crate is missing from the cache, fetch nothing and report it. Run one cargo command at a time,
and not `tools/gate.py`. Never print a whole large file, case, diff or list: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Run no
`conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix each `python3`
with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local path, user,
machine or account name, or private repository name.

## What and why

**The owner's words.** [DEC-105](../decisions/log.md#dec-105-2026-11-23-vendor-names-and-one-tools-flags-leave-the-spec-now):
asked whether product names and the plan and apply flags leave the specification's own text,
"Now, one docs order (Recommended)": vendor names move to the Agent packages' READMEs and Type
descriptions; the plan and apply text moves to the CLI's own guide; the spec README leads with
the six families; this amends DEC-008's placement of that text. The same moves amend where
ruling 82 point 6 (o2, o3, F95), ruling 63 point 2, ruling 53 point 5 and DEC-081 ("`conduit
init` writes neither") placed text in `spec/README.md`; IN_FORCE.md follows.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 4,
nothing assumes a particular vendor's agent: the binding text then names no Agent product, as
the engine's sources already name none. At risk: 3, a shared thing is owned once: CTR-013 keeps
whole its sentence that a login Resource's directory is shared with every Agent of that login;
only one product's file list leaves it. No promise's meaning changes.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)). Yes,
by existing practice: Kubernetes moved each cloud vendor's provider out of its tree, documented
with that provider; the Container Storage Interface specification names no storage vendor; and
Terraform documents its language apart from its CLI. Not taken: renaming other tools (Docker,
the keychain, `github.com`, the precedents the spec cites), which name what a provider wraps or
rule 3 asks for, not an Agent product; the Type descriptions DEC-105 also names, on the owner's
words "one docs order": a Type description's change is a contract release, not a document
(PKG-025), whose install waits for a stage boundary (DEC-080), and ruling 82 point 6, o3, put
the Codex browser limit "not in the descriptions"; DEC-002's rewrite of "What this is".

**When it runs.** On the decision log's branch (DEC-082 to DEC-124 on top of `main`), or on
`main` once it merges. Through Conduit: one Codex worker turn, the blind Codex verifier and
Claude reader at once, one fix turn, the gate (DEC-046, DEC-071, DEC-033). By ruling 89, each
decision says what must hold; how is yours. The words, the heading names and these choices are
the coordinator's, open to the owner's strike (DEC-029): the names counted are the Agent
products and their makers; the Codex browser paragraph leaves with no neutral sentence; Claude's
paste route and sign-in become a `carried: false` login's; the author path's `conduit init` and
`conduit install` sentences leave too; the guide is the existing `engine/CLI.md`, and no
`docs/cli/` is made; the status follows "What this is" under its own heading; IN_FORCE.md's
lines are repointed in place.

**Shared files.** Orders 299, 302, 303, 305 and 307 change `spec/CHANGES.md`; 299 and 303 also
`spec/README.md`; 299, 303, 305 and 307 `spec/interface/implementation.yaml`; 299
`spec/promises/contracts.yaml`; 306 `IN_FORCE.md`: whichever merges later rebases and reruns its
checks.

**Stop rule.** Stop and report only for a contradiction between promises (give both), a step
needing a change outside Scope (give its failing line), or a file outside Scope that must change.
Where this order's wording contradicts a promise it does not change, the promise governs: apply
it and list it under `own_choices`, with every choice no decision governs.

## The decisions

Wrap each insertion at its file's width; rewrap a paragraph you shorten.

1. **What is counted.** Under `spec/`, the strings `claude`, `codex`, `anthropic`, `openai` and
   `chatgpt`, in any case: they cover Claude Code, Claude.ai, `.claude.json`, `claude-code.v2`
   and Codex's browser login. The start check prints 12 in four files; after this order, 0. No
   quoted external URL under `spec/` holds one, so none is excluded.
2. **Three observables,** each promise's `when`, `then`, `level` and `layer` untouched:
   - IFC-049 (`spec/interface/implementation.yaml`): "A Claude Agent's paste route, the
     person's own delivery," becomes "A carried: false login's paste route, the person's own
     delivery,"; "The connection Conduit manages for a Claude sign-in does not bind" becomes
     "The connection Conduit manages for a carried: false sign-in does not bind". The property
     keeps the old scope; PND-013's Interface paste is another route.
   - CTR-013 (`spec/promises/contracts.yaml`): "shared with every Agent of that login,
     including, for a Claude account home, per-session files, `.claude.json`, usage totals and
     auto memory while on, as DEC-042 accepted." becomes "shared with every Agent of that login,
     as DEC-042 accepted." agent-claude-code's README already lists those files under "The .v3
     Agent".
   - NAM-024 (`spec/promises/naming.yaml`): "A default requires a new version segment, as in
     `claude-code.v2` and `codex.v2`." becomes "A default requires a new version segment."
3. **Pending actions, sign-in and the network** (`spec/README.md`):
   - "Claude Code Agent Types state carried: false in v0, because Anthropic's terms forbid
     intermediating Claude.ai credentials: Conduit never reads, holds or relays the credential
     that such a sign-in returns." becomes the one neutral sentence "In v0 a login whose return
     the engine may not carry states carried: false: Conduit never reads, holds or relays the
     credential that such a sign-in returns." The rest of its paragraph stays.
   - "A Claude Agent's paste route is" becomes "A carried: false login's paste route is"; "for
     a Claude sign-in does not bind" becomes "for a carried: false sign-in does not bind".
   - The paragraph "On the engine's own runner, Codex's browser login may open ..." leaves
     whole; IFC-049 and PND-011 already state the relay and the guard.
4. **The facts that survive,** in the Agent packages' READMEs:
   - `packages/agent-claude-code/README.md`, "The .v2 sign-in hook", after its first paragraph
     (ending "(CTR-013, PND-011)."):

     ```text
     `claude-code.v2` and `account-home.v1` state `carried: false` in v0 because Anthropic's terms forbid intermediating Claude.ai credentials. What Conduit then does is in the [specification](../../spec/README.md#pending-actions-sign-in-and-the-network).
     ```

   - `packages/agent-codex/README.md`: the sentence beginning "The browser limit is in the",
     which links the spec, becomes the two sentences that left it, word for word:

     ```text
     On the engine's own runner, Codex's browser login may open the default browser of the engine user's graphical session, where one exists. The hook still relays the printed URL under IFC-049, and the account check (PND-011) stays the only guard.
     ```

5. **Plan and apply,** to `engine/CLI.md`, the CLI's own guide:
   - In "What the short form's meaning covers", the first paragraph stays. The three paragraphs
     from "Plan and apply are tooling composed of existing requests." to "not further engine
     operations." leave, and one paragraph takes their place:

     ```text
     Plan and apply, the reference CLI's commands that bring a running project to its documents, are tooling composed of existing requests and no engine operation; [the CLI's guide] (a link, from spec/README.md, to `../engine/CLI.md#duties-under-the-short-form`) holds their duties and flags.
     ```

   - In `engine/CLI.md`, under "## Plan and apply" and before "### Apply the plan you
     reviewed", a new "### Duties under the short form" holds those three paragraphs word for
     word, every sentence, even one whose fact CLI.md already states.
   - In "An author of documents", "`conduit init` writes a flow-style skeleton. It writes
     neither `fleet` nor `pool`." leaves, and "Package pins belong to the manifest: ... Neither
     command is an engine operation." becomes "Package pins belong to the manifest." CLI.md's
     "Initialize a project or a store" and "Install packages" already state the rest; its
     opening paragraph gains, after "This guide describes the CLI.", the sentence "None of its
     commands is an engine operation."
6. **The opening** (`spec/README.md`):
   - "## What this is" comes first, under the title. The four paragraphs above it now, from
     "Status on 2026-11-11" to "no older document supplies it.", follow that section word for
     word under a new "## Status", before "## Reading paths".
   - After its first paragraph, a new second paragraph:

     ```text
     A set of documents declares objects of six families. An Interface is where a person, or a system outside, acts, and an Agent is a participant whose model decides its own actions: the two actors, which make requests on their own behalf. A Resource is a shared thing that instances use, owned by exactly one Pool, and a Fleet owns Agents as a Pool owns Resources. A Conduit is the relationship of one consumer to one target for one capability, and every permission is an open Conduit. Each object is a template or an instance made from one ([definitions.md] (a link, from spec/README.md, to `definitions.md#objects-and-documents`)).
     ```

   - The sentences of the present second paragraph from "The storage of records and the
     engine's own state" to "reports every restriction as mediated (ACC-027)." become a
     paragraph of their own, word for word, after the one beginning "The provider contracts are
     not a fourth layer." and before "The following are deliberately outside it."
7. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Changed: CTR-013, IFC-049 and NAM-024 name no Agent product in their observables, IFC-049 scoping by `carried: false` instead, and what each promises is unchanged (DEC-105).
   - Changed: `spec/README.md#pending-actions-sign-in-and-the-network` names no Agent product, scoping by `carried: false` instead, and the Agent packages' READMEs hold the rest (DEC-105).
   - Changed: plan and apply's duties and flags leave `spec/README.md#what-the-short-forms-meaning-covers`, and two commands' sentences leave the author path, for the CLI's guide, `engine/CLI.md` (DEC-105).
   - Changed: `spec/README.md#what-this-is` opens the page and leads with the six families; the proof of identity follows the layers, and the status follows under its own heading (DEC-105).
   ```

8. **`IN_FORCE.md`,** in place (line: old → new), adding no bullet:
   - 79: "and spec/README.md." → "and engine/CLI.md (DEC-105)."; 126: "in spec/README.md." →
     "in engine/CLI.md (DEC-105)."; 96: "reading path." → "reading path, and engine/CLI.md
     (DEC-105)."; 81: "skeleton (63 point 2)." → "skeleton, in engine/CLI.md (63 point 2;
     DEC-105)."
   - 80: "**In spec/**: spec/README.md, "What the short form's meaning covers"." →
     "engine/CLI.md, "Duties under the short form" (DEC-105), not spec/."; 184: "spec/README.md,
     "What the short form's meaning covers" (59 A3)" → "engine/CLI.md, "Duties under the short
     form" (59 A3; DEC-105)".
   - 228: "a Claude Agent's" → "a carried: false login's"; 233: "a Claude sign-in" →
     "a carried: false sign-in"; 229: "**In spec/**: on" → "On", and "spec/README.md, not in"
     → "packages/agent-codex/README.md (DEC-105), not in".
9. **Counts.** No promise is added, removed or moved: the totals `check_schemas.py` compares,
   `engine/STANDING.md` and experiment 012's account stay as they are (none of the three
   promises has a row there).

## Start check

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
v='claude|codex|anthropic|openai|chatgpt'
n() { echo "$(grep -c "short form's meaning covers" IN_FORCE.md) $(grep -c DEC-105 IN_FORCE.md) $(j IN_FORCE.md 'Claude sign-in') $(j IN_FORCE.md "Claude Agent's paste route") $(grep -cE '^\s*[-*] ' IN_FORCE.md)"; }
sed -n '/^## DEC-105, /,/^## DEC-106, /p' docs/decisions/log.md | grep -c '^Status: accepted, not done'  # 1
grep -rioE "$v" spec | cut -d: -f1 | LC_ALL=C sort | uniq -c | awk '{print $2, $1}' | paste -sd, -  # spec/README.md 6,spec/interface/implementation.yaml 2,spec/promises/contracts.yaml 2,spec/promises/naming.yaml 2
grep -rliE "https?://[^ ]*($v)" spec | wc -l | tr -d ' '  # 0
sed -n 3p spec/README.md | cut -d: -f1  # Status on 2026-11-11
grep -cE '`conduit [a-z]|--(reopen|recreate|prune|replace|adopt|with-resources|with-agents|withhold|store)\b' spec/README.md  # 12
echo "$(grep -c '^## Plan and apply$' engine/CLI.md) $(ls docs/cli 2>/dev/null | wc -l | tr -d ' ')"  # 1 0
grep -c '^The browser limit is in the \[specification\]' packages/agent-codex/README.md  # 1
k packages/agent-claude-code/README.md 'The home shares `.claude.json` for account details' 'Auto memory would use the shared home.'  # 1 1
n  # 2 0 1 1 344
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | grep '^8\. '  # 8. Compared the promise totals that spec/README.md states with the promise files: 521 promises in 27 files, 226 of the layer `language`, 245 of the layer `convention` and 50 of the layer `interface`; 0 errors.
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check | tail -2 | head -1  # engine/STANDING.md is current; promise rows: 521.
```

Where a line moved, work from where it stands and list it under `own_choices`.

## Scope

Change, as the decisions say: `spec/README.md`, `spec/interface/implementation.yaml`,
`spec/promises/contracts.yaml`, `spec/promises/naming.yaml` and `spec/CHANGES.md`;
`engine/CLI.md`; `packages/agent-claude-code/README.md` and `packages/agent-codex/README.md`;
`IN_FORCE.md`, decision 8's lines alone. Not the decision log, `engine/` sources or
`engine/STANDING.md`, any other file under `packages/` (no `contracts.yaml` or `conduit.yaml`:
no release), `docs/`, `conformance/`, `experiments/` or `tools/`. Where a test fails because the
engine reads the changed spec, report it and change no engine file.

## Done when

Each line prints otherwise before this order and passes after, with `j`, `k`, `n` and `v` as above:

```sh
grep -rioE "$v" spec | wc -l | tr -d ' '  # 0; 12 before
sed -n 3p spec/README.md | cut -d: -f1  # ## What this is; Status on 2026-11-11 before
grep -oE '^(## What this is|A set of documents declares|Conduit innovates only|The specification has three|The provider contracts are not|The storage of records and|## Status|## Reading paths)' spec/README.md | paste -sd'|' -  # ## What this is|A set of documents declares|Conduit innovates only|The specification has three|The provider contracts are not|The storage of records and|## Status|## Reading paths; before: ## What this is|Conduit innovates only|The specification has three|The provider contracts are not|## Reading paths
k spec/README.md 'In v0 a login whose return the engine may not carry states carried: false' "A carried: false login's paste route is the person" 'manages for a carried: false sign-in does not bind'  # 1 1 1; 0 0 0 before
k spec/interface/implementation.yaml "A carried: false login's paste route, the person" 'manages for a carried: false sign-in does not bind'  # 1 1; 0 0 before
k spec/promises/contracts.yaml 'shared with every Agent of that login, as DEC-042 accepted.'  # 1; 0 before
k spec/promises/naming.yaml 'A default requires a new version segment. When installing'  # 1; 0 before
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 -c 'import glob, subprocess, yaml
p = lambda t: {x["id"]: x for x in yaml.safe_load(t) or [] if isinstance(x, dict) and "id" in x}
for f in sorted(glob.glob("spec/promises/*.yaml") + glob.glob("spec/conventions/*.yaml") + glob.glob("spec/interface/*.yaml")):
    a, b = p(subprocess.run(["git", "show", "HEAD:" + f], capture_output=True).stdout), p(open(f).read())
    for i in sorted(a.keys() | b.keys()):
        print(*(i + " " + k for k in sorted(set(a.get(i, {})) | set(b.get(i, {}))) if a.get(i, {}).get(k) != b.get(i, {}).get(k)), sep="\n")' | grep . | paste -sd, -  # IFC-049 observable,CTR-013 observable,NAM-024 observable; nothing before
grep -cE '`conduit [a-z]|--(reopen|recreate|prune|replace|adopt|with-resources|with-agents|withhold|store)\b' spec/README.md  # 0; 12 before
k spec/README.md 'Plan and apply are tooling composed of existing requests.' 'Narrowing never waits and never needs a flag' 'Replacement keeps names stable'  # 0 0 0; 1 1 1 before
k engine/CLI.md 'Plan and apply are tooling composed of existing requests.' 'Narrowing never waits and never needs a flag' 'Replacement keeps names stable' 'None of its commands is an engine operation.'  # 1 1 1 1; 0s before
grep -c '^### Duties under the short form$' engine/CLI.md  # 1; 0 before
k packages/agent-claude-code/README.md "Anthropic's terms forbid intermediating Claude.ai credentials"  # 1; 0 before
k packages/agent-codex/README.md "Codex's browser login may open the default browser of the engine user's graphical session"  # 1; 0 before
grep -c 'spec/README.md#pending-actions-sign-in-and-the-network' packages/agent-codex/README.md  # 0; 1 before
n  # 0 7 0 0 344; 2 0 1 1 344 before
git diff -U0 -- spec/CHANGES.md | grep -c '^+- .*(DEC-105)\.$'  # 4; 0 before
```

Guards, which pass before and must still pass. Run the whole vectors test once, last; never the
whole engine suite:

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | grep -E '^(No-case ratchet|Errors)'  # head equals base; Errors: 0
for t in check_docs check_schemas check_model_account check_private_paths; do python3 tools/$t.py | tail -1; done  # Errors: 0, four times
python3 tools/check_schemas.py | grep '^8\. '  # the line the start check printed, word for word
python3 tools/promise_standing.py --check | tail -2  # the start check's line; Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # 0 changes; 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(README\.md|CHANGES\.md|interface/implementation\.yaml|promises/(contracts|naming)\.yaml)|engine/CLI\.md|packages/agent-(claude-code|codex)/README\.md|IN_FORCE\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && cargo test --offline --release -p conduit-loader --test examples)  # reads spec/
(cd engine && cargo test --offline --release -p conduit-conformance --test core_boundary)  # reads spec/core
(cd engine && RUST_TEST_THREADS=4 cargo test --offline --release -p conduit-core)  # the core's tests
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release -p conduit-conformance --test vectors)  # the whole vectors test, once, last
```

Report each changed sentence, where each moved paragraph now stands, and every check's last line.

## What follows

The admin marks DEC-105 done with this order's pull request, its Type-description part not taken
on the owner's "one docs order", open to the owner's strike. DEC-106's move order takes plan,
apply and author to a client crate, and this guide with them, and repoints to the guide the two
links of `docs/implementation/plan-and-apply/README.md` to the short form's section. DEC-002's
rewrite of "What this is" is its own spec order; a later sign-in order may take more of
IFC-049's login detail to the packages.

## Record

Run on 2026-11-23 through Conduit itself, in Pacific time, on `main` with the decision log
through DEC-131. Written, built, read and fixed in stage 3 of experiment 028; it is no order of
stage 3.

**Worker-1's turn** (14:20 to 14:34) carried out DEC-105: the vendor count under `spec/` went from
12 to 0, the facts moved to the packages' READMEs and the CLI's guide, and the spec README opens
with the six families. It reported, rightly, that the docs guard failed on this order's own text,
whose quoted lines held two links the guard read from this directory; the coordinator named them
instead of linking them.

**Two blind reads** (14:40 to 15:00). The Codex verifier, `root-verifier-2.turn.v1`, passed it
with no finding; the Claude reader (`reader-1`) had no must, one should (the moved paragraphs read
as what the CLI does today) and four nits, one the order's own text. One fix turn, by worker-2 in
a new conversation since worker-1 had moved on, applied the rest. The gate's run on the squash is
the coordinator's, at the merge.
