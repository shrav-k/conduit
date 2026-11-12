use rmcp::ServiceExt;
use scenario_s2_linear::LinearServer;

#[tokio::main]
async fn main() {
    // Never render SDK, environment, header or transport errors: they can contain values.
    if run().await.is_err() {
        eprintln!("S2 Linear server initialization or stdio transport failed");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), ()> {
    let endpoint = std::env::var("S2_LINEAR_BASE_URL").map_err(|_| ())?;
    let key = std::env::var("S2_LINEAR_API_KEY").map_err(|_| ())?;
    if key.is_empty() {
        return Err(());
    }
    let server_key = key.clone();
    // Every line the server writes passes through a redactor before stdout, so that no
    // message the SDK builds on its own (a protocol error echoing a field, for one) can carry
    // the key, in its raw or its JSON-escaped form.
    let escaped = serde_json::to_string(&key).map_err(|_| ())?;
    let escaped = escaped[1..escaped.len() - 1].to_owned();
    let (server_out, redactor_in) = tokio::io::duplex(64 * 1024);
    let pump = tokio::spawn(async move {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        let mut lines = BufReader::new(redactor_in).lines();
        let mut out = tokio::io::stdout();
        while let Ok(Some(line)) = lines.next_line().await {
            let line = line
                .replace(&key, "[redacted]")
                .replace(&escaped, "[redacted]");
            if out.write_all(line.as_bytes()).await.is_err()
                || out.write_all(b"\n").await.is_err()
                || out.flush().await.is_err()
            {
                break;
            }
        }
    });
    // Await the pump on every path: when the server or its session ends, early or not, the
    // server's writer is dropped, the pump reads to its end and every answer reaches stdout.
    let served = async move {
        LinearServer::new(endpoint, server_key)
            .map_err(|_| ())?
            .serve((tokio::io::stdin(), server_out))
            .await
            .map_err(|_| ())?
            .waiting()
            .await
            .map_err(|_| ())?;
        Ok::<(), ()>(())
    }
    .await;
    let pumped = pump.await.map_err(|_| ());
    served.and(pumped)
}
