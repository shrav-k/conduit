# 025 — A73: 1Password with no person present

Not binding. The [specification](../../spec/README.md) alone binds. A73 in the
[register](../../ROADMAP.md#experiments-and-probes) asks whether a 1Password item can be read
with no prompt answered, by a service account and through the desktop app's session.

## Stage 1: the desk check

Registered on 2026-11-16 by work order 190 before any page was read, following
[FR-487](../../docs/proposals/full-review/findings/G3-native-probes.md#fr-487): 1Password's
published documentation, read on one date, with quotations; no account, no vault, no run and no
1Password command. Each question is answered separately for the service account and for the
desktop app's integration with 1Password CLI. Where the documentation names an SDK reading
through the desktop app, that is recorded under the desktop route.

1. **What authorizes a read.** What credential or act lets a read happen, and what limits the vaults and items it reaches? Include any vault that the documentation says a service account can never be given.
2. **Whether a prompt can appear.** Can any read ask a person for anything, such as an approval, a biometric check or a password, and when? How long does an approval last, idle and in all, where the documentation says?
3. **Locked, closed or absent.** What happens to a read when the desktop app is locked, quit or not installed, and when no person is logged in to the computer's desktop, as for a process started over SSH or by a background service? Does the service account need the app or a desktop session at all? Say plainly whether the documentation says that a read through the desktop app can succeed while the app is locked: it says so, it says not, or it does not say. Under FR-487, stage 2 runs the desktop case only if it can.
4. **Rate limits.** The stated limits, by plan, period and kind of request, and what a caller receives past one.
5. **The token.** Where the documentation says a service account's token is kept or passed; its lifetime; how it is rotated and revoked; and what a caller receives with a revoked or expired one. For the desktop route, what session or credential is kept, and where, as far as the documentation says.
6. **Errors.** The errors or exit statuses documented for a revoked or expired token, a missing item, a locked app and no network. Stage 2 would measure these.
7. **Plans and accounts.** Which kinds of 1Password account or plan the documentation says can make a service account or use the desktop integration.

The check decides nothing and drafts questions for the owner where its reading bears on a ruling.

## Stage 2: not registered

Stage 2 is not registered. FR-487 sets its terms: it runs only if a 1Password store is ordered.
It takes the service account alone, on the vault and service account that
[the 1Password guide](../../docs/guides/1password-and-linear.md) has the owner make, under
[experiment 005's safety rules](../005-macos-keychain/README.md#safety). The token is passed by
file and never printed. Its measures are reads with no prompt out of n; latency; and the error
for a revoked token, a missing item and no network.

The desktop case runs only if stage 1 shows that a read can succeed while the app is locked,
and only after the owner approves it; otherwise, as FR-487 has it, the desktop case is recorded
from the documentation as a store that needs a person present. If the owner strikes the plan's
reading that “or else” allows the file store whenever A73 is late, stage 2 runs before S10's
freeze ([the review's experiments](../../docs/proposals/full-review/experiments.md), A73's row).
Stage 2 is pre-registered in this README before it runs.

Results: [results-2026-11-16.md](results-2026-11-16.md).
