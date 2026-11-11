# Work order 075 (S70): an Agent's own tools in the specification

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; list in `vectors` the one case this order adds. This is a
specification order: it changes no code. This order has network access, to fetch crates and for
tests that bind a loopback port; use it for nothing else. Never print a whole large file or
fixture: use `head`, `tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a credential.

## What and why

[Ruling 75](../proposals/ruling-75/README.md), point B1 (lines 39 to 42), settles how an Agent
gets its client's own tools: "An Agent has its client's own tools exactly when its template names
the directory it works in. An Agent that names none has `.v1`'s tools." There is no switch. Point
B2 (lines 43 to 46) puts every limit on that reach on the runtime provider, "chosen per Agent with
`runtime:`", and "its limits are not Agent inputs"; the first confining provider is built later.
[Ruling 70](../proposals/ruling-70/README.md), point 2 (lines 33 to 35), which ruling 75 leaves in
force: "In v0 the tools run as the engine's user, with the client's own sandbox and approval
prompts off. Nothing confines them. The Type's contract says so, and inspection shows an Agent's
own tools as reach that does not pass through Conduit (ruling 52)." Ruling 75 amends ruling 70's
points 1 and 3 and [ruling 74](../proposals/ruling-74/README.md)'s points 2 and 3 (ruling 75,
lines 106 to 110): own tools are no longer on by default, and the temporary switch is gone.

The specification says the opposite today. `spec/README.md` lines 82 to 84: "An Agent's own
means, its built-in tools, its shell and sandbox, its network, and the services of the account it
signs in with are outside the documents; inspecting an instance shows its reach through Conduit
only." The engine can show only what a Type declares. So this order writes:

- how a Type states that its Agents have their own tools, and which input turns them on: a Type
  member, `toolsDirectory`, that names the place in the configuration whose value, where it is a
  string, is the directory the tools work in, as `signIn` names `/account` under CTR-013;
- how `inspect.v1` shows that reach with its directory, on the Agent and one hop out, on whoever
  holds a Conduit that can start the Agent's model work
  ([ruling 61](../proposals/ruling-61/README.md), B13, line 29, for own tools only), and how a
  plan shows it;
- that this version reports the reach as neither enforced nor mediated, in ACC-027's terms,
  because it passes through no enforcement point, and that confining it is a runtime provider's
  (ruling 75, B2; ACC-027, PRV-006).

[Ruling 52](../proposals/ruling-52/README.md), point 2 (line 23), is the rule for every sentence:
"Be as explicit and descriptive as possible over every connection, since the callers are often
agents".
[IN_FORCE.md](../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach)
line 200 holds all of this as *spec pending, orders 050a and 050*.

**What waits on this order.** Orders 050a and 076 build `codex.v2` and `claude-code.v2` in
`packages/agent-codex` and `packages/agent-claude-code`, on a branch that merges with order 050
as the Agent packages 0.2.0 ([the batch 1 plan](../proposals/spec-batch-1/README.md), lines 622
to 625). Order 076 (`docs/work-orders/076-agents-v2-folder-grant.md`) removes 050a's input
`ownTools` and applies B1: a `.v2` Agent has its own tools exactly when its input `directory`
(`[string, 'null']`, default null) holds a string. Neither order's work is on `main`; do not look
for it. The `.v2` Types can state this order's member only once this order merges: `type-spec.v1`
has `additionalProperties: false` (`spec/schemas/contracts.yaml` line 182), so today the engine
rejects a Type that states it. After this order merges, the admin adds
`toolsDirectory: {directory: /directory}` to both `.v2` Types on that branch; that is not this
order's work. The member's name does not contain `ownTools`, for two reasons: ruling 75, B1
(line 39), says "`ownTools` is removed from the `.v2` Types before 0.2.0 publishes", and order
076's check `grep -rn "ownTools" packages/` (its line 80) must stay true after the admin adds
the member.
This order must merge before 0.2.0 publishes. NAM-024 (`spec/promises/naming.yaml` lines
414 to 440) lets a later release "add optional things" and forbids it to "remove or change
anything the earlier one held". Whether a member that declares reach is such an optional thing is
not settled, and the admin does not rely on it.

**A defect on the way.** `type-spec.v1` rejects `signIn` too. Order 038a put its shape under
`contract.v1`'s `$defs` (`spec/schemas/contracts.yaml` lines 107 to 112), which nothing
references, and `type-spec.v1` lists no `signIn`. The admin checked it at 342c5ba with the
validator of `tools/check_schemas.py`: a Type of family Agent that states
`signIn: {account: /account}` fails with "Additional properties are not allowed ('signIn' was
unexpected)". Order 050 gives the `.v2` Types `signIn` (the plan's C12, lines 315 to 318), so this
order fixes it beside the new member, and has the schema check CTR-013's three hooks as it
checks a Pool's `pool-management.v1` (step 2). One staged file states the opposite:
`docs/work-orders/036b-staged/038a.md` line 16 says "type-spec.v1 accepts signIn; CTR-013 carries
its cross-document constraints". Do not edit it; report it (step 8).

This order's size is about 0.3 to 0.5k lines. Line numbers are those of `main` at 342c5ba. Where a
line has moved since, find it by the words quoted.

Read, in this order:

1. The rulings: [ruling 75](../proposals/ruling-75/README.md), part B (lines 37 to 48) and "What
   it amends" (lines 103 to 110); [ruling 70](../proposals/ruling-70/README.md), whole;
   [ruling 74](../proposals/ruling-74/README.md), point 3 (lines 33 to 35);
   [ruling 61](../proposals/ruling-61/README.md), B13 (line 29);
   [ruling 52](../proposals/ruling-52/README.md), point 2 (line 23);
   [ruling 76](../proposals/ruling-76/README.md), point 1 (lines 20 to 25). Then IN_FORCE.md
   lines 198 to 201, and line 269.
2. The promises:
   - the Type: CTR-004 (`spec/promises/contracts.yaml` lines 92 to 126), CTR-006 (lines 155 to
     185) and CTR-013 (lines 281 to 299);
   - the place a pointer names: MAN-017 (`spec/promises/manifest.yaml` lines 363 to 395); PRV-016
     (`spec/promises/providers.yaml` lines 462 to 486), the form of a pointer that no control may
     change; and PRV-007 (line 197), under which "A place that holds null gives no variable and
     no argument";
   - NAM-024, and SCH-009 (`spec/promises/schemas.yaml` line 121), for the default null;
   - mediation and confinement: ACC-027 (`spec/conventions/access.yaml` lines 431 to 450) and
     PRV-006 (`spec/promises/providers.yaml` lines 165 to 195);
   - inspection: LIF-013 (`spec/conventions/lifecycle.yaml` lines 227 to 262), and ACC-033
     (`spec/conventions/access.yaml` line 575) and OPS-036 (`spec/conventions/operations.yaml`
     line 788) for who sees an inventory;
   - IFC-033 (`spec/interface/implementation.yaml` line 887), which puts a Type's description
     into the description of every tool whose target is one of the Type's instances.
3. The shapes: `contract.v1` and `type-spec.v1` (`spec/schemas/contracts.yaml` lines 58 to 121
   and 123 to 193), and that file's head comment (lines 1 to 56); `inspect.v1`
   (`spec/core/operations.yaml` lines 255 to 376); `inventory.v1` (`spec/core/results.yaml`
   lines 361 to 482). Then `spec/core/README.md` lines 348 to 351 and its table rows for
   `inspect.v1` (line 520), `type-spec.v1` (line 598) and `inventory.v1` (line 621); and in
   `spec/schemas/README.md` line 44 (the row of `contracts.yaml`), lines 169 to 214 (what the
   schemas cannot check) and lines 287 to 293 (sign-in declarations).
4. `spec/README.md` lines 74 to 86, and the plan paragraph, lines 348 to 357. In
   `spec/definitions.md`, the rows Agent (line 61), inventory (162), enforcement point (184),
   mediated (193), enforced (194), model work (223), runtime provider (308) and isolation plugin
   (430).
5. [The batch 1 plan](../proposals/spec-batch-1/README.md): C12 (lines 315 to 322) and order 050
   (lines 622 to 625). Order 076, steps 1 and 2.
6. How a `load` case is written: `conformance/README.md`, "Inputs and files" (line 23);
   `conformance/language/loader/package-pin/case.yaml`;
   `conformance/language/registration/runtime-wrong-family/case.yaml`.
7. [CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises" (line 120), and `spec/README.md`
   lines 302 to 305: "Specification files carry no change narrative; the reasons are in the
   records".

## What to do

Write each promise in the specification's form and terms. Cite no ruling inside a promise, a
schema description or a definition: none under `spec/promises/`, `spec/conventions/` or
`spec/interface/` cites one today. The rulings this order cites are for you; their reasons go in
step 5's "Where the reasons are" line.

1. **The Type member: CTR-014** (ruling 75, B1; ruling 70, point 2). Add CTR-014 to
   `spec/promises/contracts.yaml`, after CTR-013, with layer `language` and level `must`. Apply
   these decisions. Do not reopen them.
   - **Form.** A Type states `toolsDirectory: {directory: <JSON Pointer into configuration>}`,
     with exactly that one member, as `signIn` holds `{account: ...}`. `codex.v2` will state
     `toolsDirectory: {directory: /directory}`. Give the Type member no name that contains
     `ownTools`: ruling 75, B1, removes `ownTools` from the `.v2` Types. The inventory member of
     step 3 keeps the name `ownTools`, since the ruling speaks of the Types.
   - **Family.** Only a Type of family Agent states it.
   - **The place.** The place and every member on its path are required, and the place is
     confined to string or null by MAN-017's test, as CTR-013's account place is.
   - **Fixed for the Agent's life.** The Type lists no control whose `changes` holds the pointer,
     a pointer inside it or a pointer over it, compared as CTR-006 compares pointers. This is
     PRV-016's form. It is the admin's reading of B1's "its template names the directory": a
     control would change the reach after the template. Report it as the admin's reading.
   - **Rejection.** A Type that breaks any of these rules is rejected when its package is
     installed or its project is submitted, with a diagnostic naming the Type and the cause, and
     the control where one is the cause.
   - **What the value means**, in the observable:
     - a string: the Agent has its client's own tools, as the Type's contract states them, and
       they work in the directory the string names. Their reach does not pass through Conduit:
       no Conduit grants it, no restriction limits it, and no record shows its use. This version
       reports it as neither enforced nor mediated, whatever the Agent's runtime provider
       (ACC-027, PRV-006). On the engine's own runner nothing confines it. Every string counts,
       the empty string among them: what an implementation does with a directory it cannot use
       is the Type's contract's, and inspection errs toward showing reach. This is the admin's
       reading; report it as one;
     - null: the Agent has none of the own tools that a string turns on, and has only what its
       Type's contract states for that case. Do not write that it has no tools, or no reach
       outside Conduit: ruling 75, B1, gives an Agent that names no directory "`.v1`'s tools",
       and `.v1` keeps some of its client's tools (`engine/CONFORMANCE.md` lines 187 and 188,
       and 203 and 204). A template that writes nothing there gets null where the Type's schema
       gives that default (SCH-009);
     - the engine reads the value only to report it (step 3). It checks neither statement, nor
       that the directory exists. The implementation receives the value as any other
       configuration value, through its item's binding (MAN-017), and a null gives it no
       variable and no argument (PRV-007).
   - **Confinement is a reporting rule.** Write it as ACC-027 does: "This version reports
     mediated for every restriction, whatever the consumer's runtime provider is"
     (`spec/conventions/access.yaml` lines 448 and 449). Never write that nothing confines an
     Agent's own tools whatever its runtime provider: PRV-006 lets an instance be given a
     runtime provider whose contract confines what it runs (`spec/promises/providers.yaml` lines
     190 to 193). The member declares no limit on the reach. The limits, such as the hosts
     allowed or a directory mounted read-only, are the runtime provider's (ruling 75, B2; step 4).
   - **The Type's description.** It states what the tools are and what they reach when they are
     on. IFC-033 carries that description into the description of every tool whose target is the
     Agent. The wording is the package's duty; the engine does not check it.
   - **A Type that states no `toolsDirectory`** declares nothing about such reach, and inspection
     says so (step 3). `codex.v1` and `claude-code.v1` stay as they are (ruling 70, "What it
     amends").
   - **CTR-004.** Its sentence "A Type may also state signIn ..." (lines 124 and 125) also names
     `toolsDirectory`, of CTR-014.
   - **Vectors.** CTR-013 and CTR-014 keep `vectors: []`, the form that CONTRIBUTING.md, "Writing
     promises", rule 1, gives. The case of step 6 is tied to them by the coverage index.

2. **The schema** (`spec/schemas/contracts.yaml`; ruling 75, B1, and CTR-013). Apply these
   decisions.
   - `type-spec.v1` gains two properties, after `records`, in this shape:

     ```yaml
         signIn:
           type: object
           properties:
             account: {$ref: github.com/shrav-k/conduit/core#pointer.v1}
           required: [account]
           additionalProperties: false
         toolsDirectory:
           type: object
           properties:
             directory: {$ref: github.com/shrav-k/conduit/core#pointer.v1}
           required: [directory]
           additionalProperties: false
     ```

     `signIn`'s shape is the one now at `contract.v1`'s `$defs/signIn` (lines 107 to 112).
     Remove that definition, which nothing references. The removal changes no verdict, but it
     is a change to the core at 0.1.0 that is not an addition: the batch 1 plan's reading 18
     (line 183) records such changes as exceptions. Report it as one.
   - Two rules go into `type-spec.v1` as one `dependentSchemas` at its root, between
     `additionalProperties: false` (line 182) and the existing `if` (line 183). Do not
     restructure the `if` and `then` (lines 183 to 193). The `signIn` rule is CTR-013's three
     hooks, stated as the Pool rule states `pool-management.v1`, with `contains` and `const`:

     ```yaml
       dependentSchemas:
         toolsDirectory:
           properties:
             family:
               const: Agent
         signIn:
           required:
           - requires
           properties:
             requires:
               allOf:
               - contains:
                   const: github.com/shrav-k/conduit/core#sign-in-hook.v1
               - contains:
                   const: github.com/shrav-k/conduit/core#account-hook.v1
               - contains:
                   const: github.com/shrav-k/conduit/core#sign-out-hook.v1
     ```

     A qualified reference never carries a version (`qualified-reference.v1`), and a Type
     outside the core package can select the core's hooks only by it, so the `const` matches
     every way such a Type can list them. The profile admits `dependentSchemas` and `contains`
     (`spec/schema-profile.md` line 70), and `manifest.v1` already uses `dependentSchemas`
     (`spec/schemas/manifest.yaml` line 192). The loader walks it
     (`engine/crates/loader/src/schemas.rs` line 347) and applies it through jsonschema 0.61
     (`engine/crates/loader/Cargo.toml` line 12). The admin checked this shape at 342c5ba with
     the validator of `tools/check_schemas.py`, which then reported "Errors: 0", and with the
     conformance runner's strict loader: an Agent Type with `toolsDirectory`, an Agent Type with
     `signIn` and the three hooks, and a Resource Type with `signIn` and the three hooks are
     accepted; `toolsDirectory` on a Resource or a Pool Type, empty, or with a second member, is
     rejected, and so is `signIn` with two of the hooks or with no `requires`.
   - The description of `type-spec.v1` names both members and the two rules.
   - The file's head comment:
     - under "Promises encoded" (lines 8 to 11), add to `type-spec.v1` CTR-013 (`signIn` holds
       exactly `account`, a pointer; a Type that states it lists the three hooks under
       `requires`) and CTR-014 (`toolsDirectory` holds exactly `directory`, a pointer; only a
       Type of family Agent states it);
     - under "Not expressed here, and checked by the engine" (lines 40 to 51), add the other
       rules of CTR-013 and CTR-014: that the place each pointer names, and every member on its
       path, is required by the Type's `schema` and confined to string or null, which relates
       values in one document; and that no control of a Type that states `toolsDirectory` has
       `changes` that reach its place (CTR-014), which needs the contracts of the controls.
   - Every Type that the engine accepts today stays accepted. `cargo test --release` shows it.
   - Add no example under `spec/schemas/examples/`. The loader's tests compare its verdict on
     each example with `engine/crates/loader/tests/expected/invalid-verdicts.txt`, under
     `engine/`, which this order does not change.

3. **Inspection: LIF-013, `inspect.v1` and `inventory.v1`** (ruling 70, point 2; ruling 61, B13;
   ruling 52, point 2). Apply these decisions.
   - **On the Agent.** `inventory.v1` gains `ownTools`. It is present for every Agent, and for no
     other instance:
     - `stated`: whether the Agent's Type states `toolsDirectory`;
     - `pointer`: present exactly when `stated` is true. It is the Type's pointer;
     - `directoryNamed`: present exactly when `stated` is true. It is true where the value at the
       place, in the resolved configuration after SCH-009 has filled defaults, is a string, and
       false where it is null. It reports what the template wrote, from which the reach follows;
       it is not a switch, and nothing sets it;
     - `directory`: present exactly when `directoryNamed` is true. It is that string;
     - `enforcement`: present exactly when `directoryNamed` is true, and then `neither`, its only
       value in this version. It answers ACC-027's question for this reach: the reach passes
       through no enforcement point, so it is neither enforced nor mediated. It is reported so
       whatever the Agent's runtime provider, as step 1 says.

     Where `directoryNamed` is false, LIF-013 says what step 1 says of null: the Agent has none
     of the own tools that a string turns on, and has only what its Type's contract states for
     that case, which inspection does not list. It does not say that the Agent reaches nothing
     outside Conduit.

     That the member is present for every Agent, with `stated: false` where the Type states
     nothing, is the admin's reading of ruling 52, point 2: a caller then reads that the Type
     declares nothing, and does not have to infer it from an absent member. Report it as the
     admin's reading.
   - **No runtime member yet.** Add no member that names the runtime provider. Inspecting the
     Agent already shows its `runtime` in `declared` (PRV-006). A member that names a confining
     provider comes with the first such provider (ruling 75, B2), and `inventory.v1` already
     lets additional members extend its shape.
   - **One hop.** This applies ruling 61's B13 to own tools only. An entry of `consumes` carries
     `targetOwnTools` when its Conduit is open, its target is an Agent, and its Conduit lists an
     operation whose contract is marked `startsModelWork: true`. `targetOwnTools` is the target's
     `ownTools` member, as above. Whoever holds such a Conduit reaches, through the target's
     model work, whatever the target's own tools reach. It goes one hop only: it does not show
     what the target reaches through its own Conduits. Batch 2 writes the rest of B13
     (IN_FORCE.md line 201).
   - **Root too.** `targetOwnTools` is shown whoever the consumer is, root included. B13 names
     "each non-root turn-holder". Showing it for root as well is the admin's reading: root's
     requests start the target's model work as anyone's do, and ruling 52, point 2, asks that
     every connection be shown. Report it as the admin's reading.
   - **Who sees it.** Whoever may inspect the Agent, or the holder, sees it with the rest of the
     inventory (LIF-013, ACC-033). `targetOwnTools` shows whoever may inspect the holder one value
     of the target's configuration, the value at the target Type's pointer, as `directoryNamed`
     and `directory`, and that pointer. It discloses no other part of the target's configuration.
     That an inspector of the holder may learn the target's directory is the admin's reading:
     under B1 naming the directory is what turns the reach on, and the holder's reach cannot be
     shown without it. Report it as the admin's reading. State in LIF-013, in the
     specification's terms, what the member lets an inspector learn.
   - **The shape.** Define the shape once inside `inventory.v1`, under a local `$defs`, as
     `contract.v1` does, and refer to it from both places. The loader rewrites local pointers
     (`engine/crates/loader/src/schemas.rs` line 294). Add no Schema contract, so that the core
     package's export counts do not change. `stated` is required; the other members are optional
     in the schema, and LIF-013 says when each is present. Leave the shape open, as
     `inventory.v1` is.
   - **Where to write it:**
     - LIF-013;
     - the description of `inspect.v1`, at the end of its second paragraph
       (`spec/core/operations.yaml` lines 286 to 288). Order 038b rewrites line 276, in the first
       paragraph: leave that paragraph and the blank line after it as they are;
     - the description of `inventory.v1` (`spec/core/results.yaml` lines 365 to 375);
     - `spec/core/README.md` lines 348 to 351, and its table rows for `inspect.v1` (line 520),
       `type-spec.v1` (line 598) and `inventory.v1` (line 621).
   - **The names.** Use the key names above unless a check or a promise forbids one. Report any
     name you change, and why.

4. **ACC-027 and PRV-006** (ruling 70, points 2 and 4; ruling 75, B2).
   - **ACC-027.** Keep "This version reports mediated for every restriction". Add that an Agent's
     own tools are no restriction on a Conduit: inspection reports their reach apart, as neither
     enforced nor mediated (LIF-013, CTR-014). An Agent whose own tools are on, and whose runtime
     provider does not confine it, as the engine's own runner does not, is a consumer "that is
     not confined", in this promise's words: it may reach by its own means what a target's
     implementation reaches.
   - **PRV-006.** Add that a runtime provider whose contract confines what it runs is also the
     route by which an Agent's own tools are confined. The limits of that confinement, such as the
     hosts it allows or a directory it mounts read-only, are that provider's contract and
     configuration; CTR-014 declares none of them. This version reports the reach of an Agent's
     own tools as neither enforced nor mediated, whatever the runtime provider (CTR-014,
     LIF-013).
   - Change no other promise of `spec/conventions/access.yaml` or
     `spec/promises/providers.yaml`.

5. **The rest of `spec/`.**
   - **`spec/README.md` lines 82 to 84** (ruling 70, point 2). Replace "inspecting an instance
     shows its reach through Conduit only" with what CTR-014 and LIF-013 now say: an Agent's own
     tools are declared by its Type, and inspection shows their reach apart from its reach
     through Conduit. The services of the account an Agent signs in with stay outside the
     documents. Keep the sentence on confinement (lines 85 and 86), and add that this version
     reports the reach of own tools as neither enforced nor mediated.
   - **The plan paragraph** (`spec/README.md` lines 348 to 357; ruling 61, B13). Add that a plan
     prints, beside each Agent it registers or replaces, the `ownTools` member that inspection
     would show. Beside each Conduit it opens whose target is an Agent and that lists an operation
     marked `startsModelWork: true`, it prints the target's. This section holds statements, not
     promises; keep it so. Add these as sentences at the end of the paragraph, and change none of
     its other sentences: ruling 76 changes that paragraph's flags, which IN_FORCE.md line 79
     holds as spec pending, and a later order rewrites them.
   - **Not decided here:** whether a plan counts naming a directory as a widening that needs a
     named flag. Ruling 76, point 1, keeps the named flags for widening and does not say whether
     this is one. Write nothing on it, and report it as a question for the admin.
   - **Counts in `spec/README.md`:** the Contents row of `promises/contracts.yaml` (line 204), its
     promises and the promises its vectors test slices of, and the totals (lines 250 and 251).
     The slices are counted from the promises the coverage index lists. Recount each from the
     files, and any other count you change. Order 038b changes the same two lines of totals;
     the admin recounts them when the second of the two orders merges.
   - **"Where the reasons are"** in `spec/README.md` (line 610): add one line for rulings 70, 74
     and 75, with ruling 75's part B. Name there, as the reasons for the inventory members,
     ruling 61's B13 and ruling 52's point 2.
   - **`spec/definitions.md`:** add a row for "own tools", directly after the row "enforced"
     (line 194), away from order 038b's edits. Amend the inventory row (line 162). Amend
     enforcement point (184), mediated (193), enforced (194), runtime provider (308) and
     isolation plugin (430) only where they must agree.
   - **`spec/schemas/README.md`:**
     - line 44, the row of `contracts.yaml`: in its third column, say that `type-spec.v1` holds
       `signIn` and `toolsDirectory`. Leave the second column as it is: `tools/check_schemas.py`
       checks that it lists exactly the Schema contracts the file holds;
     - make "Sign-in declarations" (lines 287 to 293) true of the schema: the schema checks the
       closed shape and the three hooks under `requires`; the engine checks that the place is
       required and confined to string or null, rules that relate values in one document, the
       pointer and the Type's own `schema`. Do not call these rules cross-document. Give
       `toolsDirectory` a paragraph beside it;
     - add one row to the table of what the schemas cannot check (lines 169 to 214), directly
       after the row of SEC-005 and PRV-016 (line 203), for the rules of CTR-013 and CTR-014 that
       no schema states: the place and every member on its path required by the Type's `schema`,
       and the place confined to string or null (CTR-013, CTR-014); and no control whose
       `changes` reaches the place (CTR-014). Its "Why a schema cannot" column says that the first
       two relate values in one document, the pointer and the Type's own `schema`, and that the
       third needs the contracts of the controls. Its "Tool" column is "—":
       `tools/check_schemas.py` checks none of them. The three hooks are not in this row, since
       the schema states them.

6. **Vectors.** The runner can check today only what the strict loader decides: a `load` step
   validates each document against the schemas, without registration. Write one case,
   `loader-own-tools`, at `conformance/language/loader/own-tools/case.yaml`, in its neighbours'
   form, with layer `language` and the promises CTR-014, CTR-013 and SUB-005. Give it one file per
   step, each holding one Type contract, and these `load` steps:
   - an Agent Type whose schema requires `directory` as `[string, 'null']` with default null, and
     which states `toolsDirectory: {directory: /directory}`: accepted;
   - the same Type with family Resource: rejected, with a SUB-005 diagnostic;
   - an Agent Type whose `toolsDirectory` is empty: rejected, with a SUB-005 diagnostic;
   - an Agent Type whose `toolsDirectory` holds a second member, such as `hosts`: rejected, with a
     SUB-005 diagnostic. B2 puts such limits on the runtime provider;
   - an Agent Type that states `signIn: {account: /account}`, whose schema requires `account` as
     `[string, 'null']`, and which requires the three hooks of CTR-013: accepted;
   - the same Type with `sign-out-hook.v1` left out of `requires`: rejected, with a SUB-005
     diagnostic.

   Each accepted document meets every rule of its promise, so the case stays true once the build
   adds the checks the loader lacks. Write each expectation from the specification, and check it
   against what the loader reports. The admin ran a draft of these six steps with the
   conformance runner at 342c5ba: the loader accepted the first and the fifth, and located the
   four rejections at `/spec/family`, `/spec/toolsDirectory`, `/spec/toolsDirectory` and
   `/spec/requires`.

   List the case in `conformance/manifest.yaml`:
   - beside the `loader-*` entries, after `loader-package-pin-document` (lines 145 to 154). Order
     041b appends its entries at the end of `cases`; an entry there would conflict with it;
   - with the same promises as the case, in the same order: the runner compares them;
   - with `safety_rows: ["loader-own-tools"]`, as each neighbouring `loader-*` entry carries its
     own ID. These are group tags, not safety tags (IN_FORCE.md line 262).

   Order 071 moves the entries of `cases` into pieces. If your tree has it,
   `conformance/manifest.yaml` has no `cases`: put the entry in
   `conformance/language/loader/index.yaml` instead, sorted by `id`, as `conformance/README.md`
   then says.

   Declare the case in `docs/work-orders/075-declared.yaml`, as one shape of kind `added`. Change
   no existing case or fixture.

   No other vector is possible yet. Say why in your report, for each:
   - CTR-014's rules on the place and on controls need code in the engine, at install and at
     submission. None is built.
   - The inventory members need the inventory, which order 042 builds, and code that reads the
     Type's pointer.
   - Plans are built by order 052a.

   For each, list the vectors a build order would add: an ID, its promises, and what it shows.

7. **IN_FORCE.md line 200.** Write the part this order lands as an **In spec/** line, in that
   file's form, as line 188 does for sign-in. It points to CTR-014, LIF-013, ACC-027 and PRV-006
   (70 point 2; 61 B13, for own tools, one hop; 75 B1, B2). State confinement there as step 1
   does. What stays spec pending, for orders 050a, 076 and 050: the `.v2` Types themselves, with
   their `directory` input, their statement of the member and their descriptions, and the
   client's sandbox and approval prompts off in v0. Keep the sentence that the first confining
   provider comes later, through `runtime:`, with the limits in its own configuration. Leave
   lines 198, 199 and 201 as they are.

8. **Text for later orders and for the admin, in your report only.** Put each in a finding of
   class `gap`. In its `what_the_build_did`, write that this order builds nothing.
   - **For order 050:** the sentences that replace two paragraphs of `engine/CONFORMANCE.md`,
     I3-04 (lines 186 to 205: "The launch disables the client's shell ..." and "All
     restrictions are mediated (ACC-027) ..."). Cover `.v2` with a directory named and with none.
     Do not edit `engine/`.
   - **For the CLI reference:** the line that `docs/implementation/plan-and-apply/README.md`
     needs for step 5's plan sentences. Order 069 wrote that file and order 069b revises it
     under ruling 76; do not edit it.
   - **For a build order:** CTR-014's checks at install and at submission, and the inventory
     members, with the vectors of step 6. Say whether order 042, which builds the inventory,
     should build them or a later order.
   - **For the admin:**
     - whether `accountServices` and `peerMessaging` should be declared the same way. They also
       reach outside Conduit, through the client's login and its peers. Whether NAM-024 lets a
       later release add such a member is not settled: give your reading, marked as yours. Do not
       declare them;
     - `docs/work-orders/036b-staged/038a.md` line 16, "type-spec.v1 accepts signIn; CTR-013
       carries its cross-document constraints": the first half was false before this order, and
       the second is not true after it;
     - `docs/implementation/SAFETY_CATALOGUE.md` classes every promise of `spec/`, and CTR-014 is
       new: give the class you would give it, by the catalogue's own definition;
     - the batch 1 plan, line 322, still describes `ownTools` as an input with default true;
     - step 5's question on widening.

9. **Before you finish, check:**
   - no text still says that inspection shows reach through Conduit only. Search `spec/` and
     `IN_FORCE.md` for "through Conduit only", "own means" and "outside the documents";
   - no text you wrote says that nothing confines an Agent's own tools beyond the engine's own
     runner, and none makes the directory a bound on their reach in this version. Search what you
     changed for "confine" and "directory";
   - no text you wrote describes a switch that turns own tools on or off. Search what you
     changed for "switch", "default true", "turn off" and "enabled";
   - `ownTools` names only the inventory member, and is part of `targetOwnTools`; no Type
     member and no contract schema uses it;
   - no text you wrote says that an Agent whose Type's place holds null has no tools, or reaches
     nothing outside Conduit;
   - no promise, schema description or definition you wrote or amended cites a ruling;
   - one wording throughout: "own tools", and "neither enforced nor mediated";
   - every count you changed, recounted from the files;
   - no trailing white space; no absolute local path, user name or credential in any file.

## Scope

Change:

- `spec/promises/contracts.yaml`: CTR-014, which is new, and CTR-004;
- `spec/schemas/contracts.yaml`: `type-spec.v1`, `contract.v1`'s `$defs/signIn`, which goes, and
  the head comment's "Promises encoded" and "Not expressed here, and checked by the engine";
- `spec/core/results.yaml`: `inventory.v1`;
- `spec/core/operations.yaml`: the end of the second paragraph of `inspect.v1`'s description only;
- LIF-013 in `spec/conventions/lifecycle.yaml`, ACC-027 in `spec/conventions/access.yaml`, and
  PRV-006 in `spec/promises/providers.yaml`;
- `spec/README.md`, `spec/definitions.md`, `spec/core/README.md` and `spec/schemas/README.md`, as
  step 5 says;
- `conformance/language/loader/own-tools/`, which is new, and its entry in
  `conformance/manifest.yaml`, or in `conformance/language/loader/index.yaml` if order 071 is in
  your tree;
- `docs/work-orders/075-declared.yaml`, which is new;
- `IN_FORCE.md`, line 200.

Do not change:

- `engine/`, `packages/` or `tools/`;
- `spec/names.yaml`, or anything under `spec/schemas/examples/`;
- any other promise, CTR-013 among them; any existing case or fixture;
  `conformance/s1-promises.yaml`; or the `s1_coverage` rows of `conformance/manifest.yaml`;
- `docs/implementation/`, the CLI reference and the safety catalogue among it, or
  `docs/OVERVIEW.md`. Report what any of them needs;
- any other line of `IN_FORCE.md`;
- any other file under `docs/work-orders/`, `036b-staged/038a.md` among them;
- a record: the rulings' records, the ledger, `docs/AUDIT.md`, `ROADMAP.md`, `docs/proposals/`
  and `docs/work-orders/README.md`.

Report anything else that needs changing.

Orders in flight edit the same files. Keep your edits small and local, so that either order
rebases easily. Line numbers are those of `main` at 342c5ba.

- Order 038b edits:
  - CTR-009 in `spec/promises/contracts.yaml` (lines 234 and 235), and ACC-032 in
    `spec/conventions/access.yaml` (line 572);
  - `spec/promises/providers.yaml`: its nearest hunk before PRV-006 ends at line 162;
  - `spec/core/results.yaml`: its head comment, and lines 521 to 529, outside `inventory.v1`;
  - `spec/core/operations.yaml`, among others line 276, in the first paragraph of
    `inspect.v1`'s description. Step 3 keeps your text at the end of the second;
  - `spec/README.md` lines 61, 216 to 221, 250 and 251, and 420 to 424. Lines 250 and 251 are
    the totals, which this order changes too: the admin recounts them at merge (step 5);
  - `spec/definitions.md` lines 48, 82, 124 to 128, 149 and 150, 269, 314 to 319, 359 and 425.
    At lines 314 to 317 it rewrites four rows and adds six before them, six lines after
    "runtime provider" (line 308); amend that row only if it must agree, and change no other
    line near it. Step 5 puts the new row after "enforced" (line 194), away from these;
  - `spec/core/README.md` lines 21 to 154, 363 and 364, 473 to 498, 555 to 561 and 637;
  - `IN_FORCE.md` lines 184, 216 and 221 to 227.
- Order 041b appends entries at the end of `conformance/manifest.yaml`'s `cases`, and adds rows
  to `s1_coverage`. Step 6 keeps your entry away from both.
- Order 071 moves the entries of `cases` into pieces. Step 6 says where your entry goes if it is
  in your tree.

## Done when

Every item of steps 1 to 9 is written, or its absence is reported with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py --declared docs/work-orders/075-declared.yaml`, with that
  interpreter: the one added case, and no flip, undeclared change or error
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `git diff --check`

Each command in the block below prints nothing. The first two cover the files that
`git diff --check` does not see, because they are not tracked; the third shows that no promise
text cites a ruling; the fourth, that no contract schema and no document of the case names
`ownTools`.

```bash
grep -rn -E '[[:space:]]+$' conformance/language/loader/own-tools docs/work-orders/075-declared.yaml
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/' conformance/language/loader/own-tools docs/work-orders/075-declared.yaml
grep -n -i 'ruling' spec/promises/contracts.yaml spec/promises/providers.yaml spec/conventions/access.yaml spec/conventions/lifecycle.yaml spec/core/results.yaml spec/core/operations.yaml spec/schemas/contracts.yaml | grep -v -E '^[^:]+:[0-9]+:#'
grep -rn 'ownTools' spec/schemas/contracts.yaml conformance/language/loader/own-tools
```

`git status --porcelain` prints these lines and no other; report any other line and why. If order
071 is in your tree, `conformance/language/loader/index.yaml` stands in place of
`conformance/manifest.yaml`.

```text
 M IN_FORCE.md
 M conformance/manifest.yaml
 M spec/README.md
 M spec/conventions/access.yaml
 M spec/conventions/lifecycle.yaml
 M spec/core/README.md
 M spec/core/operations.yaml
 M spec/core/results.yaml
 M spec/definitions.md
 M spec/promises/contracts.yaml
 M spec/promises/providers.yaml
 M spec/schemas/README.md
 M spec/schemas/contracts.yaml
?? conformance/language/loader/own-tools/
?? docs/work-orders/075-declared.yaml
```

In `engine/`, because its tests read `spec/` and `conformance/`:

- While you work, run `cargo test --release -p conduit-loader -p conduit-core -p conduit-conformance`
  as often as you need. These crates read `spec/core/`, `spec/schemas/` and `conformance/`.
- Once the text is final, run `cargo test --release` once.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers
only. If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load
(finding 4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail
in your sandbox: say so, name each, and the admin reruns them.

Report:

- the promises added and amended, by ID;
- the key names as written, in the Type (`toolsDirectory`) and in `inventory.v1` (`ownTools`,
  `targetOwnTools` and their members);
- the removal of `contract.v1`'s `$defs/signIn`, as a change to the core at 0.1.0 that is not an
  addition (step 2);
- the case, in `vectors`, and whether it passes;
- every reading you applied. Mark as the admin's the five readings this order names: the place
  fixed for the Agent's life, and every string counting as a directory named (step 1);
  `ownTools` for every Agent, with `stated: false` where the Type states nothing;
  `targetOwnTools` for root too; and its disclosure of the target's directory (step 3);
- the text of step 8;
- each check's last line.

Do not `git add` or commit anything.
