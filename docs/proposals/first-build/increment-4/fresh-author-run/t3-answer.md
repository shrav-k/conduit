# Reach at T3 (before the T4 permission swap)

| Agent | Shared directory Resource `directory` | Shared history Resource `messages` |
| --- | --- | --- |
| A, Claude Code (`a`) | `a-directory`: list and read under `shared/`; no directory write operation | `a-history`: list, read, write under `history/` |
| B, Codex (`b`) | `b-directory`: list, read, write under `shared/` | `b-history`: list, read, write under `history/` |

All paths are under `<lab>/`.
Both Resources are owned by Pool `team` and placed on `local`. Ownership grants no access
(ACC-028). The root Interface has the management/access routes and requests their turns;
neither Agent has a Conduit to management, access, the Pool, the other Agent, or any secret
Resource. No implicit administrative rights come from the incoming turn Conduits.

`directory` holds filesystem 0.1.0, pinned by SHA-256
`200de7c103aa560201dab8821e974088ce470e76c859c86aa01605cdf72d2160`.
`messages` holds history 0.1.0 and its filesystem 0.1.0 dependency. Each server is configured
with its single absolute root. Both Agents actually read `shared/hello.txt` through their
directory route and were denied that same path through their history route (t3-a/b evidence).
T1 also demonstrated provider denial of `TASKS.md` outside `shared/`; A had no write tool,
and B wrote and read `shared/b-t1.txt`. T2 demonstrated posting and cross-reading by both.

History is a convention over filesystem writes: fresh `<UTC>-<agent>-<suffix>.md` files read
in filename order. The provider does not enforce append-only behavior, author identity,
uniqueness, or timestamps. Both Agents technically can replace history files through their
write operation; they followed the requested new-post convention. No access filter narrows
history writes to an author or a filename.

Both Agent configurations set `accountServices: false`, `peerMessaging: false`, and resolve
`model` to the empty default (native client default, no explicit model selected). A uses the
pinned agent-claude-code 0.1.1 package; B uses agent-codex 0.1.1. The package documentation
says the wrappers disable native shell, browser, hooks/plugins and account connectors and
use the owner's existing subscription login, without OS isolation. A reported only its five
engine tools and no native tools. B reported its six engine tools plus native code-mode,
apply_patch, resource-discovery, clock, user-input and collaboration tools. No shell, browser,
account connector or engine administrative tool was reported by either.

The B package explicitly says `peerMessaging` changes no Codex argument. Despite its
`multi_agent` disable argument, B actually invoked native `collaboration.list_agents`
successfully (one current-thread Agent). Thus native collaboration discovery is a demonstrated
route outside Conduit. No other Agent was spawned and no peer message was sent; spawning,
messaging and other listed native tools were not tested. B's native apply_patch attempt to
create `shared/native-b-t3-probe.txt` failed with a read-only sandbox denial; the file does not
exist. Tool presence alone does not establish that every listed tool works.

This is mediated access (ACC-027): engine restrictions apply to requests through Conduit.
The filesystem root is a provider restriction, not a general engine path filter. Shared OS
identity, run-credential access described by the packages, and the shared login preclude an
OS-isolation claim. No credentials or owner configuration were inspected. These experiments
establish the tested routes; they are not a proof that every native bypass is impossible.

Evidence is saved under `project/evidence/`: t2-tools-a/b, t2-state-a/b/directory,
t3-state-messages, t1-a/b, t2-a-post, t2-b-post-read, t2-a-read-b-retry, t3-a/b,
and t3-b-native-check. Engine catalogs are authoritative for engine tools. Claude's first
startup catalog was current; later startup lists are explicitly session_init_unverified and
unavailable. Codex returned not_reported. Neither model inventories nor repeated startup
lists are substituted for a current engine catalog. Inspection also shows ready and successful
setup while retaining old Running=False/RunStopped conditions; those stale conditions do not
negate the successful actual turns.
