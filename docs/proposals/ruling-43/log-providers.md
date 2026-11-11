# Ruling 43: log of the providers group

Date: 2026-11-06. This is the account of the editor who applied [ruling 43](README.md) to two
promise files: [providers](../../../spec/promises/providers.yaml) and
[placement](../../../spec/promises/placement.yaml). The ruling is the owner's: it accepts the
[proposal on work across machines](../multi-machine/README.md) with D5's second part as option b
and Q1 tightened. The choices listed under [Choices beyond the record](#choices-beyond-the-record)
and the names under [Coined names](#coined-names) are this editor's, and each stands unless the
coordinator or the owner strikes it.

What was applied: the rows of the proposal's
[section 9, "In v0"](../multi-machine/README.md#in-v0) that touch these two files, with the
detail of [section 3][s3] (the gate in substance, the means a provider may use, what does not
count), section 5 (Q1 to Q5) and sections 1.2 to 1.5; and the items of the
[log of ruling 42's integration](../ruling-42/log-integration.md) that waited on this ruling, as
far as they touch these files. Not applied: the proposal's section "In v1, for A65", which is
for the tracker.

[s3]: ../multi-machine/README.md#3-d5-who-may-reach-a-server-on-another-machine

## Checks

| Check | Result |
| --- | --- |
| Validation of [CONSOLIDATION.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/CONSOLIDATION.md#validation) | `447 promises ok`, with every group's files as they stood when it ran |
| Every promise in the two files has exactly the seven fields, its text fields folded with `>-` | Yes |
| A line over 96 characters in the two files | None |
| An ID or a DL item cited in the two files that does not exist | None. DL17, IFC-042, MAN-018 and SEC-024 were already written by their groups when this check ran |
| `python3 tools/check_docs.py` | 1 error, not in these files: `spec/core/README.md` links to `docs/proposals/ruling-43/log-core.md`, which its group had not yet written; 0 before that link was added |
| `python3 tools/set_names.py --check` | `Errors: 0`; it counts the names of `identity` that `spec/names.yaml` now holds, which are the ones these files use |
| `python3 tools/check_schemas.py` | `Errors: 0` |
| `git diff --check` | Clean |

## Counts

15 promises changed and 3 added; none removed and none moved between files.

| File | Changed | Added |
| --- | --- | --- |
| `spec/promises/providers.yaml` | PRV-002, PRV-003, PRV-007, PRV-008, PRV-009, PRV-011, PRV-012, PRV-014, PRV-018, PRV-020, PRV-021, PRV-022, PRV-023 | PRV-025, PRV-026, PRV-027 |
| `spec/promises/placement.yaml` | PLC-023, PLC-026 | None |

Each of the two files has a paragraph in its head comment naming ruling 43, what changed and
this log.

## New IDs

PRV-025 and PRV-026 were assigned in advance by the coordinator. PRV-027 is this group's own,
added in its own file, as the work order allows.

| ID | From | What it says |
| --- | --- | --- |
| PRV-025 | D5, A; section 3, "The gate, in substance" | Where a runtime provider other than the engine's own runner runs an item, or an item reached by `remote` that is not `plain` serves an instance, the server stands behind a gate that admits a request only with cryptographic proof, by DL13's mechanism, that an enforcement point sent it, in a form that no receiver can present to another. The gate is the server itself, something between the network and the server, or, for a run, the provider's own end of an authenticated, confidential channel to the run's machine. Nothing else on that machine reaches the server, root and the provider's own user or group apart. The provider's contract states its gate, that user or group, and how long the gate takes to close a stream once the engine's side goes silent. The observable lists the means of section 3 and what does not count. |
| PRV-026 | Q3, b | After a restart, before a provider call to a runtime, machine or package-source provider whose run on the engine's own runner ended with the engine, the engine has its runner run it again, its secrets read again for the caller of the setup that last asked for its run, as the Conduits stand then. |
| PRV-027 | X10 of the verification of ruling 42, as this ruling settles it | A provider call to `stop.v1` or `inspect-run.v1` gives `instance` and `machine`, the latter as the status of the instance's node holds it, retired or not: its labels, and the value of `kubernetes.io/hostname` as `hostname`. |

## Each change

| What | Promise | Why, with the proposal's section |
| --- | --- | --- |
| The gate | PRV-025, new | Section 9, row "New, in `providers.yaml`"; section 3, "The gate, in substance", points 1 to 3 and 5, with "Means a provider may use" and "What does not count" in the observable; section 1.3. Point 4, the engine's side, is IFC-017 and IFC-041 (the interface group); PRV-025's observable points to them. |
| A provider's run made again after a restart | PRV-026, new | Section 9, row "New, in `providers.yaml`; SEC-011"; Q3, b. The second exception to SEC-011 is the secrets group's; PRV-026 says the read belongs to the setup that last asked for the provider's run, as SEC-011 now does. |
| The machine in `stop.v1` and `inspect-run.v1` | PRV-027, new; PRV-008, `then`; PRV-009, observable | X10 of the [verification of ruling 42](../ruling-42/log-integration.md#each-finding), item 5 of its [Still needs a decision](../ruling-42/log-integration.md#still-needs-a-decision), settled by the work order: the same machine input as `run.v1`, so that a provider that keeps no state can act. LIF-036 (the conventions group) and the core package's `stop.v1` and `inspect-run.v1` say the same. |
| `url` reported by `inspect-run.v1` | PRV-009, observable | Section 9, row "`inspect-run.v1`, PRV-009"; section 3, the engine's side. The output is the core group's, and that the engine reaches the latest URL is IFC-017's. |
| Delivery over an authenticated, confidential channel | PRV-011, `then` and observable; PRV-022, `then` | Section 9, row "PRV-011, PRV-022"; section 1.4, "Delivered in confidence". PRV-011's observable also says that the run's machine and its gate are inside the domain of trust for what the run serves, as the row for `spec/README.md` has it. |
| What `identity` carries, under fixed names; the gate may use it, nothing else may | PRV-022, `then` and observable; PRV-007, `then` | Section 9, row "PRV-011, PRV-022"; Q1, a, "The names of the three things `identity` carries are fixed once"; section 6, D17. PRV-007 now gives `identity` "under the names that PRV-022 fixes". |
| A credential lasts as long as its instance; renewal is not v0's | PRV-022, observable | X9, settled: section 1.4, "As long as the instance, in v0". The rule is DL13's (the implementation-defined group); PRV-022 keeps the provider's leave to replace a file in place, which section 6, D17, keeps for v1. |
| A name of `identity` and a variable of the engine's environment | PRV-023, `then` and observable | X3 and item 2 of [Needs a decision](../ruling-42/log-integration.md#needs-a-decision), settled by Q1 and MAN-018 for a template; this covers the one case MAN-018 cannot, the list set on the engine's own runner. See [choice 5](#choices-beyond-the-record). |
| A `package` whose transport the provider neither serves nor bridges | PRV-012, `when` and observable | Section 9, row "PRV-012, MAN-010"; section 4, "A gap D20 did not raise: stdio across machines". |
| `gone` only where the provider knows the machine no longer exists, in the rule | PRV-014, `then` and observable | Section 9, row "PRV-014"; scenario S7. The observable keeps the example and gives Kubernetes' force deletion as the reason. |
| Not from the address, machine, network, runtime provider or run | PRV-018, `then` and observable | Section 9, row "IFC-022, PRV-018"; Q1, a, last point. The observable says that a certificate that mutual TLS presents once for a connection is part of what each request presents. |
| Stops a provider owes survive the engine's restart and the provider's retirement | PRV-021, `then` and observable; PRV-020, observable | Section 9, row "PRV-021, PRV-020"; section 6, D11. This also settles the point of item 5 of [Needs a decision](../ruling-42/log-integration.md#needs-a-decision), and X6, on a stop that a provider still holds when it is retired. |
| Provider servers and the engine's own runner reached over stdio, behind no gate | PRV-002, PRV-003, observables | Section 3, "Observable": "The engine's own runner meets it by stdio"; section 8, the row for the engine's own runner. PRV-002 also points to PRV-026. |
| A process of the engine's own operating-system user can make the engine's proof | PRV-003, observable | Q5, a; section 8, the row for the engine's own runner. The statement itself is `spec/README.md`'s and IFC-022's (other groups). |
| The run's server stands behind a gate | PRV-007, observable | Section 3; points to PRV-025 and IFC-017. |
| `local` keeps one hostname across a move | PLC-026, observable | X7, settled by Q2: the engine keeps its hostname across a move. The rule is DL17's; PLC-026 points to it, and says that a value tied to `local` keeps its meaning. |
| The machine provider run again after a restart | PLC-023, observable | Q3, b, whose example is the engine's regular report from the machine provider under DL5. |

Of the [questions for the research on work across machines](../ruling-42/log-integration.md#for-the-research-on-work-across-machines),
these files now answer: who may reach a run's `url` (PRV-025); how a refreshed credential reaches
a run (it does not in v0: PRV-022); what stops a run left on a machine that comes back (PRV-021,
PRV-020, and PRV-025, under which such a run's gate admits only an enforcement point). D20 needs
no text: row 17's guard stays in PRV-007, as section 4 recommends. The others are other groups'.

## Coined names

The ruling fixes the names of what `identity` carries, and leaves them to the text. They are
chosen by the precedents the proposal cites, and agree with those that the interface, the
implementation-defined and the core groups wrote; `spec/names.yaml` now holds them. For the
owner to strike.

| Name | What it is | Precedent |
| --- | --- | --- |
| `CONDUIT_URL` | The variable that holds the engine's address, as the run sees it | Kubernetes gives a Pod the API server's address in a variable (`KUBERNETES_SERVICE_HOST`); `url` is the name in force for a Streamable HTTP URL (N5 of the vocabulary audit) |
| `CONDUIT_IDENTITY_DIR` | The variable that names the directory in which the run finds the files of `identity` | systemd's `CREDENTIALS_DIRECTORY`, which PRV-011 already cites for `LoadCredential=` |
| `ca.crt` | The trust bundle: the root certificates of DL13's mechanism | Kubernetes fixes the path of a Pod's `ca.crt` |
| `token` | The run's token, in the form of a JWT-SVID | Kubernetes fixes the path of a Pod's `token` |
| `tls.crt`, `tls.key` | The run's certificate, in the form of an X509-SVID, and its private key | The keys of a Kubernetes TLS Secret |

The terms gate and enforcement point, which PRV-025 uses, are the ruling's and the proposal's,
from NIST's policy enforcement point; decision point is not used in these files. This group did
not write their rows in `spec/definitions.md`.

## Choices beyond the record

1. **A directory, named by a variable.** Section 1.4 cites Kubernetes, which fixes absolute paths
   inside a container. The engine's own runner starts plain processes that share one machine,
   so fixed absolute paths would collide; a variable names a directory per run, and the files
   inside it have fixed names. This makes two variables where Q1 names one.
2. **`url`, not `endpoint`.** The proposal names the new optional output of `inspect-run.v1`
   `endpoint`, the old name of `run.v1`'s output, which N5 of ruling 42 renamed to `url`. The
   core group and the interface group use `url` too.
3. **A new ID for the machine of a stop or an inspection (PRV-027).** No promise describes the
   call to `inspect-run.v1`, so one promise covers both calls, and PRV-008's `then` points to it.
4. **PRV-025 binds the server's side; the engine's side stays in IFC-017 and IFC-041.** Point 4
   of section 3 is the engine's obligation, so it was left to those promises, one obligation per
   promise. PRV-025's `when` is a run by any runtime provider other than the engine's own runner,
   whether or not it has returned its `url`, so that the gate stands from the start.
5. **`identity` wins over the list set on the engine's own runner** (PRV-023). MAN-018 rejects a
   template whose item binds `CONDUIT_URL` or `CONDUIT_IDENTITY_DIR`, but the list names
   variables of the engine's environment, which no template governs. Where the list names one of
   them, the run gets the value of `identity`, as ruling 42's integration recommended for the item
   case (its option a).
6. **The names of `identity` are a closed list** (PRV-022: "under these names and no other"), so
   that MAN-018's check is exact. `token` is present where DL13's mechanism uses the token form,
   and `tls.crt` and `tls.key` where it uses mutual TLS or the run's server stands behind a gate,
   as DL13 now says.
7. **"Restarts on another machine, or runs as several parts"** in place of "fails over", which
   `spec/definitions.md` lists among the words the promises do not use. DL17 and SEC-024 use the
   same words.
8. **PRV-026 covers only the three provider capabilities that the engine reaches by provider
   calls**, as Q3 frames it, not a secret Resource, which is placed and run as any Resource is.
   See [open item 1](#needs-a-decision).
9. **PRV-011's channel** binds only what a provider carries to a machine other than the one its
   own server runs on, which is `local` under PRV-002; on one machine the provider's own means
   serve.

## Needs a decision

1. **A secret Resource on the engine's own runner after a restart.** `spec/definitions.md` counts
   a secret Resource among the providers, but Q3 speaks of provider calls, which no secret
   Resource receives, so PRV-026 leaves it out. Its process may end with the engine, as DL16
   states; then every read through it, SEC-023's after a restart among them, fails until an
   actor sets it up again, which is the friction on one machine that the owner asked about and
   item 4 of ruling 42's [Still needs a decision](../ruling-42/log-integration.md#still-needs-a-decision)
   noted. Options: (a) as now; (b) PRV-026 also runs again, before a read through it, a secret
   Resource whose run on the engine's own runner ended with the engine, its own secrets read for
   the caller of the setup that last asked for its run. Recommendation: (b), the same mechanism
   on one more kind of provider. DL16 now says "apart from the servers of providers, which
   PRV-026 has the engine run again", which reads as covering a secret Resource; under (a) it
   should say the servers of the providers that PRV-026 names.
2. **For other groups.** `spec/definitions.md` needs rows for gate, enforcement point and trust
   bundle, which PRV-025 and PRV-022 use, and its row for `identity` should name the fixed
   names; its row for a runtime provider can mention the gate and the channel. `spec/README.md`
   still states D5 as a gap, and its bullet on a stop that a provider holds when it is retired
   says that only its contract decides, where PRV-021 now keeps that stop.
