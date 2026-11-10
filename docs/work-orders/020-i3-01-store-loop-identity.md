# Work order 020 (I3-01): the shell's store, its loop and its identities

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. This order has network access, to fetch crates and for tests
that bind a loopback port; use it for nothing else.

## What and why

Increment 3 of [the first build](../implementation/FIRST_BUILD.md#increments) is the shell: the
thin part that makes the pure core runnable. This is its first order. It builds what every later
order of the increment stands on: a durable store, the loop that feeds the core and carries out
what the core asks for, recovery after a crash, the retention of outputs, and the credentials
the engine gives each run. It starts no process, serves no HTTP and runs no Agent: orders 021
(the runner), 022 (the server consumers connect to) and 023 (the Agent types) do.

Read: [the handoff](../proposals/first-build/increment-3-handoff.md), all of it, which is the
interface you build on, and its items H01 and H04; the first build's increment 3 and "What
increment 3 needs"; `reference-model/CONFORMANCE.md`; the core as it stands
(`reference-model/crates/core/`), and the conformance runner
(`reference-model/crates/conformance/`), which shows how a `Context` and `Evidence` are built;
and these promises: REC-001, OPS-008, OPS-018, OPS-034, IMP-007, FST-001, FST-005, IFC-022,
IFC-032, IFC-042, PRV-022, PRV-024, SEC-018.

## The decisions to apply

1. **A new crate**, `reference-model/crates/shell`, package `conduit-shell`, with a library and
   a binary named `conduit`. It depends on `conduit-core` and `conduit-loader` and never
   decides anything the core decides: access, outcomes, records and lifecycle stay in the core.
   Where the shell would need to decide one, it stops with an embedding error and you report it.
2. **The store** is a directory the user names. The core's whole `State`, which includes its
   retained records, is committed as one file replaced atomically: written to a temporary file
   in the same directory, flushed with `fsync`, renamed over the old one, and the directory
   flushed. One commit per transition. The store's directory is created with mode 0700 and its
   files 0600. A store holds a format version, and the shell refuses a store whose version it
   does not know. Record it as the DL2 choice for storage.
3. **The loop.** One writer at a time: a transition reads the latest committed state, calls
   `conduit_core::step`, and commits the new state and records **before** it carries out any
   intent or returns any final reply (DL2, REC-001). Intents are carried out outside the lock by
   an `Executor` trait, one method per intent kind of the handoff, so that orders 021 and 022
   plug in. Each completion comes back as its own input through the same lock: `complete` by
   invocation and attempt, `setup-report` by invocation. A pending reply waits, inside the
   shell, until a later transition ends the invocation, and it never reaches a caller as a
   success. An intent kind that no executor serves ends as the core says when the shell reports
   it failed or missing; it is never dropped silently. This order supplies a scripted executor
   for tests only.
4. **Evidence** comes from the shell: the time from the system clock in RFC 3339 UTC, IDs as
   random UUIDs from the operating system's generator, and revision values and five-character
   suffix candidates as the core's contract asks. Supply enough for one transition; if the core
   reports exhaustion, retry the same input once with a larger supply and then stop with an
   embedding error. The state you retry from is the committed one, unchanged.
5. **Recovery.** When the shell opens a store, it gives the core one `recover` input before it
   accepts anything else, and commits the result. Nothing that was dispatched before the crash
   is sent again (IMP-007). Show it with crash tests: a child process that the test kills (a)
   after the commit and before the dispatch, and (b) after the dispatch and before the
   completion is committed. Each test reopens the store and shows that the invocation ended
   unknown, has one record of its end, and was not dispatched twice.
6. **Retention of outputs (DL10).** The owner asked, in the
   [first review](../proposals/first-build/to-review-with-the-owner.md#the-first-review-2026-11-10),
   for a cap on what item 3 keeps: the content of `plain` calls kept for repeats. The shell
   expires retained outputs through the core's `expire-outputs` input, never by editing state
   itself, when their total size passes a cap or their age passes a limit, oldest first. Defaults:
   64 MiB in total and 7 days, both settable in the shell's configuration. Expiry never drops
   invocation state. Add the policy to `CONFORMANCE.md` as the shell's DL10 choice, replacing
   the line that says no production policy exists, and show it with a test that passes the cap.
7. **Identities (DL13).** The shell holds one signing key per store, an ECDSA P-256 key made at
   first start, kept in the store with mode 0600 and never in the core's state, in a record or
   in a log. For an instance it issues what PRV-022 names: `token`, a JWT-SVID signed with ES256
   whose `sub` is the instance's SPIFFE ID, `spiffe://<engine-id>/instance/<instance UUID>`,
   whose `aud` is the engine's address, with `iat`, no `exp` (a credential lasts as long as its
   instance, under DL13) and a `jti`; `engine-id`, the engine's ID, a trust domain made at first
   start (`<random 26 lowercase letters and digits>.conduit.local`); `jwks.json`, the JWK Set of
   the public key; and `ca.crt`, a self-signed root certificate in PEM for the same key. Since
   the first build serves `http` on loopback (DL13, PRV-022), `ca.crt` checks nothing; say so in
   `CONFORMANCE.md`. The shell writes them into a directory it creates per run, mode 0700,
   files 0600, under the store, and gives `CONDUIT_URL` and `CONDUIT_IDENTITY_DIR` for it. The
   names come from the run contract the intent cites, as the handoff says, never from literals.
   Verifying a token checks the signature, the algorithm, `aud`, the SPIFFE ID's form and trust
   domain, and asks the core's state whether the instance still exists (IFC-022, IFC-032); the
   result is the `identity` value the core's `Invoke` takes, `{kind: instance, id}`, or
   `{kind: none}` with no token, or `{kind: unreadable}` with a token that does not verify.
   A test shows that no part of an issued identity appears in the committed state, the records
   or the shell's output (PRV-024, SEC-018).
8. **The command line.** `conduit init --store DIR` (first start, with the machine's host name
   and no labels unless given), `conduit register --store DIR PROJECT_DIR` (the project path of
   the handoff, with the S1 packages preinstalled from `conformance/fixtures/s1/` as the
   runner does), `conduit inspect --store DIR` and `conduit records --store DIR` (export, as
   JSON lines). Registration's setup will stop at its first runner intent while no runner
   exists; that is expected here, and the command says which intent waits. `conduit serve`
   comes in order 022.
9. **Tests** run the loop end to end on S1 with the scripted executor standing in for the
   runner and the servers, as the `s1-whole` vector's completions do, and check that the
   records the shell exports match those the vector expects, apart from times and IDs.
10. **The log of increment 3**: create `docs/proposals/first-build/log-increment-3.md` in the
    form of the log of increment 2, and log there any finding under ruling 46, numbered 3-01 on.
    A case the core cannot decide is a finding; unfinished shell work is not.

## Scope

Create and change only: `reference-model/crates/shell/`, `reference-model/Cargo.toml`,
`reference-model/Cargo.lock`, `reference-model/README.md`, `reference-model/CONFORMANCE.md`,
`docs/proposals/first-build/log-increment-3.md` and `docs/proposals/first-build/README.md`.
The core's public interface may gain what the shell needs to call it, with no change to any
decision; say what you changed and why. Report anything else that needs changing; do not change
`spec/` or `conformance/`.

## Done when

The shell commits every transition before any intent runs, recovers after both crash points
without dispatching twice, expires outputs by its cap, issues and verifies identities that never
reach a record, and runs S1 on the scripted executor with the vector's records. From the
repository root: `python3 tools/check_docs.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass;
in `reference-model/`, `cargo test --release`, `cargo clippy --all-targets -- -D warnings` and
`cargo fmt --check` pass. Check the files you create for trailing white space yourself; do not
stage anything.
