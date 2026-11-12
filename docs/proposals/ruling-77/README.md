# Ruling 77: the Windows computer's setup: an SSH provider, one run user, the engine's certificate through tailscale serve, and the distro locked down

Status: in force (ruled by the owner on 2026-11-12). Holds now: [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

The owner's Windows computer runs Ubuntu-24.04 under WSL2 and is reached over Tailscale. It is
Conduit's second machine ([ruling 72](../ruling-72/README.md), point 3). Its decisions, M2-1 to
M2-5 and M2-7, waited for the owner; M2-1 to M2-4 were held in
[the plan for S3](../../../experiments/014-s3-two-machines/plan.md) and
[the providers' plan](../../implementation/PROVIDERS.md). Earlier on 2026-11-12
the owner asked for more information on them, and made Ubuntu-24.04 Conduit's distro
([ruling 72's later answers](../ruling-72/README.md#later-answers)).

The owner: "Windows computer is up again, let's discuss the best options". The coordinator answered
with one package: a recommendation for each decision, the reason for it, and where it departs from
the owner's words in ruling 50. The owner: "Let's try your recommended setup for now, we can modify
later if need be".

The owner accepted the setup for now and may revisit any point of it. Each point is in force until
the owner changes it.

## The decisions

1. **The Windows computer joins through a small SSH provider (M2-1).** It has two parts, each its
   own crate ([ruling 40](../ruling-40/README.md)): a machine provider, SSH-M, and a runtime
   provider, SSH-R. Tailscale stays the network underneath, and a candidate machine provider for
   later; it is not made a Resource for this computer.
   - The reason: when the owner stops Ubuntu, Windows stays online in Tailscale. A provider built
     on Tailscale would report the computer present while nothing on it can run. An SSH probe
     reaches Ubuntu itself, so the computer is reported present only when Ubuntu answers.
   - This departs from the owner's words in [ruling 50](../ruling-50/README.md): "tailscale should
     be a resource in the pool". The coordinator's recommendation said so, and gave the reason
     above; the owner accepted it.
2. **One run user, `conduit-run`, with no sudo (M2-2).** Conduit's runs on the Windows computer use
   that one account.
   - The SSH provider's contract states that PRV-025's isolation is unmet until each Agent has a
     user of its own.
   - The work-order workers and the coordinator's SSH key move to `conduit-run`. It reaches less
     than the owner's own user, which has sudo and which the workers used until now.
3. **The engine's certificate comes through `tailscale serve` (M2-3).** On the Mac,
   `tailscale serve` terminates TLS for the engine's https address with a Let's Encrypt
   certificate. The Mac is first renamed in the tailnet to a name that means nothing.
   - That one hostname goes into the public certificate-transparency logs.
   - Access stays limited to the tailnet, and each request still carries a token.
   - `tailscale serve` is the TLS terminator in front of the plain loopback listener that
     [order 079](../../work-orders/079-shell-https-address.md) built on the side branch `s3/m2`
     (`--https-listen`), so the
     engine needs no change for it.
4. **The distro is locked down (the rest of M2-4).** In Ubuntu-24.04, Windows interop is off, and so
   are `appendWindowsPath` and automount: nothing in the distro starts Windows programs, has the
   Windows path or mounts the Windows drives.
   - With interop off, the coordinator can no longer stop the distro from inside it. On a pause,
     the owner runs `wsl --terminate Ubuntu-24.04`; a desktop shortcut can hold the command.

## The other questions

- **M2-5, when the computer is Conduit's**, answers ruling 72's point 3 and changes no decision, so
  it is recorded in [ruling 72's later answers](../ruling-72/README.md#later-answers): the computer
  is Conduit's whenever it is on and the owner has not paused it.
- **M2-7, WSL's memory**, needs no ruling: WSL keeps its default memory limit.
- M2-6 is settled by PLC-023, as ruling 72's later answers record.

## What follows

- **The setup.** The steps that create `conduit-run`, move the coordinator's key to it and write the
  lockdown need the owner's sudo, so the coordinator gives them to the owner; the owner also signs
  Codex in as `conduit-run`. Before the end-to-end run, for point 3, the owner renames the Mac in
  the tailnet and then turns on HTTPS certificates in the tailnet's admin console, in that order,
  because the store fixes the engine's https address on first use.
- **S3's faults.** S3's pre-registration cuts the network with a firewall rule set from Linux and
  makes the machine lost with `wsl --shutdown`, which a coordinator script restores
  ([experiment 014](../../../experiments/014-s3-two-machines/README.md)). With interop off the
  coordinator can no longer do either from inside the distro, so S3's cut and loss need the owner's
  hand at a barrier or another means, which S3's plan settles before its runs.

## What it amends

Ruling 50: for the Windows computer, this departs from the owner's direction that Tailscale should
be a Resource in the pool: an SSH provider takes Tailscale's place (point 1). Ruling 50's note that
Tailscale is a candidate machine provider stands, for later, and Tailscale stays the network. Ruling 50's
three points stand, and so does [ruling 56](../ruling-56/README.md), point 6, on what a Tailscale
provider shows as evidence.

Ruling 76, point 6, stands: until the computer is a node, workers still run there over SSH, as
`conduit-run` once that account exists (point 2).
