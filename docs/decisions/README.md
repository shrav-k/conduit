# The owner's decisions

Status: draft, not yet in force. Until [AGENTS.md "What holds"](../../AGENTS.md#what-holds) names
this directory, the rulings and [IN_FORCE.md](../../IN_FORCE.md) hold as before, and nothing here
binds. The owner adopted this framework on 2026-11-20
([DEC-001](log.md#dec-001-2026-11-20-the-decision-framework)). Its homes are:

- [`spec/`](../../spec/README.md) binds alone, and [spec/CHANGES.md](../../spec/CHANGES.md) has one
  line per change to it;
- [HOW-WE-WORK.md](../../HOW-WE-WORK.md), at the root, holds the rules of the work, one per line;
- [the log](log.md) holds the owner's decisions;
- [ROADMAP.md](../../ROADMAP.md) holds the work still to do;
- git history holds every retired file.

## The log

One entry per decision the owner confirmed, numbered `DEC-001` and up. Archived rulings keep
their numbers, are cited as "ruling N", and never get a new one.

**Append-only.** Once merged, an entry never changes, except its Status line: "accepted, not done"
may become "done", naming the pull requests. A correction is a new entry that cites the old one.

**Only the owner's decisions** are logged:
[HOW-WE-WORK rule 6](../../HOW-WE-WORK.md#1-the-rules-of-the-work).

**Private entries.** An entry about the owner's own accounts or machines says only "private, held
by the coordinator".

### The entry form

At most 1.5 KB:

```markdown
## DEC-NNN, YYYY-MM-DD, a short title

Owner's words: question -> "the answer, verbatim"

Lands in: promise IDs in spec/ | a HOW-WE-WORK.md line | a ROADMAP.md row

Status: done | accepted, not done (what is missing)
```

Text before "->" names the question as it was put. Only quoted text is the owner's: the shortest
passage that carries the decision. "(Rec.)" or "(Recommended)" marks the option the coordinator
recommended.

## Directions

There are no frozen records. A direction that `spec/` cannot hold lives in its log entry, plus one
HOW-WE-WORK line or one ROADMAP row where it shapes the work.

## The archive

Git is the archive: [HOW-WE-WORK section 8](../../HOW-WE-WORK.md#8-records-where-each-fact-lives),
in the form `spec/README.md` uses for removed promise files.
