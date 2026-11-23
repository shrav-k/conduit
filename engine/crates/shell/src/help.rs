const COMMANDS: [(&str, &str, &str); 17] = [
    (
        "init",
        "[--label KEY=VALUE] [--trust-domain DOMAIN]",
        "init --store /tmp/conduit --label purpose=demo",
    ),
    (
        "register",
        "PROJECT_DIR [--package ARCHIVE#SHA256]",
        "register --store /tmp/conduit ./project",
    ),
    (
        "submit",
        "PROJECT_DIR",
        "submit --store /tmp/conduit ./project",
    ),
    (
        "replace",
        "PROJECT_DIR KIND/NAME",
        "replace --store /tmp/conduit ./project Resource/directory-template",
    ),
    (
        "install",
        "[ARCHIVE#SHA256] [--mirror DIR] [--dry-run]",
        "install --store /tmp/conduit ./package.tar.gz#SHA256",
    ),
    (
        "retire",
        "KIND/NAME",
        "retire --store /tmp/conduit Resource/directory",
    ),
    (
        "unregister",
        "KIND/NAME",
        "unregister --store /tmp/conduit Resource/directory",
    ),
    ("inspect", "[--setup]", "inspect --store /tmp/conduit"),
    ("pending", "[--agent NAME]", "pending --store /tmp/conduit"),
    (
        "answer",
        "ACTION ANSWER",
        "answer --store /tmp/conduit 00000000-0000-4000-8000-000000000001 dismissed",
    ),
    (
        "sign-in",
        "KIND/NAME [--login NAME]",
        "sign-in --store /tmp/conduit Agent/a",
    ),
    ("records", "", "records --store /tmp/conduit"),
    (
        "serve",
        "[--https-url URL] [--https-listen ADDRESS] [--https-roots FILE]",
        "serve --store /tmp/conduit",
    ),
    (
        "tools",
        "[--as NAME|KIND/NAME]",
        "tools --store /tmp/conduit --as Agent/a",
    ),
    (
        "call",
        "TOOL [ARGUMENTS-JSON | --arguments-file PATH|-]",
        "call --store /tmp/conduit management.inspect.v1 '{\"object\":{\"kind\":\"Interface\",\"name\":\"root\"}}'",
    ),
    (
        "plan",
        "[PROJECT_DIR] [--reopen OBJECT] [--recreate OBJECT] [--replace OBJECT]",
        "plan --replace shared",
    ),
    (
        "apply",
        "[PROJECT_DIR] [--reopen OBJECT] [--recreate OBJECT] [--replace OBJECT]",
        "apply --replace shared",
    ),
];
const DESCRIPTIONS: [&str; 17] = [
    "Create the project's conduit.yaml skeleton unless a manifest already exists. With --store, initialize a private store and root identity instead.",
    "Register a local project, open root's submitted Conduits and set up instances. Owns runs until exit; stop serve first. --package explicitly installs an archive before registration.",
    "Submit a project to serve, open root's submitted Conduits and set up instances. Prints each attempted step after its results; completed steps persist on failure. Relative fetch/program values must be made absolute.",
    "Replace the named template with the matching template in PROJECT_DIR, using its current revision.",
    "Install an archive's missing dependencies first, one request per archive, then install and pin the named archive at its exact version and digest. Without an archive, restore the manifest's pins. Find dependencies in the per-user cache, at an absolute file: fetch, then in --mirror directories and configured mirrors. Keep verified archives by lowercase digest. --dry-run prints the proposed installs and pin without sending installs or writing files.",
    "Retire an instance through serve, routing Resources through their owning Pool.",
    "Unregister an object through serve using its current revision; retire live instances first.",
    "Inspect every object as root, one JSON line each. --setup shows each instance's last setup. Uses serve if the store is active. Refused inspections also cause a nonzero exit.",
    "List the pending actions raised at root through serve, one JSON line each as list.v1 gives them. --agent NAME selects actions naming that Agent. Prints nothing when none is raised.",
    "Answer a setup action raised at root through serve, through root's Conduit to the target that authorizes inspection of its subject; a setup takes dismissed. Sends nothing for an action not raised at root or of another kind.",
    "Sign in an Agent or Resource through serve and root's Conduit to its owner: shows the engine's message and the full URL, opens the browser only after you type y, and retries each time you press Enter until the engine answers. --login asks for one of the Type's stated logins; the engine refuses it where the sign-in hook is plain, as in the 0.2.0 Agent packages. Carries no return from another machine.",
    "Export ordered CloudEvents as JSON lines. Uses serve while it holds the store. Gaps cause a nonzero exit.",
    "Hold the store, set up registered instances and serve MCP on its fixed loopback URL. With an https address, also serve /mcp for that address on a second loopback listener. Records the current context before printing its JSON ready line. SIGTERM or Ctrl-C ends runs.",
    "List tools presented to root or a named instance. Set up the instance first and open its Conduits if its list is empty.",
    "Call a presented root tool once through serve. Arguments must be a JSON object (default {}). --arguments-file - reads standard input to its end. Failed or unknown outcomes cause a nonzero exit.",
    "Read the project and engine as root, and print the requests apply would send. A missing approval operand stops before engine reads. An unused or ambiguous operand stops after discovery.",
    "Compute, print and record the plan, then send its requests as root without a prompt. A missing approval operand stops before engine reads. An unused or ambiguous operand stops after discovery.",
];
pub fn print(command: Option<&str>) -> Result<(), String> {
    if let Some(name) = command {
        if !COMMANDS.iter().any(|(c, _, _)| *c == name) {
            return Err(format!(
                "help for unknown command {name:?}; run conduit --help for commands"
            ));
        }
    } else {
        println!("Conduit author commands. Usage: conduit COMMAND [ARGUMENTS]\n");
        println!(
            "conduit --version\n  Show the crate version and build commit without opening a store. Uncommitted changes are omitted.\n"
        );
    }
    for (i, (name, arguments, example)) in COMMANDS.iter().enumerate() {
        if command.is_none_or(|c| c == *name) {
            let store = match *name {
                "init" => {
                    println!("conduit init");
                    "--store DIR"
                }
                "register" | "serve" => "--store DIR",
                _ => "[--store DIR]",
            };
            println!(
                "conduit {name} {store} {arguments}\n  {}\n  Example: conduit {example}\n",
                DESCRIPTIONS[i]
            );
        }
    }
    println!(
        "--store DIR: select a private store explicitly and bypass the context. Required for serve and register.\n\
         CONDUIT_CONFIG_DIR: absolute directory for context.json and packages/. When unset or empty, use $HOME/.config/conduit.\n\
         Without --store, serving commands check the recorded store and its loopback URL as root, then print the context file, store and URL to stderr. If the context is missing, invalid or stale, they exit 2 and say to start conduit serve --store DIR or give --store DIR.\n\
         --help: show usage without opening a store.\n\
         Runner/store options for init, register and serve (init requires --store for these options):\n  --url URL: fixed loopback /mcp address (default http://127.0.0.1:3000/mcp).\n  --output-cap-bytes N: retained output cap (default 67108864).\n  --output-age-seconds N: retained output age (default 604800).\n  --initialize-timeout-seconds N: provider initialization wait (default 30).\n  --call-timeout-seconds N: provider call wait (default 120).\n  --pass-environment NAME: allow an environment name into provider runs; repeatable.\nOptions for serve alone:\n  --https-url URL: canonical https /mcp address, with no loopback host; fixed in the store at first use.\n  --https-listen ADDRESS: second HTTP listener at 127.0.0.1:PORT or [::1]:PORT; required with an https address, including a fixed one.\n  --https-roots FILE: CERTIFICATE PEM roots for that address; checked content replaces the stored copy. Requires an https address.\nServing commands use the URL fixed in the store. Any printed failed result causes exit 1."
    );
    Ok(())
}
