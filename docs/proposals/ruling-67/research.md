# Research behind ruling 67: how projects record decisions, and what agents read

Not binding. The working paper behind [ruling 67](README.md), condensed on 2026-11-10 from two
research reports, a synthesis and a critic, run as a workflow. Claims a researcher could not read
at the source were dropped.

## What other projects do

Every mature process keeps a *record* of each decision, mostly frozen once decided, apart from a
*living reference* that says what holds now and cites the records.

- **Architecture decision records** (Nygard, 2011; MADR; adr-tools): one short file per decision,
  numbered and never reused; a reversed decision is kept and marked superseded. "Large documents are
  never kept up to date." adr-tools writes both ends of a supersession link mechanically. Microsoft's
  and AWS's guidance treats accepted records as append-only; some teams find editing works better.
  The tradition has no document of what holds now: its known gap.
- **Rust**: RFC 1636 (2016) found that "RFCs are a poor source of information about language
  features for the ordinary Rust user" and made a Reference entry a condition of stabilization. The
  RFC repository "is not intended as a reference manual".
- **Python**: PEP 1 calls a resolved PEP "a historical document rather than a living
  specification"; the language reference is maintained elsewhere. PEPs carry Replaces and
  Superseded-By headers, and PEP 12 a banner pointing at the current documentation.
- **IETF**: an "Updates" document "cannot stand on its own", so readers chain amendments; the
  untyped tag drifted in meaning. Stable series numbers (BCP, STD) outlive the documents behind
  them.
- **Go, TC39, Kubernetes, Apache**: a living specification or table, one short row per decision,
  and a topical index over chronological minutes.

## What helps an agent

- Keep the always-loaded file small and load the rest on demand by path. Claude Code's memory
  guidance targets under 200 lines per `CLAUDE.md`, and an `@` import costs its full size in every
  session. Codex stops reading `AGENTS.md` files past 32 KiB.
- Contradictory instructions let a model pick either. Text that is related but outdated distracts
  more than unrelated text (Chroma, "Context rot", 2025): superseded rulings are such text.
- Write each rule whole, with its scope and exceptions in it, and delete cautions whose reason has
  passed; keep dates and the state of the work out of standing rules.
- A banner in each historical file reaches an agent that arrives by search, whichever client it is.
- Context files help when they hold specific instructions, not overviews (arXiv 2602.11988).

## What the critic changed

The synthesis proposed a file named `DECISIONS.md`, line identifiers, ten points for the owner and
an evaluation gate. The critic found: the name is already used by six files under
`docs/proposals/`; identifiers clash with the promise prefixes; a coordinator's line must not outrank
the owner's words; the pending rulings were already listed twice, in `AGENTS.md` and
`spec/README.md`; several rulings share batch records or have none; and a one-run evaluation is
noise. Ruling 67 takes each correction.
