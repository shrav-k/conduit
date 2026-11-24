# Building a client of Conduit outside this repository

For someone who builds a client of Conduit (an app, a panel, a tool) in a repository of their
own, in any language. This page is a guide and binds nothing: `spec/` alone binds, and
`engine/CONFORMANCE.md` states the first build's choices; both win where this page differs.
Words used here, most defined in `spec/definitions.md`:

- **Engine**: the program that holds Conduit's state and decides every request. This
  repository's first build is the `conduit` command, built from `engine/`.
- **Interface**: the object through which a person acts. **Root** is the Interface that exists
  from the start, and the only one with access then.
- **Conduit**: a declared, recorded link that lets one object call a listed set of operations on
  another. An object can do only what its open Conduits list.
- **Promise**: one numbered rule in `spec/`, such as IFC-022 (`spec/README.md`, "The form of a
  promise").
- **MCP**: the Model Context Protocol. The engine serves each consumer of an open Conduit, such
  as an Interface, an MCP server whose tools are the operations its open Conduits list.
- **Pending action**: something that waits on a person, such as a sign-in or a failed setup.

## 1. What a client is, and what it is not

A client is a program that speaks to the engine for an Interface. It may act for any Interface
it holds a credential for; what it may do is set by that Interface's open Conduits (IFC-022). It
is an MCP client of the server the engine presents to that Interface (IFC-012), or a program
that runs the `conduit` command and reads its output. Until DEC-074's client token exists, a
client is only the second and never opens root's token file (DEC-090), and records name root, not
your client.

The owner's app, in v0, is a client of root's Interface on the engine's machine (DEC-074). In
whole, DEC-074 decides: the app presents a token whose subject is root, with a claim naming the
client and no scope; Conduits alone decide what it may do (IFC-022); records name the client;
all of it on the engine's machine. That token is not built yet (section 2).

A client decides nothing: the engine alone decides each request (`spec/definitions.md`,
"engine"). So a client is not:

- **an authority.** It adds no rule of its own: a state, cause or verdict it shows is a value the
  engine served. The spec paragraph that says so is decided, not yet written (DEC-075).
- **a cache of permissions.** Each request is decided anew, and no decision is reused
  (ACC-036). Send an effectful request only when the person acts, and show the answer.
- **the engine's panel.** Code and fingerprint evidence, including device codes, stays off the
  wire and is shown only in the engine's own panel (PND-004, DEC-063, IFC-049). That panel is not
  built yet (DEC-075, order 051c), so today a client cannot complete a login that needs a device
  code shown there. Login returns are different: PND-013 permits a page's opaque code or a
  redirect return containing an authorization code through `signInReturn`, subject to its
  checks. Such a return is neither evidence nor a prompt (PND-004).
- **a keeper of secrets.** Documents, state, results and records hold only a secret's reference
  (SEC-001, SEC-010).

What stays the client's: layout, grouping, search, colours, labels, totals from served data,
and marks it keeps for itself, such as an unread dot.

## 2. The contract today

There is no single contract file yet: the contract is the promises below, the core operations in
`spec/core/` and `engine/CONFORMANCE.md`.

| Area | Promises | In short |
| --- | --- | --- |
| Transport | IFC-002 | MCP, at the baseline revision 2026-07-28. The first build's transport is in the list below the table. |
| Who asks | IFC-022, IFC-042, IFC-032; DL13 in `spec/implementation-defined.yaml` | Each request carries a Bearer token: a signed JSON Web Token in SPIFFE's form (a JWT-SVID) that names the instance, with the engine's address as audience. The engine checks it on every request. A missing or bad token gets HTTP 401. |
| Request form | IFC-036, IFC-023, OPS-008, OPS-009 | A present `Origin` header must equal the engine's own origin, or HTTP 403. A request that puts a name under Conduit's prefix in `_meta` declares Conduit's extension, or gets -32021. An effectful request may carry an idempotency key; after a lost reply, repeat it with the same key. |
| Tools | IFC-012, IFC-013, IFC-033, IFC-038 | The operations of the Interface's open Conduits, apart from `get-secret.v1`, controls the target lacks, and operations whose contracts the target does not hold (IFC-012, IFC-033). Each is named `<Conduit's local name>.<export name>`, for example `management.inspect.v1`. Each has an `inputSchema`, an `outputSchema` where the operation declares outputs, and the hints its contract states (IFC-033). A call to a tool not presented gets JSON-RPC error -32602, with the reason in the error's `data`. |
| Results | IFC-025, IFC-026, OPS-001, OPS-005, IFC-050, ACC-056 | Either an outcome (succeeded, failed, unknown) or a reason (refused, invalid, conflict, unavailable), from the engine or from the implementation, never both. Diagnostics name the deciding promise as `ruleId`. Every result says in full what happened and why (DEC-010). |
| Pending actions | PND-001, PND-002, PND-003, PND-006, PND-007, PND-009 | `list.v1` lists them; `answer.v1` answers some. Each kind has its own answers, and no answer grants anything. Each result of an Interface's request carries the `unanswered` count when it is not zero. Kinds are open strings: show an unknown kind plainly. |
| Sign-in | IFC-049, PND-004, PND-005, PND-010, PND-011, PND-013, CTR-013 | `sign-in.v1` must declare URL elicitation. The engine relays one URL with its own message: show it word for word, and open the URL only when the person agrees. Sign in the login Resource where an Agent's Type names one (CTR-013, DEC-042). Per-Agent sign-in stays, for compatibility Types only, until DEC-078's trim: show whichever subject the engine names. Show "signed in" only after the engine's account check. |
| Records | REC-001, REC-002 | Every request is recorded, reads included, except an unreadable request from which no instance can be established (IFC-036). A record holds an operation's inputs and outputs only where its Type's contract says so, except that `install.v1` records confirmed shared client IDs in `data.inputs.confirmClientIds` (REC-002). The Claude Code and Codex Agent packages keep neither, so their turns' text is not recorded. |
| Root's own acts | ACC-032, ACC-054, ACC-057, SEC-008, PND-009, PND-010 | Among them, only root: registers Agents and value Resources (ACC-057); opens a Conduit listing `get-secret.v1` (SEC-008), or one whose consumer is root (ACC-054); declares nodes lost, purges records and runs the enrollment verbs (ACC-032); signs in login Resources (PND-010); and answers a login's notice (PND-009). |
| Where root starts | FST-002, FLEET-005 | Conduits to `management`, `access` and `system`, and one to each later Pool and Fleet. Their operations: `spec/core/capabilities.yaml`, `spec/core/operations.yaml`. |

The first build's choices (`engine/CONFORMANCE.md`, "Consumer server" and the DL13 rows):

- It serves MCP over Streamable HTTP at `/mcp`, and accepts revisions 2025-06-18 and 2025-11-25
  beside the baseline, and no other transport yet (DL12, a choice still open).
- The address is fixed at `conduit init`: by default `http://127.0.0.1:3000/mcp`, loopback only.
- `GET /records` on that listener takes root's token and returns every record as CloudEvents
  JSON, each time, unpaged. It is not a request.
- The optional https address admits only tokens given to runs under a runtime provider. Root's
  token is refused there, so no client can act as root over https today.

### Not built yet, as of `main` at `d7e63ebd`

| What | Decided in | State |
| --- | --- | --- |
| The client's own token (section 1) | DEC-074 | No order yet. Open: how it is issued, the claim's name, how a record names the client (REC-002 would change). |
| Root's acts opened to a scoped Interface, one at a time | DEC-074 | After the token. |
| An authorization server (OAuth 2.1, RFC 9728 metadata) | DEC-074, DL13, IFC-042 | After the engine's panel. |
| The engine's panel | DEC-075 | Order 051c, not written yet. An app may be built now; its engine needs follow the panel (DEC-091). |
| `conduit plan --output json`; `conduit apply --expect sha256:HEX` | DEC-065, DEC-075 | Not built. `plan` prints text only. |
| A change feed | left to the engine (DL8) | Not built: a client polls. |
| An Agent's request for access, as a new pending kind | DEC-064, DEC-079 | Ruled for v0; designed; no order yet. |
| Health conditions reported by implementations | DEC-066, DEC-079 | Ruled for v0; not built. |
| `history.v1`: an Agent's replies made readable | DEC-067, DEC-079 | Ruled for v0; not built. |
| Usage limits kept and shown | DEC-077 | Kept and shown by `inspect.v1` (order 289); no Agent package reports them yet. |

DEC-074's steps, in order: the panel; the client token's spec and engine orders; then one order
per root-only act opened to a scoped Interface, each with its own guard.

## 3. Connecting and authenticating today

There are two ways in. A client uses only the first today (section 1).

**Through the CLI.** The engine runs on macOS or Linux only (DL16). Install rustup and the
toolchain that `engine/rust-toolchain.toml` pins. In `engine/`, run
`cargo build --release -p conduit-shell`; the command it builds is
`engine/target/release/conduit`. Run `conduit init --store DIR` once, and leave
`conduit serve --store DIR` running. `serve` records where the engine is in `context.json`, in
the directory `CONDUIT_CONFIG_DIR` names, or else in `.config/conduit` under your home directory
(`engine/CLI.md`, "Engine context"). The other commands find the engine there, or take
`--store DIR`, and act as root. Your client runs them and reads standard output.

| Command | Gives |
| --- | --- |
| `conduit pending [--agent NAME]` | the pending actions raised at root, one JSON line each, as `list.v1` gives them |
| `conduit inspect` | every object, one JSON line each |
| `conduit tools [--as NAME]` | the tools presented to root, or to a named instance |
| `conduit call TOOL 'JSON'` | one call as root, printing the full result; exits non-zero for any result that did not succeed: a reason, or an outcome failed or unknown |
| `conduit records` | the records, ordered, as JSON lines |
| `conduit plan [DIR]` | the steps apply would send, as text; a change needing approval stops, naming its flag |
| `conduit apply [DIR]` | prints the plan, then sends it as root with no prompt. Both take approval flags; see `conduit plan --help` and `conduit apply --help` |
| `conduit sign-in KIND/NAME` | the engine's message and URL; opens the browser only after `y` |

`conduit apply` and the approval flags are built. The statements in `engine/CLI.md` that they
are not built and that passing an approval flag makes the shipped planner exit 2 are stale;
use the command help above for current usage.

`conduit answer` sends only `dismissed`, for a setup action. Acknowledge a notice with
`conduit call` and `answer.v1`. A sign-in is answered by `conduit sign-in`. A further
`set-up.v1` answers a setup action `retried`, and retiring its instance answers it `retired`
(PND-006). Enrollment and machine actions (PND-008) are specified but not built; the first build
raises only setup, notice and sign-in actions. `conduit sign-in` asks the person questions, so
run it where the person sees it.

DEC-075 rules that a document change from an app reaches the engine only through
`conduit plan --output json` and `conduit apply --expect sha256:HEX`. Neither is built, so an
app has no ruled way to send document changes yet.

**MCP directly** waits for DEC-074's client token (section 1): then send requests to the URL in
`context.json`, with that token as a Bearer token.

Once a client speaks MCP, the first two points hold; the third holds now:

- **No foreign Origin.** A browser or webview sends its own `Origin`, and the engine answers
  HTTP 403 to each such call (IFC-036). Make engine calls from a native or backend layer.
- **Declare what you use**: Conduit's extension when you put names under its prefix in `_meta`,
  and URL elicitation for `sign-in.v1` (IFC-036, IFC-049). HTTP 401 means the token was refused.
- **Reads are not free.** Each read through MCP is a recorded request (REC-001). Poll gently, and
  drive a "waiting" badge from the `unanswered` count (PND-002). Over the CLI, the result that
  `conduit call` prints carries the count in its `_meta`, under a name ending in `/unanswered`;
  other commands say the count only as text on standard error. Or count `conduit pending`'s
  lines.

## 4. What may change, and how a client learns of it

Everything here is a draft until v0, and nothing promises compatibility yet.

- **Names.** The `apiVersion` and the prefix Conduit puts on its names in `_meta` come from
  `spec/names.yaml`; the extension's name is that prefix followed by `implementation-interface`
  (IFC-038). They change together when the project moves to its own organization. Keep them in
  one place in your code.
- **`spec/CHANGES.md`** has one line per change to `spec/`, naming the promise IDs and the
  decision. **`docs/decisions/log.md`** holds the owner's decisions, `DEC-NNN`, append-only,
  each with a Status line.
- **`engine/CONFORMANCE.md`** holds the first build's choices, such as the address and the token.
- **At run time**, `server/discover` lists Conduit's extension at revision `v0`, and the server
  notifies a change to the tool list (IFC-038). `conduit --version` prints the version and build
  commit, and `context.json` records it. Note which engine and which `spec/` commit you built on.

## 5. Proposing a change to the contract

Inside your own client, you decide. What changes a promise in `spec/`, the engine, this contract,
the decision log or how the project works goes to the owner, who rules.

Open a pull request; the project keeps its tracking in this repository, not in GitHub issues,
for now (DEC-031). In its description give:

1. the question, in one line, and what your client needs and why;
2. the promise IDs it touches;
3. which existing standard or tool already does this (`spec/README.md`, "Existing standards
   first"), since Conduit invents only connection and access (DEC-003);
4. two or three options, one marked as recommended, and what a client sees under each.

Do not record a recommendation as accepted. The owner's answer becomes a log entry in the
owner's words, and the change to `spec/` follows with its `spec/CHANGES.md` line.

## 6. Reporting a bug in the engine

First read the promise. If `spec/` says nothing about the behavior, that is a gap in `spec/`, not
an engine bug: report it as a gap. Report by pull request, and give:

- the output of `conduit --version`, and the MCP revision your client used;
- the steps from `conduit init --store DIR`;
- the request and the full result, with no token, sign-in URL, code or secret in them;
- the promise you expected to hold, by ID, and what happened instead.

The best report is a failing test or a conformance vector under `conformance/`. The coordinator
settles each finding in the first build's open log,
`docs/implementation/first-build/log-increment-4.md`.

## 7. The local gate

`main` is protected: every change comes by pull request and is squash-merged with a message
written by hand. A merge waits on the PASS of `tools/gate.py`. CI runs on each pull request as a
second opinion; merges do not wait for it (ruling 80). Here the engine is Rust, and a package may
be written in any language; other code is Rust, apart from `tools/`'s Python checkers and the
Workflow tool's JavaScript (DEC-025, DEC-111). Before you push, from the root:

```sh
git fetch origin               # the gate compares with origin/main as last fetched
python3 tools/gate.py --plan   # lists the gates your change needs; runs only discovery
python3 tools/gate.py          # runs them; its last line says PASS only if all passed
```

`AGENTS.md`, "Checks", says what it needs: Python 3 with PyYAML and jsonschema, Git, rustup,
and each Rust directory's crates in cargo's cache, since it runs cargo offline (run `cargo fetch`
once there). Put its last line, and the gates its record under `target/gate/` lists, in your pull
request. Two rows:

- **private paths**: the tree holds no home-directory path, no email address outside the
  reserved example domains, and no private name (`tools/check_private_paths.py`).
- **identity**: it fails any commit not made under the neutral identity `AGENTS.md` names, which
  is for Agents and the coordinator. The owner has ruled (DEC-089) that it will also pass
  commits by a contributor listed in a file the owner approves, and that a squash keeps them as
  author. Until that is built, a gate run on a person's branch ends in FAIL even if every other
  row passes. Give that result unchanged in your pull request. The coordinator squashes the
  branch as the work-order identity and reruns the gate before the merge, following the
  [merge procedure](implementation/METHOD_AS_RUN.md#7-merge-and-the-record), so main names that
  identity as author until DEC-089 is built.
