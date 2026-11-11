# Increment 4: how the author's work on S1 is measured

Not binding. The [specification](../../../../spec/README.md) alone binds. Written on 2026-11-10,
before the coordinator does S1's tasks on the engine, so that the counting is fixed before any
number is known. It applies [the measures](../../../implementation/FIRST_BUILD.md#measures) and
their [counting rules](../../../implementation/FIRST_BUILD.md#counting-rules-added-after-the-baseline-ran)
to the build, in the way the baseline (A70,
[its results](../../../../experiments/007-s1-baseline/results-2026-11-09.md#how-each-measure-was-counted))
applied them to the agents' own configuration. Where the run departs from this page, the
results say so.

## Who and what

- **Two authors, apart.** The coordinator does T1 to T7 first. It knows the design, so its
  numbers are a floor on what the language costs someone who knows it. A fresh Codex author
  then does them with only the specification, S1's tasks and what the engine ships
  (ruling 48, point 4). The two are reported side by side, never merged.
- **The engine and the clients.** The `conduit` binary as work order 025 leaves it; Claude Code
  2.1.318 as agent A and Codex 0.167.0 as agent B, each through the Agent package the build
  ships; Node 21.5.0. The filesystem server is 2.1.0 at T1 and 2.2.0 after T5, as in the
  baseline.
- **A lab of its own.** A fresh store and fresh shared and history directories, outside the
  repository, written `<lab>/` in the results. The project directory the author writes starts
  empty: the author does not copy `reference-model/examples/s1/`.

## What the author writes, and what is provider work

- **Authored**: every file of the author's project (its manifest, templates and instances, and
  any document submitted later), and every command the author types that carries a value the
  author chose. Lines are non-blank lines added, changed or removed, comments excluded, as in
  the baseline; a document separator `---` is a line. The package digests in the manifest are
  copied from a tool's output: they are counted as lines and listed apart, so a reader can
  subtract them.
- **Provider work, counted apart**: the five packages the build ships, the filesystem package
  in 0.1.0 and 0.2.0, the history's, and the two Agent packages with their wrappers. Each is
  measured by the same measures once, and listed apart, with the note that A70's harness flags
  are the same kind of work and were not counted.
- **Not counted**: the commands of the harness that do no author's work: building the binary
  and the packages, starting `conduit serve`, and each check. A check is a turn requested with
  `conduit call` or a `conduit tools --as`, as in increment 3's closeout.

## How each measure is counted for the build

| Measure | For the build |
| --- | --- |
| Files edited | Each file of the author's project created or changed in the task |
| Lines written | As above; each command the author types that is not a check is listed with its lines |
| Concepts met | Each term of `spec/definitions.md` the author must use or understand, numbered when first needed; then, apart, each concept of Claude Code or Codex the author must use, unless an Agent Type the build ships carries it; then each of the engine's commands, as the baseline counted a tool's flags |
| Grants made by hand | Each declaration that gives one Agent access to one thing: a declared Conduit in an Agent's template, or a standalone Conduit whose consumer is an Agent. A Conduit whose consumer is the root Interface gives the person access, not an Agent, and is listed apart |
| Adapters written | Each wrapper, shim or script the author writes, in files and lines |
| Duplicated configuration | Each value written in more than one place, on the author's files at the task's end |
| Per-agent changes of access | The files edited and the requests made to change what one Agent can reach, A and B apart |
| Inspection effort | For T3, the files read and commands run to answer it, and the answer's coverage of reach around the engine |
| Provider-specific code | Each line in the author's files naming a backing tool: the npm server, a tool name only it defines, or a client by its own name. A Type's address, such as `example.org/s1/filesystem#local-directory.v1`, names a contract, not a tool, and is counted apart so the reader can judge |
| Recovery steps that need a person | Each manual action after a failure to reach the intended state |
| Observed time and mistakes | From the task's start line in the author's log to its end line, checks included; each mistake and how it was found |

The counting rules after the baseline apply as written. In particular:

- **Requests and time without access.** Every request the author makes, by command, and every
  interval in which an Agent has no access it should have, taken from the records: for T4 and
  T7 the time from a `close.v1` to the matching `open.v1`. The baseline's restarts and lost
  sessions are listed beside them.
- **T4 and T5 against the better baseline too.** Compared with the baseline's numbers, and with
  an allow list in each client and the server's `--read-only`, as the rule says.
- **T3 covers reach around Conduit.** The answer gives what each Agent reaches by its own means,
  from what the build ships and states, and counts what it took to find.

## The S1 rerun

The rules above describe the original runs. For the rerun,
[ruling 63](../../ruling-63/README.md), point 1, fixes these counting rules:

- lines that `conduit install` writes (package pins) are written by a tool, not by the author,
  consistent with ruling 53's decision 5;
- a command counts as a line only when it carries a value the author chose, such as a path or a
  name; a bare `conduit apply` does not;
- a line removed and later restored within one task counts once, the baseline's own rule;
- the block-style reading and the strict reading (every command a line) are reported beside the
  primary one.

Packages' standard Conduit templates are counted as provider work (ruling 63, point 3).

## The routes for T4 to T6

The owner chose to run these by the routes the specification allows today
([finding 4-01](../log-increment-4.md)). The author picks the route at the task and records
why. Each route's cost, sessions lost and tool names changed are recorded with the measures,
since they are what 4-01's decision needs.

## What the checks show

Each task ends when the real clients show its intended state, as in increment 3's closeout:
each Agent's engine list, a read or write through the engine by a turn, a refusal where one is
intended, and for T5 that `append_file` and `copy_file` reach no Agent. Each check's evidence is
kept, redacted of the lab's path and of any token.

## Options for the rerun's counting rules

Ruling 63 settles the primary chosen-value command reading, tool-written package pins,
remove/restore counted once, strict and block-style side readings, and package templates as
provider work. These are settled rules above, not options to put again to the owner.

The earlier papers leave these measurement questions or sensitivities to state before the
rerun; none is settled by this addendum:

- The short-form critic’s [style sensitivity](../../language-shape/short-form/critic.md):
  T1 is 32 lines with block maps/flow operation lists, about 38 in idiomatic block YAML.
  It recommended flow style for init/render; report the style used and the block reading.
- Its gap 10 asks how the fresh author discovers the short form, Types, operations and config
  keys, and asks for requests, windows without access, concepts, duplicated values and contract
  address lines beside the line count. Framed examples alone may lead an author to canonical JSON.
- The simplification [critic](../../simplification/research/critic.md), gap 3, asks which run
  judges the bar: it recommends coordinator versus the experienced baseline author, with the
  fresh run measuring learnability. That comparison choice remains to be stated explicitly.
- Gap 18’s paper sensitivity: three separate installs instead of one made T1 18 in that paper;
  actual rerun commands and mistakes must be counted. Ruling 63’s command rule now governs.
- The earlier synthesis proposed a JSON side reading and counted the history convention as
  provider work. Ruling 63 explicitly settles templates and block/strict readings, not those
  two proposals; keep them labelled proposals pending the rerun protocol.
