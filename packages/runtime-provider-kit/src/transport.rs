//! Keep provider inputs out of rmcp's Debug/Trace request logging. Private extensions have
//! opaque Debug output in rmcp 3.9.0. This transport also never logs malformed input lines.
use crate::names;
use rmcp::{
    RoleServer,
    model::*,
    service::{RxJsonRpcMessage, TxJsonRpcMessage},
    transport::Transport,
};
use std::{io, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    sync::Mutex,
};

#[derive(Clone)]
pub(crate) struct PrivateCall {
    pub params: CallToolRequestParams,
    pub meta: RequestMetaObject,
}
pub(crate) struct PrivateTransport<R, W> {
    read: BufReader<R>,
    write: Arc<Mutex<W>>,
    line: Vec<u8>,
}
impl<R: AsyncRead, W> PrivateTransport<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self {
            read: BufReader::new(read),
            write: Arc::new(Mutex::new(write)),
            line: Vec::new(),
        }
    }
}
impl<R, W> Transport<RoleServer> for PrivateTransport<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send + 'static,
{
    type Error = io::Error;
    fn send(
        &mut self,
        item: TxJsonRpcMessage<RoleServer>,
    ) -> impl Future<Output = io::Result<()>> + Send + 'static {
        let write = self.write.clone();
        async move {
            let mut bytes = serde_json::to_vec(&item)
                .map_err(|_| io::Error::other("cannot encode MCP result"))?;
            bytes.push(b'\n');
            let mut write = write.lock().await;
            write.write_all(&bytes).await?;
            write.flush().await
        }
    }
    async fn receive(&mut self) -> Option<RxJsonRpcMessage<RoleServer>> {
        // Keep partial input across cancellation of receive by the MCP event loop.
        loop {
            let n = self.read.read_until(b'\n', &mut self.line).await.ok()?;
            if n == 0 {
                return None;
            }
            if self.line.iter().all(u8::is_ascii_whitespace) {
                self.line.clear();
                continue;
            }
            let message = serde_json::from_slice::<RxJsonRpcMessage<RoleServer>>(&self.line);
            // The byte buffer contains credentials until this point; retain no copy.
            self.line.clear();
            let mut message = message.ok()?;
            if let JsonRpcMessage::Request(request) = &mut message {
                let meta = request.request.get_meta().clone();
                if let ClientRequest::CallToolRequest(call) = &mut request.request {
                    let params = std::mem::replace(
                        &mut call.params,
                        CallToolRequestParams::new("runtime-operation"),
                    );
                    call.extensions.insert(PrivateCall { params, meta });
                    if let Some(meta) = call.extensions.get_mut::<RequestMetaObject>() {
                        meta.retain(|k, _| !k.starts_with(names::META_PREFIX));
                    }
                }
            }
            return Some(message);
        }
    }
    async fn close(&mut self) -> io::Result<()> {
        self.write.lock().await.shutdown().await
    }
}
