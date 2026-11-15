# Work order 158: wording the fresh author reads at the pin: flags recorded with the plan, and CTR-013's citation corrected

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access only because the
engine's tests bind loopback ports, which the sandbox refuses without it; use it for nothing else,
and reach no other machine. Run one cargo command at a time, and never print a whole large file or
log. Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's
value, and never print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 20 of [its orders](../proposals/full-review/orders.md))
found two passages in `spec/` that the S1 rerun's fresh author reads and that say what they
should not. The author's reading path in `spec/README.md` sends it to the short form's limits,
and the pin freezes these words as they stand:

- [What the short form's meaning covers](../../spec/README.md#what-the-short-forms-meaning-covers)
  says a dangerous act's flag is "counted when used", a cascade is "counted under one flag" and a
  plan "counts every widening". That is ruling 55 A4's counting rule for the S1 rerun, in a
  section of `spec/README.md` that states limits, not promises, and a builder cannot tell what is
  counted or by whom ([FR-288](../proposals/full-review/findings/P02-s1-bar.md#fr-288), item 4,
  with its verifier's correction). PKG-031, which the last of those sentences cites, says the same
  in its observable.
- CTR-013's observable says "The engine starts no login and holds no token it yields (SEC-020)".
  SEC-020 says nothing about a login's token: it says the engine serves no `get-secret.v1` and
  passes no value of its own environment to a run. What does say it is PRV-029 (the engine reads
  none of a state directory's contents) and the limits in "Pending actions, sign-in and the
  network" (an installed package holds the credential of every login it runs)
  ([FR-434](../proposals/full-review/findings/E04-signin-isolation.md#fr-434), as its verifier
  corrected it; fix F99 in
  [the coordinator's decisions](../proposals/full-review/decisions.md#secrets-pending-actions-and-sign-in)).

Both are wording fixes that add and remove no behavior: the coordinator's under ruling 37, and for
CTR-013 under ruling 46 point 4 as well. This order logs them for the owner to strike (ruling 48).

## The decisions it applies

Line numbers are those of `main` at `e8a7bcd1`. If a line has moved, find the phrase by its words.

1. **The limits and PKG-031 record a flag and count nothing** (FR-288 item 4). In
   `spec/README.md`, in the paragraph that begins "Plan and apply are tooling" (lines 351 to 363),
   three phrases change and no other words do:
   - "named flag, counted when used:" becomes "named flag, recorded with the plan when used:".
   - "Each cascade is one command, printed and counted under one flag:" becomes "Each cascade is
     one command under one flag, printed and recorded with the plan:".
   - "A plan prints and counts every widening, including" becomes "A plan prints and records every
     widening, including". The plan is that sentence's subject, so "records" says "recorded with
     the plan".

   Rewrap from line 353 only as far as the change needs, with no line over 94 characters (the
   paragraph's widest line now), and stop at the first line that ends where it ended before.
   In `spec/promises/packages.yaml`, PKG-031's observable (line 439), "a plan prints and counts any
   widening" becomes "a plan prints and records any widening". The line stays within 94
   characters, and nothing else in PKG-031 changes. The safety catalogue's quotation of PKG-031 is
   written from an earlier commit, as its header says, and stays.
2. **CTR-013 cites what says it** (FR-434, the verifier's first fix; F99). In
   `spec/promises/contracts.yaml`, CTR-013's observable (lines 310 and 311), "The engine starts no
   login and holds no token it yields (SEC-020)." becomes "The engine starts no login and holds
   no token it yields (PRV-029 and the limits in spec/README.md)." The parenthesis follows
   PND-013's "(spec/README.md)" (`spec/promises/pending.yaml`, line 293). Rewrap the observable from
   line 311 as little as needed, at its four-space indent, with no line over 94 characters.
   SEC-020 gains no sentence. CTR-013's `when`, `then`, layer, level and vectors do not change, no
   promise moves, and nothing else in its observable changes here. The finding that an engine
   obligation sits in a language promise's observable is logged by the review's order 11, not
   here.
3. **IN_FORCE.md follows in the same change, and keeps the owner's words** (FR-288's verifier:
   "counted when used" is ruling 55 A4's ruled wording, so IN_FORCE.md changes in the same edit).
   - Topic 4's "In spec/" line that names "What the short form's meaning covers" (line 80): "Each
     dangerous act takes a named flag, counted when used:" becomes "Each dangerous act takes a
     named flag, recorded with the plan when used:". The rest of the line stays, its citations
     included.
   - Topic 13 gains one bullet in the ruling's words, at the same indent, right after the bullet
     that begins "A command counts as a line only if" (line 308):
     `  - Each dangerous act's named flag is counted when used (55 A4).` So the counting rule stays
     in force beside the rerun's other counting rules. Put no date in it: `check_rulings.py`
     refuses one there.
   - Ruling 55's and ruling 75's records do not change (ruling 67, point 4). Topic 9's line on
     75 C1 (line 210) already cites CTR-013 and the limits, and stays.
4. **Two texts that copy the old words follow.**
   - `experiments/013-s2-shared-login/plan.md` (F99: "together with S2's plan"): in row S2-LOG-P
     (line 89), "ruling 75 C1, C6, C7; SEC-020" becomes "ruling 75 C1, C6, C7; CTR-013, PRV-029".
     At line 241, "(75 C1; the engine holds none, SEC-020)" becomes "(75 C1; the engine holds
     none, CTR-013)", which keeps the line's length. Only the citations change. The file is an
     order-level plan that binds nothing and changes nothing accepted in the experiment's
     `README.md`, so it gets no dated amendment.
   - `docs/implementation/plan-and-apply/README.md`, section "Ordering and access" (lines 124 and
     125), quotes the limits: "A plan prints and counts every widening" becomes "A plan prints and
     records every widening", so that the quotation matches decision 1 word for word. Line 124
     grows to 95 characters, which its section already allows; rewrap nothing. Nothing else in the
     file changes: line 95 cites ruling 55 A4 for "counted when used", and line 218 cites ruling
     59 A3 and the batch-1 plan, not the limits.

## Scope

Change only `spec/README.md`, `spec/promises/contracts.yaml`, `spec/promises/packages.yaml`,
`IN_FORCE.md`, `experiments/013-s2-shared-login/plan.md` and
`docs/implementation/plan-and-apply/README.md`. In each, change only the lines the decisions name
and the rewrapping they need: under 30 changed lines in all.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
(cd engine && cargo test --release)
```

Each of these prints nothing:

```bash
git grep -n -e 'counted when used' -e 'counted under one flag' -e 'counts every widening' -e 'counts any widening' -- spec
git grep -n 'counts every widening' -- docs/implementation/plan-and-apply/README.md
git grep -n 'SEC-020' -- spec/promises/contracts.yaml experiments/013-s2-shared-login/plan.md
```

Each of these prints 1. The first six read their file with its line breaks as spaces, so a rewrap
does not hide a phrase:

```bash
tr -s ' \n' '  ' < spec/README.md | grep -c 'a named flag, recorded with the plan when used: `--reopen`'
tr -s ' \n' '  ' < spec/README.md | grep -c 'Each cascade is one command under one flag, printed and recorded with the plan: register its Conduit'
tr -s ' \n' '  ' < spec/README.md | grep -c 'A plan prints and records every widening, including one caused by'
tr -s ' \n' '  ' < docs/implementation/plan-and-apply/README.md | grep -c 'A plan prints and records every widening, including one caused by'
tr -s ' \n' '  ' < spec/promises/packages.yaml | grep -c 'a plan prints and records any widening'
tr -s ' \n' '  ' < spec/promises/contracts.yaml | grep -c 'holds no token it yields (PRV-029 and the limits in spec/README.md)\.'
grep -c 'Each dangerous act takes a named flag, recorded with the plan when used:' IN_FORCE.md
grep -c "^  - Each dangerous act's named flag is counted when used (55 A4)\.$" IN_FORCE.md
grep -c 'counted when used' IN_FORCE.md
```

`git status --short` lists only the six files in scope. Do not stage anything. Report each choice
you make as your own, and every check's last line.
