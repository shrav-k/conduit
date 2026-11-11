# 005 — macOS keychain, read with no person present

Status: executed 2026-11-06, and run again by a review the same day with the same outcome in
every case ([re-run](#re-run-by-the-review)). Exploratory Rust code under the exploratory-code
ruling, which is ruling 3 of the [ruling log](../../docs/AUDIT.md#ruling-log) in the plan audit.
It was started on the owner's answer recorded
[after ruling 40](../../docs/proposals/ruling-40/README.md#after-the-ruling): on throwaway
keychains, never the login keychain. It probes a store that a provider package
would read (candidate in action A63 of the [action items](../../docs/AUDIT.md#action-items)). It
is not a provider package, a Conduit engine or conformance evidence.

## Question

Probe E10 of the [standing probes](../../docs/archive/plan-audit-history.md#standing-probes-and-comparisons):
"Does Keychain work unattended? Access from a daemon, a rebuilt unsigned binary and a locked
session." Put as six questions about what a macOS keychain secret store package would face when
an engine reads a secret for an instance with no person present:

1. A binary on an item's access list reads it from an unlocked keychain, with user interaction
   disabled and no terminal. Does it succeed silently?
2. A binary not on the access list reads it. What is returned?
3. An item made for any application (`security add-generic-password -A`). Does an unlisted
   binary read it?
4. The trusted binary is rebuilt with changed code, ad-hoc signed as cargo produces it. Is it
   still trusted?
5. The keychain is locked. What does a read return, and does it succeed after the keychain is
   unlocked through the API with its password?
6. How long does a read take?

## Method

A Rust probe, `src/bin/probe.rs`, drives two reader binaries built from the same library,
`src/lib.rs`:

- `reader`, the trusted binary, whose code includes a build tag fixed at compile time from
  `PROBE_READER_TAG`.
- `other-reader`, a different binary, never put on any access list.

The probe builds both with cargo into `target/scratch/build/`, as cargo produces them: the linker
signs them ad hoc, with no team identifier. It then creates two throwaway keychains with
`SecKeychainCreate` (through the `security-framework` crate), each with a password it generates,
under `target/scratch/run-<time>/`. They differ in one thing, the database format:

| Keychain | Path under the run directory | Database format |
| --- | --- | --- |
| `legacy` | `e10-legacy.keychain` | The older format. No partition IDs. |
| `partition` | `Library/Keychains/e10-partition.keychain-db` | The format the login keychain uses, which adds an integrity entry and a partition list to each item's access list. |

The format depends on the path. In Apple's Security source,
[`CommonBlob::getCurrentVersionForDb`](https://github.com/apple-oss-distributions/Security/blob/main/OSX/libsecurityd/lib/ssblob.cpp)
creates a keychain in the newer format (`version_partition`) only when its path contains
`/Library/Keychains` and does not start with `/Library/Keychains` or `/System/Library/Keychains`.
[`KeychainImpl::hasIntegrityProtection`](https://github.com/apple-oss-distributions/Security/blob/main/OSX/libsecurity_keychain/lib/Keychains.cpp)
turns on integrity protection, and so partition IDs, only for that format. The first run of the
probe used only a keychain directly in the scratch directory; its items had no partition entries,
which led to adding the second keychain. The recorded runs confirm the difference twice. The
`security` tool printed the database version of each keychain, 256 for `legacy` and 512 for
`partition` (the output of `set-generic-password-partition-list` in the results). The recorded
access lists show it too: only the items in `partition` carry the integrity and partition entries.

In each keychain the probe adds four items, with generated values:

| Item | How it was added | Recorded access list, `legacy` | Recorded access list, `partition` |
| --- | --- | --- | --- |
| `listed` | `security add-generic-password ... -T <reader> <keychain file>` | Decrypt: `reader` | Decrypt: `reader`; partitions: `apple-tool:` |
| `listed-partition` | As `listed`, then `security set-generic-password-partition-list -S apple-tool:,cdhash:<reader> -k <keychain password> <keychain file>` | Decrypt: `reader` | Decrypt: `reader`; partitions: `apple-tool:`, `cdhash:ebf84c8d…` |
| `api-created` | By `reader` itself, through `SecKeychainAddGenericPassword`, with default access | Decrypt: `reader` | Decrypt: `reader`; partitions: `cdhash:ebf84c8d…` |
| `any-app` | `security add-generic-password ... -A <keychain file>` | Decrypt: any application | Decrypt: any application; partitions: `apple-tool:` |

The partition ID of `reader` was taken from `api-created`, whose partition list the system
sets to the creator's: `cdhash:` and the binary's code-directory hash. In `legacy`, which has no
partition list, `set-generic-password-partition-list` exited 0 and recorded nothing: its exit
status does not say whether a partition list was set. The probe reads each access list through
`SecKeychainItemCopyAccess` without reading the value.

Every read is made by a reader started with stdin and stdout closed, stderr to a file, and
`setsid`, so it has no controlling terminal. Before opening the keychain the reader calls
`SecKeychainSetUserInteractionAllowed(false)` and confirms that it took effect, so a read that
would need a dialog fails instead. It searches only the throwaway keychain it is given, and
reports the result code and the SHA-256 of the value, never the value. The probe compares that
digest with the digest of the value it generated.

The order of the run: questions 1, 2, 3 and 6 on each keychain while unlocked; question 5 on
each (lock with `SecKeychainLock`, read, unlock with `SecKeychainUnlock` and the keychain's
generated password, read); then cargo rebuilds `reader` with another build tag, in place at the
same path, and question 4 is asked on both keychains.

### Safety

- Before anything else the probe records the output of `security list-keychains -d user` and
  `security default-keychain -d user`. It compares both byte for byte after creating each
  keychain, after adding the items, after each lock and unlock, and at the end, and stops if
  either differs. Apple's source states that `SecKeychainCreate` adds a new keychain to the
  search list only if it is the login or System keychain
  ([`shouldAddToSearchList`](https://github.com/apple-oss-distributions/Security/blob/main/OSX/libsecurity_keychain/lib/StorageManager.cpp)).
  It tests the login keychain by whether the path contains `/login.keychain`, and the throwaway
  file names do not.
- No other keychain is opened, read, listed, locked, unlocked or deleted. Every `security`
  command names the throwaway file as its last argument, and the probe refuses one that does not
  lie under the run's scratch directory. The probe never runs `security find-generic-password`,
  which cannot disable user interaction.
- The probe itself also disables user interaction. Every child process runs with stdin closed and
  a timeout: 30 seconds, and 600 seconds for cargo.
- At the end each throwaway keychain is locked and its file removed, which avoids the search
  list code in `SecKeychainDelete`. Then the run directory is removed.

## Results

Run on 2026-11-06 from 18:23:56 to 18:23:59 UTC, on an Apple M2 Max, macOS 26.6.2 (25G83),
rustc 1.97.1, security-framework 3.7.0. The probe ran under Claude Code, with no terminal, in the
user's logged-in graphical session: `launchctl managername` printed `Aqua`. `cargo test --release`
passed: six tests of the helpers, none of which touches a keychain. The raw output is
[results.json](results.json), and the console output is [results.txt](results.txt).

Every read was made with user interaction disabled in the reader, as a session leader with no
controlling terminal (opening `/dev/tty` failed with "Device not configured"). No reader timed
out, and no read waited: the longest refused read took 7.5 ms. Every successful read returned
the generated value, by its SHA-256. Every refusal returned the same code and message:

```text
status -25293 errSecAuthFailed "The user name or passphrase you entered is not correct."
```

No read returned errSecInteractionNotAllowed (-25308).

| Case | Reader | Item | `legacy` keychain | `partition` keychain |
| --- | --- | --- | --- | --- |
| 1a | `reader` v1 | `listed` (`-T reader`) | read, 0 | **refused, -25293** |
| 1b | `reader` v1 | `listed-partition` | read, 0 | read, 0 |
| 1c | `reader` v1 | `api-created` | read, 0 | read, 0 |
| 2a | `other-reader` | `listed` | refused, -25293 | refused, -25293 |
| 2b | `other-reader` | `listed-partition` | refused, -25293 | refused, -25293 |
| 2c | `other-reader` | `api-created` | refused, -25293 | refused, -25293 |
| 3a | `other-reader` | `any-app` (`-A`) | **read, 0** | refused, -25293 |
| 3b | `reader` v1 | `any-app` (`-A`) | read, 0 | **refused, -25293** |
| 5a | `reader` v1 | `api-created`, keychain locked | refused, -25293 | refused, -25293 |
| 5b | `reader` v1 | `listed-partition`, keychain locked | refused, -25293 | refused, -25293 |
| 5c | `reader` v1 | `any-app`, keychain locked | refused, -25293 | refused, -25293 |
| 5d | `reader` v1 | `api-created`, unlocked again through the API | read, 0 | read, 0 |
| 5e | `reader` v1 | `listed-partition`, unlocked again | read, 0 | read, 0 |
| 5f | `reader` v1 | `any-app`, unlocked again | read, 0 | refused, -25293, as in 3b |
| 4a | `reader` v2, rebuilt | `listed` | refused, -25293 | refused, -25293 |
| 4b | `reader` v2, rebuilt | `listed-partition` | refused, -25293 | refused, -25293 |
| 4c | `reader` v2, rebuilt | `api-created` | refused, -25293 | refused, -25293 |
| 4d | `reader` v2, rebuilt | `any-app` | read, 0 | refused, -25293, as in 3b |

During 5a to 5c, `SecKeychainGetStatus` in the reader reported the keychain locked (status bits
2); before and after, unlocked (status bits 7). `SecKeychainLock` and `SecKeychainUnlock` with the
generated password both returned 0.

**Code signatures** (`codesign -dv --verbose=4`):

| Binary | Code directory | Signature | Team identifier | CDHash |
| --- | --- | --- | --- | --- |
| `reader` v1 | `flags=0x20002(adhoc,linker-signed)` | adhoc | not set | `ebf84c8dbfbc7a88bfbbb70970bf78d87e4d4a3c` |
| `reader` v2, rebuilt at the same path | `flags=0x20002(adhoc,linker-signed)` | adhoc | not set | `7b7f14389cefbac3f5db19f4e09ea6849160b9bc` |
| `other-reader` | `flags=0x20002(adhoc,linker-signed)` | adhoc | not set | `c8737d5e05c32f6ca0d375a1ed5038b86bc42565` |

The rebuild changed only the build tag. It changed the code-directory hash, and cargo replaced
the file at the same path with a new one (a new inode).

**Timing** (question 6), on the item `reader` v1 read first in each keychain:

| Measure | `legacy`, item `listed` | `partition`, item `listed-partition` |
| --- | --- | --- |
| First read in a new process | 1.7 ms | 4.3 ms |
| Next 19 reads in the same process: min, median, max | 0.55, 0.73, 1.24 ms | 0.94, 1.16, 2.10 ms |
| One read in each of five new processes | 1.3 to 2.8 ms | 2.4 to 5.0 ms |
| Each of those processes, start to exit | 16.0 to 16.3 ms | 14.9 to 16.0 ms |
| Refused reads in questions 1 to 3 | 6.1 to 6.2 ms (3 reads) | 5.0 to 7.2 ms (6 reads, 1a included) |

In the results files this last measure is under the key `refused_reads_in_questions_2_and_3`,
which is older than its content: it counts every refused read in questions 1 to 3, and in
`partition` that includes 1a, the slowest (7.2 ms). The probe polls a child every 5 ms, so a
process's time is known to about 5 ms. The first start of a newly built binary took longer:
207.5 ms for `other-reader` in case 2a and 211.4 ms for the rebuilt `reader` in case 4a, against
about 16 ms for each later start. The cause was not investigated.

**Search list and default keychain.** Both unchanged byte for byte. Before and after, each
command printed the same 64 bytes, with SHA-256 `101ab3e1…cec7846`:

```text
    "~/Library/Keychains/login.keychain-db"
```

(the home directory replaced with `~`). The probe recorded the check in `search_list_check` of
the results, and it was repeated outside the probe against copies saved before the first run,
with the same result. Both throwaway keychain files were removed, and no keychain file remains
under `target/scratch/`.

**Earlier runs the same day.** The probe ran three times before the review. The first used only
a keychain in the older format. The second added the `partition` keychain, and the third,
recorded above, changed which items are read in question 5 and removed two unused dependencies.
The session that ran them reported that every read an earlier run shared with the third had the
same outcome and code, and that the search list and the default keychain were unchanged after
each. Their output was overwritten, so the files here do not show this. The review's run, below,
is the independent repetition that is kept.

### Re-run by the review

A review ran the probe again, as [Run it](#run-it) says, on 2026-11-06 from 18:31:31 to 18:31:33
UTC, on the same machine, macOS, toolchain and crate versions, in the same kind of session
(`Aqua`, attributes `0x2030`). Cargo ran offline (`CARGO_NET_OFFLINE=true`), and the whole probe
ran under a 900-second alarm. Its raw output is [results-rerun.json](results-rerun.json) and its
console output [results-rerun.txt](results-rerun.txt), renamed after the run so that the run
above keeps its files.

- **Every case matched.** All 46 reads had the same outcome, code, message and value check as
  the run above. Each reader also reported the same state: user interaction off, session leader,
  no terminal, same session attributes, and the same keychain lock state before each read. The
  console output is byte for byte the same.
- **Access lists:** the same entries on every item in both keychains. In two items, `api-created`
  in `legacy` and `listed` in `partition`, the entries came back in another order, and the
  integrity digests differ.
- **Binaries:** all three CDHashes were the same as in the run above (`ebf84c8d…`, `7b7f1438…`,
  `c8737d5e…`). A build of unchanged source with the same tag gave the same code-directory hash.
  Whether such a rebuild keeps access was not tested, since each run uses new keychains.
- **Safety:** the review saved `security list-keychains -d user` and
  `security default-keychain -d user` before it ran anything. After the run, both were byte for
  byte the same as those copies, the same 64 bytes as above. The probe's own checks also
  reported both unchanged. The file where the user's search list is written,
  `~/Library/Preferences/com.apple.security.plist` (`kUserDefaultPath` in Apple's
  [`DLDBListCFPref.cpp`](https://github.com/apple-oss-distributions/Security/blob/main/OSX/libsecurity_keychain/lib/DLDBListCFPref.cpp)),
  did not exist before the run and did not exist after it; it was only checked with `stat`.
  Both throwaway keychain files were removed, and the run directory was gone.

Timing in the re-run had the same pattern, with somewhat different values:

| Measure | `legacy`, item `listed` | `partition`, item `listed-partition` |
| --- | --- | --- |
| First read in a new process | 1.6 ms | 3.0 ms |
| Next 19 reads in the same process: min, median, max | 0.61, 0.80, 1.18 ms | 0.94, 1.19, 2.07 ms |
| One read in each of five new processes | 1.3 to 2.7 ms | 2.4 to 5.0 ms |
| Each of those processes, start to exit | 15.9 to 16.2 ms | 15.9 to 16.1 ms |
| Refused reads in questions 1 to 3 | 5.4 to 6.6 ms (3 reads) | 4.4 to 7.1 ms (6 reads, 1a included) |

Across the re-run's cases, successful reads took 0.6 to 5.0 ms, refused reads of an unlocked
keychain 4.4 to 7.1 ms, and reads of a locked keychain 2.7 to 3.5 ms. A new binary's first start
again took longer: 215.9 ms (2a) and 208.4 ms (4a).

## What it means for a macOS keychain provider package

- **In the user's logged-in session, reads with no person present work, fast and silently,
  under three conditions.** The keychain is unlocked; the reading binary is on the item's access
  list; and, in the login keychain's format, the binary's partition ID is on the item's
  partition list (1b, 1c). Then a read with no person and no terminal took 0.5 to 5.2 ms (0.6 to
  5.0 ms in the re-run). The reads ran in the user's logged-in session, not as a daemon (see
  below), so this does not answer E10's daemon part.
- **The person's own tool is not enough in the login keychain's format.** An item added with
  `security add-generic-password -T <package binary>` gets the partition `apple-tool:` only, and
  the package's ad-hoc binary is refused (1a, `partition`). Of the routes tried, it read the
  person's item only after its partition was added, which needs the keychain's password (1b): for
  the login keychain, by default the person's login password. The other route that worked is for
  the package's binary to create the item itself (1c), which means the package writes the
  secret. The secrets proposal leaves writing to the person's own tools
  ([S5](../../docs/proposals/secrets/README.md#s5-where-the-value-is-stored)). A person
  answering a dialog is a third possible route, not tried, by rule.
- **`-A` does not mean any application in the login keychain's format.** The `any-app` item had
  the partition `apple-tool:`, and both probe binaries were refused (3a, 3b). In the older format
  `-A` let an unlisted binary read (3a), so there it limits nothing beyond who can open the
  keychain file while it is unlocked.
- **A rebuild loses access.** A changed build of an ad-hoc binary has a new code-directory hash.
  The partition list names the old hash, as recorded. The access list names the binary by its
  path, and still the new build at the same path was refused in the older format, which has no
  partition list. Every item whose access list named the old build was refused to the new one
  (4a to 4c), in both formats; only the `-A` item in the older format stayed readable (4d). A
  package installed from source would lose access at every upgrade that changes its binary,
  until a person authorizes it again. A rebuild of unchanged source gave the same code-directory
  hash in both runs; whether that keeps access was not tested. A binary signed with a stable
  identity, whose partition is `teamid:` and whose designated requirement survives a rebuild,
  might keep access; that was not tested either.
- **A locked keychain fails at once, with the same code as a refusal.** With interaction
  disabled, a read of a locked keychain returned -25293 in 2.7 to 3.7 ms across both runs (5a to
  5c). Unlocking needs the keychain's password, which for the login keychain is by default the
  person's login password. The code alone does not say whether the keychain was locked or the
  binary was not trusted; asking `SecKeychainGetStatus` first does.
- **A package must disable user interaction itself.** It was disabled in every reader here, and
  no read waited. What a reader that leaves it enabled gets, a dialog or a wait, was not tested,
  by rule. The SDK's header says only that `SecKeychainSetUserInteractionAllowed` "Turns on or
  off any optional user interaction".
- **The code here was errSecAuthFailed, not errSecInteractionNotAllowed.** In these conditions no
  read returned -25308. A package should treat -25293 as "could not be read", as it would
  -25308, and not rely on getting -25308. The message of -25293, about a user name or passphrase,
  misleads if passed on. Other conditions (a daemon, an SSH session, `SecItemCopyMatching`) were
  not tested and may return other codes.
- **The interface is deprecated.** `SecKeychainCreate`, `SecKeychainFindGenericPassword`, the
  lock, status and user-interaction functions and the access-list functions are marked
  `API_DEPRECATED` with `macos(..., 10.10)` in the SDK's headers. They worked on macOS 26.6.2.
  Apple's [TN3137](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains)
  says the file-based keychain "is on the road to deprecation" and is "not officially deprecated,
  but some of the APIs surrounding it are". It also says the data-protection keychain needs
  entitlements "authorized by a provisioning profile", in an app-like bundle. And it says that
  programs "like a launchd daemon, must target the file-based keychain", the kind probed here.
  These are Apple's statements, not tested here.

## What it means for the secrets contract

Nothing here binds, and nothing here decides. Points for the owner:

- The proposal's draft SEC-12 ([promises that would follow](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow))
  has a locked keychain give the outcome failed, with no other source tried. With interaction
  disabled that is reachable unattended, and fast. With it enabled it was not measured. A
  candidate addition, not decided: an implementation never waits for a person, with an
  observable bound on how long a read with no person present may take.
- An implementation can say why a read failed (locked, or not trusted) only by asking the
  keychain's status. SEC-12's message, "says the store could not be read", needs no more.
- Option B2 of [S5](../../docs/proposals/secrets/README.md#s5-where-the-value-is-stored) puts the
  keychain in every engine. An engine built from source is signed ad hoc, so each rebuild that
  changes the engine's binary would lose access to items it was trusted for (question 4). This
  is evidence for the owner's decision on S5, not a ruling on it.
- In the login keychain's format, reading an item the person added with their own tool needed a
  step taken with the keychain's password first: adding the package's partition. The
  alternatives are a dialog (not tested) or letting the package write the value. For an ad-hoc
  binary that step would be needed again after every rebuild that changes it. Where that step
  belongs, whether in a package's setup or outside Conduit, is not settled here.

## Not tested, and why

- **The login keychain.** Not touched, by rule. The `partition` keychain has its database format
  and the same entries on each item, as recorded, but it was unlocked with its own password and
  not at login, is not in the search list or the default, and had automatic locking turned off.
- **A launchd daemon or agent.** Not installed, by rule. The readers ran detached, with no
  terminal, but in the user's graphical security session: `SessionGetInfo` reported attributes
  `0x2030`, with graphic access and TTY set, not root and not remote (`0x2000` is not named in
  the public header). A daemon runs in another session, as root or another user, where the lock
  state and the keychains it can see may differ. This is the first part of E10, and it is open.
  TN3137 (cited above) says such a daemon must use the file-based keychain, the kind probed
  here. Whether it could reach a user's login keychain, and in what lock state, is what remains
  to be tested.
- **An SSH session, a session before the first graphical login, after sleep or screen lock, and
  another user.** Not run.
- **The data-protection keychain.** According to TN3137 it needs entitlements authorized by a
  provisioning profile, in an app-like bundle, and is available only in a user's login context.
  This probe's binaries have none of that.
- **A binary signed with a Developer ID or another stable identity** (partition `teamid:`).
  No signing identity was used, since making one would write to the user's keychain.
- **Reading with user interaction allowed**, and what dialog appears. Not done, by rule.
- **Reading through `/usr/bin/security find-generic-password`**, which is in the `apple-tool:`
  partition and so might read the items the person adds with the same tool. Not run, because the
  tool cannot disable user interaction. A package that read this way would trust any process of
  the user that can run that tool.
- **Items added through Keychain Access**, which is signed by Apple and may get another
  partition. Not run.
- **`SecItemCopyMatching`** against a file keychain, in place of `SecKeychainFindGenericPassword`.
  Not compared.
- **A long-lived reader across lock and unlock**, automatic lock timeouts, and why a new binary's
  first start took about 210 ms. Not examined.

## Run it

```sh
cargo build --release
./target/release/probe        # or: cargo run --release
cargo test --release          # tests of the helpers; none touches a keychain
```

The probe needs macOS with `/usr/bin/security` and `/usr/bin/codesign`. It builds the readers into
`target/scratch/build/`, writes `results.json` here and prints the summary that `results.txt`
holds. A new run overwrites `results.json`. The review's run was renamed to `results-rerun.json`
and `results-rerun.txt` afterward, so that each recorded run keeps its own files. The probe exits
0 when the run completed, 1 when it stopped, and 3 when the search list or the default keychain
changed, which it then prints. It never shows a dialog on purpose; if one ever
appears, cancel it, since every child process is killed after its timeout.

## Evidence log

| Date | Work | Evidence |
| --- | --- | --- |
| 2026-11-06 | Built the probe; ran it three times; recorded the third run | The tables above; [results.json](results.json); [results.txt](results.txt); `cargo test --release` passed; search list and default keychain unchanged after every run |
| 2026-11-06 | Review: ran the probe again; checked the claims against both raw files, the SDK's headers, Apple's Security source and TN3137; corrected a mislabelled timing row and statements that went beyond what was measured | [Re-run](#re-run-by-the-review); [results-rerun.json](results-rerun.json); [results-rerun.txt](results-rerun.txt); same outcome in all 46 cases; `cargo test --release` passed; search list and default keychain unchanged, checked against copies saved before the run; no throwaway keychain left |
