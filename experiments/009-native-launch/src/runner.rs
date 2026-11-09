use crate::{
    Client, clean_command,
    client::{Config, arguments, settings},
    private_dir, prompt, run_environment, write_json,
};
use anyhow::{Context, Result, bail, ensure};
use real_client_harness::stub::start_agents;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout},
    time::timeout,
};

/// Six individual sessions plus three simultaneous pairs: nine scenarios, twelve Agents.
pub fn matrix() -> Vec<(String, Vec<Client>)> {
    let mut runs = Vec::new();
    for repetition in 1..=3 {
        for client in [Client::Claude, Client::Codex] {
            runs.push((format!("{}-r{repetition}", client.name()), vec![client]));
        }
        runs.push((
            format!("pair-r{repetition}"),
            vec![Client::Claude, Client::Codex],
        ));
    }
    runs
}

pub fn validate_output(output: &Path) -> Result<()> {
    ensure!(output.is_absolute(), "output must be absolute");
    ensure!(!output.exists(), "output must be fresh");
    // Canonicalize the nearest existing parent, so a symlink cannot hide a repository path.
    let mut ancestor = output;
    let mut suffix = Vec::new();
    while !ancestor.exists() {
        suffix.push(ancestor.file_name().context("output ancestor")?.to_owned());
        ancestor = ancestor.parent().context("output parent")?;
    }
    let mut resolved = ancestor.canonicalize()?;
    for part in suffix.into_iter().rev() {
        resolved.push(part);
    }
    ensure!(
        !output
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir)),
        "output cannot contain parent components"
    );
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    ensure!(
        !resolved.starts_with(repo),
        "output must be outside the repository"
    );
    Ok(())
}
fn wrapper_path(client: Client) -> Result<PathBuf> {
    Ok(std::env::current_exe()?
        .parent()
        .context("executable parent")?
        .join(format!("{}-wrapper", client.name())))
}
fn config(client: Client, agent: &Path, url: &str, seconds: u64) -> Result<Config> {
    Ok(Config {
        client,
        program: client.name().into(),
        helper: wrapper_path(client)?,
        evidence: agent.into(),
        cwd: agent.join("work"),
        env: run_environment(crate::inherited_environment(), url, &agent.join("identity")),
        url: url.into(),
        timeout: Duration::from_secs(seconds),
    })
}
pub fn dry_run(output: &Path, seconds: u64) -> Result<()> {
    validate_output(output)?;
    for (id, clients) in matrix() {
        println!("RUN {id}");
        println!(
            "{}",
            json!({"stub":"experiment 008 start_agents, loopback port zero","change":"after every first-turn result: close files, open notes, notify; then second turns","parallel":clients.len()==2})
        );
        for client in clients {
            let agent = output.join(&id).join(client.name());
            let config = config(client, &agent, "http://127.0.0.1:<port>/", seconds)?;
            println!(
                "{}",
                json!({"instance":format!("spiffe://experiment.test/conduit/instance/{id}-{}",client.name()),"wrapper":wrapper_path(client)?,"wrapper_args":["--evidence",agent.display().to_string(),"--timeout-secs",seconds.to_string()],"environment":config.env,"cwd":config.cwd,"settings":settings(),"mcp":{"mcpServers":{"stub":{"type":"http","url":config.url,"headersHelper":crate::client::helper_command(&config.helper)}}},"first_client_args":arguments(&config,None,prompt(1)),"second_client_args":if client==Client::Codex {json!(arguments(&config,Some("<exact-thread-id-from-turn-1>"),prompt(2)))} else {json!("same Claude process: streamed user message after first result")},"turns":[prompt(1),prompt(2)],"authentication":"client reports only; shared owner login requested, no API-key variables"})
            );
        }
    }
    println!("9 scenarios, 12 Agent sessions, 24 turns; dry run only");
    Ok(())
}

/// A scripted MCP client. All exchanged JSON is retained, including notifications.
pub struct Connection {
    child: Child,
    input: Option<ChildStdin>,
    lines: Lines<BufReader<ChildStdout>>,
    log: fs::File,
    next_id: u64,
    deadline: Duration,
    group: Option<u32>,
}
impl Connection {
    pub fn start(
        program: &Path,
        args: &[String],
        env: &BTreeMap<String, String>,
        cwd: &Path,
        evidence: &Path,
        seconds: u64,
    ) -> Result<Self> {
        let mut command = clean_command(program, env, cwd);
        command
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(fs::File::create(evidence.join("wrapper-stderr.txt"))?);
        command.process_group(0);
        let mut child = command.spawn().context("start wrapper")?;
        let group = child.id();
        Ok(Self {
            input: child.stdin.take(),
            lines: BufReader::new(child.stdout.take().context("wrapper stdout")?).lines(),
            child,
            log: fs::File::create(evidence.join("exchange.jsonl"))?,
            next_id: 0,
            deadline: Duration::from_secs(seconds.saturating_add(15)),
            group,
        })
    }
    async fn send(&mut self, value: Value) -> Result<()> {
        writeln!(
            self.log,
            "{}",
            json!({"direction":"to_wrapper","message":value})
        )?;
        self.log.flush()?;
        let input = self.input.as_mut().context("wrapper stdin closed")?;
        input.write_all(format!("{value}\n").as_bytes()).await?;
        input.flush().await?;
        Ok(())
    }
    pub async fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        self.next_id += 1;
        let id = self.next_id;
        self.send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
            .await?;
        timeout(self.deadline, async {
            while let Some(line) = self.lines.next_line().await? {
                let value: Value =
                    serde_json::from_str(&line).context("wrapper emitted invalid JSON")?;
                writeln!(
                    self.log,
                    "{}",
                    json!({"direction":"from_wrapper","message":value})
                )?;
                self.log.flush()?;
                if value["id"] == id {
                    return Ok(value);
                }
            }
            bail!("wrapper EOF before response")
        })
        .await
        .context("wrapper response deadline")?
    }
    pub async fn initialize(&mut self) -> Result<()> {
        let init=self.request("initialize",json!({"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"experiment-009-runner","version":"0.1.0"}})).await?;
        ensure!(
            init.get("result").is_some(),
            "wrapper initialization failed: {init}"
        );
        self.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
            .await?;
        let tools = self.request("tools/list", json!({})).await?;
        ensure!(
            tools["result"]["tools"]
                .as_array()
                .is_some_and(|tools| tools.len() == 1 && tools[0]["name"] == "turn"),
            "wrapper must expose exactly turn"
        );
        Ok(())
    }
    pub async fn turn(&mut self, turn: usize) -> Result<Value> {
        let response = self
            .request(
                "tools/call",
                json!({"name":"turn","arguments":{"message":prompt(turn)}}),
            )
            .await?;
        ensure!(
            response.get("error").is_none() && response["result"]["isError"] == false,
            "wrapper turn failed: {response}"
        );
        Ok(response)
    }
    pub async fn stop(&mut self) {
        self.input.take();
        if timeout(Duration::from_secs(3), self.child.wait())
            .await
            .is_err()
        {
            self.kill_group();
            let _ = self.child.wait().await;
        }
        self.kill_group();
    }
    fn kill_group(&mut self) {
        if let Some(id) = self.group.take() {
            // Each wrapper owns its process group; clean up the native client on timeout too.
            unsafe {
                libc::kill(-(id as i32), libc::SIGKILL);
            }
        }
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.kill_group();
    }
}

pub async fn run(output: &Path, seconds: u64) -> Result<()> {
    validate_output(output)?;
    for client in [Client::Claude, Client::Codex] {
        ensure!(
            wrapper_path(client)?.is_file(),
            "build all binaries first: cargo build --release --bins"
        );
    }
    fs::create_dir_all(output.parent().context("output parent")?)?;
    private_dir(output)?;
    let mut failures = 0;
    for (id, clients) in matrix() {
        println!("RUN {id}");
        let run = output.join(&id);
        private_dir(&run)?;
        let mut agents = Vec::new();
        for client in clients {
            let agent = run.join(client.name());
            private_dir(&agent)?;
            private_dir(&agent.join("work"))?;
            agents.push((
                client,
                agent,
                format!(
                    "spiffe://experiment.test/conduit/instance/{id}-{}",
                    client.name()
                ),
            ));
        }
        let identity_paths: Vec<_> = agents
            .iter()
            .map(|(_, path, _)| path.join("identity"))
            .collect();
        let identities: Vec<_> = agents
            .iter()
            .zip(&identity_paths)
            .map(|((_, _, subject), path)| (subject.as_str(), path.as_path()))
            .collect();
        let stub = start_agents(&run, &identities).await?;
        let result = run_scenario(&agents, &stub.url, &stub, seconds).await;
        write_json(
            &run.join("status.json"),
            &json!({"completed":result.is_ok(),"error":result.as_ref().err().map(|e|format!("{e:#}"))}),
        )?;
        stub.cancel.cancel();
        let mut task = stub.task;
        if timeout(Duration::from_secs(3), &mut task).await.is_err() {
            task.abort();
        }
        if result.is_err() {
            failures += 1;
        }
    }
    ensure!(
        failures == 0,
        "{failures} scenarios failed; inspect retained records"
    );
    println!("9 scenarios completed; observations require classification");
    Ok(())
}
async fn run_scenario(
    agents: &[(Client, PathBuf, String)],
    url: &str,
    stub: &real_client_harness::stub::RunningStub,
    seconds: u64,
) -> Result<()> {
    let mut connections = Vec::new();
    for (client, agent, subject) in agents {
        let config = config(*client, agent, url, seconds)?;
        let program = wrapper_path(*client)?;
        let args = vec![
            "--evidence".into(),
            agent.display().to_string(),
            "--timeout-secs".into(),
            seconds.to_string(),
        ];
        write_json(
            &agent.join("launch.json"),
            &json!({"program":program,"args":args,"environment":config.env,"cwd":config.cwd,"instance":subject,"login":"owner shared; client authentication report required"}),
        )?;
        // A version command is not a model turn and gets exactly the run's environment.
        let version = timeout(
            Duration::from_secs(10),
            clean_command(&config.program, &config.env, &config.cwd)
                .arg("--version")
                .stdin(Stdio::null())
                .output(),
        )
        .await;
        match version {
            Ok(Ok(value)) => {
                fs::write(agent.join("version.txt"), value.stdout)?;
                fs::write(agent.join("version-stderr.txt"), value.stderr)?;
                write_json(
                    &agent.join("version-status.json"),
                    &json!({"exit_code":value.status.code()}),
                )?;
            }
            _ => write_json(
                &agent.join("version-status.json"),
                &json!({"status":"unavailable"}),
            )?,
        }
        let mut connection =
            Connection::start(&program, &args, &config.env, &config.cwd, agent, seconds)?;
        connection.initialize().await?;
        connections.push(connection);
    }
    let result = async {
        for turn in 1..=2 {
            stub.log.begin_turn(turn);
            if turn == 2 {
                stub.change_access().await;
            }
            if connections.len() == 2 {
                let (left, right) = connections.split_at_mut(1);
                // Both explicit calls are polled together, against the same live stub.
                let (first, second) = tokio::join!(left[0].turn(turn), right[0].turn(turn));
                for (index, result) in [first, second].into_iter().enumerate() {
                    if let Ok(value) = &result {
                        write_json(
                            &agents[index].1.join(format!("turn-{turn}-response.json")),
                            value,
                        )?;
                    }
                    result?;
                }
            } else {
                let response = connections[0].turn(turn).await?;
                write_json(
                    &agents[0].1.join(format!("turn-{turn}-response.json")),
                    &response,
                )?;
            }
        }
        Ok(())
    }
    .await;
    for connection in &mut connections {
        connection.stop().await;
    }
    result
}
