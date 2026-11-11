Not binding. The [specification](../../../spec/README.md) alone binds.

# How a person checks what they admit: verification methods for enrollment

Research by the coordinator, 2026-11-10, on decision 5 of the [enrollment proposal](../enrollment/README.md).

## 1. The question

Decision 5 of the enrollment proposal recommended that, before admitting a pending machine, the
person checks "a short code shown on the machine and in the control panel, plus `id`, hostname,
`os`, `arch` and labels." The owner doubted that this should be settled now: "it feels like this
might be something we should also make customizable or research canonical method".

The same question arises for a peer engine (ruling 51; networking paradigms, section 6), whose
person must know the bundle accepted is the other engine's, and for a sign-in (ruling 49), whose
code the person carries to another device. Ruling 51 decision 5 makes the pending action generic
(a kind, a subject, evidence, an expiry and an outcome), so one answer should serve all three.

## 2. Methods compared

| Method | What the person does | What it proves | Attacks stopped | Assumptions | Usability | Who uses it |
| --- | --- | --- | --- | --- | --- | --- |
| SAS with hash commitment (ZRTP, Matrix emoji or decimal, Bluetooth numeric comparison) | Compares a short string shown on both ends, then confirms on each | Both ends derived the same key from the same exchange | MITM on a fresh key exchange: one guess, success 1 in 2^n | A trusted display on each end; both ends present, or a voice channel the person trusts | Good: 4 to 7 symbols | ZRTP (32-bit value), Matrix (42-bit emoji, 3x13-bit decimal), Bluetooth (6 digits, not read) |
| Passkey entry (Bluetooth) | Reads a code on one end, types it on the other | The person carried the code between the two ends | MITM, wrong device | A display on one end, input on the other | Good | Bluetooth (not read) |
| PAKE invitation code (Magic Wormhole SPAKE2; CPace, OPAQUE) | Carries a short code from one end to the other once | Both ends know the same code; the resulting key is strong | Passive and active interception; the relay gets one guess per code | The code travels by a channel the attacker does not see in time | Very good: `7-crossover-clockwork` | Magic Wormhole (16-bit default); CPace is a draft; OPAQUE (RFC 9807) is client-server login, not pairing |
| QR scan (Matrix, Signal, WhatsApp linked devices) | Points one device's camera at the other's screen | The scanning device read keys the other displays; Matrix's code also carries what the other believes about the scanner | MITM, wrong device | Physical presence; a camera | Very good | Matrix, Signal, WhatsApp |
| Fingerprint comparison, then TOFU (SSH, Salt, Signal safety numbers) | Compares a hash of a public key with one obtained another way (phone, the other's screen), once; later connections compare with the stored key | This key belongs to that party | MITM and wrong key at first contact (only if compared); key change later | A second channel for the fingerprint; people actually compare | Poor for long strings; often skipped | SSH (RFC 4251 4.1), Salt (`salt-key -f`, `master_finger`), Signal (60 digits or QR) |
| Issuer pin carried with the secret (kubeadm CA hash, k3s, Swarm) | Nothing; the pin travels in the command | The joining machine reached the intended control plane | Rogue control plane | The command reaches the machine intact | Invisible | kubeadm, k3s, Docker Swarm |
| Approval with no shown evidence (Tailscale device approval, Kubernetes CSR) | Reads a listing (name, owner, subject) and approves | Only what the listing says | Unsolicited joins, if the person reads carefully | Names are meaningful and not forged | Good, and weak | Tailscale ("review details about the device and user"; no code); Kubernetes CSR (subject, signer, usages) |
| Signed node key (Tailscale Tailnet Lock) | Copies the new node's key from the device (`tailscale lock status`) or the console to a trusted signing node | A trusted key holder vouched for this node key | A compromised coordination server adding nodes | Signing nodes are trusted | Moderate | Tailscale |
| Out-of-band user code (RFC 8628 device grant) | Reads a code on the device and enters it at a URL on another device | The approving account and the device's session are linked | Not phishing: RFC 8628 says an attacker can start the flow and send the victim the URL; the server "SHOULD display information about the device" | The person knows which device they are approving | Good | OAuth device flow (Codex sign-in) |
| Binding message (OpenID CIBA) | Sees the same short message on the requesting and the approving device | The approval answers this transaction | Approving the wrong one of several requests | Both displays trusted | Good | CIBA |
| Attestation (TPM EK, Android key attestation, cloud identity documents; SPIRE `tpm_devid`, `aws_iid`; Teleport `tpm`, `ec2`) | Nothing at join time; for TPM, someone registers the EK hash once beforehand (`teleport tpm identify`) | A key is held in given hardware, or a platform vouches for the instance | Stolen join secret, re-imaged machine claiming an old identity | A hardware or platform vendor's root; for TPM, a prior registration | Invisible after setup | SPIRE, Teleport, Android, AWS |

Notes.

- **Commitment is what makes a short string safe.** ZRTP: hash commitment "constrains the attacker
  to only one guess", because the attacker must choose its public values before seeing either
  party's. Without it the attacker can search for a matching string and the string must be much
  longer. Matrix copies the scheme. A short code cut from a plain key hash, with no commitment,
  is a fingerprint, and is only as strong as an attacker's inability to grind a key whose short
  hash matches a target it knows.
- **What it binds is a key, not a name.** Every method that resists an active attacker compares
  something derived from a key. Hostname, OS and labels catch honest mistakes and prove nothing.
- **Attestation removes the comparison only where a vendor root or a platform exists.** For an
  owner's Windows computer, TPM joining moves the comparison earlier (someone reads the EK hash
  off the machine) rather than removing it.
- **Could a model see or tamper with it?** The values compared are not secret, so a model seeing
  them does no harm. Tampering is the danger: every method assumes a trusted display on each end.
  If the control panel's text passes through a model, the model can show any code it likes. A
  code shown on the machine by the provider's own client is trusted only while nothing a model
  drives has a terminal on that machine first.

## 3. The threat in Conduit's enrollment

Under Option C the join secret is single-use, short-lived and tagged, and is shown only at the root
Interface. The machine joins pending, carrying the enrollment it used. Three cases matter.

1. **The secret is stolen and used first on another machine.** Single use means one machine is
   pending for that enrollment, and the person's own machine fails to join. A person who sees the
   failure and still admits the pending entry admits the attacker. Defence: evidence the person
   can check against the machine in front of them, such as a code derived from the node key that
   the provider's client shows on the machine and the provider reports. Since the attacker does
   not learn the person's node key in time, a short code is enough here; grinding needs a known
   target.
2. **Relay or phishing.** Someone gets the person to run a command, or to admit a pending machine,
   that the attacker started (RFC 8628's remote phishing). Defence: only an enrollment opened at
   the root Interface can produce an admissible pending machine. A machine pending with no
   enrollment, or with another provider's, is shown but cannot be admitted. The machine-side code
   also catches it.
3. **The wrong machine of the owner's own.** Two enrollments are open and the person runs the
   wrong command on the wrong computer, so labels land on the wrong machine. Defence: hostname,
   `os` and `arch` shown beside the labels the enrollment will give; the code catches it too.

Managed enrollment needs no MITM resistance between the person's two screens: no key exchange
runs between machine and engine. The machine talks to the provider's pinned coordination server
(enrollment proposal, threat 4) and the engine reads the provider's API over TLS. So SAS with
commitment is more than it needs; a fingerprint of the machine's key, shown on the machine by the
provider and in the panel by the engine, fits. Peer introduction differs: two engines exchange
trust bundles over a channel neither controls, so a pinned root fingerprint, a PAKE code or a SAS
is warranted there.

## 4. Fix one method, or make it a choice?

**(a) Conduit fixes one method.** For example, a code that every machine provider must produce.
Simple to test, but it puts a key scheme into the language, which ruling 40 leaves to the
provider, and a cloud provider attests rather than showing a code.

**(b) The method is the provider's, declared in its contract; Conduit fixes the shape and a
floor.** The pending action carries `evidence`: a list of items, each with a kind (`code`,
`fingerprint`, `attestation`, `fact`), a value, where the person finds it on the machine (for
example "`tailscale status`"), and what it proves. The engine shows the items and records the
outcome with the evidence that was shown. The machine provider's contract states which items it
gives. Conduit fixes three things: admission binds the provider's machine `id` and the enrollment;
an entry with no enrollment opened at the root Interface cannot be admitted; and the evidence
includes at least one item tied to the machine's key or to attestation, never only facts the
machine reports.

**(c) A configurable verification setting per enrollment or Pool** (none, compare code, attested),
with a minimum the engine enforces. The most choice, but a policy surface, which ruling 41 removed
("policy" is not used for access), and "none" undoes ruling 50 point 2.

**Recommended: (b), with one input of (c) reserved.** Reasons:

- Ruling 40: the machine's key, its display and its API are the provider's. Conduit's part is who
  may join (the enrollment, the admission) and what a joined machine may be given (labels), which
  (b) keeps.
- It is the shape ruling 51 already accepted. `evidence` is its fourth field, and the same
  structure carries a peer engine's root fingerprint and a sign-in's account expected and user
  code, with no new mechanism.
- It meets the owner's wish to make it customizable where the choice is real, in the provider,
  without a setting that can turn the check off. If a provider offers several methods (a TPM or a
  code), `enroll.v1` may later take an input naming the kind of evidence required among those the
  contract declares. The engine refuses an enrollment that asks for one the provider does not
  give. Reserve the input now and accept none in v0.

One rule follows from the model concern: the engine renders the evidence to the person itself, at
the root Interface's control panel, and never as text that a model relays or writes. A pending
action shown through an Agent's or the coordinator's output carries no evidence.

## 5. Under the recommendation

**The engine's own machine provider.** It enrolls nothing, so it gives no evidence and raises no
enrollment. For peer introduction (v1), the engine itself is the party, so the evidence is the
fingerprint of its trust domain's root, shown at both root Interfaces, as kubeadm pins a CA hash.
Whether a PAKE code or a SAS should replace the pasted fingerprint is a v1 question.

**Tailscale.** Evidence: a `fingerprint` item derived from the device's node key, which
`tailscale status` shows on the machine and the device API reports. The engine shows it in the
same short form (to be decided, for example the first 8 characters after `nodekey:`). Add `fact`
items for hostname, `os`, `arch`, the enrollment and the labels from its tags, and the device `id`.
Admission calls the authorization endpoint as before. Two
facts need checking in A74: that the device API returns the node key, and that the Windows client
shows it to a person without the admin console.

**v0 needs:** the `evidence` field with the four kinds; the floor; rendering by the engine only;
Tailscale's node-key fingerprint; vectors for a stolen secret used elsewhere (fingerprint differs,
and the admission the person refuses removes it), a pending machine with no enrollment (not
admissible), and a panel that a model writes (no evidence shown).

**Later:** the reserved input for choosing the evidence kind; `attestation` items (a cloud identity
document, or a TPM EK registered at enrollment); peer introduction's fingerprint, PAKE or SAS; and
evidence for sign-ins on other people's Interfaces (ruling 49, v1).

## 6. Decisions for the owner

1. **Who chooses the method.** Recommended: the machine provider, declared in its contract
   (option b), not Conduit (a) and not a per-enrollment setting (c).
2. **The floor the engine enforces.** Recommended: admission binds the machine `id` and the
   enrollment; nothing pending without a root-Interface enrollment can be admitted; at least one
   evidence item is tied to the machine's key or to attestation.
3. **The shape of evidence.** Recommended: a list of items, each with a kind (`code`,
   `fingerprint`, `attestation`, `fact`), a value, where to find it on the machine, and what it
   proves; recorded with the outcome.
4. **Who shows it.** Recommended: the engine alone, at the root Interface's control panel; never
   relayed through a model's output.
5. **Customization.** Recommended: reserve an input on `enroll.v1` naming the evidence kind
   required among those the provider declares; accept none in v0. Do not call it a policy.
   Superseded by [ruling 56, point 5](../ruling-56/README.md): stronger checks may be required in v0; the minimum may never be weakened.
6. **Tailscale's evidence.** Recommended: a short fingerprint of the node key plus hostname, `os`,
   `arch`, labels and `id`; confirm in A74 that both ends can show the node key.
7. **Peer introduction and sign-ins.** Recommended: the same field; for peers, the root fingerprint
   in v1, with PAKE or SAS studied then; for sign-ins, the account expected and, in a device flow,
   the user code, as RFC 8628 asks.
8. **The word "outcome".** It already names how a request ends (succeeded, failed, unknown).
   Recommended: name the pending action's result something else, such as its answer (admitted or
   refused), when it is written into the specification.

## 7. Sources read

- RFC 8628, OAuth 2.0 Device Authorization Grant: https://www.rfc-editor.org/rfc/rfc8628.html
- OpenID CIBA Core 1.0, `binding_message`: https://openid.net/specs/openid-client-initiated-backchannel-authentication-core-1_0.html
- RFC 6189, ZRTP (first third only; SAS rendering as base32 or word lists not read): https://www.rfc-editor.org/rfc/rfc6189.html
- Matrix specification, end-to-end encryption module (SAS, QR): https://raw.githubusercontent.com/matrix-org/matrix-spec/main/content/client-server-api/modules/end_to_end_encryption.md
- Magic Wormhole documentation: https://magic-wormhole.readthedocs.io/en/latest/welcome.html
- CPace, draft-irtf-cfrg-cpace-21 (abstract): https://datatracker.ietf.org/doc/draft-irtf-cfrg-cpace/
- RFC 9807, OPAQUE (abstract): https://www.rfc-editor.org/rfc/rfc9807.html
- Signal, safety number updates: https://signal.org/blog/safety-number-updates/
- RFC 4251, SSH architecture, section 4.1: https://www.rfc-editor.org/rfc/rfc4251.html
- Salt minion configuration, `master_finger`: https://docs.saltproject.io/en/latest/ref/configuration/minion.html
- Kubernetes certificate signing requests: https://kubernetes.io/docs/reference/access-authn-authz/certificate-signing-requests/
- Tailscale device approval: https://tailscale.com/kb/1099/device-approval
- Tailscale Tailnet Lock: https://tailscale.com/kb/1226/tailnet-lock
- WireGuard: https://www.wireguard.com/
- SPIRE agent node attestors: https://spiffe.io/docs/latest/deploying/spire_agent/
- Teleport join methods (TPM, delegated): https://goteleport.com/docs/reference/deployment/join-methods/
- Android key attestation: https://developer.android.com/privacy-and-security/security-key-attestation
- AWS instance identity documents: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-identity-documents.html
- WhatsApp linked devices: https://faq.whatsapp.com/1317564962315842

Unverified (from memory, or the page did not load or did not say):

- Bluetooth Secure Simple Pairing: the NIST SP 800-121 PDF and the Bluetooth SIG page did not
  give the text. The 6-digit value, Just Works having no MITM protection, and passkey entry are
  from memory.
- Apple device pairing (Apple Watch and others): the Platform Security page did not load the
  section. Not used.
- Signal linked devices: the support page refused the request. Matrix's QR scheme stands in for it.
- What Tailscale's admin console shows on a pending device (the page lists no fields); whether the
  device API returns the node key; whether `tailscale status` shows the node key on Windows. The
  enrollment proposal's own list of unverified Tailscale points still stands.
- Teleport TPM credential activation (the page did not describe it).
