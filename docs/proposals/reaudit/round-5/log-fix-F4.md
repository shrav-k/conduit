# Re-audit, round 5: log of the fix pass for area F4

Date: 2026-11-07. Area F4, the implementation interface, the protocol and credentials, applies
items M02 and M45 to M77 of [DECISIONS.md](DECISIONS.md) that change its files, with M17's
words in IFC-026 and IFC-044 and M90's in IFC-022, in its files:
`spec/interface/implementation.yaml` and `spec/implementation-defined.yaml`. M78 is rejected
and changes nothing. Nothing else was edited apart from this log, and nothing was committed or
staged.

Every changed promise keeps its ID, and none was moved, added or removed. One item of the
implementation-defined list, DL11, was removed, and its ID is not reused. Each file gained a
head-comment paragraph for round 5 that names what changed and the items. Text was rewrapped
only from each change on, at no more than 96 characters a line, apart from the documents of
DL13 and DL17, which most of their changes rewrote and which were rewrapped whole.

## Applied

| Item | Files | Promise IDs | What changed |
| --- | --- | --- | --- |
| M02 | `implementation.yaml`, `implementation-defined.yaml` | IFC-041, IFC-044, DL17 | IFC-041 `then`: the decided sentence, under which the engine connects to an item at a loopback host, and sends on such a connection, only on an engine that can neither restart on another machine nor run as several parts, and otherwise the part or the request reports unavailable. It stands after the sentence on a place that `secret-reference.v1` governs, not before it, so that "that place" there keeps its antecedent. IFC-044 observable: the decided last two sentences. DL17: the decision says that DL17 follows, and its reason that "no other document needs to change" is then false, so the sentence on SEC-024, in one text with M63, says that such an engine also rejects a template whose item is reached by `remote` at a loopback host, and connects to no such item for one registered before, under SEC-013 and IFC-041. |
| M17 | `implementation.yaml` | IFC-026, IFC-044 | IFC-026: "the form of the result" in place of "the form of the answer". IFC-044: "which unencrypted `http` cannot prove". As decided. |
| M45 | `implementation.yaml` | IFC-019 | `then`: an implementation cannot apply a filter whose `arguments` hold an argument for an input that its version of the requirement does not declare. Observable: the decided sentence on a later release's optional input, NAM-024, CON-022 and ACC-015. One word differs: "the version of the requirement that it supports" in place of "that it implements", since `definitions.md` has an implementation implement operations, and the requirements it applies are those its `supports` states under PKG-019, as IFC-019's observable says. CON-022's part is F2's. |
| M46 | `implementation-defined.yaml` | DL17 | The decided text: the runs of the instances on `local`, whichever runtime provider made them, are on one machine at a time, and every run on the machine that held them has ended before the engine asks for, stops, inspects or reaches one on another. |
| M47 | `implementation-defined.yaml` | DL13 | The decided text on the engine's address that it gives a run: `https`, or `http` on loopback only for a run on the engine's own runner whose process ends when the engine stops, on an engine that can neither restart on another machine nor run as several parts. PRV-022's part is F1's. |
| M48 | `implementation.yaml` | IFC-017 | `then`: the engine follows no HTTP redirect from a run or from a `remote` item that is not plain. Observable: the decided sentence, under which a response that redirects is no result. As decided. |
| M49 | `implementation.yaml`, `implementation-defined.yaml` | IFC-041, DL13 | IFC-041 `then` and observable as decided, the observable in one text with M66: a token only from the authorization server that DL13 states or one that the user configured for the item's URL. DL13: how the user configures such a server for an item's URL, and how the engine authenticates to it. |
| M50 | `implementation.yaml` | IFC-016 | `then`: `tasks/get`, `tasks/update`, `tasks/cancel` and a `subscriptions/listen` that names the task are decided anew for the instance whose request began the invocation, and from any other report refused. Observable: the decided sentence on a task's ID. As decided. |
| M51 | `implementation-defined.yaml` | DL13 | The decided text: the engine presents both forms of proof to a run's gate, and DL13 leaves which it presents to a `remote` item and which a run presents beside its token. |
| M52 | `implementation-defined.yaml` | DL13 | The token toward a run names as its audience the instance's ID as the run's own `token` names it in `sub`; after the SPIFFE forms, the decided sentence on gates that admit only SPIFFE IDs and those that admit the ID in `engine-id`. As decided. |
| M53 | `implementation.yaml`, `implementation-defined.yaml` | IFC-042, IFC-022, DL13 | IFC-042 `when` and observable as decided. IFC-042 `then`: the decided text, "whose audience is ... `https` URI or, at such an `http` URL, the token that the run was given in `token`", would read as a token whose audience is a token, so the same rule is a second sentence: "At such an `http` URL it accepts in the same way the token that the run was given in `token`." IFC-022 observable and DL13 as decided. |
| M54 | `implementation.yaml` | IFC-015 | Observable: the decided sentences, under which an implementation reached by `remote` makes no onward request in this version. |
| M55 | `implementation.yaml` | IFC-032 | `when`: "what the request presents establishes no instance, under IFC-022". Observable: the decided sentence that leaves to ACC-037 a valid credential whose instance cannot be read, with no 401. ACC-037's part is F3's. |
| M56 | `implementation.yaml` | IFC-017 | Observable: the decided sentence, before the last, on the engine's own runner having no process for the run. |
| M57 | `implementation-defined.yaml`, `implementation.yaml` | DL13, IFC-017 | DL13: the roots against which the engine checks a `remote` item's certificate. IFC-017: "against the roots that DL13 states for it". As decided. |
| M58 | `implementation.yaml` | IFC-016 | `then`: the engine declares the tasks extension to an implementation only where the request that the call serves declared it. Observable: the decided sentences on -32602 and on `notifications/tasks`. As decided, after M50's. |
| M59 | `implementation.yaml`, `implementation-defined.yaml` | IFC-025, DL12 | IFC-025 `then`: a plain implementation's `content`, unchanged, after the text block of M71. Observable: the decided sentence on why it is passed on, and "gives `content` no other meaning". DL12: the decided clause. |
| M60 | `implementation.yaml` | IFC-037, IFC-007, IFC-016 | IFC-037 `then`, IFC-007 observable and IFC-016 observable as decided: the engine declares no capability but Conduit's extension and the tasks extension, and answers or relays no input request. OPS-007's part is F3's. |
| M61 | `implementation.yaml` | IFC-001 | Observable: the decided sentences on `x-mcp-header`, reading `tools/list`, and one resend after -32020 as one invocation. |
| M62 | `implementation.yaml` | IFC-033 | `then`: the tool's `description` is the contract's `metadata.description`, and `readOnlyHint` is true "exactly where" the contract states no `effect`. As decided. |
| M63 | `implementation-defined.yaml` | DL17 | The decided text, in one sentence with M02's: each instance made from the template before then is replaced too, under INS-020, and "no document but such a template needs to change". |
| M64 | `implementation-defined.yaml` | DL17 | The decided text on what the engine's short-lived credential bounds. |
| M65 | `implementation.yaml` | IFC-004 | `then`: the list holds for a call that carries Conduit's keys, and the decided shorter list for a call to a plain implementation. Observable: the decided sentences. |
| M66 | `implementation.yaml` | IFC-041 | Observable: RFC 8705 dropped, in the sentence as M49 rewrites it. |
| M67 | `implementation-defined.yaml` | DL13 | The decided sentence on the issuer of the mechanism's tokens, after the sentence that M49 rewrites. |
| M68 | `implementation.yaml` | IFC-037 | Observable: the decided sentence in place of the citation of the era cache. |
| M69 | `implementation-defined.yaml` | DL11 | Removed whole. The round-5 head-comment paragraph says why, that nothing replaces it, and that DL11 is not reused. The list has 15 items. |
| M70 | `implementation-defined.yaml` | DL17 | The decided sentence on a decision among competing changes, committed against the state it read. |
| M71 | `implementation.yaml`, `implementation-defined.yaml` | IFC-025, IFC-012, DL12 | IFC-025 `then`: the decided sentence on the text block. Observable: the decided replacement on `content`. IFC-012: the error's `message`, in the text of M75. DL12: "beyond what IFC-025 fixes". |
| M72 | `implementation.yaml` | none (head comment) | "IFC-014, IFC-043 and IFC-044, rules on documents". The comment's lines were rewrapped from there until their old breaks resume. |
| M73 | `implementation-defined.yaml` | DL4 | Subject "Access decisions that cannot be made", and the decided document. |
| M74 | `implementation-defined.yaml` | DL17 | The decided sentence on fetching a `file:` URL, after the Kubernetes sentence. |
| M75 | `implementation.yaml` | IFC-012, IFC-025, IFC-013 | IFC-012 and IFC-025 observables as decided. IFC-013 observable also: "and is answered as IFC-012 states", dropping ", with the protocol's error for an unknown tool", which at a revision whose answer to an unknown tool is a tool result would contradict IFC-012 as M75 words it. |
| M76 | `implementation.yaml`, `implementation-defined.yaml` | IFC-038, DL10 | IFC-038: the protocol forbids a cache to share the list across authorization contexts. DL10: the `ttlMs` of a tool list. As decided. |
| M77 | `implementation.yaml`, `implementation-defined.yaml` | IFC-036, IFC-032, DL12 | IFC-036 `when` and `then` as decided. IFC-032 `when` also excepts a request that the protocol has a server reject "with an error or an HTTP status of its own", so that a request whose `Origin` the engine does not accept, and that presents no credential, is answered 403 under IFC-036 and not 401 under IFC-032. DL12: subject and the decided sentence on `Origin`. |
| M90 | `implementation.yaml` | IFC-022 | IFC-022 observable: "on another machine as on the machine the engine runs on" in place of "as on the engine's own", the retired phrase in short, which M47 and M53 do not rewrite. The decided edits of DL13 and IFC-042 are replaced by M47's and M53's, as [items that change one text](DECISIONS.md#items-that-change-one-text) says. |

## Not applied

| Item | Files | Promise IDs | Why |
| --- | --- | --- | --- |
| M90 | `spec/promises/providers.yaml` | PRV-022 | Outside F4's files. The sentence is the one that M47 rewrites, which replaces M90's edit of it; F1 applied M47's text there ([log-fix-F1.md](log-fix-F1.md)). |
| M69 | `spec/README.md`, `spec/conventions/lifecycle.yaml` | LIF-027 | Outside F4's files: the count of the implementation-defined list, now 15, and LIF-027's pointer to ACC-003. |

## Left in F4's files by other items

These decisions name a text in F4's files, but no change to it was in F4's list, and no other
area may edit these files. They were not applied.

- M03: DL17 still says "one run serves one instance, under TPL-019", as M46's decided text
  keeps it, and IFC-017's observable says "under PRV-007 and TPL-019". M03 has DL17 stop citing
  TPL-019 for it, and PRV-007 now says that the run is asked for only where the provider reports
  it `stopped`, "so one run serves one instance".
- M07: DL16 does not yet state the registry types that the engine's own runner runs, which
  PRV-003 now cites.
- M09: DL17 does not yet say that a part that stops while the engine goes on counts as a restart
  for the runs of providers that it held, which PRV-002 and PRV-026 now say.
- M97: IFC-041's observable ("a credential that names the engine's ID") and DL13 ("issues a
  credential that names the engine's ID") still say "the engine's ID" for the ID that the
  mechanism gives, which `definitions.md` now calls the engine's credential ID.

## Checks

Run on the working tree with every area's edits in it, as they stood when this pass ended.

- The validation of [CONSOLIDATION.md](../CONSOLIDATION.md#validation): 450 promises ok. The
  implementation-defined list loads with 15 items, each with `id`, `subject` and `document`.
- `python3 tools/check_docs.py`: 0 errors.
- `python3 tools/set_names.py --check`: 0 errors.
- `python3 tools/check_schemas.py`: 0 errors.
- `git diff --check`: no output.
- No line of the two changed files is longer than 96 characters.
