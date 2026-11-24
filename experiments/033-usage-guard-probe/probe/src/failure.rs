use std::sync::OnceLock;
use std::{fmt, io};
/// The fixed reason of the refusal that ended the run, if one did.
static REASON: OnceLock<&'static str> = OnceLock::new();
pub fn reason() -> Option<&'static str> {
    REASON.get().copied()
}
#[derive(Clone, Copy, Debug)]
pub enum Step {
    Spawn,
    Scratch,
    Settings,
    Write,
    Listing,
    /// Not a failure: a client start or a write withheld after SIGINT, SIGTERM or SIGHUP.
    /// `interrupted: true` reports it; no `probe_failed` line prints.
    Interrupted,
}
impl Step {
    pub fn name(self) -> &'static str {
        match self {
            Self::Spawn => "spawn",
            Self::Scratch => "scratch",
            Self::Settings => "settings",
            Self::Write => "write",
            Self::Listing => "listing",
            Self::Interrupted => "interrupted",
        }
    }
    pub fn error(self, error: io::Error) -> io::Error {
        io::Error::new(error.kind(), self)
    }
    /// A refusal under this step for a fixed reason, which prints as `probe_failed.reason`.
    pub fn refuse(self, reason: &'static str) -> io::Error {
        let _ = REASON.set(reason);
        self.error(io::Error::other(reason))
    }
}
impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
impl std::error::Error for Step {}
