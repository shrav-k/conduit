# Work order 029: the documents after rulings 49 to 54

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail` or `grep`.

## What and why

On 2026-11-10 the owner made rulings 49 to 54 and gave a direction for Projects; two
investigations of the language's shape were recorded; work orders 025 to 028 changed the engine.
The admin has already updated the owner's review list, the tracker's state and waiting list, the
roadmap's step 3 and the build plan. Several other documents lag. Read first: rulings 49 to 54
(`docs/proposals/ruling-49/README.md` to `ruling-54/README.md`),
`docs/proposals/owner-direction/2026-11-10.md` and `2026-11-10-projects.md`,
`docs/proposals/language-shape/README.md`, `docs/proposals/simplification/README.md`,
`docs/proposals/enrollment/README.md`, `docs/proposals/sign-in/README.md`, the log of increment 4
and the records of work orders 025 to 028.

## What to do

1. **`spec/README.md`, its status only.** Say, dated 2026-11-10, that rulings 49 to 54 are accepted
   and not yet written into the specification, each in one line with a link, and that the
   language's shape is being settled first so that each is written once. Change no promise, schema
   or other file under `spec/`.
2. **The Codex Agent's stated routes.** In `packages/agent-codex/README.md` and in
   `reference-model/CONFORMANCE.md`, state finding 4-06 as settled
   (`docs/proposals/first-build/log-increment-4.md`): Codex 0.167.0 keeps six `collaboration.*`
   tools under every switch tried; a sub-agent it spawns, in the admin's trial, had no shell, could
   not read outside its sandbox and was refused a write by the read-only sandbox, and its requests
   of the engine carry the parent Agent's credential; so it is a route to more parallel model work
   within the Agent's own access, not around it. Keep the statement of finding 3-07 (Codex reads
   its home's `AGENTS.md`) as it is.
3. **Indexes.** In `docs/README.md`, list the proposal records added since order 026: `sign-in/`,
   `enrollment/` (with `networking-paradigms.md` and `verification-methods.md`), `simplification/`,
   `language-shape/`, `owner-direction/` (both records), and rulings 49 to 54, each with one line
   on what it holds, in the document's existing style. In `docs/proposals/first-build/README.md`,
   make increment 4's row state its findings and its two runs from the log. In
   `docs/work-orders/README.md`, list orders 027, 028 and this one.
4. **A staleness pass.** Read `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/OVERVIEW.md`,
   `reference-model/README.md` and `docs/guides/` for statements made stale by rulings 49 to 54 or
   work orders 025 to 028 (for example the commands `install`, `submit`, `replace`, `retire`,
   `unregister`, `--help`, and that `register` no longer preinstalls S1's packages). Fix them from
   the records, dated, keeping each document's role. Report each change as a finding.

## Scope

Change: `spec/README.md` (its status and contents prose only), `packages/agent-codex/README.md`,
`reference-model/CONFORMANCE.md`, `reference-model/README.md`, `docs/README.md`,
`docs/proposals/first-build/README.md`, `docs/work-orders/README.md`, `README.md`, `AGENTS.md`,
`CONTRIBUTING.md`, `docs/OVERVIEW.md`, `docs/guides/`. Do not change a promise, a schema, a ruling
record, the ruling log or any record under `docs/proposals/` other than the index named. Report
anything else that needs changing.

## Done when

The four items are done. From the repository root: `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass. Check the files you change for
trailing white space yourself; do not stage anything.

## Record

Run on 2026-11-10: the worker in 14 minutes, then a fresh verifier in 4, which found three stale
statements. The admin widened the scope for a second round to those, three outside-scope findings
of the worker's own, the removal of the rule that ruling 57 revoked, and the mention of rulings 55
to 59. The worker's links to the newest ruling records failed in its tree, which predated them, and
resolved after the rebase; its Rust run could not bind loopback ports in its sandbox, and no code
changed.
