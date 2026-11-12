//! MC7b reference models: saved records, passive observation and racing installs.
use crate::harness::*;
use stateright::Property;
use std::collections::BTreeSet;
use std::time::Duration;

const E: usize = 0;
const S: usize = 1;
const A: usize = 2;

fn config(name: &'static str, faults: &[Fault]) -> Config {
    let mut c = Config {
        name,
        nodes: vec![
            "engine/source-E",
            "source-S/runtime/archive-service",
            "actor/reader",
        ],
        budgets: [0; 9],
        disabled_reasons: [
            Some("reordering isolated in its own configuration"),
            Some("loss isolated in its own configuration"),
            Some("wire duplication outside MC7b"),
            Some("partition outside MC7b"),
            Some("no partition to heal"),
            Some("crash isolated in its own configuration"),
            Some("no crash to recover from"),
            Some("permanent machine loss outside MC7b"),
            Some("quiescent completion needs no retry or timeout in this part"),
        ],
        messages_per_link: 16,
        unique_state_ceiling: 190_000,
        timeout: Duration::from_secs(9),
        bounds: "three nodes; finite scripts and one-use local choices; at most 16 queued messages per link; see MC7b pre-registration",
    };
    for f in faults {
        c.budgets[*f as usize] = 1;
        c.disabled_reasons[*f as usize] = None;
    }
    c
}
fn initial<P: Protocol>(data: P::Node, ledger: P::Ledger) -> System<P> {
    State {
        nodes: vec![
            Node {
                data,
                status: Status::Running
            };
            3
        ],
        links: vec![vec![]; 9],
        cut: BTreeSet::new(),
        used: [0; 9],
        ledger,
    }
}
fn drained<P: Protocol>(s: &System<P>) -> bool {
    s.links.iter().all(Vec::is_empty)
}
macro_rules! safety {
    ($name:expr, $ids:expr, $condition:expr) => {
        NamedProperty {
            property: Property::<Harness<Self>>::always($name, $condition),
            promises: $ids,
        }
    };
}
macro_rules! witness {
    ($name:expr, $ids:expr, $condition:expr) => {
        NamedProperty {
            property: Property::<Harness<Self>>::sometimes($name, $condition),
            promises: $ids,
        }
    };
}

pub mod record {
    use super::*;
    pub const SAFETY: [&str; 5] = ["MC7b R1", "MC7b R3", "MC7b R4", "MC7b R5", "MC7b R6"];
    pub const REACH: [&str; 7] = [
        "R events",
        "R existing",
        "R purges",
        "R repeat",
        "R reads",
        "R missing",
        "R unknown",
    ];
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Part {
        Base,
        Unknown,
        RequestLoss,
        RequestReorder,
        ReadLoss,
        ReadReorder,
        Crash,
        Unreadable,
    }
    impl Part {
        pub const ALL: [Self; 8] = [
            Self::Base,
            Self::Unknown,
            Self::RequestLoss,
            Self::RequestReorder,
            Self::ReadLoss,
            Self::ReadReorder,
            Self::Crash,
            Self::Unreadable,
        ];
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Variant {
        Correct,
        OmitRecord,
        OmitRefusalRecord,
        OmitPurgeRecord,
        OmitRepeatRecord,
        AlterRecord,
        PurgeIndex,
        RootBypass,
        PurgeInclusive,
        ForgetIndex,
        ReverseRead,
        HideGap,
    }
    impl Variant {
        pub const BROKEN: [Self; 11] = [
            Self::OmitRecord,
            Self::OmitRefusalRecord,
            Self::OmitPurgeRecord,
            Self::OmitRepeatRecord,
            Self::AlterRecord,
            Self::PurgeIndex,
            Self::RootBypass,
            Self::PurgeInclusive,
            Self::ForgetIndex,
            Self::ReverseRead,
            Self::HideGap,
        ];
        pub fn part(self) -> Part {
            match self {
                Self::ForgetIndex => Part::Crash,
                Self::HideGap => Part::Unreadable,
                _ => Part::Base,
            }
        }
        pub fn target(self) -> &'static str {
            SAFETY[match self {
                Self::OmitRecord | Self::OmitRefusalRecord | Self::OmitRepeatRecord => 0,
                Self::AlterRecord => 1,
                Self::OmitPurgeRecord => 2,
                Self::ReverseRead => 3,
                Self::HideGap => 4,
                _ => 2,
            }]
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Record {
        id: u8,
        time: u8,
        kind: u8,
        purge: Option<(u8, u32)>, // cutoff and removed record IDs
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Data {
        records: Vec<Record>, // saved at the source
        next: u8,
        emitted: bool,
        key: Option<bool>, // saved: true succeeded, false unknown
        effects: u8,
        stage: u8,
        expected: [Vec<u8>; 2],
        received: [Vec<Record>; 2],
        unread_used: bool,
        recovered: bool,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Message {
        Invoke(bool),
        Purge {
            root: bool,
            before: u8,
        },
        SourcePurge {
            root: bool,
            before: u8,
        },
        Read(usize),
        Frame {
            source: usize,
            expected: Vec<u8>,
            records: Vec<Record>,
        },
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Change {
        source: usize,
        before: Vec<Record>,
        after: Vec<Record>,
        purge: bool,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Purge {
        source: usize,
        root: bool,
        before_time: u8,
        before: Vec<Record>,
        after: Vec<Record>,
        key_before: Option<bool>,
        key_after: Option<bool>,
        refused: bool,
        evidence: Option<Record>,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Repeat {
        original: bool,
        answer: bool,
        before_effects: u8,
        after_effects: u8,
        purged: bool,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Read {
        source: usize,
        expected: Vec<u8>,
        records: Vec<Record>,
        gaps: Vec<u8>,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Ledger {
        events: Vec<(usize, Record)>, // required occurrences, independent of record writes
        changes: Vec<Change>,
        purges: Vec<Purge>,
        repeats: Vec<Repeat>,
        reads: Vec<Read>,
        existing: bool,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Local {
        Events,
        Next,
        Read(bool),
        Finish,
    }
    #[derive(Clone)]
    pub struct Records {
        pub part: Part,
        pub variant: Variant,
    }
    pub fn model(variant: Variant, part: Part) -> Harness<Records> {
        let faults: &[Fault] = match part {
            Part::RequestLoss | Part::ReadLoss => &[Fault::Loss],
            Part::RequestReorder | Part::ReadReorder => &[Fault::Reorder],
            Part::Crash => &[Fault::Crash, Fault::Recovery],
            _ => &[],
        };
        Harness::new(config("MC7b records", faults), Records { part, variant })
    }
    impl Records {
        fn event(
            &self,
            s: &mut System<Self>,
            source: usize,
            kind: u8,
            purge: Option<(u8, u32)>,
        ) -> Record {
            let d = &mut s.nodes[source].data;
            let r = Record {
                id: d.next,
                time: d.next + 1,
                kind,
                purge,
            };
            d.next += 1;
            // IDs belong to occurrences, even when the record write is omitted. Purge
            // evidence (kind 14) is required by REC-004 and checked separately by R4.
            if kind < 14 {
                s.ledger.events.push((source, r));
            }
            r
        }
        fn keep(&self, s: &mut System<Self>, source: usize, record: Record) -> Record {
            let before = s.nodes[source].data.records.clone();
            s.nodes[source].data.records.push(record);
            s.ledger.changes.push(Change {
                source,
                before,
                after: s.nodes[source].data.records.clone(),
                purge: false,
            });
            record
        }
        fn purge(&self, s: &mut System<Self>, source: usize, root: bool, before_time: u8) {
            let before = s.nodes[source].data.records.clone();
            let key_before = s.nodes[E].data.key;
            let allowed = root || self.variant == Variant::RootBypass;
            if allowed {
                let inclusive = self.variant == Variant::PurgeInclusive;
                s.nodes[source].data.records.retain(|r| {
                    r.purge.is_some()
                        || if inclusive {
                            r.time > before_time
                        } else {
                            r.time >= before_time
                        }
                });
                if self.variant == Variant::PurgeIndex {
                    s.nodes[E].data.key = None;
                }
            }
            let after = s.nodes[source].data.records.clone();
            let removed = before
                .iter()
                .filter(|r| !after.contains(r))
                .fold(0, |mask, r| mask | (1 << r.id));
            s.ledger.changes.push(Change {
                source,
                before: before.clone(),
                after: after.clone(),
                purge: allowed,
            });
            let evidence = if allowed {
                let event = self.event(s, source, 14, Some((before_time, removed)));
                if self.variant != Variant::OmitPurgeRecord {
                    self.keep(s, source, event);
                }
                Some(event)
            } else {
                let event = self.event(s, source, 1, None); // request ends without dispatch
                if self.variant != Variant::OmitRefusalRecord {
                    self.keep(s, source, event);
                }
                None
            };
            s.ledger.purges.push(Purge {
                source,
                root,
                before_time,
                before,
                after,
                key_before,
                key_after: s.nodes[E].data.key,
                refused: !allowed,
                evidence,
            });
        }
        fn frames(&self, c: &Config, s: &mut System<Self>, source: usize, unread: bool) {
            let records = s.nodes[source].data.records.clone();
            s.ledger.changes.push(Change {
                source,
                before: records.clone(),
                after: records.clone(),
                purge: false,
            });
            let expected: Vec<_> = records.iter().map(|r| r.id).collect();
            let mut frames = [vec![], vec![]];
            for (i, r) in records.into_iter().enumerate() {
                if !(unread && i == 0) {
                    frames[i % 2].push(r);
                }
            }
            for records in frames {
                s.send(
                    c,
                    source,
                    A,
                    Message::Frame {
                        source,
                        expected: expected.clone(),
                        records,
                    },
                );
            }
        }
    }
    impl Protocol for Records {
        type Node = Data;
        type Message = Message;
        type Ledger = Ledger;
        type Local = Local;
        fn gate_remaining_faults(&self) -> bool {
            true
        }
        fn fault_allowed(&self, f: Fault, src: usize, dst: usize) -> bool {
            match f {
                Fault::Crash | Fault::Recovery => src == E,
                Fault::Loss | Fault::Reorder => match self.part {
                    Part::RequestLoss | Part::RequestReorder => src == A && dst == E,
                    Part::ReadLoss | Part::ReadReorder => src == S && dst == A,
                    _ => false,
                },
                _ => false,
            }
        }
        fn fault_enabled(&self, s: &System<Self>, f: Fault) -> bool {
            match f {
                Fault::Crash => s.nodes[A].data.stage == 2,
                Fault::Recovery => true,
                Fault::Loss | Fault::Reorder => match self.part {
                    Part::RequestLoss | Part::RequestReorder => s.nodes[A].data.stage == 3,
                    _ => s.nodes[A].data.stage == 6,
                },
                _ => false,
            }
        }
        fn initial(&self, _: &Config) -> System<Self> {
            initial::<Self>(Data::default(), Ledger::default())
        }
        fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, dst: usize, msg: Message) {
            match msg {
                Message::Invoke(repeat) => {
                    let original = self.part != Part::Unknown;
                    let d = &mut s.nodes[E].data;
                    let before_effects = d.effects;
                    let cached = d.key.is_some();
                    let answer = if let Some(answer) = d.key {
                        answer
                    } else {
                        d.effects += 1;
                        d.key = Some(original);
                        original
                    };
                    let after_effects = d.effects;
                    if repeat {
                        let purged = s.ledger.purges.iter().any(|p| p.root);
                        s.ledger.repeats.push(Repeat {
                            original,
                            answer,
                            before_effects,
                            after_effects,
                            purged,
                        });
                    }
                    if cached {
                        let ended = self.event(s, E, 1, None); // request ends without dispatch
                        if self.variant != Variant::OmitRepeatRecord {
                            self.keep(s, E, ended);
                        }
                    } else {
                        let dispatched = self.event(s, E, 2, None);
                        self.keep(s, E, dispatched);
                        let ended = self.event(s, E, 3, None);
                        self.keep(s, E, ended);
                    }
                }
                Message::Purge { root, before } => {
                    self.purge(s, E, root, before);
                    if root || self.variant == Variant::RootBypass {
                        s.send(c, E, S, Message::SourcePurge { root, before });
                    }
                }
                Message::SourcePurge { root, before } => self.purge(s, S, root, before),
                Message::Read(source) => {
                    if source == E {
                        self.frames(c, s, E, false);
                    } else {
                        s.nodes[S].data.stage = 1;
                    }
                }
                Message::Frame {
                    source,
                    expected,
                    records,
                } => {
                    s.nodes[dst].data.expected[source] = expected;
                    s.nodes[dst].data.received[source].extend(records);
                }
            }
        }
        fn timer_enabled(&self, _: &System<Self>, _: usize) -> bool {
            false
        }
        fn timer(&self, _: &Config, _: &mut System<Self>, _: usize) {
            unreachable!()
        }
        fn crash(&self, d: &mut Data) {
            if self.variant == Variant::ForgetIndex {
                d.key = None;
            }
        }
        fn recover(&self, d: &mut Data) {
            d.recovered = true;
        }
        fn local_actions(&self, s: &System<Self>, n: usize, out: &mut Vec<Local>) {
            if n <= S && !s.nodes[n].data.emitted {
                out.push(Local::Events);
            }
            if n == S && s.nodes[S].data.stage == 1 {
                out.push(Local::Read(false));
                if self.part == Part::Unreadable && !s.nodes[S].data.unread_used {
                    out.push(Local::Read(true));
                }
            }
            if n == A && s.nodes[..2].iter().all(|d| d.data.emitted) && drained::<Self>(s) {
                if s.nodes[A].data.stage < 6 {
                    out.push(Local::Next);
                } else if s.nodes[A].data.stage == 6 && s.nodes[S].data.stage == 2 {
                    out.push(Local::Finish);
                }
            }
        }
        fn local_step(&self, c: &Config, s: &mut System<Self>, n: usize, action: Local) {
            match action {
                Local::Events => {
                    for kind in (n as u8 * 7)..(n as u8 * 7 + 7) {
                        let event = self.event(s, n, kind, None);
                        if !(self.variant == Variant::OmitRecord && kind == 9) {
                            self.keep(s, n, event);
                        }
                    }
                    s.nodes[n].data.emitted = true;
                }
                Local::Read(unread) => {
                    self.frames(c, s, S, unread);
                    s.nodes[S].data.stage = 2;
                    s.nodes[S].data.unread_used = unread;
                }
                Local::Next => {
                    match s.nodes[A].data.stage {
                        0 => s.send(c, A, E, Message::Invoke(false)),
                        1 => {
                            let before = s.nodes[E].data.records.clone();
                            s.ledger.existing = !before.is_empty();
                            if self.variant == Variant::AlterRecord {
                                s.nodes[E].data.records[0].kind = 99;
                            }
                            s.ledger.changes.push(Change {
                                source: E,
                                before,
                                after: s.nodes[E].data.records.clone(),
                                purge: false,
                            });
                        }
                        2 => {
                            s.send(
                                c,
                                A,
                                E,
                                Message::Purge {
                                    root: false,
                                    before: 3,
                                },
                            );
                            s.send(
                                c,
                                A,
                                E,
                                Message::Purge {
                                    root: true,
                                    before: 3,
                                },
                            );
                        }
                        3 => s.send(
                            c,
                            A,
                            E,
                            Message::Purge {
                                root: true,
                                before: 30,
                            },
                        ),
                        4 => s.send(c, A, E, Message::Invoke(true)),
                        5 => {
                            s.send(c, A, E, Message::Read(E));
                            s.send(c, A, E, Message::Read(S));
                        }
                        _ => unreachable!(),
                    }
                    s.nodes[A].data.stage += 1;
                }
                Local::Finish => {
                    for source in [E, S] {
                        let d = &mut s.nodes[A].data;
                        let mut records = d.received[source].clone();
                        records.sort_by_key(|r| r.id);
                        if self.variant == Variant::ReverseRead && source == S {
                            records.reverse();
                        }
                        let gaps = if self.variant == Variant::HideGap {
                            vec![]
                        } else {
                            d.expected[source]
                                .iter()
                                .copied()
                                .filter(|id| !records.iter().any(|r| r.id == *id))
                                .collect()
                        };
                        s.ledger.reads.push(Read {
                            source,
                            expected: d.expected[source].clone(),
                            records,
                            gaps,
                        });
                    }
                    s.nodes[A].data.stage = 7;
                }
            }
        }
        fn properties(&self) -> Vec<NamedProperty<Self>> {
            vec![
                safety!(SAFETY[0], &["REC-001"], |_, s| s.ledger.events.iter().all(
                    |(source, event)| s
                        .ledger
                        .changes
                        .iter()
                        .any(|c| { c.source == *source && c.after.contains(event) })
                )),
                safety!(SAFETY[1], &["REC-003"], |_, s| {
                    [E, S].into_iter().all(|source| {
                        let mut previous: Option<&[Record]> = None;
                        s.ledger
                            .changes
                            .iter()
                            .filter(|c| c.source == source)
                            .all(|c| {
                                let continuous = previous.is_none_or(|records| records == c.before);
                                previous = Some(&c.after);
                                continuous
                                    && (c.purge || c.before.iter().all(|r| c.after.contains(r)))
                            })
                    })
                }),
                safety!(SAFETY[2], &["REC-004"], |_, s| {
                    s.ledger.purges.iter().all(|p| {
                        let expected: Vec<_> = p
                            .before
                            .iter()
                            .copied()
                            .filter(|r| r.purge.is_some() || !p.root || r.time >= p.before_time)
                            .collect();
                        let removed = p
                            .before
                            .iter()
                            .filter(|r| !p.after.contains(r))
                            .fold(0, |mask, r| mask | (1 << r.id));
                        let evidence = if p.root {
                            p.evidence.is_some_and(|r| {
                                r.purge == Some((p.before_time, removed))
                                    && s.ledger
                                        .changes
                                        .iter()
                                        .any(|c| c.source == p.source && c.after.contains(&r))
                            })
                        } else {
                            p.evidence.is_none()
                        };
                        p.after == expected
                            && p.refused != p.root
                            && p.key_before == p.key_after
                            && evidence
                    }) && s.ledger.repeats.iter().all(|r| {
                        !r.purged || (r.original == r.answer && r.before_effects == r.after_effects)
                    }) && s.ledger.changes.iter().all(|c| {
                        c.before
                            .iter()
                            .filter(|r| r.purge.is_some())
                            .all(|r| c.after.contains(r))
                    })
                }),
                safety!(SAFETY[3], &["REC-005"], |_, s| s
                    .ledger
                    .reads
                    .iter()
                    .all(|r| r.records.windows(2).all(|w| w[0].id < w[1].id))),
                safety!(SAFETY[4], &["REC-006"], |_, s| s.ledger.reads.iter().all(
                    |r| {
                        let missing: Vec<_> = r
                            .expected
                            .iter()
                            .copied()
                            .filter(|id| !r.records.iter().any(|v| v.id == *id))
                            .collect();
                        missing == r.gaps
                    }
                )),
                witness!(REACH[0], &["REC-001"], |_, s| (0..14).all(|kind| s
                    .ledger
                    .events
                    .iter()
                    .any(|(_, event)| event.kind == kind))),
                witness!(REACH[1], &["REC-003"], |_, s| s.ledger.existing),
                witness!(REACH[2], &["REC-004"], |_, s| s
                    .ledger
                    .purges
                    .iter()
                    .any(|p| p.root)
                    && s.ledger.purges.iter().any(|p| !p.root)),
                witness!(REACH[3], &["REC-004"], |_, s| s
                    .ledger
                    .repeats
                    .iter()
                    .any(|r| r.purged)),
                witness!(REACH[4], &["REC-005"], |_, s| s.ledger.reads.len() == 2),
                witness!(REACH[5], &["REC-006"], |_, s| s.ledger.reads.iter().any(
                    |r| r
                        .expected
                        .iter()
                        .any(|id| !r.records.iter().any(|v| v.id == *id))
                )),
                witness!(REACH[6], &["REC-004"], |_, s| s
                    .ledger
                    .repeats
                    .iter()
                    .any(|r| !r.original && r.purged)),
            ]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use stateright::Model;

        #[test]
        fn purge_evidence_requires_a_saved_record_at_its_source() {
            for source in [E, S] {
                for variant in [Variant::Correct, Variant::OmitPurgeRecord] {
                    let m = model(variant, Part::Base);
                    let mut s = m.init_states().remove(0);
                    m.protocol.purge(&mut s, source, true, 3);
                    assert!(s.ledger.purges[0].evidence.is_some());
                    // A valid evidence value alone is insufficient, even with empty storage.
                    for node in &mut s.nodes {
                        node.data = Data::default();
                    }
                    let r4 = m.property(SAFETY[2]);
                    assert_eq!((r4.condition)(&m, &s), variant == Variant::Correct);
                    if variant == Variant::Correct {
                        // A snapshot from the other source cannot prove that it was saved here.
                        for change in &mut s.ledger.changes {
                            change.source = 1 - source;
                        }
                        assert!(!(r4.condition)(&m, &s));
                    }
                }
            }
        }

        #[test]
        fn invocation_records_follow_the_saved_key_answer() {
            for part in [Part::Base, Part::Unknown] {
                for repeat in [false, true] {
                    for variant in [
                        Variant::Correct,
                        Variant::OmitRepeatRecord,
                        Variant::ForgetIndex,
                        Variant::PurgeIndex,
                    ] {
                        let m = model(variant, part);
                        let mut s = m.init_states().remove(0);
                        m.protocol
                            .receive(&m.config, &mut s, A, E, Message::Invoke(false));
                        if variant == Variant::ForgetIndex {
                            m.protocol.crash(&mut s.nodes[E].data);
                        } else if variant == Variant::PurgeIndex {
                            m.protocol.purge(&mut s, E, true, 30);
                        }
                        let cached = s.nodes[E].data.key.is_some();
                        let effects = s.nodes[E].data.effects;
                        let start = s.ledger.events.len();
                        m.protocol
                            .receive(&m.config, &mut s, A, E, Message::Invoke(repeat));
                        let kinds: Vec<_> = s.ledger.events[start..]
                            .iter()
                            .map(|(_, event)| event.kind)
                            .collect();
                        assert_eq!(kinds, if cached { vec![1] } else { vec![2, 3] });
                        assert_eq!(s.nodes[E].data.effects, effects + u8::from(!cached));
                        assert_eq!(s.nodes[E].data.key, Some(part != Part::Unknown));
                        for (_, event) in &s.ledger.events[start..] {
                            assert_eq!(
                                s.nodes[E].data.records.contains(event),
                                variant != Variant::OmitRepeatRecord
                            );
                        }
                        for node in &mut s.nodes {
                            node.data = Data::default();
                        }
                        assert_eq!(
                            (m.property(SAFETY[0]).condition)(&m, &s),
                            variant != Variant::OmitRepeatRecord
                        );
                    }
                }
            }
        }

        #[test]
        fn refused_purge_requires_its_own_record() {
            for seeded in [false, true] {
                for variant in [Variant::Correct, Variant::OmitRefusalRecord] {
                    let m = model(variant, Part::Base);
                    let mut s = m.init_states().remove(0);
                    let r1 = m.property(SAFETY[0]);
                    if seeded {
                        for source in [E, S] {
                            m.protocol
                                .local_step(&m.config, &mut s, source, Local::Events);
                        }
                    }
                    assert!((r1.condition)(&m, &s));
                    let occurrences = s.ledger.events.len();
                    m.protocol.receive(
                        &m.config,
                        &mut s,
                        A,
                        E,
                        Message::Purge {
                            root: false,
                            before: 3,
                        },
                    );
                    assert_eq!(s.ledger.events.len(), occurrences + 1);
                    let (source, event) = *s.ledger.events.last().unwrap();
                    assert_eq!((source, event.kind), (E, 1));
                    assert!(s.ledger.purges.last().unwrap().refused);
                    assert_eq!(
                        s.nodes[source].data.records.contains(&event),
                        variant == Variant::Correct
                    );
                    // The fourteen category witnesses cannot substitute for this occurrence.
                    for node in &mut s.nodes {
                        node.data = Data::default();
                    }
                    assert_eq!((r1.condition)(&m, &s), variant == Variant::Correct);
                }
            }
        }
    }
}

pub mod observe {
    use super::*;
    pub const SAFETY: [&str; 4] = ["MC7b O1", "MC7b O3", "MC7b O4", "MC7b O6"];
    pub const REACH: [&str; 5] = [
        "O observed",
        "O inspected",
        "O drift",
        "O unreachable",
        "O returned",
    ];
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Part {
        Base,
        Crash,
    }
    impl Part {
        pub const ALL: [Self; 2] = [Self::Base, Self::Crash];
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Variant {
        Correct,
        AdoptObserved,
        StaleCurrent,
        NoLastTime,
        ForgetTime,
        AutoRepair,
        InspectEffect,
        InspectAccess,
    }
    impl Variant {
        pub const BROKEN: [Self; 7] = [
            Self::AdoptObserved,
            Self::StaleCurrent,
            Self::NoLastTime,
            Self::ForgetTime,
            Self::AutoRepair,
            Self::InspectEffect,
            Self::InspectAccess,
        ];
        pub fn part(self) -> Part {
            if self == Self::ForgetTime {
                Part::Crash
            } else {
                Part::Base
            }
        }
        pub fn target(self) -> &'static str {
            SAFETY[match self {
                Self::AdoptObserved => 0,
                Self::StaleCurrent | Self::NoLastTime | Self::ForgetTime => 1,
                Self::AutoRepair => 2,
                _ => 3,
            }]
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Declared {
        definition: u8,
        config: u8,
        revision: u8,
    }
    const DECLARED: Declared = Declared {
        definition: 7,
        config: 1,
        revision: 4,
    };
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Condition {
        value: Option<bool>,
        last: Option<u8>,
    }
    const NEVER: Condition = Condition {
        value: None,
        last: None,
    };
    const LOCAL: Condition = Condition {
        value: Some(true),
        last: Some(1),
    };
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Data {
        stage: u8,
        declared: Declared,
        conditions: [Condition; 3],
        effects: u8,
        access: u8,
        lifecycle: [bool; 3],
    }
    impl Default for Data {
        fn default() -> Self {
            Self {
                stage: 0,
                declared: DECLARED,
                conditions: [NEVER, NEVER, LOCAL],
                effects: 0,
                access: 1,
                lifecycle: [true; 3],
            }
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Message {
        Drift,
        Unreachable(bool),
        Returned(bool),
        Inspect,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Local {
        Initial,
        Drift,
        Unreachable(bool),
        Returned(bool),
        Inspect,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Emission {
        input: Option<Message>,
        before: Declared,
        after: Declared,
        conditions: [Condition; 3],
        effects: (u8, u8),
        access: (u8, u8),
        lifecycle: ([bool; 3], [bool; 3]),
        dispatches: Vec<&'static str>,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Ledger {
        emissions: Vec<Emission>,
    }
    #[derive(Clone)]
    pub struct Observation {
        pub variant: Variant,
        pub part: Part,
    }
    pub fn model(variant: Variant, part: Part) -> Harness<Observation> {
        let faults: &[Fault] = if part == Part::Crash {
            &[Fault::Crash, Fault::Recovery]
        } else {
            &[]
        };
        Harness::new(
            config("MC7b observation", faults),
            Observation { variant, part },
        )
    }
    impl Observation {
        fn emit(&self, s: &mut System<Self>, input: Option<Message>) {
            let d = &mut s.nodes[E].data;
            let before = d.clone();
            let mut dispatches = vec![];
            match input {
                None => {
                    d.conditions[0] = Condition {
                        value: Some(true),
                        last: Some(1),
                    }
                }
                Some(Message::Drift) => {
                    d.conditions[0] = Condition {
                        value: Some(false),
                        last: Some(2),
                    };
                    if self.variant == Variant::AdoptObserved {
                        d.declared.config = 0;
                    }
                    if self.variant == Variant::AutoRepair {
                        dispatches.extend(["setup", "control", "open", "retirement"]);
                    }
                }
                Some(Message::Unreachable(true)) => {
                    if self.variant != Variant::StaleCurrent {
                        for condition in &mut d.conditions[..2] {
                            condition.value = None;
                        }
                    }
                    if self.variant == Variant::NoLastTime {
                        d.conditions[0].last = None;
                    }
                }
                Some(Message::Returned(_)) => {
                    d.conditions[0] = Condition {
                        value: Some(true),
                        last: Some(3),
                    }
                }
                Some(Message::Inspect) => {
                    if self.variant == Variant::InspectEffect {
                        d.effects += 1;
                    }
                    if self.variant == Variant::InspectAccess {
                        d.access |= 2;
                    }
                }
                Some(Message::Unreachable(false)) => {}
            }
            s.ledger.emissions.push(Emission {
                input,
                before: before.declared,
                after: d.declared,
                conditions: d.conditions,
                effects: (before.effects, d.effects),
                access: (before.access, d.access),
                lifecycle: (before.lifecycle, d.lifecycle),
                dispatches,
            });
        }
    }
    impl Protocol for Observation {
        type Node = Data;
        type Message = Message;
        type Ledger = Ledger;
        type Local = Local;
        fn gate_remaining_faults(&self) -> bool {
            true
        }
        fn fault_allowed(&self, f: Fault, src: usize, _: usize) -> bool {
            matches!(f, Fault::Crash | Fault::Recovery) && src == E
        }
        fn fault_enabled(&self, s: &System<Self>, f: Fault) -> bool {
            f == Fault::Recovery || (f == Fault::Crash && s.nodes[E].data.stage == 2)
        }
        fn initial(&self, _: &Config) -> System<Self> {
            initial::<Self>(Data::default(), Ledger::default())
        }
        fn receive(&self, _: &Config, s: &mut System<Self>, _: usize, _: usize, message: Message) {
            self.emit(s, Some(message));
            s.nodes[E].data.stage += 1;
        }
        fn timer_enabled(&self, _: &System<Self>, _: usize) -> bool {
            false
        }
        fn timer(&self, _: &Config, _: &mut System<Self>, _: usize) {
            unreachable!()
        }
        fn crash(&self, d: &mut Data) {
            if self.variant == Variant::ForgetTime {
                d.conditions[0].last = None;
            }
        }
        fn recover(&self, _: &mut Data) {}
        fn local_actions(&self, s: &System<Self>, n: usize, out: &mut Vec<Local>) {
            let stage = s.nodes[E].data.stage;
            if n == E && stage == 0 {
                out.push(Local::Initial);
            }
            if n == S {
                match (stage, s.nodes[S].data.stage) {
                    (1, 0) => out.push(Local::Drift),
                    (3, 1) => out.extend([Local::Unreachable(false), Local::Unreachable(true)]),
                    (4, 2) => out.extend([Local::Returned(false), Local::Returned(true)]),
                    _ => {}
                }
            }
            if n == A
                && ((stage == 2 && s.nodes[A].data.stage == 0)
                    || (stage == 5 && s.nodes[A].data.stage == 1))
            {
                out.push(Local::Inspect);
            }
        }
        fn local_step(&self, c: &Config, s: &mut System<Self>, n: usize, action: Local) {
            match action {
                Local::Initial => {
                    self.emit(s, None);
                    s.nodes[E].data.stage = 1;
                }
                Local::Drift => {
                    s.send(c, n, E, Message::Drift);
                    s.nodes[n].data.stage += 1;
                }
                Local::Unreachable(value) => {
                    s.send(c, n, E, Message::Unreachable(value));
                    s.nodes[n].data.stage += 1;
                }
                Local::Returned(restarted) => {
                    s.send(c, n, E, Message::Returned(restarted));
                    s.nodes[n].data.stage += 1;
                }
                Local::Inspect => {
                    s.send(c, n, E, Message::Inspect);
                    s.nodes[n].data.stage += 1;
                }
            }
        }
        fn properties(&self) -> Vec<NamedProperty<Self>> {
            vec![
                safety!(SAFETY[0], &["OBS-001"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .all(|e| e.before == e.after)),
                safety!(SAFETY[1], &["OBS-003"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .all(|e| e.input != Some(Message::Unreachable(true))
                        || e.conditions
                            == [
                                Condition {
                                    value: None,
                                    last: Some(2)
                                },
                                NEVER,
                                LOCAL
                            ])),
                safety!(SAFETY[2], &["OBS-004"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .all(|e| e.dispatches.is_empty())),
                safety!(SAFETY[3], &["OBS-006"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .all(|e| e.effects.0 == e.effects.1
                        && e.access.0 == e.access.1
                        && e.lifecycle.0 == e.lifecycle.1)),
                witness!(REACH[0], &["OBS-001"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .any(|e| e.input.is_none())),
                witness!(REACH[1], &["OBS-006"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .any(|e| e.input == Some(Message::Inspect))),
                witness!(REACH[2], &["OBS-004"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .any(|e| e.input == Some(Message::Drift))),
                witness!(REACH[3], &["OBS-003"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .any(|e| e.input == Some(Message::Unreachable(true)))),
                witness!(REACH[4], &["OBS-004"], |_, s| s
                    .ledger
                    .emissions
                    .iter()
                    .any(|e| matches!(e.input, Some(Message::Returned(_))))),
            ]
        }
    }
}

pub mod install {
    use super::*;
    pub const SAFETY: [&str; 5] = ["MC7b I24", "MC7b I25", "MC7b I27", "MC7b I13", "MC7b I17"];
    pub const REACH: [&str; 12] = [
        "I race",
        "I conflict",
        "I same",
        "I denied-same",
        "I invalid-same",
        "I claimants",
        "I fetch",
        "I used",
        "I different-firsts",
        "I done",
        "I failed",
        "I bad",
    ];
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Part {
        Provider,
        Https,
        File,
        Same,
        FailProvider,
        FailHttps,
        FailFile,
        BadProvider,
        BadHttps,
        BadFile,
        RequestLoss,
        RequestReorder,
        FetchLoss,
        FetchReorder,
        Crash,
    }
    impl Part {
        pub const ALL: [Self; 15] = [
            Self::Provider,
            Self::Https,
            Self::File,
            Self::Same,
            Self::FailProvider,
            Self::FailHttps,
            Self::FailFile,
            Self::BadProvider,
            Self::BadHttps,
            Self::BadFile,
            Self::RequestLoss,
            Self::RequestReorder,
            Self::FetchLoss,
            Self::FetchReorder,
            Self::Crash,
        ];
        fn failed(self) -> bool {
            matches!(self, Self::FailProvider | Self::FailHttps | Self::FailFile)
        }
        fn bad(self) -> bool {
            matches!(self, Self::BadProvider | Self::BadHttps | Self::BadFile)
        }
        fn scheme(self) -> Scheme {
            if matches!(self, Self::File | Self::FailFile | Self::BadFile) {
                Scheme::File
            } else {
                Scheme::Https
            }
        }
        fn claim(self) -> Scheme {
            if matches!(
                self,
                Self::Https
                    | Self::File
                    | Self::FailHttps
                    | Self::FailFile
                    | Self::BadHttps
                    | Self::BadFile
            ) {
                Scheme::Custom
            } else {
                Scheme::Https
            }
        }
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Variant {
        Correct,
        FailedAsSuccess,
        FetchConflict,
        FetchSame,
        SkipAuthorization,
        DoubleClaim,
        ForeignOwner,
        WrongRoute,
        Unchecked,
    }
    impl Variant {
        pub const BROKEN: [Self; 8] = [
            Self::FailedAsSuccess,
            Self::FetchConflict,
            Self::FetchSame,
            Self::SkipAuthorization,
            Self::DoubleClaim,
            Self::ForeignOwner,
            Self::WrongRoute,
            Self::Unchecked,
        ];
        pub fn part(self) -> Part {
            match self {
                Self::FailedAsSuccess => Part::FailProvider,
                Self::Unchecked => Part::BadProvider,
                _ => Part::Provider,
            }
        }
        pub fn target(self) -> &'static str {
            SAFETY[match self {
                Self::FailedAsSuccess => 0,
                Self::FetchConflict => 1,
                Self::FetchSame | Self::SkipAuthorization => 2,
                Self::DoubleClaim | Self::ForeignOwner => 3,
                _ => 4,
            }]
        }
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Scheme {
        Https,
        File,
        Custom,
        Other,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Route {
        Provider(u8),
        Https,
        File,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Provider {
        id: u8,
        system: bool,
        scheme: Scheme,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Request {
        id: u8,
        address: u8,
        version: u8,
        digest: u8,
        fetch: Scheme,
        permitted: bool,
        valid: bool,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Answer {
        Succeeded,
        Conflict,
        Refused,
        Invalid,
        Unavailable,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Archive {
        Good,
        Mismatch,
        Failed,
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Data {
        stage: u8,
        providers: Vec<Provider>, // saved immutable claims, all not retired in this script
        installed: [Option<u8>; 2], // saved address/version slots
        pending: [Option<Request>; 7], // saved decided first installs
        jobs: [Option<Request>; 7],
        choice_used: bool,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Message {
        Register(Provider),
        Install(Request),
        Fetch(Request, Route),
        Archive(Request, Archive),
        UnneededFetch(Request),
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Local {
        Next,
        Fetch(u8, Archive),
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Registration {
        candidate: Provider,
        before: Vec<Provider>,
        after: Vec<Provider>,
        invalid: bool,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Decision {
        request: Request,
        prior: Option<u8>,
        answer: Option<Answer>,
        before: [Option<u8>; 2],
        after: [Option<u8>; 2],
        fetched: usize,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct Fetch {
        request: Request,
        given: Request,
        route: Route,
        expected: Route,
        prior: Option<u8>,
    }
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub struct End {
        request: Request,
        failed: bool,
        answer: Answer,
        dispatched: bool,
        before: [Option<u8>; 2],
        after: [Option<u8>; 2],
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum Verification {
        Check { request: Request, actual: u8 },
        Use { request: Request, actual: u8 },
    }
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub struct Ledger {
        registrations: Vec<Registration>,
        decisions: Vec<Decision>,
        fetches: Vec<Fetch>,
        archives: Vec<(Request, Archive)>,
        ends: Vec<End>,
        verification: Vec<Verification>,
        done: bool,
    }
    #[derive(Clone)]
    pub struct Packages {
        pub variant: Variant,
        pub part: Part,
    }
    pub fn model(variant: Variant, part: Part) -> Harness<Packages> {
        let faults: &[Fault] = match part {
            Part::RequestLoss => &[Fault::Loss],
            Part::RequestReorder | Part::FetchReorder => &[Fault::Reorder],
            Part::FetchLoss => &[Fault::Loss, Fault::Timer],
            Part::Crash => &[Fault::Crash, Fault::Recovery],
            _ => &[],
        };
        Harness::new(config("MC7b installs", faults), Packages { variant, part })
    }
    fn route(providers: &[Provider], scheme: Scheme) -> Route {
        if let Some(p) = providers.iter().find(|p| p.scheme == scheme) {
            Route::Provider(p.id)
        } else {
            match scheme {
                Scheme::Https => Route::Https,
                Scheme::File => Route::File,
                _ => unreachable!(),
            }
        }
    }
    impl Packages {
        fn request(&self, id: u8, address: u8, digest: u8) -> Request {
            Request {
                id,
                address,
                version: 1,
                digest,
                fetch: self.part.scheme(),
                permitted: true,
                valid: true,
            }
        }
        fn fetch(&self, c: &Config, s: &mut System<Self>, r: Request, prior: Option<u8>) {
            let expected = route(&s.nodes[E].data.providers, r.fetch);
            let selected =
                if self.variant == Variant::WrongRoute && matches!(expected, Route::Provider(_)) {
                    Route::Https
                } else {
                    expected
                };
            s.ledger.fetches.push(Fetch {
                request: r,
                given: r,
                route: selected,
                expected,
                prior,
            });
            let message = if prior.is_some() {
                Message::UnneededFetch(r)
            } else {
                Message::Fetch(r, selected)
            };
            s.send(c, E, S, message);
        }
        fn decide(&self, c: &Config, s: &mut System<Self>, r: Request) {
            let before = s.nodes[E].data.installed;
            let prior = before[r.address as usize];
            let same = prior == Some(r.digest);
            let answer = if same && self.variant == Variant::SkipAuthorization {
                Some(Answer::Succeeded)
            } else if !r.permitted {
                Some(Answer::Refused)
            } else if !r.valid {
                Some(Answer::Invalid)
            } else {
                prior.map(|digest| {
                    if digest == r.digest {
                        Answer::Succeeded
                    } else {
                        Answer::Conflict
                    }
                })
            };
            let count = s.ledger.fetches.len();
            if answer.is_none() {
                s.nodes[E].data.pending[r.id as usize] = Some(r);
                self.fetch(c, s, r, prior);
            } else if (answer == Some(Answer::Conflict) && self.variant == Variant::FetchConflict)
                || (answer == Some(Answer::Succeeded) && self.variant == Variant::FetchSame)
            {
                self.fetch(c, s, r, prior);
            }
            s.ledger.decisions.push(Decision {
                request: r,
                prior,
                answer,
                before,
                after: s.nodes[E].data.installed,
                fetched: s.ledger.fetches.len() - count,
            });
        }
        fn complete(&self, s: &mut System<Self>, r: Request, archive: Archive) {
            assert_eq!(s.nodes[E].data.pending[r.id as usize].take(), Some(r));
            let before = s.nodes[E].data.installed;
            let failed = archive == Archive::Failed;
            let (answer, dispatched) = if failed {
                if self.variant == Variant::FailedAsSuccess {
                    (Answer::Succeeded, true)
                } else {
                    (Answer::Unavailable, false)
                }
            } else {
                let actual = if archive == Archive::Mismatch {
                    r.digest + 1
                } else {
                    r.digest
                };
                if self.variant != Variant::Unchecked {
                    s.ledger
                        .verification
                        .push(Verification::Check { request: r, actual });
                }
                if actual != r.digest && self.variant != Variant::Unchecked {
                    (Answer::Invalid, false)
                } else {
                    s.ledger
                        .verification
                        .push(Verification::Use { request: r, actual });
                    s.nodes[E].data.installed[r.address as usize] = Some(r.digest);
                    (Answer::Succeeded, true)
                }
            };
            s.ledger.ends.push(End {
                request: r,
                failed,
                answer,
                dispatched,
                before,
                after: s.nodes[E].data.installed,
            });
        }
    }
    impl Protocol for Packages {
        type Node = Data;
        type Message = Message;
        type Ledger = Ledger;
        type Local = Local;
        fn gate_remaining_faults(&self) -> bool {
            true
        }
        fn fault_allowed(&self, f: Fault, src: usize, dst: usize) -> bool {
            match f {
                Fault::Crash | Fault::Recovery | Fault::Timer => src == E,
                Fault::Loss | Fault::Reorder => match self.part {
                    Part::RequestLoss | Part::RequestReorder => src == A && dst == E,
                    Part::FetchLoss | Part::FetchReorder => {
                        (src == E && dst == S) || (src == S && dst == E)
                    }
                    _ => false,
                },
                _ => false,
            }
        }
        fn fault_enabled(&self, s: &System<Self>, f: Fault) -> bool {
            if f == Fault::Recovery {
                return true;
            }
            if s.nodes[A].data.stage != 1 {
                return false;
            }
            if f == Fault::Crash {
                s.nodes[E].data.pending.iter().any(Option::is_some)
            } else {
                matches!(f, Fault::Loss | Fault::Reorder | Fault::Timer)
            }
        }
        fn initial(&self, c: &Config) -> System<Self> {
            let mut s = initial::<Self>(Data::default(), Ledger::default());
            for (id, system, scheme) in [
                (0, true, self.part.claim()),
                (1, true, self.part.claim()),
                (2, false, Scheme::Other),
            ] {
                s.send(c, A, E, Message::Register(Provider { id, system, scheme }));
            }
            s
        }
        fn receive(&self, c: &Config, s: &mut System<Self>, _: usize, _: usize, message: Message) {
            match message {
                Message::Register(candidate) => {
                    let before = s.nodes[E].data.providers.clone();
                    let duplicate = before.iter().any(|p| p.scheme == candidate.scheme);
                    let allowed = (candidate.system || self.variant == Variant::ForeignOwner)
                        && (!duplicate || self.variant == Variant::DoubleClaim);
                    if allowed {
                        s.nodes[E].data.providers.push(candidate);
                    }
                    s.ledger.registrations.push(Registration {
                        candidate,
                        before,
                        after: s.nodes[E].data.providers.clone(),
                        invalid: !allowed,
                    });
                }
                Message::Install(r) => self.decide(c, s, r),
                Message::Fetch(r, _) => {
                    s.nodes[S].data.jobs[r.id as usize] = Some(r);
                }
                Message::Archive(r, archive) => self.complete(s, r, archive),
                Message::UnneededFetch(_) => {}
            }
        }
        fn timer_enabled(&self, s: &System<Self>, n: usize) -> bool {
            n == E
                && drained::<Self>(s)
                && s.nodes[S].data.jobs.iter().all(Option::is_none)
                && s.nodes[E].data.pending.iter().any(Option::is_some)
        }
        fn timer(&self, _: &Config, s: &mut System<Self>, _: usize) {
            let r = s.nodes[E]
                .data
                .pending
                .iter()
                .flatten()
                .next()
                .copied()
                .unwrap();
            s.ledger.archives.push((r, Archive::Failed));
            self.complete(s, r, Archive::Failed);
        }
        fn crash(&self, _: &mut Data) {}
        fn recover(&self, _: &mut Data) {}
        fn local_actions(&self, s: &System<Self>, n: usize, out: &mut Vec<Local>) {
            if n == S {
                for (i, request) in s.nodes[S].data.jobs.iter().enumerate() {
                    if let Some(r) = request {
                        out.push(Local::Fetch(i as u8, Archive::Good));
                        if r.id == 6 && !s.nodes[S].data.choice_used {
                            if self.part.failed() {
                                out.push(Local::Fetch(i as u8, Archive::Failed));
                            }
                            if self.part.bad() {
                                out.push(Local::Fetch(i as u8, Archive::Mismatch));
                            }
                        }
                    }
                }
            }
            if n == A
                && drained::<Self>(s)
                && s.nodes[E].data.pending.iter().all(Option::is_none)
                && s.nodes[S].data.jobs.iter().all(Option::is_none)
                && s.nodes[A].data.stage < 3
            {
                out.push(Local::Next);
            }
        }
        fn local_step(&self, c: &Config, s: &mut System<Self>, n: usize, action: Local) {
            match action {
                Local::Fetch(id, archive) => {
                    let r = s.nodes[n].data.jobs[id as usize].take().unwrap();
                    s.nodes[n].data.choice_used |= archive != Archive::Good;
                    s.ledger.archives.push((r, archive));
                    s.send(c, S, E, Message::Archive(r, archive));
                }
                Local::Next => {
                    match s.nodes[A].data.stage {
                        0 => {
                            s.send(c, A, E, Message::Install(self.request(0, 0, 1)));
                            s.send(
                                c,
                                A,
                                E,
                                Message::Install(self.request(
                                    1,
                                    0,
                                    if self.part == Part::Same { 1 } else { 2 },
                                )),
                            );
                        }
                        1 => {
                            let digest = s.nodes[E].data.installed[0].unwrap(); // actor's bounded post-race inspection input
                            for (id, value, permitted, valid) in [
                                (2, digest, true, true),
                                (3, 3 - digest, true, true),
                                (4, digest, false, true),
                                (5, digest, true, false),
                            ] {
                                let mut r = self.request(id, 0, value);
                                r.permitted = permitted;
                                r.valid = valid;
                                s.send(c, A, E, Message::Install(r));
                            }
                            s.send(c, A, E, Message::Install(self.request(6, 1, 3)));
                        }
                        2 => s.ledger.done = true,
                        _ => unreachable!(),
                    }
                    s.nodes[A].data.stage += 1;
                }
            }
        }
        fn properties(&self) -> Vec<NamedProperty<Self>> {
            vec![
                safety!(SAFETY[0], &["PKG-024"], |_, s| s.ledger.ends.iter().all(
                    |e| !e.failed
                        || (e.answer == Answer::Unavailable
                            && !e.dispatched
                            && e.before == e.after)
                )),
                safety!(SAFETY[1], &["PKG-025"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .all(|d| !(d.request.permitted
                        && d.request.valid
                        && d.prior.is_some_and(|v| v != d.request.digest))
                        || (d.answer == Some(Answer::Conflict)
                            && d.fetched == 0
                            && d.before == d.after))),
                safety!(SAFETY[2], &["PKG-027"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .all(|d| {
                        if d.prior != Some(d.request.digest) {
                            return true;
                        }
                        let expected = if !d.request.permitted {
                            Answer::Refused
                        } else if !d.request.valid {
                            Answer::Invalid
                        } else {
                            Answer::Succeeded
                        };
                        d.answer == Some(expected) && d.fetched == 0 && d.before == d.after
                    })),
                safety!(SAFETY[3], &["PRV-013"], |_, s| s
                    .ledger
                    .registrations
                    .iter()
                    .all(|r| {
                        let forbidden = !r.candidate.system
                            || r.before.iter().any(|p| p.scheme == r.candidate.scheme);
                        r.after.iter().all(|p| {
                            p.system
                                && r.after
                                    .iter()
                                    .filter(|other| p.scheme == other.scheme)
                                    .count()
                                    == 1
                        }) && (!forbidden || (r.invalid && r.before == r.after))
                    })),
                safety!(SAFETY[4], &["PRV-017"], |_, s| {
                    s.ledger.fetches.iter().all(|f| {
                        f.prior.is_some() || (f.route == f.expected && f.request == f.given)
                    }) && s
                        .ledger
                        .verification
                        .iter()
                        .enumerate()
                        .all(|(i, e)| match e {
                            Verification::Check { .. } => true,
                            Verification::Use { request, actual } => {
                                *actual == request.digest
                                    && s.ledger.verification[..i].iter().any(|v| {
                                        *v == Verification::Check {
                                            request: *request,
                                            actual: *actual,
                                        }
                                    })
                            }
                        })
                }),
                witness!(REACH[0], &["PKG-025", "PKG-027"], |_, s| [0, 1].iter().all(
                    |id| s
                        .ledger
                        .decisions
                        .iter()
                        .any(|d| d.request.id == *id && d.prior.is_none())
                )),
                witness!(REACH[1], &["PKG-025"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .any(|d| d.request.permitted
                        && d.request.valid
                        && d.prior.is_some_and(|v| v != d.request.digest))),
                witness!(REACH[2], &["PKG-027"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .any(|d| d.request.permitted
                        && d.request.valid
                        && d.prior == Some(d.request.digest))),
                witness!(REACH[3], &["PKG-027"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .any(
                        |d| !d.request.permitted && d.prior == Some(d.request.digest)
                    )),
                witness!(REACH[4], &["PKG-027"], |_, s| s
                    .ledger
                    .decisions
                    .iter()
                    .any(|d| !d.request.valid && d.prior == Some(d.request.digest))),
                witness!(REACH[5], &["PRV-013"], |_, s| s.ledger.registrations.len()
                    == 3),
                witness!(REACH[6], &["PRV-017"], |_, s| !s.ledger.fetches.is_empty()),
                witness!(REACH[7], &["PRV-017"], |_, s| s
                    .ledger
                    .verification
                    .iter()
                    .any(|v| matches!(v, Verification::Use { .. }))),
                witness!(REACH[8], &["PKG-025", "PKG-027"], |_, s| {
                    let first = s
                        .ledger
                        .ends
                        .iter()
                        .find(|e| e.request.id == 0 && e.answer == Answer::Succeeded);
                    let second = s
                        .ledger
                        .ends
                        .iter()
                        .find(|e| e.request.id == 1 && e.answer == Answer::Succeeded);
                    first
                        .zip(second)
                        .is_some_and(|(a, b)| a.request.digest != b.request.digest)
                }),
                witness!(REACH[9], &["PKG-024"], |_, s| s.ledger.done),
                witness!(REACH[10], &["PKG-024"], |_, s| s
                    .ledger
                    .archives
                    .iter()
                    .any(|(_, a)| *a == Archive::Failed)),
                witness!(REACH[11], &["PRV-017"], |_, s| s
                    .ledger
                    .archives
                    .iter()
                    .any(|(_, a)| *a == Archive::Mismatch)),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stateright::Model;

    fn verdicts<P: Protocol>(
        label: &str,
        m: Harness<P>,
        broken: Vec<&'static str>,
        missing: Vec<&'static str>,
    ) {
        let report = check_all(m);
        println!("MC7b {label}: {report}");
        let mut actual = (report.broken, report.missing_witnesses);
        let mut expected = (broken, missing);
        actual.0.sort();
        actual.1.sort();
        expected.0.sort();
        expected.1.sort();
        assert_eq!(actual, expected, "MC7b table mismatch: {label}");
    }
    fn refute<P: Protocol>(label: &str, m: Harness<P>, target: &'static str, empty: P::Node) {
        println!("MC7b first refutation {label}: {target}");
        let trace = assert_refuted(m.clone(), target);
        let path = stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
        let mut s = path.last_state().clone();
        for node in &mut s.nodes {
            node.data = empty.clone();
        }
        assert!(
            !(m.property(target).condition)(&m, &s),
            "safety must read only the emission ledger"
        );
        println!("MC7b first trace {label}: {} actions", trace.len());
    }
    #[test]
    fn records_table() {
        use record::*;
        for (v, p) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            let missing = REACH
                .into_iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    let reached = match i {
                        5 => matches!(p, Part::Unreadable | Part::ReadLoss),
                        6 => p == Part::Unknown,
                        _ => true,
                    };
                    (!reached).then_some(name)
                })
                .collect();
            verdicts(
                &format!("records {v:?}/{p:?}"),
                model(v, p),
                if v == Variant::Correct {
                    vec![]
                } else {
                    vec![v.target()]
                },
                missing,
            );
        }
    }
    #[test]
    fn records_first_refutations() {
        use record::*;
        for v in Variant::BROKEN {
            refute(
                &format!("records {v:?}"),
                model(v, v.part()),
                v.target(),
                Data::default(),
            );
        }
    }
    #[test]
    fn observation_table() {
        use observe::*;
        for (v, p) in Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
        {
            verdicts(
                &format!("observation {v:?}/{p:?}"),
                model(v, p),
                if v == Variant::Correct {
                    vec![]
                } else {
                    vec![v.target()]
                },
                vec![],
            );
        }
    }
    #[test]
    fn observation_first_refutations() {
        use observe::*;
        for v in Variant::BROKEN {
            refute(
                &format!("observation {v:?}"),
                model(v, v.part()),
                v.target(),
                Data::default(),
            );
        }
    }
    fn install_rows() -> Vec<(install::Variant, install::Part)> {
        use install::*;
        Part::ALL
            .into_iter()
            .map(|p| (Variant::Correct, p))
            .chain(Variant::BROKEN.into_iter().map(|v| (v, v.part())))
            .chain([
                (Variant::Unchecked, Part::BadHttps),
                (Variant::Unchecked, Part::BadFile),
            ])
            .collect()
    }
    #[test]
    fn installs_table() {
        use install::*;
        for (v, p) in install_rows() {
            let missing = REACH
                .into_iter()
                .enumerate()
                .filter_map(|(i, name)| {
                    let reached = match i {
                        8 => p != Part::Same,
                        10 => matches!(
                            p,
                            Part::FailProvider | Part::FailHttps | Part::FailFile | Part::FetchLoss
                        ),
                        11 => matches!(p, Part::BadProvider | Part::BadHttps | Part::BadFile),
                        _ => true,
                    };
                    (!reached).then_some(name)
                })
                .collect();
            verdicts(
                &format!("installs {v:?}/{p:?}"),
                model(v, p),
                if v == Variant::Correct {
                    vec![]
                } else {
                    vec![v.target()]
                },
                missing,
            );
        }
    }
    #[test]
    fn installs_first_refutations() {
        use install::*;
        for (v, p) in install_rows()
            .into_iter()
            .filter(|(v, _)| *v != Variant::Correct)
        {
            refute(
                &format!("installs {v:?}/{p:?}"),
                model(v, p),
                v.target(),
                Data::default(),
            );
        }
    }
    #[test]
    fn concurrent_first_installs_witness() {
        let m = install::model(install::Variant::Correct, install::Part::Provider);
        println!("MC7b shortest Correct/Provider different-firsts witness");
        let trace = assert_reached(m.clone(), install::REACH[8]);
        let path = stateright::Path::from_actions(&m, m.init_states().remove(0), &trace).unwrap();
        let mut s = path.last_state().clone();
        for node in &mut s.nodes {
            node.data = install::Data::default();
        }
        assert!((m.property(install::REACH[8]).condition)(&m, &s));
        println!(
            "MC7b different-firsts shortest witness: {} actions",
            trace.len()
        );
    }
    #[test]
    fn configuration_and_property_inventory() {
        fn inventory<P: Protocol>(m: Harness<P>, safety: &[&str], reach: &[&str]) {
            m.config.validate();
            assert_eq!(m.config.unique_state_ceiling, 190_000);
            assert!(m.config.timeout < Duration::from_secs(10));
            assert!(m.protocol.gate_remaining_faults());
            let properties = m.protocol.properties();
            assert_eq!(properties.len(), safety.len() + reach.len());
            for name in safety.iter().chain(reach) {
                assert!(properties.iter().any(|p| p.property.name == *name));
            }
        }
        for p in record::Part::ALL {
            inventory(
                record::model(record::Variant::Correct, p),
                &record::SAFETY,
                &record::REACH,
            );
        }
        for p in observe::Part::ALL {
            inventory(
                observe::model(observe::Variant::Correct, p),
                &observe::SAFETY,
                &observe::REACH,
            );
        }
        for p in install::Part::ALL {
            inventory(
                install::model(install::Variant::Correct, p),
                &install::SAFETY,
                &install::REACH,
            );
        }
    }
}
