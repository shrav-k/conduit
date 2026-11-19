# Work order 206: a login's redirect path and page refused where no URL could pass IFC-049

You are a Codex worker carrying out one work order in this repository. It runs on the Mac or on
the second machine; it needs nothing of Linux. Read [AGENTS.md](../../AGENTS.md), its section
"If you were given a work order" above all, then this order. Your report must match the schema in
`004-i2-01-vectors-and-first-start.schema.json`, the schema the orders share: list in `vectors`
the case whose steps and fixtures you add, with the promises it tests and whether it passes. This
order has network access only because the engine's tests bind loopback ports, which the sandbox
refuses without it; use it for nothing else, add no crate, and reach no other machine. Run one
cargo command at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not
run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**What it builds.** The schema `sign-in-login.v1` refuses a redirect login whose `path` holds `?`
or `#`, and a page or service login whose `page` holds user information. Today both pass the
schema, yet no URL holding either could pass IFC-049's checks, so a Type can state a login that
can never be relayed or returned, or that relays a credential.

**Where it comes from.** [Order 048a-1's Record](048a1-build-sign-in-type-checks.md#record) left
open its second verifier's two musts: "a redirect path that forbids `?` and `#`, and a page URL
that forbids user information", for "the follow-up order on CTR-013's comparisons across Types,
or a schema order". This is the schema order. The page point is also item 5 of
[FR-138](../proposals/full-review/findings/L12-schemas.md#fr-138), whose least change reads
"forbid user information in `page`, or say why it is allowed"; its verifier found a `page`
holding a user name and password accepted, where `authorization` and `issuer` refuse one. The
full review's F121 asks that a rule only a schema states get a clause in its promise
([the decisions](../proposals/full-review/decisions.md#objects-naming-schemas-and-packages)), so
CTR-013 gains one for each.

**Why a schema change, not a loader check.** Each rule bears on one literal string of one login,
which a JSON Schema `pattern` states; [spec/schemas/README.md](../../spec/schemas/README.md)
leaves to the engine only what its section "What a schema cannot express" lists, and a pattern
on one literal string is not among them.
The loader already validates every document against `spec/schemas/contracts.yaml` and reports a
failure under SUB-005, and the engine reads the schemas from `spec/` when it runs, so no source
changes. The grounds in the specification:

- **The redirect path.** IFC-049 has a redirect login's `redirect_uri` be `http://` "with the
  stated host, one stated port (or 1024 to 65535 for any) and stated path, without query", and
  PND-013's fourth check compares a return's path with the relayed `redirect_uri`'s. A raw `?`
  starts a query and a raw `#` a fragment (RFC 3986, section 3), so a `redirect_uri` built from a
  path holding either has a query or a fragment, and fails. The first verifier of 048a-1 also
  found that order 048a-1's overlap test compares such paths as strings, though IFC-049 compares
  the path without query; once the schema refuses them, the strings are the paths.
- **The page.** IFC-049: "The URL is HTTPS only, contains no credential or personal data". For a
  page or service login the relayed URL carries `page` as its `redirect_uri`, so user
  information in `page` is a credential or personal data in that URL. `authorization` and
  `issuer` already refuse user information; `page`'s pattern alone admits it, through its group
  right after `^https://`.

**What it does not take.** CTR-013's comparisons across Types, which order 048a-1 left "to a
later order" (its decision 9, and [order 117](117-plan-s2-s3.md) in the sentence after its 048a-1
entry): client IDs shared across addresses at install, registration and replacement,
`confirmClientIds`, the REC-002 record, and the `logins` outputs of `install.v1` and
`inspect.v1`. They compare one Type with what the engine holds, which is not this behavior and
not one behavior among themselves (FR-264); the coordinator writes them as the orders that
"What this order does not do" names.

**When this order starts.** After nothing: it starts from `main` at `d891543f` or later. Stop,
report and change nothing if `conformance/language/contracts/sign-in-logins/case.yaml` does not
hold exactly 27 steps, or if any file decision 4 creates already exists.

**Who decided what.** Everything here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): reading the two points as schema changes, the
two patterns, CTR-013's clause, the three examples and three steps, and the declaration. The page
point settles FR-138's item 5 by tightening, a coordinator's fix under the full review's F121
(FR-138, routed to the coordinator), open to the owner's strike as IN_FORCE.md's "who decides"
keeps F1 to F144, and logs for the owner; it moves none of S1's author counts, since it touches
no CLI flag, short-form key, template or default. Apply them as written and report them under
`decisions` as the coordinator's.

## What to read

- `spec/schemas/contracts.yaml`, the Schema `sign-in-login.v1`: `authorization`'s pattern, the
  `redirect` object's `path`, and `page`'s pattern.
- `spec/promises/contracts.yaml`, CTR-013's `then`.
- `spec/interface/implementation.yaml`, IFC-049's `then` (the URL "contains no credential or
  personal data") and its `observable` (the paragraph that begins "A sign-in-hook.v1 URL matches
  exactly one login").
- `spec/promises/pending.yaml`, PND-013's fourth check.
- `spec/schemas/README.md`, the section "Logins a Type states" and the table row for
  `examples/invalid`.
- `conformance/language/contracts/sign-in-logins/case.yaml`, steps 21 to 26 (the page and
  service patterns) and step 8 (the redirect host), whose shape the new steps follow.

## The decisions it applies

1. **The two patterns of `sign-in-login.v1`.** In `spec/schemas/contracts.yaml`:
   - The line `        path: {type: string, pattern: '^/'}` in the `redirect` object becomes
     `        path: {type: string, pattern: '^/[^?#]*$'}`. A path still begins with `/`, and now
     holds neither `?` nor `#` anywhere. Nothing else about the path changes: a space or a
     backslash stays allowed (see decision 7).
   - `page`'s `pattern` line becomes `authorization`'s `pattern` line with only its last group,
     `(/[^?# \\]*)?`, replaced by `page`'s own last group, the path-and-query group that
     follows `page`'s port group (it begins with a `/` or a `?` and holds no `#`). The effect is to
     delete `page`'s optional user-information group, the 15 characters right after
     `^https://`, and nothing else: the host and port groups, which `authorization`'s line and
     `page`'s share character for character, and `page`'s path-and-query group stay as they are.
     So a page keeps its query, which IFC-049 allows ("queries are allowed" in the README), and
     still refuses a fragment.

   Nothing else in the file changes: not `sign-in-login.v1`'s `description`, not the header
   comment, not any other pattern.

2. **CTR-013 states both rules.** In `spec/promises/contracts.yaml`, CTR-013's `then`, the line

   ```text
       loopback host, ports and path, and page and service the HTTPS page used as redirect_uri.
   ```

   becomes these two:

   ```text
       loopback host, ports and a path that holds neither ? nor #, and page and service the HTTPS
       page used as redirect_uri, which holds no user information.
   ```

   Nothing else in CTR-013 changes: not its `when`, `observable`, `level` or `vectors`.

3. **The schemas README.** In `spec/schemas/README.md`, section "Logins a Type states", the four
   lines from `hosted redirect pages without fragments (queries are allowed), loopback redirect
   hosts, ports` to `section 3.1).` become:

   ```text
   hosted redirect pages without fragments or user information (queries are allowed), loopback
   redirect hosts and ports, redirect paths without `?` or `#`, and carried and issuer where
   allowed. It rejects every unlisted member. A device login states its clientId too, though its
   relayed verification URL carries none (RFC 8628 section 3.1).
   ```

   In the table of example directories, the row for `examples/invalid` says `77 files` where it
   says `74 files`. Change nothing else in the file.

4. **Three examples and three vector steps.** Three files under `spec/schemas/examples/invalid/`,
   each one Type document that `sign-in-login.v1` rejects and named for CTR-013, the promise it
   breaks: `CTR-013-redirect-path-with-a-query.yaml` (path `/callback?flow=browser`),
   `CTR-013-redirect-path-with-a-fragment.yaml` (path `/callback#done`) and
   `CTR-013-page-with-user-information.yaml` (page `https://user`, then the at sign, U+0040, then
   `example.org/code`). Each is the Resource Type `signed-resource.v1` with a string-or-null
   `account`, the three hooks under `requires` and one login. In the case
   `contracts-sign-in-logins`, three fixtures and three steps appended after step 27, each a
   `load` of one fixture that is rejected with one SUB-005 diagnostic at a `pattern` of
   `sign-in-login.v1`, as steps 21 to 26 are:
   - step 28, `redirect-path-query.json`: `agent.json` with login 0's `redirect.path`
     `/callback?flow=browser`; pointer `/properties/redirect/properties/path/pattern`;
   - step 29, `redirect-path-fragment.json`: `agent.json` with login 0's `redirect.path`
     `/callback#done`; the same pointer;
   - step 30, `page-user-information.json`: `resource.json` with login 0's `page` the same value
     as the third example's; pointer `/properties/page/pattern`.

   The loader's test `every_invalid_example_is_rejected_by_the_step_its_name_gives` reads the
   verdict of `check_schemas.py` on each file under `invalid/` from the fixture
   `engine/crates/loader/tests/expected/invalid-verdicts.txt`, whose header says that a file
   added there needs a line. The fixture gains three lines,
   `CTR-013-page-with-user-information.yaml schema`,
   `CTR-013-redirect-path-with-a-fragment.yaml schema` and
   `CTR-013-redirect-path-with-a-query.yaml schema`, in that order, right after the line for
   `CTR-012-control-input-schema-with-another-root-keyword.yaml`, where their names sort. Without
   them that test fails, as the coordinator's scratch run showed.

   Steps are appended, not inserted, so steps 1 to 27 keep their indices. Each fixture and the
   case are written as the case's files are, JSON with two-space indentation and a final newline.
   The case's `id`, `layers`, `promises` (`[CTR-013, SUB-005]`), `title`, `profile` and
   `initial` do not change, so its coverage index entry and `engine/STANDING.md` do not either.

   The script below applies decisions 1 to 4 exactly. It writes the at sign as `chr(64)`, reads
   the `apiVersion` and the three `requires` entries from `resource.json`, and stops on an
   assertion, before or part way through its edits, if a file is not as this order expects; then
   restore every file it touched with `git checkout --`, delete any file it created, stop and
   report the assertion. From the repository root, write it to
   `engine/target/order-206-write.py` (run `mkdir -p engine/target` first; Git ignores `target/`)
   and run `python3 engine/target/order-206-write.py`. Delete it before you finish.

   ```python
   import json
   from pathlib import Path

   AT = chr(64)
   CASE = Path("conformance/language/contracts/sign-in-logins")
   EXAMPLES = Path("spec/schemas/examples/invalid")


   def load(name):
       return json.loads((CASE / name).read_text())


   def write_json(name, value):
       (CASE / name).write_text(json.dumps(value, indent=2) + "\n")


   def replace_once(path, old, new):
       text = Path(path).read_text()
       assert text.count(old) == 1, (path, old)
       Path(path).write_text(text.replace(old, new))


   # Decision 1: the two patterns of sign-in-login.v1.
   replace_once("spec/schemas/contracts.yaml",
                "        path: {type: string, pattern: '^/'}\n",
                "        path: {type: string, pattern: '^/[^?#]*$'}\n")
   schema = Path("spec/schemas/contracts.yaml").read_text()
   block = "      type: string\n      format: uri\n      pattern: "
   auth_line = schema.split("    authorization:\n" + block, 1)[1].split("\n", 1)[0]
   page_line = schema.split("    page:\n" + block, 1)[1].split("\n", 1)[0]
   assert auth_line.endswith("(/[^?# \\\\]*)?$'"), auth_line
   new_page_line = auth_line[:-len("(/[^?# \\\\]*)?$'")] + "([/?][^# \\\\]*)?$'"
   assert page_line.replace("([^/?#" + AT + " \\\\]*" + AT + ")?", "") == new_page_line, page_line
   replace_once("spec/schemas/contracts.yaml",
                "    page:\n" + block + page_line + "\n",
                "    page:\n" + block + new_page_line + "\n")

   # Decision 2: CTR-013's then.
   replace_once("spec/promises/contracts.yaml",
                "    loopback host, ports and path, and page and service the HTTPS page used as redirect_uri.\n",
                "    loopback host, ports and a path that holds neither ? nor #, and page and service the HTTPS\n"
                "    page used as redirect_uri, which holds no user information.\n")

   # Decision 3: spec/schemas/README.md.
   replace_once("spec/schemas/README.md",
                "hosted redirect pages without fragments (queries are allowed), loopback redirect hosts, ports\n"
                "and paths, and carried and issuer where allowed. It rejects every unlisted member. A device\n"
                "login states its clientId too, though its relayed verification URL carries none (RFC 8628\n"
                "section 3.1).\n",
                "hosted redirect pages without fragments or user information (queries are allowed), loopback\n"
                "redirect hosts and ports, redirect paths without `?` or `#`, and carried and issuer where\n"
                "allowed. It rejects every unlisted member. A device login states its clientId too, though its\n"
                "relayed verification URL carries none (RFC 8628 section 3.1).\n")
   replace_once("spec/schemas/README.md",
                "| [examples/invalid](examples/invalid) | 74 files,",
                "| [examples/invalid](examples/invalid) | 77 files,")


   resource = load("resource.json")
   HEADER = """#
   # This file carries no change narrative; the reasons are in the records listed in spec/README.md, 'Where the reasons are'.

   apiVersion: {api}
   kind: Type
   metadata:
     name: signed-resource.v1
   spec:
     family: Resource
     schema:
       type: object
       properties:
         account: {{type: [string, 'null']}}
       required: [account]
     requires:
   {requires}  signIn:
       account: /account
       logins:
   """.format(api=resource["apiVersion"],
              requires="".join(f"  - {r}\n" for r in resource["spec"]["requires"]))
   REDIRECT = """    - name: browser
         return: redirect
         authorization: 'https://example.org/authorize'
         clientId: placeholder-resource-browser
         redirect: {{host: 127.0.0.1, ports: [8400], path: '{path}'}}
         carried: true
   """
   PAGE = """    - name: page
         return: page
         authorization: 'https://example.org/authorize'
         clientId: placeholder-resource-page
         page: '{page}'
         carried: false
   """
   examples = [
       ("CTR-013-redirect-path-with-a-query.yaml",
        "a redirect login's `path` holds `?`, which no redirect_uri's path can",
        REDIRECT.format(path="/callback?flow=browser")),
       ("CTR-013-redirect-path-with-a-fragment.yaml",
        "a redirect login's `path` holds `#`, which no redirect_uri's path can",
        REDIRECT.format(path="/callback#done")),
       ("CTR-013-page-with-user-information.yaml",
        "a page login's `page` holds user information, which no relayed URL may",
        PAGE.format(page="https://user" + AT + "example.org/code")),
   ]
   for name, first, login in examples:
       path = EXAMPLES / name
       assert not path.exists(), path
       text = f"# Example. Invalid: {first}\n# hold (CTR-013, IFC-049).\n" + HEADER + login
       path.write_text(text)
   replace_once("engine/crates/loader/tests/expected/invalid-verdicts.txt",
                "CTR-012-control-input-schema-with-another-root-keyword.yaml schema\n",
                "CTR-012-control-input-schema-with-another-root-keyword.yaml schema\n"
                + "".join(f"{name} schema\n" for name, _, _ in sorted(examples)))

   fixtures = []
   agent = load("agent.json")
   agent["spec"]["signIn"]["logins"][0]["redirect"]["path"] = "/callback?flow=browser"
   fixtures.append(("redirect-path-query.json", agent,
                    "/properties/redirect/properties/path/pattern"))
   agent = load("agent.json")
   agent["spec"]["signIn"]["logins"][0]["redirect"]["path"] = "/callback#done"
   fixtures.append(("redirect-path-fragment.json", agent,
                    "/properties/redirect/properties/path/pattern"))
   resource = load("resource.json")
   resource["spec"]["signIn"]["logins"][0]["page"] = "https://user" + AT + "example.org/code"
   fixtures.append(("page-user-information.json", resource, "/properties/page/pattern"))

   case = load("case.yaml")
   assert len(case["steps"]) == 27, len(case["steps"])
   for name, value, pointer in fixtures:
       assert name not in case["files"] and not (CASE / name).exists(), name
       write_json(name, value)
       case["files"][name] = name
       case["steps"].append({
           "input": {"kind": "load", "files": [name]},
           "expect": {
               "accepted": False,
               "diagnostics": [{"ruleId": "SUB-005", "level": "error"}],
               "diagnostic_facts": [{
                   "index": 0,
                   "message_contains": ["sign-in-login.v1"],
                   "keyword_location": True,
                   "keyword": "pattern",
                   "contract": "sign-in-login.v1",
                   "pointer": pointer,
                   "root_schema": "contract.v1",
               }],
           },
       })
   write_json("case.yaml", case)
   print(f"edited 4 files; wrote {len(examples)} examples, {len(fixtures)} fixtures; the case has {len(case['steps'])} steps")
   ```

   The code block is indented three spaces for this list; save it with that indent removed, so
   that `import json` starts its line. No expectation is generated from the engine's output.

5. **The declaration.** `docs/work-orders/206-declared.yaml` comes with this order, with four
   shapes: the added inputs (the three `files` entries and the three steps' `input`), the three
   flips `absent -> false` of `/steps/27/expect/accepted` to `/steps/29/expect/accepted`, the
   added expectations, and the three added fixtures. The coordinator ran it against this work
   in a scratch copy of `d891543f`; check that it matches and change it only to make it so.

6. **Nothing else changes.** No source under `engine/`, `packages/` or `experiments/`, and no
   other test fixture; not order
   048a-1's overlap test in `engine/crates/core/src/packages.rs`, which compares redirect paths
   as strings and is now exact; not IFC-049, PND-013, IN_FORCE.md, `engine/STANDING.md`,
   `conformance/manifest.yaml`, the coverage index, `conformance/expected-failures.yaml` or
   `docs/proposals/`.

7. **What this order does not do.**
   - **CTR-013's comparisons across Types**, in three orders the coordinator writes next, each
     after the one before (FR-264; the coordinator's split, open to the owner's strike):
     1. the shared client ID at registration and replacement: a submitted project's Type whose
        login states a client ID that a Type held at another address also states (an installed
        package's, or a submission's that a template or instance holds) reports conflict under
        CTR-013, after access and validity in OPS-023's order and before dispatch, with no
        confirmation; the diagnostic names CTR-013, its cause, the client ID and both Types, and
        holds no URL (`register.v1`'s description, SUB-009 for `replace.v1`);
     2. the same comparison at `install.v1`, with `confirmClientIds`: conflict unless confirmed;
        refused where a caller other than root gives it; invalid for an ID shared with no such
        Type; and REC-002 recording only the confirmed client IDs as inputs;
     3. the optional `logins` outputs of `install.v1` (each package Type stating signIn, by its
        qualified reference) and of `inspect.v1` (its object's Type's logins).
   - **A space or a backslash in a redirect path**, which `authorization`'s pattern refuses in its
     path and the new path pattern still allows. No verifier raised it; it is left for the
     coordinator to weigh, and is not changed here.

## Scope

Change only `spec/schemas/contracts.yaml` (two pattern lines), `spec/promises/contracts.yaml`
(CTR-013's `then`, one line becoming two), `spec/schemas/README.md` (one paragraph and one table
row), `conformance/language/contracts/sign-in-logins/case.yaml` and
`engine/crates/loader/tests/expected/invalid-verdicts.txt` (three lines added). Create only the
three examples and the three fixtures decision 4 names. `docs/work-orders/206-declared.yaml`
comes with this order; change it only as decision 5 says. Anything else that needs changing goes
in your report.

## Done when

From the repository root, after the script has run, each of these passes. `/usr/bin/python3` is
the Mac's interpreter with PyYAML; where `python3` has PyYAML, it serves as well.

The script printed exactly `edited 4 files; wrote 3 examples, 3 fixtures; the case has 30 steps`.
This prints eight lines, each ending `: OK`, and exits 0 (`sha256sum -c` on Linux):

```bash
shasum -a 256 -c <<'EOF'
2f400d47ce903742f97cb01c96200dd40da29076ef901f93750d0c23fcc4760c  conformance/language/contracts/sign-in-logins/case.yaml
9e1c049c24701e8d012f37be3321cd33ca3702870cf02dcb229ed6370d2adfd3  conformance/language/contracts/sign-in-logins/redirect-path-query.json
51f908756e8992fd25910276406910824d28dae4b838be1d78fffcfc1b2fa94f  conformance/language/contracts/sign-in-logins/redirect-path-fragment.json
7fc01d9cca9a2ba3f546cd71e34246155be8728baf87713c032dbba35289d003  conformance/language/contracts/sign-in-logins/page-user-information.json
e8c99ebd4eecf6c2f9d3c693c7f346c92fdc75921b8febaf7576ae8ef5e1b84b  spec/schemas/examples/invalid/CTR-013-redirect-path-with-a-query.yaml
e346a6c748f94426f09dcd19cc07cc7e3bee90584fbffd4a184fd88b5663914e  spec/schemas/examples/invalid/CTR-013-redirect-path-with-a-fragment.yaml
8bdd82acf0f8b503912bbcac89b6054261ffe67f41bac71b019c3e79d332f1ba  spec/schemas/examples/invalid/CTR-013-page-with-user-information.yaml
96fa3010210ab5ff628c9983b4740e028560244d9d59f9fcd49809fc86ca3242  engine/crates/loader/tests/expected/invalid-verdicts.txt
EOF
```

The hashes of the case and of `invalid-verdicts.txt` are those of `d891543f`'s files with this
order's change; if either file changed on `main` since, report its hash, and the other six must
still match.

This prints `[True, True, False, False] [True, True, False, False]`: the page pattern accepts a
page with and without a query and refuses user information with and without a password, and the
path pattern accepts `/callback` and `/` and refuses `?` and `#` (on `d891543f` it prints `True`
eight times):

```bash
/usr/bin/python3 -c "import re, yaml; d=[x for x in yaml.safe_load_all(open('spec/schemas/contracts.yaml')) if x and x['metadata']['name'] == 'sign-in-login.v1'][0]['spec']['properties']; p=re.compile(d['page']['pattern']); q=re.compile(d['redirect']['properties']['path']['pattern']); a=chr(64); print([bool(p.search(u)) for u in ['https://example.org/code', 'https://example.org/code?flow=page', 'https://user' + a + 'example.org/code', 'https://user:pw' + a + 'example.org/code']], [bool(q.search(x)) for x in ['/callback', '/', '/callback?x', '/callback#x']])"
```

Each of these prints 1. The `tr` checks read a file with its line breaks as spaces:

```bash
tr -s ' \n' '  ' < spec/promises/contracts.yaml | grep -c 'redirect the loopback host, ports and a path that holds neither ? nor #, and page and service the HTTPS page used as redirect_uri, which holds no user information. Only redirect and page state carried'
tr -s ' \n' '  ' < spec/schemas/README.md | grep -c 'hosted redirect pages without fragments or user information (queries are allowed), loopback redirect hosts and ports, redirect paths without `?` or `#`, and carried and issuer where allowed.'
grep -c '^| \[examples/invalid\](examples/invalid) | 77 files,' spec/schemas/README.md
grep -cF "path: {type: string, pattern: '^/[^?#]*\$'}" spec/schemas/contracts.yaml
```

This prints exactly these four lines, tab-separated, so no other line of the four files changed:

```bash
git diff --numstat -- engine/crates/loader/tests/expected/invalid-verdicts.txt spec/promises/contracts.yaml spec/schemas/README.md spec/schemas/contracts.yaml
```

```text
3	0	engine/crates/loader/tests/expected/invalid-verdicts.txt
2	1	spec/promises/contracts.yaml
5	5	spec/schemas/README.md
2	2	spec/schemas/contracts.yaml
```

Each of these passes:

```bash
/usr/bin/python3 tools/check_schemas.py | grep -c 'rejected CTR-013-'
/usr/bin/python3 tools/check_schemas.py | tail -1
/usr/bin/python3 tools/set_names.py --check
/usr/bin/python3 tools/check_rulings.py
/usr/bin/python3 tools/check_docs.py
/usr/bin/python3 tools/check_roadmap.py
/usr/bin/python3 tools/promise_standing.py --check
/usr/bin/python3 -m unittest discover -s tools/tests
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/206-declared.yaml | grep -v '^{'; echo "exit ${PIPESTATUS[0]}"
git diff --check
```

The first prints 3, and `check_schemas.py`'s step 4 reads `4. 77 invalid documents in 77 files
were each rejected by a schema, or rejected when read.`; the second prints `Errors: 0`, and so do
`set_names.py`, `check_rulings.py`, `check_docs.py` and `check_roadmap.py` on their last lines.
`promise_standing.py --check` prints `engine/STANDING.md is current; promise rows: 520.` and
`Errors: 0`. The unit tests end `OK` (129 tests at `d891543f`; report the count). The vector
delta prints three `Flip:` lines, each `declared by "each added step is rejected"`, then
`Declarations read from docs/work-orders/206-declared.yaml: 4 shapes.`,
`Vector delta: 42 changes; 3 flips; 0 undeclared; 0 errors.` and `exit 0`; run it under bash,
whose `PIPESTATUS` it reads. `git diff --check` prints nothing.

From `engine/`, one command at a time, each passes:

```bash
cargo test --release -p conduit-loader --test examples
cargo run --release -p conduit-conformance -- .. contracts-sign-in-logins
cargo run --release -p conduit-conformance -- .. contracts-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The first reports `8 passed; 0 failed`; the second prints
`PASS contracts-sign-in-logins (30 steps; CTR-013, SUB-005)` and ends `1 cases passed`; the third
ends `6 cases passed`; the vectors report `2 passed; 0 failed`; and
no line of the full suite reports a failure. The threads are 4 because at 24 the runner ran out
of memory (order 071's Record).

Next, show that the steps and the examples fail without decision 1. From the repository root,
one at a time:

```bash
cp spec/schemas/contracts.yaml engine/target/contracts.yaml.fixed
git show HEAD:spec/schemas/contracts.yaml > spec/schemas/contracts.yaml
(cd engine && cargo run --release -p conduit-conformance -- .. contracts-sign-in-logins)
/usr/bin/python3 tools/check_schemas.py | tail -4
cp engine/target/contracts.yaml.fixed spec/schemas/contracts.yaml
(cd engine && cargo run --release -p conduit-conformance -- .. contracts-sign-in-logins)
/usr/bin/python3 tools/check_schemas.py | tail -1
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed` followed by
`contracts-sign-in-logins [CTR-013, SUB-005]: expected diagnostic absent`; the first
`check_schemas.py` must print three lines `ERROR spec/schemas/examples/invalid/CTR-013-...:
accepted, and it must be rejected`, one for each example, then `Errors: 3`. After the restore
the runner ends `1 cases passed` and `check_schemas.py` prints `Errors: 0`. The engine reads the
schemas when it runs, so no rebuild comes between. Then
`git diff --numstat -- spec/schemas/contracts.yaml` prints `2	2	spec/schemas/contracts.yaml`
again.

The S1 short-form spike reads `spec/`, so from `experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`, and no `test result`
line reports a failure.

Last, after deleting `engine/target/order-206-write.py` and `engine/target/contracts.yaml.fixed`,
`git status --short` prints exactly these eleven lines, in this order:

```text
 M conformance/language/contracts/sign-in-logins/case.yaml
 M engine/crates/loader/tests/expected/invalid-verdicts.txt
 M spec/promises/contracts.yaml
 M spec/schemas/README.md
 M spec/schemas/contracts.yaml
?? conformance/language/contracts/sign-in-logins/page-user-information.json
?? conformance/language/contracts/sign-in-logins/redirect-path-fragment.json
?? conformance/language/contracts/sign-in-logins/redirect-path-query.json
?? spec/schemas/examples/invalid/CTR-013-page-with-user-information.yaml
?? spec/schemas/examples/invalid/CTR-013-redirect-path-with-a-fragment.yaml
?? spec/schemas/examples/invalid/CTR-013-redirect-path-with-a-query.yaml
```

Do not stage anything. In your report, mark each choice you made yourself as your own, and give
the last line of every check, each exit status and each count above.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then adds
the new files with intent to add and runs `python3 tools/check_private_paths.py` and
`git diff --check`: the page example and fixture hold an address at `example.org`, a domain
reserved for examples, which the check exempts. Before the merge the admin runs the local gate
that [ruling 80](../proposals/ruling-80/README.md), point 10, has each merge wait on,
`tools/gate.py`; CI runs on the pull request as well, and its `docs` job reads
`206-declared.yaml` through `--declared-changed`. A fresh Codex verifier and a fresh read-only
Opus reader read the result before it merges ([ruling 79](../proposals/ruling-79/README.md),
point 2).

## Record

Run on 2026-11-18, from about 20:39 to 21:35 UTC, on the Mac from `main` `ad58ae75` (this order's
text), in two rounds: the worker (1567 s) and a fresh Codex verifier (244 s), then the worker again
(1178 s) and a second fresh Codex verifier (331 s). The change is the eleven paths of Scope, written
by the order's script: decision 1's two patterns of `sign-in-login.v1`, `page`'s losing only its
user-information group and the redirect `path`'s becoming `'^/[^?#]*$'`; decision 2's two lines in
CTR-013's `then`; decision 3's paragraph and `77 files` in the schemas README; and decision 4's
three examples, three lines of `invalid-verdicts.txt`, and three fixtures and steps 28 to 30 of
`contracts-sign-in-logins`. `206-declared.yaml` matched as it came. All eight hashes matched, the
case passed its 30 steps, the full engine suite reported 351 passed, 0 failed and 4 ignored, and
the S1 spike 9 passed; with `HEAD`'s schema the case failed with `expected diagnostic absent` and
`check_schemas.py` printed three `accepted, and it must be rejected` lines and `Errors: 3`. The
worker's own choices: cargo offline on the pinned toolchain; a first extraction of the script that
dropped a blank line of the examples' header, so that three hashes missed, which it undid from
`HEAD` and reran with the blank lines kept; and an exit trap that restored the fixed schema in the
check without decision 1. Its two findings, 048a-1's redirect path and FR-138's item 5, report the
two gaps this order closes. Both verifiers found no must and the same two shoulds: `spec/README.md`
still counted 74 invalid examples where the schemas README now counts 77; and the new rejection's
SUB-005 message, the loader's pattern failure and `check_schemas.py`'s alike, quotes the rejected
`page` URL with its user information, a password included, which RFC 3986, section 3.2.1, advises
against rendering. Round 2 changed no file: both lie outside Scope, and the worker recorded them
for the coordinator.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on a scratch copy of
`origin/main` `6be93b30` (order 051b) with the branch's change applied; `main` had touched none of
the order's eleven paths since `ad58ae75`. It ran every check of "Done when": the eight hashes (OK),
the pattern probe (`[True, True, False, False] [True, True, False, False]`), the four counts (1
each), the numstat (the four lines), `check_schemas.py` (3 rejections of `CTR-013-`, step 4's 77,
Errors: 0), `set_names.py --check`, `check_rulings.py`, `check_docs.py`, `check_roadmap.py` and
`check_model_account.py` (Errors: 0 each), `promise_standing.py --check` (520 promise rows, Errors:
0), the tool tests (129, OK), the vector delta (three flips, each declared by
`each added step is rejected`, 4 shapes, `42 changes; 3 flips; 0 undeclared; 0 errors.`, exit 0),
the loader's examples (8 passed), the case (`PASS contracts-sign-in-logins (30 steps; CTR-013,
SUB-005)`), the six `contracts-` cases, the vectors (2 passed), the full engine suite (356 passed,
0 failed, 4 ignored), the check without decision 1 (the case failed, `Errors: 3`, and both passed
again after the restore), and the S1 spike (9 passed). With the new files added as intent-to-add,
`check_private_paths.py` printed Errors: 0 and `git diff --check` nothing; author and committer
were the work-order identity, and the only at signs added are the patterns' character classes and
the intended user-information URL at `example.org`, which decision 4 requires. It found no must and
three shoulds:

- R1: nothing logged the page tightening of CTR-013's `then`, or the redirect-path one beside it,
  for the owner's strike, as ruling 71, point 4, and this order's "Who decided what" require; the
  owner's review list ended at row 47.
- R2: `spec/README.md` still counted 74 invalid examples, where `spec/schemas/README.md` counts 77;
  Scope did not name it, and both Codex verifiers raised it.
- R3: the new rejection quotes the rejected `page` URL, user information included, in its SUB-005
  message, from the loader and from `check_schemas.py` alike; it is existing behavior, as for
  `authorization`, and outside Scope, but no Record carried it.

The fixer rebased the branch onto `origin/main` at `6be93b30` (order 051b) with the work-order
identity, without conflict, and applied each finding:

- R1: the table "From the targeted review of the sign-in surface" of
  `docs/proposals/first-build/to-review-with-the-owner.md` gains row 48, in the reader's words: a
  page or service login's `page` holds no user information, and a redirect login's `path` holds
  neither `?` nor `#`, the coordinator's tightening of CTR-013, open to the owner's strike under
  ruling 71, point 4. The tightening is row 48 of the owner's review list.
- R2: `spec/README.md`'s row for `schemas/examples/invalid` says `Draft; 77 files`. This widens
  Scope by that one line, and R1's row widens it under `docs/proposals/`, which decision 6 left
  unchanged; both are the coordinator's at the merge.
- R3: the code is unchanged. It is a finding for a follow-up loader or schema-checker order: a
  pattern failure for `sign-in-login.v1`'s `page`, `authorization` and `issuer` should leave out
  the instance value and keep the schema name, the instance pointer and the keyword location, with
  a step whose fixture holds a synthetic password and whose expectation checks that the password is
  absent from the message.

The fixer then ran on the rebased branch, with the files of this commit before it was made, every
check of "Done when", with `origin/main` in place of `HEAD` where the change is now committed: the
eight hashes (`: OK` each, exit 0), the pattern probe
(`[True, True, False, False] [True, True, False, False]`), the four counts (1 each), the numstat
against `origin/main` (the four lines), `check_schemas.py` (3 rejections of `CTR-013-`, step 4's
`77 invalid documents in 77 files`, Errors: 0), `set_names.py --check`, `check_rulings.py`,
`check_docs.py` and `check_roadmap.py` (Errors: 0 each), `promise_standing.py --check`
(`engine/STANDING.md is current; promise rows: 520.`, Errors: 0), the tool tests (129, OK), the
vector delta from `origin/main` (three flips, each declared by `each added step is rejected`,
4 shapes, `42 changes; 3 flips; 0 undeclared; 0 errors.`, exit 0), the loader's examples
(8 passed; 0 failed), the case (`PASS contracts-sign-in-logins (30 steps; CTR-013, SUB-005)`,
`1 cases passed`), `contracts-` (`6 cases passed`), the vectors (2 passed; 0 failed), the full
engine suite with 4 test threads and 4 vector threads (exit 0; 356 passed, 0 failed, 4 ignored),
the check without decision 1 on `origin/main`'s schema (`FAIL 0 cases passed; 1 failed`,
`expected diagnostic absent`, three `accepted, and it must be rejected` lines and `Errors: 3`;
after the restore `1 cases passed`, Errors: 0 and the numstat `2 2` again), and the S1 spike
(`tests/spike.rs` 9 passed; 0 failed). `python3 tools/check_docs.py` and
`python3 tools/check_private_paths.py` printed Errors: 0, and `git diff --check origin/main...HEAD`
and `git diff --check` printed nothing. The gate's run on this order's squash, which the merge
waits on, is the coordinator's at the merge.
