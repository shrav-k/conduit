# Work order 134 (PLAN-G): ruling 77 in the plans, and S3's fault method

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

## What and why

The plans of S2 to S10 and A63 (orders 117 to 121) held four of the owner's choices open, M2-1 to
M2-4, and named no answer. [Ruling 77](../proposals/ruling-77/README.md) answers them: read it
first. In short, as the admin gave it: SSH-M and SSH-R are the Windows computer's machine and
runtime providers, and Tailscale stays the network (M2-1); runs there execute as one user,
`conduit-run`, with no sudo (M2-2); the engine's `https` address is served by `tailscale serve`
after the Mac's tailnet name is changed (M2-3); and the Windows computer's Ubuntu-24.04 is locked
down, with interop, `appendWindowsPath` and automount off (M2-4). M2-5 is on demand (ruling 72's
later answer). This order writes those answers into the plans, keeping each real dependency, and
puts to the owner a question that ruling 77 raises for S3: how S3 cuts the network and loses the
machine.

## The decisions it applies

1. **Replace each hold with ruling 77's answer, keeping the dependency.** Find every mention with
   `grep -n "M2-[1-5]"` in the files of the Scope. Then:
   - "the M2-1-selected provider", "the selected second-host provider" and the like become "SSH-M
     (ruling 77)"; "SSH-M is only the admin's candidate" and "a Tailscale machine provider is the
     other candidate" go;
   - a wait on M2-3 becomes a wait on the work it needs: "ADDR and DSP, and `tailscale serve` set
     up after the Mac's tailnet rename (ruling 77)". It stays a dependency of every run that a
     runtime provider makes (S2-R3 and S2-R4, S10's Docker-only and all-swapped configurations and
     S10-R2, S6-STOP's live runs, the engine's use of A63-RP and A63-DR);
   - a wait on M2-2 becomes "the run user `conduit-run`, with no sudo (ruling 77)", and a wait on
     M2-1 and M2-2 for SSH-R or SSH-M is removed: both are unblocked;
   - M2-4 becomes "the distro locked down (ruling 77)": say what that removes (decision 2);
   - "Owner hold M2-…" lines in a plan's box and "For the owner" become one line each naming the
     ruling's answer, or go where nothing remains to ask;
   - lines that say "no answer to M2-1 to M2-4 is selected" say instead that ruling 77 answered
     them;
   - PROVIDERS.md's question D says SSH-R and SSH-M are separate crates and no longer wait.

   Change nothing else in those lines, and keep each plan's sections and their order.
2. **S3's cut and loss** (`experiments/014-s3-two-machines/plan.md`). S3's README makes a
   partition with a firewall rule and a loss with `wsl --shutdown`, run by the coordinator. With
   interop off, nothing inside the distro can run a Windows command, so the coordinator cannot run
   `wsl --shutdown` or set the Windows firewall from there; and `conduit-run` has no sudo, so it
   cannot set a Linux-side rule either. Add to S3's plan, under "For the owner", one question with
   these options, each with its cost to the owner and what it changes on the owner's machine, and
   the coordinator's recommendation:
   - **(A) The owner's hand at a barrier.** At each cut, heal and loss, the run stops at a barrier
     and prints the one command to run on Windows (a PowerShell rule that blocks the Mac's tailnet
     address, its removal, and `wsl --shutdown`), and goes on when the owner confirms. Count the
     actions per run from S3's README (four partition cells and one loss phase per workload, two
     workloads) and give the owner's time per run and for the four runs. Nothing new is installed
     or granted.
   - **(B) A Windows-side mechanism set up once:** for example, a Windows OpenSSH account whose key
     can run only the cut, heal and shutdown commands, reachable only from the Mac's tailnet
     address. Give the one-time setup time, and that it adds a remote entry to the Windows host.
   - **(C) A helper inside the distro:** a dedicated fault user (not `conduit-run`) with a sudo
     rule for one root-owned script that sets and removes a Linux-side rule and powers the distro
     off. Say that it adds a privilege that ruling 77's "no sudo" kept out, and that a power-off
     from inside may not be the same loss as `wsl --shutdown` (S3's T6): it is to be checked.

   **The coordinator's recommendation, to write as one: (A)** for the four counted runs. The owner
   is present for S3's runs anyway (enrollment, sign-ins and recovery), it installs and grants
   nothing, and each action is one line. (B) is the fallback if (A)'s time proves too much. Make
   S3-H's barriers wait for the owner's confirmation under (A), and say that S3-H changes with the
   answer. Decide nothing: S3's runs wait for the owner's answer, a new owner step.
3. **Findings only, no edits:** the second-machine guide's section on how S3 cuts the network
   (`docs/guides/second-machine.md`) describes the old method: report it, with what would change.

## Scope

Change only these files: `experiments/013-s2-shared-login/plan.md`,
`experiments/014-s3-two-machines/plan.md`, `experiments/015-s4-partial-setup/plan.md`,
`experiments/017-s6-revoke-in-flight/plan.md`, `experiments/020-s9-start-and-repair/plan.md`,
`experiments/021-s10-swapped-providers/plan.md` and `docs/implementation/PROVIDERS.md`. Change no
README, guide, ruling, order or record.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`
- `git diff --check`

Run with `bash`; it prints only lines that still say a choice of M2-1 to M2-4 is held:

```bash
grep -n -i -E "M2-[1-4].*(held|hold|candidate|selected|stays|remain)" experiments/0*/plan.md docs/implementation/PROVIDERS.md
```

Report each line you changed, by file and line, and each place where keeping the dependency needed
a choice. Do not stage anything.
