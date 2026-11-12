use serde_json::Value;
use std::{
    io,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, ReadBuf};
const LIMIT: usize = 16 * 1024 * 1024;
#[derive(Default)]
struct Data {
    wire: Vec<u8>,
    logs: Vec<u8>,
    overflow: bool,
}
#[derive(Clone, Default)]
pub struct Observations(Arc<Mutex<Data>>);
impl Observations {
    pub fn add(&self, bytes: &[u8]) {
        self.append(bytes, false);
    }
    pub fn add_wire(&self, bytes: &[u8]) {
        self.append(bytes, true);
    }
    fn append(&self, bytes: &[u8], wire: bool) {
        let mut held = self.0.lock().unwrap();
        let left = LIMIT.saturating_sub(held.wire.len() + held.logs.len());
        let dst = if wire { &mut held.wire } else { &mut held.logs };
        dst.extend_from_slice(&bytes[..bytes.len().min(left)]);
        held.overflow |= bytes.len() > left;
    }
    pub fn contains_marker(&self, markers: &[String]) -> bool {
        let held = self.0.lock().unwrap();
        held.overflow
            || markers.iter().any(|m| {
                if m.is_empty() {
                    return true;
                }
                [&held.wire, &held.logs]
                    .iter()
                    .any(|b| b.windows(m.len()).any(|w| w == m.as_bytes()))
                    || held.wire.split(|b| *b == b'\n').any(|line| {
                        serde_json::from_slice::<Value>(line).is_ok_and(|v| decoded_contains(&v, m))
                    })
            })
    }
    pub fn overflowed(&self) -> bool {
        self.0.lock().unwrap().overflow
    }
}
fn decoded_contains(v: &Value, marker: &str) -> bool {
    match v {
        Value::String(s) => s.contains(marker),
        Value::Array(a) => a.iter().any(|v| decoded_contains(v, marker)),
        Value::Object(m) => m
            .iter()
            .any(|(k, v)| k.contains(marker) || decoded_contains(v, marker)),
        _ => false,
    }
}
// The MCP parser may stop on malformed input. A separate reader still observes the
// remaining stdout through EOF, without depending on successful protocol parsing.
pub struct WireReader {
    receiver: tokio::sync::mpsc::UnboundedReceiver<Vec<u8>>,
    pending: std::io::Cursor<Vec<u8>>,
}
impl WireReader {
    pub fn channel() -> (tokio::sync::mpsc::UnboundedSender<Vec<u8>>, Self) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        (
            sender,
            Self {
                receiver,
                pending: std::io::Cursor::new(Vec::new()),
            },
        )
    }
}
impl AsyncRead for WireReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.pending.position() as usize >= this.pending.get_ref().len() {
            match this.receiver.poll_recv(cx) {
                Poll::Ready(Some(bytes)) => this.pending = std::io::Cursor::new(bytes),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => return Poll::Pending,
            }
        }
        Pin::new(&mut this.pending).poll_read(cx, buf)
    }
}
pub async fn pump(
    mut stdout: impl AsyncRead + Unpin,
    observations: Observations,
    sender: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
) {
    use tokio::io::AsyncReadExt;
    let mut buf = [0; 4096];
    let mut sender = Some(sender);
    while let Ok(n) = stdout.read(&mut buf).await {
        if n == 0 {
            break;
        }
        observations.add_wire(&buf[..n]);
        if observations.overflowed() {
            sender.take();
        }
        if let Some(tx) = &sender {
            let _ = tx.send(buf[..n].to_vec());
        }
    }
}
