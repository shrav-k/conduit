# The providers (A63): the order-level plan

> **In short**
> - Admin proposal A63-H1: a separate provider-vector driver crate in the engine workspace; no shell behavior changes.
> - Admin proposal A63-H2: test trust roots, run certificates, gate clients and fault controls in that crate.
> - Admin proposal PSD1: source admission, fetch intents and serialized install decisions in the core, after 082.
> - Admin proposal PSD2: package-source MCP dispatch and archive delivery in the shell, on DSP's common call path.
> - Admin proposal PSD3: the engine's own HTTPS fetching, with digest verification before archive use.
> - Admin question A: where A63-H lives, and who maintains its vector index; recommendation: a separate engine crate.
> - Admin decision B (2026-11-11): (b), packaged local providers built alongside S10's accepted configurations.
> - Admin question C: local-machine enrollment qualification; recommendation: expose the limitation before claiming the common suite passes.
> - Admin question D: SSH's crate boundary; recommendation: separate provider crates, with SSH-R reusing A63-RP's server and gate.
> - Admin question E: merge order and real-client interpretation; recommendation: preserve the pin and seek the admin's reading of D4.
> - Owner: no new question; M2-1 to M2-4 stay held.

## Status

Written on 2026-11-11 by work order 120, against `main` at `31acd73`. A plan for the lane
coordinators and the admin: it binds nothing, and changes nothing the owner accepted.
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched. A fresh skeptic reviews this split before dispatch.

## What `main` has now

Evidence below is the tree at that commit, not the presence of an order file or its commit title.
[A63](../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept) remains open;
[roadmap steps 3 and 4](../../ROADMAP.md#the-order-of-the-work) distinguish first providers from swaps.

| Contract / prerequisite | Present, with evidence | Missing / consequence |
| --- | --- | --- |
| Runtime | Built-in stdio process groups, lifetime guard and private instance directories: [DL13/DL16](../../engine/CONFORMANCE.md), lines 33–40; core emits runtime calls. | DL16 line 40 leaves provider calls unserved; DL13 line 59 leaves gate TLS unfinished. ADDR/079 and DSP/080 are order files, not implemented paths. A63-RP and A63-DR do not exist. |
| Machines | Local first-start report; enrollment promises ENR-001–012 are live. [capabilities.yaml](../../spec/core/capabilities.yaml), lines 157–173, still lists only reports/placement. | [operations.yaml](../../spec/core/operations.yaml), lines 991–1040, still reports no id/pending state and places by hostname. Enrollment operations exist separately at lines 1369–1555. 049a-1 must land both staged patches after PIN; 049b supplies the fixture, not a real provider. |
| Package source | Local file installation and archive validation: [DL7](../../engine/CONFORMANCE.md), lines 24–25 and 55; shell reads file bytes in `shell/src/lib.rs` near line 381. | `core/src/requests.rs` lines 968–998 restrict public fetching to local files. No source MCP dispatch, HTTPS fetching or scheme admission exists; PSD is required. |
| Secrets | Locator, engine-only reads and delivery are specified by SEC-005–024; secret collision/admission vectors exist. | DL16's unserved boundaries include secrets (CONFORMANCE.md lines 266–267). No store package exists; SEC-D supplies engine delivery, A63-FS/KC supply stores. A22 remains open; logged-in-session use does not wait for it. |
| Common suites | [case form](../../conformance/README.md#inputs-and-files) and [runner](../../engine/crates/conformance/src/lib.rs), lines 18–29 and 66–95, drive loader/core cases. | No MCP provider driver or per-tool report. One shared suite per contract is A63-H plus A63-VR/VM/VP/VS; core cases alone cannot establish P3. |
| Agent packages / runs | [agent-codex](../../packages/agent-codex/README.md) provides the standalone crate/archive model; both current manifests are 0.1.1 with `.v1` Types. | 050a/076's `.v2` packages and 050's separate homes/sign-in have not landed. S2's service/filter work, Docker, isolated test stores and owner sign-ins remain prerequisites. |
| Already met scenario needs | 041b at `1091a71` implements both container-retirement operations and REV-010 held control inputs; [CONFORMANCE.md](../../engine/CONFORMANCE.md), lines 125–132. | Do not order these again. Control dispatch and declare-lost remain unfinished. S3's G4 is partly stale: enrollment promises are present, while capability bindings and engine work still wait. |
| Files versus merged work | `git log` includes 070–072, 075–077, 079–080, 049a1, 050a and 082 titles. | The tree still clones State (`core/src/lib.rs:303–305`), has one manifest list and no shell `list.v1`; the named implementation changes are absent. 081's operation already forbids duplicate runs, but its promised lifecycle/core/vector changes have not landed. 038c's alignment is also still an order. |

The [S10 prerequisites and gaps](../../experiments/021-s10-swapped-providers/README.md#what-it-needs-first)
need no new dispatch language: DL7/DL16 are implementation gaps. The safety-field gap G4 still
needs SV1 (after 071), an admin format/spec proposal, before portable safety cases are written.
S2 G1 still needs B2-S's driven-Agent secret rule; S2 G2's locators/cache/session behavior belongs
to package contracts. S3's portable loss-cause gap stays with order 117's plan, not A63.
S7's repeated-argument gap remains open: its evidence may justify a later admin spec order;
no provider assumes a new MAN-010 binding. D01 remains S2's definition, reused by S7/A63.
No additional language is proposed here. A new missing behavior must be reported before use.

## The contracts

| Core capability | First / second implementation and existing tool | Exercise / engine prerequisite |
| --- | --- | --- |
| `runtime-provider.v1` | A63-RP local process on OS process/supervisor facilities; A63-DR on Docker CLI/daemon. SSH-R is a third, on OpenSSH/systemd. | S2 D20, S3 isolated consumers, S10 T3: ADDR then DSP; container Agents also need 050. |
| `machine-provider.v1` | A63-MP on local OS machine reports; the second-host provider M2-1 selects, SSH-M candidate on OpenSSH, or a Tailscale provider on that tool. | S3 supplies machines' second-implementation evidence for S10 P3: 049a-1, 049b, DSP, 051c; see admin C. |
| `package-source.v1` | A63-PS on filesystem directory reads, claiming `file`; A63-GS on Git, claiming `git+https`. | S7 integration and S10 T4: PSD, immutable claims and fresh install caches. Unclaimed `file`/`https` remain engine fetches. |
| `secret-provider.v1` | S2/S10 first: A63-KC on macOS Security APIs; shipped A63-FS on filesystem files. S10 second: A63-1P on 1Password CLI after A73, else A63-FS. | S2, S3 and S10 T2/T5: SEC-D; B2-S/C/H before scenarios depend on driven-Agent restrictions. File/keychain also pass the same suite. |

## The orders

Sizes are estimates before pipeline fixes; line estimates include Rust, contracts, fixtures and docs.
Suffixes split this plan's shared IDs; other plans cite A63-H/RP/MP/DR and PSD unsuffixed.
`now` means dependencies are in this tree, subject to the fresh-skeptic review and admin placement.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A63-H1 | harness | Item launcher, MCP driver and report skeleton | PRV-002/005, IFC-037 | — | M, ~700 lines | admin | now |
| A63-H2 | harness | Gate/trust fixtures and fault controls | PRV-025/028, IFC-017 | A63-H1 | L, ~900 | admin | after H1 |
| A63-VR | vectors | One runtime suite | PRV-007–012/019/021/022/025/028 | A63-H2, SV1, 081 | M, ~500 | admin | after H2/SV1/081 |
| A63-VM | vectors | Reports, placement, enrollment suite | PRV-014/015, ENR-002/004–010 | A63-H2, SV1, 049a-1, 049b, admin C | M, ~600 | admin | after dependencies |
| A63-VP | vectors | One source suite | PRV-016/017, PKG-026 | A63-H1, SV1 | S, ~350 | admin | after H1/SV1 |
| A63-VS | vectors | One store suite | SEC-005/006/019/022 | A63-H1, SV1 | S, ~350 | admin | after H1/SV1 |
| A63-RP1 | provider | Local process operations and Type; the shared server library | PRV-007–012/019/027 | A63-H1 | M, ~800 | lane 8 | after H1 |
| A63-RP2 | provider | The shared gate, recovery and owed stops | PRV-021/022/025/028 | A63-RP1, A63-H2 | L, ~1,000 | lane 8 | after RP1/H2 |
| A63-MP1 | provider | Local report/placement package | PRV-014/015 | A63-H1 | S, ~400 | lane 8 | after H1 |
| A63-MP2 | provider | Complete machine contract after staging | ENR-002/007/008, PRV-014/015 | A63-MP1, 049a-1, 049b, admin C | M, ~700 | lane 8 | after dependencies |
| A63-PS | provider | Local-directory source | PRV-016/017 | A63-H1 | S, ~400 | lane 8 | after H1 |
| A63-FS | provider | Shipped file store | SEC-005/006/020/022 | A63-H1 | S, ~350 | lane 8 | after H1 |
| A63-KC | provider | Logged-in-session keychain store | SEC-005/006/022 | A63-H1, throwaway setup | M, ~650 | lane 8 | after H1/owner setup |
| A63-DR1 | provider | Docker operations and Type | PRV-007–012/019/027 | A63-H1 | M, ~800 | lane 8 | after H1 |
| A63-DR2 | provider | Docker gate, stop durability, Agent profile | PRV-021/022/025/028, ACC-027 | A63-DR1, A63-H2, Docker setup | L, ~1,000 | lane 8 | after dependencies |
| A63-GS | provider | Git archive source | PRV-016/017, PKG-026 | A63-H1 | M, ~600 | lane 8 | after H1 |
| A63-1P | provider | 1Password store | SEC-005/006/020/022 | A63-H1, A73 and approvals | M, ~600 | lane 8 | after A73 |
| PSD1 | core | Admission, source selection and fetch boundary | PRV-013/016/017, PKG-025–027 | PIN, 082, 070 | L, ~1,000 | admin | after PIN/082/070 |
| PSD2 | shell | Source provider calls and verified bytes | PRV-005/017, PKG-026 | PSD1, DSP | L, ~900 | admin | after PSD1/DSP |
| PSD3 | shell | Unclaimed HTTPS fetches | PRV-003/017, PKG-026 | PSD2, ADDR/DSP TLS support | M, ~600 | admin | after PSD2 |

All packages follow [agent-codex's form](../../packages/agent-codex/README.md): standalone Rust crate,
`Cargo.lock`, pinned toolchain, ignored output, `conduit.yaml`, `contracts.yaml`, README, tests and
reproducible archive builder. Proposed Types below are Resource Types, offering only the named
provider capability; provider-call implementations declare Conduit's extension and are not plain.
Secret Types offer only `secret-provider.v1`, no controls, and records exclude values.
Every scope below forbids edits to other providers, engine or spec unless explicitly assigned.
Package checks: release tests, clippy with warnings denied, format and archive/schema checks;
scripted backing-tool fixtures run before real-tool tests. Missing tool/host cells are reported untested.
Admin engine scopes also run engine release tests/clippy and repository checks, with their vectors.

**A63-H1.** Proposed `engine/crates/provider-conformance/`, its workspace entry/lockfile and
`conformance/provider-contracts/` format documentation; hot existing conformance `lib.rs`: none.
Launch the pinned package's selected item using its manifest bindings, initialize MCP, discover the
extension, and drive provider calls with real config/revision/caller/invocation metadata.
Secrets use engine-read request metadata instead: `get-secret.v1` is not a provider call.
Test malformed replies/discovery and launcher failures with scripted packages; no scenario snapshot changes.

**A63-H2.** Same new crate, test-only root/certificate issuer and gate clients, no production identity changes.
Give each test a private store root and a per-instance X509-SVID/key plus engine proof, shaped as DSP/080.
Check absent/instance/wrong-audience proof refusal, wrong-root/instance TLS and URL takeover before any MCP bytes,
silent-cut closure to the declared bound, and direct-backend exclusion from another run principal.
Use bounded children and synthetic markers; report only marker matches, never values or keys.

**A63-VR / VM / VP / VS.** Four separate orders, each only its contract's cases and shared provider index
under `conformance/provider-contracts/`; the admin owns that index. Use `provider.yaml`, not `case.yaml`,
so today's recursive core runner cannot mistake them for its inputs. Every safety case uses SV1's field
after SV1 merges, following SV4 and the [safety catalogue](SAFETY_CATALOGUE.md); no invented field beforehand.
Each order validates its format/index and runs the suite on available implementations, listing missing ones.

Proposed vector form, beside the existing core form: stable id/title/contract, layers/promises/profile,
fixture aliases, required tool features, ordered MCP operations/fault actions, captures and exact assertions.
Implementation descriptors supply package pin/item/Type/config, tool/version command and `<lab>` fixtures;
expected results live once per contract, never in a provider-specific copy. Unknown steps/assertions fail.
Reports name revision, contract/suite and package digests, tool/version, cases, failures and untested cells.
Capture only generated nonsecret IDs/URLs and assertion verdicts; keep secret results inside the driver.

Runtime cases: run/inspect/stop; repeated `run.v1` for a running run succeeds with its current URL,
starting no other run; when that run's state is uncertain, start nothing and follow the merged
specification's outcome. Qualification of these repeated-run cases waits for 081 to merge.
Also cover machine mismatch, unsupported registry/transport and relative command, delivered identity/secrets,
latest URL, gate proof/TLS, silent cut, whole-tree stop and owed stop surviving engine restart/provider
retirement. Both process/Docker pass them.
Machine cases: canonical labels, duplicate ids/hostnames, omitted/unreachable versus known gone, selectors,
no pending placement, enrollment action/expiry/evidence/labels, accept/remove and provider-scoped ids.
Reports/placement scaffolding can precede 049a-1; id/pending/enrollment cases wait for both patches and 049b.
Source cases: same pinned archive/base64 bytes, missing/corrupt archive, claim lifetime and unsupported scheme;
engine admission/digest assertions stay in PSD's core/shell vectors. Store cases: exact locator read,
unreadable/locked/missing fails without fallback, no diagnostic echo, no controls/other capability.
Engine-only access/delivery/restart checks stay in SEC-D; a direct harness read proves no engine authorization.
S10 V1–V6 (`s10-secret-store-equivalence`, `s10-runtime-equivalence`, `s10-package-source-equivalence`,
`s10-scheme-claim-admission`, `s10-swapped-rotation-recovery`, `s10-all-providers-swapped`) remain order 119's.

**A63-RP1 / RP2.** One crate `packages/provider-local-process/`, Type `local-process.v1`, runtime capability.
D15 planned: implements MCP mapping, stdio-to-HTTP bridge, gate and run ledger; delegates process groups and
supervision to OS facilities (launchd/systemd); persists run ownership, private run/secret/identity files and
owed-stop intent; operator configures protected storage, an existing supervisor and distinct run/gate principals.
Gate serves DSP's run certificate, admits only engine proof and exposes no backend listener to other runs;
the gate/control principal is never a run's principal/group (PRV-025). Test this under actual OS identities.
RP1 tests operations with fixtures; RP2 adds supervisor crash/restart, whole-tree stop and durable-stop tests.
State contract includes private persistent Agent homes and cleanup; initially support pinned npm/stdio items
and absolute commands only, declaring registry/transports. Full runtime suite; engine use waits DSP.
Its library holds the parts that SSH-R and A63-DR reuse, as the next paragraph states.

**What A63-RP shares with SSH-R.** A63-RP and SSH-R, the SSH runtime on the admin's side that waits
on M2-1 and M2-2, both implement `runtime-provider.v1`. A63-RP is built so that SSH-R, and later
A63-DR, reuse two of its parts, which its crate exposes as a library with a stable interface:

- **The provider's MCP server.** `server/discover` with Conduit's extension (PRV-005); `run.v1`,
  `stop.v1` and `inspect-run.v1` mapped onto a backend interface of three calls (start a run from
  its staged item, stop its whole process tree, report its state); the run ledger keyed by
  instance ID, so that a repeated `run.v1` answers with the running run and its current URL and
  starts no other (PRV-007, as order 081 settles it); owed stops kept durably and carried out at
  the next contact, through a provider restart and the instance's removal (PRV-021); and the
  staging of each run's private files from what the call carries, the identity, the run's
  certificate and key, and the environment, in a 0700 directory (PRV-022, PRV-029).
- **The gate.** An HTTPS listener on the engine's machine that serves the run's certificate as DSP
  issues it (PRV-028, IFC-017); admits a request only with the engine's proof, a token whose
  subject is the engine's ID, whose audience is the instance's SPIFFE ID and whose signature
  checks against the store's JWKS, and refuses anything else before any MCP bytes (PRV-025);
  carries the MCP stream's bytes between HTTP and the run's stdio channel; and closes a stream
  within the bound the provider's contract declares once the backend channel goes silent.
- **What stays each runtime's own:** how a run is started and reached (a local child process;
  `ssh` and `systemd-run` on the other machine; a container), the operating-system principals of
  the runs and the gate, how a stop reaches the whole tree, where the provider keeps its state,
  the sweep at startup, and each contract's text.
- **Where the shared code lives:** in A63-RP's crate, `packages/provider-local-process/`, as its
  library: the server in RP1 and the gate in RP2. SSH-R and A63-DR depend on it by path. The
  library is no provider, so ruling 40's one crate per provider is unaffected; moving it into a
  crate of its own is the admin's choice when SSH-R is dispatched. A63-H2 tests the gate through
  A63-RP, and the runtime suite runs the same cases against every runtime.

**A63-MP1 / MP2.** One crate `packages/provider-local-machine/`, Type `local-machine.v1`, machine capability.
D15: implements report normalization/selector matching and enrollment protocol; delegates hardware facts and
key storage to OS APIs; persists stable machine identity/enrollment labels and decisions; operator sets lab
identity/evidence display and allowed labels. MP1 targets today's contract, MP2 only the activated contract.
Tests canonicalization/no match, fixed labels, key continuity, accept/remove/expiry and local-node exception.
Full common machine-suite qualification is blocked on admin C; engine use waits 049a-1/049b and DSP/051c.

**A63-PS.** `packages/provider-local-directory/`, Type `local-directory.v1`, package-source capability, `file` only.
D15: implements URI/root containment and base64 replies; delegates reads to the filesystem; persists no fetched
archive cache (only configured root); operator puts pinned archives under `<lab>` and fixes `schemes: [file]`.
Test real disposable directories, missing files/escape/symlinks and corrupt bytes; pass source suite.
Engine use waits PSD; it still checks every returned digest, independently of provider success.

**A63-FS.** `packages/provider-file/`, Type `file-secret.v1`, secret capability on filesystem files.
D15: implements one-file locator/read and safe errors; delegates permissions/storage to the OS; holds no
value cache or extra persistent copy; operator creates a private throwaway file and grants read access.
Test missing/unreadable files, no environment/other-file fallback and no value-bearing output except the
read result; pass store suite. Ship as a package with the reference engine; engine use waits SEC-D.

**A63-KC.** `packages/provider-keychain/`, Type `keychain-secret.v1`, secret capability on Security APIs.
D15: implements explicit keychain/item locator, disabled interaction and bounded read; delegates custody/ACLs
to Security; persists no value cache; operator unlocks the throwaway keychain and authorizes this binary.
Contract states logged-in-session-only support, locator form, failed reads and upgrade reauthorization.
Test locked/not-trusted/missing items and changed binary under [005's safety rules](../../experiments/005-macos-keychain/README.md#safety):
compare default/search list byte-for-byte at each change, never read other keychains, close stdin, bound
children (30s; cargo 600s), lock/remove throwaway files and clean up. No `find-generic-password` CLI read.
Pass store suite; SEC-D enables engine use. A22's daemon case stays open and does not block this package.

**A63-DR1 / DR2.** One crate `packages/provider-docker/`, Type `docker.v1`, runtime capability on Docker.
D15: implements operation mapping/gated stdio bridge; delegates containers, kill/remove and inspection to
Docker; persists per-ID homes/volumes, daemon container identity and owed-stop intents; operator supplies
Docker installation/context, pinned images and an existing supervisor that drains stops without the provider.
Gate stays outside the workload container, serves DSP's certificate, authenticates engine proof and holds
an attached private backend channel; no Docker socket, host identity directory or gate key is mounted in Agents.
DR1 tests CLI mapping; DR2 tests gate/daemon failures, full-tree stop and stop recovery through retirement;
pass runtime suite, declare secret retention in daemon configuration/private volumes and cleanup explicitly.
Declare supported pinned package registry/transports; reject unsupported/relative commands. Freeze portable
Agent items before S10: existing host-relative wrapper commands cannot simply run inside a container.
050 supplies private home/state mounts and independent sign-in per home. Qualify Claude's browser callback
on its own machine and Codex's browser redirect path (device code only where enabled), forwarding only the
necessary callback; no copied login. Container engine URL/trust translations are D15/D16 edits. Use waits DSP/050.

**A63-GS.** `packages/provider-git/`, Type `git-source.v1`, package-source capability on Git CLI, `git+https` only.
D15: implements URI-to-pinned-commit/archive selection and base64 reply; delegates object transfer to Git;
persists a disposable clone/object cache, never credentials; operator prepares identical pinned archive blobs
in a disposable repository and configures Git's credential mechanism outside `fetch`. No `git archive`
recompression substitutes bytes. Test real local repository fixtures offline, bad refs/blobs/bytes and
credential-free errors; HTTPS transport qualification later needs the configured tool. Suite pass/use waits PSD.

**A63-1P.** `packages/provider-1password/`, Type `onepassword-secret.v1`, secret capability on `op` CLI.
D15: implements item/field locator and bounded noninteractive read; delegates access/custody to 1Password;
keeps no value cache; operator configures the throwaway vault/service account after A73's dated observations.
Its bootstrap credential is another secret Resource, delivered through SEC-D, never engine environment.
Test CLI fixtures now only after the probe's approved scope; real service-account and desktop-session modes
are qualified only as observed, with separate desktop-case approval. Pass store suite; SEC-D enables use.

**PSD1.** Core admission/installation plus own vectors; hot `registration.rs`, `requests.rs`, manifest pieces
(or unsplit manifest) and runner `lib.rs` for fetch-completion inputs: yes; `access.rs`/`lifecycle.rs`: none.
Enforce system ownership, case-insensitive unique live scheme claims and immutable schemes at admission;
select provider/fallback before install dispatch, serialize each address/version following 082/MC7b-002.
Add selection/unready/collision/illegal-control, concurrent first-install and digest-refusal vectors, including
the engine portion of `s10-scheme-claim-admission` by coordination with order 119; no new spec behavior.

**PSD2 / PSD3.** Shell fetching, tests and CONFORMANCE updates; hot `shell/src/lib.rs` and `runner.rs`: yes;
`server.rs`, `identity.rs`, core hot files and CLI tables: none planned. Reuse DSP discovery/call metadata;
decode source base64, verify digest before extraction/use; translate missing/unready to PKG-024.
PSD3 supplies unclaimed HTTPS with normal TLS verification, credential-free fetch URLs and no credential
leak on redirects. Test provider precedence, file/HTTPS fallback, bad TLS/base64/digest, no fetch when already
installed and concurrent first installs. Keep archive checking shared with DL7; no new archive format.

## Shared with other plans

| Shared ID, not owned here | Needed behavior / sequencing |
| --- | --- |
| 070, 071; SV1–SV4 (lane 6) | State/runner seams and index pieces; safety field, existing tags, S1 missing cases, and later orders' own tags. A63 owns no S1 safety work. |
| PIN; 082; 081; 077 | Freeze through 053 first; serialized install rule before PSD; duplicate-run/lifecycle fixes before fault qualification; fenced reconciliation before relying on failed reconciliation. Files alone are not completion. |
| AGT, ADDR, DSP; SSH-R | Windows Agent item, HTTPS audience, certificates/generic provider calls; SSH runtime is a third runtime, not a substitute for process/Docker pairing. SSH-R's gate/owed stops are tested by the same runtime suite. |
| 049a-1, 049b, 049a-2, 051c; SSH-M candidate | Activate both patches/core, fixture vectors, pending enrollment actions, root panel/report loop; M2-1's selected provider supplies real second-host evidence. Lane 9 owns this work. |
| 050/050a/076, 048a-1/2, 048b-relay/join, 051b | Separate homes, hooks, account check, pending sign-in relay/commands and independent browser sign-in; packages currently lack them. |
| SEC-D; FLT-D; S2-F; S2-L (order 117) | Secret authorization/delivery/records/recovery, onward identity/filters/support check, issue/deadline contracts, Linear variants/stub. D01/D09/D20 and S2's traces stay there; A6 refresh/discovery comes from S2, loss from S4. |
| B2-S, B2-C, B2-H | Driven-Agent secret rule in spec, core enforcement and shell delivery; drafts merge only after owner judges the rerun's bar. |
| S3 work (117); S7/S10 work (119) | Real two-machine faults and machine-suite report; repeated-argument/D01 evidence; S10 V1–V6, D15/D16 and twelve counted replays. This plan supplies packages/suites, no counted run. |

## The order of the work

Dispatch proposal from today's main: review split/admin A, H1 first, then H2; FS, PS, MP1, KC,
RP1, GS and DR1 can follow H1 in that priority, independently of engine dispatch (KC waits throwaway setup).
They run crate tests against scripted tools and real disposable local fixtures through H1; this is preparation,
not a common-suite pass. RP2/DR2 follow H2 and actual gate/supervisor setup. A73 can run beside first providers;
1P waits its observations/approvals. VR/VP/VS wait SV1; VR also waits 081; VM/MP2 wait activated enrollment contracts.
H1/H2 add isolated test tooling only; if the admin finds they alter S1's measured snapshot, place them on a
side branch until PIN. No order edits root's tools or S1's starting snapshot before PIN on main.

Critical path: PIN → ADDR → DSP → runtime integration; PIN → 049a-1/049b → selected second-host provider →
machine qualification; PIN/082 → PSD1 → PSD2 → PSD3 → sources. In parallel, SEC-D/050 and B2-S/C/H unlock
S2; S2's final D09 plus paired suite reports unlock order 119's S10 freeze and twelve replays (about one
hour owner presence per counted run, scheduling/counts owned there). S3 supplies the second machine report.
The last S10 replay waits for the longest of those chains, Docker/sign-ins and A73 or the file-store fallback.
A74's stage-0 observations do not prove S3 faults, owed stops, Docker isolation or second-account behavior.

## For the admin

- **A — harness placement.** Engine workspace crate shares loader/archive/identity test utilities but touches
  admin workspace/lockfile; extending the core runner shares indexing but couples deterministic cases to live
  tools and edits its hot `lib.rs`; standalone `conformance/provider-harness/` isolates dispatch now but duplicates
  MCP/trust bootstrapping and pins. **Recommendation:** separate engine crate, with its own provider index;
  approve H1 for now without changing production behavior. All index/runner changes stay with the admin.
- **B — first implementations.** (a) Keep built-ins first, add shell adapters to A63-H and omit packaged locals:
  less package work, two harness execution paths. (b) Build packaged locals too, pass identical MCP suites,
  leave S10's accepted configurations: more packages and explicit built-in shell evidence still needed for P3.
  (c) Substitute packaged locals in S10: simpler single MCP comparison but changes accepted configurations.
  **Decided:** (b), by the admin on 2026-11-11, logged as a coordinator decision in order 120's Record.
  The built-ins stay S10's first implementations. Do not count a package's report as a built-in's.
- **C — local machine and enrollment.** PLC-026 keeps local never pending and forbids another node for its
  hostname; a local-only provider cannot demonstrate a second host's successful admission. Options: qualify
  common local/refusal cases plus report positive enrollment as missing, or expand the first implementation's
  backing tool/topology to exercise positive enrollment. **Recommendation:** keep MP1 local, settle MP2's
  qualification before dispatch, and withhold full paired-contract evidence until the same suite passes both.
  049b's fixture is useful for protocol tests and supplies no real-tool second implementation.
- **D — SSH boundary / gate reuse.** SSH-R and SSH-M are currently planned in `packages/provider-ssh`, while
  ruling 40 says one crate per provider. Options: separate runtime/machine crates, or seek a ruling exception.
  **Recommendation:** separate crates before dispatch. SSH-R reuses A63-RP's server and gate, as
  "What A63-RP shares with SSH-R" states; transport, OS principals and stop persistence remain each
  runtime's own. The shared work is in RP1/RP2's estimates, counted once in D15.
- **E — merge and clients.** Options: merge dispatch/index/core changes serially after PIN, or stage them on
  side branches and rebase at PIN. **Recommendation:** packages now, ADDR→DSP then PSD2, PSD1 after 082;
  serialize shared manifest updates and core hot files with batch 1. 075–077/038c and 081/082 still need work.
  D4's “only real-client run” beyond S1/S2 and the explicit second subscriptions for S3/S7/S8/S10 admit
  competing readings for replay scheduling. Recommendation: admin clarify how S10's S1/S2 replays count
  before order 119 freezes runs; retain its accepted twelve replays/accounts meanwhile, add no run here.

## For the owner

No new question: the admin chose (b), so S10's accepted configurations stand, with the built-ins as
its first implementations. Existing M2-1–M2-4 remain held: provider selection, remote run user, engine
TLS termination and machine lockdown. This plan answers none of them.

## Limits

Planned D15 fields and estimates are not observations. Package tests before dispatch do not prove engine
integration, gate exclusion without actual principals, or swappability before both real tools pass the suite.
S10 preserves its frozen tasks/access/results/attribution and edit/replacement ledger (D16); no live migration,
daemon keychain support or local enforcement is claimed. A63 closes only with four paired contract reports
and S10 passing every accepted swap, with S3's machine evidence. M2 choices, A22/A73 and newly found
language gaps remain with their owners; any required spec fix is proposed to the admin before use.
