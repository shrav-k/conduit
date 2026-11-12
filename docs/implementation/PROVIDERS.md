# The providers (A63): the order-level plan

> **In short**
> - Lane 8 A63-H1/H2a/H2b and suites: standalone provider contracts under reserved 022.
> - Lane 8 A63-RK1/RK2: independent shared runtime kit, before provider implementations.
> - Admin proposal PSD1a: source Type and immutable scheme admission in core/loader.
> - Admin proposal PSD1b: source selection, serialized installs and archive decisions after 082.
> - Admin proposal PSD2: source MCP dispatch through DSP in shell.
> - Admin proposal PSD3: unclaimed HTTPS fetching after PSD1 and DSP TLS, not PSD2.
> - Admin proposal A63-MDIS: place/list-machines dispatch beyond 051c's report loop.
> - Admin decisions A/C/D (2026-11-11): harness at 022; MP1 local/MP2 qualification first; SSH crates separate.
> - Admin decision B (2026-11-11): packaged locals accompany S10's unchanged configurations.
> - Admin E: the D4 reading recorded below, confirmed by the owner on 2026-11-12; S4 holds the common merge sequence.
> - Owner's answers (ruling 77, 2026-11-12): M2-1 SSH-M/SSH-R; M2-2 `conduit-run`, with no sudo; M2-3 `tailscale serve` after the Mac's tailnet rename; M2-4 the distro locked down (interop, `appendWindowsPath` and automount off). M2-5 is on demand (ruling 72's later answer).
> - Owner steps: distinct OS users for RP/H2 tests; keychain binary partition-list authorization; ruling 77's setup on the Windows computer (`conduit-run`, the lockdown, Codex signed in as `conduit-run`); before the end-to-end run, the Mac renamed in the tailnet, then HTTPS certificates turned on.

## Status

Written on 2026-11-11 by work order 120, against `main` at `31acd73`. A plan for the lane
coordinators and the admin: it binds nothing, and changes nothing the owner accepted.
Orders that change `engine/` or `spec/` are proposals to the admin.
No order named here has been dispatched. A fresh skeptic reviews this split before dispatch.

Revised on 2026-11-11 by work order 121, against `1b717c408291b632d285a6bf0a782596478cc770`.

**Built so far (2026-11-12, the lane coordinator):** A63-RK1 (orders 122 and 124), A63-H1 (orders
123 and 125), A63-RK2 (order 126), A63-MP1 (order 128), A63-PS (order 131), A63-FS (order 132),
SSH-R (orders 137, 140 and 145, and its drainer, orders 146, 149 and 150), the kit's reasons for a
failed staging or start (order 138), and the kit's rule that no run starts on a machine that owes a
stop (orders 129 and 130), under the owner's answer to MC4-002 of 2026-11-12: every runtime provider
Conduit ships carries out a stop it still owes on a machine before it starts any run there, and its
contract says so. A63-RP1 is order 127. Since order 127, A63-RP1 also wires the kit's gate and leads
each run's process group with a guard, so no run outlives the provider; A63-RP2 keeps the external
drainer of owed stops and the separate operating-system users; registry packages are A63-RPN.

**Lane SP's checkpoint (2026-11-12, at the admin's word, for the owner's usage limits).** Lane SP
(ruling 71: plans, providers and scenario servers, model checking, A74/S3, scenario runs and
probes) stopped here; nothing of the lane runs, and every unfinished branch is on GitHub.

- **Built and merged since the status above:** the kit's reasons for a failed staging or start (138)
  and its rule for a start after its own owed stop, under order 081 (141); SSH-R's durable directory
  outside the engine's lifecycle (145); S2's adapter row (139); S4's scripted client SCR1 (142 and
  148) and effect servers S4-SRV1 (143); and SSH-R's stop-drainer (146, 149 and 150), which outlives
  the provider. **Stopped unmerged:** S4-SRV2 (order 144), its branch `wo/144-s4-srv2-hook` pushed,
  with two musts left for a finishing order, named in its Record.
- **The queue, in order:**
  1. S4-SRV2's finishing order, stacked on `wo/144-s4-srv2-hook`, for the two musts its Record
     names;
  2. SSH-R's key sweep for retired providers (order 147, written, not dispatched);
  3. SSH-R's loopback-`sshd` tier, an order not yet written (a `loginctl` stand-in, `PermitUserRC
     no` and a `PidFile` in its temporary directory; a `drainInterval` above five minutes, which
     order 146's worker found the configuration allows);
  4. S4-SRV1b (the plain partial task) and S4-SRV2b (the hook's onward request and E20's test),
     then SCR2 and S4-H, in [S4's plan](../../experiments/015-s4-partial-setup/plan.md);
  5. the other scenario servers the plans name: S5-SRV, S6-SRV, S9-SRV (reusing S4-SRV2's hook),
     S8-C, and A63's KC, DR and GS;
  6. S2-LOG, Linear's OAuth login through the Interface (A82), after the core lane's 048a, 048b and
     051b;
  7. model checking: MC4's P1 registered again, and the models of orders 109, 110, 112 and 113
     reviewed against order 081's promises, after 077 lands; the MC2 rerun, after 077; MC9 after
     batch 2; SV1 after 071; MX0 at the admin's word.
- **Open for the owner:** S3's cut, heal and loss method (S3-OF, options A to C in
  [S3's plan](../../experiments/014-s3-two-machines/plan.md)); and the plain Linear server for
  S2's runs, chosen with the owner before them, to the requirements in
  [S2's plan](../../experiments/013-s2-shared-login/plan.md).
- **Waiting on others:** SSH-M on 049a1 in `s3/m2`; PRV-021's three cases in the admin's next
  spec order.

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
| Files versus merged work | `git log` includes 070–072, 076–077, 079–080, 049a1, 050a and 082 titles. | The tree still clones State (`core/src/lib.rs:303–305`), has one manifest list and no shell `list.v1`; the named implementation changes are absent. 081's operation already forbids duplicate runs, but its promised lifecycle/core/vector changes have not landed. 038c's alignment is also still an order. |

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
| `runtime-provider.v1` | A63-RP local process on OS process groups; A63-DR on Docker CLI/daemon. SSH-R is a third, on OpenSSH/systemd. | S2 D20, S3 isolated consumers, S10 T3/T5: ADDR and DSP, and `tailscale serve` set up after the Mac's tailnet rename ([ruling 77](../proposals/ruling-77/README.md)) for engine use; container Agents also need 050. |
| `machine-provider.v1` | A63-MP on local OS machine reports; SSH-M on OpenSSH, the second-host provider that [ruling 77](../proposals/ruling-77/README.md) selects. A Tailscale provider is a later candidate. | S3 supplies machines’ second-implementation evidence: 049a-1, 049b, DSP, 051c and proposed A63-MDIS; see admin C. |
| `package-source.v1` | A63-PS on filesystem directory reads, claiming `file`; A63-GS on Git, claiming `git+https`. | S10 T4 (S7 only if fetching HTTPS archives): PSD, immutable claims and fresh caches. Unclaimed `file`/`https` remain engine fetches. |
| `secret-provider.v1` | S2/S10 first: A63-KC on macOS Security APIs; shipped A63-FS on filesystem files. S10 second: A63-1P on 1Password CLI after A73, else A63-FS. | S2, S3 and S10 T2/T5: SEC-D; B2-S/C/H before scenarios depend on driven-Agent restrictions. File/keychain also pass the same suite. |

Order 075 is merged (`55cf747`): an Agent Type states the reach of its client’s own tools.

## The orders

A scenario vector goes in the core or shell order that makes it pass if that order stays at most L;
otherwise an admin vectors order immediately follows it and depends only on the core and shell orders
its cases need, never on a lane package, owner step or run. A provider-contract case goes in lane 8’s
A63-VR, A63-VM, A63-VP or A63-VS suite in [the provider plan](PROVIDERS.md).

Sizes include tests before pipeline fixes: L about 550–900 lines, M about 300–600.
Suffixes name independently dispatchable parts; A63-RK/H/RP/MP/DR and PSD remain shared IDs.
022 is reserved for A63 alone; no other order in these plans takes it.
Packages test themselves with scripted MCP clients, as agent-codex does; none waits for A63-H1.
H1 gates suite qualification only. Every crate has its own Cargo.lock, pinned toolchain and ignored output.

| ID | Kind | What it does | Promises | Depends on | Size | Who | Starts |
| --- | --- | --- | --- | --- | --- | --- | --- |
| A63-RK1 | harness | Shared runtime server and backend interfaces | PRV-005/007–012/019/021/022 | — | L, 650–850 lines | lane 8 | now; first runtime order |
| A63-RK2 | harness | Shared per-run gate and liveness | PRV-025/028, IFC-017 | A63-RK1 | L, 650–900 | lane 8 | after RK1 |
| A63-H1 | harness | Standalone launcher, MCP driver, format/index and reports | PRV-002/005, IFC-037 | — | L, 650–850 | lane 8 | now |
| A63-H2a | harness | Private trust fixtures and gate refusals | PRV-025/028, IFC-017 | A63-H1, A63-OUS | M, 400–600 | lane 8 | after dependencies |
| A63-H2b | harness | Fault controls using A74's silent cut | PRV-025/028 | A63-H2a, S3-A741 | M, 350–550 | lane 8 | after dependencies |
| A63-VR | vectors | Runtime suite, including S3 V5 and S10 V2 | PRV-007–012/019/021/022/025/028 | A63-H2; safety cases after SV1; repeated-run cases after 081 | M, 400–600 | lane 8 | per case readiness |
| A63-VM | vectors | Machine reports, placement and enrollment suite; stdio, no gate | PRV-014/015, ENR-002/004–010 | A63-H1, 049a-1, 049b, admin C; safety cases after SV1 | M, 400–600 | lane 8 | after dependencies |
| A63-VP | vectors | Exact bytes for each claimed URL scheme | PRV-017 | A63-H1; safety cases after SV1 | S, 200–350 | lane 8 | non-safety cases after H1 |
| A63-VS | vectors | Store contract suite | SEC-005/006/019/022 | A63-H1; safety cases after SV1 | S, 200–350 | lane 8 | non-safety cases after H1 |
| A63-RP1 | provider | Process operations, the kit's gate, a guard per run, running repeated-run case (order 127) | PRV-007–012/019/022/025/027/028 | A63-RK1, A63-RK2, orders 129 and 130 | L, 650–850 | lane 8 | dispatched as order 127 |
| A63-RP2 | provider | External drainer of owed stops after retirement, separate OS principals | PRV-021/025 | A63-RP1, 081, A63-OUS | M, 400–650 | lane 8 | after dependencies |
| A63-RPN | provider | Registry packages (npm over stdio) in the local-process provider, as DL16's runner runs them | PRV-007/012, MAN-010 | A63-RP1 | M, 400–600 | lane 8 | after RP1 |
| A63-MP1 | provider | Local machine report with id and placement | PRV-014/015, ENR-002 | — | M, 300–450 | lane 8 | now |
| A63-MP2 | provider | Activated machine contract qualification | ENR-002/007/008, PRV-014/015 | A63-MP1, 049a-1, 049b, admin C qualification settled | L, 550–750 | lane 8 | after dependencies |
| A63-PS | provider | Local-directory source, file scheme | PRV-016/017 | — | M, 300–450 | lane 8 | now |
| A63-FS | provider | Shipped file store | SEC-005/006/022 | — | S, 200–350 | lane 8 | now |
| A63-KC | provider | Logged-in-session keychain store | SEC-005/006/022 | S2-O throwaway test keychain, A63-OKC | L, 550–750 | lane 8 | after owner setup |
| A63-DR1 | provider | Docker operations and Type | PRV-007–012/019/027 | A63-RK1 | L, 650–850 | lane 8 | after RK1 |
| A63-DR2a | provider | Kit gate, whole-tree stop and durable recovery | PRV-021/022/025/028 | A63-DR1, A63-RK2, 081, Docker setup | L, 600–850 | lane 8 | after dependencies |
| A63-DR2b | provider | Container Agent profile and second-machine channel | PRV-006/011/022/029, ACC-027 | A63-DR2a, 050; second-host qualification after S3-O2 | M, 350–550 | lane 8 | after dependencies |
| A63-GS | provider | Git source, git+https only | PRV-016/017 | — | M, 400–600 | lane 8 | now |
| A63-1P | provider | 1Password store | SEC-005/006/022 | A73 and its approvals | M, 400–600 | lane 8 | after A73 |
| PSD1a | core | Source Type and immutable scheme admission | PRV-013/016 | PIN, 070, SV1 | M, 400–600 | admin | after dependencies |
| PSD1b | core | Source selection, serialized install and archive decisions | PRV-017, PKG-025–027 | PSD1a, 082 | M, 400–600 | admin | after dependencies |
| PSD2 | shell | Source MCP dispatch and verified bytes | PRV-005/017, PKG-026 | PSD1, DSP | L, 650–900 | admin | after dependencies |
| PSD3 | shell | Unclaimed HTTPS fetches | PRV-003/017, PKG-026 | PSD1, DSP TLS support | M, 400–600 | admin | after dependencies; not PSD2 |
| A63-MDIS | shell | Proposed engine place/list-machines dispatch beyond 051c's report loop | PRV-005/015 | PIN, 049a-1, DSP, SV1 | M, 350–550 | admin | after dependencies |
| A63-OUS | owner | Distinct OS users for process-provider and H2 tests | PRV-025 | — | owner step | owner | before their principal tests |
| A63-OKC | owner | Authorize keychain binary on item's partition list | SEC-006 | S2-O test keychain, built test binary | owner step | owner | before KC real reads |

All provider packages follow [agent-codex's form](../../packages/agent-codex/README.md).
Resource Types offer their named provider capability and declare Conduit's extension; secret Types
have no controls or other capability. Package checks: release tests, clippy with warnings denied,
fmt and archive/schema checks. Missing tools/hosts are untested cells, never passes. Engine scopes
also run engine checks, vector delta and experiment 011 for core/loader changes. No scope changes spec.

**A63-RK1 / RK2 — What the runtimes share.** The library is its own crate,
`packages/runtime-provider-kit/`, outside the engine workspace. RK1 has no dependency; RK2 supplies
its gate. A63-RP, A63-DR and SSH-R depend on this kit by path, never on each other's provider crates.
This plan reads “self-contained” (IN_FORCE topic 5) as each crate keeping its own lockfile and toolchain.

- **Backend calls:** start from a staged item and return a duplex byte channel to the run's stdio;
  stop the whole tree; report state. File staging and the state store (run ledger and owed stops)
  are backend interfaces too, since SSH-R stages on the other machine. RK1 maps MCP to them.
- **Lifetime:** follow SSH-R's model. A run is tied to its channel and ends when the channel ends;
  stop every run the provider does not hold at startup; a configured bound is the backstop.
  RP uses process groups, with no launchd/systemd supervisor in its setup or tests.
- **Gate:** RK2 supplies one listener per run on its own port, serving that run's certificate.
  DSP checks no host, and an IP URL has no SNI, so one listener cannot select runs' certificates.
  Verify the engine's proof before forwarding any bytes. A separate gate principal protects the backend.
- **Silent cut:** PRV-025's bound runs from the enforcement point's connection, checked by the
  gate's own liveness probe: TCP keepalive, since DSP's client has no HTTP/2.
- **Owed stops:** survive provider restart, instance removal and provider retirement (PRV-021).
  Each backend names an external mechanism that drains them after the provider server never runs again.
  A successful stop stays true, cancels an in-flight start and is followed by no restart (081).
  Contracts, OS principals, storage locations and actual tree-stop mechanisms remain backend-specific.

**A63-H1.** Create `experiments/022-provider-contracts/`, its standalone harness crate, contract
suite directories, own index and `provider.yaml` form documentation. Add its A63 row to
experiments/README.md and the roadmap register in that future order. No engine workspace entry,
conformance case form or seam with the core runner; the runner and manifest remain the admin's.
Launch selected pinned items with bindings, MCP discovery and proper provider-call metadata;
secret reads use engine-read metadata instead. Test launcher failures and malformed replies locally.

**A63-H2a / H2b.** Same crate, own fixture gate, never through A63-RP. H2a issues private roots,
per-run SVIDs and engine proofs; tests absent/instance/wrong-audience proof, TLS roots/instance,
URL takeover before bytes, and backend exclusion with A63-OUS's distinct OS users. H2b reuses
[S3-A741](../../experiments/014-s3-two-machines/plan.md)'s configurable silent cut rather than
building another; adds fault scheduling and liveness-bound observations. Bounded children and synthetic
markers only; keys/values never enter reports. No provider or production identity edits.

**A63-VR / VM / VP / VS.** Cases under `experiments/022-provider-contracts/` and their own index,
not `conformance/`; kind vectors here means these provider cases. All four suites and H1/H2 are lane 8's.
Use stable id/title/contract, promises/profile, tool features, fixtures, ordered calls/faults, captures
and exact assertions. Descriptors pin item/Type/config/tool/version; reports pin revision/suite/digests,
failures and untested cells. Unknown assertions fail. Keep expected results once per contract.
A safety case carries the safety-promise field required by ruling 68.2, named as SV1 names it once
SV1 merges. Non-safety VP/VS cases need not wait for SV1. No new field name is invented here.

VR covers run/inspect/stop, machine mismatch, supported registries/transports, relative commands,
identity/secrets, current URL, proof/TLS, silent cuts and durable owed stops. It owns S3 V5
`s3-gate-identity` and S10 V2 `s10-runtime-equivalence`, both safety PRV-025, on the runtime pair
(and SSH-R for S3). Only repeated-run cases wait for 081; RP1 already tests the running case,
while uncertain-state qualification waits. A whole-tree stop is a parameter each runtime's contract
declares, not a universal suite assertion. Each contract's declared stop must be tested as such.
VM is reached over stdio without H2/gates; it covers reports, placement, identity, enrollment,
admission/refusal/expiry/removal, omitted/unreachable versus gone and fixed labels. Positive local
qualification waits for admin C; a fixture alone is no real-tool second implementation.
VP asks only for exact bytes for a URL whose scheme the provider claims, under PRV-017: corrupt
archives, unsupported schemes and a claim's lifetime are engine checks in PSD, not VP assertions.
VS covers locator reads, unreadable/locked/missing failures without fallback or echo, and no controls
or other capability. S10 V1/V3 pair equivalence is recorded here in VS/VP; their safety assertions
SEC-007/PKG-026 stay with the engine. No safety promise is changed.

**A63-RP1 / RP2.** `packages/provider-local-process/`, Type `local-process.v1`, runtime capability.
D15: own MCP/backend mapping, staging and durable ledger; kit server/gate; OS process groups;
private run files/homes and owed-stop intents. Operator supplies private storage and distinct run/gate
principals (A63-OUS), no launchd/systemd supervision. RP1 tests operations and running repeated run;
RP2 connects the kit gate, recovery/startup sweep and principals. Its external stop-drainer helper
survives server retirement, reads durable obligations and stops process groups; test it after server exit.
Declare pinned npm/stdio and absolute commands, unsupported registry/transport rejection and cleanup.
Engine use needs ADDR and DSP, and `tailscale serve` set up after the Mac's tailnet
rename (ruling 77); no provider-run qualification through the engine can reach its HTTPS address before that is set up.
DL13's gate TLS and credential delivery stay a limit until DSP. Direct MCP suite tests use harness trust fixtures.

**A63-MP1 / MP2.** `packages/provider-local-machine/`, Type `local-machine.v1`, machine capability.
D15: report normalization/selectors/enrollment; OS facts/key storage; stable identity/labels/decisions;
operator supplies lab evidence and allowed labels. MP1 stays local and emits machine `id` now:
PRV-014 already requires ENR-002's ID. MP2's qualification is settled before dispatch (admin C).
Test canonical labels, selectors, key continuity, accept/remove/expiry and local-node exceptions.
Engine use also needs A63-MDIS beyond 051c's reports; both packages eventually pass A63-VM.

**A63-PS.** `packages/provider-local-directory/`, Type `local-directory.v1`, source capability, file only.
D15: URI/root containment/base64 mapping; filesystem reads; configured root, no archive cache;
operator supplies pinned archives under `<lab>`. Test disposable directories/escapes/symlinks and exact
bytes, including corrupt bytes returned unchanged. A63-VP qualifies contract bytes; PSD checks archives.

**A63-FS.** `packages/provider-file/`, Type `file-secret.v1`, store capability.
D15: one-file locator/safe read/errors; OS permissions; no value cache/copy; private throwaway file.
Test missing/unreadable and no fallback/echo; pass VS. Ship with engine; engine use waits SEC-D.
SEC-020 binds the engine and is not listed as this package's obligation.

**A63-KC.** `packages/provider-keychain/`, Type `keychain-secret.v1`, store capability.
D15: explicit locator, disabled interaction and bounded read; Security APIs/ACLs; no value cache;
operator unlocks S2-O's throwaway keychain and authorizes this binary on the item's partition list
(A63-OKC, experiment 005). State session/locator/failed-read/upgrade reauthorization behavior.
Test locked/not-trusted/missing and rebuilt binary under [005's safety rules](../../experiments/005-macos-keychain/README.md#safety):
compare default/search list byte-for-byte, never read others, close stdin, bound children (30s;
cargo 600s), lock/remove throwaways; no CLI secret read. Pass VS; SEC-D enables use, not A22.

**A63-DR1 / DR2a / DR2b.** `packages/provider-docker/`, Type `docker.v1`, runtime capability.
D15: backend/CLI mapping and kit gate; Docker containers/kill/inspect; per-ID homes/volumes,
daemon identities/owed stops; operator supplies context/images and an external stop-drainer helper.
The helper drains durable stops after server retirement, including reconnect to the second daemon;
channel loss and configured lifetime bound stop runs too. Test successful stop cancelling pending start.
DR1 tests mapping; DR2a gate/daemon faults, declared tree-stop and durable recovery; DR2b after 050
qualifies Agent homes/browser callbacks and states own-tool limits under PRV-006 as amended by 075.
Gate stays outside the container: “no gate key is mounted” means the gate's own key. Deliver the
run's `tls.key` to it under PRV-022; mount no Docker socket or host identity directory in Agents.
Support a Docker daemon/context on the second machine and PRV-011's confidential channel to it;
S3-O2 prepares that endpoint. Host-relative wrappers do not run in containers: 050's owner supplies
Agent items, S2-L its service items, and S3-S1ITEM is proposed for S1's. Qualify each architecture,
private home and independent login/callback; no copied login. Record URL/trust translations in D15/D16.
Engine use/qualification waits ADDR and DSP, and `tailscale serve` set up after the Mac's tailnet rename (ruling 77), and 050 for the Agent profile.

**A63-GS.** `packages/provider-git/`, Type `git-source.v1`, source capability, git+https only.
D15: pinned commit/blob selection/base64; Git transfer; disposable object cache, no credential copy;
operator supplies identical archive blobs in a disposable HTTPS repository and Git's credential helper.
No recompression substitutes bytes. Offline local-repository tests precede real HTTPS qualification.
Suite pass needs A63-H1/VP, not PSD; engine use waits PSD. Errors reveal no credential.

**A63-1P.** `packages/provider-1password/`, Type `onepassword-secret.v1`, store capability on op.
D15: locator/bounded noninteractive read; 1Password custody; no value cache; throwaway account/vault.
A73 (S10's lane-11 probe) precedes qualification, with separate owner approval before desktop access.
Bootstrap credential is another secret Resource delivered through SEC-D. Pass VS on observed modes.

**PSD1a / PSD1b.** Core/loader admission and fetch boundary. Hot files: core registration.rs/requests.rs,
loader and packages modules for PRV-016's Type check; conformance runner lib.rs/own manifest pieces;
no access.rs/lifecycle.rs. 1a checks system ownership, unique case-insensitive claims, immutable schemes
and illegal Type controls, with S10 V4 `s10-scheme-claim-admission` (safety PRV-013).
1b selects providers/fallback, serializes address/version after 082 and checks archives before use;
keeps S10 V3 `s10-package-source-equivalence` safety PKG-026 here. Own fixtures/completions test
unready/collision/concurrent-install/digest refusal; no S10-H1 or A63-GS dependency. Split either
vector payload to an immediately following admin vectors order if it would exceed L.

**PSD2 / PSD3.** Shell fetch/tests/CONFORMANCE updates; hot shell lib.rs/runner.rs; no server.rs,
identity.rs, core hot files or CLI tables. Scripted MCP source and local HTTPS fixtures inside engine
tests, never a lane source package. PSD2 reuses DSP metadata, decodes base64, checks digest before
extraction, maps missing/unready to PKG-024. PSD3 needs PSD1 and DSP TLS, not PSD2: normal TLS,
credential-free URLs and redirects. Test precedence, fallback, bad TLS/base64/digest, cache/concurrency.

**A63-MDIS.** Proposed admin engine dispatch for place.v1 and list-machines.v1 beyond 051c's report
loop; PRV-015 is a safety promise in the catalogue. Hot shell runner.rs/lib.rs/server.rs; core
requests.rs only if the dispatch intent needs it; runner lib.rs/manifest pieces for its safety cases.
Use internal scripted machine servers, no lane package dependency; qualification remains A63-VM.
Admin must place this additional proposal in S4's merge sequence before dispatch; no behavior invented.

**A63-OUS / A63-OKC.** Owner creates distinct users for RP/H2's actual principal tests and separately
authorizes the keychain binary on its test item's partition list. No credentials or local names in
records; unavailable principal/ACL setup is an untested cell, never a passing test.

## Shared with other plans

| Shared ID, not owned here | Needed behavior / sequencing |
| --- | --- |
| 070, 071; SV1–SV4 (lane 6) | State/runner seams and index pieces; safety field, existing tags, S1 missing cases, and later orders' own tags. A63 owns no S1 safety work. |
| PIN; 082; 081; 077 | Freeze through 053 first; serialized install rule before PSD; duplicate-run/lifecycle fixes before fault qualification; fenced reconciliation before relying on failed reconciliation. Files alone are not completion. |
| AGT, ADDR, DSP; SSH-R | Windows Agent item, HTTPS audience, certificates/generic provider calls; SSH runtime is a third runtime, not a substitute for process/Docker pairing. SSH-R's gate/owed stops are tested by the same runtime suite. |
| 049a-1, 049b, 049a-2, 051c; SSH-M | Activate both patches/core, fixture vectors, pending enrollment actions, root panel/report loop; SSH-M, ruling 77's answer to M2-1, supplies real second-host evidence. Lane 9 owns this work. |
| 050/050a/076, 048a-1/2, 048b-relay/join, 051b | Separate homes, hooks, account check, pending sign-in relay/commands and independent browser sign-in; packages currently lack them. |
| SEC-D; FLT-D; S2-F; S2-L (order 117) | Secret authorization/delivery/records/recovery, onward identity/filters/support check, issue/deadline contracts, Linear variants/stub. D01/D09/D20 and S2's traces stay there; A6 refresh/discovery comes from S2, loss from S4. |
| B2-S, B2-C, B2-H | Driven-Agent secret rule in spec, core enforcement and shell delivery; drafts merge only after owner judges the rerun's bar. |
| S3 work (117); S7/S10 work (119) | Real two-machine faults and machine-suite report; repeated-argument/D01 evidence; placed S10 V1–V6 (VR owns V2), D15/D16 and twelve counted replays. This plan supplies packages/suites, no counted run. |

## The order of the work

Dispatch S3-A741 first for reusable silent cuts, and A63-RK1 as the first runtime order with no
dependency. H1, FS, PS, MP1 and GS can start now; packages test with their own scripted MCP clients,
not H1. KC waits S2-O/A63-OKC; RP1/DR1 follow RK1. RK2 precedes RP2/DR2a, with real OS/Docker
setup for those tests; DR2b follows 050 and joins S3-O2 for the second daemon. H2a uses its own
gate, H2b reuses A741. H1 gates suite qualification; only safety cases wait SV1, only VR's
repeated-run cases wait 081. MP2 waits activated contracts and settled qualification C.

Critical joins: PIN → ADDR → DSP plus `tailscale serve` set up after the Mac's tailnet rename (ruling 77) → runtime engine use; PIN → 049a-1/049b →
SSH-M → VM report; PIN/SV1 → PSD1a → 082/PSD1b → PSD2, with PSD3 independently
joining PSD1/DSP TLS. SEC-D/050/B2-S/C/H, S2's D09, paired suite reports and owner sign-ins join
[S10's freeze and twelve replays](../../experiments/021-s10-swapped-providers/plan.md).
A73's branch remains provisional as that plan states.

Use [S4's proposed hot-file merge order](../../experiments/015-s4-partial-setup/plan.md#for-the-admin):
the admin interleaves PSD1a then PSD1b in core, PSD2 and PSD3 in shell; A63-MDIS needs an assigned
slot too. Provider/harness crates stay outside those engine hot files and outside S1's pinned snapshot.

## For the admin

- **A — decided by the admin, 2026-11-11:** A63-H lives in its own crate outside engine/ at
  `experiments/022-provider-contracts/`. Its suites/index use provider.yaml with no seam with the
  conformance runner's case form. H1/H2 and the four suites are lane 8's; runner/manifest stay admin's.
  H1 creates the directory and its A63 row in experiments/README.md and the roadmap register.
- **B — decided, 2026-11-11:** option (b) of order 120 stands. Packaged local providers pass the
  suites alongside second providers; built-ins remain S10's accepted first configuration, with no
  built-in adapter or claim that a package's report is a built-in's.
- **C — decided by the admin, 2026-11-11:** MP1 stays local; settle MP2's qualification before
  dispatch. Withhold full paired evidence until the same suite passes both; 049b is only a fixture.
- **D — decided by the admin, 2026-11-11:** SSH-R and SSH-M are separate provider crates
  (`packages/provider-ssh-runtime/` and `packages/provider-ssh-machine/`), under ruling 40.
  SSH-R reuses A63-RK by path, not RP or H. Both are unblocked: [ruling 77](../proposals/ruling-77/README.md)
  selects both for the Windows computer, and SSH-R's runs use one account, `conduit-run`, without sudo;
  its contract states that PRV-025's isolation is unmet until each Agent has its own user.
  SSH-R’s external stop-drainer helper survives server retirement and retries SSH to stop the
  recorded remote systemd units on next contact. **Built (2026-11-12):** orders 137 and 140
  deferred it at the admin's word; order 145 moved SSH-R's state into a durable directory outside
  the engine's lifecycle, under the admin's decision under PRV-021, and orders 146, 149 and 150
  built the drainer, at most one per instance, kept alive by the provider. Order 147's sweep of
  retired providers' keys is still to come. Channel lifetime and the configured bound
  remain backstops; stop success cancels an in-flight start and permits no restart (081).
- **E — admin's decision, 2026-11-11:** Planned on the admin's reading of ruling 75 D4 (2026-11-11): it limits the optional replays of S4, S5, S6 and S9 only. The owner confirmed it on 2026-11-12 ([ruling 75's later answer](../proposals/ruling-75/README.md#later-answers)).
- **Uncovered dispatch, proposed:** accept A63-MDIS for place/list-machines beyond 051c's report
  loop, or assign that scope to an existing admin order. **Recommendation:** its own M order,
  with PRV-015 safety cases and an explicit slot in S4's canonical merge sequence.

## For the owner

On 2026-11-12 the owner answered the M2 questions: M2-1 to M2-4 in [ruling 77](../proposals/ruling-77/README.md), M2-5 in
[ruling 72's later answer](../proposals/ruling-72/README.md#later-answers), and M2-7 needed no ruling (WSL keeps its default
memory limit). M2-5 is on demand, whenever the computer is on and the owner has not paused it.
M2-1: SSH-M and SSH-R, with Tailscale the network.
A Tailscale provider is a later candidate. M2-2: one run user,
`conduit-run`, without sudo. M2-4: the distro locked down (ruling 77), with Windows interop, `appendWindowsPath` and automount off in Ubuntu-24.04: nothing in the distro starts Windows programs, has the Windows path or mounts Windows drives.
M2-3: `tailscale serve` on the Mac, with a Let's Encrypt certificate, after the Mac is renamed in the tailnet;
it terminates TLS for the HTTPS address every provider run receives through DSP/079, in front of order
079's loopback listener (`--https-listen`). Engine use and engine-facing qualification of A63-RP/DR
therefore wait on ADDR and DSP, and `tailscale serve` set up after the Mac's tailnet rename (ruling 77); DL13's gate TLS and
credential delivery stay a limit until DSP. Direct suite tests use harness fixtures.
A63-OUS asks for distinct OS users before RP/H2 principal tests; A63-OKC asks for binary
partition-list authorization on S2-O's throwaway item, following experiment 005.

## Limits

Planned D15 fields and estimates are not observations. Package tests before dispatch do not prove engine
integration, gate exclusion without actual principals, or swappability before both real tools pass the suite.
S10 preserves its frozen tasks/access/results/attribution and edit/replacement ledger (D16); no live migration,
daemon keychain support or local enforcement is claimed. A63 closes only with four paired contract reports
and S10 passing every accepted swap, with S3's machine evidence. A22/A73 and newly found language gaps
remain with their owners, and the M2 choices are ruling 77's; any required spec fix is proposed to the admin before use.
