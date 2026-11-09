use anyhow::{Result, bail};
use clap::Parser;
use real_client_harness::{
    runner,
    stub::{self, Case},
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Experiment 008: fixed real-client matrix; no agent is run with --dry-run")]
struct Args {
    #[arg(long)]
    dry_run: bool,
    #[arg(long)]
    follow_up: bool,
    #[arg(long)]
    only: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,
    #[arg(long)]
    serve: bool,
    #[arg(long, value_enum, default_value = "c1")]
    case: Case,
    #[arg(long, default_value_t = 0)]
    ttl_ms: u64,
    #[arg(long)]
    first_401: bool,
    #[arg(long, default_value_t = 0)]
    port: u16,
    #[arg(long)]
    headers_helper: bool,
    #[arg(long)]
    identity_dir: Option<PathBuf>,
    #[arg(long)]
    helper_log: Option<PathBuf>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.headers_helper {
        return stub::headers_helper(
            &args
                .identity_dir
                .ok_or_else(|| anyhow::anyhow!("--identity-dir required"))?,
            &args
                .helper_log
                .ok_or_else(|| anyhow::anyhow!("--helper-log required"))?,
        );
    }
    // A dry run may render a placeholder; no real output directory has a default.
    let output = match args.output {
        Some(output) => output,
        None if args.dry_run && !args.serve => PathBuf::from("<output>"),
        None => bail!("provide --output /absolute/path/outside/repository for an actual run"),
    };
    if args.serve {
        if args.dry_run {
            bail!("--serve and --dry-run cannot be combined")
        }
        let output = runner::prepare_output(&output)?;
        let server =
            stub::start(args.case, args.ttl_ms, args.first_401, &output, args.port).await?;
        println!("{}", server.url);
        tokio::signal::ctrl_c().await?;
        server.cancel.cancel();
        server.task.abort();
        return Ok(());
    }
    runner::run(
        &output,
        args.dry_run,
        args.only.as_deref(),
        args.timeout_secs,
        args.follow_up,
    )
    .await
}
