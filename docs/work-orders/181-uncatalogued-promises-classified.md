# Work order 181: the uncatalogued promises classified

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes five documents and no code; it
has no network access and needs none. It needs no cargo command; if you run one, run one at a
time. Never print a whole large file: read the safety catalogue, `account.yaml`, `G5.md` and
`IN_FORCE.md` by line ranges or with `grep`. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never read or print a key, a token or any
credential.

## What and why

On `main` at `ec926025`, `python3 tools/check_model_account.py` ends `Errors: 0` and prints:

```text
Rows: 245 (safety 217, not safety 14, uncatalogued 14)
Uncatalogued IDs: ENR-001, ENR-002, ENR-004, ENR-006, ENR-007, ENR-008, ENR-009, ENR-010, ENR-011, ENR-012, PKG-032, PND-008, PND-013, PRV-030
```

These 14 promises of the convention layer, and three that the tool does not read (CTR-014 and
ENR-005 of the language layer, ENR-003 of the interface layer), have no row in
[the safety catalogue](../implementation/SAFETY_CATALOGUE.md). The catalogue was written from
`spec/` at `e94c057` (its line 7). Orders 038b (ENR-001 to ENR-012 and PND-008), 038c (PND-013),
075 (CTR-014), 081 (PRV-030) and 082 (PKG-032) added them afterwards. The full review of
2026-11-14 found this
([FR-448](../proposals/full-review/findings/G1a-model-checking-register.md#fr-448), as its
verifier corrected it). This is order 41 of [its orders](../proposals/full-review/orders.md),
done when the tool exits 0 with an empty list of uncatalogued IDs.
[Order 153](153-model-account-checked.md), decision 2, left the classing to this order.

The class decides what v0 must show. Under
[ruling 78](../proposals/ruling-78/README.md#the-decisions), point 3.2, and
[IN_FORCE.md, topic 13](../../IN_FORCE.md#13-proof-the-build-experiments-and-measurement)
(line 290), each convention-layer safety promise in the catalogue must be held by a model, or have
a safety vector or a stated reason. A promise outside the catalogue is in no such count, and its
row in [the account](../../experiments/012-model-checking/account.yaml) has a class that says
nothing and no standing. The coordinator's D30 ([decisions](../proposals/full-review/decisions.md))
gives each of the 17 a row, classed by the catalogue's own test and not all provisionally as
safety, and makes the count in IN_FORCE.md (line 267 at the review's commit, 290 now) the recount.

The classing is the coordinator's, open to the owner to strike (the catalogue's lines 5 and 6).
This order states it, and you apply it. FR-448's verifier found that the analyst's 231 rested on
the analyst's own provisional classing and that ENR-005 needs a row too; its counterargument holds
classing ENR-001, ENR-002 and ENR-010 as safety arguable under the catalogue's exclusion of
document validity alone. Decision 1 answers each of the three with the slice its row quotes. No
promise, case, vector, model or tool changes.

## The decisions it applies

Line numbers are those of `main` at `ec926025`; if a line has moved, find it by its words. Each
block below is indented three spaces to sit in its list item: write its lines without those three
spaces.

1. **Each of the 17 is a safety promise, by the catalogue's own test.** The test (the catalogue's
   lines 20 to 29) asks whether a breach could grant access that should not be granted, disclose
   secret or identity material, lose or repeat an effect or perform one without authorization, or
   give a wrong result or account of who did what. An unknown outcome treated as known is an
   effect breach. The test includes what binds a gate, provider or implementation, counts
   overlapping safeguards separately, and excludes document validity alone, refusal of valid input
   and diagnostic wording alone. So a promise that guards one unsafe outcome is a safety promise,
   even where another promise guards the same outcome, and its row quotes that slice and takes its
   kind. Each of the 17 guards one, quoted in its row in decision 2. On the verifier's three, and
   on CTR-014:
   - ENR-001's result gives only the two IDs. A result that gave more could carry the join
     action, and with it a join secret, to a model.
   - ENR-002's `carriesSecret`, the slice the verifier names, tells the engine whether it may
     relay the action beyond root's panel at all (ENR-003, IFC-049).
   - ENR-010 keeps a machine's reports from changing the labels that selectors read, which
     PLC-007 and PRV-015 already guard as access.
   - CTR-014 rejects a Type contract, but not for its validity alone: a control over its pointer
     would let a call that a Conduit grants move reach that no Conduit grants. The catalogue
     classes PRV-006, whose “no operation changes it” fixes an instance's runtime, as access.

   None of the 17 goes under "The others" or "For the coordinator". By kind: access 6 (CTR-014,
   ENR-004, ENR-005, ENR-006, ENR-010, ENR-011), secret 5 (ENR-001, ENR-002, ENR-003, PND-008,
   PND-013) and effect 6 (ENR-007, ENR-008, ENR-009, ENR-012, PKG-032, PRV-030). By layer:
   language 2, convention 14, interface 1. Each is classed as `spec/` states it now. Later orders
   move four: the review's fixes F125 and F130 move ENR-005 and ENR-011, and
   [ruling 78](../proposals/ruling-78/README.md#the-decisions), point 16, changes ENR-002 and
   ENR-010 (IN_FORCE.md line 241). When they land, these rows follow them, as the catalogue's
   line 7 asks.
2. **The catalogue's rows.** Each quotation is the named field's text with its whitespace folded
   (the catalogue's line 27), and each layer and level is the promise's own. No row names an S1
   trace: no trace in `conformance/s1-safety-cases.md` requires any of the 17. In the tables under
   "## Safety promises", in each file's own order:
   - CTR-014 after CTR-013's row (line 112), PKG-032 after PKG-031's (line 176) and PRV-030 after
     PRV-029's (line 331):

   ```text
   | CTR-014 | language | must | access | `then`: “The Type lists no control whose `changes` holds the pointer, a pointer inside it or a pointer over it” — admitting such a control lets whoever may call it move the directory in which the Agent’s own tools work, reach that no Conduit grants or limits. | — |
   | PKG-032 | convention | must | effect | `observable`: “Of requests that name different digests, at most one installs its package, and nothing is put in the place of an installed package” — installing a second archive in place of the first can run an implementation that the first install did not authorize. | — |
   | PRV-030 | convention | must not | effect | `then`: “It reports succeeded while any run that it started for that ID is running” — reporting a stop done while a run of the instance still acts treats a running effect as stopped and can let a second run act for the same instance. | — |
   ```

   - a section for `spec/promises/enrollment.yaml`, in the form of the others, between the
     providers table and the line `### spec/promises/pending.yaml` (line 333), the place that the
     Contents table of `spec/README.md` gives the file (its line 219):

   ```text
   ### spec/promises/enrollment.yaml

   Read: 12 promises. Source: [spec/promises/enrollment.yaml](../../spec/promises/enrollment.yaml).

   | ID | layer | level | kind | why a breach is unsafe | S1 traces |
   | --- | --- | --- | --- | --- | --- |
   | ENR-001 | convention | must | secret | `then`: “The result gives only enrollment and pendingAction, their two IDs” — returning more, such as the join action, can put a join secret in a tool result that a model or another reader sees. | — |
   | ENR-002 | convention | must | secret | `then`: “It returns message, carriesSecret” — reporting an action that carries a secret as carrying none lets the engine relay that secret beyond root’s panel. | — |
   | ENR-003 | interface | must not | secret | `then`: “Any part of it appears in a record, result, inspection, observation, diagnostic or _meta” — publishing the join action on those surfaces discloses its secret to their readers. | — |
   | ENR-004 | convention | must | access | `then`: “No node represents it and the provider places no instance on it” — placing an instance on a machine that root has not admitted can disclose its requests to that machine. | — |
   | ENR-005 | language | must | access | `observable`: “code and fact alone never meet the minimum” — admitting a machine on a code or on what it says of itself makes a node of a machine whose identity nothing checked. | — |
   | ENR-006 | convention | must | access | `then`: “The request is dispatched only if all applicable checks hold” — dispatching an admission whose checks fail can make a node of a machine that root did not enroll or whose fingerprint no one compared. | — |
   | ENR-007 | convention | must | effect | `observable`: “An unknown accept call likewise makes no node and ends unknown” — making a node, or ending failed, after an acceptance whose outcome is unknown treats an unknown outcome as known. | — |
   | ENR-008 | convention | must | effect | `observable`: “A failed or unknown removal is reported as such, with its cause, never as a successful removal” — reporting such a removal as done treats a machine that may still be on the provider’s network as removed. | — |
   | ENR-009 | convention | must | effect | `observable`: “No expiry dispatches removal, admission or any other later work” — removing or admitting a machine on expiry performs work that no request authorized. | — |
   | ENR-010 | convention | must | access | `then`: “No later report adds, changes or removes a node's labels” — a machine whose reports could change its labels could match a selector that excludes it and receive the requests of workloads placed there. | — |
   | ENR-011 | convention | must not | access | `then`: “This establishes which instance sent a request or permits any request” — taking a join or an admission as identity or permission grants access that no open Conduit lists. | — |
   | ENR-012 | convention | must | effect | `then`: “Nothing is removed until root asks refuse-machine.v1” — removing a retired node’s machine without root’s request performs an effect that no request authorized. | — |
   ```

   - PND-013 after PND-011's row (line 347) and PND-008 after PND-007's (line 349), where
     `spec/promises/pending.yaml` puts them:

   ```text
   | PND-013 | convention | must | secret | `observable`: “The value appears in no result, structuredContent, response _meta, notification, diagnostic, log or record” — publishing a sign-in return discloses the login’s authorization code to the readers of those surfaces. | — |
   | PND-008 | convention | must | secret | `then`: “Only root's own panel shows their machine evidence, code, fingerprint or join action” — showing them on another surface can disclose a join secret or a device code to a reader other than root. | — |
   ```

3. **The catalogue's counts.** The "Read:" lines of contracts (line 103), packages (156),
   providers (301) and pending (335) give 12, 24, 30 and 13 promises. Lines 9 to 12 become:

   ```text
   Promises read: 519; safety: 411; for the coordinator: 0; others: 108. Safety by kind: access 199,
   secret 44, effect 125, result 43. Safety by layer: language 135, convention 231, interface 45.
   Safety in the seven convention files: 128 (G5’s list as ruling 39 worded it); safety of the
   convention layer anywhere: 231 (the wider reading, for the owner).
   ```

   After line 7, add these two lines. C is the first eight hexadecimal digits of the commit your
   tree starts from, as `git rev-parse --short=8 HEAD` prints them:

   ```text
   Work order 181 added the rows of the 17 promises it lacked, CTR-014, ENR-001 to ENR-012, PKG-032,
   PND-008, PND-013 and PRV-030, read from `spec/` as of commit `C`.
   ```

4. **Two pending lines that have landed.** Under "## Not yet written", the bullet "Topic 9, Agent
   client tools" (lines 814 and 815) and the bullet "Topic 10, managed enrollment" (lines 818 to
   820) each gain a last line, the first as IN_FORCE.md's topic 9 marks it (line 215), the second
   as its topic 10 does (line 239). That section's own rule (line 794) counts no line marked
   **In spec/** again:

   ```text
     **In spec/**: CTR-014, classed above, with LIF-013, ACC-027 and PRV-006 (IN_FORCE.md, topic 9).
   ```

   ```text
     **In spec/**: ENR-001 to ENR-012 and PND-008, classed above; PLC-026 changed with them and
     PLC-025 did not (IN_FORCE.md, topic 10).
   ```

5. **The account's rows.** In `experiments/012-model-checking/account.yaml`, delete the 14 rows
   whose class is `uncatalogued` (lines 1419 to 1438, 1457 and 1458, 1461 to 1464, and 1467 and
   1468). Insert their safety rows where the catalogue's order puts them, so that the safety rows
   stay first and in that order and the others stay sorted by ID (order 153, decision 1). The
   standings are G5's (`G5.md` lines 58 to 63), and no property checks any of the 14:
   ENR-001, ENR-004, ENR-006 to ENR-009, ENR-011, ENR-012, PKG-032 and PRV-030 are
   `not modelled`; ENR-002 and ENR-010, each decided in one step, are `static`; PND-008 and
   PND-013 are `left to MC9`. Each note names what FR-448 says the promise needs, with D28 for the
   enrollment model and the review's order 24 for MC9a. No row has `model`, `read`, `digest`,
   `was` or `outside`: none has run in a model, and nothing in the tree yet shows the two static
   rows. CTR-014, ENR-003 and ENR-005 get no account row: the account holds the convention layer
   only.
   - PKG-032 after PKG-029's row (lines 94 to 97), before `- id: SUB-009` (line 98):

   ```yaml
   - id: PKG-032
     class: safety
     standing: not modelled
     note: Written by work order 082 to settle MC7b-002, after MC7b ran; no model has checked it. FR-448
       names MC7b's rerun for it. Classed by work order 181.
   ```

   - PRV-030, then the ten ENR rows in ID order, after PRV-029's row (lines 461 to 468), before
     `- id: PND-001` (line 469). ENR-004, ENR-006, ENR-007, ENR-008, ENR-009, ENR-011 and ENR-012
     each have, after their own `id` line, the three lines of ENR-001:

   ```yaml
   - id: PRV-030
     class: safety
     standing: not modelled
     note: Written by work order 081 after MC4 ran; no model has checked it. FR-448 names MC4's re-registration
       for it, and a case in experiment 022's runtime suite. Classed by work order 181.
   - id: ENR-001
     class: safety
     standing: not modelled
     note: Enrollment, written by work order 038b, is in no model. FR-448 and D28 name the enrollment
       model for it. Classed by work order 181.
   - id: ENR-002
     class: safety
     standing: static
     note: Decided in one step by the machine provider's enroll.v1. FR-448 names a case in experiment
       022's machines suite for it, not a model. Classed by work order 181.
   - id: ENR-010
     class: safety
     standing: static
     note: Decided in one step, when a node is made and at each later report of its machine. FR-448 names
       a vector for it, not a model. Classed by work order 181.
   ```

   - PND-013 after PND-011's row, before `- id: PND-006` (line 501), and PND-008 after PND-007's
     row, before `- id: SEC-004` (line 509). PND-008's row is PND-013's with its own `id`:

   ```yaml
   - id: PND-013
     class: safety
     standing: left to MC9
     note: FR-448 names MC9a, order 24 of the full review, for it. Classed by work order 181.
   ```

6. **G5.md says so.** In `experiments/012-model-checking/G5.md`:
   - the paragraph of lines 5 to 10 becomes:

   ```text
   Since work order 153 the rows are in [account.yaml](account.yaml), which
   `tools/check_model_account.py` checks and whose counts it prints. [Standings](#standings) and
   [The account](#the-account) describe those rows, except the table of refuted or contested
   promises, which is as of 2026-11-11. Work order 168 brought the introduction's citation and the
   two counts under [What G5 still needs](#what-g5-still-needs) up to date. Work order 181 classed
   as safety the 14 convention-layer promises that the catalogue lacked. It changed the count below,
   the heading [Outside the 231](#outside-the-231) and the MC9 bullet to match, and added the last
   bullet under What G5 still needs. The rest of this file is the account of 2026-11-11.
   ```

   - in line 12, the bold "217 convention-layer safety promises" becomes the bold "231
     convention-layer safety promises", and the heading of line 83, "## Outside the 217", becomes
     "## Outside the 231". No link in the tree names the heading's old anchor;
   - in the MC9 bullet, the sentence of lines 194 and 195, which begins “PND-001 to PND-012”,
     becomes the sentence below. It keeps the range that orders 113 and 116 wrote:

   ```text
     “PND-001 to PND-012” is a range, not twelve rows of this bullet: PND-004 is of the interface
     layer, and PND-008, which work order 181 classed with PND-013, is in the last bullet.
   ```

   - after the bullet "Partial clauses and configurations" (lines 217 to 223), before the
     paragraph that begins "The catalogue and the source accounting agree" (line 225), a last
     bullet:

   ```text
   - **The 14 that work order 181 classed:** the convention-layer promises that the catalogue
     lacked until then
     ([FR-448](../../docs/proposals/full-review/findings/G1a-model-checking-register.md#fr-448)).
     No model has run on any of them, and no conformance case names one. PND-008 and PND-013 are
     `left to MC9`, for MC9a, order 24 of
     [the full review's orders](../../docs/proposals/full-review/orders.md). ENR-001, ENR-004,
     ENR-006 to ENR-009, ENR-011 and ENR-012 are `not modelled`, for the enrollment model (D28 of
     [the review's decisions](../../docs/proposals/full-review/decisions.md)). PKG-032 is
     `not modelled`, for MC7b's rerun, and PRV-030 for MC4's re-registration. ENR-002 and ENR-010
     are `static`: a case in experiment 022's machines suite is to show the first, and a vector the
     second.
   ```

7. **The count elsewhere.** In `IN_FORCE.md` line 290, "Of the 217 convention-layer safety
   promises" becomes "Of the 231 convention-layer safety promises" (D30). In
   `experiments/012-model-checking/README.md` line 5, "all 217 convention-layer safety promises"
   becomes "all 231 convention-layer safety promises". Nothing else in either file changes, and
   neither line is rewrapped.
8. **What stays.**
   - No promise, case, vector, expected failure, model or tool changes. Rule 8 of
     `CONTRIBUTING.md`, which [order 163](163-a24-at-entry.md) put in force, binds a new or changed
     promise, and none changes. [Order 174](174-vector-changes-declared.md)'s declarations cover
     vector changes, and there are none: add no `docs/work-orders/181-declared.yaml`.
   - The evidence that FR-448 names is later work, and this order writes none of it: the
     properties, cases and vector that decision 5's notes name for the 14; for ENR-003, a vector
     that no join action reaches a record; and, as its verifier added, a vector for ENR-005.
   - `tools/check_model_account.py` and its tests do not change.
   - In the catalogue nothing else changes: no existing row, "The others", "For the coordinator",
     "S1 traces", the line "Written from `spec/` as of commit `e94c057`", or any other line of
     "Not yet written". Three existing rows, PKG-031, PLC-026 and PRV-015, quote text that `spec/`
     no longer holds. Leave them, and say so in your report.
   - In G5.md nothing else changes. The bullets on the 77 static decisions and the 12 explicit
     exclusions count their own sets, though the tool will print `static: 80` and
     `not modelled: 22`: 2 and 10 of those are rows of decision 5. One static row, OPS-011, was
     graded by [order 160](160-mc2-rerun-077.md) after the bullet's count was written. Report that
     difference and leave it. The closing paragraph, which speaks of "the 217", is the account of
     2026-11-11, as the introduction says.
   - These cite 217 and do not change: the tracker's row A40 in `docs/AUDIT.md`, which describes
     order 116's account; `docs/guides/full-review.md` line 119, a brief of 2026-11-12; ruling 75's
     record; every Record; the review's decisions and orders.

## Scope

Change only:

- `docs/implementation/SAFETY_CATALOGUE.md`: decisions 2 to 4;
- `experiments/012-model-checking/account.yaml`: decision 5;
- `experiments/012-model-checking/G5.md`: decision 6;
- `experiments/012-model-checking/README.md`, line 5, and `IN_FORCE.md`, line 290: decision 7.

Rewrap no paragraph that this order does not change. Keep each Markdown line that you write at
most 100 characters, except a table row and the two lines of decision 7, which keep their form.

## Done when

From the repository root, each passes. The Python checks need PyYAML, hence `/usr/bin/python3`:

```sh
python3 tools/check_model_account.py               # exit status 0; last line Errors: 0
python3 tools/check_model_account.py | grep -c -x -F -e 'Rows: 245 (safety 231, not safety 14, uncatalogued 0)' -e 'Uncatalogued IDs: ' -e '  left to MC9: 12' -e '  not modelled: 22' -e '  static: 80' -e 'Static rows with no conformance case and no outside evidence: 26'   # prints 6
/usr/bin/python3 tools/check_model_account.py --history | tail -n 1   # Errors: 0
awk '/^## /{s=$0} /^\| [A-Z]+-[0-9]+ \|/{n[s]++} END{for(k in n) print n[k], k}' docs/implementation/SAFETY_CATALOGUE.md | sort   # 108 ## The others, then 411 ## Safety promises
awk -F' [|] ' '/^## /{s=$0} s=="## Safety promises" && /^[|] [A-Z]+-[0-9]+ [|]/{print "kind", $4; print "layer", $2}' docs/implementation/SAFETY_CATALOGUE.md | sort | uniq -c   # access 199, effect 125, result 43, secret 44; convention 231, interface 45, language 135
grep -c -x -F -e 'Promises read: 519; safety: 411; for the coordinator: 0; others: 108. Safety by kind: access 199,' -e 'secret 44, effect 125, result 43. Safety by layer: language 135, convention 231, interface 45.' -e 'convention layer anywhere: 231 (the wider reading, for the owner).' docs/implementation/SAFETY_CATALOGUE.md   # prints 3
/usr/bin/python3 -c '
import re, yaml
text = open("docs/implementation/SAFETY_CATALOGUE.md", encoding="utf-8").read()
pairs = re.findall(r"^### (spec/\S+\.yaml)\n\nRead: (\d+) promises\.", text, re.M)
differ = [f for f, n in pairs if int(n) != sum(1 for p in yaml.safe_load(open(f, encoding="utf-8")) if isinstance(p, dict) and "id" in p)]
print(len(pairs), "sections;", len(differ), "differ:", " ".join(differ))
'   # 27 sections; 0 differ:
/usr/bin/python3 -c '
import glob, re, yaml
fields = {}
for path in glob.glob("spec/promises/*.yaml") + glob.glob("spec/conventions/*.yaml") + glob.glob("spec/interface/*.yaml"):
    for p in yaml.safe_load(open(path, encoding="utf-8")) or []:
        if isinstance(p, dict) and "id" in p:
            fields[p["id"]] = {k: " ".join(v.split()) for k, v in p.items() if isinstance(v, str)}
ids = set("CTR-014 ENR-001 ENR-002 ENR-003 ENR-004 ENR-005 ENR-006 ENR-007 ENR-008 ENR-009 ENR-010 ENR-011 ENR-012 PKG-032 PND-008 PND-013 PRV-030".split())
found = wrong = 0
for line in open("docs/implementation/SAFETY_CATALOGUE.md", encoding="utf-8"):
    m = re.match(r"\| ([A-Z]+-[0-9]+) \| ([a-z]+) \| ([a-z ]+) \| [a-z]+ \| `([a-z]+)`: “(.+?)” — ", line)
    if m and m[1] in ids:
        f = fields[m[1]]
        found += 1
        wrong += not (m[2] == f["layer"] and m[3] == f["level"] and m[5] in f.get(m[4], ""))
print(found, "rows,", wrong, "wrong")
'   # 17 rows, 0 wrong
/usr/bin/python3 -c '
import re, yaml
section, order = None, []
for line in open("docs/implementation/SAFETY_CATALOGUE.md", encoding="utf-8"):
    if line.startswith("## "):
        section = line.strip()
    m = re.match(r"\| ([A-Z]+-[0-9]+) \| convention \|", line)
    if m and section == "## Safety promises":
        order.append(m[1])
rows = yaml.safe_load(open("experiments/012-model-checking/account.yaml", encoding="utf-8"))
safety = [r["id"] for r in rows if r["class"] == "safety"]
rest = [r["id"] for r in rows[len(safety):]]
print(len(order), safety == [r["id"] for r in rows[:len(safety)]], safety == order, rest == sorted(rest))
'   # 231 True True True
grep -c -w 217 IN_FORCE.md experiments/012-model-checking/README.md docs/implementation/SAFETY_CATALOGUE.md experiments/012-model-checking/G5.md   # 0, 0, 0 and 1 (G5's closing paragraph)
/usr/bin/python3 tools/vector_delta.py HEAD   # Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.
python3 tools/check_rulings.py                # Errors: 0
python3 tools/check_docs.py                   # Errors: 0
python3 tools/check_private_paths.py          # Errors: 0
git diff --check
git status --short                            # the five files in scope, and nothing else
```

The 26 static rows with no evidence are `main`'s 24, ENR-002 and ENR-010. On `main` the second
command prints 0, the fourth 394 safety rows, the sixth 0, the seventh `26 sections; 4 differ`,
the eighth `0 rows`, and the ninth `217 True True True`.

Then show that the account and the catalogue agree only together. With the catalogue changed and
the account as on `main`, the tool must fail on each of the 14 rows. From the repository root, one
at a time:

```sh
cp experiments/012-model-checking/account.yaml account.yaml.181
git show HEAD:experiments/012-model-checking/account.yaml > experiments/012-model-checking/account.yaml
python3 tools/check_model_account.py | grep -c "^[A-Z]*-[0-9]*: class must be 'safety' according to the catalogue$"   # prints 14
python3 tools/check_model_account.py | tail -n 1   # Errors: 14 (the tool alone exits 1)
mv account.yaml.181 experiments/012-model-checking/account.yaml
python3 tools/check_model_account.py | tail -n 1   # Errors: 0
```

Do not stage anything. In your report, give C, the last line of every check, the two failing
lines of the restore check, the difference of decision 8 about OPS-011, the three stale rows of
decision 8, and each choice you made yourself, marked as your own. The admin then stages the files
and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. A fresh Codex
verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-15 on the Mac from `main` `6ef168a2`: the worker (280 s) and a fresh Codex verifier
(246 s), which found nothing in one round. The promises the safety catalogue left uncatalogued are
classified under the catalogue's own rule, and the model-checking account, G5.md, the experiment's
README and IN_FORCE.md's count follow.
A fresh read-only Opus reader then verified the order (ruling 79, point 2). It ran the account's
tool with and without its history, the order's checks and the docs and rulings checks, and read
each classification against the catalogue's rule and the promise's text. It found every class
justified by the rule, nothing decided that the rule does not decide, and no must or should.
