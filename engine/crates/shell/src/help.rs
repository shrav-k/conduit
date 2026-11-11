const COMMANDS: [(&str, &str, &str); 12] = [
    (
        "init",
        "[--label KEY=VALUE]",
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
        "ARCHIVE#SHA256",
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
    ("inspect", "", "inspect --store /tmp/conduit"),
    ("records", "", "records --store /tmp/conduit"),
    ("serve", "", "serve --store /tmp/conduit"),
    (
        "tools",
        "[--as NAME|KIND/NAME]",
        "tools --store /tmp/conduit --as Agent/a",
    ),
    (
        "call",
        "TOOL [ARGUMENTS-JSON]",
        "call --store /tmp/conduit management.inspect.v1 '{\"object\":{\"kind\":\"Interface\",\"name\":\"root\"}}'",
    ),
];
const DESCRIPTIONS: [&str; 12] = [
    "Initialize a private store and root identity; repeat is safe.",
    "Register a local project, open root's submitted Conduits and set up instances. Owns runs until exit; stop serve first. --package explicitly installs an archive before registration.",
    "Submit a project to serve, open root's submitted Conduits and set up instances. Prints each attempted step after its results; completed steps persist on failure. Relative fetch/program values must be made absolute.",
    "Replace the named template with the matching template in PROJECT_DIR, using its current revision.",
    "Install an archive through serve. SHA256 is the archive's digest (optional sha256: prefix). Install dependencies first.",
    "Retire an instance through serve, routing Resources through their owning Pool.",
    "Unregister an object through serve using its current revision; retire live instances first.",
    "Inspect every object as root, one JSON line each. Uses serve if the store is active. Refused inspections also cause a nonzero exit.",
    "Export ordered CloudEvents as JSON lines; stop serve first. Gaps cause a nonzero exit.",
    "Hold the store, set up registered instances and serve MCP on its fixed loopback URL. SIGTERM or Ctrl-C ends runs.",
    "List tools presented to root or a named instance. Set up the instance first and open its Conduits if its list is empty.",
    "Call a presented root tool once through serve. ARGUMENTS-JSON must be an object (default {}). Failed or unknown outcomes cause a nonzero exit.",
];
pub fn print(command: Option<&str>) -> Result<(), String> {
    if let Some(name) = command {
        if !COMMANDS.iter().any(|(c, _, _)| *c == name) {
            return Err(format!(
                "help for unknown command {name:?}; run conduit --help for commands"
            ));
        }
    } else {
        println!("Conduit author commands. Usage: conduit COMMAND --store DIR [ARGUMENTS]\n");
    }
    for (i, (name, arguments, example)) in COMMANDS.iter().enumerate() {
        if command.is_none_or(|c| c == *name) {
            println!(
                "conduit {name} --store DIR {arguments}\n  {}\n  Example: conduit {example}\n",
                DESCRIPTIONS[i]
            );
        }
    }
    println!(
        "--store DIR: initialized private store (required except for help).\n--help: show usage without opening a store.\nRunner/store options for init, register and serve:\n  --url URL: fixed loopback /mcp address (default http://127.0.0.1:3000/mcp).\n  --output-cap-bytes N: retained output cap (default 67108864).\n  --output-age-seconds N: retained output age (default 604800).\n  --initialize-timeout-seconds N: provider initialization wait (default 30).\n  --call-timeout-seconds N: provider call wait (default 120).\n  --pass-environment NAME: allow an environment name into provider runs; repeatable.\nServing commands use the URL fixed in the store. Any printed failed result causes exit 1."
    );
    Ok(())
}
