//! Installation values are managed through root's existing engine operations.
use crate::{
    Config, Result, author, server,
    sign_in::{Consent, Person, consent},
    user_config,
};
use conduit_loader::Names;
use serde_json::{Value, json};
use std::{
    io::Write,
    path::{Path, PathBuf},
};

const LIST: &str = "management.list.v1";
const INSPECT: &str = "management.inspect.v1";
const WARNING: &str = "A value is not secret: the engine keeps it as configuration, conduit inspect and conduit value list --show show it, and conduit records exports it. A password or token goes on a secret Resource, named by a secret reference (SEC-001), never in a value.";

enum Request<'a> {
    Set(&'a str, Option<&'a str>),
    List(bool),
    Unset(&'a str),
}

impl Request<'_> {
    fn verb(&self) -> &'static str {
        match self {
            Self::Set(..) => "set",
            Self::List(..) => "list",
            Self::Unset(..) => "unset",
        }
    }
}

fn options(args: &[String]) -> Result<(Option<&Path>, Request<'_>)> {
    let mut store = None;
    let mut node = None;
    let mut show = false;
    let mut words = Vec::new();
    let mut args = args.iter().map(String::as_str);
    while let Some(arg) = args.next() {
        match arg {
            "--store" if store.is_none() => {
                store = Some(Path::new(
                    args.next()
                        .filter(|arg| !arg.starts_with('-'))
                        .ok_or("--store needs DIR")?,
                ));
            }
            "--node" if node.is_none() => {
                node = Some(
                    args.next()
                        .filter(|arg| !arg.starts_with('-'))
                        .ok_or("--node needs NODE")?,
                );
            }
            "--show" if !show => show = true,
            arg if !arg.starts_with("--") => words.push(arg),
            _ => return Err(format!("unexpected argument {arg:?}")),
        }
    }
    let request = match words.as_slice() {
        ["set", name] if !show => Request::Set(name, node),
        ["list"] if node.is_none() => Request::List(show),
        ["unset", name] if node.is_none() && !show => Request::Unset(name),
        _ => return Err("expected set NAME [--node NODE], list [--show], or unset NAME".into()),
    };
    Ok((store, request))
}

fn valid_name(name: &str) -> bool {
    let [first, rest @ ..] = name.as_bytes() else {
        return false;
    };
    name.len() <= 63
        && first.is_ascii_lowercase()
        && rest
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}

pub async fn run(
    args: &[String],
    person: &mut dyn Person,
    terminal: bool,
    output: &mut dyn Write,
) -> i32 {
    let (store, request) = match options(args) {
        Ok(options) => options,
        Err(error) => {
            person.tell(&format!(
                "conduit value: {error}; run conduit value --help for usage"
            ));
            return 2;
        }
    };
    if let Request::Set(name, _) = request {
        if !valid_name(name) {
            person.tell(&format!(
                "conduit value set: {name:?} is not a name.v1; nothing was read or sent"
            ));
            return 2;
        }
        if !terminal {
            person.tell("conduit value set: reads the value from your terminal, and standard input is not a terminal; a script registers a value Resource as root with management.register.v1 (MCP or conduit call); nothing was read or sent");
            return 2;
        }
    }
    let verb = request.verb();
    let store = match store {
        Some(store) => store.to_owned(),
        None => match user_config::current().await {
            Ok((file, context)) => {
                person.tell(&format!(
                    "conduit value: using {file:?} for store {:?} at {}",
                    context.store,
                    user_config::escape_line_breaks(&context.url)
                ));
                context.store
            }
            Err(error) => {
                person.tell(&format!("conduit value {verb}: {error}; start conduit serve --store DIR or give --store DIR"));
                return 2;
            }
        },
    };
    if !store.join("state.json").is_file() {
        person.tell(&format!(
            "conduit value {verb}: store is not initialized; run conduit init; nothing was sent"
        ));
        return 2;
    }
    let result = async {
        let url = Config::for_store(&store)?.engine_url;
        let names = conduit_loader::read_file(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../spec/names.yaml"),
        )
        .map_err(|error| error.to_string())?;
        let names = Names::from_node(
            &names
                .first()
                .ok_or("missing spec/names.yaml document")?
                .root,
        )
        .map_err(|error| error.to_string())?;
        let engine = Engine {
            store: &store,
            url,
            names,
        };
        let inventory = engine.inventory().await?;
        match request {
            Request::Set(name, node) => engine.set(&inventory, name, node, person, output).await,
            Request::List(show) => engine.list(&inventory, show, person, output).await,
            Request::Unset(name) => engine.unset(&inventory, name, person, output).await,
        }
    }
    .await;
    match result {
        Ok(code) => code,
        Err(error) => {
            person.tell(&format!("conduit value {verb}: {error}"));
            1
        }
    }
}

struct Engine<'a> {
    store: &'a Path,
    url: String,
    names: Names,
}

struct Inventory {
    objects: Vec<Value>,
    tombstones: Vec<Value>,
    routes: Value,
}

fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_default()
}

fn entries(value: &Value) -> &[Value] {
    value.as_array().map_or(&[], Vec::as_slice)
}

fn outputs(reply: &Value) -> &Value {
    &reply["structuredContent"]
}

fn reference(row: &Value) -> Value {
    json!({"kind":row["kind"],"id":row["id"]})
}

fn live(row: &Value) -> bool {
    matches!(row["state"].as_str(), Some("registered" | "ready")) && row["template"] == false
}

fn write_line(writer: &mut dyn Write, line: impl std::fmt::Display) -> Result<()> {
    writeln!(writer, "{line}").map_err(|error| error.to_string())
}

impl Inventory {
    fn resource(&self, name: &str) -> Option<&Value> {
        self.objects
            .iter()
            .find(|row| row["kind"] == "Resource" && row["name"] == name)
    }

    fn node_name(&self, config: &Value) -> Value {
        let node = &config["node"]["id"];
        self.objects
            .iter()
            .find(|row| row["id"] == *node)
            .map_or(Value::Null, |row| row["name"].clone())
    }
}

struct Observation<'a> {
    object: &'a Value,
    output: Value,
}

impl Observation<'_> {
    fn config(&self) -> &Value {
        &self.output["declared"]["config"]
    }
}

#[derive(Default)]
struct Scan<'a> {
    observed: Vec<Observation<'a>>,
    failed: Vec<(&'a Value, Value)>,
}

impl Scan<'_> {
    fn fills(&self, id: &Value) -> Vec<Value> {
        let mut uses: Vec<_> = self.observed.iter().flat_map(|seen| {
            seen.output["values"].as_object().into_iter().flatten()
                .filter(move |(_, source)| source["id"] == *id)
                .map(|(place, _)| json!({"kind":seen.object["kind"],"name":seen.object["name"],"place":place}))
        }).collect();
        uses.sort_by(|left, right| {
            (
                text(&left["kind"]),
                text(&left["name"]),
                text(&left["place"]),
            )
                .cmp(&(
                    text(&right["kind"]),
                    text(&right["name"]),
                    text(&right["place"]),
                ))
        });
        uses
    }
}

fn fill_names(uses: &[Value]) -> String {
    uses.iter()
        .map(|entry| {
            format!(
                "{}/{} {}",
                text(&entry["kind"]),
                text(&entry["name"]),
                text(&entry["place"])
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn where_node(node: &Value) -> String {
    match node.as_str() {
        Some(name) => format!("tied to node {name}"),
        None => "not tied to a node".into(),
    }
}

fn refused(person: &mut dyn Person, message: impl AsRef<str>) -> Result<i32> {
    person.tell(message.as_ref());
    Ok(2)
}

impl Engine<'_> {
    async fn call(&self, tool: &str, input: Value) -> Result<Value> {
        server::client(self.store, None, Some(tool), input).await
    }

    async fn inventory(&self) -> Result<Inventory> {
        let reply = self.call(LIST, json!({})).await?;
        author::ensure_success(&reply, LIST)?;
        let view = outputs(&reply);
        Ok(Inventory {
            objects: view["objects"]
                .as_array()
                .ok_or("management.list.v1 returned no objects")?
                .clone(),
            tombstones: view["tombstones"]
                .as_array()
                .ok_or("management.list.v1 returned no tombstones")?
                .clone(),
            routes: author::state(self.store)?,
        })
    }

    async fn inspect(&self, inventory: &Inventory, row: &Value) -> Result<Value> {
        let tool = author::setup_inspection_tool(entries(&inventory.routes["objects"]), row);
        self.call(&tool, json!({"object":reference(row)})).await
    }

    async fn scan<'a>(&self, inventory: &'a Inventory) -> Result<Scan<'a>> {
        let mut result = Scan::default();
        for row in inventory.objects.iter().filter(|row| live(row)) {
            let reply = self.inspect(inventory, row).await?;
            if author::succeeded(&reply) {
                result.observed.push(Observation {
                    object: row,
                    output: outputs(&reply).clone(),
                });
            } else {
                result.failed.push((row, reply));
            }
        }
        Ok(result)
    }

    fn is_value(&self, output: &Value) -> bool {
        output["declared"]["type"] == format!("{}#value.v1", self.names.core_package)
    }

    async fn set(
        &self,
        inventory: &Inventory,
        name: &str,
        node: Option<&str>,
        person: &mut dyn Person,
        output: &mut dyn Write,
    ) -> Result<i32> {
        let mut node_ref = Value::Null;
        let mut named_node = Value::Null;
        if let Some(node) = node {
            let mut nodes = Vec::new();
            for row in inventory
                .objects
                .iter()
                .filter(|row| live(row) && row["kind"] == "Resource")
            {
                let reply = self.inspect(inventory, row).await?;
                author::ensure_success(
                    &reply,
                    &format!("inspect.v1 of Resource/{}", text(&row["name"])),
                )?;
                if outputs(&reply)["declared"]["type"]
                    == format!("{}#node.v1", self.names.core_package)
                {
                    nodes.push(text(&row["name"]));
                    if row["name"] == node {
                        node_ref = reference(row);
                        named_node = json!(node);
                    }
                }
            }
            if node_ref.is_null() {
                nodes.sort_unstable();
                return refused(
                    person,
                    format!(
                        "conduit value set: root sees no node named {node:?}; its nodes are [{}]; nothing was read or sent",
                        nodes.join(",")
                    ),
                );
            }
        }
        let held = inventory.resource(name);
        let held_view = if let Some(row) = held {
            let reply = self.inspect(inventory, row).await?;
            author::ensure_success(&reply, &format!("inspect.v1 of Resource/{name}"))?;
            let view = outputs(&reply);
            let ty = if row["template"] == true {
                text(&view["declared"]["spec"]["type"])
            } else {
                text(&view["declared"]["type"])
            };
            if ty != format!("{}#value.v1", self.names.core_package) {
                return refused(
                    person,
                    format!(
                        "conduit value set: Resource/{name} exists with Type {ty}, not value.v1; choose another name; nothing was read or sent"
                    ),
                );
            }
            if row["template"] == true {
                return refused(
                    person,
                    format!(
                        "conduit value set: Resource/{name} is a template, not a value instance; choose another name; nothing was read or sent"
                    ),
                );
            }
            Some(view.clone())
        } else {
            if let Some(stone) = inventory.tombstones.iter().find(|stone| {
                stone["object"]["kind"] == "Resource" && stone["object"]["name"] == name
            }) {
                let id = text(&stone["id"]);
                let old = text(&stone["object"]["id"]);
                let pool = text(&stone["owner"]["name"]);
                let time = text(&stone["time"]);
                let cause = &stone["cause"];
                let store = shell_quote(&self.store.display().to_string());
                let input = shell_quote(&json!({"id":id}).to_string());
                return refused(
                    person,
                    format!(
                        "conduit value set: Tombstone id={id} of Resource/{name} id={old}, owned by Pool/{pool}, retired at {time} (cause {cause}), holds the name; to reuse the name, lift it as root first: conduit call --store {store} access.lift.v1 {input}, then run conduit value set {name} again; nothing was read or sent"
                    ),
                );
            }
            None
        };
        let held_id = held.map(|row| text(&row["id"]));
        let mut held_node = Value::Null;
        let mut held_value = Value::Null;
        let mut fills = Vec::new();
        if let Some(view) = &held_view {
            let config = &view["declared"]["config"];
            held_node = inventory.node_name(config);
            held_value = config["value"].clone();
            let scan = self.scan(inventory).await?;
            if let Some((object, _)) = scan.failed.first() {
                return Err(format!(
                    "inspect.v1 of {}/{} did not succeed, so fills may be incomplete",
                    text(&object["kind"]),
                    text(&object["name"])
                ));
            }
            fills = scan.fills(&view["id"]);
        }
        person.tell(WARNING);
        if let Some(id) = held_id {
            person.tell(&format!("{name} holds {held_value} ({}) as Resource/{name} id={id} and fills [{}]; each keeps {held_value} until it is replaced (OBJ-010)",where_node(&held_node),fill_names(&fills)));
        }
        let answer = match person.ask(&format!("Type the value of {name}, then Enter: ")) {
            Ok(answer) => answer,
            Err(error) => {
                return refused(
                    person,
                    format!("conduit value set: {error}; nothing was sent"),
                );
            }
        };
        let Some(value) = answer else {
            return refused(
                person,
                "conduit value set: no value was read: standard input ended; nothing was sent",
            );
        };
        if value.is_empty() {
            return refused(
                person,
                "conduit value set: the value is empty; nothing was sent",
            );
        }
        if let Some(index) = value.chars().position(char::is_control) {
            return refused(
                person,
                format!(
                    "conduit value set: the value holds a control character at character {}; a value is one line of text; nothing was sent",
                    index + 1
                ),
            );
        }
        let value = json!(value);
        let same = held_view.as_ref().is_some_and(|view| {
            value == held_value && view["declared"]["config"]["node"] == node_ref
        });
        let replacing = match held_id {
            Some(_) if !same => format!(", replacing {held_value} ({})", where_node(&held_node)),
            _ => String::new(),
        };
        person.tell(&format!(
            "{name} = {value} ({}){replacing}",
            where_node(&named_node)
        ));
        let question = match (held_id, same) {
            (Some(id), true) if held.is_some_and(|row| row["state"] == "ready") => {
                write_line(
                    output,
                    format!(
                        "unchanged Resource/{name} id={id}: the engine holds this value and node; nothing was sent"
                    ),
                )?;
                return Ok(0);
            }
            (Some(id), true) => format!(
                "Resource/{name} id={id} holds this value and node but is not set up; set it up as root in the engine at {}? [y/N] ",
                self.url
            ),
            (Some(id), false) => format!(
                "Save it as root in the engine at {}? This retires Resource/{name} id={id}, lifts its tombstone and registers the new value. [y/N] ",
                self.url
            ),
            _ => format!("Save it as root in the engine at {}? [y/N] ", self.url),
        };
        let answer = match person.ask(&question) {
            Ok(answer) => answer,
            Err(error) => {
                return refused(
                    person,
                    format!("conduit value set: {error}; nothing was sent"),
                );
            }
        };
        if !matches!(consent(answer.as_deref()), Consent::Open) {
            return refused(
                person,
                "conduit value set: not saved: the answer was not y; nothing was sent",
            );
        }
        let mut arguments = json!({"value":value});
        if !node_ref.is_null() {
            arguments["node"] = node_ref;
        }
        self.save(
            Save {
                name,
                node: named_node,
                arguments,
                old: held_id,
                setup_only: same,
            },
            person,
            output,
        )
        .await
    }

    async fn list(
        &self,
        inventory: &Inventory,
        show: bool,
        person: &mut dyn Person,
        output: &mut dyn Write,
    ) -> Result<i32> {
        let scan = self.scan(inventory).await?;
        let mut value_rows = Vec::new();
        for observation in scan
            .observed
            .iter()
            .filter(|seen| self.is_value(&seen.output))
        {
            let row = observation.object;
            let config = observation.config();
            let mut data = json!({"fills":scan.fills(&row["id"]),"id":row["id"],"live":true,"name":row["name"],"node":inventory.node_name(config)});
            if show {
                data["value"] = config["value"].clone();
            }
            value_rows.push(data);
        }
        let mut old_ids = std::collections::BTreeSet::new();
        for seen in &scan.observed {
            for source in seen.output["values"]
                .as_object()
                .into_iter()
                .flat_map(|map| map.values())
            {
                let id = text(&source["id"]);
                if !id.is_empty() && !value_rows.iter().any(|row| row["id"] == id) {
                    old_ids.insert(id);
                }
            }
        }
        for id in old_ids {
            let reply = self
                .call(INSPECT, json!({"object":{"kind":"Resource","id":id}}))
                .await?;
            let old = author::succeeded(&reply).then_some(outputs(&reply));
            let mut row = json!({"fills":scan.fills(&json!(id)),"id":id,"live":false,"name":null,"node":null});
            if let Some(old) = old {
                row["name"] = old["declared"]["name"].clone();
                row["node"] = inventory.node_name(&old["declared"]["config"]);
            }
            if show {
                row["value"] = old.map_or(Value::Null, |old| {
                    old["declared"]["config"]["value"].clone()
                });
            }
            value_rows.push(row);
        }
        value_rows.sort_by(|left, right| {
            (left["name"].as_str(), text(&left["id"]))
                .cmp(&(right["name"].as_str(), text(&right["id"])))
        });
        for row in &value_rows {
            write_line(output, row)?;
        }
        for (object, reply) in &scan.failed {
            write_line(output, reply)?;
            person.tell(&format!("conduit value list: inspect.v1 of {}/{} did not succeed, so fills may be incomplete",text(&object["kind"]),text(&object["name"])));
        }
        if value_rows.is_empty() && scan.failed.is_empty() {
            person.tell(
                "conduit value list: root sees no value Resource and no instance filled by one",
            );
        }
        Ok(i32::from(!scan.failed.is_empty()))
    }

    async fn unset(
        &self,
        inventory: &Inventory,
        name: &str,
        person: &mut dyn Person,
        output: &mut dyn Write,
    ) -> Result<i32> {
        let missing = || {
            format!(
                "conduit value unset: root sees no value Resource named {name}; conduit value list names those it sees; nothing was sent"
            )
        };
        let Some(row) = inventory.resource(name).filter(|row| live(row)) else {
            return refused(person, missing());
        };
        let reply = self.inspect(inventory, row).await?;
        author::ensure_success(&reply, &format!("inspect.v1 of Resource/{name}"))?;
        if !self.is_value(outputs(&reply)) {
            return refused(person, missing());
        }
        let scan = self.scan(inventory).await?;
        if let Some((object, _)) = scan.failed.first() {
            return Err(format!(
                "inspect.v1 of {}/{} did not succeed, so fills may be incomplete",
                text(&object["kind"]),
                text(&object["name"])
            ));
        }
        let fills = scan.fills(&row["id"]);
        if !fills.is_empty() {
            return refused(
                person,
                format!(
                    "conduit value unset: Resource/{name} fills [{}]; each keeps its value until it is replaced or retired, so a value an instance names is not retired; retire or replace each, then unset again; nothing was sent",
                    fill_names(&fills)
                ),
            );
        }
        let tool = "system.retire.v1";
        let target = format!("Resource/{name}");
        if self
            .attempt(output, 1, tool, &target, json!({"instance":reference(row)}))
            .await?
            .is_none()
        {
            return Err(format!("1 {tool} did not succeed; nothing more was sent"));
        }
        let id = text(&row["id"]);
        write_line(
            output,
            format!(
                "unset Resource/{name} id={id}: retired; its tombstone holds the name until root lifts it with access.lift.v1; conduit value set {name} then registers it again"
            ),
        )?;
        Ok(0)
    }
}

fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

struct Save<'a> {
    name: &'a str,
    node: Value,
    arguments: Value,
    old: Option<&'a str>,
    setup_only: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Retire,
    FindTombstone,
    Lift,
    Register,
    Setup,
}

impl Step {
    fn tool(self) -> &'static str {
        match self {
            Self::Retire => "system.retire.v1",
            Self::FindTombstone => LIST,
            Self::Lift => "access.lift.v1",
            Self::Register => "management.register.v1",
            Self::Setup => "system.set-up.v1",
        }
    }
}

impl Engine<'_> {
    async fn attempt(
        &self,
        output: &mut dyn Write,
        number: usize,
        tool: &str,
        target: &str,
        input: Value,
    ) -> Result<Option<Value>> {
        let reply = match self.call(tool, input).await {
            Ok(reply) => reply,
            Err(_) => {
                write_line(output, format!("{number} {tool} {target} outcome=unknown"))?;
                return Ok(None);
            }
        };
        let success = author::succeeded(&reply);
        let outcome = reply["_meta"]
            .as_object()
            .and_then(|meta| {
                meta.iter()
                    .find(|(key, _)| key.ends_with("/outcome") || key.ends_with("/reason"))
                    .and_then(|(_, value)| value.as_str())
            })
            .unwrap_or(if success { "succeeded" } else { "failed" });
        write_line(
            output,
            format!("{number} {tool} {target} outcome={outcome}"),
        )?;
        if success {
            Ok(Some(reply))
        } else {
            write_line(output, &reply)?;
            Ok(None)
        }
    }

    async fn save(
        &self,
        save: Save<'_>,
        person: &mut dyn Person,
        output: &mut dyn Write,
    ) -> Result<i32> {
        use Step::{FindTombstone, Lift, Register, Retire, Setup};
        let steps: &[Step] = if save.setup_only {
            &[Setup]
        } else if save.old.is_some() {
            &[Retire, FindTombstone, Lift, Register, Setup]
        } else {
            &[Register, Setup]
        };
        let name = save.name;
        let mut id = save.old.map(str::to_owned);
        let mut stone = None;
        let mut retired = false;
        let mut lifted = false;
        for (index, step) in steps.iter().copied().enumerate() {
            let n = index + 1;
            let tool = step.tool();
            let target = match step {
                FindTombstone | Lift => format!("Tombstone/{name}"),
                _ => format!("Resource/{name}"),
            };
            let args = match step {
                Retire | Setup => json!({"instance":{"kind":"Resource","id":id}}),
                FindTombstone => json!({}),
                Lift => json!({"id":stone}),
                Register => json!({"documents":[{
                    "apiVersion":self.names.api_version,"kind":"Resource","metadata":{"name":name},
                    "from":format!("{}#value",self.names.core_package),"owner":"system","arguments":save.arguments
                }]}),
            };
            let reply = self.attempt(output, n, tool, &target, args).await?;
            let mut failure = "did not succeed".to_owned();
            let succeeded = if let Some(reply) = reply {
                match step {
                    Retire => retired = true,
                    FindTombstone => {
                        stone = entries(&outputs(&reply)["tombstones"])
                            .iter()
                            .find(|tombstone| {
                                tombstone["object"]["kind"] == "Resource"
                                    && tombstone["object"]["id"].as_str() == save.old
                            })
                            .and_then(|tombstone| tombstone["id"].as_str())
                            .map(str::to_owned);
                        if stone.is_none() {
                            failure = format!(
                                "listed no tombstone of Resource/{name} id={}",
                                save.old.unwrap_or("?")
                            );
                        }
                    }
                    Lift => lifted = true,
                    Register => {
                        id = entries(&outputs(&reply)["objects"])
                            .iter()
                            .find(|row| row["kind"] == "Resource" && row["name"] == name)
                            .and_then(|row| row["id"].as_str())
                            .map(str::to_owned);
                        if id.is_none() {
                            failure = format!("returned no ID for Resource/{name}");
                        }
                    }
                    Setup => {}
                }
                (step != FindTombstone || stone.is_some()) && (step != Register || id.is_some())
            } else {
                false
            };
            if !succeeded {
                if n < steps.len() {
                    write_line(output, format!("not sent {}..{}", n + 1, steps.len()))?;
                }
                let ending = if step == Setup {
                    format!(
                        "; Resource/{name} id={} is registered but not set up; conduit value set {name} again sets it up",
                        id.as_deref().unwrap_or("?")
                    )
                } else if retired {
                    let recovery = if lifted {
                        format!("conduit value set {name} again registers it")
                    } else {
                        format!(
                            "check management.list.v1 for its tombstone and lift it as root with access.lift.v1 before running conduit value set {name} again"
                        )
                    };
                    format!(
                        "; Resource/{name} id={} is retired and its new value is not registered; instances keep what they hold; {recovery}",
                        save.old.unwrap_or("?")
                    )
                } else {
                    String::new()
                };
                person.tell(&format!(
                    "conduit value set: {n} {tool} {failure}; nothing more was sent{ending}"
                ));
                return Ok(1);
            }
        }
        let id = id.ok_or("register.v1 returned no ID for the value")?;
        write_line(
            output,
            format!(
                "saved Resource/{name} id={id} node={}",
                save.node.as_str().unwrap_or("-")
            ),
        )?;
        Ok(0)
    }
}
