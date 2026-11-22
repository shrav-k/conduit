# Work order 266: a Type that shares a client ID with a Type at another address reports conflict at install and at submission, and root alone confirms it with `confirmClientIds` (CTR-013)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list under `vectors` the case you add, with the promises it tests. This
order needs no network: it reaches no host and fetches no crate. Run cargo offline, one command at
a time; if a build asks for a crate not in the cache, stop and report, and fetch nothing. Never
print a whole large file, case, diff or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `conduit` outside a test,
`npm`, `codex`, `claude`, the dogfood harness or `tools/gate.py`, and sign in nowhere. Prefix each
`python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no absolute local
path, user, machine or private repository name.

## What and why

**The finding.** Order 263's worker: "CTR-013 requires an install to detect a client ID shared by
Types at different held addresses. The core validates individual Type login declarations in
packages.rs, but neither archives.rs nor the install.v1 request path implements cross-address
sharing or confirmClientIds."

**What `spec/` settles.** CTR-013's observable: "The engine compares the client IDs of a Type's
logins with every Type it holds at another address: every installed package's Types and every
submission's Types held by a template or instance (SUB-008, PKG-029). Types at one address share
freely", and installing such a package "reports conflict unless the request confirms it in
install.v1's optional confirmClientIds list. Only root may give that input; another caller giving
it reports refused. A confirmation naming a client ID shared with no such Type reports invalid."
Registering or replacing with such a project "reports conflict for such sharing, with no
confirmation", in OPS-023's order, before dispatch; each diagnostic names "CTR-013, its cause,
the client ID and both Types where shared, and reveals no URL value", and "A confirming install
record keeps only those input client IDs under REC-002."

**Today.** `packages.rs` checks one Type's logins (`sign_in_login_faults`); `install.v1`'s
schema admits `confirmClientIds` and the core ignores it. `example.org/s1/agents` and
`example.org/conduit/agent-codex` state one Codex client ID, and nothing stops installing both.

**Order 263.** Its decision 4 stops `conduit install` before any request on what the archives in
hand fail of CTR-013, "checked by the core's own code". With this check in the core's admission
path, its preflight gains it with no CLI change; it sends no `confirmClientIds`, so a share among
the archives in hand stops its run before any `install.v1`, and one with a held Type at the
engine's conflict reply (263's exit 1).

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 7,
everything is attributable: a Type cannot take another package's login identity unseen, and the
confirming install's record keeps what root confirmed; 2, access is inspectable. 5 is not at
risk: `spec/` reserves confirming to root, as ACC-032 reserves `purge.v1`.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes: it keeps a promise already written, by existing practice. dpkg refuses a package that would
overwrite a file another package owns unless `--force-overwrite` is given; OAuth treats a public
client's ID as impersonable (RFC 8252, section 8.6). Not taken: a confirmation at submission
(CTR-013 accepts none); a CLI flag (263 sends none); confirming per login.

**When it runs.** On `main` from `d7c81f61`, beside orders 255, 258, 262, 263 and 265. Through
Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for under 2 hours), a Codex
verifier and a Claude reader, blind and at once, running only what settles a doubt, one fix turn,
the gate (DEC-046, DEC-033). By ruling 89, each decision says what must hold; how is yours. The
texts, names and case, and the readings that root means root's own act serving none, that a
request changing nothing answers as today, that the `install-archive` input confirms nothing,
and that a record keeps the IDs only for a dispatched install, are the coordinator's, open to the
owner's strike.

**Shared files.** 255 changes core `requests.rs` (`reject`, `recover`), `explain.rs`,
`packages.rs`, `lib.rs` (`Input`) and CONFORMANCE's DL10 and DL2; 258 any core file (its
submission and Pool checks among them), `packages.rs` beside `toolsDirectory`'s check,
CONFORMANCE's DL16 and DL19, and new cases' `index.yaml`; 262 `requests.rs` (`complete`), a
`lib.rs` `mod` line and DL8; 263 the shell and CONFORMANCE's DL7 rows; 265 `registration.rs`
beside `Held::package_list` and `records.rs`'s `inspect.v1` branch; all of them
`engine/STANDING.md`. Touch only: in `requests.rs`, the `install.v1` block and one arm of
`record_content`; in `access.rs`, one check beside ACC-057's; in `explain.rs`, one
`REFUSAL_CAUSES` row; in `registration.rs`, one call in `prepare` beside LIF-041's check and the
reason in `reject`; in `lib.rs`, one `mod` line; in `records.rs`, `preflight`'s `replace.v1`
branch and, if needed, `replacement`; `packages.rs` and the shell not at all; in CONFORMANCE, a
new last section. Put the code in a new `client_ids.rs`. Whichever merges
later rebases and reruns `promise_standing.py --write`.

**Stop rule.** Change nothing under `spec/`. Stop, change nothing further, and report where:

- a wording needs a choice that CTR-013 and `spec/` do not make: report it with the options;
- a correct result needs a change outside the scope: a shell or loader source,
  `engine/crates/conformance/` beyond decision 7's tests, another case, `packages/`, a
  `Cargo.toml` or a lockfile;
- an existing test or case fails because it installs or holds Types at two addresses sharing a
  client ID, or `vector_delta.py` shows a change beyond the added case: report each with the
  expected and observed reply; change none;
- order 263 has merged and its preflight does not reach this check through the core: report it;
- a start-check line prints otherwise, a crate is missing, or a step needs the network.

A choice no decision governs is yours: list it under `own_choices`.

## The decisions

1. **What is compared** (CTR-013, SUB-008, PKG-029). A Type of the request (of the package an
   install adds, or of the project a `register.v1`, `replace.v1` or `register-project` submits)
   *shares* a client ID where a login of it states a `clientId` that a login of a *held* Type at
   another address states, compared byte for byte. Held: every Type of every installed package,
   each version, as `packages::catalog_entries` lists them; and every Type of each submitted
   project a template or an instance holds, as the request finds the state. Types at one address
   share freely: sibling Types, other versions, other submissions.
2. **Install** (CTR-013, OPS-023). After every validity check of `archives::install` and before
   it publishes: an item of `confirmClientIds` that the package shares with no held Type reports
   invalid; else a shared ID it does not list reports conflict; else the package installs as
   today. The check is in the core's admission path, which `install.v1` and the `install-archive`
   input both run, the latter confirming nothing. An address and version already installed at
   that digest answers as today, whatever `confirmClientIds` holds. A rejection changes no state.
3. **Root alone confirms** (CTR-013, OPS-023). An `install.v1` whose arguments hold
   `confirmClientIds`, even `[]`, from any sender but root on its own behalf and serving none (as
   `access.rs` reads root for ACC-032 and ACC-057), reports refused under CTR-013 with ACC-056's
   diagnostic, before its archive is read. `REFUSAL_CAUSES` gains spec's `CTR-013` cause word
   for word: "Only root may confirm a shared client ID in install.v1's confirmClientIds."
4. **Submission** (CTR-013, SUB-009). A `register.v1`, `replace.v1` or `register-project` whose
   project has a Type that shares a client ID, and that is otherwise valid, reports conflict
   before dispatch, beside any OBJ-025 or LIF-041 conflict. Nothing is registered or changed. A
   `replace.v1` that changes nothing (SUB-009) answers as today, even where it shares: its trial
   drops the old template, so `records::preflight`'s branch decides that, and keeps a CTR-013
   conflict's reason, rule and diagnostics as the `register.v1` branch does, not today's invalid
   under SUB-009. `reject` cannot take the reason from the rule ID: CTR-013 also names invalid
   `signIn` faults.
5. **The diagnostics** (CTR-013; ruling 52). Each has `ruleId` `CTR-013`, `level` `error`, no
   URL, and `message.text` exactly as below; one per client ID, Type of the request, and held Type
   with its place, in byte order of the text. NEW and HELD are qualified references,
   `ADDRESS#NAME`; PLACE is `installed package ADDRESS VERSION` or
   `a submitted project held by a template or instance`.

   ```text
   Client ID ID is stated by Type NEW in this package and by Type HELD in PLACE, at another address; root may confirm the sharing by sending install.v1 for this package with confirmClientIds listing ID.
   Client ID ID is stated by Type NEW in this project and by Type HELD in PLACE, at another address; a submission accepts no confirmation: give the Type another client ID, or have root install the project as a package with confirmClientIds listing ID.
   confirmClientIds lists ID, which no Type of this package shares with a Type at another address; list only shared client IDs.
   ```

   In order: an install's conflict, a submission's, an install's invalid confirmation, one per
   such ID.
6. **The record** (REC-002). Each record naming the invocation of a dispatched `install.v1` whose
   arguments hold `confirmClientIds` holds `inputs` exactly `{"confirmClientIds": LIST}`, LIST as
   sent. Other installs' records hold no `inputs`, as today.
7. **Tests**, in `engine/crates/conformance/src/lib.rs`'s `explanation_regressions` module (see
   `started`, `call`). Write them first, run them on the unchanged code, and report each failing
   line.
   - `install_reports_a_client_id_shared_across_addresses`: an install whose Type shares with an
     installed package's Type, and one sharing with a held submission's: conflict, decision 5's
     text, nothing installed; another version of one package, and sibling Types of one package,
     sharing an ID: installed. Today each installs.
   - `install_confirm_client_ids_is_roots_alone`: a caller other than root, and an onward request
     serving root, giving `confirmClientIds`: refused under CTR-013, archive unread; root
     confirming the shared ID: installed, its records as decision 6 says; root listing an unshared
     ID too: invalid, one diagnostic; the `install-archive` input: conflict. Today each installs.
   - `submission_reports_a_client_id_shared_across_addresses`: `register.v1` and `replace.v1`
     sharing with an installed Type and with a held submission's: conflict, decision 5's text,
     nothing changed; a resubmission at the held submission's own address sharing only with it,
     and a `replace.v1` changing nothing after root confirmed a sharing package: succeeds. Today
     each succeeds.
   - Change no existing test: one that now fails falls under the stop rule.
8. **The case** `contracts-client-ids-shared`, at
   `conformance/language/contracts/client-ids-shared/`, listed in that directory's `index.yaml`
   with `promises` `["CTR-013"]`, its `case.yaml` of profile `first-build`; each install an
   `install.v1` request with an `archive_file`, as `conventions/changes/t5/requests.json` sends
   one. From first start, root:
   1. registers a project at `example.org/conformance/client-ids` whose Type states `signIn` with
      client IDs `app_EMoamEEZ73f0CkXaXp7hrann` and `placeholder-one-address`, and a template of
      it (model: `contracts-sign-in-resolution`'s signed agent): succeeds;
   2. installs `fixtures/agents/agent-codex/contracts.tar.gz`: conflict, the first text naming
      `example.org/conduit/agent-codex#codex.v2` and that held Type; state unchanged;
   3. the same with `confirmClientIds` `["app_EMoamEEZ73f0CkXaXp7hrann","placeholder-unshared"]`:
      invalid, one diagnostic, for `placeholder-unshared`;
   4. with `["app_EMoamEEZ73f0CkXaXp7hrann"]`: succeeds, its records as decision 6 says;
   5. registers a project at `example.org/conformance/client-ids-other` whose Type states
      `app_EMoamEEZ73f0CkXaXp7hrann`: conflict, two diagnostics, one per held Type;
   6. registers at `example.org/conformance/client-ids` a project whose Type states
      `placeholder-one-address` alone: succeeds.

   Where the case can make, in its own steps, a caller other than root with an open Conduit to
   `management` listing `install.v1`, a step of it with `confirmClientIds` reports refused under
   CTR-013; else decision 7's test alone shows it (`own_choices`). Each step fails on the unchanged
   engine where this order changes its reply; report the runner's failing line. Declare it in
   `docs/work-orders/266-declared.yaml`, in the form of [256's](256-declared.yaml).
9. **The documents.** `engine/CONFORMANCE.md` gains, last, `## Client IDs shared across addresses
   (order 266)`, one paragraph starting `From order 266,`: `install.v1`, `register.v1` and
   `replace.v1` compare client IDs as CTR-013 says; only root on its own behalf confirms; the
   `install-archive` input (`conduit register --package`) confirms nothing; a request that changes
   nothing answers as before. Run `python3 tools/promise_standing.py --write`.

## What to read

CTR-013, PKG-029, SUB-008, SUB-009, OPS-023, REC-002, ACC-032 and ACC-056; `install.v1`,
`register.v1` and `replace.v1` in `spec/core/operations.yaml`. In `engine/crates/core/src/`:
`archives.rs` (`install`), `requests.rs` (the `install.v1` block, `record_content`), `access.rs`
(ACC-057's check), `explain.rs` (`REFUSAL_CAUSES`), `records.rs` (`preflight`, `replacement`),
`registration.rs` (`prepare`, `reject`), `inventory.rs` (`instance_type_and_package`) and
`packages.rs` (`catalog_entries`, the `signIn` checks). The cases `contracts-sign-in-resolution` and `conventions/changes/t5`, by step.

## Before you change anything

From the repository root; stop, change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor d7c81f61 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
j spec/promises/contracts.yaml "reports conflict unless the request confirms it in install.v1's optional confirmClientIds list."
j spec/core/operations.yaml 'no confirmation is accepted. The diagnostic names CTR-013, the shared client ID and both Types.'
j spec/core/operations.yaml 'only root gives it, and a request from another caller giving it reports refused.'
git grep -l confirmClientIds -- engine/crates | wc -l | tr -d ' '
grep -c '^pub(crate) fn install($' engine/crates/core/src/archives.rs
grep -c '^| CTR-013 | language | must | ' engine/STANDING.md
grep -c '^## Client IDs shared across addresses' engine/CONFORMANCE.md
test -e conformance/language/contracts/client-ids-shared; echo "case $?"
git grep -lE 'fn (install_reports_a_client_id_shared_across_addresses|install_confirm_client_ids_is_roots_alone|submission_reports_a_client_id_shared_across_addresses)\(' -- engine | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, `1`, `2`, `1`, `0`, `1`, `1`, `0`, `case 1`, `0`, `engine 0` and `0`.

## Scope

Change, in `engine/crates/core/src/`, a new `client_ids.rs`, `archives.rs`, and as "Shared
files" says `requests.rs`, `access.rs`, `explain.rs`, `records.rs`, `registration.rs` and
`lib.rs`; decision 7's module; create the case directory and `docs/work-orders/266-declared.yaml`;
add one entry to `conformance/language/contracts/index.yaml`; `engine/CONFORMANCE.md` as decision
9 says; `engine/STANDING.md` by the tool alone. Change nothing else: not `spec/`, another case,
`packages/`, `tools/`, `docs/decisions/`, a shell or loader source, the rest of
`engine/crates/conformance/`, `Cargo.toml` or `Cargo.lock`. Report anything else that needs changing.

## Done when

Each line of the first group prints otherwise on `main` today and passes after. The second group
are guards, which pass today and must still pass; run its last cargo line, the whole engine
suite, once, last.

```sh
ok='^test result: ok\. 1 passed'
(cd engine && cargo test --offline --release -p conduit-conformance --lib install_reports_a_client_id_shared_across_addresses 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-conformance --lib install_confirm_client_ids_is_roots_alone 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo test --offline --release -p conduit-conformance --lib submission_reports_a_client_id_shared_across_addresses 2>&1 | grep -c "$ok")  # 1; today 0
(cd engine && cargo run --offline --release -q -p conduit-conformance -- .. contracts-client-ids-shared 2>&1 | grep -c '^PASS contracts-client-ids-shared ')  # 1; today 0
grep -cF "Only root may confirm a shared client ID in install.v1's confirmClientIds." engine/crates/core/src/explain.rs  # 1; today 0
grep -c '^## Client IDs shared across addresses (order 266)$' engine/CONFORMANCE.md  # 1; today 0
s() { sed -n 's/^| CTR-013 | language | must | \([0-9]*\) | .*/\1/p'; }
test "$(s < engine/STANDING.md)" -gt "$(git show HEAD:engine/STANDING.md | s)"; echo "standing $?"  # standing 0; today standing 1
```

```sh
n=conformance/language/contracts/client-ids-shared
git add -N "$n" docs/work-orders/266-declared.yaml engine/crates/core/src/client_ids.rs  # so the checks see the new files
(cd engine && cargo fmt --all -- --check)
(cd engine && cargo clippy --offline -p conduit-core -p conduit-conformance --all-targets -- -D warnings)
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_docs.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_schemas.py | tail -1  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/promise_standing.py --check  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_private_paths.py  # Errors: 0
PYTHONDONTWRITEBYTECODE=1 /usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/266-declared.yaml; echo "exit $?"  # 0 undeclared; 0 errors; then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(engine/crates/(core/src/[a-z_]+|conformance/src/lib)\.rs|engine/(CONFORMANCE|STANDING)\.md|conformance/language/contracts/(index\.yaml|client-ids-shared/.+)|docs/work-orders/266-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --offline --release)  # the whole engine suite, every vector among it
git reset -q -- "$n" docs/work-orders/266-declared.yaml engine/crates/core/src/client_ids.rs  # stage nothing
```

Report, per decision, what changed and where; each new test's and the case's failing line on the
unchanged code, then its passing line; any test changed, and why; and every check's last line.

Stage nothing. The verifier's turn follows; then the gate, a read-only Claude reader, and the
merge.

## What follows

Named here and not in this order:

- A way for root to confirm through `conduit install`, which sends no `confirmClientIds` (263).
- Found while writing: `install.v1`'s optional `logins` output and `inspect.v1`'s, which CTR-013
  names, appear unbuilt; and "Root alone installs in v0" (ruling 78 point 10) is ruled, not built.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time.

**Worker-2's first four turns** (02:45 to 03:04) each stopped before a change the order's text
did not settle, and the coordinator decided each, open to the owner's strike:
- order 263's merged preflight steps the core over the engine's whole state, so every CTR-013
  collision the core finds, among archives in hand or with a held Type, stops the CLI's install
  in its preflight with exit 2; `install.v1` sent directly still reports conflict;
- `archive_file` is read only by the internal fixture path, so the portable case passes its
  archives inline, the smallest contracts-only archives that show the collision;
- the portable format's invoke gains an optional `archive` member for it;
- `confirmClientIds` inputs are recorded on the dispatching request's and invocation's records,
  not a keyed repeat's.
The fourth turn built the order: a Type sharing a client ID with a held Type at another address
reports conflict naming both and what root sends to confirm; `confirmClientIds` is root's alone;
submission and replacement check the same; the 11-step case and three regressions pass.

**Two blind reads at once** (03:05 to 03:10), the owner's answer for A77's window:
`turn-verifier-2.turn.v1` found two musts and a should; the Claude reader passed it with one
should. One fix turn (03:11 to 03:17), each must reproduced first:
- a name clash (OBJ-025) returned early and hid the CTR-013 conflict; both are now reported;
- two held submissions at one address with one Type name and client ID collapsed into one
  diagnostic; matches are now kept per holder;
- the root-only refusal is shown to come before any archive admission;
- the portable format's `archive` member is now described and schema-checked member by member.

At the merge the gate's whole suite failed two existing portable cases,
`sections-located-shared-fleet` and `sections-located-supplied-fleet`, whose step 3 ended invalid:
the reordered early return let OBJ-002 reach the default-Conduit check, which then reported
IFC-014. A fix turn by worker-1 applied the existing default-Conduit exception to same-kind Pool
and Fleet clashes too, with a regression that failed first; both cases report conflict under
OBJ-002 again, and the whole vectors target and the core's tests pass.

The gate's run on the squash is the coordinator's, at the merge.
