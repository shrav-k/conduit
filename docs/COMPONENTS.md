# Components

Not binding: a one-page list for orientation, written 2026-11-08 against `main` at `2ca84f3`. The
[specification](../spec/README.md) alone binds, and its [definitions](../spec/definitions.md)
hold the full meaning of every term below.

Conduit is a configuration language for the access around AI agents. Engineers use it to declare
exactly what each agent can reach (files, folders, services like GitHub, other agents) and which
operations it may perform. Credentials stay in stores like 1Password or the Keychain, so agents
use services without ever seeing the secrets.

## What an author writes

- **`.yaml` documents**: templates, instances and contracts, each with `apiVersion`, `kind` and
  `metadata`, the way Kubernetes documents look ([objects](../spec/promises/objects.yaml)).
- **Manifest (`conduit.yaml`)**: one per project or package. It names the address, pins packages
  by version and digest, and lists the servers that do the work ([manifest](../spec/promises/manifest.yaml)).
- **Templates**: reusable definitions with typed inputs ([templates](../spec/promises/templates.yaml)).
- **Instances**: concrete objects, written as `from:` a template plus `arguments:`
  ([instantiation](../spec/promises/instantiation.yaml)).
- **Contracts**: five kinds, which are Type, Capability, Operation, Requirement and Schema
  ([contracts](../spec/promises/contracts.yaml)).

See [the worked project](../spec/schemas/examples/valid/project/) for all of these together.

## The five families

- **Agent**: a participant whose model decides its own actions, such as a coding agent.
- **Interface**: where a person or an outside system acts on the system.
- **Resource**: a shared thing, such as a folder, a service login, a secret or a provider.
- **Pool**: owns and groups Resources.
- **Conduit**: one consumer joined to one target, listing the operations it allows. Every
  permission is an open Conduit ([conduits](../spec/promises/conduits.yaml)).

## Access

- **Capability**: the list of operations a type offers. Offering one is not permission to use it.
- **Requirement**: what a Conduit's template applies to it: a bound, the shared-host requirement,
  or a filter that the target applies to each call.
- **`management` and `access`**: two built-in Resources. Managing objects, and opening and
  closing Conduits, are authorized against them ([access](../spec/conventions/access.yaml)).
- **Broker**: a Resource that gives an actor narrower management access than a Conduit can carry.
- **Gate**: stands before every server the engine does not run itself, and admits only requests
  the engine proves it sent, by mutual TLS or a signed token ([providers](../spec/promises/providers.yaml)).

## Running it

- **Engine**: the program that reads the documents and decides every request. None exists yet.
- **Implementations**: the MCP servers that do the work, reached by `command`, `package` or
  `remote`. A `plain` server is an existing one, used unchanged
  ([implementation interface](../spec/interface/implementation.yaml)).
- **Tools**: what the engine presents to an agent for each operation of its open Conduits.
- **Providers**: swappable infrastructure behind contracts: the runtime (such as Docker), the
  machines, package sources and secret stores (such as the Keychain or 1Password).
- **Nodes and placement**: where Agents and Resources run, on one machine or several
  ([placement](../spec/promises/placement.yaml)).
- **First start**: the root Interface and the built-in objects that exist before anything is
  submitted ([first start](../spec/promises/first-start.yaml)).

## Packages and projects

- **Package**: a manifest and its exports at an address, each release pinned by version and
  digest ([packages](../spec/promises/packages.yaml)).
- **Core package**: built into every engine: the management verbs, the provider capabilities and
  the reference schemas ([core](../spec/core/README.md)).
- **Project**: an author's own directory of documents, handed to the engine as one submission
  ([submissions](../spec/promises/submissions.yaml)).

## Secrets

- **Secret Resource**: locates one secret in a store. Documents hold only a reference to it, and
  the engine reads the value before a run and passes it to the server that needs it, never to an
  Agent ([secrets](../spec/promises/secrets.yaml)).

## Shared rules every engine keeps

- **Lifecycle**: register (nothing runs), set up, retire, reconcile, unregister and purge
  ([lifecycle](../spec/conventions/lifecycle.yaml)).
- **Outcomes**: a request succeeds, fails or ends unknown. Idempotency keys make a retry safe
  ([operations](../spec/conventions/operations.yaml)).
- **Revisions**: a control changes settings only against the instance's current revision
  ([revisions](../spec/conventions/revisions.yaml)).
- **Records**: who asked, what, when and how it ended, for each request ([records](../spec/conventions/records.yaml)).
- **Nothing implicit**: nothing starts model work or repeats an effect on its own
  ([implicit](../spec/conventions/implicit.yaml)).

## How it is proven

- **Promises**: 454 numbered rules in three layers: language, shared conventions and the
  implementation interface ([contents](../spec/README.md#contents)).
- **JSON Schemas and examples**: every document's schema, with valid and invalid examples
  ([schemas](../spec/schemas/README.md)).
- **Checkers**: the Python scripts in `tools/` that check links, names and schemas.
- **Experiments**: [004](../experiments/004-stateright-evaluation/README.md), a model checker
  evaluation, and [005](../experiments/005-macos-keychain/README.md), a Keychain probe.
- **Still to come**: conformance vectors, the Rust reference model and provider packages under
  `packages/` ([experiment program](AUDIT.md#experiment-program)).
