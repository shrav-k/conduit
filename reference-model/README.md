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

It holds one crate so far, the strict loader of increment 1 (A43), in
[crates/loader](crates/loader), whose package is `conduit-loader`. The core and the shell will be
crates beside it.

## Running it

The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml), Rust 1.97.1, which
rustup installs on first use. Run the commands from this directory, so that the pin applies:

```sh
cargo test --release
cargo clippy --all-targets -- -D warnings
```

`Cargo.lock` is committed, and the build output, `target/`, is ignored. The tests read the
specification from `../spec/`, so they run inside a checkout of this repository.

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
  fractional part is binary64 (SCH-020). JSON as RFC 8259 writes it, with the same narrowings,
  so that it means the same as YAML (SCH-013); a JSON file holds one document or an array of
  them (SCH-018). Files are UTF-8, and values nest at most 128 levels deep, a limit that DL10
  leaves to an engine.
- **A project's directory** (`project.rs`). The manifest is the one file named `conduit.yaml`,
  `conduit.yml` or `conduit.json`, and it holds one document (MAN-001); more than one is
  rejected (MAN-002). The documents are the other files of the directory itself whose names
  end in `.yaml`, `.yml` or `.json`; nothing below it is read, and no other file (MAN-011,
  SUB-010). A link is followed; one that leads nowhere, or an entry that cannot be read, under
  such a name, is rejected under SUB-010 and never skipped. Files handed over without a
  directory are read by the same rules.
- **Validation** (`schemas.rs`, `formats.rs`). Each document is validated against `document.v1`
  and the manifest against `manifest.v1`, after the Schema contracts of `spec/schemas/` and
  `spec/core/` are bundled exactly as `bundle()` in `tools/check_schemas.py` bundles them. The
  validator is JSON Schema 2020-12 with the profile's formats asserted (SCH-001), `uuid` and
  `duration` checked by the grammars that the profile cites (SCH-004), and `pattern` matched as
  ECMA-262 with the `u` flag, in the edition that JSON Schema 2020-12 cites (SCH-016). The loader
  also rejects a document whose own schemas hold a pattern that is not one of ECMA-262
  (`authored.rs`): its reading, pending finding 1-03 of the log. `Specification::check` gives
  that verdict beside the schemas', as reading a project does; `Specification::validate` applies
  the schemas alone.

It does not resolve references, instantiate templates or apply the rules of
`tools/check_schemas.py` that no schema can express: those come with the core, in increment 2.

## The tests, and the done-when

The tests are in [crates/loader/tests](crates/loader/tests). `examples.rs` and `formats.rs` hold
increment 1's done-when; the others test the rest of what it covers.

| Test | What it shows |
| --- | --- |
| `tests/examples.rs` | Every directory under `spec/schemas/examples/valid/` is read as a project and accepted; every file under `invalid/` is rejected by the step that `check_schemas.py` rejects it by, when read or by a schema, as the fixture `tests/expected/invalid-verdicts.txt` records; every document under `invalid-by-rule/` passes the schemas; the core package's own documents are valid; every example means the same written again as JSON (SCH-013); and the trees of examples are flat, so these tests read every file that the tool reads |
| `tests/reading.rs` | The syntax rules, case by case, in YAML and in JSON: the characters YAML allows; every fault of a file reported, in any order of its documents and of their members (SUB-001); each fault with the same rule and place in YAML and in JSON (SCH-013); a document marker that the parser would read as content; and a file of some megabytes read in linear time |
| `tests/formats.rs` | The grammars of `uuid`, with its edges of version and variant, and of `duration`; each format asserted; patterns matched as ECMA-262 where Python's `re` and Rust's `regex` read them otherwise; a pattern that is not ECMA-262, in the edition that JSON Schema 2020-12 cites; and the verdict of `Specification::check` on one |
| `tests/project.rs` | Two manifests; a manifest that is not one document; documents in a directory below, not read; a file with another ending, ignored; a link followed, and one that leads nowhere rejected; the order of documents, within a file and across files, which means nothing (SUB-001) |
| `tests/rules.rs` | Every promise the loader cites is one that `spec/` defines |

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
- Its lookahead stops at 255 levels of flow collections, which the loader reports as the limit
  that DL10 leaves to an engine. The loader's own limit is 128 levels.

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
edition of ECMA-262, and the loader rejects the two pieces of syntax it accepts that the
edition of 2020, which JSON Schema 2020-12 cites, does not: a group that sets flags and a name
given to two groups. The crate cannot be given another engine for `patternProperties`; no
schema of the specification uses it, and the
[log of increment 1](../docs/proposals/first-build/log-increment-1.md) records what is left for
the core.
