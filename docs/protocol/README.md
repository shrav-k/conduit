# Protocol topics (superseded)

Nothing in this directory binds. The specification is [`spec/`](../../spec/README.md). Read its
README first, then [`spec/definitions.md`](../../spec/definitions.md) for the terms. Where a
file here differs from the specification, the file here is wrong.

The seven topic files are the design as it stood before it was written as numbered promises.
They predate rulings 16 to 37 of the plan audit, and every area they cover has since been
written in `spec/`. They keep their paths because about 200 links point at their headings. A
behavior that is stated in a topic file and not in `spec/` is a gap in `spec/`, not a rule; the
[audit of the documents outside `spec/`](../proposals/docs-audit/README.md) found the known
ones, and its [report](../proposals/docs-audit/reader-older-text.md) lists them.

## Contract map

The current map is the [Contents table of the specification](../../spec/README.md#contents).
The table below says what each topic file is now kept for, and where its subject is now stated.

| File | Kept for | Its subject is now in |
| --- | --- | --- |
| [MODEL.md](MODEL.md) | Source material for contracts still to be written: Interface attachments and observability. Links. | `spec/promises/objects.yaml`, `contracts.yaml`, `naming.yaml` and `packages.yaml`; `spec/conventions/access.yaml` |
| [CONFIGURATION.md](CONFIGURATION.md) | Links only. | `spec/promises/schemas.yaml`, `objects.yaml` and `submissions.yaml`; `spec/schema-profile.md`; `spec/conventions/revisions.yaml` |
| [LIFECYCLE.md](LIFECYCLE.md) | Links only. | `spec/promises/instantiation.yaml`, `templates.yaml` and `graphs.yaml`; `spec/conventions/lifecycle.yaml`, `operations.yaml` and `revisions.yaml` |
| [CONDUITS.md](CONDUITS.md) | Source material for contracts still to be written: messaging and provider integrations. Links. | `spec/promises/conduits.yaml`; `spec/conventions/access.yaml`; `spec/interface/implementation.yaml` |
| [POOLS.md](POOLS.md) | Source material for a contract still to be written: shared authentication. Links. | `spec/promises/pools.yaml` |
| [RESOURCES.md](RESOURCES.md) | Source material for type contracts still to be written: the directory, worktree, memory and workspace types. Links. | `spec/promises/pools.yaml`, `placement.yaml` and `contracts.yaml` |
| [AGENT.md](AGENT.md) | Source material for a type contract still to be written: the Agent type. Links. | `spec/promises/placement.yaml`; `spec/conventions/implicit.yaml` |

## Where rulings are logged

The owner's rulings are in the [ruling log of the plan audit](../AUDIT.md#ruling-log). Each
batch of rulings has a record under [`docs/proposals/`](../proposals/); a record is history, and
the specification governs where they differ. The current worked example is
[`spec/schemas/examples/valid/`](../../spec/schemas/examples/valid/).
