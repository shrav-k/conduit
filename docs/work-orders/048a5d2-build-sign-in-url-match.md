# Work order 048a5d2 (E7a-5d2): IFC-049's URL match for the sign-in hook's URL request

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the two cases you add, with the promises each
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate, and
reach no other machine. Run one cargo command at a time. Never print a whole large file, case,
fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**Where it sits.** Order 048a-5d's refresh (its file
[048a5d-build-sign-in-hook-part.md](048a5d-build-sign-in-hook-part.md), under "Where it sits") cut
048a-5d in two, each after the one before, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)), and recorded the cut
in [order 117](117-plan-s2-s3.md)'s entry and in [the index of work orders](README.md). 048a-5d
builds the `sign-in-hook.v1` part and its answer's form; this order is the second half: "048a-5d2,
next: IFC-049's match, the URL matched to exactly one stated login and the asked-for one, a failing
URL ending `sign-in.v1` failed with a diagnostic naming IFC-049, the check and the field, holding no
URL value; its Done-when covers a plain hook's URL matched (ruling 86, point 4)."

After it comes 048a-5ef. On 2026-11-19 the owner chose, among options the coordinator put with a
recommendation, to batch the rest of the sign-in chain in pairs, waiving FR-264's one behavior per
order for them as order 048a-5b did: 048a-5e and 048a-5f become one order, 048a-5ef (the engine's
message and the input-required output, with `signInLogin`'s members and DL18's expiry, and polling
on the retry), and 048a-5g and 048a-5h one, 048a-5gh. The owner also took orders 048a-7, 048a-8
and 051b-3, the carried return, off 0.2.0's path, so that order 048b-join follows 048a-5gh.
[Ruling 88](../proposals/ruling-88/README.md) records them with the owner's words, and its point 2
keeps this order as it stands, with the Done-when line that ruling 86, point 4, gives it. This
order is unchanged by them, save that 048a-5ef, not 048a-5e, follows it.
It is on the sign-in chain, so A77's amendment of
2026-11-19 does not pair it ([work orders](../implementation/WORK_ORDERS.md)), and under
[ruling 87](../proposals/ruling-87/README.md) it runs one Codex round: where the verifier finds a
must, no second round runs, and the fresh Opus reader, given the verifier's report, and the Opus
fixer take it up.

**Written ahead.** The coordinator wrote this order on 2026-11-19 against `main` at `c097ec81`, and
against the texts of order 048a-5d as refreshed that day, of order 207, merged since as #399,
and of order 209 (`209-build-keyed-hooks-and-sign-in-fixes.md`), neither 048a-5d nor 209 yet
merged. It names what those orders build by the names their texts give.

**Its dispatch refresh.** The coordinator refreshed it on 2026-11-19, for its dispatch, against
`main` at `0840a65f`, where order 209's text (#407) and order 048a-5d's text as refreshed for its
dispatch (#408) have merged and neither build has. Every change below is the coordinator's, open to
the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):

- it was checked against 048a-5d's text as merged in #408, whole, for each point the text written
  ahead asked the coordinator to recheck at dispatch: the names 048a-5d gives
  (`hook_url_action`, `hook_input`, `keep_hook_input`, `hook_answer_cause`, `end_sign_in`,
  `HOOK_UNRELAYED_CAUSE`, `refuse-sign-in-hook-answer`, `end-sign-in-unrelayed` and its facts),
  the state of its `sign_in_hook_input_kept_nowhere`, the steps of its two cases that decisions 5
  and 6 copy, the definitions they use (URLACTION, ACCOUNT(X), HOOK(M), ENDED(O, RULE, I),
  PLAIN-SIGN(N), PLAIN-HOOK), and the DL18 sentence and the section's sentence that decision 7
  changes all stand as this text cites them, and no decision changes for them. 048a-5d has no
  Record yet, so the names are its text's; where its build on your base names one otherwise, use
  the name you find and say so;
- one change of 048a-5d's refresh that this text had not followed: its decision 10 also completes
  the hook in order 209's case `requests-sign-in-setup-check-failed-hook`, with URLACTION, at a new
  step 16 that captures `$signhook1`. Assumption (c) names it, and "Done when" runs that case alone
  too; its URL matches `device`, so it keeps its expectations;
- order 048a-5ef's text as merged (#406) still asks of this order only the passing branch reached
  with the matched login's name (decision 3) and `sign_in_url_mismatch_kept_nowhere`'s last copy,
  URLACTION with no `asked_login` (decision 4); its assumption (i) agrees with decisions 3 to 6, and
  nothing changes for it;
- ruling 88's record has merged and is cited and linked in place of the coordinator's notes;
  ruling 87's record is linked;
- under the owner's choice on 2026-11-19 of the lever "Pipeline the chain", as the coordinator
  applies it: on the predecessor's round-1 branch (the coordinator's notes (private); not yet in a
  ruling), this order may start from order 048a-5d's round-1 branch, before 048a-5d merges: the
  start check accepts either base and tells them apart by 048a-5d's Record, as 048a-5d's text tells
  its own two bases apart by 209's, and the paragraph at the foot lists what the coordinator
  rebases and rereads when 048a-5d merges (a readiness finding in the coordinator's notes
  (private): the texts written ahead refused the stacked dispatch);
- under the owner's choice on 2026-11-19 of the lever "Less prescriptive orders", as the
  coordinator applies it (the coordinator's notes (private); not yet in a ruling), the start check
  keeps only what the work needs: the check of 048a-5c's Record is dropped; so are the greps of
  `hook_answer_cause`, `end_sign_in` and `HOOK_UNRELAYED_CAUSE`, which no decision edits and whose
  use in decision 4's tests compilation checks; and so are the lengths of 048a-5d's two cases,
  which no decision reads, replaced by a check of the steps that decisions 5 and 6 copy last and
  that the mutations name. The two checks of `engine/CONFORMANCE.md`'s sentences no longer stop
  the start, since decision 7 already says what to do where 048a-5d merged them otherwise (a
  readiness finding in the coordinator's notes (private): a literal string that a fixer rewords
  would otherwise stop the next dispatch);
- in "Done when", the vectors run is folded into the whole engine suite, which runs once, last,
  after the mutations, as in 048a-5d's text; the spike's test count is no longer pinned; and the
  greps of `url_match`, `url_match_cause` and the two new tests' names, which compilation, decision
  4's tests and the reported test count already guarantee, are dropped;
- after the dispatch read (the coordinator's notes (private)), also in "Done when", the greps of
  the phrase "does not match exactly one login of the instance" and of `"asked_login"`, which check
  how the code is written, not what it does, are dropped: decision 4's unit test asserts
  `url_match_cause`'s whole text for both arms, and mutation 3 shows that decision 3 reads
  `asked_login`; the check that "with its URL unchecked" is gone now allows the base that start
  items 15 and 16 allow; and decision 3 is stated by its effect as well as by 048a-5d's step
  numbers, since 048a-5d's round-1 build does not follow its text's step order.

**What it builds.** One behavior: the match of the URL in the sign-in hook's URL request against the
logins the instance's Type states. IFC-049's observable: "A sign-in-hook.v1 URL matches exactly one
login the instance's Type states, and the one asked for where present (PND-010). Its origin and path
equal authorization; it has no user information, fragment or repeated parameter. For device it is
authorization itself, with no query, code or client. For redirect, page and service, client_id
equals clientId, response_type is code, and redirect_uri is for redirect http:// with the stated
host, one stated port (or 1024 to 65535 for any) and stated path, without query; for page and
service it is the stated page. For redirect and page, code_challenge_method is S256, code_challenge
is 43 base64url-alphabet characters (RFC 7636), and state is at least 22 RFC 3986 unreserved
characters. A URL failing a check is not relayed: sign-in.v1 ends failed with a diagnostic naming
IFC-049, the check and field, holding no URL value, code or state." PND-010: "A plain hook, one
whose implementation is marked `plain` and whose item's `keyed` does not list sign-in-hook.v1,
gets no Conduit keys (IFC-007), hence no name, so a request that asks it for a login reports
invalid before any part, and no login starts that the person did not ask for; without a name its
URL still matches one stated login under IFC-049." CTR-013
has the logins and their members, and requires that "No URL may pass IFC-049's checks for two of
its logins", which order 048a-1's check (`sign_in_login_faults` in
`engine/crates/core/src/packages.rs`) enforces at install and submission. No promise changes:
IFC-049 and CTR-013 state all of it, and this order builds it.

048a-5d's hook part reads the answer's form (one URL elicitation) and then, for a URL elicitation,
takes the interim ending: the part ends unknown and `sign-in.v1` ends failed under PND-010 with
`HOOK_UNRELAYED_CAUSE`, its URL neither checked nor relayed. This order puts the match between the
two: a URL that fails it ends `sign-in.v1` failed under IFC-049, and one that passes goes on to the
interim ending as before, which 048a-5ef replaces by the relay. So, for the Mac's dogfood, the URL
that a 0.2.0 Agent package's plain hook gives for its default login is matched before 048a-5ef
relays it (ruling 86, point 4: "the order that builds IFC-049's URL match, that its URL is
matched").

**What the orders before it bring, which this order assumes.** Orders 048a-5c, 048a-6, 048b-relay1
and 048b-relay2 have merged, each with its Record; order 207 must have merged, and the work of
orders 209 and 048a-5d must be on the base this order starts from, merged to `main` or on
048a-5d's round-1 branch ("When this order starts"). It assumes, as their decisions and the amendments their Records accept
state them; where a Record names a function or member otherwise, use that name:

- **(a) From 048a-5c:** in `engine/crates/core/src/sign_in.rs`, `stated_logins(instance, context)
  -> Result<Vec<Value>>`, the logins that the instance's Type states, read through
  `inventory::instance_type`; and the asked-for login's name kept on the `sign-in.v1` invocation as
  its member `asked_login`, a string, absent where the request asked for none.
- **(b) From 207 and 209:** PND-010's refusal before any part of a `signInLogin` aimed at a plain
  sign-in hook, so that a plain hook's `sign-in.v1` never has an `asked_login`; order 207's plain
  manifest, `conformance/conventions/requests/sign-in-login-plain-hook/manifest.json`.
- **(c) From 048a-5d:** in `sign_in.rs`, `hook_input(t, a, c, i, result)`, called from `complete`
  for a `sign-in-hook.v1` part's `input_required` answer, which in this order: (1) where the
  parent `P`, the `sign-in.v1` invocation, has ended or is not in the phase `sign-in-pending`, ends
  the part and drops the answer; (2) calls `hook_url_action(result)`, which gives the key of the
  hook's one input request and its requestState, or the first failing check of the answer's form as
  `(check, field)`; (3) where the instance is gone or retired, ends `P` failed under `PND-003`; (4)
  where the form failed `(C, F)`, ends the part unknown under IFC-010, pushes the part onto `P`'s
  `sign_in.parts`, pushes the decision `refuse-sign-in-hook-answer` citing `IFC-049` and `IFC-010`
  with the facts `{"invocation": S, "hook": H, "check": C, "field": F}`, and ends `P` failed under
  `IFC-049` with `hook_answer_cause(C, F)`; (5) where it passed, calls `keep_hook_input` and ends
  the part unknown under IFC-010 with the interim part cause; (6) pushes the part onto `P`'s
  `sign_in.parts`; and (7) pushes the decision `end-sign-in-unrelayed`, citing `PND-010`, `IFC-010`
  and `IFC-049`, with the facts `{"invocation": S, "hook": H}`, and ends `P` failed under `PND-010`
  with `HOOK_UNRELAYED_CAUSE`. Steps 5 to 7 are the interim ending. Its end, `end_sign_in(t, a, c,
  i, outcome, rule, cause)`; its unit tests `sign_in_hook_answer_form` and
  `sign_in_hook_input_kept_nowhere`; its cases `requests-sign-in-hook-part` (25 steps, on 048a-2's
  fixture, whose `signed-server` carries Conduit's keys) and `requests-sign-in-plain-hook-part` (8
  steps, on order 207's plain manifest), each completing the hook with URLACTION, whose URL is
  `https://example.org/device`; the hook added to the cases `requests-sign-in-account-first` and
  `requests-keyed-hook-dispatch` and to order 209's `requests-sign-in-setup-check-failed-hook` (its
  step 16, completing `$signhook1`), each completed with URLACTION; and DL18's text, "Until order
  048a-5e builds the relay, a URL elicitation ends `sign-in.v1` failed with a diagnostic naming
  PND-010 and that order, unrelayed and, until order 048a-5d2, with its URL unchecked, and the hook
  part ends unknown, so no URL action waits and no wait is chosen."

Where one of them merged, or 048a-5d was built on your base, otherwise than this says, apply the
nearest sound reading of each decision below to what you find, and say so; but where a check of
the start below that stops prints otherwise, stop.

**What the engine already does, so that you do not redo it.** No line numbers are given, since order
048a-5d's work, on your base, moves them: find each place by its name.

- `packages.rs` already parses URLs with the `url` crate (`use url::Url;`), which the core's
  `Cargo.toml` lists: `sign_in_login_overlap` compares two logins' `authorization` by `origin()`
  and `path()`. Decision 1 compares the same way. Add no crate.
- `stated_logins` gives an empty list where the Type states no `logins`; `hook_input` runs only for
  a `sign-in.v1` that passed `expected_account`, so the instance's Type resolves.
- Every fixture whose Type states `signIn` and whose cases complete a `sign-in-hook.v1` part
  (`sign-in-lapse-and-conflict`'s and `sign-in-hold`'s documents, which order 207's plain manifest
  and order 209's `keyed-hook-dispatch` reuse, and order 209's
  `sign-in-setup-check-failed-hook`'s, a copy of `sign-in-hold`'s with its template's selection
  added) states the same two logins: `browser`, a redirect
  login (`authorization` `https://example.org/authorize`, `clientId` `placeholder-signed-agent`,
  `redirect` host `127.0.0.1`, ports `[8400, 8401]`, path `/callback`, `carried: true`) and
  `device`, a device login (`authorization` `https://example.org/device`, the same `clientId`).
  URLACTION's URL is `device`'s `authorization` itself, so it passes decision 1 and every step that
  completes a hook with it keeps its expectation.

**When this order starts.** Order 207 must have merged to `main`, and order 048a-5ef must not have.
The work of orders 209 and 048a-5d must be on your base, the commit your working tree starts from
(`HEAD`), in one of two ways, either of which serves: 048a-5d has merged to `main`, after 209; or
the coordinator dispatched this order stacked on 048a-5d's round-1 branch, whose tip holds
048a-5d's build, committed, before 048a-5d's read and merge, with 209's work beneath it, 209 merged
or still on its own round-1 branch. You tell them apart by 048a-5d's Record: the admin writes it at
the foot of 048a-5d's file at the merge, so on 048a-5d's round-1 branch that file has no
`## Record` heading (the second check), and 209's file there may still end "Not yet run." (the
first check's second line). On either base, the two cases of 048a-5d and the three of 209 are
present and pass (the start's runner runs below). Report which base you started from and `HEAD`'s
commit. Nothing below reads which it is. On 048a-5d's round-1 branch, the copies of this order's
file and of 048a-5d's in your tree may be older than the texts that merged to `main`; where they
differ, the texts on `main` govern: read them with `git show origin/main:docs/work-orders/FILE`,
`FILE` being the file's name.
Before you change anything, check from the repository root that you find what they give, and stop,
change nothing and report if any check prints otherwise, save where an item below says not to stop:

```bash
grep -c '^Not yet run\.$' docs/work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md
grep -c '^## Record' docs/work-orders/048a5d-build-sign-in-hook-part.md
grep -c '^Not yet run\.$' docs/work-orders/048a5ef-build-sign-in-relay-and-polling.md
grep -c 'pub(crate) fn hook_input(' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn hook_url_action(' engine/crates/core/src/sign_in.rs
grep -c '^fn stated_logins(' engine/crates/core/src/sign_in.rs
grep -rl '"end-sign-in-unrelayed"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"refuse-sign-in-hook-answer"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'fn sign_in_hook_input_kept_nowhere' engine/crates/core/src/sign_in.rs
grep -rho '"asked_login"' engine/crates/core/src | wc -l | tr -d ' '
grep -c '"id": "requests-sign-in-hook-part"\|"id": "requests-sign-in-plain-hook-part"' conformance/conventions/requests/index.yaml
python3 -c 'import json; s = lambda d: json.load(open("conformance/conventions/requests/%s/case.yaml" % d))["steps"]; a = s("sign-in-hook-part"); b = s("sign-in-plain-hook-part"); print(a[7]["capture"], a[9]["input"]["invocation"], b[3]["capture"], b[4]["input"]["invocation"])'
python3 -c 'import json; c = json.load(open("conformance/conventions/requests/sign-in-plain-hook-part/case.yaml")); print(c["files"]["manifest"], json.load(open("conformance/conventions/requests/sign-in-login-plain-hook/manifest.json"))["implementations"]["signed-server"]["plain"])'
python3 -c "import json; print(*[sorted((l['name'], l['return'], l['authorization']) for x in json.load(open('conformance/conventions/requests/%s/documents.json' % d)) if x['kind'] == 'Type' and 'signIn' in x['spec'] for l in x['spec']['signIn']['logins']) for d in ('sign-in-lapse-and-conflict', 'sign-in-hold', 'sign-in-setup-check-failed-hook')], sep='\n')"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'unrelayed and, until order 048a-5d2, with its URL unchecked, and the hook part ends unknown'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Order 048a-5d takes that request into .sign-in.v1., not as IFC-016.s lapse trigger (.requests-sign-in-plain-hook-part.).'
grep -c 'fn url_match' engine/crates/core/src/sign_in.rs
ls -d conformance/conventions/requests/sign-in-url-match conformance/conventions/requests/sign-in-plain-url-match 2>/dev/null | wc -l | tr -d ' '
```

They print, in order:

1. two lines: `207-keyed-hooks-and-sign-in-spec-fixes.md:0`, order 207's Record; and
   `209-build-keyed-hooks-and-sign-in-fixes.md:0` where 209 has merged, or `:1` where it is still
   on its round-1 branch beneath 048a-5d's; report which;
2. `1` where 048a-5d has merged, or `0` on 048a-5d's round-1 branch; report which. Where it prints
   `1` and the first check's second line ends `:1`, stop: 048a-5d merges only after 209;
3. `1`: order 048a-5ef's text is in the tree with no Record, so it has not merged and the interim
   ending still stands; if the file is missing, its text has not merged either, and the interim
   ending stands as well;
4. to 6. `1` three times: 048a-5d's `hook_input` and `hook_url_action`, which decision 3 changes
   and reads, and 048a-5c's `stated_logins`;
7. and 8. `1` and `1`: the files that write 048a-5d's two decisions, whose facts decision 3 changes;
9. `1`: 048a-5d's unit test that decision 4 changes;
10. a number other than `0`: `asked_login`'s writes and reads;
11. `2`: 048a-5d's two cases;
12. `{'hook1': '/reply/hook'} $hook1 {'hook1': '/reply/hook'} $hook1`: the last steps that decisions
    5 and 6 copy, step 8 of `requests-sign-in-hook-part` and step 4 of
    `requests-sign-in-plain-hook-part`, capture the hook, and the steps that the mutations name,
    that case's step 10 and the plain case's step 5, complete it;
13. `../sign-in-login-plain-hook/manifest.json True`: 048a-5d's plain case runs on order 207's
    plain item;
14. three lines, each
    `[('browser', 'redirect', 'https://example.org/authorize'), ('device', 'device', 'https://example.org/device')]`:
    the logins that the fixtures state;
15. and 16. `1` and `1`: DL18's sentence and the last sentence of the section on ruling 86, as
    048a-5d's text leaves them. Where either prints `0`, do not stop: find the place by its meaning,
    apply decision 7 to what you find, and report it, before and after, whole;
17. and 18. `0` and `0`: none of this order's work.

Then name in your report `hook_input`, its step that tests the answer's form, its interim ending,
`stated_logins` and `sign_in_hook_input_kept_nowhere`, with their files and lines as you find them.
Still before any change, keep the rows this order changes, and from `engine/`, one at a time, run
these and keep each last line: "Done when" compares with them.

```bash
mkdir -p engine/target && grep -E '^\| (PND-010|IFC-010|IFC-049) \|' engine/STANDING.md > engine/target/standing-048a5d2-before.txt
```

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
```

Each passes. `requests-` runs the two cases of 048a-5d and the three of 209 among the others, and
each must pass and none be an expected failure; where one fails, stop and report it, since on
048a-5d's round-1 branch a failing case of 048a-5d's or 209's is that order's to fix, not this
order's.

Orders that land beside this one (050, 051b and others): whichever lands second keeps both changes
and writes `engine/STANDING.md` again with the tool.

**Who decided what.** IFC-049, PND-010 and CTR-013 decide that a sign-in hook's URL is matched to
exactly one login of the instance's Type, and to the asked-for one where one was asked for; every
check of the match; that a plain hook's URL is matched alike; and that a URL failing a check is not
relayed and ends `sign-in.v1` failed with a diagnostic naming IFC-049, the check and the field,
holding no URL value, code or state. Ruling 86, point 4, decides that this order's Done-when shows
a plain hook's URL matched. Everything else here is the coordinator's, each open to the owner's
strike ([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-5d into 048a-5d and this
order; the order of the checks, the general ones before those of a login, the names of the checks
and of their fields, and naming the login a check concerns; comparing origins and paths as the
`url` crate gives them, and parameters by their decoded values; reading "for redirect ... one stated
port" as a port written in decimal without a leading zero, and "it is the stated page" as equality
with the page's text; that a URL may hold parameters the checks do not name; which check is named
where no login matches (decision 1's rule of the latest failing row); the check `exactly one login`
for a case that CTR-013 leaves no way to reach; taking a failing URL to 048a-5d's ending for a
failing form, with the decision `refuse-sign-in-hook-answer` and a new cause; adding the matched
login's name to the facts of `end-sign-in-unrelayed`, for 048a-5ef; the unit tests; the two cases,
their steps, URLs, IDs and times; the change to 048a-5d's unit test; and the text in
`engine/CONFORMANCE.md`; and the dispatch refresh's changes listed above. Apply them as written
and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-049, its observable above all, IFC-006, IFC-007 and IFC-010
   (`spec/interface/implementation.yaml`); PND-010 (`spec/promises/pending.yaml`); CTR-013
   (`spec/promises/contracts.yaml`); `sign-in-login.v1` (`spec/schemas/contracts.yaml`).
2. Order 048a-5d (its file `048a5d-build-sign-in-hook-part.md`), whole, as merged in #408, with
   its Record where it has merged; order 048a-5c, its decisions 1 and 2 and its Record; order
   209's decision (c), and its Record where it has merged.
3. `sign_in_login_faults` and `sign_in_login_overlap` in `engine/crates/core/src/packages.rs`.
4. The cases `requests-sign-in-hook-part` and `requests-sign-in-plain-hook-part`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: S**: about 0.25 to 0.35k changed lines of Rust with two new unit tests and one changed; two
new cases, of 24 steps on 048a-2's fixture and of 8 steps on order 207's plain manifest; and two
places in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **The match.** In `sign_in.rs`, add
   `pub(crate) fn url_match(url: &str, logins: &[Value], asked: Option<&str>) -> std::result::Result<String, (&'static str, &'static str, Option<String>)>`,
   which gives the name of the one login the URL matches, or the check it fails as
   `(check, field, login)`, `login` being the name of the login the check concerns, or `None`. It
   parses the URL with `Url::parse`. A parameter's value is the value that `query_pairs()` gives
   for that name, decoded; a parameter that is absent fails its row. A login's members are read as
   strings and integers, and a member of another type fails the row that reads it.

   | Row | Check | Field | Passes where |
   | --- | --- | --- | --- |
   | 1 | `parses as a URL` | `params.url` | `Url::parse` accepts the URL |
   | 2 | `no user information` | `userinfo` | its `username()` is empty and its `password()` is `None` |
   | 3 | `no fragment` | `fragment` | its `fragment()` is `None` |
   | 4 | `no repeated parameter` | `query` | no two of its `query_pairs()` have the same name |
   | 5 | `a stated login` | `origin and path` | some login's `authorization` parses, and has the URL's `origin()` and `path()` |
   | 6 | `the asked-for login` | `origin and path` | where a login was asked for, it is one of row 5's logins |
   | 7 | `no query` | `query` | a device login: the URL's `query()` is `None` |
   | 8 | `the client ID` | `client_id` | a redirect, page or service login: the value of `client_id` equals the login's `clientId` |
   | 9 | `response type code` | `response_type` | the same logins: the value of `response_type` is `code` |
   | 10 | `the redirect URI` | `redirect_uri` | the same logins: the value of `redirect_uri` is as decision 2 says |
   | 11 | `S256` | `code_challenge_method` | a redirect or page login: the value of `code_challenge_method` is `S256` |
   | 12 | `a code challenge` | `code_challenge` | the same logins: the value of `code_challenge` has exactly 43 characters, each an ASCII letter, an ASCII digit, `-` or `_` |
   | 13 | `a state` | `state` | the same logins: the value of `state` has at least 22 characters, each an ASCII letter, an ASCII digit, `-`, `.`, `_` or `~` |
   | 14 | `exactly one login` | `params.url` | no two candidates pass every row of 7 to 13 that applies to them |

   It decides in this order. Rows 1 to 5 are tested in turn, each failing with `login` `None`.
   Row 6 fails with the asked-for name as `login`. The candidates are then row 5's logins, in the
   order the Type states them, or, where a login was asked for, that login alone. A login whose
   `return` is none of the four is not a candidate; report whether any path reaches one. Where no
   candidate remains, it fails row 14 with `login` `None`. Each candidate is tested against the
   rows of 7 to 13 that apply to its `return`, in row order, and passes where it passes all of
   them. Where exactly one candidate passes, the function gives its `name`. Where two or more
   pass, it fails row 14 with `login` `None`: CTR-013's check at install leaves no way to reach
   this; report whether any path does. Where none passes, it fails the first failing row of the
   candidate whose first failing row comes latest in the table, the first such candidate in the
   Type's order where several tie, with that candidate's name as `login`.
   The function reads nothing but its arguments, and its `Err` holds no part of the URL.

2. **The redirect URI.** For a redirect login, with `redirect.host` H, `redirect.ports` P and
   `redirect.path` Q, the value of `redirect_uri` passes row 10 where it is the text
   `http://`, then H, then `:`, then a port, then Q, with nothing before or after, the port being
   one or more ASCII digits not starting with `0`, whose number is in P where P is a list (read
   as 048a-5c's `may_start_fixed_ports` reads a port, a whole-valued decimal included) and from
   1024 to 65535 where P is `any`. For a page or service login, it passes where it equals the
   login's `page`, byte for byte. Nothing else is compared, and the value is never parsed as a URL.

3. **`hook_input` matches the URL.** In `hook_input`, where `hook_url_action` passed, after
   048a-5d's step 4 and before its step 5, add a step: take the URL, the string `params.url` of
   the member of `result`'s `inputRequests` under the key `K` that `hook_url_action` gave; read
   the logins with `stated_logins` for the instance, `P`'s `object`; and call
   `url_match(url, &logins, P["asked_login"].as_str())`. By its effect, since 048a-5d's build may
   not follow its text's step order: the match runs only where the form passed and the instance is
   present and not retired, and before `keep_hook_input` is called; on a mismatch,
   `keep_hook_input` is never called.
   - Where it fails `(C, F, L)`: do as 048a-5d's step 4 does for a failing form, ending the part
     with the same cause, without calling `keep_hook_input`, and pushing the part onto
     `sign_in.parts`, but give the decision `refuse-sign-in-hook-answer` the facts
     `{"invocation": S, "hook": H, "check": C, "field": F}`, with the member `"login": L` added
     where `L` is a name, and end `P` failed under `IFC-049` with the cause
     `url_match_cause(C, F, L)`; then return.
   - Where it gives a name `L`: go on to 048a-5d's step 5, as now. In step 7, the decision
     `end-sign-in-unrelayed` gains the member `"login": L` in its facts, which become
     `{"invocation": S, "hook": H, "login": L}`. 048a-5ef removes the decision and keeps `L` for its
     relay, whose decision `relay-sign-in-url` names it. `HOOK_UNRELAYED_CAUSE` does not change.
   - Where 048a-5d's build shares one ending or a helper, such as `refuse_hook_answer`, between
     these branches, extend it rather than duplicate it, and report the shape you found (the
     coordinator's, from the dispatch read).

   Add `pub(crate) fn url_match_cause(check: &str, field: &str, login: Option<&str>) -> String`,
   giving, where `login` is a name `N`,
   `The sign-in hook's URL does not match the login "N" of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check "{check}" at the field {field}.`,
   and, where it is `None`,
   `The sign-in hook's URL does not match exactly one login of the instance's Type, so nothing was relayed and the instance is not signed in: it fails the check "{check}" at the field {field}.`
   `N` is written as the login's name, between the quotes shown. Nothing of the URL, its message or
   its requestState reaches a reply, a record, a decision's facts, an intent or a diagnostic
   (IFC-049: "holding no URL value, code or state"). The match runs for a plain hook's answer and a
   keyed one's alike, as `hook_input` does; a plain hook is never asked for a login (order 209), so
   its URL is matched against every stated login.

4. **The unit tests.** Write the values as `json!` values. Let CC be
   `E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM` (RFC 7636's example challenge, 43 characters)
   and ST be `Wt3qK8xZp_LmN2vB-c.Y7h~R` (24 characters). Let LOGINS be five logins: `browser` and
   `device` as the fixtures state them (see "What the engine already does"); `loopback`, a redirect
   login with `authorization` `https://auth.example.net/oauth/authorize`, `clientId`
   `loopback-client`, `redirect` `{"host": "localhost", "ports": "any", "path": "/auth/callback"}`
   and `carried` `true`; `code-page`, a page login with `authorization`
   `https://pages.example.com/oauth/authorize`, `clientId` `page-client`, `page`
   `https://pages.example.com/oauth/code/callback` and `carried` `false`; and `service`, a service
   login with the same `authorization`, `clientId` `service-client` and `page`
   `https://service.example.com/done?from=login`. Let these URLs be:
   - BROWSER: `https://example.org/authorize?response_type=code&client_id=placeholder-signed-agent&redirect_uri=http%3A%2F%2F127.0.0.1%3A8401%2Fcallback&scope=openid&code_challenge=CC&code_challenge_method=S256&state=ST`;
   - LOOPBACK: `https://auth.example.net/oauth/authorize?response_type=code&client_id=loopback-client&redirect_uri=http%3A%2F%2Flocalhost%3A1455%2Fauth%2Fcallback&code_challenge=CC&code_challenge_method=S256&state=ST`;
   - PAGE: `https://pages.example.com/oauth/authorize?response_type=code&client_id=page-client&redirect_uri=https%3A%2F%2Fpages.example.com%2Foauth%2Fcode%2Fcallback&code_challenge=CC&code_challenge_method=S256&state=ST`;
   - SERVICE: `https://pages.example.com/oauth/authorize?response_type=code&client_id=service-client&redirect_uri=https%3A%2F%2Fservice.example.com%2Fdone%3Ffrom%3Dlogin`;

   CC and ST being written in place. "X with `p` V" below is X with parameter `p`'s value replaced
   by V, written percent-encoded where the URL needs it.
   - In `sign_in.rs`'s tests, `sign_in_url_match`, asserting with `assert_eq!` that `url_match`
     with LOGINS and no asked-for login gives:
     - `Ok("device")` for `https://example.org/device`, and also with the asked-for `device`;
       `Ok("browser")` for BROWSER, also with the asked-for `browser`, for BROWSER with
       `redirect_uri` on port 8400, and for BROWSER without `scope` and with `prompt=login` added;
       `Ok("loopback")` for LOOPBACK, and for LOOPBACK with `redirect_uri` on port 1024 and on
       port 65535; `Ok("code-page")` for PAGE; `Ok("service")` for SERVICE;
     - `Err(("parses as a URL", "params.url", None))` for `not a url`;
     - `Err(("no user information", "userinfo", None))` for `https://example.org/device` with
       the user information `user`, and with `user:secret`, before its host, joined to it by the
       at sign;
     - `Err(("no fragment", "fragment", None))` for `https://example.org/device#x` and for
       `https://example.org/device#`;
     - `Err(("no repeated parameter", "query", None))` for BROWSER with `&state=ST` appended;
     - `Err(("a stated login", "origin and path", None))` for `https://example.org/devices`,
       `http://example.org/device` and `https://example.org:8443/device`;
     - `Err(("the asked-for login", "origin and path", Some("browser".into())))` for
       `https://example.org/device` with the asked-for `browser`;
     - `Err(("no query", "query", Some("device".into())))` for
       `https://example.org/device?user_code=WDJB-MJHT` and for `https://example.org/device?`;
     - with `Some("browser".into())` as the login: `("the client ID", "client_id", ..)` for
       BROWSER with `client_id` `another-client`, and without `client_id`;
       `("response type code", "response_type", ..)` for BROWSER with `response_type` `token`;
       `("the redirect URI", "redirect_uri", ..)` for BROWSER with `redirect_uri` on port 8402,
       on port `08401`, with the host `localhost` in place of `127.0.0.1`, with the path
       `/callback/x`, with the scheme `https`, and with `?x=1` appended to it;
       `("S256", "code_challenge_method", ..)` for BROWSER with `code_challenge_method` `plain`;
       `("a code challenge", "code_challenge", ..)` for BROWSER with CC's last character removed,
       and with CC's last character replaced by `=`; `("a state", "state", ..)` for BROWSER with
       ST's last three characters removed, and with ST's last character replaced by `!`;
     - `Err(("the redirect URI", "redirect_uri", Some("loopback".into())))` for LOOPBACK with
       `redirect_uri` on port 1023;
     - `Err(("the redirect URI", "redirect_uri", Some("code-page".into())))` for PAGE with
       `redirect_uri` `https://service.example.com/done?from=login`: `code-page` fails row 10 and
       `service` row 8, so the later row is named;
     - `Err(("response type code", "response_type", Some("service".into())))` for SERVICE without
       `response_type`;
     - and, with LOGINS' `device` and a copy of it named `device-again`, the two alone,
       `Err(("exactly one login", "params.url", None))` for `https://example.org/device`.

     Last, `url_match_cause("no query", "query", Some("device"))` and
     `url_match_cause("a stated login", "origin and path", None)` each give decision 3's text
     whole.
   - 048a-5d's `sign_in_hook_input_kept_nowhere` builds `s1` with no Type, which `stated_logins`
     now reads. Give `s1` the `type` and `held` members that `explain.rs`'s test
     `sign_in_keyed_repeats_require_url_elicitation` gives its Agent, with the core's `type.v1`
     stating `signIn` `{"account": "/account", "logins": [browser, device]}`, the two logins of
     the fixtures, and change nothing else of the test: its assertions hold as they stand, since
     URLACTION's URL matches `device`. Say in your report how you built it, and where a helper
     shares it with the next test.
   - In `sign_in.rs`'s tests, `sign_in_url_mismatch_kept_nowhere`: on a fresh copy of that
     test's `Transition`, with `h1`'s `plain` `true`, call `complete` for `h1`, attempt 1, with
     `Completion::Result { result: R }`, R being URLACTION with `params.url`
     `https://example.org/device?user_code=WDJB-MJHT`, and assert: `h1`'s `state` is `unknown`, it
     has no member `hook_input`, and its `cause` is one diagnostic whose `ruleId` is `IFC-010`;
     `g1`'s `state` is `failed`, its `sign_in.phase` `ended` and its last part
     `{"name": "sign-in", "outcome": "unknown", "invocation": "h1"}`; `t.reply`'s diagnostics are
     one, whose `ruleId` is `IFC-049` and whose text ends with
     `url_match_cause("no query", "query", Some("device"))`; one decision is named
     `refuse-sign-in-hook-answer`, with the facts
     `{"invocation": "g1", "hook": "h1", "check": "no query", "field": "query", "login": "device"}`;
     none is named `end-sign-in-unrelayed` or `raise-sign-in-action`;
     `t.state.pending_actions` and `t.state.sign_in_holds` are empty; and the string
     `serde_json::to_string` gives for
     `json!([t.reply, t.records, t.request_replies, t.external_intents, t.state.records])`, and the
     facts of every decision in `t.decisions`, hold none of `WDJB-MJHT`, `user_code`,
     `example.org/device`, `opaque-hook-state-0001` and `Open the device page.`. On a fresh copy
     with `g1`'s member `asked_login` `"browser"` and URLACTION itself, the same holds, but for a
     diagnostic ending with
     `url_match_cause("the asked-for login", "origin and path", Some("browser"))` and the facts
     `{"invocation": "g1", "hook": "h1", "check": "the asked-for login", "field": "origin and path", "login": "browser"}`.
     On a fresh copy with URLACTION itself and no `asked_login`: `g1` is `failed` with one
     diagnostic whose `ruleId` is `PND-010` and whose text ends with `HOOK_UNRELAYED_CAUSE`; the
     decision `end-sign-in-unrelayed` has the facts
     `{"invocation": "g1", "hook": "h1", "login": "device"}`; none is named
     `refuse-sign-in-hook-answer`; and the same absences hold.

5. **One portable case, on 048a-2's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-url-match/` with one file, `case.yaml`: ID
   `requests-sign-in-url-match`, `layers: [convention, interface]`, `promises: [PND-010, IFC-049]`,
   title
   `Match the sign-in hook's URL to exactly one stated login and the asked-for one, and end sign-in failed under IFC-049 on a URL that does not match`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and the `files` of
   `requests-sign-in-hook-part`, unchanged. Write it as 048a-5d's decision 7 writes that case. It
   must meet 046a's portable-case rule as it stands.

   Definitions are 048a-5d's decision 7's, and its "Root asks T with A and M", S, URL and LOGIN(X)
   048a-5's. A new step's evidence is its time, `2026-11-12T12:00:00Z`, and `ids` the 100 strings
   `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
   190000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings `url-match-` followed by the
   decimal of n, a `-` and the decimal of j, and `suffixes` the 16 strings `u` followed by the
   four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n is the step's number in
   this case; a copied step keeps its own evidence. URLOF(U) is 048a-5d's URLACTION with its
   `params.url` U in place of `https://example.org/device`, the rest unchanged; BROWSER is decision
   4's, with CC and ST written in place. Its steps:
   1. to 8. `requests-sign-in-hook-part`'s steps 1 to 8, `input`, `expect` and `capture`, whole,
      each with its evidence: first start, registration, the setup with its account check, a
      sign-in `$signin1` whose check `$check1` finds no account, and its hook `$hook1`.
   9. A completion for `$hook1`, URLOF(BROWSER). `expect` ENDED(`"failed"`, `"PND-010"`,
      `"$signin1"`): the redirect login's URL is matched, and meets the interim ending.
   10. Root asks `group.sign-in.v1` with S and URL. `expect` step 7's; `capture`
       `{"signin2": "/reply/invocation", "check2": "/reply/hook"}`.
   11. A completion for `$check2`, ACCOUNT(`null`). `expect` step 8's; `capture`
       `{"hook2": "/reply/hook"}`.
   12. A completion for `$hook2`, URLOF(BROWSER with `redirect_uri` on port 8402). `expect`
       ENDED(`"failed"`, `"IFC-049"`, `"$signin2"`): a port the login does not state.
   13. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "browser"}`). `expect` step
       7's; `capture` `{"signin3": "/reply/invocation", "check3": "/reply/hook"}`.
   14. A completion for `$check3`, ACCOUNT(`null`). `expect` `{"reply": {"pending": true}}` with
       HOOK(M) whose `M` also holds `"META/signInLogin": {"name": "browser"}`, and
       `"absent": [K(conduit), K(idempotencyKey)]`; `capture` `{"hook3": "/reply/hook"}`.
   15. A completion for `$hook3`, URLACTION. `expect` ENDED(`"failed"`, `"IFC-049"`,
       `"$signin3"`): the device login's URL, where `browser` was asked for.
   16. to 18. Steps 13 to 15, each capture's names ending in `4` in place of `3`, with
       `{"name": "device"}` in place of `{"name": "browser"}` in steps 16 and 17. Step 18 `expect`s
       ENDED(`"failed"`, `"PND-010"`, `"$signin4"`): the asked-for login's URL is matched.
   19. to 21. Steps 10 to 12, each capture's names ending in `5` in place of `2`, step 21's
       completion for `$hook5` being URLOF(`https://example.org/device?user_code=WDJB-MJHT`). Step
       21 `expect`s ENDED(`"failed"`, `"IFC-049"`, `"$signin5"`): a device URL carries no code.
   22. to 24. Steps 10 to 12, each capture's names ending in `6`, step 24's completion for
       `$hook6` being URLOF(BROWSER with the host `example.net` in place of `example.org`). Step 24
       `expect`s ENDED(`"failed"`, `"IFC-049"`, `"$signin6"`): a URL of no stated login.

6. **One portable case of a plain hook, on order 207's plain manifest (ruling 86, point 4).** Add
   the directory `conformance/conventions/requests/sign-in-plain-url-match/` with one file,
   `case.yaml`: ID `requests-sign-in-plain-url-match`, `layers: [convention, interface]`,
   `promises: [PND-010, IFC-010, IFC-049]`, title
   `Match a plain sign-in hook's URL to a stated login, and end sign-in failed under IFC-049 on a plain hook's URL that does not match`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and the `files` of
   `requests-sign-in-plain-hook-part`, unchanged. Write it as decision 5 writes its case.
   Definitions as decision 5's, with 048a-5d's PLAIN-SIGN(N) and PLAIN-HOOK; a new step's `ids` are
   the 100 strings `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
   210000 + 100n + k, `revisions` the 16 strings `plain-url-match-` followed by the decimal of n, a
   `-` and the decimal of j, and `suffixes` the 16 strings `w` followed by the four-digit,
   zero-padded decimal of 100n + j. Its steps:
   1. to 4. `requests-sign-in-plain-hook-part`'s steps 1 to 4, `input`, `expect` and `capture`,
      whole, each with its evidence: first start, registration, PLAIN-SIGN(1), and the plain check
      that finds no account, going on to the plain hook `$hook1`.
   5. A completion for `$hook1`, URLOF(BROWSER). `expect` ENDED(`"failed"`, `"PND-010"`,
      `"$signin1"`): the plain hook's URL for the redirect login is matched, and meets the interim
      ending.
   6. PLAIN-SIGN(2).
   7. A completion for `$check2`, with the completion value of step 4's `input`. `expect`
      PLAIN-HOOK; `capture`
      `{"hook2": "/reply/hook"}`.
   8. A completion for `$hook2`, URLOF(BROWSER with ST's last three characters removed). `expect`
      ENDED(`"failed"`, `"IFC-049"`, `"$signin2"`): a state of 21 characters fails the match.

   The routes of decisions 5 and 6 are new and have not been run. Before running either, check
   that no ID repeats across its steps' `ids`, the copied steps' included. If 046a's runner check
   refuses a case, if a copied step gives a reply other than its case states, or if a step's reply
   differs from what it states, stop and report the error: do not change the expectations to pass
   it, and do not change the inputs beyond what these decisions say. No expectation is generated
   from the engine's output. Each coverage index entry holds `id`, `case`
   (`conventions/requests/sign-in-url-match/case.yaml` and
   `conventions/requests/sign-in-plain-url-match/case.yaml`), `promises` and, where its neighbours
   in that piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write
   both in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves them
   into `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte
   for byte as it was. No other case changes: every other step that completes a hook does so with
   URLACTION, whose URL matches `device`; where one now gives another reply, stop and report it.

7. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each place by its text read across
   line breaks, as 048a-5d leaves it.
   - In DL18's row, replace "unrelayed and, until order 048a-5d2, with its URL unchecked," with
     "unrelayed, once its URL has matched a stated login (order 048a-5d2),". Then, right after the
     sentence that begins "A hook answer that is not one URL elicitation ends `sign-in.v1` failed
     under IFC-049", add:

     ```text
     From order 048a-5d2, the hook's URL is matched under IFC-049 to exactly one login of the instance's Type, and to the asked-for one where a login was asked for, a plain hook's alike; a URL that does not match ends `sign-in.v1` failed under IFC-049, the hook part ending unknown, with a diagnostic naming the check, the field and, where the check concerns one, the login, and holding no URL value, code or state (`requests-sign-in-url-match`, `requests-sign-in-plain-url-match`). Where no login matches, the check named is the latest that any candidate reached, which IFC-049 leaves to the engine.
     ```

   - In the section "Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)", after
     the sentence "Order 048a-5d takes that request into `sign-in.v1`, not as IFC-016's lapse
     trigger (`requests-sign-in-plain-hook-part`).", add: "Order 048a-5d2 matches its URL as it
     matches a keyed hook's (`requests-sign-in-plain-url-match`)." Change no other word of the
     section.

   If 048a-5d merged either place otherwise, change what it merged in their place, and report each
   before and after, whole. Nothing else in the file changes. After the other decisions, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
   `promises`. Report the rows of PND-010, IFC-010 and IFC-049 and the totals; the totals change
   only by what those rows change.

8. **The declaration.** `docs/work-orders/048a5d2-declared.yaml` holds two shapes, one `added` for
   each new case. The coordinator wrote it with this order; check that it matches and change it
   only to make it so.

9. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
   record changes. No fixture or manifest changes. Nothing is relayed: the engine's message, the
   input-required output and polling (048a-5ef) are not built, and no shell code, `runner.rs`, CLI
   command (051b) or package (050) changes. The interim ending's cause and decision, and their
   naming of order 048a-5e, stay as 048a-5d wrote them, for 048a-5ef to remove. Report as findings,
   for the coordinator: that IFC-049 says the URL "contains no credential or personal data and is
   not pre-authenticated", and that no check here reads a parameter the rows do not name, such as a
   login hint holding an address, since the 0.2.0 Agent packages' URLs carry parameters of their
   own; that IFC-049 does not say which check is named where no login matches, so decision 1's
   latest-row rule is the engine's; that origins and paths are compared as the `url` crate
   normalizes them (case of the host, a default port written out), and parameters by their decoded
   values; whether row 14, or a candidate whose `return` is none of the four, is reached by any
   path; that the cases assert no diagnostic's words, since IFC-050 leaves wording to the engine, so
   the unit tests alone pin the check, the field and the login named, and the absence of the URL's
   values; and any place where IFC-049, PND-010 or CTR-013 left you a choice this order does not
   settle.

## Scope

Change only `engine/crates/core/src/sign_in.rs` (`url_match`, `url_match_cause`, `hook_input`'s new
step and step 7's facts, and the tests of decision 4), `engine/CONFORMANCE.md` (decision 7's two
places), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml`, and the new directories
`conformance/conventions/requests/sign-in-url-match/` and
`conformance/conventions/requests/sign-in-plain-url-match/`. Where 048a-5d put `hook_input` or its
tests in another file, change them there, and only there, and say so.
`docs/work-orders/048a5d2-declared.yaml` comes with this order; change it only as decision 8 says.
`conformance/manifest.yaml` holds the new entries only until the pieces tool moves them, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, `requests.rs`, `lifecycle.rs`, the
shell, the experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any fixture, manifest or other case, `docs/implementation/` or
`docs/proposals/`: report any defect as a finding. `experiments/011-short-form-spike/` changes only
if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-url-match
cargo run --release -p conduit-conformance -- .. requests-sign-in-plain-url-match
cargo run --release -p conduit-conformance -- .. requests-sign-in-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-plain-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-dispatch
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell first_start_presents_final_lists
```

The third reports two more tests passed than the start's run, 0 failed, the new ones being
`sign_in_url_match` and `sign_in_url_mismatch_kept_nowhere`; report both counts. The pieces tool
prints `Nothing to move.` The seven single-case runner commands each end `1 cases passed`;
`requests-sign-in-` gives two more cases than at the start, every one passing, and `requests-` two
more cases than at the start, with the start's expected failures, each still failing; report each
count. The shell test passes. If any other case, or any shell, CLI or experiment test, fails
because a hook's URL is now matched, stop and report it: do not change it. The threads are 4
because at 24 the runner ran out of memory (order 071's Record).

Next, show that the match is what the cases check, a plain hook's included. From `engine/`, first
keep a copy:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
```

Then make each change below by hand, alone, in `hook_input`'s call of `url_match` (decision 3), run
its commands, restore the file with `cp` from its copy, and run them again. Each run before the
restore must print `FAIL 0 cases passed; 1 failed`, failing at the step named, and each run after
it `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time,
so cargo would not rebuild the crate. Git ignores `target/`. Report the error each failing run
printed.

| Change, by hand | Commands (`cargo run --release -p conduit-conformance -- ..` and) | Must fail at |
| --- | --- | --- |
| Replace the call's result with `Ok(String::from("device"))`, so that every URL matches | `requests-sign-in-url-match`, then `requests-sign-in-plain-url-match` | step 12, then step 8: a URL that matches no login meets the interim ending under PND-010 where IFC-049 is expected |
| Replace the call's result with `Err(("a stated login", "origin and path", None))`, so that no URL matches | `requests-sign-in-plain-url-match`, then `requests-sign-in-hook-part` | step 5, then step 10: a plain hook's matching URL, then a keyed hook's, ends under IFC-049 where the interim ending under PND-010 is expected (ruling 86, point 4) |
| Pass `None` in place of the asked-for login | `requests-sign-in-url-match` | step 15: the device login's URL matches where `browser` was asked for |

Last, with the file restored, run the whole engine suite once, from `engine/`, and report its last
lines; it passes, the vectors among it:

```bash
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release
```

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 -m unittest discover -s tools/tests
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5d2-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/fixtures conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-account-first conformance/conventions/requests/sign-in-fixed-port conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/requests/keyed-hook-dispatch conformance/conventions/requests/keyed-not-in-implements conformance/conventions/requests/sign-in-setup-check-failed-hook conformance/conventions/requests/sign-in-hook-part conformance/conventions/requests/sign-in-plain-hook-part conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/core/src/requests.rs engine/crates/core/src/lifecycle.rs engine/crates/shell tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5d2-declared.yaml: 2 shapes.`,
then `Vector delta: 2 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. No promise's text changes, so no row of the model-checking account goes stale; where
`check_model_account.py` reports one stale, stop and report it. The unit tests end `OK`; report the
count. The `git status` command prints nothing, or a change under the spike's directory, reported
with its reason, is the only line it may print.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
RUST_TEST_THREADS=4 cargo test --release
```

Its `tests/spike.rs` run passes with 0 failed; report its count.

Each of these prints 1, and each printed 0 at the start. The `tr` checks read the file with its
line breaks as spaces, so a rewrap hides nothing:

```bash
grep -c '"id": "requests-sign-in-url-match"' conformance/conventions/requests/index.yaml
grep -c '"id": "requests-sign-in-plain-url-match"' conformance/conventions/requests/index.yaml
grep '^| DL18 |' engine/CONFORMANCE.md | grep -c 'From order 048a-5d2, the hook.s URL is matched under IFC-049 to exactly one login'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'unrelayed, once its URL has matched a stated login (order 048a-5d2),'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Order 048a-5d2 matches its URL as it matches a keyed hook.s (.requests-sign-in-plain-url-match.).'
```

This prints 0; where the start's item 15 printed 1, it printed 1 then. Where it printed 0, report
instead the DL18 text decision 7 replaced, before and after.

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'with its URL unchecked'
```

This prints `IFC-010:1 IFC-049:2 PND-010:2`, each row's number of cases against the start's, and
printed all zeros at the start:

```bash
python3 -c 'import re; rows = lambda p: {l.split("|")[1].strip(): int(l.split("|")[4]) for l in open(p) if re.match(r"^\| (PND-010|IFC-010|IFC-049) \|", l)}; b = rows("engine/target/standing-048a5d2-before.txt"); a = rows("engine/STANDING.md"); print(" ".join(k + ":" + str(a[k] - b[k]) for k in sorted(b)))'
```

This prints `24 8`, the new cases' lengths, and
`['$hook1', '$hook2', '$hook3', '$hook4', '$hook5', '$hook6'] ['$hook1', '$hook2']`, the steps that
complete the hooks:

```bash
python3 -c 'import json; print(*[len(json.load(open("conformance/conventions/requests/%s/case.yaml" % d))["steps"]) for d in ("sign-in-url-match", "sign-in-plain-url-match")])'
python3 -c 'import json; s = lambda d: json.load(open("conformance/conventions/requests/%s/case.yaml" % d))["steps"]; print([s("sign-in-url-match")[k]["input"]["invocation"] for k in (8, 11, 14, 17, 20, 23)], [s("sign-in-plain-url-match")[k]["input"]["invocation"] for k in (4, 7)])'
```

This prints 0, as at the start:

```bash
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 7 names with the totals, the names, files and lines that the start
asks for, the signatures of `url_match` and `url_match_cause`, the line in `hook_input` where
decision 3's step stands, the DL18 sentences and the section's sentence that decision 7 changed,
before and after, and whether any path reaches row 14 or a candidate whose `return` is none of the
four.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5d2-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. Under ruling 87 a fresh Codex verifier reads the one round, and a fresh read-only Opus
reader reads the result, and the verifier's report where it holds a must, before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2, as ruling 87 amends it). The admin writes
this order's Record at the foot of this file when it merges.

Where the coordinator dispatched this order stacked on order 048a-5d's round-1 branch, the result
is read there, with `tools/gate.py --base` set to the commit this order started from, that
branch's tip, since `origin/main` lacks 048a-5d's work (and, until 209 merges, 209's); it merges
only after 048a-5d has merged, and then only once the coordinator has done the following (the
coordinator's, under that lever):

- rebase this order's branch onto `main` with `git rebase --onto origin/main` from that commit,
  048a-5d's round-1 tip as this order started from it, so that only this order's commits are
  replayed onto 048a-5d's squash, committing and rebasing under the work orders' identity. Where
  048a-5d's branch was itself rebased, onto 209's squash, after this order started, the commit
  this order started from is still the one to rebase from;
- resolve any conflict where 048a-5d's verifier, reader or fixer changed what this order changes
  or reads after 048a-5d's round 1: `hook_input` and the tests in
  `engine/crates/core/src/sign_in.rs`, `sign_in_hook_input_kept_nowhere` among them, DL18's row
  and the section on ruling 86 in `engine/CONFORMANCE.md`, and
  `conformance/conventions/requests/index.yaml`, keeping 048a-5d's merged text and applying this
  order's decisions to it; write `engine/STANDING.md` again with
  `python3 tools/promise_standing.py --write` rather than by hand, and run the pieces tool again
  where the index conflicts;
- run this order's start check in a clean worktree of `main` after 048a-5d's merge, and where
  048a-5d's merge changed anything that a decision reads (a name, a decision's facts, a step of
  048a-5d's cases that decisions 5 and 6 copy, a definition they use, the DL18 or section sentence
  that decision 7 changes, the state of `sign_in_hook_input_kept_nowhere`), amend this order's
  text and the build to it, saying so in the Record;
- run again the seven single-case runner commands, `requests-sign-in-` and `requests-`, the
  whole engine suite, the `vector_delta.py` check with this order's declaration, and
  `tools/gate.py` against `origin/main`;
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.

## Record

Run on 2026-11-19, from about 18:15 to 18:48 UTC, on the Mac, stacked on order 048a-5d's round 1
checkpoint `1a067eff`, itself on order 209's round 1 checkpoint `08e706fa`, in one round: the
worker (1633 s), then a fresh Codex verifier (322 s). The change is the six files of Scope.
`sign_in.rs` gains `url_match`, decision 1's 14 rows in their order with decision 2's redirect URI,
and `url_match_cause`; `hook_input` matches the URL after the form and retirement checks and before
`keep_hook_input`, and a mismatch ends `sign-in.v1` failed under IFC-049 with the hook part unknown
under IFC-010, where a match adds the login's name to the interim ending under PND-010. The unit
tests `sign_in_url_match` and `sign_in_url_mismatch_kept_nowhere` share `hook_part_fixture`, which
builds h1's contract and held target through `publication_object`. The cases
`requests-sign-in-url-match` (24 steps) and `requests-sign-in-plain-url-match` (8 steps) copy the
predecessors' steps with their evidence; DL18 and the ruling 86 section change as decision 7 says,
and STANDING moves by `IFC-010:1 IFC-049:2 PND-010:2`. The worker's own choices: failures ranked
by explicit row numbers over a map of decoded parameters, a login with a non-string name never a
candidate; `HookAnswerFault` extending the shared refusal helper, with an active-subject lookup
shared by both hook endings; the fixture's held sign-in hook Operation and published target
snapshot; a whole-valued decimal port test; and a type annotation for the all-`Err` mutation. It
reported 24 sign_in tests (22 at the start), 15 and 82 cases with the 2 expected failures (13 and
80 at the start), the three mutations failing at the steps named, 382 engine tests passed, the
spike's 9 tests passed, and `Vector delta: 2 changes; 0 flips; 0 undeclared; 0 errors.` It noted
five findings: an extra parameter's value gets no content check (F01), the latest failing row on
ties (F02), `url` crate normalization of origin and path (F03), no admitted Type reaching row 14 or
an unknown return kind (F04), and diagnostic wording pinned by unit tests alone (F05).

Codex's verifier found no must and six shoulds, each a test-coverage gap: no test of decision 1's
"that login alone"; the mismatch test's surfaced values omitting `t.input_required`; the check that
h1 holds no `hook_input` after a mismatch running only after `requests::finish` removes it; no user
information input with an empty username and a password; no tie between candidates failing the same
row; and no page comparison that a normalizing comparison would pass.

A fresh read-only Opus reader then verified the order (ruling 79, point 2, as ruling 87 amends it)
at `59ddb296`. Warm, in the worktree, it ran fmt and clippy (exit 0), the sign_in tests (24
passed), the pieces tool (`Nothing to move.`), the six single-case runs and
`requests-sign-in-setup-check-failed-hook` (1 cases passed each), `requests-sign-in-` (15) and
`requests-` (82 and the same 2 expected failures), the shell test (1 passed), the three mutations
(each failing at its step and error, each passing after the `cp` restore), the Python checks
(Errors: 0 each), the tool tests (OK), `vector_delta.py 1a067eff HEAD` (2 changes, 0 flips, 0
undeclared), the order's greps, the standing delta, the cases' lengths and hook steps, and the
commit identity. It checked that the copied steps are byte-equal to order 048a-5d's and that no
evidence ID repeats (2374 and 774), and that putting the fixture's held target back to the bare
instance fails both transition tests. It found one must and six shoulds, the shoulds confirming the
verifier's six:

- R1: `grep -c 'does not match exactly one login of the instance'` on `sign_in.rs` printed 1 where
  the order asks 2, since `url_match_cause` spliced the login into one `format!`.
- R2 to R7: the verifier's six, in its order. Deleting the asked-for filter (R2), removing the
  password check (R5) or writing `>=` for the tie's `>` (R6) left every test passing; R3 named the
  omitted `t.input_required`, R4 the assertion that `requests::finish` makes vacuous, and R7 a page
  mismatch that a normalizing comparison would also reject.

The fixer kept the branch on `1a067eff`, without a rebase, and applied each finding:

- R1: `url_match_cause` writes decision 3's two whole texts, one `format!` each; the grep prints 2.
- R2: `sign_in_url_match` rejects the service URL with `code-page` asked for, at the client ID,
  naming `code-page`.
- R3: the mismatch test's shared ending assertion checks `t.input_required` is empty.
- R4: the cheaper choice the reader offered: that a mismatch ends without calling
  `keep_hook_input` is checked by reading `hook_input` (the call stands under `matched.is_ok()`),
  not by a test, since `requests::finish` removes `hook_input` from every ending invocation.
- R5: the user-information loop adds a URL on the device endpoint with an empty username and a
  password, rejected at `no user information`.
- R6: the page URL without its client ID, where `code-page` and `service` both fail row 8, names
  `code-page`, the first in the Type's order.
- R7: the page URL whose stated page has its host capitalized names `code-page` at the redirect
  URI, `service` failing the earlier row 8.

On the worktree, each of R2, R5, R6 and R7's mutations (the asked-for filter deleted, the password
check removed, `>=` for `>`, and the page compared lowercased) failed `sign_in_url_match` at its new
assertion, and the file was restored with `cp` and compared equal. The fixer then ran, warm, from
`engine/`: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` (exit 0), the sign_in
tests (`24 passed; 0 failed`), the pieces tool (`Nothing to move.`), the six single-case runs (`1
cases passed` each), `requests-sign-in-` (`15 cases passed`), `requests-` (`82 cases passed; 2
expected failures`), the shell test (1 passed), and the order's three mutations, which failed at
step 12 then step 8, step 5 then step 10, and step 15, each with the error the order names, and
passed again after the `cp` restore; and from the root, `python3 tools/check_docs.py` and
`python3 tools/check_private_paths.py` (Errors: 0 each) and `git diff --check` (clean). The vectors,
the full engine suite and the spike were not rerun: the fix changes only `url_match_cause`'s form,
whose texts are unchanged, and tests. The gate's run on this order's squash, which the merge waits
on, is the coordinator's at the merge.

After order 048a-5d merged (#414, `148258e6`), on 2026-11-19, the branch was rebased onto `main` at
`593707c3` with `git rebase --onto origin/main 1a067eff`, after a check that `1a067eff`, 048a-5d's
round-1 checkpoint this order started from, was an ancestor of its tip and that 048a-5d's squash was
on `main`, so that only this order's two commits were replayed, under the work orders' identity.
Nothing conflicted: both commits applied as they were, the Record's commit included, since `main`
still held `Not yet run.` at this file's foot. Between `1a067eff` and 048a-5d's squash, its fixes
and its rebase onto 209's squash changed `requests.rs` (209's `_meta` on `server/discover` and its
fence of the hook's attempt in both arms of `complete_hook_discovery`), a test in `runner.rs`, the
five `server/discover` intents of `requests-keyed-hook-dispatch`, step 7 of
`requests-sign-in-plain-hook-part` (the reader's R2: its `invocation-ended` fact for `$check2`
ending failed) and 048a-5d's text, which gained only its Record since this order's dispatch refresh
(#410). They left alone `sign_in.rs`, `sign_in_hook_input_kept_nowhere` among its tests,
`engine/CONFORMANCE.md`, `conformance/conventions/requests/index.yaml` and
`requests-sign-in-hook-part`, so none of the places this order changes or reads moved.
`promise_standing.py --write` left `engine/STANDING.md` unchanged, and the pieces tool was not run
again, since the index did not conflict. Following 048a-5d's fixes needed no change in this order's
work or text: the steps that decisions 5 and 6 copy, steps 1 to 8 of `requests-sign-in-hook-part`
and 1 to 4 of `requests-sign-in-plain-hook-part`, are equal, read as JSON, to those on `main`, with
the same `files`; R2's step 7 is not copied, and this order's own step 7 completes `$check2` with no
account, so a failed check's fact does not belong in it; and no step of the two new cases expects a
`server/discover` intent. This order's start check, run on a clean copy of `main`'s tree at
`593707c3` (by `git archive`), printed what the order gives at every item: `:0` for both 207 and 209
(209 merged), `1` (048a-5d merged), `1` (048a-5ef not merged), `1` six times, `3` for `asked_login`,
`2`, the captures and invocations of item 12, `../sign-in-login-plain-hook/manifest.json True`, the
two stated logins three times, `1` and `1` for the two sentences of `engine/CONFORMANCE.md`, and `0`
and `0`; `hook_input` at `sign_in.rs` line 394, `stated_logins` at 790, `HOOK_UNRELAYED_CAUSE` at
132 and `sign_in_hook_input_kept_nowhere` at 1219; and its runs gave the `sign_in` filter 22 passed,
`requests-sign-in-` 13 cases passed and `requests-` 80 cases passed with the 2 expected failures
(`requests-repeat-failed-earlier-succeeded` and `requests-repeat-failed-earlier-unanswered`). So
048a-5d's merge changed nothing a decision reads, and the order's text needed no amendment. The
rebase changed no file but this Record. Warm in the worktree, from `engine/` with 4 test and vector
threads: `cargo fmt --check` (exit 0), clippy with `-D warnings` (exit 0), the `sign_in` tests (24
passed, 2 more than on `main`) and `lifecycle` (9 passed), the seven single-case runs (each `1 cases
passed`), `requests-sign-in-` (15 cases passed) and `requests-` (82 cases passed; the same 2
expected failures) and the shell test `first_start_presents_final_lists` (1 passed); from the root,
`promise_standing.py --check` (Errors: 0; PND-010 at 12, IFC-010 at 25 and IFC-049 at 5, from 10, 24
and 3 on `main`), `vector_delta.py origin/main WORKTREE` with the order's declaration (2 changes, 0
flips, 0 undeclared, 0 errors, exit 0), `check_docs.py` (Errors: 0) and `git diff --check` (clean).
The whole engine suite, `tools/gate.py` against `origin/main` and the reader's read of this rebase
remain the coordinator's before the merge.
