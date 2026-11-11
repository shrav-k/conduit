# Work order 080 (DSP): the shell calls runtime providers and reaches their runs over TLS

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has network access, to fetch
crates and for tests that bind a loopback port; use it for nothing else. Never print a whole large
file: use `head`, `tail`, `grep` or a count. Never run `env`, `printenv`, `set` or `export -p`,
never print an environment variable's value, and never print a token or a private key, even one
that a test made.

## What and why

Your tree is order 079's branch, not `main`. Both orders are built on the side branch `s3/m2`,
which holds the engine work for running Agents on a second machine through Conduit, under the plan
that the owner let the coordinator adopt on 2026-11-11
([ruling 76, later answers](../proposals/ruling-76/README.md#later-answers)). Nothing on `s3/m2`
merges to `main` before the S1 rerun's pin. Order 079 (ADDR) gives the engine a second address, an
`https` URL, and makes `CONDUIT_URL` and each run's token follow the runner: the loopback address
for the engine's own runner, the `https` one for a run that a runtime provider makes. Both orders
edit `identity.rs`, so this one is stacked on 079. Read order 079
(`docs/work-orders/079-shell-https-address.md`) and its change (`git diff origin/s3/m2`) before
you start. Line numbers below are those of `origin/s3/m2` at 342c5ba; order 079 may have moved
some.

Today the core emits a provider call for `run.v1`, `stop.v1` and `inspect-run.v1` of an instance
that names a runtime provider, and records it (`engine/crates/core/src/lifecycle.rs`, `emit`,
703-747), but the shell serves none. `Executor::provider_call` is the trait default, `Missing`
(`engine/crates/shell/src/lib.rs`:81-83), and neither `impl Executor for Runner` nor
`impl Executor for Arc<Inner>` (`runner.rs`:236-348) overrides it, so such a setup reports its run
failed and its inspection unknown (`lib.rs`:629-655). A run is only ever a local stdio child:
`Wire` is tied to the child's stdin and stdout (`runner.rs`:940-999). No `tls.crt` or `tls.key` is
made (`identity.rs`:139-204), and reqwest is built without TLS (`shell/Cargo.toml`:31). The DL16
row "executor coverage" of `engine/CONFORMANCE.md` (line 40) says that provider calls are
unserved.

This order serves them. The shell calls a runtime provider through the provider's own run, which
the engine's own runner holds (PRV-002), and asks it `server/discover` first (PRV-005, IFC-037).
For each run that a provider makes, it issues a certificate in the form of an X509-SVID from the
store's root. It reaches that run, at the `url` the provider returns, through a bridge,
`conduit __http-bridge`, which carries the MCP stream's bytes between stdio and Streamable HTTP and
checks, before it sends anything, that what answers proves by TLS that it serves the instance
(IFC-017, PRV-028). The run table, `Wire`, process groups and dispatch stay as they are: to them a
bridged run is one more local process. The core, the specification and every vector stay as they
are.

Read first: PRV-002, PRV-005, PRV-007, PRV-008, PRV-009, PRV-022, PRV-024, PRV-025, PRV-027 and
PRV-028 in `spec/promises/providers.yaml`; IFC-008, IFC-009, IFC-017, IFC-022 and IFC-037 in
`spec/interface/implementation.yaml`; OPS-007 in `spec/conventions/operations.yaml`; `run.v1`,
`stop.v1` and `inspect-run.v1` in `spec/core/operations.yaml`, from line 813; and DL13 and DL16 in
`spec/implementation-defined.yaml`.

Four choices of that plan are still before the owner: which machine provider enrolls another
machine, which operating-system user runs execute as there, how TLS is terminated at the engine's
`https` address, and how the other machine is locked down. Nothing in this order may depend on any
of them. It names no machine provider, network, remote access method or operating-system user; it
is tested against a scripted local provider and a loopback TLS gate only.

## What to do

1. **TLS for the shell** (`engine/crates/shell/Cargo.toml`, `lib.rs`, `main.rs`).
   - reqwest gains the feature `rustls-no-provider`, not `rustls`, which in reqwest 0.13 brings
     aws-lc-rs.
   - Add `rustls` 0.23 with default features off and the features `ring` and `std`. The bridge of
     step 5 builds its own `rustls::ClientConfig` and hands it to reqwest whole, through
     `ClientBuilder::tls_backend_preconfigured`; it must be the rustls that reqwest itself uses,
     or reqwest does not recognize the configuration.
   - **The process's crypto provider.** Under `rustls-no-provider`, every reqwest client built
     without a preconfigured TLS configuration, `reqwest::Client::new()`, `reqwest::get` and
     `ClientBuilder::build()` among them, takes rustls's process-default provider, and panics
     with "No rustls crypto provider is configured" where none is installed
     (`reqwest-0.13.5/src/async_impl/client.rs`, lines 721-723 and 2498-2507, in the cargo
     registry). rustls's `CryptoProvider::get_default` installs nothing from crate features. So
     add to the shell's library one public function, `install_crypto_provider`, that installs
     `rustls::crypto::ring::default_provider()` as the process default and does nothing on a
     later call or where a default is already installed.
     - Call it first in `main` (`main.rs`:437-447), before the test for `__run-guard`, so that it
       precedes `__http-bridge` and `run()`. That covers the client of `conduit tools`
       (`server::client`, `server.rs`:776-883, which builds it at line 816); `server.rs` does
       not change.
     - Call it also in every test process that builds such a client, before it builds one:
       `src/server_tests.rs`, which builds clients directly (lines 35, 45, 117, 251, 603, 672,
       786, 999, 1186 and 1421) and through `server::client` (lines 414, 453, 462, 490, 1278,
       1299 and 1352), and `tests/serve.rs` (lines 51 and 241). Put the call in the helpers that
       those tests share where you can, and otherwise at the start of a test; change those files
       in nothing else. A test that misses it panics with the message above, so the full test
       run of Done when finds any you missed. Report each place you call it.
   - The chain is checked by rustls-webpki, the library rustls uses, never by hand-written path
     validation (step 5 says how). x509-parser 0.18, today a dev-dependency, may become a
     dependency to read the certificate's URI. rcgen 0.14 stays; enable its `x509-parser` feature
     only if its `Issuer` needs it to load the stored root.
   - A TLS crate that only the tests use, such as tokio-rustls 0.26 for the test's gate, goes
     under `[dev-dependencies]` with default features off and its `ring` feature where it has
     one: the defaults of tokio-rustls 0.26 and hyper-rustls 0.27, and axum-server's
     `tls-rustls`, bring aws-lc-rs, and `Cargo.lock` lists dev-dependencies too.
   - `engine/Cargo.lock` gains no aws-lc-rs, aws-lc-sys, OpenSSL or native-tls crate. It does gain
     rustls-platform-verifier, which `rustls-no-provider` always brings, and the crates it uses
     on each platform, all of which the lockfile lists: rustls-native-certs and openssl-probe,
     which looks for certificate files and links no OpenSSL, security-framework, jni,
     webpki-root-certs and others. Name every new crate and its version in your report.

2. **The run's certificate and the engine's proof** (`identity.rs`, `lib.rs`).
   - **`tls.crt` and `tls.key`** for each provider call to `run.v1`: a fresh P-256 key, and a
     certificate issued from the store's root (`authority.json`, `identity.rs`:36-115) with rcgen's
     `Issuer`, so that its issuer name and key identifier match the root's. It is an X509-SVID as
     DL13 states one: exactly one subject alternative name, the URI
     `spiffe://<trust-domain>/conduit/instance/<id>`, and no other name; not a CA; key usage
     digitalSignature; extended key usage serverAuth and clientAuth, since it serves as both; and a
     `notAfter` no earlier than the `exp` of the run's token, so that it does not lapse while the
     instance exists. Both files are PEM. Take their names from `certificateFile` and `keyFile` in
     `spec/names.yaml`, which the shell embeds at build time and reads itself, in the way that the
     core's `run_variables` embeds and reads its three names (`core/src/lib.rs`:20-45, with
     `include_bytes!` and `conduit_loader::read_bytes`). Do not change the core or add to
     `run_variables`, which gives only the three variables. Check that the two names are the
     optional files of `run.v1`'s `identity` in the held contract, read as `identity.rs`:153-173
     reads its four required files; write neither name by hand.
   - Give them to a provider's run only, never to a run on the engine's own runner (PRV-007,
     PRV-022; DL13 gives them "for every run whose server stands behind a gate"). Write `tls.key`
     to no file on the engine's machine: it goes into the call and nowhere else.
   - **`RunIdentity`** gains the files' content, so that the provider call need not read them back
     from disk. It keeps no `Debug` or `Serialize` implementation (`identity.rs`:49-53).
   - **The engine's token toward a run's gate.** Its `sub` is the engine's ID,
     `spiffe://<trust-domain>/conduit/engine`, as `engine-id` holds it; its `aud` is one value, the
     instance's SPIFFE ID exactly as the `sub` of the run's own `token` names it (DL13); its
     header, signing key, `iat`, `exp` and `jti` are made as `token` makes them
     (`identity.rs`:126-138). `verify` (`identity.rs`:205-243) must refuse it at every engine
     address, order 079's `https` one included.
   - **The engine's certificate**, for a gate that asks for one in the TLS handshake: DL13 has the
     engine present both its certificate and its token to a run's gate. It is an X509-SVID whose one
     URI is the engine's ID, issued as above for each bridge, with its key only in that bridge's
     directory (step 5).

3. **One call to a provider, for any operation** (`runner.rs`). Write it once, so that orders
   049a1 and 051c can add the machine provider's calls on top of it.
   - **The provider's run** is the run table's entry under the intent's `provider`, which the
     engine's own runner started when the provider was set up (PRV-002). Where there is none, or
     it has ended, send nothing, and answer as the missing executor does (`lib.rs`:629-655).
     Running the provider again before the call, as PRV-026 asks after a restart, is not built
     here; `serve` sets up every ready Resource again when it starts (`main.rs`:334-414), the
     provider among them.
   - **Discovery first.** Before the first provider call on each run of a provider, send it
     `server/discover` through `call` (`runner.rs`:1068-1114), and keep with that run whether the
     result lists `<metaPrefix>implementation-interface` with `revision` `v0` under
     `capabilities.extensions`, the form that the engine's own server gives (`server.rs`:269). A
     later run of the provider is asked again (IFC-037). Where the result does not list it, or
     cannot be read, send no call (PRV-005).
   - **The request** is `tools/call`. Its `name` is the operation's name after `#`, as the core
     names a tool (`core/src/requests.rs`:565-581). Its `arguments` are the intent's `inputs`,
     with `run.v1`'s `identity` replaced as step 4 says. Its `_meta` holds exactly the keys that
     PRV-005 gives a provider call: `instance`, which is the intent's `provider`; `operation`;
     `config` and `revision`, the provider's own, from the intent; `invocation` and `caller` where
     the intent holds them; and no `filters`, `conduit` or `idempotencyKey`. It also declares
     Conduit's extension at `v0` in `io.modelcontextprotocol/clientCapabilities`, in the form that
     `server.rs`:606-607 reads (IFC-037). Take the prefix from the context's `meta_prefix`, as the
     core does.
   - **Sending.** Through `call`, under a key that no request's attempt can have, and within the
     shell's `call_timeout`, as `dispatch_later` bounds a dispatch (`runner.rs`:1022-1045).
   - **Reading the result**, as IFC-009 and OPS-007 read one (PRV-005). A result states
     succeeded, failed or unknown in `<metaPrefix>outcome`. A result that holds no outcome, or both
     an outcome and a reason, or whose `isError` disagrees with its outcome, is unknown; so is a
     succeeded result whose outputs fail the operation's `outputSchema`, checked with
     `conduit_core::schemas::standalone_valid` against the held core contract, read as
     `identity.rs`:153-160 reads `run.v1`'s input schema. No result before the deadline, or a run
     that closes, gives no result. The error -32021 whose `data.requiredCapabilities` has the shape
     that the core's `capability_error` accepts (`core/src/requests.rs`:28-42) says that the
     provider performed nothing, both where it names Conduit's extension (IFC-008) and where it
     names only other capabilities, which the call did not declare (IFC-047): a provider call
     declares Conduit's extension and nothing else. The core reads both so at
     `requests.rs`:1292-1309; write the same reading in the shell, and leave the core's private
     function as it is. A result that gives a reason in place of an outcome also says that the
     provider performed nothing. Any other error is no result (OPS-007).
   - For an operation other than the three of step 4, `provider_call` answers `Missing`, as today:
     the core emits none yet, and order 049a1 adds the machine provider's intents and reports.
     If order 049a1 is already in your tree when you start (its MachineReport and its machine
     provider intents exist), send its three machine calls (enroll, accept and remove) through this
     step's function, map their replies as 049a1's step 10 does, and keep 049a1's sentence in the
     DL16 row. Say in your report which case applied.

4. **`provider_call` for `run.v1`, `stop.v1` and `inspect-run.v1`.** Override it in both
   `impl Executor for Runner` and `impl Executor for Arc<Inner>`; `ServingExecutor` already passes
   it on (`server.rs`:177-178). Each call is deferred and reports through the shell with the
   completion executor, as `builtin_runner` reports a run (`runner.rs`:283-310), in the forms
   `{kind: run, outcome, url}`, `{kind: stop, outcome}` and `{kind: inspect-run, state}`.
   - **`run.v1`'s inputs.** `identity` becomes `{environment, files}`. In `environment` are the two
     variables of the materialized identity, so `CONDUIT_URL` is the `https` address that order 079
     gives a provider's run; `CONDUIT_IDENTITY_DIR` is the materialized directory on the engine's
     machine, which the provider may give as the run sees it (`run.v1`). In `files` are the content
     of `ca.crt`, `jwks.json`, `engine-id` and `token` as materialized, and `tls.crt` and `tls.key`
     from step 2 (PRV-007, PRV-022). Every other input is the core's, unchanged, so the run gets
     only the item's own environment: add no variable of the engine's environment, none of the six
     names at `runner.rs`:425-429, no name that `Config::pass_environment` lists, and no
     state-directory variable, which only the engine's own runner gives (PRV-029, PRV-007,
     SEC-020). Where order 079 leaves a provider's run without an `https` address, as in a store
     that has none, send nothing and report the run failed: PRV-022 gives every run off the
     engine's own runner an `https` URL.
   - **Reports.**
     - `run.v1`: succeeded with a `url` gives succeeded with that `url` once step 5's bridge has
       initialized a session there, and unknown if it cannot, since the run may be running; failed
       gives failed; performed nothing, or not sent, gives failed; unknown, or no result, gives
       unknown.
     - `stop.v1`: succeeded and failed as stated; anything else, or not sent, gives unknown.
     - `inspect-run.v1`: succeeded with a valid `state` gives that state, except that `running`
       gives `running` only where a bridge's session reaches the run, as the next points say;
       anything else, or not sent, gives unknown, so the part reports unavailable (PRV-007).
   - **A run reached.** Once the bridge's session is initialized, enter it in the run table under
     the instance, as `start` enters a local run (`runner.rs`:533-553), marked as a provider's run
     and holding its `url`.
   - **`stop.v1`** ends the instance's bridge first, removing its entry and stopping its group, as
     the built-in stop ends a local run (`runner.rs`:268-275), and then calls the provider.
   - **`inspect-run.v1`** reports the state the provider gives, `stopped` and `unknown` as given.
     Where it is `running`, and the table holds no live session for the instance, or the result's
     `url` differs from the one the live bridge reaches, the shell first starts a bridge to the
     latest `url`, the result's or else the entry's, and once its session is initialized replaces
     the entry, ends the old bridge and reports `running` (IFC-017, PRV-009). Where no bridge's
     session is initialized, because there is no `url` or the bridge fails, the shell leaves a
     diagnostic and reports `unknown`, as `run.v1` does when its bridge fails, so that the part
     reports unavailable and nothing is dispatched to a server that has proved nothing (IFC-017:
     "Where the engine cannot verify who answers, nothing is dispatched"; PRV-028). The run's
     `Running` condition then says `Unknown` where the provider said `running` (PRV-009); step 8
     reports this.
   - **Setup cancellation** ends a bridge that its setup started and starts none, through the
     existing paths (`runner.rs`:329-347, 358-362), and sends the provider nothing: stopping a run
     belongs to a retirement (PRV-008).
   - **Dropping the runner** stops each bridge's group, but submits no `builtin-run-stopped` for a
     provider's run (`runner.rs`:214-228): the core refuses that observation for an instance that
     names a runtime provider (`core/src/lib.rs`:363-372).
   - **Diagnostics** are fixed texts that name the operation and what happened, such as a provider
     with no run, a discovery result without Conduit's extension, or a certificate that did not
     prove the instance ([ruling 52](../proposals/ruling-52/README.md), point 2). They hold no
     argument, identity value or result content (`runner.rs`:190).

5. **The bridge, `conduit __http-bridge <url>`.**
   - **Entry.** `main` hands `__http-bridge` to its own function before `run()`, as it does
     `__run-guard` (`main.rs`:437-442), and the bridge opens no store. Put it in its own module.
     It uses no rmcp type, and parses JSON only to read what it must, such as the negotiated
     protocol version.
   - **Start.** The runner starts it through `Inner::spawn` (`runner.rs`:350-416), with the
     runner's guard program, which is the `conduit` executable, and the arguments `__http-bridge`
     and the `url`, so that it has its own process group and lifetime socket and ends with the
     engine. No other value goes in argv. Its environment holds one variable, `HTTP_BRIDGE_DIR`,
     which names a private directory under the store, made as the runner's other directories are
     (`runner.rs`:155-157), with files at mode 0600: the store's root certificate alone, never the
     roots that order 079 adds to `ca.crt` for the engine's address, which may be a public
     authority's; the SPIFFE ID that the certificate must name; the engine's token toward that
     instance; and the engine's certificate and key (step 2). Use that name exactly: a variable
     whose name begins with `CONDUIT_` and is not one of the three names of `spec/names.yaml`
     fails `python3 tools/set_names.py --check` (`tools/set_names.py`:249-254).
   - **Reading the files.** The bridge reads every file in that directory once, when it starts and
     before it sends anything, keeps what it read in memory for its whole life, and never opens
     them again: every later POST, GET stream, DELETE and new TLS connection uses that copy. The
     runner removes the directory once the bridge's MCP initialization has ended, either way.
   - **The check, before anything is sent** (IFC-017, PRV-028). A `url` that is not `https`, or
     that holds user information or a fragment, is refused: a listener that proves nothing is no
     proof. The TLS client is rustls with the `ring` provider. Its server-certificate verifier
     accepts only a chain to the store's root whose end certificate is not a CA and names exactly
     one URI, equal to the expected SPIFFE ID. It does not check the `url`'s host, which no
     instance's certificate names, so it does not delegate to rustls's `WebPkiServerVerifier`,
     which checks the server name and would refuse every instance's certificate. It checks the
     chain with `rustls::client::verify_server_cert_signed_by_trust_anchor`, on a
     `rustls::server::ParsedCertificate`, against a root store that holds the store's root alone
     and with the provider's signature algorithms; rustls-webpki performs that check, and it
     checks no name. It checks the handshake's signatures with
     `rustls::crypto::verify_tls12_signature` and `verify_tls13_signature`. It reads the URI and
     the CA flag with x509-parser. If you use rustls-webpki directly instead, it is the version
     that rustls uses, and you say why. rustls runs the verifier during the handshake, so no HTTP
     byte and no token leaves before the check passes. The client presents the engine's certificate
     where the gate asks for one. It follows no redirect, uses no proxy and no cookie store, and
     asks for no content encoding.
   - **Bytes.** Each line on stdin is one JSON-RPC message, sent unchanged, without its newline, as
     the body of one POST to the `url`, with `Content-Type: application/json`,
     `Accept: application/json, text/event-stream`, the engine's token as `Authorization: Bearer`,
     `Mcp-Session-Id` once the server has assigned one, and `MCP-Protocol-Version` once
     initialization has named one. Every request, GET and DELETE included, carries the token. A
     JSON body, and the data of each server-sent event, is written to stdout unchanged, as one
     line: a raw carriage return or line feed in it, which valid JSON allows only as whitespace,
     becomes a space, and nothing else changes. A `202` with no body writes nothing. The first
     message, the initialization, is sent alone; later messages are sent as they arrive, without
     waiting for earlier responses, and one writer keeps each message whole on stdout.
   - **The GET stream.** Once initialized, the bridge opens the GET stream for messages that the
     server starts. A `405` means there is none. If it ends while the session lasts, the bridge
     opens it again, at most once a second.
   - **Ending.** At the end of stdin the bridge sends DELETE for its session and exits 0. A
     response that it cannot pass on as JSON-RPC ends it: a redirect, a `401` or `403`, a `404` for
     its session, any other status without a JSON-RPC body, or a transport error. A JSON-RPC body
     is passed on whatever the status. Ending, it closes stdout and exits, so that the engine's
     session ends as a local run's does when its process exits. A refusal under the check exits
     with a status that no other ending uses; the runner reads it through the lifetime socket, as
     it reads the npm helper's (`runner.rs`:697-717), and records the diagnostic of step 4.
   - **Not built:** resuming a stream with `Last-Event-ID`, and the `Mcp-Param` headers of
     revision 2026-07-28. The bridge carries the revision that the runner's initialization names
     (`runner.rs`:517-518); find which it is, and state it in the DL16 row.

6. **Tests.** They bind loopback ports only, must pass on Linux and on macOS, and none sleeps for
   minutes.
   - **Credentials** (unit tests): a provider run's `tls.crt` chains to the store's root, names
     the instance's SPIFFE ID as its one URI, is not a CA, has both extended key usages, does not
     lapse before the token's `exp`, and matches `tls.key`; a run on the engine's own runner gets
     neither file; the engine's token toward a gate has the `sub`, `aud` and signature of step 2,
     and `verify` refuses it at every engine address; the engine's certificate names the engine's
     ID.
   - **The bridge**, run as `conduit __http-bridge` against a loopback TLS server in the test, with
     certificates that the test issues from a test store's root:
     - stdin lines with unusual but valid spacing and escapes reach the server byte for byte; a
       JSON body and the data of server-sent events reach stdout byte for byte, one line each;
       `Mcp-Session-Id`, `MCP-Protocol-Version` and the token are carried as step 5 says; the end
       of stdin sends DELETE;
     - after initialization the test removes the bridge's directory, and a further message
       still reaches the server with the token, and so does the DELETE at the end of stdin;
     - each of these is refused, with the refusal's exit status, and the server receives no HTTP
       request: a certificate from the store's root that names another instance; a self-signed
       certificate that names the right instance; a certificate that names the right instance from
       another root; an `http` URL;
     - a server that asks for a client certificate receives the engine's;
     - a redirect is not followed: its target receives nothing, and the bridge ends.
   - **Provider calls**, through the shell and the runner, against a scripted provider: an ignored
     test function spawned as the provider's own run, as `scripted_server` is
     (`tests/runner.rs`:196-283), registered with documents shaped like those of
     `conformance/conventions/setup/review-relative-provider/documents.json`. It answers
     `server/discover` and `tools/call` for the three operations, records every message it
     receives, and, for `run.v1`, starts in its own process a loopback TLS gate. The gate serves
     the call's `tls.crt` and `tls.key`; admits only a token whose `sub` is the call's `engine-id`,
     whose `aud` is the instance's SPIFFE ID and whose signature checks against the call's
     `jwks.json`, and answers anything else with 401; answers MCP itself; counts the requests it
     receives; and its `url` is what `run.v1` returns. Cover:
     - a setup of an instance that names the provider: `server/discover` comes before any
       `tools/call`; then `inspect-run.v1`, answered `stopped`, and `run.v1`, each with exactly
       PRV-005's `_meta` keys and the extension declared; `run.v1`'s `environment` is exactly the
       item's bindings, with none of the six names, no name of `Config::pass_environment` and no
       state-directory variable. Set `Config::pass_environment` to one variable that the test
       process holds and that is none of the six, such as `CARGO_MANIFEST_DIR`, which `cargo test`
       and nextest set for each test process; assert first that the test process holds it, and
       then that `environment` lacks it. Its `identity` holds order 079's `https` address and the
       six files; the setup succeeds, and a dispatched call to the instance reaches the gate
       through the bridge and completes;
     - the gate serving a self-signed certificate that names the instance: the gate receives no
       request, the run part reports unknown, and the diagnostics say that the certificate did not
       prove the instance;
     - a discovery result without the extension: the provider receives no `tools/call`, and the
       run part reports unavailable;
     - `run.v1` answering failed, unknown, an outcome that its `isError` contradicts, and
       succeeded without a `url`: each report is step 4's;
     - a retirement: `stop.v1` reaches the provider with the instance and its `machine`, the
       bridge's process ends, and the stop's outcome follows the provider's;
     - after the gate has ended the bridge's session, answering `404` for it, a further setup's
       `inspect-run.v1`, answered `running` with the `url`, starts a new bridge, and a dispatched
       call completes again;
     - a further setup's `inspect-run.v1`, answered `running` with the `url` of a gate that serves
       a self-signed certificate naming the instance: that gate receives no request, the part
       reports unavailable, and the diagnostics say that the certificate did not prove the
       instance;
     - the provider-call function of step 3 sends `list-machines.v1`, which the core does not emit
       yet, with PRV-005's keys, and returns the result it read;
     - after these, exported records and inspection results hold no part of `token`, `tls.crt` or
       `tls.key` (PRV-024);
     - dropping the runner with a provider's run in the table ends that run's bridge.

7. **`engine/CONFORMANCE.md` and `engine/README.md`.**
   - Rewrite the DL16 row "executor coverage" (line 40): provider calls of `run.v1`, `stop.v1` and
     `inspect-run.v1` are served through the provider's own run, discovery first, with the keys and
     the reading of step 3 and the reports of step 4; other provider operations are not yet
     reported; remote connections remain unserved.
   - Add a DL16 row for the runs of other runtime providers: the bridge, what it carries and what
     it does not (step 5), the revision it carries, what ends it, the variable that names its
     directory, and that a provider whose own run has ended is not run again before a call
     (PRV-026, not built).
   - Add to the DL13 rows: the run's X509-SVID and when it is given; the engine's token and
     certificate toward a gate; and the check before anything is sent, against the store's root
     alone. Leave order 079's text about the engine's `https` address as it stands.
   - Under "Choices still required", remove from the DL13 and DL16 rows (lines 59-60) only what
     this order builds, and add what it leaves: PRV-026, and the reports that step 8's first
     finding describes.
   - Two sentences outside those rows say the same, and change with them; leave the rest of their
     paragraphs as it is:
     - lines 263-264, "Runtime providers, remote connections and secrets still have the unserved
       boundaries listed above; this order qualifies the built-in runner on S1, not those
       providers.": runtime provider calls are served as the DL16 rows state, remote connections
       and secrets keep the unserved boundaries listed above, and that order qualified the
       built-in runner on S1, not runtime providers;
     - lines 318-319, "they do not qualify remote transport or runtime providers, which remain
       unfinished as above.": they qualify neither remote transport, which remains unfinished as
       above, nor the runtime provider calls that the DL16 rows describe.
   - `engine/README.md`, line 225, says "Runtime providers and remote connections remain
     unserved". Say instead that runtime provider calls are served as CONFORMANCE's DL16 rows
     state, and that remote connections remain unserved.

8. **Findings, in your report.** The open increment's log is on `main`; the admin writes your
   findings into it when `s3/m2` merges. Report at least these:
   - IFC-017 and PRV-028 have the part report unavailable, with a record that says why, where the
     engine cannot verify what answers a run's `url`, and PRV-005 has the record of a call that was
     not sent name the missing extension. The core's reports (`lifecycle.rs`:10-26) can say
     neither, so this order reports unknown or failed as step 4 says. Where a provider's
     `inspect-run.v1` gives `running` and no bridge proves the instance, the shell reports
     `unknown`, so the part is unavailable as IFC-017 has it, but the `Running` condition then
     says `Unknown` where PRV-009 has it follow the provider's report: with the core's reports,
     PRV-009 and IFC-017 cannot both hold there.
   - PRV-026, running a provider again before a call, is not built.
   - The shell reads no `server/discover` (IFC-037) and, over Streamable HTTP, no `tools/list`
     before its first call (IFC-001) when it dispatches to an instance's run, local or bridged.
   - Anything else in which the specification and what you built differ.

## Scope

Change:

- `engine/crates/shell/`: `Cargo.toml`, `src/` (`runner.rs`, `lib.rs`, `identity.rs`, `main.rs`,
  a new module for the bridge, and `server_tests.rs` for step 1's calls only) and `tests/`;
- `engine/Cargo.lock`;
- `engine/CONFORMANCE.md`: its DL13 and DL16 rows, and the two sentences of step 7 at lines
  263-264 and 318-319;
- `engine/README.md`: the sentence of step 7.

Do not change:

- `engine/crates/core/`, `engine/crates/loader/` or `engine/crates/conformance/`;
- `spec/`, `conformance/` or any vector, so there is no `080-declared.yaml`;
- `packages/`;
- `server.rs`, the engine's `https` address, its listener and its TLS, or which address a run
  gets: they are order 079's;
- the open increment's log, `docs/work-orders/` or any record.

Order 079 also edits `lib.rs`, `identity.rs` and `main.rs`. Add to its changes; do not rework
them. Order 068 (`docs/work-orders/068-test-gate-and-ci.md`), in flight on `main`, edits the
shell's test files, `src/tests.rs`, `tests/runner.rs`, `tests/serve.rs` and `tests/cli.rs` among
them, puts the timing-sensitive tests in a nextest group that it chooses by test name, and adds a
check that fails on any tracked file holding a home-directory or `/privat[e]/tmp/` path. So add each
of your tests as a new function, change an existing test function only by step 1's call, list in
your report each new test that binds a loopback port or waits on a clock, so that the admin can
place it in that group at the rebase, and write no such path in any file. Report anything else
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
  - `grep -nE 'Runtime provider calls and remote connections are unserved|Runtime providers, remote connections and secrets still have|remote transport or runtime providers, which remain unfinished|Runtime providers and remote connections remain unserved' engine/CONFORMANCE.md engine/README.md`
  - `grep -rnE '/(Users|home)/[^/ ]+/|/privat[e]/tmp/|/tmp/claude' --exclude-dir=target engine/crates/shell engine/CONFORMANCE.md engine/README.md`

In `engine/`:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release -p conduit-shell`, and then the full `cargo test --release` once;
- `grep -c '^name = "aws-lc' Cargo.lock` and
  `grep -cE '^name = "(openssl|openssl-sys|openssl-src|native-tls)"$' Cargo.lock` each print `0`,
  and `grep -A1 '^name = "rustls"$' Cargo.lock` shows one version, 0.23.

Report the totals passed and failed. Tests that bind loopback ports may fail in your sandbox; if
they do, say so, and the admin reruns them. Under load, timing-sensitive shell tests have failed and
then passed with two test threads (finding 4-18); if you rerun so, say so. Do not stage anything.
