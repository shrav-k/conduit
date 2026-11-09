use clap::Parser;
#[derive(Parser)]
struct Args {
    /// Required even for a dry run. A fresh absolute directory outside this repository.
    #[arg(long)]
    output: std::path::PathBuf,
    #[arg(long)]
    dry_run: bool,
    #[arg(long, default_value_t = 180, value_parser = clap::value_parser!(u64).range(1..))]
    timeout_secs: u64,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.dry_run {
        return native_launch_harness::runner::dry_run(&args.output, args.timeout_secs);
    }
    native_launch_harness::runner::run(&args.output, args.timeout_secs).await
}
