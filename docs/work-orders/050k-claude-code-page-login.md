# Work order 050k (P2k): `claude-code.v2` states its code-page login, not choosable in 0.2.0, and the sign-in hook's failures name PND-010

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order runs on the Mac or on the Linux node;
it needs neither. It has network access only for tests that bind a loopback port; it adds no
crate, so cargo fetches nothing, and you use the network for nothing else. Run one cargo command
at a time. Never print a whole large file: use `head`, `tail`, `grep`, `sed -n` or a count. Never
run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Never read or print any process's
environment or arguments, never run a command that lists every process on the machine, such as
`ps -e`, `ps aux` or `top`, and never signal a process that a test of this order did not start.
Never run the real `claude` or `codex` programs, not even for their help or version, never read
their program files, and sign in nowhere. Every `Config` a test builds and every wrapper a test
starts uses the package's scripted client (`--client-program`), as every test at `BASE` does, so
that no login ever reaches a real client or a real service. The tests use temporary directories
only, and the engine is run only to initialize a store and register a project.

**The login's literals.** The Type's second login states three values read from the client: its
authorization URL, its client ID and its page. They are written in
`packages/agent-claude-code/contracts.yaml` alone, by decision 1's script, from a file that the
coordinator puts in your tree before you start, `target/wo050k/login.json` (ignored build output,
never committed), as "Where the login's literals come from" says. Never print that file or any
value in it, copy none of them into any other file, and quote none in your report, a test's
output or a log: each value is named by its member (`authorization`, `clientId`, `page`), never
shown. If the file is missing, stop, change nothing and report.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: the Claude lane's branch, `wo/050-lane-claude`, at or after `7ff73bad`, which holds orders
050a, 076 and 078, 050a's Record, orders 050c to 050f and 050h, each with its Record, order 050i
with its Record, and order 050j, the lane's second, with its Record and A77's record of it
(`docs/work-orders/050j-a77.yaml`); `7ff73bad` is 050j's last commit, the coordinator's judgment
and the third read's fixes. This order is the third and last of the Claude lane (050i, 050j,
050k). The Codex lane (050l, 050m) is on a branch of its own and does not share `BASE`; neither
lane holds the other's work. `BASE`'s tip holds this order file, committed: the coordinator puts
it on the lane's branch after this text merges on `main`, and sets `BASE` to that branch's tip.
Before you change anything, check from the repository root that you find there what that gives,
and stop, change nothing and report if any check prints otherwise. The Python lines need a Python
with PyYAML, as `tools/check_schemas.py` does (AGENTS.md, "Checks").

```bash
P=packages/agent-claude-code
git merge-base --is-ancestor 7ff73bad "$BASE"; echo "lane $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050k-claude-code-page-login.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050j-claude-sign-in-redirect-login.md
python3 -c "import yaml; d=[d for d in yaml.safe_load_all(open('packages/agent-claude-code/contracts.yaml')) if d['metadata']['name'] == 'claude-code.v2'][0]; l=d['spec']['signIn']['logins']; print(len(l), l[0]['name'], l[0]['return'], l[0]['carried'], d['metadata']['description'].endswith(' and then empties that home.'))"
for s in 'pub const LOGIN_DID_NOT_START: &str = "client login did not start";' 'pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL";' 'break Err(anyhow!("client login failed: {cause}"));' 'Err(_) => break Err(anyhow!("client login failed: client exit: status unknown")),' '"client login failed: no URL within {} seconds",'; do grep -cF "$s" $P/src/sign_in.rs; done
grep -c 'PND-010' $P/src/sign_in.rs
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs
grep -cE '#\[(tokio::)?test' $P/tests/wrapper.rs
for s in "/// claude-code.v2's one stated login, read from" '    assert_eq!(logins.len(), 1);' '    let prefix = format!("http://{}:", redirect["host"].as_str().unwrap());' 'eprintln!("sign-in IFC-049 failures {failures:?}");' 'Some("client login failed: client exit: status 3"),' 'Some("client login failed: no URL within 3 seconds"),'; do grep -cF "$s" $P/tests/wrapper.rs; done
grep -c 'PND-010' $P/tests/wrapper.rs
for s in 'and it states one login, `browser`, a redirect login with `carried: false`\.' 'The login.s literals, its authorization URL, its client ID and its redirect.s host and path, are in `contracts.yaml` alone\.' 'Its failures use fixed texts:' 'which holds a second URL, is discarded' 'The login and its hand-over were read from' 'and printed a second URL\.' 'without a name, the hook starts its one login\.' 'The code-page login is order 050k.s\.'; do tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c -- "$s"; done
grep -c '"name": "code-page"' $P/contracts.yaml
python3 -c "import json; c=[x for x in json.load(open('experiments/027-a5-sign-in/results-2026-11-16.json'))['clients'] if x['client'] == 'claude-code'][0]; s=c['steps']['4']; print(c['version'], s['shim_called'], s['login_args'], [(u['source'], u['return'], u['redirect_host'] is not None, u['ifc_049'], u['listens']) for u in s['urls']])"
python3 -c "import json, yaml; l=json.load(open('target/wo050k/login.json')); b=[d for d in yaml.safe_load_all(open('packages/agent-claude-code/contracts.yaml')) if d['metadata']['name'] == 'claude-code.v2'][0]['spec']['signIn']['logins'][0]; print(sorted(l), l['authorization'] == b['authorization'] and l['clientId'] == b['clientId'], all(isinstance(v, str) and v.startswith('https://') for v in (l['authorization'], l['page'])), l['page'] != l['authorization'])"
```

They print, in order:

- `lane 0`, `head 0`, `order 0` and `0`; then `1` (050j's Record).
- From order 050j (one line): `1 browser redirect False True` (one login, `browser`, a redirect
  login with `carried: false`, and the description ending with order 050f's retire sentence).
- From order 050j's `src/sign_in.rs`: `1` for each of the five fixed failure texts that decision 2
  changes, then `0` (no text names PND-010).
- `18` and `26` (the two `wrapper.rs`).
- From order 050j's `tests/wrapper.rs`: `1`, `1`, `1` and `1` (the helpers and the line that
  decision 3 changes), `2` and `1` (the two expected failure texts written out), then `0`.
- From order 050j's README: `1` for each of the eight texts that decision 4 changes.
- `0` (no code-page login yet).
- Stage 0's facts for Claude Code's login (one line):

  ```text
  2.1.318 True [] [('shim', 'redirect', True, [], 'yes'), ('printed', 'page', False, [], 'not checked')]
  ```

  (version 2.1.318; the login called the browser shim with a loopback redirect URL, and printed a
  second URL, a page login's, for which the probe's checks of IFC-049 found no failure).
- The literals file: `['authorization', 'clientId', 'page'] True True True` (its members; the
  authorization and the client ID equal the browser login's; the authorization and the page are
  `https://` strings, and differ; no value printed).

**What it builds.** `claude-code.v2` states a second login, `code-page`: Claude Code's own
code-page login, `return: page`, `carried: false`, with the authorization URL and client ID that
the browser login already states and the page that the client uses as its `redirect_uri`. It is
stated now and cannot be chosen in 0.2.0: the `wrapper-v2` item stays `plain`, the engine, once
order 209 builds order 207's text, refuses a `signInLogin` aimed at a plain hook before any part
runs ("What the specification already says"), and without a name the hook starts the browser login,
as order 050j built it. The Type's description lists the code-page login. The sign-in hook's fixed
failure texts each end with ` (PND-010)`, as `sign-in-hook.v1` asks. The tests read both logins from
`contracts.yaml` and show that each login's URL, in the form of the client's builder, passes
IFC-049's checks for its own login and fails the other's at `redirect_uri` alone, and that the URL
the hook relays fails the code-page login's checks there. Registration with the engine shows that
the Type, stating two logins, passes CTR-013's checks.

It carries, from the coordinator's split of order 050 (its 050k entry, "claude-code.v2's
code-page login, stated beside the browser login"):

- "Item 2's code-page login: return: page, carried: false, literals read from the client; stated
  now (ruling 82 point 4) and not choosable in 0.2.0: the engine refuses a signInLogin aimed at the
  plain hook before any part (ruling 86, point 3 (c)), and without a name the hook starts the
  browser login". The targeted review's
  [`claude-code.v2` list](../proposals/review-signin-2026-11-16/README.md#050-claude-codev2), which
  the owner accepted with [ruling 82](../proposals/ruling-82/README.md), item 2: "The logins: the
  redirect login and, if Q5's a and Q1 (vi) stand, the code-page login (`return: page`, `carried:
  false`), every literal read from the client at a version the README names [...] Done when a
  package test with a fake client shows each login's URL passing IFC-049's match, and the README
  names the client version read." Q5's a stands: ruling 82, point 4, "A login that 0.2.0 does not
  state never joins `.v2`".
- "Item 3 (o1), amended by the owner's ruling 86, point 1 (2026-11-18): the item that implements
  sign-in-hook.v1 is no longer keyed here; it stays plain (plain: true stays) in 0.2.0 [...]; no
  package test of signInLogin reaching the hook". So the review's item 3 done-when, a package
  test of `signInLogin` reaching the hook, is not this order's, nor any order's of 0.2.0.
- "Q1 (vi): claude-code.v2's description lists the code-page login". Ruling 82, point 5, the
  owner's answer 7 ("All six, keep the clause"): "(vi) `claude-code.v2` lists Claude's code-page
  login."

It carries also, from the Record of [order 050j](050j-claude-sign-in-redirect-login.md) on the
Claude lane's branch, the second verifier's finding (4), which the coordinator judged a should
and the order's fault: "`sign-in-hook.v1` asks a hook that cannot start a login to fail with a
diagnostic naming PND-010 and why, and decision 4's fixed texts name none: it is carried to order
050k, which ends those texts with ` (PND-010)`". 050j's Record leaves no other finding to this
order.

**Where the login's literals come from.** Claude Code has no public source. As for order 050j,
the coordinator read the login from the client's own program at the version that the package's
README names, 2.1.318: `bin/claude.exe` of Claude Code's npm package (`claude-code`, in the
`anthropic-ai` scope) at version 2.1.318, the macOS arm64 build, whose bundled modules each open
with the line `// Version: 2.1.318`. It ran no client, read no file but that program, and printed
no value. The program holds, by identifier:

- the OAuth configuration of the production service, an object whose members
  `CLAUDE_AI_AUTHORIZE_URL` and `CLIENT_ID` are the browser login's `authorization` and
  `clientId`, which order 050j stated, and whose member `MANUAL_REDIRECT_URL`, a string literal,
  is the code-page login's `page` (an `https://` URL; the development service's configuration
  builds its URLs from template strings and is not this one);
- the function that builds the authorization URL, whose `redirect_uri` is `MANUAL_REDIRECT_URL`
  for the code-page URL and the loopback template for the browser login's;
- the OAuth flow, which builds both URLs from one state and one challenge, the code-page URL with
  `isManual` true and the browser login's with it false, where its listener started; gives the
  code-page URL to its caller; and then opens the browser login's;
- `auth login`'s handler, which prints the URL it is given and reads its standard input by lines,
  each a code and a state separated by `#`, which it hands to the flow as a code pasted by hand.

So the code-page login is the second route of one `claude auth login`: the person opens the
printed URL, the service's page shows a code, and the person pastes it, with its state, into that
login's standard input on the Agent's machine. The coordinator's script, below, read those
identifiers and wrote the three values into `target/wo050k/login.json` as
`{"authorization", "clientId", "page"}`, with mode 0600. It runs with `CLAUDE_PROGRAM` naming that
program file, under a Python run with `-I`, from the tree's root; it prints only counts, and must
print exactly the line below it. A reader who has that program may run it again and compare the
file with `contracts.yaml` by "Done when"'s check; you do not run it.

```python
import json, os, re
program = open(os.environ["CLAUDE_PROGRAM"], "rb").read()
versions = sorted(set(v.decode() for v in re.findall(rb"\n// Version: ([0-9.]+)\n", program)))
authorize = re.findall(rb'CLAUDE_AI_AUTHORIZE_URL:"([^"]+)"', program)
start = program.find(b'CLAUDE_AI_AUTHORIZE_URL:"')
config = program[start:program.find(b"}", start)]
client = re.findall(rb',CLIENT_ID:"([^"]+)"', config)
page = re.findall(rb',MANUAL_REDIRECT_URL:"([^"]+)"', config)
builder = re.findall(rb'"redirect_uri",[\w$]+\?[\w$]+\(\)\.MANUAL_REDIRECT_URL:`', program)
both = re.findall(rb'([\w$]+)=[\w$]+\(\{\.\.\.([\w$]+),isManual:!0\}\),([\w$]+)=[\w$]+===null\?void 0:[\w$]+\(\{\.\.\.\2,isManual:!1\}\)', program)
order = re.findall(rb'if\(([\w$]+)===void 0\)await ([\w$]+)\(([\w$]+)\);else if\([\w$]+\?\.skipBrowserOpen\)await \2\(\3,\1\);else await \2\(\3\),await [\w$]+\(\1\)', program)
paste = re.findall(rb'\(\{input:process\.stdin\}\);[\w$]+\.on\("line",\(([\w$]+)\)=>\{let\[([\w$]+),([\w$]+)\]=\1\.trim\(\)\.split\("#"\);if\(!\2\|\|!\3\)\{process\.stderr\.write\(`[^`]*`\);return\}[\w$]+\("tengu_oauth_manual_entry",\{\}\),[\w$]+\.handleManualAuthCodeInput\(\{authorizationCode:\2,state:\3\}\)\}\)', program)
shape = re.compile(rb"https://(\[[^/?#\x40 \\]+\]|[^:/?#\x40\[\] \\]+)(:[0-9]*)?([/?][^# \\]*)?")
print(versions, len(authorize), len(client), len(page), len(builder), len(both), len(order), len(paste), all(shape.fullmatch(p) for p in page))
login = {"authorization": authorize[0].decode(), "clientId": client[0].decode(), "page": page[0].decode()}
os.makedirs("target/wo050k", exist_ok=True)
with os.fdopen(os.open("target/wo050k/login.json", os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600), "w") as f:
    json.dump(login, f)
```

```text
['2.1.318'] 1 1 1 1 1 1 1 True
```

(one version; one authorization, one client ID and one page in the production configuration; the
builder's code-page `redirect_uri`; both URLs built from one state and one challenge; the code-page
URL given to the caller before the browser login's is opened; the paste read from standard input
as a code and a state; and the page of the form `sign-in-login.v1`'s `page` pattern accepts, which
admits no user information). Stage 0 shows the same at 2.1.318 from outside
([A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass), its JSON as the
start check reads it): with no terminal, the login printed a page URL for which the probe's checks
of IFC-049 found no failure. Stage 0 checked no literal, and no code-page login was completed.

**What the specification already says, so that nothing here changes it.**
`sign-in-login.v1` ([`spec/schemas/contracts.yaml`](../../spec/schemas/contracts.yaml)): a login
with `return: page` states `page`, an `https://` URL, and `carried`, and no `redirect` or
`issuer`. [CTR-013](../../spec/promises/contracts.yaml): "page and service [state] the HTTPS page
used as redirect_uri, which holds no user information. Only redirect and page state carried,
required for both [...] Names differ within the Type [...] No URL may pass IFC-049's checks for
two of its logins". [IFC-049](../../spec/interface/implementation.yaml): "A sign-in-hook.v1 URL
matches exactly one login the instance's Type states, and the one asked for where present
(PND-010) [...] For redirect, page and service, client_id equals clientId, response_type is code,
and redirect_uri is for redirect http:// with the stated host, one stated port (or 1024 to 65535
for any) and stated path, without query; for page and service it is the stated page. For redirect
and page, code_challenge_method is S256, code_challenge is 43 base64url-alphabet characters (RFC
7636), and state is at least 22 RFC 3986 unreserved characters". So the two logins share
`authorization` and `clientId`, and differ in `redirect_uri`: no URL passes the checks for both.
`sign-in-hook.v1` ([`spec/core/operations.yaml`](../../spec/core/operations.yaml)): "Start one of
its Type's stated logins, the one signInLogin asks for where present, or fail with a diagnostic
naming PND-010 and why"; for a page login on the engine's retry, the return goes to "the program's
input (PND-013)", which `carried: false` never asks for. [PND-010](../../spec/promises/pending.yaml)
as order 207 (on `main`, not on this branch) amends it under ruling 86, point 3 (c) (order 207's
text is merged on `main`, but neither its change to `spec/` nor order 209's engine build has
landed, on `main` or on this branch; until they land, a request naming `code-page` reaches the plain
hook, which gets no name and starts the browser login, whose URL fails IFC-049's match for
`code-page`): a `signInLogin` naming a stated login "reports invalid where the instance's
sign-in-hook.v1 is a plain hook [...] with a diagnostic naming PND-010 and the login", before any
part; "A request may start its asked-for login or, without one, any of its Type's logins". The
engine checks every URL and every name; the hook does neither.

**What the branch already does, so that you do not redo it.** At `BASE`, in
`packages/agent-claude-code/`, as the start check shows:

- `contracts.yaml`'s `claude-code.v2` states `signIn` with the account place `/account` and one
  login, `browser`, written as one JSON line under `logins`, and requires the four hooks, each
  served by `#wrapper-v2` (order 050j). `conduit.yaml`'s `wrapper-v2` item is `plain: true` and
  maps `sign-in-hook.v1` to the tool `sign-in`.
- `src/sign_in.rs` has order 050j's sign-in hook, which runs `claude auth login` with the wrapper's
  own program as its browser, relays the first URL handed over, discards the client's standard
  input, output and error, and fails with fixed texts: `LOGIN_DID_NOT_START`,
  `LOGIN_URL_UNUSABLE` and three `client login failed:` formats.
- `tests/wrapper.rs` has 26 tests and order 050j's helpers `stated_login`, which reads the one
  login from `contracts.yaml`, `login_template`, `ifc_049_failures`, for a redirect login,
  `redirect_port`, `assert_no_url_canary`, `assert_handed_over_url`, `sign_in_setup` and
  `process_gone`, and its three sign-in tests.
- The README's `## The .v2 sign-in hook` describes one login and ends with "The code-page login is
  order 050k's."

## The decisions it applies

Every change is in `packages/agent-claude-code/`, in four files: `contracts.yaml`,
`src/sign_in.rs`, `tests/wrapper.rs` and `README.md`. Run each script once, from the repository
root, with Python 3; each prints its last word and nothing else. Where an assertion of a script
fails (a text at `BASE` differs from what the start check read), stop and report rather than
adapt it.

### 1. The Type, in `contracts.yaml`, in `claude-code.v2` alone

The script adds the code-page login after the browser login's line, as one JSON line, and the
description's sentence, Q1 (vi), after the description's last line:

```bash
python3 - <<'EOF'
import json
lit = json.load(open("target/wo050k/login.json"))
login = {"name": "code-page", "return": "page", "authorization": lit["authorization"],
         "clientId": lit["clientId"], "page": lit["page"], "carried": False}
path = "packages/agent-claude-code/contracts.yaml"
lines = open(path).read().split("\n")
at = [i for i, line in enumerate(lines) if line.startswith('    - {"name": "browser", ')]
assert len(at) == 1 and lines[at[0] - 1] == "    logins:"
assert lines[at[0] + 1] == "  offers: ['#conversation.v1']"
lines.insert(at[0] + 1, "    - " + json.dumps(login))
text = "\n".join(lines)
old = "    keeps that login for that directory, and then empties that home.\n"
new = old + """    Without a login asked for by name, its sign-in starts the client's browser login; the Type
    also states the client's code-page login.
"""
assert text.count(old) == 1
open(path, "w").write(text.replace(old, new))
print("written")
EOF
```

The login's members, in order: `name` `code-page`, `return` `page`, `authorization`, `clientId`,
`page` and `carried` false. Nothing else in `contracts.yaml` changes: not the browser login, not
`requires` or `implementation`, not `.v1`, not `turn.v1`. The description's new sentence reads
"Without a login asked for by name, its sign-in starts the client's browser login; the Type also
states the client's code-page login.", as order 050m's sentence (v) for `codex.v2` reads with
"device" in place of "code-page". `conduit.yaml` does not change: the `wrapper-v2` item stays
`plain`, and its tool `sign-in` stays as order 050j wrote it.

### 2. The failure texts name PND-010, in `src/sign_in.rs`

`sign-in-hook.v1` asks a hook that cannot start a login to "fail with a diagnostic naming PND-010
and why". Each of order 050j's fixed failure texts of the login's start ends with ` (PND-010)`:

```bash
python3 - <<'EOF'
path = "packages/agent-claude-code/src/sign_in.rs"
text = open(path).read()
for old, new in [
    ('pub const LOGIN_DID_NOT_START: &str = "client login did not start";',
     'pub const LOGIN_DID_NOT_START: &str = "client login did not start (PND-010)";'),
    ('pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL";',
     'pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL (PND-010)";'),
    ('break Err(anyhow!("client login failed: {cause}"));',
     'break Err(anyhow!("client login failed: {cause} (PND-010)"));'),
    ('Err(_) => break Err(anyhow!("client login failed: client exit: status unknown")),',
     'Err(_) => break Err(anyhow!("client login failed: client exit: status unknown (PND-010)")),'),
    ('"client login failed: no URL within {} seconds",',
     '"client login failed: no URL within {} seconds (PND-010)",'),
]:
    assert text.count(old) == 1, old
    text = text.replace(old, new)
open(path, "w").write(text)
print("edited")
EOF
```

Then run `cargo fmt` in `packages/agent-claude-code/`, which wraps the status-unknown arm as

```rust
            Err(_) => {
                break Err(anyhow!(
                    "client login failed: client exit: status unknown (PND-010)"
                ));
            }
```

Nothing else in `sign_in.rs` changes: the cause after `client login failed: ` is still the exit's
(`client exit: status N`, `client exit: signal N` or `client exit: status unknown`), and the
home's and the configuration's refusals, shared with every launch and with the account and
sign-out hooks, keep their texts (decision 6).

### 3. The tests, in `tests/wrapper.rs`

The script makes `stated_login` read two logins and return the browser login, adds
`stated_logins`, teaches `login_template` and `ifc_049_failures` the page login, has the relay
test check the relayed URL against the code-page login, writes ` (PND-010)` into the three
expected failure texts written out, and adds one test:

```bash
python3 - <<'EOF'
path = "packages/agent-claude-code/tests/wrapper.rs"
t = open(path).read()
def rep(a, b, n=1):
    global t
    assert t.count(a) == n, a[:80]
    t = t.replace(a, b)

rep('''/// claude-code.v2's one stated login, read from `contracts.yaml` (one JSON line under `logins`).
fn stated_login() -> Value {''', '''/// claude-code.v2's stated logins, read from `contracts.yaml` (one JSON line each under `logins`).
fn stated_logins() -> Vec<Value> {''')
rep('''        .map(|line| serde_json::from_str(&line["    - ".len()..]).unwrap())
        .collect();
    assert_eq!(logins.len(), 1);
    logins[0].clone()
}
''', '''        .map(|line| serde_json::from_str(&line["    - ".len()..]).unwrap())
        .collect();
    assert_eq!(logins.len(), 2);
    logins
}

/// The browser login, the one the sign-in hook starts when no login is asked for by name.
fn stated_login() -> Value {
    stated_logins()
        .into_iter()
        .find(|login| login["name"] == "browser")
        .unwrap()
}
''')
rep('''fn login_template(login: &Value) -> String {
    let redirect = &login["redirect"];
    format!(
        "{}?code=true&client_id={}&response_type=code&redirect_uri={}{{port}}{}&scope={}&code_challenge={{challenge}}&code_challenge_method=S256&state={{state}}",
        login["authorization"].as_str().unwrap(),
        encode(login["clientId"].as_str().unwrap()),
        encode(&format!("http://{}:", redirect["host"].as_str().unwrap())),
        encode(redirect["path"].as_str().unwrap()),
        encode("user:profile"),
    )
}''', '''fn login_template(login: &Value) -> String {
    let redirect = &login["redirect"];
    let redirect_uri = match login["return"].as_str().unwrap() {
        "page" => encode(login["page"].as_str().unwrap()),
        _ => format!(
            "{}{{port}}{}",
            encode(&format!("http://{}:", redirect["host"].as_str().unwrap())),
            encode(redirect["path"].as_str().unwrap()),
        ),
    };
    format!(
        "{}?code=true&client_id={}&response_type=code&redirect_uri={}&scope={}&code_challenge={{challenge}}&code_challenge_method=S256&state={{state}}",
        login["authorization"].as_str().unwrap(),
        encode(login["clientId"].as_str().unwrap()),
        redirect_uri,
        encode("user:profile"),
    )
}''')
rep('''    let redirect = &login["redirect"];
    let prefix = format!("http://{}:", redirect["host"].as_str().unwrap());
    let port_ok = |port: u16| match &redirect["ports"] {
        Value::String(any) if any == "any" => port >= 1024,
        Value::Array(ports) => ports.contains(&json!(port)),
        _ => false,
    };
    let redirect_ok = get("redirect_uri")
        .and_then(|uri| uri.strip_prefix(&prefix))
        .and_then(|rest| {
            let (port, path) = rest.split_at(rest.find('/')?);
            Some(port.parse::<u16>().is_ok_and(port_ok) && path == redirect["path"])
        })
        == Some(true);
''', '''    let redirect = &login["redirect"];
    let port_ok = |port: u16| match &redirect["ports"] {
        Value::String(any) if any == "any" => port >= 1024,
        Value::Array(ports) => ports.contains(&json!(port)),
        _ => false,
    };
    // For page, redirect_uri is the stated page itself; for redirect, the stated loopback form.
    let redirect_ok = match login["return"].as_str() {
        Some("page") => get("redirect_uri").is_some_and(|uri| Some(uri) == login["page"].as_str()),
        _ => redirect["host"].as_str().is_some_and(|host| {
            get("redirect_uri")
                .and_then(|uri| uri.strip_prefix(&format!("http://{host}:")))
                .and_then(|rest| {
                    let (port, path) = rest.split_at(rest.find('/')?);
                    Some(port.parse::<u16>().is_ok_and(port_ok) && path == redirect["path"])
                })
                == Some(true)
        }),
    };
''')
rep('''    let failures = ifc_049_failures(&url, &login);
    eprintln!("sign-in IFC-049 failures {failures:?}");
    assert!(failures.is_empty());
''', '''    let failures = ifc_049_failures(&url, &login);
    eprintln!("sign-in IFC-049 failures {failures:?}");
    assert!(failures.is_empty());
    let code_page = stated_logins()
        .into_iter()
        .find(|login| login["name"] == "code-page")
        .unwrap();
    let other = ifc_049_failures(&url, &code_page);
    eprintln!("sign-in IFC-049 failures against code-page {other:?}");
    assert_eq!(other, ["redirect_uri"]);
''')
rep('Some("client login failed: client exit: status 3"),',
    'Some("client login failed: client exit: status 3 (PND-010)"),', 2)
rep('Some("client login failed: no URL within 3 seconds"),',
    'Some("client login failed: no URL within 3 seconds (PND-010)"),')
t = t.rstrip("\n") + '''

#[test]
fn v2_stated_logins_each_match_one_url() {
    let logins = stated_logins();
    let names: Vec<_> = logins.iter().map(|login| login["name"].clone()).collect();
    assert_eq!(names, [json!("browser"), json!("code-page")]);
    let (browser, code_page) = (&logins[0], &logins[1]);
    assert!(browser["return"] == "redirect");
    assert!(code_page["return"] == "page");
    assert_eq!(code_page["carried"].as_bool(), Some(false));
    assert!(code_page.get("redirect").is_none() && code_page.get("issuer").is_none());
    assert_eq!(code_page["authorization"], browser["authorization"]);
    assert_eq!(code_page["clientId"], browser["clientId"]);
    assert!(code_page["page"].as_str().unwrap().starts_with("https://"));
    // Claude Code 2.1.318 builds both URLs from one state and one challenge.
    let state = "synthetic-state-0123456789abcdef";
    let challenge = "synthetic-challenge-0123456789abcdefghijklm";
    assert_eq!(challenge.len(), 43);
    let urls: Vec<String> = logins
        .iter()
        .map(|login| {
            login_template(login)
                .replace("{port}", "49152")
                .replace("{state}", state)
                .replace("{challenge}", challenge)
        })
        .collect();
    for (i, url) in urls.iter().enumerate() {
        for (j, login) in logins.iter().enumerate() {
            let failures = ifc_049_failures(url, login);
            eprintln!("URL of {} against {}: {failures:?}", logins[i]["name"], login["name"]);
            if i == j {
                assert!(failures.is_empty());
            } else {
                assert_eq!(failures, ["redirect_uri"]);
            }
        }
    }
}
'''
open(path, "w").write(t)
print("edited")
EOF
```

Then run `cargo fmt` in `packages/agent-claude-code/`, which wraps the new test's `eprintln!`.

So: `stated_login` still returns the browser login, which every sign-in test uses as before, and
`stated_logins` returns both, in the Type's order. `login_template` builds a page login's URL with
`redirect_uri` the stated page, and a redirect login's as before. `ifc_049_failures` checks a page
login's `redirect_uri` against the stated page and a redirect login's as before, and every other
check alike. The test `v2_sign_in_relays_the_handed_over_url_and_the_login_completes` also checks
that the URL the hook relays, which passes the browser login's checks, fails the code-page login's
at `redirect_uri` alone: the hook's URL, asked for no login, matches exactly one login, the browser
login (IFC-049, PND-010). The new test, `v2_stated_logins_each_match_one_url`, reads both logins,
checks the code-page login's shape, builds each login's URL in the form of Claude Code 2.1.318's
builder from one state and one challenge, and checks that each passes its own login's checks and
fails the other's at `redirect_uri` alone. No scripted client hands the code-page URL over: the
hook never starts that login in 0.2.0, and the client's printed output, where that URL stands, is
what order 050j's URL canary stands for. In `v2_sign_in_error_paths_hold_no_canary` the expected
texts of `exit`, `exit-with-descendant` and `deadline` gain ` (PND-010)`; the cases that compare
with `LOGIN_DID_NOT_START` or `LOGIN_URL_UNUSABLE` take the new texts from the constants. Every
other test keeps passing unchanged. The test count in `tests/wrapper.rs` goes from 26 to 27;
Codex's stays 18.

### 4. The README, `packages/agent-claude-code/README.md`

Write no absolute local path, no login literal and no client version beyond 2.1.318. The script
changes the section `## The .v2 sign-in hook` in eight places and adds, at its end, the subsection
`### The code-page login`:

```bash
python3 - <<'EOF'
path = "packages/agent-claude-code/README.md"
t = open(path).read()
def rep(a, b):
    global t
    assert t.count(a) == 1, a[:70]
    t = t.replace(a, b)
rep("""`claude-code.v2` states `signIn`: its account place is `/account`, and it states one login,
`browser`, a redirect login with `carried: false`. Its `requires` lists `sign-in-hook.v1`,
""", """`claude-code.v2` states `signIn`: its account place is `/account`, and it states two logins,
each with `carried: false`: `browser`, a redirect login, and `code-page`, a page login
([the code-page login](#the-code-page-login)). Its `requires` lists `sign-in-hook.v1`,
""")
rep("""The login's literals, its authorization URL, its client ID and its redirect's host and path, are
in `contracts.yaml` alone. Its redirect states `ports: any`, since the client's listener takes a
port that the system assigns at each start.
""", """The logins' literals, the authorization URL and the client ID that the two share, the browser
login's redirect host and path, and the code-page login's page, are in `contracts.yaml` alone.
The browser login's redirect states `ports: any`, since the client's listener takes a port that
the system assigns at each start.
""")
rep("""Its failures use fixed texts:

- `client login did not start`.
- `client login failed: client exit: status N`, `client login failed: client exit: signal N` or
  `client login failed: client exit: status unknown`, where the login ends before or as it
  hands its URL over.
- `client login failed: no URL within 30 seconds`.
- `client login gave no usable URL`, for a URL longer than the limit, with a space or a
""", """Its failures use fixed texts, each naming PND-010, as `sign-in-hook.v1` asks of a hook that
cannot start a login:

- `client login did not start (PND-010)`.
- `client login failed: client exit: status N (PND-010)`,
  `client login failed: client exit: signal N (PND-010)` or
  `client login failed: client exit: status unknown (PND-010)`, where the login ends before or as
  it hands its URL over.
- `client login failed: no URL within 30 seconds (PND-010)`.
- `client login gave no usable URL (PND-010)`, for a URL longer than the limit, with a space or a
""")
rep("""only the URL handed to the browser: the client's printed output, which holds a second URL, is
discarded, and no answer, diagnostic or log of the wrapper holds any of it.
""", """only the URL handed to the browser: the client's printed output, which holds the code-page
login's URL, is discarded, and no answer, diagnostic or log of the wrapper holds any of it.
""")
rep("""The login and its hand-over were read from Claude Code 2.1.318's program, `bin/claude.exe` of
its npm package at that version, the macOS arm64 build, by identifier: the OAuth
configuration's `CLAUDE_AI_AUTHORIZE_URL` and `CLIENT_ID`; the authorization URL's builder,
whose loopback `redirect_uri` gives the host and the path; the callback listener, started with no
port; the opener, which runs `BROWSER` with the URL as its one argument; and `auth login`, which
takes the claude.ai login unless `--console` is given.
""", """The logins and the hand-over were read from Claude Code 2.1.318's program, `bin/claude.exe` of
its npm package at that version, the macOS arm64 build, by identifier: the OAuth
configuration's `CLAUDE_AI_AUTHORIZE_URL`, `CLIENT_ID` and `MANUAL_REDIRECT_URL`; the
authorization URL's builder, whose loopback `redirect_uri` gives the browser login's host and
path and whose other `redirect_uri` is `MANUAL_REDIRECT_URL`, the code-page login's page; the
callback listener, started with no port; the OAuth flow, which builds both URLs from one state
and one challenge, gives the code-page URL to its caller and opens the browser login's; the
opener, which runs `BROWSER` with the URL as its one argument; and `auth login`, which takes the
claude.ai login unless `--console` is given, prints the code-page URL and reads a pasted code,
with its state, as a line of its standard input.
""")
rep("""redirect URL that passed IFC-049's checks while its listener accepted connections, and printed a
second URL.
""", """redirect URL that passed IFC-049's checks while its listener accepted connections, and printed a
second URL, a page login's, for which the probe's IFC-049 checks found no failure.
""")
rep("""opens (PND-010, ruling 86, point 3); without a name, the hook starts its one login. Only the
hand-over on macOS was observed, and a real client's closeout is still required. The code-page
login is order 050k's.
""", """opens (PND-010, ruling 86, point 3); without a name, the hook starts the browser login. Only
the hand-over on macOS was observed, and a real client's closeout is still required.

### The code-page login

`code-page` is the second route of the same `claude auth login`. From the state and challenge of
the browser login's URL, the client builds a URL whose `redirect_uri` is the service's own HTTPS
page, prints it, and waits for the code that the page shows, pasted with its state as a line of
the login's standard input. Its `return` is `page` and it states `carried: false`: no Conduit
process carries, holds or records the code, and the person pastes it on the Agent's own machine,
at it or over a connection to it (CTR-013, PND-013).

The Type states it now, since a login that this release does not state never joins
`claude-code.v2` (ruling 82, point 4). It cannot be chosen in this release. The `wrapper-v2` item
is `plain`, so the engine refuses a `sign-in.v1` request that names this login, or any other,
before any part runs (PND-010, ruling 86, point 3), and the hook, asked for no login, starts the
browser login. The hook discards the client's standard input and output, so it relays no
code-page URL, and the login it starts takes no pasted code. Choosing the code-page login waits
for a later release, whose keyed item gets the login's name (ruling 86, points 1 and 2).
""")
open(path, "w").write(t)
print("edited")
EOF
```

Nothing else in the README changes: the opening paragraph, which names the `sign-in` hook and its
section, stays as order 050j wrote it.

### 5. Check the Type with the engine

As order 050j's decision 9 did, for this package alone. This checks what the package tests
cannot: that the Type, stating two logins, passes CTR-013's checks at admission and registration
(order 048a-1's), among them distinct names and no URL passing IFC-049's checks for both.

- In `engine/`, run `cargo build --release`. Change nothing there.
- In `packages/agent-claude-code/`, run `cargo run --release --bin package`, which prints
  `ARCHIVE#SHA256`.
- In a new temporary directory T outside the repository, run
  `engine/target/release/conduit init --store T/store`, then write the project R under T: a
  `conduit.yaml` that pins `example.org/conduit/agent-claude-code` at version 0.2.0 with that
  digest, as `engine/crates/shell/tests/agents.rs` writes one, and one template file, the
  template `v2-sign-in`, `kind: Agent`, `spec.type`
  `example.org/conduit/agent-claude-code#claude-code.v2`, `config: {account: example-account}`,
  with no `implementation`.
- Register it with
  `engine/target/release/conduit register --store T/store --package ARCHIVE#SHA256 R`. It holds
  no instance, so it starts no wrapper and no client. Report the reply's `accepted` and its
  diagnostics.
- Expected: accepted, with no diagnostic, and exit status 0. If the reply differs, change nothing
  to make it pass: report it, since this order changes no engine source, and the coordinator
  decides.
- Then show that the check is live: copy `contracts.yaml` to `target/contracts.yaml.fixed`, change
  in the code-page login's line alone `"name": "code-page"` to `"name": "browser"`, build the
  archive again and register with a fresh store under a new T. Expected: exit status 1, the command
  fails with `package admission failed` and a reply with `accepted` false and one diagnostic whose
  `ruleId` is `CTR-013` and whose message text ends with
  `signIn: logins 0 and 1 are both named "browser"`. Report its exit status, its `ruleId` and that
  ending, but no path. Restore with `cp target/contracts.yaml.fixed contracts.yaml`, check that
  `cmp` prints nothing, and build the archive once more.

### 6. What this order does not do

No package test of `signInLogin` reaching the hook, no `keyed` list and no change to
`conduit.yaml`: under the owner's ruling 86, point 1, the `wrapper-v2` item stays `plain` in
0.2.0, and the review's item 3 takes effect in a later release, by the route ruling 86, point 2
gives. So the hook gets no login name, starts the browser login as order 050j built it, and never
starts the code-page login; nothing in `src/main.rs`, `src/lib.rs` or the scripted client
(`tests/fixtures/client.rs`) changes, and the tool's description, "Start the client's browser
login in this Agent's own home and give its URL.", stays true. The engine's refusal of a
`signInLogin` aimed at a plain hook is order 207's text and order 209's build, not this order's.
The home's and the configuration's refusals, which every launch and the account and sign-out hooks
share and which name their cause, and the `.v1` hook's refusal, which no Type stating `signIn`
reaches, keep their texts (the coordinator's reading of "and why": their causes are named, and
ending them with ` (PND-010)` would change the shared refusals of order 050c). No change to `.v1`,
to `turn.v1`, to the browser login, to `requires` or `implementation`, to order 050h's retire hook,
to order 050i's account and sign-out hooks, or to `Cargo.toml`, `Cargo.lock`, `src/bin/package.rs`,
`tests/package.rs` or the fixtures; nothing about `apiKey`, which ruling 83 leaves out of `.v2`;
nothing in `packages/agent-codex/`; and nothing under `engine/`, `conformance/` or `spec/`.
Connection item 3's canary, that no engine or Interface process holds a code-page code, is order
050o's. The 0.2.0 fixture's `.v2` Types and the loader vectors are order 050n's, as order 050j
records. The engine's test `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
(`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n, as
[order 050a](050a-agents-v2-own-tools.md) records; neither you nor the verifier counts it against
this order. Do not run the engine's tests and do not change it.

## Who decided what

The code-page login's literals are Claude Code 2.1.318's, read from its program by the
coordinator, as "Where the login's literals come from" says. That `claude-code.v2` states the
code-page login now, `return: page` and `carried: false`, is the review's item 2 and the owner's
answer 4 (ruling 82, point 4, Q5's a); that its description lists it is the owner's answer 7
(ruling 82, point 5, Q1 (vi)); that the item stays `plain` in 0.2.0, with no package test of
`signInLogin` reaching the hook, is the owner's ruling 86, point 1; that a login asked for by name
is refused before any part is ruling 86, point 3 (c). That the fixed failure texts name PND-010 is
`sign-in-hook.v1`'s, carried by the coordinator's judgment of order 050j's second verifier's
finding (4), a should and the order's fault. The split of order 050 into 050b to 050o, this
order's place third and last in the Claude lane, and its base are the coordinator's, open to the
owner's strike ([ruling 48](../proposals/ruling-48/README.md)). Everything else here is the
coordinator's too, each open to the owner's strike: the login's name, `code-page`; its place after
`browser`, as one JSON line; the description's sentence, in order 050m's form; ` (PND-010)` at the
end of each text, and the home's, the configuration's and the `.v1` refusals left as they are; the
tests' helpers, the new test, its synthetic state and challenge and its port `49152`, and the
relay test's check against the code-page login; the reading of item 2's done-when for the code-page
login, whose URL the test builds in the form of the client's builder rather than having the scripted
client hand it over, since the hook never starts that login in 0.2.0; the README's wording and its
subsection; and the registration probe and its live check. Apply them as written and report them
under `decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-claude-code/`: `contracts.yaml`, `src/sign_in.rs`,
`tests/wrapper.rs` and `README.md`. Nothing else, in the packages or outside them. The build's
ignored output (`target/`, `dist/`, and `engine/target/` for decision 5) may change, and so may
anything under T; `target/wo050k/login.json` stays as the coordinator put it.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_private_paths.py
python3 tools/set_names.py --check
git diff --check
```

Each prints what is said beside it:

```bash
P=packages/agent-claude-code
git status --short | grep -vE '^ M packages/agent-claude-code/(contracts\.yaml|src/sign_in\.rs|tests/wrapper\.rs|README\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- docs packages/agent-codex $P/conduit.yaml $P/src/lib.rs $P/src/main.rs $P/src/bin $P/tests/fixtures $P/tests/package.rs $P/Cargo.toml $P/Cargo.lock; echo $?   # 0
git diff --numstat "$BASE" -- $P/contracts.yaml   # 3, 0 and the path
git diff --numstat "$BASE" -- $P/src/sign_in.rs   # 9, 5 and the path
grep -c 'PND-010' $P/src/sign_in.rs   # 5
grep -E '"client login (failed|did not|gave)[^"]*"' $P/src/sign_in.rs | grep -vc '(PND-010)"'   # 0
grep -cF 'pub const LOGIN_DID_NOT_START: &str = "client login did not start (PND-010)";' $P/src/sign_in.rs   # 1
grep -cF 'pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL (PND-010)";' $P/src/sign_in.rs   # 1
grep -c '^    - {"name": "code-page", "return": "page", ' $P/contracts.yaml   # 1
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs   # 18
grep -cE '#\[(tokio::)?test' $P/tests/wrapper.rs   # 27
grep -c 'PND-010' $P/tests/wrapper.rs   # 3
grep -c '^fn stated_logins() -> Vec<Value> {$' $P/tests/wrapper.rs   # 1
grep -c '^fn v2_stated_logins_each_match_one_url() {$' $P/tests/wrapper.rs   # 1
grep -cF 'assert_eq!(other, ["redirect_uri"]);' $P/tests/wrapper.rs   # 1
grep -c '(PND-010)`' $P/README.md   # 6
grep -c '^### The code-page login$' $P/README.md   # 1
grep -c '(#the-code-page-login)' $P/README.md   # 1
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'without a name, the hook starts its one login\.'   # 0
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'The code-page login is order 050k.s\.'   # 0
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'without a name, the hook starts the browser login\.'   # 1
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'each with `carried: false`: `browser`, a redirect login, and `code-page`, a page login'   # 1
grep -ci 'apikey' $P/contracts.yaml   # 0
grep -rnE '/(Users|home)/|/(private/)?tmp/' $P --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The script below, run from the repository root with a Python that has PyYAML, must print exactly
the line below it: whether every other document of `contracts.yaml` is `BASE`'s, whether
`claude-code.v2`'s spec is `BASE`'s but for `signIn`, whether `signIn` is `BASE`'s with the
code-page login, of the literals file's values, appended, whether the description is `BASE`'s
followed by the new sentence, and whether the rest of the document is `BASE`'s. It prints no value.

```bash
python3 - "$BASE" <<'EOF'
import json, subprocess, sys, yaml
base = sys.argv[1]
path = "packages/agent-claude-code/contracts.yaml"
was = list(yaml.safe_load_all(subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout))
now = list(yaml.safe_load_all(open(path)))
lit = json.load(open("target/wo050k/login.json"))
sentence = "Without a login asked for by name, its sign-in starts the client's browser login; the Type also states the client's code-page login."
pick = lambda docs: [d for d in docs if d["metadata"]["name"] == "claude-code.v2"][0]
others = lambda docs: [d for d in docs if d["metadata"]["name"] != "claude-code.v2"]
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
w, n = pick(was), pick(now)
page = {"name": "code-page", "return": "page", "authorization": lit["authorization"], "clientId": lit["clientId"],
        "page": lit["page"], "carried": False}
print(others(now) == others(was),
      less(n["spec"], ("signIn",)) == less(w["spec"], ("signIn",)),
      n["spec"]["signIn"] == {"account": "/account", "logins": w["spec"]["signIn"]["logins"] + [page]},
      n["metadata"]["description"] == w["metadata"]["description"] + " " + sentence,
      less(n["metadata"], ("description",)) == less(w["metadata"], ("description",))
      and less(n, ("spec", "metadata")) == less(w, ("spec", "metadata")))
EOF
```

```text
True True True True True
```

And the literals stay in `contracts.yaml` alone: this prints, for the authorization URL, the
client ID and the page, the tracked files of the working tree that hold it, and no value.

```bash
python3 - <<'EOF'
import json, subprocess
lit = json.load(open("target/wo050k/login.json"))
for value in (lit["authorization"], lit["clientId"], lit["page"]):
    found = subprocess.run(["git", "grep", "-l", "-F", "-e", value], capture_output=True, text=True).stdout.split()
    print(found)
EOF
```

```text
['packages/agent-claude-code/contracts.yaml']
['packages/agent-claude-code/contracts.yaml']
['packages/agent-claude-code/contracts.yaml']
```

In `packages/agent-claude-code/`, one command at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release
cargo run --release --bin package
```

`cargo test --release` reports, for `tests/wrapper.rs`, 27 passed and 0 failed, and every other
test target as at `BASE` (the library's unit tests 2 and the package test 1); the package test
reads the new `contracts.yaml`. Report the totals; what the sign-in tests and the new test printed,
taken from `cargo test --release --test wrapper -- --nocapture v2_sign_in` and
`cargo test --release --test wrapper -- --nocapture v2_stated_logins` (each error case's text now
ends with ` (PND-010)` where it is one of decision 2's; the new test prints four lines, two with
`[]` and two with `["redirect_uri"]`); the digest that `package` prints (it is a host build's), as
the archive's file name and digest alone; and decision 5's two replies, each exit status, each
`accepted` and each diagnostic's `ruleId`, and the second's message ending. Say on which system the
tests ran.

Then show that the new test fails when the page login's `redirect_uri` goes unchecked:

```bash
cp tests/wrapper.rs target/wrapper.rs.fixed
```

Make one edit to `tests/wrapper.rs`: in `ifc_049_failures`, replace
`Some("page") => get("redirect_uri").is_some_and(|uri| Some(uri) == login["page"].as_str()),` with
`Some("page") => true,`. Then run `cargo test --release --test wrapper v2_stated_logins`; it must
fail, at the assertion that the browser login's URL fails the code-page login's checks at
`redirect_uri` (left `[]`, right `["redirect_uri"]`). Restore with
`cp target/wrapper.rs.fixed tests/wrapper.rs` (not `mv`, so that cargo rebuilds), rerun the same
command, which passes, and check that `cmp target/wrapper.rs.fixed tests/wrapper.rs` prints
nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Quote no login literal, no URL and no path under a home directory in it. Tests that bind loopback
ports may fail in your sandbox; if they do, say so, and the admin reruns them. Where the sandbox
denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at its preflight, and any
test that stops a tree with recorded groups may fail with the stop's note; say so, name each such
test, report the counts passed and failed for that reason, and the reader reruns them on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on the Claude lane's branch: it does not merge alone, and
its Record is written at the foot of this file on that branch, as orders 050c's to 050j's are. A
fresh Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](../proposals/ruling-79/README.md), point 2). The local gate that [ruling
80](../proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order 050n's
merge, read with the expected engine failure known.

## Record

Run on 2026-11-19, from about 11:06 to 11:18 UTC and again from about 15:29 to 15:40 UTC, on the Mac
(macOS, arm64), from `wo/050-lane-claude` at `acfe6273` (this order's text), with the coordinator's
literals file in place for Claude Code 2.1.318, in two rounds: the worker (448 s) and a fresh Codex
verifier (260 s), then the worker again (394 s) and a second fresh Codex verifier (270 s). It was
one of A77's paired orders, in the code group and off the sign-in chain, dispatched with one round
and continued from the coordinator's report. The change is the four files of Scope. `claude-code.v2`
states, after `browser` and as one JSON line, the login `code-page`, `return: page` and `carried:
false`, with the browser login's `authorization` and `clientId` and the client's page, and its
description gains Q1 (vi)'s sentence; `src/sign_in.rs`'s fixed failure texts of the login's start
end with ` (PND-010)`; `tests/wrapper.rs` reads both logins (`stated_logins`), teaches
`login_template` and `ifc_049_failures` the page login, has the relay test check the relayed URL
against the code-page login, and gains `v2_stated_logins_each_match_one_url` (27 tests); the
README's `## The .v2 sign-in hook` takes decision 4's eight changes and the subsection `### The
code-page login`. In round 1 the worker applied decisions 1 to 6 as written; its own choices were
`/usr/bin/python3` for the YAML scripts and cargo run offline.

The paired read of round 1: the merged list held 9 findings, 6 of Codex's verifier's, 2 marked must,
3 should and 1 nit, and 3 of the Opus paired reader's (claude-opus-5-5, Claude Code 2.1.318, effort
high, no MCP server, nothing loaded, 98 s), 1 should and 2 nits. The coordinator judged the list
blind before opening the key: #1 and #2, on the README's 0.2.0 text, rejected as decision 4's
prescribed text, which describes the 0.2.0 release, whose engine builds orders 207 and 209 before it
ships (as judged for order 050j's paired finding 1); #3 (one fixed text names PND-010 for three
distinguishable start failures) and #8 (`assert_eq!` on the shared literals would print them on
failure) accepted as shoulds and carried to round 2; #4, #5 and #6 (helper doc comments describe the
browser login alone), #7 (the relay fixture writes the literals to a temporary file) and #9 (the
synthetic S256 challenge has nonzero pad bits) nits; no must.

Round 2 changed prescribed code and text, and each change stands as the coordinator's amendment,
open to the owner's strike:

- Decision 2: `LOGIN_DID_NOT_START` is removed, and its six call sites each take one of six fixed
  texts, `client login did not start: <cause> (PND-010)`, with these causes: could not resolve state
  directory, capture directory overlaps client home, could not create capture directory, client
  process tree is closed, could not spawn client, client process ID unavailable (paired finding 3).
  Three of them are tested: could not spawn client (`no-client`), capture directory overlaps client
  home (`capture-is-home`) and could not create capture directory (`capture-blocked`). The other
  three, client process tree is closed, client process ID unavailable and could not resolve state
  directory, are reached only by races or by states that a scripted client cannot set up (a tree
  already closed, a child with no PID, a state directory that vanishes after the home check), and
  are left untested.
- Decision 3: the `no-client` and `capture-is-home` cases expect the new texts; a `capture-blocked`
  case (a regular file at the capture path) is added; the authorization and clientId checks use
  `assert!` with messages that name only the field (paired finding 8).
- Decision 4: the README lists the six texts.
- The amended values of "Done when": the numstat of `src/sign_in.rs` 23, 11; `PND-010` in
  `src/sign_in.rs` 10; the `LOGIN_DID_NOT_START` constant 0; `PND-010` in `tests/wrapper.rs` 6; the
  lines holding `` (PND-010)` `` in the README 11. The `git diff --quiet` line prints 1 once this
  Record and A77's record are committed, for those two files of `docs/` alone; every other line of
  "Done when" stands as written.

The second verifier found 1 must, 2 shoulds and 1 nit, which the runner passed on unjudged: (1), a
must, the `.v2` hook still fails to start a login without naming PND-010 when the home check or the
configuration check refuses (`sign_in.rs` lines 389 and 394), where `sign-in-hook.v1` asks for "a
diagnostic naming PND-010 and why"; (2), a should, the README presents the engine's refusal of a
named login before any part runs as available on this branch; (3), a should, the login matrix's
synthetic challenge, ending in `m`, has nonzero pad bits, which a conforming S256 encoder cannot
give; (4), a nit, `ifc_049_failures`'s doc comment still names a redirect login alone. The
coordinator's judgment: (1) accepted as a should, not a must, the order's fault, since decision 6
kept the home's and the configuration's refusals as they are so that order 050c's shared refusals
would not change, and the work follows the order; it is carried to order 050n (the coordinator's
choice), which can end those two refusals with ` (PND-010)` in the hook alone, leaving the shared
texts unchanged; (2) rejected, as paired findings 1 and 2 were, the prescribed text of the 0.2.0
release; (3) a nit, the order's synthetic value, as paired finding 9; (4) a nit, as paired findings
5 and 6, fixed below. In round 2 the worker passed 26 of 27 wrapper tests, the library's 2 and the
package test 1: the sandbox denied the PID-scoped `ps` of `v2_client_tree_stop_leaves_no_survivor`,
which the reader's run on the Mac passed. Decision 5's registration gave exit 0, `accepted: true`
and no diagnostic, and, with the code-page login renamed `browser`, exit 1, `package admission
failed`, `accepted: false` and one diagnostic, `CTR-013`, whose message ends `signIn: logins 0 and 1
are both named "browser"`; the host archive's digest was `4bfc0d97...016cb`.

A fresh read-only Opus reader then read the result on the Mac (ruling 79, point 2), on the branch at
`87a01e21` on `acfe6273`. `check_docs`, `check_private_paths`, `set_names --check` and
`check_schemas` gave Errors: 0, and `git diff --check` was clean; every count of "Done when" printed
what the order says but the five that round 2's amendment changes; the structural script printed
`True True True True True`, and the literal search found `contracts.yaml` alone for each value;
`cargo fmt --check` and `clippy -D warnings` were clean; on a scratch copy, `cargo test --release`
gave the library's 2, the package test 1 and 27 wrapper tests passed, 0 failed,
`v2_client_tree_stop_leaves_no_survivor` among them; the `--nocapture` runs printed each error
case's text ending in `(PND-010)` where it is one of decision 2's, the relay `[]` and
`["redirect_uri"]`, and the matrix's four lines, two `[]` and two `["redirect_uri"]`, with no
literal and no `https://` in any output; the `redirect_uri` mutation failed at left `[]`, right
`["redirect_uri"]`, and passed after the restore, `cmp` silent; the digest was `4bfc0d97...016cb`;
and decision 5's registration, rerun, gave the worker's two replies. A scan of the added lines found
no private detail, and both commits had the work-order identity. It found two musts and three
shoulds:

- M1: five of the order's lines of "Done when", as written, no longer print their expected values,
  since round 2's amendment to prescribed code is recorded nowhere in the order.
- M2: neither this Record nor A77's record of this order, `docs/work-orders/050k-a77.yaml`, exists
  on the branch, so the paired read's numbers and the round-2 judgments are recorded nowhere.
- S1: round 2's verifier marked a must that nobody has judged: the `.v2` hook still fails to start a
  login without naming PND-010 when the home or configuration check refuses.
- S2: round 2's amendment tests three of its six new start-failure texts; the other three are
  untested, and the Record does not say so.
- S3: the remaining findings of round 2's verifier need their judgment in the Record, and the stale
  doc comments, accepted as nits, are still in the code.

The fixer kept the branch on `wo/050-lane-claude` at `acfe6273`, without a rebase, since the lane
had not moved past it, and acted on each finding:

- M1: this Record's amendment lines for decisions 2, 3 and 4 and the amended values of "Done when",
  above.
- M2: this Record, and `docs/work-orders/050k-a77.yaml`, written from the paired read's list and
  key, the runner's logs and the readers' reports, with `merged: no` and `ci` none until order
  050n's merge. Its `paired.findings` are the list's nine findings, verbatim with the coordinator's
  judgments; the key's paths and digests of the private reports are left out; each value that no
  file at hand gives, the Codex client's version and the later Opus passes' models, clients,
  efforts, seconds and tokens, reads `not recorded`, for the coordinator to fill.
- S1: the reader's option (a), the least change: the judgment above, (1) a should, the order's
  fault, citing decision 6, carried to order 050n. No code changes for it. The coordinator may take
  option (b) instead, ending the two refusals with ` (PND-010)` in the hook alone.
- S2: the sentence on the three untested texts, in decision 2's amendment line above.
- S3: the judgments above, (2) rejected, (3) a nit, (4) a nit; and, as the coordinator's change, the
  two helper doc comments in `tests/wrapper.rs`: `login_template`'s now reads "A stated login's URL
  in the form Claude Code 2.1.318's builder gives (the browser login's is the one the scripted login
  hands over), built from the stated login; the login fills `{port}`, `{state}` and `{challenge}`.",
  and `ifc_049_failures`'s "IFC-049's checks of a URL for a redirect or page login, each failing
  check named." This fixes paired findings 4, 5 and 6 and the second verifier's nit.

The fixer then ran on the Mac, from the worktree's root, on the files of this commit before it was
made: every count of "Done when" against `acfe6273` (each printed what the order says, or the
amended value above; the status count printed 0 before this Record and A77's record were added and 2
after, and the `git diff --quiet` line printed 1 after, both for these two files of `docs/`), the
structural script under `/usr/bin/python3` (`True True True True True`), the literal search
(`contracts.yaml` alone, three times), `check_docs`, `check_private_paths` and `set_names --check`
(Errors: 0 each), and `git diff --check` (clean). In `packages/agent-claude-code/`, one cargo
command at a time with `RUST_TEST_THREADS=4`: `cargo fmt --check` (exit 0), `cargo clippy
--all-targets -- -D warnings` (clean), `cargo test --release` (the library's 2, the package test 1
and 27 wrapper tests passed, 0 failed), `cargo test --release --test wrapper -- --nocapture
v2_sign_in` (3 passed; 8 lines name PND-010, and no output holds `https://`) and `cargo test
--release --test wrapper -- --nocapture v2_stated_logins` (1 passed; four lines, two `[]` and two
`["redirect_uri"]`). The `redirect_uri` mutation of "Done when" failed at `tests/wrapper.rs:3717`,
left `[]`, right `["redirect_uri"]`, and passed after the restore, `cmp` silent. The archive and
decision 5's registration were not run again: `contracts.yaml` and `conduit.yaml` are unchanged
since the reader's run. The gate's run on this order's squash is the coordinator's, at order 050n's
merge.
