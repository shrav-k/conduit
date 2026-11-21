# Work order 050j (P2j): `claude-code.v2` states `signIn`, with its redirect login and sign-in hook

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
their program files, and sign in nowhere. This order adds a client login: every `Config` a test
builds and every wrapper a test starts uses the package's scripted client (`--client-program`), as
every test at `BASE` does, so that no login ever reaches a real client or a real service. The tests
use temporary directories only, and the engine is run only to initialize a store and register a
project.

**The login's literals.** The Type's login states four values read from the client: its
authorization URL, its client ID, and its redirect's host and path. They are written in
`packages/agent-claude-code/contracts.yaml` alone, by decision 2's script, from a file that the
coordinator puts in your tree before you start, `target/wo050j/login.json` (ignored build output,
never committed), as "Where the login's literals come from" says. Never print that file or any
value in it, copy none of them into any other file, and quote none in your report, a test's
output or a log: each value is named by its member (`authorization`, `clientId`,
`redirect.host`, `redirect.path`), never shown. If the file is missing, stop, change nothing and
report.

## What and why

**Where this order starts.** Your tree is the branch that the coordinator names as `BASE`, not
`main`: the Claude lane's branch, `wo/050-lane-claude`, at or after order 050i's tip, `85da0fc5`,
which holds orders 050a, 076 and 078, 050a's Record, orders 050c, 050d, 050e, 050f and 050h, each
with its Record, and order 050i, the lane's first, with its Record (its checkpoint `47ac99b9`, its
round 2 `28c04902`, and the reader's findings and the Record, `85da0fc5`). The lane holds `main`
with order 048a-1's merge (`facb4ff4`, #352), CTR-013's checks in the loader, which this order's
registration needs. This order is the second of the Claude lane (050i, 050j, 050k). The Codex lane
(050l, 050m) is on a branch of its own and does not share `BASE`; neither lane holds the other's
work. `BASE`'s tip holds this order file, committed: the coordinator puts it on the lane's branch
after this text merges on `main`, and sets `BASE` to that branch's tip. Before you change anything,
check from the repository root that you find there what that gives, and stop, change nothing and
report if any check prints otherwise. The Python lines need a Python with PyYAML, as
`tools/check_schemas.py` does (AGENTS.md, "Checks").

```bash
P=packages/agent-claude-code
git merge-base --is-ancestor 85da0fc5 "$BASE"; echo "lane $?"
git merge-base --is-ancestor facb4ff4 "$BASE"; echo "048a-1 $?"
test "$(git rev-parse HEAD)" = "$(git rev-parse "$BASE")"; echo "head $?"
git ls-files --error-unmatch docs/work-orders/050j-claude-sign-in-redirect-login.md >/dev/null; echo "order $?"
git status --short | wc -l
grep -c '^## Record' docs/work-orders/050h-agents-v2-retire-hook.md
grep -c '^## Record' docs/work-orders/050i-claude-account-sign-out-hooks.md
python3 -c "import yaml; s=[d for d in yaml.safe_load_all(open('packages/agent-claude-code/contracts.yaml')) if d['metadata']['name'] == 'claude-code.v2'][0]['spec']; print(s['schema']['required'].count('account'), s['schema']['properties']['account'], 'signIn' in s, s['requires'], s['implementation'])"
grep -c "^    implements: \['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1', 'github.com/shrav-k/conduit/core#account-hook.v1', 'github.com/shrav-k/conduit/core#sign-out-hook.v1'\]$" $P/conduit.yaml
grep -c "^      'github.com/shrav-k/conduit/core#sign-out-hook.v1': sign-out$" $P/conduit.yaml
grep -c '^pub mod sign_in;$' $P/src/lib.rs
for s in '    retired: bool,' '    turns_started: usize,' '            turns_started: 0,' 'const POLL: Duration = Duration::from_millis(20);' 'fn signal_group(group: i32, start: u64, signal: i32, require_leader: bool) -> bool {' 'pub helper: PathBuf,'; do grep -cF "$s" $P/src/lib.rs; done
for s in 'pub const STATUS_DEADLINE: Duration = Duration::from_secs(30);' 'pub async fn account_within(&self, deadline: Duration) -> Result<Option<String>>' 'pub async fn sign_out_within(&mut self, deadline: Duration) -> Result<String>'; do grep -cF "$s" $P/src/sign_in.rs; done
for s in 'if (r.name == "account" || r.name == "sign-out") && self.1 {' '"name":"sign-out"' 'if args.first().map(String::as_str) == Some("--headers-helper") {'; do grep -cF "$s" $P/src/main.rs; done
grep -c 'if args == \["auth", "status"\] {' $P/tests/fixtures/client.rs
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs
grep -cE '#\[(tokio::)?test' $P/tests/wrapper.rs
grep -c '\["turn", "retire", "account", "sign-out"\]' $P/tests/wrapper.rs
diff packages/agent-codex/src/lib.rs $P/src/lib.rs | grep -c '^[<>]'
grep -cE 'sign-in-hook|signIn|logins:' $P/conduit.yaml
grep -cE 'sign-in-hook|signIn|logins:' $P/contracts.yaml
cat $P/src/lib.rs $P/src/main.rs $P/src/sign_in.rs $P/tests/wrapper.rs $P/tests/fixtures/client.rs | grep -cE 'login_arguments|logins\.jsonl|AGENT_SIGN_IN_CAPTURE|"name":"sign-in"|URL_CANARY|fn stated_login|fn ifc_049_failures'
grep -c '^## The .v2 account and sign-out hooks$' $P/README.md
grep -c '^## The .v2 sign-in hook$' $P/README.md
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'which the Type does not require yet\.'
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c '`claude-code.v2` has yet to state `signIn` or include the hooks in `requires`, so no template takes them by default\.'
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c "If the wrapper refused the named directory at startup but accepted the state directory, the status runs in the wrapper's working directory\."
tr '\n' ' ' < docs/work-orders/050i-claude-account-sign-out-hooks.md | grep -c 'which listed the seven files of Scope)'
python3 -c "import json; c=[x for x in json.load(open('experiments/027-a5-sign-in/results-2026-11-16.json'))['clients'] if x['client'] == 'claude-code'][0]; s=c['steps']['4']; print(c['version'], s['shim_called'], s['login_args'], [(u['source'], u['return'], u['redirect_host'] is not None, u['ifc_049'], u['listens']) for u in s['urls']])"
python3 -c "import json; l=json.load(open('target/wo050j/login.json')); print(sorted(l), sorted(l['redirect']), all(isinstance(v, str) and v for v in (l['authorization'], l['clientId'], l['redirect']['host'], l['redirect']['path'])))"
```

They print, in order:

- `lane 0`, `048a-1 0`, `head 0`, `order 0` and `0`; then `1` and `1` (050h's and 050i's
  Records).
- From orders 050f, 050h and 050i (one line):

  ```text
  1 {'type': ['string', 'null']} False ['github.com/shrav-k/conduit/core#retire-hook.v1'] {'#turn.v1': '#wrapper-v2', 'github.com/shrav-k/conduit/core#retire-hook.v1': '#wrapper-v2'}
  ```

  (`account` required, a string or null with no default, no `signIn`, `retire-hook.v1` alone
  under `requires`, with the turn item as its default).
- From order 050i: `1` and `1` (the `wrapper-v2` item implements and serves the account and
  sign-out hooks); `1` (its module line); `1` for each of the six strings of `lib.rs` (the
  `Session` fields and `new` that this order extends, `POLL`, `signal_group` and the wrapper's own
  program as `helper`); `1` for each of the three strings of `sign_in.rs`; `1` for each of the
  three strings of `main.rs`; `1` (the scripted client's status branch); `18` and `23` (the two
  `wrapper.rs`); `3` (050i's three `.v2` tool lists); and `3` (the two `lib.rs` still one source
  but for the `CLAUDE` line and 050i's module line).
- Nothing of this order yet: `0` and `0`; `0`; `1` and `0` (050i's README section, no sign-in
  section).
- The texts that decisions 1 and 8 change, each once: `1` (the opening paragraph's "which
  the Type does not require yet."), `1` (050i's section's sentence on `signIn`), `1` (S1's
  sentence) and `1` (S2's clause).
- Stage 0's facts for Claude Code's login (one line):

  ```text
  2.1.318 True [] [('shim', 'redirect', True, [], 'yes'), ('printed', 'page', False, [], 'not checked')]
  ```

  (version 2.1.318; the login, started with no argument and no terminal, called the browser shim;
  the URL handed to the shim is a redirect to a loopback host that failed none of IFC-049's checks
  while its listener accepted a connection, and a second URL, printed, is a page login).
- The literals file: `['authorization', 'clientId', 'redirect'] ['host', 'path'] True` (its
  members, each a non-empty string; no value printed).

**What it builds.** `claude-code.v2` states `signIn`, as CTR-013 asks: the account place
`/account`, and one login, `browser`, Claude Code's own browser login, a redirect login with
`carried: false`. Its `requires` gains `sign-in-hook.v1`, `account-hook.v1` and
`sign-out-hook.v1` beside order 050h's `retire-hook.v1`, each with the `wrapper-v2` item as its
default. That item gains the sign-in hook, served as the tool `sign-in`: it runs `claude auth
login` in the Agent's own home with the wrapper's own program as the client's browser, keeps the
URL the client hands to that browser, and answers it as a URL request, while the login keeps
running and waits at its own loopback listener for the person to complete it there. No return
passes through Conduit (`carried: false`, PND-013), and the hook relays nothing the client prints.
Order 050i's account and sign-out hooks, built for this, are now required. Registration with the
engine shows that the Type passes CTR-013's checks. The order also carries two shoulds left by
order 050i's last reader (decision 1).

It carries, from [the coordinator's plan of 050](117-plan-s2-s3.md) (order 117's 050 entry:
"050j, claude-code.v2 states signIn, with its redirect login and sign-in hook. After 050i, and
048a-1 merged with the branch rebased onto it"), the 050j entry of the coordinator's split:

- "claude-code.v2 item 1's signIn: account place /account; requires lists sign-in-hook.v1,
  account-hook.v1, sign-out-hook.v1 and retire-hook.v1, each with a default". The targeted
  review's [`claude-code.v2` list](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/review-signin-2026-11-16/README.md#050-claude-codev2),
  which the owner accepted with [ruling 82](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md), item 1: "The Type
  states `signIn` with its required account place `/account`; `requires` lists `sign-in-hook.v1`,
  `account-hook.v1`, `sign-out-hook.v1` and `retire-hook.v1`, each with a default in
  `implementation`". Its `toolsDirectory`, its `account` and the absence of `apiKey` are at `BASE`
  (orders 050f and 202, [ruling 83](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-83/README.md)).
- "Item 2's redirect login: every literal read from the client at a version the README names;
  redirect.ports any where the port varies across starts, the fixed list where it does not". Item
  2: "every literal read from the client at a version the README names; `redirect.ports` is `any`
  where the port varies across starts and the fixed list where it does not [...] Done when a
  package test with a fake client shows each login's URL passing IFC-049's match, and the README
  names the client version read." The code-page login of item 2 is order 050k's.
- "The sign-in hook relays the URL of the login it started (IFC-049's match), shown by a package
  test with a fake client", and item 5: "the hook relays the URL of the login it started".
- "Item 5's URL canary: a fake client prints a canary URL beside the one it hands over, and the
  canary appears nowhere". Item 5: "No client output, URL, claim or token in any result,
  diagnostic, log or record [...] a fake client that prints a canary URL beside the one it hands
  over [...] each canary absent from every result, diagnostic, record and log." Order 050i's
  canary test of a fake home stands; this order adds the URL's.
- "An engine registration check that the Type passes CTR-013 (048a-1's checks)".
- Ruling 82's o1 ([point 6](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md#the-decisions)) and IN_FORCE.md's
  line on it: "the item that implements `sign-in-hook.v1` carries Conduit's keys [...] in
  `claude-code.v2`". The review's item 3 asks it "where the Type states more than one login".
  The owner's ruling 86, point 1 (2026-11-18), amends o1 for 0.2.0: the Agent packages' `.v2`
  items stay plain in 0.2.0, and o1 takes effect in a later release, by a route that a spec check
  picks (ruling 86, point 2). So the item stays `plain` here, and order 050k does not key it
  either (see "What this order does not do").

**Where the login's literals come from.** Claude Code has no public source. The coordinator read
the login from the client's own program at the version that the package's README names, 2.1.318
(its launch baseline, and A5's stage 0): `bin/claude.exe` of Claude Code's npm package
(`claude-code`, in the `anthropic-ai` scope) at version 2.1.318, the macOS arm64 build, whose
bundled modules each open with the line `// Version: 2.1.318`. It ran no client, read no file
but that program, and printed no value. The program holds, by identifier:

- the OAuth configuration of the production service, an object whose member
  `CLAUDE_AI_AUTHORIZE_URL` is a string literal: the `authorization` (an `https://` origin and a
  path, with no query); the same object's member `CLIENT_ID`, also a string literal: the
  `clientId` (a second configuration, for a local development service, builds its URLs from
  template strings and is not this one);
- the function that builds the authorization URL, which appends, in order, `code`, `client_id`
  (`CLIENT_ID` unless another client is given), `response_type` `code`, `redirect_uri`, `scope`,
  `code_challenge`, `code_challenge_method` `S256` and `state`, and, only where given, `orgUUID`,
  `login_hint` and `login_method`; for the browser login its `redirect_uri` is a template string
  `http://` + a host + `:` + the listener's port + a path, with no query: the `redirect.host` and
  the `redirect.path`; for the code-page login it is the configuration's `MANUAL_REDIRECT_URL`
  (order 050k's);
- the callback listener, which listens on its given port or else on port 0, and which the OAuth
  flow starts with no port, `authCodeListener.start()`: the system assigns a new port at each
  start, so `redirect.ports` is `any`;
- the OAuth flow, which builds both URLs from one state and one challenge, gives the code-page URL
  to its caller, which `auth login` prints, and opens the browser login's URL;
- the opener, which takes `BROWSER` from the environment, or else `open`, and runs it with the URL
  as its one argument;
- `auth login`'s handler, which takes the claude.ai login unless `--console` is given (or a
  managed setting forces a method), adds `login_hint` only from `--email`, and, with standard
  input closed and its listener running, still waits for the browser's return.

The coordinator's script, below, read those identifiers and wrote the four values into
`target/wo050j/login.json` as `{"authorization", "clientId", "redirect": {"host", "path"}}`, with
mode 0600. It runs with `CLAUDE_PROGRAM` naming that program file, under a Python run with `-I`,
from the tree's root; it prints only counts, and must print exactly the line below it. A reader
who has that program may run it again and compare the file with `contracts.yaml` by decision 2's
check; you do not run it.

```python
import json, os, re
program = open(os.environ["CLAUDE_PROGRAM"], "rb").read()
versions = sorted(set(v.decode() for v in re.findall(rb"\n// Version: ([0-9.]+)\n", program)))
authorize = re.findall(rb'CLAUDE_AI_AUTHORIZE_URL:"([^"]+)"', program)
start = program.find(b'CLAUDE_AI_AUTHORIZE_URL:"')
client = re.findall(rb',CLIENT_ID:"([^"]+)"', program[start:program.find(b"}", start)])
redirect = set(re.findall(rb'"redirect_uri",[\w$]+\?[\w$]+\(\)\.MANUAL_REDIRECT_URL:`http://([^:`/]+):\$\{[\w$]+\}(/[^`?#]*)`', program))
starts = re.findall(rb"authCodeListener\.start\(([^)]*)\)", program)
listen = re.findall(rb'this\.localServer\.listen\([\w$]+\?\?0,"[0-9.]+"', program)
opener = re.findall(rb'[\w$]+=[\w$]+!==void 0\?[\w$]+\?\?void 0:[\w$]+\.BROWSER,', program)
run = re.findall(rb'\(([\w$]+)\|\|"open",\[[\w$]+\],', program)
default = re.findall(rb'[\w$]+=[\w$]+\?[\w$]+==="claudeai":![\w$]+,', program)
print(versions, len(authorize), len(client), len(redirect), starts == [b""], len(listen), len(opener), len(run), len(default))
(host, path), = redirect
login = {"authorization": authorize[0].decode(), "clientId": client[0].decode(), "redirect": {"host": host.decode(), "path": path.decode()}}
os.makedirs("target/wo050j", exist_ok=True)
with os.fdopen(os.open("target/wo050j/login.json", os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600), "w") as f:
    json.dump(login, f)
```

```text
['2.1.318'] 1 1 1 True 1 1 1 1
```

(one version; one authorization, one client ID and one loopback redirect form; the listener
started with no port; the listener's port-0 default; the opener's `BROWSER`; the opener run with
the URL as its one argument; and `auth login`'s claude.ai default). Stage 0 shows the same at
2.1.318 from outside ([A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass),
its JSON as the start check reads it): with no terminal, the login gave the shim on `BROWSER` a
redirect URL to a loopback host that failed none of IFC-049's checks, with a listener accepting
connections, and printed a page URL. Stage 0 ran on macOS alone: the hand-over on Linux is not
observed, and a real client's closeout, not this order, confirms it.

**What the specification already says, so that nothing here changes it.**
[CTR-013](../../spec/promises/contracts.yaml): a Type that states `signIn` "holds exactly account,
a pointer into configuration, and logins, a non-empty list of sign-in-login.v1. Every login value
is literal [...] redirect [states] the loopback host, ports and a path that holds neither ? nor #
[...] Only redirect and page state carried, required for both [...] The account place and every
member on its path are required, and the place is confined to string or null by MAN-017's test
[...] The Type requires the three core operations sign-in-hook.v1, account-hook.v1 and
sign-out-hook.v1"; "A Type states signIn only where account-hook.v1 reports an account attested
by the service: [...] reported when asked". `sign-in-login.v1`
([`spec/schemas/contracts.yaml`](../../spec/schemas/contracts.yaml)): `redirect` holds `host` (one
of `localhost`, `127.0.0.1`, `[::1]`), `ports` (`any` or a list) and `path`; `issuer` is optional,
and this login states none. [`spec/core/operations.yaml`](../../spec/core/operations.yaml):
`sign-in-hook.v1` "Begin[s] the client login and return[s] URL-mode elicitation to the engine
inside sign-in.v1 or setup [...] Start one of its Type's stated logins, the one signInLogin asks
for where present, or fail [...]; give its URL under IFC-049. On the engine's retry carrying
signInReturn [...] give the value only to the program waiting on that login [...] otherwise fail
with a diagnostic naming PND-013". [IFC-049](../../spec/interface/implementation.yaml): "A
sign-in-hook.v1 URL matches exactly one login the instance's Type states [...] Its origin and path
equal authorization; it has no user information, fragment or repeated parameter [...] client_id
equals clientId, response_type is code, and redirect_uri is for redirect http:// with the stated
host, one stated port (or 1024 to 65535 for any) and stated path, without query [...]
code_challenge_method is S256, code_challenge is 43 base64url-alphabet characters (RFC 7636), and
state is at least 22 RFC 3986 unreserved characters"; "No text from the hook is relayed"; "No
Agent receives a sign-in's URL". [PND-013](../../spec/promises/pending.yaml) and the limits in
[spec/README.md](../../spec/README.md): Claude Code Agent Types state `carried: false` in v0, and
"the person completes the login on the Agent's own machine, at it or over a connection that
Conduit opens, limits and closes", which order 050o builds. So the hook answers a URL request and
no request state, and a call carrying a return is refused, naming PND-013. The engine checks the
URL and writes the message; the hook does neither.

**What the branch already does, so that you do not redo it.** At `BASE`, in
`packages/agent-claude-code/`, as the start check shows:

- `src/lib.rs` has `client_home`, which checks and creates the home with every launch's refusals;
  `client_command`, which builds the client's command with the launch's environment,
  `CLAUDE_CONFIG_DIR` set to the home, both API-key variables removed, the client directory as its
  working directory, its own process group and `kill_on_drop`; `process`; `signal_group`, which
  signals a group only where its leader's start time still matches; `POLL`; the tree's state with
  `groups` and `closed`, and its `stop`, which stops every recorded group; `Config::helper`, the
  wrapper's own program, which `Config::from_env` takes from `current_exe()` and the tests set to
  the wrapper binary; and `Session`, with `shutdown`, which stops the tree.
- `src/sign_in.rs` (order 050i) has the account hook, `account` and `account_within`, and the
  sign-out hook, `sign_out` and `sign_out_within`, and their constants.
- `src/main.rs`'s `Wrapper(Arc<Mutex<Session>>, bool)`, the second field `config.v2`, lists `turn`
  and, for `.v2`, `retire`, `account` and `sign-out`; its `call_tool` serves `account` and
  `sign-out` first, then `retire`, and answers any other name but `turn` with
  `ErrorData::invalid_params("unknown tool", None)`. Its `run` reads its arguments first and
  serves `--headers-helper`.
- The scripted client (`tests/fixtures/client.rs`) serves order 050h's logout and order 050i's
  status before it writes `client-started`, and otherwise runs turns.
- `tests/wrapper.rs` has 23 tests and, from order 050i, the helpers `write_signed_in_login`,
  `hook_lines` and `assert_no_hook_canary`, the constant `ACCOUNT`, and three `.v2` tool lists,
  `["turn", "retire", "account", "sign-out"]`, at `tests/wrapper.rs:1409`, `:2158` and `:2863`.

## The decisions it applies

Every change is in `packages/agent-claude-code/`, but decision 1's sentence in order 050i's file.
Nothing in `packages/agent-codex/` changes: the Codex lane's orders 050l and 050m write that
package's hooks, and order 050n joins the two sources again (see "Who decided what").

Every code block in this list is indented as part of the list; remove from each of its lines the
indentation of its opening fence (three spaces under a numbered decision, five under a sub-item).

1. **The two shoulds that order 050i's last reader left.**
   - S1, in `packages/agent-claude-code/README.md`: in the item `` `client status did not start`
     `` of "The .v2 account and sign-out hooks", the sentence "If the wrapper refused the named
     directory at startup but accepted the state directory, the status runs in the wrapper's
     working directory." reads as if it contradicted the sentence before it. Line 430 at `BASE`,

     ```markdown
       at startup but accepted the state directory, the status runs in the wrapper's working directory.
     ```

     becomes the two lines

     ```markdown
       at startup but accepted the state directory, the client directory is the wrapper's working
       directory, and the status runs there.
     ```

     and line 429 does not change.
   - S2, in `docs/work-orders/050i-claude-account-sign-out-hooks.md`: the Record's clause "which
     listed the seven files of Scope" holds only for a run before the Record was added. Line 967
     at `BASE`,

     ```markdown
     files of Scope), its script under `/usr/bin/python3` (`True True True True`), `check_docs`,
     ```

     becomes the two lines

     ```markdown
     files of Scope and, once the Record was added, this order file), its script under
     `/usr/bin/python3` (`True True True True`), `check_docs`,
     ```

     and nothing else in that file changes.

2. **The Type**, in `contracts.yaml`, in `claude-code.v2` alone. Run this script once, from the
   repository root, with Python 3. It reads the literals file, prints `written` and nothing
   else, and replaces the Type's six lines from `toolsDirectory` to the retire hook's default:

   ```bash
   python3 - <<'EOF'
   import json
   lit = json.load(open("target/wo050j/login.json"))
   core = "github.com/shrav-k/conduit/core#"
   login = {"name": "browser", "return": "redirect", "authorization": lit["authorization"],
            "clientId": lit["clientId"], "redirect": {"host": lit["redirect"]["host"], "ports": "any",
            "path": lit["redirect"]["path"]}, "carried": False}
   path = "packages/agent-claude-code/contracts.yaml"
   old = f"""  toolsDirectory: {{directory: /directory}}
     offers: ['#conversation.v1']
     requires: ['{core}retire-hook.v1']
     implementation:
       '#turn.v1': '#wrapper-v2'
       '{core}retire-hook.v1': '#wrapper-v2'
   """
   new = f"""  toolsDirectory: {{directory: /directory}}
     signIn:
       account: /account
       logins:
       - {json.dumps(login)}
     offers: ['#conversation.v1']
     requires: ['{core}sign-in-hook.v1', '{core}account-hook.v1', '{core}sign-out-hook.v1', '{core}retire-hook.v1']
     implementation:
       '#turn.v1': '#wrapper-v2'
       '{core}sign-in-hook.v1': '#wrapper-v2'
       '{core}account-hook.v1': '#wrapper-v2'
       '{core}sign-out-hook.v1': '#wrapper-v2'
       '{core}retire-hook.v1': '#wrapper-v2'
   """
   text = open(path).read()
   assert text.count(old) == 1
   open(path, "w").write(text.replace(old, new))
   print("written")
   EOF
   ```

   Every code block in this list is indented as part of the list; remove from each of its lines
   the indentation of its opening fence (three spaces under a numbered decision, five under a
   sub-item) before you use it, so that the YAML that the script
   replaces and writes has `toolsDirectory`, `signIn`, `offers`, `requires` and
   `implementation` at two spaces, as `contracts.yaml` has them, and the heredoc's `EOF` at the
   line's start. The login is one line, a JSON object, which YAML reads as a flow mapping, so that the
   package's tests read it with `serde_json` and no YAML parser: its members, in order, `name`
   `browser`, `return` `redirect`, `authorization`, `clientId`, `redirect` with `host`, `ports`
   `any` and `path`, and `carried` false. Nothing else in `contracts.yaml` changes: not the
   description (order 050k adds its code-page sentence), not `schema`, not `.v1`, not `turn.v1`.

3. **The item**, in `conduit.yaml`, in `wrapper-v2` alone: its `implements` line becomes

   ```yaml
       implements: ['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1', 'github.com/shrav-k/conduit/core#account-hook.v1', 'github.com/shrav-k/conduit/core#sign-out-hook.v1', 'github.com/shrav-k/conduit/core#sign-in-hook.v1']
   ```

   and under `tools`, after the `sign-out` line, add

   ```yaml
         'github.com/shrav-k/conduit/core#sign-in-hook.v1': sign-in
   ```

   `plain: true` stays, and nothing else in the manifest changes.

4. **The hook**, in `src/lib.rs` and `src/sign_in.rs`.
   - In `src/lib.rs`, add exactly two lines: `    login: Option<sign_in::Login>,` after the
     field line `    turns_started: usize,` of `pub struct Session`, and
     `            login: None,` after the line `            turns_started: 0,` of
     `Session::new`. Nothing else in `lib.rs` changes; the child module uses its parent's private
     items.
   - In `src/sign_in.rs`, replace the first 10 lines at `BASE`, the module comment and the `use`
     lines, which end at the blank line above `/// How long the account hook waits`, with these
     20 lines:

     ```rust
     //! claude-code.v2's sign-in hooks (orders 050i and 050j): Claude Code's own status, logout and
     //! login, run in this Agent's own home alone.
     use super::{
         ClientTree, Config, LOGOUT_DEADLINE, POLL, Session, client_command, client_home, exit_cause,
         logout, process, signal_group,
     };
     use anyhow::{Result, anyhow, bail, ensure};
     use serde_json::Value;
     use std::{
         fs,
         io::{Read, Write},
         os::unix::fs::{DirBuilderExt, OpenOptionsExt},
         path::{Path, PathBuf},
         process::Stdio,
         time::Duration,
     };
     use tokio::{
         io::AsyncReadExt,
         time::{Instant, sleep, timeout},
     };
     ```

     and append, after the file's last line, a blank line and:

     ```rust
     /// How long the sign-in hook waits for the client to hand over its login's URL (A5 stage 0's wait).
     pub const LOGIN_URL_DEADLINE: Duration = Duration::from_secs(30);
     /// How long a started login may wait for its return before the wrapper stops it.
     pub const LOGIN_LIMIT: Duration = Duration::from_secs(600);
     /// The most bytes of a handed-over URL that the sign-in hook reads.
     pub const LOGIN_URL_LIMIT: u64 = 8_192;
     /// Set on the login alone: the wrapper's own program, run as the login's browser, keeps the URL.
     pub const CAPTURE_VARIABLE: &str = "AGENT_SIGN_IN_CAPTURE";
     /// The capture directory's name inside the instance's state directory, beside the home.
     pub const CAPTURE_NAME: &str = "sign-in-capture";
     pub const LOGIN_DID_NOT_START: &str = "client login did not start";
     pub const LOGIN_URL_UNUSABLE: &str = "client login gave no usable URL";
     /// The sign-in hook's answer to a call carrying a return: its login carries none.
     pub const SIGN_IN_RETURN_REFUSED: &str = "sign-in hook: this login carries no return (PND-013)";
     /// The fixed message of the hook's URL request; the engine writes its own (IFC-049).
     pub const SIGN_IN_MESSAGE: &str = "Sign the client in to this Agent's own home.";

     /// The client's own browser login, the one that claude-code.v2 states as `browser`.
     pub fn login_arguments() -> Vec<String> {
         vec!["auth".into(), "login".into()]
     }

     /// A login the sign-in hook started that still waits for its return.
     pub struct Login {
         group: i32,
         start: Option<u64>,
         watch: tokio::task::JoinHandle<()>,
     }

     impl Login {
         fn stop(self) {
             if let Some(start) = self.start {
                 signal_group(self.group, start, libc::SIGKILL, true);
             }
             self.watch.abort();
         }
     }

     /// The wrapper's own program run as the login's `BROWSER`: keep the first URL handed over.
     pub fn capture(file: &Path, url: &str) -> Result<()> {
         let own = file.with_extension(std::process::id().to_string());
         let mut written = fs::OpenOptions::new()
             .write(true)
             .create_new(true)
             .mode(0o600)
             .open(&own)?;
         written.write_all(url.as_bytes())?;
         drop(written);
         // A second URL never replaces the first.
         let _ = fs::hard_link(&own, file);
         fs::remove_file(&own)?;
         Ok(())
     }

     fn read_url(file: &Path) -> Result<String> {
         let mut bytes = Vec::new();
         fs::File::open(file)
             .and_then(|f| f.take(LOGIN_URL_LIMIT + 1).read_to_end(&mut bytes))
             .map_err(|_| anyhow!("{LOGIN_URL_UNUSABLE}"))?;
         match String::from_utf8(bytes) {
             Ok(url)
                 if url.len() as u64 <= LOGIN_URL_LIMIT
                     && url.starts_with("https://")
                     && url.bytes().all(|b| b.is_ascii_graphic()) =>
             {
                 Ok(url)
             }
             _ => bail!("{LOGIN_URL_UNUSABLE}"),
         }
     }

     fn spawn_login(
         c: &Config,
         home: &Path,
         tree: &ClientTree,
         file: &Path,
     ) -> Result<(tokio::process::Child, i32, Option<u64>)> {
         let mut state = tree.0.lock().unwrap();
         ensure!(!state.closed, "{LOGIN_DID_NOT_START}");
         let child = client_command(c, Some(home), &login_arguments())
             .env("BROWSER", &c.helper)
             .env(CAPTURE_VARIABLE, file)
             .stdin(Stdio::null())
             .stdout(Stdio::null())
             .stderr(Stdio::null())
             .spawn()
             .map_err(|_| anyhow!("{LOGIN_DID_NOT_START}"))?;
         let pid = child.id().ok_or_else(|| anyhow!("{LOGIN_DID_NOT_START}"))? as i32;
         let start = process(pid).ok().flatten().map(|p| p.start);
         state.groups.insert(pid, start);
         Ok((child, pid, start))
     }

     /// Start the client's login and wait for the URL it hands to its browser. Its errors are fixed
     /// texts: no byte of the client's output reaches them.
     async fn start_login(
         c: &Config,
         home: &Path,
         tree: &ClientTree,
         deadline: Duration,
     ) -> Result<(String, Login)> {
         let capture: PathBuf = home
             .parent()
             .ok_or_else(|| anyhow!("{LOGIN_DID_NOT_START}"))?
             .join(CAPTURE_NAME);
         let _ = fs::remove_dir_all(&capture);
         fs::DirBuilder::new()
             .mode(0o700)
             .create(&capture)
             .map_err(|_| anyhow!("{LOGIN_DID_NOT_START}"))?;
         let file = capture.join("url");
         let (mut child, group, start) = match spawn_login(c, home, tree, &file) {
             Ok(spawned) => spawned,
             Err(error) => {
                 let _ = fs::remove_dir_all(&capture);
                 return Err(error);
             }
         };
         let until = Instant::now() + deadline;
         let outcome = loop {
             if fs::symlink_metadata(&file).is_ok() {
                 let url = read_url(&file);
                 break match child.try_wait() {
                     Ok(Some(status)) => Err(anyhow!("client login failed: {}", exit_cause(status))),
                     _ => url,
                 };
             }
             match child.try_wait() {
                 Ok(Some(status)) => break Err(anyhow!("client login failed: {}", exit_cause(status))),
                 Ok(None) => {}
                 Err(_) => break Err(anyhow!("client login failed: client exit: status unknown")),
             }
             if Instant::now() >= until {
                 break Err(anyhow!(
                     "client login failed: no URL within {} seconds",
                     deadline.as_secs()
                 ));
             }
             sleep(POLL).await;
         };
         let _ = fs::remove_dir_all(&capture);
         let url = match outcome {
             Ok(url) => url,
             Err(error) => {
                 // The login's own group alone: a conversation's client, if one runs, is not stopped.
                 if let Some(start) = start {
                     signal_group(group, start, libc::SIGKILL, true);
                 }
                 let _ = child.kill().await;
                 return Err(error);
             }
         };
         let watch = tokio::spawn(async move {
             if timeout(LOGIN_LIMIT, child.wait()).await.is_err() {
                 if let Some(start) = start {
                     signal_group(group, start, libc::SIGKILL, true);
                 }
                 let _ = child.kill().await;
             }
         });
         Ok((
             url,
             Login {
                 group,
                 start,
                 watch,
             },
         ))
     }

     impl Session {
         /// The sign-in hook: start the client's login in this Agent's own home and give its URL.
         pub async fn sign_in(&mut self) -> Result<String> {
             self.sign_in_within(LOGIN_URL_DEADLINE).await
         }
         /// The sign-in hook, with the URL's deadline given; `sign_in` gives LOGIN_URL_DEADLINE.
         pub async fn sign_in_within(&mut self, deadline: Duration) -> Result<String> {
             ensure!(self.config.v2, "sign-in hook: a .v1 Agent has none");
             if self.config.state_dir.is_none()
                 && let Some(error) = &self.config.configuration_error
             {
                 bail!("{error}");
             }
             if let Some(login) = self.login.take() {
                 login.stop();
             }
             let home = client_home(&self.config)?;
             let (url, login) = start_login(&self.config, &home, &self.tree, deadline).await?;
             self.login = Some(login);
             Ok(url)
         }
     }
     ```

     formatted with `cargo fmt`. Nothing else in the file changes.

   So the sign-in hook checks the configuration's refusal and the home as the account hook does;
   stops a login that an earlier call started and that still waits; makes a new directory
   `sign-in-capture`, mode 0700, beside the home in the state directory; runs `claude auth login`
   with the launch's environment, `BROWSER` set to the wrapper's own program and
   `AGENT_SIGN_IN_CAPTURE` naming the file `url` in that directory, and its standard input, output
   and error discarded, its group recorded in the tree; waits at most 30 seconds for the file,
   which the wrapper's own program, run as the browser, writes with mode 0600, keeping the first
   URL; reads at most 8,192 bytes, which must be ASCII with no space or control character and
   begin `https://`; removes the directory; and answers the URL, the login still running. Where
   the login ends before or as it hands the URL over, or the URL is unusable, or no URL comes in
   time, it stops the login's own group, not the tree, removes the directory and fails with a
   fixed text. A started login is stopped after 600 seconds by its own task, by the next call, or
   with the tree: by a failed turn, a sign-out, a retirement or the wrapper's end. The hook never
   reads the client's output, so no byte of it, the printed code-page URL among them, can reach an
   answer, and it never checks the URL against the Type's login: the engine does, under IFC-049.
   Where the code cannot be applied as written (a name at `BASE` differs from what the start check
   read), stop and report rather than adapt it.

5. **The tool and the browser, in `src/main.rs`.**
   - Add, as the file's first line,
     `use agent_adapter::sign_in::{CAPTURE_VARIABLE, SIGN_IN_MESSAGE, SIGN_IN_RETURN_REFUSED, capture};`,
     and change `path::PathBuf,` in the `use std` block to `path::{Path, PathBuf},`.
   - `list_tools`, where the second field is true, lists after `sign-out`:

     ```json
     {"name":"sign-in","description":"Start the client's browser login in this Agent's own home and give its URL.","inputSchema":{"type":"object","properties":{},"additionalProperties":false}}
     ```

     A `.v1` wrapper lists `turn` alone, as at `BASE`.
   - `call_tool` begins, before order 050i's block for `account` and `sign-out`:

     ```rust
     if r.name == "sign-in" && self.1 {
         if r.request_state.is_some() || r.input_responses.is_some() {
             let refused =
                 CallToolResult::error(vec![ContentBlock::text(SIGN_IN_RETURN_REFUSED)]);
             return Ok(refused.into());
         }
         if r.arguments.as_ref().is_some_and(|args| !args.is_empty()) {
             return Err(ErrorData::invalid_params("expected no arguments", None));
         }
         let mut session = tokio::select! {
             biased;
             _ = context.ct.cancelled() => {
                 return Err(ErrorData::internal_error("hook cancelled", None));
             }
             session = self.0.lock() => session,
         };
         // Once the hook has the session, no cancellation cuts it; it has its own deadline.
         return Ok(match session.sign_in().await {
             Ok(url) => {
                 let request = json!({"method":"elicitation/create","params":{"mode":"url","message":SIGN_IN_MESSAGE,"url":url}});
                 let value =
                     json!({"resultType":"input_required","inputRequests":{"login":request}});
                 CallToolResponse::InputRequired(serde_json::from_value(value).unwrap())
             }
             Err(error) => {
                 CallToolResult::error(vec![ContentBlock::text(error.to_string())]).into()
             }
         });
     }
     ```

     A `.v1` wrapper answers `sign-in` as any unknown tool. The answer has no `requestState` and
     no `_meta`; its one input request is under the key `login`, and its message is fixed text,
     since the engine relays its own (IFC-049). A turn in flight holds the session, and the hook
     waits for it.
   - In `run`, immediately after the line that collects `args` and before `--headers-helper`:

     ```rust
     // Run as a login's BROWSER by the sign-in hook alone, which sets the variable on that login.
     if let Some(file) = std::env::var_os(CAPTURE_VARIABLE).filter(|_| args.len() == 1) {
         return capture(Path::new(&file), &args[0]);
     }
     ```

   Nothing else in `main.rs` changes.

6. **The scripted client**, `tests/fixtures/client.rs`.
   - Before `fn emit`, add

     ```rust
     /// The canary URL the scripted login prints beside the URL it hands to its browser.
     const URL_CANARY: &str = "https://synthetic-url-canary.invalid/oauth?state=synthetic-url-canary";
     ```

   - In `main`, after the status branch and before the descendant branch, add

     ```rust
     if args == ["auth", "login"] {
         login();
         return;
     }
     ```

   - Before `#[tokio::main]`, add:

     ```rust
     /// 43 characters of the base64url alphabet, as Claude Code's state and challenge have.
     fn random43() -> String {
         const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
         let mut bytes = [0; 32];
         std::fs::File::open("/dev/urandom")
             .unwrap()
             .read_exact(&mut bytes)
             .unwrap();
         let mut text = String::new();
         for chunk in bytes.chunks(3) {
             let n = chunk.iter().fold(0u32, |n, &b| n << 8 | u32::from(b)) << (8 * (3 - chunk.len()));
             for i in 0..=chunk.len() {
                 text.push(char::from(ALPHABET[(n >> (18 - 6 * i) & 63) as usize]));
             }
         }
         text
     }

     fn login() {
         use std::{
             collections::BTreeMap,
             fs,
             net::TcpListener,
             path::{Path, PathBuf},
             time::{Duration, Instant},
         };

         let home = PathBuf::from(
             std::env::var_os("CLAUDE_CONFIG_DIR").expect("login without the client home"),
         );
         let mut entries: Vec<_> = fs::read_dir(&home)
             .unwrap()
             .map(|entry| entry.unwrap().file_name().into_string().unwrap())
             .collect();
         entries.sort();
         let environment: BTreeMap<_, _> = std::env::vars().collect();
         let evidence = json!({
             "args": ["auth", "login"],
             "home": home,
             "environment": environment,
             "entries": entries,
             "cwd": std::env::current_dir().unwrap(),
         });
         let mut log = fs::OpenOptions::new()
             .create(true)
             .append(true)
             .open("logins.jsonl")
             .unwrap();
         writeln!(log, "{evidence}").unwrap();
         fs::write("login.pid", std::process::id().to_string()).unwrap();

         // Client output that the wrapper discards: the canary URL is printed beside the one handed over.
         println!("If the browser didn't open, visit: {URL_CANARY}");
         io::stdout().flush().unwrap();
         eprintln!("login error {URL_CANARY}");
         if let Ok(exit) = fs::read_to_string("login-exit") {
             std::process::exit(exit.trim().parse().unwrap());
         }
         if Path::new("login-silent").exists() {
             std::thread::sleep(Duration::from_secs(30));
             return;
         }
         let listener = TcpListener::bind("127.0.0.1:0").unwrap();
         let port = listener.local_addr().unwrap().port();
         let state = random43();
         let url = fs::read_to_string("login-url")
             .unwrap()
             .replace("{port}", &port.to_string())
             .replace("{state}", &state)
             .replace("{challenge}", &random43());
         let browser = std::env::var_os("BROWSER").expect("login without BROWSER");
         let _ = Command::new(browser)
             .arg(&url)
             .stdin(Stdio::null())
             .stdout(Stdio::null())
             .stderr(Stdio::null())
             .status();
         if Path::new("login-exit-after-url").exists() {
             std::process::exit(6);
         }
         // Wait at most 30 s for the return at the login's own loopback listener.
         listener.set_nonblocking(true).unwrap();
         let until = Instant::now() + Duration::from_secs(30);
         let mut stream = loop {
             match listener.accept() {
                 Ok((stream, _)) => break stream,
                 Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                     if Instant::now() >= until {
                         std::process::exit(5);
                     }
                     std::thread::sleep(Duration::from_millis(20));
                 }
                 Err(error) => panic!("login listener failed: {error}"),
             }
         };
         stream.set_nonblocking(false).unwrap();
         let mut head = Vec::new();
         let mut byte = [0];
         while !head.ends_with(b"\r\n\r\n") && head.len() < 4096 {
             if stream.read(&mut byte).unwrap() == 0 {
                 break;
             }
             head.push(byte[0]);
         }
         let head = String::from_utf8_lossy(&head);
         let target = head.split(' ').nth(1).unwrap_or_default();
         if !target.ends_with(&format!("&state={state}")) {
             stream
                 .write_all(
                     b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                 )
                 .unwrap();
             std::process::exit(4);
         }
         fs::copy("login-result", home.join("fixture-login")).unwrap();
         stream
             .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
             .unwrap();
     }
     ```

   So the scripted login records its launch in `logins.jsonl` and its PID in `login.pid` in its
   working directory, prints the canary URL to its standard output and to its standard error,
   exits early where `login-exit` says, hangs without a URL where `login-silent` is there, and
   otherwise binds a loopback listener on a port the system assigns, fills the test's template
   `login-url` with that port and a fresh state and challenge, hands the URL to `$BROWSER` as its
   one argument, as Claude Code 2.1.318 does, and waits at most 30 seconds for one request at its
   listener whose target ends with that state, where it copies `login-result` into the home as
   `fixture-login` and answers 200. It never writes in the home otherwise. Nothing else in the file
   changes.

7. **The tests**, in `tests/wrapper.rs`.
   - Extend the import from `agent_adapter::sign_in` with `CAPTURE_NAME`, `CAPTURE_VARIABLE`,
     `LOGIN_DID_NOT_START`, `LOGIN_URL_UNUSABLE`, `SIGN_IN_MESSAGE`, `SIGN_IN_RETURN_REFUSED` and
     `login_arguments`.
   - **Three tests change in their tool lists alone.** Each of the three `.v2` lists
     `["turn", "retire", "account", "sign-out"]` (`tests/wrapper.rs:1409`, `:2158` and `:2863` at
     `BASE`) becomes `["turn", "retire", "account", "sign-out", "sign-in"]`. Every `.v1` list stays
     `["turn"]`, and every other test keeps passing unchanged.
   - Add at the file's end these helpers, verbatim, formatted with `cargo fmt`:

     ```rust
     /// What the scripted login prints beside the URL it hands over, and what no answer may hold.
     const URL_CANARY: &str = "synthetic-url-canary";

     /// claude-code.v2's one stated login, read from `contracts.yaml` (one JSON line under `logins`).
     fn stated_login() -> Value {
         let contracts =
             fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("contracts.yaml")).unwrap();
         let document = contracts
             .split("\n---\n")
             .find(|document| document.contains("\n  name: claude-code.v2\n"))
             .unwrap();
         let logins: Vec<Value> = document
             .lines()
             .skip_while(|line| *line != "    logins:")
             .skip(1)
             .take_while(|line| line.starts_with("    - "))
             .map(|line| serde_json::from_str(&line["    - ".len()..]).unwrap())
             .collect();
         assert_eq!(logins.len(), 1);
         logins[0].clone()
     }

     fn encode(text: &str) -> String {
         text.bytes()
             .map(|b| match b {
                 b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                     char::from(b).to_string()
                 }
                 _ => format!("%{b:02X}"),
             })
             .collect()
     }

     fn decode(text: &str) -> Option<String> {
         let bytes = text.as_bytes();
         let mut out = Vec::new();
         let mut i = 0;
         while i < bytes.len() {
             if bytes[i] == b'%' {
                 let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
                 out.push(u8::from_str_radix(hex, 16).ok()?);
                 i += 3;
             } else {
                 out.push(bytes[i]);
                 i += 1;
             }
         }
         String::from_utf8(out).ok()
     }

     /// The URL the scripted login hands over, in the form Claude Code 2.1.318's builder gives,
     /// built from the stated login; the login fills `{port}`, `{state}` and `{challenge}`.
     fn login_template(login: &Value) -> String {
         let redirect = &login["redirect"];
         format!(
             "{}?code=true&client_id={}&response_type=code&redirect_uri={}{{port}}{}&scope={}&code_challenge={{challenge}}&code_challenge_method=S256&state={{state}}",
             login["authorization"].as_str().unwrap(),
             encode(login["clientId"].as_str().unwrap()),
             encode(&format!("http://{}:", redirect["host"].as_str().unwrap())),
             encode(redirect["path"].as_str().unwrap()),
             encode("user:profile"),
         )
     }

     /// The query's parameters, each decoded; None where one does not decode.
     fn parameters(url: &str) -> Option<Vec<(String, String)>> {
         let query = url.split_once('?').map_or("", |(_, query)| query);
         query
             .split('&')
             .map(|pair| {
                 let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
                 Some((decode(name)?, decode(value)?))
             })
             .collect()
     }

     /// IFC-049's checks of a URL for a redirect login, each failing check named.
     fn ifc_049_failures(url: &str, login: &Value) -> Vec<&'static str> {
         let mut failures = Vec::new();
         let base = url.split_once('?').map_or(url, |(base, _)| base);
         if base != login["authorization"].as_str().unwrap() {
             failures.push("origin and path");
         }
         if base["https://".len().min(base.len())..]
             .split('/')
             .next()
             .is_some_and(|authority| authority.contains('\u{40}'))
         {
             failures.push("user information");
         }
         if url.contains('#') {
             failures.push("fragment");
         }
         let Some(pairs) = parameters(url) else {
             failures.push("query");
             return failures;
         };
         let names: std::collections::BTreeSet<_> = pairs.iter().map(|(name, _)| name).collect();
         if names.len() != pairs.len() {
             failures.push("repeated parameter");
         }
         let get = |name: &str| {
             pairs
                 .iter()
                 .find(|(n, _)| n == name)
                 .map(|(_, value)| value.as_str())
         };
         if get("client_id") != login["clientId"].as_str() {
             failures.push("client_id");
         }
         if get("response_type") != Some("code") {
             failures.push("response_type");
         }
         let redirect = &login["redirect"];
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
         if !redirect_ok {
             failures.push("redirect_uri");
         }
         if get("code_challenge_method") != Some("S256") {
             failures.push("code_challenge_method");
         }
         let base64url = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
         if !get("code_challenge").is_some_and(|v| v.len() == 43 && v.chars().all(base64url)) {
             failures.push("code_challenge");
         }
         let unreserved = |c: char| c.is_ascii_alphanumeric() || "-._~".contains(c);
         if !get("state").is_some_and(|v| v.len() >= 22 && v.chars().all(unreserved)) {
             failures.push("state");
         }
         failures
     }

     /// The port that a URL's redirect_uri names.
     fn redirect_port(url: &str) -> u16 {
         let pairs = parameters(url).unwrap();
         let uri = &pairs.iter().find(|(n, _)| n == "redirect_uri").unwrap().1;
         let rest = uri.strip_prefix("http://").unwrap();
         let authority = rest.split('/').next().unwrap();
         authority.rsplit_once(':').unwrap().1.parse().unwrap()
     }

     fn assert_no_url_canary(text: &str) {
         assert_no_hook_canary(text);
         assert!(!text.contains(URL_CANARY));
     }

     fn sign_in_setup(parent: &Path) -> (Config, std::path::PathBuf, Value) {
         let login = stated_login();
         let mut c = config(parent, false, false);
         c.v2 = true;
         c.env.insert("MCP_TOOL_TIMEOUT".into(), "10000".into());
         prepare(&c).unwrap();
         let home = c.state_dir.as_ref().unwrap().join(HOME_NAME);
         fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
         fs::write(c.cwd.join("login-url"), login_template(&login)).unwrap();
         let result =
             json!({"token": LOGIN_TOKEN, "claims": {"account": LOGIN_CLAIM}, "email": ACCOUNT});
         fs::write(c.cwd.join("login-result"), result.to_string()).unwrap();
         (c, home, login)
     }

     fn process_gone(pid: i32) -> bool {
         (0..100).any(|_| {
             let gone = unsafe { libc::kill(pid, 0) } != 0;
             if !gone {
                 std::thread::sleep(Duration::from_millis(20));
             }
             gone
         })
     }
     ```

     `stated_login` reads the Type's login from `contracts.yaml`, so that no test holds a literal;
     `login_template` builds the URL in the form of Claude Code 2.1.318's builder; and
     `ifc_049_failures` applies IFC-049's checks for a redirect login, as quoted above, and names
     each that fails. "Set up" below means `sign_in_setup`: `config(...)` with `c.v2` true,
     `MCP_TOOL_TIMEOUT` `10000`, `prepare` run, the home `state/client-home` made with mode 0700,
     and `login-url` and `login-result` written in the client directory, `c.cwd`.
   - **A new test, `v2_sign_in_relays_the_handed_over_url_and_the_login_completes`**, through
     `Session`. Plant the default home, and set up. The stated login's `name`, `return`,
     `redirect.ports` and `carried` are `browser`, `redirect`, `any` and false. `sign_in()`
     answers a URL for which `ifc_049_failures` gives no failure and that passes
     `assert_no_url_canary`. `logins.jsonl` holds one line, with `args` `login_arguments()`,
     `environment` (by `assert_client_environment`) the launch's environment as order 050i's tests
     build it plus `BROWSER` `c.helper` and `AGENT_SIGN_IN_CAPTURE` the state directory's
     `sign-in-capture/url`, `cwd` the client directory and `entries` `[]`. `sign-in-capture` is
     absent, `client-started` is absent, and `account()` answers `Ok(None)`. Then, as the person's
     browser on this machine, the test connects to `127.0.0.1` at the URL's redirect port and sends
     `GET` with the stated path, `?code=synthetic-code&state=` and the URL's state, the stated host
     and the port as `Host`; the reply begins `HTTP/1.1 200 `. Within 10 seconds `account()`
     answers `Ok(Some(ACCOUNT))`; the home's entries are then `["fixture-login"]`, its mode 0700;
     `client-started` is still absent; and the default home passes `assert_default_home`. It
     `eprintln!`s the failures list and the line's `args` and `entries`, and never the URL.
   - **A new test, `v2_sign_in_error_paths_hold_no_canary`**, through `Session`, one case at a
     time, each with the default home planted, set up, and the home (in `outside-link`,
     `elsewhere`) holding `write_signed_in_login(home, Some(ACCOUNT))`. Files named under "Set up"
     are written in the client directory. Each call is `sign_in()` but in `deadline`.

     | Case | Set up | Answer | Login lines |
     | --- | --- | --- | --- |
     | `no-client` | `c.program` a missing file under the temporary directory | `LOGIN_DID_NOT_START` | 0 |
     | `exit` | `login-exit` holding `3` | `client login failed: client exit: status 3` | 1 |
     | `deadline` | `login-silent`; the call is `sign_in_within(Duration::from_secs(3))` | `client login failed: no URL within 3 seconds` | 1 |
     | `unusable` | `login-url` holding `http://insecure.invalid/{port}` | `LOGIN_URL_UNUSABLE` | 1 |
     | `outside-link` | the home a link to `elsewhere`, a directory beside `state` | `client home (CLAUDE_CONFIG_DIR): the home resolves outside CONDUIT_STATE_DIR` | 0 |
     | `unset` | `c.state_dir` `None` | `client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset` | 0 |
     | `configuration-unset` | `c.state_dir` `None`, `c.configuration_error` `invalid configuration: client home (CLAUDE_CONFIG_DIR): CONDUIT_STATE_DIR is unset` | that configuration text | 0 |
     | `configuration` | `c.configuration_error` `invalid configuration: effort (AGENT_EFFORT): expected lowercase ASCII letters` | a URL for which `ifc_049_failures` gives no failure | 1 |

     Each error's text is compared exactly and passes `assert_no_url_canary`, as does the URL of
     `configuration`. In each case: the call returns within 10 seconds of its start; the lines of
     `logins.jsonl` are as many as the table says; where a login started and the answer is an
     error, the process that `login.pid` names is gone within 2 seconds (`process_gone`);
     `sign-in-capture` is absent; `client-started` is absent; `fixture-login` still holds its bytes
     as written; and the default home passes `assert_default_home`. It `eprintln!`s each case's
     error, or that the URL passed, and never a URL.
   - **A new test, `v2_sign_in_tool_through_the_wrapper`**, through the wrapper binary over stdio,
     its `.v2` wrapper started as order 050i's `v2_account_and_sign_out_tools_through_the_wrapper`
     starts it (no `AGENT_DIRECTORY`, `CONDUIT_STATE_DIR` the state directory,
     `CLAUDE_CONFIG_DIR` pointing elsewhere, the raw standard output kept), after the default home
     is planted and the test is set up. In this order:
     1. the listed tools' names are `["turn", "retire", "account", "sign-out", "sign-in"]`,
        written as one `assert_eq!` against the literal
        `["turn", "retire", "account", "sign-out", "sign-in"]`, once;
     2. `sign-in` with `{"extra":true}`, sent with `call_tool_once`, is an error of the call whose
        text passes `assert_no_url_canary`, and no `logins.jsonl` exists;
     3. `sign-in` with `{}`, sent with `call_tool_once`, answers `CallToolResponse::InputRequired`
        whose JSON passes `assert_no_url_canary` and holds `resultType` `input_required`,
        `inputRequests` with exactly the key `login`, whose request has `method`
        `elicitation/create`, `params.mode` `url`, `params.message` `SIGN_IN_MESSAGE` and a
        `params.url` for which `ifc_049_failures` gives no failure, and no `requestState` and no
        `_meta`; and a connection to `127.0.0.1` at that URL's redirect port opens (the login
        waits);
     4. `sign-in` with `{}` and `requestState` `synthetic-request-state` answers a complete result
        with `isError` true and one text block, `SIGN_IN_RETURN_REFUSED`;
     5. `logins.jsonl` holds one line, with `args` `login_arguments()`, `environment[BROWSER]` the
        wrapper binary's canonical path and `environment[CLAUDE_CONFIG_DIR]` the home's canonical
        path; and the default home passes `assert_default_home`.

     After `cancel`, the wrapper exits successfully, and a connection to that port is refused (the
     wrapper's end stopped the login with the client's tree). Neither the raw standard output nor
     the standard error holds `URL_CANARY`, `LOGIN_TOKEN`, `LOGIN_CLAIM`, `ACCOUNT`,
     `synthetic-test-token`, `synthetic-anthropic-key` or `synthetic-openai-key`. Then a `.v1`
     wrapper, started in a fresh temporary directory as order 050i's test starts it, lists
     `["turn"]`, answers `sign-in` with `{}` as an error of the call whose text passes
     `assert_no_url_canary`, and leaves no `logins.jsonl`. It `eprintln!`s step 3's key, method,
     mode and message, and never the URL.

   The test count in `packages/agent-claude-code/tests/wrapper.rs` goes from 23 to 26; Codex's
   stays 18.

8. **The README**, `packages/agent-claude-code/README.md`, beside decision 1's S1. Write no
   absolute local path, no login literal and no client version beyond 2.1.318. Three changes.

   First, in the opening paragraph, lines 11 to 13 at `BASE`,

   ```markdown
   tool. In this package the `.v2` item also exposes `account` and `sign-out`, the
   [account and sign-out hooks](#the-v2-account-and-sign-out-hooks), which the Type does not
   require yet. In `.v1`, both reach inputs are required booleans.
   ```

   become

   ```markdown
   tool. In this package the `.v2` item also exposes `account` and `sign-out`, the
   [account and sign-out hooks](#the-v2-account-and-sign-out-hooks), and `sign-in`, the
   [sign-in hook](#the-v2-sign-in-hook), which `claude-code.v2` requires with its `signIn`. In
   `.v1`, both reach inputs are required booleans.
   ```

   Order 050h's sentence before them, "The plain MCP wrapper exposes `turn`, and each `.v2` item
   also `retire`; the manifest maps each versioned operation or hook to its tool.", stays as it
   is.

   Second, in "The .v2 account and sign-out hooks", lines 410 to 412 at `BASE`,

   ```markdown
   `claude-code.v2` has yet to state `signIn` or include the hooks in `requires`, so no
   template takes them by default. The engine dispatches these hooks only as parts of
   `sign-in.v1` or a setup for a Type that states `signIn` (CTR-013, PND-011).
   ```

   become

   ```markdown
   `claude-code.v2` states `signIn` and lists both hooks under `requires`, with the `wrapper-v2`
   item as each one's default ([the sign-in hook](#the-v2-sign-in-hook)). The engine dispatches
   these hooks only as parts of `sign-in.v1` or a setup for a Type that states `signIn` (CTR-013,
   PND-011).
   ```

   Third, add at the end, after a blank line, this section, verbatim:

   ```markdown
   ## The .v2 sign-in hook

   `claude-code.v2` states `signIn`: its account place is `/account`, and it states one login,
   `browser`, a redirect login with `carried: false`. Its `requires` lists `sign-in-hook.v1`,
   `account-hook.v1`, `sign-out-hook.v1` and `retire-hook.v1`, each with the `wrapper-v2` item as
   its default in `implementation`. A string at `/account` is the expected account, and null
   disables both sign-in actions and the account check (CTR-013). The engine dispatches the hooks
   only as parts of `sign-in.v1` or of a setup (CTR-013, PND-011).

   The login's literals, its authorization URL, its client ID and its redirect's host and path, are
   in `contracts.yaml` alone. Its redirect states `ports: any`, since the client's listener takes a
   port that the system assigns at each start.

   The `wrapper-v2` item serves `sign-in-hook.v1` as the tool `sign-in`, which takes no arguments.
   The `.v1` item lists no such tool.

   1. The sign-in hook checks the home as every launch does, with the same refusals, and stops a
      login that an earlier call of the hook started and that still waits.
   2. It runs `claude auth login` with `CLAUDE_CONFIG_DIR` set to the home and the launch's
      environment, with `BROWSER` set to the wrapper's own program, and with
      `AGENT_SIGN_IN_CAPTURE` naming a file in a new directory, `sign-in-capture`, beside the home
      in the state directory. It passes neither `--console` nor `--email`, and discards the
      client's standard input, output and error. Run as that browser, the wrapper keeps the first
      URL the client hands over in that file, with mode 0600.
   3. It waits at most 30 seconds for that URL, reads at most 8,192 bytes of it, and removes the
      directory. It answers an input-required result with one URL request under the key `login`:
      mode `url`, the message `Sign the client in to this Agent's own home.` and the URL. It gives
      no request state. The engine checks the URL under IFC-049 and writes its own message.

   The login keeps running after the answer and waits at its own loopback listener for the
   person's browser to return there, on the Agent's machine or over a connection to it. No return
   passes through Conduit (`carried: false`). The wrapper stops the login after 600 seconds, and
   earlier with a failed turn, a sign-out, a retirement, the wrapper's end or a later call of the
   hook, since the login is part of the client's process tree. A running conversation's client
   keeps running. The hook does not show that a sign-in completed: the account hook does (PND-011).

   Its failures use fixed texts:

   - `client login did not start`.
   - `client login failed: client exit: status N`, `client login failed: client exit: signal N` or
     `client login failed: client exit: status unknown`, where the login ends before or as it
     hands its URL over.
   - `client login failed: no URL within 30 seconds`.
   - `client login gave no usable URL`, for a URL longer than the limit, with a space or a
     character that is not ASCII, or not beginning `https://`.

   After any of them the hook stops the login's own process group and removes the directory. The
   home checks give the launch's home refusals, and the configuration's refusal applies where the
   state directory is unset. A call that carries a request state or input responses is answered
   `sign-in hook: this login carries no return (PND-013)`. The hook relays only the URL handed to
   the browser: the client's printed output, which holds a second URL, is discarded, and no answer,
   diagnostic or log of the wrapper holds any of it.

   The login and its hand-over were read from Claude Code 2.1.318's program, `bin/claude.exe` of
   its npm package at that version, the macOS arm64 build, by identifier: the OAuth
   configuration's `CLAUDE_AI_AUTHORIZE_URL` and `CLIENT_ID`; the authorization URL's builder,
   whose loopback `redirect_uri` gives the host and the path; the callback listener, started with no
   port; the opener, which runs `BROWSER` with the URL as its one argument; and `auth login`, which
   takes the claude.ai login unless `--console` is given.
   [A5's stage 0](../../experiments/027-a5-sign-in/results-2026-11-16.md#pass) recorded on macOS
   that Claude Code 2.1.318's login, started with no terminal, handed its browser a loopback
   redirect URL that passed IFC-049's checks while its listener accepted connections, and printed a
   second URL.

   Limits: the `wrapper-v2` item is `plain` in this release, so the hook gets no Conduit keys
   (IFC-007), and the item is keyed only in a later release (ruling 86, points 1 and 2). A plain
   hook's URL request inside `sign-in.v1` is still relayed to the person, under IFC-007, IFC-016,
   IFC-010 and PND-010 (ruling 86, point 3). The engine refuses a `sign-in.v1` request that asks
   for a login by name (`signInLogin`) for this plain hook before any part runs, so no browser
   opens (PND-010, ruling 86, point 3); without a name, the hook starts its one login. Only the
   hand-over on macOS was observed, and a real client's closeout is still required. The code-page
   login is order 050k's.
   ```

9. **Check the Type with the engine**, as order 050i's decision 7 does, for this package alone.
   This checks what the package tests cannot: that the Type, stating `signIn`, passes CTR-013's
   checks at admission and registration (order 048a-1's).
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
   - Expected: accepted, with no diagnostic. If the reply differs, change nothing to make it pass:
     report it, since this order changes no engine source, and the coordinator decides.
   - Then show that the check is live: copy `contracts.yaml` to `target/contracts.yaml.fixed`,
     remove `'github.com/shrav-k/conduit/core#sign-in-hook.v1', ` from the Type's `requires` line
     alone, build the archive again and register with a fresh store under a new T. Expected: the
     command fails with `package admission failed` and a reply with `accepted` false and one
     diagnostic whose `ruleId` is `SUB-005` and whose `keywordLocation` ends with
     `/dependentSchemas/signIn/properties/requires/allOf/0/contains`. Report its exit status, its
     `ruleId` and `keywordLocation`, but no path. Restore with
     `cp target/contracts.yaml.fixed contracts.yaml`, check that `cmp` prints nothing, and build
     the archive once more.

10. **What this order does not do.** No code-page login and no second login, which are order
    050k's; no `signInLogin` and no keys for the item that implements `sign-in-hook.v1`, which no
    order of 0.2.0 gives: under the owner's ruling 86, point 1, the `wrapper-v2` item stays
    `plain` in 0.2.0, and ruling 82's o1 takes effect in a later release. A plain hook's URL
    request inside `sign-in.v1` is relayed, as IFC-007, IFC-016 and PND-010 say and as order 207
    fixes [IFC-010](../../spec/interface/implementation.yaml)'s then-clause to say (ruling 86,
    point 3 (b)); and the engine refuses a `signInLogin` aimed at a plain sign-in hook before any
    part runs, so no browser opens (PND-010 as order 207 amends it, ruling 86, point 3 (c); the
    engine half is order 209's). Nothing on this unpublished branch dispatches the hook yet, which
    harms nothing. No description change (order 050k adds the code-page sentence, Q1 (vi)); nothing about
    `apiKey`, which ruling 83 leaves out of `.v2`; no change to `.v1`, to `turn.v1`, to the
    `wrapper` item, to order 050h's retire hook or `logout`, to order 050i's account and sign-out
    hooks, or to `Cargo.toml`, `Cargo.lock`, `src/bin/package.rs`, `tests/package.rs` or
    `tests/fixtures/stale-init.json`; no wording of an existing refusal changed; nothing in
    `packages/agent-codex/`; and nothing under `engine/`, `conformance/` or `spec/`. The 0.2.0
    fixture's `.v2` Types (`conformance/fixtures/s1/packages/agents-0.2.0`) state no `signIn`;
    order 050n, which moves `conformance/fixtures/agents/` to 0.2.0, aligns them. Item 1's
    loader-vector done-when is not met here; the engine registration (decision 9) shows the Type
    passes CTR-013, and the loader vector with a template binding /account is order 050n's, with
    the fixture (the coordinator's reading). The engine's test
    `live_s1_scripted_agents_call_engine_with_distinct_run_identities`
    (`engine/crates/shell/tests/agents.rs`) fails on this branch by design until order 050n moves
    `conformance/fixtures/agents/` to 0.2.0, as [order 050a](050a-agents-v2-own-tools.md) records.
    It is the one expected failure on `wo/050-agents-0.2.0` and its lanes, and it is not this
    order's: neither you nor the verifier counts it against this order. Do not run the engine's
    tests and do not change it.

## Who decided what

The login's literals are Claude Code 2.1.318's, read from its program by the coordinator, as
"Where the login's literals come from" says; `ports: any` is the coordinator's reading of the
listener started with no port, as the review's item 2 rule gives it. `carried: false` is the
specification's, for Claude Code Agent Types in v0 (ruling 80 point 8, amending ruling 75 C3).
That the Type states `signIn` with the account place `/account` and requires the four hooks, each
with a default, is the review's item 1; the URL canary and the relayed URL are its item 5; both are
the review's must-state lists, which the owner accepted with ruling 82. The item kept `plain` in
0.2.0, with ruling 82's o1 and the review's item 3 taking effect in a later release, is the
owner's, ruling 86, point 1; that a plain hook's URL is relayed and that a login asked for by name
is refused before any part are ruling 86, point 3. The two shoulds of decision 1 are order 050i's
last reader's, with its fixes. The split of order 050 into 050b to 050o, this order's place second
in the Claude lane, and its base are the coordinator's, open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)). Everything else here is the coordinator's too, each
open to the owner's strike: the login's name, `browser`; the login written as one JSON line;
`claude auth login` with no flag; the wrapper's own program as the client's browser, through
`BROWSER` and `AGENT_SIGN_IN_CAPTURE`, keeping the first URL in a file of mode 0600 in a directory
beside the home; the URL's limits of 30 seconds and 8,192 bytes and its `https://` and ASCII check;
the login's limit of 600 seconds; a later call stopping an earlier login; the login stopped with
the tree; a running conversation's client left running; the answer's key `login`, its message and
no request state; a call carrying a request state or input responses refused, naming PND-013; the
tool's name `sign-in` and its description; the texts; the wording of the README; the scripted
login; the test helpers, the login read from `contracts.yaml` and IFC-049's checks written in the
tests; and the registration probe and its live check. Apply them as written and report them under
`decisions` as the coordinator's.

## Scope

Change only, in `packages/agent-claude-code/`: `contracts.yaml`, `conduit.yaml`, `src/lib.rs`,
`src/sign_in.rs`, `src/main.rs`, `tests/wrapper.rs`, `tests/fixtures/client.rs` and `README.md`;
and, for decision 1, `docs/work-orders/050i-claude-account-sign-out-hooks.md`. Nothing else, in
the packages or outside them. The build's ignored output (`target/`, `dist/`) may change, and so
may anything under T; `target/wo050j/login.json` stays as the coordinator put it.

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
git status --short | grep -vE '^ M (packages/agent-claude-code/(contracts\.yaml|conduit\.yaml|src/lib\.rs|src/sign_in\.rs|src/main\.rs|tests/wrapper\.rs|tests/fixtures/client\.rs|README\.md)|docs/work-orders/050i-claude-account-sign-out-hooks\.md)$' | wc -l   # 0
git diff --quiet "$BASE" -- packages/agent-codex $P/Cargo.toml $P/Cargo.lock $P/src/bin $P/tests/package.rs $P/tests/fixtures/stale-init.json; echo $?   # 0
git diff --numstat "$BASE" -- $P/src/lib.rs   # 2, 0 and the path
git diff --numstat "$BASE" -- docs/work-orders/050i-claude-account-sign-out-hooks.md   # 2, 1 and the path
diff packages/agent-codex/src/lib.rs $P/src/lib.rs | grep -c '^[<>]'   # 5
grep -c 'pub const LOGIN_URL_DEADLINE: Duration = Duration::from_secs(30);' $P/src/sign_in.rs   # 1
grep -c 'pub const LOGIN_LIMIT: Duration = Duration::from_secs(600);' $P/src/sign_in.rs   # 1
grep -c 'pub const LOGIN_URL_LIMIT: u64 = 8_192;' $P/src/sign_in.rs   # 1
grep -c 'vec!\["auth".into(), "login".into()\]' $P/src/sign_in.rs   # 1
grep -c 'stdout(Stdio::null())' $P/src/sign_in.rs   # 1
grep -c '"name":"sign-in"' $P/src/main.rs   # 1
grep -c "^    implements: \['#turn.v1', 'github.com/shrav-k/conduit/core#retire-hook.v1', 'github.com/shrav-k/conduit/core#account-hook.v1', 'github.com/shrav-k/conduit/core#sign-out-hook.v1', 'github.com/shrav-k/conduit/core#sign-in-hook.v1'\]$" $P/conduit.yaml   # 1
grep -c "^      'github.com/shrav-k/conduit/core#sign-in-hook.v1': sign-in$" $P/conduit.yaml   # 1
grep -cE '#\[(tokio::)?test' packages/agent-codex/tests/wrapper.rs   # 18
grep -cE '#\[(tokio::)?test' $P/tests/wrapper.rs   # 26
grep -c '\["turn", "retire", "account", "sign-out", "sign-in"\]' $P/tests/wrapper.rs   # 4
grep -c '\["turn", "retire", "account", "sign-out"\]' $P/tests/wrapper.rs   # 0
grep -c '^## The .v2 sign-in hook$' $P/README.md   # 1
grep -c '(#the-v2-sign-in-hook)' $P/README.md   # 2
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'which the Type does not require yet\.'   # 0
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'the client directory is the wrapper.s working directory, and the status runs there\.'   # 1
tr '\n' ' ' < $P/README.md | tr -s ' ' | grep -c 'The plain MCP wrapper exposes `turn`, and each `.v2` item also `retire`; the manifest maps each versioned operation or hook to its tool\.'   # 1
tr '\n' ' ' < docs/work-orders/050i-claude-account-sign-out-hooks.md | grep -c 'which listed the seven files of Scope and, once the Record was added, this order file)'   # 1
grep -ci 'apikey' $P/contracts.yaml   # 0
grep -ci 'apikey' $P/conduit.yaml   # 0
grep -rnE '/(Users|home)/|/(private/)?tmp/' $P --exclude-dir=target --exclude-dir=dist | wc -l   # 0
```

The two scripts below, run from the repository root with a Python that has PyYAML, must each print
exactly the line below it. The first checks the manifest: whether `wrapper` is `BASE`'s, whether
the rest of the manifest is `BASE`'s, whether `wrapper-v2` is `BASE`'s with the sign-in hook added
to `implements` and `tools` alone, and whether the items' names are `BASE`'s.

```bash
python3 - "$BASE" <<'EOF'
import subprocess, sys, yaml
base = sys.argv[1]
core = "github.com/shrav-k/conduit/core#"
path = "packages/agent-claude-code/conduit.yaml"
was = yaml.safe_load(subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout)
now = yaml.safe_load(open(path))
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
n, w = now["implementations"]["wrapper-v2"], was["implementations"]["wrapper-v2"]
print(now["implementations"]["wrapper"] == was["implementations"]["wrapper"],
      less(now, ("implementations",)) == less(was, ("implementations",)),
      n["implements"] == w["implements"] + [core + "sign-in-hook.v1"]
      and n["tools"] == {**w["tools"], core + "sign-in-hook.v1": "sign-in"}
      and less(n, ("implements", "tools")) == less(w, ("implements", "tools")),
      sorted(now["implementations"]) == sorted(was["implementations"]))
EOF
```

```text
True True True True
```

The second checks the contracts: whether every other document is `BASE`'s, whether
`claude-code.v2`'s spec is `BASE`'s but for `signIn`, `requires` and `implementation`, whether
those two are decision 2's, whether `signIn` is decision 2's with the literals file's values, and
whether the documents' metadata, descriptions included, are `BASE`'s. It prints no value.

```bash
python3 - "$BASE" <<'EOF'
import json, subprocess, sys, yaml
base = sys.argv[1]
core = "github.com/shrav-k/conduit/core#"
path = "packages/agent-claude-code/contracts.yaml"
was = list(yaml.safe_load_all(subprocess.run(["git", "show", f"{base}:{path}"], capture_output=True, text=True, check=True).stdout))
now = list(yaml.safe_load_all(open(path)))
lit = json.load(open("target/wo050j/login.json"))
pick = lambda docs: [d for d in docs if d["metadata"]["name"] == "claude-code.v2"][0]["spec"]
others = lambda docs: [d for d in docs if d["metadata"]["name"] != "claude-code.v2"]
w, n = pick(was), pick(now)
hooks = [core + h for h in ("sign-in-hook.v1", "account-hook.v1", "sign-out-hook.v1", "retire-hook.v1")]
login = {"name": "browser", "return": "redirect", "authorization": lit["authorization"], "clientId": lit["clientId"],
         "redirect": {"host": lit["redirect"]["host"], "ports": "any", "path": lit["redirect"]["path"]}, "carried": False}
less = lambda d, keys: {k: v for k, v in d.items() if k not in keys}
print(others(now) == others(was),
      less(n, ("signIn", "requires", "implementation")) == less(w, ("requires", "implementation")),
      n["requires"] == hooks and n["implementation"] == {"#turn.v1": "#wrapper-v2", **{h: "#wrapper-v2" for h in hooks}},
      n["signIn"] == {"account": "/account", "logins": [login]},
      [d["metadata"] for d in now] == [d["metadata"] for d in was])
EOF
```

```text
True True True True True
```

And the literals stay in `contracts.yaml` alone: this prints, for the authorization URL and the
client ID, the tracked files of the working tree that hold it, and no value.

```bash
python3 - <<'EOF'
import json, subprocess
lit = json.load(open("target/wo050j/login.json"))
for value in (lit["authorization"], lit["clientId"]):
    found = subprocess.run(["git", "grep", "-l", "-F", "-e", value], capture_output=True, text=True).stdout.split()
    print(found)
EOF
```

```text
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

`cargo test --release` reports, for `tests/wrapper.rs`, 26 passed and 0 failed, and every other
test target as at `BASE` (the library's unit tests 2 and the package test 1); the package test reads
the new `contracts.yaml` and `conduit.yaml`. Report the totals; what the three new tests printed,
taken from `cargo test --release --test wrapper -- --nocapture v2_sign_in`; the digest that
`package` prints (it is a host build's), as the archive's file name and digest alone; and decision
9's two replies, each `accepted` and each diagnostic's `ruleId`, and the second's
`keywordLocation`. Say on which system the tests ran.

Then show that the URL canary test fails when the login's output reaches the wrapper's:

```bash
cp src/sign_in.rs target/sign_in.rs.fixed
```

Make one edit to `src/sign_in.rs`: in `spawn_login`, replace `.stdout(Stdio::null())` with
`.stdout(Stdio::inherit())`. Then run `cargo test --release --test wrapper v2_sign_in_tool_through`;
it must fail, at the assertion that the wrapper's raw standard output holds no `URL_CANARY` (the
scripted login's printed line reaches it). Restore with `cp target/sign_in.rs.fixed src/sign_in.rs`
(not `mv`, so that cargo rebuilds), rerun the same command, which passes, and check that
`cmp target/sign_in.rs.fixed src/sign_in.rs` prints nothing.

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, and give the last line of every check and each count above.
Quote no login literal, no URL and no path under a home directory in it. Tests that bind loopback
ports may fail in your sandbox; if they do, say so, and the admin reruns them. Where the sandbox
denies PID-scoped `ps`, `v2_client_tree_stop_leaves_no_survivor` stops at its preflight, and any
test that stops a tree with recorded groups may fail with the stop's note (on macOS the sign-in
hook records the login's group, and a sign-out, a retirement or the wrapper's end stops it); say
so, name each such test, report the counts passed and failed for that reason, and the reader reruns
them on the Mac.

The admin then runs `python3 tools/check_private_paths.py` and `git diff --cached --check` on the
staged change. This order is a checkpoint on the Claude lane's branch: it does not merge alone, and
its Record is written at the foot of this file on that branch, as orders 050c's to 050i's are. A
fresh Codex verifier and a fresh read-only Opus reader read the result before the checkpoint stands
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md), point 2). The local gate that [ruling
80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md) has a merge wait on, `tools/gate.py`, runs at order 050n's
merge, read with the expected engine failure known.

## Record

Run on 2026-11-19, from about 08:37 to 09:17 UTC, on the Mac (macOS, arm64), from
`wo/050-lane-claude` at `b4891ae2` (this order's text), with the coordinator's literals file in
place for Claude Code 2.1.318, in two rounds: the worker (774 s) and a fresh Codex verifier (374 s),
then the worker again (611 s) and a second fresh Codex verifier (295 s). It was A77's first paired
order, in the code group and off the sign-in chain, dispatched with one round and continued from the
coordinator's report. The change is the nine files of Scope. `claude-code.v2` states `signIn` with
the account place `/account` and the redirect login `browser`, `ports: any` and `carried: false`,
and requires the four hooks, each served by `#wrapper-v2`; the `wrapper-v2` item maps
`sign-in-hook.v1` to the tool `sign-in`; `src/lib.rs` gains its two lines; `src/sign_in.rs` gains
decision 4's hook, which runs `claude auth login` in the Agent's own home with the wrapper's own
program as its browser and relays the first URL handed over; `src/main.rs` serves the tool and the
browser helper; the scripted client gains a login; `tests/wrapper.rs` gains three tests (26); the
README gains `## The .v2 sign-in hook` and decision 1's S1, and order 050i's Record its S2. In round
1 the worker applied decisions 1 to 10 as written; its own choices were `/usr/bin/python3` for the
YAML scripts, cargo run offline, the response variants matched before serializing, and the
through-the-wrapper test's connection kept open through the cancel.

The paired read of round 1: the merged list held 6 findings, all Codex's verifier's, 4 marked must
and 2 should; the Opus paired reader (claude-opus-5-5, Claude Code 2.1.318, effort high, no MCP
server, nothing loaded, 145 s) found none. The coordinator judged the list blind before opening the
key: #1, on the README's 0.2.0 text, rejected as decision 8's prescribed text; #2 (the request's
URL-elicitation capability unchecked before the login starts), #3 (starting sign-in could erase the
client home and its stored login, where the capture directory overlapped the home), #5 (group
survivors after the login's leader exits) and #6 (the success test does not pin the exact URL
relayed) accepted as shoulds and carried to round 2; #4 (a URL at most one poll late is accepted) a
nit; no must.

Round 2 changed prescribed code and text, and each change stands as the coordinator's amendment,
open to the owner's strike:

- Decision 4's `src/sign_in.rs`: the capture directory is created, mode 0700, directly inside the
  canonical state directory, after a check that it and the resolved home do not overlap; the hook
  watches the login's leader with `waitid` and `WNOWAIT` (`login_exited`) and, on a failure before
  the answer, kills the login's group while the unreaped leader still holds its ID, then reaps it
  (`stop_starting_login`). Tested by the error cases `capture-is-home` and `exit-with-descendant`,
  whose descendant's socket must close while the conversation's client keeps answering.
- Decision 5's `src/main.rs`: a `sign-in` call whose request does not declare URL elicitation
  (IFC-045) is answered `-32021` with URL elicitation in `data.requiredCapabilities`, before any
  login starts. Tested over stdio in `v2_sign_in_tool_through_the_wrapper`, with no login record
  left by the incapable call.
- Decision 6's scripted client: it writes `login-url.sha256`, the digest of the exact URL it hands
  to its browser, and its `login-exit` mode can leave a live descendant in the login's group.
  Decision 7's tests compare that digest with the relayed URL's (`assert_handed_over_url`, in the
  Session and stdio tests) without printing either.
- Decision 8's README text: the capability rule, the capture directory's refusal, and that a failure
  stops the login's group and removes the capture directory the hook created.

The second verifier found 4 musts and 1 should, which the runner passed on unjudged: (1) a login
whose leader exits after the URL is handed over leaves its group's survivors to neither the
600-second watch nor the next call; (2) a TERM during the capture leaves `sign-in-capture` behind
and every later sign-in fails; (3) a FIFO at the capture path blocks the read past the 30-second
deadline; (4) `sign-in-hook.v1` asks for a diagnostic naming PND-010 where a login fails to start,
and the order's fixed texts name none; and a should, that the test helper `process_gone` reads a
permission error as an exit. The coordinator judged them: (1) accepted as a should, implementation,
fixed by the first fixer's S1; (2) accepted as a must, implementation, fixed by its M1; (3) accepted
as a should, implementation, fixed by its S2; (4) accepted as a should, the order's fault, since
`sign-in-hook.v1` asks a hook that cannot start a login to fail with a diagnostic naming PND-010 and
decision 4's fixed texts name none: it is carried to order 050k, which ends those texts with `
(PND-010)`; and the should on `process_gone` a nit on a prescribed test helper. In round 2 the
worker passed 25 of 26 wrapper tests, the library's 2 and the package test 1: the sandbox denied the
PID-scoped `ps` of `v2_client_tree_stop_leaves_no_survivor`, which the reader's rerun on the Mac
passed. Decision 9's registration gave `accepted: true` with no diagnostic, and, with
`sign-in-hook.v1` removed from `requires`, `accepted: false` with one SUB-005 diagnostic whose
`keywordLocation` ends `/dependentSchemas/signIn/properties/requires/allOf/0/contains`; the host
archive's digest was `258ffea2...cb1fe`.

A fresh read-only Opus reader then read the result on the Mac (ruling 79, point 2), on the branch at
`9e8f867b` on `b4891ae2`. `check_docs`, `check_private_paths` and `set_names --check` gave Errors:
0, and `git diff --check` was clean; the diff held the nine files of Scope; every count of "Done
when" printed what the order says; both scripts, under `/usr/bin/python3`, printed
`True True True True` and `True True True True True`; both literal searches found `contracts.yaml`
alone; `cargo fmt --check` and `clippy -D warnings` were clean; `cargo test --release` gave the
library's 2, the package test 1 and 26 wrapper tests passed, 0 failed, three times; the stdout
mutation failed at the assertion that the wrapper's standard output holds no `URL_CANARY` and passed
after the restore, `cmp` silent; the digest was `258ffea2...cb1fe`; and decision 9's two replies
were the worker's. A scan of the added lines found no private detail, and both commits had the
work-order identity. On a scratch copy, a probe that cut a sign-in short while the login held it
left the capture directory behind, and the next sign-in failed with `client login did not start`. It
read the second verifier's musts as follows: (2) is M1's cause, (1) is S1, (3) is S2 and (4) is S3;
the should on `process_gone` is a nit on a prescribed helper. It found three musts and three
shoulds:

- M1: round 2's exclusive creation of the capture directory makes a directory that an interrupted
  call left (a TERM, a dropped future, a crash) a lasting refusal with a text that does not say why.
- M2: this Record was not written, and must carry round 2's changes as the coordinator's amendments,
  the paired read's numbers and the second verifier's unjudged findings.
- M3: A77's record for this order, `docs/work-orders/050j-a77.yaml`, is absent.
- S1: after the URL is handed over, a leader that exits on its own leaves its group's survivors to
  neither the 600-second watch nor the next call (the second verifier's must 1).
- S2: the URL's read follows a link and blocks on a FIFO with no bound (the second verifier's must
  3).
- S3: the fixed texts of a login that fails to start name no PND-010, which `sign-in-hook.v1` asks
  for (the second verifier's must 4); the coordinator decides.

The fixer kept the branch on `wo/050-lane-claude` at `b4891ae2`, without a rebase, since the lane
had not moved, and acted on each finding:

- M1: in `start_login`, after the overlap check, the two lines
  `// A directory an interrupted call left holds no home: the check above rules that out.` and
  `let _ = fs::remove_dir_all(&capture);`; the error case `capture-exists` is now `capture-stale`,
  which expects a URL that passes IFC-049, one login line and the directory gone with its planted
  file; the README's item 2 reads "The hook refuses a capture directory that overlaps the resolved
  home, and removes one that an interrupted earlier call left." This answers the second verifier's
  must 2 too.
- M2: this Record.
- M3: not written here. The order's own record of A77 is, by WORK_ORDERS.md's "The record each order
  keeps", the coordinator's to commit in its first pull request after the merge's checks run on
  `main` has ended, with `merged` and `ci` known; the coordinator writes it at order 050n's merge,
  from the paired read's list and key and with each private value a placeholder. The second
  fixer wrote it after all, as below.
- S1: the watch task polls `login_exited` every 20 ms for at most 600 seconds and then calls
  `stop_starting_login`, so the group is killed while the exited leader, still unreaped, holds its
  ID, whether the leader exits or the limit comes. The scripted login's `login-exit-after-url` mode
  now leaves a live descendant, as `login-exit` does, and exits a second after handing its URL over;
  the new error case `exit-after-url-with-descendant` expects the URL, then the descendant's socket
  closed within 5 seconds, while the conversation's client keeps answering.
- S2: `read_url` opens the entry with `O_NOFOLLOW` and `O_NONBLOCK` and requires a regular file, and
  answers `client login gave no usable URL` otherwise. The scripted login's new `login-fifo` mode
  makes a FIFO at the capture path; the new error case `fifo`, with `login-silent`, expects that
  text within 10 seconds and the login gone.
- S3: no change in this order. The coordinator judged it a should, the order's fault, and carries it
  to order 050k, which ends decision 4's fixed failure texts with ` (PND-010)` as `sign-in-hook.v1`
  asks.

Each of these changes to decision 4's code, decision 6's scripted login, decision 7's tests and
decision 8's text stands, as round 2's do, as the coordinator's amendment, open to the owner's
strike.

The second verifier's should on `process_gone` was left as the prescribed helper has it. Each fix
taken out failed its test, on the worktree and restored after: without the removal, `capture-stale`
failed at the sign-in's result; with round 2's watch, `exit-after-url-with-descendant` failed with
`login descendant kept running`; without `O_NONBLOCK`, the test hung at the `fifo` case until a
60-second alarm ended it. The fixer then ran on the Mac, from the worktree's root, on the files of
this commit before it was made: every count of "Done when" against `b4891ae2` (each printed what the
order says; the status count printed 0 before the Record was added and 1, this order file, after),
both scripts under `/usr/bin/python3` (`True True True True` and `True True True True True`), both
literal searches (`contracts.yaml` alone), `check_docs`, `check_private_paths` and
`set_names --check` (Errors: 0 each), and `git diff --check` (clean). In
`packages/agent-claude-code/`, one cargo command at a time with `RUST_TEST_THREADS=4`:
`cargo fmt --check` (exit 0), `cargo clippy --all-targets -- -D warnings` (clean),
`cargo test --release` (the library's 2, the package test 1 and 26 wrapper tests passed, 0 failed),
`cargo test --release --test wrapper -- --nocapture v2_sign_in` (3 passed; the error cases printed
their texts, `fifo` `client login gave no usable URL`, and `capture-stale` and
`exit-after-url-with-descendant` `URL passed IFC-049`) and `cargo run --release --bin package`
(`agent-claude-code.tar.gz#79f8d751...da19d`, a host build's). The stdout mutation of "Done when"
failed at the assertion that the wrapper's standard output holds no `URL_CANARY` and passed after
the restore, `cmp` silent. Decision 9's registration was not run again: `contracts.yaml` and
`conduit.yaml` are unchanged since the reader's run.

A second fresh read-only Opus reader then read the result on the Mac, at `af632176` on `b4891ae2`.
`check_docs`, `check_private_paths` and `set_names --check` gave Errors: 0, and `git diff --check`
was clean, on the tree and from `b4891ae2`; every count of "Done when" printed what the order says;
both scripts, under `/usr/bin/python3`, printed `True True True True` and `True True True True
True`; both literal searches found `contracts.yaml` alone; `cargo fmt --check` and `clippy -D
warnings` were clean; on a scratch copy, `cargo test --release` gave the library's 2, the package
test 1 and 26 wrapper tests passed, 0 failed, and the wrapper's 26 passed three more times; the
`--nocapture` run printed each case's text or `URL passed IFC-049`, and no URL; the stdout mutation
failed at the assertion that the wrapper's standard output holds no `URL_CANARY` and passed after
the restore, `cmp` silent; the digest was `79f8d751...da19d`; and decision 9's registration, rerun
with this archive, gave `accepted: true` with no diagnostic and, with `sign-in-hook.v1` removed from
`requires`, exit 1, `accepted: false` and one SUB-005 diagnostic whose `keywordLocation` ends
`/dependentSchemas/signIn/properties/requires/allOf/0/contains`. Each of the first fixer's fixes
taken out failed its test: without M1's removal, `capture-stale` failed at the sign-in's result, and
with round 2's watch, `exit-after-url-with-descendant` failed with `login descendant kept running`.
A scan of the added lines found no private detail, and the three commits had the work-order
identity. It found one must and four shoulds:

- M1: A77's record for this order, `docs/work-orders/050j-a77.yaml`, is still absent, though
  WORK_ORDERS.md's template allows an order not yet merged (`merged: no` and the reason).
- S1: this Record gives the reader's reading of the second verifier's four musts and its should, not
  the coordinator's judgment, which WORK_ORDERS.md asks for when the coordinator writes the Record.
- S2: this Record does not say that the first fixer's changes to decision 4's code, decision 6's
  scripted login, decision 7's tests and decision 8's text stand as the coordinator's amendments.
- S3: this Record's words for paired finding #3 dropped its harm, the loss of the client home and
  its stored login.
- S4: the README names neither the stop of the login's group once the login's own process exits nor
  the refusal of a URL handed over as anything but a regular file.

The second fixer kept the branch on `wo/050-lane-claude` at `b4891ae2`, without a rebase, since the
lane had not moved past it, and acted on each finding:

- M1: `docs/work-orders/050j-a77.yaml`, written from the paired read's list and key with each
  private value a placeholder, `merged: no` and `ci` none until order 050n's merge. Its
  `paired.findings` are the list's six findings, verbatim with the coordinator's judgments; the
  key's paths and digests of the private reports are left out; each value that no file at hand
  gives, the judgments of the second Codex pass and the later Opus passes' models, clients, seconds
  and tokens, reads `not recorded`, for the coordinator to fill. The first fixer's M3 item says so.
- S1: no change by the fixer, whose place it was not to judge in the coordinator's name; the
  coordinator then gave its judgment of the second verifier's findings, above, and the A77 record
  carries it.
- S2: the reader's sentence, after the first fixer's list.
- S3: the reader's words for #3, in the paired read's paragraph, rewrapped.
- S4: in the README's `## The .v2 sign-in hook`, the wrapper stops the login "with its process
  group, after 600 seconds or once the login's own process exits", and `client login gave no usable
  URL` answers also a URL "handed over as anything but a regular file, such as a link or a FIFO".
  This change to decision 8's text stands, as the first fixer's do, as the coordinator's amendment,
  open to the owner's strike.

The second fixer then ran on the Mac, from the worktree's root, on the files of this commit before
it was made: every count of "Done when" against `b4891ae2` (each printed what the order says, but
the status count, which printed 2, this order file and A77's record, both in this commit), both
scripts under `/usr/bin/python3` (`True True True True` and `True True True True True`), both
literal searches (`contracts.yaml` alone), `check_docs`, `check_private_paths` and `set_names
--check` (Errors: 0 each), and `git diff --check` (clean). In `packages/agent-claude-code/`, one
cargo command at a time with `RUST_TEST_THREADS=4`: `cargo fmt --check` (exit 0), `cargo clippy
--all-targets -- -D warnings` (clean), `cargo test --release` (the library's 2, the package test 1
and 26 wrapper tests passed, 0 failed), `cargo test --release --test wrapper -- --nocapture
v2_sign_in` (3 passed; each case printed its text or `URL passed IFC-049`, as in the reader's run)
and `cargo run --release --bin package` (`agent-claude-code.tar.gz#79f8d751...da19d`, unchanged, a
host build's). The stdout mutation of "Done when" failed at the assertion that the wrapper's
standard output holds no `URL_CANARY` and passed after the restore, `cmp` silent. Decision 9's
registration was not run again: `contracts.yaml` and `conduit.yaml` are unchanged since the reader's
run. The gate's run on this order's squash is the coordinator's, at order 050n's merge.

A third fresh read-only Opus reader then reread the branch at `391ca546` and reran every check of
"Done when" on this Mac, each as before. It found one must, that this Record and A77's record gave
the paired Opus reader's effort as default where it ran at high, and three shoulds: the
coordinator's judgment of the second verifier's findings, absent; the Codex passes' `loaded`, given
as empty where the key records none and Codex reads the repository's `AGENTS.md` by itself; and the
A77 record's `fixes`, left empty. The coordinator applied all four: the effort, the judgment above,
`loaded` with its source, and the fixers' work under `fixes`, with a pass for this read. The lane's
fast-forward follows.
