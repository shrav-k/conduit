# 032 — A85: the gateway probe

Every result of this experiment is obtained under one operating-system user, on one machine, with
fake tokens, against a stub gateway, inside a network namespace that holds loopback alone. So no
result here shows what a real gateway, a vendor's service or a real login does, or that Agents are
kept apart. It shows what each client, as installed, sends and tries to reach.

Pre-registered on 2026-11-18 by work order 205, before its harness was written and before any
run. Not run: the harness (order 205b) and the run (order 205c) come later, each after the one
before it merges. The text below changes only by a dated amendment, written before the run it
changes.

## Why

[Ruling 83](../../docs/proposals/ruling-83/README.md), point 2, sets a direction: an Agent never
holds a model service's API key, and model API access comes later as a declared route, a
credential-injecting gateway that a provider runs, to which the Agent's client is pointed instead
of the vendor. Its point 3 has a grounded study read each client's gateway support before the route
is designed. The study recommended this probe, so that what it read is seen in the clients
themselves. The study's record is not yet in the tree: the coordinator commits it under
`docs/research/` by a separate change before order 205b is written, and links it from A85's
Source cell and from here; the link here is added by a dated amendment. The study has no register
row of its own: it ran with no order, so ruling 83's planned row for it is taken up by A85, by the
coordinator's reading. The claims under test, K1 to K4, are the study's statements, not facts:

- **K1.** Claude Code can be pointed at a gateway by `ANTHROPIC_BASE_URL`, with the gateway's
  credential in `ANTHROPIC_AUTH_TOKEN` or given by an `apiKeyHelper`.
- **K2.** With a base URL set and no gateway credential, Claude Code may send a saved subscription
  login's token to the gateway.
- **K3.** Codex 0.167.1 takes a custom model provider with `base_url`, `wire_api = "responses"`,
  and either `env_key`, an auth command, or no key.
- **K4.** In such a provider, `requires_openai_auth = false` keeps the ChatGPT login off.

The probe decides nothing. Its results go, with the study, to the route's design, which comes to
the owner (ruling 83, point 3). It gates nothing in 0.2.0.

## Questions

- **Q1. Hosts.** In each case, which host names does each client look up, and which addresses and
  ports does it try to connect to, besides the stub gateway?
- **Q2. Credentials.** For each way of giving the gateway a credential (K1, K3), which request
  header carries which fake token, on which method and path?
- **Q3. The silent-login case.** With a base URL and no gateway credential, in a fresh home that
  holds no login (K2, K4): does the client send the gateway anything, and does any request carry a
  credential header, empty or not?
- **Q4. A 401.** When the gateway answers every request with 401: how many requests does the
  client send, does it run its helper or auth command again, does it look up or try to reach any
  other host, does it start a login, and what does it report?
- **Q5. A streamed tool call.** When the gateway streams a call to a tool, does the client take it
  and send its result back to the gateway, with the same credential?

## The clients and the machine

- **Claude Code 2.1.318**, the native-client launch baseline that the
  [Claude Code Agent package](../../packages/agent-claude-code/README.md) names, and **Codex
  0.167.1**, the version that [experiment 027](../027-a5-sign-in/README.md) recorded and its reads
  of Codex's source cover. No other version runs.
- Each is installed with npm into this directory's `target/clients/`, which Git ignores: Claude Code
  as the package `claude-code` of the npm scope `anthropic-ai`, and Codex as the package `codex` of
  the npm scope `openai`, each at its version. Installing is the one step that reaches the network;
  it runs outside the namespace, before any case, under `env -i`, with `PATH` as in the cases (the
  directory of `node`, then `/usr/bin` and `/bin`), with `HOME` and the npm cache in a scratch
  directory under `target/`, and with `--ignore-scripts`. If a client's `--version`, run outside the
  namespace under `env -i`, with `PATH` as in the cases (the directory of `node`, then `/usr/bin`
  and `/bin`), does not then print its version, that client is installed once more with its script,
  and the record says so, with the hosts its script reached if they are known. A client whose
  `--version` still prints nothing after the scripted install runs no case. The record keeps each
  package's integrity from the lockfile. Inside the namespace, each client's `--version` must print
  its version before any of its cases runs.
- **The Linux node only**: the second machine's WSL Ubuntu 24.04, on x86_64. Never the Mac, whose
  clients may read the login keychain, and never Windows outside WSL.

## The method

The harness is a Rust crate in this directory, `harness/`, that uses Rust's standard library
alone, so that it builds with no crate fetched. Order 205b writes it, with its tests, and
`HARNESS.md`, the method as built, before any client runs. It has five parts.

**The namespace.** The cases run as the Linux node's ordinary user that runs the work orders, with
no privilege; its name is never recorded. Each case runs inside a namespace of its own, made by
`unshare --user --map-root-user --net --mount --uts`. If `unshare --user` fails on the node, the
registered alternative is `bwrap --unshare-user --unshare-net --unshare-uts`, with the same masks,
each given as a `--tmpfs`, and the same self-test; under `bwrap` the host name is given by
`--hostname probe-205`, a file inside a mask by `--dir` and `--file`, and the logger keeps
`CAP_NET_BIND_SERVICE` by `--cap-add`, or else the self-test fails and stop rule 1 applies. Inside
it, before anything else:

1. loopback is brought up, and the namespace holds no other interface; the namespace's host name
   is set to `probe-205`;
2. an empty `tmpfs` is mounted over each of these directories that exists, and the case reaches none
   of them: `/run` as a whole, which holds the user runtime directory (`/run/user/` and the user's
   numeric ID), the session bus, `/run/nscd` and `/run/WSL`, and to which `/var/run` links; `/tmp`;
   `/var/tmp`; `/dev/shm`; `/mnt`, which holds the Windows drives and WSL's own sockets;
   `/etc/codex`; `/etc/claude-code`; and the running user's real home directory. So no session bus,
   keyring, name cache, managed setting, login, Windows drive or path-bound Unix socket in those
   directories answers; a new network namespace cuts off abstract sockets alone. Under the home's
   mask, this experiment's directory is bind-mounted back if it lies under the home, and so is the
   directory of the `node` that installed the clients if it lies under the home. Nothing else is
   masked or bound back: the list is fixed here, not chosen by the harness;
3. bind mounts cover `/etc/resolv.conf` with a file whose one line is `nameserver 127.0.0.1`,
   `/etc/hosts` with one that holds only the loopback names and the line `127.0.0.1 probe-205`,
   and `/etc/nsswitch.conf` with a copy of itself whose `hosts:` line is `hosts: files dns`, so
   that every lookup reaches the logger. Where `/etc/resolv.conf` is a symbolic link whose target
   lies under a mask of step 2, the target's path is made inside that mask and the one-line file
   is written there, with no mount; where its target lies elsewhere, the file is bind-mounted over
   the target;
4. the harness's DNS logger and stub gateway start;
5. the client starts in a nested user namespace that maps the user's own numeric ID, so that it
   runs with no capability, under `strace -f -qq -e trace=connect,execve`, with its environment
   cleared as `env -i` clears it and then set to exactly the case's variables.

**The tools.** Before anything runs, `command -v strace` must find `strace` on the node, and,
when the alternative is needed, `command -v bwrap` must find `bwrap`. If one finds nothing,
nothing runs, and the coordinator asks the owner to install it: an install needs `sudo`, which is
the owner's.

**The self-test.** Before each sitting's first case, in a namespace made as above, the harness
checks eleven things, and nothing runs unless all eleven hold: the namespace lists loopback alone;
a TCP connection to `192.0.2.1:443` and one to `[2001:db8::1]:443`, both addresses reserved for
documentation, each fail at once as unreachable; a lookup of `probe-205-selftest.invalid` through
the system's resolver reaches the DNS logger; `/etc/resolv.conf`, read inside the namespace, holds
exactly the line `nameserver 127.0.0.1`; the namespace's host name is `probe-205`; each client's
login path, `.claude/.credentials.json`, `.claude.json` and `.codex/auth.json` relative to the
real home, is absent, each checked by `stat` alone, never opened, and the home's path never
printed; nothing is mounted under `/mnt`; `/run/WSL` in the namespace is absent or empty; the
case's environment holds no `WSL_INTEROP`; a TCP listener on the root namespace's loopback cannot
be reached from the case namespace, since the node uses WSL's mirrored networking; and a search
of `/run`, `/tmp`, `/var/tmp` and `/dev/shm` in the case namespace finds no socket, the check
printing only the count.

**The DNS logger** listens on UDP `127.0.0.1:53`, records each query's name and type, and answers
every query with NXDOMAIN, so that no name resolves and no host but the stub is reached.

**The stub gateway** listens on `127.0.0.1:18205`, speaks HTTP/1.1, answers each request within a
second, and closes the connection after each answer. In mode `text` it answers:

- `POST /v1/messages`, its query aside: as Anthropic's Messages API, a stream of server-sent
  events when the body's `stream` is true, otherwise one JSON message; either way the text `probe`;
- `POST /v1/messages/count_tokens`: `{"input_tokens": 1}`;
- `POST /v1/responses`: as OpenAI's Responses API, a stream of server-sent events, the text
  `probe`;
- anything else: 404, with a JSON error body.

In mode `tool`, a model request whose body declares at least one tool and carries no result for
the stub's own call is answered with a streamed call to the tool `conduit_probe_tool`, which
neither client has, with the arguments `{"marker": "probe-205"}`; every other request is answered
as in mode `text`. In mode `unauthorized`, every request is answered 401, with a JSON error body in
the shape of the API that its path names.

The streams are fixed here. For the Messages API the events are, in order: `message_start`, whose
message has the `id` `msg_probe_1`, the request's `model`, and a `usage` of input and output tokens
1; `content_block_start` at index 0; one `content_block_delta`; `content_block_stop`;
`message_delta`, with its `stop_reason` and output tokens 1; and `message_stop`. For text, the
block is `{"type": "text", "text": ""}`, its delta a `text_delta` of `probe`, and the stop reason
`end_turn`. For the tool call, the block is a `tool_use` with the `id` `toolu_probe_1`, the `name`
`conduit_probe_tool` and an empty `input`, its delta an `input_json_delta` whose `partial_json` is
the arguments, and the stop reason `tool_use`. For the Responses API the events are, in order:
`response.created`, whose response has the `id` `resp_probe_1`; `response.output_item.added`; one
delta; `response.output_item.done`, with the whole item; and `response.completed`, with that `id`
and a `usage` of input tokens 1, output tokens 1 and total tokens 2. For text, the item is a
`message` of role `assistant` with the `id` `msg_probe_1`, whose content is one `output_text` of
`probe`, and the delta a `response.output_text.delta`. For the tool call, the item is a
`function_call` with the `id` `fc_probe_1`, the `call_id` `call_probe_1` and the `name`
`conduit_probe_tool`, and the delta a `response.function_call_arguments.delta` holding the
arguments; the item added has empty `arguments`, and the item done has them whole.

**The helper.** The harness's binary, run as `helper ID CASE`, prints the fake token named `ID` of
the case `CASE`, and appends one line, the time and `ID`, to that case's helper log. It is Claude
Code's `apiKeyHelper` and Codex's auth command in the cases that use them.

**Fake tokens.** At the start of each case the harness makes its fake tokens, each `probe-205-`
followed by 32 hexadecimal digits from the system's random source, and keeps them in the case's
directory under `target/`, with mode 0600. A token is named everywhere by its ID, T-A to T-D. Its
value is never printed, and never written to a file that is committed.

## The cases

Each case runs once, in a home of its own under `target/scratch/`, new, with mode 0700 and empty
but for the file its row names; its working directory is a new, empty directory beside that home.
Its environment holds exactly `HOME`, `TMPDIR` (a directory of the case), `PATH` (the directory of
the `node` that installed the clients, then `/usr/bin` and `/bin`), and the variables its row
names. Claude Code runs as `claude -p "Reply with the word probe." --output-format stream-json
--verbose`. Codex runs, with `CODEX_HOME` set to the directory `.codex` in the case's home, as
`codex exec --json --skip-git-repo-check "Reply with the word probe."`. Neither is given a model:
each request records the model that the client chose. Claude Code's `ANTHROPIC_BASE_URL` is
`http://127.0.0.1:18205`, and Codex's provider's `base_url` is `http://127.0.0.1:18205/v1`. A case
that has not ended after 120 seconds is killed and recorded as a timeout.

| Case | Client | The gateway's credential | Stub mode |
| --- | --- | --- | --- |
| CC-1 | Claude Code | `ANTHROPIC_BASE_URL`, and `ANTHROPIC_AUTH_TOKEN` set to T-A | text |
| CC-2 | Claude Code | `ANTHROPIC_BASE_URL`, and `apiKeyHelper` in the home's `.claude/settings.json`, the helper giving T-B | text |
| CC-3 | Claude Code | `ANTHROPIC_BASE_URL` alone: the silent-login case | text |
| CC-4 | Claude Code | as CC-1 | unauthorized |
| CC-5 | Claude Code | as CC-2 | unauthorized |
| CC-6 | Claude Code | as CC-1 | tool |
| CX-1 | Codex | the provider's `env_key = "PROBE_GATEWAY_TOKEN"`, that variable set to T-C | text |
| CX-2 | Codex | the provider's auth command, the helper giving T-D | text |
| CX-3 | Codex | no key and no auth command: the silent-login case | text |
| CX-4 | Codex | as CX-1 | unauthorized |
| CX-5 | Codex | as CX-2 | unauthorized |
| CX-6 | Codex | as CX-1 | tool |

Every Codex case's `config.toml` in `CODEX_HOME` sets `model_provider = "probe-gateway"` and holds
one table, `[model_providers.probe-gateway]`, with `name = "probe gateway"`, the `base_url` above,
`wire_api = "responses"`, `requires_openai_auth = false`, and the credential its row names, and
nothing else. The keys of an auth command are those that Codex's source at the tag `rust-v0.167.1`
names: order 205b reads them there and quotes them, with path and line, in `HARNESS.md` before any
run. If that source holds no auth command for a provider, CX-2 and CX-5 do not run, and are
recorded as "no auth command at 0.167.1", which refutes K3's auth command.

## What is recorded

For each case, in a reduced record that is committed:

- **Each request the stub received**: its order and its time from the case's start; its method;
  its path; the names of its query's keys; the names of its headers, in order; for each header
  whose value holds a fake token, the token's ID and whether it came alone or after `Bearer `; the
  values of the headers `content-type`, `accept`, `user-agent`, `anthropic-version`,
  `anthropic-beta`, `openai-beta`, `originator` and those beginning `x-stainless-`; and for every
  other header, its value's length only. From a JSON body: the values of `model` and `stream`, its
  top-level key names, the number of its `messages` or `input` items and of its `tools`, and
  whether it carries a result for the stub's call (`toolu_probe_1` or `call_probe_1`), with that
  result's `is_error` where the Messages API gives one. A body sent compressed is recorded by its
  encoding alone.
- **Each DNS query** the logger received: its name and its type. A name is recorded whole, a
  login's host included, since which hosts a client tries to reach is Q1's question; a host name is
  public, never an account value.
- **Each connection attempted**, from the trace: the address family, the address and port, and
  the result, with an error's name. For a Unix socket, the last part of its path only.
- **Each program run**, from the trace: the last part of its path only.
- **The helper's runs**: their number, and each run's time and token ID.
- **The client's end**: its exit status, or the timeout; and the first 400 characters of its
  standard error and of the last line of its standard output, with every absolute path replaced
  by `PATH`.

Never recorded, in a committed file or in a message: a token's value, the probe's fakes included;
the text of a request's body; an absolute path, a user name or a machine name; the value of a
header that the list above does not name. The raw files (the stub's and the logger's full logs,
the trace, the client's whole output) stay under `target/` and are never committed. The stub's raw
log keeps only the length of any credential-header value that is not one of the case's fake
tokens, so that no real token is ever written to disk. Before anything is committed, the
private-path check passes on it, run on the Linux node through
`tools/gate.py --derive-private-names`, or as `tools/check_private_paths.py` given the derived
names; a search of it for the node's host name finds none; and a search of it for each fake
token's value finds none.

## Predictions

Written before the harness and before any run, from the study and the clients' documentation as
the study read it:

| Case | Predicted |
| --- | --- |
| CC-1 | Each model request is `POST /v1/messages`, with `authorization` carrying T-A after `Bearer ` and no `x-api-key`; exit 0 |
| CC-2 | Each model request carries T-B in `x-api-key`, in `authorization` after `Bearer `, or in both; the helper runs at least once; exit 0 |
| CC-3 | No model request reaches the stub; the client exits non-zero, saying that it is not logged in |
| CC-4 | One model request, carrying T-A as in CC-1; exit non-zero, reporting an authentication failure |
| CC-5 | The helper runs again after the 401 and the request is sent once more: two model requests, at least two runs of the helper; exit non-zero |
| CC-6 | A later model request carries a result for `toolu_probe_1`, with `is_error` true, and T-A as in CC-1; exit 0 |
| CX-1 | Each model request is `POST /v1/responses`, with `authorization` carrying T-C after `Bearer `; exit 0 |
| CX-2 | As CX-1, with T-D; the helper runs at least once; exit 0 |
| CX-3 | Model requests reach the stub with no `authorization` header; no login starts; exit 0 |
| CX-4 | One model request, carrying T-C as in CX-1; exit non-zero, reporting an authentication failure |
| CX-5 | The helper runs again after the 401 and the request is sent once more: two model requests, at least two runs of the helper; exit non-zero |
| CX-6 | A later model request carries a result for `call_probe_1`, and T-C as in CX-1; exit 0 |

And in every case: no browser opens and no login starts; no credential header (`authorization`,
`x-api-key`, `proxy-authorization`, `cookie` or `api-key`) carries a value that is not one of the
case's own fake tokens; and each client, in at least one of its cases, looks up a name other than
the stub's, which reaches nothing.

## What each outcome means

- **Q2.** As predicted: the gateway can take each client's credential from the header seen, and
  the route's design can name that header for each client. Otherwise the design names the header
  seen, and K1 or K3 is refuted for each form of credential that sends no token.
- **Q3.** No credential header in CC-3 or CX-3 shows that a fresh home gives the gateway nothing. It
  does not test K2 or K4, which need a saved login (see Not shown). A credential header with a
  value in either case is stop rule 2.
- **Q4.** A helper or auth command run again after a 401 means that the route can renew an Agent's
  gateway credential with no restart of its client; none means that the design must state a
  restart for a renewal. A lookup, a connection or a login after a 401 that the same client's other
  cases do not show goes to the owner with the results.
- **Q5.** A result for the stub's call sent back means that a gateway route carries a client's tool
  use unchanged. If a client rejects the stub's stream, the stream is compared first with the API's
  public documentation: a fault of the stub is fixed by a dated amendment here, and the case runs
  once more; a well-formed stream that a client rejects is recorded as the client's.
- **Q1.** The names and connections other than the stub's are the egress, beyond the gateway, that
  the route's design must allow or refuse for each client. A lookup of a vendor's model service in
  a case with a base URL set means that the client does not send all its model traffic to the base
  URL, which the design must answer.
- **K4.** CX-3 shows only that, with `requires_openai_auth = false` and no login present, no login
  starts and no credential is sent. K4 itself is not tested here (see Not shown). A login started
  or a credential header in CX-3 still refutes it.

## Stop rules

1. The self-test fails, or a namespace cannot be made by `unshare` or by `bwrap`: nothing runs. No
   case ever runs outside the namespace, on the Mac, or on Windows outside WSL.
2. A credential header (`authorization`, `x-api-key`, `proxy-authorization`, `cookie` or
   `api-key`) carries a value, not empty, that is not one of the case's own fake tokens: every run
   stops; the record keeps the header's name and the case, never the value; and the coordinator
   tells the owner before anything else runs. A value is empty when, after trimming, it is empty or
   only `Bearer`; an empty value is recorded under Q3, and this rule does not apply to it.
3. A program run whose name is a browser opener or a shell of Windows (`xdg-open`,
   `sensible-browser`, `x-www-browser`, `www-browser`, `wslview`, `open`, `explorer.exe`,
   `cmd.exe`, `powershell.exe` or `pwsh`), or any program under `/mnt/`: that client's remaining
   cases do not run, and it is recorded and goes to the owner.
4. A client's `--version` inside the namespace prints another version: none of that client's
   cases runs.
5. A timeout: the case is recorded as one, and runs again at most once, after a fault of the
   harness is fixed and recorded in `HARNESS.md`.
6. A client rejects the stub's stream: Q5's rule above, with at most one rerun of each case.
7. A fake token's value, an absolute path, a user name or a machine name found in a file about to
   be committed: nothing is committed until the record is reduced again, and it is a fault of the
   harness.
8. `command -v strace`, or `command -v bwrap` when the alternative is needed, finds nothing on the
   node: nothing runs, and the coordinator asks the owner to install it.

## Not shown

- K2 itself, whether a saved login's token is sent: every home here is new and holds no login. A
  case with a planted fake login needs a dated amendment here before it runs.
- K4 itself, whether `requires_openai_auth = false` keeps a saved ChatGPT login off: every Codex
  home here holds no login. A case with a planted fake ChatGPT login needs a dated amendment here
  before it runs.
- A real gateway, TLS, a vendor's service, a real model, or the vendors' terms.
- What a host other than the stub would be sent: every lookup fails, so nothing but the stub is
  reached.
- Interactive use, the clients on the Mac or on Windows, and any other version.
- What a client reads from a session bus, a keyring, a name cache, a managed setting, the real
  home, a socket under `/run`, `/tmp`, `/var/tmp` or `/dev/shm`, or the Windows drives: the
  namespace hides them.
- Agents kept apart: one operating-system user runs every case.

## Where the results go

The run, order 205c, writes its results at the end of this README, under a heading
`## Results YYYY-MM-DD` naming the day it ran, each case against its prediction and each question
answered, with the reduced record beside this README as `records-YYYY-MM-DD.json`.
