Not binding. The [specification](../../../spec/README.md) alone binds. The research behind the
name of the agent-owning family, 2026-11-10: four generators (agentic and organizational metaphors,
prior art in agent frameworks and infrastructure, Conduit's own imagery, and the constraints of the
vocabulary), a shortlist, three judges per candidate and a critic. [Ruling 60](../ruling-60/README.md)
chose the name.

## Scores

| Candidate | Fit | Collisions (10 = none) | Future |
| --- | --- | --- | --- |
| Fleet | 7.5 | 6 | 7 |
| Flotilla | 4 | 7 | 5 |
| Colony | 5 | 5 | 5 |
| Homeport | 4 | 3 | 3 |
| Hangar | 4 | 6 | 3 |
| Unit | 3 | 5 | 5 |
| Realm | 4 | 5 | 3 |
| Project | 4 | 2 | 5 |

## The recommendation

RECOMMENDED: Fleet.

Why:
- It is the only candidate that scores well on all three judgments: fit 7.5, collisions 6, future 7. Every other candidate fails the owner's agentic ask, has a ruled collision inside the repo, or both.
- It answers the owner's request directly and with current industry wording. IBM writes "your agentic fleet", LangChain renamed Agent Builder to LangSmith Fleet, and decommissioning agent fleets is now a common topic.
- It does this through an existing paradigm (ruling 40), not a coined image. Fleet management is owning, administering and retiring units that act for themselves, which is the administer-versus-model-decides split this family exists for.
- It implies no people (ruling 15), is short, sits beside Pool without blurring, and is free in the repo, so ruling 42's one meaning per word holds.
- It settles the reason the owner raised: "project" keeps its submission meaning, and the A5(a) rewording never happens.

The trade-off, stated plainly. Fleet's weakest axis is collisions (6). Its outside meanings all point against the family's rules: a fleet is dispatched or run (use), configured all at once (applied to every member), and in infrastructure it is usually machines. None of this breaks a ruling, but each needs a written disclaimer and discipline in prose. The definition row must say:
- owning permits nothing; running an Agent's turns takes a Conduit to the Agent;
- a Fleet applies nothing to its Agents together, runs nothing and holds no count of copies;
- it is engine-local, has no placement, contains no Pool, and is not a group of machines (Conduit groups machines with a Pool and labels);
- an Agent never changes Fleet.

Prose must never say "fleet operator". The short-form `operator:` key is root's turn Conduit, which is use, so pairing it with administration would invite exactly the confusion the language is built to avoid. The S1 rerun with a fresh author is the place to check whether `fleet: s1` is misread as granting use. If it is, the disclaimers are not enough.

RUNNER-UP: Flotilla, as a fallback only. Choose it if the owner decides Fleet's machine meaning (fleetdm, GKE, enrollment, ruling 51's managed machines) is decisive. It wins on collisions (7 against 6) and loses clearly on fit (4) and future (5): its size meaning, its "command" image, and the principle-2 exception it needs. The gap between first and second is large.

If the owner rules out ship imagery altogether, or holds principle 2 strictly, I would not offer Colony. Its shared-nest meaning gives readers the wrong model of access. The honest fallback is then a word with no agentic meaning: Unit if keeping "project" for submissions matters more, or keeping Project with A5(a) if prior art matters more. Neither meets the owner's ask.

## The ranking

Scores are fit / collisions / future, out of 10. Before ranking I checked three facts in the repo (read-only). "fleet" appears only in docs/proposals/enrollment/README.md (lines 78 and 255, where it names the fleetdm product) and nowhere in spec/. Vocabulary principle 2 is real: "Plain technical words for authoring and operation; flow imagery only around Conduits" (docs/proposals/vocabulary/README.md:45). No promise prefix FLEET- or FLT- exists yet; POOL- does.

1. FLEET (7.5 / 6 / 7, mean 6.8)
- Best point: it is the only candidate that gives the owner an agentic name and is also an existing paradigm. "Fleet management" means registering, setting up, watching and retiring units that run themselves. That is the family's split: the Fleet administers, and the Agent's model decides. It pairs well with Pool: a pool is a store of passive things to draw on, and a fleet is a set of self-propelled units under one administrator. It implies no people (ruling 15). Because it is a plain industry term, it arguably fits principle 2 without an exception. It is free in the repo, and it leaves "project" with its meaning.
- Worst point: its outside meanings pull against the family's rules. LangSmith Fleet grants run and edit. Copilot's /fleet means parallel subagents. fleetdm, GKE and Azure fleets roll policy out to every member. Those three readings work against "owning grants no use", "nothing applies to all members" and "the family runs nothing". The word also suggests machines (fleetdm hosts, GKE and Azure clusters, EC2), which sit right beside nodes and enrollment. The phrase "fleet operator" also collides with the short-form `operator:` key, which is root's turn Conduit and therefore use.
- In the promises: "An Agent's owning Fleet is fixed when the Agent is instantiated, and no operation changes it." Ruling 30 becomes "its owning Pool for a Resource, its owning Fleet for an Agent, `management` for everything else." A3 becomes "Root holds a Conduit to every Pool and every Fleet; a Pool and a Fleet may not share a name."
- Short form: `fleet: s1`, and `fleets: conduit-core: holds: ...`.
- CLI: `conduit fleet list`, `conduit fleet retire s1 --with-agents`.
- Names: `core#fleet-management.v1` beside `pool-management.v1`, and promise IDs FLEET-001 to mirror POOL-001.

2. FLOTILLA (4 / 7 / 5, mean 5.3)
- Best point: it has the cleanest collisions of the agentic words. It is not in the repo, it has no major technical meaning, and it leaves "fleet" free for machines.
- Worst point: it barely answers the request. It highlights ships, not agents. Its size meaning is wrong at both ends: the owner's packages file has one Agent, and fifty would be a fleet. "Under one command" suggests the group directs its Agents, against "owning grants no use". As pure imagery it needs an explicit exception to principle 2. It is easy to misspell, there is a small `create-flotilla` agent orchestrator in Conduit's own niche, and the Gaza flotillas give it a news association.
- In the promises it is mechanically fine ("an Agent's owning Flotilla", "a Conduit to the Flotilla"), but long: `flotilla: s1`, `conduit flotilla list`, `flotilla-management.v1`.

3. COLONY (5 / 5 / 5)
- Best point: it is the most strongly agentic word (the ant colony is the textbook multi-agent system). Membership for life matches A1, and it is distinct from Pool and implies no people.
- Worst point: a colony suggests a shared nest, stigmergy and shared memory. That gives readers the wrong model of access, the one area Conduit innovates in (ruling 40), and contradicts "private and shared stay separate". "Root holds a Conduit to every colony" reads as colonial rule. "A colony of one" sounds wrong. The Colony is an MCP server whose `colony_*` tools sit in the same client ecosystem, and ColonyOS uses the word for executors spread across machines.
- Reads as `colony: s1`, `colonies:`, `conduit colony list`. The promise prefix would need to avoid COL-, which sits next to CON-.

4. UNIT (3 / 5 / 5)
- Best point: the organizational-unit paradigm (AD and AWS OUs) matches the v1 authority model best: delegated administration, granular Interfaces, names per container (C8), and staying inside one engine.
- Worst point: it has no agentic meaning, and the bare word loses the OU precedent. "unit" already appears in normative spec text as a systemd unit (spec/promises/providers.yaml:472, spec/implementation-defined.yaml:288), which there means a single run, the reverse of a group. `unit: s1` reads as a unit of measure, and "unit" commonly means one item.
- "An Agent's owning Unit" sounds generic, and `conduit unit list` reads flat.

5. PROJECT, the baseline of keeping the name (4 / 2 / 5)
- Best point: it has the strongest prior art (OpenAI, Azure AI Foundry and GCP projects) and the most natural CLI (`conduit project list`, compare `gcloud projects list`).
- Worst point: it collides with a term already in force (spec/definitions.md:16 and :65). The word has about 347 uses in spec/, about 1,900 in conformance/ (including the `register-project` step kinds), and appears in the Rust `struct Project` (reference-model/crates/core/src/packages.rs:16 and loader/src/project.rs:57). A5(a) would become mandatory. S1's own short file would hold three meanings of the word. It also suggests the "homes every object" model and member lists the owner rejected, and it has no agentic theme.
- Reads as "an Agent's owning Project", `project: s1`, `conduit project list`.

## Consequences

If Fleet is chosen:

1. "project" keeps its meaning (ruling 59 A5 closes as "keep")
- spec/definitions.md:16 and :65 (SUB-005), "submitted project", "a package is a project that has been published", the "project's address" (MAN-015) and the `#name` scope (NAM-022, NAM-027) all stay as they are.
- Nothing is reworded: none of the roughly 49 to 55 promises, none of the conformance step kinds (`register-project`, `resolve-project`, `project-documents`, `project-manifest`), and neither Rust `struct Project` (reference-model/crates/core/src/packages.rs:16, reference-model/crates/loader/src/project.rs:57). All of these already use the submission meaning.
- Optional hygiene: S1's Conduit is named `project` (docs/proposals/projects/brief.md:466, 472), which gives tools like `project.file.read.v1`. That is a local name the author chose, not a spec term, so ruling 42 does not force a rename. Renaming it to `shared` or `repo` would remove a codebase meaning of "project". But the S1 baseline prompts name "the server named project" (experiments/007-s1-baseline/evidence-2026-11-09/prompts.txt), so renaming costs comparability with the baseline. Decide that separately.

2. The family's own names
- `kind: Fleet`, templates `core#fleet` and `core#fleet.v1`, the capability `core#fleet-management.v1` (beside `pool-management.v1`) and `fleet-reference.v1`.
- The brief's promise prefix PRJ- becomes FLEET-, mirroring POOL-. No FLEET- or FLT- prefix exists in spec/.
- Unchanged: `retire-with-agents.v1` (it names the Agents, not the family), `list.v1` and `act.v1`. Acting for a Fleet keeps the A3 exclusion: "every operation except the cascades and acting for a Fleet".
- Ruling 30's line becomes "its owning Pool for a Resource, its owning Fleet for an Agent, `management` for everything else".
- A3 becomes "Root holds a Conduit to every Pool and every Fleet; a Pool and a Fleet may not share a name".

3. Short form, CLI and tool names
- Short form (A6): the top-level key `fleet: s1` owns every Agent in the file. The section `fleets: <name>: holds: ...` holds its Conduits. An Agent declared elsewhere states `owner: <fleet>`.
- CLI: `conduit fleet list`, `conduit fleet inspect <f>`, `conduit fleet retire <f> --with-agents`. The cascade flag stays explicit, as A3 and ruling 55 require.
- Tool names barely change, because their prefix is the target's `as` name (`s1.list.v1`, `conduit-core.trees.make.v1`), not the family word.

4. spec/definitions.md
- Add a Fleet row: "one of the six families: what owns and groups Agents, as a Pool owns Resources. Ownership permits nothing; using an Agent is a Conduit to the Agent. A Fleet runs nothing, applies nothing to its Agents together and holds no count of copies. It is engine-local, has no placement and contains no Pool. It is not a group of machines (a machine is a node)."
- The owner row changes from "The Pool instance that owns a Resource" to "the Pool or Fleet instance". Any name would require this.
- Note that "fleet" carries only this meaning; outside products (fleetdm, GKE Fleet, Azure Fleet Manager, LangSmith Fleet, Copilot fleet mode) are cited by name. Relabel docs/proposals/enrollment/README.md:78 as "Fleet (fleetdm)".
- Prose rules: never "fleet operator", never "dispatch" applied to a Fleet (dispatch already has a request meaning in definitions.md), and never "the fleet" meaning all Agents.
- Separately, the brief's line 180 ("A viewer or operator is a Conduit to P") already uses "operator" for administration, while the `operator:` key at line 443 is use. Reword it whatever the name.

5. Records
- Ruling 59's record ("calls the family 'Project' until its name is chosen") and the brief's PRJ- IDs and "P" shorthand are updated when the name is ruled.
- Record whether Fleet counts as a plain technical word under vocabulary principle 2 (my reading) or as an exception, so the owner rules on it explicitly.

## Excluded candidates

Crew: CrewAI's brand, and a CrewAI crew runs tasks and a process, which is orchestration the family never does. It implies people (ruling 15), and the ship image inverts so that Agents become sailors. Cell: implies isolation or confinement the family does not give (ACC-027, ACC-028). Rust's std::cell::Cell collides in the reference model. It has dark connotations (terrorist and sleeper cells), the repository's own prose uses table cells, and an AWS cell is a replica of a stack, not an owner. Agency: Agent and agency differ by two letters in agent.v1/agency.v1 and 'conduit agent list'/'conduit agency list'. Its main sense in AI writing, an agent's capacity to act (OWASP LLM06 'Excessive Agency'; the repository's own docs/research/vocabulary/PRECEDENTS.md), would become a second meaning that ruling 42 forbids. It suggests an organization of people, and Agency Swarm's agencies define communication flows among agents. Squad: people (ruling 15), and AWS Agent Squad is an orchestrator. House: weak agentic theme; the verb 'to house' blurs with hold and owner; it sounds close to 'host', which the promises do not use for a machine; it suggests a machine or a home directory. Company: implies a legal entity or tenant above everything and people with roles; ChatDev and MetaGPT companies run role workflows; heavy for a light grouping. Guild: people; guild membership is voluntary and members govern the guild, the inverse of this family; Discord's API calls a server a guild. Bureau: an office of people, a bureaucratic tone, awkward spelling for a CLI noun and key, and odd for one or two Agents. Stable: a homograph of an adjective already in force (the ID row's 'stable identity', ruling 55's stable names), so ruling 42 rules it out. Swarm: Docker Swarm is infrastructure cited about 28 times in docs/ and a plausible runtime provider. OpenAI Swarm means decentralized handoffs between agents, the opposite of an owned, administered group, and the word suggests loss of control. Team: ruling 15 already uses 'Teams' for several people, spec/ uses 'team' (28 occurrences, among them example Pool names), framework teams are coordination runtimes, and Claude Code agent teams run inside a Conduit Agent. Group: Pool's own definition 'groups' Resources, PRV-025 uses the OS group, and in identity systems a group receives access, the grant object rulings 27 to 29 exclude. It is not agentic. Society: tied to CAMEL, implies role play and social interaction (coordination) and people, and reads oddly in the CLI. Workforce: a CAMEL class name and vendor marketing; implies a coordinator assigning tasks and people; 9 letters. Agentspace: Google's product brand, a coined compound (ruling 40), long, and suggests a container of every kind of object. Workspace: not agentic. The spec's valid example project and S1 already name a Pool 'workspace' (12 occurrences in spec/), so 'a Conduit to the workspace' would already mean a Pool. It suggests a container of every kind of object, the model the owner rejected (owner-direction/2026-11-10-projects.md), and people as members. Realm is kept as the infrastructure word instead. Harbor: the Laude Institute's Harbor runs Claude Code and Codex agents, S1's own Agents, with its own 'harbor' CLI, and CNCF Harbor is a registry. Ships come and go, against ownership for life, and the spelling splits between harbor and harbour. Station: Cloudship AI's Station is a platform for managing MCP agents, Conduit's exact niche, and 'stationed at' reads as placement (PLC-001). Basin: Pool and Basin are both containers of water, so the image blurs the very distinction the family exists for; it is passive, not agentic. Marina: 'an Agent's owning Marina' reads as a person's name; leisure connotation; boats lease slips rather than belong for life; no software paradigm. Dock: 'to dock' means to attach or connect, which overlaps connection (ruling 40) and the dropped word 'attach'. Docker and the macOS Dock dominate the word, and docking is temporary. Wellspring: long and poetic with no paradigm; claims the family originates requests, which it does not; its short forms 'source' (a term in force) and 'spring' are taken. Hub: hub-and-spoke says traffic passes through the hub, while requests go through Conduits and owning grants no use, so it misdescribes connection. Docker Hub and Hugging Face Hub make it read as a registry. Squadron: overlaps Fleet and Flotilla and adds problems: personnel (ruling 15), a hierarchy of flights, squadrons and wings while the family is flat, sorties flown together implying coordination, and 8 letters. Wing: an ambiguous everyday word. 'Under its wing' implies protection the family does not give (ACC-027), the military wing brings a hierarchy and people, and 'an Agent's owning wing' is awkward. Its agentic tie is weak. Roost: playful against vocabulary principle 2; idioms ('rule the roost', 'come home to roost') work against it; implies idle Agents; the weakest agentic tie. 'NOT A CANDIDATE' entry: this is the criteria and the taken-word list, not a name. It was used to check the shortlist: none of the eight is a term in spec/definitions.md, and only Unit, Project and Homeport (through 'port' and 'home') touch a word in force or one the vocabulary record dropped, each noted in its entry.

## The critic

I agree with the recommendation: Fleet is the right name, and I found no better word. The ranking holds, but Fleet's collisions are understated (I'd score them 5, not 6) and the comparison with Flotilla was scored unevenly. Both need fixing before this goes to the owner.

Repo facts checked (read-only):
- "fleet" appears 0 times in spec/, conformance/ and reference-model/crates.
- POOL is the only promise prefix longer than three letters.
- The systemd "unit" lines at providers.yaml:472 and implementation-defined.yaml:288 are real.
- "team" has about 30 uses in spec/, not 28. That doesn't matter.

Corrections:

1. **Fleet now names a group of machines, and it replaced "team".**
   - fleetdm 4.82.0 (March 2026) renamed "teams" to "fleets". Each host enrolls into exactly one fleet through that fleet's enroll secret, and admins are scoped per fleet.
   - That is the closest structural precedent for this family, but it applies to machines. It is also the successor word to "team", the concept ruling 15 deferred.
   - The recommendation says docs/proposals/enrollment/README.md:78 uses "fleet" only as a product name. It doesn't. The cells use it as an ordinary noun for a host group: "Enroll secret per fleet" and "Fleet chosen by the secret". Relabelling only the row header won't fix this; the cells need rewording too.

2. **"Fleet", "Agent", "enroll" and "policy" already travel together in operations tools.**
   - Elastic Fleet manages Elastic Agents. Each agent is enrolled in one policy, and a policy change applies to every agent in it.
   - Datadog Fleet Automation upgrades and configures Datadog Agents remotely.
   - Conduit has Agents, enrollment (ruling 56), managed machines (ruling 51) and nodes. An operations reader will take "a Fleet of Agents" to mean daemons on hosts with configuration pushed to all of them. That is exactly what the definition row must deny.

3. **Agent products use "fleet" to mean use and running.**
   - LangSmith Fleet (renamed from Agent Builder, March 2026) is a workspace for "creating, using, and managing your fleet of agents", with sharing and permissions. There, "the fleet" means all of your agents, and it covers use.
   - Copilot CLI's /fleet (April 2026) runs subagents in parallel under a main agent.
   - Both work against "owning grants no use" and "a Fleet runs nothing".

4. **The Fleet and Flotilla scores were judged unevenly.**
   - Flotilla lost points for "under one command" and for size. But a fleet's dictionary sense is also "ships under a single command", and a fleet of one Agent (the owner's packages file has one) is as odd as a flotilla of one.
   - The fit gap is narrower than 7.5 against 4. Fleet still wins on brevity, familiarity and the "fleet management" idea, so the order stands.

Where the name appears:
- **Promises:** fine ("an Agent's owning Fleet", "a Conduit to the Fleet", retire-with-agents, ruling 30's line).
- **Short form:** `fleet: s1` sits above `pools: workspace:`. In GKE, a fleet contains clusters, which contain node pools, so readers may assume a Fleet contains the Pool, or even the machine. The "contains no Pool / not a group of machines" disclaimer has to carry real weight.
- **CLI:** `conduit fleet list` is fine on its own. Users will guess wrong when it sits next to the coming node and enrollment commands.
- **Tool names:** unaffected, because the prefix is the target's `as` name.
- **Promise prefix:** FLEET- would be the only five-letter prefix. FLT- fits the three-letter house style; FLEET- mirroring POOL- is also defensible. Put it to the owner.

What to add:
- **Definition row:** a Fleet's Agents are never configured or upgraded together (the Elastic and Datadog reading). This goes beside "not where Agents run, not a set of machines".
- **A decision for the owner now, while it is cheap:** groups of machines in v1 (rulings 51 and 56) will never be called fleets. They stay a Pool plus labels.
- **S1 rerun questions for the fresh author.** If any answer is yes, the disclaimers have failed:
  - Can root run agent-a's turns through fleet s1?
  - Does fleet s1 include the workspace Pool or the machine?
  - Does a change to fleet s1 apply to all of its Agents?

Sources:
- https://blog.langchain.com/introducing-langsmith-fleet
- https://docs.langchain.com/langsmith/fleet
- https://github.blog/2026/04/01/run-multiple-agents-at-once-with-fleet-in-copilot-cli/
- https://docs.github.com/en/copilot/concepts/agents/copilot-cli/fleet
- https://www.elastic.co/docs/reference/fleet/agent-policy
- https://www.elastic.co/docs/reference/fleet/manage-elastic-agents-in-fleet
- https://docs.datadoghq.com/agent/fleet_automation/
- https://fleetdm.com/guides/fleets
- https://fleetdm.com/docs/using-fleet/enroll-hosts

No better single word turned up. Words checked beyond the brief's list, and why each loses:
- **Hive:** worse than Colony. "Hive mind" means shared memory, and Apache Hive is a well-known product.
- **Pack:** clashes with package (`cargo package`, `npm pack`).
- **Herd:** "thundering herd" is a CS term, and "cattle, not pets" means replaceable units, the opposite of an Agent's fixed owner.
- **Flock:** flock(2) is a file-lock call.
- **Pod:** a Kubernetes pod is placement on one node.
- **Nest:** "nested instantiation" is a term in force.
- **Ensemble:** ML ensembles combine outputs, which goes against the agents' independence.
- **Cast:** a type cast.
- **Troupe:** implies people (ruling 15).
- **Roster:** implies people, and it is a list, not an owner.
- **Namespace:** it holds every object and scopes names, which is the model the owner rejected. Linux namespaces also mean isolation.
- **Tenant, Account:** they imply a customer or credentials, against PRJ-009.
- **Stable:** already excluded, rightly. It is an adjective in force, though it is the best picture of owner versus rider.

Fallback stays as in the brief: Flotilla, but only if the owner reserves "fleet" for v1's managed machines. Because fleetdm has now renamed "teams" to "fleets", that reservation is a real choice. Put it to the owner explicitly rather than leaving it as a footnote.
