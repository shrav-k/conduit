# Work order 154: the plan-and-apply reference corrected against spec/

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has no network access, and needs none.
The case files run to 50 KB, so never print a whole one: use `sed -n`, `grep` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
print a key, a token or a credential.

## What and why

The full review of 2026-11-14 ([its record](../proposals/full-review/README.md), order 8 of
[its orders](../proposals/full-review/orders.md)) found that the plan-and-apply reference,
[docs/implementation/plan-and-apply/](../implementation/plan-and-apply/README.md), no longer
matches `spec/` or its own cases ([FR-302](../proposals/full-review/findings/P03-plan-apply.md#fr-302)):

- `spec/README.md` requires a plan to print `ownTools` beside each Agent it registers. Neither the
  reference nor case 01, which registers two Agents, does this.
- A no-op plan is said to be two lines long. No plan from case 05 on can be that short.
- The value at a secret-reference place is kept out of the ledger, although a valid value there is
  only a reference.
- Four open points can be closed.
- A heading still points to an order (K3b) that wrote no safety cases.

Three of the eight edits an author can make are not stated whole. A renamed Conduit is undecided,
and a removal and a moved package pin are stated only in pieces
([FR-291](../proposals/full-review/findings/P03-plan-apply.md#fr-291) rows 4, 5 and 8, as its
verifier corrected them). Order 052a and two of the review's orders build on this text: order 23,
the planner's difference table, and order 26, which sends the cases' requests to an engine
(decision D26 of [its decisions](../proposals/full-review/decisions.md)). So the reference is
corrected first. It binds nothing, and no promise changes.

## The decisions it applies

1. **The `ownTools` lines.** Source: the P03 verifier's
   [missed point](../proposals/full-review/findings/P03-plan-apply.md#what-the-verifier-says-the-analyst-missed)
   and its correction to FR-302. Since order 075,
   [spec/README.md](../../spec/README.md#what-the-short-forms-meaning-covers) has required two
   lines:
   - beside each Agent a plan registers or replaces, the `ownTools` member that inspection would
     show;
   - beside each Conduit the plan opens whose target is an Agent and that lists an operation marked
     `startsModelWork: true`, the target's `ownTools` (CTR-014, LIF-013).

   Make these changes:
   - **Text grammar.** In the reference's "Text grammar and statuses", where it says what `detail`
     lines name, add both lines and their form:
     - `detail ownTools Agent/NAME JSON CTR-014` goes after the other `detail` lines of the step
       that registers or replaces the Agent. There is one line per Agent, in the order of that
       step's `objects=` list.
     - `detail targetOwnTools Agent/NAME JSON CTR-014` goes after the `operations=` line of such
       an open.

     JSON is the member as `inventory.v1` gives it, in sorted-key compact JSON, with one
     exception. A `directory` that is an absolute path is printed as `{"pathSha256":"HEX"}`, as
     the document lines print `root`, because plan text holds no path ("Kept state and privacy").
   - **Read, compare, print, keep, send.** In that section, say that the member is computed from
     the Type's contract and the resolved configuration. It therefore needs the package's
     contents, as any resolution does (O7). Inspection results and their verification do not
     change.
   - **Case 01.** Add the two lines below right after
     `detail defaults=[root->Pool/team as=team,root->Fleet/s1 as=s1] FLEET-005`, in both `printed`
     and `recorded.text`. Also add `CTR-014` to the case's `sources`. The fixture agents 0.2.0
     (`conformance/fixtures/s1/packages/agents-0.2.0/contracts.json`, the same files as its
     install archive) has Types `claude-code.v2` and `codex.v2`. Neither states a
     `toolsDirectory`, so the member is `stated: false` alone.

     ```text
     detail ownTools Agent/agent-a {"stated":false} CTR-014
     detail ownTools Agent/agent-b {"stated":false} CTR-014
     ```

     No other case registers or replaces an Agent or opens a Conduit to one, so their plans stay
     as they are.
   - **The digest.** Re-derive case 01's digest with Python's `hashlib.sha256` over the UTF-8
     bytes of `recorded.text`. Then use a script, not hand edits, to replace the old hex
     `6bdd6bfbd979f2acd9faad689cafc57de87bc35dd642a14b683616475881f6f0` wherever it occurs:
     - case 01's `recorded.path` and `recorded.digest`;
     - case 01's 15 request keys and its `kept`;
     - every later case's `applied` and `kept`.

     Nothing outside the nine cases holds the old hex.
   - **The counts table.** In "Case command and output counts", case 01's row becomes 37 plan
     lines and 52 stdout lines. The paragraph after the table says that cases 01, 02, 03, 05 and
     06 keep their plan digests. It now says that cases 02, 03, 05 and 06 keep them, and that:
     - ruling 76 changed the plan text of cases 04, 07, 08 and 09;
     - the `ownTools` lines changed the plan text of case 01;
     - each changed digest was re-derived with `hashlib.sha256`.

     S1's counts do not change, because the new lines are plan text, not authored lines.
2. **A plan with nothing to send.** Source: FR-302. In "Text grammar and statuses", replace
   "No-op text is exactly `plan\nno changes\n`" with the rule below, and keep the rest of that
   sentence:
   - A plan with no step prints `plan`, then each `difference` line, then `no changes`. It has no
     `affects` line and no `flags` line.
   - With no difference, as in case 03, the plan is exactly `plan\nno changes\n`.
   - Every plan from case 05 on prints the retained-pin `difference` line, so a no-op there has
     at least three lines.
   - A no-op still sends nothing and writes nothing.
3. **A secret reference is kept and compared as a reference.** Source: FR-302's O5, with its
   verifier's caution. At a place that `secret-reference.v1` governs, a valid value is a reference
   to a Resource, never the secret itself. Any other value there makes the document rejected
   (SEC-001).
   - **Check first.** The planner checks every such place locally, during translation and
     resolution, before it prints any step.
   - **A literal or a bare name.** The plan stops with status 2. It prints `plan`, then one line
     `stop KIND/NAME place=POINTER SEC-001` for each such place, then `stopped`. Each stop line
     names the document, and the place by its JSON Pointer. The plan has no step, no `detail`
     line and no `difference` line. No line holds any part of the value, as SEC-003 requires of a
     diagnostic. Nothing is kept or sent.
   - **A value that passes.** It is printed, kept in `.conduit/state.yaml` and compared like any
     other reference: as written, `{kind, name}`, with the ID it resolves to as inspection shows
     it (OBJ-009, SEC-004). Where the schema allows null, as for S1's `apiKey`, a null value is
     not a reference.
   - **Text to rewrite to match.**
     - In "Kept state and privacy", four sentences: "Keep no absolute path, store path,
       credential, or value at a place governed by a secret reference."; "Never fingerprint a
       credential or a secret-reference value."; "For a secret reference, omit the authored value
       entirely rather than hashing it."; and "O1/O5 record the relative-path and secret
       comparison limitations", which then names O1 alone.
     - O5 becomes decided and says what this decision says. Keep its last sentence, on public
       absolute paths.
     - Safety case 15 says the same.
     - Credentials, store paths and absolute paths are handled as they are now.
4. **O1, O3 and O4 are decided.** Source: FR-302. In "Open points", each point now begins by
   saying whether it is open or decided.
   - **O1, decided for v0.** A project with a relative `fetch`, or a program given as a relative
     path, cannot be applied and must be packaged. The plan stops with status 2, naming the
     document and the place. Give the reason: making these values absolute would change the
     digest that SUB-008 takes of the manifest as handed over. It would also lose PRV-007's guard
     that a relative program runs only on the engine's own runner (SUB-010).
   - **O3, answered by [ruling 59](../proposals/ruling-59/README.md) A3.** Under that ruling, a
     cascade takes a flag that opens, uses and closes a one-time Conduit, printed in the plan.
     That Conduit is the one stated exception to narrowing before widening and to the subset rule.
     - Say so in "Ordering and access".
     - In "Replacement, removal and cascades", replace the sentence that says the global
       access-subset rule cannot include the temporary permission and that O3 records the
       limitation.
   - **O4, decided.** CON-008 closes and removes the one-time Conduit when its target retires. So
     the plan prints that closure under `affects` and sends no close.
     - The cascade pattern stays as it is, and so does the cleanup after a cascade that does not
       succeed.
     - Two sentences call this "the nearest sound reading": one in the cascade section and one in
       O4. Both now say it is the reading, citing CON-008.
     - `spec/README.md` still lists "close the Conduit" among the cascade's requests. That
       sentence contradicts CON-008, and the next specification order corrects it (decision D86).
       O4 says so. This order does not change `spec/`.
   - **O5** is decided by decision 3.
   - **O2, O7 and O8 are marked open**, and their text stays as it is. Order 069's Record named
     only O1 to O4. It is not edited, since each point now says where it stands.
5. **The safety cases' heading.** Sources: FR-302, and
   [FR-299](../proposals/full-review/findings/P03-plan-apply.md#fr-299) stage 2. The heading
   "Safety cases deferred to K3b" becomes "Safety cases for 052a's convergence check".
   - Its first lines say three things. Order 069b was K3b and wrote none of these cases. The 18
     safety cases, a cascade case and cases for `--adopt`, `--recreate` and `--prune` have no case
     file and no order. They are to be fixed inputs of the convergence check that is order 052a's
     done-when (decision D26).
   - Two sentences promise a later order: "A later order supplies the remaining safety cases and
     a cascade case." in the opening, and "A later order adds cascade and the other flags'
     cases." in "Case format and sequence". Both now point to that section instead.
   - Rename no other heading. Other documents link to "Replacement, removal and cascades" and
     "Open points".
6. **Renaming.** Source: FR-291 row 4, as its verifier corrected it. The decision is the
   coordinator's, as that finding's "Waits" says. In "Replacement, removal and cascades", after
   the paragraph that begins "A tombstone made outside this apply", add a short paragraph that
   says:
   - **Which tombstones apply lifts with no further flag.** Apply lifts a tombstone this command
     caused, with no flag, only when the same command makes the same object again under the same
     kind and name. There are two such cases:
     - the object a `--replace` names, and the Conduits incident to it (cases 04, 05 and 06);
     - a retained Conduit that is closed and made again because its resolved operations narrow
       (ruling 76 point 1; case 04, steps 2 and 5).

     Every other tombstone that a registration or an open would match needs `--reopen` (a
     Conduit) or `--recreate` (another instance), naming the object to be made. That includes a
     tombstone made outside this apply, as now. It also includes one this apply leaves by closing
     a Conduit the file dropped or by `--prune`, among them the Conduits that a pruned instance's
     retirement closes under CON-008. Reopening still takes a flag
     ([ruling 76, "What it amends"](../proposals/ruling-76/README.md#what-it-amends)).
   - **A renamed Conduit.** A Conduit renamed in the file that keeps its consumer, target and
     capability is two things. One is a flag-free close of the old name. The other is a new
     Conduit whose registration matches the old one's tombstone (LIF-041).
     - The plan needs `--reopen NEW`, naming the new Conduit by its generated name.
     - With the flag, the steps are: close OLD, list, lift that tombstone, register NEW, inspect
       NEW, open NEW. The flag is printed on the lift, the register and the open, as case 08
       prints a reopening.
     - Without the flag, the plan stops before sending anything, with
       `stop Conduit/NEW tombstone=(Conduit/OLD) requires=--reopen NEW LIF-041`. A tombstone this
       apply would make is named by its object in brackets. Case 09 gives an existing tombstone's
       ID instead.
   - **Another renamed instance.** Any other instance renamed in the file is two things: the
     removal of OLD under `--prune OLD`, and the addition of NEW.
     - NEW matches no tombstone, since a tombstone matches another instance by kind and name.
     - OLD's retirement closes its Conduits (CON-008) and leaves their tombstones. A Conduit the
       file names that matches one of them needs `--reopen` naming that Conduit, as above. One
       example is a Conduit with the same consumer and local name that now targets NEW.
   - **Text to change to match.**
     - In "Approval and command readings under ruling 76", "`--reopen` lifts an outside-apply
       Conduit tombstone" becomes: `--reopen` lifts a Conduit tombstone that the first bullet
       leaves to a flag. `--recreate` then does that for another instance, as now.
     - The sentence "A tombstone made outside this apply needs reopen (Conduit) or recreate
       (other instance)..." now refers to that rule.
     - Safety case 2 adds "in the same apply or a later one".
     - Step 3 of the replacement sequence keeps its words. It is the first case of the first
       bullet.
7. **Removal, by kind.** Source: FR-291 row 5, as its verifier corrected it. In "Replacement,
   removal and cascades", add one short list that gives each kind of removal and cites what
   decides it:
   - **A Conduit the file drops.** It is closed with no flag (ruling 76 point 1). Its tombstone
     stays, so adding it back takes `--reopen` (cases 07, 09 and 08).
   - **Another instance.** It takes `--prune N`, then its retirement. Adding it back takes
     `--recreate N`.
     - The same `--prune N` also permits removing N's inline template `N--t` (SCT-006), as
       `--replace N` covers that template's update. Once N's retirement has removed N, the
       template is removed with `unregister.v1` at its revision, carrying `expectedId`.
   - **A template the file drops that is not an inline template.** It takes `--prune` naming it,
     then `unregister.v1` as above.
   - **A Pool or Fleet that owns members.** It takes its cascade flag, as the cascade section
     says.
   - **A retirement that does not succeed.** As the reference already says, it is recovered by a
     further explicit retirement under the original permission (LIF-026). Apply never sends
     `unregister.v1` to recover it. That request asserts a shutdown settled elsewhere (LIF-017),
     and the reference leaves it to the author.
8. **A moved package pin.** Source: FR-291 row 8, as its verifier corrected it. After "Print this
   retained-pin difference.", state the rule in general, not only for T5. After
   `conduit install`:
   - **Templates.** The plan replaces each project template that refers to the moved package, and
     no other. It does this with `replace.v1` at the template's revision and with the new manifest
     (SUB-009). Case 05 replaces `shared--t` alone. For the other templates, SUB-009 would store no
     change.
   - **Instances.** Each existing instance whose resolved meaning or execution would differ under
     the new release is replaced, under `--replace` naming it (INS-020).
   - **Everything else.** Every other object keeps the pins it holds (NAM-009 to NAM-011). While
     it holds an old pin, every plan prints its retained-pin `difference` line.
   - **Contents needed.** The comparison needs the contents of both releases (O7).

## Scope

**Files you may change:**

- `docs/implementation/plan-and-apply/README.md`;
- the nine files in `docs/implementation/plan-and-apply/cases/`.

**Within the cases, change only:**

- case 01's plan text, in `printed` and `recorded.text`;
- case 01's `sources`;
- the old case-01 digest, wherever it occurs.

No request argument, result, file or other plan changes.

**Do not change:**

- `spec/`, `engine/` or `conformance/`;
- `experiments/`: experiment 023's counts stand;
- any work order, including order 069's Record;
- any ruling record, `IN_FORCE.md` or the ledger.

**Leave the reference's text on unknown outcomes and reconciliation as it is.** Points 5 and 6 of
[ruling 78](../proposals/ruling-78/README.md) change those promises through a later specification
order, and the reference will follow `spec/` once that order is written.

Report anything else that needs changing.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
```

Each of the commands below prints nothing. The third shows that every promise ID the reference
and its cases cite exists in `spec/`:

```bash
grep -n 'deferred to K3b' docs/implementation/plan-and-apply/README.md
grep -rn 'work-orders/' docs/implementation/plan-and-apply
for id in $(grep -rhoE '\b[A-Z]{3,5}-[0-9]{3}\b' docs/implementation/plan-and-apply | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
```

`grep -ci owntools docs/implementation/plan-and-apply/cases/01-t1.yaml` prints 4 or more.

The check below runs with `/usr/bin/python3` and PyYAML. It parses the nine cases and the
reference, and checks:

- each case's digest, plan path and request keys;
- the chain in which each case's `applied` equals the previous case's `kept`;
- that the old digest is gone;
- case 01's two `ownTools` lines, where they sit, and `CTR-014` in its `sources`;
- the counts table;
- that ten stale phrases are gone from the reference, and the new line forms are in it.

It must print `9 cases; ok` and exit 0:

```bash
/usr/bin/python3 - <<'EOF'
import glob, hashlib, re, yaml
d = "docs/implementation/plan-and-apply/"
seq = ["01-t1", "02-t2", "03-t3-noop", "04-t4", "05-t5", "06-t6", "07-t7-revoke",
       "09-t7-restore-unflagged", "08-t7-restore"]
assert sorted(glob.glob(d + "cases/*.yaml")) == sorted(d + "cases/" + s + ".yaml" for s in seq)
c = {s: yaml.safe_load(open(d + "cases/" + s + ".yaml")) for s in seq}
old = "6bdd6bfbd979f2acd9faad689cafc57de87bc35dd642a14b683616475881f6f0"
bad = [s + ": old digest" for s in seq if old in open(d + "cases/" + s + ".yaml").read()]
for s, x in c.items():
    r = x["recorded"]; h = hashlib.sha256(r["text"].encode()).hexdigest()
    if r["digest"] != "sha256:" + h: bad.append(s + ": digest")
    if r["path"] not in (None, ".conduit/plans/" + h + ".txt"): bad.append(s + ": path")
    if not x["printed"].startswith(r["text"]): bad.append(s + ": printed")
    if any(not q["idempotencyKey"].startswith("plan:" + h + ":") for q in x["requests"]): bad.append(s + ": keys")
for a, b in zip(seq, seq[1:]):
    if c[a]["kept"] != c[b]["applied"]: bad.append(a + " -> " + b + ": chain")
t = c["01-t1"]["recorded"]["text"].splitlines()
i = next(i for i, l in enumerate(t) if l.startswith("detail defaults="))
own = ['detail ownTools Agent/agent-a {"stated":false} CTR-014', 'detail ownTools Agent/agent-b {"stated":false} CTR-014']
if t[i + 1:i + 3] != own or sum("ownTools" in l for l in t) != 2: bad.append("01-t1: ownTools")
if "CTR-014" not in c["01-t1"]["sources"]: bad.append("01-t1: sources")
text = open(d + "README.md").read()
row = r"^\| \[(\S+)\]\(cases/\S+\) \| [^|]+ \| [^|]+ \| (\d+) \| (\d+) \| (\d+) \|$"
rows = re.findall(row, text, re.M)
for s, n, p, o in rows:
    x = c[s]
    if (len(x["requests"]), len(x["recorded"]["text"].splitlines()), len(x["printed"].splitlines())) != (int(n), int(p), int(o)): bad.append(s + ": counts")
if len(rows) != 9: bad.append("counts table")
flat = " ".join(text.split())
gone = ["deferred to K3b", "No-op text is exactly", "omit the authored value entirely",
        "or value at a place governed by a secret reference", "O1/O5 record",
        "O3 records that limitation", "nearest sound reading", "A later order supplies",
        "A later order adds", "outside-apply Conduit tombstone"]
need = ["detail ownTools Agent/", "detail targetOwnTools Agent/", "SEC-001", "tombstone=(Conduit/"]
bad += ["README still says: " + g for g in gone if g in flat]
bad += ["README lacks: " + n for n in need if n not in flat]
print(len(c), "cases;", bad or "ok")
raise SystemExit(bool(bad))
EOF
```

With decision 1's two lines exactly as given, case 01's new digest is
`sha256:ca5cd45886bb418e2603949db1be4bd0a232832e6615aba0bdd13ae3cc85f5d7`. If yours differs, report
why.

Do not stage anything. In your report, give:

- each choice you made yourself;
- the new digest and the tool that computed it;
- every check's last line;
- without fixing it, any other place where the reference contradicts `spec/`.
