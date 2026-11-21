# 027 — A5 stage 1, step 1, 2026-11-19: the account check, and the Claude scratch sign-in

Every result of this experiment is obtained under one operating-system user. Under one user, an
Agent that runs code can read, use or replace another Agent's login and act as root's Interface.
So no result here shows Agents kept apart ([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md),
point 1). This is stage 1's first step, the live account check of
[ruling 82](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-82/README.md), point 1, run on the Mac on 2026-11-19 as
[its pre-registration](README.md#stage-1-step-1-the-account-check) sets it and as its
[amendment of 2026-11-19](README.md#amendment-of-2026-11-19-the-live-check-as-it-runs) has it run,
with, in the same sitting and outside A5's gate, the Claude scratch-home sign-in that the
amendment's item 6 adds, which ran and is recorded [apart](#the-claude-scratch-sign-in). The
owner, told first, was to sign one scratch home of each client in once, through `conduit sign-in`, with the owner's primary account;
by the amendment's item 2 the sign-in line is the owner's to run, never the coordinator's, so that
no sign-in's URL reaches the coordinator's session. This file
records no account value, expected or reported; no token or claim, code, `state` value or PKCE
value; no URL of a login, whole or in part; no login literal; no app-server answer; no diagnostic's
text beyond its step and cause; and nothing else that a client printed. Each status was read
through a filter that prints only `login` and `account_named` under
[stage 0's status rules](README.md#what-is-recorded), and each `sign-in.v1` result through one that
prints only the fields recorded below. It decides nothing beyond the outcomes 027 gives each result;
what it cannot show is listed [at the end](#not-shown).

## The builds and the clients

These were fixed before the sitting, in the coordinator's preparation; the clients' versions are
also read at step 1 and C1, below.

| | Codex step | Claude scratch sign-in |
| --- | --- | --- |
| Engine | `main` at `a6dcd98b8552c5039132e8cea73ab4c1b2a798d5` (order 214, #435), which holds 048b-join (#426) and 051b-2 (#427); `cargo build --release --locked`, Rust 1.97.1; binary SHA-256 `ce0d571f6f0f4f8a4501b742223df32b35f50fb9483a898ab485b55f4a54c8c1` | the same binary |
| `serve` | a new state directory in the scratch root, `--call-timeout-seconds 1200` (amendment item 4), its output to a file no one read | the same, a second new state directory in the same scratch root |
| Package built for the check | commit `34f4e6625b46d3763df10efe97f8451cb333dd94` of the Codex lane (orders 050a, 050c to 050f, 050h, 050l and 050m, not on `main`), its `packages/agent-codex` with the `address:` line changed and nothing else | commit `f1ede6a59bbfabca8d94f1a01fbc61f60c6f6297` of the Claude lane (orders 050a, 050c to 050f, 050h and 050i to 050k, not on `main`), its `packages/agent-claude-code` with the `address:` line changed and nothing else |
| Address | `example.org/conduit/a5-stage-1-check` | `example.org/conduit/a5-stage-1-claude-check` |
| Type | `codex.v2` (amendment item 1), stating `signIn` with `account: /account` and the logins `browser` and `device` | `claude-code.v2` (C3), stating `signIn` with `account: /account` and the logins `browser` and `code-page` |
| Version, archive SHA-256 | 0.2.0, `db538259ccb3532fb5e95321f46d7fc4f8a4d1ffd2cc4a63c8581356efac0276` | 0.2.0, `2b90f1e5d3ef006767c3ba6c9e5f51280a655f8698b11c45ef22beb89a475fb3` |
| Installed | only in its scratch state directory, removed with it | the same |
| Client, from installation metadata before the sitting (not a run) | Codex 0.167.1: the package `codex` of the npm scope `openai`, and its macOS arm64 platform package | Claude Code 2.1.318: the package `claude-code` of the npm scope `anthropic-ai`, and its macOS arm64 platform package |
| Client version run, at step 1 or C1 | 0.167.1 | 2.1.318 |

The Codex lane's tip has since moved to `1ee93ee0` by order 050h's Record alone;
`packages/agent-codex` is the same there as at `34f4e662`. The Claude lane's tip is still
`f1ede6a5`.

Each state directory held one Agent template of its package's Type, whose `account` the owner
entered by a line that does not echo it, and one Agent instance of that template, `a5-check` or
`a5-claude`, owned by a Fleet `a5`, since an Agent's owner is a Fleet. The template's `directory`
was unset and its `accountServices` false.

## The Codex step

Steps 1 to 8 of [the step](README.md#the-step), as the amendment has them.

| Step | What ran | Recorded |
| --- | --- | --- |
| 1, version | `codex --version` | 0.167.1 |
| 2, everyday status | `codex login status`, in the client's own default home | `login` yes, `account_named` no |
| 3, registration and setup | the template and instance submitted through `serve`; setup's account check | sign-in action raised for `Agent/a5-check`: yes |
| 4, empty-home status | step 2's command with `CODEX_HOME` set to the instance's home and the package's configuration override, which selects the file store | `login` no, `account_named` no |
| 5, the sign-in | the owner, as root's Interface, ran `conduit sign-in` for `Agent/a5-check` in a terminal of the owner's own, its standard output to a file in the scratch root | ran |
| 6, the outcome | the result file read through the result filter; then the pending list | in the table below |
| 7, sign-out | `serve` stopped; `codex logout` in the instance's home only, with step 4's override, since [item 8's read](results-2026-11-17-logout-source.md#the-outcome-under-item-8) passed; then step 4's status again | `serve` stopped; logout exit status 0; then `login` no, `account_named` no |
| 8, everyday status again | step 2's command | `login` yes, `account_named` no; the same as at step 2: yes |

Step 6, `sign-in.v1` for `Agent/a5-check`:

| Field | Recorded |
| --- | --- |
| Outcome | succeeded |
| Reason | none |
| The action answered signed-in | yes |
| Promise IDs its diagnostics name | none |
| The owner signed in with the expected account | yes |
| Any other window appeared | yes: a tab that opened by itself before the owner typed `y`, Codex's own, as amendment item 3 expects |

A tab that Codex opens itself, before the owner types `y`, is expected (amendment item 3); the
answer that another window appeared is recorded and, alone, changes no outcome.

### The Codex step's result

Codex step outcome: **pass**, Type `codex.v2`, package `example.org/conduit/a5-stage-1-check`.

- Where not a pass, the step that failed: none.
- Its cause: none.
- What it changes: `codex.v2` states `signIn`, as order 050n writes it; this outcome is recorded here before any archive whose `codex.v2` states `signIn` is installed outside the check's scratch store.
- Every guard held: yes. For the tab that Codex opened before `y`, the browser guard rests on the sitting's instruction, from amendment item 3, that the owner close it without signing in there and sign in only in the tab opened after `y`.

## The Claude scratch sign-in

Not part of this step's gate or of A5's, by the coordinator's reading in
[ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 5, open to the owner's strike: it
decides nothing for `codex.v2`, for `claude-code.v2` or for order 050n's join, and its fail or fault
goes to the owner before any Claude dogfood slot signs in. C1 to C8 of the amendment's item 6, in a
new state directory inside the same scratch root, after step 8.

| Step | What ran | Recorded |
| --- | --- | --- |
| C1, version | `claude --version` | 2.1.318 |
| C2, everyday status | `claude auth status`, with `CLAUDE_CONFIG_DIR` unset | `login` yes, `account_named` yes |
| C3, registration and setup | the template and instance submitted through `serve`; setup's account check | sign-in action raised for `Agent/a5-claude`: yes |
| C4, empty-home status | C2's command with `CLAUDE_CONFIG_DIR` set to the instance's home | `login` no, `account_named` no |
| C5, the sign-in | the owner ran `conduit sign-in` for `Agent/a5-claude`, as for step 5 | ran |
| C6, the outcome | the result file read through the result filter; then the pending list | in the table below |
| C7, sign-out | `serve` stopped; `claude auth logout` in the instance's home only, since item 8's read passed for it; then C4's status again | `serve` stopped; logout exit status 0; then `login` no, `account_named` no |
| C8, everyday status again | C2's command | `login` yes, `account_named` yes; the same as at C2: yes |

C6, `sign-in.v1` for `Agent/a5-claude`:

| Field | Recorded |
| --- | --- |
| Outcome | succeeded |
| Reason | none |
| The action answered signed-in | yes |
| Promise IDs its diagnostics name | none |
| The owner signed in with the expected account | yes |
| Any other window appeared | no |
| A macOS Keychain dialog appeared | no |

### The Claude sign-in's result

Claude sign-in outcome: **pass**, Type `claude-code.v2`, package `example.org/conduit/a5-stage-1-claude-check`.

- Where not a pass, the step that failed: none.
- Its cause: none.
- What follows: nothing for A5's gate.
- Every guard held: yes.

## The checks after (step 9, once for both)

`BEFORE`, what `pgrep -f 'codex app-server'` printed, was kept before step 1 in a file outside the
scratch root, not in a shell variable, since the coordinator's commands ran as separate shell calls.
The third check ran on the system temporary directory that the shell starting `serve` saw, by its
absolute path (not written here).

| Check | Printed nothing |
| --- | --- |
| The scratch root removed | yes |
| `pgrep -f 'auth login\|codex login'` | yes |
| `comm -13` of `BEFORE` and `pgrep -f 'codex app-server'` | yes |
| `find` of the temporary directory, `-maxdepth 1 -name 'a5-*'` | yes |

## The owner's minutes

About 20 minutes in all, for the whole sitting; not split here between the time before the
sign-ins, the Codex step and the Claude sign-in.

## Deviations from the pre-registration

None of these changes a step's pass, fail or fault condition.

1. `BEFORE` was kept in a file, not a shell variable (above).
2. Each project added a Fleet `a5` that owns the instance, beside the one Agent template that 027
   names: the engine requires an Agent's owner to be a Fleet, and `sign-in.v1` reaches the Agent
   through it.
3. Every client command the coordinator ran ran under `env -i` with only `HOME`, `LOGNAME`,
   `PATH`, `SHELL`, `TERM` and `USER`, `PATH` holding the clients' own directory; `CODEX_HOME` or
   `CLAUDE_CONFIG_DIR` was set by the coordinator only at steps 4 and 7 and C4 and C7, to the
   instance's home in the canonical form of the path the package passes.
4. Step 1 and C1 printed only the first version-shaped token of the client's standard output.
5. `sign-in.v1`'s result holds no field that says the action answered signed-in. The result filter
   reports it from a succeeded, non-error result with an account present (PND-011), and the pending
   list read before `serve` stopped confirms it: no sign-in action left for the instance.
6. With standard error joined to standard output, the Claude status filter reads "standard output
   parses as a JSON object" as: the whole input does, or else the first block of whole lines opening
   with `{` and closing with `}` that parses as one; otherwise the text rules apply to all of it.
7. `install`'s, `submit`'s and `serve`'s output went to files that no one read, since the action's
   message names the expected account.
8. The third check ran on the temporary directory by its absolute path (above), since the
   coordinator's session and the shell that started `serve` may see different temporary directories.
9. The checks after ran 027's two `pgrep` patterns with a bracket (`[c]odex app-server`,
   `[a]uth login|[c]odex login`), which match the same processes but not the shell that runs them.
10. Before step 3, the owner wrote the expected account of each template by a line that does
    not echo it, as the amendment's item 2 has it, and the line reported success. During the
    sitting, after that entry and at the owner's request, the coordinator replaced that line
    with one that shows what is typed and saves it only when the owner answers `y`, since the
    owner judged the hidden entry a poor design. The replacement was not used for either
    template, in this step or in the Claude sign-in.

## Not shown

- Agents kept apart: everything recorded here ran under one operating-system user.
- P1 to P8 as a whole. The Codex step's pass is its own. It observes P1 only as step 4 and step 6,
  P2 only as step 6, and P7 only for the everyday status at steps 2 and 8. P3 to P6 and P8 wait for
  the rest of stage 1.
- The rest of stage 1, and stages 2 and 3: a wrong account refused and undone, the second accounts,
  refresh, lapse, and one Agent's sign-in or retirement leaving the others working.
- Codex's device login, Codex on the Windows computer or under WSL, and any version of Codex but
  the one recorded.
- For the Codex step, Claude Code, of which it asks nothing; the Claude sign-in is recorded apart, [above](#the-claude-scratch-sign-in).
- Claude Code's other logins, Claude Code on another machine, and any version of Claude Code but
  the one recorded.
- Whether either logout revoked anything at the vendor: the sign-outs rely on
  [item 8's read](results-2026-11-17-logout-source.md#the-outcome-under-item-8), and this step
  observed only each home's status after it.
- The account check's refresh block on the Mac: it is shown only by
  [order 208's probe](results-2026-11-19-refresh-probe.md), on the Linux node with a fabricated
  login; this step did not look at the home's files.
- The packages as order 050n joins them: the check ran the lane commits recorded above, and the
  coordinator judges at 050n's merge whether the pass, if any, stands for the joined tree.
- The logins' literal origins, paths, client IDs and ports.
