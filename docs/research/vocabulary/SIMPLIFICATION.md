# Simplification pass

Status: reviewer feedback on the [candidates](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/CANDIDATES.md), 2026-11-05. Nothing here is adopted;
rulings still go through the [vocabulary map](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md#review-and-migration-procedure).
This proposes fewer names for the same accepted behavior. It does not merge any behavior.

**Outcome.** The owner adopted this pass as the base of the final vocabulary on 2026-11-05
(plan-audit rulings 17 and 18, decision V1 of the
[vocabulary record](../../proposals/vocabulary/README.md#decisions)). The record marks each word
that came from here with the source **S**, and lists the five places where the ruling went
another way under
[where this differs from the simplification pass](../../proposals/vocabulary/README.md#where-this-differs-from-the-simplification-pass).
Later rulings changed a few of the words again; the [specification](../../../spec/definitions.md)
has the current ones. This file is kept as written, as the record of the reviewer's
contribution (commit `2890ef7`, [PR #5](https://github.com/shrav-k/conduit/pull/5)).

The candidate list keeps several words for one idea, and reuses one word for several ideas. Three
patterns cover most of it. Solid boxes are the words to keep; dashed boxes are the ones to drop.

```mermaid
flowchart LR
  subgraph A["1 · One idea, many names: what does the work"]
    direction LR
    plugin["plugin"] --> impl
    ib["implementation binding"] --> impl
    driver["driver (of a runtime)"] --> impl
    handler["handler (of a control)"] --> impl
    contrib["contribution"] --> export
    bundle["bundle"] --> pkg
    impl(["implementation"])
    export(["export"])
    pkg(["package"])
  end

  subgraph B["2 · One word, many meanings"]
    direction LR
    binding["binding"] --> vb(["value binding (graphs only)"])
    binding --> sel(["implementation selection"])
    binding --> rid(["resolved ID"])
    binding --> pol(["policy"])
    attach["attach"] --> join(["join / leave (Conduits)"])
    attach --> useex(["use existing (Resources)"])
    attach --> conn(["connect (frontends)"])
    ref["reference"] --> gref(["reference (graphs only)"])
    ref --> handle(["handle (live values)"])
    prep["prepare"] --> effect(["name the actual effect"])
  end

  subgraph C["3 · Pairs that are one concept"]
    direction LR
    ap["access path"] --> route(["route"])
    est["establish"] --> setup(["setup"])
    exe["execution"] --> inv(["invocation + attempt"])
    pr["public result / port"] --> io(["input / output"])
    rb["requirement binding"] --> pol2(["policy"])
    co["constituent / obligation"] --> req(["requirement"])
    del["delegation / issuance"] --> grant(["grant: issue / revoke"])
    mr["managed reference / runtime target"] --> handle2(["handle"])
    act["activation umbrella"] --> load(["load / expose"])
    form["form"] --> gone(["drop it"])
  end

  classDef drop stroke-dasharray: 5 4;
  classDef keep stroke-width: 3px;
  class plugin,ib,driver,handler,contrib,bundle,binding,attach,ref,prep,ap,est,exe,pr,rb,co,del,mr,act,form drop;
  class impl,export,pkg,vb,sel,rid,pol,join,useex,conn,gref,handle,effect,route,setup,inv,io,pol2,req,grant,handle2,load,gone keep;
```

With pattern 1 applied, the plugin-versus-implementation question in [V05](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/protocol/MODEL.md#v05)
mostly goes away. "Plugin" can stay as a casual word for "a package with implementations," but not as
a separate primitive.

## Keep these separate

These look repetitive, but each pair behaves differently when something fails:

- **provision / start / stop / unregister / purge:** creating isn't starting; stopping isn't deleting data.
- **contract version / configuration revision / package release:** meaning versus concurrency versus publication.
- **contract vs schema:** valid data isn't correct behavior.
- **capability vs grant:** offering something isn't permission to use it.
- **snapshot / feed / cursor:** current state, the change stream, and your position in it.
- **invocation context vs metadata:** trusted facts versus caller claims. Merging them is a security bug.

Open question: fold **retire** into **stop** only if the spec says stop always carries retire's
cleanup duties.
