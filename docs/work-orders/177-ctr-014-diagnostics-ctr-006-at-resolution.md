# Work order 177: CTR-014's diagnostics name what reaches the place, and CTR-006 checked at resolution

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; list in `vectors` the one case this order adds. This order has network
access only because the shell's tests in `cargo test --release` bind a loopback port; use it for
nothing else, and reach no other machine. Run one cargo command at a time, and never print a whole
large file, fixture or log. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential.

## What and why

[Order 157](157-own-tools-reported.md) made the engine check CTR-014's three rules on an Agent's
own-tools place when packages and projects are resolved
([FR-546](../proposals/full-review/findings/GL2-vectors.md#fr-546)). Its fresh Opus reader left
two shoulds to a follow-up order ([order 157's Record](157-own-tools-reported.md#record)), and this
is that order.

First, the diagnostics name the Type, the place and the rule, but not what the author must change.
Rule (c)'s text is
`rule (c): control {control} changes the place, a pointer inside it or a pointer over it`
(`engine/crates/core/src/packages.rs:630`). It does not say which input of the control's `changes`
reaches the place, at which pointer, or whether that pointer equals the place, lies over it or lies
inside it. Rule (b)'s says the place admits "a type other than string or null" (`:1305`) and names
no type. Rule (a)'s says the schema does not require "the place and every member on its path"
(`:1248`) and names no member. [Ruling 52](../proposals/ruling-52/README.md), point 2, asks every
diagnostic to say what happened as explicitly as it can.

Second, at resolution the engine accepts a Type that lists a control whose `inputSchema` neither
declares nor requires `revision`. Only publication sets `required: [revision]`
(`engine/crates/core/src/schemas.rs:1094`). CTR-006 has such a Type rejected when its package is
installed or its project submitted (`spec/promises/contracts.yaml:169-172`), and
`spec/schemas/contracts.yaml:48-51` leaves that rule to the engine, with the rule that a control
declares no output `settings`. Increment 4's log records the gap as
[finding 4-36](../proposals/first-build/log-increment-4.md). Its settlement asks for this check,
with a rejection vector whose reason is `invalid` and whose rule ID is CTR-006, and the
coordinator settles it so ([ruling 46](../proposals/ruling-46/README.md), point 4).

Third, [order 176](176-core-purity-enforced.md) gave the core a Clippy list of what it may not
call. Its Record leaves to a follow-up order two deprecated routes of the Threads group, which an
`allow(deprecated)` lets through, `std::thread::sleep_ms` and `std::thread::park_timeout_ms`,
together with the counts they change. This order adds them.

No outcome of any existing case, example or package changes. The coordinator found four Types under
`conformance/`, `packages/`, `spec/` and `experiments/` that list a control, and each of those
controls declares and requires `revision` and declares no output `settings`. Order 176 must be on
`main` before this order runs, since it creates `engine/crates/core/clippy.toml`. Line numbers are
those of `main` at `eac54d78`, which order 176 does not change in these files, and those of
`clippy.toml` as order 176 leaves it.

## The decisions it applies

1. **CTR-014's diagnostics say what to change.** Each keeps its rule ID `CTR-014`, its level, and
   the prefix it has today, `{scope}: Type {name} toolsDirectory {pointer}: ` (`:617` and `:630`).
   Their number, and their order relative to one another, do not change. Only the text after the
   prefix changes, to this:
   - **Rule (c)**, still one diagnostic for each listed control that reaches the place P:
     `rule (c): control {control} reaches the place: `, then one clause for each entry of the
     control's `changes` whose pointer reaches P, `input {input} sets {p}, which {relation} the place`.
     The clauses are joined by `; `, in the order the engine's map gives the entries, by input name
     in byte order. `{control}` is the entry of `controls` as written. `{input}` and `{p}` are
     written as JSON strings, as `serde_json::to_string` writes them, so that the empty pointer
     shows as `""`. `{relation}` compares decoded reference tokens as `pointers_reach` (`:1136`)
     does: `equals` where the tokens are equal, `lies over` where p's are a proper prefix of P's
     (the empty pointer lies over every place), and `lies inside` where P's are a proper prefix of
     p's.
   - **Rule (b)**:
     `rule (b): in a branch of the schema, the place admits {types}, where only string and null are allowed`.
     `{types}` names each type of the leaf's mask other than null and string (`mask & !9`,
     `:1302-1303`), in this order: `boolean`, `number`, `array`, `object`. One name stands alone,
     two are joined by ` and `, and more are joined by `, ` with ` and ` before the last. `number`
     also stands for `integer`, which `binding_mask` (`:1316`) does not tell apart. A place whose
     schema is only a `$ref` to `secret-reference.v1`, which the walk does not skip for CTR-014,
     admits all four. Where more than one branch fails, the text names the types of the first that
     the walk meets, since the walk stops there.
   - **Rule (a)** has four causes. For each of the first three it names the member `{member}` at
     which the walk stopped, and says why. `{member}` is the JSON Pointer of P's reference tokens up
     to and including the one the walk was testing, each token encoded as RFC 6901 encodes it (`~`
     as `~0`, then `/` as `~1`). The causes:
     - the value holding the member may be other than an object in the branch (the mask test at
       `:1262`):
       `rule (a): in a branch of the schema, the value holding {member} may be other than an object, so {member} may be absent`;
     - no part of the branch lists the member under `required` (`:1263-1265`):
       `rule (a): a branch of the schema does not list {member} under required`;
     - no part of the branch describes the member (`subs` empty, `:1294-1296`):
       `rule (a): a branch of the schema describes {member} under none of properties, patternProperties and additionalProperties`;
     - the walk cannot read a reference or a pattern (`branches` gives none, `:1251`, or
       `ecma_regex` fails, `:1276-1277`), which the checks before it make unreachable in
       practice: `rule (a): the walk could not read a reference or a pattern of the schema`.

     Where the mask test and the `required` test both fail for one member, the mask test, which
     the walk makes first, gives the cause, as on `main`.

   `binding_parts`'s error may become a `String`. How the walk learns `{member}` is your choice.
   MAN-017's result must not change: `binding` (`:1220-1240`) still maps every failure to `None`,
   the test for `own_tools: false` at `:1299-1308` keeps its masks and its skip of a secret part,
   and the `packages-binding-` cases pass unchanged.

2. **CTR-006's two rules on a listed control are checked at resolution.** In the loop of
   `Catalog::resolve` over each scope's documents (`:582-647`), which skips the core package,
   take each Type that lists `controls`, whether or not it states `toolsDirectory`. Resolve each
   listed control once, as the CTR-014 check does now (`resolve_name`, `:620-623`), and use it for
   both checks. Read its Operation's `spec`. The control breaks:
   - the first rule if its `inputSchema`'s root `properties` holds no member `revision`. REV-007
     says that every control declares the input `revision`, under CTR-006
     (`spec/conventions/revisions.yaml:103-104`), and the root's properties are the inputs
     (CTR-002, `spec/promises/contracts.yaml:38-40`);
   - the second if its `inputSchema`'s root `required` does not list `revision` (CTR-006,
     `spec/promises/contracts.yaml:164`);
   - the third if its `outputSchema`'s root `properties` holds a member `settings` (CTR-006, the
     same line).

   An absent `inputSchema` declares and requires nothing. CTR-012 holds a control's root to
   `type`, `properties`, `required` and `additionalProperties` (`spec/promises/contracts.yaml:274`),
   so for an Operation with `changes` the root is the only place to read. An Operation listed
   with no `changes` is read the same way; CTR-006's rule that it hold `changes` stays unchecked
   (decision 7). A control that breaks one or more of these rules gives
   the Type one error diagnostic whose `ruleId` is `CTR-006`. Its message is
   `{scope}: Type {name} lists control {control}: `, written as the CTR-014 prefix writes the scope
   and the Type, then the text of each rule broken, joined by `; `, in this order:
   - `its inputSchema does not declare the input revision under properties`
   - `its inputSchema does not list revision under required`
   - `its outputSchema declares the output settings`

   The resolution is then rejected, as for the loop's other errors (`:648-650`). A Type's
   diagnostics come in this order: CTR-014's rule (a) or (b), if any; then, for each control in
   the order of `controls`, its CTR-006 diagnostic and then its CTR-014 rule (c) diagnostic, each
   if any.

3. **Unit tests pin each text.** In the module's `tests` (`:1418`), beside
   `own_tools_control_pointers_compare_decoded_reference_tokens`, add the four tests below. Each
   asserts whole texts with `assert_eq!`, never a part with `contains`. The three CTR-014 tests
   assert the text after the prefix. The first two call `binding_parts` with an empty
   `Environment { scopes: BTreeMap::new() }`, one `BindingPart` whose `scope` is `project`, whose
   `root` and `schema` are the schema given and whose `secret` is false, the tokens of P from
   `pointer_tokens`, `core` equal to `"core"`, and `own_tools` true. P is `/directory` unless a
   line says otherwise. Each case below gives a schema and the text it must give.
   - `own_tools_rule_a_names_the_member_and_the_cause`:
     - `{type: object, properties: {directory: {type: string}}}`:
       `rule (a): a branch of the schema does not list /directory under required`;
     - P `/tools/directory`, `{type: object, properties: {tools: {type: object, properties:
       {directory: {type: string}}, required: [directory]}}}`:
       `rule (a): a branch of the schema does not list /tools under required`;
     - P `/tools/directory`, `{type: object, properties: {tools: {type: [object, 'null'],
       properties: {directory: {type: string}}, required: [directory]}}, required: [tools]}`:
       `rule (a): in a branch of the schema, the value holding /tools/directory may be other than an object, so /tools/directory may be absent`;
     - P `/a~1b`, `{type: object, required: [a/b]}`:
       `rule (a): a branch of the schema describes /a~1b under none of properties, patternProperties and additionalProperties`;
     - `{type: object, properties: {directory: {type: string}}, anyOf: [{required: [directory]},
       {}]}`: `rule (a): a branch of the schema does not list /directory under required`.
   - `own_tools_rule_b_names_the_admitted_types`, each schema being `{type: object, properties:
     {directory: S}, required: [directory]}` for the S given:
     - `{type: [string, 'null']}` and `{anyOf: [{type: string}, {type: 'null'}]}`: accepted, as
       today;
     - `{type: [string, number]}` and `{type: integer}`:
       `rule (b): in a branch of the schema, the place admits number, where only string and null are allowed`;
     - `{type: [boolean, array]}`: the same text with `boolean and array`;
     - `{anyOf: [{type: string}, {type: object}]}`: the same text with `object`;
     - `{}` and `{$ref: 'core#secret-reference.v1'}`: the same text with
       `boolean, number, array and object`.
   - `own_tools_rule_c_names_each_input_pointer_and_relation`, through the function that builds
     rule (c)'s text from the control as written, its `changes` and P. The control is
     `#set-tools.v1` and P is `/tools/directory`:
     - `{tools: /tools}`:
       `rule (c): control #set-tools.v1 reaches the place: input "tools" sets "/tools", which lies over the place`;
     - `{d: /tools/directory}`: one clause, `input "d" sets "/tools/directory", which equals the place`;
     - `{a: /tools/directory/x, b: /other, c: /tools/directory/y}`: two clauses, for `a` and `c`,
       each `which lies inside the place`, joined by `; `;
     - `{whole: ''}`: one clause, `input "whole" sets "", which lies over the place`;
     - `{'say "hi"': /tools/directory}`: one clause, `input "say \"hi\"" sets "/tools/directory", which equals the place`;
     - `{x: /tools/dir, y: /toolsdirectory}`: no text, so no diagnostic.
   - `listed_control_faults_name_each_rule_of_ctr_006`, through the function that builds the text
     after `lists control {control}: ` from an Operation's `spec`. Let I be `{type: object,
     properties: {revision: {type: string}, limit: {type: integer}}, required: [revision]}`:
     - `{inputSchema: I, outputSchema: {type: object, properties: {revision: {type: string}}},
       changes: {limit: /limit}}`: no fault;
     - `{changes: {limit: /limit}}`: the first two texts, joined by `; `;
     - I without its `required`: `its inputSchema does not list revision under required`;
     - I without `revision` under its `properties`:
       `its inputSchema does not declare the input revision under properties`;
     - `{inputSchema: I, outputSchema: {type: object, properties: {settings: {type: object}}},
       changes: {limit: /limit}}`: `its outputSchema declares the output settings`;
     - `{outputSchema: {type: object, properties: {settings: {type: object}}}, changes: {limit:
       /limit}}`: all three texts, joined by `; `.

   Write YAML's flow mappings above as `json!` values. The functions' names and signatures are
   your choice; resolution builds its messages through them.

4. **One rejection case.** Add `conformance/language/contracts/control-without-revision/`, with ID
   `contracts-control-without-revision`, modelled on `conformance/language/contracts/own-tools-control/`:
   the same `fixture.json`, and a `case.yaml` of the same form and step with `CTR-006` in place of
   `CTR-014` wherever that case names a rule. So the step expects `accepted: false`,
   `reason: invalid`, exactly one diagnostic `{ruleId: CTR-006}` with no message asserted,
   `required_promises: [CTR-006]`, no records, no external intents, `stateUnchanged: true`, and
   `/reply/environment`, `/reply/digest` and `/reply/contexts` absent. Its promises are CTR-006
   alone, its `layers` are `[language]`, and its title is `Reject a listed control without revision`.
   The project's `conduit.json` gives the address `example.org/control-without-revision`. Its
   `documents.json` holds two documents:
   - an Operation `set-limit.v1` whose `spec` is `{inputSchema: {type: object, properties: {limit:
     {type: integer}}, additionalProperties: false}, changes: {limit: /limit}}`;
   - a Type `limited.v1` whose `spec` is `{family: Agent, schema: {type: object, properties:
     {limit: {type: integer}}}, controls: ['#set-limit.v1']}`.

   With the fault removed, the same project resolves as accepted. To remove it, add `revision` to
   the `properties` of the Operation's `inputSchema` exactly as order 157's control fixture
   (`conformance/language/contracts/own-tools-control/project/documents.json`) declares it, and
   `required: [revision]` beside them. Check this once, say how in your report, and commit nothing
   of it.

   The coverage index and the declaration follow order 157's decision 5. The case's entry holds
   its `id`, its `case` path relative to `conformance/`, `promises: [CTR-006]` and
   `safety_rows: []`, in that order, and belongs in `conformance/language/contracts/index.yaml`,
   before `contracts-own-tools-control`. Write it in a `cases` list in `conformance/manifest.yaml`,
   then, from `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with no
   flag, which moves it and leaves `manifest.yaml` byte for byte as it was. Write
   `docs/work-orders/177-declared.yaml` with one `added` declaration for the case, in the form of
   `docs/work-orders/157-declared.yaml`, and nothing more. CTR-006 keeps `vectors: []`, nothing
   under `spec/` changes, and `conformance/expected-failures.yaml` keeps its empty list, since the
   case passes.

5. **Finding 4-36 settles.** In `docs/proposals/first-build/log-increment-4.md`, row 4-36 (line
   47), the cell "What the build did", now `Nothing yet.`, becomes:

   > Order 177 checks at resolution that each control a Type lists declares `revision` under its
   > `inputSchema`'s `properties`, lists it under `required` and declares no output `settings`. The
   > case `contracts-control-without-revision` is rejected as `invalid` with CTR-006.

   The Settlement cell, now "Open. A later order checks ... No change to `spec/`.", becomes:

   > Fixed by order 177 for the rules on `revision` and `settings`. CTR-006's other rules on a
   > listed control are not yet checked at resolution. No change to `spec/`.

   In that cell, "order 177" is a Markdown link to this order's own file in `docs/work-orders/`,
   the one whose name begins `177-`, written relative to the log as the row's link to order 157's
   Record is. Nothing else in the log changes.

6. **Two more Threads entries.** In `engine/crates/core/clippy.toml`, after the
   `std::thread::park_timeout` line of `disallowed-methods` (line 63), add a line for
   `std::thread::sleep_ms` and then one for `std::thread::park_timeout_ms`. Each has the form of the
   line above it and the same reason as the other Threads entries, copied from it, and neither
   carries `allow-invalid`. Both functions are deprecated but still in `std`, and nothing in the
   core forbids the `deprecated` lint, so an `#[allow(deprecated)]` lets either through today. The
   list then has 60 methods, 13 types and 7 macros, 80 entries in all, so order 176's first count
   prints 80 where its Record says 78. Order 176's text and Record are history and do not change,
   and neither do `engine/README.md` and `engine/crates/core/src/lib.rs`.

7. **What stays.**
   - Rule (c) still compares pointers with `pointers_reach`, and the three rules accept and reject
     what they do today. Order 157's three `contracts-own-tools-` cases and three
     `inventory-own-tools-` cases pass unchanged; they assert no message.
   - No other rule of CTR-006 is checked here: not that a listed operation holds `changes`, not
     that each key of `changes` names one of its inputs, not that each pointer addresses a place
     the Type's schema describes
     ([FR-107](../proposals/full-review/findings/L09-packages.md#fr-107) asks what "describes"
     means), not that no two pointers overlap, and not which schema the input `revision` has.
     That gap is the coordinator's to log; add no row for it.
   - The by-rule outcome table, `engine/crates/shell/tests/expected/by-rule-outcomes.txt`, does not
     change. No example set lists a control that this check refuses, and the two
     `CTR-006-*` sets still end in the error of finding 4-35. If a line of it changes, report it
     and change nothing for it.
   - No other source changes, and neither does any existing case or fixture,
     `conformance/case.schema.json` or anything under `conformance/case-schema/`.

## Scope

Change only `engine/crates/core/src/packages.rs`, `engine/crates/core/clippy.toml` (decision 6
alone), `conformance/language/contracts/index.yaml`, and the two cells of row 4-36 in
`docs/proposals/first-build/log-increment-4.md`. Add
`conformance/language/contracts/control-without-revision/` (`case.yaml`, `fixture.json`,
`project/conduit.json` and `project/documents.json`) and `docs/work-orders/177-declared.yaml`.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- own_tools_ listed_control_
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. contracts-
cargo run --release -p conduit-conformance -- .. packages-binding-
cargo run --release -p conduit-conformance -- .. inventory-own-tools-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third command reports 5 passed and 0 failed: the four new tests and order 157's. The pieces
tool prints `Nothing to move.` The three runner commands end `5 cases passed`, `8 cases passed`
and `3 cases passed`. Then, from `engine/`, these four print 80, 2, 2 and 0:

```bash
grep -c 'path = "' crates/core/clippy.toml
grep -c 'allow-invalid = true' crates/core/clippy.toml
grep -c -E 'path = "std::thread::(sleep_ms|park_timeout_ms)"' crates/core/clippy.toml
cargo clippy --color never -p conduit-core --all-targets -- -D warnings 2>&1 | grep -c '^warning'
```

Next, show that the tests and the case fail with decisions 1 and 2 undone. From `engine/`, run
`cp crates/core/src/packages.rs target/packages.rs.fixed`. Then, in `crates/core/src/packages.rs`
alone, put back `main`'s texts and turn the new check off, keeping the code compiling: every
rule (a) cause gives `main`'s text of `:1248`, rule (b) gives `main`'s text of `:1305`, rule (c)
gives `rule (c): control {control} changes the place, a pointer inside it or a pointer over it`,
`main`'s text after the prefix at `:630`, for any control that reaches the place, and the CTR-006
check finds no fault. Then run these one at a time:

```bash
cargo test --release -p conduit-core --lib -- own_tools_ listed_control_
cargo run --release -p conduit-conformance -- .. contracts-control-
cp target/packages.rs.fixed crates/core/src/packages.rs
cargo test --release -p conduit-core --lib -- own_tools_ listed_control_
cargo run --release -p conduit-conformance -- .. contracts-control-
```

The first must report 1 passed and 4 failed, and the second must fail with the line
`FAIL 0 cases passed; 1 failed`. After the restore, the third reports 5 passed and the fourth ends
`1 cases passed`.

Then show that the two new entries refuse what they name. Write `engine/target/purity-probe-177.rs`
with exactly these lines. The probe is compiled but never run:

```rust
#[allow(deprecated)]
pub fn purity_probe_177() {
    std::thread::sleep_ms(0);
    std::thread::park_timeout_ms(0);
}
```

Then, from `engine/`, run these one at a time:

```bash
cp crates/core/src/lib.rs target/lib.rs.fixed
cp crates/core/clippy.toml target/clippy.toml.fixed
cat target/lib.rs.fixed target/purity-probe-177.rs > crates/core/src/lib.rs
cargo clippy --color never -p conduit-core --all-targets -- -D warnings > target/probe-177.txt 2>&1; echo "exit $?"
grep -o -E 'disallowed method .?std::thread::(sleep_ms|park_timeout_ms)' target/probe-177.txt | sort -u | wc -l
grep -c 'the core starts no thread, sleeps for nothing' target/probe-177.txt
git show HEAD:engine/crates/core/clippy.toml > crates/core/clippy.toml
cargo clippy -p conduit-core --all-targets -- -D warnings
cp target/lib.rs.fixed crates/core/src/lib.rs
cp target/clippy.toml.fixed crates/core/clippy.toml
cargo clippy -p conduit-core --all-targets -- -D warnings
```

With the list, Clippy exits non-zero, the count of refused paths is 2 and the Threads reason is
printed at least once. With `main`'s list, the probe passes, so only the new entries refuse it.
After the restore, the last command passes. Restore with `cp`, not `mv`: a moved file keeps its
older modification time, so cargo would not recheck the crate. Git ignores `target/`. Report each
exit status and each count. After that, `git status --short` must list only the files in scope.

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared-changed; echo "exit $?"
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/check_docs.py
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/expected-failures.yaml engine/crates/shell/tests/expected spec
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/177-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`. The `git status`
command prints nothing. Then, from the repository root, these two print 1 and 0:

```bash
grep -c -F 'Fixed by [order 177]' docs/proposals/first-build/log-increment-4.md
grep -c '^| 4-36 .*| Nothing yet\. |' docs/proposals/first-build/log-increment-4.md
```

Do not stage anything. In your report, mark each choice you made yourself as your own, and give
the last line of every check, each exit status and count above, and the message of the
diagnostic each of the four rejection cases gets. With the decisions applied they are:

- `contracts-own-tools-optional`:
  `project: Type "tools-agent.v1" toolsDirectory /directory: rule (a): a branch of the schema does not list /directory under required`
- `contracts-own-tools-not-string`:
  `project: Type "tools-agent.v1" toolsDirectory /directory: rule (b): in a branch of the schema, the place admits number, where only string and null are allowed`
- `contracts-own-tools-control`:
  `project: Type "tools-agent.v1" toolsDirectory /tools/directory: rule (c): control #set-tools.v1 reaches the place: input "tools" sets "/tools", which lies over the place`
- `contracts-control-without-revision`:
  `project: Type "limited.v1" lists control #set-limit.v1: its inputSchema does not declare the input revision under properties; its inputSchema does not list revision under required`

If a message differs, report it and say why.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. On
the pull request, the admin shows the engine build, the four engine shards and the doctests
passing on Linux, and CI's `docs` job reads `177-declared.yaml` through `--declared-changed`. A
fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).
