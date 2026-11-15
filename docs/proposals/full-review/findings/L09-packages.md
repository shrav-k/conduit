# L09-packages: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Reviewed at commit 2eb8f258 (2026-11-12). Most of this part can be followed without asking anyone: packages and the manifest lean on existing practice (Semantic Versioning, exact pins, a digest of the archive, server.json), and the short form is a pure translation whose generated names cannot collide with authored ones. The weakest point is installation as a trust boundary: `install.v1` reads whatever place its caller names and reports what it found (F1), the address that the client-ID confirmation and bounds rely on is authenticated by nothing (F2), and how an archive's bytes are read is left to each engine (F5). Layering is mostly right, but CTR-013 is a language promise whose `observable` carries a convention that depends on state and on the caller (F4); CTR-002, CTR-004, MAN-010, MAN-011 and PKG-032 also hold several obligations each, with binding behavior placed in `observable`. In the short form the one real fault is SCT-011 reaching into framed documents, which makes a canonical document's validity depend on its file (F3). I would cut PKG-012, move the client-ID rule into one convention promise or drop it in favor of the digest pin, and ask the owner whether a submitted project's contracts need a naming regime apart from a package's (F13). Beyond the known lack of a removal operation, no promise here decides a fetch that never ends, duplicate or special archive members, or how a diagnostic names a file. A smaller point: the `vectors` field is empty in 67 of the slice's 68 promises while coverage is recorded in the conformance manifest, so the field misleads a reader.

**What the reader read.** Read in full, line by line: spec/promises/packages.yaml (473 lines), manifest.yaml (428), contracts.yaml (367) and sections.yaml (299); docs/guides/full-review.md; spec/README.md (all 678 lines); AGENTS.md lines 1 to 80, which include "What holds".

spec/definitions.md: read the sections Core terms, Objects and documents, Resolving and registration, Placement, Ownership, First start, Relationships, Access, Implementations and the implementation interface, Configuration and change, Packages, What is left to each engine, and Words the promises do not use. Long rows were cut at about 750 to 2,500 characters. Not read except by search: Requests and operations, Pending actions and sign-in, Providers, Secrets, Lifecycle, and Inspection, observation and records (lines 198 to 281 and 304 to 378).

Cited texts opened: spec/promises/naming.yaml, submissions.yaml, fleets.yaml and first-start.yaml in full; OBJ-001, OBJ-003, OBJ-013, OBJ-014, OBJ-022; SCH-018; IFC-014, IFC-050 and the end of IFC-049; OPS-023; ACC-033; PRV-003, PRV-005, PRV-007, PRV-012, PRV-017, PRV-029; PND-005, PND-010, PND-011, PND-013; REV-006, REV-007, REV-015; DL7, DL10 and the end of DL17; in spec/core, `install.v1`, `diagnostic.v1`, `target-reference.v1`, `object-management.v1`, the scope and Fleet templates, and "What this package does not settle"; spec/schemas/sectioned.yaml (lines 1 to 200 and 520 to 961), manifest.yaml (lines 1 to 140) and instances.yaml (parts); spec/names.yaml; the sectioned example project; the one vector conformance/language/packages/duplicate-exports/case.yaml; engine/CONFORMANCE.md rows DL7 and DL10, as evidence only.

Checked for known points: docs/AUDIT.md "Known to be open" and "Layer decisions", the cold readers' known-open list, the brief's queued list, IN_FORCE.md's Open and Spec pending lines with topics 4 and 5, and searches of the increment 4 log, the review list and docs/LANGUAGE_DEFECTS.md.

Not read: the other promise files, the conventions and interface files beyond the promises named above, the rest of spec/core and spec/schemas, engine/, packages/ and experiments/. Nothing was built or run, and no file was changed.


## FR-100

**install.v1 reads any place its caller names, and PKG-026 tells the caller what it found**

L09/F1 · gap · unsafe · verifier: confirmed, high · decides: owner · new

**Promises.** PKG-026, PKG-023, PKG-024, NAM-012, PRV-017, PRV-003, MAN-013, MAN-014, IFC-050

**Where.** spec/promises/packages.yaml:329

**Quote.** "Installation returns a diagnostic naming the two digests. The archive is not used, and its digest is not put in place of the one the request names, as PKG-021 states for an item of a manifest."

**Situation.** Steps to the harm. An actor M, not root, is the consumer of a Conduit to `management` that lists only `install.v1` (whole operations are the narrowest access to `management`, spec/README.md:466). (1) M asks to install any address and version with a made-up digest and `fetch: file:///&lt;any path the engine's operating-system user can read>`, or an `https` URL that only the engine's machine reaches. The engine fetches it itself (packages.yaml:278 "Every engine accepts `https` and `file`"; PRV-003, PRV-017); no promise limits the place. (2) PKG-026 returns the SHA-256 of what was fetched, and invalid against PKG-024's unavailable tells M whether the place exists. That confirms a guessed value and lets a short secret be found offline. (3) M repeats with the digest it learned. The digest now matches, the engine reads the file as an archive, and PKG-022, MAN-013, MAN-014 and SUB-005 return diagnostics that name its members and places in its YAML. Where a package-source provider claims the scheme, the fetch is made with that provider's credentials. IFC-050 requires the cause in full and withholds only what ACC-056, LIF-013 and PND-004 name.

**Counterargument.** A Conduit to `management` is root's by default, and the README already says that `register.v1` and `replace.v1` trust their consumer with programs; installing can be read as the same trust. A SHA-256 does not reveal a high-entropy secret. Cargo and npm also print both checksums.

**Least change.** At the least, add to spec/README.md, What access does not narrow: "A Conduit to `management` that lists `install.v1` trusts its consumer with every place the engine can fetch: the engine reads the `file` or `https` URL that the request names as its own operating-system user (PRV-003)." For the owner: PKG-023's `when` gains "or its `fetch` names a place outside those that the engine's conformance document states under DL7", and PKG-026's diagnostic states the fetched digest only to the root Interface.

**Precedent.** Nix's `allowed-uris` under restricted evaluation and Terraform's `provider_installation` mirrors: the operator, not the requester, names the places a fetch may read. OWASP's guidance on server-side request forgery: allow-list the destination and do not return what was fetched.

**Who decides.** owner. Limiting where `install.v1` may fetch and what its diagnostic discloses changes behavior and scope, which the owner rules. The README sentence alone adds no behavior and is the coordinator's (rulings 37 and 46 point 4).

**Verifier.** confirmed, high. Quote verbatim at spec/promises/packages.yaml:329-331. Each step is allowed by the text: spec/conventions/access.yaml:616 "(9) Installing a package or an operation reserved to root under ACC-032: `management`" (any actor whose Conduit lists install.v1); spec/promises/providers.yaml:535-536 "Where none claims it and the scheme is `https` or `file`, the engine fetches the archive itself"; PKG-023 (packages.yaml:261-264) rejects only a relative `fetch`, user information and an unclaimed scheme. No text in spec/ limits the place: spec/implementation-defined.yaml:228-229 only says "every machine that may run the engine holds each file that an install names". The stated trust covers other operations only: spec/README.md:469-471 "A Conduit to `management` that lists `register.v1` or `replace.v1` trusts its consumer to register, or to replace, any object"; ACC-030 says the same and nothing of install.v1. The harm is concrete for the shipped file secret store: packages/provider-file/README.md:8 "The locator is the instance configuration `{path: &lt;absolute path to one file>}`", so PKG-026's diagnostic returns the SHA-256 of a secret's own file to a caller with no Conduit to that Resource. Not recorded: docs/AUDIT.md:174-244, docs/guides/cold-readers.md:39-80, IN_FORCE.md (install lines 86, 92, 95, 193), the review list, G5.md, the increment logs and order 082's Record hold nothing on it.

**Verifier's corrections.** Class gap and the unsafe mark stand, narrowly: what leaves the engine is a digest, whether the place exists (PKG-024 unavailable against PKG-026 invalid), and a GET made from the engine's network position or with a package-source provider's credentials; no content is returned. The title's "tells the caller what it found" overstates this. Step 3 holds only where the named file is itself a gzip-compressed tar: for anything else PKG-022 returns only "a diagnostic naming the archive" (packages.yaml:249). Only root holds install.v1 by default (spec/core/templates.yaml:121), and a holder of register.v1 can already run programs, so the affected case is a Conduit that lists install.v1 alone. Who decides is as stated: the README sentence is the coordinator's, the limit on places and on the diagnostic is the owner's.


## FR-101

**CTR-013's root confirmation is keyed by an address that anyone who may install can claim**

L09/F2 · gap · verifier: confirmed, medium · decides: owner · new

**Promises.** CTR-013, PND-013, NAM-011, PKG-017, SUB-008

**Where.** spec/promises/contracts.yaml:313

**Quote.** "The engine compares the client IDs of a Type's logins with every Type it holds at another address: every installed package's Types and every submission's Types held by a template or instance (SUB-008, PKG-029). Types at one address share freely, including sibling Types, package versions and submissions at one address."

**Situation.** Root installs example.org/agents 0.2.0, whose Type states client ID C. M, not root, holds `install.v1` on `management` and installs its own archive as example.org/agents 9.9.9. NAM-011 installs it beside the first; PKG-017 asks only that the archive's manifest state the address asked for; no promise ties an address to a publisher. Its Types state C, and since versions at one address share freely there is no conflict and no confirmation. The same holds for a second submission at a submitted project's address (submissions.yaml:145 "Two submissions may state one address"). Yet the same Type at any other address reports conflict until root confirms, and a project that states it cannot be registered at all. PND-013 (pending.yaml:282) names the purpose: to stop "a Type claiming another Type's client". The check so binds the honest publisher at a new address and not the one who reuses the trusted address, and whoever installs first at an address owns its client IDs.

**Counterargument.** Whoever may install or register through `management` is already trusted (README, What access does not narrow). A template uses the added version only where a manifest pins its digest, and on the engine's own runner a program of the engine's user can read every state directory anyway. The exemption spares root a confirmation on every upgrade of one package.

**Least change.** CTR-013: "Types of one package version, and of one submission, share freely. A Type of another version at that address, or of another submission at it, is compared as one at another address is, unless the request to install is root's own." If the owner prefers less machinery: remove the confirmation and state in the README limits that the digest pin is the only defense.

**Precedent.** Android's update rule: an update shares the installed app's identity only where the same key signed it. Go authenticates a module path by fetching from it and by its checksum database. Conduit authenticates no address, so only a package identified by its digest can be the unit.

**Who decides.** owner. The rule is the owner's (ruling 75, C6: "A client ID that two installed Types claim needs root's recorded confirmation"). Which installs it reaches is behavior; the exemption by address is the admin's reading.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/contracts.yaml:313-317. The steps hold on the text: spec/promises/naming.yaml:181 "A second version is installed beside the first"; packages.yaml:147-148 (PKG-017 checks only that the archive's manifest states the address and version asked for); naming.yaml:16-37 (NAM-001 fixes an address's form and authenticates nothing); submissions.yaml:145 "Two submissions may state one address". Purpose at spec/promises/pending.yaml:281-282 "where the URL carries a client ID, a Type claiming another Type's client". The owner's words carry no exemption by address: docs/proposals/ruling-75/README.md:71-72 "A client ID that two installed Types claim needs root's recorded confirmation." The exemption is recorded as the admin's reading: docs/work-orders/038c-spec-sign-in-return.md:185-187 ""another address" exempts sibling Types in one package and the submissions at one address, as well as a package's versions". It is not on docs/proposals/first-build/to-review-with-the-owner.md (no row on client IDs or logins). The known point at docs/AUDIT.md:241 ("races across addresses, between two Types claiming one client ID") is a race, a different situation.

**Verifier's corrections.** None to class or to who decides (owner; the spec is narrower than the ruling's words, which strengthens that routing). The unsafe mark is arguable and I leave it false: the same-address path defeats the check's stated purpose, but every step needs an actor who may install through `management`, and spec/README.md:418-419 names the digest pin as the defense. Add to the situation that IFC-049's sign-in message names the package (interface/implementation.yaml:1367-1368), which for the reused address is the trusted one.


## FR-102

**SCT-011 supplies an owner to framed Agents, which OBJ-013, FLEET-002 and instance.v1 reject without one**

L09/F3 · contradiction · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-202.

**Promises.** SCT-011, OBJ-013, FLEET-002, SUB-005, SUB-010, SCT-003

**Where.** spec/promises/sections.yaml:185

**Quote.** "The translation makes Fleet X from the core template `fleet`, and supplies X as the owner of every Agent instance defined anywhere in that file, whether sectioned or framed. An explicitly written owner equal to X is accepted; one naming another Fleet is rejected."

**Situation.** One file holds a sectioned document with `fleet: dev` and a framed Agent instance document with no `owner`. SCT-011 supplies `dev`. OBJ-013 (objects.yaml:200) says of a framed instance document that "`owner`, `consumer` and `target` are required for those families"; FLEET-002 (fleets.yaml:34) says "An Agent document with no `owner` ... is rejected"; `instance.v1` requires `owner` (spec/schemas/instances.yaml:101-105), and SUB-005 rejects what a Schema contract rejects. Builder A validates each framed document as it is read and rejects the file. Builder B supplies the owner first and accepts it. The converse also follows from SCT-011: the same framed Agent with `owner: ops` is valid in one file and rejected in another that states `fleet: dev`, so a canonical document's validity depends on which file holds it, although SCT-003 hands "the framed documents read beside them" over unchanged.

**Counterargument.** SCT-011 is the later and more specific rule, ruling 59 A6 says the Fleet owns every Agent in the file, and SUB-010 says that SCT-011 supplies owners per file before the boundaries are discarded. A builder can read OBJ-013 as applying after translation.

**Least change.** Either OBJ-013 and FLEET-002 gain "unless its file's `fleet` supplies it (SCT-011)", with a note in spec/schemas/README.md that `instance.v1` is applied after SCT-011. Or SCT-011 reads "every Agent definition in the sectioned documents of that file", which leaves a framed document's meaning independent of its file. The first keeps the words of ruling 59 A6.

**Precedent.** kubectl: a namespace given with `-n` fills a manifest that omits one and is an error where the manifest names another. Kustomize's `namespace:` does the same. The rule is stated once, at the place that supplies the value.

**Who decides.** coordinator. A contradiction whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/sections.yaml:185-187. Against it, with no exception written: spec/promises/objects.yaml:200-201 "`owner`, `consumer` and `target` are required for those families" (when: "A framed instance document is read"); spec/promises/fleets.yaml:34-35 "An Agent document with no `owner`, or an owner that is not a Fleet, is rejected"; spec/schemas/instances.yaml:100-105 (`required` lists `owner` for an Agent); spec/schemas/README.md:101 routes "A framed document, with `kind`" to `document.v1`. The exception is written only for sectioned definitions: sections.yaml:111-112 "an Agent requires its Fleet `owner` unless SCT-011 supplies it". submissions.yaml:274 "SCT-011 supplies owners per file before its boundaries are discarded" gives an order and amends neither OBJ-013 nor the schema. No record holds it (searched SCT-011 across docs/, IN_FORCE.md:74, experiments/011, experiments/023); experiment 011's code took the supplying reading (experiments/011-short-form-spike/src/lib.rs:339), which is no decision.

**Verifier's corrections.** Class and who decides stand. One overstatement: SCT-003 does not say framed documents are handed over unchanged; it says "every other check apply unchanged to that set" (sections.yaml:55-56), which favors the reading that supplies the owner first. The fault is the missing exception in OBJ-013, FLEET-002 and `instance.v1`, so the first of the two offered fixes is the right one. That a framed Agent's validity depends on its file is SCT-011's intent (SCT-004, sections.yaml:70) and ruling 59 A6, not a second fault.


## FR-103

**CTR-013 holds a convention with three reasons and a root-only input in the `observable` of a language promise, and clashes with PKG-027**

L09/F4 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** CTR-013, PKG-027, PKG-023, PKG-025, OPS-023

**Where.** spec/promises/contracts.yaml:317

**Quote.** "Installing a package whose Type shares such a client ID reports conflict unless the request confirms it in install.v1's optional confirmClientIds list. Only root may give that input; another caller giving it reports refused. A confirmation naming a client ID shared with no such Type reports invalid."

**Situation.** CTR-013 has `layer: language`, and its `then` is the static shape of `signIn`. The rule on client IDs shared across addresses stands only in its `observable`. It depends on what is installed and held and on who the caller is, and it defines three reasons and an input and an output of `install.v1`. The form of a promise makes `then` "the single obligation" (spec/README.md:291), and by LD1 (docs/AUDIT.md:406) a rule that the documents alone cannot answer is a convention. Two consequences. An engine that claims the Language level ("documents in, resolved objects or diagnostics out") is bound by a root-only access rule. And the install promises PKG-023 to PKG-032 never cite it: PKG-027 says a request for a package already installed with the same digest "changes nothing and has the outcome succeeded", while this text has it report conflict unless it confirms. For a package that root installed with a confirmation, a later plain repeat by another actor, such as the install-first part of an apply, is succeeded under one reading and conflict under the other, and under the second only root could ever repeat it.

**Counterargument.** The text is in spec/ and binds wherever it stands. PKG-027's "not dispatched where another promise has a request to install not dispatched" can be read to include CTR-013. One vector tests the static part.

**Least change.** Split it. CTR-013 keeps the shape and the static rejections. A new convention promise in packages.yaml: "when: An actor asks to install a package, or to register or replace with a project, one of whose Types states in a login a client ID that a Type the engine holds at another address states. then: The request is not dispatched and reports conflict, unless it is a request to install whose `confirmClientIds` names each such ID." The root-only case and the unmatched confirmation become its stated cases, with one sentence: "A request for a package already installed with the same digest adds no Type and is PKG-027."

**Precedent.** The file's own pattern: PKG-026 (language: the package is rejected) beside PKG-023 (convention: what the request reports).

**Who decides.** coordinator. A defect of form and an ambiguity; the split adds no behavior, and the reading chosen for PKG-027 adds none (rulings 37 and 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/contracts.yaml:317-320; `layer: language` at contracts.yaml:283; the `then` (288-300) holds only the static shape. spec/README.md:291 "The single obligation"; spec/README.md:320 Language level: "documents in, resolved objects or diagnostics out"; LD1 at docs/AUDIT.md:406. packages.yaml cites CTR-013 nowhere (grep: no match). The clash is real: packages.yaml:343 "changes nothing and has the outcome succeeded" against contracts.yaml:317-318 "reports conflict unless the request confirms it", and PKG-027 itself admits other promises: packages.yaml:351-352 "the request is not dispatched where another promise has a request to install not dispatched". Order 082 knew that CTR-013 "gives a request to install new answers" (docs/work-orders/082-spec-records-and-installs.md:164-167) and wrote PKG-032 generically; no record decides the repeat of a confirmed install. Not in the known lists.

**Verifier's corrections.** None. The divergence matters to plan and apply's install-first step: under the second reading a plain repeat of a package that root installed with a confirmation reports conflict for every caller, and only root can give the input that clears it. The same question arises for a package installed first whose client ID a later, confirmed package at another address shares.


## FR-104

**Which bytes of an archive are the manifest and the documents is left to each engine**

L09/F5 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** MAN-014, MAN-013, PKG-022, PKG-004, SUB-011

**Where.** spec/promises/manifest.yaml:315

**Quote.** "A package's archive holds a member whose path is absolute or has a `..` segment, or a member that is a link to a place outside the archive."

**Situation.** An archive holds two members named `conduit.yaml`, or two documents files of one name, or a member that is a hard link, a device or a FIFO, or it unpacks to far more than its compressed size. MAN-014 decides only absolute paths, `..` and links that leave the archive, and PKG-022 leaves "which of the tar formats and extensions an engine reads" to DL7. So what one digest means is each engine's choice: the first build documents "Last member at a path wins" (engine/CONFORMANCE.md:24), and an engine or a review tool that takes the first member reads another manifest, with other implementation items, under the same address, version and digest. The digest is the stated defense (spec/README.md:419, "choosing packages, pinned by digest"), and PKG-004 pins the bytes, not their reading. SUB-011 bounds "the size of a package's archive" only where an engine states a limit, and says nothing of the size once unpacked.

**Counterargument.** DL7 is a documented choice. Publishers use ordinary tools, which emit no duplicate members, and the first build's choice matches what `tar` does when it extracts.

**Least change.** MAN-014's `when` adds: "or two members with one path, or a member that is neither a regular file, a directory nor a link to a place inside the archive". SUB-011's examples add "the size of a package's archive once unpacked".

**Precedent.** Go's module zip rules (golang.org/x/mod/zip): duplicate paths, paths that differ only in case, files that are not regular and contents over a size limit are rejected. crates.io applies like checks to a crate's tarball.

**Who decides.** coordinator. A small finding that adds a rejection rule and moves none of S1's author counts (ruling 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/manifest.yaml:315-316. DL7 covers only "which of the tar formats and extensions an engine reads in an archive" (spec/implementation-defined.yaml:61-62; packages.yaml:251-252), not two members at one path. The first build still files its choice there: engine/CONFORMANCE.md:24 "Last member at a path wins except replacing a nonempty directory or using a non-directory parent." manifest.yaml:305-306 "Unpacking the archive into an empty directory gives the project's directory as it was published" is no rule for a crafted archive. SUB-011 names "the size of a package's archive" (submissions.yaml:285-286) and nothing unpacked. No record: searched AUDIT, the first-build logs (1-06 and 1-16 at docs/proposals/first-build/README.md:62, 72), order 013's Record (docs/implementation/WORK_ORDERS.md:513-525), round 6 and LANGUAGE_DEFECTS.

**Verifier's corrections.** Narrow the situation. A member that is not a regular file at a name MAN-011 reads is already decided: manifest.yaml:261-264 "anything that is not a regular file, is rejected". A link to a place inside the archive is accepted by MAN-011 ("A link to a file is read as that file") and by MAN-014's silence. Undecided are two members at one path, the unpacked size, and special members below the root. Unsafe stays false, though a tool that reads the first member and an engine that reads the last see different manifests under one pinned digest. The precedent for crates.io was not checked here.


## FR-105

**A control may change the `signIn` account place; CTR-014 forbids the same for `toolsDirectory`**

L09/F6 · gap · verifier: confirmed, low · decides: coordinator · new

**Promises.** CTR-013, CTR-014, CTR-006, PND-005, PND-011, PND-013, PRV-016

**Where.** spec/promises/contracts.yaml:302

**Quote.** "A string at the account place is the expected account. Null disables both sign-in actions and the account check, as for API-key configuration."

**Situation.** A Type states `signIn` with the account pointer `/account` and lists a control whose `changes` is `{account: /account}`. CTR-006 accepts it, since the schema describes the place. CTR-013 has no rule against it, although CTR-014 (contracts.yaml:340) forbids exactly this for `toolsDirectory`, PRV-016 for `/schemes` and SEC-005 for a locator. An actor with a Conduit that lists the control sets the account to another string, or to null, while the instance stays signed in. PND-005 and PND-011 check the account only within setup and `sign-in.v1`, so model work goes on under the old login, inspection shows an expected account that is not the one in use, and with null later setups check nothing. PND-013 (pending.yaml:285) says of the account check: "It alone stops that attack".

**Counterargument.** The Type's author chose to list the control, and whoever opened the Conduit chose to list it. A deliberate control that switches the account, followed by a setup, is a sound use, and the next setup undoes a wrong account.

**Least change.** CTR-013's `then` adds CTR-014's sentence: "The Type lists no control whose `changes` holds the account pointer, a pointer inside it or a pointer over it, compared by reference tokens as CTR-006 compares pointers."

**Precedent.** The specification's own CTR-014, PRV-016 and SEC-005. Kubernetes makes such fields immutable once the object exists, as a Pod's `serviceAccountName` is.

**Who decides.** coordinator. A small finding that adds a rejection rule, following a rule already in the same file (ruling 71 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/contracts.yaml:302-303. CTR-013's `then` (288-300) requires the place and confines it to string or null and has no rule on controls, while CTR-014 has one: contracts.yaml:340-342 "The Type lists no control whose `changes` holds the pointer, a pointer inside it or a pointer over it" and "The place is fixed for the Agent's life." The same pattern: providers.yaml:505 (PRV-016) and secrets.yaml:127 "It lists no control" (SEC-005). The account is checked only in setup and sign-in.v1: pending.yaml:141-143 and 204-205; no promise reacts to a control that changes the place. pending.yaml:285 "It alone stops that attack". No record holds it.

**Verifier's corrections.** Class gap, not unsafe and coordinator under ruling 71 point 4 all stand. Severity is low: it needs a Type whose author lists a control over the account place and a Conduit that lists that control, both choices of trusted parties, and the next setup undoes a wrong account (PND-011) except where the control wrote null.


## FR-106

**Nothing bounds the engine's own fetch, and PKG-032 makes every later install of that version wait on it**

L09/F7 · gap · verifier: known, medium · decides: coordinator · new

**Promises.** PKG-032, PKG-024, PRV-017, PRV-003

**Where.** spec/promises/packages.yaml:450

**Quote.** "The engine decides requests to install one address and version one at a time. It begins to decide a request only once every request to install that address and version that it began to decide before has ended, and decides it from what is installed then."

**Situation.** A request to install A at V names an `https` `fetch` whose server accepts the connection and never answers: a hung mirror, or a caller's own URL. PKG-032's observable says "Deciding includes fetching the archive, before dispatch". DL10 bounds "the time allowed for a call" (spec/implementation-defined.yaml:83), but the engine's own fetching is no call (PRV-003: "no provider call reaches them"), and no other item bounds it. The request never ends, PKG-024's "cannot be fetched now" is never reached, and every later request for A at V, root's with a good URL among them, waits for it.

**Counterargument.** The first caller can cancel, a restart of the engine ends the request, and any careful engine sets a timeout of its own. Cargo also blocks on the lock of its package cache.

**Least change.** DL10 adds "the time allowed for fetching an archive". PKG-024's `when` adds "or is not fetched within the time that the engine states under DL10", so the earlier request ends and the waiting one is decided.

**Precedent.** Cargo's `http.timeout` and `net.retry`.

**Who decides.** coordinator. A small finding that adds one stated limit (ruling 71 point 4).

**Verifier.** known, medium. Quote verbatim at spec/promises/packages.yaml:450-452, and the gap is real (DL10 bounds only "the time allowed for a call", spec/implementation-defined.yaml:83). It is already recorded as the coordinator's to settle: docs/work-orders/082-spec-records-and-installs.md:489-490 "no promise or DL item bounds how long a fetch may take, so one slow fetch holds back every later request to install that address and version"; its Record, :740-741 "For the coordinator, whether a purge leaves a gap, recreating a lost record, and a bound on a fetch's duration." docs/AUDIT.md:238-244 points to that Record.

**Verifier's corrections.** Mark known; the reviewer's situation is the recorded one. The fix offered (a DL10 item and a clause in PKG-024, after Cargo's `http.timeout`) is sound and is the coordinator's. The reviewer could not have found it in the lists the brief names: see notes.


## FR-107

**CTR-006 does not say which places a type's schema "describes"**

L09/F8 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** CTR-006, REV-007, REV-015, MAN-017

**Where.** spec/promises/contracts.yaml:165

**Quote.** "A Type contract lists it under `controls`, and each pointer in its `changes` addresses a place in the configuration that the type's `schema` describes."

**Situation.** A Type's schema is `{type: object, properties: {limits: {type: object, additionalProperties: {type: integer}}, tags: {type: array, items: {type: string}}}}`, with nothing required, and a control's `changes` is `{cpu: /limits/cpu, first: /tags/0}`. Reading 1: a place is described only where each token names a member that `properties` lists, so both pointers are rejected. Reading 2: a place is described where any subschema applies to it, so both are accepted. Under reading 2, and for any pointer below a member that is not required, REV-007's "exactly those values are stored, each at the pointer" does not say what is stored when `/limits` is absent or `/tags` is empty. MAN-017, CTR-013 and CTR-014 each give an exact test for their pointers; CTR-006 gives none.

**Counterargument.** REV-015 checks the resulting configuration against the schema before dispatch, so nothing invalid is stored under either reading, and most controls address a top-level property.

**Least change.** CTR-006: "A place is described where each reference token on the way to it names a member that `properties` lists, a `$ref` standing for the schema it leads to, and every member on the way to it, the place itself apart, is required."

**Precedent.** JSON Patch (RFC 6902): `add` requires the parent of the target location to exist. Kubernetes structural schemas require every field to be specified under `properties`.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/contracts.yaml:165-166. No text in spec/ defines when a schema describes a place: spec/schemas/README.md:184 and spec/schemas/contracts.yaml:49 repeat the word, and spec/schema-profile.md:26 gives only pointer syntax and overlap. Input destinations have a parent rule (spec/promises/instantiation.yaml:146-148, INS-011: "a destination whose parent is not present in the template"); a control's pointer has none, and spec/conventions/revisions.yaml:97-98 "the values it was sent applied at those pointers" does not say what is stored where the parent is absent. Round 6 settled only overlap (docs/proposals/reaudit/round-6/DECISIONS.md:88). Not in the known lists.

**Verifier's corrections.** Add one piece of text the reviewer missed: MAN-017 uses the word of `additionalProperties` too (spec/promises/manifest.yaml:385-386 "neither `properties` nor `additionalProperties` describes"), which tilts maps toward reading 2. Nothing speaks for `items`, `patternProperties`, a `$ref`, a branch of `anyOf` or a `{}` schema, so the ambiguity stands. The offered fix picks reading 1 and would reject Types that reading 2 accepts; it is still the coordinator's, as the settling of an ambiguity.


## FR-108

**PKG-016's observable forbids two instances that share an outside store, and no test can tell**

L09/F9 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** PKG-016

**Where.** spec/promises/packages.yaml:132

**Quote.** "The results such an implementation returns for one instance are the same whether or not it has also served others, in one process or several."

**Situation.** A package's directory implementation serves two Resources whose configurations name one `root`, or two instances over one database. A write through one changes what a read through the other returns. Reading 1, the literal one: the results for one instance must be the same whether or not the implementation served the other, so the package breaks PKG-016 and cannot claim the Implementation level. Reading 2: only state that the implementation keeps of its own may not cross instances, and what two configurations name in common is theirs. The observable is a universal negative over every result, so no vector can separate the readings. The promise has `layer: interface` and says nothing of the wire.

**Counterargument.** The ruled question (L47, docs/LANGUAGE_DEFECTS.md:194) was only whether plugin state is kept per instance, and most readers will take reading 2.

**Least change.** PKG-016's `then`: "Its behavior for that instance depends on state that it keeps from having served another instance. What the configurations of two instances name in common, such as one directory, is not such state."

**Precedent.** Kubernetes CSI: one driver serves many volumes and keeps no state across them, while two volumes may be backed by one share.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/packages.yaml:132-133; `then` at :130 "Its behavior for that instance depends on its having served another instance." The ruled question was narrower: docs/LANGUAGE_DEFECTS.md:194 "Whether plugin state is isolated per instance or shared when two instances use one plugin." spec/README.md:74-75 puts "How one type behaves ... or a directory's edge cases" in the type's contract. PKG-016 is cited by no other promise (grep over spec/).

**Verifier's corrections.** Not a sound second reading: no careful builder holds that two Resources over one directory or one database break the promise, and no engine behavior turns on it. What survives is a defect of form, not an ambiguity: the observable is a universal over every result that no vector can check (spec/README.md:298 "If nothing could tell, it is not a promise"), and the `then` is broader than L47's question. The offered rewording is right and is the coordinator's. The layer remark carries little weight, since the Implementation level binds an implementation "whatever its layer" (spec/README.md:322).


## FR-109

**PKG-029 does not say whether anything is fetched before it reports conflict**

L09/F10 · ambiguity · verifier: confirmed, low · decides: coordinator · new

**Promises.** PKG-029, PKG-025, PKG-026, OPS-023, PRV-017

**Where.** spec/promises/packages.yaml:398

**Quote.** "Where the actor is not permitted, or another reason also applies, OPS-023 says which is reported."

**Situation.** A template holds a submission at example.org/x. An actor asks to install example.org/x 1.0.0 with a `fetch` whose archive has another digest than the one asked for, which PKG-026 makes invalid. Reading 1: PKG-029 is decided from the address alone, as PKG-025 is decided "from what is installed: nothing is fetched", so the request reports conflict and no fetch and no `fetch-package.v1` call is made. Reading 2: OPS-023 puts invalid before conflict, so the engine fetches to learn whether invalid applies, reports invalid, and records a provider call. PKG-025 states its answer in words; PKG-029 does not.

**Counterargument.** Both readings install nothing. The difference is a reason and a recorded fetch, in a rare double fault.

**Least change.** PKG-029's `then` adds PKG-025's sentence: "It is decided from the address and from what the engine holds: nothing is fetched."

**Precedent.** PKG-025's own wording.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/packages.yaml:398-399. PKG-025 says it in words (packages.yaml:305-306 "It is decided from what is installed: nothing is fetched"; :311-312 "whatever digest the archive at the place of `fetch` has"); PKG-029 does not. PKG-023 names only two cases that fetch nothing: packages.yaml:274-275 "nothing is fetched for those two". OPS-023 puts invalid before conflict (spec/conventions/operations.yaml:493-495), and whether PKG-026's invalid applies cannot be known without fetching. install.v1's description (spec/core/operations.yaml:618-621) does not settle it. No record.

**Verifier's corrections.** None. The text leans to reading 2 (fetch, then invalid before conflict), since PKG-029's package is not installed and PKG-024 and PKG-026 "are about a package that is not yet installed" (packages.yaml:312-313); a builder who mirrors PKG-025 takes reading 1. Both install nothing.


## FR-110

**SCT-014 promises authored places across files, and a diagnostic can name only a JSON Pointer**

L09/F11 · gap · verifier: known, low · decides: coordinator · known-new-situation

**Promises.** SCT-014, SCT-011, SCT-013, NAM-027, SCT-015

**Where.** spec/promises/sections.yaml:257

**Quote.** "A supplied Fleet owner maps to `/fleet`; an explicitly written owner maps to its authored owner. A Fleet shared by several files names their supplying places."

**Situation.** Known: the core package leaves "How a diagnostic names a document or an object beyond the JSON Pointer of its logical location" to each engine (spec/core/README.md:706). The new situation: the short form makes that naming the thing observed. Files a.yaml and b.yaml each state `fleet: dev` and a framed Fleet `dev` clashes with it under NAM-027, or both files define `agents.worker`. SCT-014, SCT-013 and NAM-027 require the diagnostic to name both authored places, but `diagnostic.v1` gives a place as a JSON Pointer, and the pointer is the same in both files (`/fleet`, `/agents/worker`). A file that holds two sectioned documents has the same fault. No vector can compare what these promises require, and SCT-015's equality "with authored locations under SCT-014" cannot be checked across files. An author, or an Agent that writes the files, is told a place and not which file holds it.

**Counterargument.** The core package records the naming of documents as unsettled until the vectors fix it, an engine may add other SARIF members, and within one document the pointer is enough.

**Least change.** SCT-014's `then` adds: "Each location also names the file, relative to the directory handed over, in SARIF's `physicalLocation.artifactLocation.uri`, and, for a file of several documents, the document's index."

**Precedent.** SARIF 2.1.0 `physicalLocation` (`artifactLocation.uri` and `region`).

**Who decides.** coordinator. A gap in a diagnostic, closed with an existing SARIF member; small, and it moves none of S1's author counts (ruling 71 point 4; ruling 52's explicitness rule).

**Verifier.** known, low. Quote verbatim at spec/promises/sections.yaml:257-258. The point is a stated choice inside spec/: spec/core/README.md:705-706 "How a diagnostic names a document or an object beyond the JSON Pointer of its logical location, and the `source` and `type` of a record's CloudEvent: an engine names them." An engine can keep SCT-014 by its own means: spec/core/results.yaml:74-75 "An engine may add what else SARIF's `result` holds." docs/AUDIT.md:185-189 lists what the core package leaves as known to be open.

**Verifier's corrections.** The situation offered as new is the recorded one. Two framed documents of one name in two files already need NAM-027's "diagnostic naming the two" (naming.yaml:483) with one pointer each; the short form adds promises to it and no new case. One sub-claim is wrong: SCT-015 does not compare authored locations, its observable excludes them (sections.yaml:274-276 "apart from generated IDs and the authored locations in diagnostics"). Making the file's name portable through SARIF's `physicalLocation` would change the stated choice of ruling 41 item 24 and fits ruling 52; it is a small added feature, so the coordinator's under ruling 71 point 4, logged for the owner.


## FR-111

**PKG-012 is never the deciding rule**

L09/F12 · needless-complexity · verifier: weakened, low · decides: coordinator · new

**Promises.** PKG-012, MAN-007, NAM-027, SCH-012, SUB-005

**Where.** spec/promises/packages.yaml:119

**Quote.** "Installation returns a diagnostic naming the repeated export name. An implementation is an export under PKG-019. One that has the name of a document of its package is rejected under NAM-027, whether or not the package exports that document."

**Situation.** Every way in which a package can export two things under one name is already rejected by another rule: a name written twice in `exports` by `manifest.v1` under SUB-005 (MAN-007), two documents of one name or an implementation named as a document by NAM-027, and two implementations of one name by SCH-012. The one vector tagged PKG-012 expects the `ruleId` SUB-005 (conformance/language/packages/duplicate-exports/case.yaml:48). A builder who implements PKG-012 as a check of its own emits another `ruleId` than a builder who follows MAN-007.

**Counterargument.** It states the invariant in one place for a reader and does no harm.

**Least change.** Remove PKG-012 and add to the file's head comment: "Removed: PKG-012 → MAN-007 and NAM-027."

**Precedent.** The file's own register of removed IDs, such as "PKG-002 → NAM-006".

**Who decides.** coordinator. Removing a duplicate changes no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/packages.yaml:119-121. The other rules do cover each case: manifest.yaml:154-155 "A name written twice is rejected by `manifest.v1`, under SUB-005"; naming.yaml:476-477 and :488-489 (NAM-027, and SCH-012 for two implementations). But PKG-012 is the promise that the schema encodes for a name written twice: spec/schemas/manifest.yaml:14 lists it, and :52-53 "`uniqueItems` does reject a name that `exports` writes twice (PKG-012)". Reporting under SUB-005 is the rule for every schema-encoded promise: submissions.yaml:67-69 "rejected under this promise, whichever promise that contract encodes".

**Verifier's corrections.** The vector's `ruleId` SUB-005 is no evidence of redundancy, and no builder is misled on the `ruleId`: SUB-005 fixes it for every promise a schema encodes. PKG-012's `then` is also the only `then` that states that an export's name is unique; MAN-007 says so only in its observable. So it is a small duplication, not a promise that never decides. If it is removed, MAN-007's `then` must take the sentence, and the citations at spec/core/README.md:69, naming.yaml:488, packages.yaml:271, spec/schemas/README.md:177, spec/schemas/manifest.yaml:14 and :53 and the row at conformance/manifest.yaml:6651 must change. Coordinator's; little is gained.


## FR-112

**Contracts at an address have two regimes, and three promises exist only to keep them apart**

L09/F13 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** MAN-016, MAN-015, PKG-029, MAN-004, SUB-008, NAM-009, NAM-022, CTR-013

**Where.** spec/promises/manifest.yaml:356

**Quote.** "A qualified reference at that address reaches the installed package's exports, so the project's contracts cannot be named by it as well. The other order, installing a package at the address of a submitted project, reports conflict under PKG-029 while a template or an instance holds a submission at that address."

**Situation.** Contracts and implementation items at an address are held in two ways: as an installed package (address, version, the archive's digest, one digest for a version under PKG-025) and as a submitted project (address, optional version, SUB-008's digest, several submissions at one address). Keeping the two apart costs MAN-015, MAN-016 and PKG-029, most of MAN-004's observable, a second digest, clauses in NAM-009, NAM-022, NAM-028, PKG-005, SUB-009, CTR-013 and ACC-029, and three rows of definitions. An author who tries a local change to an installed package cannot submit it under its own address (MAN-016), and a bound that names `address#name` passes without notice from a submission's operation to a later package's (PKG-029's observable).

**Counterargument.** A submitted project lets an author edit a contract and register again with no packaging and no change of version, which S1's edit cycle measures. The regime was settled over several rulings and audit rounds.

**Least change.** For the owner: treat a submission's contracts as one more version of the package at its address, named by a pseudo-version made from SUB-008's digest, and held and listed as any version is (NAM-009, NAM-011). MAN-016 and PKG-029 then go, and PKG-005, NAM-009 and CTR-013 lose their clauses on submissions.

**Precedent.** Go's pseudo-versions and `replace` directives, and Cargo's path dependencies: unpublished content is one more version of the same module.

**Who decides.** owner. It removes rules and changes how a project's contracts are named, which is behavior the owner rules.

**Verifier.** weakened, low. Quote verbatim at spec/promises/manifest.yaml:356-359. The facts check out: MAN-016 (manifest.yaml:349-353) bars a submission at an installed package's address, and packages.yaml:395-397 has a bound's `address#name` cover a later package's operation. The regime is ruled, and the earlier form was replaced: docs/proposals/ruling-log.md:497 "a project's contracts are kept as an unpublished package, named by a digest of its documents" was replaced by "Ruling 41, item 11: a project that holds contracts states `address`, and outside its own documents they are `address#name` (MAN-015, SUB-008)."

**Verifier's corrections.** This is a design alternative to a ruled decision, not a fault in the text. The proposal also adds behavior with rules of its own: a package and submissions together at one address, a pseudo-version that other projects' manifests could pin, and so a submission's contracts selectable from outside, which SUB-008 excludes today (submissions.yaml:141-142 "no document outside the project selects it"). Keep it as an optional question for the owner, to raise only if S1's rerun shows the two regimes cost authors lines or edits. The reviewer's own confidence is low.


## Further points the verifier noticed

Not verified by a second reader.

- **Nothing bounds the size of what the engine's own fetch reads, and the place is the caller's to name** (spec/promises/submissions.yaml:295). SUB-011: "an unstated limit on these is not applied", and DL10 lets an engine state a limit on an archive's size without requiring one (spec/implementation-defined.yaml:79-81); the first build states none (engine/CONFORMANCE.md:29 "No stated byte limit for files or archives"). With F1's caller-named `file:` place, a device or a pipe that never ends, or a very large file, is read without a promise that ends or rejects the request, and PKG-032 then holds back every later install of that address and version. It joins F1 and the known fetch-duration point of order 082. Least change: PRV-003's own `file:` fetching reads a regular file only, and DL10 requires every engine to state its archive size limit. Gap; coordinator under ruling 71 point 4.
- **MAN-011 sends an unreadable member of a package's archive to SUB-010, which is about a directory handed over for registration** (spec/promises/manifest.yaml:264). "is rejected under SUB-010 and never skipped" is written for both cases of MAN-011's `when` ("whether it was submitted or is the content of a package's archive"), but SUB-010's `when` is "An engine is handed a directory to register" (spec/promises/submissions.yaml:238-239). For an archive member that is a link leading nowhere or not a regular file, two builders can emit `ruleId` SUB-010 or MAN-011. Defect, low; coordinator: name MAN-011 as the deciding rule at installation.

## The verifier's notes

Checked at commit 2eb8f258 (2026-11-12), read-only; nothing was built, run or written. All 13 quotes are verbatim at the cited lines, and the cited supporting lines were opened. I read the four slice files in full, the relevant parts of spec/README.md, spec/implementation-defined.yaml in full, and the cited promises in naming, providers, submissions, objects, fleets, pending, access, operations, revisions and the core package.

Outcome: 7 confirmed (F1 high and unsafe in a narrow sense; F2, F3, F4, F5, F8 medium; F6, F10 low), 3 weakened (F9, F12, F13, all low), 2 known (F7, F11), none refuted.

- **F1** is the one finding that is unsafe. The shipped file secret store keeps one secret per file, so the digest that PKG-026 returns is the digest of a secret's value. No content is returned.
- **F7** is recorded only in order 082's file and its Record. docs/AUDIT.md:238-244 and docs/guides/full-review.md:138 carry two of that order's questions and drop the fetch-duration bound, so the reviewer could not have found it in the lists the brief names. The tracker's list should take it.
- **F2**: the exemption for Types at one address is the admin's reading in order 038c. It narrows the owner's words in ruling 75 C6 and is not on the review list, so it goes to the owner.
- **F12**: the reviewer missed that `manifest.v1` names PKG-012 as the promise its `uniqueItems` encodes, and that SUB-005 fixes the `ruleId` for every schema-encoded promise.

The reviewer's count holds: 68 promises in the slice (24, 17, 12, 15), 67 with an empty `vectors` field.
