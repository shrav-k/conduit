# Model access through a gateway: the two clients, existing gateways and the route

> Research; it does not bind. Written on 2026-11-18 as the record of the grounded study that
> [ruling 83](../proposals/ruling-83/README.md#the-decisions), point 3, has run before a
> model-access route is designed. It condenses the coordinator's study of that day.

Ruling 83, point 2, sets a direction: an Agent never holds a model service's API key; model API
access comes later as a declared route, a credential-injecting gateway that a provider runs and to
which the Agent's client is pointed, which Conduit opens, limits and closes per Agent. Point 3 asked
for each client's gateway support at the pinned versions, existing gateways, the route's place in
Conduit's model, and the terms where the sources state them. The study decides nothing and gates
nothing in 0.2.0; the route's design, with this record and the probe's results, comes to the owner.

**Method.** Every statement below is a reading of public documentation or public source, made on
2026-11-18, not a run: no client or gateway ran, nothing was built, no one signed in and no key was
used. Each claim names its source, marked **doc** (documentation, unversioned, read on that date and
cited by section) or **src** (source at the tag and commit named, cited by path and line). A binary
can differ from its tag's source, and a page can change after it was read. A sentence that begins
**Inference** is reasoning, the study's or this record's, not what a source says.

## Sources

| ID | Kind | Source | Where | Version or date |
| --- | --- | --- | --- | --- |
| CC1 | doc | Claude Code, "Other LLM gateways" | <https://code.claude.com/docs/en/llm-gateway> | read 2026-11-18 |
| CC2 | doc | Claude Code, "Connect Claude Code to an LLM gateway" | <https://code.claude.com/docs/en/llm-gateway-connect> | read 2026-11-18 |
| CC3 | doc | Claude Code, "Claude Code gateway compatibility guide" | <https://code.claude.com/docs/en/llm-gateway-protocol> | read 2026-11-18 |
| CC4 | doc | Claude Code, "Authentication" | <https://code.claude.com/docs/en/authentication> | read 2026-11-18 |
| CC5 | doc | Claude Code, "All settings" | <https://code.claude.com/docs/en/settings-reference> | read 2026-11-18 |
| CC6 | doc | Claude Code, "Environment variables" | <https://code.claude.com/docs/en/env-vars> | read 2026-11-18 |
| CC7 | doc | Claude Code, "Settings files and precedence" | <https://code.claude.com/docs/en/settings> | read 2026-11-18 |
| CC8 | doc | Claude Code's `CHANGELOG.md`, repository `anthropics/claude-code` | <https://github.com/anthropics/claude-code/blob/0bd56cf67610ac307b8c8d2a64e95cb1ec6a381d/CHANGELOG.md> | commit `0bd56cf67610` |
| CX1 | src | Codex, paths under `codex-rs/` | <https://github.com/openai/codex/tree/rust-v0.167.1> | tag `rust-v0.167.1`, commit `0a94398fa60c` |
| CX2 | doc | Codex, "Advanced configuration", "Custom model providers" | <https://learn.chatgpt.com/docs/config-file/config-advanced> | read 2026-11-18 |
| CX3 | doc | Codex, "Connect to a gateway" | <https://learn.chatgpt.com/docs/enterprise/connect-to-a-gateway> | read 2026-11-18 |
| CX4 | doc | Codex, "Gateway compatibility requirements" | <https://learn.chatgpt.com/docs/enterprise/gateway-compatibility> | read 2026-11-18 |
| LL1 | src | LiteLLM, `litellm/proxy/auth` and `litellm/proxy/pass_through_endpoints` only | <https://github.com/BerriAI/litellm/tree/v1.110.0> | tag `v1.110.0`, commit `096ff016817c` |
| LL2 | doc | LiteLLM, "Virtual Keys", in part | <https://docs.litellm.ai/docs/proxy/virtual_keys> | read 2026-11-18 |
| LL3 | doc | LiteLLM, "Codex CLI" client setup, in part | <https://docs.litellm.ai/docs/proxy/client_setup/codex_cli> | read 2026-11-18 |
| EV1 | src | Envoy, three API protos: `credential_injector.proto`, `generic.proto`, `api_key_auth.proto` | <https://github.com/envoyproxy/envoy/tree/v1.40.2> | tag `v1.40.2`, commit `b2d89e0ae539` |
| EV2 | doc | Envoy, credential injector and API key auth filters | <https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/credential_injector_filter> | pages for 1.41.0-dev, read 2026-11-18 |
| AT1 | doc | Anthropic Commercial Terms of Service, in part | <https://www.anthropic.com/legal/commercial-terms> | effective 2025-06-17, read 2026-11-18 |
| OA5 | doc | OpenAI API, "Production best practices", in part | <https://developers.openai.com/api/docs/guides/production-best-practices> | read 2026-11-18 |
| E010 | record | [Experiment 010's second reading](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/experiments/010-subscription-terms/results-2026-11-16.md), cited by its IDs AN1, AN2, OA3 | this repository | 2026-11-16 |

Claude Code's source is not public, so every claim about it is documentation, which is at or before
2.1.318 in each version it names; CC8 is its only versioned record. 2.1.318 is the baseline of the
[Claude Code Agent package](../../packages/agent-claude-code/README.md); CX1 is the tag that
[experiment 027](../../experiments/027-a5-sign-in/README.md) read. OpenAI's Terms of Use and its
page on API key safety answered HTTP 403 and were not read.

## Claude Code 2.1.318

- **Base URL** (doc CC6, `ANTHROPIC_BASE_URL`; doc CC3, "API formats"). The variable routes requests
  through a proxy or gateway, which must serve the Anthropic Messages format, `/v1/messages`, with
  `/v1/messages/count_tokens` optional.
- **The gateway's credential**, in one of three settings (doc CC2, "Set the credential variable";
  doc CC4, "Authentication precedence"): `ANTHROPIC_AUTH_TOKEN`, sent as `Authorization: Bearer`;
  `ANTHROPIC_API_KEY`, sent as `X-Api-Key` and in `-p` mode always used when present; or
  `apiKeyHelper`, a settings key holding a command line whose output is sent "as both the
  `X-Api-Key` and `Authorization: Bearer` headers" (doc CC5, `apiKeyHelper`). The helper's output is
  cached for five minutes by default (`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`) and the command runs
  again when a request fails with 401 or 403, and before a request when a cached JWT has expired
  (2.1.246 or later), only where `ANTHROPIC_AUTH_TOKEN` is not set (doc CC5). The subscription login
  comes last in precedence (doc CC4).

- **Where the settings go** (doc CC2, "Set in a settings file"; doc CC7; doc CC5, `env`). A settings
  file's `env` block carries both variables and wins over the shell; `--settings` can set any user
  key. The Claude package already launches the client with `--setting-sources ""` and `--settings`
  (`packages/agent-claude-code/src/lib.rs`). **Inference:** a route needs new keys in the file the
  package writes, not a new launch mechanism.
- **A gateway credential replaces the subscription** (doc CC1, "Subscriptions and gateways"):
  requests "carry that credential in place of a developer's claude.ai subscription login", and the
  saved login is kept but not sent.
- **A base URL alone does not** (doc CC1, same section; doc CC3, "Request headers", row
  `Authorization`). With no gateway credential and a saved claude.ai login, "Requests still route
  through the gateway, but a saved claude.ai login remains the active credential", and
  `Authorization` carries the login's token. This is the claim that
  [order 205](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/205-gateway-probe.md) states as K2 for experiment 032.
  **Inference:** a route Agent's home must hold no claude.ai login, or a missing route credential
  would send the login's token to the gateway with no error.
- **A failing helper is a failure** (doc CC4, "Credential management"): a helper that exits with an
  error, times out or prints nothing makes requests fail "within three attempts"; CC2's
  troubleshooting says they "carry a placeholder key". Whether 2.1.318 ever falls back to a saved
  login is not stated.
- **Traffic outside the gateway** (doc CC2, "Turn off traffic outside the gateway path"; doc CC3,
  "Optional endpoints and startup traffic"). Nonessential traffic goes to Anthropic and third
  parties unless `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, which also disables auto-updates; the
  fast mode check and the WebFetch domain check still call `api.anthropic.com`.
- **Leaks the changelog records as fixed before the pin** (doc CC8, 2.1.246 and 2.1.299): several
  requests to Anthropic had carried a gateway's key; "a credential is now only sent to its own
  host". The changelog is not a run: Q1 of experiment 032 is to confirm that at 2.1.318 the route
  token reaches only the gateway's host.
- **What a gateway must do** (doc CC3, "Streaming", "Request headers", "Response headers"): stream
  server-sent events as they arrive, through `message_delta` and `message_stop`, with `ping` events;
  forward `anthropic-version` and `anthropic-beta` unchanged, as open lists; return
  `text/event-stream`, an integer `retry-after`, and error bodies.

## Codex 0.167.1

All under CX1 unless another source is named.

- **The provider table and its one wire format** (src `model-provider-info/src/lib.rs:112-237`; doc
  CX4, "Requests and endpoints"). A provider has `name`, `base_url`, `env_key`, `auth`, `wire_api`,
  `env_http_headers` and `requires_openai_auth`, among others, and denies unknown fields; `wire_api`
  takes only `responses`, the Responses API at `/v1/responses`.
- **A provider of its own ID** (src `config/src/config_toml.rs:176-177`, `948-978`;
  `model-provider-info/src/lib.rs:788-848`). `model_provider` names it; built-in IDs cannot be
  redefined. The built-in provider's `openai_base_url` (src `config/src/config_toml.rs:435`) keeps
  `requires_openai_auth: true` (src `model-provider-info/src/lib.rs:568-614`). **Inference:** that
  would send the home's own credential to the base URL, so it is not the route.
- **How a credential is chosen** (src `model-provider/src/auth.rs:250-309`, `379-391`): first the
  variable `env_key` names, else `experimental_bearer_token`; then, where `requires_openai_auth` is
  false and there is no `auth` table, no credential and no header; only otherwise the home's login.
  An `env_key` whose variable is missing or empty is an error naming it, not a fallback (src
  `model-provider-info/src/lib.rs:519-538`).
- **An auth command** (src `model-provider/src/auth.rs:260-283`,
  `login/src/auth/manager.rs:2508-2514`, `protocol/src/config_types.rs:563-590`; doc CX2). A
  provider with an `auth` table gets its own manager, whose home is `"non-existent"`, so it never
  reads the home's login. The table holds `command`, `args`, `timeout_ms` (5,000 by default) and
  `refresh_interval_ms` (300,000 by default; 0 reruns only after a 401). The command gets no
  standard input and prints the token. `auth` cannot be combined with `env_key`, a bearer token or
  `requires_openai_auth` (src `model-provider-info/src/lib.rs:409-435`).
- **The ChatGPT login** (src `model-provider/src/provider.rs:421-427`, `514-546`). Only a provider
  that requires OpenAI auth and has no other credential takes the first-party path that resolves it.
  With `requires_openai_auth = false`, the default, it is not sent: the claim order 205 states as
  K4. CX3 also documents ChatGPT sign-in through a gateway ("Enable ChatGPT sign-in"); that is a
  subscription passing through a gateway, outside ruling 83's direction.
- **Set by `-c` alone** (src `exec/src/cli.rs:44-46`, `utils/cli/src/config_override.rs:20-102`,
  `config/src/loader/mod.rs:86-103`, `256-271`, `427-433`). Under `--ignore-user-config` the user's
  `config.toml` is not loaded, but auth still uses `CODEX_HOME`; `-c` values form a session layer;
  `model_provider` and `model_providers` are denied only to project-local config. The Codex package
  already launches with `--ignore-user-config`, `--strict-config` and `-c` settings
  (`packages/agent-codex/src/lib.rs`). Two limits: system and managed config still load, and
  "Managed config wins over CLI config" (src `config/src/loader/mod.rs:357-358`); and a `-c` value
  is on the command line, so the token belongs in the `env_key` variable or the command's output
  (doc CX3, "Use an API key": "Don't put the credential in TOML").
- **What the client still holds.** The gateway token, in its environment or in memory. The default
  shell environment policy passes every variable on, `*TOKEN*` and `*KEY*` included (src
  `protocol/src/config_types.rs:232-268`, `protocol/src/shell_environment.rs:138-145`); the Codex
  package disables its shell tools. MCP servers and other services "can have separate network paths
  and credentials" (doc CX4, "Data and tool boundaries").
- **What a gateway must do** (doc CX4): accept `POST /v1/responses`; forward server-sent events
  incrementally, through `response.completed`; keep function-call items and their outputs;
  "Authenticate developers separately from the gateway's upstream provider identity"; and route "the
  exact model name recognized by your deployed Codex version". LL3 raises `stream_idle_timeout_ms`
  and `stream_max_retries` for long turns.

## Existing gateways and their per-caller checks

| | LiteLLM proxy (LL1, LL2) | Envoy (EV1, EV2) | A gateway of Conduit's own |
| --- | --- | --- | --- |
| Per-caller check | Virtual keys with models, budgets, RPM and TPM; made at `/key/generate`, blocked at `/key/block` (doc LL2) | API key auth filter: keys mapped to client names, routes limited by `allowed_clients`; `work_in_progress` at v1.40.2 (src `api_key_auth.proto:15-16`, `40-128`) | Whatever the route issues |
| Injection | From the proxy's configuration; "The LiteLLM virtual key is not forwarded to Anthropic" (src `llm_passthrough_endpoints.py:2889-2895`) | Credential injector, `ACTIVE`, any header with a prefix; `overwrite` defaults to false (src `credential_injector.proto:14-74`, `generic.proto:19-40`) | To be written |
| A caller's credential sent on | Possible: with no Anthropic credential on the proxy, the pass-through accepts the caller's own key or OAuth token (src `llm_passthrough_endpoints.py:2889-2898`) | Not with `overwrite` set true | Refused by design |
| Formats | `/v1/messages`, `/v1/responses`, and pass-through routes (src `llm_passthrough_endpoints.py:1080-1094`, `3370-3407`) | Pass-through, any format | Pass-through |
| Needs | Postgres and a master key for virtual keys (doc LL2), and a Python service | Configuration, a secret source, a reload or control plane to add a key | One binary |

Envoy's injector "does not handle end user authentication" (src `credential_injector.proto:20-29`),
and whether it buffers a stream was not read. CC1 notes that any gateway "needs to be kept updated
as Claude Code evolves". **Inference:** the function needed is narrow (check a per-Agent token,
refuse any other credential, inject the vendor key, stream unchanged, read usage from the final
events), and LiteLLM's pass-through could carry a subscription token upstream on the
base-URL-only path above where the proxy holds no Anthropic credential of its own, so the study
leans to a gateway of Conduit's own, in Rust, which is the owner's choice.

## How a route maps onto the specification as it stands

These are the study's readings of the specification, made at `e4ceb4c87e17`. For this record they
were checked again at `a701c1ee`, which changes PND-005, PND-010 and the description of
`sign-in.v1` but keeps the sentences relied on below, PND-005's null-account sentence among them;
that re-check is the record's, not the study's. None of the readings is decided.

- **A route is not a Conduit as Conduits work today.** Nothing governs model access now: IMP-001
  says when model work starts, not how it reaches a model. A Conduit permits the MCP operations it
  lists (CON-001, ACC-003); the engine is the only enforcement point (PRV-025); closing one ends all
  its access (ACC-011). A client's HTTP request to a gateway passes through no engine, and a token
  the client holds ends only when the gateway refuses it. **Inference:** the route needs new
  language for an enforcer other than the engine and for a close that is a call to it.
- **What is a provider, with no core change.** The gateway is a Resource, run by a runtime provider.
  Its vendor key reaches its run by the existing secrets path: a Resource may consume
  `get-secret.v1`, opened by root (SEC-008), read before the run and passed in its `secrets`
  (SEC-010), so the key stays with the gateway. Its Type, implementation and contract state its
  formats, how it checks a route token and injects the key, what it logs and keeps (as PRV-011 has a
  provider state where it keeps secrets), and where it must run.
- **Existing shapes that serve.** `get-secret.v1`'s Conduit permits no request its consumer sends,
  and the engine's reads go through it (SEC-007, SEC-008): the nearest shape for a route whose
  operations only the engine dispatches, to issue an Agent's token at setup and revoke it at close.
  SEC-013 and SEC-024 send a secret header only over https or loopback. IFC-018 sends a Conduit's
  filters to the target, as a route's budget and models could be. CTR-013 and PND-005 already treat
  a null `account` as no sign-in. NAM-024 lets a later release add an optional member with no
  `default`, such as a `modelRoute` on `claude-code.v2` and `codex.v2`.
- **What is new language** (the study's list, sized for batch 2): the route's form, with three
  options: (a) a Conduit for a new core capability whose operations only the engine dispatches, (b)
  a managed connection with conditions only, as ruling 80 point 8 has for sign-in under IFC-049, or
  (c) an engine-mediated proxy, which keeps PRV-025 true but puts every prompt through the engine;
  who may open one (root, at first); the route credential, issued per Agent, passed as SEC-010
  passes a secret and revoked at close; one declared form of model access per Agent; where the token
  may go; per-route usage records; the `.v2` member; and B13's text. The study leans to (a).
  **Inference:** IFC-049's "no process running Conduit's code ... terminates" cannot hold for a
  gateway that Conduit ships, so the route needs its own condition.
- **Confinement** (the specification's README, PRV-029; ruling 70). Under one operating-system user
  an unconfined Agent can read the gateway's key file and another Agent's route token.
  **Inference:** until Agents are confined, the key stays out of every Agent only where the gateway
  runs as another user, on another machine or hosted, and per-token records show which route was
  used, not which Agent used it.

## Terms

A reading of public documents, not legal advice. Anthropic documents the pattern: "the provider key
stays server-side; developers hold gateway credentials instead" (doc CC1, "What a gateway
provides"). API keys are its route for products using Claude, and its Consumer Terms bar automated
access except through an API key or where Anthropic otherwise explicitly permits it (E010, AN1 and
AN2, question 3). The Commercial Terms govern API keys, make the customer "responsible for all
activity under its account", and bar resale except as approved (doc AT1, D.4(a), D.5); AN2 bars
sharing an API key "with anyone else" (E010, question 1). **Inference:** an owner's gateway holding
the owner's key does not share it with another person. Subscriptions may not be routed or
intermediated by third-party developers (E010, AN1, question 4). **Inference:** so the base-URL-only
path matters, since a gateway that receives a claude.ai token, even by accident, is intermediating
it. The binary must not be modified and no built-in authentication method removed (E010, AN1); the
route uses documented settings only. The packages' removal of `ANTHROPIC_API_KEY` is the tension
that ruling 83 left [waiting for the owner](../AUDIT.md#waiting-for-the-owner); the route does not
change it. OpenAI calls API keys "the right default for automation" (E010, OA3), and asks gateways
to keep upstream credentials on the gateway (doc CX4) and applications to use a service account (doc
OA5). Nothing read limits how many Agents one key serves, beyond rate limits and the holder's
responsibility.

## What the probe is to settle

[Work order 205](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/205-gateway-probe.md) registered
[experiment 032](../../experiments/032-gateway-probe/README.md) (A85) on 2026-11-18, with its
harness and its run in orders 205b and 205c: the two pinned clients, fake tokens, a stub gateway in
Rust and a network namespace that holds loopback alone. As order 205 words them, it states the
study's claims K1 to K4, tests K1 and K3 against the clients themselves, and tests only the
fresh-home side of K2 and K4: which hosts each tries to reach (Q1), which header carries which
credential (Q2), what each sends with a base URL and no credential in a home with no login (Q3),
what each does on a 401, the helper's rerun included (Q4), and whether a streamed tool call passes
(Q5). By its own Not shown, it leaves untested K2 and K4 themselves, since no home holds a login; a
real gateway, TLS and the vendors; and the clients on other machines and versions. A85's row in
[the register](../../ROADMAP.md#experiments-and-probes) covers this study, which ran with no order,
and its Source cell links this record, as experiment 032's README does since its amendment of
2026-11-18.

## What stays open

- **2.1.318's own behavior**, documentation until experiment 032 runs; and K2 and K4, each needing
  a case with a planted fake login, by a dated amendment to experiment 032, as order 205's Not shown
  says. **Inference:** such a case could also show whether a failing helper falls back to a saved
  login; order 205 does not plan it.
- **Codex's model catalog through a custom provider at 0.167.1**: which request for models, if any,
  goes to the gateway, and whether a routed name must be one the pinned version knows.
- **An auth command beside `requires_openai_auth = false`.** Experiment 032's cases CX-2 and CX-5
  set both, which Codex may reject at load, as read above; order 205's Record leaves this to the
  coordinator as finding R205-2-2, for a dated amendment before order 205b.
- **The packages' own launch.** Order 205's cases configure each client by a home's files, not by
  the packages' `--settings` and `-c` mechanisms, and set no
  `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`.
- **The route's form** in the language, and whether closing it can be a revoke at the gateway
  without weakening ACC-011; who opens it; how it and `account` exclude each other, which ruling 83
  says a later `oneOf` cannot do.
- **Where the gateway runs** relative to Agents, and whether its URL is loopback, https on a private
  network or hosted; and what `account: null` means in `.v2` before a route exists.
- **The choice of gateway**: LiteLLM, Envoy or Conduit's own, read in source and documentation only;
  none was run, and LiteLLM's release history and advisories were not read.
- **Terms unread**: OpenAI's Terms of Use and Usage Policies (HTTP 403), and Anthropic's Usage
  Policy and Service Specific Terms for this question.
