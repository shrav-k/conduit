use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
};

use s2_linear_stub::{INTERFACE_VERSION, IssueFixture, Stub, StubConfig};
use serde_json::json;

#[tokio::main]
async fn main() {
    if run().await.is_err() {
        eprintln!("stub failed; use: s2-linear-stub FIXTURE_JSON NEW_PRIVATE_CONNECTION_JSON");
        std::process::exit(1);
    }
}

async fn run() -> io::Result<()> {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).collect();
    if args.len() != 2 {
        return Err(io::Error::other("expected two paths"));
    }
    let issues: Vec<IssueFixture> = serde_json::from_slice(&std::fs::read(&args[0])?)
        .map_err(|_| io::Error::other("invalid synthetic fixture"))?;
    let config = StubConfig::new(issues);
    let api_key = config.api_key.clone();
    let control_token = config.control_token.clone();
    let stub = Stub::start(config).await?;
    let connection = json!({
        "interface_version": INTERFACE_VERSION,
        "graphql_url": stub.graphql_url(), "control_url": stub.control_url(),
        "api_key": api_key.expose(), "control_token": control_token.expose(),
    });
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&args[1])?;
    file.write_all(serde_json::to_string(&connection)?.as_bytes())?;
    file.sync_all()?;
    println!(
        "S2 stub interface {INTERFACE_VERSION} listening on {}",
        stub.address()
    );
    tokio::signal::ctrl_c().await?;
    stub.shutdown().await
}
