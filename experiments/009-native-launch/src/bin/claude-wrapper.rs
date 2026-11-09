use clap::Parser;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("--headers-helper") {
        return native_launch_harness::headers_helper();
    }
    native_launch_harness::wrapper::serve(
        native_launch_harness::Client::Claude,
        native_launch_harness::wrapper::Args::parse(),
    )
    .await
}
