# The reference model

Not binding. The specification in [spec/](../spec/README.md) alone binds, and this code follows
it: where the two differ, the code is wrong, or the difference is a finding of the first build.

This is the codebase of the first build, as [ruling 46](../docs/proposals/ruling-46/README.md)
shaped it: one Cargo workspace in Rust that will hold the pure core, the reference model (A38),
in which every decision cites a promise, and later the shell that makes it runnable, the first
slice of the reference engine (A50). [The first build](../docs/implementation/FIRST_BUILD.md)
pre-registers its increments and what each must show, and the
[roadmap](../ROADMAP.md#the-order-of-the-work) says where it stands. Each increment logs what it
finds under [docs/proposals/first-build/](../docs/proposals/first-build/README.md).

It holds the strict loader of increment 1 (A43), in [crates/loader](crates/loader), whose
package is `conduit-loader`; the pure `conduit-core` in [crates/core](crates/core); and the
`conduit-conformance` runner in [crates/conformance](crates/conformance). The shell comes later.

## Running it

The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml), Rust 1.97.1, which
rustup installs on first use. Run the commands from this directory, so that the pin applies:

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release -p conduit-conformance
```

`Cargo.lock` is committed, and the build output, `target/`, is ignored. The tests read the
specification from `../spec/`, so they run inside a checkout of this repository.

## What increment 2 covers so far

I2-01 implements deterministic first start: exactly five ready objects and root's two open
declared Conduits, retaining their built-in template provenance, ownership, placement and
machine labels. `conduit-core::step` takes immutable state, an input and a supplied context,
and returns state, promise-citing decisions, records, external intents and a semantic reply.
Time, UUID allocations, revisions, generated suffixes and the local machine report enter as
values. No core code reads a file, clock, network or process. Allocation errors leave the
caller's state untouched. Restart uses retained state and makes no new identities.

The [portable cases](../conformance/README.md) run through `conduit-conformance`, the file
boundary. They include protected-template rejection, root's LIF-034 lifecycle refusals and
the four original loader settlements. Every decision cites known promise IDs, each record
is checked against `record.v1`, and exact object and record arrays detect omissions and extra
effects. The runner validates cases against their JSON Schema and checks the coverage index.
The loader adds a public compiler for a built-in Schema contract with resolved references and
rejects two initial YAML byte order marks under SUB-005, applying the admin's settlement 2-01.
A permitted byte order mark before a later YAML document remains accepted.

I2-02 adds [authored schema evaluation](crates/core/src/schemas.rs) and E05 tool publication.
Supplied exact package/project scopes keep each manifest's release pins and each Schema
contract's local root. Authored validation uses separate in-memory resources; publication
copies each reached contract once into the tool root and rewrites its references, preserving
siblings, local definitions and recursion. Both paths reuse the loader's validator, formats
and ECMA pattern engine. `patternProperties` is specialized to the finite set of instance keys,
with original schema positions retained for references and `additionalProperties` handled by
the validator. The published schema itself contains the original patterns.

The core exposes `evaluate-schema`, `check-authored` and `tool-definition` inputs. The first
fills supplied values (with an operation-input mode and an output mode that fills nothing).
The second is the semantic admission check for schemas after strict loading, shared by future
installation/submission transitions; it checks default validity and reserved root copy keys.
The third reads the operation and schemas held by an explicitly supplied target and open
Conduit. It handles control narrowing and omission, handles, descriptions and hints. These
vectors start with established snapshots and claim no package installation, registration or
opening. The runner checks every snapshot contract through the loader, uses the actual core
contracts, and compares authored and standalone verdicts for every E05 sample in both schema
directions. Standalone validation checks all references are local and resolve before compiling
with no package resolver. Conflicting unconditional defaults stop for the admin (finding 2-03).

General registration, access, invocation handling and the shell belong to
later orders. Unsupported inputs are embedding errors, not invented Conduit reasons. The
[increment's log](../docs/proposals/first-build/log-increment-2.md) records the settled BOM case
and the verification corrections: registration checks the existing owning Pool's authority
before exposing protected-template diagnostics, and lifecycle records keep the Conduit's
target distinct from the named object. Authorized vectors start from an explicit snapshot
containing root's additional open Conduit to system; they claim no general registration or
opening implementation.

## What increment 1 covers

The loader reads Conduit's documents as the specification says, and rejects what it must
reject. Every rejection is a value, a `Diagnostic`, which names the promise it rests on, the
file, the document within the file and the place within the document, as a JSON Pointer and as
a line and a column, and nothing panics on any input. A file that is not YAML or JSON is
rejected for that alone; otherwise every fault of every document is reported, so that the same
documents in any order give the same diagnostics (SUB-001), and a fault is named alike in the
YAML and the JSON form of a document (SCH-013). Reading takes time in proportion to the size of
a file. What reads the file system is at the edge: `read_file`, `read_project` and
`Specification::read`. Everything else takes what was read, so the core can use it unchanged.

- **One document, strictly** (`yaml.rs`, `json.rs`, `number.rs`). YAML 1.2.2 with its core
  schema, narrowed as the [schema profile](../spec/schema-profile.md#yaml-features-not-accepted)
  says: a duplicate key, an anchor, an alias, a merge key, a tag and a key that is not a string
  are rejected (SCH-012), and so are `.inf`, `.nan`, `0x1F` and `0o17` (SCH-017) and numbers out
  of range (SCH-006, SCH-021), and a character that YAML 1.2.2 does not allow where it stands.
  A number with no fractional part is an integer however it is written (SCH-005); one with a
  fractional part is binary64 (SCH-020), and an integer where that value is whole, held to the
  range of SCH-006, so `9007199254740993.5` is rejected. JSON as RFC 8259 writes it, with the
  same narrowings, so that it means the same as YAML (SCH-013); a JSON file whose value is an
  array holds its items as documents, and any other holds its value as one (SCH-018). Files are
  UTF-8, with one leading byte order mark accepted and ignored in either syntax; a noncharacter may stand in a
  string, and a surrogate that is not paired may not, nor, in YAML, a pair written as two
  escapes. Values nest at most 128 levels deep, a limit that DL10 leaves to an engine, past
  which a document is rejected under SUB-011.
- **A project's directory** (`project.rs`). The manifest is the one file named `conduit.yaml`,
  `conduit.yml` or `conduit.json`, and it holds one document (MAN-001); more than one is
  rejected (MAN-002). The documents are the other files of the directory itself whose names
  end in `.yaml`, `.yml` or `.json` and do not begin with a dot, names and endings compared
  exactly; nothing below it is read, and no other file (MAN-011, SUB-010). A link is followed,
  and one to a directory is not entered; one that leads nowhere, or an entry that cannot be
  read, under such a name, is rejected under SUB-010 and never skipped. Files handed over
  without a directory are read by the same rules.
- **Validation** (`schemas.rs`, `formats.rs`). Each document is validated against `document.v1`
  and the manifest against `manifest.v1`, after the Schema contracts of `spec/schemas/` and
  `spec/core/` are bundled exactly as `bundle()` in `tools/check_schemas.py` bundles them. The
  validator is JSON Schema 2020-12 with the profile's formats asserted (SCH-001), `uuid` and
  `duration` checked by the grammars that the profile cites (SCH-004), and `pattern` matched as
  ECMA-262 with the `u` flag, in the edition that JSON Schema 2020-12 cites (SCH-016). The loader
  also rejects a document whose own schemas hold a pattern outside the interoperable tokens
  of JSON Schema 2020-12, core, section 6.4, or one that is not ECMA-262 in Unicode mode
  (`authored.rs`), as SCH-016 says. `Specification::check` gives that verdict beside the
  schemas', as reading a project does; `Specification::validate` applies the schemas alone. A
  rejection by a schema rests on SUB-005, and names the Schema contract, the instance's place
  and the failing keyword's location (`keywordLocation`, and `absoluteKeywordLocation` where
  known, to be carried in the SARIF result's `properties`).

It does not resolve references, instantiate templates or apply the rules of
`tools/check_schemas.py` that no schema can express: those come with the core, in increment 2.

## The tests, and the done-when

The tests are in [crates/loader/tests](crates/loader/tests). `examples.rs` and `formats.rs` hold
increment 1's done-when; the others test the rest of what it covers.

| Test | What it shows |
| --- | --- |
| `tests/examples.rs` | Every directory under `spec/schemas/examples/valid/` is read as a project and accepted, and the file there whose name begins with a dot is not read; every file under `invalid/` is rejected by the step that `check_schemas.py` rejects it by, when read or by a schema, as the fixture `tests/expected/invalid-verdicts.txt` records, a rejection by a schema always under SUB-005; every document under `invalid-by-rule/` passes the schemas, and `Specification::check` rejects those named for SCH-016 and no other; the core package's own documents are valid; every example means the same written again as JSON (SCH-013); and the trees of examples are flat, so these tests read every file that the tool reads |
| `tests/reading.rs` | The syntax rules, case by case, in YAML and in JSON: the characters YAML allows; a number read as a whole binary64 value, held to the range of SCH-006; I-JSON's two rules, and a surrogate that is not paired; every fault of a file reported, in any order of its documents and of their members (SUB-001); each fault with the same rule and place in YAML and in JSON (SCH-013); a document marker that the parser would read as content; and a file of some megabytes read in linear time |
| `tests/pattern_rewrites.rs` | All eight historical patterns compared with their rewrites, on example strings and generated and boundary strings, using `regress` for both |
| `tests/formats.rs` | The grammars of `uuid`, with its edges of version and variant, and of `duration`; each format asserted; patterns matched as ECMA-262 where Python's `re` and Rust's `regex` read them otherwise; a pattern that is not ECMA-262 or uses tokens outside the interoperable subset; and the verdict of `Specification::check` on one |
| `tests/project.rs` | Two manifests; a manifest that is not one document; documents in a directory below, not read; a file with another ending, or whose name begins with a dot, ignored; a link followed, and one that leads nowhere rejected; the order of documents, within a file and across files, which means nothing (SUB-001) |
| `tests/rules.rs` | Every promise the loader cites is one that `spec/` defines, with an ID in the form of a diagnostic's `ruleId` |

## Choices

**The schemas are read from `spec/` when the loader runs**, not embedded when it is built. The
specification is the source of truth, and the first build's findings change it: read at run
time, the tests always run against the specification as it stands, and nothing is copied. The
names that [spec/names.yaml](../spec/names.yaml) defines, the core package's address among
them, are read from that file too, and never written here: `tools/set_names.py --check` scans
this directory, and a test writes its documents with the names it reads. The shell, which must
carry the core package with it (NAM-020), will decide how to embed it.

**YAML: [saphyr-parser](https://crates.io/crates/saphyr-parser) 0.3.3.** `serde_yaml` is no
longer maintained: its last release is `0.9.34+deprecated`. `serde_yml`, a fork of it, is
deprecated too. `yaml-rust2` and `saphyr` come from the same maintainers, both from `yaml-rust`
with fixes toward the YAML test suite, and `yaml-rust2` says that it will have only basic
maintenance and that new work goes into `saphyr`. The loader uses `saphyr`'s parser alone, which
gives every event with its anchor, its tag, the style of a scalar and its place, and builds the
values itself, so every narrowing of the profile is a decision written in `yaml.rs` and none is
a default of a crate. Where the parser does not do what YAML 1.2.2 says, the loader does,
in its own code, and none of it is left to chance:

- It does not check the characters of the stream, `c-printable`: it takes a C0 control
  character anywhere, and DEL, a C1 control character, U+FFFE and U+FFFF outside a quoted
  scalar. The loader rejects each as not YAML.
- It does not check the version of a `%YAML` directive, which YAML 1.2.2 has a reader reject
  when it is not 1.x. The loader does, for a directive named `YAML` exactly.
- It reads a line `---` into a block scalar at the root as content, where YAML 1.2.2 has it
  begin a document. The loader rejects such a text as not YAML rather than read it otherwise
  than YAML does; the fault is to be reported to the parser's maintainers.
- Its lookahead stops at 255 levels of flow collections, which the loader reports, under
  SUB-011, as the limit that DL10 leaves to an engine. The loader's own limit is 128 levels.

**JSON: [serde_json](https://crates.io/crates/serde_json)**, with its `raw_value` feature. It
checks the syntax. The values are built from the raw text of each, so that two members with one
key are found, which `serde_json` would otherwise keep one of silently, and so that a number is
read from the digits that write it.

**The validator: [jsonschema](https://crates.io/crates/jsonschema) 0.61.0**, without its default
features, so it fetches nothing. It asserts formats when asked, and checks `date-time` and
`date` by RFC 3339 and `uri` and `uri-reference` by RFC 3986. Three things it does otherwise
than the profile asks, the loader gives it. Its `uuid` is replaced by a check written from the
grammar of RFC 9562 that SCH-004 cites, and its `duration`, which takes the letters as capitals
only, by one written from the grammar of RFC 3339, appendix A, whose letters match either case,
as ABNF has it (finding 1-13). Its `pattern`, which translates a pattern into the syntax of
another engine as far as it can, is replaced by [regress](https://crates.io/crates/regress), an
engine of ECMA-262's regular expressions, in Unicode mode. `regress` follows the current
edition of ECMA-262. The loader tokenizes every pattern against the interoperable tokens of
JSON Schema 2020-12, core, section 6.4, plus escaped pattern punctuation, before compiling it.
The old patterns and their rewrites are compared with `regress`'s PikeVM backend in
`tests/pattern_rewrites.rs`, avoiding costly backtracking in the historical patterns. The crate cannot be given another engine for `patternProperties`; no
schema of the specification uses it, and the core matches it itself when it validates the
schemas that authors write, from increment 2, as the
[log of increment 1](../docs/proposals/first-build/log-increment-1.md) records.
