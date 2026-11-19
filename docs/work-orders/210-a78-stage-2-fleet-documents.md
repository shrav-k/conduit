# Work order 210: A78's stage 2: the multi-slot Fleet's documents, written before 0.2.0 merges

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order adds three YAML files and changes
one Markdown file. It has no network access and needs none. It needs no cargo command; if you run
one, run one at a time. Do not run `conduit`, `codex`, `claude`, the dogfood harness or anything
else against a store, and sign in nowhere. Never print a whole large file: read
`experiments/028-a78-dogfood/README.md` with `grep -n` and `sed -n`. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a key,
a token or any credential.

## What and why

**What it writes.** [A78](../../ROADMAP.md#experiments-and-probes) has Conduit run its own work
orders through Agents of the `conduit-core` Fleet ([ruling 70](../proposals/ruling-70/README.md);
[ruling 61](../proposals/ruling-61/README.md), B12). Stage 1, order 042, ran on one slot: a Codex
worker and a Codex verifier from order 076's branch, on the owner's own Codex home, with the
Fleet's documents in
[experiment 028's inputs](../../experiments/028-a78-dogfood/inputs/fleet/templates.yaml). The
next stage runs on the store that is initialized again once order 050n merges the Agent packages
0.2.0, in which each Agent keeps its client's login in a home of its own (order 050c) and names
its account (order 050f), and the owner signs each Agent in once through `conduit sign-in`
([order 051b-2](051b2-cli-sign-in.md); [ruling 85](../proposals/ruling-85/README.md), point 5).
This order writes that stage's Fleet documents now, beside the sign-in chain, so that the store
can be made the day 050n merges: two pairs of a Codex worker and a Codex verifier, and one Claude
reader, the fresh read on Claude Opus that comes before each merge
([ruling 79](../proposals/ruling-79/README.md)). It adds a section to 028's README that says what
they are, what each placeholder stands for, and what must hold before stage 2's first turn.

**What it does not do.** It registers nothing: no order has run in stage 2, and its claim,
measures and pass conditions are registered by a later order, a dated amendment merged before the
first stage-2 turn ([ruling 45](../proposals/ruling-45/README.md)). It does not re-pin the
dogfood engine, change the harness, which stays outside the tree, change stage 1's inputs or its
registration, or name any account. The documents follow the inputs that order 050f gave the
`.v2` Types, as the coordinator's two lane branches of 0.2.0, not yet published, hold them
(`wo/050-lane-codex` at `34f4e662` and `wo/050-lane-claude` at `f1ede6a5`): `directory`,
`turnDeadlineSeconds`, `effort`, `accountServices`, `model` and `account` in both Types, and
`peerMessaging` in `claude-code.v2` alone; `account` has no default, a string or null. You do
not need those branches: the first check under "Done when" holds the two Types' inputs.

**Why now.** The coordinator's notes (private) owe the multi-slot Fleet's documents and the
parallel harness before 050n. The documents bind nothing and need nothing unmerged, so they can
merge now; the harness is the coordinator's, outside the tree. Every choice below is the
coordinator's, logged for the owner to strike ([ruling 48](../proposals/ruling-48/README.md)):

- two pairs, not three: the coordinator dispatches at most two Codex runs at once on the Mac, and
  each Agent costs the owner one sign-in; a third pair is four more documents and two sign-ins;
- one Claude reader, `reader-1`, working in a clone of its own into which the harness brings each
  order's branch, so that no Conduit and no directory of its names a worker's slot. 028 says of
  its Claude verifier slot that "its reach over the worker's tree waits for the owner's answer on
  what orders 4 and 5 test (D59)"; the coordinator proposes to read that reach as a Conduit or a
  directory naming the worker's tree, which `reader-1` does not have, and order 211's dated
  amendment records that reading, or the owner's answer, before `reader-1`'s first turn; its own
  tools, like every unconfined Agent's, still reach the worker's slot;
- `reader-1`'s model is `opus`, Claude Code's name for its current Opus model, with effort `high`
  and a one-hour deadline; the Codex workers keep stage 1's four hours, and the verifiers one hour,
  each with effort `high`;
- the deadlines are written as numbers, and only the archives' versions and digests, the clones'
  paths and the two accounts are placeholders, written in angle brackets, as the coordinator's
  placeholders in 028's README are;
- until order 203's `new-conversation.v1` is installed with the packages 0.2.1, a fresh
  conversation still restarts the engine and ends every Agent's conversation, so stage 2 sends
  one turn at a time whatever the number of slots; root's turn Conduits gain that operation by a
  later change, made with 0.2.1.

## What to read

- [experiments/028-a78-dogfood/README.md](../../experiments/028-a78-dogfood/README.md): its
  placeholder table, its Files table, "What it cannot show" and "The Claude verifier slot".
- [Stage 1's Fleet documents](../../experiments/028-a78-dogfood/inputs/fleet/instances.yaml),
  with [its templates](../../experiments/028-a78-dogfood/inputs/fleet/templates.yaml) and
  [its manifest](../../experiments/028-a78-dogfood/inputs/fleet/conduit.yaml): stage 2's keep their
  shape.
- [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed), its paragraph on
  Conduit building Conduit.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor c097ec81 HEAD; echo "main $?"
git ls-files --error-unmatch docs/work-orders/210-a78-stage-2-fleet-documents.md >/dev/null; echo "order $?"
test -e experiments/028-a78-dogfood/stage-2; echo "stage-2 $?"
git status --short | wc -l
R=experiments/028-a78-dogfood/README.md
grep -c '^## Stage 2' $R
grep -c -e '^## Files$' -e '^## What it cannot show$' -e '^### The Claude verifier slot$' -e '^### Not registered here$' $R
grep -c '^| \[inputs/plan.md\](inputs/plan.md) | Coordinator.s plan cited by ruling 74 |$' $R
tail -n 1 $R | grep -c 'it did not register\.$'
```

They print `main 0`, `order 0`, `stage-2 1`, `0`, `0`, `4`, `1` and `1`: the README's last
section is "Not registered here", whose last line ends "it did not register.". If another change
has moved that line, put decision 4's block after the README's last line all the same, and say
so.

## The decisions it applies

Each block below is written word for word, with DATE replaced by the date you write it, as
YYYY-MM-DD. Do not rewrap a block: decisions 1 to 4's lines are at most 100 characters; decision
5's rows stay whole on one line, as the table's other rows do; and each link sits whole on one
line, because `check_docs.py` checks a link only when its text and its target share a line.
Each YAML file ends with one newline after its last line.

1. **The manifest.** Create `experiments/028-a78-dogfood/stage-2/fleet/conduit.yaml` with exactly
   this text.

```yaml
# The conduit-core project's manifest for stage 2 of A78, written by work order 210 before the
# Agent packages 0.2.0 merge. The harness fills the <...> values into a copy outside the
# repository and submits that copy; this file is not submittable as it stands.
apiVersion: github.com/shrav-k/conduit/v0
packages:
  example.org/conduit/agent-codex:
    version: <codex-version>
    digest: sha256:<codex-digest>
  example.org/conduit/agent-claude-code:
    version: <claude-version>
    digest: sha256:<claude-digest>
```

2. **The templates.** Create `experiments/028-a78-dogfood/stage-2/fleet/templates.yaml` with
   exactly this text.

```yaml
# Templates of the conduit-core Fleet for stage 2 of A78 (ruling 70; ruling 61 B12), written by
# work order 210 against the .v2 inputs of the Agent packages 0.2.0 (order 050f). Each Agent
# names a directory, so its client's own tools are on (ruling 75 B1), and nothing confines them
# in v0 (ruling 70). Each names the owner's primary account of its client (ruling 84 point 2).
# The Pool for B12's tree slots; it stays empty in stage 2, as in stage 1.
apiVersion: github.com/shrav-k/conduit/v0
kind: Pool
metadata:
  name: tree-pool
spec:
  type: github.com/shrav-k/conduit/core#pool.v1
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: trees-admin
spec:
  capability: github.com/shrav-k/conduit/core#pool-management.v1
  operations:
  - github.com/shrav-k/conduit/core#register.v1
  - github.com/shrav-k/conduit/core#register-replacement.v1
  - github.com/shrav-k/conduit/core#inspect.v1
  - github.com/shrav-k/conduit/core#set-up.v1
  - github.com/shrav-k/conduit/core#retire.v1
  - github.com/shrav-k/conduit/core#reconcile.v1
---
# A Codex worker: four hours a turn. Account services stay off (ruling 48 point 1).
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: codex-worker
spec:
  type: example.org/conduit/agent-codex#codex.v2
  inputs:
    directory:
      schema:
        type: string
        minLength: 1
      to:
      - /config/directory
    account:
      schema:
        type: string
        minLength: 1
      to:
      - /config/account
  config:
    accountServices: false
    turnDeadlineSeconds: 14400
    effort: high
---
# A Codex verifier: the same Agent with one hour a turn. Nothing makes it read-only in v0; its
# prompt says so, and the harness checks the tree after each of its turns.
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: codex-verifier
spec:
  type: example.org/conduit/agent-codex#codex.v2
  inputs:
    directory:
      schema:
        type: string
        minLength: 1
      to:
      - /config/directory
    account:
      schema:
        type: string
        minLength: 1
      to:
      - /config/account
  config:
    accountServices: false
    turnDeadlineSeconds: 3600
    effort: high
---
# The Claude reader: the fresh read on Claude Opus before each merge (ruling 79), one hour a
# turn, in a clone of its own. Nothing makes it read-only in v0; the harness checks its clone.
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: claude-reader
spec:
  type: example.org/conduit/agent-claude-code#claude-code.v2
  inputs:
    directory:
      schema:
        type: string
        minLength: 1
      to:
      - /config/directory
    account:
      schema:
        type: string
        minLength: 1
      to:
      - /config/account
  config:
    accountServices: false
    peerMessaging: false
    turnDeadlineSeconds: 3600
    effort: high
    model: opus
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: codex-turn
spec:
  capability: example.org/conduit/agent-codex#conversation.v1
  operations: [example.org/conduit/agent-codex#turn.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: claude-turn
spec:
  capability: example.org/conduit/agent-claude-code#conversation.v1
  operations: [example.org/conduit/agent-claude-code#turn.v1]
```

3. **The instances.** Create `experiments/028-a78-dogfood/stage-2/fleet/instances.yaml` with
   exactly this text.

```yaml
# Instances of the conduit-core Fleet for stage 2 of A78, written by work order 210. The harness
# fills the <...> values. Root's default Conduit to the Fleet is made and opened at registration
# (FLEET-005); it gives no right to use an Agent, so root holds a turn Conduit to each.
apiVersion: github.com/shrav-k/conduit/v0
kind: Fleet
metadata:
  name: conduit-core
from: github.com/shrav-k/conduit/core#fleet
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Pool
metadata:
  name: trees
from: tree-pool
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: worker-1
from: codex-worker
owner: conduit-core
arguments:
  directory: <slot-1>
  account: <codex-account>
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: verifier-1
from: codex-verifier
owner: conduit-core
arguments:
  directory: <slot-1>
  account: <codex-account>
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: worker-2
from: codex-worker
owner: conduit-core
arguments:
  directory: <slot-2>
  account: <codex-account>
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: verifier-2
from: codex-verifier
owner: conduit-core
arguments:
  directory: <slot-2>
  account: <codex-account>
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Agent
metadata:
  name: reader-1
from: claude-reader
owner: conduit-core
arguments:
  directory: <slot-r>
  account: <claude-account>
---
# Root's explicit route to the Pool, as in stage 1.
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: root-trees
from: trees-admin
consumer:
  kind: Interface
  name: root
target:
  kind: Pool
  name: trees
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-worker-1
from: codex-turn
consumer:
  kind: Interface
  name: root
target:
  kind: Agent
  name: worker-1
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-verifier-1
from: codex-turn
consumer:
  kind: Interface
  name: root
target:
  kind: Agent
  name: verifier-1
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-worker-2
from: codex-turn
consumer:
  kind: Interface
  name: root
target:
  kind: Agent
  name: worker-2
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-verifier-2
from: codex-turn
consumer:
  kind: Interface
  name: root
target:
  kind: Agent
  name: verifier-2
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: turn-reader-1
from: claude-turn
consumer:
  kind: Interface
  name: root
target:
  kind: Agent
  name: reader-1
```

4. **The README's new last section.** At the end of `experiments/028-a78-dogfood/README.md`,
   after the section "Not registered here", add one blank line and then this block.

```markdown
## Stage 2: the multi-slot Fleet, written before 0.2.0

Written on DATE by work order 210, before order 050n merges the Agent packages 0.2.0. It
registers nothing: no order has run in stage 2, and the claim, the measures and the pass
conditions of stage 2's orders are registered by a dated amendment, merged before the first of
them runs ([ruling 45](../../docs/proposals/ruling-45/README.md)). Its choices are the
coordinator's, each open to the owner's strike
([ruling 48](../../docs/proposals/ruling-48/README.md)).

Stage 2 runs on the store that is initialized again once 0.2.0 merges. The `conduit-core` Fleet's
documents for it are [its manifest](stage-2/fleet/conduit.yaml),
[its templates](stage-2/fleet/templates.yaml) and [its instances](stage-2/fleet/instances.yaml).
They are templates, not rendered copies: the harness fills the placeholders below into a copy
outside the repository and submits that copy.

| Agent | Type | Directory | Turn deadline | Root's Conduit |
| --- | --- | --- | --- | --- |
| `worker-1` | `codex.v2` | `<slot-1>` | 14400 s | `turn-worker-1` |
| `verifier-1` | `codex.v2` | `<slot-1>` | 3600 s | `turn-verifier-1` |
| `worker-2` | `codex.v2` | `<slot-2>` | 14400 s | `turn-worker-2` |
| `verifier-2` | `codex.v2` | `<slot-2>` | 3600 s | `turn-verifier-2` |
| `reader-1` | `claude-code.v2` | `<slot-r>` | 3600 s | `turn-reader-1` |

- Every Agent has effort `high` and its account services off
  ([ruling 48](../../docs/proposals/ruling-48/README.md), point 1), and `reader-1` has the model
  `opus` and peer messaging off. Each names a directory, so its client's own tools are on and
  nothing confines them in v0 ([ruling 70](../../docs/proposals/ruling-70/README.md)).
- Each keeps its client's login in a home of its own inside its state directory
  ([order 050c](../../docs/work-orders/050c-agents-v2-client-home.md)), which keeps the accounts
  apart for the clients but is no wall against an Agent that runs code
  ([ruling 84](../../docs/proposals/ruling-84/README.md), point 2). Each names in `account` the
  owner's primary account of its client, and no Agent names a second account until the owner says
  so (ruling 84, point 3).
- Two pairs of a worker and a verifier, each pair in a slot clone of its own: the coordinator
  dispatches at most two Codex runs at once on the Mac, and each Agent costs the owner one
  sign-in. A third pair is four more documents and two more sign-ins.
- `reader-1` is [the Claude verifier slot](#the-claude-verifier-slot) of stage 1's registration:
  the fresh read on Claude Opus before each merge
  ([ruling 79](../../docs/proposals/ruling-79/README.md)). It works in a clone of its own, into
  which the harness brings each order's branch, so no Conduit and no directory of its names a
  worker's slot. The coordinator proposes to read the reach over the worker's tree that waits for
  D59 as a Conduit or a directory naming that tree, which `reader-1` does not have; order 211's
  dated amendment records it, or the owner's answer, before `reader-1`'s first turn. Its own
  tools, like every unconfined Agent's, still reach the worker's slot.
- The Pool `trees` stays empty, as in stage 1: no tree is granted through Conduit before orders 4
  and 5.

Placeholders, filled only in the rendered copy and never committed:

| Placeholder | Stands for | Class |
| --- | --- | --- |
| `<codex-version>`, `<codex-digest>` | The agent-codex archive's version and SHA-256 | other |
| `<claude-version>`, `<claude-digest>` | The same for the agent-claude-code archive | other |
| `<slot-1>`, `<slot-2>` | The absolute paths of the two slot clones | path |
| `<slot-r>` | The absolute path of the reader's clone | path |
| `<codex-account>` | The owner's primary ChatGPT account, as Codex names it | account |
| `<claude-account>` | The owner's primary Claude account, as Claude Code names it | account |

Before stage 2's first turn:

1. Order 050n has merged the Agent packages 0.2.0, and the store is initialized again with their
   archives; by the coordinator's notes (private), stage 1's store holds archives that also carry
   the version 0.2.0.
2. Before the store is initialized with these documents, and so before `reader-1` becomes a
   dogfood slot or is signed in, the harness runs nothing from a slot or from the reader's clone,
   and reads its own helpers only from its own directory and the main checkout (D58;
   [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed)). A22 no longer runs
   first ([ruling 84](../../docs/proposals/ruling-84/README.md), point 1).
3. The owner has signed each of the five Agents in once, on the Mac, through `conduit sign-in`
   ([order 051b-2](../../docs/work-orders/051b2-cli-sign-in.md);
   [ruling 85](../../docs/proposals/ruling-85/README.md), point 5), one Agent at a time: Codex's
   browser login listens on fixed ports, and a sign-in that may share one with another in
   progress reports unavailable
   ([order 048a-5c](../../docs/work-orders/048a5c-build-sign-in-fixed-port.md)).
4. Until [order 203](../../docs/work-orders/203-new-conversation.md)'s `new-conversation.v1` is
   installed with the packages 0.2.1, a fresh conversation still restarts the engine, and a
   restart ends every Agent's conversation. Until then the harness sends one turn at a time,
   whatever the number of slots. Root's turn Conduits gain that operation by a later change, made
   with 0.2.1, and the slots then run at once.

Every Agent of stage 2 runs under the owner's operating-system user too, so
[What it cannot show](#what-it-cannot-show) holds for every result of stage 2.
```

5. **The Files table.** In the README's table under "Files", after the row of
   `inputs/plan.md`, add these three rows, each on one line.

```markdown
| [stage-2/fleet/conduit.yaml](stage-2/fleet/conduit.yaml) | Stage 2's manifest template, written before 0.2.0 |
| [stage-2/fleet/instances.yaml](stage-2/fleet/instances.yaml) | Stage 2's Fleet, Agents, Pool and Conduit instances |
| [stage-2/fleet/templates.yaml](stage-2/fleet/templates.yaml) | Stage 2's Agent, Pool and Conduit templates |
```

6. **Nothing else.** Change no other line of the README, and no other file.

## Scope

Create the three files of decisions 1 to 3, and change only
`experiments/028-a78-dogfood/README.md`, as decisions 4 and 5 say. Change nothing under
`experiments/028-a78-dogfood/inputs/`, and do not change `ROADMAP.md`, `experiments/README.md`,
`IN_FORCE.md`, the handover, `tools/` or any order's file. Anything else you find that needs
changing goes in your report.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_docs.py             # Errors: 0
python3 tools/check_private_paths.py    # Errors: 0
git diff --check                        # prints nothing
git status --short                      # exactly: M experiments/028-a78-dogfood/README.md and ?? experiments/028-a78-dogfood/stage-2/
```

Then each of these prints what is beside it. The commands that start with `tr` read the file with
its line breaks as spaces.

```sh
F=experiments/028-a78-dogfood/stage-2/fleet
R=experiments/028-a78-dogfood/README.md
S='/^## Stage 2: the multi-slot Fleet, written before 0.2.0$/,0'
ls $F | tr '\n' ' '                                         # conduit.yaml instances.yaml templates.yaml
cat $F/*.yaml | wc -l                                        # 268
cat $F/*.yaml | grep -c "$(printf '\100')"                 # 0 (no at sign)
cat $F/*.yaml | awk 'length > 100' | wc -l                   # 0
grep -c '^## Stage 2: the multi-slot Fleet, written before 0.2.0$' $R   # 1
awk "$S" $R | awk 'length > 100' | wc -l                     # 0
awk "$S" $R | grep -c '^| `[a-z]*-[12r]` |'                  # 5
awk "$S" $R | grep -c '^[1-4]\. '                            # 4
tr -s ' \n' '  ' < $R | grep -c 'It registers nothing: no order has run in stage 2'   # 1
grep -c '^| \[stage-2/fleet/' $R                             # 3
awk "$S" $R | grep -c "$(printf '\100')"                  # 0 (no at sign in the section)
```

Then the two checks below, each run from the repository root. The first reads each Agent of the
instances through its template, fills the defaults of its Type's inputs as the lanes give them,
and validates its configuration against the Type's inputs.

```sh
/usr/bin/python3 - experiments/028-a78-dogfood/stage-2/fleet <<'PY'
import sys, copy, yaml, jsonschema
D = sys.argv[1]
def docs(f):
    return [x for x in yaml.safe_load_all(open(f"{D}/{f}")) if x]
codex = {"directory": {"type": ["string", "null"], "default": None},
         "turnDeadlineSeconds": {"type": "integer", "minimum": 1, "maximum": 604800,
                                 "default": 180},
         "effort": {"type": "string", "default": ""},
         "accountServices": {"type": "boolean", "default": False},
         "model": {"type": "string", "default": ""},
         "account": {"type": ["string", "null"]}}
claude = dict(codex, peerMessaging={"type": "boolean", "default": False})
types = {"example.org/conduit/agent-codex#codex.v2": codex,
         "example.org/conduit/agent-claude-code#claude-code.v2": claude}
m = yaml.safe_load(open(f"{D}/conduit.yaml"))
print("manifest", sorted(m["packages"]))
t = {x["metadata"]["name"]: x for x in docs("templates.yaml")}
i = docs("instances.yaml")
print("templates", " ".join(f"{x['kind']}:{n}" for n, x in t.items()))
print("instances", " ".join(f"{x['kind']}:{x['metadata']['name']}" for x in i))
agents = [x for x in i if x["kind"] == "Agent"]
ok = 0
for a in agents:
    tp = t[a["from"]]["spec"]
    props = types[tp["type"]]
    cfg = copy.deepcopy(tp.get("config", {}))
    for k, v in a.get("arguments", {}).items():
        jsonschema.validate(v, tp["inputs"][k]["schema"])
        for p in tp["inputs"][k]["to"]:
            assert p.startswith("/config/"), p
            cfg[p[len("/config/"):]] = v
    for k, s in props.items():
        if k not in cfg and "default" in s:
            cfg[k] = s["default"]
    jsonschema.Draft202012Validator({"type": "object", "properties": props,
                                     "required": sorted(props),
                                     "additionalProperties": False}).validate(cfg)
    ok += 1
    name = a["metadata"]["name"]
    turn = [c["metadata"]["name"] for c in i if c["kind"] == "Conduit"
            and c["target"] == {"kind": "Agent", "name": name}
            and c["consumer"] == {"kind": "Interface", "name": "root"}]
    print(name, tp["type"].split("#")[1], a["owner"], cfg["directory"], cfg["account"],
          cfg["turnDeadlineSeconds"], cfg["effort"], cfg["model"] or "-", turn)
print("agents", len(agents), "valid", ok)
PY
```

It prints:

```text
manifest ['example.org/conduit/agent-claude-code', 'example.org/conduit/agent-codex']
templates Pool:tree-pool Conduit:trees-admin Agent:codex-worker Agent:codex-verifier Agent:claude-reader Conduit:codex-turn Conduit:claude-turn
instances Fleet:conduit-core Pool:trees Agent:worker-1 Agent:verifier-1 Agent:worker-2 Agent:verifier-2 Agent:reader-1 Conduit:root-trees Conduit:turn-worker-1 Conduit:turn-verifier-1 Conduit:turn-worker-2 Conduit:turn-verifier-2 Conduit:turn-reader-1
worker-1 codex.v2 conduit-core <slot-1> <codex-account> 14400 high - ['turn-worker-1']
verifier-1 codex.v2 conduit-core <slot-1> <codex-account> 3600 high - ['turn-verifier-1']
worker-2 codex.v2 conduit-core <slot-2> <codex-account> 14400 high - ['turn-worker-2']
verifier-2 codex.v2 conduit-core <slot-2> <codex-account> 3600 high - ['turn-verifier-2']
reader-1 claude-code.v2 conduit-core <slot-r> <claude-account> 3600 high opus ['turn-reader-1']
agents 5 valid 5
```

The second fills each placeholder with a stand-in, reads each file strictly as
`tools/check_schemas.py` does, and validates the manifest against `manifest.v1` and every other
document against `document.v1` from `spec/schemas/`. It prints `documents valid 21`, and nothing
else.

```sh
/usr/bin/python3 - experiments/028-a78-dogfood/stage-2/fleet <<'PY'
import re, sys, tempfile
from pathlib import Path
sys.path.insert(0, "tools")
import check_schemas as c
D = Path(sys.argv[1])
stand = {"codex-version": "0.2.0", "claude-version": "0.2.0", "codex-digest": "0" * 64,
         "claude-digest": "1" * 64, "slot-1": "/s/slot-1", "slot-2": "/s/slot-2",
         "slot-r": "/s/slot-r", "codex-account": "a", "claude-account": "b"}
schemas = {}
for g in ("schemas", "core"):
    for p in c.files_of(c.ROOT / "spec" / g):
        for d in c.load(p):
            if isinstance(d, dict) and d.get("kind") == "Schema":
                schemas[d["metadata"]["name"]] = d["spec"]
b = c.bundle(schemas, c.names()["corePackage"])
n = 0
with tempfile.TemporaryDirectory() as t:
    for f, name in (("conduit.yaml", "manifest.v1"), ("templates.yaml", "document.v1"),
                    ("instances.yaml", "document.v1")):
        text = (D / f).read_text()
        left = set(re.findall(r"<([a-z0-9-]+)>", text)) - set(stand)
        if left:
            print(f, "unknown placeholders", sorted(left))
        p = Path(t) / f
        p.write_text(re.sub(r"<([a-z0-9-]+)>", lambda m: stand.get(m.group(1), m.group(0)), text))
        for d in c.load(p):
            errs = list(c.validator(b, name).iter_errors(d))
            if errs:
                print(f, d.get("metadata", {}).get("name", "-"), c.explain(errs[0]))
            n += not errs
print("documents valid", n)
PY
```

The coordinator renders the documents with real values, submits them to a store and records what
that shows only once order 050n has merged; this order checks what can be checked before. The
admin then has a fresh, read-only Opus reader read the diff before it merges (ruling 79). Do not
stage anything. Report each choice you make as your own, and every check's last line.
