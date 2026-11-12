use crate::backend::Channel;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_util::sync::CancellationToken;
/// Observes EOF/errors even though ownership of the stream passes to the gate.
pub(crate) struct Observed {
    pub channel: Channel,
    pub ended: CancellationToken,
}
impl Drop for Observed {
    fn drop(&mut self) {
        self.ended.cancel();
    }
}
impl AsyncRead for Observed {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let before = buf.filled().len();
        let nonempty = buf.remaining() > 0;
        let result = Pin::new(&mut self.channel).poll_read(cx, buf);
        if matches!(&result, Poll::Ready(Err(_)))
            || (nonempty && matches!(&result, Poll::Ready(Ok(()))) && buf.filled().len() == before)
        {
            self.ended.cancel();
        }
        result
    }
}
impl AsyncWrite for Observed {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let result = Pin::new(&mut self.channel).poll_write(cx, buf);
        if matches!(&result, Poll::Ready(Err(_)))
            || (!buf.is_empty() && matches!(&result, Poll::Ready(Ok(0))))
        {
            self.ended.cancel();
        }
        result
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let result = Pin::new(&mut self.channel).poll_flush(cx);
        if matches!(&result, Poll::Ready(Err(_))) {
            self.ended.cancel();
        }
        result
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let result = Pin::new(&mut self.channel).poll_shutdown(cx);
        if result.is_ready() {
            self.ended.cancel();
        }
        result
    }
}
