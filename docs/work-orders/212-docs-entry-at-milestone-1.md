# Work order 212: the entry documents at milestone 1: a README for visitors, a map by reader, one dated state

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order rewrites and edits Markdown files,
creates six, and adds one rule and one option to `tools/check_docs.py` with a test. It has no
network access and needs none. Run cargo only where "Done when" says, offline, with
`CARGO_NET_OFFLINE=true` on the command's own line, one cargo command at a time. Do not run
`conduit`, `codex`, `claude`, the dogfood harness or anything else against a store, and sign in
nowhere. Never print a whole large file: read with `grep -n`, `sed -n`, `head` or a count. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Read nothing outside the repository.

**Every file you write is public.** Write no absolute local path, no user or machine name, no
email address and no at sign. Where a file you quote holds one, leave that part out, as section 1
says for the example. Call the second machine the Linux node, as
[ruling 85](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-85/README.md) defines it, and cite anything held outside the tree
as "the coordinator's notes (private)". `python3 tools/check_private_paths.py` checks part of
this; the rest is yours. The repository's own public GitHub address, as the existing permalinks
and the core package's address give it, is not a private name and may be written.

## What and why

**What it does.** On 2026-11-19 the owner found the README and the docs structure "very out of
date" and asked for a review now, applied when the dogfood reaches its checkpoint. The owner's
manual review of the README and the docs, on Claude Fable, produced a plan with drafts (the
coordinator's notes, private), and the owner chose among its options; ruling 90 records the
choices, and the coordinator's own, open to the owner's strike. The checkpoint is
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md)'s milestone 1: the Agents of the owner's primary
accounts signed in through Conduit on the Mac, running Conduit's work orders one turn at a time.
This order is the plan's step A. It:

- rewrites `README.md` for visitors first, then contributors, in about 110 lines, with a quick
  start and one example of the language;
- replaces `docs/README.md` with a map that routes each reader by what they came for and lists no
  single order and no single ruling;
- keeps one dated statement of where the work stands, the tracker's
  [Where things stand](../AUDIT.md#where-things-stand), which every other entry document links
  instead of stating the state itself;
- rewrites the accounts guide from the merged sign-in orders, fixes the other guides' dated
  lines, and fixes the Overview's named wrong lines, leaving its full re-read to a later order;
- trims `AGENTS.md` to near 150 lines, its check commands and its worker rules unchanged, and
  makes `CONTRIBUTING.md` the one home of what each check needs;
- edits `spec/README.md`'s non-normative lines, minimally;
- repairs the work-order index and splits it into live and merged orders, adds the archive rows
  that are owed, creates the archive of merged orders' index, empty, and adds to
  `tools/check_docs.py` the rule that one row names each order file; and
- keeps every heading that another file links.

**What it does not do.** No order file moves. Step B, an order of its own, run in the next quiet
window of the merge queue, moves merged orders whole into `docs/archive/work-orders/` and
repoints every link to them by script. This order changes no file under `packages/`, no
experiment, `IN_FORCE.md`, the ledger, any live order's text, and no ruling record but the
one-line note on DA12 in section 5.

**Why now.** Every entry document a visitor meets was last brought up to date between
2026-11-10 and 2026-11-12. The README's first paragraph says the language has five families
where [spec/definitions.md](../../spec/definitions.md) says six, and the README never names
[IN_FORCE.md](../../IN_FORCE.md). The map stops at ruling 67. The same state of the build is
written in six places at six dates. This directory's index calls itself the open orders while
most of the files it lists have merged. At the checkpoint, sign-in through Conduit and the Agent
packages 0.2.0 first exist on `main`, so a quick start can run, which is why the review advised
applying it then (the coordinator's reading).

## The decisions it applies

The owner's choices of 2026-11-19, which ruling 90 records:

1. **Two orders.** This is step A, the entry documents. Step B, the move of merged orders, is a
   later order. Nothing moves here.
2. **Visitors first, then contributors.** The README is about 110 lines, with a quick start and
   one example.
3. **The archived order file is the record.** A merged order moves whole, with its Record, into
   `docs/archive/work-orders/`, and that file is its record; this amends
   [ruling 58](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-58/README.md), point 4. Here, the index, the method's page
   and the archive say so (sections 11, 13 and 14), the archive's index is created, empty, and
   the check that keeps it is added (section 15). No row is written to the record for any merged
   order still in the tree (037 on, but 060 to 066).
4. **The Overview: fixed now, read whole later.** Its named wrong lines are fixed here (section
   7); a full re-read against the specification is a separate order after the checkpoint.

The coordinator's, each open to the owner's strike, as ruling 90 records them:

5. **A minimal edit of `spec/README.md`'s non-normative lines** in this order (section 6).
6. **A77's records in one place**, `docs/implementation/a77-records/`, rather than beside their
   orders, so that a merged order moves with its declaration alone (section 11).
7. **DA12's wording:** the check commands in both `AGENTS.md` and `CONTRIBUTING.md`, and what
   each needs in `CONTRIBUTING.md` alone (section 5).
8. **Links inside ruling records** to moved orders are repointed by step B's script. This order
   moves nothing, so it repoints none, and it keeps the archive heading that a ruling record
   links (section 14).

Rulings that bind how it is done:

9. [Ruling 57](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-57/README.md): a document that does not bind may be edited,
   condensed or removed once every relevant piece of it is preserved in `spec/` or a live
   document. Condense only what another live document already holds, and say where in your
   report.
10. [Ruling 67](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-67/README.md): what holds, in order: `spec/`, IN_FORCE.md,
    the ledger, a ruling's record. The README and the map state it in those four lines.
11. [Ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 2: "exact text is prescribed only for the
    specification's wording, diagnostics and prescribed cases", and a Done-when that pins a line
    count is dropped. So the README and the map in sections 1 and 2, the Fable review's drafts,
    are models: this text, or yours to the same effect, keeping every fact, link and heading each
    holds. Giving them, and the few rules and rows quoted word for word below, is the
    coordinator's choice, open to the owner's strike. The specification's lines of section 6 are
    prescribed. Elsewhere this order names what must change and where the facts come from, and
    the wording is yours. The two length checks under "Done when" follow the owner's "about 110"
    for the README and IN_FORCE.md's "near 150" for AGENTS.md; they are not pinned counts.
12. **Every heading that another file links stays**, or its links are repointed in this change.
    `python3 tools/check_docs.py` proves it. Headings known to be linked from outside their file:
    in `docs/AUDIT.md`, every heading it has today, among them
    `### At the handover of 2026-11-12`, `### What happens next`, `### For the next reader` and
    `## Where things stand`; in `docs/archive/README.md`, `## Removed from the tree` and
    `## Additional roadmap candidates`; in `docs/README.md`, `## Records of rulings`; in
    `AGENTS.md`, `## Read first`, `## What holds`, `## Rules`,
    `## If you were given a work order` and `## Checks`; in `CONTRIBUTING.md`, `## Checks` and
    each heading it has today; in `docs/implementation/HANDOVER.md`, `## The queue`, `## Roles`,
    `## The second machine` and `## Restarting the lane coordinator`; in `engine/README.md`,
    `## The core's boundary`, `## Running it` and `## Change a serving project`; in
    `docs/work-orders/README.md`, `## Shared and staged files`.
13. **AGENTS.md stays what every Codex worker reads.** Its command block under `## Checks`, its
    section "If you were given a work order" but for the first sentence (section 4), and the two
    worker rules section 4 names stay word for word.
14. **Where ruling 90's record differs** from decisions 1 to 8, the record governs: apply the
    nearest sound reading and say so in your report.

## What to read

- [Ruling 90's record](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-90/README.md), and its row in
  [the ledger](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-log.md).
- [Ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md): points 2, 3 and 5.
- [IN_FORCE.md](../../IN_FORCE.md): topics 13 and 15.
- [CONTRIBUTING.md, "Keeping decisions and documents current"](../../CONTRIBUTING.md#keeping-decisions-and-documents-current).
- The Records of orders [048b-join](048b-join-build-sign-in-through-the-interface.md),
  [050n](050n-agent-packages-joined.md) and [051b-2](051b2-cli-sign-in.md), and those of the
  lane orders 050n joined, [050j](050j-claude-sign-in-redirect-login.md) and
  [050m](050m-codex-sign-in-hook.md), for what sign-in does and what the packages 0.2.0 are.
- [Experiment 027](../../experiments/027-a5-sign-in/README.md): its amendment of 2026-11-19 and
  the live check's results file beside it, `results-*-1-step-1.md`.
- [Experiment 028](../../experiments/028-a78-dogfood/README.md): the dogfood's dated results, for
  what has run through Conduit.
- [ROADMAP.md](../../ROADMAP.md): step 3's row and the register's rows A5, A77 and A78.
- The current versions of every file in section "Scope", with `grep -n '^#'` first.

## Before you change anything

From the repository root, check that the checkpoint has been reached, and stop, change nothing
and report if any line prints otherwise:

```sh
git status --short | wc -l
git ls-files --error-unmatch docs/work-orders/212-docs-entry-at-milestone-1.md >/dev/null; echo "order $?"
for f in 048b-join-build-sign-in-through-the-interface 050n-agent-packages-joined 051b2-cli-sign-in; do
  F=docs/work-orders/$f.md; echo "$f $(grep -c '^## Record$' $F) $(grep -c '^Not yet run\.$' $F)"
done
ls experiments/027-a5-sign-in | grep -c -- '-1-step-1\.md$'
git grep -l -F '<its first words>' -- 'experiments/028-a78-dogfood/*/results-*.md' | wc -l
test -f docs/proposals/ruling-90/README.md; echo "ruling-90 $?"
grep -c '^| 90 | ' docs/proposals/ruling-log.md
test -e docs/archive/work-orders; echo "archive $?"
git ls-files 'docs/work-orders/*-a77.yaml' | wc -l
```

They print `0`; `order 0`; each of the three orders followed by `1 0` (a Record, filled); `1` or
more (the live check's results merged); `1` or more (a turn has run through Conduit on 0.2.0:
a stage-2 result under experiment 028, which opens with the first paragraph of 028's "What stage
2 cannot show"; put that paragraph's first words, as they stand on your base, for
`<its first words>`; a result of stage 1's order 2 or 3 does not count);
`ruling-90 0`; `1`; `archive 1` (step B has not run); and a count of A77 records, any number,
which section 11 uses. If the three orders have merged but a Record still reads "Not yet run.",
stop: the checkpoint is not reached until the coordinator has written it. If no stage-2 result
is there, stop: milestone 1 is not reached until a turn has run.

Then note the facts that the README's markers and the tracker need, from the sources each names
in sections 1 and 3. Where a source does not say, write what it does say and report the gap;
never write an expected outcome as a result.

## The changes

Write DATE below as the date you write it, YYYY-MM-DD. Keep each link whole on one line, because
`check_docs.py` checks a link only when its text and its target share a line, and wrap prose at
100 characters, as the files do.

### 1. README.md

Replace the whole file with the text below, or yours to the same effect (decision 11), filling
each `FILL-AT-CHECKPOINT` from the source it names, and removing the marker and its parenthesis.
Keep every fact, link and heading it holds; change a fact only where it is false on your base,
and report each such change. The links resolve on `main` today; `check_docs.py` says if one no
longer does.

The example is an excerpt of `spec/schemas/examples/valid/sectioned-project/s1.yaml`: each
Agent's `type` and its `consumes` entry for the shared directory, with each Agent's `config` line
left out (it names an account, and this file holds no address). If those lines changed on your
base, quote them as they stand. Then two sentences of your own, from the example: which Agent may
read the shared directory and which may read and write it; and that closing a Conduit takes the
access away, and restoring it is an explicit request.

Filled, the text below runs to about 125 lines: the example takes it past the owner's "about
110". Cut prose toward 110 where you can without dropping a fact or a link, and report the
count; the coordinator puts any overrun to the owner.

````markdown
# Conduit

Conduit is a configuration language for the relationships between independent agents and the
things they share. An engineer writes a few documents that say what each AI agent may reach
through Conduit, such as a directory, a service login or another agent, and which operations it
may call there. An engine reads the documents and holds the agents to them: a request goes
through only when an open Conduit from that agent to that target lists the operation, and every
request is recorded. A document holds a reference to a secret, never its value.

## Why

Languages such as Kubernetes, Docker Compose and Terraform declare workloads, which do what
their configuration says. An agent decides for itself what to do. So Conduit declares what
surrounds the agent: what it may reach, under which limits, and who answers for what it did.
Conduit invents only the connection and the access. Everything underneath is existing
infrastructure the user picks, reached through a contract that any tool can implement: the
store of secrets, the runtime, the machines, the source of packages. The wire between the
engine and everything it runs is the Model Context Protocol.

## What works today

Status on FILL-AT-CHECKPOINT (the date you write): a draft language and its first build, not a release.

- The specification, every rule a numbered promise that can be checked ([spec/](spec/README.md)).
- An engine in Rust: a strict loader, a pure core in which every decision cites a promise, a
  conformance runner, which reports how many portable vectors it passes, and the `conduit`
  command line ([engine/](engine/README.md)).
- FILL-AT-CHECKPOINT, the whole bullet: which Agents are signed in through Conduit on a Mac,
  each in a package with a home of its own (the packages' version, from 050n's Record and the
  packages' Cargo.toml; the dated first sign-in, from experiment 027's live-check results). Codex
  first; Claude Code only if its sign-in ran and D58's guard is in the harness, as 028's results
  record (IN_FORCE.md, topic 15; ruling 89, point 5). Write what has happened, not what is planned.
- Conduit runs its own work orders, one turn at a time: FILL-AT-CHECKPOINT (the dated evidence
  under `experiments/028-a78-dogfood/`, linked).
- Providers behind contracts: a local process runtime, an SSH runtime, the local machine, a
  local directory of packages and a file store of secrets
  ([the providers](docs/implementation/PROVIDERS.md)).
- The safety promises, model-checked under loss and reordering
  ([experiment 012](experiments/012-model-checking/README.md)).

Not yet: the dogfood's Agents run unconfined under one operating-system user, and every result
says so; two machines; a second provider of each contract; the two independent builds. The
engine's [conformance document](engine/CONFORMANCE.md) states its choices and limits, and the
tracker says [where things stand](docs/AUDIT.md#where-things-stand).

## Quick start on a Mac

You need Git and Rust through rustup; the pinned toolchain installs on first use. The tests
read `spec/`, so run them inside the checkout.

```sh
git clone FILL-AT-CHECKPOINT (the repository's clone URL: the GitHub address that docs/archive/README.md's permalinks use, with .git)
cd conduit/engine
cargo test --release                        # the loader, the core, the vectors and the shell
cargo run --release -p conduit-conformance  # the portable vectors and the coverage they reach
```

To run two real Agents, follow [live S1](engine/examples/s1/README.md): build and pack both
Agent packages, start `conduit serve`, install them, submit the project, sign each Agent in and
request a turn with `conduit call`. Claude Code or Codex must be installed; each signs in through
`conduit sign-in` FILL-AT-CHECKPOINT (the command's form, from 051b-2's Record, as
[accounts](docs/guides/accounts.md) gives it).

## The language in one example

A project is a manifest and a few files. In the short form, one file declares a Fleet of
Agents, the Resources they share and the Conduits between them
([the short form](spec/promises/sections.yaml)). Below, an excerpt of the
[checked example](spec/schemas/examples/valid/sectioned-project/s1.yaml): its two Agents' access
to the shared directory. Each Agent's `config`, left out, names the account it signs in as and
its client's settings.

```yaml
agents:
  agent-a:
    type: example.org/s1/agents#claude-code.v2
    consumes:
      project: {to: {kind: Resource, name: shared}, from: example.org/s1/filesystem#read}
  agent-b:
    type: example.org/s1/agents#codex.v2
    consumes:
      project: {to: {kind: Resource, name: shared}, from: example.org/s1/filesystem#read-write}
```

FILL-AT-CHECKPOINT: two sentences, as section 1 of order 212 says.

## Where it is going

The [roadmap](ROADMAP.md) holds the order of the work. Next: FILL-AT-CHECKPOINT (the roadmap's
order after milestone 1, from step 3's row and the rows of A78 and A5; at the time of writing:
the dogfood's second milestone, parallel slots; the S1 rerun against the authoring bar;
scenarios S2 to S10, among them two machines, a shared service login and swapped providers).
After that, a cold read, the readiness gate and two independent builds from the specification alone.
The repository is public, but the language is not released: v0 is announced once it passes the
experiment program and the owner judges it worth publishing, under its public name, ConduitLang.

## How the repository is organized

| Path | What it holds |
| --- | --- |
| [spec/](spec/README.md) | The specification: promises, definitions, schemas, examples, the core package. It alone binds. |
| [engine/](engine/README.md) | The first build: loader, core, conformance runner and shell, one Cargo workspace. |
| [conformance/](conformance/README.md) | The portable vectors, their fixtures and the coverage manifest. |
| `packages/` | One Rust crate per Agent, provider or scenario server, and the library they share: [Claude Code](packages/agent-claude-code/README.md), [Codex](packages/agent-codex/README.md) and the rest. |
| [experiments/](experiments/README.md) | Evidence: each experiment's method, dated results and counted artifacts. |
| `tools/` | The Python checkers and the local gate. |
| [IN_FORCE.md](IN_FORCE.md), [ROADMAP.md](ROADMAP.md) | Every decision in force outside the specification; the order of the work. |
| [docs/](docs/README.md) | The map of everything else: the tour, the tracker, the guides, the records, the archive. |

## Contributing

Read [AGENTS.md](AGENTS.md), then [CONTRIBUTING.md](CONTRIBUTING.md). What holds, in order:
the specification alone binds; [IN_FORCE.md](IN_FORCE.md) holds every other decision in force;
[the ledger](docs/proposals/ruling-log.md) says what the owner ruled; a ruling's record under
`docs/proposals/` says why. Where two differ, the earlier governs. The owner makes the rulings:
a change of behavior is put as options with a recommendation, and waits. Code is Rust with a
lockfile and a pinned toolchain; the checkers in `tools/` are the exception. Before a commit run
the checks [AGENTS.md lists](AGENTS.md#checks); merges wait on `python3 tools/gate.py`. Never
paste a credential into a chat with an agent; never commit a private path or name.

## License and status

Original Conduit documentation, contracts and code are licensed under [Apache-2.0](LICENSE).
Copyright 2026 Conduit contributors. Reproduced third-party material keeps its own terms
([scope](CONTRIBUTING.md#license-and-attribution)). The specification is a draft, not a
release. The owner rules on the language's direction; collaborators propose and review
([ownership](CONTRIBUTING.md#project-ownership-and-publication)).
````

Where a filled line runs past 100 characters, wrap it. If the packages' READMEs that the table
links still describe the `.v1` packages 0.1.1 ("a separate login home is future work"), keep the
links and report it: those files are not in this order's scope.

### 2. docs/README.md

Replace the whole file with the text below, or yours to the same effect (decision 11). Two of its
links are to files this order creates,
`implementation/README.md` and `proposals/README.md` (sections 11 and 12), so they resolve once
both exist. The `### Records of rulings` heading is kept as a pointer, because other files link
it. The archive of merged orders gets no row here: step B adds it when the first files move.

````markdown
# Documentation map

Not binding. The [specification](../spec/README.md) alone binds behavior and
[IN_FORCE.md](../IN_FORCE.md) holds every other decision in force. This page routes a reader to
the document they need; the top [README](../README.md) says what Conduit is and how to run it.

## Start by what you came for

| You are | Read, in this order |
| --- | --- |
| A visitor | The [README](../README.md); then the [overview](OVERVIEW.md), a tour of the language in ordinary words with one worked example. |
| An author of documents | [spec/definitions.md](../spec/definitions.md), then the author's [reading path](../spec/README.md#reading-paths), which begins with the short form; the [checked examples](../spec/schemas/examples/); [plan and apply](implementation/plan-and-apply/README.md). |
| A writer of a package or provider | The contract writer's [reading path](../spec/README.md#reading-paths); the [providers' plan](implementation/PROVIDERS.md); a package to copy: [Claude Code](../packages/agent-claude-code/README.md), [Codex](../packages/agent-codex/README.md). |
| An engine builder | The builder's [reading path](../spec/README.md#reading-paths); the [conformance cases](../conformance/README.md); the first build's [choices](../engine/CONFORMANCE.md) and [each promise's standing](../engine/STANDING.md). If you were asked to build independently, read [independent builds](implementation/INDEPENDENT_BUILDS.md) first. |
| A contributor, or an agent with a work order | [AGENTS.md](../AGENTS.md), then [CONTRIBUTING.md](../CONTRIBUTING.md); the [method](implementation/README.md) and the [orders](work-orders/README.md). |
| A cold reader or reviewer | The [cold readers' instructions](guides/cold-readers.md); for a whole-repository review, the [full review's record](proposals/full-review/README.md). |
| The coordinator taking over | The [handover](implementation/HANDOVER.md), [the method as run](implementation/METHOD_AS_RUN.md), then [how the work is governed](../IN_FORCE.md#15-how-the-work-is-governed). |
| The owner | [What waits for the owner](AUDIT.md#waiting-for-the-owner); the [guides](guides/README.md); [where the dogfood stands](AUDIT.md#where-things-stand). |

## What holds, in this order

1. [spec/](../spec/README.md): the language, its conventions and the implementation interface.
2. [IN_FORCE.md](../IN_FORCE.md): every other decision in force; *spec pending* marks what is ruled and not yet written.
3. [The ledger](proposals/ruling-log.md): what the owner ruled, when, in the owner's words.
4. A ruling's record, `proposals/ruling-NN/README.md`: why. Its first line says whether it holds.

Where any two differ, the earlier governs (ruling 67). Everything else here does not bind.

## The state of the work

| Document | What it is |
| --- | --- |
| [AUDIT.md](AUDIT.md) | The tracker: [where things stand](AUDIT.md#where-things-stand), dated; what waits for the owner, what is open, the action items. |
| [ROADMAP.md](../ROADMAP.md) | The order of the work, scenarios S1 to S10, one row per experiment and probe. |
| [LANGUAGE_DEFECTS.md](LANGUAGE_DEFECTS.md) | The first defect sweep and what became of each finding. |

## Explanation and guides

| Document | What it is |
| --- | --- |
| [OVERVIEW.md](OVERVIEW.md) | A tour of the language with one worked example. The specification governs where they differ. |
| [guides/](guides/README.md) | What the owner sets up by hand (accounts, 1Password and Linear, the second machine, Docker), how collaborators read cold, the move to the public name. |

## How the work runs

| Document | What it is |
| --- | --- |
| [implementation/](implementation/README.md) | The method: roles and the form of an order, the method as run, the handover, the first build's pre-registration, the providers, the safety catalogue, plan and apply, the independent builds. |
| [work-orders/](work-orders/README.md) | The queue of orders, the report schemas and the vector declarations; its README lists the live orders and, until they move to the archive, the merged ones with their Records. |

## Why: the records

Find a ruling's record through the [ledger](proposals/ruling-log.md), whose rows link each one.
[proposals/](proposals/README.md) lists the records by kind: the ledger; the ruling records,
numbered and by subject; the working papers of open batches and reviews (the first build, the
full review, spec batch 1, the sign-in review). The specification's
[Where the reasons are](../spec/README.md#where-the-reasons-are) links each design record from
the rules it explains.

### Records of rulings

Kept as a pointer: the records are listed in [proposals/](proposals/README.md).

## Evidence and history

| Location | What it is |
| --- | --- |
| [experiments/](../experiments/README.md) | Each experiment's method, dated results and counted artifacts. |
| [research/](research/README.md) | The sourced comparisons behind the decisions. |
| [archive/](archive/README.md) | What left or moved within the tree, with its last version in git; attribution and dissent. |

## Keeping this map current

A document that does not bind may be edited, condensed or removed once every relevant piece is
preserved in `spec/` or a live document and the archive's index lists it (ruling 57). Work
state belongs in AUDIT.md and the roadmap; decision state in IN_FORCE.md and the ledger
([keeping documents current](../CONTRIBUTING.md#keeping-decisions-and-documents-current)).
`python3 tools/check_docs.py` checks every link on this page.
````

Before you replace it, check that every document today's map lists is reachable from the new
map, from `docs/proposals/README.md`, or from an index the new map links; list in your report
any that is not, and add it to the index where it belongs.

### 3. The one dated state: docs/AUDIT.md

The tracker's `## Where things stand` is the one dated statement of the work's state. Bring it
and the sections under it to your base, from the Records and results named under "What to
read", ROADMAP.md's rows, IN_FORCE.md's topics 13 and 15 and ruling 89:

- The file's opening line: `State: DATE`. The table's header: `State on DATE`. Each row says
  what holds on your base, in the row's own form: batch 1 and later spec orders written; the
  sign-in chain and the packages 0.2.0 merged; milestone 1 reached, with its evidence linked;
  how the work runs now (a worker on Codex or Opus, one Codex verifier round, a fresh Opus reader
  before each merge, the local gate; rulings 79, 85, 87 and 89, as IN_FORCE.md, topic 15, has
  them), and where orders run (the Mac and the Linux node). No row quotes a vector count: write
  that the runner reports how many there are, as `spec/README.md` does.
- `### At the handover of 2026-11-12`: keep the heading; condense its text to at most five
  lines that say it was the state at that date and point at
  [the handover's queue](../implementation/HANDOVER.md#the-queue), which keeps that state as a
  dated section (section 11). Before condensing, check each fact of the section against
  HANDOVER's dated queue, keep any that is missing there in the five lines or add it to the
  dated part (at the time of writing, order 074 among those merged since 2026-11-11, and the
  dogfood engine still pinned before 070), and list them in your report.
- `### What happens next`: keep the heading; rewrite it to what follows milestone 1: milestone 2
  (parallel slots once order 203 ships), the S1 rerun's pin at milestone 1 (ruling 89, point 7),
  the second accounts after the pin, and what ROADMAP.md's rows put after them.
- `### For the next reader`: keep the heading and its link targets; cut the text to a pointer at
  the cold readers' guide and the full review's record.
- Under `## Waiting for the owner` and `## Known to be open`, bring the lines that state a
  2026-11-10 or 2026-11-12 build state to your base or date them as history: the rows that say
  the Windows computer is idle or needs the owner's sudo (bring them to ruling 85's state, the
  Linux node), the "494" vector counts (the runner's form), the increment-4 and Codex-package
  follow-ups. Condense D11's row to two lines that keep its links: ruling 80, point 10 settled
  it. Leave `## Layered claim` as it is: its "five families" is a dated claim.

Change nothing else in the tracker. Then make each of these link the tracker's
`#where-things-stand` in place of the state it states today, keeping at most one dated line of
its own where the line says what the document itself was read against: CONTRIBUTING.md's
opening (section 5), the Overview's two "As of" paragraphs and its last section (section 7),
engine/README.md's state paragraph (section 9), the guides' index and the cold readers' guide
(section 10), and HANDOVER.md's dogfood lines (section 11). `AGENTS.md` and the README already
link it.

### 4. AGENTS.md

Trim it to near 150 lines, as IN_FORCE.md, topic 15 asks, not far under:

- Keep everything above `## Checks` but two passages. In "Read first", item 6 becomes: for a
  coordinator taking over, the handover, then [the full review's record](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/README.md)
  (path from AGENTS.md: `docs/proposals/full-review/README.md`), since the full review ran on
  2026-11-14. In "If you were given a work order", the first sentence becomes: under rulings 48
  and 79, the coordinator's session plans and decides, and Codex or Opus sessions carry out work
  orders, as work orders describes; keep its links. Its bullets stay word for word.
- Under `## Checks`, keep the heading, the line before the command block and the command block
  word for word. Replace everything from "What each check needs:" to the end of the `gate.py`
  bullet with one sentence, "What each check needs, and what each covers, is in
  CONTRIBUTING.md. Two of its rules bind every session that changes what they name:", with
  "CONTRIBUTING.md" a link to `CONTRIBUTING.md#checks`, followed by two bullets copied word for
  word: the `promise_standing.py` bullet (it ends "so that the file is committed with the change.") and the
  `cargo test --release` bullet (it ends "which a fixture records."). Live orders invoke the
  first by name ("as AGENTS.md asks").
- Keep the two paragraphs after the `gate.py` bullet (the names that `spec/names.yaml` assigns,
  and running `cargo test --release` in each Rust directory you change) word for word.

### 5. CONTRIBUTING.md, and DA12

- The opening paragraph's dated state (from "As of 2026-11-10" to "a claim of whole-language
  conformance.") becomes one or two sentences: what the repository holds, with a link to the
  tracker's `#where-things-stand` for where the work stands. Keep the first sentence, DA11's
  description.
- The reading-order paragraph after it becomes one sentence that sends the reader to
  [AGENTS.md, "Read first"](../../AGENTS.md#read-first) for the order, and to the map for
  everything else, keeping the links to the cold readers' instructions and to `engine/`'s
  "Running it".
- In "What binds", repoint `docs/README.md#records-of-rulings` at `docs/proposals/README.md`.
- In "Where work belongs": the work-orders bullet says the coordinator plans and decides and
  Codex or Opus sessions carry out the orders (rulings 48 and 79); the code bullet drops "as of
  2026-11-10 holds the Claude Code and Codex Agent packages" for what `packages/` holds: one
  crate per Agent, provider or scenario server, and the library they share.
- In "Keeping decisions and documents current", DA12's clause, "and the same check list in
  AGENTS.md and this guide (DA12)", becomes "the check commands in AGENTS.md and this guide, and
  what each needs in this guide alone (DA12, as the coordinator amended it on DATE, open to the
  owner's strike)".
- In `## Checks`: the command block gains, in AGENTS.md's order, the four commands it lacks
  (`check_rulings.py`, `check_model_account.py`, `promise_standing.py --check` and
  `check_private_paths.py`), each with AGENTS.md's comment, so that the two blocks are the same.
  "What each check needs" becomes AGENTS.md's list as it stands before section 4's trim, every
  bullet, in AGENTS.md's wording; drop this file's shorter `gate.py` bullet and its closing
  "AGENTS.md says the rest." (a link to `AGENTS.md#checks`). "What the checks cover" keeps its bullets; its
  `check_docs.py` bullet adds the rule of section 15, and its "As of 2026-11-10" in the
  `cargo test` bullet and in the paragraph after the list are dated as history or dropped.
- In "Keeping decisions and documents current", "keep dissent in the archive index" becomes
  "keep dissent in the archive (its index links it)", since section 14 moves the collaborator
  sections, their dissent among them, out of the index; name the change in your report.
- In `docs/proposals/docs-audit/README.md`, DA12's row: append to its last cell
  "Amended on DATE by the coordinator, open to the owner's strike (ruling 90): the commands in
  both, what each needs in `CONTRIBUTING.md` alone." Change nothing else in that record.

### 6. spec/README.md, minimally

Change these lines, and nothing else under `spec/`. Each pair below is the text today, then the
text it becomes; a line break inside a sentence may fall where the file's wrapping puts it.

1. In the status paragraph, the date and the build's sentence. Today:

```text
Status on 2026-11-11: draft, begun 2026-11-05.
...
The first build has a strict loader, a pure core on S1 and a runnable engine shell in [engine/](../engine/README.md).
```

It becomes:

```text
Status on DATE: draft, begun 2026-11-05.
...
The first build, in [engine/](../engine/README.md), has a strict loader, a pure core and a
runnable engine; where it stands is in [the tracker](../docs/AUDIT.md#where-things-stand).
```

2. The paragraph on `docs/protocol`. Today it begins "The specification alone binds. Nothing
   under [docs/protocol]" and ends "no older document supplies it." It becomes, with SAME-LINK
   the paragraph's permalink today, unchanged:

```text
The specification alone binds. The design as it stood before the specification,
[docs/protocol](SAME-LINK), left the tree under ruling 39: it binds nothing, and git keeps it
as source material for the type contracts that are still to be written. A behavior that is
missing from `spec/` is a gap in `spec/`, and no older document supplies it.
```

3. Under `## Contents`, "vector coverage is the recorded S1 coverage as of 2026-11-10" becomes
   "vector coverage is S1's, as the coverage manifest records it".
4. The sentence on the four checks. Today, then what it becomes:

```text
These are the four checks that [AGENTS.md](../AGENTS.md#checks) lists, and that page says what each needs.
```

```text
These are four of the checks that [AGENTS.md](../AGENTS.md#checks) lists, and
[CONTRIBUTING.md](../CONTRIBUTING.md#checks) says what each needs.
```

5. Under `## Where the reasons are`, add one line, in the list's form, for each ruling from 76
   on whose decisions `spec/` now holds, which you find in IN_FORCE.md and the ledger (rulings
   78, 82 and 86 are the candidates the review named; check each, and report which you added).

### 7. docs/OVERVIEW.md

Fix its named wrong lines; do not re-read the rest:

- The preamble (from "Not binding." to the end of that paragraph) becomes one line: not binding,
  written from the specification and last read against it on 2026-11-07, so it does not yet
  teach batch 1's additions (the Fleet beyond its row below, the short form, refusal tiers,
  pending actions, sign-in, lifting and tombstones); the specification governs where they differ,
  and a full re-read is a later order.
- The two "As of 2026-11-10" paragraphs after "What Conduit is", and the closing "As of
  2026-11-10" paragraph, become one line that links the tracker's `#where-things-stand`. The
  Project sentence ("this is the basis of an investigation, not a sixth family already in
  `spec/`") goes with them: ruling 60 made the sixth family the Fleet.
- `## The five families` becomes `## The six families`, its sentence says six, and its table
  gains a Fleet row, in the words of [spec/definitions.md](../../spec/definitions.md)'s family
  list and its Fleet entry; the Pool row loses "It is never a consumer or a caller" only if the
  definitions no longer say so. No file links the old heading.
- `## One user journey` and its paragraph are deleted: no file links that heading (check with
  `git grep -n 'one-user-journey'`; if one does, keep it and report).
- The layer table and the list of what is outside the layers, near the end, become two
  sentences that link [spec/README.md](../../spec/README.md#what-this-is), which states both.

### 8. ROADMAP.md

- Step 3's state cell is rewritten to milestone 1, from A78's and A5's rows, keeping its first
  word, an allowed one for `tools/check_roadmap.py`.
- The "In parallel" row's "the work orders that Codex workers carry out" becomes "the work orders
  that Codex or Opus workers carry out (ruling 79)".
- Above the first table, after the paragraph under `## The order of the work`, add a paragraph for
  the visitor the README sends here; this text, or yours to the same effect, in about ten lines:
  "**In plain words.** Conduit is built in steps. Step 2 wrote the language down as numbered
  promises. Step 3, the current one, builds a first engine beside the language and runs one
  scenario, S1, two AI agents sharing a project directory, through every method; since its
  first milestone Conduit also runs its own work orders through its own Agents (A78). Steps 4
  and 5 add a second provider of each contract and nine more scenarios, among them two machines
  and a shared service login; step 6 checks the safety promises with a model checker; step 7 is
  the readiness gate, then two engines built from the specification alone. The register below
  has one row per experiment and probe: it is the coordinator's working record, not a first
  read."

Change no register row.

### 9. engine/README.md and live S1

- The "State on 2026-11-10" paragraph becomes one or two sentences: the increments, by name, done
  and open, and a link to the tracker's `#where-things-stand`; the links to the closeout and the
  increment-4 runs stay.
- In `## Running it`, the paragraph on real Agents says, from 050n's and 051b-2's Records, how
  each Agent signs in now: in a home of its own, through `conduit sign-in`, as the
  accounts guide (`../docs/guides/accounts.md`) gives the steps; drop the shared logins and the
  "pending as of 2026-11-10" sentence.
- `## Runnable Agent packages (work order 023)` condenses to a short paragraph: what the live
  project uses now, and links to the closeout results and to CONFORMANCE.md for the 2026-11-10
  facts it records today. Every fact you drop must be in one of those two files; say which in
  your report. Keep the heading.
- The per-increment notes below stay as they are.
- In [engine/examples/s1/README.md](../../engine/examples/s1/README.md), if its steps do not
  already sign each Agent in through `conduit sign-in`, add the step after the install, with the
  command's form from 051b-2's Record. Change no command you have not found in a Record or in the
  engine's own help text in its sources.

### 10. The guides

- `docs/guides/README.md`: title "Guides"; the credential rule stays a paragraph of its own; the
  accounts row's "For" and "Needed by" brought to the sign-in as built; the closing paragraph's
  state (from "The accounts guide describes" to "a flow the current shell supplies.") becomes one
  link to the tracker's `#where-things-stand`; the full-review row says the review ran on
  2026-11-14 and links its record.
- `docs/guides/accounts.md`: rewrite it whole for the owner holding it at the sign-in, from the
  Records of 051b-2, 050n, 050j and 050m and experiment 027's live-check results: what to have
  ready (which account each Agent is for; the owner's own logins untouched); how `conduit
  pending`, `conduit answer` and `conduit sign-in` show and complete a sign-in, in the forms the
  Records give; reading the URL before opening it, and signing in as the account named; Codex's
  browser login and its device login refused with a diagnostic; Claude Code's redirect login,
  with no return carried by Conduit; what A5's stages are now, from ROADMAP.md's A5 row. Keep the
  title and link ruling 49. Write no command or behavior that no Record or merged text states.
- `docs/guides/cold-readers.md`: its dated lines (the "brought up to date on 2026-11-12"
  sentences and the "As of 2026-11-12" list's build-state items) become a link to the tracker's
  `#where-things-stand` and the items that still hold; keep its link to `#for-the-next-reader`.
- `docs/guides/second-machine.md`: name the machine as ruling 85 defines it, the Linux node,
  whose Linux runs under WSL on the Windows computer, so that it and IN_FORCE.md's "the Windows
  computer" read as one machine; bring its 2026-11-12 state lines (049a1 checkpointed, the three
  parts that "can run now", the computer idle) to your base or date them as history.
- `docs/guides/full-review.md`: one line under its title that the review ran on 2026-11-14 and
  where its record is; its "494 vectors" is a dated record and stays.

### 11. docs/implementation/

- Create `docs/implementation/README.md`: one line that the folder is not binding, then a table
  of its pages by reader (a worker, the coordinator, a builder), one line each:
  WORK_ORDERS.md, METHOD_AS_RUN.md, HANDOVER.md, FIRST_BUILD.md, PROVIDERS.md,
  SAFETY_CATALOGUE.md, INDEPENDENT_BUILDS.md, ENGINE_BUILD_BRIEF.md, `plan-and-apply/` and
  `a77-records/`. `docs/archive/README.md` lists an earlier `docs/implementation/README.md` that
  left the tree: leave that row; it names a past version.
- `HANDOVER.md`: in `## Dogfooding`, the lines that put the next step at re-pinning the dogfood
  engine are dated as history ("On 2026-11-12, ..."), not replaced, and followed by the
  checkpoint's state and a link to the tracker's `#where-things-stand`. Under
  `## The queue`, keep the heading; put above the 2026-11-12 table a short list of the live
  orders on your base, one line each with what it waits for, from the work-order index's live
  table, and head the old part "State at 2026-11-12, `main` at `57813f06`", kept whole as dated
  history.
- `METHOD_AS_RUN.md`: after its first paragraph, a dated note: rulings 85, 87 and 89 changed the
  method after this page's date (merges on the local gate, one Codex round, a fresh Opus reader,
  the pipelined chain); where this page differs, IN_FORCE.md, topic 15, holds, until an order
  brings the page to them. Also repoint section 3's `docs/work-orders/NNN-a77.yaml` to
  `a77-records/NNN-a77.yaml`, with a link to WORK_ORDERS.md's amendment of DATE below.
  Change nothing else.
- `WORK_ORDERS.md`:
  - `## Roles`: "A Codex worker" becomes a worker on Codex or Opus (ruling 79, point 3), in the
    table's form; change nothing else in the table.
  - Item 8 of `## An order` and the opening paragraphs of `## The record` say the record's form
    under ruling 90: orders 001 to 036k and 060 to 066 have their rows on this page; a later
    merged order keeps its Record in its file, and at a condensation the file moves whole, with
    its declaration, to the archive of merged orders, whose index links its Record, and that
    file is its record; no row is written here for it. `## The record` links the archive of
    merged orders' index (`../archive/work-orders/README.md`) as the place that links each moved
    order's Record. That the generated index serves as the record page's per-file links is the
    coordinator's reading of the owner's option ("the record page links each file's Record
    section"), open to the owner's strike, unless ruling 90's record states it. If ruling 90's
    change already wrote this, leave it and say so.
  - A77's records: unless ruling 90's change already placed them, add the section below after
    the last amendment under `## A77's measures, registered before the first paired read`,
    before `## The record`, word for word but for DATE, with "ruling 90" as a link to its
    record. If the start check counted A77 records under `docs/work-orders/`, move each with
    `git mv` into `a77-records/` and say so.
- Create `docs/implementation/a77-records/README.md`: one paragraph, not binding, that this
  directory holds A77's per-order records, `NNN-a77.yaml`, in the form
  [A77's registration](../implementation/WORK_ORDERS.md#the-record-each-order-keeps) gives
  (path from the new file: `../WORK_ORDERS.md#the-record-each-order-keeps`), placed here by its
  amendment of DATE.

The section for `WORK_ORDERS.md`:

```markdown
### Amendment of DATE: the records in one place

Amended on DATE by the coordinator, open to the owner's strike, as ruling 90 records it.
Each of A77's records, `NNN-a77.yaml`, is committed under [a77-records/](a77-records/README.md)
instead of `docs/work-orders/`, so that a merged order moves to the archive of merged orders
with its declaration alone. Nothing else changes: when a record is committed, what it holds
and when it is complete stand as registered and as amended before.
```

### 12. docs/proposals/README.md (new)

An index of `docs/proposals/` in three groups, one line each, so that the map lists no single
ruling: the ledger; the ruling records, `ruling-NN/` (one line for the range, pointing at the
ledger, whose rows link each) and each record by subject (the folders with a `Status:` line, each
with the ruling it records, from today's `docs/README.md` table and the ledger); and the working
papers of batches and reviews, open or closed (`first-build/`, `full-review/`, `spec-batch-1/`,
`reaudit/`, `review-signin-2026-11-16/`, `condensation/` and any other folder that is neither).
List every folder under `docs/proposals/` once. Not binding, and say that the ledger is the
third item of what holds.

### 13. docs/work-orders/README.md

- The opening paragraph becomes, in substance: not binding; the admin writes each order before
  dispatch; the directory is the queue; merged orders keep their Records at the foot of their
  files until a condensation moves them whole to the archive of merged orders, whose index links
  each Record and whose file is each order's record (ruling 90, amending ruling 58, point 4);
  orders 001 to 036k and 060 to 066 left the tree earlier, and their rows are in
  [the record](../implementation/WORK_ORDERS.md#the-record).
- Replace `## Orders still in the directory` by two sections, each a table in file-name order:
  `## Live orders` (open, written ahead or deferred; the linkless rows of orders not yet written
  stay here, with the paragraph that explains them, brought to your base) and
  `## Merged orders` (one row for each order whose `## Record` is filled, that is, present and
  not "Not yet run."). An order merged without a filled Record stays under "Live orders", and you
  list each such one in your report (205b's is the known one).
- Keep each existing row's text as it is. Add a row for each order file that has none: at the
  time of writing, 171a, 171b, 172a, 172b, 178, 179, 193a, 193b, 194, 195, 196, 197, 198, 199
  and 200. Each new row's "What" is your summary of the order's title and first section, in the
  rows' style.
- Under `## Shared and staged files`, the last paragraph ("An order keeps its Record ... removes
  the order and its own schema.") becomes the rule: an order keeps its Record at the foot of its
  file after it merges; at a condensation, a merged order that no live order names, by link, path
  or file name, moves whole with its declaration to the archive of merged orders (link
  `../archive/work-orders/README.md`), and that file is its record (ruling 90); and "Declarations
  stay with the orders that use them." stays.

### 14. docs/archive/

- `docs/archive/README.md`, `## Removed from the tree`: add the row ruling 58, point 9 owes for
  what order 067 removed:
  `| docs/work-orders/: orders 001 to 036k and 060 to 066, with their own report schemas | Merged work orders, removed by order 067 once their rows were in the record (ruling 58, points 4 and 9). | [The record](../implementation/WORK_ORDERS.md#the-record), one row and the preserved Record and decisions per order. | LINK |`,
  with the first cell's path in backticks as the other rows have them, and LINK a permalink in
  the form of the table's `e271d1c` rows to the tree `docs/work-orders` at
  `90c3948cbfb1ecc393c4a07881f67a27dc08ffde`, the commit before order 067 merged, labeled
  `90c3948`.
- After that section, add `## Moved within the tree`: merged work orders move whole to
  `docs/archive/work-orders/`, linked from the index there as `work-orders/README.md`, whose own
  index lists each (ruling 90), and a moved file keeps its history in git (`git log --follow`).
- Move the collaborator sections, from `## Format choice` to the line before `## Attribution`,
  into a new `docs/archive/collaborator-proposals-2026.md`, with a first line that says what it
  is and links back to the index; in the index, a one-line pointer to the new file where they
  were, and keep a heading `## Additional roadmap candidates` with one line pointing at the
  same heading in the new file (a ruling record links the index's anchor). Repoint any other
  link into the moved sections. `## Attribution` stays in the index.
- Create `docs/archive/work-orders/README.md` with the text that section 15's
  `--write-archive-index` writes for an empty directory.

### 15. The one-row rule in tools/check_docs.py

Add to `tools/check_docs.py`, with the standard library alone, as functions that take the
repository root so that a test can run them on a temporary directory:

- **The rule.** Each top-level `docs/work-orders/*.md` but `README.md` and `verify.md` is named by
  exactly one table row of `docs/work-orders/README.md`: a line that starts `| [` and whose first
  link's target is that file name. And `docs/archive/work-orders/README.md` is exactly the text
  that `--write-archive-index` would write. Each failure is one error, naming the file, in the
  checker's form, counted in its last line, `Errors: N`.
- **The option.** `python3 tools/check_docs.py --write-archive-index` writes
  `docs/archive/work-orders/README.md`: the header below, then one row per
  `docs/archive/work-orders/*.md` but `README.md`, in file-name order,
  a table row whose cells are NNN linking FILE, TITLE, and "Record" linking `FILE#record`, where NNN is the file name's part before the
  first `-`, and TITLE its first line with the leading `# ` and any leading `Work order NNN: `
  removed. It prints what it wrote and exits 0. `--walk` and the new option may be given
  together or alone; any other argument is refused, as today.
- **The header**, word for word:

  ```markdown
  # Merged work orders

  Not binding. A merged work order moves here whole, with its Record, from
  [docs/work-orders/](../../work-orders/README.md), and this file is its record (ruling 90,
  amending ruling 58, point 4). Orders 001 to 036k and 060 to 066 left the tree earlier; their
  rows are in [the record](../../implementation/WORK_ORDERS.md#the-record). This index is
  generated: `python3 tools/check_docs.py --write-archive-index` writes it, and
  `python3 tools/check_docs.py` fails when it differs.

  | Order | What | Record |
  | --- | --- | --- |
  ```

- The module's docstring says what the rule checks and what the option writes; CONTRIBUTING.md's
  `check_docs.py` bullet says it too (section 5).
- A test, `tools/tests/test_order_rows.py`, on a temporary directory: a file with one row passes;
  a file with no row, and one with two, each fail; an archive index that differs fails, and
  passes after the option writes it; an empty archive writes the header alone.

## Scope

Change only: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `ROADMAP.md`, `spec/README.md` (the
lines of section 6), `engine/README.md`, `engine/examples/s1/README.md`, `docs/README.md`,
`docs/AUDIT.md`, `docs/OVERVIEW.md`, `docs/guides/README.md`, `docs/guides/accounts.md`,
`docs/guides/cold-readers.md`, `docs/guides/second-machine.md`, `docs/guides/full-review.md`,
`docs/implementation/HANDOVER.md`, `docs/implementation/METHOD_AS_RUN.md`,
`docs/implementation/WORK_ORDERS.md`, `docs/work-orders/README.md`, `docs/archive/README.md`,
DA12's row in `docs/proposals/docs-audit/README.md`, and `tools/check_docs.py`. Create only:
`docs/implementation/README.md`, `docs/implementation/a77-records/README.md`,
`docs/proposals/README.md`, `docs/archive/collaborator-proposals-2026.md`,
`docs/archive/work-orders/README.md` and `tools/tests/test_order_rows.py`; and move, with
`git mv`, any A77 record section 11 names. Any other link that one of these changes breaks is in
scope only to repoint it, and you list each. Do not change `IN_FORCE.md`, the ledger, any ruling
record's text but DA12's row, any order's file, `packages/`, `experiments/`, `conformance/`,
`engine/CONFORMANCE.md` or any other tool. Anything else you find that needs changing goes in
your report.

## Done when

From the repository root, each passes, with its last line as beside it:

```sh
python3 tools/check_docs.py                      # Errors: 0
python3 tools/check_rulings.py                   # Errors: 0
python3 tools/check_roadmap.py                   # Errors: 0
python3 tools/check_schemas.py                   # Errors: 0
python3 tools/check_private_paths.py             # Errors: 0, once the new files are added with git add -N
python3 tools/promise_standing.py --check        # passes: nothing under spec/promises changes
python3 -m unittest discover -s tools/tests      # OK
python3 -m unittest discover -s tools/tests -p 'test_order_rows.py'   # OK, five or more tests run
git diff --check                                 # prints nothing
(cd engine && CARGO_NET_OFFLINE=true cargo test --release)   # passes: section 6 changes spec/
```

Then each of these prints what is beside it:

```sh
git grep -c 'FILL-AT-CHECKPOINT' -- . ':!docs/work-orders' | wc -l            # 0
git grep -l -i 'five families' -- . ':!docs/proposals' ':!docs/archive' ':!docs/work-orders' ':!experiments'
                                     # ROADMAP.md, docs/AUDIT.md and spec/schemas/contracts.yaml only
git grep -l '494 ' -- . ':!docs/proposals' ':!docs/archive' ':!docs/work-orders' ':!experiments'
                                     # docs/guides/full-review.md and docs/implementation/HANDOVER.md only
git grep -n -E '(State|Status) on 20|[Aa]s of 20' -- README.md CONTRIBUTING.md docs/README.md docs/guides/README.md docs/guides/accounts.md docs/guides/cold-readers.md spec/README.md
                                     # only lines that carry DATE, or that date history as history
T=$(mktemp) && cp docs/archive/work-orders/README.md "$T" && python3 tools/check_docs.py --write-archive-index >/dev/null && cmp "$T" docs/archive/work-orders/README.md && echo same
                                     # same
wc -l < AGENTS.md                    # near 150 (IN_FORCE.md, topic 15; not a pinned count)
wc -l < README.md                    # about 110, or the count section 1 reports, with the overrun named for the owner (not a pinned count; report it)
```

And by reading the diff: AGENTS.md's command block, its two kept bullets, its two closing
paragraphs and its work-order bullets are unchanged in text; `README.md` is about 110 lines, or the count section 1 reports, with the overrun named for the
owner, and has no marker left, its count reported; every link the old `docs/README.md` held is reachable from the new map or an
index it links; HANDOVER's 2026-11-12 queue is whole under its dated heading.

The coordinator then has a fresh, read-only Opus reader read the diff before it merges
(ruling 79), lands it between merges, as it touches the index every merge edits, and checks that
the private runners' next run finds `verify.md` and the live orders where they were. Do not
stage anything but with `git add -N` for the check above. Report each choice you made yourself,
each fact you could not source, each heading you kept or repointed, and every check's last line.

## Record

Not yet run.
