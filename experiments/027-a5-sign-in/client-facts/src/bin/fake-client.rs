//! Synthetic client for integration tests only; never consults a real client home.
use std::fs::{self, OpenOptions};
use std::io::{Write, stdout};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use url::Url;

fn marker(name: &str) -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap()).join(name)
}
fn record(name: &str) {
    writeln!(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(marker(name))
            .unwrap(),
        "yes"
    )
    .unwrap();
}
fn authorization_url(listener: &TcpListener) -> Url {
    let mut url = Url::parse("https://example.org/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &"c".repeat(16))
        .append_pair("response_type", "code")
        .append_pair(
            "redirect_uri",
            &format!(
                "http://127.0.0.1:{}/callback",
                listener.local_addr().unwrap().port()
            ),
        )
        .append_pair("code_challenge_method", "S256")
        .append_pair("code_challenge", &"p".repeat(43))
        .append_pair("state", &"s".repeat(32));
    url
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--version"] {
        println!("fixture 1.2.3");
        return;
    }
    // The scratch home, named by whichever home variable the probe set.
    let scratch = std::env::var_os("CLAUDE_CONFIG_DIR")
        .or_else(|| std::env::var_os("CODEX_HOME"))
        .map(PathBuf::from);
    if args.iter().any(|a| a == "status") {
        if scratch
            .as_ref()
            .is_none_or(|home| home.join("signed-in").exists())
            || marker("shared-home").exists()
        {
            println!("Logged in\nEmail: {}@example.org", "a".repeat(8));
        } else {
            println!("Not logged in");
        }
        return;
    }
    if args.iter().any(|a| a == "logout") {
        // A logout outside a scratch home would touch the everyday login.
        let home = scratch.expect("logout without a scratch home");
        record("logout-started");
        fs::remove_file(home.join("signed-in")).unwrap();
        return;
    }
    if !args.iter().any(|a| a == "login") {
        std::process::exit(2);
    }
    record("login-started");
    // Fail the test if the probe gives a controlling terminal or leaks its environment.
    // SAFETY: these functions only inspect the current process and null stdin.
    unsafe {
        assert_eq!(libc::getsid(0), libc::getpid());
        assert_eq!(libc::getpgrp(), libc::getpid());
        assert_eq!(libc::isatty(0), 0);
    }
    for (key, _) in std::env::vars_os() {
        assert!(
            [
                "HOME",
                "USER",
                "LOGNAME",
                "PATH",
                "BROWSER",
                "A5_SHIM_LOG",
                "CLAUDE_CONFIG_DIR",
                "CODEX_HOME"
            ]
            .iter()
            .any(|allowed| key == *allowed)
        );
    }
    if args.iter().any(|a| a == "--mode=complete") {
        // A login that signs in at once, so that step 5 finds the scratch home signed in.
        fs::write(scratch.unwrap().join("signed-in"), "yes").unwrap();
        return;
    }
    let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
    let url = authorization_url(&listener);
    if args.iter().any(|a| a == "--mode=ignore-term") {
        // SAFETY: ignoring TERM is the explicit synthetic cancellation test.
        unsafe {
            libc::signal(libc::SIGTERM, libc::SIG_IGN);
        }
    }
    if args.iter().any(|a| a == "--mode=browser") {
        assert!(
            Command::new(std::env::var_os("BROWSER").unwrap())
                .arg(url.as_str())
                .status()
                .unwrap()
                .success()
        );
    } else {
        println!("{url}");
        stdout().flush().unwrap();
    }
    // Emit only when the probe reaches its listener checks, after its initial URL
    // snapshot. Keep both listeners alive until cancellation.
    let _second_listener = if args.iter().any(|a| a == "--mode=late-redirect") {
        listener.accept().unwrap();
        let second = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        println!("{}", authorization_url(&second));
        stdout().flush().unwrap();
        Some(second)
    } else {
        None
    };
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
