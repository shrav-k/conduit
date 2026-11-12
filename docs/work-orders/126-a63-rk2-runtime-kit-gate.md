# Work order 126 (A63-RK2): the runtime-provider kit's gate

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order has network access, to fetch crates and
for tests that bind a loopback port; use it for nothing else. Run one cargo command at a time, and
never print a whole large file or log: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never print a
token, a private key or a certificate's key, even one that a test made.

## What and why

Orders 122 and 124 built `packages/runtime-provider-kit/`: the MCP server for
`runtime-provider.v1` and the interfaces a runtime implements, among them `Gate` and `GateHandle`
(`src/backend.rs`, near lines 154 to 173), with a fake gate in the tests. This order is A63-RK2,
the real gate that the local-process provider, the Docker provider and the admin's SSH runtime will
share ([PROVIDERS.md](../implementation/PROVIDERS.md), its paragraph "A63-RK1 / RK2 — What the
runtimes share" and row A63-RK2). It stands between the engine and a run's server: the engine
reaches the run only through it, over TLS with the run's certificate, and only with the engine's
proof.

Read first:

- PRV-022, PRV-025 and PRV-028 in `spec/promises/providers.yaml`, and the observable text under
  PRV-025 (near lines 767 to 830) on the gate's liveness check; the description of `run.v1` in
  `spec/core/operations.yaml` (near lines 864 to 908) on `identity`, `tls.crt`, `tls.key` and the
  `url`; IFC-017 in `spec/interface/implementation.yaml`; DL13 in
  `spec/implementation-defined.yaml`; and OPS-027 in `spec/conventions/operations.yaml`, on how a
  request is cancelled over stdio and over Streamable HTTP;
- order 080 (`docs/work-orders/080-shell-runtime-provider-calls.md`), the client side of this
  gate: its step 2 on the run's certificate and the engine's token toward a run's gate, and its
  step on the bridge (from about line 236): what the engine checks before it sends anything, and
  how it carries the MCP stream (POST bodies, server-sent events, `Mcp-Session-Id`, the GET
  stream, DELETE) with `Authorization: Bearer`. The gate must accept exactly what that bridge
  sends. The bridge is not built yet (its order runs on the admin's side branch): follow its
  order's text;
- the engine's token and its check, as reference, not as a dependency:
  `engine/crates/shell/src/identity.rs`, near lines 120 to 140 (an ES256 JWT with `kid` "store")
  and near lines 205 to 243 (`verify`);
- the kit: `packages/runtime-provider-kit/README.md`, `src/backend.rs` and `src/channel.rs`.

## The decisions it applies

1. **Where it lives, and its crates.** In the kit, as a module `gate` with a public type that
   implements `Gate`, its configuration type, and its own tests. The kit stays standalone: its own
   `Cargo.lock`, no crate of this repository. Use:
   - `rustls` 0.23 with default features off and `ring` and `std` (TLS 1.3, which order 080's
     client uses); `tokio-rustls` 0.26 with default features off and `ring`; ALPN `http/1.1`
     only; PEM read through `rustls::pki_types::pem::PemObject`, not `rustls-pemfile`;
   - `hyper` 1 with `server` and `http1`, and `hyper-util` with `tokio`; do not turn on
     `with_half_close`; add `net` to the kit's `tokio` features;
   - `socket2` 0.6 with `all`, for keepalive's interval and count and for reading them back;
   - `ring` for ES256 verification (`ECDSA_P256_SHA256_FIXED`) and for random session IDs, and
     `base64`;
   - for tests only: `rcgen` 0.14, `p256`, and `x509-parser` 0.18 with default features off.

   No crate may bring `aws-lc-rs`, OpenSSL or native-tls: the kit's `Cargo.lock` holds none of
   them (decision 10's grep). Name every new crate and its version in your report.
2. **One listener per run** (PROVIDERS.md). `open` binds a fresh TCP listener for that run alone,
   on the bind address the configuration gives (loopback by default) and port 0, and the handle's
   `url` is `https://<host>:<port>/mcp`, where the configuration's advertised host is separate
   from the bind address. A bind address of `0.0.0.0` or `::` is refused; an IPv6 host is
   bracketed in the URL. One listener per run, because the engine's bridge checks no host and an
   IP URL carries no SNI, so one listener cannot choose among several runs' certificates.
3. **TLS with the run's certificate** (PRV-028, IFC-017). The listener serves `tls.crt` with
   `tls.key` from the run's `identity`, kept in memory only. A run whose `identity` lacks them
   cannot be gated: `open` fails, with no listener left behind. The gate asks for no client
   certificate: DL13 lets a gate check either proof, and the token is required anyway.
4. **The engine's proof, before any byte reaches the run** (PRV-025, DL13). Every request, POST,
   GET and DELETE alike, carries `Authorization: Bearer <token>`. The gate admits a request only
   where the token:
   - is a JWT whose header's `alg` is exactly `ES256`, has no `crit`, and whose `kid` names a key
     of `jwks.json` in the run's `identity`, against which its signature verifies;
   - has a `sub` byte-equal to the content of `engine-id`, trimmed of surrounding whitespace;
   - has an `aud` that is either a string equal to the instance's SPIFFE ID or an array with
     exactly one element equal to it: the engine's tokens carry `aud` as an array
     (`identity.rs`, near line 132). The instance's ID is, as DL13 says, the `sub` of the run's
     own `token` in `identity`; `open` fails where the URI of `tls.crt` differs from it;
   - has an integer `exp` later than now.

   Anything else answers 401 with an empty body and `WWW-Authenticate: Bearer`, and reaches
   neither the run nor its channel. The gate checks no `Host`, `Origin` or `MCP-Protocol-Version`
   header, and no `Accept` on a GET.
5. **Sessions.** One live session per run. An `initialize` opens one, with a random
   `Mcp-Session-Id` that the reply carries, and is forwarded to the run. A later `initialize`
   replaces the live session: that session's open requests are cancelled (decision 7), its GET
   stream closes, its ID answers 404 from then on, and the new `initialize` is forwarded to the
   run. Report as a finding that a stdio server then sees a second `initialize`; order 080's
   bridge opens a new session when it restarts. After `initialize`, a request without the session
   ID answers 400, and one with an unknown or ended ID answers 404, each with an empty body. A
   DELETE ends the session, never the channel.
6. **The byte bridge.** Behind the check, the gate carries the MCP stream between Streamable HTTP
   and the run's channel byte for byte, with no MCP types:
   - each POST body is written to the channel as one line. A POST of a notification or of a
     response answers 202 with no body. A POST of a request is answered with the run's reply to
     it: as server-sent events whenever its `Accept` allows `text/event-stream`, and as JSON only
     when it does not;
   - a line the run writes is a response only where it has no `method`; a response is matched to
     the open request with the same JSON-RPC `id`. The run's own requests and notifications, and
     nothing else, go to the GET stream;
   - lines for the GET stream are buffered, up to a limit the configuration sets, while no GET
     stream is open; a new GET replaces the session's old stream;
   - a refusal carries no JSON-RPC body, since order 080's bridge passes any JSON-RPC body on.

   Keep to what order 080's bridge sends and what an MCP server needs, and report anything the
   protocol needs that this leaves out as a finding.
7. **Cancellation behind stdio** (OPS-027). Whenever a request's response stream ends before its
   reply arrives (a cut connection, a client that closes, a DELETE, a replaced session, or
   `close`), the gate writes `notifications/cancelled` with that request's ID to the channel, and
   discards the run's reply if it comes later: never to the GET stream, where MCP allows no
   response.
8. **The silent-cut bound** (PRV-025). The gate checks each connection's liveness with TCP
   keepalive, which it sets on every accepted socket from its configuration (idle time, interval
   and count), since the engine's client has no HTTP/2. On Linux it also sets
   `TCP_USER_TIMEOUT` to the bound, since keepalive pauses while data waits to be acknowledged;
   report the absence of an equivalent on macOS as a finding. The configuration type, not the
   handle, reports the bound (idle time plus interval times count) as `silent_cut_bound()`, since
   the kit holds handles as `Arc<dyn GateHandle>`, so that a runtime's contract can state it. A
   connection that fails the check ends, and decision 7 applies to its requests.
9. **The handle.** `close` closes the listener, every open connection and the channel, and
   returns only once they are closed; it may be called twice. `ended` resolves when the channel
   ends or the gate closes. `open` honors its cancellation token: cancelled, it leaves no listener.
   Nothing the gate logs, answers or keeps outside memory holds `token`, `tls.key` or any
   identity value, and its URL holds none. The gate trait does not change.
10. **What it does not do:** no OS principals (the runtime runs its runs as another user than the
    gate's, which A63-RP2 states), no runtime, and no change to the server or the backend traits
    beyond what wiring the gate needs (say what, if anything). Update the kit's README where it
    says there is no TLS or production gate (near lines 49, 72 to 75 and 158 to 162), to describe
    this gate.
11. **Tests**, with a test authority that `rcgen` makes at test time (a root, and a run
    certificate naming the instance's SPIFFE URI), an ES256 token made with `p256` whose `aud` is
    an array, as the engine makes it, and a TLS client in the tests that checks the chain and the
    URI as order 080's bridge does:
    - the handshake proves the run's certificate, and a client that expects another instance's
      URI refuses it; `open` fails where `tls.crt` names another instance than the run's token;
    - no token, an `aud` for another instance, another `sub`, a past `exp`, a bad signature,
      another `alg`, a `crit` header, an unknown `kid`: each 401 with an empty body, and the fake
      run receives no byte; and an `aud` given as a one-element array and as a string both pass;
    - an initialize; a request answered as server-sent events and one answered as JSON; a
      notification posted (202); a notification from the run on the GET stream, including one
      buffered before the GET opened; a run's request whose ID equals one of the engine's, which
      goes to the GET stream; the session ID required after initialization (400), an ended one
      (404); a second initialize replacing the session; and a DELETE that leaves the channel open;
    - a connection reset while a request is in flight: `notifications/cancelled` for that ID
      reaches the channel, and the run's late reply is discarded;
    - the keepalive idle time, interval and count read back from an accepted socket equal the
      configuration, and `silent_cut_bound()` reports their bound. A real silent cut needs packet
      dropping, which needs root: a Linux-only test of it may be added with `#[ignore]`;
    - two runs: two listeners on two ports, each serving its own certificate;
    - `close` twice, `ended` after the channel ends, and a cancelled `open` leaving no listener;
    - no token, key or identity value in any log line or answer, by synthetic markers.

## Scope

Change only `packages/runtime-provider-kit/`, its README included.

## Done when

In `packages/runtime-provider-kit/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`, run twice; report both runs' totals and any test that differed

From the repository root:

- `python3 tools/check_docs.py`
- `python3 tools/set_names.py --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
D=packages/runtime-provider-kit
grep -n -E "^name = \"(aws-lc-rs|aws-lc-sys|openssl|openssl-sys|native-tls)\"" $D/Cargo.lock
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]|/tmp/claud[e]|\.codex/auth\.json" $D --exclude-dir=target
grep -rn -E "[[:space:]]+$" $D --exclude-dir=target
git status --porcelain --ignored $D | grep target/ | grep -v '^!!'
```

`target/` stays ignored and unstaged. Report each protocol behavior decision 6 leaves out, and
each choice of the keepalive's defaults and of the buffer's limit, as one of your own choices. Do
not stage anything.

## Record

Run on 2026-11-11 on the Mac, from base 8103ad3: the worker and a fresh verifier (386 s), then a
second worker round (275 s) and a fresh verifier (348 s), which found nothing to change. Round
one found no must and two shoulds, both in the tests, both fixed in round two: the keepalive test
now reads back the settings of a socket the gate itself accepted; and the leakage test sends a
valid engine token through an admitted request and checks that exact token against the logs,
headers and bodies. The kit now has the gate in `src/gate`: a TLS listener per run with
`rustls` and `ring`, the engine's proof checked on every request, one live session per run, the
byte bridge with `notifications/cancelled` behind stdio, and TCP keepalive for PRV-025's bound.
The lane coordinator ran the crate's tests on the result, 61 passed, and found no `aws-lc-rs`,
OpenSSL or native-tls in its lockfile.

The worker's findings: a replaced session forwards another `initialize` to the persistent stdio
server (an ambiguity for order 080 and for the runtimes' contracts); and macOS has no
`TCP_USER_TIMEOUT`, so with unacknowledged data outstanding a silent cut can take longer than the
keepalive bound there (a gap a runtime's contract must state on macOS).
