//! One actor owns the session, correlation and cancellation ordering.
use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use ring::rand::{SecureRandom, SystemRandom};
use serde::de::{DeserializeOwned, IgnoredAny};
use serde_json::{Value, json};
use std::future::Future;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Range,
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
    raw_id: Bytes,
    sender: mpsc::Sender<Bytes>,
    sse: bool,
    dispatch: Dispatch,
    cancel_sent: bool,
}
enum Dispatch {
    Initialize,
    Waiting { data: Bytes, id: Range<usize> },
    Sent(Value),
}
impl Pending {
    fn respond(&self, data: Bytes) -> bool {
        self.sender
            .try_send(if self.sse { event(data) } else { data })
            .is_ok()
    }

    fn respond_to(&self, data: Bytes, wire_id: &Value, span: Range<usize>) -> bool {
        self.respond(if self.id == *wire_id {
            data
        } else {
            replace_id(data, span, &self.raw_id)
        })
    }
}
enum Initialization {
    Waiting(String),
    Ready {
        response: Bytes,
        id: Range<usize>,
        notified: bool,
    },
}
struct Session {
    id: String,
    pending: HashMap<String, Pending>,
    sent: HashMap<String, String>,
    cancelled: HashSet<String>,
    initializing: Option<String>,
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
    initialization: Option<Initialization>,
    // These stdio IDs may still receive a reply after their HTTP request has ended.
    retired: HashSet<String>,
    ticket: u64,
    commands: mpsc::UnboundedSender<Command>,
    output: mpsc::UnboundedSender<Bytes>,
    config: GateConfig,
}
impl State {
    fn cancel(&mut self, id: &str, ticket: u64) {
        if self
            .session
            .as_ref()
            .and_then(|s| s.pending.get(id))
            .is_some_and(|p| p.ticket == ticket)
        {
            self.cancel_request(id);
        }
    }

    fn retire(&mut self, pending: Pending) -> Option<String> {
        let wire_id = match &pending.dispatch {
            Dispatch::Initialize => {
                if let Some(Initialization::Waiting(id)) = &self.initialization {
                    self.retired.insert(id.clone());
                }
                return None;
            }
            Dispatch::Waiting { .. } => return None,
            Dispatch::Sent(id) => id,
        };
        let key = wire_id.to_string();
        self.retired.insert(key.clone());
        self.send_cancel(&pending, wire_id);
        Some(key)
    }

    fn send_cancel(&self, pending: &Pending, wire_id: &Value) {
        if !pending.cancel_sent {
            let data = Bytes::from(
                json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":wire_id}})
                    .to_string(),
            );
            let _ = self.output.send(data);
        }
    }

    fn cancel_request(&mut self, key: &str) {
        let session = self.session.as_mut().unwrap();
        let Some(pending) = session.pending.remove(key) else {
            return;
        };
        session.cancelled.insert(key.to_owned());
        if matches!(pending.dispatch, Dispatch::Initialize) {
            session.initializing = None;
        }
        if let Some(wire_key) = self.retire(pending) {
            self.session.as_mut().unwrap().sent.remove(&wire_key);
            self.forward_waiting(&wire_key);
        }
    }

    fn client_cancel(&mut self, key: &str, data: Bytes, id: Range<usize>) {
        let session = self.session.as_mut().unwrap();
        let Some(pending) = session.pending.get_mut(key) else {
            return;
        };
        if matches!(pending.dispatch, Dispatch::Initialize)
            || !session.cancelled.insert(key.to_owned())
        {
            return;
        }
        let Dispatch::Sent(wire_id) = &pending.dispatch else {
            // A waiting request has never reached the server. Answer locally so
            // the bridge can finish the POST without dispatching the request.
            let pending = session.pending.remove(key).unwrap();
            pending.respond(Bytes::from(
                json!({"jsonrpc":"2.0","id":pending.id,"error":{"code":-32800,"message":"Request cancelled"}})
                    .to_string(),
            ));
            return;
        };
        // Keep a dispatched POST and its correlation alive while the server
        // processes the cancellation.
        let data = if *wire_id == pending.id {
            data
        } else {
            replace_id(data, id, wire_id.to_string().as_bytes())
        };
        pending.cancel_sent = true;
        let _ = self.output.send(data);
    }

    fn end_session(&mut self) {
        if let Some(session) = self.session.take() {
            for pending in session.pending.into_values() {
                self.retire(pending);
            }
        }
    }

    fn awaiting_initialize(&self, wire_key: &str) -> bool {
        matches!(&self.initialization, Some(Initialization::Waiting(key)) if key == wire_key)
    }

    fn wire_id(&self, key: &str, pending: &Pending) -> Value {
        if !self.retired.contains(key) {
            return pending.id.clone();
        }
        let session = self.session.as_ref().unwrap();
        let mut alias = format!("conduit-gate/{}", pending.ticket);
        loop {
            let id = Value::String(alias.clone());
            let key = id.to_string();
            if !self.retired.contains(&key)
                && !session.pending.contains_key(&key)
                && !session.sent.contains_key(&key)
                && !self.awaiting_initialize(&key)
            {
                return id;
            }
            alias.push('/');
        }
    }

    fn forward_waiting(&mut self, key: &str) {
        let Some(session) = &self.session else { return };
        // A client might choose a string already in use as a gate-assigned ID.
        // Wait for that request so the fresh client ID can still pass through.
        if session.sent.contains_key(key)
            || (!self.retired.contains(key) && self.awaiting_initialize(key))
        {
            return;
        }
        let Some(pending) = session.pending.get(key) else {
            return;
        };
        let Dispatch::Waiting { data, id: span } = &pending.dispatch else {
            return;
        };
        let id = self.wire_id(key, pending);
        let wire_key = id.to_string();
        let data = if id == pending.id {
            data.clone()
        } else {
            replace_id(data.clone(), span.clone(), wire_key.as_bytes())
        };
        let session = self.session.as_mut().unwrap();
        session.sent.insert(wire_key, key.to_owned());
        session.pending.get_mut(key).unwrap().dispatch = Dispatch::Sent(id);
        let _ = self.output.send(data);
    }

    fn reply(&mut self, id: &Value, span: Range<usize>, message: &Envelope, line: Bytes) {
        let wire_key = id.to_string();
        if message.result != message.error {
            // A JSON-RPC request has one response. Its stdio ID is now safe to reuse.
            self.retired.remove(&wire_key);
        }
        if self.awaiting_initialize(&wire_key) {
            self.initialization =
                (message.result && !message.error).then(|| Initialization::Ready {
                    response: line.clone(),
                    id: span.clone(),
                    notified: false,
                });
            if let Some(session) = &mut self.session
                && let Some(key) = session.initializing.take()
            {
                let pending = session.pending.remove(&key).unwrap();
                if !pending.respond_to(line, id, span) {
                    session.cancelled.insert(key);
                }
            }
            self.forward_waiting(&wire_key);
            return;
        }
        let Some(session) = &mut self.session else {
            return;
        };
        let Some(key) = session.sent.remove(&wire_key) else {
            return;
        };
        let pending = session.pending.remove(&key).unwrap();
        if !pending.respond_to(line, id, span) {
            // The reply may beat the Drop guard's queued Cancel to the actor.
            session.cancelled.insert(key);
            self.send_cancel(&pending, id);
        }
        self.forward_waiting(&wire_key);
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
            let Some(message) = Envelope::parse(&data) else {
                return empty(StatusCode::BAD_REQUEST);
            };
            Some(message)
        } else {
            None
        };
        let method_span = message.as_ref().and_then(|message| message.method.clone());
        let method_name =
            match method_span.map(|span| serde_json::from_slice::<String>(&data[span])) {
                Some(Ok(method)) => Some(method),
                Some(Err(_)) => return empty(StatusCode::BAD_REQUEST),
                None => None,
            };
        let request_span = message
            .as_ref()
            .filter(|_| method_name.is_some())
            .and_then(|message| message.id.clone());
        let request_id = match request_span
            .as_ref()
            .map(|span| serde_json::from_slice(&data[span.clone()]))
        {
            Some(Ok(id @ (Value::String(_) | Value::Number(_)))) => Some(id),
            Some(_) => return empty(StatusCode::BAD_REQUEST),
            None => None,
        };
        let initialize = method_name.as_deref() == Some("initialize");
        if initialize {
            if request_id.is_none() {
                return empty(StatusCode::BAD_REQUEST);
            }
            // Do not tear down a session before random generation has succeeded.
            let mut random = [0; 32];
            if SystemRandom::new().fill(&mut random).is_err() {
                return empty(StatusCode::SERVICE_UNAVAILABLE);
            }
            self.end_session();
            self.session = Some(Session {
                id: URL_SAFE_NO_PAD.encode(random),
                pending: HashMap::new(),
                sent: HashMap::new(),
                cancelled: HashSet::new(),
                initializing: None,
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
        if method_name.as_deref() == Some("notifications/initialized") {
            if request_id.is_some() {
                return empty(StatusCode::BAD_REQUEST);
            }
            if let Some(Initialization::Ready { notified, .. }) = &mut self.initialization
                && !*notified
            {
                *notified = true;
                let _ = self.output.send(data);
            }
            return empty(StatusCode::ACCEPTED);
        }
        if request_id.is_none() && method_name.as_deref() == Some("notifications/cancelled") {
            if let Some(params) = message.unwrap().params
                && let Some(span) = cancellation_id(&data, params)
                && let Ok(id) = serde_json::from_slice::<Value>(&data[span.clone()])
            {
                self.client_cancel(&id.to_string(), data, span);
            }
            return empty(StatusCode::ACCEPTED);
        }
        if let Some(id) = request_id {
            let key = id.to_string();
            let span = request_span.unwrap();
            if session.cancelled.contains(&key) || session.pending.contains_key(&key) {
                return empty(StatusCode::CONFLICT);
            }
            self.ticket += 1;
            let (send, receiver) = mpsc::channel(1);
            let guard = Cancel {
                commands: self.commands.clone(),
                ticket: self.ticket,
                id: key.clone(),
            };
            let pending = Pending {
                ticket: self.ticket,
                id,
                raw_id: Bytes::copy_from_slice(&data[span.clone()]),
                sender: send,
                sse,
                dispatch: if initialize {
                    Dispatch::Initialize
                } else {
                    Dispatch::Waiting {
                        data: data.clone(),
                        id: span.clone(),
                    }
                },
                cancel_sent: false,
            };
            let response = stream_response(
                GateBody {
                    receiver: Some(receiver),
                    one: true,
                    ended: None,
                    _cancel: Some(guard),
                },
                sse,
                initialize.then_some(session.id.as_str()),
            );
            if initialize {
                match &self.initialization {
                    Some(Initialization::Ready {
                        response: cached,
                        id,
                        ..
                    }) => {
                        pending.respond(replace_id(cached.clone(), id.clone(), &pending.raw_id));
                        return response;
                    }
                    Some(Initialization::Waiting(_)) => {}
                    None => {
                        let wire_id = self.wire_id(&key, &pending);
                        let wire_key = wire_id.to_string();
                        let data = if wire_id == pending.id {
                            data
                        } else {
                            replace_id(data, span, wire_key.as_bytes())
                        };
                        let _ = self.output.send(data);
                        self.initialization = Some(Initialization::Waiting(wire_key));
                    }
                }
                let session = self.session.as_mut().unwrap();
                session.initializing = Some(key.clone());
                session.pending.insert(key, pending);
            } else {
                session.pending.insert(key.clone(), pending);
                self.forward_waiting(&key);
            }
            response
        } else {
            let _ = self.output.send(data);
            empty(StatusCode::ACCEPTED)
        }
    }
    fn line(&mut self, line: Bytes) -> bool {
        let Some(message) = Envelope::parse(&line) else {
            return false;
        };
        if message.method.is_none() {
            if let Some(span) = message.id.clone()
                && let Ok(id) = serde_json::from_slice::<Value>(&line[span.clone()])
            {
                self.reply(&id, span, &message, line);
            }
            // Unmatched responses, including every late cancelled reply, never enter GET.
            return true;
        }
        let Some(session) = self.session.as_mut() else {
            return true;
        };
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

#[derive(Default)]
struct Envelope {
    method: Option<Range<usize>>,
    id: Option<Range<usize>>,
    params: Option<Range<usize>>,
    result: bool,
    error: bool,
}

impl Envelope {
    fn parse(data: &[u8]) -> Option<Self> {
        std::str::from_utf8(data).ok()?;
        let mut envelope = Self::default();
        let mut version = false;
        object_fields(data, |name, span| match name {
            "jsonrpc" => {
                version =
                    serde_json::from_slice::<String>(&data[span]).is_ok_and(|value| value == "2.0");
            }
            "method" => envelope.method = Some(span),
            "id" => envelope.id = Some(span),
            "params" => envelope.params = Some(span),
            "result" => envelope.result = true,
            "error" => envelope.error = true,
            _ => {}
        })?;
        version.then_some(envelope)
    }
}

fn whitespace(data: &[u8]) -> &[u8] {
    let count = data
        .iter()
        .take_while(|&&byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        .count();
    &data[count..]
}

fn take_json<T: DeserializeOwned>(rest: &mut &[u8]) -> Option<T> {
    let mut values = serde_json::Deserializer::from_slice(rest).into_iter::<T>();
    let value = values.next()?.ok()?;
    *rest = &rest[values.byte_offset()..];
    Some(value)
}

// Decode member names for matching, but only check the syntax of each value.
// Skipping values keeps their exact bytes and does not impose a native number range.
fn object_fields(data: &[u8], mut field: impl FnMut(&str, Range<usize>)) -> Option<()> {
    let mut rest = whitespace(whitespace(data).strip_prefix(b"{")?);
    if let Some(tail) = rest.strip_prefix(b"}") {
        return whitespace(tail).is_empty().then_some(());
    }
    loop {
        let name = take_json::<String>(&mut rest)?;
        rest = whitespace(whitespace(rest).strip_prefix(b":")?);
        let start = data.len() - rest.len();
        take_json::<IgnoredAny>(&mut rest)?;
        field(&name, start..data.len() - rest.len());
        rest = whitespace(rest);
        if let Some(tail) = rest.strip_prefix(b"}") {
            return whitespace(tail).is_empty().then_some(());
        }
        rest = rest.strip_prefix(b",")?;
    }
}

fn cancellation_id(data: &[u8], params: Range<usize>) -> Option<Range<usize>> {
    let mut id = None;
    object_fields(&data[params.clone()], |name, span| {
        if name == "requestId" {
            id = Some(params.start + span.start..params.start + span.end);
        }
    })?;
    id
}

fn replace_id(data: Bytes, span: Range<usize>, replacement: &[u8]) -> Bytes {
    if &data[span.clone()] == replacement {
        return data;
    }
    let mut bytes = Vec::with_capacity(data.len() - span.len() + replacement.len());
    bytes.extend_from_slice(&data[..span.start]);
    bytes.extend_from_slice(replacement);
    bytes.extend_from_slice(&data[span.end..]);
    Bytes::from(bytes)
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
        initialization: None,
        retired: HashSet::new(),
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
            initialization: None,
            retired: HashSet::new(),
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
        let session = response.headers()["mcp-session-id"]
            .to_str()
            .unwrap()
            .to_owned();
        assert!(output_rx.try_recv().is_ok());
        assert!(state.line(Bytes::from_static(
            br#"{"jsonrpc":"2.0","id":0,"result":{}}"#
        )));
        let response = state.http(
            Method::POST,
            Some(session),
            Bytes::from_static(br#"{"jsonrpc":"2.0","id":7,"method":"tools/call"}"#),
            false,
        );
        assert_eq!(response.status(), StatusCode::OK);
        assert!(output_rx.try_recv().is_ok());
        drop(response);
        // Deliberately process the reply before the Drop guard's cancellation command.
        assert!(state.line(Bytes::from_static(
            br#"{"jsonrpc":"2.0","id":7,"result":{}}"#
        )));
        let cancel: Value = serde_json::from_slice(&output_rx.try_recv().unwrap()).unwrap();
        assert!(
            cancel["method"] == "notifications/cancelled" && cancel["params"]["requestId"] == 7
        );
        if let Command::Cancel { ticket, id } = command_rx.try_recv().unwrap() {
            state.cancel(&id, ticket);
        }
        assert!(output_rx.try_recv().is_err());
        assert!(state.session.unwrap().cancelled.contains("7"));
        assert!(!state.retired.contains("7"));
    }
}
