# Work order 254: every request fails closed when its record cannot be kept, and SEC-024's IPv4 address is RFC 3986's

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors`
lists the case this order adds. It needs no network and changes no engine code; if a crate is
missing from the cache, stop and fetch nothing. Run one cargo command at a time, and not
`tools/gate.py`. Never print a whole large file, diff or list: use `head`, `tail`, `grep`,
`sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and sign in nowhere.
Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is public: no
absolute local path, user or machine name, or private repository name.

## What and why

**What.** [DEC-044](../decisions/log.md#dec-044-2026-11-21-records-fail-closed-and-research-before-deciding)
logs the owner's answers. On SEC-016: "Unavailable now, record late (Recommended)", with SEC-024's
wording confirmed. On any request whose record cannot be kept, after the coordinator explained
that Vault refuses a request no audit device can record: "Can this work in parallel? Then we
should do spec and engine now". This order writes the spec; order 255 builds it in the engine.

**Today.** SEC-016 alone fails closed, and DL10 leaves other requests to each engine: ruling 41's
narrowing ([S13](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/secrets/README.md#s13-records-of-reads)).
[Ruling 78](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-78/README.md)
point 17, F15, widened it; IN_FORCE topic 7 holds it as *spec pending*, with the coordinator's
unrecorded answer, which DEC-044 replaces: the record is kept late. SEC-024 never says what an
IPv4 address is; order 236 found the engine reads RFC 3986's `IPv4address`, and
`tools/check_schemas.py` reads it through Python's `ipaddress`.

**Vectors.** REC-007 and SEC-016 have none that is portable: a case's caller persists state and
records together before any intent (conformance/README.md), so a record that cannot be kept is
the shell's fault, which order 255 tests. SEC-024's refusals are portable, whatever DL17 says; a
spelling of `::1` that counts is not, so order 255 adds two to its unit test.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): serves
2, as no access is acted on without the record that makes it inspectable, and 6, as an
invocation whose end cannot be recorded ends unknown and nothing repeats it (OPS-012).

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, as existing practice: Vault completes no request that no enabled audit device can record;
kube-apiserver's `blocking-strict` audit mode fails a request whose RequestReceived event cannot
be written; RFC 3986, section 3.2.2, admits "only the dotted-decimal form of four octets", as
Python's `ipaddress` and Rust's `Ipv4Addr` read it. Rule 7 makes the restatements citations.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for
under an hour), a Codex verifier, a Claude reader and the gate (DEC-030, DEC-033). Order 255,
the engine's, runs beside it and merges after it. By ruling 89, each decision says what must
hold; how is yours. The set wording, OPS-023's order, the catalogue's row, the case's id, and
pinning the spellings of `::1` that count in order 255's unit test rather than the case are the
coordinator's, open to the owner's strike (HOW-WE-WORK rule 6).

**Stop rule.** Change only the files in Scope. Stop, change nothing further, and report where
the set wording contradicts a promise no decision changes (give both; choose no other words), a
check refuses a line as given (give its error), the case fails on the unchanged engine or needs
an engine change, or the start check prints otherwise. A choice no decision governs is yours:
list it under `own_choices`.

## The decisions

Wrap each insertion at its file's width.

1. **REC-007,** in `spec/conventions/records.yaml`, after REC-006:

   ```yaml
   - id: REC-007
     layer: convention
     level: must
     when: >-
       The record of a request's access decision, under ACC-002, or of the dispatch of its
       invocation or of that invocation's end, under REC-001, cannot be kept.
     then: >-
       A request not yet dispatched is not dispatched, and reports unavailable. An invocation
       already dispatched ends unknown. Each record is kept when a record can next be kept.
     observable: >-
       The target's implementation receives no call for a request that is not dispatched, and its
       result carries an error diagnostic naming this promise, or SEC-016 for a read of a secret,
       which says that the record could not be kept, under IFC-050. OPS-023 puts this case after
       ACC-037's and before refused: the request reports unavailable even where it would be
       refused, and its result carries no other diagnostic, but a request that ACC-037 decides
       carries ACC-037's diagnostic alone. A request that the engine finds at a restart decided
       and not dispatched was never dispatched: it ends unavailable, not unknown. The result of an
       invocation already dispatched has the outcome unknown, and its end is recorded as unknown,
       as IMP-007 has an invocation in progress across a restart end. The records of each, of its
       access decision and of how it ended among them, are kept when a record can next be kept:
       REC-001 is met late, and not excepted. A request that only reads, such as `inspect.v1`, and
       a read of a secret, under SEC-010, are recorded and decided as any other request. SEC-016
       is this promise for a read of a secret. How the engine behaves when its storage is
       exhausted, beyond this promise, is DL10 of the implementation-defined list.
   ```

2. **Its citations.**
   - REC-001's observable, after "its reason where it has no outcome under OPS-005.": "A record of
     a request that cannot be kept when it occurs is kept late, under REC-007."
   - OPS-023's `then`, from "otherwise refused," through "examined or reported.", becomes:
     "otherwise unavailable where the access decision cannot be made under ACC-037, else
     unavailable where it cannot be recorded under REC-007, else refused; then invalid; then
     conflict; then unavailable for any other cause. Where the access decision cannot be made,
     nothing else about the request is examined or reported; where it cannot be recorded, nothing
     else is reported." Its observable, after "nothing about any other reason that applies",
     gains ", unless its access decision cannot be recorded, when it reports unavailable under
     REC-007". Promises saying OPS-023 puts refused first or before another reason need no
     change: ACC-037's and REC-007's cases share refused's place.
   - IMP-007's observable, after "has the outcome unknown.": "A request found at a restart
     decided and not dispatched ends unavailable under REC-007, not unknown."
   - DL10, after "and behavior when storage is exhausted": ", beyond what REC-007 fixes for a
     request whose record cannot be kept".

3. **SEC-016,** whole:

   ```yaml
   - id: SEC-016
     layer: convention
     level: must
     when: >-
       A read would be dispatched, and the record of its access decision or of its dispatch cannot
       be kept.
     then: >-
       It is not dispatched, and reports unavailable at once. Its record is kept when a record can
       next be kept.
     observable: >-
       No value is read without a record of the read under REC-001. The read's result carries an
       error diagnostic naming SEC-016, which says that the record could not be kept, and SEC-012
       says what follows: the run or the connection that it was for is not asked for or made, and
       the setup or the retirement that it belongs to ends as OPS-022 counts its parts. A read
       that the engine finds at a restart decided and not dispatched ends unavailable under
       REC-007, and its setup or retirement ends under OPS-022 and IMP-007 as any other. This is
       REC-007 for a read of a secret.
   ```

4. **SEC-024.** After "The URL is compared as the Remote writes it.": "An IPv4 address is one that
   RFC 3986's `IPv4address` matches, four decimal octets with no leading zeros, so `127.1`,
   `0x7f000001` and `0177.0.0.1` are not IPv4 addresses and do not count; `[::1]` is compared as
   an IPv6 address, so `[0:0:0:0:0:0:0:1]` counts and `[::ffff:127.0.0.1]` does not." Its
   restatements become citations:
   - `spec/schema-profile.md`, the `remote` URL row: from "A template whose item binds a header
     to a secret" through "among them (SEC-024)." becomes "Which URLs may carry a secret, and
     which hosts count as loopback, read as RFC 3986 has them, is SEC-024."
   - `spec/core/README.md`: from "A header that carries a secret, and the value" through
     "(SEC-024, SEC-013, IFC-041, DL17)." becomes "Where a header that carries a secret, and the
     value that an implementation of `get-secret.v1` reached by `remote` returns, may go is
     SEC-024, with SEC-013, IFC-041 and DL17."

5. **`spec/README.md`, Contents:** the records row reads "Draft; 7 promises; vectors test slices
   of 6 promises", and the totals sentence gains one promise in all and one of the layer
   `convention`.

6. **`spec/CHANGES.md`,** under `## [Unreleased]`, after its last item:

   ```text
   - Added: REC-007, a request whose access decision or dispatch cannot be recorded reports unavailable, as does one found undispatched at a restart; a dispatched one whose end cannot be recorded ends unknown; each record is kept late (DEC-044).
   - Changed: REC-001, a request's record that cannot be kept is kept late, under REC-007 (DEC-044).
   - Changed: SEC-016, REC-007 for a read of a secret: unavailable at once, and recorded late (DEC-044).
   - Changed: OPS-023, unavailable under REC-007 comes after ACC-037's and ahead of refused (DEC-044).
   - Changed: IMP-007, a request found undispatched at a restart ends unavailable under REC-007 (DEC-044).
   - Changed: DL10 leaves storage exhaustion beyond REC-007 to each engine (DEC-044).
   - Changed: SEC-024, an IPv4 address is RFC 3986's IPv4address, `[::1]` is compared as an IPv6 address, and its restatements in `spec/schema-profile.md` and `spec/core/README.md` become citations (DEC-044).
   ```

7. **IN_FORCE.md, topic 7.** The line beginning "*Spec pending, ruling 78*: when the engine"
   becomes, on one line: "- **In spec/**: a request whose record cannot be kept fails closed:
   REC-007, with SEC-016 its case for reads of secrets and OPS-023 its place among the reasons;
   one decided while the state cannot be read: ACC-037 (78 point 17, F15, amending 41's secrets
   record, as DEC-044 amends it)." Remove the sub-line beginning "*Coordinator's, spec pending*
   (F15)", which DEC-044 replaces. Keep the sub-line on `commit-measures.jsonl`: that file holds
   no record under REC-001 (80 point 5). In the line "Records, record.v1 and purging: REC-001 to
   REC-006", REC-006 becomes REC-007.

8. **The case,** `secret-reads-hosts-as-written`, at
   `conformance/conventions/secret-reads/hosts-as-written/case.yaml`, listed in that directory's
   `index.yaml` with `promises` `["SEC-024"]`; layer `convention`, profile `first-build`. Its
   steps are `resolve-project`s of one project whose template selects a `remote` item, marked
   `plain`, that binds a header other than `Authorization` to a place that
   `secret-reference.v1` governs. At `https://example.invalid/mcp` it is accepted; at
   `http://127.1:8080/mcp`, `http://0x7f000001/mcp`, `http://0177.0.0.1/mcp` and
   `http://[::ffff:127.0.0.1]/mcp`, each step reports invalid with a diagnostic naming SEC-024
   and leaves the state unchanged. The shape is yours; models:
   `conformance/language/packages/state-dir-binding/` and
   `spec/schemas/examples/invalid-by-rule/SEC-024-secret-header-to-http.yaml`. Declare it in
   `docs/work-orders/254-declared.yaml` (`kind: added`). Then run
   `python3 tools/promise_standing.py --write`.

9. **The model account and the catalogue** (AGENTS.md; orders 171b and 181).
   - `experiments/012-model-checking/account.yaml`: SEC-016's row becomes `standing: text
     changed, not rerun` with `was: restated`, its note gaining "Work order 254 made it REC-007's
     case for reads."; after REC-006's row, one with `id: REC-007`, `class: safety`, `standing:
     not modelled` and `note: Written by work order 254; not yet modelled.`; IMP-007's note gains
     "Work order 254 added REC-007's undispatched request."
   - `docs/implementation/SAFETY_CATALOGUE.md`: after REC-006's row, "| REC-007 | convention |
     must | access | `then`: “A request not yet dispatched is not dispatched” — dispatching a
     request whose access decision was not recorded acts on access no record shows. | — |"; in
     its head, after order 171b's sentence, "Work order 254 added the row of REC-007, which it
     wrote."; the counts become 521 read, 413 safety, access 200, convention 233 (both places)
     and 129 in the seven convention files; records.yaml's reads "Read: 7 promises".

## What to read

The promises named, and ACC-037, OPS-022, IFC-050 and DL17; `conformance/README.md`, "Running"
and "Reviewing vector changes"; `secret_transport` and its tests in
`engine/crates/core/src/packages.rs`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor f7b1af11 HEAD; echo "main $?"
git status --short | wc -l | tr -d ' '
git grep -l 'REC-007' -- spec conformance engine tools experiments docs/implementation | wc -l | tr -d ' '
grep -c '^- id: REC-' spec/conventions/records.yaml
sed -n '/^- id: SEC-016$/,/^- id: SEC-017$/p' spec/promises/secrets.yaml | grep -c 'is DL10\.$'
grep -c 'and behavior when storage is exhausted\.$' spec/implementation-defined.yaml
grep -c -e 'Today only reads of secrets fail closed' -e 'an exception to REC-001, and DL10 says' IN_FORCE.md
sed -n '/^## DEC-044/,/^## DEC-045/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
grep -cE '^\| SEC-024 \| .*\| no case, no reason \|$' engine/STANDING.md
sed -n '/^- id: SEC-016$/,/^- id: /p' experiments/012-model-checking/account.yaml | grep -c '^  standing: restated$'
test -e conformance/conventions/secret-reads/hosts-as-written; echo "case $?"
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
```

They print `main 0`, `0`, `0`, `6`, `1`, `1`, `2`, `1`, `1`, `1`, `case 1` and `engine 0`.

## Scope

Change `spec/conventions/records.yaml`, `operations.yaml` and `implicit.yaml`,
`spec/promises/secrets.yaml`, `spec/implementation-defined.yaml`, `spec/schema-profile.md`,
`spec/core/README.md`, `spec/README.md` (Contents alone), `spec/CHANGES.md`, `IN_FORCE.md`,
`experiments/012-model-checking/account.yaml` and `docs/implementation/SAFETY_CATALOGUE.md`, as
the decisions say; the new `conformance/conventions/secret-reads/hosts-as-written/` and one
entry of that directory's `index.yaml`; the new `docs/work-orders/254-declared.yaml`; and
`engine/STANDING.md` by the tool. Change nothing else, not
`docs/decisions/log.md` (DEC-044 stays not done), the engine or `tools/`; report what else needs
changing.

## Done when

Each line of the first group prints otherwise on `main` at `f7b1af11` ("today") and passes after:

```sh
j() { tr '\n' ' ' < "$2" | tr -s ' ' | grep -oF -- "$1" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$s" "$f"; done | tr '\n' ' '; echo; }
grep -c '^- id: REC-007$' spec/conventions/records.yaml  # 1; 0 today
k spec/conventions/records.yaml 'or SEC-016 for a read of a secret,' 'is kept late, under REC-007.' 'puts this case after ACC-037'  # 1 1 1; 0 0 0 today
k spec/conventions/operations.yaml 'else unavailable where it cannot be recorded under REC-007, else refused;' 'when it reports unavailable under REC-007.'  # 1 1; 0 0 today
j 'unavailable under REC-007, not unknown.' spec/conventions/implicit.yaml  # 1; 0 today
j 'beyond what REC-007 fixes' spec/implementation-defined.yaml  # 1; 0 today
k spec/promises/secrets.yaml 'This is REC-007 for a read of a secret.' 'for other requests when its storage is exhausted' '`IPv4address` matches'  # 1 0 1; 0 1 0 today
j 'an IPv4 address in `127.0.0.0/8`' spec/schema-profile.md  # 0; 1 today
j 'whose host is a loopback address' spec/core/README.md  # 0; 1 today
j 'Draft; 7 promises; vectors test slices of 6 promises' spec/README.md  # 1; 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-044)\.$'  # 7; 0 today
k IN_FORCE.md 'Today only reads of secrets fail closed' 'an exception to REC-001, and DL10 says' 'as DEC-044 amends it' 'REC-001 to REC-007 ('  # 0 0 1 1; 1 1 0 0 today
grep -cE '^\| REC-007 \| convention \| must \| 0 \| 0 \| outside \| none \| no case, no reason \|$' engine/STANDING.md  # 1; 0 today
grep -cE '^\| SEC-024 \| convention \| must \| 1 \| 0 \| outside \| core \| case passes \|$' engine/STANDING.md  # 1; 0 today
sed -n '/^- id: SEC-016$/,/^- id: SEC-017$/p' experiments/012-model-checking/account.yaml | grep -c '^  was: restated$'  # 1; 0 today
grep -c '^- id: REC-007$' experiments/012-model-checking/account.yaml  # 1; 0 today
k docs/implementation/SAFETY_CATALOGUE.md '| REC-007 | convention |' 'read: 521; safety: 413' 'access 200,' 'convention 233,' 'files: 129' 'anywhere: 233' 'Read: 7 promises'  # 1 1 1 1 1 1 1; 0 0 0 0 0 0 0 today
(cd engine && cargo run --offline --release -p conduit-conformance -- .. secret-reads-hosts-as-written 2>&1 | grep -c '^PASS secret-reads-hosts-as-written ')  # 1; 0 today
```

Guards, which pass today and must still pass. Run the engine's suite once, last:

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; its no-case ratchet shows head equal to base
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/254-declared.yaml; echo "exit $?"  # ends 0 undeclared; 0 errors. then exit 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^(spec/(conventions/(records|operations|implicit)\.yaml|promises/secrets\.yaml|implementation-defined\.yaml|schema-profile\.md|core/README\.md|README\.md|CHANGES\.md)|IN_FORCE\.md|engine/STANDING\.md|experiments/012-model-checking/account\.yaml|docs/implementation/SAFETY_CATALOGUE\.md|conformance/conventions/secret-reads/(index\.yaml|hosts-as-written/.+)|docs/work-orders/254-declared\.yaml)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes
```

Your report gives each changed sentence before and after; the case's steps, each URL with its
reply; and every check's last line and the ratchet's line. Stage nothing.

## What follows

- Order 255: REC-007 and SEC-016 in the engine, with tests, the DL10 row of
  `engine/CONFORMANCE.md`, and `[0:0:0:0:0:0:0:1]` and `[::0:1]` in
  `sec_024_reads_the_host_as_written`. DEC-044 is done once 255 and the PRV-007 research are.
- Found while writing: REC-007 does not reach the record of a request that cannot be read but
  whose sender is established; `spec/schemas/README.md` and `spec/core/operations.yaml` still
  list SEC-024's hosts; SUB-011 says DL10 leaves storage exhaustion to the engine.

## Record

Not yet run.
