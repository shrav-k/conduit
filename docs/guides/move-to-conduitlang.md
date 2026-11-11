# Moving to ConduitLang

Not binding. The [specification](../../spec/README.md) alone binds. The steps to move the project to
its public name, ConduitLang ([ruling 65](../proposals/ruling-65/README.md)), in the order to take
them. Steps 1 to 3 need the owner's own accounts; steps 4 to 7 are the coordinator's once those
exist. Availability was checked on 2026-11-10 and can change: do step 1 first, since it is free and
fixes the names everything else depends on. Never paste a password or token into a chat with an
agent.

The language stays **Conduit**. The organization, domain and package prefix are **conduitlang**,
one spelling everywhere, as golang, ziglang and cuelang do.

## 1. GitHub organizations (free; do first)

1. On GitHub, signed in as the repository's owner: your avatar, then **Your organizations**, then
   **New organization**, and choose the **Free** plan.
2. Name it `conduitlang`. Contact email: your own. It belongs to your personal account.
3. Make a second, empty organization, `conduit-lang`, only so that no one else takes the hyphenated
   spelling. Put nothing in it.
4. **Transfer the repository**: in `shrav-k/conduit`, **Settings**, then **General**, **Danger Zone**,
   **Transfer ownership**, to `conduitlang`, keeping the name `conduit`. GitHub redirects every old
   URL, the dated permanent links in this repository's documents included, as long as no new
   repository is ever made at `shrav-k/conduit`. Collaborators keep their access; check their roles
   afterwards under the organization's **People**.
5. Tell the coordinator it is done. Its local checkout's remote is updated with
   `git remote set-url origin https://github.com/conduitlang/conduit.git`.

## 2. The npm organization (free)

On npmjs.com, signed in: your avatar, **Add Organization**, name `conduitlang`, the free plan for
public packages. This reserves the `@conduitlang/` scope. No package is published now.

## 3. Domains (paid; optional and can wait)

About 10 to 15 US dollars a year each, at any registrar (Cloudflare Registrar sells at cost; others
include Namecheap and Porkbun). In order of value:

1. `conduitlang.dev`, the main domain (`.dev` requires HTTPS, which every host provides).
2. `conduitlang.org` and `conduitlang.com`, redirected to the main one.

At checkout the registrar shows whether a name is premium or reserved, which a lookup cannot. Until a
domain exists, the identifiers come from the GitHub organization (rulings 33 and 34), so nothing
waits on this step.

## 4. The identifiers (the coordinator, after step 1)

The names the specification assigns are defined once, in `spec/names.yaml`, and one tool rewrites
them everywhere:

```sh
python3 tools/set_names.py --github conduitlang/conduit          # shows what would change
python3 tools/set_names.py --github conduitlang/conduit --write  # rewrites spec/, engine, conformance, packages
python3 tools/set_names.py --check
```

They become `github.com/conduitlang/conduit/v0` (`apiVersion`), `github.com/conduitlang/conduit/core`
(the core package) and `io.github.conduitlang.conduit/` (the `_meta` prefix); the old values move to
`retired`, which the check then refuses. Package digests and conformance fixtures that embed a name
are rebuilt in the same change, every check and test run, and the result merged as one pull request.
If a domain is bought later, `--domain conduitlang.dev` moves the names to it the same way.

## 5. Crates (nothing to do now)

crates.io names are claimed by publishing, and placeholder crates are discouraged. The first crate
published uses the prefix `conduitlang-` (for example `conduitlang-engine`). `conduit` and
`conduit-core` are taken by other projects.

## 6. Documents and the organization's own setup (the coordinator, after step 1)

- Every document that names `shrav-k/conduit` is updated (`git grep -n shrav-k/conduit`); dated
  permanent links may stay, since GitHub redirects them.
- Ruling 64's waiting items: the organization's `.github` repository (contribution guidance, a
  security contact, issue templates), `CODEOWNERS`, and branch protection that requires the CI
  checks.

## 7. Before any public launch: a trademark check

Live and pending software trademarks on "CONDUIT" exist. Before v0 is published, check the
language's name with a trademark search (in the United States, the USPTO's search) or a lawyer. The
risk attaches to the language's name whatever the organization is called.
