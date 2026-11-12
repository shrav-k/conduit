# Return implementation after 048b

Build PND-013 after 048b and before S3. Keep IFC-049's requestState checks before every return
check; a return uses only a tools/call retry from the receiving Interface in its bound session
and invocation. No Agent sees the sign-in URL, login metadata or return. Retry only the hook
that started that login, with its own requestState, accept and signInReturn {value}; record
that part without the value. Every diagnostic names its rule and cause, with no value, code,
state, URL value, error_description or error_uri. Polling and the account check still decide.

| Planned case | Promises |
| --- | --- |
| redirect-delivered | PND-013, IFC-006, IFC-049 |
| page-delivered | PND-013, IFC-006, IFC-049 |
| not-carried | CTR-013, PND-013 |
| device-not-accepted | PND-013 |
| service-not-accepted | PND-013 |
| state-mismatch | PND-013 |
| address-mismatch | PND-013 |
| issuer-mismatch | PND-013 |
| malformed | PND-013 |
| other-session | PND-010, PND-013, IFC-049 |
| after-wait | PND-010, PND-013, IFC-049 |
| twice | PND-013 |
| refused-not-used | PND-013 |
| oauth-error-passed-on | PND-013, IFC-049 |
| undelivered | PND-013, PND-011 |
| never-recorded | PND-012, PND-013, REC-002 |
| hook-retry-recorded-without-value | PND-013, REC-002 |
| return-on-first-request | PND-013, OPS-023 |
| return-on-other-operation | PND-013, OPS-023 |
| return-under-tasks-update | PND-013, IFC-016, OPS-023 |
| return-with-decline | PND-010, PND-013 |
| injected-other-account-undone | CTR-013, PND-011, PND-013 |

Exercise each ordered check and reason: not-accepted (including a plain hook),
already-returned, malformed (length, printable ASCII, parse, user information, fragment,
repeated parameters), address-mismatch (scheme, host, port, path), state-mismatch and
issuer-mismatch. Cover issuer absent and present; all three returned values; redirect and
page with carried true and false; each via; cancel as well as decline; a refused value then a
valid value; a used value even after hook failure; error code normalization; kept query fields;
expired and consumed requestState; and the listener's matching Host, path and state and stop.
Test listener selection off the node, on the node and where placement is unknown or bind fails.
The Interface receives results only for its own sign-in, and the initiating hook alone gets
its value. The return is lost at restart and never serialized in responses or persisted.

## Agent packages, order 050

Claude Code Types state each login with carried: false. In v0 a Claude Agent signs in only
with a browser on its own machine (ruling 75 C3).

Codex's browser login is redirect with carried: true; its device login is device. The hook
starts the browser login unless the person asks for the device login, which stays an option
where the account has it on (ruling 75 C5). Read the client IDs, authorization origins, hosts,
ports and paths from the clients in order 050; none is guessed here. Account hooks must learn
the service-attested account, never a configured or person-supplied claim (CTR-013).

The real-client probe must confirm that a carried redirect completes Codex's login and that
the account check is right (75 C5). Contrary to this order's premise, the current tree already
has its roadmap row, A81, before S3. It remains evidence to obtain, not a result of this order.

## CLI

Open a loopback listener for one login only where the CLI knows it is off the machine of the
node signInLogin names. On that machine the program's listener receives it. Where uncertain
or unable to bind, use paste. Match Host, path and state and stop after the matching request
or DL18. Read a paste from the terminal with echo off, never an argument, standard input or
the environment. The general conduit call command refuses to set signInReturn. No model
output supplies a return (PND-013); the engine cannot detect a console that breaks that rule.

Show logins at install and in inspection. A named confirmClientIds flag is counted when used
(ruling 55 A4); the CLI order chooses its name. This handoff invents no flag or command.

## Safety catalogue handoff

The admin adds PND-013 as a convention-layer promise (75 E1), with the checks, one-use rule,
restricted destination, transient value, receiving Interface and listener limits above.
The existing quoted excerpts of CTR-004, CTR-013, PND-004, PND-005, PND-010, PND-012, IFC-006,
IFC-049 and REC-002 are preserved verbatim; none requires a changed quotation. DL18 now also
bounds the return. The catalogue is outside this order's scope.

Proposed new classification row for the admin to review:

| ID | layer | level | kind | why a breach is unsafe | S1 traces |
| --- | --- | --- | --- | --- | --- |
| PND-013 | convention | must | secret | `observable`: “The value appears in no result, structuredContent, response _meta, notification, diagnostic, log or record” — disclosing the return exposes authorization material outside the initiating login. | — |
