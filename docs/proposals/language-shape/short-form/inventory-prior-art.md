Not binding. The [specification](../../../../spec/README.md) alone binds. A working paper of the investigation of 2026-11-10, kept as it was returned, with local paths removed; the [proposal](../README.md) is its summary.

# Making the short form the language: how normative concise forms are defined by translating them into a core, and what that means for Conduit

## Key points
1. Defining a concise form by translating it into a core is common and normative in practice: Haskell's kernel, Scheme's derived expressions (formal semantics on primitives only), WebAssembly's text abbreviations ("rewrite rules... before applying the core grammar rules"), Dhall's record completion (`T::r` = `(T.default ⫽ r) : T.Type`), Rust's `for` loops. The core stays small because every rule of the core is stated over the core alone.
2. An option the first investigation did not cost: make the short form normative by translating it into today's canonical documents, with no `revise.v1`, `swap.v1` or restart in place. From design C's own cost verdict, about half of its roughly 80 promises came from revise and swap. Expansion alone is roughly 30 to 40 promise touches (my estimate), against 11 to 14 for the layered route.
3. No surveyed system makes the short form change deployed objects implicitly. CloudFormation runs the SAM transform, then builds a change set from the processed template, which is executed as a separate step. Terraform writes identity changes as `moved`/`removed` blocks and keeps them forever. SpiceDB refuses schema changes that would orphan data. So OBJ-002 and SUB-007 can stay, with change made of today's requests, `expectedId` and tombstones.
4. Thesis 2 for what authors write requires the translation rules themselves to be promises, tested by input/expected pairs (Dhall's A/B acceptance tests, SAM's translator input/output templates) plus an equivalence property: submitting the rendered expansion gives the same objects as submitting the short form. Haskell's own `let` exception shows that a translation identity can be wrong, so each rule needs vectors.
5. Name one authority (the translation) and allow an as-if rule. Haskell: "it is not expected that it will use them directly". Rust shows the risk of two descriptions: `?` is defined per type, and its translation is only "roughly equivalent".
6. Defaults must depend only on the text and the version it declares, and be written into the stored object with provenance. Precedents: Kubernetes static defaults are per API version and "stored... Subsequent GETs... include the default values explicitly"; Rust editions are "skin deep"; SAM stamps `Metadata: aws:sam:connectors`. Anti-precedent: GitHub Actions' default token permissions come from organization settings and changed in 2023. This conflicts with INS-013's observable.
7. Generated names: deterministic, derived from the author's keys, under a reserved separator, and bound to core entities whatever names are in scope (Haskell's Prelude rule, Rust's lang items). Never hash content into them: SAM warns its hashed logical IDs "might change".
8. Diagnostics must speak in the author's text. Validate the authored form against a schema of that form (Compose's oneOf short|long), keep the original beside the expansion (GHC's HsExpansion), tag generated code (rustc's DesugaringKind), and name every contributing location (Nix, CUE, HCL Subject/Context ranges). SAM's "Resource with id [X]" is the floor. SUB-005's JSON Pointer rule should extend to point into the short-form document.
9. The long form stays legal inside the short form, and round-tripping is not required. Compose implementations "must allow use of both short and long syntaxes within the same document", and SAM templates carry plain CloudFormation resources. ALFA's lossless round-trip with XACML complicated its grammar and its successor dropped the requirement. This keeps ruling 4's scope guard for S2 to S10.
10. Access shorthands must fail closed and never group operations beyond the pinned contract. GitHub Actions sets unspecified permissions to none once any is given. SAM Connectors' Read/Write expand from a profiles file the translator owns, which shows the risk. The expansion writes the full operation list, as SAM writes the full action list into the policy.
11. Fields that carry identity are immutable in practice: Verified Permissions forbids changing a policy's effect, principal or resource; Kubernetes' roleRef is immutable; SpiceDB uses preconditions on writes. Identity changes are explicit statements, not inferences.
12. Normative status and where the expansion runs are separate choices. SpiceDB, CloudFormation and Compose accept the short form in the engine; OpenFGA compiles in clients. Kubernetes never made a short form normative; generators multiplied and kustomize was later folded into kubectl without a specification, which is the long-run path of the layered approach.
13. Specific hazards in Conduit's spec for a normative short form:
   - SUB-008 takes the contracts digest over the manifest as handed over, so keep the short form out of the manifest;
   - SUB-005 locations;
   - INS-013, which forbids marking derived values;
   - CON-016, if `capability` may be inferred;
   - OBJ-003 and NAM-027 for generated names;
   - MAN-001, if a lock file is added.
14. A normative short form in spec/ also answers critic point 3: under ruling 48 the fresh author would learn the format from spec/ alone, so decision 9's separate format reference is no longer needed.

## Findings
## 0. Bottom line

The sources support defining a concise form normatively by translating it into a core. Languages and configuration systems do this often, and the core stays small and testable. That holds only under five conditions:

1. Every rule of the core is stated over the core alone.
2. The translation is total and deterministic, and its rules are themselves normative.
3. The translation depends only on the author's text and a version the text declares. It never depends on the caller, on history or on the environment.
4. Diagnostics speak in the author's text.
5. The core form stays writable beside the short form.

No precedent surveyed makes the short form also change deployed objects in place. Change is always computed separately: the expansion is diffed against what exists, and the result is executed only on an explicit step (a CloudFormation change set, a Terraform plan), or the system refuses the change (SpiceDB). For Conduit this means a third option the first investigation did not cost:

- **Normative expansion only.** The short form is defined in spec/ by translation into today's canonical documents. Objects stay immutable (OBJ-002, SUB-007). Change is made of today's requests, through the accepted plan/apply, `expectedId` and tombstones.
- **How it differs from the costed options.** It is not design E, whose CLI defines the meaning. It is not designs C and D, which bundled `revise.v1`, `swap.v1` and restart in place.
- **What it would cost, roughly.** Reading design C's own cost verdict (docs/proposals/simplification/research/design-C-agent-centric.md:710-800), about half of C's roughly 80 promises came from revise and swap: about 11 new and 23 amended. The project-file side was about 12 new and about 15 amended, plus about 8 of the 17 the design named itself. So expansion-only is roughly 30 to 40 promises touched, against 11 to 14 for the layered route and 50 to 80 for C and D.
- **Status of that estimate.** This is my estimate from C's verdict, not a recount.

## 1. Languages: a concise form defined by translation into a core

### Haskell 2010 Report

- **What is normative.** The Report. Section 1.2 says: "the meaning of such syntactic sugar is given by translation into simpler constructs. If these translations are applied exhaustively, the result is a program written in a small subset of Haskell that we call the Haskell kernel." It adds: "Although the kernel is not formally specified, it is essentially a slightly sugared variant of the lambda calculus." Source: https://www.haskell.org/onlinereport/haskell2010/haskellch1.html
- **How the short form's meaning is specified.** By identities given where each construct is introduced. For `do` (3.14): `do {e} = e`; `do {e;stmts} = e >> do {stmts}`; `do {p <- e; stmts} = let ok p = do {stmts}; ok _ = fail "..." in e >>= ok`. List comprehensions are in 3.11. Chapter 3 says:
  - "Except in the case of let expressions, these translations preserve both the static and dynamic semantics."
  - "Free variables and constructors used in these translations always refer to entities defined by the Prelude."
  - On implementation (3.17.3): "it is not expected that it will use them directly." This is an as-if rule.
  - Source: https://www.haskell.org/onlinereport/haskell2010/haskellch3.html
- **Lesson from the `let` exception.** The translation fails to preserve static semantics there: using `case` "precludes a fully polymorphic typing" (3.12). A translation identity can be subtly wrong, so each rule needs tests.
- **Errors.**
  - GHC "type checks programs in their original Haskell form before the desugarer converts them into Core code", because "it improves error messages, as those message are based on the same structure that the user sees" (https://archives.haskell.org/code.haskell.org/ghc-scp/ghc/docs/comm/the-beast/typecheck.html).
  - Newer GHC expands some constructs before type checking. It keeps the original next to the expansion (`HsExpansion`, now `XXExprGhcRn`/`ExpandedThingRn`) so that errors still show the source form (https://hackage.haskell.org/package/ghc-9.12.1/docs/GHC-Hs-Expr.html).
- **Changes to deployed objects.** Not applicable.

### Scheme, R7RS

- **What is normative.** Chapter 4: "Expression types are categorized as primitive or derived... Derived expression types are not semantically primitive, but can instead be defined as macros. Suitable syntax definitions of some of the derived expressions are given in section 7.3."
- **The core.** 7.2 gives "a formal denotational semantics for the primitive expressions of Scheme" only. 7.3 gives "syntax definitions for the derived expression types in terms of the primitive expression types (literal, variable, call, lambda, if, and set!)... except for quasiquote".
- **Caveats on individual rules.** "possible implementation" for `delay`; "suitable for an implementation with no threads" for `parameterize`.
- **Sources.** https://standards.scheme.org/corrected-r7rs/r7rs-Z-H-6.html and https://standards.scheme.org/corrected-r7rs/r7rs-Z-H-9.html
- **What this shows.** The formal core is kept small by giving semantics only to primitives. Everything else is a normative translation.

### Rust Reference

- **`for` loops.** `[expr.loop.for.desugar]`: "A `for` loop is equivalent to a `loop` expression containing a `match` expression as follows", followed by the full expansion. Hygiene is stated:
  - "`IntoIterator`, `Iterator`, and `Option` are always the standard library items here, not whatever those names resolve to in the current scope."
  - "The variable names `next`, `iter`, and `val` are for exposition only, they do not actually have names the user can type."
  - Source: https://doc.rust-lang.org/reference/expressions/loop-expr.html
- **The `?` operator.** It is defined by behavior per type (Result, Option, ControlFlow, Poll). The `Try::branch` translation is only "roughly equivalent", in a note (https://doc.rust-lang.org/reference/expressions/operator-expr.html). There are two descriptions, and only one is the authority.
- **Errors.** rustc tags generated code with `DesugaringKind` (QuestionMark, ForLoop, Await, TryBlock, ...), so diagnostics can name the construct the user wrote (https://doc.rust-lang.org/nightly/nightly-rustc/rustc_span/hygiene/enum.DesugaringKind.html).
- **Versioning.** Editions version the surface:
  - "Each crate chooses its edition within its Cargo.toml file."
  - "crates in one edition must seamlessly interoperate with those compiled with other editions."
  - Changes "tend to be 'skin deep'. All Rust code - regardless of edition - will ultimately compile down to the same internal representation."
  - Source: https://doc.rust-lang.org/edition-guide/editions/index.html

### Java records and Kotlin data classes

These are defined as implicit declarations unless written explicitly, a "schema with defaults" style rather than a syntactic translation.

- **Java (JLS 21, 8.10.3 and 8.10.4).**
  - "If a record class has a record component for which an accessor method is not declared explicitly, then an accessor method for that record component is declared implicitly, with the following properties..."
  - `equals`, `hashCode` and `toString` are implicitly declared "if a record class R does not explicitly declare a method with the same modifiers, name, and signature".
  - An explicit accessor or canonical constructor must satisfy listed conditions, "or a compile-time error occurs". For example, a public record's canonical constructor must be public.
  - Source: https://docs.oracle.com/javase/specs/jls/se21/html/jls-8.html#jls-8.10
- **Kotlin.** "If there are explicit implementations of equals(), hashCode(), or toString() in the data class body or final implementations in a superclass, then these functions are not generated" (https://kotlinlang.org/docs/data-classes.html).
- **Lesson.** An explicit override of a derived member is allowed but constrained, so it cannot weaken what the derivation guarantees.

### WebAssembly text format

- "Abbreviations are defined by rewrite rules specifying their expansion into the core syntax. These expansions are assumed to be applied, recursively and in order of appearance, before applying the core grammar rules." The core text grammar "closely mirrors the grammar of the abstract syntax" (https://webassembly.github.io/spec/core/text/conventions.html).
- Inline exports: `(func id? (export name) ...)` expands to a function plus a separate `(export name (func id'))`. A side condition requires the generated `id'` not to be bound already (https://webassembly.github.io/spec/core/text/modules.html).
- Decoding produces "an internal representation of a module... modelled by abstract syntax" (https://webassembly.github.io/spec/core/intro/overview.html).

### Dhall standard

- **Translation rules in the standard.** Record completion is normative sugar: "`T::r` is syntactic sugar for `(T.default ⫽ r) : T.Type`". The standard gives it as a rule in beta-normalization and in type inference: "To type-check a record completion, desugar the operator and type-check the desugared form." Record puns, dotted fields and duplicate fields are desugared at parse time by named functions (`desugar-puns`, `desugar-dotted-fields`, `desugar-duplicates`). Sources: https://github.com/dhall-lang/dhall-lang/blob/master/standard/type-inference.md, beta-normalization.md and record.md.
- **Acceptance tests.**
  - Each case is a pair: `...A.dhall` (input) and `...B.dhall` (expected). Parser tests "parse A and encode it to its binary CBOR representation" and compare it with B.
  - On disagreement, either the implementation fails, or "the standard is underspecified... This should be considered a bug of the specification".
  - Source: https://github.com/dhall-lang/dhall-lang/blob/master/tests/README.md
- **Versioning.** "An implementation MUST NOT mix and match functionality from different releases within any given run." A non-breaking change keeps every program that type-checked or normalized before doing so, except that "the semantic integrity check... always changes with a new release of the standard" (standard/versioning.md).

### Research results

- **λJS.** "We reduce JavaScript to a core calculus... We explicate the desugaring process that turns JavaScript programs into ones in the core... We demonstrate faithfulness to JavaScript using real-world test suites" (https://cs.brown.edu/people/sk/Publications/Papers/Published/gsk-essence-javascript/). A small core plus a desugaring can be validated as a whole against end-to-end tests.
- **Resugaring** (Pombrio and Krishnamurthi, PLDI 2014) computes evaluation steps "in terms of the surface syntax", so authors never see the core (https://cs.brown.edu/people/sk/Publications/Papers/Published/pk-resuarging/). This is the academic form of "show the engine's results in the author's text".

## 2. Configuration and authorization systems with two forms

### Docker Compose (Compose Specification) and Kubernetes objects

- **What is normative.**
  - The Compose Specification is itself normative and concise: "The specification defines the expected configuration syntax and behavior. Unless noted, supporting any of these is optional." Its JSON Schema, schema/compose-spec.json, is part of the specification repository.
  - Kubernetes never adopted a normative short form. Kompose "is a conversion tool for Docker Compose to container orchestrators such as Kubernetes" and warns "Our conversions are not always 1-1" (https://kompose.io/).
  - Kubernetes' own generators stayed outside the API. Eventually kubectl took in one of them: "Since 1.14, kubectl also supports the management of Kubernetes objects using a kustomization file... kubectl apply -k" (https://kubernetes.io/docs/tasks/manage-kubernetes-objects/kustomization/). A layer that is used widely becomes normative in practice, without a specification.
- **How the short form's meaning is specified.** As a schema with per-field defaults. Both syntaxes are mandatory:
  - "There are two syntaxes defined for configs. To remain compliant to this specification, an implementation must support both syntaxes. Implementations must allow use of both short and long syntaxes within the same document."
  - The short syntax "only specifies the config name", and the target "defaults to /<source>".
  - Access is explicit: "Services can only access configs when explicitly granted by the configs attribute."
  - Sources: compose-spec container_spec.md and 01-status.md, https://github.com/compose-spec/compose-spec
  - One default is permissive: for volumes, `rw` "is the default if none is specified" (https://docs.docker.com/reference/compose-file/services/).
- **Errors.** The JSON Schema describes the author's form (for example, `ports` items are `oneOf` number, string or object), so validation errors come before normalization and name author paths, such as "services.web Additional property volume is not allowed". That example is from forum reports, not an official page.
- **Consistency between the forms.** `docker compose config` "renders the actual data model to be applied", "expands short-notation into the canonical format", and `--hash` prints a configuration hash for each service (https://docs.docker.com/reference/cli/docker/compose/config/).
- **Changes to deployed objects.** Defined by the implementation, not the spec. `docker compose up` "picks up the changes by stopping and recreating the containers (preserving mounted volumes)". `--remove-orphans` is opt-in (https://docs.docker.com/reference/cli/docker/compose/up/).
- **Strictness.** By default the spec warns about and ignores unsupported attributes ("Default: warn... but ignore them"; Strict mode rejects). That is weaker than Conduit's strict loader.
- **Kubernetes defaults.**
  - "When the API call completes, all static defaults will have been set and stored. Subsequent GETs of the resource will include the default values explicitly."
  - Static defaults "are specific to each API version", and "default values can evolve and change in newer API versions" (https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md).
  - "the default values must be semantically equivalent in all currently supported API versions" (api_changes.md).

### AWS CloudFormation and SAM (`Transform: AWS::Serverless-2016-10-31`)

- **What is normative.** SAM is "an extension of CloudFormation templates... shorthand syntax with fewer lines of code" (https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification.html). The service runs the transform itself: the short form is accepted by the engine.
- **How the short form's meaning is specified.**
  - By a documented table of "Generated CloudFormation resources" for each scenario, a combination of SAM resource and properties. "AWS SAM uses the LogicalIds of the AWS SAM resources in your template file to construct the LogicalIds of the CloudFormation resources it generates." Authors may reference generated resources by logical ID or by "referenceable property".
  - Caveat: "The LogicalIds of some generated resources include a unique hash value... We don't recommend referencing these resources by LogicalId because the hash values might change" (https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-specification-generated-resources.html).
  - The translator is open source and its meaning is pinned by golden pairs, tests/translator/input and tests/translator/output (https://github.com/aws/serverless-application-model/tree/develop/tests/translator).
- **SAM Connectors, a direct analogue of a one-line Conduit grant.**
  - The syntax is `Connectors: {MyConn: {Properties: {Destination: {Id: MyTable}, Permissions: [Read, Write]}}}`. It "Configures permissions between two resources".
  - `Read` "includes... IAM actions that allow reading data", and `Write` "IAM actions that allow initiating and writing data".
  - The output is an `AWS::IAM::ManagedPolicy` listing the concrete actions, with provenance stamped in `Metadata: {aws:sam:connectors: {FunctionToTableConnector: {Source: {Type...}, Destination: {Type...}}}}` (https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-connector.html).
  - The mapping from Read/Write to actions lives in the translator's data file (samtranslator/model/connector_profiles/profiles.json), so it belongs to the tool and not to the author's pinned version.
- **Errors.**
  - Transform failures are reported by logical ID, for example "Resource with id [X] is invalid", not by line. This is from GitHub issue reports.
  - The processed template must still "pass validation checks", and "CloudFormation resolves macros first, and then processes the template" (https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/template-macros-overview.html).
- **Changes to deployed objects.**
  - Edit the template; the transform runs again; "CloudFormation generates a change set that includes the processed template content. After you review the change set, execute it."
  - "Processing macros can add multiple resources that you might not be aware of... we strongly suggest you use change sets."
  - Skipping the review needs an explicit acknowledgement, `CAPABILITY_AUTO_EXPAND`.
  - "When using the update rollback feature, CloudFormation uses a copy of the original template."
  - Sources: same page, and https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/transform-section-structure.html
- **Coexistence.** A SAM template may hold plain CloudFormation resources beside SAM ones, so the long form is always available.

### Terraform: native HCL and JSON syntax

- **What is normative.** Both syntaxes. "The HCL information model is designed to support multiple concrete syntaxes" (https://github.com/hashicorp/hcl/blob/main/spec.md).
  - The JSON spec: "For JSON bodies, the schema is crucial to allow differentiation of attribute definitions and block definitions" (json/spec.md).
  - Terraform: "The JSON syntax is defined in terms of the native syntax... Everything that can be expressed in native syntax can also be expressed in JSON syntax" (https://developer.hashicorp.com/terraform/language/syntax/json).
- **Errors.** Every `hcl.Diagnostic` carries `Subject *Range` ("exactly the construct that is problematic") and `Context *Range` (hcl/diagnostic.go).
- **Consistency.** One information model, decoded by one schema.
- **Changes to deployed objects.**
  - Plan, then apply.
  - Identity changes are stated explicitly in the configuration: a `moved` block records `from` and `to`, and "The address change does not destroy the resource". "We strongly recommend that you retain all historical moved blocks", because "Removing a moved block is a breaking change" (https://developer.hashicorp.com/terraform/language/modules/develop/refactoring).
  - A `removed` block can set `destroy = false` (https://developer.hashicorp.com/terraform/language/block/removed).

### GitHub Actions workflow syntax

- **Normative status.** Documentation only. There are several short forms: `on: push`, a list, or a map.
- **Fail-closed shorthand for access.** "If you specify the access for any of these permissions, all of those that are not specified are set to none". `read-all`, `write-all` and `{}` are the other shorthands.
- **A default from outside the file.** When `permissions` is absent, the default comes from "the default setting for the enterprise, organization, or repository" (https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax). On 2023-02-02 that default became read-only for new enterprises, organizations and personal repositories only (https://github.blog/changelog/2023-02-02-github-actions-updating-the-default-github_token-permissions-to-read-only/). The same file grants different access depending on where it runs. This is a direct counterexample to thesis 2.

### Nix module system

- **Normative status.** No formal specification; the semantics is the library, nixpkgs lib/modules.nix.
- **Defaults are priorities.** `mkOptionDefault = mkOverride 1500`, `mkDefault = mkOverride 1000`, a plain definition is `defaultOverridePriority = 100`, and `mkForce = mkOverride 50`. Definitions of one option merge by its type.
- **Errors list every contributing file.**
  - "The option `x' is defined multiple times while it's expected to be unique... Definition values:" followed by "- In `<file>'" for each definition.
  - Conflicting values report "has conflicting definition values", with each file (lib/options.nix, `showDefs`).
  - `definitionsWithLocations` records where each definition came from (https://github.com/NixOS/nixpkgs/blob/master/lib/modules.nix).
- **Changes to deployed objects.** `nixos-rebuild switch` activates a new whole-system generation, and `--rollback` returns to the previous one (man page, https://www.mankier.com/8/nixos-rebuild).

### CUE

- **Normative status.** One language with a normative specification. Defaults are part of the value lattice, not a separate expansion: "Any element of a disjunction can be marked as a default by prefixing it with an asterisk". When defaults unify, ⟨v1,d1⟩ & ⟨v2,d2⟩ => ⟨v1&v2, d1&d2⟩.
- **Closedness and order.** Definitions are closed; "Referencing a definition will recursively close it". Unification "is commutative, associative, and idempotent. As a consequence, order of evaluation is irrelevant" (https://cuelang.org/docs/reference/spec/).
- **Errors.** Conflict errors list the position of every conflicting value, for example both the data file and the schema file (CUE commit "add more positions to conflict errors", https://cue.googlesource.com/cue/+/0eb7cc590a24bb01024bc81f6cbdb0ef9898b3b2%5E%21).

### Cedar

- **Two syntaxes over one abstract syntax tree.** "To allow for programmatically constructing and parsing policies, Cedar supports a JSON policy format." The JSON "does not preserve comments, whitespace, or newline characters" (https://docs.cedarpolicy.com/policies/json-format.html). Schemas have a Cedar format and a JSON format, and only the Cedar format allows comments.
- **The core is small and formally tested.**
  - "We wrote models of the Cedar evaluator, authorizer, and validator in Lean"; "our parsers are not modeled in Lean".
  - The parser is tested by a round-trip property: "the composition of pretty-printing and parsing produces a policy identical to the one generated".
  - "The Lean models are an order of magnitude smaller than their Rust counterparts": 1,673 lines against 15,693.
  - Source: https://arxiv.org/html/2407.01688v1
- **Changes to deployed objects (Amazon Verified Permissions).** In a static policy you can change only "The action referenced by the policy" and "A condition clause". You "can't change" the effect, the principal or the resource. A template-linked policy changes only through its template (https://docs.aws.amazon.com/verifiedpermissions/latest/apireference/API_UpdatePolicy.html). Fields that carry identity cannot be changed in place.

### SpiceDB, against OpenFGA

These two show the same authorization model with the two choices.

- **SpiceDB: the short form is accepted by the server.**
  - `WriteSchemaRequest.schema` is the schema text, and `ReadSchemaResponse.schema_text` is "the textual form of the current schema" (https://github.com/authzed/api/blob/main/authzed/api/v1/schema_service.proto).
  - The DSL is compiled into core protobufs: "Compile compilers the input schema into a set of namespace definition protos" (pkg/schemadsl/compiler/compiler.go).
  - A generator produces "a DSL view" back from the core, and flags core constructs that the DSL deliberately cannot express: "_this unsupported here. Please rewrite into a relation and permission" (pkg/schemadsl/generator/generator.go).
  - Changing a schema with data is guarded: "any removal of a relation with data, or a relation or permission referenced by [another]... is disallowed", followed by a documented migration in several steps (https://authzed.com/docs/spicedb/modeling/migrating-schema).
  - Relationship writes are explicit operations (CREATE, TOUCH, DELETE) with preconditions (MUST_MATCH, MUST_NOT_MATCH) (authzed/api v1 core.proto and permission_service.proto). The preconditions are analogous to the accepted `expectedId`.
- **OpenFGA: the short form exists only in clients.** "The DSL adds syntactic sugar on top of JSON for ease of use, but compiles down to JSON before being sent to OpenFGA's API. The JSON syntax is accepted by the API" (https://openfga.dev/docs/configuration-language).

### ALFA and XACML: a cautionary authorization precedent

- ALFA "lays down the basic principle to compile policies expressed in ALFA into XACML 3.0 policies. ALFA does not bring new semantics to XACML... ALFA has been designed in such a way that lossless round-trip translations is possible."
- Its successor's authors write that ALFA "remained as a draft under OASIS" and that the round-trip requirement led "to unnecessary complications in ALFA's existing grammar". ALFA 2.0 aims to be "not tied to XACML and not limited by the need to provide round-tripping" (https://www.ietf.org/archive/id/draft-brossard-alfa-authz-00.txt).

## 3. Lessons for Conduit

### L1. The short form can be the language while the core stays small, if every rule of the core is stated over the core only

- **Precedents.** Scheme's formal semantics covers only primitive expressions. Cedar's Lean model covers the abstract syntax tree and is a tenth of the size of the Rust. WebAssembly's validation and execution run on the abstract syntax, and λJS validates a desugaring and a small core together.
- **For Conduit.** Today's canonical documents become the kernel. The roughly 455 promises keep speaking only about kernel documents. The short form adds one block of translation promises; it neither rewrites the core promises nor doubles them.
- **Where the first investigation's high costs came from.** Design C's verdict shows that most of its large cost came from `revise.v1`, `swap.v1` and restart in place: about 11 new and about 23 amended promises, including the safety-bearing ACC-024, PLC-009, SEC-014 and IFC-014. Only about 12 new and about 15 to 25 amended were for the file itself.
- **Hazards on the file side that remain.**
  - SUB-008: the contracts digest covers "the manifest as it was handed over", so putting the short form inside the manifest changes contract identity on every edit to a grant. Keep the short form out of the manifest, or exclude its sections from the hash.
  - SUB-005: diagnostics must point at the authored text.
  - INS-013: its observable forbids saying which values came from arguments, which conflicts with marking derived values.
  - CON-016: if `capability` may be omitted, inferring it must become a normative, deterministic, stored rule.
  - OBJ-003 and NAM-027: generated names.
  - MAN-001: whether a lock is a second file.

### L2. Define the meaning by translation, and test each rule with input/expected pairs and one equivalence property

- **Precedents.** The Haskell Report gives identities for each construct. Dhall states `T::r` as a rule in both normalization and type inference. WebAssembly gives rewrite rules applied before the core grammar.
- **How they are tested.** Dhall's acceptance tests are A/B pairs, and a disagreement is a bug in the specification. SAM's translator is pinned by input/output templates. Cedar uses a round-trip property.
- **For Conduit.**
  - A new vector kind, `expand`: an authored file in, the expected canonical documents out, or the expected diagnostics with authored locations.
  - A property: submitting the rendered expansion gives exactly the objects that submitting the short form gives. This is the synthesis's "test A", made normative.
  - Haskell's `let` exception shows why: even carefully written identities can differ in meaning, so every rule needs its own vectors.
  - This is what gives thesis 2 to what authors write. The core can cite, for every derived object, the expansion promise that made it.

### L3. Name one authority and allow an as-if rule

- Haskell: "it is not expected that it will use them directly". Rust's for-loop expansion is normative ("is equivalent to"), but its `?` translation is only "roughly equivalent" in a note beside a behavior defined per type, so there are two descriptions.
- **For Conduit.** The translation is the authority. An engine may expand the short form any way it likes, but the objects, records and diagnostics must equal those of the expanded documents. Never describe the same construct twice normatively.

### L4. Expansion must depend only on the text and its declared version; derived values are fixed when written and carry provenance

- **Anti-precedents.**
  - GitHub Actions: with `permissions` absent, access depends on organization settings, and the default changed for new repositories in 2023.
  - SAM: a hash in some generated logical IDs "might change"; the actions behind Connectors' Read/Write belong to the translator's profiles.json.
- **Precedents to copy.**
  - Kubernetes static defaults are "specific to each API version", and once stored, "Subsequent GETs of the resource will include the default values explicitly".
  - Rust editions are chosen per crate, "skin deep", and compile to one internal representation.
  - Dhall forbids mixing releases within a run.
  - SAM stamps `Metadata: aws:sam:connectors` on what it generates.
- **For Conduit.**
  - Tie every default to the `apiVersion` stated in the file, never to the caller, history, the installed tool or the environment. Design C's skeptic objection (5) was "the derived defaults depend on who runs apply and on history".
  - Write every derived value into the stored canonical document: operations listed in full, the capability, the Pool owner, the operator Conduits.
  - Record the rule that made each, so that a later language or package version never changes what an existing object means. This addresses ruling 53's concern about a package version flipping a default unseen.
  - Any grouping of operations, if it is ever wanted, must come from the pinned package's contract, never from the language or the tool (ruling 25 already keeps names fully qualified).

### L5. Generated names: deterministic, derived from the author's keys, reserved, and not hashes of content

- Rust: the generated names are "for exposition only, they do not actually have names the user can type". WebAssembly checks that a generated id is not already bound. SAM builds logical IDs from the author's IDs and warns against relying on hashed ones. Haskell and Rust bind translation names to core entities "regardless of" what is in scope.
- **For Conduit.**
  - Derived Pool, operator and turn Conduit names come from keys, under a reserved separator such as the synthesis's `--`.
  - They refer to core-package addresses at the pinned core version, unaffected by authored names such as an author's resource named `access`.
  - Never hash content into a name: every edit would then become a replacement.
  - Expose a documented stable handle, as SAM's "referenceable properties" do, where authors must refer to a derived object.

### L6. Diagnostics speak in the author's text

- Validate the authored form against a schema of the authored form before expanding: Compose's JSON Schema has oneOf short|long, and its errors name author paths.
- Keep the original next to the expansion (GHC's `HsExpansion`), and tag what was generated (rustc's `DesugaringKind`).
- Name every contributing definition (Nix's "- In `file'" lines, CUE's positions of each conflicting value), and give exact source ranges (`Subject` and `Context` of `hcl.Diagnostic`).
- SAM's server-side transform reports only "Resource with id [X]". That is the floor to beat.
- **For Conduit.**
  - A promise that every diagnostic about an expanded document names the authored location (a JSON Pointer into the short-form document, plus line where the loader has it; the loader already has positions, per C's verdict), the expansion rule, and the kernel promise ID that rejected it.
  - SUB-005's JSON Pointer rule extends naturally to this.

### L7. The long form stays legal inside the short form; do not require round-tripping

- Compose: implementations "must allow use of both short and long syntaxes within the same document". SAM templates carry plain CloudFormation resources. WebAssembly abbreviations are optional.
- ALFA's requirement of lossless round-trips with XACML "led to unnecessary complications" in its grammar. SpiceDB's generator simply flags core constructs the DSL does not express (`_this`).
- **For Conduit.**
  - The short form is a superset that admits canonical documents. This keeps ruling 4's scope guard: S2 to S10 constructs not yet in the short form stay writable, normatively, in the same submission.
  - The expansion must be total, and `render` must always produce valid canonical documents.
  - Do not require every canonical document to be expressible in the short form.

### L8. A normative short form does not need an engine that applies changes; identity changes are explicit and fields that carry identity are immutable

- No surveyed system makes the short form change objects implicitly:
  - CloudFormation computes a change set on the processed template, executed separately, and skipping the review needs `CAPABILITY_AUTO_EXPAND`.
  - Terraform plans and then applies. Identity changes are written as `moved` and `removed` blocks, kept forever.
  - Compose's recreation is defined by the implementation, not the spec.
  - SpiceDB refuses schema changes that would orphan data and documents migrations in several steps. Its writes carry preconditions.
  - Amazon Verified Permissions forbids changing a policy's effect, principal and resource. Kubernetes keeps RoleBinding `roleRef` immutable (prior-art map).
- **For Conduit.**
  - Keep OBJ-002 and SUB-007. Change made through the short form is diff(expansion, what exists), turned into today's requests and executed only on explicit request, with `expectedId` and engine-enforced tombstones (accepted in ruling 53).
  - An identity change, such as a replacement, a rename or a retirement, is a statement in the file and not an inference.
  - Whether the plan function itself becomes normative is a separate, later choice. CloudFormation's create-then-execute change set is the precedent for a pure, server-computed plan that never runs implicitly.

### L9. Access shorthands fail closed

- GitHub Actions: specifying any permission sets the rest to none. Compose configs are granted only explicitly. Against that, Compose volumes default to `rw`, a permissive default.
- SAM Connectors compress a grant to `Destination` plus `Permissions: [Read, Write]`, but the expansion into actions belongs to the translator.
- **For Conduit.**
  - A one-line grant may omit what can be derived deterministically: the capability, at most, inferred only when unique, otherwise refused with the candidates named.
  - It never omits or groups operations beyond the pinned contract.
  - Reach flags default to false (ruling 53, item 7).
  - The expansion writes the full operation list into the stored Conduit, as SAM writes the full action list into the policy.

### L10. Where the expansion runs is a separate choice from whether it is normative

- SpiceDB, CloudFormation with SAM, and Compose accept the short form in the engine. OpenFGA compiles in its clients and its API accepts only JSON.
- Kubernetes shows what happens to a layer that is never made normative: generators multiply (Helm, Kustomize, kompose's "not always 1-1"), and one is later folded into kubectl (`apply -k`) without a specification.
- **For Conduit.**
  - spec/ can define the short form normatively, giving thesis 2, a second implementation that must agree (A34), and a fresh author who learns it from spec/ alone (ruling 48, critic point 3).
  - The first build can still run the expansion in its pure core, or in a shared pure crate used by the CLI. Normative status does not force an engine-side `apply`.

## 4. The options, from this evidence

1. **Layered (design E, as recommended).** The short form's meaning is defined by a CLI.
   - Cost: about 11 to 14 promises.
   - Thesis 2 is untested for authored text.
   - About 27 promises are mirrored in the CLI, so each future core change costs about 1.3 to 1.5 times as much (synthesis).
   - It follows Kubernetes' trajectory.
2. **Normative expansion only (not costed before).** Today's documents are the kernel, and the short form is defined by translation promises in spec/ with `expand` vectors. Change keeps the accepted explicit plan/apply over today's requests.
   - Roughly 30 to 40 promise touches. That is my estimate: design C's file-side changes without revise or swap.
   - About 40 to 80 new A/B vectors, also an estimate.
   - No safety-bearing change to ACC, PLC or SEC promises.
   - Precedents: Haskell, Scheme, WebAssembly, Dhall, SAM, SpiceDB, Compose.
3. **Normative form plus engine apply (C and D).** About 50 to 80 promises, with major safety objections. No surveyed precedent requires this bundle.

What should be settled first under option 2:

- where the short form lives (beside the manifest, not inside it, because of SUB-008);
- the diagnostic-location promise;
- defaults tied to the version and written into the stored document with provenance (and INS-013's observable);
- the rules for generated names;
- whether `capability` inference is normative or the field stays required (CON-016);
- the `expand` vector kind;
- whether accepting the short form is required of every engine (as SpiceDB does) or only that its expansion be honored.

## Unverified
- The cost estimates for normative expansion only (about 30 to 40 promise touches, about 40 to 80 `expand` vectors) are mine. They come from reading design C's skeptic breakdown (design-C-agent-centric.md:710-800) and are not a recount promise by promise. Which of the 17 promises C named itself belong to the file side and which to revise is my judgement.
- SAM: the exact transform error text ("Transform AWS::Serverless-2016-10-31 failed with: Invalid Serverless Application Specification document... Resource with id [X] is invalid") comes from search results citing GitHub issues (for example aws/serverless-application-model issue 1470). I did not open them. That profiles.json holds the Read/Write-to-IAM mappings is inferred from the file name and directory. Its contents were not read.
- Compose: the form of validation errors ("services.X Additional property Y is not allowed") comes from forum posts, not official documentation. That Compose validates the author's form before normalization is inferred from the schema's oneOf definitions and the description of `docker compose config`.
- GHC: how far current versions expand before type checking (ExpandedThingRn, HsCtxt for error context) varies by version. I relied on search summaries of Hackage pages and commit messages.
- CUE: that conflict errors list every conflicting position comes from test fixtures and a commit found by search, not from the language specification. Exact output differs by version.
- Nix: the rollback through generations (`nixos-rebuild switch --rollback`) is from secondary man-page mirrors, not the current NixOS manual. The priority values and error texts were read in nixpkgs master source.
- SpiceDB: whether ReadSchema returns text regenerated from the stored core definitions, or the stored original text, is not confirmed. The generator exists and the proto says "textual form of the current schema".
- Zanzibar's namespace configuration as the "core" beneath SpiceDB's DSL is stated from general knowledge of the 2019 paper. Only SpiceDB's compiler output, core NamespaceDefinition protos, was checked in source.
- Dhall: the sentence that the semantic integrity check "always changes with a new release of the standard" is quoted as written in versioning.md. Its exact intent was not checked against release practice.
- ALFA: its OASIS status is from the IETF draft's own account and secondary pages. The reasons for ALFA's limited adoption beyond "unnecessary complications" are not established.
- The WebAssembly overview page did not state outright that validation and execution are defined on the abstract syntax. I relied on its "modelled by abstract syntax" wording and on the conventions page.
- Kubernetes RoleBinding `roleRef` immutability is taken from the earlier prior-art map, not re-fetched here.
- Not checked: whether the five core additions accepted in ruling 53 interact with a normative expansion beyond what is noted, in particular `as`, local names and IFC-014 for tool names that derived Conduits generate.