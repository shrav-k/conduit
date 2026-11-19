# 032 — A85: the gateway probe

Every result of this experiment is obtained under one operating-system user, on one machine, with
fake tokens, against a stub gateway, inside a network namespace that holds loopback alone. So no
result here shows what a real gateway, a vendor's service or a real login does, or that Agents are
kept apart. It shows what each client, as installed, sends and tries to reach.

Pre-registered on 2026-11-18 by work order 205, before its harness was written and before any
run. Not run: the harness (order 205b) and the run (order 205c) come later, each after the one
before it merges. The text below changes only by a dated amendment, written before the run it
changes.

Amended on 2026-11-18, before order 205b is written, by the coordinator's change that commits the
study's record: under Why, the sentence saying that the record is not yet in the tree now links
it. Nothing else changes.

Amended on 2026-11-19, before order 205b's harness is written and before any run, by the
coordinator's change that writes order 205b, from the findings that order 205's Record leaves to
this amendment (R205-2-1, R205-2-2, R205-4, R205-5 and R205-6, and two of its verifiers' musts)
and from order 208's namespace and `--version` homes. Each change is made in place below:

1. **A PID namespace with its own `/proc`** (R205-2-1; order 208, which found that without one a
   process's `/proc` entries reach past every mask). The `unshare` line gains
   `--pid --fork --mount-proc --kill-child`, and the `bwrap` alternative
   `--unshare-pid --proc /proc`. The self-test checks twelve things, not eleven: the twelfth, that
   the case namespace's `/proc` lists no process outside it, printing only the count. "Not shown"
   adds another process's `/proc` entries to what the namespace hides.
2. **The auth command, read at the tag** (R205-2-2). The coordinator read Codex's validation at
   `rust-v0.167.1` and quotes it under "The cases": `auth` conflicts with `requires_openai_auth`
   only where that field is true, so CX-2 and CX-5 keep `requires_openai_auth = false`. Their
   `auth` table is fixed there, and a configuration that Codex rejects at load is a fault of the
   case, run once more as stated there, never a refutation of K3.
3. **The installs and `--version` outside the cases** (R205-4; order 208's `--version` homes). They
   no longer run with the real home reachable or with `HOME` unset: each runs in a namespace whose
   mask covers the real home, with a new scratch home, a `--version` with no network, as "The
   clients and the machine" states. There and in the method's step 2, where the `node` lies under
   the home, the installation it belongs to is bound back with its directory, so that `npm`, which
   links into that installation's `lib/node_modules/npm`, can start under the mask.
4. **One clock for the record** (R205-5). Every event recorded has its time from the case's start,
   on one clock, and the stub's answers are recorded, so that "after a 401" is settled by times;
   `strace` gains `-ttt`.
5. **What reaches the DNS logger** (R205-6). A name that `/etc/hosts` holds is answered from that
   file and never reaches the logger; the method's step 3, the DNS logger and "Not shown" now say
   so.
6. **The private-path check of new files** (order 205's first verifier). The check reads the files
   Git tracks, so it runs after `git add -N` of each new file.
7. **Empty headers, and cases with no request** (order 205's verifiers). An empty credential header
   in CX-3 refutes nothing under K4, as it triggers nothing under stop rule 2; and a case that
   sends the stub no model request refutes nothing under Q2.

The coordinator also read, at the tag, the events Codex's stream parser acts on (order 205's
second verifier): it acts on `response.created`, `response.output_item.added`,
`response.output_text.delta`, `response.output_item.done` and `response.completed`, and logs and
skips, among others, `response.in_progress`, `response.content_part.added`,
`response.content_part.done`, `response.output_text.done`,
`response.function_call_arguments.delta` and `response.function_call_arguments.done`
(`codex-rs/codex-api/src/sse/responses.rs:386-559`); a stream that closes before
`response.completed` is an error (`:617-618`). So the fixed Responses streams below stay as they
are, and Q5's rule stands for any rejection. The questions, the predictions and the stop rules do
not change; the twelve cases change only as item 2 states, and the meanings of the outcomes only
as item 7 states.

## Why

[Ruling 83](../../docs/proposals/ruling-83/README.md), point 2, sets a direction: an Agent never
holds a model service's API key, and model API access comes later as a declared route, a
credential-injecting gateway that a provider runs, to which the Agent's client is pointed instead
of the vendor. Its point 3 has a grounded study read each client's gateway support before the route
is designed. The study recommended this probe, so that what it read is seen in the clients
themselves. The study's record, linked here by the amendment of 2026-11-18 above, is
[Model access through a gateway](../../docs/research/model-access-gateways.md); A85's Source cell
links it too. The study has no register row of its own: it ran with no order, so ruling 83's
planned row for it is taken up by A85, by the coordinator's reading. The claims under test, K1 to
K4, are the study's statements, not facts:

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
  it runs outside the case namespace, before any case, as the next item states, with `HOME` and the
  npm cache in a scratch directory under `target/`, and with `--ignore-scripts`. If a client's
  `--version`, run outside the case namespace as the next item states, does not then print its
  version, that client is installed once more with its script, and the record says so, with the
  hosts its script reached if they are known. A client whose `--version` still prints nothing after
  the scripted install runs no case. The record keeps each package's integrity from the lockfile.
  Inside the case namespace, each client's `--version` must print its version before any of its
  cases runs, with its environment as a case's, in a new home under `target/scratch/` that is
  removed after it.
- **Outside the cases** (amended on 2026-11-19, item 3). Each install, and each `--version` run
  before the cases, runs outside the case namespace but never with the real home in reach: in a
  namespace of its own, made by
  `unshare --user --map-root-user --mount --pid --fork --mount-proc --kill-child`, or by `bwrap` as
  the method's alternative where `unshare --user` fails, in which an empty `tmpfs` masks the
  running user's real home, and this experiment's directory, the directory of the `node` that
  installed the clients and, where it lies under the home, the installation it belongs to (the
  parent of that directory, which holds `lib/node_modules/npm`) are bound back under it as the
  method's step 2 binds them. Nothing else is masked. A `--version` run adds
  `--net`, so that it reaches no network. An install keeps the node's network, which it needs, and
  so does a scripted install, whose scripts then reach the network but not the real home. Each runs
  under `env -i`, with `PATH` as in the cases (the directory of `node`, then `/usr/bin` and `/bin`);
  `HOME` the install's scratch directory, or, for a `--version`, a new directory under
  `target/version-home/`; `TMPDIR` a new directory beside that `HOME`; and, for each step of Codex,
  `CODEX_HOME` the directory `.codex` in that `HOME`. None runs with the real home, nor with `HOME`
  unset, nor a step of Codex with `CODEX_HOME` unset: every start of Codex's binary, `--version`
  included, reads `CODEX_HOME`'s `.env` and makes `tmp/arg0-linux/` there before it parses its
  arguments, and with `CODEX_HOME` unset that is the home's `.codex` (order 208's reading of the
  tag). Each `--version` home is removed after its run.
- **The Linux node only**: the second machine's WSL Ubuntu 24.04, on x86_64. Never the Mac, whose
  clients may read the login keychain, and never Windows outside WSL.

## The method

The harness is a Rust crate in this directory, `harness/`, that uses Rust's standard library
alone, so that it builds with no crate fetched. Order 205b writes it, with its tests, and
`HARNESS.md`, the method as built, before any client runs. It has five parts.

**The namespace.** The cases run as the Linux node's ordinary user that runs the work orders, with
no privilege; its name is never recorded. Each case runs inside a namespace of its own, made by
`unshare --user --map-root-user --net --mount --uts --pid --fork --mount-proc --kill-child`, with a
PID namespace and a `/proc` of its own (amended on 2026-11-19, item 1), so that no process outside
it, and no root, working directory or environment of one, can be reached through `/proc`. If
`unshare --user` fails on the node, the registered alternative is
`bwrap --unshare-user --unshare-net --unshare-uts --unshare-pid --proc /proc`, with the same masks,
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
   mask, this experiment's directory is bind-mounted back if it lies under the home, and so are the
   directory of the `node` that installed the clients and, where it lies under the home, the
   installation it belongs to (the parent of that directory, which holds `lib/node_modules/npm`).
   Nothing else is masked or bound back: the list is fixed here, not chosen by the harness;
3. bind mounts cover `/etc/resolv.conf` with a file whose one line is `nameserver 127.0.0.1`,
   `/etc/hosts` with one that holds only the loopback names and the line `127.0.0.1 probe-205`,
   and `/etc/nsswitch.conf` with a copy of itself whose `hosts:` line is `hosts: files dns`, so
   that every lookup of a name that this `/etc/hosts` does not hold reaches the logger; a name it
   holds is answered from the file and never reaches the logger (amended on 2026-11-19, item 5).
   Where `/etc/resolv.conf` is a symbolic link whose target lies under a mask of step 2, the
   target's path is made inside that mask and the one-line file is written there, with no mount;
   where its target lies elsewhere, the file is bind-mounted over the target;
4. the harness's DNS logger and stub gateway start;
5. the client starts in a nested user namespace that maps the user's own numeric ID, so that it
   runs with no capability, under `strace -f -qq -ttt -e trace=connect,execve`, with its environment
   cleared as `env -i` clears it and then set to exactly the case's variables.

**The tools.** Before anything runs, `command -v strace` must find `strace` on the node, and,
when the alternative is needed, `command -v bwrap` must find `bwrap`. If one finds nothing,
nothing runs, and the coordinator asks the owner to install it: an install needs `sudo`, which is
the owner's.

**The self-test.** Before each sitting's first case, in a namespace made as above, the harness
checks twelve things, and nothing runs unless all twelve hold: the namespace lists loopback alone;
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
printing only the count; and the case namespace's `/proc` lists no process outside it: `/proc/1`
and every numbered entry of `/proc` has an `NSpid` line in its `status` that holds its own number
alone, as order 208's probe checks it, the check printing only the count of entries that do not
(amended on 2026-11-19, item 1).

**The DNS logger** listens on UDP `127.0.0.1:53`, records each query's name, type and time, and
answers every query with NXDOMAIN, so that no name it is asked resolves; the names that
`/etc/hosts` holds are loopback names, so no host but the stub is reached.

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

The coordinator read them at the tag, commit `0a94398fa60c`, on 2026-11-19 (amended on that day,
item 2), under `codex-rs/`:

- A provider's `auth` is `pub auth: Option<ModelProviderAuthInfo>`
  (`model-provider-info/src/lib.rs:178`), and `requires_openai_auth` is a `bool` under
  `#[serde(default)]` (`:223-224`), so an explicit `false` reads as the default does.
- The provider's `validate` returns at once where there is no `auth` (`:409-411`). Otherwise it
  refuses an empty `command`, and collects a conflict for `env_key` where it is set, for
  `experimental_bearer_token` where it is set, and for `requires_openai_auth` only where it is true:
  `if self.requires_openai_auth {` then `conflicts.push("requires_openai_auth");` (`:424-426`). Any
  conflict fails it with "provider auth cannot be combined with" and the conflicts' names (`:432`).
  Loading the configuration runs it for each provider, prefixing `model_providers.{key}: ` to its
  message (`config/src/config_toml.rs:1013-1015`).
- The auth table, `ModelProviderAuthInfo` (`protocol/src/config_types.rs:570-591`), holds
  `command` (`:572`), of which its comment says "Bare names are resolved via `PATH`; paths are
  resolved against `cwd`." (`:571`); `args` (`:576`), empty by default; `timeout_ms` (`:580`),
  5,000 by default (`:564`); `refresh_interval_ms` (`:585`), 300,000 by default (`:565`), of which
  its comment says "Set to `0` to disable proactive refresh and only rerun after a 401 retry path."
  (`:583`); and `cwd` (`:590`).

So CX-2 and CX-5 keep `requires_openai_auth = false`, and their credential is the table
`[model_providers.probe-gateway.auth]` holding `command`, the absolute path of the harness's
binary, and `args`, `["helper", "T-D", "CX-2"]` in CX-2 and `["helper", "T-D", "CX-5"]` in CX-5,
and no other key. Order 205b checks each quote above against the tag before it copies it into
`HARNESS.md`; a quote that differs there stops order 205b, which reports it. If Codex nonetheless
refuses a case's configuration at load, that is a fault of the case, never a refutation of K3: the
case is recorded so, and, where the refusal names `requires_openai_auth`, CX-2 and CX-5 each run
once more with that key left out, and the record says so.

## What is recorded

Every time recorded is in milliseconds from the case's start, read from one clock, the system's
real-time clock, which `strace -ttt` also prints; an event is after a 401 when its time is later
than the time at which the stub sent the case's first 401 (amended on 2026-11-19, item 4).

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
- **Each answer the stub sent**: the request it answered, its status, and the time it was sent.
- **Each DNS query** the logger received: its name, its type and its time. A name is recorded
  whole, a login's host included, since which hosts a client tries to reach is Q1's question; a
  host name is public, never an account value.
- **Each connection attempted**, from the trace: its time, the address family, the address and
  port, and the result, with an error's name. For a Unix socket, the last part of its path only.
- **Each program run**, from the trace: its time, and the last part of its path only.
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
names, after `git add -N` of each new file, since the check reads only the files Git tracks
(amended on 2026-11-19, item 6); a search of it for the node's host name finds none; and a search
of it for each fake token's value finds none.

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
  seen, and K1 or K3 is refuted for each form of credential that sends no token. A case that sends
  the stub no model request answers nothing under Q2 and refutes nothing; it is recorded so
  (amended on 2026-11-19, item 7).
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
  starts and no credential is sent. K4 itself is not tested here (see Not shown). A login started,
  or a credential header in CX-3 whose value is not empty as stop rule 2 defines it, still refutes
  it; an empty one is recorded under Q3 and refutes nothing (amended on 2026-11-19, item 7).

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
- What a host other than the stub would be sent: every lookup that reaches the logger fails, and
  the names that `/etc/hosts` holds are loopback names, so nothing but the stub is reached
  (amended on 2026-11-19, item 5).
- Interactive use, the clients on the Mac or on Windows, and any other version.
- What a client reads from a session bus, a keyring, a name cache, a managed setting, the real
  home, a socket under `/run`, `/tmp`, `/var/tmp` or `/dev/shm`, the Windows drives, or the `/proc`
  entries of a process outside the namespace (amended on 2026-11-19, item 1): the namespace hides
  them.
- Agents kept apart: one operating-system user runs every case.

## Where the results go

The run, order 205c, writes its results at the end of this README, under a heading
`## Results YYYY-MM-DD` naming the day it ran, each case against its prediction and each question
answered, with the reduced record beside this README as `records-YYYY-MM-DD.json`.
