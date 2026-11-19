use std::fs::{self, File, FileTimes, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use refresh_probe::OVERRIDE;
use serde_json::{Value, json};

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--idle-descendant") {
        // The ignored TERM disposition is inherited from the fake server.
        loop {
            thread::park();
        }
    }
    if run().is_err() {
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let home = std::env::var_os("HOME").ok_or_else(|| io::Error::other("fake home"))?;
    let codex =
        std::env::var_os("CODEX_HOME").ok_or_else(|| io::Error::other("fake Codex home"))?;
    let home = Path::new(&home);
    let script: Value = serde_json::from_reader(File::open(home.join("fake-script.json"))?)?;
    let mut descendant = if script["ignore_term"] == true {
        // The tests exercise the real group-stop path.
        unsafe { libc::signal(libc::SIGTERM, libc::SIG_IGN) };
        Some(
            Command::new(std::env::current_exe()?)
                .arg("--idle-descendant")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        )
    } else {
        None
    };
    let leader = std::process::id() as i32;
    let mut pids = leader.to_string();
    if let Some(child) = &mut descendant {
        let pid = child.id() as i32;
        // Record the pair only after confirming both live in the leader's group.
        if unsafe { libc::getpgid(leader) != leader || libc::getpgid(pid) != leader }
            || child.try_wait()?.is_some()
        {
            return Err(io::Error::other("fake descendant not in client group"));
        }
        pids.push_str(&format!(" {pid}"));
    }
    fs::write(home.join("fake-pid"), pids)?;

    let mut record = OpenOptions::new()
        .create(true)
        .append(true)
        .open(home.join("fake-record.jsonl"))?;
    let mut names: Vec<_> = std::env::vars_os()
        .map(|(name, _)| name.to_string_lossy().into_owned())
        .collect();
    names.sort();
    writeln!(
        record,
        "{}",
        json!({"environment":names,"override":std::env::var(OVERRIDE).ok()})
    )?;

    let mut stdout = io::stdout().lock();
    for line in io::stdin().lock().lines() {
        let line = line?;
        writeln!(record, "{line}")?;
        let request: Value = serde_json::from_str(&line)?;
        match request["method"].as_str() {
            Some("initialize") => {
                if let Some(method) = script["notification"].as_str() {
                    writeln!(stdout, "{}", json!({"method": method}))?;
                }
                if let Some(line) = script["initialize_line"].as_str() {
                    writeln!(stdout, "{line}")?;
                } else {
                    writeln!(stdout, "{}", json!({"id": 1, "result": {}}))?;
                }
                stdout.flush()?;
            }
            Some("account/read") => {
                let action = script["action"].as_str().unwrap_or("nothing");
                let delay = Duration::from_millis(script["delay_ms"].as_u64().unwrap_or(0));
                if delay.is_zero() {
                    apply(action, Path::new(&codex))?;
                }
                if script["eof"] == true {
                    return Ok(());
                }
                if let Some(line) = script["account_line"].as_str() {
                    writeln!(stdout, "{line}")?;
                } else {
                    writeln!(stdout, "{}", json!({"id": 2, "result": {"account": null}}))?;
                }
                stdout.flush()?;
                if !delay.is_zero() {
                    thread::sleep(delay);
                    apply(action, Path::new(&codex))?;
                }
            }
            _ => {}
        }
    }
    if let Some(mut child) = descendant {
        child.kill()?;
        child.wait()?;
    }
    Ok(())
}

fn apply(action: &str, home: &Path) -> io::Result<()> {
    let login = home.join("auth.json");
    match action {
        "nothing" => Ok(()),
        "rewrite" => OpenOptions::new()
            .append(true)
            .open(login)?
            .write_all(b"\n"),
        "rename" => {
            let replacement = home.join("replacement");
            fs::copy(&login, &replacement)?;
            let modified = fs::metadata(&login)?.modified()?;
            File::open(&replacement)?.set_times(FileTimes::new().set_modified(modified))?;
            fs::rename(replacement, login)
        }
        "mtime" => {
            let file = File::options().write(true).open(login)?;
            let modified = file.metadata()?.modified()? - Duration::from_secs(20);
            file.set_times(FileTimes::new().set_modified(modified))
        }
        "auth.lock" | "auth.json.lock" => fs::write(home.join(action), b""),
        "remove" => fs::remove_file(login),
        "directory" => fs::create_dir_all(home.join("tmp/x")),
        _ => Err(io::Error::other("unknown fake action")),
    }
}
