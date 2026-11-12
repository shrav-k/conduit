# Work order 125 (A63-H1, finished): close what H1's final verifier left open

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token, a private key or a secret value, even one that a test made.

## What and why

Work order 123 (`docs/work-orders/123-a63-h1-provider-contract-harness.md`) built
`experiments/022-provider-contracts/`, the harness for provider-contract vectors. Its two rounds
are spent, and its final verifier left three musts: two in how a report is redacted, and one in
how a call that the provider rejects for its protocol revision is read. This order starts from
order 123's last commit, which is where your working tree begins, and closes the three. Order
123's decisions all still apply; read that order and the crate's README first.

## The decisions it applies

Fix each finding as its fix says, each with a regression test that fails on order 123's code and
passes on yours. Line numbers are those of order 123's last commit in
`experiments/022-provider-contracts/src/lib.rs`.

1. **Values given directly in a call's arguments are redacted** (near line 294; decision 7 of
   order 123). The set of values to redact holds only a case's `markers` and `fixtures`, so a
   token, a secret or another identity value written directly in a call's arguments can reach a
   report. Add every literal string value of a call's `identity` and `secrets` arguments to that
   set before the report is written. Collect literal strings only: skip `$param`, `$fixture` and
   `$capture` objects, whose names (such as `hostname`) would otherwise erase ordinary words,
   since redaction matches substrings (`report.rs`, near line 43). Add no new case field; a
   case's `markers` are already collected. A literal secret written in an `equal` assertion of a
   `get-secret.v1` case is collected too; the case form's document says that secrets in a case
   are written as markers or fixtures. Test, in both the JSON and the
   Markdown report: an identity token given directly in a call's arguments and repeated in a
   report field does not appear.
2. **Lab labels used as keys are redacted** (near line 406). The walk that collects values visits
   only an object's values, so a label name used as a key, with an empty value, is never
   collected. Include the keys of objects under `labels` only, in the descriptor's configuration
   and parameters and in fixtures, skipping `$` keys: not every object key, since keys such as
   `machine`, `labels` or a configuration key would erase case IDs and details. Test: an
   empty-valued label whose key also occurs in a report field does not appear, and a case ID
   containing the word `machine` still does.
3. **A rejection for the protocol revision is retried once** (near line 149; IFC-046 at
   `spec/interface/implementation.yaml`, near lines 1282 to 1284, and OPS-007). The mechanics:
   - the error is -32022, `rmcp` 3.9.0's `UNSUPPORTED_PROTOCOL_VERSION` (its `model.rs`, near line
     636, in the cargo registry);
   - the server's revisions are the error's `data.supported` (near lines 692 to 701), or, where
     absent, those its `server/discover` lists; the harness's are
     `ProtocolVersion::KNOWN_VERSIONS`;
   - the retry uses the newest revision both list, other than the one rejected; where there is
     none, nothing more is sent, and the first error is read as OPS-007 reads it;
   - `rmcp` sets each request's protocol version from the session (`service.rs`, near lines 883 to
     887), so set the retry's through `PeerRequestOptions.meta`, which is applied after it (near
     lines 889 to 891). The two sends are one call, read from the second answer;
   - a call already sent once more, for -32020 or for a revision error, is never sent again for
     the other (IFC-046: "has not sent that call once more already").

   This is the one repeat IFC-046 allows; the README's statement that effectful operations never
   repeat names it as the exception. Tests, with a fixture that checks the retried call's
   protocol version: a fallback that succeeds at a different revision; no shared revision; a
   retry that is itself rejected; and a call that drew -32020 first is not retried for a
   revision error.
4. **Nothing else changes:** no new behavior beyond these three, and no change outside the crate.
   If a fix needs a choice that `spec/` or order 123 does not make, take the narrowest, put it
   behind one function, and report it as a finding.

## Scope

Change only `experiments/022-provider-contracts/`.

## Done when

In `experiments/022-provider-contracts/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`

From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/check_roadmap.py`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=experiments/022-provider-contracts
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep target/ | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report, for each of the three, the test that shows it fixed.
Do not stage anything.
