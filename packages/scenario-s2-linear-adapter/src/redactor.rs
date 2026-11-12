//! Line buffering keeps secrets split across transport writes from escaping.
use std::{
    io,
    pin::Pin,
    task::{Context, Poll, ready},
};
use tokio::io::AsyncWrite;

pub struct Redactor<W> {
    inner: W,
    patterns: Vec<String>,
    line: Vec<u8>,
    pending: Vec<u8>,
    offset: usize,
}
impl<W: AsyncWrite + Unpin> Redactor<W> {
    pub fn new(inner: W, key: &str) -> Self {
        let encoded = serde_json::to_string(key).unwrap();
        Self {
            inner,
            patterns: vec![encoded[1..encoded.len() - 1].to_owned(), key.to_owned()],
            line: Vec::new(),
            pending: Vec::new(),
            offset: 0,
        }
    }
    fn redact_line(&mut self) -> io::Result<()> {
        let mut text = String::from_utf8(std::mem::take(&mut self.line))
            .map_err(|_| io::Error::other("Invalid protocol encoding"))?;
        for pattern in &self.patterns {
            if !pattern.is_empty() {
                text = text.replace(pattern, "[redacted]");
            }
        }
        self.pending.extend_from_slice(text.as_bytes());
        Ok(())
    }
    fn drain(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        while self.offset < self.pending.len() {
            let n = ready!(Pin::new(&mut self.inner).poll_write(cx, &self.pending[self.offset..]))?;
            if n == 0 {
                return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
            self.offset += n;
        }
        self.pending.clear();
        self.offset = 0;
        Poll::Ready(Ok(()))
    }
}
impl<W: AsyncWrite + Unpin> AsyncWrite for Redactor<W> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        ready!(this.drain(cx))?;
        for byte in bytes {
            this.line.push(*byte);
            if *byte == b'\n' {
                this.redact_line()?;
            }
        }
        // The framing sink calls poll_flush after each message, including its last.
        Poll::Ready(Ok(bytes.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        ready!(this.drain(cx))?;
        Pin::new(&mut this.inner).poll_flush(cx)
    }
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if !this.line.is_empty() {
            this.redact_line()?;
        }
        ready!(this.drain(cx))?;
        Pin::new(&mut this.inner).poll_shutdown(cx)
    }
}
