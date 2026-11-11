# Principles review: the whole design against the owner's principles

Status: ruling 41: in force; amended by 42 (D6, D8, D12, names); amended by 43 (item 4c, Q3); amended by 44 (D14); amended by 58 (M22, M41) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PRV-001 to PRV-026](../../../spec/promises/providers.yaml), [SEC-001 to SEC-023](../../../spec/promises/secrets.yaml). This record says why.

Was: **proposed, not ruled.** Dated 2026-11-06. The owner asked for the whole design to be
gone through against the principles below, in the owner's words "with a fine tooth comb aligning
on these principles", and for the outstanding decisions to be brought back. This is that record.
Nothing here is part of the specification until the owner rules; where this record and `spec/`
differ, `spec/` governs.

How it was made. Reviewers read the specification, the core package, the schemas and the
[secrets proposal](../secrets/README.md), one area each: secrets, the language, access,
operations, the runtime and graphs. Each change they proposed was checked by two skeptics against
the specification and against the standard it cites, and each skeptic's corrections are folded
into the item they correct. A change that both skeptics refuted is listed in
[section 6](#6-considered-and-not-proposed). A final check of the whole record then found
contradictions between items, lines that were not in plain words, choices that rulings had
accepted and that were changed without saying so, and areas no reviewer had read: the
[implementation-defined list](../../../spec/implementation-defined.yaml), the location exports
of the core package and its [README](../../../spec/core/README.md). Those are fixed here. What
the final check added from its own reading of `spec/` is marked "added in the final check".

Promise IDs are those of [spec/](../../../spec/README.md) on this date, and terms are those of
[spec/definitions.md](../../../spec/definitions.md). The IDs [SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) to [SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) are drafted
in the secrets proposal and are not in `spec/`. Each item ends with the numbers of the review's
candidates it comes from, C1 to C58; these are this review's own, and not the choices of the same
numbers in the [core-and-schemas record](../core-and-schemas/README.md), which this record calls
"choice C15 of the core-and-schemas record" and so on.

## Contents

- [1. The principles](#1-the-principles)
- [2. Decisions for the owner](#2-decisions-for-the-owner)
- [3. Alignments the coordinator will make](#3-alignments-the-coordinator-will-make)
- [4. Secrets, reduced](#4-secrets-reduced)
- [5. Checked and found aligned](#5-checked-and-found-aligned)
- [6. Considered and not proposed](#6-considered-and-not-proposed)

## 1. The principles

In the owner's words:

- "The underlying infrastructure should be able to swapped out to bitbucket or Mac keychain etc,
  we don't care as long as it follows a shared contract." [Ruling 40](../ruling-40/README.md)'s
  record reads the first name as Bitwarden, a secret store; Bitbucket is a host of code and
  packages. This review covers both: secret stores in [section 4](#4-secrets-reduced), and where
  packages come from in decision 21.
- "We should not innovate on the architecture ourselves just the contracts."
- "We shouldn't overcomplicate this."

The standing direction ([purpose](../../AUDIT.md#purpose),
[current direction](../../AUDIT.md#current-direction),
[layered claim](../../AUDIT.md#layered-claim)): adopt existing paradigms and standards for
anything that is not what makes Conduit different, and invent nothing where a standard exists.
The relationship is the product: an agent decides for itself, so the language declares what it
may reach, under which limits, and who answers for it. The first user is one developer on one
machine; teams come later. Behavior beyond a few shared conventions belongs to type contracts or
to each engine's runtime profile.

Read as three tests for every part of the design:

- **P1. Swappable behind a contract.** Anything that is infrastructure (a store of secrets or
  data, a runtime or process supervisor, a transport, the store of records, identity and
  authentication, scheduling and placement, isolation, package distribution) is reached only
  through a shared contract: a type contract, a capability, an operation, a requirement or the
  implementation interface. The specification names no particular backing system, and the
  engine's own built-in is acceptable only as one implementation of that same contract.
- **P2. No architectural invention.** Where a standard or a widely used paradigm already does the
  job, Conduit adopts it as it is. Conduit's own inventions are limited to its contracts: the
  relationship model (Conduits and what they permit) and the shape of its type, capability,
  operation and requirement contracts.
- **P3. Not overcomplicated.** Machinery the first user does not need, rules that belong in one
  type's contract rather than in the language, and promises that exist only to patch other
  invented machinery are flagged.

P2 does not ask to replace the relationship model: who may reach what, through which Conduit,
under which limits, and who answers for it. No item below replaces it with a generic standard.
Items 3a and 3b come closest, since policies are part of how access is narrowed, and each says
what it would give up.

## 2. Decisions for the owner

Rule by number. Where an item has lettered lines, each line can be ruled apart. The items are
ordered by how much they simplify the design, largest first; the last ones add a little or
change nothing. Item 31 is the exception: the final check added it, and it is placed last so
that no other number changes. Each item says what the design does now, the standard to adopt,
the recommendation, and what it costs or reverses. Ruling 40 already decided the direction of
items 1, 2, 8 and 31: those items say what is decided, which the coordinator applies, and ask
only what it left open.

1. **Setup and retirement without graphs.** Now: a template may write `setup` and `retire`
   graphs that the engine runs, and an implementation may be a graph; about 26 GRA promises,
   five schemas and step clauses in about ten other promises serve them. Already decided (ruling
   40, point 3c): declared Conduits and entries keep their dependency order (GRA-022, CON-004,
   POOL-013 move to the Conduit and Pool promises), and an operation built from others becomes an
   ordinary implementation, a server that makes onward requests (IFC-020). Standard: Terraform's
   provider create and delete; Kubernetes PostStart and PreStop hooks; Compose `post_start` and
   `pre_stop`; workflow engines run as separate servers. Left for you:
    - 1a. An instance's own setup and teardown work. Recommended: two core Operation contracts,
      a setup hook and a retire hook, chosen through the `implementation` map like any operation;
      the setup hook runs once declared Conduits, entries and the runtime are up, the retire hook
      before the stop. Alternatives: hooks named by the Type contract, or no hook at all.
    - 1b. Every request to an implementation that carries Conduit's keys also carries the
      instance's configuration and its revision in `_meta`, as Terraform sends a provider the
      resource's configuration. Without it, once setup steps go, such an implementation has no
      way to learn its instance's configuration, such as the directory it is to serve. A plain
      server does not read Conduit's keys; decision 25d gives it its configuration as arguments
      and variables.
    - 1c. A further setup leaves what is already done as it is (an open Conduit, a ready
      Resource, a running runtime) and repeats the hook only as its operation identity allows
      (OPS-009, OPS-012). Three rules about running steps again go with graphs: DL1's item on
      what a re-run does, OPS-028's rule for giving each step an operation identity, and
      GRA-026's rules on the outputs of steps that ran before. One skeptic refuted a fixed re-run
      rule while graphs stay; this is the rule once they go.

   Cost: composing two operations means writing a small MCP server rather than YAML; a setup
   step can no longer run before one particular Conduit opens; a composed operation that starts
   model work without being marked for it is no longer caught when its template is registered
   (IMP-004), only refused when it asks for that work (IMP-008). Ruling it closes action A19 and
   questions E8 and D6. The value bindings that build a map or a list out of several values
   (`object` and `list`, GRA-019) go with graphs. If requirement composition stays (item 7), a
   constituent's forwarded inputs keep them, and the
   [graphs record](../graphs-and-schemas/README.md)'s statement that they have no counterpart
   in an existing framework is corrected: Terraform's object and list constructors are one.
   (C40, C41, C42, C43, C44.)

2. **The runtime provider contract.** Now: a template declares named runtimes, pairs of start
   and stop operations; the instance holds each handle; a reconciliation may state a handle
   (TPL-015); a type may allow restoring, and OBS-007 has the engine stop a restored runtime for
   no actor, which nothing can observe. Already decided (ruling 40, point 3a): a runtime provider
   runs an instance's implementation, stops it and reports its status. The coordinator writes it
   in the shape of Kubernetes's CRI and Nomad's task drivers: run takes the implementation item,
   its environment and the instance, and returns the endpoint; the engine's own process runner
   and Docker are two implementations. `runtimes`, held handles, `restoresRuntimes`, OBS-005 and
   OBS-007 go. With OBS-007 goes the one request that the engine sends on nobody's behalf;
   whether the read of a secret before a run is another is decision 4c. Who starts a process and
   over which transport it is reached, which IFC-017 and DL12 leave to each engine today, become
   the provider's: its run returns the endpoint. Left for you:
    - 2a. Identity, and one run per instance. Recommended: the engine names each run by the
      instance's ID, as the OCI runtime's `create <container-id>` and Nomad's task IDs let the
      caller name a run; stop and status take that name, and stopping what is not running
      succeeds, as CRI's stop calls do. No handle is held, `reconcile.v1` loses `handle`, and an
      actor settles a run of unknown outcome by reading its status. So one run serves one
      instance: this removes the choice that IFC-017 and DL12 give each engine over how many
      instances share a process, and DL12 keeps only the protocol revisions and the `content`
      member of a result. Alternative: the provider returns an opaque handle, as CRI's
      `CreateContainer` returns the `container_id` the runtime assigns, kept as Terraform keeps
      private state; one run could then serve several instances.
    - 2b. Restart policy. Recommended: the language says nothing; it is the provider's own
      setting (systemd `Restart=`, Compose `restart:`).
    - 2c. Secrets in a run (the secrets proposal's item 8f). This line decides whether an item
      that needs a secret can run on any provider, which is P1. Recommended (A): before each
      run it asks for, the engine reads each secret through the instance's own Conduit and
      passes the values to the provider in the run request, apart from the rest of the item. The
      provider delivers each as the variable the item names, by its own means, and states in its
      contract where it keeps the value: Docker keeps it in the container's configuration, which
      `docker inspect` shows, containerd in the OCI `config.json`, and systemd in the unit, whose
      variables its own documentation says unprivileged clients can read. The standard ways to
      keep a value out of those places hand it over as a file: systemd's `LoadCredential=`,
      Compose's `secrets:` and a Kubernetes secret volume. A provider may use them, setting the
      variable from the file as many images do for a `*_FILE` variable. The guarantee
      (section 4) counts the provider's place as inside the trust domain, as it counts the host.
      A restart that the provider makes under its own policy (2b) reuses the value it holds and
      makes no read; closing the Conduit does not recall it, as closing recalls no copy (the
      secrets proposal's item 6). (B), the earlier recommendation: the provider may not write
      the values down, and every start or restart is preceded by its own read. That conflicts
      with P1 and with 2b. Docker, containerd and systemd all write the environment down, and
      none of them reads anew when it restarts a process, so every item that needs a secret
      could run only on the engine's own runner.
    - 2d. Enforcement later. A restriction is reported as enforced only where the provider ran
      the consumer under a runtime class whose contract confines it to reach targets only through
      the engine, as Kubernetes RuntimeClass names gVisor or Kata. The user picks that class for
      the instance, as item 31b says, so enforcement is opted into through the language
      (preservation test 9). Ruling 15's "plugins that select an isolation framework" are read as
      such classes. In v0 every restriction stays mediated (ACC-027), and "isolation plugin"
      leaves the definitions.

   Cost: the runtime decisions of rounds 2 to 4, which ruling 39 accepted, go, as ruling 40
   already said; a type that needs several long-lived processes runs them as one.
   (C45, C46, C47, C53, C6, C57.)

3. **Policies.** Now: a policy is a requirement applied on `access` and enforced by the target's
   implementation. `engine` supports only `bound.v1`, so an actor-wide policy makes its actors'
   requests to `management`, `access`, Pools, locations and every `plain` server unavailable
   (ACC-015, ACC-026, ACC-031). Policies also pin a package version and are judged again along a
   chain (ACC-017), and need ACC-042 and parts of about six more promises. Standard: a separate
   policy decision point behind the OpenID AuthZEN Authorization API 1.0 (final, January 2026),
   served by OPA, Cedar, Cerbos or Topaz, with the engine as the enforcement point. This item
   changes what Conduit is for, and each line says what it gives up.
    - 3a. Remove policies, `apply-policy.v1` and `remove-policy.v1`. Recommended. What it gives
      up: the purpose says each agent's access "can be inspected, narrowed and revoked
      independently of both ends", and ruling 29 made a temporary limit a policy (CC3). Without
      policies an open Conduit can no longer be narrowed in place: narrowing becomes opening a
      narrower Conduit and closing the wider one, which an agent with the right to open can
      still do (preservation test 5), and an agent can no longer apply a policy through
      `access`. Reverses CC12 (ruling 28), CC3 (ruling 29), RD8 (ruling 19), U7 (ruling 32) and
      ruling 38 point 7. The smaller fix, policies never applying to targets the engine
      implements, was rejected at ruling 38 point 7 and still locks actors out of plain servers.
    - 3b. An outside decision point. Where the user names one, the engine asks it about each
      permitted request from a caller other than the root Interface (subject: the caller;
      action: the operation and its arguments; resource: the target) and dispatches only on
      `true`. It only narrows, no answer reports unavailable, and its answer is recorded. What it
      gives up: a limit kept there cannot be applied by an agent through Conduit (test 5) or
      seen by inspecting a Conduit (test 2), so it is a second rule beside the relationship
      model. Tested against P3: the first user needs none. Recommended: not in v0; later, if
      wanted, a provider that the user picks (item 31).

   If 3a and 8a both pass, a Requirement contract's `appliesTo` has only `conduit` left, and the
   key itself can go. (C26.)

4. **One way to read a secret in v1.** Now (secrets proposal, not ruled): an implementation may
   read a secret when it needs it (item 3a), which brings [SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and exceptions to ACC-009,
   ACC-017, ACC-031, ACC-035, IFC-020 and OPS-016; the engine also reads at process start, as
   ruling 39 points 3 and 4 accepted. Standard: Kubernetes `secretKeyRef`, ECS `valueFrom` and
   Cloud Run read at start; MCP says a stdio server takes its credential from the environment.
    - 4a. One path. Recommended: in v1 a secret is read only by the engine, through the
      instance's own Conduit, before each run it asks a provider for (2c). Reading at use waits,
      with workload identity, as the later steps.
    - 4b. The reservation, in plain words. No actor's request for `reveal-secret.v1` is ever
      dispatched: no consumer is presented a tool for it, and a call for it reports refused.
      Only the engine's read under 4a is dispatched. The root Interface may still open a Conduit
      that lists `reveal-secret.v1` for a Resource or an Agent, and that Conduit is the one the
      read goes through. This is not ACC-032's form, which would refuse to open that Conduit.
      It replaces the proposal's [SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and its output mark `secret` (section 4).
    - 4c. Whose read it is (added in the final check). Recommended: the read belongs to the
      request that asked for the run: the setup of the instance, or a retirement that runs it to
      serve its retire hook (1a). Its record names that invocation and its caller. It is decided
      by the instance's own Conduit and the filters on it alone, and carries none of that
      caller's restrictions, so who set the reader up does not change what it reads. The engine
      asks for a run only within such a request, and a restart that a provider makes on its own
      makes no read (2c), so every request the engine sends has a caller. Alternative: the
      proposal's read for no actor (its item 8e, [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), which keeps a request with no caller
      in IFC-006 and REC-002.

   Cost: in v1 a process sees a secret only as a variable, which ECS and Kubernetes rank lowest;
   an implementation reached over HTTP gets no secret unless 25c passes, and without 25c a login
   to a remote service cannot be established once and shared under restriction, the first thing
   the [purpose](../../AUDIT.md#purpose) says Conduit is for; each item that needs a secret runs
   one process per instance; a rotated value arrives at the next run the engine asks for. (C4.)

5. **Composite Conduit templates: remove them.** Now: a composite Conduit template expands into
   several pairwise Conduits, with its own naming and length rule (CON-003, CON-029), a whole
   decision at setup (CON-004), dotted step paths (GRA-024), an extra segment in tool names
   (IFC-012) and `components` (CON-016, CON-019). Standard: Kubernetes has no expanding binding,
   since one RoleBinding binds one role; reuse lives in templates, as with Terraform modules and
   Helm charts. Recommended: remove them; a template that needs two capabilities of one target
   declares two Conduits, which is what a composite expands to today. Letting one Conduit span
   capabilities is not recommended: it changes what one Conduit is. Cost: reverses RD4 (ruling
   19), ruling 38 point 3 and ruling 39 point 5; two declared Conduits can be decided apart, one
   opened and one refused; a named bundle can no longer be opened onto a live instance by one
   document. (C27.)

6. **Results and retrieval as MCP and idempotent APIs define them.** Now: `isError` is true on
   any result without outputs, including a succeeded one whose content is withheld (IFC-025); a
   repeat of an invocation still running gets a result with neither outcome nor reason; a result
   whose Conduit closed while it ran is withheld (ACC-034, OPS-017); and `retrieve-result.v1`
   reads a result with no open Conduit, the one exception to "every permission is an open
   Conduit" (IFC-024). Standard: MCP's `isError`, "whether the tool call ended in an error"; the
   HTTP Idempotency-Key draft and Google AIP-155, where a repeat with the same key returns the
   earlier result and gets 409 while it is still running; access decided once per request, as
   in Kubernetes and OAuth; MCP's tasks extension for work that outlasts its call.
    - 6a. An invocation permitted when it was dispatched delivers its outputs, even if its
      Conduit closes while it runs. Every later read is decided anew.
    - 6b. `isError` is true only where the request ended with a reason or the outcome is failed
      or unknown. A repeat of an invocation still running reports conflict, with the engine's ID.
    - 6c. Remove `retrieve-result.v1`, IFC-024, OPS-015 and their carve-outs. A caller that lost
      a result repeats the request with the same identity through an open Conduit (OPS-009). The
      tasks extension is allowed where both sides declare it, and `tasks/get` and `tasks/cancel`
      are decided under the Conduit the invocation went through.

   Cost: revocation no longer reaches a response already in flight; a caller whose Conduit
   closed learns the outcome from the records. Reverses part of OD3 (ruling 20), U6 (ruling 32),
   the round-2 rule on `isError` that ruling 38 point 11 asked to revisit, and a core export
   accepted at ruling 37. (C30, C31, C32.)

7. **Requirement composition: remove it.** Now: a Requirement may be built from constituents
   with forwarded inputs (CTR-010), which needs CTR-011 to keep `bound.v1` from being hidden, a
   cycle check, a second matching rule for `supports` and composition clauses in about eighteen
   places. Standard: contracts are atomic, as Terraform provider schemas and Kubernetes CRDs
   are; Cedar, IAM and Kubernetes admission combine restrictions by applying several side by
   side, which a Conduit template's `requirements` map already does. Azure Policy initiatives are
   the model if bundles return for teams. Recommended: remove `constituents`, CTR-010 and
   CTR-011. Cost: reverses GP1 (decision DA3) and round-2 decisions R2-G3 and 10, which ruling 39
   accepted; no bundle can be built from other packages' requirements, and a bundle applied as a
   policy becomes several. (C18.)

8. **Placement after ruling 40, point 3b.** Now: locations are Resources registered by hand with
   facts, which a control changes; a placement need is a Requirement contract with a schema over
   facts (CTR-005); the shared-host requirement is checked on every request, because
   `set-facts.v1` could change a location's `host` (PLC-013); a default location is a control of
   `system` (PLC-003, PLC-016); how it is detected that a location cannot be reached is each
   engine's (DL5). Already decided: locations stay in the language; a provider reports the
   machines, their labels and when one is lost, and places instances; what placement and loss
   mean for access stays Conduit's. The coordinator removes what point 3b removed:
    - the default location: PLC-003, PLC-004, PLC-016, PLC-022, `system-settings.v1`,
      `set-default-location.v1` and `system-pool.v1`;
    - locations registered by hand, and their facts: `set-facts.v1`, `location-settings.v1`,
      `facts` in PLC-008, PLC-021, the argument `facts` of the core template `location`, and
      the facts that DL6 gives `local`. The type `location.v1` stays, with no control.

   Left for you:
    - 8a. A placement need is written inline, in the Type and in the implementation item, as a
      Kubernetes `nodeSelector`: a map of label to value, matched against the labels the
      provider reports. Both maps must match, as Kubernetes merges a RuntimeClass's selector
      with a Pod's: a label that both name with different values is rejected. Requirement
      contracts no longer apply to placement.
    - 8b. The shared-host requirement compares the provider's `kubernetes.io/hostname` label and
      is decided once, when the Conduit is instantiated. Nothing changes a location's host once
      `set-facts.v1` is gone, and an instance's placement never changes (PLC-009); Kubernetes
      likewise checks pod affinity only when it places a pod, not while the pod runs. The check
      on each request goes. Reverses decision G13 (ruling 32).
    - 8c. A document that gives no placement and no labels is placed by the provider, as the
      Kubernetes scheduler places a Pod; a one-machine provider places it on that machine.
      Alternative: v0 promises `local`.
    - 8d. How a reported machine becomes a location (added in the final check). Recommended: the
      engine registers a location from the core template `location` for each machine the
      provider reports, named by its hostname, as a Kubernetes kubelet registers its own Node;
      its labels are reported status, not configuration. FST-001's `local` stays, as the
      location of the machine the engine runs on, where `management` and `access` are placed.
      Alternative: the root Interface registers each location, naming a machine the provider
      reports.
    - 8e. A lost machine (added in the final check). Recommended: the provider reports two
      things, as Kubernetes's node lifecycle does. Unreachable: requests to instances placed
      there report unavailable and nothing is retired; this replaces DL5. Gone: the machine no
      longer exists, as a cloud provider reports a deleted machine, after which Kubernetes
      deletes the Node. Gone has PLC-017's effect, as if the root Interface had declared the
      location lost: its instances are retired and their Conduits closed. The root Interface may
      still declare one lost. Alternatives: every report of loss retires, and since retiring is
      forward-only, a laptop that sleeps would lose its instances for good; or only the root
      Interface's declaration retires.

   Cost: changes the shape of a placement need accepted under rulings 28 and 32.
   (C54, C55, C56.)

9. **Re-exports: drop them from v0.** Now: a package may re-export another package's contract
   (MAN-008, PKG-013, PKG-014), so bounds, policies, `supports` and the wire each need a rule
   about the package that originally exports a name. Standard: a Kubernetes kind belongs to one
   API group and a Terraform resource to one provider, and a consumer names the source.
   Recommended: drop them; `exports` lists only the package's own documents. They can be added
   back later without breaking anything; removing them after packages are published would break
   those packages. Cost: reverses PR5 (ruling 24), choice C49 of the core-and-schemas record
   (ruling 37), and round-2 and round-4 decisions that ruling 39 accepted. (C13.)

10. **Rename: drop it from v0.** Now: `rename.v1` changes an object's name (OBJ-026), with
    OBJ-005, OBJ-027 and OBJ-028, CON-026's check of tool names on each rename, and LIF-022's
    protection of first-start names; renaming an open Conduit changes its consumer's tool names.
    Standard: Kubernetes `metadata.name` "Cannot be updated". Recommended: a name is fixed once
    the object is made; the resolved form `{kind, id}` stays, since a name can be reused after
    an object is unregistered. Cost: reverses DR5 (ruling 21) and the rename part of ruling 38
    point 12; renaming becomes registering a new object and retiring the old, so a renamed Pool
    sets up its Resources again. (C22.)

11. **The developer's own contracts get a usable name.** Now: a project's contracts are kept as
    an unpublished package named by a hash of its documents (SUB-008) and written
    `<digest>#<name>`; bounds and policies name published contracts only (ACC-029, ACC-031). So
    the developer cannot give an agent a bounded right to open Conduits to their own tools, or
    apply their own requirement as a policy, without building and installing an archive.
    Standard: Go's `module` line, which names a module under development by its declared path.
    Recommended (A): a project with contracts states `address`, and `version` is optional; its
    contracts are `address#name` everywhere; a project may not state the address of an installed
    package. Smaller (B): let a bound name `#name` and keep the hash. Cost: A reverses coordinator
    decisions 4B-4, R2-S6 and R1-2.6/R2-C4, which ruling 39 accepted; B does nothing for policies,
    and a bound stops covering an operation each time the project is edited. If 3a passes, there
    are no policies, and this item is about bounds alone. (C11.)

12. **Install before register.** Now: a document whose package is not installed is registered
    with a diagnostic, and every check that needs the package is made again at instantiation
    (PKG-006, PKG-028, SUB-008, INS-022), with deferral clauses in about fifteen places.
    Standard: Kubernetes rejects a custom resource whose CRD is not installed; Terraform runs
    `init` first; Helm builds a chart's dependencies first. Recommended: a registration or an
    installation is rejected when its manifest lists a package not installed at that version and
    digest, with a diagnostic naming each; the reference engine's command line may offer one
    command that installs them, as `terraform init` does. PKG-007 stays, since an installed
    package can still be removed. Cost: reverses PR2 (ruling 24) and the "missing packages are
    reported" half of ruling 38 point 8. (C12.)

13. **Secret stores: none built into the engine, and each store's own credential.** Now (secrets
    proposal, not ruled): `engine` reads a secret from a file or from a variable of its own
    environment ([SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), which every engine must then do; the variable source needs [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow),
    under which anyone who may register can withhold `PATH` from every process. A store in a
    package reaches its store with whatever credential its client finds in the environment it
    is given (the proposal's item 16), which item 2 does not say. Standard: Dapr lists its local
    file and local environment stores as components beside Vault and AWS, behind one API;
    External Secrets Operator and Dapr read a store's own credential from another secret
    (`auth.secretRef`, `auth.secretStore`).
    - 13a. Recommended: `engine` implements no store. A file store `{path}` ships in the
      standard library beside the reference engine (layer decision LD5), as one implementation
      of the same contract, and no store reads the engine's own environment in v1, so [SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow),
      [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and the PLC-020 exception go. The reasons: every engine would otherwise have
      to implement two stores, and the variable source needs [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow). Alternative: keep the
      variable source, counting only secrets that an open reading Conduit leads to (the
      proposal's least-certain item 12). Cost: the first user installs the standard library once
      and moves an exported token into a file or a store; v1 departs from Compose's and Dapr's
      environment source.
    - 13b. A store's own credential (added in the final check). The Bitwarden store needs
      `BWS_ACCESS_TOKEN`, and the 1Password store a service account token, in the environment
      of the store's process. Recommended: such a credential is itself a secret Resource, read
      for the store's process under decision 4 and bound by 25d, as External Secrets Operator
      and Dapr read a store's credential from another secret; a chain ends at a store that needs
      no secret, the file store or the keychain. Cost: the store's item binds a secret, so each
      secret Resource of that store runs its own process (the proposal's 8d); the token itself
      is kept once, in a file or the keychain. Alternative: the provider passes named variables
      of the user's environment through to the process, as Compose's `environment:` does for a
      name given without a value. The credential is then outside every Conduit and unrecorded,
      as the proposal's item 16 accepted, and item 2's run contract must promise the
      pass-through.

    What else a store needs from where it runs, such as the home directory for an AWS profile
    or the user's login session for the keychain, is part of its contract (section 4). (C2,
    C7.)

14. **Filters bind the target only: remove `obliges`.** Now: a requirement may state `obliges:
    [consumer]`, which makes the consumer's implementations state it in `supports`, checked only
    against the manifest; IFC-034 publishes filters in each tool's `_meta`. The target already
    applies every filter (CON-022, IFC-018, IFC-019), and `obliges: [target]` adds nothing.
    Standard: OAuth and MCP enforce at the resource server. Recommended: remove `obliges`,
    IFC-034, and the consumer clauses of CON-022 and ACC-038; guidance for a consumer comes back
    in the target's results, as HTTP's `Cache-Control: no-store` does, and machine-enforced limits
    on a consumer wait for item 2d. Cost: reverses round-2 decisions 8 and 10, part of decision 7
    and the `obliges` half of GP15 (decision DA3), which ruling 39 accepted. (C19.)

15. **Controls: keep Terraform's model, without clamping.** Now: a control is sent, the
    implementation confirms the settings it applied, which may differ from those asked (a limit
    of 20 clamped to 15), and exactly those are stored (REV-007), with REV-008, REV-013 and the
    `settings` input of `reconcile.v1` to check them. This is Terraform's apply model, which
    ruling 30 chose. One reviewer proposed storing the declared change first and reporting whether
    it took effect, as Kubernetes `observedGeneration` does; one skeptic refuted that, because
    without a controller one observed revision can show a failed setting as in force.
    Recommended: keep the model and forbid clamping, as Terraform does ("saved exactly as-is"): a
    control applies exactly what it was sent or fails, the engine checks the result against the
    type's schema before dispatch, and on success stores what was sent. The output `settings`,
    REV-008, most of REV-013 and the `settings` input of `reconcile.v1` go. Cost: an
    implementation can no longer clamp; reverses that part of OR8 (ruling 20). (C37.)

16. **Operation contracts written as MCP tool definitions.** Now: an Operation contract's inputs
    and outputs are maps of members (`schema` or `handle: true`, `optional`, `default`) that
    IFC-033 turns into MCP's `inputSchema` and `outputSchema` (CTR-002, OBJ-015, OPS-026), and a
    tool carries none of MCP's annotations. Standard: the MCP Tool definition, JSON Schema with
    `required` and `default`, and MCP's tool annotations.
    - 16a. The schema form. Recommended: `spec` holds `inputSchema` and `outputSchema` as MCP
      writes them, the root limited to `type: object`, `properties`, `required` and
      `additionalProperties`; a handle is a top-level property whose schema is a core
      `handle.v1`; a control keeps REV-006. Template inputs keep their member form. Cost: narrows
      CS6 (ruling 28): a contract marks an optional input by `required`, a template by
      `optional: true`. (C15.)
    - 16b. Tool hints. Recommended: the engine sets MCP's `readOnlyHint` true on a tool only
      where its contract states no `effect`, is not a control and does not start model work
      (IFC-033); a contract may carry `idempotentHint`, `destructiveHint` and `openWorldHint`,
      which the engine passes on as hints. This line stands whatever 16a decides. Earlier drafts
      also listed its first half as alignment C16, which is withdrawn. (C16.)

17. **Removal after retirement.** Now: an instance is retired, then removed by a second request,
    `unregister.v1`, once its shutdown is confirmed, naming a revision that can no longer change
    (LIF-017). Standard: Kubernetes removes an object when its finalizers finish; Docker keeps a
    stopped container until `docker rm`. Recommended: remove a retired instance once its shutdown
    is confirmed, as Kubernetes does; one whose shutdown is pending or unknown stays until it is
    settled, and one on a location declared lost is still removed by request. `unregister.v1`
    stays for templates. Alternative: keep two steps, cite Docker, and make the revision optional.
    Cost: a closed Conduit is removed when it closes, so a request through it reports invalid
    rather than refused; a removed instance is known only from its records. Reverses OL3 (ruling
    30), ruling 29's "exists until unregistered" and part of LIF-023. (C38.)

18. **Conduit's keys as an MCP extension.** Now: every request to an implementation that is not
    plain carries `interfaceVersion`, and an implementation that does not support it answers with
    a tool result marked as an error (IFC-006, IFC-008, IFC-029). Standard: MCP's extension
    negotiation, an identifier with a vendor prefix advertised in `server/discover` and declared
    on each request in `clientCapabilities`, with a `revision` setting and the error -32021.
    Recommended: define the keys as one extension under the prefix in `spec/names.yaml`, whose
    `revision` setting is `v0`. The engine reads an implementation's discovery result before its
    first request; where the extension is missing, the request reports unavailable, and the
    engine never falls back to sending it without the keys. `plain` stays in the manifest. Cost:
    modifies decision I2 (rulings 31 to 33); IFC-008 and IFC-029 are rewritten, not removed.
    (C50.)

19. **One Type per secret store.** Now (secrets proposal): one core type `secret.v1` with no
    schema, so no locator is checked when it is written and no control can move one (CTR-004,
    CTR-006); a new version of a secret means a new secret Resource and remaking its readers.
    Standard: Terraform's per-provider schemas and External Secrets Operator's typed providers;
    in Conduit, every Pool type already offers the core `pool-management.v1`. Recommended: the
    shared contract is the core capability `secret-reveal.v1`; a secret Resource is one whose
    type offers it; each store is its own Type with a locator schema (`op-secret.v1`,
    `keychain-secret.v1`, `bws-secret.v1`, the file store's). In v0 such a type lists no control,
    so only the root Interface decides what a reader receives. Cost: refines ruling 38 option B
    ("a secret type in the core package") to a core capability, and reopens option D of the
    proposal's S1; secret types multiply, one per store. (C3.)

20. **The schema profile: JSON Schema 2020-12 with stated exclusions.** Now: the profile admits a
    subset, without `if`, `not`, `propertyNames`, `contains`, `dependentRequired` or
    `uri-reference`, on the claim that Kubernetes narrows OpenAPI the same way, which it does not;
    so rules the schemas could state are checked by hand-written code. Standard: JSON Schema
    2020-12 as OpenAPI 3.1 and MCP use it. Recommended: accept the applicator vocabulary, and the
    validation vocabulary except `multipleOf`; keep out `$id`, `$schema`, remote `$ref`, `$anchor`
    (it collides with `#name`), `$dynamicRef`, the unevaluated vocabulary and `$vocabulary`;
    reject a `default` under any conditional keyword (extending SCH-008); add `uri-reference`.
    Then the key rules of OBJ-022 and NAM-001, part of CTR-004 and MAN-006's syntax move into the
    schemas. Cost: partly reverses SD1 and SD2 (ruling 23). (C17.)

21. **Where packages come from.** Now: how an address becomes a place to fetch from, and which
    `fetch` schemes work, is each engine's (DL7, NAM-012), so a manifest that installs on one
    engine can fail on another. Ruling 40 point 2 names the source of packages as something the
    user swaps. Standard: `https:` and `file:` URLs (RFC 3986, RFC 8089) checked against the
    pinned digest, as server.json's `mcpb` packages pair a URL with `fileSha256`; mirrors.
    Recommended: every engine accepts `https:` and `file:` URLs, a relative `fetch` resolving to
    `file:`; a scheme it does not accept reports invalid; `install.v1` with no `fetch` for a
    package not installed reports invalid; an engine may take the same bytes from a cache or a
    mirror the user names; a URL never carries a credential, which comes from the scheme's own
    mechanism (netrc, Docker's credential helpers, the macOS keychain's among them). Any other
    source, such as an OCI registry, is a package-source provider that the user picks by the
    scheme of `fetch` (item 31). Moving a package from GitHub to Bitbucket's downloads or to a
    folder then changes one URL on every engine. Cost: reverses NE's "by default it is also the
    source" (ruling 27), which `spec/` never implemented. (C9.)

22. **Cancellation: the protocol's own.** Now: whether and how an invocation can be cancelled is
    each engine's (DL14). Standard: MCP's cancellation, `notifications/cancelled` on stdio, a
    closed stream on Streamable HTTP, `tasks/cancel` for a task; a receiver may ignore one it
    cannot honor. Recommended: delete DL14, since how long the engine waits is already DL3 and
    DL10; a protocol cancellation of the request that dispatched an invocation cancels it under
    OPS-027, and the engine passes it on to the implementation by the protocol's means. No
    confirmation format is invented, so a cancelled invocation is usually unknown until it is
    reconciled. Cost: a dropped HTTP connection stops the implementation's work and leaves the
    outcome unknown. Alternative: the engine may keep the work running so its result can still be
    fetched. (C35.)

23. **A busy instance.** Now: a request to set up, retire or change an instance while another is
    in progress is each engine's (DL9, REV-012): one engine may queue it, another reject it.
    Standard: Terraform's state lock fails at once by default; HTTP 409, gRPC ABORTED, Azure's
    AnotherOperationInProgress. Recommended: such a request, unless it is a repeat under OPS-009,
    is not dispatched and reports conflict at once, including the engine's own steps.
    Alternative reason: unavailable. Cost: an engine may no longer queue; reverses the clause of
    DL9 that ruling 31 accepted. (C36.)

24. **Shapes that were left to the conformance vectors.** Now: a diagnostic fixes only `message`
    (`diagnostic.v1`), so a stale revision, a wrong state and a taken name all report conflict
    and differ only in prose; an inspection result fixes only its top-level names; a record has
    no shape. Choices C16 and C38 of the core-and-schemas record, which ruling 37 accepted, left
    these minimal on purpose, and the tracker lists them as
    [settled with the conformance vectors](../../AUDIT.md#for-the-next-reader). Each line fixes
    one now; the cost of each is that the shape is fixed before the vectors.
    - 24a. Diagnostics. Standard: the SARIF 2.1.0 `result` (`ruleId`, `level`, `message.text`,
      and `locations` with a JSON Pointer as a logical location); Kubernetes `metav1.Status` is
      the alternative. Recommended: SARIF's subset, `ruleId` being the promise ID; every result
      that reports invalid or conflict carries an `error` diagnostic naming the promise that
      decided it; refused results stay bare under OPS-023; no table of HTTP statuses. Cost:
      renumbering a promise becomes a change of version. (C24.)
    - 24b. Conditions. Recommended: inspection reports `conditions` in the Kubernetes
      `metav1.Condition` shape as it is (`type`, `status`, `reason`, `message`,
      `lastTransitionTime`, `observedGeneration`) (LIF-011). The time of the last successful
      observation and the origin that OBS-002 and OBS-003 require are not in that shape; they
      are added as `lastHeartbeatTime`, which Kubernetes's NodeCondition has, and `origin`,
      which is Conduit's own, and are marked as additions. `shutdown` stays as drafted until
      item 2 is applied. (C23, an alignment in earlier drafts, which added the two members
      without saying that `metav1.Condition` lacks them.)
    - 24c. Records. Recommended: a core schema `record.v1`, a CloudEvent with REC-002's fields in
      `data`; DL8 stays, and an engine may also export records in that form. (C34, an alignment
      in earlier drafts.)

25. **Implementation items: adopt MCP's server.json.** Now: an item names a `command` or a `url`
    (MAN-009, MAN-010); a published server is reached by a hand-written command line, a remote
    server cannot be given headers, `[docker, run, ...]` fixes the container runtime, and a plain
    server can be given nothing from its instance's configuration. Standard: the MCP Registry's
    server.json, schema 2025-12-11: a Package (`registryType`, `identifier`, `version`,
    `runtimeHint`, `transport`, `packageArguments`, `environmentVariables`) and a Remote (`type`,
    `url`, `headers`). The package declares what the server takes, `isSecret` marks a secret, and
    the client's own configuration supplies each value.
    - 25a. Packages. Recommended: keep `command` for local and in-package programs; add
      `package`, one server.json Package, pinned to that schema and with a required digest.
      Which package types a runtime provider can run is part of its contract (item 2). Cost: a
      newer server.json is a change to the specification; MAN-009, MAN-010, PKG-019 and the item
      schema are rewritten.
    - 25b. Remote servers. Recommended: replace `url` with `remote`, limited to
      `streamable-http`.
    - 25c. Headers of a remote server (added in the final check). Recommended: a `remote` may
      name headers whose values come from places in the instance's configuration, as
      server.json's Remote leaves them to the client. A place that `secret-reference.v1` governs
      is read through the instance's own Conduit when the engine connects for the instance's
      setup, as before a run (decision 4), sent only to that URL and held only for that
      connection. Without it, no remote server that needs a credential can be used in v1, so a
      login to a remote service cannot be established once and shared under restriction.
      Alternative: no headers until credentials for remote servers are decided.
    - 25d. Values from the instance's configuration (added in the final check). Recommended: a
      `command` or `package` item may give each argument and each variable it declares the place
      in the instance's configuration that supplies its value: a JSON Pointer that the Type's
      schema must require, checked when a template selects the item, as the secrets proposal's
      8b had it for secrets. A place that `secret-reference.v1` governs is filled by the read of
      decision 4, which is what server.json marks `isSecret`. This is also how a plain server,
      such as an off-the-shelf filesystem server, learns the directory it serves once graphs go,
      which 1b does not reach. Alternative, for secrets only: bind a variable to the local name
      of a declared Conduit (C5 of earlier drafts). No Type states such names, so a package's
      item would depend on how each template names its Conduits, and nothing would check it when
      the package is installed; Kubernetes `secretKeyRef` and Compose `secrets:` bind a value in
      the workload's own definition, as this line does, not in the image.

    (C10, C5.)

26. **Identity: a contract, with the mechanism swappable.** Now: what a request presents to
    identify its instance is each engine's (DL13, IFC-022). Ruling 40 point 2 left identity to
    each engine "for now", in standard formats, and point 3e requires a request that crosses
    machines to carry cryptographic proof of the instance that sent it, by a mechanism that is
    swappable: mutual TLS, SPIFFE or MCP Authorization. Standard: MCP Authorization, under which
    an HTTP server is an OAuth 2.1 resource server (RFC 9728, RFC 8707, Bearer tokens) and any
    authorization server can be used; mutual TLS; SPIFFE's X.509 and JWT identity documents.
    - 26a. The contract. Recommended: IFC-022 stays the promise, and DL13 is written as a
      contract that any of the three mechanisms implements: from what a request presents, the
      mechanism establishes exactly one instance or none, by a cryptographic proof wherever the
      request crosses machines. A token or a certificate says who is asking and nothing more,
      and the Conduits still decide. Stdio on one machine stays DL13.
    - 26b. MCP Authorization as the engine's mechanism on HTTP: the engine's server is a
      protected resource; the instance is found from the token's issuer and subject; tokens are
      checked as RFC 9068 JWTs or by RFC 7662 introspection. As one implementation of 26a it
      keeps 3e; as the only one it would narrow 3e, and it is not recommended as that.
    - 26c. The engine issues tokens itself: `root`'s to a file at first start, as kubeadm writes
      `admin.conf`, and an Agent's into its environment.

    Tested against P3: the first user is on one machine over stdio and needs neither 26b nor
    26c. Recommended: 26a now; 26b and 26c when a client connects over HTTP or a second machine
    joins. Cost: whether Claude Code and Codex connect over HTTP unattended waits on the native
    probe (A4). (C48.)

27. **Requests for input from a server.** Now: when an MCP server asks the user to confirm or to
    give input (`input_required`), the call ends unknown (IFC-010), so a server that confirms
    before acting, or signs in through a URL, cannot finish through Conduit. Standard: MCP's
    multi round-trip requests: the server returns `inputRequests` and `requestState`, and the
    client retries with `inputResponses`. The relay: the engine relays `elicitation/create`
    only, passes `inputRequests` on, issues its own protected `requestState` naming the
    invocation, and treats the retry as the next round of the same invocation, decided again
    through the Conduit and recorded. It declares no `roots` or `sampling` capability: both are
    deprecated at the baseline, and sampling would be model work. Tested against P3: it adds a
    rule for rounds beside OPS-009 and OPS-010, and the first user needs it only for a server
    that asks for input; such a call ends unknown, which is safe. Recommended: not in v0;
    IFC-010 stays, and the relay is the next step. Alternative: the relay now. Cost of the
    relay: changes the table for plain servers that ruling 31 accepted. (C49.)

28. **Bounds and what delegation can narrow.** Now: a bound narrows by Pools, targets and
    operations (ACC-029), never by filters, and a Conduit to `management` cannot carry one, so
    handing out `register.v1` hands out all registration. Delegation never widens (ACC-030), but a
    delegator cannot hand out access that must come with its filters. Standard: AWS states the
    trust that remains ("Maria effectively trusts Zhang"); Kubernetes limits which roles a
    delegate may bind with `bind` and `resourceNames`. Recommended for v0: keep the bound and
    state in ACC-030 that a right to open trusts the delegate with unfiltered access within its
    scope, and that `register.v1` or `replace.v1` on `management` trusts it with every object of
    that kind. For teams: a bound that names the templates it covers. Cost: no machinery; the
    purpose's "delegate a narrower slice" holds along targets and operations only. (C28.)

29. **The package digest: keep it.** Now: a package is identified by the SHA-256 of its archive as
    distributed (PKG-004, PKG-022), as OCI blobs, npm and Cargo are, so a host that rebuilds the
    archive cannot meet the pin. One reviewer proposed Go's `h1:` hash of the unpacked files, so a
    package could come from any carrier; one skeptic refuted it. Recommended: keep PKG-004, whose
    digest is already OCI's blob digest, and close the real gap with item 21; if checkouts or
    repacking mirrors matter later, add an optional `h1:` hash beside it, as Terraform's lock file
    keeps both. Cost of the alternative: it reverses PD3 (ruling 24), `sha256sum` no longer checks
    a package, and links and file modes need new rules. (C8.)

30. **Generated names: keep DL15.** Now: an unnamed instance takes its template's name and a
    generated suffix whose length and characters are each engine's (DL15, OBJ-003), as the
    Kubernetes API itself leaves them. One reviewer proposed fixing five characters, as the
    Kubernetes API server does; one skeptic refuted it. Recommended: keep DL15 and cite the API
    server's generator as one choice that meets it; fix the length only if composite Conduit
    templates stay (item 5), since only then does a suffix's length decide whether CON-029 rejects
    a nested composite. Cost: none. (C25.)

31. **Providers: what one is in the language, and how the user picks one** (added in the final
    check). Now: ruling 40 point 2 makes the runtime and the source of packages swappable by the
    user, and point 3b the machines; the owner asked to go "as far as we can in the 'swappable
    by user' direction". Section 4 makes a secret store objects in the language: a Type, a
    Resource and a Conduit. Items 2, 8 and 21 say what the other providers do, but not what they
    are in the language or how a user picks one, and earlier drafts left the choice with the
    engine in three places: 2d's runtime class, 3b's decision point and 21's other sources.
    Standard: Kubernetes reaches runtimes, storage and networks through CRI, CSI and CNI, and a
    workload picks one by naming a class object, a Pod's `runtimeClassName` or a claim's
    `storageClassName`, which maps to a handler set up on the node; Git picks the helper for a
    URL by its scheme (`git-remote-<scheme>`).
    - 31a. What a provider is. Recommended: as a secret store is, an installed package's
      implementation of a core provider capability, one for each kind: run, stop and status for
      a runtime; the machines, their labels and their loss for machines; fetching an address and
      version for a package source. The engine's own process runner, its own machine and its
      `https:` and `file:` fetching are the built-in implementations of the same capabilities.
      The engine's runner starts each provider's own server, as kubelet reaches its runtime at a
      socket its configuration names. Alternative: engine plugins set in the runtime profile,
      which leaves the choice to the engine, against ruling 40 point 2.
    - 31b. How the user picks one. Recommended: the user registers a Resource of a provider's
      type, configured as that provider needs (a Docker context, a gVisor runtime class). An
      Agent or a Resource names the runtime it runs on as it names its placement, by a standard
      argument fixed for life, as a Pod names its `runtimeClassName`; one that names none runs
      on the engine's own runner. A runtime class that confines its consumer (2d) is picked the
      same way, which is how enforcement is opted into through the language (preservation
      test 9). A package source is picked by the scheme of `fetch`, as Git picks a remote
      helper. The machines provider is the one the root Interface registers, and with none,
      the engine's own machine. Alternative: the runtime is picked by the Type or by the
      implementation item, which takes the choice from the user.

    Cost: one more standard argument on every Agent and Resource. Tested against P3: the first
    user picks none and gets the built-in ones.

## 3. Alignments the coordinator will make

The coordinator may make these under ruling 37 without a ruling; the owner can strike any by its
candidate number. Earlier drafts listed five more here. C1, an input `locator` that the engine
fills, is withdrawn: under 1b a store already learns its locator as every implementation learns
its configuration, and if 1b is rejected, what replaces it serves stores too. C5 changes the
shape of a contract and is now the alternative in decision 25d. C16 is decision 16b. C23 and C34
fix shapes that ruling 37 left to the conformance vectors, as it left the diagnostic, and are
now decisions 24b and 24c.

- C33. OPS-008 becomes a contract: the engine keeps what repeats and reconciliations need as its
  own state, which a purge never touches; REC-004 drops its list of kept fields.
- C51. The engine's server declares `tools.listChanged` and notifies the streams that asked when
  an instance's tool list changes; tool lists carry `cacheScope: private` (IFC-012).
- C52. Trace context travels only in the `_meta` keys MCP reserves, as OpenTelemetry's MCP
  conventions say; nothing takes caller, restrictions or access from it.
- C58. The machines provider reports Kubernetes's well-known labels `kubernetes.io/hostname`,
  `kubernetes.io/os` and `kubernetes.io/arch`, with GOOS and GOARCH values.

Changes to choices that rulings accepted. Each of these three reverses a choice that a ruling
accepted, as the eight changes that ruling 38 accepted did, and is listed with what it reverses.
The owner can strike any by number, or ask for it to be put as a decision:

- C14. Read `.yml` files and accept `conduit.yml`, as kubectl and kustomize do; a directory with
  two manifest names is rejected (MAN-001, MAN-002, MAN-011, SUB-005). Reverses choices C43 and
  C53 of the core-and-schemas record (ruling 37), where accepting `.yml` was the rejected
  alternative.
- C20. Drop `narrows`: a requirement of the core package is applied by the engine and never sent;
  any other by the target's implementations. CON-030 goes, and three rejections are lifted.
  Reverses CTR-005's `narrows` and CON-030, which ruling 39 accepted.
- C21. `revision.v1` becomes an opaque non-empty string compared only for equality, as an HTTP
  entity tag is (RFC 9110); REV-003 to REV-005 stand unchanged. Reverses choice C15 of the
  core-and-schemas record (ruling 37), where a whole number was chosen and the opaque string was
  the rejected alternative.

## 4. Secrets, reduced

The smallest core for secrets under these principles, as the secrets reviewer found it, with the
decisions above that it depends on. The owner rules on the secrets proposal as a whole, so the
table at the end of this section says what becomes of each of its lines under this review.

What stays in the core contract:

1. Marking a secret: a place that holds a secret holds only a reference to a secret Resource; a
   literal is rejected, and neither a diagnostic nor a record holds it ([SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow),
   [SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). `secret-reference.v1` stands only at a place directly, as `items`, or beside
   `{type: null}` (the proposal's 2b).
2. The capability `secret-reveal.v1` and its one operation, `reveal-secret.v1`, which has no
   inputs. A store learns its locator from the instance's configuration that every request
   carries under decision 1b, as any implementation learns its own. With decision 19, a secret
   Resource is any Resource whose type offers the capability.
3. What every store must do ([SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)): read only the store its locator names, never take a value
   from configuration, fail closed, and never fall back.
4. Who may read: only a Resource or an Agent, through a Conduit that only the root Interface
   opens ([SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), ruling 39), and only by the engine's read: no actor's request for
   `reveal-secret.v1` is dispatched (decision 4b). No list of readers is kept anywhere else.
5. How a value arrives: before each run it asks a provider for, the engine reads through the
   instance's own Conduit and hands the value to the provider, which sets the variable that the
   item binds (decisions 4a, 2c and 25d); if 25c passes, the engine sends it to a remote server
   as a header. The engine asks for the run only once the instance's declared Conduits are
   open. A failed read stops the run, and a request that was to be dispatched to it reports
   unavailable. One run serves one instance (2a). The read belongs to the request that asked
   for the run (4c). The value is fixed for the life of the run.
6. Records: every read is recorded without its value, and a read whose record cannot be kept is
   refused ([SEC-8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-19](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
7. Where the value goes ([SEC-10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), reduced): the engine keeps no copy to stand in for a later
   read, and puts the value nowhere but where point 5 delivers it: not in an inspection result,
   an observation, a diagnostic, a record, any result, or the `_meta` of any request.
8. What an implementation does with it ([SEC-11](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)): an implementation that carries Conduit's keys
   never puts a value it was given into a result, a diagnostic, or the `arguments` or `_meta` of
   a request it sends to the engine. The engine cannot check this, so it binds at the
   Implementation level, as IFC-031 does.

What the guarantee covers (the proposal's item 10, reduced) is stated beside these as a limit,
not a promise. The engine's host and operating-system user, and the place where the chosen
provider keeps a run's environment (2c), are inside the trust domain of every secret the engine
reads. A value in a process's environment is readable by that process, by what it starts, by
root and by the user's other processes that the kernel admits, and serves every caller of that
process alike. An Agent's model that can run commands can read its own credential, and only the
root Interface's choice limits which secret reaches an Agent's process. A copy that a store's
package keeps is outside the engine. A value that an implementation returns or sends is data from
then on. Opening a reading Conduit also trusts the secret's locator, which any registrant writes.

What moves into each store package's contract:

- The form of its locator and the schema that checks it (decision 19).
- Versions or stages (S11), one key of a structured secret (S17) and caching (S12), each in the
  store's own form.
- How the package reaches its store: its own credential (decision 13b), and what else it needs
  from where it runs, such as the home directory for an AWS profile or the user's login session
  for the keychain, which the runtime that runs it must give it; and the limits of unattended
  use, such as a locked keychain.
- Where it must run, and for the file store its encoding, final newline and largest value (now
  the proposal's [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).

The same contract, five stores. Each implements `reveal-secret.v1` and reads its locator from its
instance's configuration; none needs a setup step, memory kept per instance or a change to the
core. Two need a credential of their own, which decision 13b decides how they get:

- **Bitwarden:** `bws secret get <locator.id>`, with `BWS_ACCESS_TOKEN` (decision 13b).
- **1Password:** `op read <locator.reference>`, an `op://` path, with a service account token
  (decision 13b).
- **Mac keychain:** `security find-generic-password -s <service> -a <account> -w`, or
  `SecItemCopyMatching`, under the user's login session, so only a runtime that runs it in that
  session can serve it; it fails closed when locked or denied. Experiment 005 probes this.
- **AWS Secrets Manager:** `GetSecretValue` with the locator's `secretId`, and a version stage
  where one is given, under the AWS profile in the home directory, or with keys bound as
  decision 13b says.
- **The file store of the standard library:** a package that reads `{path}` at each request,
  placed where the file is (decision 13a). If the owner keeps a file store in `engine` instead,
  it is still one implementation of the same operation, but keeps the special cases that 13a
  removes; a store that reads the engine's own variables is 13a's alternative.

What becomes of each line of the secrets proposal:

| Proposal line | Drafted promise | Under this review |
| --- | --- | --- |
| 1, one core type `secret.v1` | — | Replaced: one Type per store, and the core capability `secret-reveal.v1` (decision 19) |
| 2a, the reference schema | [SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 1) |
| A reference names a secret Resource | [SEC-2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept, reworded: the Resource's type offers `secret-reveal.v1` |
| 2b, a branch only beside `{type: null}` | [SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 1). If decision 20 admits `if`, `not` and the other applicators, the rule covers them too |
| 2c, a record keeps only the resolved form | [SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 1) |
| 3a, 3b and 3c, a read when it is needed | [SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped (decision 4a) |
| 4a, 4b and 4d, who may read | [SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 4) |
| 4c, no tool for an Agent | [SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Replaced by the reservation (decision 4b) |
| The output mark `secret` | [SEC-4](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped: one operation is read, and the rules name it; a mark can return with issued credentials (item 15) |
| 4e, no step reads a secret | [SEC-5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-21](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped: no graphs (decision 1) and no actor's read (4b) |
| 5a, values in existing stores | — | Kept, without a store in `engine` (13a) |
| 5b, SOPS files | — | Moved to a store package |
| 5c, no value from configuration | [SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 3) |
| 5d, `engine`'s file and variable on `local` | [SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped (13a) |
| 5e, the locator through a setup step | [SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s observable | Dropped: decision 1b carries it (point 2) |
| 6, rotation and revocation | — | Kept |
| 7a, identity stays DL13 | — | Kept; see decision 26 |
| 7b and 7c, an Agent's credential | — | Kept; 7c as guidance |
| 8a, the read at start | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 5) |
| 8b, the key `environment` | [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept, in the form decision 25d chooses |
| 8c, a failed read | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 5) |
| 8d, one process per instance | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept, as decision 2a for every item |
| 8e, a read for no actor | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Replaced by decision 4c |
| 8f, the engine starts the process | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Replaced by decision 2c |
| 8g, Conduits open before the start | [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept without graphs: a run is asked for once the declared Conduits are open (point 5) |
| 8h, a read during retirement | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept only with 1a's retire hook: a run that serves the hook belongs to the retirement under ACC-039. With no hook, nothing is run in order to retire, and the line is dropped |
| 9, one string | [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept; [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) moves to the file store's package |
| 10, what the guarantee covers | — | Kept, reduced (after point 8) |
| 11, version or stage | — | Moved to each store |
| 12, freshness and caching | [SEC-13](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Split: the engine keeps no copy (point 7); whether a store reads at each request or keeps a copy is the store's |
| 13, records of reads | [SEC-8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-19](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 6) |
| No retrieval of a read | [SEC-9](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped: a read before a run returns no result to any instance, whatever 6c decides |
| Where the engine puts a value | [SEC-10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept, reduced (point 7) |
| An implementation sends no value back | [SEC-11](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept (point 8) |
| 14, no encryption promised | — | Kept |
| 15, stored secrets only | — | Kept; issued credentials come later with their own contract |
| 16, a store's own credential | — | Replaced by decision 13b |
| 17, one key of a structured secret | — | Moved to each store |
| 18, what a started process inherits | [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Dropped (13a); what else a run receives is the provider's (2c, 13b) |
| 19, the value on rotation | [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) | Kept: fixed for the life of a run; a provider's own restart reuses it (2c) |
| 20, names | — | `secret.v1` dropped (decision 19), the mark `secret` dropped (above) and [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) not used; `secret-reveal.v1`, `reveal-secret.v1`, `secret-reference.v1` and the key `environment` of a `command` item (25d) stay |
| 21, a file beside the variable | — | Kept as a later step; a provider may already carry the value as a file (2c) |

## 5. Checked and found aligned

- **Secrets.** A secret's place is marked by `$ref` to a core schema; only the root Interface
  opens a Conduit that reads one; no reader list is kept on a secret; values stay in existing
  stores, behind one operation, as the CSI driver, External Secrets Operator and Dapr do;
  rotation happens in the store; reads are recorded without the value; identity stays DL13; each
  store keeps its own credential and its own syntax for versions.
- **Language.** The Kubernetes envelope, names and references; Go-style addresses and a manifest
  that pins versions and digests; SemVer, and versions in contract names; JSON Schema 2020-12,
  JSON Pointer, and the RFC formats; Kubernetes's rules for defaults; safe archives; instantiation
  by declared inputs and pointer destinations; plain servers keep their own schema dialect.
- **Access.** Every permission is an open Conduit, and the right to open is itself one, like
  Kubernetes `bind` and AWS `iam:PassRole`; delegation only narrows; access is denied by default,
  restrictions only narrow, and each request is decided anew and fails closed; filters are
  enforced at the target, as at an OAuth resource server; onward requests follow RFC 8693
  delegation; ownership permits nothing. The Conduit itself is a relationship in Zanzibar's sense.
- **Operations.** Three outcomes, as Jepsen has them; the four reasons and their order, as the
  Kubernetes request pipeline; the operation identity as an idempotency key; revisions as
  optimistic concurrency; records as audit events; declared state kept apart from observed; a
  forward-only lifecycle like finalizers; explicit replacement; `declare-lost.v1`; model work only
  through a marked operation.
- **Runtime.** MCP as the one wire, with its own version negotiation; handles as opaque strings;
  a tool list for each instance; the protocol's errors and its ban on passing tokens through;
  explicit placement fixed for life; Pools ordered as Compose orders services; first start like
  Kubernetes's bootstrap. Nothing names an operating system, container runtime, database, secret
  store, identity provider or registry.
- **Graphs.** What stays after item 1 follows Terraform and Compose: declared Conduits and
  entries are set up in the order their references and declared dependencies give, each before
  what needs it, and retired in the reverse order; the order is checked when a template is
  registered; no string is templated; and nothing is rolled back.
- **The core package** (added in the final check). Its contracts are Conduit's own: the
  management verbs, access, the reference schemas and the schemas of documents, exported as
  documents as Kubernetes's built-in API groups are, and named as the commands of `kubectl` are.
  `engine` implements them because they are the relationship model and the management of its
  objects, not infrastructure, so P1 does not ask for them to be swappable. The items above take
  out of it: the graph, runtime and component schemas (items 1, 2 and 5), `retrieve-result.v1`
  (6c), `rename.v1` (10), the policy operations (3a), the default-location type, capability and
  control and the location capability and control (8), and the `handle` and `settings` inputs of
  `reconcile.v1` (2a, 15). Its README's counts, its account of retrieval and policies, and its
  list of what it does not settle change with them and with item 24.
- **The implementation-defined list** (added in the final check). DL2 (where a dispatch record
  is written, how attempts are numbered, a late result, and a digest by RFC 8785 and SHA-256),
  DL3, DL4 (an unreachable `access` or `management`, under the one control plane of ruling 40
  point 3f), DL8 and DL10 stay each engine's, as ruling 40 point 2 leaves the storage of records
  and the engine's own state, in standard formats. DL11, what a consumer with only some of its
  Conduits open may do, stays: each request is still decided by its own Conduit. DL5 becomes the
  machines provider's report (8e); DL6 loses `local`'s facts (8); DL12 gives who starts a
  process, how many instances share one and the transport to the runtime provider (2a). DL1,
  DL7, DL9, DL13, DL14 and DL15 are items 1, 21, 23, 26, 22 and 30.

## 6. Considered and not proposed

- **C29, restrictions on the wire as RFC 9396 authorization details.** RFC 9396's details are
  grants that combine by union, while Conduit's restrictions all apply and only narrow; MCP does
  not use RFC 9396, and the `{requirement, arguments}` item is what authors already write.
- **C39, the Pool cascade as a `propagation` input of `retire.v1`.** Kubernetes's Foreground
  marks the owner deleted first, the opposite of LIF-021, and folding the cascade into
  `retire.v1` would make access depend on an input or grant it through `management`.
