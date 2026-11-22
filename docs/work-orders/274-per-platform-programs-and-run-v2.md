# Work order 274: an item may state `platforms`, run elsewhere only through `run.v2`, which checks the archive on the node (DEC-049, the spec)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`; `vectors` is
empty. It needs no network and changes no engine code; if a crate is missing from the cache, stop
and fetch nothing. Run one cargo command at a time, and not `tools/gate.py`. Never print a whole
large file, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential. Run no `conduit` outside a test, no `codex` or `claude`, and
sign in nowhere. Prefix each `python3` with `PYTHONDONTWRITEBYTECODE=1`. Every file you write is
public: no absolute local path, user or machine name, or private repository name.

## What and why

**The owner's answer** ([DEC-049](../decisions/log.md#dec-049-2026-11-21-per-platform-programs-in-one-archive)):
PRV-007 -> "Per-platform in one archive (Recommended)". Capacity comes first (DEC-046), so a
second machine must run an Agent package's program. This order writes the spec.

**Today.** PRV-007 asks only the engine's own runner to run a `command` item whose program is a
relative path, and `run.v1` promises a provider never such an item. `implementation-item.v1`
rejects `platforms`. The engine keeps only an archive's unpacked members
(`InstalledPackage.files`, `engine/crates/core/src/archives.rs`), which do not repack to the
bytes PKG-004's digest is over. ENR-010 fixes each node's `kubernetes.io/os` and
`kubernetes.io/arch`.

**No case.** This order adds no promise ID, so CONTRIBUTING's rule 8 asks no case and the
no-case ratchet holds; the engine order adds the cases. The model account needs no row: PRV-007
and PRV-019 stand `text changed, not rerun`, the others are `language`.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); HOW-WE-WORK rule 1): 3, a
shared thing is owned once: one archive and digest, the engine's copy the one source carried; 4,
nothing assumes a vendor's agent: any package may ship per-platform programs. None at risk.

**Should this exist** ([HOW-WE-WORK rule 3](../../HOW-WE-WORK.md#1-the-rules-of-the-work)).
Yes, by existing practice: NuGet's `runtimes/` folders ship every platform in one package; a
kubelet admits no Pod whose `kubernetes.io/os` differs from its Node's; an OCI descriptor's
`digest`, `size` and `urls` are checked before use; a CSI plugin declares its optional calls.
Not taken (the coordinator's design notes): a program on the node's PATH, unpinned; per-platform
archives; `run.v2` in `runtime-provider.v1`, binding every runtime provider.

**When it runs.** On `main` from `621f9316` (#546). Through Conduit: one Codex worker turn, a
blind Codex verifier and Claude reader, one fix turn, the gate (DEC-046, DEC-033). By ruling 89,
each decision says what must hold; how is yours. The words, `run.v2`'s form, admitting it by
`requires`, keeping every installed archive, and a submitted project's item on the engine's
runner are the coordinator's, open to the owner's strike.

**Shared files.** Orders 271 and 273 also change `spec/core/operations.yaml`, `README.md` and
`spec/CHANGES.md`; whichever merges later rebases.

**Stop rule.** Stop, change nothing further, and report where a wording needs a choice this
order does not make (give the options), a set wording contradicts a promise no decision changes
(give both), a check refuses a line as given (give its error), a test or case fails (give its
failing line), or the start check prints otherwise. A choice no decision governs is yours: list
it under `own_choices`.

## The decisions

Wrap each insertion at its file's width.

1. **MAN-010** (`spec/promises/manifest.yaml`).
   - `then`, after "`nodeSelector` is a label selector.": "`platforms` is a list of one or more
     pairs, no pair twice, each `<os>/<arch>` in the values that `kubernetes.io/os` and
     `kubernetes.io/arch` take under PRV-014, such as `linux/arm64`. It is stated only beside a
     `command` whose program is a relative path, and that pair's program is then
     `platforms/<os>/<arch>/` followed by that path."
   - `observable`: after "and only the engine's own runner runs it, under PRV-007,", insert
     "unless its item is a package's and states `platforms`,"; before the "is published as a
     `package` item" that follows, insert "is a package's item that states `platforms` or". At
     its end: "`platforms` beside `package` or `remote` or beside a program that is not a
     relative path, empty, or with a pair written twice or not as `<os>/<arch>`, is rejected
     with a diagnostic naming the item."
2. **MAN-013.** `then`, after "as MAN-011 states.": "For each pair that the `platforms` of an
   implementation item of its manifest lists, it holds that pair's program, at the path MAN-010
   gives, as a regular file whose owner may execute it." `observable`, after "gives the
   project's directory as it was published.": "An archive that lacks one is rejected when the
   package is installed, with a diagnostic naming the item, the pair and the path. The digest of
   PKG-004, taken over the archive as distributed (PKG-022), fixes every such program, so one
   address and version has one digest whichever node runs it (PKG-025)."
3. **PLC-007** (`spec/promises/placement.yaml`). `when`, after "since nothing of the instance
   runs.": "Or, where an item that is run and is selected for the instance states `platforms`,
   the fixed `kubernetes.io/os` and `kubernetes.io/arch` of the node are not one of its pairs."
   `observable`, after "before any effect.": "For `platforms`, the diagnostic names the item, the
   node's pair and the listed pairs; a node that lacks either label matches none. The pairs are
   no selector: `linux/amd64` and `darwin/arm64` admit neither `linux/arm64` nor `darwin/amd64`."
4. **PRV-001** (`spec/promises/providers.yaml`). `when`, after "`package-source.v1` or
   `secret-provider.v1`": ", or lists `run.v2` under `requires`". `then`, at its end: "One that
   lists `run.v2` under `requires` and does not offer `runtime-provider.v1` is rejected in the
   same way." `observable`, at its end: "`runtime-provider.v1` does not list `run.v2`, so a
   provider that implements only `run.v1` stays valid: a runtime provider implements `run.v2`
   where its type lists it under `requires`, and TPL-004 then gives each instance an
   implementation of it. Wherever a promise names the operations of
   `runtime-provider.v1`, as PRV-004, PRV-005, PKG-020 and PKG-030 do, `run.v2` is among them;
   and wherever a promise or an Operation's description names `run.v1` as the call that asks
   for, starts or gives the inputs of a run, as PRV-011, PRV-019, PRV-030, SEC-018 and `stop.v1`
   do, a call of `run.v2` is meant too."
5. **PRV-007.**
   - `then`: after "the engine asks it for the run with `run.v1`,", insert " or with `run.v2` as
     below,".
   - `then`: from "It asks no runtime provider other than its own runner" through "that part ends
     failed, and nothing is run." becomes: "Where the item states `platforms`, its program is the
     one for the pair of the instance's node, under MAN-010. For a `command` item whose program
     the manifest gives as a relative path, whether or not SUB-010 has resolved it against a
     directory, the engine asks no runtime provider other than its own runner for the run, unless
     the item is a package's and states `platforms`; then it asks another only with `run.v2`,
     never with `run.v1`, giving `run.v1`'s inputs, and in `program` that program's path from the
     archive's root, and in `archive` the `digest`, `size` and `urls` of the package's archive as
     the engine keeps it. The engine keeps each installed package's archive exactly as it was
     distributed, for as long as the package is installed, and `urls` holds first a `file` URL of
     that copy. Where the engine may not ask the provider, or the provider does not implement
     `run.v2` under PRV-001, that part ends failed, nothing is run, and its diagnostic names the
     item, the provider and which of the two holds."
   - `observable`, after "which another runtime provider would not find.": "A package's item
     that states `platforms` is the exception: a provider that implements `run.v2` takes the
     archive to the run's machine and checks it there, under PRV-019, and `run.v1` promises a
     provider never such an item. The kept copy's SHA-256 is the package's digest, under
     PKG-004, and every runtime provider runs on the engine's machine, under PRV-002, so it can
     read that copy. An item of a submitted project has no archive, so only the engine's own
     runner runs it. Where the engine keeps the copy is DL7."
6. **PRV-019.** `when`: after "asked with `run.v1`", insert " or `run.v2`". `then`, at its end:
   "Asked with `run.v2`, on that machine and before each unpacking of the archive, it checks
   that the SHA-256 of the bytes it unpacks is the `digest` of `archive`. Where it is not, it
   runs nothing, and the run ends failed, its diagnostic naming the digest given and the one
   found. Otherwise it unpacks the archive into a directory of that run's own and runs `program`
   from it, in place of the first string of the item's `command`, the other strings being its
   arguments." `observable`, at its end: "How the archive's bytes
   reach that machine, and where the provider keeps them, is its contract; a check before each
   unpacking, and not only on arrival, finds bytes altered since."
7. **`run.v2`**, a new Operation document after `run.v1`'s in `spec/core/operations.yaml`, and
   `- run.v2` after `- run.v1` in `spec/core/conduit.yaml`'s exports.
   - Its description says, in `run.v1`'s style: it runs a package's item that states `platforms`
     as `run.v1` runs one, every rule stated of `run.v1`'s inputs, call and result holding for it
     but the one that `item` is never a `command` item whose program is a relative path; what
     `program` and `archive` are (PRV-007); the check before each unpacking (PRV-019); that the
     bytes' carriage is the provider's contract; that a provider implements it by its type's
     `requires` (PRV-001); and that it is a provider call (PRV-005).
   - `inputSchema`: `run.v1`'s, with these two in `properties` and in `required`:

     ```yaml
     program: {type: string, minLength: 1}
     archive:
       type: object
       properties:
         digest: {$ref: github.com/shrav-k/conduit/core#digest.v1}
         size: {type: integer, minimum: 1}
         urls:
           type: array
           items: {type: string, format: uri}
           minItems: 1
       required: [digest, size, urls]
       additionalProperties: false
     ```

   - `outputSchema`: `run.v1`'s. `effect`: "Checks the archive's digest on the machine given in
     `machine`, then starts the run of `program` for the instance there, under its runtime
     provider's own means, or answers with its existing run under PRV-019."
   - `run.v1`'s description: after "which only the engine's own runner runs", insert ", or, for
     a package's item that states `platforms`, a provider asked with `run.v2` (PRV-007)"."
   - `spec/core/capabilities.yaml`, at the end of `runtime-provider.v1`'s description: "A
     runtime provider may also implement `run.v2`, which runs a package's item that states
     `platforms` on another machine: this capability does not list it, and the provider's type
     lists it under `requires` (PRV-001)."
   - `spec/core/README.md`: after the `run.v1` row of the operations table, "| `run.v2` | No
     capability: required by a runtime provider's type | The engine, by a provider call |
     PRV-001, PRV-007, PRV-019 |", and after that of the exports table, "| `run.v2` | Operation |
     PRV-001, PRV-007, PRV-019 |". In its prose, "fifteen" becomes "sixteen", and "from run.v1,
     stop.v1" becomes "from run.v1, run.v2, stop.v1".
   - `spec/definitions.md`: in the `provider call` row, "call of an operation of" becomes "call
     of `run.v2` or of an operation of"; in the `run` row, after "with `run.v1`", insert " or
     `run.v2`".
8. **`implementation-item.v1`** (`spec/schemas/manifest.yaml`): `platforms`, an array of at
   least one unique string matching `^[a-z0-9]+/[a-z0-9]+$`, allowed only beside `command`
   (`dependentRequired`); a sentence of its description and its line in the file's header cite
   MAN-010. That the program is a relative path stays MAN-010's rule, outside the schema.
9. **DL7** (`spec/implementation-defined.yaml`): after "a cache of the archives it fetches",
   insert " beyond the copy of each installed package's archive that it keeps under PRV-007".
10. **`spec/CHANGES.md`**, under `## [Unreleased]`, after its last item:

    ```text
    - Changed: MAN-010 and MAN-013, a relative-path item may state `platforms`, `<os>/<arch>` pairs, each pair's program in the archive, or the package is rejected (DEC-049).
    - Changed: PLC-007, such an item is placed only on a node whose os and arch are a listed pair (DEC-049).
    - Changed: PRV-007, another runtime provider runs a package's such item only through `run.v2`, given the program and the kept archive's descriptor; any other relative-path item still runs on the engine's machine alone (DEC-049).
    - Changed: PRV-019, `run.v2` checks the archive's SHA-256 on the machine before each unpacking, and on a mismatch runs nothing and fails, naming both digests (DEC-049).
    - Added: `run.v2`, which a runtime provider implements by its type's `requires` (PRV-001), and `platforms` in `implementation-item.v1` (DEC-049).
    ```

## What to read

The promises named, PKG-004, PKG-022, PRV-002, PRV-014, ENR-010, TPL-004 and CTR-004; `run.v1`;
`runtime-provider.v1`; `implementation-item.v1`.

## Before you change anything

From the repository root:

```sh
git merge-base --is-ancestor 621f9316 HEAD; echo "main $?"
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
sed -n '/^## DEC-049/,/^## DEC-050/p' docs/decisions/log.md | grep -c '^Status: accepted, not done'
j spec/core/operations.yaml "whose program is a relative path, which only the engine's own runner runs."
j spec/promises/providers.yaml 'It asks no runtime provider other than its own runner'
j spec/promises/providers.yaml 'asked with `run.v1` to run an item for an instance.'
j spec/promises/manifest.yaml '`nodeSelector` is a label selector.'
j spec/promises/manifest.yaml 'is published as a `package` item, pinned as above;'
j spec/promises/placement.yaml 'since nothing of the instance runs.'
j spec/implementation-defined.yaml 'a cache of the archives it fetches,'
j spec/core/README.md 'none of these fifteen'
j spec/definitions.md 'asks for with `run.v1` in a setup'
git grep -l -e 'run\.v2' -e 'platforms' -- spec | wc -l | tr -d ' '
h=$(cd engine && rustc -vV | sed -n 's/^host: //p')
(cd engine && cargo metadata --offline --locked --format-version 1 --filter-platform "$h" >/dev/null 2>&1); echo "engine $?"
git status --short | wc -l | tr -d ' '
```

They print `main 0`, then `1` ten times, `0`, `engine 0` and `0`.

## Scope

Change, as the decisions say, `spec/promises/manifest.yaml`, `placement.yaml` and
`providers.yaml`, `spec/core/operations.yaml`, `conduit.yaml`, `capabilities.yaml` and
`README.md`, `spec/schemas/manifest.yaml`, `spec/definitions.md`,
`spec/implementation-defined.yaml` and `spec/CHANGES.md`. Change nothing else: not `engine/` (no
promise is added, so `engine/STANDING.md` needs no write), `conformance/`, `experiments/`,
`docs/`, `packages/` or `tools/`. Report what else needs changing.

## Done when

Each line of the first group prints otherwise on `main` at `621f9316` ("today") and passes after:

```sh
j() { tr -s ' \n' '  ' < "$1" | grep -oF -- "$2" | wc -l | tr -d ' '; }
k() { f=$1; shift; for s; do j "$f" "$s"; done | tr '\n' ' '; echo; }
k spec/promises/manifest.yaml '`platforms` is a list of one or more' "unless its item is a package's" 'states `platforms` or is published as a' '`platforms` beside `package` or `remote`' 'the item, the pair and the path.' 'whichever node runs it (PKG-025).'  # 1 six times; 0 today
k spec/promises/placement.yaml 'are not one of its pairs.' "the node's pair and the listed pairs"  # 1 1; 0 0 today
k spec/promises/providers.yaml 'or lists `run.v2` under `requires`' 'and does not offer `runtime-provider.v1`' 'PKG-030 do, `run.v2` is among them' 'a call of `run.v2` is meant too.' 'only with `run.v2`, never with `run.v1`' 'as long as the package is installed' 'submitted project has no archive' 'asked with `run.v1` or `run.v2` to run' 'the digest given and the one found' 'in place of the first string of'  # 1 ten times; 0 today
sed -n '/^  name: run.v2$/,/^---$/p' spec/core/operations.yaml | grep -c -e 'core#digest.v1}$' -e '^      program:' -e '^      archive:'  # 3; 0 today
j spec/core/operations.yaml 'states `platforms`, a provider asked with `run.v2`'  # 1; 0 today
grep -c '^  - run.v2$' spec/core/conduit.yaml  # 1; 0 today
j spec/core/capabilities.yaml 'A runtime provider may also implement `run.v2`'  # 1; 0 today
grep -c '^| `run.v2` |' spec/core/README.md  # 2; 0 today
k spec/core/README.md 'none of these sixteen' 'from run.v1, run.v2, stop.v1'  # 1 1; 0 0 today
k spec/definitions.md 'call of `run.v2` or of an operation of' 'with `run.v1` or `run.v2` in a setup'  # 1 1; 0 0 today
sed -n '/^  name: implementation-item.v1$/,/^---$/p' spec/schemas/manifest.yaml | grep -c '^    platforms:'  # 2; 0 today
j spec/schemas/manifest.yaml '^[a-z0-9]+/[a-z0-9]+$'  # 1; 0 today
j spec/implementation-defined.yaml "installed package's archive that it keeps under PRV-007,"  # 1; 0 today
sed -n '/^## \[Unreleased\]/,$p' spec/CHANGES.md | grep -c '(DEC-049)\.$'  # 5; 0 today
```

Guards, which pass today and must still pass. Run the engine's suite once, last: it reads
`spec/core`, which gains `run.v2`.

```sh
export PYTHONDONTWRITEBYTECODE=1
python3 tools/check_decisions.py HEAD WORKTREE | tail -1  # Errors: 0; its no-case ratchet shows head equal to base
python3 tools/check_docs.py | tail -1  # Errors: 0
python3 tools/check_schemas.py | tail -1  # Errors: 0
python3 tools/check_model_account.py | tail -1  # Errors: 0
python3 tools/promise_standing.py --check | tail -1  # Errors: 0
python3 tools/set_names.py --check | tail -1  # ends Errors: 0
python3 tools/check_private_paths.py | tail -1  # Errors: 0
git diff --check  # prints nothing
git status --short -uall | sed 's/^...//' | grep -vE '^spec/(promises/(manifest|placement|providers)\.yaml|core/(operations|conduit|capabilities)\.yaml|core/README\.md|schemas/manifest\.yaml|definitions\.md|implementation-defined\.yaml|CHANGES\.md)$' | wc -l | tr -d ' '  # 0
(cd engine && CARGO_NET_OFFLINE=true RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release)  # every test passes
```

Report each changed sentence before and after, `run.v2`'s description, and every check's last
line and the ratchet's. Stage nothing.

## What follows

DEC-049's orders, in sequence; R-3 can be written once the first three are merged:

- **The engine's**: decisions 1 to 5 built, each archive kept as distributed outside
  `state.json` (DL7's row of `engine/CONFORMANCE.md`), the cases, and `tools/check_schemas.py`'s
  PKG-020 and PKG-030 rules counting `run.v2`; it may split.
- **The kit's and ssh-runtime's**: `core#run.v2` in ssh-runtime's `requires`; the kept copy
  carried, kept by digest on the node and checked with `sha256sum` before each unpacking.
- **The packages'**: the two Agent packages for `darwin/arm64`, `linux/amd64` and `linux/arm64`,
  Linux static against musl (DEC-049 narrows DEC-035); the cross toolchain only on the owner's
  approval.
- **The vendor client's**: the contract names the client's lowest tested version, which the
  wrapper checks, naming the path and version found.

DEC-049 is done once these four and this order are merged. Found while writing: `place.v1` gets
no pairs, so a machine provider may choose a node none admits.

## Record

Run on 2026-11-22 through Conduit itself, by the dogfood's own Agents, in Pacific time, on
`main`.

**Worker-2's turn** (07:45 to 07:50) wrote DEC-049's promises in spec/: an implementation item
whose program is a relative path may state `platforms`; its archive holds each pair's program at
`platforms/<os>/<arch>/<program>`; such an item is placed only on a node whose labels are a
listed pair; another runtime provider is asked for it only with the new `run.v2`, which gives
the archive's descriptor and is checked on the node before unpacking. The spec and tool checks
pass, and the no-case ratchet stays at 106.

**Two blind reads at once** (07:51 to 07:56): `turn-verifier-2.turn.v1` found six musts and a
should where a promise still named `run.v1` alone or disagreed with the new text: MAN-009's key
list, PKG-019's placement, PRV-007's limits on ordinary `run.v1` calls, PRV-019's archive branch
on a call that answers with an existing run, PRV-028's and IFC-017's run URLs, REC-002's record
subject, and the provider guide's restart wording. The Claude reader passed it with two nits.
The coordinator widened the scope to those promises, spec/ only, and decided that a provider
asked with `run.v2` checks the archive's byte length against `size` before its digest, as an
OCI descriptor's size is checked. One fix turn (07:56 to 08:00) applied all nine; `run.v1`'s
behaviour is unchanged for every item that states no `platforms`.

**At the gate**, `runtime-provider-kit`'s schema-drift test failed: the kit keeps a copy of
the schemas `run.v1` reaches, and `implementation-item.v1` gained `platforms`. The coordinator
regenerated `src/schemas.json` with the test's own converter; the copy differs only by the
`platforms` property and its dependency, and the kit's 94 tests pass. Spec orders that change a
schema `run.v1` reaches change this copy too.

The gate's run on the squash is the coordinator's, at the merge.
