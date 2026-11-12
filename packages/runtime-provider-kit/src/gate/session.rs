//! One actor owns the session, correlation and cancellation ordering.
use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ring::rand::{SecureRandom, SystemRandom};
use serde_json::{Value, json};
use std::future::Future;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    sync::oneshot,
};

pub(super) enum Command {
    Http {
        method: Method,
        session: Option<String>,
        data: Bytes,
        sse: bool,
        reply: oneshot::Sender<Response<GateBody>>,
    },
    Cancel {
        ticket: u64,
        id: String,
    },
    Shutdown,
}
struct Cancel {
    commands: mpsc::UnboundedSender<Command>,
    ticket: u64,
    id: String,
}
impl Drop for Cancel {
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Cancel {
            ticket: self.ticket,
            id: self.id.clone(),
        });
    }
}
pub(super) struct GateBody {
    receiver: Option<mpsc::Receiver<Bytes>>,
    one: bool,
    ended: Option<Pin<Box<tokio_util::sync::WaitForCancellationFutureOwned>>>,
    // Owned by the response itself, so dropping an unpolled response also cancels it.
    _cancel: Option<Cancel>,
}
impl GateBody {
    pub(super) fn empty() -> Self {
        Self {
            receiver: None,
            one: true,
            ended: None,
            _cancel: None,
        }
    }
}
impl Body for GateBody {
    type Data = Bytes;
    type Error = Infallible;
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<std::result::Result<Frame<Bytes>, Infallible>>> {
        if self
            .ended
            .as_mut()
            .is_some_and(|ended| ended.as_mut().poll(cx).is_ready())
        {
            self.receiver = None;
            return Poll::Ready(None);
        }
        let Some(receiver) = &mut self.receiver else {
            return Poll::Ready(None);
        };
        match receiver.poll_recv(cx) {
            Poll::Ready(Some(bytes)) => {
                if self.one {
                    self.receiver = None;
                }
                Poll::Ready(Some(Ok(Frame::data(bytes))))
            }
            Poll::Ready(None) => {
                self.receiver = None;
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
    fn is_end_stream(&self) -> bool {
        self.receiver.is_none()
    }
}
struct Pending {
    ticket: u64,
    id: Value,
    sender: mpsc::Sender<Bytes>,
    sse: bool,
}
struct Session {
    id: String,
    pending: HashMap<String, Pending>,
    stream: Option<Stream>,
    buffered: VecDeque<Bytes>,
}
struct Stream {
    sender: mpsc::Sender<Bytes>,
    stop: CancellationToken,
}
impl Drop for Stream {
    fn drop(&mut self) {
        self.stop.cancel();
    }
}
struct State {
    session: Option<Session>,
    // Stdio has no session discriminator. A cancelled ID cannot safely be reused: an old
    // reply would be indistinguishable from the new request's reply.
    cancelled: HashSet<String>,
    ticket: u64,
    commands: mpsc::UnboundedSender<Command>,
    output: mpsc::UnboundedSender<Bytes>,
    config: GateConfig,
}
impl State {
    fn cancel(&mut self, id: &str, ticket: u64) {
        let Some(session) = self.session.as_mut() else {
            return;
        };
        if session.pending.get(id).is_some_and(|p| p.ticket == ticket) {
            let pending = session.pending.remove(id).unwrap();
            self.cancelled.insert(id.to_owned());
            self.send_cancel(pending.id);
        }
    }
    fn send_cancel(&self, id: Value) {
        let data =
            json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":id}});
        let _ = self.output.send(Bytes::from(data.to_string()));
    }
    fn end_session(&mut self) {
        if let Some(session) = self.session.take() {
            for (id, pending) in session.pending {
                self.cancelled.insert(id);
                self.send_cancel(pending.id);
            }
        }
    }
    fn http(
        &mut self,
        method: Method,
        supplied_session: Option<String>,
        data: Bytes,
        sse: bool,
    ) -> Response<GateBody> {
        let message = if method == Method::POST {
            // The stdio transport admits exactly one JSON object per line. Refusals never
            // carry JSON-RPC, which the engine bridge would forward regardless of status.
            if data.contains(&b'\n') || data.contains(&b'\r') {
                return empty(StatusCode::BAD_REQUEST);
            }
            match serde_json::from_slice::<Value>(&data) {
                Ok(v) if v.is_object() && v["jsonrpc"] == "2.0" => Some(v),
                _ => return empty(StatusCode::BAD_REQUEST),
            }
        } else {
            None
        };
        let request_id = message
            .as_ref()
            .filter(|v| v.get("method").is_some())
            .and_then(|v| v.get("id"));
        let id_key = match request_id {
            Some(Value::String(_) | Value::Number(_)) => Some(request_id.unwrap().to_string()),
            Some(_) => return empty(StatusCode::BAD_REQUEST),
            None => None,
        };
        if message
            .as_ref()
            .is_some_and(|v| v.get("method").is_some_and(|m| !m.is_string()))
        {
            return empty(StatusCode::BAD_REQUEST);
        }
        let initialize = message
            .as_ref()
            .is_some_and(|v| v["method"] == "initialize")
            && id_key.is_some();
        if initialize {
            if self.cancelled.contains(id_key.as_ref().unwrap()) {
                return empty(StatusCode::CONFLICT);
            }
            // Do not tear down a session before random generation has succeeded.
            let mut random = [0; 32];
            if SystemRandom::new().fill(&mut random).is_err() {
                return empty(StatusCode::SERVICE_UNAVAILABLE);
            }
            self.end_session();
            // Replacement might just have cancelled another initialize with this same ID.
            if self.cancelled.contains(id_key.as_ref().unwrap()) {
                return empty(StatusCode::CONFLICT);
            }
            self.session = Some(Session {
                id: URL_SAFE_NO_PAD.encode(random),
                pending: HashMap::new(),
                stream: None,
                buffered: VecDeque::new(),
            });
        } else {
            let Some(id) = supplied_session else {
                return empty(StatusCode::BAD_REQUEST);
            };
            if self.session.as_ref().is_none_or(|s| s.id != id) {
                return empty(StatusCode::NOT_FOUND);
            }
        }
        if method == Method::DELETE {
            self.end_session();
            return empty(StatusCode::OK);
        }
        let session = self.session.as_mut().unwrap();
        if method == Method::GET {
            let (send, receiver) = mpsc::channel(self.config.buffer_limit);
            while let Some(data) = session.buffered.pop_front() {
                let _ = send.try_send(event(data));
            }
            let stop = CancellationToken::new();
            session.stream = Some(Stream {
                sender: send,
                stop: stop.clone(),
            });
            return stream_response(
                GateBody {
                    receiver: Some(receiver),
                    one: false,
                    ended: Some(Box::pin(stop.cancelled_owned())),
                    _cancel: None,
                },
                true,
                None,
            );
        }
        let mut response = if let Some(id) = id_key {
            if self.cancelled.contains(&id) || session.pending.contains_key(&id) {
                return empty(StatusCode::CONFLICT);
            }
            self.ticket += 1;
            let (send, receiver) = mpsc::channel(1);
            let guard = Cancel {
                commands: self.commands.clone(),
                ticket: self.ticket,
                id: id.clone(),
            };
            session.pending.insert(
                id,
                Pending {
                    ticket: self.ticket,
                    id: request_id.unwrap().clone(),
                    sender: send,
                    sse,
                },
            );
            stream_response(
                GateBody {
                    receiver: Some(receiver),
                    one: true,
                    ended: None,
                    _cancel: Some(guard),
                },
                sse,
                None,
            )
        } else {
            empty(StatusCode::ACCEPTED)
        };
        if initialize {
            response.headers_mut().insert(
                "mcp-session-id",
                hyper::header::HeaderValue::from_str(&session.id).unwrap(),
            );
        }
        let _ = self.output.send(data);
        response
    }
    fn line(&mut self, line: Bytes) -> bool {
        let Ok(message) = serde_json::from_slice::<Value>(&line) else {
            return false;
        };
        if !message.is_object() || message["jsonrpc"] != "2.0" {
            return false;
        }
        let Some(session) = self.session.as_mut() else {
            return true;
        };
        if message.get("method").is_none() {
            if let Some(id) = message.get("id")
                && let Some(pending) = session.pending.remove(&id.to_string())
                && pending
                    .sender
                    .try_send(if pending.sse { event(line) } else { line })
                    .is_err()
            {
                // The HTTP response may have dropped before its queued Cancel reaches
                // this actor. A simultaneously arriving reply must not hide that cut.
                self.cancelled.insert(id.to_string());
                self.send_cancel(pending.id);
            }
            // Unmatched responses, including every late cancelled reply, never enter GET.
            return true;
        }
        if let Some(stream) = session.stream.take() {
            match stream.sender.try_send(event(line.clone())) {
                Ok(()) => {
                    session.stream = Some(stream);
                    return true;
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {}
                Err(mpsc::error::TrySendError::Full(_)) => {
                    self.end_session();
                    return true;
                }
            }
        }
        if session.buffered.len() == self.config.buffer_limit {
            // Never silently drop a server request. Overflow ends the session, not the run.
            self.end_session();
        } else {
            session.buffered.push_back(line);
        }
        true
    }
}
fn event(data: Bytes) -> Bytes {
    let mut frame = Vec::with_capacity(data.len() + 8);
    frame.extend_from_slice(b"data: ");
    frame.extend_from_slice(&data);
    frame.extend_from_slice(b"\n\n");
    frame.into()
}
fn stream_response(body: GateBody, sse: bool, session: Option<&str>) -> Response<GateBody> {
    let mut response = Response::new(body);
    response.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        hyper::header::HeaderValue::from_static(if sse {
            "text/event-stream"
        } else {
            "application/json"
        }),
    );
    if sse {
        response.headers_mut().insert(
            hyper::header::CACHE_CONTROL,
            hyper::header::HeaderValue::from_static("no-cache"),
        );
    }
    if let Some(session) = session {
        response.headers_mut().insert(
            "mcp-session-id",
            hyper::header::HeaderValue::from_str(session).unwrap(),
        );
    }
    response
}
pub(super) async fn run(
    channel: Channel,
    mut receiver: mpsc::UnboundedReceiver<Command>,
    commands: mpsc::UnboundedSender<Command>,
    config: GateConfig,
    stop: CancellationToken,
) {
    let (reader, mut writer) = tokio::io::split(channel);
    let mut reader = BufReader::new(reader).take(config.message_limit as u64 + 2);
    let (output, mut input) = mpsc::unbounded_channel::<Bytes>();
    let writer_stop = stop.clone();
    let writer_task = tokio::spawn(async move {
        while let Some(bytes) = input.recv().await {
            if writer.write_all(&bytes).await.is_err()
                || writer.write_all(b"\n").await.is_err()
                || writer.flush().await.is_err()
            {
                writer_stop.cancel();
                break;
            }
        }
        // Dropping the writer closes this half even if the backend's shutdown stalls.
    });
    let mut state = State {
        session: None,
        cancelled: HashSet::new(),
        ticket: 0,
        commands,
        output,
        config,
    };
    let mut line = Vec::new();
    loop {
        tokio::select! {
            command = receiver.recv() => match command {
                Some(Command::Http { method, session, data, sse, reply }) => { let _ = reply.send(state.http(method, session, data, sse)); },
                Some(Command::Cancel { ticket, id }) => state.cancel(&id, ticket),
                Some(Command::Shutdown) | None => break,
            },
            read = reader.read_until(b'\n', &mut line) => {
                let Ok(n) = read else { break; };
                if n == 0 || line.last() != Some(&b'\n') { break; }
                line.pop();
                if line.last() == Some(&b'\r') { line.pop(); }
                if line.len() > state.config.message_limit || line.contains(&b'\r') { break; }
                if !state.line(Bytes::from(std::mem::take(&mut line))) { break; }
                reader.set_limit(state.config.message_limit as u64 + 2);
            }
        }
    }
    state.end_session();
    drop(state);
    drop(reader);
    // Give queued cancellations a chance to reach a cooperative run, but close must also
    // finish if a run has stopped reading its pipe. No payload survives this task.
    let mut writer_task = writer_task;
    if tokio::time::timeout(Duration::from_secs(1), &mut writer_task)
        .await
        .is_err()
    {
        writer_task.abort();
        let _ = writer_task.await;
    }
    stop.cancel();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dropped_response_is_cancelled_even_if_reply_precedes_queued_cancel() {
        let (commands, mut command_rx) = mpsc::unbounded_channel();
        let (output, mut output_rx) = mpsc::unbounded_channel();
        let mut state = State {
            session: None,
            cancelled: HashSet::new(),
            ticket: 0,
            commands,
            output,
            config: GateConfig::default(),
        };
        let response = state.http(
            Method::POST,
            None,
            Bytes::from_static(br#"{"jsonrpc":"2.0","id":0,"method":"initialize"}"#),
            false,
        );
        assert!(response.status() == StatusCode::OK);
        assert!(output_rx.try_recv().is_ok());
        drop(response);
        // Deliberately process the reply before the Drop guard's cancellation command.
        assert!(state.line(Bytes::from_static(
            br#"{"jsonrpc":"2.0","id":0,"result":{}}"#
        )));
        let cancel: Value = serde_json::from_slice(&output_rx.try_recv().unwrap()).unwrap();
        assert!(
            cancel["method"] == "notifications/cancelled" && cancel["params"]["requestId"] == 0
        );
        if let Command::Cancel { ticket, id } = command_rx.try_recv().unwrap() {
            state.cancel(&id, ticket);
        }
        assert!(output_rx.try_recv().is_err());
        assert!(state.cancelled.contains("0"));
    }
}
