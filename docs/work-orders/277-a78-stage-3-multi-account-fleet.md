# Work order 277: A78's stage 3: the multi-account Fleet, registered before its first turn

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match `004-i2-01-vectors-and-first-start.schema.json`, the schema the
orders share; leave `vectors` empty. No network, no cargo; do not run `conduit`, `codex` or
`claude`, touch no store, and sign in nowhere. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, token
or credential. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: no absolute local path, user or machine name, or account detail; accounts go by label:
`primary-chatgpt`, `second-chatgpt`, `primary-claude`, `second-claude`.

## What and why

**The owner's decisions** set stage 3: DEC-046 (every account runs Agents, the coordinator
alone on `primary-claude`; worker pairs "two each to start"; one shared Codex login per account
per machine; a limit pauses only its account; growth a pair at a time, by measurement), DEC-047
(usage in records; role templates), DEC-042 to DEC-045 (Pool-owned logins), DEC-050 (both
reads on every order, blind, at once) and DEC-053 (an Opus drafter on `second-claude`). The
packages 0.2.4 give the login Resources, role inputs, usage per turn (IFC-051), a limit's
`resetsAt` and `new-conversation.v1` (order 253's `conversation` template).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests)): 3, one login per account,
linked, never copied; 2, root's explicit Conduits; 6, an explicit `new-conversation.v1`. None
at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: registering before results is existing practice (registered reports; ruling 45), on the
packages' own Types. Not taken: registering after the turns (gap 16); a script around the
engine.

**When it runs.** On `main` after orders 260 and 269 and DEC-050 to DEC-055: one Codex turn and
both reads outside the dogfood Fleet, since it changes 028's directory, then the gate; merged
before stage 3's first turn.

**Stop rule.** Stop and report where the check under "Done when" fails, a value needs a
private detail, a wording needs a choice no decision makes, or the start check prints
otherwise. List your own choices under `own_choices`.

## The decisions

Each YAML file has lines of at most 100 characters, no at sign and one final newline, and opens
with a comment naming stage 3 of A78, order 277 and the packages 0.2.4.

1. **The manifest**, `experiments/028-a78-dogfood/stage-3/fleet/conduit.yaml`: three comment
   lines, then lines 4 to 11 of stage 2's `conduit.yaml`.
2. **The templates**, `stage-3/fleet/templates.yaml`, in order: the Pool `login-pool`
   (`core#pool.v1`); the Conduit `logins-admin`, capability `core#pool-management.v1`,
   operations `inspect.v1`, `list.v1`, `set-up.v1`, `sign-in.v1`, `answer.v1`, `retire.v1`,
   `reconcile.v1`, written whole as in stage 2's `trees-admin`; the Resources
   `codex-account` (`agent-codex#codex-login.v1`) and `claude-account`
   (`agent-claude-code#account-home.v1`), with an input `account` to `/config/account`; the
   Agents `worker-role` and `verifier-role` (`codex.v4`), `reader-role` and `drafter-role`
   (`claude-code.v4`), with inputs `directory` and `login` to `/config/directory` and
   `/config/login`; each input's schema `{type: string, minLength: 1}`. Each Agent's `config`:
   `accountServices: false`, on the Claude two `peerMessaging: false`, `effort: high`;
   `turnDeadlineSeconds` 14400 on `worker-role`, else 3600; `model` `<codex-model>` on the
   Codex two, `opus` on the Claude two; `outputSchema` `<report-schema>` on `worker-role` and
   `drafter-role`, else `<verify-schema>`; and `instructions`, a folded block (`>-`), its lines
   indented alike, loading as the role's text, a space, then "Never print an environment
   variable's value, and never read or print a credential. End the turn with your report alone:
   one JSON object in your output schema."
   - `worker-role`: "You are a Codex worker of the conduit-core Fleet. Each turn's message names
     one work order of the repository in your directory, or lists the fixes to make. Carry it
     out as AGENTS.md's section "If you were given a work order" says, in your directory alone.
     Do not commit, push or switch branches."
   - `verifier-role`: "You are a Codex verifier of the conduit-core Fleet. Your standing order is
     docs/work-orders/verify.md in your directory; each turn's message names the order and its
     base commit. Read blind: you are given no other read, and the Claude reader reads the same
     change at the same time (DEC-050). Change nothing."
   - `reader-role`: "You are the Claude reader of the conduit-core Fleet: the fresh read before
     each merge. Your standing order is docs/work-orders/verify.md in your directory; each
     turn's message names the order and the branch to read. Read blind: you are given no other
     read, and the order's Codex verifier reads at the same time (DEC-050). Change nothing."
   - `drafter-role`: "You are the drafter of the conduit-core Fleet (DEC-053). Each turn's
     message is the coordinator's brief for one work order: write its first draft as one file
     in your directory, in the form of the merged orders the brief names. The coordinator
     approves or rewrites it. Do not commit, push or switch branches."
3. **The instances**, `stage-3/fleet/instances.yaml`, in order: a comment that the Fleet
   `conduit-core` is stage 2's, kept on the store, and `owner: conduit-core` names it (none
   declared, OBJ-002); the Pool `logins` (`login-pool`); the Resources `primary-chatgpt`,
   `second-chatgpt` (`codex-account`) and `second-claude` (`claude-account`), `owner: logins`,
   `account: <NAME-account>`; the Agents `worker-1`, `verifier-1`, `worker-2` and on to
   `verifier-4` (`worker-role`, `verifier-role`), `directory: <slot-N>`, `login` `primary-chatgpt`
   for pairs 1 and 2, `second-chatgpt` for 3 and 4, then `reader-a` (`reader-role`, `<slot-r>`)
   and `drafter` (`drafter-role`, `<slot-d>`) on `second-claude`, each `owner: conduit-core`;
   the Conduit `root-logins` (`logins-admin`) from the Interface `root` to the Pool; then per
   Agent, in order, `root-NAME` from its package's `conversation` template, from `root` to the
   Agent.
4. **The registration.** After the README's last line, a blank line and
   `## Stage 3: registered before its first turn`, lines at most 100 characters but one with a
   link, each link on one line. It opens "Registered on DATE by work order 277, before stage 3's
   first turn (ruling 45).", DATE as YYYY-MM-DD, and says: it holds for every order root sends
   through stage 3's Fleet, turns at once; it changes only by a dated amendment merged first; it
   applies DEC-042 to DEC-047 and DEC-050 to DEC-055, linked; its choices are the coordinator's,
   open to the owner's strike (ruling 48); the amendment of 2026-11-20's method holds
   (`conduit call` of root's tools, records as evidence, each manual step a gap), not its
   struck reading that `reader-1` waits for a guard; no order changing this directory goes
   through Conduit. Then, in order, saying at least:
   1. `### Stage 3's documents`: the files, linked, are templates, rendered by hand (a gap), the
      copy read back against its values, no account value printed. A table
      `| Agent | Type | Account | Directory | Deadline | Root's Conduit |`, names in backticks; a
      placeholder table: versions (0.2.4), digests, accounts as each client names them, the six
      working copies, `<codex-model>` as `codex` names it, never empty, and `<report-schema>`
      and `<verify-schema>`, each one quoted string, the JSON of the orders' shared schema and
      `verify.schema.json` at the engine's commit, by SHA-256. No account home for
      `primary-claude` (DEC-046). Stage 2's Fleet `conduit-core` owns the Agents; its Pool
      `trees`, `root-trees` and templates stay unused; a new store takes its Fleet first.
   2. `### Before the sitting, and at it`, each step recorded by a dated amendment before turn
      one, documents in parts. Before, the coordinator, the pairs working on: builds the
      dogfood engine from `main` with orders 258, 260, 262 and 269, served with a call timeout
      above 14400 s; installs the 0.2.4 archives into stage 2's store, with versions, digests
      and commits; submits the account-free documents and `primary-chatgpt`'s Resource, set up;
      has the owner type the second accounts, shown and confirmed. At the owner's signal:
      retires the four Codex Agents one at a time, each hook logging out only its own login,
      the everyday Codex login file's stat (never opened) unchanged after each, or stops and
      tells the owner (the everyday Claude login and a vendor's revocation are not shown);
      lifts their tombstones and those of root's `turn-worker-1` to `turn-verifier-2`, which
      match by consumer, target and capability too (LIF-041); has `primary-chatgpt` signed in,
      then per second account its notice, its Resource submitted and set up, its sign-in: each
      the owner's, one at a time (Codex's fixed ports), by `sign-in.v1` through the Pool. Then
      the signed-in accounts' Agents and root's Conduits are submitted and set up with no
      sign-in; a restart shows every setup passing and none pending; `reader-1` is retired
      last, once `reader-a` has read.
   3. `### Stage 3's claim`: what orders 2 and 3 claim, with turns at once across pairs and
      accounts on one store, and conditions 8 to 12: a login's one sign-in carries its Agents in
      parallel, a lapse taking one model call at a time until root answers (PND-005); usage in
      records (IFC-051); the pause by account; both reads blind and at once. Nothing about
      access (D57), Agents kept apart (D60), speed (D54) or the drafts.
   4. `### Stage 3's measures`: per turn, from the records: Agent, account, pair, span, outcome,
      `usage`, turns in flight at dispatch, in all and on its login; from root's kept call
      result, matched by invocation ID as in stage 2's results (beyond the amendment's
      records-only rule): failure class, `failedTurn.limit` and why no `usage`; per order: both
      reads' spans, overlap and waits, fix turns, tokens by role; per account: each sign-in
      action and notice, with its call and the time to root's answer, and each usage-limit
      failure, with `resetsAt` and the turns held; per stage: the most turns at once, in all,
      per account and per login, and tokens per account per day; with no pass condition, each
      of the drafter's first five drafts: its order, span, tokens and what the coordinator
      kept, changed or rewrote (DEC-053). Results name accounts by label, read records by named
      fields, and are per stage, as stage 2's, until the owner answers its results' item 3.
   5. `### Stage 3's pass conditions`, all needed, one not shown failing: stage 2's 1 to 7 as
      stage 2 reads them, 4 set aside, recorded as not shown (DEC-055); 8, no
      sign-in action on a Resource but a lapse report, after which no two model calls ran on
      that login until root answered; 9, each read but a repair follows a successful
      `new-conversation.v1` or a restart; 10, each succeeded turn has `usage` with model and
      version, none `usageDropped`, an omission named by its package rule; 11, no turn on an
      account between its usage-limit failure and `resetsAt` (if null, the coordinator's
      recorded read that it reopened), every turn on its pair's account; 12, both reads on one
      checkpoint, overlapping, neither's message holding the other's report. The claims hold
      when the first five orders pass and, over them, turns ran at once on both ChatGPT
      accounts and two pairs at once on one login.
   6. `### Pair 4, by measurement`: the owner's DEC-054, since the
      Mac met S2 timeouts and a full disk at two pairs and the gate (2026-11-22): `worker-4` and
      `verifier-4` are set up but sent no turn until a dated amendment shows, over five
      consecutive orders with pairs 1 to 3, no `usage-limit` failure on `second-chatgpt`, no
      turn lost to the machine (a timeout, a full disk, a killed process), and each gate run
      PASS or failing only on the order's change. For the same reason one reader, `reader-a`,
      reads for every pair. More takes new documents and an amendment.
   7. `### Stage 3's stop rules`: stage 2's 1, 2 (for a Resource's account check), 4 and 5; 3
      becomes: a sign-in action that is no lapse report, or a second lapse on one Resource in a
      day, stops that account until the owner signs it in again; a turn on a paused account,
      or work moved between accounts, stops stage 3, the owner's lift. A usage limit pauses; a
      read it holds waits, and nothing merges unread. No API key is used.
   8. `### What ends stage 3`: the owner, an upgrade of the store's Agent packages, a new
      initialization or a change of the account map, unless a dated amendment merged before
      says otherwise; pair 4's start does not.
   9. `### What stage 3 cannot show`: What it cannot show holds; any Agent's tools reach every
      working copy and login and can log a shared login out for all (ruling 70); tokens are the
      client's report, not the charge or the week left, and leave out the coordinator, S3, S7
      and S10; a refresh failure failing no turn is unseen; blind is only what each message
      carries; one Mac shows no other machine.
5. **The pointers.** In the README's first paragraph, after the sentence linking stage 2's
   registration, add "Stage 3, the multi-account Fleet on the Agent packages 0.2.4, is
   registered below.", its last two words a link to this section. In the Files table, after
   stage 2's rows, a row per stage-3 file, in their form.

## Before you change anything

From the repository root; stop and report if any line prints otherwise:

```sh
P=packages/agent
test -e experiments/028-a78-dogfood/stage-3; echo "stage-3 $?"
cat $P-codex/contracts.yaml $P-claude-code/contracts.yaml | grep -c -E '^  name: (codex|claude-code)\.v4$'
cat $P-codex/conduit.yaml $P-claude-code/conduit.yaml | grep -c '^version: 0\.2\.4$'
grep -c -E '^## DEC-05[03], ' docs/decisions/log.md
git status --short | wc -l | tr -d ' '
```

They print `stage-3 1`, `2`, `2`, `2` and `0`.

## Scope

Create the three files of decisions 1 to 3 and change only the README, by decisions 4 and 5;
not `stage-2/` or `inputs/`. Report anything else that needs changing.

## Done when

```sh
F=experiments/028-a78-dogfood/stage-3/fleet; R=experiments/028-a78-dogfood/README.md
S='/^## Stage 3: registered before its first turn$/,0'
[ "$(tail -n +4 ${F%3/fleet}2/fleet/conduit.yaml)" = "$(tail -n +4 $F/conduit.yaml 2>&1)" ]; echo "manifest $?"  # manifest 0; today 1
awk "$S" $R | grep -c '^### '  # 9; today 0
awk "$S" $R | grep -c -E '^\| `(worker-[1-4]|verifier-[1-4]|reader-a|drafter)` \|'  # 10; today 0
tr -s ' \n' '  ' < $R | grep -c "Registered on [0-9-]* by work order 277, before stage 3's first turn"  # 1; today 0
grep -c -e '^| \[stage-3/fleet/' -e '\[registered below\](#stage-3-registered-before-its-first-turn)\.' $R  # 4; today 0
```

Then this check; today it fails at once.

```sh
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 - <<'PY'
import hashlib, re, sys, tempfile, yaml, jsonschema as js
from pathlib import Path
sys.path.insert(0, "tools")
import check_schemas as c
D = Path("experiments/028-a78-dogfood/stage-3/fleet")
L = lambda p: [x for x in yaml.safe_load_all(Path(p).read_text()) if x]
T = {f"{p}#{x['metadata']['name']}": x["spec"]["schema"] for p in ("agent-codex",
     "agent-claude-code") for x in L(f"packages/{p}/contracts.yaml") if x["kind"] == "Type"}
t, i = {x["metadata"]["name"]: x["spec"] for x in L(D / "templates.yaml")}, L(D / "instances.yaml")
for x in (x for x in i if x["kind"] in ("Agent", "Resource")):
    tp, n = t[x["from"]], x["metadata"]["name"]
    pk, ty = tp["type"].split("/")[-1].split("#")
    s = dict(T[pk + "#" + ty])
    s["properties"] = P = {k: {"type": "string"} if "$ref" in v else v
                           for k, v in s["properties"].items()}
    cfg = {k: v.get("default") for k, v in P.items()} | tp.get("config", {}) | {
        tp["inputs"][k]["to"][0][8:]: v for k, v in x["arguments"].items()}
    js.Draft202012Validator(s).validate(cfg)
    via = [(y["metadata"]["name"], y["from"]) for y in i if y["kind"] == "Conduit"
           and y["target"]["name"] == n and y["consumer"] == {"kind": "Interface", "name": "root"}]
    want = [("root-" + n, f"example.org/conduit/{pk}#conversation")]
    assert via == ([] if x["kind"] == "Resource" else want), n
    assert (x["owner"], cfg.get("effort", "high"), cfg.get("accountServices"),
            cfg.get("peerMessaging")) in (("logins", "high", None, None),
            ("conduit-core", "high", False, None), ("conduit-core", "high", False, False)), n
    h = cfg.get("instructions")
    print(n, ty, cfg.get("login", cfg.get("account")), *(cfg.get(k, "-") for k in (
          "directory", "turnDeadlineSeconds", "model", "outputSchema")),
          hashlib.sha256(h.encode()).hexdigest()[:8] if h else "-")
v = {"codex-version": "0.2.4", "claude-version": "0.2.4", "codex-digest": "0" * 64,
     "claude-digest": "1" * 64}
S = {d["metadata"]["name"]: d["spec"] for g in ("schemas", "core")
     for p in c.files_of(c.ROOT / "spec" / g) for d in c.load(p)
     if isinstance(d, dict) and d.get("kind") == "Schema"}
b, n = c.bundle(S, c.names()["corePackage"]), 0
with tempfile.TemporaryDirectory() as tmp:
    for f in ("conduit.yaml", "templates.yaml", "instances.yaml"):
        p = Path(tmp) / f
        p.write_text(re.sub(r"<([a-z0-9-]+)>", lambda m: v.get(m[1], "x"), (D / f).read_text()))
        for d in c.load(p):
            e = list(c.validator(b, "manifest.v1" if f[0] == "c" else "document.v1").iter_errors(d))
            if e:
                print(f, d.get("metadata", {}).get("name"), c.explain(e[0]))
            n += not e
print("documents valid", n, *(y["metadata"]["name"] + ":" + y["from"] for y in i
                              if y["kind"] == "Conduit" and y["target"]["kind"] == "Pool"))
PY
```

It prints exactly:

```text
primary-chatgpt codex-login.v1 <primary-chatgpt-account> - - - - -
second-chatgpt codex-login.v1 <second-chatgpt-account> - - - - -
second-claude account-home.v1 <second-claude-account> - - - - -
worker-1 codex.v4 primary-chatgpt <slot-1> 14400 <codex-model> <report-schema> 7354c028
verifier-1 codex.v4 primary-chatgpt <slot-1> 3600 <codex-model> <verify-schema> 6b247b10
worker-2 codex.v4 primary-chatgpt <slot-2> 14400 <codex-model> <report-schema> 7354c028
verifier-2 codex.v4 primary-chatgpt <slot-2> 3600 <codex-model> <verify-schema> 6b247b10
worker-3 codex.v4 second-chatgpt <slot-3> 14400 <codex-model> <report-schema> 7354c028
verifier-3 codex.v4 second-chatgpt <slot-3> 3600 <codex-model> <verify-schema> 6b247b10
worker-4 codex.v4 second-chatgpt <slot-4> 14400 <codex-model> <report-schema> 7354c028
verifier-4 codex.v4 second-chatgpt <slot-4> 3600 <codex-model> <verify-schema> 6b247b10
reader-a claude-code.v4 second-claude <slot-r> 3600 opus <verify-schema> d0922f38
drafter claude-code.v4 second-claude <slot-d> 3600 opus <report-schema> 1e325135
documents valid 34 root-logins:logins-admin
```

Guards, passing today and after:

```sh
git add -N experiments/028-a78-dogfood/stage-3
cat $F/*.yaml | grep -c "$(printf '\100')"; cat $F/*.yaml | awk 'length > 100' | wc -l  # 0, 0
awk "$S" $R | grep -v '](' | awk 'length > 100' | wc -l  # 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | grep -cvE ' experiments/028-a78-dogfood/(README\.md|stage-3/fleet/(conduit|templates|instances)\.yaml)$'  # 0
git reset -q -- experiments/028-a78-dogfood/stage-3  # stage nothing
```

Report per decision what you wrote, and each check's output.

## What follows

The setup amendment; stage 3's first result.

## Record

Run on 2026-11-22 in Pacific time, outside the dogfood Fleet as this text says, since it changes
experiment 028's directory. Ruling 79 lets Opus implement, so an Opus worker carried it out in
place of the Codex turn this text names, and two Opus reads, one as a verifier and one as a
reader, read it blind and at once, as DEC-050 has every order read.

**The build** wrote `stage-3/fleet/{conduit,templates,instances}.yaml` and the README's stage 3
registration, its pointers and three Files rows. The start check, every Done-when line and the
document check printed what this text expects, ending `documents valid 34
root-logins:logins-admin`, with the four instruction hashes unchanged.

**The reads.** The verifier passed it with a should and two nits; the reader found ten shoulds and
three nits, nearly all in wording this text gave. One fix pass applied them: where stage 3 begins
and what it judges; condition 8's window and the setup's own sign-in action; condition 9 as the
first turn of a new conversation; condition 10's usage rule taken from each package's own
omissions; condition 11 on dispatched turns; condition 12 from root's kept messages; the drafter's
report mapped to the shared schema; the reader's role text on its checkout; step 10 after
`reader-a`'s first fresh read; and the citation of DEC-046 for the coordinator's account. Its
readings open to the owner's strike: `<slot-r>` is `reader-1`'s clone, the two never reading at
once; stop rule 3's day is a calendar day in PST; no turn may carry `usageDropped`.

The owner's DEC-054 (pair 4 by measurement) and DEC-055 (condition 4 set aside) were written into
this text before the build. The gate's run on the squash is the coordinator's, at the merge.
