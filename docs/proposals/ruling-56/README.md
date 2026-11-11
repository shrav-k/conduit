# Ruling 56: how a person checks a joining machine

Status: **ruled by the owner on 2026-11-10** (plan-audit ruling 56). This is the record of a
ruling. Where it and the specification differ, the specification governs.

Ruling 52 left the fifth decision of the [enrollment proposal](../enrollment/README.md) to research:
what a person checks before admitting a machine. The [verification methods](../enrollment/verification-methods.md)
research put eight decisions; ruling 55 left them for discussion, and the coordinator explained the
threats (a leaked join secret used on another machine, the wrong one of one's own machines, a relayed
link) and what each kind of check proves, and put four further questions. The owner's answer:
"Regarding enrollment, I find the path you suggest acceptable for v0, I accept all your
recommendations".

## The decisions

| # | Decision | Answer |
| --- | --- | --- |
| 1 | Who chooses the method | **The machine provider**, declared in its contract |
| 2 | The minimum the engine enforces, whatever the provider | Admission binds the machine's stable `id` and the enrollment it joined by; nothing pending can be admitted without an enrollment that root started; at least one item of evidence is derived from the machine's key or from attestation, so facts alone never suffice |
| 3 | The shape of the evidence | A list of items, each with a kind (`code`, `fingerprint`, `attestation`, `fact`), a value, where to find it on the machine and what it proves, recorded with the answer |
| 4 | Who shows it | **The engine alone**, at the root Interface's control panel; never relayed through a model's output |
| 5 | Customizing | **Upward only, in v0**: an enrollment may require more than the minimum (for example attestation, or a person's comparison of the fingerprint even where attestation exists), never less |
| 6 | Tailscale's evidence | A short fingerprint of the node key, with the hostname, `os`, `arch`, labels and `id`; that both ends can show the node key is to be confirmed in A74 |
| 7 | Peers and sign-ins | The same evidence field: for peer engines the fingerprint of the trust root in v1, with PAKE codes or short authentication strings studied then; for sign-ins the account expected and, in a device flow, the user code |
| 8 | The word for a pending action's result | Not "outcome", which names how a request ends: its **answer** (admitted or refused, and so on) |
| 9 | A machine with no screen to compare, such as a cloud VM | Attestation meets the minimum; where the provider offers none, the enrollment is **refused**, unless the person gives an explicit flag accepting a weaker check, which is recorded |
| 10 | How a person compares | **A fingerprint of 6 to 8 characters** shown on the machine and in the panel, in v0; scanning a QR code is a later option where a camera is at hand |
| 11 | Who may admit | **Root alone in v0** (ruling 50); machines stay at the engine level, outside Projects |

## What follows

With ruling 50's other decisions and rulings 51 and 52, enrollment is ready to be written into
`spec/` once the shape of the language and the Project rulings are settled; it is needed before S3.
