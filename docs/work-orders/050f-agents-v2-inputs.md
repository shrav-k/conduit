# Work order 050f (P2f): the `.v2` inputs: account, toolsDirectory, and the description sentences (i) and (ii)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only for tests that
bind a loopback port; it adds no crate, so cargo fetches nothing, and you use the network for
nothing else. Run one cargo command at a time. Never print a whole large file: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Never
read or print any process's environment or arguments, never run a command that lists every
process on the machine, such as `ps -e`, `ps aux` or `top`, and never signal a process that a test
of this order did not start. Never run the real `codex` or `claude` programs, not even for their
help or version, and sign in nowhere. The tests use each package's scripted client, in temporary
directories only, and the engine is run only to initialize a store and register projects.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: `wo/050-agents-0.2.0`, which holds orders 050a, 076 and 078, 050a's Record, and orders 050c,
050d and 050e, each with its Record, rebased onto `main` after order 048a-1 (#352) and order 202's
work merged. `BASE`'s tip holds this order file, committed: the coordinator rebases the branch onto
`main` after this order's text merges, and sets `BASE` to the branch's tip. Before you change
anything, check from the repository root that you find there what that gives, and stop, change
nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor e4ceb4c8 "$BASE"; echo "main $?"
grep -c 'Ruling 83: subscription only' engine/crates/conformance/examples/make_archives.rs
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050f-agents-v2-inputs.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050a-agents-v2-own-tools.md docs/work-orders/050c-agents-v2-client-home.md docs/work-orders/050d-agents-v2-process-tree.md docs/work-orders/050e-agents-v2-failed-turn-fields.md
test "$(git rev-parse "$BASE:packages/agent-codex")" = d78544fea0262d436d48bb6547656b5003c3151b; echo "codex $?"
test "$(git rev-parse "$BASE:packages/agent-claude-code")" = 90039f30c0174377931d1648ab1cffd4f6bbbedf; echo "claude $?"
python3 -c "import json; d=json.load(open('conformance/fixtures/s1/packages/agents-0.2.0/contracts.json')); v=[t['spec']['schema'] for t in d if t['metadata']['name'].endswith('.v2')]; print([s['required'] for s in v], ['default' in s['properties']['account'] for s in v], ['apiKey' in s['properties'] for s in v], ['oneOf' in s for s in v])"
grep -cE 'apiKey|toolsDirectory' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
grep -c "On the engine's own runner" packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml
grep -c '0\.167' packages/agent-codex/contracts.yaml
grep -cE '/(account|apiKey)$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml
```

They print `main 0` and `1` (`e4ceb4c8` is `main` after order 048a-1's merge; the grep is order
202's work), `head 0`, `order 0` and `0`; then `1` for each order file (the four Records); `codex 0`
and `claude 0` (the two packages are those 050e's Record describes, since no commit on `main` after
the cut touches them); `[['account'], ['account']] [False, False] [False, False] [False, False]`
(the fixture as order 202 realigned it); `0` for each `contracts.yaml`; `1` for Codex and `0` for
Claude Code; `1`; and `0` for each `conduit.yaml`.

**What it builds.** The `.v2` Types at `BASE` take no account: their schemas hold `directory`,
`turnDeadlineSeconds`, `effort`, `accountServices` and `model` (and, for `claude-code.v2`,
`peerMessaging`), every one required with a default, and no `oneOf`; they state no
`toolsDirectory`; `codex.v2`'s description names "Codex 0.167.0"; and `claude-code.v2`'s says its
tools "run as the engine's operating-system user" with no sentence for another runtime provider.
This order gives both `.v2` Types the inputs that MAN-017 and NAM-024 freeze first: `account`,
required with no default, a string or null, as order 044 gave it to the fixture
`conformance/fixtures/s1/packages/agents-0.2.0` and order 202 kept it; no `apiKey` and no `oneOf`
([ruling 83](../proposals/ruling-83/README.md)); `toolsDirectory: {directory: /directory}`; and the
description sentences (i) and (ii) of ruling 82, point 5. No item binds `account`, so it reaches
neither the wrapper nor the client, and the wrapper does not change but for decision 9's words: its
refusal of a home that holds a `.env` no longer gives a reason that names `apiKey`.

It carries, from [the coordinator's plan of 050](117-plan-s2-s3.md) (order 117's 050 entry: "050f,
the `.v2` inputs: account, toolsDirectory, and the description sentences (i) and (ii), with no
apiKey, and the wrappers' `.env` refusal reworded without its apiKey reason (ruling 83)"), and
from the targeted review of the 0.2.0 sign-in surface
([section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists)):

- [`claude-code.v2`](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2)'s item 1,
  its schema part: "`toolsDirectory: {directory: /directory}`; `account` is required, with no
  default: a string or null under the fixture's `oneOf` where Q2 puts `apiKey` in [...]; `apiKey`
  as Q2's answer allows. Done when a loader vector accepts the Type and a template binding
  `/account`, and `/apiKey` where Q2 puts it in, passes MAN-017." Ruling 83 answers Q2 for 0.2.0:
  `apiKey` is not in `.v2`, so the `oneOf` and every part on `apiKey` fall away, and `account` is a
  string or null on its own. Its `signIn`, its `requires` and its hooks are orders 050h to 050j's.
- [`codex.v2`](../proposals/review-signin-2026-11-16/README.md#050-codexv2)'s item 1, the part
  common to both outcomes of the live check: "`account` and `apiKey` as Q2 allows", which under
  ruling 83 is `account` alone. Whether `codex.v2` states `signIn` waits for that check (ruling 82,
  point 1), and is orders 050l and 050m's.
- Item 6 of both lists, for sentences (i) and (ii): the description "names no client version",
  and carries "decision 25's other-runtime sentence in both, with 'On the engine's own runner'"
  (Q1). Sentences (iii) to (vi) are other orders': (iii) is at `BASE` already ("fails and ends the
  conversation"), (iv) is 050h's, (vi) 050k's and (v) 050m's. The review's verdicts
  [M16, M17 and M87](../proposals/review-signin-2026-11-16/verdicts.md) and its finding
  [V17](../proposals/review-signin-2026-11-16/findings.md#v17-what-the-v2-descriptions-would-freeze)
  say why: a client version in a frozen description is "wrong at publication and uncorrectable
  within .v2", and without decision 25's sentence "a later Claude PATH item would make the frozen
  description false".

Ruling 83, the owner's ruling of 2026-11-18, sets the inputs. The Agent packages 0.2.0 are
subscription-only: `apiKey` is not in `.v2`; `account` stays required with no default, a string or
null, and the `oneOf` of `account` and `apiKey` goes; API access comes later as a declared gateway
route. It reverses [ruling 82](../proposals/ruling-82/README.md#the-decisions)'s point 2, which put
`apiKey` in `.v2` as an explicit setting. Point 1 of ruling 82 still holds: the engine never falls
back from a sign-in to a key, and with no `apiKey` there is no key to fall back to. Its point 5
gives the descriptions' sentences (i) and (ii). A null `account` signs in to no subscription.

The fixture `conformance/fixtures/s1/packages/agents-0.2.0`, which order 044 shaped and
[order 048a-1](048a1-build-sign-in-type-checks.md) made require `apiKey`, holds no `apiKey` and no
`oneOf` since [order 202](202-agents-v2-fixture-subscription-only.md) realigned it under ruling 83.
This order states `account` as the fixture states it and changes nothing under `conformance/`;
decision 7 compares every member of the fixture's schema.

`toolsDirectory` is CTR-014's member (`spec/promises/contracts.yaml`), whose checks
[order 157](157-own-tools-reported.md), the full review's order 15, built: "On the branch of orders
050a and 076, the `.v2` Agent Types of the Agent packages' 0.2.0 will state
`toolsDirectory: {directory: /directory}`". With it, the engine reads `directory` only to report
the reach of the client's own tools when the instance is inspected (CTR-014's observable,
LIF-013).

**What the coordinator probed.** In a scratch directory outside the repository, the coordinator
made both `contracts.yaml` as decisions 1 to 3 give them, packed each with `BASE`'s `conduit.yaml`
and a placeholder wrapper into an archive, and registered decision 6's projects with an engine
built from order 048a-1's branch. A and P were accepted in both packages with no diagnostic, and N
was refused in both with one error, MAN-017's, naming `negative-account`, `PROBE_API_KEY` and
`/apiKey`. It also saw that a template with `config: {}`, or one that sets `apiKey`, is accepted at
registration with no diagnostic: the whole schema, with `additionalProperties: false`, is checked
when an instance is made (INS-014). Decision 7's check passed on the probe's files and failed on
`BASE`'s. That was a probe, not a result: your run is the evidence.

**What the branch already does, so that you do not redo it.** Each package's `contracts.yaml`
holds the 0.1.1 contracts as a byte prefix, then the `.v2` Type as the last YAML document
(050a's decision 2). Its `conduit.yaml`'s `wrapper-v2` item (and, in `agent-codex`,
`wrapper-v2-path`) binds `/directory`, `/turnDeadlineSeconds`, `/effort`, `/accountServices` and
`/model` (and, in `agent-claude-code`, `/peerMessaging`), each `plain: true`. Each README's
`## The .v2 Agent` opens with "All properties are required after SCH-009 fills their defaults;
authors may omit them." and a table of the inputs with the columns `Input`, `Default / bounds` and
`Wrapper variable`. The Codex README records the client version read ("Native-client launch
baseline: Codex 0.167.0", and "Codex 0.167.0 still offers its collaboration tools (finding
4-06)"), and its section `## The .v2 Agent's own home` says "An API key is used only where a
template sets `apiKey` explicitly, never as a fallback (ruling 82, points 1 and 2).", which ruling
83 makes untrue. The wrapper's refusal of a home that holds a `.env` ends with "since an API key is
used only where a template sets apiKey", in each package's `src/lib.rs` and `tests/wrapper.rs`, and
the Codex README quotes it in its list of refusals; ruling 83 makes it untrue too, and decision 9
rewords it.

## The decisions it applies

1. **The schema.** In each package's `.v2` Type (`codex.v2`, `claude-code.v2`), under
   `spec.schema`:
   - after the last line of `properties`, add:

     ```yaml
           account: {type: [string, 'null']}
     ```

   - append `account` to the end of `required`, which keeps every name it holds, in order.

   Nothing else in the schema changes: no `apiKey` and no `oneOf`; every other property, its
   default and its bounds stay; and `additionalProperties` stays false. Write `'null'` quoted, so
   that YAML reads a string. So `account` has no default and a template states it, as a string or
   as null; and a template that sets `apiKey` names a property the schema does not hold, which
   INS-014 refuses when an instance is made. `account` is the fixture's member as order 044 gave
   it; order 048a-1 added `apiKey` to `required`, and order 202 removed `apiKey` and the `oneOf`.

2. **`toolsDirectory`.** In each `.v2` Type's `spec`, after `schema` and before `offers`, add the
   line `  toolsDirectory: {directory: /directory}`. `directory` is required and confined to
   string or null, and the Types list no control, so CTR-014's three rules hold.

3. **The descriptions (i) and (ii).** In the `.v2` Types' `metadata.description` alone:
   - In `codex.v2`, "although Codex 0.167.0 still offers its collaboration tools" becomes
     "although Codex still offers its collaboration tools". The description then names no client
     version; the README keeps the version read.
   - In `claude-code.v2`, "They run as the engine's operating-system user and nothing confines them
     in v0." becomes "On the engine's own runner, they run as the engine's operating-system user
     and nothing confines them in v0."; and after "no Conduit grants, limits or records it (ruling
     70)." insert "Under another runtime provider, these tools run on the machine of the instance's
     node, as the operating-system user and with the limits that its contract states.", the words
     `codex.v2`'s description holds at `BASE`.

   Nothing else in either description changes. Both are folded scalars (`>-`), so rewrap the lines
   you touch as you like, at the same four-space indent and within 100 columns; decision 7 compares
   the text YAML reads.

4. **No binding, and no other change to the wrapper.** No item of `conduit.yaml` binds `/account`,
   and `conduit.yaml`, `Cargo.toml` and `Cargo.lock` do not change. The wrapper's environment, its
   arguments and its refusals are as at `BASE`, and the wrapper does not change but for decision
   9's words.

5. **The READMEs**, both, in the package's own client's terms. Write no absolute local path and no
   client version beyond those the Codex README already records.
   - In `## The .v2 Agent`, "All properties are required after SCH-009 fills their defaults;
     authors may omit them." becomes "All properties are required after SCH-009 fills their
     defaults. Authors may omit every one but `account`, which has no default." Keep the new
     sentence on one line, as each README keeps its sentences.
   - The input table gains one row, after its last:

     ```text
     | `account` | none; a string or null | none |
     ```

   - After the paragraph that follows the table, add this paragraph, exactly, each sentence on a
     line of its own:

     ```text
     `account` names the subscription account that the client is to be signed in to, and has no default, so every template states it; a null `account` signs in to no subscription.
     The package's 0.2.0 is subscription-only: the Type states no `apiKey` and takes no API key, and API access comes later as a declared route (ruling 83).
     Nothing falls back from a sign-in to a key (ruling 82, point 1).
     No item of this package binds `account`, so it reaches neither the wrapper nor the client.
     ```

   - After that paragraph, add a second paragraph, exactly, each sentence on a line of its own:
     in both READMEs its first line is

     ```text
     The Type states `toolsDirectory: {directory: /directory}` (CTR-014), so inspection reports the reach of the client's own tools wherever `directory` is a string; the engine reads `directory` for nothing else.
     ```

     in `agent-codex/README.md` its second line is

     ```text
     The Type's description names no client version (ruling 82, point 5); this README records the version read.
     ```

     and in `agent-claude-code/README.md` its second line is

     ```text
     The Type's description says that these tools run as the engine's operating-system user on the engine's own runner, and, under another runtime provider, on the machine of the instance's node with the limits that provider's contract states.
     ```

   - In `agent-codex/README.md` alone, in `## The .v2 Agent's own home`, replace "An API key is
     used only where a template sets `apiKey` explicitly, never as a fallback (ruling 82, points 1
     and 2).", which `BASE` wraps over three lines, with this sentence on a line of its own, and
     rewrap the paragraph's other lines as you like:

     ```text
     No template gives a `.v2` Agent an API key: the Type states no `apiKey` (ruling 83), and nothing falls back from a sign-in to a key (ruling 82, point 1).
     ```

     Change the quoted `.env` refusal as decision 9 says, since it quotes the wrapper.

   Write no other sentence that holds `since the Agent holds no API key (ruling 83)`: "Done when"
   counts it once in each file that holds it.

6. **Check the inputs with the engine**, as [order 050a](050a-agents-v2-own-tools.md)'s step 11
   does. This checks what the package tests cannot: MAN-017's binding of `/account`, that the Type
   holds no `/apiKey` to bind, CTR-014's checks on the Type, and that the shape registers.
   - In `engine/`, run `cargo build --release`. Change nothing there.
   - In each package, run `cargo run --release --bin package`, which prints `ARCHIVE#SHA256`.
   - In a new temporary directory T outside the repository, for NAME `agent-codex` (Types
     `codex.v2` and `codex.v1`) and then `agent-claude-code` (`claude-code.v2` and
     `claude-code.v1`), run `engine/target/release/conduit init --store T/store-NAME`, then write
     three projects under T, each a `conduit.yaml` that pins `example.org/conduit/NAME` at version
     0.2.0 with that digest, as `engine/crates/shell/tests/agents.rs` writes one (no `address`
     unless said), and one file per template, `kind: Agent`, `spec.type`
     `example.org/conduit/NAME#TYPE`:
     - A: the `.v2` template `v2-account` with `config: {account: example-account}`, and the `.v1`
       template `v1-template` with `accountServices: false` and `peerMessaging: false`.
     - P: `address: example.org/order-050f-probe` and an item, beside `packages`:

       ```yaml
       implementations:
         probe:
           implements: ['example.org/conduit/NAME#turn.v1']
           command: [agent-probe]
           environment:
             PROBE_ACCOUNT: /account
           plain: true
           tools:
             'example.org/conduit/NAME#turn.v1': turn
       ```

       and one `.v2` template, `probe-account`, with A's `config` and
       `implementation: {example.org/conduit/NAME#turn.v1: '#probe'}` in its `spec`.
     - N, the negative control: P with the line `PROBE_API_KEY: /apiKey` added under
       `environment`, and its template named `negative-account` in place of `probe-account`, so that
       it does not clash with P's template in the store (OBJ-002).
   - Register each in turn with
     `engine/target/release/conduit register --store T/store-NAME --package ARCHIVE#SHA256 PROJECT`.
     No project holds an instance, so none starts a wrapper, a client or the program
     `agent-probe`, which need not exist. Report each reply's `accepted` and its diagnostics.
   - Expected: A and P are accepted, each with no diagnostic, and so none citing MAN-017 or
     CTR-014. N is not accepted, with one error citing MAN-017 that names `negative-account`,
     `PROBE_API_KEY` and `/apiKey`, since the schema holds no `apiKey`. If A or P is not accepted,
     or gives a diagnostic, or N is accepted, change nothing to make it pass: report the reply,
     since this order changes no engine source, and the coordinator decides.

7. **The shape, checked against the fixture.** Decision 7's script in "Done when" reads each `.v2`
   Type from the package's `contracts.yaml` and from `BASE`'s, and the fixture's `.v2` schema. It
   prints, for each Type, whether every property the fixture's schema holds is equal in the
   package's; whether the package's schema holds `apiKey`; whether it holds a `oneOf`; whether
   `required` ends with `account`; whether `additionalProperties` is equal; whether `account` states
   a default; whether the schema, less `account`, is `BASE`'s; whether the `spec`, less `schema` and
   `toolsDirectory`, is `BASE`'s; the `toolsDirectory`; whether the description is `BASE`'s with
   decision 3's edits; whether it holds `0.167`; and whether it holds decision 25's sentence.

8. **What this order does not do.** No `signIn`, no hook, no `requires` entry and no login, which
   are orders 050h to 050m's; no `apiKey`, no `oneOf` and no item that binds `/account`; no
   `get-secret` Conduit and no secret read; no description sentence beyond (i) and (ii); no change
   to `.v1`, to `turn.v1` or to `conversation.v1`; and none under `engine/`, `conformance/` (whose
   fixture order 202 realigned), `spec/` or `docs/`. The loader vector of item 1's done-when, and the
   move of `conformance/fixtures/agents/` to 0.2.0, are order 050n's, which owns `conformance/`. The
   engine's test `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
   (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
   `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records. It
   is the one expected failure on `wo/050-agents-0.2.0`, and it is not this order's: neither you nor
   the verifier counts it against this order. Do not run the engine's tests and do not change it.

9. **The `.env` refusal**, which ruling 83's What follows names this order to carry. In each
   package's `src/lib.rs`, the refusal of a home that holds a `.env` ends with
   `since an API key is used only where a template sets apiKey`; it ends instead with
   `since the Agent holds no API key (ruling 83)`. Each package's `tests/wrapper.rs` constant
   `DOTENV_REFUSAL` changes the same way, and the Codex README's quote of it, in
   `## The .v2 Agent's own home`, follows. Keep each text on the one line it holds at `BASE`.
   Nothing else in `src/` or `tests/` changes.

## Who decided what

The inputs are the owner's: no `apiKey` in `.v2`, which makes the packages 0.2.0 subscription-only,
and `account` required with no default, a string or null, with no `oneOf`
([ruling 83](../proposals/ruling-83/README.md), which reverses ruling 82's point 2); no fallback
from a sign-in to a key (ruling 82, point 1); and the description sentences (i) and (ii) (ruling 82,
point 5, answer 7). `toolsDirectory` is the review's item 1, which the owner accepted with ruling
82, and order 157's plan for these Types. The split of order 050 into 050b to 050o, this order's
place on `wo/050-agents-0.2.0` after 050e, 048a-1 and order 202's work, and its base are the
coordinator's, open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)).
Everything else here is the coordinator's too, each open to the owner's strike: the YAML's order and
form; no item binding `account` in this order; the `.env` refusal's new words, which follow ruling
83's example; the README's wording; and the registration probe with its negative control. Apply them
as written and report them under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-codex/` and `packages/agent-claude-code/`: `contracts.yaml` and
`README.md`; and `src/lib.rs` and `tests/wrapper.rs`, for decision 9's words alone. Nothing else, in
the packages or outside them. The build's ignored output (`target/`, `dist/`) may change, and so may
anything under T.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each prints what is said beside it:

```bash
git status --short | grep -vE '^ M packages/agent-(codex|claude-code)/(contracts\.yaml|README\.md|src/lib\.rs|tests/wrapper\.rs)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml packages/agent-codex/Cargo.toml packages/agent-claude-code/Cargo.toml packages/agent-codex/Cargo.lock packages/agent-claude-code/Cargo.lock; echo $?   # 0
for NAME in agent-codex agent-claude-code; do head -c "$(wc -c < conformance/fixtures/agents/$NAME/contracts.yaml)" packages/$NAME/contracts.yaml | cmp - conformance/fixtures/agents/$NAME/contracts.yaml; echo "$NAME $?"; done   # agent-codex 0, agent-claude-code 0
grep -c '0\.167' packages/agent-codex/contracts.yaml   # 0
grep -c '^  toolsDirectory: {directory: /directory}$' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml   # 1 each
grep -cE '/(account|apiKey)$' packages/agent-codex/conduit.yaml packages/agent-claude-code/conduit.yaml   # 0 each
grep -c 'apiKey' packages/agent-codex/contracts.yaml packages/agent-claude-code/contracts.yaml   # 0 each
grep -cE '^\| `account` \| none; a string or null \| none \|$' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cE '^\| `apiKey` \|' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 0 each
grep -c 'ruling 83' packages/agent-codex/README.md packages/agent-claude-code/README.md   # at least 1 each
grep -c 'sets `apiKey` explicitly' packages/agent-codex/README.md   # 0
grep -c 'toolsDirectory' packages/agent-codex/README.md packages/agent-claude-code/README.md   # at least 1 each
grep -c 'authors may omit them' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 0 each
grep -c 'Authors may omit every one but `account`, which has no default.' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF '`account` names the subscription account that the client is to be signed in to, and has no default, so every template states it; a null `account` signs in to no subscription.' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF "The package's 0.2.0 is subscription-only: the Type states no \`apiKey\` and takes no API key, and API access comes later as a declared route (ruling 83)." packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF 'Nothing falls back from a sign-in to a key (ruling 82, point 1).' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF 'No item of this package binds `account`, so it reaches neither the wrapper nor the client.' packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF "The Type states \`toolsDirectory: {directory: /directory}\` (CTR-014), so inspection reports the reach of the client's own tools wherever \`directory\` is a string; the engine reads \`directory\` for nothing else." packages/agent-codex/README.md packages/agent-claude-code/README.md   # 1 each
grep -cxF "The Type's description names no client version (ruling 82, point 5); this README records the version read." packages/agent-codex/README.md   # 1
grep -cxF 'No template gives a `.v2` Agent an API key: the Type states no `apiKey` (ruling 83), and nothing falls back from a sign-in to a key (ruling 82, point 1).' packages/agent-codex/README.md   # 1
grep -cxF "The Type's description says that these tools run as the engine's operating-system user on the engine's own runner, and, under another runtime provider, on the machine of the instance's node with the limits that provider's contract states." packages/agent-claude-code/README.md   # 1
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs   # 15 each
for f in packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs; do git show "$BASE:$f" | sed 's/since an API key is used only where a template sets apiKey/since the Agent holds no API key (ruling 83)/' | cmp -s - "$f"; echo "$f $?"; done   # 0 each
grep -c 'template sets apiKey' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs packages/agent-codex/README.md   # 0 each
grep -c 'since the Agent holds no API key (ruling 83)' packages/agent-codex/src/lib.rs packages/agent-claude-code/src/lib.rs packages/agent-codex/tests/wrapper.rs packages/agent-claude-code/tests/wrapper.rs packages/agent-codex/README.md   # 1 each
grep -rnE '/(Users|home)/|/(private/)?tmp/' packages/agent-codex packages/agent-claude-code --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The third line is 050a's byte check: the 0.1.1 contracts stay a byte prefix of each package's
`contracts.yaml`.

Decision 7's script, run from the repository root with a Python that has PyYAML, as
`tools/check_schemas.py` needs (AGENTS.md, "Checks"), must print exactly the two lines below it:

```bash
python3 - "$BASE" <<'EOF'
import json, subprocess, sys, yaml
base = sys.argv[1]
fixture = {t["metadata"]["name"]: t["spec"]["schema"]
           for t in json.load(open("conformance/fixtures/s1/packages/agents-0.2.0/contracts.json"))
           if t["metadata"]["name"].endswith(".v2")}
runtime = ("Under another runtime provider, these tools run on the machine of the instance's node, "
           "as the operating-system user and with the limits that its contract states.")
edits = {"codex.v2": [("although Codex 0.167.0 still offers", "although Codex still offers")],
         "claude-code.v2": [("prompts off. They run as", "prompts off. On the engine's own runner, they run as"),
                            ("records it (ruling 70). The client's", "records it (ruling 70). " + runtime + " The client's")]}
for package, name in [("agent-codex", "codex.v2"), ("agent-claude-code", "claude-code.v2")]:
    path = f"packages/{package}/contracts.yaml"
    pick = lambda text: [d for d in yaml.safe_load_all(text) if d["metadata"]["name"] == name][0]
    now = pick(open(path).read())
    was = pick(subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout)
    s, f = now["spec"]["schema"], fixture[name]
    described = was["metadata"]["description"]
    for old, new in edits[name]:
        assert described.count(old) == 1, (name, old)
        described = described.replace(old, new)
    rest = dict(s)
    rest["properties"] = {k: v for k, v in s["properties"].items() if k != "account"}
    rest["required"] = [k for k in s["required"] if k != "account"]
    print(name,
          all(s["properties"].get(k) == v for k, v in f["properties"].items()),
          "apiKey" in s["properties"],
          "oneOf" in s,
          s["required"][-1:] == ["account"],
          s["additionalProperties"] == f["additionalProperties"],
          "default" in s["properties"].get("account", {}),
          rest == was["spec"]["schema"],
          {k: v for k, v in now["spec"].items() if k not in ("schema", "toolsDirectory")}
          == {k: v for k, v in was["spec"].items() if k != "schema"},
          now["spec"].get("toolsDirectory"),
          now["metadata"]["description"] == described,
          "0.167" in now["metadata"]["description"],
          runtime in now["metadata"]["description"])
EOF
```

```text
codex.v2 True False False True True False True True {'directory': '/directory'} True False True
claude-code.v2 True False False True True False True True {'directory': '/directory'} True False True
```

In each of `packages/agent-codex/` and `packages/agent-claude-code/`, one command at a time:

```bash
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 15 passed and 0 failed, the dotenv test
with decision 9's `DOTENV_REFUSAL`, and every other
test target as at `BASE` (the library's unit tests 2 and the package test 1 in each, and in
`agent-codex` the manifest test 1); the package test reads the new `contracts.yaml` from the
archive. Report the totals, the digest that `package` prints (it is a host build's), and decision
6's replies, each `accepted` and each diagnostic's `ruleId` and text. Say on which system the
tests ran. Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the
admin reruns them. Where the sandbox denies PID-scoped `ps`,
`v2_client_tree_stop_leaves_no_survivor` stops at its preflight; say so, report 14 passed and that
one failed for that reason, and the reader reruns it on the Mac.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on `wo/050-agents-0.2.0`: it does not merge alone, and its
Record is written at the foot of this file on that branch, as orders 050c's to 050e's are. A fresh
Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](../proposals/ruling-79/README.md), point 2). The local gate that
[ruling 80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order
050n's merge, read with the expected engine failure known.
