//! MC5a: gate, connection, attribution and runner reference model; no engine code.
use crate::harness::*;
use stateright::Property;
use std::collections::BTreeMap;
use std::time::Duration;

const E: usize = 0;
const M: usize = 1;
const A: usize = 2;
// Fixed model input, not the environment of the process running these tests.
const ENGINE_ENV: [(&str, &str); 7] = [
    ("HOME", "engine-home"),
    ("PATH", "engine-path"),
    ("USER", "engine-user"),
    ("TERM", "() shell-function"),
    ("GITHUB_TOKEN", "engine-exported-token"),
    ("CONDUIT_URL", "wrong-engine-url"),
    ("CONDUIT_IDENTITY_DIR", "wrong-engine-directory"),
];
pub const GATE: &str = "PRV-025: gate admits only proof";
pub const BEFORE: &str = "PRV-028: proof before sending";
pub const ATTRIBUTION: &str = "PRV-018: identity rests on what is presented";
pub const HOLDERS: &str = "PRV-022 PRV-024: identity material stays with its holders";
pub const RUNNER: &str = "PRV-003 PRV-023 PRV-029: own runner keeps its promises";
pub const CHANNEL: &str = "PRV-011 PRV-022: channel authenticates the other machine";
pub const SERVED: &str = "MC5a: proven request reached I";
pub const REFUSED: &str = "MC5a: X refused at gate";
pub const TAKEOVER: &str = "MC5a: X took u and received connect";
pub const X_NONE: &str = "MC5a: X attributed to none";
pub const I_TOKEN: &str = "MC5a: I attributed by token";
pub const RERUN: &str = "MC5a: L1 and L2 set up after recovery";
pub const CARRIED: &str = "MC5a: channel carried values to M2";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Part {
    Base,
    ReorderA,
    LossA,
    ReorderM2,
    LossM2,
    Cut,
    Crash,
}
impl Part {
    pub const ALL: [Self; 7] = [
        Self::Base,
        Self::ReorderA,
        Self::LossA,
        Self::ReorderM2,
        Self::LossM2,
        Self::Cut,
        Self::Crash,
    ];
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variant {
    Correct,
    OpenGate,
    SendFirst,
    StaleProof,
    Address,
    Diagnostic,
    InheritToken,
    OtherIdentity,
    OtherState,
    Unauthenticated,
}
impl Variant {
    pub const BROKEN: [Self; 9] = [
        Self::OpenGate,
        Self::SendFirst,
        Self::StaleProof,
        Self::Address,
        Self::Diagnostic,
        Self::InheritToken,
        Self::OtherIdentity,
        Self::OtherState,
        Self::Unauthenticated,
    ];
    pub fn part(self) -> Part {
        if self == Self::OtherState {
            Part::Crash
        } else {
            Part::Base
        }
    }
    pub fn target(self) -> &'static str {
        match self {
            Self::OpenGate => GATE,
            Self::SendFirst | Self::StaleProof => BEFORE,
            Self::Address => ATTRIBUTION,
            Self::Diagnostic => HOLDERS,
            Self::InheritToken | Self::OtherIdentity | Self::OtherState => RUNNER,
            Self::Unauthenticated => CHANNEL,
            Self::Correct => panic!("correct has no refutation"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Process {
    I,
    X,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Class {
    Contents,
    Secret,
    Identity,
    Nothing,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Place {
    Run(Process),
    Gate,
    Provider,
    Record,
    Inspection,
    Observation,
    Diagnostic,
    Result,
    Meta,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Connection {
    id: u8,
    process: Process,
    generation: u8,
    cut: u8,
    crash: u8,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identity {
    token_subject: Process,
    certificate_subject: Process,
    environment: BTreeMap<&'static str, String>,
    files: BTreeMap<String, String>,
}
fn identity_i() -> Identity {
    Identity {
        token_subject: Process::I,
        certificate_subject: Process::I,
        environment: BTreeMap::from([
            ("CONDUIT_URL", "https://engine.example:443/conduit".into()),
            ("CONDUIT_IDENTITY_DIR", "identity/I".into()),
        ]),
        files: [
            "ca.crt",
            "jwks.json",
            "engine-id",
            "token",
            "tls.crt",
            "tls.key",
        ]
        .into_iter()
        .map(|name| (format!("identity/I/{name}"), format!("{name}-I")))
        .collect(),
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    Run {
        authenticated: bool,
        confidential: bool,
        identity: Identity,
    },
    Request(u8),
    Proof(Connection, Option<Process>),
    Call(Connection, u8, bool),
    Identify(Process, bool),
    Reset(u8),
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Data {
    // Saved engine data: next connection ID, completed request IDs and setup generation.
    next: u8,
    sent: u8,
    pending: u8,
    setup: u8,
    started: bool,
    // Volatile engine data: connection and its proof; P ends with the engine.
    connection: Option<Connection>,
    proof: Option<u8>,
    provider_running: bool,
    local_running: [bool; 2],
    recovery: bool,
    // M2's provider part, I and X; stop/restart never consume harness crash budget.
    remote_started: bool,
    identity: Option<Identity>,
    generation: u8, // 0 I running, 1 X owns u, 2 restarted I
    probe: bool,
    identified: bool,
    x_identified: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConnectLog {
    connection: Connection,
    certificate: Option<Process>,
    proved: bool,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SendLog {
    connection: Connection,
    proof: Option<u8>,
    classes: Vec<Class>,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Attribution {
    sender: Process,
    token: bool,
    same_address: bool,
    instance: Option<Process>,
    basis: &'static str,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RunLog {
    instance: u8,
    setup: u8,
    variables: BTreeMap<&'static str, String>,
    files: BTreeMap<String, String>,
    state: String,
    empty_initial: bool,
    mode: u16,
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Ledger {
    connections: Vec<ConnectLog>,
    sends: [Option<SendLog>; 2],
    // (requester, proof made by engine). Fixed slots preserve history without log ordering.
    reached: [Option<(Process, bool)>; 3],
    attribution: [Option<Attribution>; 2],
    appearances: Vec<(Class, Place)>,
    runs: Vec<RunLog>,
    channel: Option<(bool, bool)>,
    // The provider dispatch is retained even if its network message is lost.
    provider_dispatch: Option<(Vec<Class>, Identity)>,
    remote_identity: Option<Identity>,
    x_observed: Vec<Class>,
    x_received: Vec<Class>,
    refused: bool,
    takeover: bool,
    restarted: bool,
    recovered_setup: bool,
    provider_local: bool,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Local {
    Setup,
    Connect,
    Send,
    Probe,
    IdentifyI,
    StopTake,
    IdentifyX,
    Restart,
}
#[derive(Clone)]
pub struct Gates {
    pub variant: Variant,
    pub part: Part,
}

pub fn model(variant: Variant, part: Part) -> Harness<Gates> {
    let mut budgets = [0; 9];
    for f in match part {
        Part::ReorderA | Part::ReorderM2 => vec![Fault::Reorder],
        Part::LossA | Part::LossM2 => vec![Fault::Loss],
        Part::Cut => vec![Fault::Cut, Fault::Heal],
        Part::Crash => vec![Fault::Crash, Fault::Recovery],
        Part::Base => vec![],
    } {
        budgets[f as usize] = 1;
    }
    Harness::new(
        Config {
            name: "MC5a split gates and identity",
            nodes: vec!["engine/M1", "M2", "A"],
            budgets,
            disabled_reasons: std::array::from_fn(|i| {
                (budgets[i] == 0).then_some(
                    "split faults; duplication, permanent loss and time are outside MC5a",
                )
            }),
            messages_per_link: 8,
            unique_state_ceiling: 190_000,
            timeout: Duration::from_secs(10),
            bounds: "two A requests, three connections, one remote run/stop/restart, one X gate probe, two identity requests, two local setup generations, one of each split fault",
        },
        Gates { variant, part },
    )
}
fn current(s: &System<Gates>, c: Connection) -> bool {
    c.cut == s.used[Fault::Cut as usize]
        && c.crash == s.used[Fault::Crash as usize]
        && c.generation == s.nodes[M].data.generation
        && s.cut.is_empty()
}
fn expected_run(instance: u8, setup: u8) -> RunLog {
    let dir = format!("identity/L{instance}");
    let state = format!("state/L{instance}");
    let mut variables = BTreeMap::from([
        ("HOME", "engine-home".into()),
        ("PATH", "listed-path".into()),
        ("USER", "engine-user".into()),
        ("LISTED", format!("L{instance}")),
        ("PASSWORD", format!("secret-L{instance}")),
        ("CONDUIT_URL", "https://engine.example:443/conduit".into()),
        ("CONDUIT_IDENTITY_DIR", dir.clone()),
        ("CONDUIT_STATE_DIR", state.clone()),
    ]);
    // LOGNAME/SHELL absent, TERM starts with (), so neither is inherited.
    variables.insert("LOGNAME", "listed-logname".into());
    let files = [
        "ca.crt",
        "jwks.json",
        "engine-id",
        "token",
        "tls.crt",
        "tls.key",
    ]
    .into_iter()
    .map(|name| {
        (
            format!("{dir}/{name}"),
            match name {
                "token" | "tls.crt" | "tls.key" => format!("{name}-L{instance}"),
                _ => name.to_string(),
            },
        )
    })
    .collect();
    RunLog {
        instance,
        setup,
        variables,
        files,
        state,
        empty_initial: setup == 0,
        mode: 0o700,
    }
}
fn deliver_run(instance: u8, setup: u8) -> RunLog {
    let mut run = expected_run(instance, setup);
    // Apply the provider inputs after the filtered inheritance. Identity and state
    // are installed last. The user pass list is empty in this bounded model.
    let mut variables: BTreeMap<_, _> = ENGINE_ENV
        .into_iter()
        .filter(|(name, value)| {
            ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"].contains(name)
                && !value.starts_with("()")
        })
        .map(|(name, value)| (name, value.to_string()))
        .collect();
    variables.extend([
        ("PATH", "listed-path".into()),
        ("LOGNAME", "listed-logname".into()),
        ("LISTED", format!("L{instance}")),
        ("PASSWORD", format!("secret-L{instance}")),
        ("CONDUIT_URL", "https://engine.example:443/conduit".into()),
        ("CONDUIT_IDENTITY_DIR", format!("identity/L{instance}")),
        ("CONDUIT_STATE_DIR", format!("state/L{instance}")),
    ]);
    run.variables = variables;
    run
}
impl Gates {
    fn setup(&self, c: &Config, s: &mut System<Self>) {
        let setup = u8::from(s.nodes[E].data.started);
        for instance in 1..=2 {
            let mut run = deliver_run(instance, setup);
            if self.variant == Variant::InheritToken && instance == 1 {
                run.variables
                    .insert("GITHUB_TOKEN", "engine-exported-token".into());
            }
            if self.variant == Variant::OtherIdentity && instance == 2 {
                let other = expected_run(1, setup);
                run.files = other.files;
                run.variables
                    .insert("CONDUIT_IDENTITY_DIR", "identity/L1".into());
            }
            if self.variant == Variant::OtherState && instance == 2 && setup == 1 {
                run.state = "state/L1".into();
                run.variables.insert("CONDUIT_STATE_DIR", run.state.clone());
            }
            s.ledger.runs.push(run);
        }
        if setup == 0 {
            for place in [
                Place::Record,
                Place::Inspection,
                Place::Observation,
                Place::Diagnostic,
                Place::Result,
                Place::Meta,
            ] {
                s.ledger.appearances.push((Class::Nothing, place));
            }
        }
        s.ledger.provider_local = true;
        s.ledger.recovered_setup |= setup == 1;
        s.nodes[E].data.provider_running = true;
        s.nodes[E].data.local_running = [true; 2];
        s.nodes[E].data.started = true;
        s.nodes[E].data.recovery = false;
        s.nodes[E].data.setup = setup;
        if setup == 0 {
            let authenticated = self.variant != Variant::Unauthenticated;
            let identity = identity_i();
            s.ledger.provider_dispatch =
                Some((vec![Class::Secret, Class::Identity], identity.clone()));
            s.send(
                c,
                E,
                M,
                Message::Run {
                    authenticated,
                    confidential: authenticated,
                    identity,
                },
            );
            if self.variant == Variant::Diagnostic {
                s.ledger
                    .appearances
                    .push((Class::Identity, Place::Diagnostic));
            }
        }
    }
}
impl Protocol for Gates {
    type Node = Data;
    type Message = Message;
    type Ledger = Ledger;
    type Local = Local;
    fn fault_allowed(&self, fault: Fault, src: usize, dst: usize) -> bool {
        match fault {
            Fault::Loss | Fault::Reorder => match self.part {
                Part::LossA | Part::ReorderA => (src, dst) == (A, E),
                Part::LossM2 | Part::ReorderM2 => (src, dst) == (E, M) || (src, dst) == (M, E),
                _ => false,
            },
            Fault::Cut | Fault::Heal => (src, dst) == (E, M) || (src, dst) == (M, E),
            Fault::Crash | Fault::Recovery => src == E,
            _ => false,
        }
    }
    fn initial(&self, c: &Config) -> System<Self> {
        let mut s = State {
            nodes: (0..3)
                .map(|_| Node {
                    data: Data::default(),
                    status: Status::Running,
                })
                .collect(),
            links: vec![vec![]; 9],
            cut: Default::default(),
            used: [0; 9],
            ledger: Ledger::default(),
        };
        s.send(c, A, E, Message::Request(0));
        s.send(c, A, E, Message::Request(1));
        s
    }
    fn receive(&self, _c: &Config, s: &mut System<Self>, _src: usize, _dst: usize, msg: Message) {
        match msg {
            Message::Run {
                authenticated,
                confidential,
                identity,
            } => {
                s.nodes[M].data.remote_started = true;
                s.nodes[M].data.identity = Some(identity.clone());
                s.ledger.remote_identity = Some(identity);
                s.ledger.channel = Some((authenticated, confidential));
                for class in [Class::Secret, Class::Identity] {
                    for place in [Place::Provider, Place::Gate, Place::Run(Process::I)] {
                        s.ledger.appearances.push((class, place));
                    }
                    if !authenticated || !confidential {
                        s.ledger.x_observed.push(class);
                    }
                }
            }
            Message::Request(id) => {
                if s.nodes[E].data.sent & (1 << id) == 0 {
                    s.nodes[E].data.pending |= 1 << id;
                }
            }
            Message::Proof(conn, certificate) => {
                if current(s, conn)
                    && s.nodes[E].data.connection == Some(conn)
                    && certificate == Some(Process::I)
                {
                    s.nodes[E].data.proof = Some(conn.id);
                    let log = &mut s.ledger.connections[conn.id as usize];
                    log.certificate = certificate;
                    log.proved = true;
                }
            }
            Message::Call(conn, id, proof) => {
                if current(s, conn) {
                    if conn.process == Process::I && proof {
                        s.ledger.reached[id as usize] = Some((Process::I, proof));
                    } else if conn.process == Process::X {
                        s.ledger.x_received.extend([Class::Contents, Class::Secret]);
                        s.ledger
                            .appearances
                            .push((Class::Secret, Place::Run(Process::X)));
                    }
                }
            }
            Message::Identify(sender, token) => {
                let by_address = sender == Process::X && self.variant == Variant::Address;
                s.ledger.attribution[usize::from(sender == Process::X)] = Some(Attribution {
                    sender,
                    token,
                    same_address: true,
                    instance: if token || by_address {
                        Some(Process::I)
                    } else {
                        None
                    },
                    basis: if by_address {
                        "address"
                    } else if token {
                        "token"
                    } else {
                        "none"
                    },
                });
            }
            Message::Reset(generation) => {
                if s.nodes[E]
                    .data
                    .connection
                    .is_some_and(|x| x.generation < generation)
                {
                    s.nodes[E].data.connection = None;
                    if self.variant != Variant::StaleProof {
                        s.nodes[E].data.proof = None;
                    }
                }
            }
        }
    }
    fn timer_enabled(&self, _s: &System<Self>, _node: usize) -> bool {
        false
    }
    fn timer(&self, _c: &Config, _s: &mut System<Self>, _node: usize) {
        unreachable!()
    }
    fn crash(&self, n: &mut Data) {
        n.connection = None;
        n.proof = None;
        n.provider_running = false;
        n.local_running = [false; 2];
        n.recovery = true;
        // Pending requests and instance/state-directory bindings are saved.
    }
    fn recover(&self, _n: &mut Data) {}
    fn local_actions(&self, s: &System<Self>, node: usize, out: &mut Vec<Local>) {
        let e = &s.nodes[E].data;
        let m = &s.nodes[M].data;
        if node == E {
            if !e.started || e.recovery {
                out.push(Local::Setup);
                return;
            }
            if e.pending != 0
                && m.remote_started
                && s.cut.is_empty()
                && e.next < 3
                && e.connection.is_none_or(|conn| !current(s, conn))
            {
                out.push(Local::Connect);
            }
            if e.pending != 0
                && e.connection.is_some_and(|conn| {
                    current(s, conn)
                        && (e.proof == Some(conn.id)
                            || self.variant == Variant::SendFirst
                            || (self.variant == Variant::StaleProof && e.proof.is_some()))
                })
            {
                out.push(Local::Send);
            }
        }
        if node == M && m.remote_started {
            if !m.probe && m.generation != 1 {
                out.push(Local::Probe);
            }
            if !m.identified && m.generation != 1 {
                out.push(Local::IdentifyI);
            }
            if m.generation == 0 && s.ledger.reached[..2].iter().any(Option::is_some) {
                out.push(Local::StopTake);
            }
            if m.generation >= 1 && !m.x_identified {
                out.push(Local::IdentifyX);
            }
            if m.generation == 1
                && s.ledger
                    .connections
                    .iter()
                    .any(|l| l.connection.process == Process::X)
            {
                out.push(Local::Restart);
            }
        }
    }
    fn local_step(&self, c: &Config, s: &mut System<Self>, _node: usize, action: Local) {
        match action {
            Local::Setup => self.setup(c, s),
            Local::Connect => {
                let generation = s.nodes[M].data.generation;
                let process = if generation == 1 {
                    Process::X
                } else {
                    Process::I
                };
                let conn = Connection {
                    id: s.nodes[E].data.next,
                    process,
                    generation,
                    cut: s.used[Fault::Cut as usize],
                    crash: s.used[Fault::Crash as usize],
                };
                s.nodes[E].data.next += 1;
                s.nodes[E].data.connection = Some(conn);
                if self.variant != Variant::StaleProof {
                    s.nodes[E].data.proof = None;
                }
                s.ledger.connections.push(ConnectLog {
                    connection: conn,
                    certificate: None,
                    proved: false,
                });
                // Atomic transport connect carries no application data. Proof is a lossy wire message.
                s.ledger.takeover |= process == Process::X;
                s.send(
                    c,
                    M,
                    E,
                    Message::Proof(
                        conn,
                        (process == Process::I).then(|| {
                            s.nodes[M]
                                .data
                                .identity
                                .as_ref()
                                .unwrap()
                                .certificate_subject
                        }),
                    ),
                );
            }
            Local::Send => {
                let e = &mut s.nodes[E].data;
                let id = e.pending.trailing_zeros() as u8;
                let conn = e.connection.unwrap();
                let proof = e.proof;
                e.pending &= !(1 << id);
                e.sent |= 1 << id;
                s.ledger.sends[id as usize] = Some(SendLog {
                    connection: conn,
                    proof,
                    classes: vec![Class::Contents, Class::Secret],
                });
                s.send(c, E, M, Message::Call(conn, id, true));
            }
            Local::Probe => {
                s.nodes[M].data.probe = true;
                if self.variant == Variant::OpenGate {
                    s.ledger.reached[2] = Some((Process::X, false));
                } else {
                    s.ledger.refused = true;
                }
            }
            Local::IdentifyI => {
                s.nodes[M].data.identified = true;
                let token = s.nodes[M].data.identity.as_ref().unwrap().token_subject;
                s.send(c, M, E, Message::Identify(Process::I, token == Process::I));
            }
            Local::StopTake => {
                s.nodes[M].data.generation = 1;
                s.send(c, M, E, Message::Reset(1));
            }
            Local::IdentifyX => {
                s.nodes[M].data.x_identified = true;
                s.send(c, M, E, Message::Identify(Process::X, false));
            }
            Local::Restart => {
                s.nodes[M].data.generation = 2;
                s.ledger.restarted = true;
            }
        }
    }
    fn properties(&self) -> Vec<NamedProperty<Self>> {
        vec![
            NamedProperty {
                property: Property::<Harness<Self>>::always(GATE, |_, s| {
                    s.ledger
                        .reached
                        .iter()
                        .flatten()
                        .all(|(who, proof)| *who == Process::I && *proof)
                }),
                promises: &["PRV-025"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(BEFORE, |_, s| {
                    s.ledger.x_received.is_empty()
                        && s.ledger.sends.iter().flatten().all(|send| {
                            let log = &s.ledger.connections[send.connection.id as usize];
                            send.proof == Some(send.connection.id)
                                && log.proved
                                && log.certificate == Some(Process::I)
                                && send.connection.process == Process::I
                        })
                }),
                promises: &["PRV-028"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(ATTRIBUTION, |_, s| {
                    s.ledger
                        .attribution
                        .iter()
                        .flatten()
                        .all(|a| match a.sender {
                            Process::I => {
                                a.token && a.instance == Some(Process::I) && a.basis == "token"
                            }
                            Process::X => a.instance.is_none() && a.basis == "none",
                        })
                }),
                promises: &["PRV-018"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(HOLDERS, |_, s| {
                    s.ledger.appearances.iter().all(|(class, place)| {
                        *class != Class::Identity
                            || matches!(
                                place,
                                Place::Run(Process::I) | Place::Gate | Place::Provider
                            )
                    }) && !s.ledger.x_observed.contains(&Class::Identity)
                        && s.ledger
                            .remote_identity
                            .as_ref()
                            .is_none_or(|id| *id == identity_i())
                }),
                promises: &["PRV-022", "PRV-024"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(RUNNER, |_, s| {
                    s.ledger
                        .runs
                        .iter()
                        .all(|r| *r == expected_run(r.instance, r.setup))
                }),
                promises: &["PRV-003", "PRV-023", "PRV-029"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::always(CHANNEL, |_, s| {
                    s.ledger
                        .channel
                        .is_none_or(|(auth, confidential)| auth && confidential)
                        && s.ledger.x_observed.is_empty()
                }),
                promises: &["PRV-011", "PRV-022"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(SERVED, |_, s| {
                    s.ledger.reached[..2].contains(&Some((Process::I, true)))
                }),
                promises: &["PRV-025"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(REFUSED, |_, s| s.ledger.refused),
                promises: &["PRV-025"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(TAKEOVER, |_, s| s.ledger.takeover),
                promises: &["PRV-028"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(X_NONE, |_, s| {
                    s.ledger.attribution[1]
                        .as_ref()
                        .is_some_and(|a| a.instance.is_none())
                }),
                promises: &["PRV-018"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(I_TOKEN, |_, s| {
                    s.ledger.attribution[0]
                        .as_ref()
                        .is_some_and(|a| a.instance == Some(Process::I) && a.basis == "token")
                }),
                promises: &["PRV-018"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(RERUN, |_, s| {
                    s.ledger.recovered_setup
                }),
                promises: &["PRV-029"],
            },
            NamedProperty {
                property: Property::<Harness<Self>>::sometimes(CARRIED, |_, s| {
                    s.ledger.channel.is_some()
                }),
                promises: &["PRV-011", "PRV-022"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;
    fn expected(v: Variant, part: Part) -> (Vec<&'static str>, Vec<&'static str>) {
        let mut broken = match v {
            Variant::Correct => vec![],
            Variant::Unauthenticated => vec![CHANNEL, HOLDERS],
            _ => vec![v.target()],
        };
        let mut missing = vec![];
        if part != Part::Crash {
            missing.push(RERUN);
        }
        if v == Variant::OpenGate {
            missing.push(REFUSED);
        }
        if v == Variant::Address {
            missing.push(X_NONE);
        }
        broken.sort();
        missing.sort();
        (broken, missing)
    }
    #[test]
    fn full_runs_match_pre_registration() {
        for (v, part) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            let report = check_all(model(v, part));
            println!("MC5a {v:?}/{part:?}: {report}");
            let mut broken = report.broken;
            let mut missing = report.missing_witnesses;
            broken.sort();
            missing.sort();
            assert_eq!(
                (broken, missing),
                expected(v, part),
                "MC5a table mismatch {v:?}/{part:?}"
            );
        }
    }
    #[test]
    fn first_named_refutations() {
        for v in Variant::BROKEN {
            println!("MC5a refutation {v:?}: {}", v.target());
            let m = model(v, v.part());
            let trace = assert_refuted(m.clone(), v.target());
            println!("MC5a refutation {v:?}: {} actions", trace.len());
            // Safety is a function of history, even if every node's data is erased.
            let path =
                stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
            let mut s = path.last_state().clone();
            for node in &mut s.nodes {
                node.data = Data::default();
            }
            assert!(!(m.property(v.target()).condition)(&m, &s));
        }
    }
}
