# Secrets: a secret type in the core package

Status: ruling 41: in force; amended by 42 (D6, D8, D12, names); amended by 43 (item 4c, Q3); amended by 44 (D14); amended by 58 (M22, M41) (ruled by the owner on 2026-11-06). Holds now: [IN_FORCE.md: 5. Packages, the manifest, contracts, the core package and schemas](../../../IN_FORCE.md#5-packages-the-manifest-contracts-the-core-package-and-schemas), [IN_FORCE.md: 10. Placement, nodes, machines, identity and enrollment](../../../IN_FORCE.md#10-placement-nodes-machines-identity-and-enrollment), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items), [IN_FORCE.md: 12. Secrets](../../../IN_FORCE.md#12-secrets); [PRV-001 to PRV-026](../../../spec/promises/providers.yaml), [SEC-001 to SEC-023](../../../spec/promises/secrets.yaml). This record says why.

This file is the reasoning and the record of the options. See [ruling 41](../ruling-41/README.md).

Was: **proposed, not ruled.** It follows [ruling 38](../ruling-38/README.md), point 1, which set
the direction and asked for the secret type to be explored. Its first draft was put to the owner as
point 1 of [ruling 39](../ruling-39/README.md), and the owner asked for more research first: "I feel
we need more research into how secrets are handled by cloud providers like AWS and in kubernetes
before making a determination". This version was revised after that research, which is the [research
packet on secrets](../../research/SECRETS.md), and again after a review of the revision, which found
five topics the packet left thin and several faults in the proposal; the packet's second round
covers the topics. A review of that second revision found more faults, among them a rule of the
specification that stopped the main way a secret is read, and five more thin topics, which the
packet's third round covers. A review of the third revision found more faults, most of them where
the drafted promises meet the promises on access, on how a request ends, on graphs and on
retirement; this is the fourth revision, and the packet was not changed for it. Ruling 39 accepted
points 2 to 4 as the direction of this revision. Nothing else here is accepted, and nothing here
is part of the specification. Each decision below is a recommendation that waits for the owner.
Until the owner rules, the specification says nothing of secrets, and row GP3 of the
[conversion list](../docs-audit/GAPS.md) stays unconverted.

First written on 2026-11-06, and revised four times the same day, against the specification as it
then stood.
Promise IDs are those of [spec/](../../../spec/README.md) on that day. Terms are those of
[spec/definitions.md](../../../spec/definitions.md); a new term this proposal needs is marked as
proposed where it first appears. Facts about other systems are stated in the research packet, with
the page each comes from; this proposal links the packet's section.

## Decisions for the owner

Rule by number. Where a numbered item has lettered lines, rule each line by its own number, such as
4b or 8d: each can be accepted or rejected apart from the others. Each line states the
recommendation and stands alone; the section it links gives the options, what AWS and Kubernetes
do, and the reason. A line marked "Accepted" repeats what ruling 39 already accepted as direction,
and asks nothing. A line marked "Unchanged" repeats the first draft's recommendation, which ruling
39 did not settle. A line marked "New" asks the owner something that neither ruling 39 nor the
first draft settled, even where it sits under a point that ruling 39 accepted: rejecting it does not
reverse that point. A line marked "Follows" states what an accepted point already requires wherever
the line it names is accepted, and is not ruled apart from that line. A line that the fourth review
changed says so. Items 11 to 19 and 21 were not asked by the first draft.

1. **What a secret is** ([S1](#s1-what-a-secret-is)). A Resource of one Type contract of the core
   package, `secret.v1`, owned by a Pool. The stores behind it are implementations of its one
   operation, in the core package or in ordinary packages. Unchanged.
2. **How a document marks a secret** ([S2](#s2-how-a-document-marks-a-secret)).
   - 2a. Unchanged: a reference schema of the core package, `secret-reference.v1`. Its place holds
     only a reference to a secret Resource; a literal and a bare name are rejected; no diagnostic
     quotes the value.
   - 2b. New, after the fourth review: `secret-reference.v1` may be a branch of `anyOf` or `oneOf`
     only beside `{type: null}`. A schema that offers it beside any other branch is rejected,
     because a literal would satisfy that branch and be kept as data ([SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
   - 2c. New, after the fourth review: a record keeps an argument at a place that
     `secret-reference.v1` governs only in its resolved form, and never a value that failed that
     schema, whatever the type's `records` says ([SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
3. **How the value reaches the implementation that uses it**
   ([S3](#s3-how-the-value-reaches-the-implementation-that-needs-it)).
   - 3a. Unchanged: the implementation asks for it, through its instance's own Conduit to the
     secret, with `reveal-secret.v1`, when it needs it. The engine passes the value on and keeps
     nothing.
   - 3b. New, after the third review: a read of a secret is decided by the reader's own grant. It
     is decided by the Conduit it is made through, the filters on that Conduit, and a policy that
     names the secret among its `targets` and names no `actors`. It carries none of the
     restrictions of the invocation it serves, and no policy that names `actors` applies to it
     ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). The caller's restrictions still bind the invocation that the Resource serves.
     Without this line the access promises as they stand make every read of a secret that `engine`
     serves report unavailable whenever that invocation carries a filter or a policy names its
     caller (ACC-015, ACC-017), because `engine` supports `bound.v1` alone. Changed after the
     fourth review: the line replaces only those carried restrictions and those policies. An
     onward read is still refused once a Conduit of its chain is closed or the consumer of one is
     retired (ACC-034, ACC-039), and IFC-020, under which an onward request carries the served
     invocation's restrictions, takes the exception.
   - 3c. New, after the fourth review: a policy that names a secret among its `targets` applies to
     every read of it, a read that serves the root Interface included; ACC-035's exception for the
     root Interface does not reach a read ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Without this line such a policy stops every
     read but those that serve the root Interface, so a read's outcome depends on its caller. The
     root Interface can still always remove a policy, which is a request of `access`.
4. **Who may read a secret** ([S4](#s4-who-may-read-a-secret)).
   - 4a. Accepted (ruling 39, point 2): only a Resource reads a secret, and only the root Interface
     may open a Conduit that reads one, for the first version.
   - 4b. New: an Agent may also be the consumer of such a Conduit, which the engine alone uses when
     it starts the Agent's process. [SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) say "a Resource or an Agent" for this reason.
     Point 4 applies point 3's mechanism, in which the engine reads "through the instance's own
     Conduit", so the Agent must be the consumer of one. Nothing limits which secret that Conduit
     leads to: the engine cannot tell a provider credential from any other secret, so only the
     root Interface's choice keeps another secret, such as a GitHub token, out of an Agent's
     process, where a model that can run commands can read it (S10). That the secret be the
     Agent's own provider credential is guidance to the root Interface, not a rule. This reaches
     past point 2 for whatever secret is so routed. Changed after the fourth review: the line said
     "but only to its own provider credential", which no promise carried. If the owner rejects
     this line, point 4 needs another route, and S7 says which.
   - 4c. Follows from 7b wherever 4b is accepted: no tool for an operation with a secret output is
     ever presented to an actor, even one that is the consumer of a Conduit that lists it, and a
     request that the actor sends for it anyway reports refused ([SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Only an Agent can be such
     a consumer ([SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), so this is point 4's "never presented to it as a tool" written as a rule.
     Changed after the fourth review: it was marked New, and its refusal, which ACC-003 would not
     give, is now stated.
   - 4d. New, after the third review: no list of readers is written on a secret, on its locator or
     in a package. A grant on the secret, as AWS's resource policies and GCP's allow policies are,
     is rejected as a second way in; a store's own policy binds only the package's identity at the
     store.
   - 4e. New, after the fourth review: no step of a graph names an operation that has a secret
     output ([SEC-21](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). A step with no `conduit` would read a secret through no Conduit, and a step
     of an Agent's graph would be an actor's read; in any other graph [SEC-5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) leaves the output
     nowhere to go.
5. **Where the value is stored** ([S5](#s5-where-the-value-is-stored)).
   - 5a. Unchanged: in an existing store. The engine itself reads a file, and a variable of its own
     environment. The keychain and the secret managers are ordinary packages. Nothing in Conduit
     writes a value.
   - 5b. New: a file encrypted with SOPS, named by its path, is listed among the stores a package
     may implement, decrypted at each read.
   - 5c. New: ciphertext written into a Conduit document, as Sealed Secrets keeps it in Git, is
     never read as a value: no implementation takes a value from configuration ([SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Nothing
     rejects it, since `secret.v1` states no `schema` (CTR-004), so it is stored, inspected and
     revisioned as data; guidance, not a rule, is not to write it. Changed after the fourth
     review: the line said such ciphertext "is rejected for the reason a literal is", which no
     promise did.
   - 5d. New, after the fourth review: `engine` reads its file and its variable only for a secret
     placed on `local`, the location made at first start, and a `file` that is not an absolute
     path fails the read ([SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). PLC-020, under which the engine never opens a value tied to a
     location, takes the exception.
   - 5e. New, after the fourth review: a store's implementation in a package learns its locator as
     every implementation learns its instance's configuration. The store's template's `setup`
     passes it, by a value reference with the source `config`, to a step such as the start of a
     runtime (GRA-010, GRA-020). No request carries it (IFC-006), and nothing new is added for it.
6. **Rotation and revocation** ([S6](#s6-rotation-and-revocation)). A value changes in the store,
   and the next read sees it, with nothing in Conduit set up again. Closing the Conduit stops later
   reads and cannot recall a copy; to revoke a credential, rotate it in the store. Unchanged.
7. **Authentication** ([S7](#s7-authentication)).
   - 7a. Unchanged: how an instance proves to the engine which instance it is stays outside the
     secret type. It stays item DL13 of the implementation-defined list, which leaves to each engine
     what a request presents to show which instance sent it. Outside credentials are what the type
     is for.
   - 7b. Accepted (ruling 39, point 4): an Agent's own provider credential is a secret Resource,
     set in the Agent's process environment at start and never presented as a tool. The first
     draft left it outside Conduit.
   - 7c. New, after the third review: a value set at start serves a key or a token that outlives
     the process. A federated token, which at Anthropic lasts an hour by default (a rule may allow
     up to 24 hours) and at OpenAI at most an hour, needs refreshing in a process that outlives
     it; the first version leaves that to the Agent's own tool, through its helper command or its
     profile, outside every Conduit. Guidance, not a promise: prefer the narrowest form the
     provider offers, such as a token that "can only make model requests". Corrected after the
     fourth review: the line said that a federated token at Anthropic lasts "an hour or less",
     which holds only by default.
8. **Existing servers that take a credential from the environment**
   ([S8](#s8-existing-servers-that-read-credentials-from-the-environment)).
   - 8a. Accepted (ruling 39, points 3 and 4): before the engine starts the process of such a
     server, or an Agent's own process, it reads the secret through the instance's own Conduit and
     sets the variable.
   - 8b. New, the form: an implementation item that states `command` may state `environment`, a
     map from a variable's name to a place in the configuration of the instance it serves that
     `secret-reference.v1` governs directly and the type's schema requires. A name has the form
     POSIX gives the variables of its utilities, lowercase letters allowed. The instance needs
     exactly one open Conduit to the same secret that lists `reveal-secret.v1`, usually a
     declared one; a template writes both from one input; nothing checks the pair at resolution.
     Changed after the fourth review: the line said the instance "must" have a declared Conduit,
     which no promise enforced; the grammar, the required place, and "exactly one" are added.
   - 8c. New, on failure: where any read fails, or a place that `environment` points at holds no
     reference, the process is not started, and a request that was to be dispatched to it is not
     dispatched and reports unavailable, as OPS-005 has every request that is not dispatched
     report a reason. Changed after the fourth review: [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) gave such a request the outcome
     failed, which only a dispatched request has.
   - 8d. New, one process per instance: a process started for an item that states `environment`
     serves one instance. This takes away, for such an item, the choice that IFC-017 gives every
     engine over how many instances share a process.
   - 8e. New, whose read it is: the engine makes the read at the start of the process, whatever
     caused the start, and makes it for no actor, as OBS-007 has the engine stop a runtime for no
     actor. No instance sends it, so it has no sending instance and serves no invocation. So,
     under the access promises as they stand, nothing that ACC-017 carries and no policy that
     names `actors` can reach it: it is decided by the instance's own Conduit, the filters on it
     and each policy that names the secret among its `targets` and no `actors`. No restriction
     tied to a caller binds it, nor the later invocations that use the value. Changed in the
     second revision: the first revision tied the read to the invocation that caused the start,
     which a process started at setup, or an Agent's, does not have. Changed after the fourth
     review: the line no longer rests on item 3b, and stands if 3b is rejected.
   - 8f. New, after the third review: the engine starts the process of an item that states
     `environment` itself. This takes away, for such an item, the choice that IFC-017 gives every
     engine over who starts the process: only the one that starts a process sets its environment.
   - 8g. New, after the fourth review: in a setup, every step that would be dispatched to such an
     item runs after each declared Conduit of the template that reads a secret is open, as if
     `dependsOn` named it ([SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), GRA-024). Otherwise whether a setup fails would depend on the
     order each engine picks among steps (DL1).
   - 8h. New, after the fourth review: a read at the start of a process that the engine starts to
     serve the instance's own retirement belongs to that retirement, so ACC-039 permits it after
     the instance is retired ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Otherwise a plain server whose process is not running when
     it is retired could not be started for its `retire` steps or its runtimes' stops.
9. **What a value is** ([S9](#s9-what-a-value-is)). One string. A new item of the
   implementation-defined list, [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) (proposed), says how the engine's own implementation reads
   its file and its variable; it also states the largest value the engine reads and what it does
   with a final newline. Stands, with that addition.
10. **What the guarantee covers** ([S10](#s10-what-the-guarantee-covers)). The same narrow
    statement, with more limits named: the engine's host and operating-system user are inside the
    trust domain of every secret it reads; a value set in a process environment is readable by that
    process, by what it starts, by root, and by the other processes of the same operating-system
    user that the kernel admits (on Linux those with the same groups, where the process is
    dumpable; on macOS all of them, unless the process's signature restricts it), and it serves
    every caller of that process alike; a restriction tied to a caller binds no read of a secret;
    an Agent's model that can run commands can read its own credential; a cache inside a package is
    outside the engine; a store's own credential kept in the engine's environment reaches every
    process the engine starts. Stands, extended. Extended again after the fourth review: nothing
    limits which secret reaches an Agent's process; the engine keeps out of records and results
    only the copies it makes, and a value that an implementation or an Agent returns or sends is
    data from then on; any code the engine starts can read the engine's own sources with no
    Conduit; opening a reading Conduit also trusts the secret's locator, which any registrant
    writes; any registrant can withhold a variable from every process (item 18); and a reader of
    a retired secret must be replaced.
11. **Version or stage** ([S11](#s11-version-or-stage)). New. A store's locator may name a version
    or a stage, and by default names the current one, which each store's template defines. A
    locator that names a version or a stage that does not exist fails the read, with no fallback.
    A secret whose locator pins a version changes only by a new secret Resource, and each reader
    that names the old one is then replaced (added after the fourth review, which found it
    unstated). `secret.v1` fixes no form; each store's template does.
12. **Freshness and caching** ([S12](#s12-freshness-and-caching)). New. The engine keeps no copy of
    a value, and its own implementation reads the file or the variable at each request. An
    implementation in a package may serve a read from a copy it keeps, as AWS's own clients do;
    that is its own act. The first draft required every store, those of packages included, to be
    read at each request; this keeps that rule for the engine's own file and variable only.
13. **Records of reads** ([S13](#s13-records-of-reads)). New. Every read is recorded as any request
    is, with the caller and the sending instance where there are ones, the Conduit, the secret,
    the time and how it ended, and never the value. A read at process start has neither; its
    record names the instance whose process is started. A read whose record cannot be kept is not
    dispatched. No keyed hash of the value in the first version.
14. **Encryption of what the engine keeps** ([S14](#s14-encryption-of-what-the-engine-keeps)). New.
    The engine keeps no value, so the specification promises no encryption for secrecy. A secret's
    configuration says where the value is and holds nothing secret.
15. **Issued credentials and stored secrets** ([S15](#s15-issued-credentials-and-stored-secrets)).
    New. The first version has stored secrets only. `secret.v1` is described as the fallback for a
    credential that an identity cannot replace, and the generic output mark leaves room for an
    operation that issues short-lived credentials later.
16. **A store's own credential** ([S16](#s16-a-stores-own-credential)). New, and changed after the
    review. An implementation in a package reaches its store with the credentials its store's
    client finds by default in the environment the engine gives its process: on a developer's
    machine, the client's own files under the home directory and the variables that point to them,
    such as `AWS_PROFILE`. That is outside every Conduit, and stated as such. A stored key that the
    engine reads and sets for the package is left out of the first version: as drafted it could not
    be declared, and it would cost one process for each secret Resource. Never a secret that
    another package's implementation reads.
17. **One key of a structured secret** ([S17](#s17-one-key-of-a-structured-secret)). New. A store's
    locator may name one key of a JSON secret, in the store's own form, so that one stored secret
    gives several secret Resources, each one string. The engine's own file and variable have no
    selector. AWS already uses a colon after a name for a version or a label, so a selector syntax
    of Conduit's own would collide with it.
18. **What a started process inherits** ([S18](#s18-what-a-started-process-inherits)). New. A
    process that the engine starts is given no variable that the configuration of a secret
    Resource that is not retired and that `engine` serves names as its store, apart from those set
    for it under `environment`, and never a value in its arguments. The rest of the engine's
    environment, `HOME`, `PATH` and the pointers that a store's client looks for among it, is left
    to each engine; item 16 needs it passed on. Changed after the fourth review: the line now says
    which secret Resources count, and S10 names the limit that any actor that may register a
    Resource can so withhold a variable, `PATH` among them, from every process.
19. **A process environment on rotation** ([S19](#s19-a-process-environment-on-rotation)). New. A
    value set at start is fixed for the life of the process. The engine neither watches the store
    nor restarts a process because a value changed; a new value reaches the next process the engine
    starts for the instance.
20. **Names** ([S20](#s20-names)). The names `secret.v1`, `secret-reveal.v1`, `reveal-secret.v1`
    and `secret-reference.v1`, the output mark `secret`, the key `environment` of an implementation
    item, and the number [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) for the new item of the implementation-defined list, become final.
    Point 1 of ruling 39 put the names; they are unchanged.
21. **A file beside the variable** ([S21](#s21-a-file-beside-the-variable)). New, after the review.
    In the first version the engine sets a value at process start only as a variable. A file in a
    directory made for one process, its path in a variable, as systemd does and as servers with a
    `_FILE` variable read, is recorded as the shape to adopt later. The basis: every server checked
    that takes one token accepts the variable, the pages read for the MCP runners show delivery by
    variable, and a file would not keep the value from the user's other processes: on Linux a
    file only its owner may read is open to all of them, and on macOS the environment already is.

## The problem

Conduit's purpose names the case directly. "A login, checkout or memory is owned by a Pool and used
by agents from any vendor, each under its own limits, with the credential never handed over"
([purpose](../../AUDIT.md#purpose)). The older design stated the rule that has to become a promise:
"Secrets never appear in configuration, records or events, and a credential is never handed to a
caller" (row GP3 of the [conversion list](../docs-audit/GAPS.md)).

The specification cannot state that rule today, for three reasons.

1. Nothing marks a value as secret. A value inside `config` or `arguments` is either a reference,
   where a reference schema governs its place, or data (OBJ-021). Data is stored as written, shown
   by inspection (LIF-013), sent to implementations, read by graphs (GRA-020), and kept in records
   where the type's `records` says so (REC-002).
2. Nothing says how a credential reaches the implementation that needs it, such as the
   implementation of a Resource that calls an outside service with a token.
3. Nothing stops an operation from returning a credential to the actor that asked for it.

## What the owner has already ruled

Ruling 38, point 1, put two options:

- **A.** The schema keyword `writeOnly: true`, with the engine keeping a value so marked out of
  inspection results, records, diagnostics and events.
- **B.** A marked property may hold only a reference to a secret Resource, and a literal is
  rejected. This needs a secret type in the core package.

The coordinator recommended A. The owner ruled B: "B I think we need to explore a secret type as
this will be extremely important for many areas including access and authentication."

Ruling 39 put the core of the first draft as point 1, and three narrower points beside it. The
owner asked for research on point 1, and accepted the other three as the direction for this
revision:

- Point 2: "Who may read a secret: only a Resource, and only the root Interface may open a Conduit
  that reads one, for the first version." Ruled: "Accepted".
- Point 3: "Existing servers that take a credential from an environment variable: the engine reads
  the secret through the instance's own Conduit and sets the variable when it starts the process."
  Ruled: "Accepted".
- Point 4: "An Agent's own provider credential: the same, for an Agent's own process; the value is
  set in its environment at start and never presented to it as a tool." Ruled: "Accepted".

So these are settled and are not reopened here:

- Ruling 38 chose B: a marked place holds only a reference to a secret Resource, and a literal
  there is rejected. B needs a secret type. The owner's words were "explore a secret type", one
  the owner expects to matter to "many areas including access and authentication".
- Points 2 to 4 of ruling 39, as direction. Where the research argues against one, this proposal
  says so and does not overturn it.

The secret type itself is not settled. Ruling 39's record says "1. The secret type is not decided",
and the owner rules on it from this revision. Everything else is open, and is set out below. The
owner's standing direction applies to each decision: adopt an existing paradigm for anything that
is not what makes Conduit different, and invent nothing where a standard exists
([existing standards first](../../../spec/README.md#existing-standards-first)). The first user is a
single developer on one machine; teams come later, and no rule may assume one owner. Enforcement
beyond what passes through the engine is a future, optional part.

## What existing systems do

The [research packet](../../research/SECRETS.md) covers AWS Secrets Manager and Parameter Store,
how AWS delivers credentials to workloads, the Kubernetes Secret, the Kubernetes ecosystem (the
Secrets Store CSI driver, the External Secrets Operator, Sealed Secrets, SOPS, Argo CD and Vault on
Kubernetes), GCP Secret Manager, Azure Key Vault, HashiCorp Vault, and the configuration tools and
agent tooling a developer uses beside them (Terraform, Pulumi, Docker Compose and Swarm, Helm,
1Password, Doppler, direnv, the Model Context Protocol and its registry, Claude Code, the Anthropic
SDK and Codex). Its second round adds who may give a workload a read, AWS and Kubernetes on a
developer's machine, Parameter Store in depth, Sealed Secrets and SOPS in depth, and how existing
MCP servers and their runners take a credential. Its third round adds a list of readers on the
secret, what Kubernetes gives every container, an Agent's own model credential at the providers,
processes of the same user on Linux and macOS, and delivery to an EC2 instance. Its twenty-five
[patterns across the systems](../../research/SECRETS.md#patterns-across-the-systems) are, in short:

1. A secret is referred to by name and, where the store has them, a version or a stage; the current
   version is the default.
2. One reference form fronts many stores, and a pass-through applies the store's own policy.
3. Where it can, a platform replaces a stored secret with an identity it issues credentials for.
4. Reading the value is a right of its own, apart from seeing metadata or managing the store, and
   the right to grant it is kept narrow.
5. Reading at use is ranked above copying; a copy brings the gaps of where it is copied to.
6. A value in an environment does not change without a restart, and it leaks to logs, debug
   endpoints and processes of the same user; injection at start is still the path for software that
   can read nothing else.
7. A workload whose secret cannot be read does not start.
8. Reads are audited with who, what and when, never the value.
9. Base64 is not encryption, and a display mark is not secrecy.
10. Whoever can run code with access can read the value.
11. Caches trade freshness for cost.
12. Revocation stops later reads; only an issuer recalls a copy.
13. Where a secret may flow is restricted.
14. A platform that keeps values must manage keys.
15. For AI agents, the value is kept out of the model's context, and that is called best-effort.
16. On a developer's machine, credentials are found by convention: a chain, a pointer variable, a
    file under the home directory, a session a person signs in to.
17. A command can be a credential source, in each client's own format, and whether it may run is
    decided apart from the file that names it.
18. The right to arrange a read is its own right, scoped by naming the grant, and one grants only
    what one holds unless given an explicit right to do more.
19. Some servers read a file beside the variable, and a supervisor can give each process a
    directory of its own; the pages read for the MCP runners show delivery by variable.
20. Ciphertext kept beside configuration moves the risk to the decryption key.
21. A list of readers on the secret is a second place to grant, and is guarded as one; Kubernetes
    and Azure keep who may read outside the secret.
22. What a platform gives every workload by default, its hardening guidance withholds.
23. A short-lived model credential needs refreshing, not only setting.
24. The operating-system user is the line both kernels draw, for the environment and for a file.
25. Configuration names the store and the identity, never the value.

These rows, from the first draft's survey, cover systems the packet does not. Each was read from
the system's own documentation on 2026-11-06. The column "Lesson for Conduit" is this proposal's
reading, not the source's.

| System | How a secret is referred to | How the value reaches the user of it | What is kept out, and the known gaps | Rotation and revocation | Lesson for Conduit | Source |
| --- | --- | --- | --- | --- | --- | --- |
| SPIFFE and SPIRE | No shared secret: a workload is identified by a SPIFFE ID and proves it with an SVID (X.509 or JWT) | The Workload API, which needs no "authentication token when calling the API"; the endpoint identifies the caller out of band | Keys are "short lived, rotated frequently and automatically" | Updates are streamed to the workload | Identity of a workload is best not a stored secret at all. | [Overview](https://spiffe.io/docs/latest/spiffe-about/overview/), [concepts](https://spiffe.io/docs/latest/spiffe-about/spiffe-concepts/), [Workload API](https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE_Workload_API.md) |
| OAuth 2.0 Token Exchange, RFC 8693 | A `subject_token` for whom the request is made, an `actor_token` for who acts | A new token from a token service, for an `audience` | Delegation ("A is an agent for B") is recorded in the `act` claim | Tokens are short-lived and issued per use | The standard way to act for someone at an outside service without holding their credential. Later, not v0. | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) |
| Model Context Protocol, revision 2026-07-28 | Authorization is optional and defined for HTTP: an OAuth 2.1 bearer token in the `Authorization` header of every request. Over stdio, implementations "SHOULD NOT follow this specification, and instead retrieve credentials from the environment." | The client presents the token; the server validates it was issued for itself | "MCP servers MUST NOT accept or transit any other tokens." "The MCP server MUST NOT pass through the token it received from the MCP client." Token passthrough is "explicitly forbidden". | Authorization servers "SHOULD issue short-lived access tokens" | The engine is an MCP server to its consumers and must not pass their credentials on, which IFC-030 already states. An existing stdio server takes its credential from its environment. | [Authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization), [security considerations](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations), [best practices](https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices) |
| JSON Schema 2020-12, `writeOnly` | An annotation: "the value is never present when the instance is retrieved from the owning authority" | — | An annotation only; it is in section 9, the annotation keywords, and asserts nothing | — | It says a value is not read back. It cannot reject a literal, which is what ruling 38 asks. | [Validation, section 9.4](https://json-schema.org/draft/2020-12/json-schema-validation#section-9.4) |
| OpenAPI 3.1, `format: password` | `string` with `format: password`: "A hint to UIs to obscure input." | — | A hint only | — | The same: a display hint, not a rule. | [Data types](https://spec.openapis.org/oas/v3.1.0#data-types) |
| GitHub Actions secrets | `${{ secrets.NAME }}` | Into the job's environment | Values are "redacted from logs", but with no promise for transformed values; "secrets are not passed to the runner when a workflow is triggered from a forked repository" | — | Masking printed values is a useful extra and never a guarantee. | [Using secrets](https://docs.github.com/en/actions/security-for-github-actions/security-guides/using-secrets-in-github-actions) |

## The proposal in one page

This is what the recommendations below add up to, if all of them were accepted.

- The core package exports a Type contract, proposed name `secret.v1`, of the family Resource. A
  secret is a Resource of that type, owned by a Pool like any Resource. Its configuration says
  where the value is kept, never the value. It is the fallback for a credential that no identity
  can replace (S15).
- The core package exports a reference schema, proposed name `secret-reference.v1`. A place that it
  governs holds a reference to a Resource of the type `secret.v1`, and nothing else: a literal, and
  even a bare name, is rejected. This is the mark that ruling 38 asked for.
- The value is read by one operation, proposed name `reveal-secret.v1`, in one capability, proposed
  name `secret-reveal.v1`, which `secret.v1` offers. Its one output is marked `secret: true`, a new
  mark of an Operation contract's output, beside `handle: true`.
- The right to read a secret is an open Conduit to it that lists `reveal-secret.v1`, as every
  permission is (ACC-001, ACC-003). Only the root Interface opens such a Conduit, and only for a
  Resource or an Agent. A Resource's implementation reads the value by an onward request while it
  serves an invocation. An Agent's Conduit is used only by the engine when it starts the Agent's
  process; no actor is ever presented a tool that reads a secret, and no step of a graph reads
  one.
- Every read of a secret is decided by the reader's own grant: its Conduit, the filters on it, and
  a policy that names the secret and no actor, whoever the caller. The restrictions of the caller
  it serves bind the invocation the Resource serves, and not the read.
- The engine never puts a secret output in a record, an inspection result, an observation, a
  diagnostic, a graph, or a retained result. It delivers it only to the instance that sent the
  request, or, at the start of a process, into that process's environment. An implementation that
  receives one never passes it on to an actor or into the engine again; what it does pass on is
  data from then on (S10).
- Every read is recorded with who read which secret for whom, and never the value. A read whose
  record cannot be kept is not made.
- The value stays in an existing store. The engine's own implementation of `reveal-secret.v1` reads
  a file or an environment variable, as Docker Compose does, for a secret placed on `local`, the
  location made at first start. The operating system's keychain, the secret managers and a file
  encrypted with SOPS are ordinary packages, which learn their locator at setup, as any
  implementation learns its configuration; a locator may name a version or a stage, and one key
  of a JSON secret, in the store's own form. No operation writes a secret. A package reaches its
  own store with the credentials that the store's client finds by default, outside every Conduit.
- The engine keeps no copy of a value, and reads its own sources at each request, so a value
  changed in the store is seen at the next read, with nothing set up again and no revision
  advanced. Closing the Conduit stops later reads.
- An existing server that takes its credential from its environment, and an Agent's own process,
  get the variable from the engine when it starts the process, read through the instance's one
  open Conduit to the secret by the engine for no actor. The process serves that one instance, and
  a setup starts it only after that Conduit is open. The value is fixed for
  the life of that process and serves every caller of it alike, and a process the engine starts is
  never given the engine's own secret variables.
- How an instance proves who it is to the engine stays outside the secret type (DL13).

A short illustration, not checked by any tool. A secret in a Pool's entry; the Type contract of a
Resource that uses it, whose configuration has a place for it; a Resource template that reads it;
and an instance that names it:

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: github-token
spec:
  type: github.com/shrav-k/conduit/core#secret.v1
  config:
    environment: GITHUB_TOKEN       # where the value is kept, read by the engine's implementation
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Conduit
metadata:
  name: reveal
spec:
  capability: github.com/shrav-k/conduit/core#secret-reveal.v1
  operations: [github.com/shrav-k/conduit/core#reveal-secret.v1]
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Type
metadata:
  name: issues.v1
spec:
  family: Resource
  schema:
    type: object
    properties:
      token: {$ref: github.com/shrav-k/conduit/core#secret-reference.v1}   # SEC-15 checks this
    required: [token]
    additionalProperties: false
  offers: ['#issue-reading.v1']
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: issue-tracker
spec:
  type: "#issues.v1"
  inputs:
    token:
      schema: {$ref: github.com/shrav-k/conduit/core#secret-reference.v1}
      to: [/config/token, /conduits/token/target]   # one input, both places
  conduits:
    token:
      from: reveal                  # opened at setup, only where the root Interface sets it up
---
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
from: issue-tracker
owner: tools
arguments:
  token: {kind: Pool, name: credentials, entry: github}
  # token: ghp_0123...   rejected: a literal where a secret is marked
  # token: hunter        rejected: a bare name is not accepted in a secret's place
```

The one input is written to two places. `/config/token` is a place that `secret-reference.v1`
governs in the type's schema, so it holds the reference and is inspected as one; an implementation
item's `environment` points at it ([SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). `/conduits/token/target` makes the declared Conduit
lead to the same secret, and that Conduit is what every read goes through ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), S3). An
implementation that asks needs only the second; a plain server, which cannot ask, needs both.
Nothing checks the pairing when the documents are resolved: where the instance has no open Conduit
to the secret that its configuration names, or more than one, the read at process start reports
refused and the process is not started ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Within a setup, the order is fixed: a step that
would be dispatched to the plain server's item, such as the start of a runtime, depends on the
declared Conduit `token` as if `dependsOn` named it, so the engine never starts the process
before that Conduit is open ([SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), GRA-024).

An Agent that is the consumer of a Conduit to `issue-tracker` calls its operations. Where the
Resource's implementation asks (S3), it reveals the token through its own Conduit while it serves
the Agent's invocation, calls the outside service, and returns the issues. Where it is a plain
server (S8), the engine reads the same secret through the same Conduit when it starts the process,
and sets the variable that the item's `environment` names. Either way the Agent never holds the
token.

Two more sketches, also unchecked. A secret kept in AWS Secrets Manager and read by a hypothetical
package, which names one key of a JSON secret and leaves the version to its default (S11, S17).
No request carries the secret's configuration (IFC-006), so the package learns its locator as any
implementation learns its instance's configuration: the template's `setup` reads it by value
references with the source `config`, which GRA-010 allows only there, and passes it to the start of
a runtime. The package keeps it for that instance, whose ID each later request carries (S5):

```yaml
apiVersion: github.com/shrav-k/conduit/v0
kind: Resource
metadata:
  name: db-password
spec:
  type: github.com/shrav-k/conduit/core#secret.v1
  implementation:
    github.com/shrav-k/conduit/core#reveal-secret.v1: example.com/conduit-aws#secrets-manager
    example.com/conduit-aws#client.start.v1: example.com/conduit-aws#secrets-manager
    example.com/conduit-aws#client.stop.v1: example.com/conduit-aws#secrets-manager
  config:
    secretId: arn:aws:secretsmanager:eu-west-1:123456789012:secret:prod/db-a1b2c3
    jsonKey: password               # one key of a JSON secret
    # no versionStage: the store's default, AWSCURRENT
  runtimes:
    client:
      start: example.com/conduit-aws#client.start.v1
      stop: example.com/conduit-aws#client.stop.v1
  setup:
    steps:
      locate:                       # how the package learns its locator (5e)
        runtime: client
        arguments:
          secretId: {source: config, pointer: /secretId}
          jsonKey: {source: config, pointer: /jsonKey}
```

And the manifest item of an existing stdio server that takes its token from its environment (S8),
selected for `issue-tracker` of the first sketch. The pointer leads into the configuration of the
instance it serves, so `/token` is the place that the template writes as `/config/token`, and that
`secret-reference.v1` governs in `issues.v1`:

```yaml
implementations:
  issues-server:
    implements: ["#list-issues.v1"]
    command: [github-mcp-server, stdio]
    plain: true
    tools: {"#list-issues.v1": list_issues}
    environment:
      GITHUB_PERSONAL_ACCESS_TOKEN: /token
```

## Decisions

Each decision gives the options, what each costs, what AWS and Kubernetes do, and a recommendation
with its reason. "Stands" or "Changes" says how the recommendation compares with the first draft.
"Fits" says how it sits with the promises as they stand.

### S1. What a secret is

**The question.** What kind of thing a secret is in Conduit, and where its definition lives.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | A Resource of one secret type, owned by a Pool. Access to it is a Conduit like any other. | A new Type contract, capability and operation in the core package. |
| B | A sixth family, `Secret`. | A new family touches the five-family rule (OBJ-020), every reference schema, inspection, lifecycle and access. Nothing that a secret needs is missing from a Resource. |
| C | A value that `access` or the engine stores outside any object, as `access` stores policies (ACC-031). | No ownership, no placement, no lifecycle, no Conduit: access to it would need a second mechanism, against "every permission is an open Conduit". |
| D | Any Type contract in any package that carries a new core mark, such as `secret: true` on the type. | Many secret types. The engine must trust every package's claim, and the rules of S2 to S4 must be stated against a mark rather than one type. |

**Sub-question: where the type lives.**

| Option | What it is | What it costs |
| --- | --- | --- |
| **A1** | The type, its capability and its operation in the core package; the stores behind it are implementations, in the core package (S5) or in ordinary packages. | Three exports added to the core package. |
| A2 | Only the reference schema and the output mark in the core; the type in an ordinary package. | The reference schema must check a Resource's type, and the engine can only check a type it knows. A schema in the core that names a type outside it would make the core depend on a package. |

**What AWS and Kubernetes do.** AWS keeps one kind of secret and puts several reference forms in
front of it: Parameter Store's pass-through, CloudFormation's dynamic references, ECS's `valueFrom`
and the agent wrapper `asm-exec`. The pass-through "doesn't retain data or metadata about secrets"
and honors the secret's own access policies
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). Kubernetes has one
Secret kind for every secret, kept apart from ordinary configuration by kind (a Secret against a
ConfigMap), with a `type` that names a convention for key names; the CSI driver and the External
Secrets Operator put one reference kind in front of many outside stores
([packet](../../research/SECRETS.md#the-kubernetes-secret),
[ecosystem](../../research/SECRETS.md#the-kubernetes-ecosystem)). GCP, Azure and Vault each keep a
secret as a named object with versions, inside something that groups it: a project, a vault, a
mount ([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). That grouping is what a
Pool's ownership gives.

**Recommendation: A with A1. Stands.** A secret is a Resource of the core type `secret.v1`, owned
by a Pool. One type with many stores behind it is what AWS, Kubernetes and Compose each do. A
Resource already has everything a secret needs: an owner, so a Pool groups a developer's
credentials and a bound that names the Pool covers them (ACC-029); a placement, which matters
because where the value is kept is tied to a location (PLC-020); a lifecycle; inspection; and
Conduits for access, which is what the owner asked the type to serve. The type must be in the core
package because the language checks it: OBJ-021 makes a value a reference only where one of the
core package's reference schemas governs it, and S2 needs that schema to name a type the engine
knows. The research adds one thing, taken up in S15: AWS's own order is "remove, replace, and
rotate", so a stored secret is the fallback for a credential that an identity cannot replace.

### S2. How a document marks a secret

**The question.** How a schema says "this place holds a secret", what the language then rejects,
and how that fits the [schema profile](../../../spec/schema-profile.md) and the reference schemas
that exist.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | `writeOnly: true` | Ruled out by ruling 38. It is an annotation and rejects nothing. |
| **B** | A new reference schema of the core package, `secret-reference.v1`, used by `$ref` like the others. | One more Schema contract; one rule that no schema can express, checked by the engine. |
| C | A new format, such as `format: secret-reference` | A format in the profile is a grammar that is asserted (SCH-004). A reference is a map, not a string, and has no grammar. This would invent a format. |
| D | A new keyword, such as `secret: true`, inside a schema | A keyword outside JSON Schema 2020-12. SCH-002 rejects any keyword outside the profile, and the profile invents none. |

**What `secret-reference.v1` would accept.** The forms of a reference to a Resource that
`resource-reference.v1` accepts, less one:

- `{kind: Resource, name: <name>}`;
- the reference to a Pool's entry, `{kind: Pool, name: <pool>, entry: <entry>}` (POOL-014);
- the entry reference `{entry: <name>}`, inside a Pool's template (POOL-005);
- the resolved form `{kind: Resource, id: <ID>}`, which only the engine writes (OBJ-009).

It would **not** accept a bare name, which `resource-reference.v1` accepts. A bare name is a
string. A secret that happens to be written in lowercase letters, digits and hyphens, such as a
weak password, would pass the schema as a name; resolving would then find no object of that name
and reject the document with a diagnostic that names the reference (OBJ-009), which would print the
secret. Without the bare name, every string in that place fails the schema, and a literal is
rejected by the schema itself.

**The rule no schema can express.** The Resource that the reference resolves to has the type
`secret.v1`. A reference to any other Resource makes the document rejected, as OBJ-007 rejects a
reference to a template where an instance is expected.

**A second rule, on diagnostics.** A diagnostic about a value in a place that `secret-reference.v1`
governs names the place and the rule it breaks, and never quotes the value. A JSON Schema validator
commonly prints the failing instance; this rule forbids that here. The result of `register.v1` is
returned to its caller and its content is retained for retrieval (OPS-016), so a quoted literal
would be kept.

**Where the mark may be used.** Wherever a schema is written: a type's `schema` for `config`, a
template's input, an Operation contract's input, and a Requirement contract's input. In each case
it holds a reference, so the existing rules on references apply unchanged: it is resolved when the
instance is made (OBJ-009), never redirected (OBJ-010), stored and shown in the resolved form
(OBJ-021), and a request that an actor makes on its own behalf names it by reference, never by ID
(LIF-031).

**Beside another branch (item 2b).** New after the fourth review. OBJ-021 makes a reference
schema govern a place where it is "a branch of `anyOf` or `oneOf` that the value satisfies". So a
type that writes `anyOf: [{$ref: ...secret-reference.v1}, {type: string}]` accepts `ghp_0123` as
data, by the second branch: the literal is stored, inspected and recorded, although the author
marked the place. OBJ-021 names one pairing that keeps a reference a reference, a reference schema
beside the type null, under which "a name is a reference and null is not". The recommendation
allows that one and rejects the schema in any other case ([SEC-1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)): a place that may hold a secret
holds a reference or nothing.

**In a record (item 2c).** New after the fourth review. In an Operation contract's input, a literal
fails the schema and the request reports invalid; but REC-002 keeps "the inputs where `inputs` is
true" in the target type's `records`, and [SEC-3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) covers only a diagnostic. So a record keeps an
argument at such a place only in its resolved form, and never a value that failed the schema
([SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).

**What AWS and Kubernetes do.** Both keep the value out of the configuration that names it, and
both show why a marked literal is not enough. CloudFormation "never stores the actual secure string
value. Instead, it only stores the literal dynamic reference", and returns only the reference from
its APIs; its display mark, `NoEcho`, only masks describe calls, and AWS recommends a reference to
a store instead. ECS recommends against credentials in the plain `environment` parameter
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)). On EC2, AWS says "you should
not store sensitive data, such as passwords or long-lived encryption keys, as user data", which any
software on the instance may read; a launch template's versions cannot be changed, only deleted,
and the right to read them cannot be limited to one template; and CloudFormation recommends dynamic
references over embedding a value in a template
([packet](../../research/SECRETS.md#delivering-to-an-ec2-instance)). A Conduit document is kept in
Git and read by many, which is the same exposure. A Kubernetes manifest may
carry the value in base64, which "provides no additional confidentiality over plain text", and the
write-only field `stringData` is merged into `data` and returned there
([packet](../../research/SECRETS.md#the-kubernetes-secret)). That is ruling 38's finding about
`writeOnly`, seen in practice. Two other systems bear on the details. Azure App Service, when a
reference cannot be resolved, puts "the reference string" in the setting, which is the failure a
schema that rejects literals avoids. Claude Code quotes a `${VAR}` reference unexpanded in its
warnings, "so it never shows a resolved value such as an API key", which is the rule on diagnostics
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)).

**Recommendation: B, without the bare name, with the rule on diagnostics. Stands.** It uses only
what exists: `$ref` to a Schema contract of the core package, the mechanism by which every other
reference is marked. The profile does not change. It is the only option that rejects a literal,
which the ruling requires. Because a marked place only ever holds a reference, configuration,
records, inspection, observations and graphs hold the reference and never the value, with no new
rule for any of them. The MCP registry's `isSecret` is a flag on the inputs a server's manifest
declares, not a schema keyword, so it does not bring back option D; S8 says how it relates.

**What it cannot do.** It marks only what an author marks. A token written into a place that a
plain `{type: string}` governs is data, and no rule can tell it from any other string. Parameter
Store leaves the same choice to the author of a parameter's type
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). See
[S10](#s10-what-the-guarantee-covers).

### S3. How the value reaches the implementation that needs it

**The question.** How the value gets to the implementation that uses it, without appearing in
configuration, records, events, inspection results, diagnostics or operation results; and what the
engine itself may see.

How an implementation learns anything today: it receives the `arguments` of each request and
Conduit's keys in `_meta` (IFC-006). It is not sent the instance's configuration; a graph reads
configuration with a value reference (GRA-020) and passes it as a step's arguments. An instance
that is not an actor makes a request only to serve an invocation (IFC-021), and an onward request
is permitted only under the instance's own open Conduit (ACC-016).

| Option | What it is | What it costs |
| --- | --- | --- |
| A | **Substitution at dispatch.** Where a request's arguments hold the resolved form of a secret reference, the engine puts the value in its place before it sends the request. | The value is in the arguments of ordinary requests, so every rule on arguments needs an exception: records of inputs (REC-002), the digest of contents that a purge keeps (REC-004) and that a repeat is compared by (OPS-009), the arguments of restrictions (IFC-018). It breaks OBJ-021, under which the implementation receives the resolved form. Who may read is decided by who may name the reference, not by a Conduit. |
| B | **A key in `_meta`.** Every request to an instance carries the values of the secrets its configuration names, in a new key. | The same exposure as A on every request, whether or not the operation needs the value. Who may read is again not a Conduit. |
| **C** | **The implementation asks.** The instance has a declared Conduit to the secret Resource that lists `reveal-secret.v1`. Its implementation, while serving an invocation, makes an onward request for it, and receives the value as the operation's output. | Each read is a request, with an access decision and a record. The implementation must be written to ask. An existing server that is not written for this specification cannot ask (see S8). |
| D | **A file or an environment variable at process start.** As Compose's `/run/secrets` and Kubernetes's volumes and environment. | Only useful where the implementation cannot ask; the variable is taken up in S8, which ruling 39 accepted for that case, and the file in S21. |
| E | **A handle.** `reveal-secret.v1` returns an opaque handle, and the implementation exchanges it at the store, as Vault's response wrapping does. | The implementation must then reach the store itself, with a credential of its own, which only moves the problem. |

**What AWS and Kubernetes do.** Both offer injection by the platform and reading by the workload,
and both rank an environment variable lowest. ECS recommends against environment variables,
"Because there's an elevated risk of data leakage with environment variables", and points instead
to a sidecar that reads Secrets Manager and writes the secrets to a shared volume, which the
application then reads as files, or to the application reading an encrypted S3 object under its
own role. That ranks a file, option D's other form, above a variable; it is not evidence for C.
The evidence for C is elsewhere. AWS's own provider for the CSI driver says "Security conscious
applications should use the native AWS APIs to fetch secrets"
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)). Among the alternatives to its
Secret, Kubernetes names "a service that Pods access over HTTPS, that reveals a Secret if the
client correctly authenticates", which is option C
([packet](../../research/SECRETS.md#the-kubernetes-secret)). AWS's provider reads under the Pod's
own role, so reading through the instance's own Conduit is an adopted model, not an invention.
Google says to read the API "whenever possible", and Argo CD "strongly caution[s]" against
resolving secrets into rendered content, which is what option A does
([packet](../../research/SECRETS.md#the-kubernetes-ecosystem)). On EC2, the Well-Architected
pillar says to "integrate Secrets Manager into your code to retrieve third-party secrets at
runtime", under the instance's role; AWS says its local provider's trust domain there is that of
the instance's role, and that security-conscious applications "should consider using the
language-specific AWS SDKs or caching solutions" instead
([packet](../../research/SECRETS.md#delivering-to-an-ec2-instance)).

**What the engine may see.** Under C the value passes through the engine once per read: the engine
is the server the implementation calls (IFC-012) and the client of the store's implementation. The
engine already sees every argument and result, and the first user runs the engine on their own
machine. The engine may hold the value in memory to deliver it, and keeps nothing of it: no record,
no retained content, no diagnostic, no observation and no inspection result holds it (S4). This is
the norm, not a weakness peculiar to Conduit: the CSI driver, the External Secrets Operator, which
can "read and write to all secrets across all namespaces", and AWS's local provider all see each
value they deliver. AWS names its provider's trust domain as "usually the entire host"
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). The proposal states
the same of the engine: the engine's host, and whoever can run code as the engine's
operating-system user, are inside the trust domain of every secret the engine reads (S10).

The Model Context Protocol aims for a credential collected by URL-mode elicitation to "never pass
through the LLM context, MCP client or any intermediate MCP servers"
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). Under C the engine
is such an intermediate. The proposal states that divergence; it is acceptable for one developer on
one machine. Delivery that the engine cannot read, by wrapping the value for its reader alone, stays
left out of the first version.

**Recommendation: C. Stands.** The value reaches only the implementation that asks for it, at the
moment it asks, through a Conduit that can be inspected and closed. That makes the right to read a
secret "an open Conduit like any other access", which is what the owner asked the type to serve.
Every onward read is recorded with its caller and its sending instance (S13), as CloudTrail records
each `GetSecretValue`, without the value; AWS's own local cache loses that record, and the engine
does not. Reading the store at each request gives rotation for free (S6). The existing machinery
does the rest: the declared Conduit is opened at setup (ACC-005, CON-004), and it is closed when
its consumer's shutdown is confirmed (CON-008). Which restrictions decide the read is the
sub-question below.

**Sub-question: which restrictions bind a read.** New after the third review. The second revision
said that the restrictions of the served invocation apply to a Resource's read, "so a policy can
stop a Resource from reading a secret on behalf of a particular Agent". Under the specification as
it stands that breaks the read. ACC-017 has an onward request carry the filters of the Conduit that
the served invocation came through, and the policies that apply to its caller and its target; and
ACC-031 has a policy that names `actors` apply to every request of which one of them is the
caller, an onward request included. ACC-015 then has a request report unavailable wherever the
implementation it would be dispatched to does not support the requirement of such a policy or
carried restriction, because "a restriction is never dropped because the implementation cannot
apply it"; ACC-026 says the same of every request a policy applies to. `engine` states `bound.v1`
alone in `supports` (MAN-012), and CON-028 allows `bound.v1` only on a Conduit to `access`. So, for
a secret that `engine` serves (S5, option B1):

- the read reports unavailable whenever the Agent's Conduit to the Resource carries any filter, and
  whenever any policy names that Agent among its `actors`;
- a policy that names the secret among its `targets` makes every read of it report unavailable,
  except a read that serves the root Interface, which ACC-035 exempts: the reads at process start
  of S8, which serve no one, are included, so no process that needs the secret starts while the
  policy is in force;
- a Conduit to the secret itself can carry no filter: CON-022 does not instantiate a Conduit with
  a filter that the target's implementation does not support.

A package that implements `reveal-secret.v1` fares little better. A carried filter is written
against the served operation, such as a filter on the repositories an Agent may list issues of. A
read has no inputs for such a filter to judge, so where the package states the requirement it
still cannot evaluate it, and the read reports unavailable under ACC-014.

| Option | What it is | What it costs |
| --- | --- | --- |
| R1 | As the specification stands: a read carries the served invocation's restrictions, and a policy applies to it by its caller. | Whoever narrows an Agent's Conduit to a Resource, or applies a policy to that Agent, makes every Resource it uses that reads a secret through `engine` report unavailable. The first user cannot give an Agent both limits and a credential it never holds, which is the purpose's own example. |
| R2 | `engine` states, for `reveal-secret.v1`, more requirements that it supports. | It can apply only a requirement whose meaning it knows. The core package's two, `bound.v1` and `shared-host.v1`, do not narrow a read; any other would be invented for the purpose; and a carried filter, written in another package against another operation, still could not be judged against a read. |
| **R3** | A read of a secret is decided by the reader's own grant: the Conduit it is made through, the filters on that Conduit, and each policy that names the secret among its `targets` and names no `actors`. It carries none of the restrictions of ACC-017, and no policy that names `actors` applies to it ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). Nothing else in the decision changes: ACC-034 and ACC-039 still refuse an onward read once a Conduit of its chain is closed or the consumer of one is retired. | A policy cannot stop one Resource's read for one Agent. It stops that Agent's requests to the Resource instead, which is where the value is used: the caller's restrictions still decide the invocation that the Resource serves, and the Resource's implementation applies them to what it does with the value (IFC-019). A policy that names the secret still applies, and with `engine` makes every read of it unavailable while it is in force. That stops all reads of one secret at once only where ACC-035's exception for the root Interface does not reach a read, which is the sub-question after this one; otherwise a read that serves the root Interface goes ahead. |

**What AWS and Kubernetes do.** In both, the reader's own identity decides a workload's read of its
credential, and the restrictions of whoever called the workload are not carried onto it. AWS's
provider for the CSI driver "assumes the IAM role of the Pod", ECS reads for injection under the
task execution role, and on EC2 every application reads under the one role of its instance
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[EC2](../../research/SECRETS.md#delivering-to-an-ec2-instance)). Kubernetes lets a kubelet read the
Secrets "related to pods bound to the kubelet's node"
([packet](../../research/SECRETS.md#the-kubernetes-secret)). The one model read that carries the
person into the read is Vault Enterprise's on-behalf-of token, under which "Vault allows the request
only if both the subject baseline and the OBO ceiling permit it"
([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)); it needs a store that
evaluates both, which `engine` is not, and HashiCorp's pattern for agents still keeps the
credential in the server that uses it.

**Recommendation: R3.** It is the workload model that AWS and Kubernetes use, and it keeps each
limit where it can be applied: the caller's limits on the operation the caller asked for, and the
reader's grant on the read. It also makes the onward read and the read at process start of S8 one
case: both are decided by the reader's Conduit alone. The record of an onward read still names its
caller, so who caused each read stays known (S13). What it gives up is a policy aimed at one Agent's
use of one credential through one Resource, which R1 offers only in name, since under R1 that
policy makes the Resource unavailable to the Agent altogether.

R3 replaces only what ACC-017 carries and what ACC-031 applies by caller. ACC-016 and ACC-003
still require the reader's own open Conduit that lists the operation, and the chain checks still
hold: a read made after the Agent's Conduit to the Resource is closed reports refused (ACC-034),
and so does one made after the Agent is retired, unless it serves the Agent's own retirement
(ACC-039). IFC-020, under which an onward request carries the served invocation's restrictions
"whatever else the onward request states", takes [SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s exception for those restrictions; the
read still carries the served invocation's caller, and its record names it.

**Sub-question: the root Interface's exception (item 3c).** New after the fourth review. ACC-035
says that a policy that gives only `targets` "does not apply to a request that the root Interface
makes of one of them, or to an onward request that serves the root Interface". So under R3 as the
third revision wrote it, a Resource's read that serves the root Interface escapes a policy on the
secret, and the same read serving an Agent does not: the outcome depends on the caller, against
R3's own rule that the read is decided by the reader's grant. ACC-035 exists so that "The root
Interface can therefore always remove a policy", and removing one is a request of `access`, which
no policy on a secret reaches.

| Option | What it is | What it costs |
| --- | --- | --- |
| C1 | ACC-035 reaches a read: a policy on a secret stops every read of it except those that serve the root Interface. | A read's outcome depends on who called the Resource that makes it, and no policy stops all reads of one secret. |
| **C2** | ACC-035 does not reach a read: a policy that names a secret among its `targets` applies to every read of it, whoever the caller ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). | ACC-035 gains an exception. The root Interface can still remove the policy. |

**Recommendation: C2.** It keeps R3 one rule, the one the read at process start already follows,
since that read has no caller for ACC-035 to exempt, and it gives a way to stop every read of one
secret at once. If the owner prefers C1, [SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s "every read" becomes "every read that serves no
root Interface invocation".

### S4. Who may read a secret

**The question.** Who may read; what the operation is; and what "a credential is never handed to a
caller" means for an operation's outputs.

**The right.** The consumer of an open Conduit to the secret Resource that lists
`reveal-secret.v1`, under every restriction that applies to the request (ACC-001, ACC-008). The
capability `secret-reveal.v1` lists that one operation. The operation takes no input beyond what a
request carries, because the Conduit names the target, and has one output, `value`.

**The mark.** An output of an Operation contract may hold `secret: true` (proposed). It is a mark
of the same kind as `handle: true` (CTR-002): it says what the output is, and the engine treats it
accordingly. A secret output is never recorded, whatever the type's `records` says; its content is
never retained for retrieval; no step of a graph names an operation that has one, and no value
reference may read it; the engine puts it in no inspection result, observation or diagnostic; and
it is delivered only to the instance that sent the request, or under S8 into the environment of a
process the engine starts. These bind the copy the engine makes of the output; what the reader
later sends is S10's matter. No input carries the mark in the first version, because no operation
of the first version takes a secret value as an input (S5).

**Who may be the consumer, and who opens.**

| Option | Who may read | What it costs |
| --- | --- | --- |
| A | Any consumer whose Conduit lists the operation, actors included. As Kubernetes, where anyone with `get` on a Secret reads it. | "A credential is never handed to a caller" becomes untrue as soon as an actor is given such a Conduit. An Agent's model would hold the value in its context. |
| **B** | Only a consumer that is not an actor: a Resource. | The developer cannot read a secret back through Conduit; they read it in the store, where they put it. |
| C | As B, except that the root Interface may read. | One exception to a rule that is otherwise plain. The root Interface is a person or a program acting for them, and its results are as easily logged as any other. |
| **D** | As B, and also: only the root Interface may open a Conduit that lists such an operation. | Strong and simple for one developer. A team cannot delegate the right until a later version allows it. |

Ruling 39, point 2, accepted B and D together, for the first version.

**What AWS and Kubernetes do.** Kubernetes shows the gap that B and D close: "anyone who is
authorized to create a Pod in a namespace can use that access to read any Secret in that
namespace", even through a Deployment; "list and watch access also effectively allow for users to
reveal the Secret contents"; and its mitigations only detect and limit
([packet](../../research/SECRETS.md#the-kubernetes-secret)). AWS states the risk of option A for
agents: an agent that can call `get-secret-value` will "receive plaintext secrets in their context
window", and its answer blocks direct reads, through MCP tools among other channels. AWS also
separates the role that manages and rotates a secret from the role that uses it, and calls the
right to set up rotation "not appropriate for end users"
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). Azure documents the
flaw that D avoids: under its older model, whoever may write a vault "can grant themselves data
plane access"; its fix reserves granting to two roles
([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). The External Secrets
Operator's shared store shows why one Conduit per secret is worth its cost: with one store for all
tenants, "Now all application developers have access to all the secrets"
([packet](../../research/SECRETS.md#the-kubernetes-ecosystem)). GCP and Azure both make reading the
value a right of its own, apart from seeing metadata: the same split as between inspecting a secret
Resource, which shows its locator, and reading it with `reveal-secret.v1`. Parameter Store has the
same split, and shows why the output mark should be generic: four of its operations return a value,
each authorized on its own, and denying one leaves the others open
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)).

Who may arrange a read is, in both, a question apart from who may read. AWS makes attaching a role
to a workload a permission of its own, `iam:PassRole`, scoped to named roles and to the service that
may receive them, which "helps administrators ensure that only approved users can configure a
service with a role that grants permissions"; GCP's `iam.serviceAccounts.actAs` is the same kind of
permission. AWS states why: whoever passes a role lets the service act "on behalf of" the one who
passed it. Kubernetes lets a subject create a role, or bind one, only if it already holds what the
role grants, unless it is given the `bind` or `escalate` verb, which its good practices call a way
around that protection; the first grants come from a superuser. Kubernetes has no such check on a
workload: "since Pods can run as any ServiceAccount, granting permission to create workloads also
implicitly grants the API access levels of any service account in that namespace"
([packet](../../research/SECRETS.md#who-may-give-a-workload-a-read)). D is the strict form of AWS's
gate: opening a reading Conduit is a right apart from registering the Resource or setting it up, and
in this version only the root Interface, in the place of Kubernetes's superuser, holds it.

**How points 2 and 4 fit together.** Point 2 says only a Resource reads a secret. Point 4 says an
Agent's own provider credential is set in its process environment at start and never presented to
it as a tool, by the mechanism of point 3, in which the engine reads "through the instance's own
Conduit". Read together, this proposal states them so. The first two lines and the last are what
ruling 39 accepted. The third is new, item 4b in the list for the owner. The rule on tools in the
first follows from point 4 wherever 4b is accepted, item 4c, and the rule on steps in the first is
new, item 4e.

- No actor is ever presented a tool for an operation that has a secret output, and a request that
  an actor sends for one anyway reports refused ([SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)); no step of a graph names such an
  operation ([SEC-21](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)); and the engine gives no actor a result that holds a value ([SEC-10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
- A Resource reads by an onward request, as above.
- An Agent may be the consumer of a Conduit to a secret, which the engine uses only when it starts
  the Agent's process, by the mechanism of S8 (new, [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). An Interface is never such a consumer
  ([SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). The secret is meant to be the Agent's own provider credential, but no promise can say
  so: the engine cannot tell a provider credential from any other secret, and [SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and
  [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) do not limit which secret the Conduit leads to. So whatever secret the root Interface
  routes to an Agent, a GitHub token as readily as a model key, is set in the Agent's process,
  where a model that can run commands can read it (S10). That reaches past point 2 for whatever
  secret is so routed. A checkable limit, such as at most one such Conduit in an Agent's template,
  would bound how many secrets reach an Agent, not which, and is not proposed.
- Only the root Interface opens either kind ([SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).

The research is plain about what point 4 then gives. The value sits in the Agent's own process.
Claude Code "keeps the credentials for its own API calls" while the shells, hooks and stdio servers
it starts inherit its environment unless it scrubs them
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)), and Google warns
of "dependencies that log process environment details"
([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). So for an Agent, "never handed
over" means never presented as a tool, never in a result and never recorded; it does not mean that
the Agent's model cannot read its own environment. That argues against a strict reading of point 2
for Agents. The proposal does not overturn point 4: it states the limit in S10, and recommends a
short-lived or narrowly scoped provider key where the provider offers one (S7).

**What "only the root Interface may open" means.** The opening of a declared Conduit at setup is a
request of the instance that sent the request to set up (ACC-005), also where a Pool's setup sets
up its entries (POOL-015). So a Resource or an Agent whose template declares a Conduit that reads a
secret is set up with it open only where the root Interface sent the setup. Any other actor's
request to open one reports refused, in the way ACC-032 refuses the opening of a Conduit that lists
a reserved operation. In this version no bound passes the right on.

For teams later, the paradigm is Kubernetes's rule against escalation together with an explicit,
named right to arrange, as its `bind` verb on a named role and AWS's PassRole on a named role are.
The revision named Azure's Key Vault Data Access Administrator alone, but that role limits only
which roles its holder assigns; limiting who receives them is an extra condition that must be
added ([packet](../../research/SECRETS.md#who-may-give-a-workload-a-read)). Conduit already has
both halves. A bound on a Conduit to `access` is a right to open that is apart from the right to
use, since being the consumer of a Conduit to a target does not permit opening one (ACC-006). And
a right to open passes on "only as it is or narrower" (ACC-030), which is Kubernetes's rule. So
the later version would let a Conduit to `access` whose bound names `reveal-secret.v1` and a Pool
of secrets, or named secret Resources, open reading Conduits within that bound (ACC-029). It would
keep [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s rule on the consumer whatever the bound says, as PassRole names the service that may
receive a role, so that whoever arranges a read never receives it. It would adopt no equivalent of
`escalate` or of a `bind` on every role, which Kubernetes calls a way around its protection, and
not the semantics of AWS's permissions boundaries, under which a delegate can create access it
lacks. Naming a Pool covers the secret Resources it owns later too (ACC-029), much as a grant on a
Parameter Store path covers every parameter under it, whatever is denied on the parameter itself;
the later version should say so where it is written. Both granularities have a precedent: Azure's
role at the vault corresponds to a bound that names a Pool of secrets, and GCP's grant on one secret
to a bound that names secret Resources. If teams ever need a statement on the secret's side, such
as an owner of a Pool saying which team may arrange reads of it, AWS's rule across accounts is the
paradigm, not its rule within one: both sides must allow, and an account passes on only what it was
granted, "it cannot delegate more access than the account itself has been granted"
([packet](../../research/SECRETS.md#a-list-of-readers-on-the-secret)). Such a statement would bound
the opening of Conduits, and grant nothing; and AWS's guards would come with it: no broad entries,
a check before it is attached that it locks no one out, and the right to edit it treated as a right
to escalate.

**A list of readers on the secret (item 4d).** New after the third review. The Conduit stays the
only way a read is arranged. Neither `secret.v1`, nor a locator, nor a package carries a list of
readers that acts as a grant. The research shows what the other choice brings
([packet](../../research/SECRETS.md#a-list-of-readers-on-the-secret)). AWS lets a policy attached
to a secret name who may read it. Within one account the two places are a union: "If either the
identity-based policy or the resource-based policy within the same account allows the request and
the other doesn't, the request is still allowed", and a grant there made directly to a user or a
session is not limited by a permissions boundary. AWS then has to guard the list: whoever may write
it can "escalate existing permissions like obtaining full administrative access to secrets"; the
block on broad principals is off by default; validation by automated reasoning runs before a policy
is attached; and both checks cover only the policy on the secret, so the identity's policies and
the key's policy must be inspected apart. GCP keeps an allow policy on each secret and recommends
granting there, at "the lowest level in the resource hierarchy". Kubernetes keeps who may read in
Role and RoleBinding objects, outside the Secret, and Azure in role assignments, which it says not
to scope to one secret but for two exceptions.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | No list on the secret. A read is arranged only by opening a Conduit, which only the root Interface does in this version. | A developer who wants to see who may read a secret inspects the Conduits to it, not the secret. |
| B | A list of readers in the secret's configuration, as GCP's allow policy and AWS's resource policy are, each entry a grant. | A second way in, with AWS's union rule; the list is configuration, so whoever may replace the secret Resource may rewrite it, which is AWS's escalation through `PutResourcePolicy`; and Conduit would need AWS's guards against broad entries, which invents a second access model beside the Conduit. |

**Recommendation: A.** It is Kubernetes's and Azure's recommended form, and it keeps "every
permission is an open Conduit" true. GCP is the counter-example, and is the form rejected here. A
store's own access control, such as a resource policy on a secret in Secrets Manager, governs only
what the package's own identity may read at the store (S16). It is a second gate behind the
Conduit for who reads through Conduit, but at the store it can be a way in of its own: within one
account, an Allow in a secret's resource policy can grant the package's identity a read with no
identity policy at all. A package's documentation should say which of its store's policies it
relies on.

**What "never handed to a caller" means.** The caller of a read is the actor whose invocation the
Resource is serving (IFC-020); a read at process start has none (S8). Under B the value is
delivered to the sending instance, the Resource, and never to the caller. What the Resource's
implementation does with it then is its own act, which the engine cannot see: the proposal states
that as an obligation of the implementation, at the Implementation level of conformance, in the way
that the Model Context Protocol forbids a server to pass a client's token through and IFC-031
forbids an implementation to forward Conduit's keys. The implementation uses the value toward the
outside service it is for, and never puts it in a result, a diagnostic, the arguments or `_meta` of
a request to the engine, or anything it sends an actor. Kubernetes puts the same duty on the
application: it "must avoid logging the secret data in the clear or transmitting it to an untrusted
party" ([packet](../../research/SECRETS.md#the-kubernetes-secret)).

The comparison with a handle is close. A handle is an opaque string that the engine passes back
unchanged, and "holding one permits nothing" (IFC-011). A reference to a secret is the same: an
actor may hold the resolved form `{kind: Resource, id}` of a secret, see it in inspection, and pass
it as an argument, and that permits nothing. Only a Conduit permits a read.

**Recommendation: B and D, with the generic output mark, and no list of readers on the secret. B
and D stand, as ruling 39 accepted; the Agent as a consumer, [SEC-21](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and the rejection of a list on
the secret are new, and [SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) follows from point 4.** The first draft recommended B and left D to
the owner; the owner chose both
for the first version, and the research, the second round on PassRole and escalation and the third
on lists of readers included, gives no reason to depart from that. It is the one combination under
which the sentence of GP3 is a promise that can be checked: no tool that reads a secret is ever
presented to an actor, and no actor but the root Interface can arrange a read. The mark is generic,
rather than fixed to `reveal-secret.v1`, so that a package can later define an operation that
issues a short-lived credential to a Resource, as Vault's dynamic secrets do, and have the same
rules apply (S15).

**What this cannot stop.** With D, the first draft's main limit narrows. An actor that registers a
Resource with an implementation of its own choosing can no longer open a reading Conduit for it;
only the root Interface can. The opening is the moment of trust, as `direnv allow` is for a new
`.envrc` ([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). Two
things remain. The first is the root Interface's own choice: whoever wrote the implementation that
the root Interface lets read a secret can recover its value, and the secret's locator, which
whoever registered the secret wrote, decides what is read (S10). The second is any code the engine
starts at another actor's request. Every process the engine starts runs as the engine's
operating-system user, and may be started before any invocation ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), DL12). The engine's own
environment holds the values of its `environment` sources (S5), and the kernels let a process of
the same user read it, on Linux where the engine is dumpable and on macOS in every case, and open
the files the `file` sources name
([packet](../../research/SECRETS.md#processes-of-the-same-user-on-linux-and-macos)). So an actor
that may register and set up a Resource, or install a package, whose implementation the engine
starts can read every value of the engine's own sources with no Conduit at all (S10). The
third revision said that only the root Interface's choice remained, which this does not support.
The same will hold of a delegate later:
AWS says of a delegate who may create users that "Maria effectively trusts Zhang" with the access
those users get ([packet](../../research/SECRETS.md#who-may-give-a-workload-a-read)), so delegating
the right to open a reading Conduit is, in effect, delegating the read.

### S5. Where the value is stored

**The question.** What keeps the value, and what is left to each engine.

| Option | What keeps the value | What it costs |
| --- | --- | --- |
| A | **The engine**, in a store of its own, with an operation to write a value. | The engine becomes a secret store, with encryption at rest, backup and key management to specify or leave to each engine. The value enters through a request's arguments, so records, digests and repeats need exceptions (S3, option A). It invents what existing stores already do. |
| **B** | **An existing store**, read by an implementation of `reveal-secret.v1`. The secret Resource's configuration says where the value is (a path, a variable's name, a keychain item, a secret manager's name and version), never the value. The person puts the value in the store with the store's own tools. | Nothing in Conduit writes a secret. Each store needs an implementation. |
| C | Both. | The costs of A, for a gain the first user does not need. |

**Which stores the engine reads itself.** The core package's one implementation is the engine
(MAN-012), so whatever the core package implements, every engine implements.

| Option | What `engine` reads | What it costs |
| --- | --- | --- |
| B0 | Nothing. Every store is an ordinary package. | A developer cannot use a secret without first installing a package. |
| **B1** | A file, named by an absolute path, and an environment variable of the engine's own process, named by its name. As Compose's two sources. | Two small sources. A file is protected only by the file system's permissions, and an environment variable by the engine's process. |
| B2 | B1 and the operating system's keychain. | Platform-specific code in every engine. Whether a keychain can be read unattended, from a daemon or a rebuilt binary, is still open (experiment E10 of the [tracker](../../AUDIT.md)). |

**What AWS and Kubernetes do.** Kubernetes is the clearest record of what option A costs. Its
Secrets are "by default, stored unencrypted" in etcd; a local key "fails to protect against a host
compromise"; envelope encryption needs a KMS plugin on each control-plane host, which keeps its own
credentials and caches data keys in the clear; and encrypting what is already stored means
rewriting every Secret ([packet](../../research/SECRETS.md#the-kubernetes-secret)). Kubernetes has
moved its own service-account tokens out of that store, to short-lived tokens issued on request,
and fetches registry credentials through plugins "as opposed to storing static credentials on
disk". AWS's local provider serves, from a cache, values that live in Secrets Manager
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). The CSI driver and
the External Secrets Operator read outside stores; Docker Compose has exactly B1's two sources
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). Pulumi is the one
tool read that keeps secrets itself, encrypted in its state, and it needs key providers to do so.

**Recommendation: B, with B1. Stands.** Keep the value in an existing store and read it there. This
is what the CSI driver, the External Secrets Operator, Vault and the cloud managers do, and what
the older design wanted ("custody through ... implementation bundle", the OS credential store by
default; [older text][pools], not binding). The engine reads a file or an environment variable, as
Compose does, so a single developer can start with no package. The keychain and the secret managers
are ordinary packages that implement `reveal-secret.v1`; the template of a secret selects one under
`implementation` (TPL-003). The path and the variable's name are values tied to a location
(PLC-020): they are read only at the secret Resource's placement.

**Where `engine` reads (item 5d).** New after the fourth review. PLC-020 says that the engine
"never opens" a value tied to a location "against the machine the engine itself runs on or against
any other location", and that an absolute path means "that path on the host of the Resource's
location". Under B1 the engine opens the path and reads its own environment, which has meaning only
on its own host, and nothing said what `engine` does for a secret placed elsewhere. The
recommendation: `engine` reads its sources only for a secret placed on `local`, the location made
at first start, where `management` and `access`, which `engine` also implements, are placed; for
a secret placed on any other location the read has the outcome failed and nothing is opened
([SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). A `file` that is not an absolute path fails the read in the same way: INS-018 leaves a
relative path to "the schema of the type that receives it", and `secret.v1` has none. PLC-020's
observable takes the exception for `engine` serving a secret on `local`. A requirement on placement
was the other choice, but `engine` is one item for every operation of the core package, so a
requirement on it would bind `management` and `access` too, and no core requirement says "the
engine's own host". A developer with one machine has only `local`; a secret on another location
needs a package that runs there.

These rules follow, each from the older design or the sources:

- An implementation of `reveal-secret.v1` takes the value from the store its configuration locates,
  never from the configuration itself. Otherwise a store that "reads the value from `config`" would
  bring back the literal that ruling 38 rejects.
- An implementation in a package learns that locator as every implementation learns its instance's
  configuration (item 5e, new after the fourth review). No request carries configuration: IFC-006
  sends the instance, the operation, the invocation, the caller, the restrictions and the Conduit,
  `reveal-secret.v1` has no inputs, and GRA-010 accepts a value reference with the source `config`
  only in a setup or retire graph. So the store's template writes a `setup` that passes the locator
  to a step, such as the start of a runtime, and the implementation keeps it for that instance,
  whose ID each later request carries; the AWS sketch above shows it. A read before that setup has
  succeeded finds the secret not ready. Sending the configuration with each request would be a new
  channel of IFC-006, for one operation, which this proposal does not invent.
- Where the store cannot be read (the file is missing, the keychain is locked, the manager refuses),
  the read has the outcome failed and no value, with no fallback to another source ("without silent
  plaintext fallback", [older text][pools]). Codex's `keyring` mode "fails if it is unavailable" in
  the same way ([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)).
- Nothing in the first version writes a secret's value. The developer writes it to the file, the
  shell profile or the keychain. A Resource that writes values (a password manager) can come later
  in a package, with the mark on inputs that it would need.
- The engine's own environment holds the values of its `environment` sources. 1Password warns that
  "processes on your computer can access the environment of other processes run by the same user"
  ([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)), and the
  kernels say on what terms: on Linux, processes of the same user and groups, where the process is
  dumpable; on macOS, processes of the same user, unless the process's signature restricts it
  ([packet](../../research/SECRETS.md#processes-of-the-same-user-on-linux-and-macos)). So no
  process that the engine starts inherits those variables (S18). The file source is the better of
  the two for a long-lived engine, though not because fewer processes can read it: a file that only
  its owner may read is open to every process of that user. It is better because it is inherited by
  nothing, and because it can be changed while the engine runs, whereas the engine's own variable
  keeps its value until the engine is started again.
- A command that prints a credential, as `kubectl`'s exec plugins and Claude Code's `apiKeyHelper`
  do, is an established third kind of store
  ([packet](../../research/SECRETS.md#the-kubernetes-secret),
  [tools](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). It is left to
  packages, not to `engine`: a command that a package supplies is code that reads secrets, and the
  root Interface's opening of the reading Conduit is the moment it is trusted (S4). The second round
  supports leaving it there. No command format is general: AWS's `credential_process` prints only an
  AWS key triple, Kubernetes's ExecCredential a bearer token or a client certificate, and GCP and
  Docker use formats of their own. A package for AWS or Kubernetes gets both through its own client,
  with no work. Both platforms treat a file that names a command as code that will run: "anyone with
  write access to your shared config file can change the command that is run", and kubectl's
  allowlist of plugins is kept in `kuberc`, apart from the kubeconfig that names them
  ([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). If a command
  store ever enters the core package, those are the precedents to adopt: ExecCredential v1 as the
  nearest format for one string with an expiry (a judgment, which no source states), and a list of
  allowed commands kept by the engine outside every document.
- Evidence for experiment E10 on keychains: Claude Code falls back to a file "When the Keychain
  rejects the write, such as when it's locked in an SSH session"
  ([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). A keychain is not
  reliably usable unattended, which keeps B2 out of the first version.

**Ciphertext kept beside configuration.** New after the review. Sealed Secrets and SOPS keep
encrypted values in Git beside the configuration that uses them
([packet](../../research/SECRETS.md#ciphertext-kept-beside-configuration)). The paradigm has three
forms, which the proposal treats apart.

| Form | Treatment | Why |
| --- | --- | --- |
| Ciphertext written inside a Conduit document, such as a sealed value in a secret Resource's `config`, which the engine or a package decrypts | **Never read as a value**, and advised against. Nothing rejects it: `secret.v1` states no `schema`, and such a type "puts no rule on the configuration of its instances" (CTR-004), so it is accepted and kept as data. The third revision said "Rejected", which no promise did. | An encrypted literal is still a literal: [SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) has an implementation take the value from its store, "never from that configuration". Configuration is inspected, recorded and revisioned, so the ciphertext would stay in records (REC-003), and Sealed Secrets says a value in version control must be assumed never deleted; a later leak of the key would expose every value sealed with it. The engine would become the Sealed Secrets controller, generating, renewing and keeping keys and having its owner back them up after each renewal, which is S5 option A's and S14 option B's cost. And holding the ciphertext, put under the right name, would be a second way to a value, which is why Sealed Secrets needs scopes and admits that "this scheme *does not authenticate the user*"; in Conduit a copied reference permits nothing, and only an open Conduit does (ACC-001). |
| A file encrypted with SOPS, named by its path | **Listed** among the stores an ordinary package may implement, beside the keychain and the cloud managers; not in `engine`. | Decrypting needs SOPS's key clients (KMS, age, PGP), which is platform-specific code of the kind B1 keeps out of every engine. The package adopts SOPS's format and tool unchanged. It decrypts at each read, as `sops decrypt --extract` does, never at registration, so an edited file is seen at the next read (S6); Flux's decrypt at apply leaves a plaintext copy. Its locator is the file's path and SOPS's own extract path (S17). It finds its own key as S16 says. A SOPS file may hold some values in the clear (`encrypted_regex`, `unencrypted_suffix`), so the package should say whether it reveals one. |
| A SealedSecret | **Not a store Conduit reads.** | Only its controller in the cluster can decrypt it, and the result is an ordinary Kubernetes Secret; a package that reads Kubernetes Secrets covers a cluster that uses it. Reading a sealed file directly needs the exported private keys that the project says to keep "omg-safe". |

**What is implementation-defined.** A new item of the
[implementation-defined list](../../../spec/implementation-defined.yaml), [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) in the [drafts below](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow):
how `engine` reads a file and an environment variable (the encoding, a final newline, the largest
value), and whether it reads them with its own process's rights or another's.

[pools]:
  https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/POOLS.md

### S6. Rotation and revocation

**The question.** What changing a secret does to the instances that use it, and what closing the
Conduit does.

**Rotation.**

| Option | What a change of value does | What it costs |
| --- | --- | --- |
| **A** | Nothing in Conduit. The value is changed in the store, and the next read returns the new value. No revision advances and nothing is set up again. | An implementation that kept the old value keeps using it until it reads again. The engine cannot see the change. |
| B | A control on the secret with a revision, such as `rotate.v1`, as Terraform's `_wo_version`. | Only meaningful where the engine stores the value (S5, option A). |
| C | The instances that use the secret are set up again. | An instance is set up once; setting it up again means retiring it and making another (LIF-006). It would make rotation an outage. |

**What AWS and Kubernetes do.** AWS rotates in the store: a staged protocol ends by moving
AWSCURRENT to the new version, and a reference with no version reads it. CloudFormation recommends
references with no version, "This ensures your credentials are regularly updated without requiring
template changes" ([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)).
Kubernetes changes a Secret in place, and its consumers see the change on their own terms:
environment variables never without a restart, volumes eventually, `subPath` mounts never
([packet](../../research/SECRETS.md#the-kubernetes-secret)). Revocation, in both, works at the
verifier and never recalls a copy. AWS denies role sessions issued before a time; Kubernetes has
"no specific mechanism to invalidate a token issued via TokenRequest" other than deleting the Pod;
and only Vault, by revoking a lease, invalidates a credential it issued
([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). AWS makes a deleted secret
unreadable at once, and keeps it recoverable in the store for at least seven days.

**Recommendation: A. Stands.** Rotation happens in the store, where it already happens. Which
version a locator names is S11, what a cache in a package does is S12, and what happens to a value
set in a process's environment is S19. The proposal recommends, as guidance to authors of
implementations and not as a promise, reading at each use, or again after the outside service
refuses a credential, rather than once at setup. That guidance is existing practice. `kubectl`
fetches a credential again after a 401 ([packet](../../research/SECRETS.md#the-kubernetes-secret));
Claude Code runs its `apiKeyHelper` again after a 401 or 403
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)); AWS answers the
window of single-user rotation with "an appropriate retry strategy"
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)); and AWS's SDKs run
a `credential_process` again before its expiry, and do not cache what it prints
([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). Parameter Store
itself rotates nothing: its parameter policies only delete a parameter or send a notice, and AWS
points credentials that need rotating to Secrets Manager
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). A read after such a
deletion fails, and the package reports it so.

**Revocation.**

- **Closing the Conduit** from a Resource to a secret ends its right to read: every later read
  reports refused (ACC-011, ACC-036). It does not recall a value already read. The honest statement
  is: closing the Conduit stops later reads; to revoke the credential, rotate it in the store.
- **Retiring the secret** closes every Conduit to it at once (CON-008), so no later read is
  permitted. Its records stay (REC-003). This is AWS's deletion, without the recovery window,
  because the value stays in the store and a new secret Resource can name it again. The readers
  do not follow, a point added after the fourth review. A reader's configuration keeps the
  resolved form it was made with (OBJ-010), a replacement of its declared Conduit to a retired
  target reports invalid (LIF-009), and [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) reads the secret that `environment` points at,
  which is still the retired one, so that read reports refused. So every reader that names the
  retired secret, a plain server's instance or an Agent, must be retired and made again with the
  new one before the engine can start its process, unless its type has a control for that place
  and the root Interface opens a Conduit to the new secret for it.
- **Retiring the reader** leaves its Conduit to the secret open until its shutdown is confirmed,
  for the requests of its own retirement only (ACC-039), then closes it (CON-008). Among those is
  a read at the start of a process that the engine starts to dispatch a step of that retirement or
  the stop of one of its runtimes (item 8h, [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
- **A read in progress when the Conduit is closed** runs to its end, and its content is withheld
  (ACC-034, OPS-017): the value is not delivered.

### S7. Authentication

**The question.** The owner named authentication. There are three credentials in play, and they
should not be confused.

1. **The credential an instance presents to the engine**, to show which instance it is (IFC-022,
   DL13). The engine never passes it on (IFC-030), and a request from which no instance can be
   established is refused (IFC-032), or answered with HTTP 401 where the protocol's own
   authorization is used.
2. **The credentials an implementation uses toward outside services**: a token for a code host, a
   key for a model provider, a password for a database.
3. **An Agent's own provider credential**, with which its model runs.

| Option | What the secret type covers | What it costs |
| --- | --- | --- |
| A | All three. The engine issues each instance a credential and keeps it as a secret Resource. | Circular: to read a secret, an instance must first be established by the credential that the secret would hold. The engine would store secrets (S5, option A). |
| B | The second only. The first stays the engine's own business, left to each engine by DL13. The third stays the Agent's own. | An Agent's provider credential is outside every Conduit: not inspected, recorded or closable. |
| C | The second and the third, with the Agent reading its own credential through a tool. | The Agent is an actor; its model would hold the value in its context. Point 2 of ruling 39 rules it out. |
| **D** | The second, and the third set in the Agent's process environment at start by the engine, never as a tool. | The value sits in the Agent's process, which its model may be able to read (S4, S10). |

**What AWS and Kubernetes do.** Both answer the first credential with identity, not a stored
secret. AWS's central pattern for workloads is a role whose temporary credentials the platform
delivers; on EKS a Pod's environment holds only a pointer and the path of a short-lived token file
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)). Kubernetes moved its own
service-account tokens out of Secret objects, because "any user with read access to those Secrets
can authenticate as the service account", to short-lived, audience-bound tokens issued on request
([packet](../../research/SECRETS.md#the-kubernetes-secret)). For the third, AWS sets a Lambda
function's own role keys in reserved environment variables at start
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)); the Anthropic SDK reads
`ANTHROPIC_API_KEY` by default; Claude Code puts `ANTHROPIC_AUTH_TOKEN` and `ANTHROPIC_API_KEY`
above its `apiKeyHelper` and its stored login; and `op run` and `doppler run` set a child's
environment for its lifetime
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). On a developer's
machine the AWS SDK for Rust reads the access-key variables first in its chain
([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). Against it, AWS
recommends for Lambda "AWS Secrets Manager instead of environment variables" for API keys
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)), and calls its own protection
of agents "a best-effort defense, not a security boundary"
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)).

**Recommendation: D. Changes,** to follow point 4 of ruling 39; the first draft recommended B.

- The identity of an instance is not a stored secret but something the engine establishes for each
  request. That is SPIFFE's lesson and AWS's and Kubernetes's practice. Over HTTP the protocol's own
  bearer tokens apply; over stdio the engine knows the process it started. DL13 stays as it is. If
  an engine later needs more, AWS's local credential endpoints are the shape to adopt, and the
  revision's "two shapes" are one: in EKS Pod Identity and in AWS's local secret provider alike, the
  process reads a token from a file, or a variable, that it was set up with, and sends it in a
  header with each request to a local endpoint. `AWS_CONTAINER_CREDENTIALS_FULL_URI` with
  `AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` is the container provider, which the page's table marks
  as supported by every AWS SDK and tool it lists, so an engine that ever hands AWS credentials to
  packages needs to invent nothing. Such a token stops forged, relayed and
  off-host requests; it does not separate local processes of the same user, as AWS warns of its own
  credential helper served on 127.0.0.1
  ([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). Which instance a
  request comes from would still rest on who can read the token.
- Outside credentials are exactly what the secret type is for, read as S3 describes.
- An Agent's own provider credential is a secret Resource. The Agent's template declares a Conduit
  to it, which only the root Interface opens, and the Agent's implementation item points under
  `environment` at the place in the Agent's configuration that refers to the secret; the engine
  reads it and sets the variable when it starts the Agent's process (S8), and presents no tool for
  it ([SEC-17](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). The research supports the direction and shows its limit (S4, S10). Nothing makes
  the secret the Agent's own credential: the root Interface chooses what the Conduit leads to
  (item 4b).
- **What a value set at start can carry (item 7c).** New after the third review. The providers'
  own forms differ in how long they live
  ([packet](../../research/SECRETS.md#an-agents-own-model-credential)). A value set once serves a
  key or a token that outlives the process: an Anthropic API key, Claude Code's setup-token, which
  is "a one-year OAuth token" that "can only make model requests", a Bedrock long-term key, and a
  Bedrock short-term key for a process that ends within its 12 hours. It does not serve a federated
  token in a process that outlives it. Anthropic's lasts at most an hour under a rule's default
  lifetime, which a rule may set as high as 24 hours, and each identity token that carries a `jti`
  claim is by default good for one exchange; OpenAI's "expire after at most one hour", with no
  refresh token. The condition on `jti` is the Anthropic page's own: "By default, identity tokens
  that carry a `jti` claim are single-use"
  ([Anthropic](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)).
  The packet's sentence, "By default an identity JWT is used once", leaves it out; this revision
  leaves the packet unchanged, under the repository's rule against editing what `docs/research/`
  holds. For those, each
  provider's answer is a variable that holds no secret and points at a file something keeps fresh,
  as `ANTHROPIC_IDENTITY_TOKEN_FILE` and `OPENAI_IDENTITY_TOKEN_FILE` do, or a command the tool
  runs again, as Claude Code's `apiKeyHelper`, `awsAuthRefresh` and `gcpAuthRefresh` and Codex's
  provider command are. The first version adopts neither for an Agent: an Agent that uses such a
  credential finds and refreshes it through its own tool's helper or profile, outside every
  Conduit, as a package finds its store's credential under S16. Federation also needs an issuer of
  identity tokens, which one developer's machine does not usually have (a judgment, which no
  source states). Guidance, not a promise: prefer
  the narrowest form the provider offers, such as the setup-token, a federated token limited to
  `workspace:inference`, or a Bedrock key whose IAM principal may only invoke models.
- A shared login, the purpose's first example, is a Resource whose implementation reads the
  credential and calls the outside service on the caller's behalf, exposing operations and never the
  credential. HashiCorp's validated pattern for agents does the same, putting the credential in the
  MCP server ([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). Its type belongs
  in an ordinary package, built on `secret.v1`. Refreshing a login, OAuth flows and token exchange
  under RFC 8693 are left out of the first version; RFC 8693's delegation, with an `act` claim
  naming the agent, is the standard to adopt when they come in, and Conduit already records the
  caller and the Conduit of every request for it.

**If the owner rejects an Agent as a consumer** (item 4b), two routes remain for
point 4, and neither is recommended. The Agent's credential stays outside Conduit, in the
environment the engine was started with, which is option B and loses the Conduit's record and
closing. Or the engine reads it with no Conduit at all, which would be a second way to permit a
read, against "every permission is an open Conduit".

### S8. Existing servers that read credentials from the environment

**The question.** A plain implementation is an existing Model Context Protocol server that was not
written for this specification (PKG-020). It cannot make an onward request, because it does not
know Conduit's keys (IFC-007). Many such servers run over stdio and take their credential from an
environment variable, as the protocol tells them to: "retrieve credentials from the environment"
([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). Every server the
packet checked that takes one token accepts it as a variable; how many of the servers in use accept
only a variable was not checked
([packet](../../research/SECRETS.md#delivering-a-credential-to-an-existing-mcp-server)).

| Option | What it is | What it costs |
| --- | --- | --- |
| A | Not in the first version. The developer gives such a server its credential outside Conduit, in the environment the engine was started with. | The credential is then outside every Conduit: it is not inspected, narrowed, recorded or closed. Every plain server sees the engine's whole environment. |
| **B** | An implementation item that states `command` may state `environment` (proposed): a map from a variable's name to a JSON Pointer into the configuration of the instance it serves, at a place that `secret-reference.v1` governs. Before it starts the process for an instance, the engine reads each secret through that instance's own open Conduit to it, and sets the variable. A process started this way serves that one instance. | A new key on an implementation item. Process start, which DL12 leaves to each engine, gains a rule. A new value needs a new process. The value is in the process's environment, which every source warns may be exposed. |
| C | A wrapper: an implementation that carries Conduit's keys, reads the secret, and starts the plain server itself. | Every plain server needs a wrapper written for it. |

Ruling 39, point 3, accepted B's direction: the engine reads the secret through the instance's own
Conduit and sets the variable when it starts the process. Point 4 extends it to an Agent's own
process. The key `environment`, the rule on failure, one process per instance, whose read it is
and who starts the process were not part of what was accepted: they are items 8b to 8f, and the
three sub-questions below.

**What AWS and Kubernetes do.** This is their accepted path for software that can read nothing
else, with its costs stated. ECS injects a secret named by `valueFrom` when the container starts;
"If the secret is subsequently updated or rotated, the container will not receive the updated value
automatically", and "Applications that run on the container and container logs and debugging tools
have access to the environment variables"
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)). Kubernetes sets a variable
from `secretKeyRef` at container start, does not start the Pod while a required Secret is missing,
and leaves restarts on change to third-party tools
([packet](../../research/SECRETS.md#the-kubernetes-secret)). Cloud Run does not start an instance
whose secret cannot be retrieved, and Vault Agent's process supervisor waits until every variable
has rendered before it starts the process; App Service, which passes the literal reference on
instead, is the counter-example
([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). On whose identity reads: ECS
uses a separate launcher role, while AWS's CSI provider, Cloud Run and App Service read under the
workload's own identity. Reading through the instance's own Conduit is the second model, and keeps
each read attributed and closable. On EC2, a boot script that calls AWS "must use an instance
profile", and AWS notes that the script stays on disk after it runs and that its console output is
logged; a value that CloudFormation resolves "may show up in the service whose resource it's being
used in" ([packet](../../research/SECRETS.md#delivering-to-an-ec2-instance)). So the engine reads
from the store at start and writes the value nowhere but the new process's environment.

The servers and the runners of the Model Context Protocol agree. GitHub's server, this proposal's
own example, documents its token only as a variable. Brave's, Grafana's and a GitLab server also
accept a file named by a `_FILE` variable, and each accepts the variable as well. Docker's MCP
catalog maps each secret a server declares to a variable, and ToolHive gives a stored secret to a
server it runs as a variable
([packet](../../research/SECRETS.md#delivering-a-credential-to-an-existing-mcp-server)). SOPS's
`exec-env` sets decrypted values in one child's environment and writes them nowhere
([packet](../../research/SECRETS.md#ciphertext-kept-beside-configuration)). Whether a file should
be offered beside the variable is S21.

**Sub-question: whose read it is, and when.** New after the review. The revision tied the read to
the invocation that the engine was about to dispatch to the new process. But DL12 leaves to each
engine when it starts a process, and an engine may start one at setup, or when it starts itself,
with no invocation at hand. An Agent's process serves no invocation at all when it starts.

| Option | What it is | What it costs |
| --- | --- | --- |
| B1 | As the revision drafted: the read is an onward request that serves the invocation the engine is about to dispatch to the new process, with that invocation's caller. | It fails wherever there is no such invocation: a start at setup, a start with the engine, every Agent. Where there is one, a restriction tied to its caller binds only that first read; the value then serves later invocations of other callers with no read, so the restriction appears to bind what it does not. |
| **B2** | The engine makes the read at the start of the process for no actor, whatever caused the start, as OBS-007 has it stop a runtime for no actor. The read goes through the instance's own open Conduit and is decided by that Conduit, its filters, and a policy that names the secret among its `targets` and names no `actors`. Having no caller and serving no invocation, it carries nothing under ACC-017, and no policy that names `actors` reaches it under ACC-031 as it stands, so this holds whether or not item 3b is accepted. | No restriction tied to a caller binds the read, or the value it sets; S10 says so. IFC-006, REC-002 and the definition of caller gain a second request that has no caller, beside OBS-007's stop. No instance sends the read, so it has no sending instance either: the definition of the instance that sent a request, OPS-008 and [SEC-10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) say so. It is decided by neither ACC-001, which is about a consumer's own request, nor ACC-016, which is about an onward request; [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) alone permits it. With `engine`, a policy that names the secret makes the read unavailable while it is in force, and the process is not started. |
| B3 | A process for each caller, started with a read made for that caller. | A process for each caller of each instance, and a process started again as callers change, for servers that hold one credential and serve whoever calls. |

**Recommendation: B2.** It is true of every start, and it says what the value serves: every
invocation of that process, whoever its caller. It reuses the one shape the specification already
has for a request the engine makes on its own account, and, where item 3b is accepted, it is
decided exactly as an onward read is ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). A developer who needs the use of a credential
limited per caller puts the limit on
the operation the caller asks for, and uses an implementation that asks for the secret (S3, option
C): it applies the caller's restrictions to each invocation (IFC-019), and each of its reads is
made at use and recorded with its caller. A plain server can do neither: it supports no
requirement (PKG-020), so a filter on a Conduit to it is not instantiated (CON-022) and a policy
that reaches it makes requests to it unavailable (ACC-015).

**Sub-question: who starts the process.** New after the third review. IFC-017 and DL12 also leave
to every engine "who starts its process". [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) binds only where the engine starts it: the reads
are made before the start, and only the one that starts a process sets its environment.

| Option | What it is | What it costs |
| --- | --- | --- |
| **F1** | The engine starts the process of an item that states `environment` itself, whatever IFC-017 leaves it to choose for other items. | An engine that would hand the start of its processes to something else, such as a service manager, starts these itself. |
| F2 | An engine that does not start such a process itself rejects the item. | Whether a package can be used would depend on the engine's runtime profile, which the package cannot see; the same project would be valid under one engine and rejected under another. |
| F3 | Left to each engine. | Where something else starts the process, [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) binds nothing: the process starts without the value, or with whatever its starter gives it, outside every Conduit. |

**Recommendation: F1.** A `command` is "a program, then its arguments" (MAN-010), which the engine
can run itself; F1 only rules out handing the start of these items to something else. It keeps an
item valid under every engine, and [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) true of every process that serves such an item.

**Sub-question: how many instances one process serves.** IFC-017 leaves to every engine, at the
level may, "how many instances share" a process. A process started under `environment` holds
values read from one instance's configuration, through that instance's own Conduits, and one
environment cannot hold two instances' values under the same name. So the recommendation is one
process for each instance, for an item that states `environment`. That takes the choice of IFC-017
away for such an item, and the proposal says so: IFC-017 is in the table of promises that change.
The alternative, sharing a process among instances that name the same secret Resources, would set
a value read through one instance's Conduit in a process that serves another, whose own Conduit
might since have been closed.

**Sub-question: the form of `environment` (item 8b).** Changed after the fourth review in four
points.

- **Which Conduit.** An instance may hold two open Conduits to one secret, its declared one and one
  the root Interface opened on its own, with different filters where a package serves the secret.
  The read goes through the one open Conduit of the instance to that secret that lists
  `reveal-secret.v1`; where there is none, or more than one, it reports refused and the process is
  not started ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). The root Interface, which alone opens such Conduits ([SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), can always
  keep it to one. Usually it is the declared Conduit, which a template fills from the same input
  as the configuration place, but nothing checks that pair when documents are resolved.
- **The place.** A place that `secret-reference.v1` governs only as a branch of `anyOf` beside
  `{type: null}` may hold null, and an optional member may be absent. [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) therefore accepts only
  a place that the schema governs directly and requires, every member on the way to it included,
  and [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) does not start the process where a place still holds no reference.
- **The name.** Each key is the name of a variable, with the form POSIX gives the variables of its
  own utilities, which "consist solely of uppercase letters, digits, and the" underscore and "do
  not begin with a digit", and never hold `=`; lowercase letters are allowed as well, since POSIX
  reserves the names that contain them "for applications"
  ([POSIX, section 8.1](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html)).
  POSIX is not among the packet's sources; the page was read for this revision. A name the process
  would otherwise inherit, such as `PATH`, is replaced by the value.
- **The order within a setup (item 8g).** [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) reads only through an open Conduit, and the
  declared Conduit is opened by a step of the same setup (GRA-022). GRA-024 makes only a step that
  carries `conduit` depend on that step, and GRA-002 orders steps by dependency alone. A step with
  no `conduit`, or the start of a runtime, is dispatched to the instance's own implementation, and
  so may start the process before the Conduit is open: whether the setup then fails would depend
  on the order each engine picks (DL1). So, in a template that selects an item that states
  `environment`, the resolved setup graph lists each declared Conduit that lists an operation with
  a secret output among the dependencies of every step that would be dispatched to that item, as
  GRA-024 does for a step with `conduit` ([SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). The other choice, a rule that the author must
  write `dependsOn`, would leave the fault to be found at the start of the process.

**Sub-question: a read during the instance's retirement (item 8h).** New after the fourth review.
ACC-039 permits a request through an open Conduit whose consumer is retired "only if it belongs to
the retirement of that consumer". The read at process start goes through the instance's own
Conduit to the secret and serves no invocation, so ACC-039 refuses it. A plain server whose process
is not running when its instance is retired could then not be started for the steps of its
`retire` graph or the stops of its runtimes (TPL-009); the retirement fails, and the shutdown is
"never confirmed while the instance holds a runtime" (LIF-024) until someone reconciles. The
recommendation: a read at the start of a process that the engine starts in order to dispatch a step
of the instance's retirement, or the stop of one of its runtimes, belongs to that retirement, and
ACC-039 says so.

**Recommendation: B, with B2, one process for each instance, and F1. B stands, as ruling 39
accepted; the rest is new.** It is Compose's `environment` and Kubernetes's `secretKeyRef`, fitted
to how the protocol tells stdio servers to take credentials, and the read is still a request
through a Conduit of the instance, decided, recorded and closable like any other. It is the one
place where the proposal adds a mechanism rather than a rule. The research refines it in four ways:

- The process is not started where any read fails, as Kubernetes, Cloud Run and Vault Agent do and
  App Service does not. No implementation is reached, so a request that was to be dispatched to
  the process is not dispatched and reports unavailable ([SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), as OPS-005 has every request
  that is not dispatched report a reason; "a request that has an outcome was dispatched", in the
  words of the definitions. Changed after the fourth review: [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) gave such a request the
  outcome failed. The difference matters for a repeat: OPS-024 dispatches a new attempt after
  failed, and OPS-025 decides a request after a reason afresh.
- The process gets no variable that a secret Resource names as its store, apart from those set for
  it, and no value in its arguments (S18).
- The value is fixed for the life of the process (S19).
- A package that also publishes the MCP registry's `server.json` declares the same variables there
  with `isSecret: true`. That flag tells a client to handle the value securely; `environment` says
  where Conduit takes it from. The two need not be merged in the first version
  ([packet](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)).

Remote plain servers, reached by `url`, would need the engine to present an `Authorization` header
for them; that is left out (see below).

### S9. What a value is

**The question.** What the output `value` of `reveal-secret.v1` holds.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | A string, as the Model Context Protocol carries it in `structuredContent`. | A binary key must be stored in a text form, such as base64 or PEM, by the person who stores it. |
| B | A string or bytes, with a mark for the encoding. | A second shape for one case the first user rarely has. |

**What AWS and Kubernetes do.** AWS stores a string or bytes, but its own references accept only
the string: CloudFormation's "only supported value is `SecretString`", and its agent form "Must be
SecretString". Kubernetes stores bytes, in base64, up to 1 MiB, a limit set to protect the API
server's memory. Azure accepts and returns strings, up to 25 KB. Vault's KV store holds a map
(packet: [AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

**Recommendation: A. Stands.** A token, a password and a PEM key are all text. The output's schema
is `{type: string}`. [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), the proposed item of the implementation-defined list on how the engine
reads its own file and variable (S5), gains two points from the research: the largest value the
engine reads, as Kubernetes and Azure each state one, and whether a final newline is kept, which
Kubernetes's own instructions avoid with `echo -n`. What the engine does with a file that is not
valid UTF-8 is part of [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow). A package that reads bytes from its store states what it does with a
value that is not text. Selecting one key of a structured secret is S17.

### S10. What the guarantee covers

**The question.** What the promises would claim, so that no reader takes them for more.

**What AWS and Kubernetes do.** Both state narrow guarantees and name what they leave to the user.
AWS: "You are responsible for maintaining control over your content"; a resolved value "may show up
in the service whose resource it's being used in"; its agent protection is "best-effort".
Kubernetes: "your application must avoid logging the secret data in the clear or transmitting it to
an untrusted party", and whoever can create a Pod can read its Secrets (packet:
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret)). On EC2, AWS says that "all
applications on the instance share the same role and permissions", and that its local provider's
trust domain there is the role's
([packet](../../research/SECRETS.md#delivering-to-an-ec2-instance)). Kubernetes can afford a
credential that every container receives by default because it makes that credential weak: bound to
one Pod and one audience, an hour by default, and of low privilege
([packet](../../research/SECRETS.md#what-kubernetes-gives-every-container)). The credentials a
developer keeps in the engine's environment are usually not so (a judgment, which no source
states), which is why the limits below name them.

The kernels say who can read a process's environment, in place of 1Password's advice to "assume"
it ([packet](../../research/SECRETS.md#processes-of-the-same-user-on-linux-and-macos)). On Linux,
`/proc/<pid>/environ` is guarded by ptrace's check in its READ mode: a reader with the same user
and groups passes it where the process is dumpable and holds no capability the reader lacks, and
so does a privileged reader; Yama's default does not restrict that mode, and `hidepid` guards only
against other users. On macOS, the sysctl behind `ps -E` returns the environment to the same user
or root, unless the process's signature sets `CS_RESTRICT` while SIP is on, which the hardened
runtime does not. On Linux, `/proc/<pid>/environ` shows the environment as it was at start,
whatever the process does to it later, and memory is better guarded than the environment under
Yama's default.

**Recommendation. Stands, extended; extended again after the fourth review.** State the guarantee
as exactly this, in the specification's README or in the definition of a secret, and nothing
broader:

- A place that `secret-reference.v1` governs never holds a value, only a reference, so the value is
  never in configuration, in a record, in an inspection result, in an observation, in a graph or in
  a diagnostic; and a value that fails that schema is kept in no record ([SEC-22](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).
- The engine never records, retains or shows a secret output; it delivers it only to the instance
  that sent the request, or, for a read at process start, which no instance sends, into the
  environment of the process the engine starts for the instance whose Conduit it was read
  through; no tool for it is presented to an actor, and no step of a graph reads it.
- Every read is recorded, without the value, and no read is made whose record cannot be kept.
- An implementation that receives a value does not pass it on (an obligation at the Implementation
  level, which the engine cannot check).

And state what it does not cover:

- A secret written where no secret is marked. It is data.
- The root Interface's own choice of an implementation that it lets read a secret. Whoever wrote
  that implementation can recover the value.
- The root Interface's trust in a secret's locator. Opening a reading Conduit trusts the secret's
  configuration as well as the reader, and whoever may register the secret wrote that
  configuration: `file: /proc/self/environ` on Linux, a path [SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) accepts, returns the engine's
  whole environment, every other `environment` source and every variable [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) withholds among
  it. The opening is often implicit, since a declared Conduit opens at setup "only where the root
  Interface sent it" ([SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), and a Pool's entry can be retired and replaced from a template that
  `replace.v1` has changed (LIF-009), so a reader set up later finds another store behind the same
  entry name. The guidance is to inspect a secret, and the template behind a Pool's entry, before
  setting up a reader of it.
- What an implementation does with a value outside the engine. Restrictions in this version are
  mediated, not enforced (ACC-027); an isolation plugin is a future, optional part.
- A value that an implementation, a started process or an Agent returns or sends. From then on it
  is data, which the engine cannot tell from any other: it reaches the caller in a result, the
  records where the type's `records` keeps inputs or outputs (REC-002), and the retained result,
  whose content a purge does not touch (REC-004). A plain server that echoes its environment in a
  tool result does so, and an Agent's model can paste its credential into the `arguments` of a
  request. [SEC-10](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) binds only what the engine makes of a read's output, and [SEC-11](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) binds only an
  implementation that carries Conduit's keys; masking is left out of the first version, and would
  never be a guarantee.
- A value set in a process's environment, under S8 or for an Agent under S7. It is readable by that
  process, by what it starts, by its logs and debugging tools, by root, and by the other processes
  of the same operating-system user that the kernel admits, as above.
  A plain server is bound by no obligation of this specification.
- A restriction tied to a caller, for any read of a secret. A read is decided by the reader's own
  grant ([SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)); the caller's restrictions bind the invocation that the reader serves, and only
  an implementation that applies them there (IFC-019) carries them as far as the value's use. A
  value set at process start is read once, for no actor (S8), and the process then uses it for
  every invocation it serves, whoever the caller.
- A store's own credential, where the developer keeps it in a variable of the engine's environment
  (S16). The engine passes such a variable to every process it starts, an Agent's included (S18),
  as it passes any variable that no secret Resource names.
- Whatever a process the engine starts writes to its standard error. AWS tells authors of
  credential tools not to write secrets there
  ([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)); an engine that
  keeps what a started process writes there should treat it as possibly holding a value.
- An Agent's own credential, once in its process. An Agent whose model can run commands can read
  it.
- Which secret reaches an Agent. [SEC-6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), [SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) do not limit which secret an Agent's
  Conduit leads to, since the engine cannot tell a provider credential from any other secret. Only
  the root Interface's opening keeps another secret, such as a GitHub token, out of an Agent's
  process, and any secret so routed is within reach of a model that can run commands (item 4b).
- The engine's host. Whoever can run code as the engine's operating-system user, or read its
  memory, can read every value the engine delivers. That is AWS's "domain of trust" for its own
  local provider. Both kernels draw their line at the operating-system user, so keeping a value
  from the developer's other processes would mean running its reader as another user, which this
  version does not do.
- The engine's own sources, against code the engine starts. Every implementation the engine
  starts runs as the engine's user, an Agent's and a package's chosen by another actor included.
  It can read the engine's own environment, which holds every `environment` source, on Linux where
  the engine is dumpable and on macOS in every case, and open every `file` source, whether or not
  any Conduit to those secrets was opened. So an actor that may register and set up a Resource, or
  install a package, can arrange such a read with no Conduit (S4).
- Withholding a variable. [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) withholds the variable of every secret that is not retired and
  that `engine` serves, and `secret.v1` states no `schema`, so any actor that may register a
  Resource into a Pool can register a secret with `{environment: PATH}` and so withhold `PATH`,
  or `HOME`, from every process the engine starts, which S16 needs passed on. That breaks
  processes; it reveals nothing. Inspecting the secrets shows which variables are withheld.
- Moving readers to a new secret. A reader's configuration and its declared Conduit keep naming a
  retired or superseded secret (OBJ-010, LIF-009), so the reader must be replaced, unless its type
  has a control for that place and the root Interface opens a Conduit to the new secret for it
  (S6, S11).
- A copy that an implementation in a package keeps (S12), and any copy already read, after the
  Conduit is closed.
- Later, a delegated right to open reading Conduits. Whoever holds it can arrange a read for an
  implementation they control, so delegating it is, in effect, delegating the read (S4).

### S11. Version or stage

**The question.** New. Whether a secret's locator may name a version or a stage of the value, or
always reads the latest.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | Always the current version. No locator names a version. | No rollback to a known value, and no testing of a pending one. Simple. |
| **B** | A locator may name a version or a stage, in the store's own form, and names the current one by default. | A pinned secret does not follow rotation; changing it means a new secret Resource. |
| C | A locator must pin a version. | Every rotation needs a change of configuration. Only possible for stores that have versions. |

**What AWS and Kubernetes do.** AWS reads AWSCURRENT unless a stage or a version ID is named,
recommends references with no version, and keeps versioned ones "for specific scenarios like
testing or rollback situations"; its CSI provider calls pinning a version ID "Not recommended".
Kubernetes has no versions: a Secret changes in place, and an immutable one changes only by being
deleted and made again. Google recommends the opposite of AWS: "Reference secrets by their version
number rather than using the latest alias", so that a change can be rolled back, and notes that
`latest` is not strongly consistent. Azure reads the latest unless a version is named. Parameter
Store reads the latest version unless a version number or a label follows the name after a colon;
a version that does not exist is an error, "rather than falling back to the latest or default
version"; it keeps 100 versions, so a pinned one can be dropped; and CloudFormation's rollback
fails where a pinned version is gone. ASCP and CloudFormation both default to the latest (packet:
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

**Recommendation: B.** `secret.v1` fixes no locator form. Each store's template states its own, and
its default names the current version, as AWS and Azure do. What "current" means is each store's,
and its template says so: AWSCURRENT in Secrets Manager, the latest version in Parameter Store. A
pinned version is allowed for rollback, as Google recommends, and its cost is stated: it does not
follow rotation, and since `secret.v1` has no control to change its locator and a consumer keeps
the resolved form it was made with (OBJ-010), moving to another version means a new secret
Resource, as moving to another immutable Kubernetes Secret does. The readers do not follow it
(added after the fourth review): each reader whose configuration or declared Conduit names the old
secret Resource keeps naming it (OBJ-010), and a replacement of a declared Conduit whose target is
retired reports invalid (LIF-009). So each plain server's instance and each Agent that read the old
secret through `environment` must be retired and made again before the engine can start its
process, unless its type has a control for that place and the root Interface opens a Conduit to
the new secret for it (S6, S10). A pinned version can also stop
existing in the store, as Parameter Store's cap shows. One rule is adopted from Parameter Store: a
locator that names a version or a stage that does not exist fails the read, and the implementation
never falls back to another version. That is [SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s rule against a fallback to another source,
applied to versions. The engine's own sources, a file and a variable, have no versions.

### S12. Freshness and caching

**The question.** New. Whether every implementation of `reveal-secret.v1` must read its store at
each request ([SEC-13](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) of the first draft), or may serve a read from a copy.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | Every implementation reads its store at each request. | Every read of a cloud store is a billed, rate-limited call. Stores that are eventually consistent cannot promise that a second read sees a change. |
| **B** | The engine keeps no copy of a value, and its own implementation reads its file or variable at each request. An implementation in a package may serve a read from a copy it keeps. | A package's reader may get a value up to its cache's age. |
| C | No rule. | The engine itself might keep values between requests, which S4 rules out. |

**What AWS and Kubernetes do.** AWS recommends caching on the client, since "Caching secrets
improves speed and reduces your costs", accepts staleness (300 seconds by default in its local
provider and its Lambda extension), and offers `refreshNow` to bypass the cache. Its store is
eventually consistent, and `GetSecretValue` is limited to 10,000 requests per second per Region.
Kubernetes serves volumes from a kubelet cache, and the CSI driver and the External Secrets
Operator poll (packet: [AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[ecosystem](../../research/SECRETS.md#the-kubernetes-ecosystem)).

**Recommendation: B.** A copy inside a package's implementation does not weaken the access
guarantee: each read is still a request, decided against the Conduit and recorded, and closing the
Conduit still stops reads. It weakens only freshness, which is the package's to state. The engine
itself keeps nothing, so the record never misses a read the engine served, which is the gap AWS
documents for its own local cache. The observable of [SEC-13](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) becomes one that every store can meet:
with the engine's own file, a change between two reads is seen at the second. The engine's own
variable keeps the value it had when the engine started, as a value in an environment does in
every source read ([packet](../../research/SECRETS.md#patterns-across-the-systems), pattern 6); a
new value there is seen once the engine is started again. A request
input to force a fresh read, as AWS's `refreshNow`, is left out of the first version; the guidance
of S6, to read again after a refusal, assumes a package that caches keeps its cache short.

### S13. Records of reads

**The question.** New. Whether each read is recorded, how without the value, and what happens where
the record cannot be kept.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | Each read is recorded as any request is (REC-001, REC-002), and the record never holds the value ([SEC-8](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)). A read whose record cannot be kept is not dispatched. | One new rule, for operations that have a secret output only. |
| B | As A, and the record also holds a keyed hash of the value, so an owner can check whether a leaked value was read. | A hashing key to manage and protect; a low-entropy password's hash can be guessed once the key leaks. |
| C | As A, without the rule on a record that cannot be kept. | A value could be delivered with no trace of who read it. |

**What AWS and Kubernetes do.** CloudTrail logs every `GetSecretValue`, with the caller, and never
the value; AWS tells clients to keep the value out of their own logs too. Kubernetes records Secret
access at the Metadata level ("requesting user, timestamp, resource, verb"), but only where the
operator configures a policy; GCP leaves its data-access logs off by default. Azure Key Vault's
record has no field for the value. Vault writes a keyed hash of values, offers an endpoint to hash
a known value, and "refuses to service the corresponding API request" when it cannot write its
audit log (packet: [AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

**Recommendation: A.** Conduit already records every request by default, which is stronger than
Kubernetes's and GCP's defaults and the same as AWS's. The record of an onward read names the
caller, the sending instance, the Conduit, the secret, the operation, the time and how it ended. A
read at process start under S8 has neither a caller nor a sending instance: its record names the
instance whose process is started, whose Conduit the read went through, and says that the engine
made the read for no actor at that start, as OBS-007's stop is recorded. A
store that reports which version it served, as Parameter Store's `GetParameter` does
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)), would let the record
name it, never the value; that is left out of the first version. Refusing a read whose record cannot
be kept is Vault's rule, kept to secret outputs: nothing else in this version needs it, and DL10
leaves storage exhaustion otherwise to each engine. The keyed hash of B is left out of the first
version.

### S14. Encryption of what the engine keeps

**The question.** New. Whether the specification promises encryption at rest for anything the
engine keeps about secrets.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | No promise. The engine keeps no value; a secret's configuration, its records and its references hold nothing secret. | Nothing new. How an engine protects its stored data in general stays its own. |
| B | The engine encrypts its records and configuration at rest. | Key management in every engine, for data that holds no secret. |
| C | An engine-owned encrypted store (S5, option A). | Rejected in S5. |

**What AWS and Kubernetes do.** AWS encrypts the value, and keeps the name, the description, the
rotation settings and the tags in the clear; Parameter Store encrypts only the value of a
SecureString. Kubernetes stores values unencrypted by default, and its encryption at rest is a
configuration with keys to manage; a KMS key's `key_id` is public and "must not contain any private
data". Azure warns not to put sensitive information in identifiers (packet:
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

**Recommendation: A.** Because the value stays in its store (S5) and the engine keeps no copy
(S12), there is nothing secret for the engine to encrypt. Metadata in the clear is the industry's
design, and the rule that a locator holds nothing secret ([SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow), S2) is what makes that safe.
Parameter Store shows it again: restricting decryption "doesn't hide the parameter's name,
description, or other metadata", and a parameter's ARN appears in CloudTrail in plaintext as the
encryption context ([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)).
The ciphertext kept beside configuration, which S5 advises against and no implementation reads,
would have brought key management back had it been read.

### S15. Issued credentials and stored secrets

**The question.** New. Whether the first version also has credentials issued per identity,
short-lived, as an alternative to a stored secret.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | Stored secrets only. `secret.v1` is described as the fallback for a credential that an identity cannot replace, and the generic output mark (S4) leaves room for an operation that issues short-lived credentials later. | Long-lived keys stay long-lived until their owner rotates them. |
| B | An operation that issues a short-lived credential to a Resource, as STS or Vault's dynamic secrets do, in the first version. | A lease lifecycle (issue, renew, revoke) to specify, for outside services that mostly issue static keys. |
| C | The engine issues each instance a credential for its own identity. | DL13's business, and circular if kept as a secret (S7, option A). |

**What AWS and Kubernetes do.** Both prefer identity. AWS: "The most secure credential is one that
you do not have to store, manage, or handle", and roles, STS, EKS service-account roles and Pod
Identity replace stored keys for AWS's own APIs
([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)). Kubernetes moved its own tokens
to TokenRequest, short-lived and bound to a Pod
([packet](../../research/SECRETS.md#the-kubernetes-secret)). GCP's Workload Identity and Azure's
managed identities do the same ([packet](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).
On a developer's machine AWS advises federation and temporary credentials over long-term keys, and
IAM Roles Anywhere gives machines outside AWS temporary role credentials against a private key that
can stay in a keychain or a TPM
([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). But each keeps a
secret store for what identity cannot replace, such as a token for a service outside the platform.

**Recommendation: A.** The first user's credentials are mostly static keys issued by outside
services: a code host's token, a model provider's key. The specification's README should say that
`secret.v1` is for those, and that an identity is better where the outside service accepts one.
When short-lived credentials come in, Terraform's ephemeral resources (opened, renewed where
needed, closed) and Vault's leases are the shapes to adopt (packet:
[tools](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling),
[Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

### S16. A store's own credential

**The question.** New. How an implementation in a package, such as one that reads AWS Secrets
Manager, gets the credential it needs to reach its store.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | The credentials that the store's own client finds by default in the environment the engine gives its process: on a developer's machine, the client's own files under the home directory and the variables that point to them; in a cloud, the platform's identity. | Outside every Conduit: not inspected, narrowed, recorded or closed by Conduit. Stated as such. |
| B | A key in a secret Resource that the engine's own implementation reads, set in the package's process environment at start under S8. | As drafted in the revision it cannot be declared, and it costs more than it seemed: see below. It keeps a long-term key, which AWS advises against for development. |
| C | A secret read by another package's implementation. | A chain of stores, each needing the next; no end to it. |

**Why B is not as cheap as the revision said.** The revision said "[SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) already serve
it". They do not. [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) has `environment` point at a place that `secret-reference.v1` governs "in
the instance's type", and the instance a store's package serves is a secret Resource, whose type
`secret.v1` states no `schema`; such a type "puts no rule on the configuration of its instances"
(CTR-004). So `environment` has no place in a secret Resource's configuration to point at.
Making B work would need `secret.v1` to gain a schema with a place for its store's own credential,
or [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) to accept a place that a template's own input governs; a Conduit from every secret
Resource the package serves to the key, each opened by the root Interface; and, under [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)'s one
process for each instance, one process of the package for each secret Resource it serves.

**What AWS and Kubernetes do.** Kubernetes's KMS plugin keeps its own credentials for the remote
KMS, "stored and managed by the KMS plugin independently". AWS's own clients find credentials from
the platform: a role whose temporary credentials are delivered to the compute resource. Vault's AWS
method needs no credential issued to the workload first, and its "Trusted Orchestrator" starts an
application and gives it the means to authenticate (packet:
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)). On EC2 that
identity is the instance profile, which a script that calls AWS "must use", and "all applications
on the instance share the same role and permissions"
([packet](../../research/SECRETS.md#delivering-to-an-ec2-instance)): a package's store client
there finds the role with no key in any configuration, and so does every other process the engine
starts.

On a developer's machine, which is the first user's case, neither the AWS tools nor `kubectl` get
credentials from a service; they find them by convention
([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)). The AWS SDKs
search an ordered chain and stop at the first source that works: the access-key variables, then the
profile in the plaintext files `~/.aws/config` and `~/.aws/credentials`, found through `HOME` and
chosen by `AWS_PROFILE`. That profile can hold static keys, assume a role, use IAM Identity Center,
whose sign-in with `aws sso login` writes a cache the SDKs refresh until the session ends, or name a
`credential_process` command. Container credentials and IMDS come last, and a laptop normally has
neither. Machines outside AWS get role credentials through IAM Roles Anywhere, whose helper signs
with a key that can stay in the macOS Keychain or a TPM. For development AWS advises federation and
temporary credentials over long-term keys, and where keys are used it names the shared credentials
file as their place. `kubectl` reads `KUBECONFIG` or `~/.kube/config`; on EKS it authenticates with
`aws eks get-token`, which uses the default AWS chain unless a role is named, so a Kubernetes
credential ends in the same AWS chain. AWS's own local secret provider gets its credentials the same
way, from the SDK's chain. In the [second round's
survey](../../research/SECRETS.md#delivering-a-credential-to-an-existing-mcp-server), AWS's MCP
server and the Kubernetes MCP server take their credentials just so: a named profile or boto3's
chain, and a kubeconfig. SOPS finds its age key in a file under the user's configuration directory,
or in `SOPS_AGE_KEY` or `SOPS_AGE_KEY_FILE`
([packet](../../research/SECRETS.md#ciphertext-kept-beside-configuration)).

**Recommendation: A for the first version; B left out; never C. Changes:** the revision recommended
B where a store needs a stored key. Under A a package uses its store's own client unchanged, as an
AWS package uses the AWS SDK's chain, and so gets Identity Center, `credential_process`, Roles
Anywhere and the files with no work. To make A concrete, the proposal states what it depends on, on
a developer's machine:

- the package process's environment: `HOME`, `PATH`, and the client's pointers, such as
  `AWS_PROFILE`, `AWS_CONFIG_FILE`, `AWS_SHARED_CREDENTIALS_FILE` or `KUBECONFIG`, and the
  client's key variables where the developer keeps the credential there;
- the client's own files and caches under the home directory, such as `~/.aws/config`,
  `~/.aws/credentials` and the cache that `aws sso login` writes, or `~/.kube/config`;
- whatever program those files name, such as a `credential_process` command or an exec plugin.

So A works only where the engine passes those on to the package's process. S18's option B does:
it withholds only the variables a secret Resource names. Its option C, a fixed minimal set, would
break A, and DL12 should say that an engine that gives a started process less than its own
environment may leave a store's client without the files and pointers it looks for. That is the
one promise-level consequence of A.

Guidance, not promises, follows from the research:

- **Narrow by profile.** Under A a package would otherwise get the developer's default identity,
  which is often broad. A package's configuration can name a profile of its own, through a pointer
  such as `AWS_PROFILE`, which holds no secret. That profile can be a narrow Identity Center
  permission set, an assumed role, or Roles Anywhere through `credential_process`. A Parameter
  Store package, for instance, needs `ssm:GetParameter` on named parameters only, as AWS's advice
  to grant narrow access suggests, and `kms:Decrypt` on a customer managed key, which a condition
  on the parameter's ARN can limit to those parameters
  ([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). The store's own
  policy is then a second gate behind the Conduit.
- **A file over a variable.** A store's own credential kept in a variable of the engine's
  environment, such as `AWS_ACCESS_KEY_ID` or `SOPS_AGE_KEY`, is passed to every process the engine
  starts, an Agent's included, because no secret Resource names it (S18, S10). Kept in the client's
  own file, and pointed to by a variable that holds no secret, it is not passed to them, though any
  process of the same user can still open the file.
- **A secret in another account.** An AWS package whose credential is in one account and whose
  secret is in another reads it only where both the secret's resource policy and the identity's
  own policy allow it, and where the secret is encrypted with a customer managed key whose key
  policy allows `kms:Decrypt`, since "you can't use the AWS managed key (aws/secretsmanager) for
  cross-account access" ([packet](../../research/SECRETS.md#a-list-of-readers-on-the-secret)). The
  package's documentation should say so. Within one account, a resource policy on the secret can
  grant the package's identity a read by itself (S4).
- **A session that has ended.** Identity Center needs a person to sign in, and a process the engine
  starts has no terminal; once the session ends, "Any code that creates a new client will fail
  authentication". The read then has the outcome failed, with no fallback ([SEC-12](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), and the
  package's message should tell the developer to sign in again. Kubernetes's `interactiveMode`
  shows how a client declares that a credential needs a person.

B would be the exception for a store whose client finds no credential by default, and offers no
identity of its own. When it is needed, its costs above are the ones to rule on.

### S17. One key of a structured secret

**The question.** New. Whether a locator may select one key of a secret whose value is a JSON
object, such as a username and a password stored together.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | A store's locator may name one key, in the store's own form. One stored secret then gives several secret Resources, each one string. | Nothing in the core: each store's template states its form. |
| B | A secret's value is a map, and a reference names a key, as Kubernetes's `secretKeyRef` does. | Changes S9's string to a map, and adds a key to `secret-reference.v1`. |
| C | No selection. | A JSON secret in AWS or Vault cannot be used one key at a time. |

**What AWS and Kubernetes do.** AWS recommends JSON key-value pairs for values that belong
together, and its references take a `json-key`; its CSI provider maps keys to files with
`jmesPath`. Kubernetes's `secretKeyRef` names a Secret and a key in it. The External Secrets
Operator's `property` selects one key, and Vault's KV store holds a map (packet:
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[ecosystem](../../research/SECRETS.md#the-kubernetes-ecosystem),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

Parameter Store's `GetParameter` documents two selectors after a colon, a version and a label, and
none for a key; the same colon carries a Secrets Manager stage through its pass-through, as in
`s1-secret:AWSCURRENT`
([packet](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store)). SOPS selects one
value with `--extract` and a path in its own syntax
([packet](../../research/SECRETS.md#ciphertext-kept-beside-configuration)).

**Recommendation: A.** It keeps S9's one string and invents no selector syntax: the AWS package
uses `json-key`, a Vault package names a key of its map, a SOPS package uses SOPS's extract path,
each in its store's own terms. The second round strengthens the case against a syntax of Conduit's
own: a colon after a name already means a version or a label in Parameter Store and a stage in its
pass-through, so a cross-store `:key` would collide with AWS's own form. A Parameter Store package
states that it has no key selector; related values are separate parameters, each its own secret
Resource. The engine's own file and variable have no selector in the first version.

### S18. What a started process inherits

**The question.** New. What environment a process that the engine starts gets, beyond the variables
set for it under `environment`.

| Option | What it is | What it costs |
| --- | --- | --- |
| A | The engine's whole environment, as the first draft left it. | Every process the engine starts receives every secret the engine reads from its own environment. |
| **B** | The engine's environment without any variable that a secret Resource's configuration names as its store, apart from those set for the process under `environment`; and never a value in its arguments. | One rule; what else a process inherits stays DL12. |
| C | Only the declared variables and a fixed minimal set. | Breaks servers that need `PATH`, `HOME` or a profile, and fixes a set the specification has no reason to choose. |

**What AWS and Kubernetes do.** AWS "strongly recommend[s]" blocking containers from the host's own
instance credentials, so that a workload does not inherit what it was not given. Docker Compose
grants each service its secrets explicitly. Claude Code runs a helper that a repository supplies
"without the credential variables from your environment". AWS warns that command-line parameters
can be seen by "other utilities that work in the background", although its own agent wrapper puts
values in a child's arguments (packet:
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[tools](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)). MongoDB's MCP server
warns that "Command-line arguments are visible in process listings"
([packet](../../research/SECRETS.md#delivering-a-credential-to-an-existing-mcp-server)). On a
developer's machine a store's client finds its credential through `HOME`, `PATH` and pointers such
as `AWS_PROFILE` and `KUBECONFIG`
([packet](../../research/SECRETS.md#aws-and-kubernetes-on-a-developers-machine)), which C would
withhold.

Kubernetes's own defaults are the closest case, and the research of the third round adds them
([packet](../../research/SECRETS.md#what-kubernetes-gives-every-container)). Every container gets
a ServiceAccount token it did not ask for, and variables for every Service in its namespace. The
security checklist says "Service account tokens should not be mounted into pods that do not
require them", and the RBAC good practices say "Kubernetes defaults to providing access which may
not be required in every cluster". The defaults survive because "Changing defaults is a breaking
change", in a maintainer's words. When service links are turned off, Kubernetes still sets
`KUBERNETES_SERVICE_HOST` and its siblings, "since a lot of code depends on it", which is B's
reason for passing `HOME`, `PATH` and a client's pointers on rather than choosing C. A container's
own environment is close to C, but its image supplies `PATH` and the like; a process the engine
starts on a developer's machine has no image, and the engine's environment plays that part (a
judgment, which no source states). Conduit has no installed base, so it can start from the
hardened posture, withholding its secret variables by default, rather than adopt the permissive
default that Kubernetes keeps for compatibility (a judgment too). If a switch to give less is ever
wanted, Kubernetes's shape is the one to adopt: a boolean on the identity that each workload may
override, the workload winning.

The model providers add a reason to unset rather than blank. Their orders of precedence let a
leftover credential variable shadow the intended source: Anthropic's SDK says "A variable that is
set to an empty string can still occupy its slot in the credential precedence chain. Unset unused
credential variables rather than blanking them", and Codex's federation variables, once present,
take precedence over its keys, tokens and stored logins
([packet](../../research/SECRETS.md#an-agents-own-model-credential)). B unsets a withheld
variable; it never sets one to an empty string.

**Recommendation: B.** The engine's secret variables are its own sources (S5), not something to
pass on. A variable that a secret Resource names is withheld from every process the engine starts,
so a plain server sees only the secrets its item declares. The `command` of an implementation item
is a list of literal strings (MAN-010), so the engine never puts a value into a program's
arguments; the rule says so. Everything else passes as DL12 leaves it, and S16 depends on that: a
package's store client needs `HOME`, `PATH` and its pointers. An engine may give less under DL12,
at the cost that S16 states.

What B does not withhold is a store's own credential kept in a variable that no secret Resource
names, such as `AWS_ACCESS_KEY_ID` or SOPS's `SOPS_AGE_KEY`: it reaches every process the engine
starts, an Agent's included. The engine could withhold it only by knowing every store client's
variables, a list that no standard gives and that the proposal would have to invent. So the
proposal states the limit (S10) and advises keeping such a credential in the client's own file,
pointed to by a variable that holds no secret (S16). The same holds for an Agent's provider key
left in the engine's environment: if no secret Resource names it, it reaches every process the
engine starts, and in an Agent's process it can shadow the credential that its item sets.

**Which secret Resources count.** New after the fourth review. The third revision did not say, and
registering a Resource needs no reading Conduit, while `secret.v1` states no `schema`. The
recommendation counts every secret Resource that is not retired and whose implementation of
`reveal-secret.v1` is `engine`, whether or not it is ready, so that a variable is withheld from the
moment a secret names it. The cost is that any actor that may register a Resource into a Pool can
register `{environment: PATH}`, or `HOME`, and so withhold that variable from every process the
engine starts, which S16 needs passed on; S10 names that limit. The other choice was to count only
secrets to which an open Conduit lists `reveal-secret.v1`, which only the root Interface opens
([SEC-7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)), so that no other registrant could withhold anything; but then a variable that a secret
names reaches every process started before its first reader is set up. This proposal puts the
value's secrecy before the processes' availability, and lists the choice among the least certain.

### S19. A process environment on rotation

**The question.** New. What happens to a value set in a process's environment at start when the
value changes in its store.

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | Nothing. The value is fixed for the life of the process; a process that the engine starts after the change gets the new value. The engine does not watch the store. | A long-running server uses the old value until the engine starts it again. |
| B | The engine restarts the process when the value changes, as Vault Agent's supervisor does by default. | The engine must poll the store, adding reads to the records, and a restart interrupts the invocations in progress. |
| C | The engine reads again on a timer and restarts on a difference. | B's cost on a schedule. |

**What AWS and Kubernetes do.** Neither updates a running process's environment. ECS needs a new
task or a forced deployment; Kubernetes leaves restarts on change to "third party solutions"; the
CSI driver says "The pod needs to be restarted"; Cloud Run resolves at instance start; App Service
fetches again every 24 hours, and when a change of configuration restarts the app. Vault Agent
restarts the process (packet:
[AWS delivery](../../research/SECRETS.md#how-aws-delivers-to-workloads),
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[ecosystem](../../research/SECRETS.md#the-kubernetes-ecosystem),
[GCP, Azure and Vault](../../research/SECRETS.md#gcp-azure-and-hashicorp-vault)).

**Recommendation: A.** The engine cannot see a change in a store without reading it, and an
unrequested read is not a request any actor made. A new value reaches the instance when the engine
next starts its process, which DL12 leaves to each engine; retiring the instance and making another
also does it. Restarting on change, driven by the store's own change events (AWS's rotation events
in CloudTrail, GCP's Pub/Sub notifications), is left for later.

### S20. Names

**The question.** Whether the proposed names become final. Point 1 of ruling 39 put them with the
core.

**Recommendation.** `secret.v1` (Type), `secret-reveal.v1` (Capability), `reveal-secret.v1`
(Operation), `secret-reference.v1` (Schema), the output mark `secret`, the key `environment` of an
implementation item, and the number [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) for the new item of the implementation-defined list.
They follow NAM-004 and the core package's naming. The research gives no reason to change them:
Kubernetes's `Secret`, Compose's `secrets` and AWS's `GetSecretValue` all use the plain word, and
Compose's `environment` names the same source (packet:
[Kubernetes](../../research/SECRETS.md#the-kubernetes-secret),
[AWS](../../research/SECRETS.md#aws-secrets-manager-and-parameter-store),
[tools](../../research/SECRETS.md#configuration-tools-mcp-and-agent-tooling)).

### S21. A file beside the variable

**The question.** New after the review. Whether the engine may also deliver a value at process
start as a file, for a server that can read one, and not only as a variable. The research's
strongest guidance on delivery points that way: AWS's EKS best practices say "Use volume mounts
instead of environment variables", because "The values of environment variables can
unintentionally appear in logs" ([packet](../../research/SECRETS.md#how-aws-delivers-to-workloads)),
and Kubernetes's security checklist says "the environment variable method might be more prone to
leakage due to crash dumps in logs and the non-confidential nature of environment variable in
Linux, as opposed to the permission mechanism on files"
([packet](../../research/SECRETS.md#what-kubernetes-gives-every-container)).

| Option | What it is | What it costs |
| --- | --- | --- |
| **A** | Only `environment` in the first version. The file form is left out, and the shape to adopt is recorded. | The value is in an environment that what the server starts inherits. A server that takes only a whole credential file (a kubeconfig, AWS's shared files) is reached only through the developer's own files, outside every Conduit, as S16 describes. |
| B | A second map beside `environment`, such as `files`, validated as [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) validates `environment`. For each entry the engine writes the value to a new file, readable only by the process's operating-system user, in a directory made for that one process, and sets the variable to the file's absolute path. The file is removed when the process ends, and one left by an engine that stopped first is removed when the engine next starts. | The engine writes a value to a file, which S5 otherwise avoids. Where the system offers an unprivileged process no memory-backed file system, the file is on disk while the process runs. On Linux, `memfd_create` gives a file that "lives in RAM", passed to the child as a descriptor, though like other memory it may be swapped and a process of the same user reaches it through `/proc/<pid>/fd`. On macOS, Apple documents a RAM disk made with `hdiutil`, but whether an ordinary user can mount one was not established, and it would be machinery of the engine's own. [DL16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) or DL12 then states the file's place, its mode, its cleanup and its final newline. |

**What AWS, Kubernetes and the MCP tools do.** Kubernetes offers both forms. It mounts secrets on
tmpfs, which, in AWS's words, "are automatically removed from the node when the pod is deleted".
Compose bind-mounts a single file from the host at `/run/secrets/<name>`. Swarm mounts secrets in
memory, but on Windows, which has no RAM disk driver, writes them to the container's disk "in clear
text" and removes them when the container stops. systemd, the closest model of a supervisor that
starts processes, gives each service a directory of credential files named by
`$CREDENTIALS_DIRECTORY`, "restricted to the service's user", "not propagated down the process tree"
and "immutable during the service runtime". Among MCP servers, Brave's, Grafana's and a GitLab
server accept a `_FILE` variable beside the variable; Grafana reads its file "fresh on every
request", so a rotated token needs no restart. GitHub's server, this proposal's example, takes its
token only as a variable. The protocol says "the environment" and does not define it further. The
pages read for the two MCP runners surveyed, Docker's catalog and ToolHive, show delivery by
variable; whether Docker's MCP gateway ever mounts a secret as a file was not established. The MCP
registry can describe a path variable with `format: filepath`, apart from `isSecret`
([packet](../../research/SECRETS.md#delivering-a-credential-to-an-existing-mcp-server)).

On the first user's machine, the kernels decide who can read each form
([packet](../../research/SECRETS.md#processes-of-the-same-user-on-linux-and-macos)). On Linux, a
file that only its owner may read is open to every process of that user that can search its
directory, while the environment of a process is open to the processes of the same user and groups
where the process is dumpable; both are open to a privileged process. Read together, a file there
keeps the value from no process of the user that the environment keeps it from, and is open to a
few more. On macOS the environment is open to every process of the same user and to root, unless
the process's signature restricts it; the packet did not examine files there. What a file does
change is different: it is not inherited, and it can be replaced or removed, while on Linux
`/proc/<pid>/environ` shows the value set at start for the life of the process.

**Recommendation: A.** The variable is the one form that every server checked that takes a single
token accepts, that the protocol names, and that the pages read for the MCP runners show them
using; each of those servers that reads a file also reads the variable. As above, on Linux a file
only its owner may read keeps the value from no process of the same user that the environment keeps
it from, and on macOS the environment already reaches every process of the user, so the first user
gains no protection from the user's other processes, and the file may sit on disk. What B gains is
real: the value is not inherited by what the server starts, though a child of the same user can
still open the file whose path it inherits; it stays out of dumps of the environment and out of
logs, which is the reason EKS, systemd and Kubernetes's checklist give; it can be removed or
replaced, and so would let a server that reads its file again, as Grafana's does, see a rotated
value without a restart (S19); and it reaches servers that need a whole credential file. When it
comes, systemd's directory for each process and a variable holding the path is the shape to adopt,
the `_FILE` convention is what servers read, and `format: filepath` is how a package's
`server.json` describes the variable.

## Left out of the first version

Each of these was considered and is proposed for later, not rejected.

- **Writing a secret through Conduit.** No operation sets a value. The person writes it with the
  store's own tools. A password manager Resource that takes values would need the mark on inputs,
  and exceptions for the digest of contents (REC-004, OPS-009).
- **Taking a user's credential at run time.** The Model Context Protocol's URL-mode elicitation is
  the standard: a server that needs a user's credential to a third party sends the user to a URL,
  and the credential never crosses the client. A later version could say how the engine relays such
  an elicitation from a Resource to the root Interface. A form-mode elicitation for a credential is
  never a sanctioned path.
- **An engine-owned encrypted store** (S5, S14).
- **The keychain in the core package.** An ordinary package, after experiment E10.
- **A command as a store in the core package.** An ordinary package (S5). If it comes,
  Kubernetes's ExecCredential v1 is the nearest format, and a list of allowed commands kept by the
  engine outside every document, as kubectl's `kuberc` keeps one, is the precedent.
- **A stored key for a package's own store** (S16, option B), which needs a place in a secret
  Resource's configuration for it, a Conduit from each secret Resource to the key, and a process
  for each secret Resource.
- **A file beside the variable** at process start (S21), in a directory made for one process and
  named by a variable, as systemd does, for servers that read a `_FILE` variable or a whole
  credential file.
- **Naming the version served** in the record of a read (S13).
- **Dynamic secrets and leases**, as in Vault and STS: an operation that issues a short-lived
  credential to a Resource and can revoke it at the issuer (S15). The generic output mark leaves
  room.
- **Delivery the engine cannot read**, by wrapping the value for its reader, as Vault's response
  wrapping does (S3).
- **Logins, refresh and OAuth flows**, and token exchange under RFC 8693. A login Resource type in
  a package, later.
- **A credential for a remote plain server**, which would need the engine to present an
  `Authorization` header as an OAuth client. The MCP registry's remote `headers`, with `isSecret`,
  would give the shape.
- **Workload identity**, with the engine issuing short-lived credentials to each instance, as
  SPIFFE does and as EKS Pod Identity's pointer and token file show (S7). DL13 stays as it is.
- **Refreshing an Agent's short-lived provider credential** (S7, item 7c): a file that the engine
  keeps fresh, named by a variable that holds no secret, as `ANTHROPIC_IDENTITY_TOKEN_FILE` is read.
  Anthropic accepts an issuer that is not on the public internet, with its keys uploaded, so an
  engine could in principle issue each Agent's identity token itself; that is an inference from
  the documentation, not a tested design, and it would need a signing key, rotation within the
  issuer's lifetime limit, and a fresh token for each exchange.
- **A check of the pairing at resolution**: that an instance whose implementation item states
  `environment` has a declared Conduit to each secret its configuration names there. [SEC-15](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) orders
  a setup so that the declared Conduits to secrets open first (item 8g), but checks no pair;
  [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) finds a missing Conduit, or a second one, only at the start of the process.
- **A statement on the secret's side**, for teams: an owner of a Pool of secrets bounding who may
  arrange reads of them, on AWS's rule across accounts, as a bound and never as a grant (S4).
- **Delegating the right to open a reading Conduit** through a bound, for teams (S4), on
  Kubernetes's rule against escalation and an explicit right to arrange, as `bind` and PassRole
  are, which ACC-006 and ACC-030 already give a bound.
- **A keyed hash of a revealed value in its record** (S13).
- **A request input that forces a fresh read**, as AWS's `refreshNow` (S12).
- **Restarting a process when its secret changes**, and observations of changes made in a store,
  from the store's own events (S19).
- **Masking**: the engine replacing a known secret wherever it appears in a result, as GitHub
  Actions masks logs and `op run` conceals output. Useful, never a guarantee, and an engine may
  still do it. Without it, a value that an implementation or an Agent returns or sends is data
  from then on (S10).
- **Secret values in graphs** ([SEC-5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and [SEC-21](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) reject them), and secret inputs of operations.
- **Changing a secret's locator** by a control (S11).
- **Binary values** (S9).
- **A credential composed into a configuration file**, as the External Secrets Operator's templates
  do: several values written into one file. A credential that is itself a whole file, such as a
  kubeconfig, would be one string under S9 and come with the file form of S21.
- **Detecting a secret written where none is marked**, such as by its entropy.
- **Isolation**: keeping an implementation from sending a value elsewhere. Enforcement is a future,
  optional part.

## Where the recommendation is least certain

1. **An Agent's own credential (S4, S7).** Point 4 stands, but the research shows that a value in
   an Agent's process environment is within its model's reach where the model can run commands. If
   the owner reads "never handed over" as covering an Agent's model, point 4 needs a different
   mechanism, such as a provider credential the Agent's process cannot print, and no such standard
   exists. The providers' narrow forms reduce what a leaked value is worth, as a token that "can
   only make model requests" does, and so do short lifetimes; none makes the credential unreadable
   by the Agent's process, and a helper command or a profile file still puts a usable token, or a
   refresh token, within its reach. And a value set at start does not serve a federated token in
   a process that outlives it, so point 4's mechanism serves mainly the longer-lived forms (item
   7c). Nor can any promise keep the Conduit to the Agent's own credential: whatever secret the
   root Interface routes to an Agent is within its model's reach (item 4b).
2. **The engine sees each value in transit (S3).** Acceptable for one developer on one machine, and
   the norm in AWS and Kubernetes. A team may want delivery the engine cannot read, and the Model
   Context Protocol's own aim for elicited credentials is that no intermediate server sees them.
3. **Caching in packages (S12).** It follows AWS's own advice, and makes freshness a package's
   matter. The owner may prefer A, that every store be read at each request, at a cost the first
   user would hardly notice.
4. **Pinned versions (S11).** AWS and Google disagree. The recommendation allows both, with the
   current version as the default.
5. **Refusing a read whose record cannot be kept (S13).** Vault's rule, kept narrow. The owner may
   want it for every request, or for none.
6. **The engine reads a file and an environment variable (S5, B1).** If the owner prefers the older
   design's default, the keychain, option B2 puts it in every engine; the research adds evidence
   that keychains are not reliably usable unattended.
7. **The read at process start, for no actor (S8).** It is true of every start, but it means that
   no restriction tied to a caller binds a value set at start. The owner may prefer that a plain
   server's use of its credential be limited per caller. Nothing recommended here gives that: a
   read at process start has no caller to be limited by, and a plain server applies no restriction
   (PKG-020). It would need a read made for each caller, under R1 of S3, and a process for each
   caller (S8's B3), at their costs.
8. **A store's own credential outside every Conduit (S16).** It is how AWS and Kubernetes clients
   work on a developer's machine, and invents nothing. But the package's reach at its store is
   then not inspected or closable in Conduit, and a credential kept in a variable reaches every
   process the engine starts.
9. **Only the variable at process start (S21).** EKS, systemd and Kubernetes's checklist all prefer
   a file, for reasons that hold. The recommendation rests on what MCP servers and the pages read
   for their runners show today, and on a file keeping the value from no more of the developer's
   own processes than the variable does.
10. **Which restrictions bind a read (S3, [SEC-20](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).** R3 gives up a policy aimed at one Agent's use
    of one credential through one Resource. The owner may prefer R1, the specification as it
    stands; then a Resource that reads a secret through `engine` is unavailable to every Agent
    whose Conduit to it carries a filter or whom a policy names, and the proposal should say so
    in S10 as a limit of filters and policies.
11. **The engine starts the process (S8, F1).** It takes from every engine a choice that IFC-017
    gives it today. An engine that wants to hand the start of processes to a service manager or a
    container runtime could still be allowed to, if it passes the environment that [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) and
    [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) give; that would need [SEC-16](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow) to say what "starts" covers.
12. **Which secrets' variables are withheld (S18, [SEC-18](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).** Counting every secret that is not
    retired and that `engine` serves keeps a variable from every process from the moment a secret
    names it, and lets any actor that may register a Resource withhold `PATH` from every process.
    The owner may prefer to count only secrets that an open reading Conduit leads to, which only
    the root Interface opens; then a secret's variable reaches every process started before its
    first reader is set up.
13. **`engine` reads only for a secret on `local` (S5, [SEC-14](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/secrets/README.md#promises-that-would-follow)).** It rests on `local` being the
    engine's own host, which the core package implies, by placing `management` and `access` there,
    but does not state. The owner may prefer a fact or a requirement on placement that says so.
