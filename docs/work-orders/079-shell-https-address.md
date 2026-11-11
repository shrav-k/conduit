# Work order 079 (ADDR): the engine's https address, on a second loopback listener

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access, to fetch
crates and for tests that bind a loopback port; use it for nothing else. Never print a whole large
file or fixture: use `head`, `tail`, `grep` or a count. Run one cargo command at a time. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a credential.

## What and why

Your tree is the side branch `s3/m2`, not `main`. It was cut from `main` for the engine work that
lets Agents run on the owner's Windows computer, and it merges to `main` only after the S1 rerun's
pin. Keep each change local, so that the rebase is easy. Line numbers below are from your tree
before your changes; none of the files cited differs from `main`'s.

Today the engine has one address. `Shell::open` accepts only an `http` URL on loopback
(`engine/crates/shell/src/lib.rs:163-180`) and fixes it in the store's `engine-url.json`
(lib.rs:181-195). Every token's audience is that address (`identity.rs:126-138`), verification
checks it (identity.rs:229), root's helper token has it too (lib.rs:301-316), and `serve` binds it
on loopback (`server.rs:721-773`). DL13 and PRV-022 allow an `http` URL on loopback only for a run
on the engine's own runner: "Every other run is given an `https` URL" (PRV-022's observable). At
an `https` address, IFC-042 has the engine accept a token "whose audience is the engine's address
as its canonical `https` URI". A run on another machine, which a later order of this branch starts
through a runtime provider, therefore needs a second address.

This order adds it: an `https` URL that root names when starting `serve`, fixed in the store
beside `engine-url.json`, and a second listener for it that binds loopback only. The listener is
separate so that root's helper token, and the tokens of runs on the engine's own runner, are never
valid where other machines reach the engine: they keep the loopback audience. It admits exactly
the tokens that pass today's checks with the `https` address as their audience, and the engine
issues that audience only to runs under a runtime provider. Admission rests on the audience
alone. IFC-042 has the engine accept such a token "at its `https` address whichever machine a
request comes from" (`spec/interface/implementation.yaml:1187-1189`), and IFC-022 has it establish
"the instance whose ID it names, while that instance exists" (implementation.yaml:596). So add no
other condition, such as one on where the instance is placed.

This is item 2 (ADDR) of the coordinator's plan for running Agents on the Windows computer through
Conduit. The owner let the coordinator adopt that plan's recommendations, except a choice that
changes behavior already ruled or that exposes anything beyond the private network
([ruling 76, later answers](../proposals/ruling-76/README.md#later-answers)). Four of the plan's
choices are held for the owner: the machine provider; which operating-system user runs on that
machine execute as; how TLS is terminated for the engine's `https` address; and whether that
machine's Windows interop and drives are turned off. Nothing in this order, and nothing you write,
may name or assume any of them. The third bears on this order directly: TLS may later be
terminated by a forwarder on the engine's machine in front of the new listener, or by the engine
itself, and this order must work under either. So the listener speaks plain HTTP on loopback; the
order builds no TLS server, adds no TLS dependency, names no particular terminator, and makes
nothing reachable beyond loopback. Until a terminator is chosen and set up, nothing outside this
machine reaches the `https` address.

No runtime provider call is served yet (`engine/CONFORMANCE.md:40`; the executor's
`provider_call` is the trait's default, lib.rs:81-83, which reports the run failed,
lib.rs:642-646), so nothing uses the new address end to end until the later order. This order
changes no promise, vector, record or core code.

Read first: DL13 (`spec/implementation-defined.yaml:97-157`); PRV-018, PRV-022 and PRV-024
(`spec/promises/providers.yaml:509-528`, `612-680` and `720`); IFC-022 and IFC-042
(`spec/interface/implementation.yaml:587` and `1162-1193`); in the shell, `lib.rs:21-55`,
`162-316` and `559-611`, `identity.rs`, `server.rs:498-773`, `main.rs:24-182` and `help.rs`;
rmcp 3.9.0's Host allowlist in the cargo registry, under
`rmcp-3.9.0/src/transport/streamable_http_server/tower.rs`: `allowed_hosts`, its default at line
344, and `host_is_allowed` at lines 1100-1115; and axum 0.8.9's `Router::layer`, in
`axum-0.8.9/src/routing/mod.rs:303-317`, which also wraps the router's fallback.

The decisions. Apply them, and do not reopen them:

1. **A second address, fixed in the store.** `serve` takes `--https-url URL`. The first value given
   is written to `engine-https-url.json` and never changes. It must be canonical, as IFC-042 makes
   the audience, and must not name a loopback host.
2. **A second listener, on loopback only.** `serve` takes `--https-listen ADDRESS`, at `127.0.0.1`
   or `::1` only. It is not stored, and it is required whenever an `https` address is in effect.
   The listener serves `/mcp` only and keeps its own sessions.
3. **Whom it admits.** Exactly the tokens that pass the checks of identity.rs:213-243 with the
   `https` address as audience (IFC-042). The engine issues that audience only to runs under a
   runtime provider (decision 6), and never to root.
4. **Its Host and Origin rules.** Host: the `https` address's host, and its port where the
   address names one. Origin, where present: the `https` address's origin only.
5. **The first listener is unchanged**: its address, Host allowlist, Origin rule and tokens.
6. **Tokens and `CONDUIT_URL` follow the runner.** A run on the engine's own runner gets the
   loopback address and audience, as today. A run under a runtime provider gets the `https`
   address and audience. Helper tokens in `client-tokens` keep the loopback audience. With no
   `https` address, a provider run gets no identity, and its helper token is still written.
7. **Roots for the address's certificate.** `serve` takes `--https-roots FILE`. Its checked
   content is copied into the store, and a provider run's `ca.crt` holds the store's root followed
   by those roots.
8. **No TLS here.** No TLS server, no TLS or certificate-parsing dependency, no terminator named
   in code, help or documents, and no bind beyond loopback.

## What to do

1. **The address** (decision 1).
   - Accept a URL only if: its scheme is `https`; it has no user name, password, query or
     fragment; its path is exactly `/mcp`; its host is neither `localhost` nor a loopback IP
     address (`is_loopback`, applied also to the IPv4 address that an IPv6 address maps, as in
     `[::ffff:127.0.0.1]`); its port is not 0; and it equals its own serialization by the `url`
     crate, so that the host is in lower case and port 443 is left out. Refuse anything else,
     naming the rule it breaks and, for a form that is not canonical, the canonical one.
   - The first time one is given, write it as a JSON string to `engine-https-url.json` in the
     store, with `atomic_write` (mode 0600), before any token with it as audience is issued. After
     that: the same value changes nothing; another value fails, naming both and saying that the
     store fixes the https token audience, as lib.rs:186-188 says for the first address; and no
     value means the fixed one. `Config::for_store` reads the file as it reads `engine-url.json`,
     and `main.rs` fills the address from it as line 127 does for `--url`. A `Shell` opened with
     no address in its `Config` takes the fixed one.
   - Write `engine-https-url.json`, and `engine-https-roots.pem` (step 5), only inside
     `Shell::open`, after `Store::open` (lib.rs:181) has taken the store's writer lease, as
     `engine-url.json` is written (lib.rs:181-195). `main.rs` reads and checks the options and the
     roots file, and passes their values in `Config`; it writes neither file. So a `serve` against
     a store that another shell holds fails with "store already has an active shell" and changes
     neither file.
   - `--https-url`, `--https-listen` and `--https-roots` are accepted by `serve` only, as
     `--as` is by `tools` alone (main.rs:58). Any other command refuses them as unknown arguments.
     Other commands that open the store, `register` among them, take the fixed address and roots
     from the store.
   - No command in this order changes or removes a fixed address.
2. **The listener** (decisions 2, 4 and 5).
   - ADDRESS is `127.0.0.1:PORT` or `[::1]:PORT`. PORT is not 0, and the pair is not the address
     the first listener binds. Refuse any other host, a host name (`localhost` included), and port
     0.
   - `serve` refuses to start when an `https` address is in effect, given or fixed, and
     `--https-listen` is missing; and when `--https-listen` or `--https-roots` is given without
     one. Each message names the missing option and, where there is one, the fixed address. These
     checks, and those of steps 1 and 5, all run before `engine-https-url.json` or
     `engine-https-roots.pem` is written, and neither is written before `Store::open` succeeds
     (step 1).
   - `server::serve` keeps its signature. When the shell has an `https` address, it binds both
     listeners before serving either; a failed bind ends `serve` with the error and the address.
     It serves both until shutdown, and commits recovery once, after both have stopped
     (server.rs:769-771).
   - The second listener serves the route `/mcp` only, through a second `StreamableHttpService`
     built as the first is but from that listener's own `Handler`, and attaches the same
     `authenticate` middleware the same way, with `Router::layer` (server.rs:762-764). No panel,
     metadata or other route is served there. `Router::layer` also wraps axum's fallback, so on
     both listeners a request for any other path passes through `authenticate` first: without a
     valid token it gets 401; with one it gets 404, and authenticate's non-success branch
     (server.rs:633-647) commits one refused-request record for it. That is the first listener's
     behavior today; keep it on both, and do not switch either listener to `route_layer`.
   - The second listener's `Handler` has its own `sessions` map and its own session manager. It
     shares the shell, the executor and the shutdown token. Build each listener's `Handler`, rmcp config and
     router through one function, so that order 074, which builds the session manager from
     `call_timeout` in `serve()` (server.rs:750), applies to both listeners whichever order is
     rebased on the other.
   - Its rmcp `allowed_hosts` is one entry: the `https` address's host, followed by `:PORT` where
     the address names a port. It has no loopback name. The first listener keeps rmcp's default
     (server.rs:752). rmcp matches an entry with no port at any port (`host_is_allowed`,
     tower.rs:1100-1115), and a client leaves port 443 out of `Host`, so for an address that
     leaves out port 443 the host is accepted at any port. Do not add an entry of `host:443`,
     which such a client's `Host` would never match.
   - A present `Origin` is accepted only when it equals the `https` address's origin exactly, as
     server.rs:528-545 does for the first address (DL12, IFC-036); any other gets 403.
3. **Admission** (decision 3). `authenticate` learns from its `Handler` which address it serves,
   and uses that address for both the audience and the Origin check. The first listener keeps
   calling `Shell::verify_token` (server.rs:509), which does not change (lib.rs:278-282). The
   second calls a new `Shell` method that applies the same checks, identity.rs:213-243, with the
   `https` address as the audience, and nothing more.

   Any other token establishes no instance there, and the request is answered as an invalid
   token is today: 401 with `WWW-Authenticate`, and a readable `tools/call` recorded refused with
   identity none (server.rs:546-570). Root's helper token and the tokens of runs on the engine's
   own runner keep the loopback audience (step 4), so the audience check alone refuses them on the
   second listener, as IFC-042 has it: "A token whose audience is another receiver ... establishes
   no instance" (implementation.yaml:1184-1186). No code path in this order issues root a token
   with the `https` audience. Do this in the shell: the core's `Identity`, its access decisions
   and its records do not change.
4. **Tokens and `CONDUIT_URL`** (decision 6). In `drive` (lib.rs:569-575):
   - a `run.v1` intent of kind `builtin-runner` is materialized as today, with the loopback
     address (DL13: an `http` URL on loopback only for a run on the engine's own runner), and its
     `ca.crt` holds the store's root alone;
   - a `run.v1` intent of kind `provider-call` is materialized with the `https` address: the
     token's audience and the `CONDUIT_URL` variable are that address exactly as fixed (PRV-022:
     the value "keeps the engine's address whole"), and `ca.crt` holds the store's root followed
     by the roots of step 5, each PEM block on its own lines;
   - with no `https` address in effect, a `provider-call` run is given no identity (`None`), and
     no identity directory is made for it, since DL13 gives such a run no `http` URL. Its helper
     token `client-tokens/<instance>` is still written, with the loopback audience, as
     lib.rs:292-297 writes it today, so that `conduit tools --as` still finds it for that instance
     (server.rs:809-812). The executor still reports the run failed; the later order that serves
     provider calls says why.

   Keep `materialize_identity(descriptor)` as the engine's own runner's form, with its callers
   unchanged (`tests.rs:808`, `server_tests.rs:127`, `tests/serve.rs:199`), and add a second
   public method for provider runs. Both write the helper token in `client-tokens/<instance>`, as
   lib.rs:292-297 does today, and it keeps the loopback audience for every run: it is the store
   owner's helper for the first listener (CONFORMANCE.md:118-121). No identity value enters
   state, records or output (PRV-024).
5. **Roots** (decision 7). `--https-roots FILE` names the root certificates from which the
   certificate at the `https` address is issued, where they are not the store's own (DL13: "The
   roots from which the certificate at the engine's address is issued: the mechanism's own, or
   others, such as those of a public certification authority"; PRV-022's observable,
   providers.yaml:644-648: `ca.crt` holds them).
   - Accept a file of one or more PEM blocks labelled `CERTIFICATE` whose base64 decodes, with
     only whitespace around them. Refuse an empty file; a block with any other label, naming the
     label (a private key above all); and anything else. Add no dependency for this check.
   - Copy the checked content into the store as `engine-https-roots.pem` (`atomic_write`, 0600),
     only after `Store::open` has succeeded (step 1). Given again, the copy is replaced; only runs
     materialized afterwards see the change.
   - The option serves either way of terminating TLS: a certificate issued by a public authority
     needs that authority's roots named here, and one issued from the store's own root needs none.
     With no copy, a provider run's `ca.crt` holds the store's root alone, and so checks only a
     certificate issued from that root.
6. **Help and README.** In `help.rs`:
   - the `serve` row's arguments (line 39, now empty): `[--https-url URL] [--https-listen ADDRESS]
     [--https-roots FILE]`;
   - the `serve` description (line 61): that with an `https` address it also serves `/mcp` for
     that address on a second loopback listener;
   - the options text (line 84): the three options in a list of their own, headed as options of
     `serve` alone, not under "Runner/store options for init, register and serve"; and that the
     `https` address is fixed in the store at first use.

   In `engine/README.md`, the Config paragraph (229-233) and the identity paragraph (128-132): the
   options, and that the address a run is given follows its runner.
7. **`engine/CONFORMANCE.md`.**
   - The row "DL13: loopback boundary" (line 34): its first sentence says that the first address
     is an `http` address on loopback only, and points to the new row.
   - A new row after it, "DL13: the https address", stating: the option, the file and the
     canonical form; the listener, on loopback only, serving `/mcp` only, with its own sessions;
     its Host allowlist, the address's host and, where the address names one, its port, so
     whatever forwards to it must keep the `Host` header the client sent; its Origin rule; that it
     admits exactly the tokens that pass the first listener's checks with that address as
     audience, which the engine issues only to runs under a runtime provider, so that root's
     helper token and the tokens of runs on the engine's own runner are refused there; that runs
     under a runtime provider are given the `https` address and audience and runs on the engine's
     own runner the loopback ones; what `ca.crt` holds for each, including that with no
     `--https-roots` a provider run's `ca.crt` checks only a certificate issued from the store's
     root; that helper tokens keep the loopback audience; and, in these words: "How TLS is
     terminated for the https address is not yet chosen. The engine serves no TLS, and nothing in
     this build reaches the address from beyond loopback."
   - The DL13 row of "Choices still required" (line 59): add TLS termination for the `https`
     address, and that the RFC 9728 protected resource metadata that IFC-042 has the server at an
     `https` address publish is not published. Drop "loopback-only authority", which no longer
     holds.
   - The section "Consumer server (order 022)" (lines 73-79): one sentence pointing to the new row
     for the second address and its listener.
8. **Comments.** Each new check carries a comment citing its promise or DL item: DL13, PRV-022,
   IFC-042, or DL12 and IFC-036 for Origin.
9. **Tests.** Reach the second listener with raw requests that set the `Host` header, as
   `server_tests.rs:599-626` sends raw requests. No test needs DNS, TLS, a second node or
   anything beyond loopback; an Agent placed on `local` serves every case below. Add each test as
   a new function, and leave existing test functions as they are (Scope); the shell-level and
   server tests may go in a new test module. [Order 068](068-test-gate-and-ci.md), in flight on
   `main`, edits the shell's test files, `src/tests.rs`, `tests/serve.rs` and `tests/cli.rs`
   among them, and adds nextest groups for the crash and timing-sensitive tests (its step 1).
   Separate functions let the admin place yours in those groups at the rebase.
   - **Right listener, wrong listener:** for an Agent, its provider-run token is accepted on the
     `https` listener (initialize, then `tools/list`) and refused with 401 on the first; its
     own-runner token is accepted on the first and refused with 401 on the `https` listener.
   - **Refused on the `https` listener, each with 401:** root's helper token, read from
     `client-tokens/<root>`; an Agent's own-runner token; and a token with another `https`
     audience, minted through the authority as `tests.rs:943-952` mints tokens. A readable
     `tools/call` among them is recorded refused with identity none.
   - **Root's audience:** with an `https` address in effect, the token in `client-tokens/<root>`
     has the loopback address as its only audience.
   - **Host and Origin:** with a valid token, the `https` listener accepts the configured host and
     port, and answers 403 to `127.0.0.1:<port>`, to `localhost` and to another name; the first
     listener answers 403 to the `https` address's host. With an address that leaves out port
     443, the `https` listener accepts a `Host` of the bare name. On the `https` listener, the
     address's origin is accepted and another origin gets 403.
   - **Only `/mcp`:** on the `https` listener, `/` and `/.well-known/oauth-protected-resource/mcp`
     answer 401 without a token; with a valid token they answer 404, and each commits one
     refused-request record (step 2).
   - **The address:** the first open writes `engine-https-url.json`; the same value again changes
     nothing; another value fails, naming both; none takes the fixed one; each refused URL of
     step 1 (`http`, each loopback host, another path, a query, a fragment, a user name, port 0,
     an upper-case host, `:443`) writes nothing; and a `Shell::open` given an address and roots
     while another `Shell` holds the store fails with "store already has an active shell" and
     leaves both files as they were.
   - **Materialization:** an own-runner run gets the loopback URL and audience, and `ca.crt` with
     the store's root alone; a provider run gets the `https` URL and audience, and `ca.crt` with
     the store's root followed by the named roots; the helper token keeps the loopback audience in
     both; and with no `https` address, a provider run's identity is `None` and its helper token
     is still written, with the loopback audience. If driving a provider run through the core is
     impractical, test the function that `drive` uses to choose.
   - **Options, through the binary,** in a new function beside `tests/cli.rs:191`'s
     fixed-audience test: `serve` with an `https` address and no `--https-listen` fails, naming the
     option; `--https-listen` with a non-loopback host, a host name, port 0 or the first
     listener's address fails; `--https-roots` with a private key, an empty file or stray text
     fails; `--https-url` on `init` fails as an unknown argument; and none of them writes
     `engine-https-url.json` or `engine-https-roots.pem`.

## Scope

Change:

- `engine/crates/shell/`: `src/` and `tests/`;
- `engine/CONFORMANCE.md`;
- `engine/README.md`.

Do not change:

- `engine/crates/core/`, `engine/crates/loader/`, `engine/crates/conformance/`, `conformance/`,
  `spec/` or `packages/`;
- `engine/Cargo.lock` or any `Cargo.toml`: add no dependency, no TLS crate and no PEM or X.509
  parser, and keep reqwest's features (`engine/crates/shell/Cargo.toml:31`);
- any existing test function, unless a signature that this order changes leaves it no other way
  to compile; report each such change;
- any record, log or guide under `docs/`.

This order changes no promise and no vector, so it has no declaration file; if you find that a
vector must change, stop and report it. Bind nothing but `127.0.0.1` and `::1`. Order 074 also
edits `serve()` in `server.rs`, and order 068 edits the shell's test files; keep your change in
`serve()` small and local (step 2), and your tests in new functions (step 9). Report anything else
that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_roadmap.py`
- `git diff --check`

From the repository root, these give the result stated:

- `python3 tools/vector_delta.py origin/s3/m2` ends with the line
  `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.`
- These print nothing:
  - `git status --porcelain -- spec conformance packages docs engine/Cargo.lock engine/crates/core engine/crates/loader engine/crates/conformance engine/crates/shell/Cargo.toml`
  - `grep -rniI --exclude-dir=target tailscale engine/`
  - `git diff -U0 -- engine/CONFORMANCE.md engine/README.md engine/crates/shell/src/help.rs | grep '^+' | grep -nE '/(Users|home)/'`
- `grep -c 'DL13: the https address' engine/CONFORMANCE.md` prints a number above 0, and
  `grep -c 'How TLS is terminated for the https address is not yet chosen' engine/CONFORMANCE.md`
  prints a number above 0.

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release -p conduit-shell`, with the new tests among those that pass, and then the
  full `cargo test --release` once.

Tests that bind loopback ports may fail in your sandbox; if they do, say so, and the admin reruns
them. If timing-sensitive shell tests fail under load, rerun them with `-- --test-threads=2` and
report both runs (finding 4-18).

Report:

- in `decisions`, each of the eight decisions, as applied;
- under `findings`, anything that needs changing outside this order's scope;
- under `own_choices`, the names of the new options' `Config` fields and of the new methods, the
  function that builds each listener, the text of each new error, and every other choice you made
  yourself;
- each new test, by file and function name, marking each that binds a loopback port or waits on a
  clock;
- the totals passed and failed, and each check's last line.

Do not stage anything.
