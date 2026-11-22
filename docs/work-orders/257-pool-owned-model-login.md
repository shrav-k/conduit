# Work order 257: a model login is a Resource a Pool owns, shared by its Agents, its turns in parallel

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; leave `vectors`
empty: this order adds no case. It needs no network and changes no engine code; if a crate is
missing from the cache, stop and fetch nothing. Run one cargo command at a time, and not
`tools/gate.py`. Never print a whole large file, diff or list. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token or
any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere. Prefix
each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user or machine name, or private repository name.

## What and why

**The owner's answers.** [DEC-042](../decisions/log.md#dec-042-2026-11-21-model-logins-are-pool-owned-resources):
"Pool-owned login Resource (Recommended)"; for Claude, "Account home per machine (Recommended)";
for Codex, "we should support every subscription type". DEC-046: "One login per account,
shared", turns in parallel with detection and a fallback.

**Today.** CTR-013 gives a signing-in Type its own logins; PND-005 holds only the Agent whose
login lapsed.

**Codex** (the Codex study, Q1 and Q4). One `auth.json` per account per machine, linked into
each Agent's otherwise own home. Codex guards the file, not the exchange: an OpenAI maintainer
says a reused refresh token is accepted "on the order of an hour" (Codex issue 10332), OpenAI's
CI page "Do not share the same file across concurrent jobs"; hence the fallback.

**Claude under one account home.** Claude Code's documentation (`authentication`, `claude-directory`
and `memory` under code.claude.com/docs/en/, read 2026-11-21): "Each directory has its own settings,
session history, and claude.ai login or API key"; auto memory is "on by default", shared by a
repository's worktrees, unless `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`. The `.v2` wrapper keeps no
session and loads only its own settings and MCP config. Shared: auto memory while on (order 264
turns it off, DEC-047); `.claude.json` and the `/usage` totals (DEC-047 measures per turn);
per-session files such as `file-history/`, readable by every Agent of the home (PRV-029, DL16); and
any Agent's tools can sign the account out (ruling 70). DEC-042 accepted that cost.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves
3, one login per account per machine, a Pool's, never copied; 4, any Type that signs in serves;
6, a call the fallback holds back reports unavailable, never queued. At risk, 2: the login place,
like `runtime`, is fixed configuration, not a Conduit, authorized by the Resource's Pool
(ACC-033); access ends by the Resource's sign-out or the Agent's retirement.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: Claude Code says "Parallel sessions on one machine share a saved login
and coordinate its renewal"; Codex, "The CLI and extension share the same cached login details";
Kubernetes postpones removing a PersistentVolumeClaim while a Pod uses it; PRV-020 answers
conflict instead. Not taken: a login per Agent (the owner's flaw); one Codex home per account
(shared transcripts); Business tokens (not every plan); one turn at a time from the start
(DEC-046).

**When it runs.** On `main` from #516. Through Conduit's dogfood: one Codex worker turn; a Codex
verifier and a Claude reader, blind and at once; one fix turn; the gate (DEC-046). The names
`loginResource`, login place and "an Agent of a login Resource", the notice as the fallback's lift,
root alone answering it and the Pool authorizing the Agent are the coordinator's, open to the
owner's strike.

**Stop rule.** Stop, change nothing further, and report where a set wording contradicts a promise
no decision changes (give both), a check refuses a line as given (give its error),
`vector_delta.py` shows a change or an engine test fails (name it), or the start check prints
otherwise. A choice no decision governs is yours: list it under `own_choices`.

## The decisions

Wrap each insertion at its file's width.

1. **CTR-013** (`spec/promises/contracts.yaml`). `when` becomes "A Type contract states signIn or
   loginResource." In `then`, "It holds exactly account" becomes "signIn holds exactly account",
   "The Type requires the three" becomes "A Type stating signIn requires the three", and after
   "...contracts cover them." it appends: "loginResource is allowed only for the family Agent, never
   beside signIn, and holds exactly place: a pointer into configuration, the Agent's login place,
   required, of schema resource-reference.v1, that no control's changes holds, compared as for the
   account pointer. Such a Type need not require the three hooks. A contract breaking these rules is
   rejected at install or submission. A submission whose login place refers to a Resource whose Type
   does not state signIn, that holds no string at its account place, or that is retired, is
   rejected, with a diagnostic naming CTR-013, the place and that Resource." `observable`, after
   "...in its own state directory (PRV-029).", appends: "An Agent whose Type states loginResource
   keeps its own state directory and context, and is an Agent of its login Resource: the Pool-owned
   Resource its login place refers to, whose state directory holds one account's login on one
   machine (PRV-029). The account check, sign-in.v1 and the sign-in action are that Resource's,
   never the Agent's (PND-005, PND-010). A login Resource serves whatever subscription plan its
   vendor's own sign-in serves; no Type states a plan." **ACC-033**
   (`spec/conventions/access.yaml`), before "No request is authorized against a retired Pool or
   Fleet.": "Registering an Agent whose login place refers to a Resource (CTR-013), under row 1A,
   2 or 3, also needs that Resource's owning Pool, listing the row's operation, unless that Pool is
   in the same submission."
2. **`type-spec.v1`** (`spec/schemas/contracts.yaml`), as for `toolsDirectory`: `loginResource`
   holds exactly a required `place` pointer and, under `dependentSchemas`, requires `family`
   `Agent` and `not: {required: [signIn]}`; the description says so, citing CTR-013. Where that
   file's header comments, spec/core/README.md's `type-spec.v1` row and spec/schemas/README.md
   name `toolsDirectory`, they name `loginResource` too, with its CTR-013 rules.
3. **PRV-029** (`spec/promises/providers.yaml`). In `observable`, "...passes it to no other run."
   becomes "...passes it to no other run but this: a login Resource's directory is passed to each
   run of an Agent of that Resource (CTR-013), as the value of each variable of the Agent's item
   `environment` whose pointer leads to its login place, in place of the resolved form; that run
   may read and write in it. Only the engine's own runner passes it, to a run on the Resource's
   node; elsewhere the Agent's setup fails with a diagnostic naming PRV-029 and the Resource." After
   "...belong to their contracts." it appends: "A retire-hook.v1 or sign-out-hook.v1 logs out only a
   login in its own instance's directory; an Agent's never logs out its login Resource's, which only
   that Resource's own hooks sign in or out." **MAN-017** (`spec/promises/manifest.yaml`): in
   `when`, "governs the place under SEC-001;" becomes "governs the place under SEC-001, or the
   pointer, given by `environment`, leads to the login place of that type under CTR-013, whose
   variable PRV-029 fills;".
4. **PND-005** (`spec/promises/pending.yaml`).
   - `when`, after "or its contract reports a missing or lapsed login": "; or the contract of an
     Agent of a login Resource (CTR-013) reports one, and that Resource has a string at its
     account pointer".
   - `then` appends: "For an Agent of a login Resource, the instance is that Resource: the action is
     raised for it. An Agent's report holds no instance: a refresh collision can fail one call while
     the login is good. The Resource's own hold under this promise or PND-011's SignInUndone
     condition also holds every operation that starts model work on each Agent of it, any unanswered
     action of the Resource in pendingAction, its diagnostic naming that hold's promise and the
     Resource, whose login awaits sign-in."
   - `observable` appends: "Where such an Agent's call that started model work reports it, the
     engine also raises one notice for the login Resource, unless one is unanswered: its subjects
     are the Resource and the Agent, its evidence names the Agent, the call and its invocation, and
     the call's result and record name the action, then the notice, in pendingAction, with a
     diagnostic saying that the login now takes one call that starts model work at a time until root
     answers the notice. Until then such an operation on an Agent of that Resource, while one on any
     of them runs, is not dispatched and reports unavailable, with the notice's ID in pendingAction
     and a diagnostic naming PND-005, the Resource and the running call, saying that a call on this
     login reported its login lapsed, that the request may be sent again once that call ends, and
     that root lifts this by answering the notice acknowledged. Where the Resource's hold also
     applies, both are reported, the hold's first, in pendingAction and the diagnostic. Its expiry
     lifts nothing: after it, the next call held back raises a new notice, never expiry alone.
     Otherwise calls on one login run in parallel."
5. **PND-009 and PND-010.** PND-009's observable, after "...kept for machine-removal (PND-008).",
   appends: "Only root answers a notice that PND-005 raises for a login Resource, authorized
   through that Resource's owning Pool; another caller's answer reports refused." PND-010's
   `then`, after "...otherwise the permitted request reports invalid.", appends: "For an Agent of
   a login Resource (CTR-013), the diagnostic names PND-010 and that Resource, and says that root
   sends sign-in.v1 for that Resource, which signs in all its Agents."
6. **PRV-020 and LIF-021.** PRV-020's `when` gains, before its last period, "; or a retirement of a
   login Resource would be so dispatched while an Agent of it exists, retired or not (CTR-013)". Its
   observable appends: "A login Resource is likewise retired only once every Agent of it has been
   removed or unregistered, the diagnostic naming each, so its own retire hook logs out a login no
   Agent uses." LIF-021's "Retiring a runtime provider still named by an instance inside or outside
   the Pool" becomes "Retiring a runtime provider still named by an instance, or a login Resource
   still referred to by an Agent, inside or outside the Pool,". LIF-020 and LIF-035 stand.
7. **The rest of `spec/`.**
   - `spec/definitions.md`: after the row `login`, "| login Resource | A Resource whose Type
     states signIn and that the login place of an Agent, an Agent of it, refers to: one
     account's login on one machine for its Agents, each keeping its own state directory and
     context (CTR-013, PRV-029, PND-005). |"; in the row `notice`, "answered acknowledged
     (PND-007)." becomes "and for a login Resource that takes one call that starts model work at
     a time since a call on it reported its login lapsed (PND-005), answered acknowledged
     (PND-007, PND-009)."; the row `state directory`, before "(PRV-029)", gains "; a login
     Resource's is also passed to the runs of its Agents".
   - `spec/README.md`, the limits: "a package keeping a Resource's login runs where no unconfined
     Agent runs;" gains, before its semicolon, ", but for a login Resource, which by design runs
     beside its unconfined Agents, any of which can read it (CTR-013, PRV-029; DEC-042)".
8. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Changed: CTR-013, ACC-033 and `type-spec.v1`, an Agent's login place may refer to a Pool-owned login Resource, for every plan, its Pool authorizing it (DEC-042).
   - Changed: PRV-029 and MAN-017, a login Resource's directory reaches its Agents' runs; an Agent's hooks never log it out (DEC-042).
   - Changed: PND-005, PND-009 and PND-010, an Agent's lapse raises its login Resource's action and one call at a time until root lifts it; only the Resource's own hold holds its Agents (DEC-042, DEC-046).
   - Changed: PRV-020 and LIF-021, a login Resource with an Agent reports conflict at retirement (DEC-042).
   - Changed: `login Resource`, `notice`, `state directory` and the limit at spec/README.md#pending-actions-sign-in-and-the-network on a Resource's login (DEC-042).
   ```

9. **IN_FORCE.md, three lines, no bullet added.** Topic 1: "An agent's subscription and context
   are its own, and anything shared" becomes "An agent's context is its own; its login may be a
   Pool's Resource, and anything else shared", its citation gaining ", as DEC-042 amends 15".
   Topic 9, the line ending "(75 C5).", appends: "From DEC-042 a later Type's Agent may use a
   Pool's login Resource instead, one per account per machine, its turns in parallel (DEC-046;
   CTR-013, PRV-029, PND-005); orders 259 and 264 build the packages." Topic 9, the line
   ending "unchecked in v0 (75 C1).", appends: "A model login Resource runs beside its Agents by
   design (DEC-042)."
10. **The model account.** In `experiments/012-model-checking/account.yaml`, LIF-021's row
    becomes `standing: text changed, not rerun` with `was: holds, not graded`, its note gaining
    "Work order 257 added a login Resource to its PRV-020 sentence." Then run
    `python3 tools/promise_standing.py --write`.

## What to read

The promises named, with CTR-014 and `type-spec.v1`'s `toolsDirectory`, PND-007 and PRV-006;
the limits "Pending actions, sign-in and the network" in spec/README.md.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor 5a6451e7 HEAD; echo "main $?"
for d in 042 046; do sed -n "/^## DEC-$d/,/^## DEC-/p" docs/decisions/log.md | grep -c '^Status: accepted, not done'; done
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/contracts.yaml 'when: >- A Type contract states signIn. then: >- It holds exactly account'
j spec/promises/providers.yaml 'records none and passes it to no other run. A'
j spec/promises/providers.yaml 'names that Resource in `runtime`, retired or not.'
j spec/promises/pending.yaml 'or its contract reports a missing or lapsed login.'
j spec/promises/pending.yaml 'otherwise the permitted request reports invalid.'
j spec/conventions/lifecycle.yaml 'Retiring a runtime provider still named by an instance inside or outside'
j spec/README.md "a package keeping a Resource's login runs where no unconfined Agent runs;"
j spec/promises/manifest.yaml 'unless `secret-reference.v1` governs the place under SEC-001; or the item'
j spec/conventions/access.yaml 'No request is authorized against a retired Pool or Fleet.'
j IN_FORCE.md "An agent's subscription and context are its own, and anything shared"
git grep -c 'loginResource' -- spec | wc -l | tr -d ' '
sed -n '/^- id: LIF-021$/,/^- id: /p' experiments/012-model-checking/account.yaml | grep -c '^  standing: holds, not graded$'
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, two lines `1`, ten lines `1`, `0`, `1`, `engine 0` and `0`.

## Scope

Change `spec/promises/contracts.yaml`, `providers.yaml`, `pending.yaml` and `manifest.yaml`,
`spec/schemas/contracts.yaml` and `README.md`, `spec/core/README.md` (one row),
`spec/conventions/lifecycle.yaml` and `access.yaml`, `spec/definitions.md`, `spec/README.md` (the
limits alone), `spec/CHANGES.md`, `IN_FORCE.md` and `experiments/012-model-checking/account.yaml`,
as the decisions say, and `engine/STANDING.md` by the tool. Change nothing else, not
`docs/decisions/log.md`, a case, an example, the engine, `packages/` or `tools/`; report what else
needs it.

## Done when

The first group prints only 0s on `main` today, and after, what each comment says:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/promises/contracts.yaml 'states signIn or loginResource.' 'never beside signIn' 'stating signIn requires the three' 'whatever subscription plan'  # 1 1 1 1
j spec/conventions/access.yaml "owning Pool, listing the row's"  # 1
git grep -c loginResource -- spec/schemas spec/core/README.md | wc -l | tr -d ' '  # 3
k spec/promises/providers.yaml 'no other run but this:' 'never logs out its login' 'while an Agent of it exists'  # 1 1 1
j spec/promises/manifest.yaml 'the login place of that type'  # 1
k spec/promises/pending.yaml "An Agent's report holds no instance" 'SignInUndone condition also holds' 'subjects are the Resource and the Agent' 'result and record name the action' 'now takes one call' 'next call held back raises' 'both are reported, the hold' 'Only root answers a notice' 'root sends sign-in.v1 for that Resource'  # 1 1 1 1 1 1 1 1 1
j spec/conventions/lifecycle.yaml 'a login Resource still referred to'  # 1
k spec/definitions.md '| login Resource |' 'since a call on it reported' 'passed to the runs of its Agents'  # 1 1 1
j spec/README.md 'which by design runs beside its'  # 1
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c 'DEC-042'  # 5
k IN_FORCE.md "its login may be a Pool's Resource" 'turns in parallel (DEC-046;' 'beside its Agents by design'  # 1 1 1
sed -n '/^- id: LIF-021$/,/^- id: /p' experiments/012-model-checking/account.yaml | grep -c '^  was: holds, not graded$'  # 1
```

Guards, passing today and after; the engine's suite once, last:

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0, with no bullet added to IN_FORCE.md; its no-case ratchet shows head equal to base
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"  # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(promises/(contracts|providers|pending|manifest)|schemas/contracts|conventions/(lifecycle|access))\.yaml|spec/((schemas|core)/)?(definitions|README|CHANGES)\.md|IN_FORCE\.md|experiments/012-model-checking/account\.yaml|engine/STANDING\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes
```

Report each changed sentence before and after, and every check's last line. Stage nothing.

## What follows

- Order 258, the engine, with portable cases failing today: a misplaced `loginResource`;
  MAN-017's binding; the Pool's authorization; `sign-in.v1` for an Agent of a login Resource; a
  keyed lapse holding no Agent; the Resource's own hold holding two; the notice and its expiry; the
  retirement conflict. PRV-029's delivery is a shell test.
- Orders 259, 264: a Codex login Resource Type (linked `auth.json`, `use_agent_identity` off), a Claude
  account-home one, and Agent Types stating `loginResource`, auto memory off.
- Found while writing: a plain turn, as the `.v2` turn is (IN_FORCE.md topic 9), reports a lapsed
  login only by a URL request (PND-005), which neither client sends; order 258 settles it.
- Found while writing: a re-sign-in deletes the login before the browser step (the Codex study,
  §4); whether `sign-in.v1` for a login Resource holds its Agents meanwhile is later.

## Record

Run on 2026-11-21 and 22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's first turn** (23:48 to 23:49) stopped before any edit: as written, IFC-016 and
PND-005's own sentence left an Agent's reported lapse on a login Resource unable to raise an
action. The coordinator decided, open to the owner's strike, that the IFC-016 clause order 258's
text carried is spec and moves here, with PND-005's no-action sentence amended; the DEC-042
CHANGES count is therefore 6, not 5.

**Worker-2's second turn** (23:50 to 00:26) carried out the order with that change. The whole
engine suite passed once (535 passed).

**Two blind reads at once** (00:27 to 00:36), the owner's answer for A77's window:
`turn-verifier-2.turn.v1` found two musts and a should; the Claude reader passed it with two
shoulds and a nit. One fix turn (00:37 to 01:11) applied them all, the coordinator deciding each
and widening the scope to PRV-007 and IFC-025:
- PRV-007 gives an Agent's login-place variable the login Resource's directory path in place of
  the resolved reference, which MAN-017 and PRV-029 then cite;
- IFC-025 lets a call that reports a lapse name the Resource's unanswered sign-in action and
  notice, whether or not it raised them; a combined hold lists only the IDs that exist, the
  hold's first, and its diagnostic names each cause;
- a login notice is answered through the Resource's owning Pool, and the Fleet route is PND-007's;
- the Agent whose call reported the lapse is told it may be a refresh collision, that it may
  resend, and that root signs the Resource in if the login is gone;
- CTR-013 says what an Agent's client writes in the login Resource's directory is shared with
  every Agent of it, as DEC-042 accepted.

The whole engine suite then stopped at `s2_dependent_call_after_close`, as it did once in order
261's run, both times with another worker's suite running; this order changes no engine file, and
the test passes alone. Its two-second wait around close and release is taken for a flake under
the machine's load, for a later order. The gate's run on the squash is the coordinator's, at the
merge.
