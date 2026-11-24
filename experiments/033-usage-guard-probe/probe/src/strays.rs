//! A client's descendants, as the probe saw them. A descendant that leaves the client's process
//! group (by `setsid` or `setpgid`) outlives the group's stop, and once its parent has exited it
//! is the init process's child, so no parent chain leads back to the client. The probe therefore
//! looks often while the client runs and keeps every descendant it saw, by pid and start time,
//! so a pid the system has since reused is never taken for one of them.
use std::collections::BTreeMap;
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
compile_error!("usage-probe finds a client's descendants on macOS and Linux only");

/// What the system says of one process: its start time, its parent, and whether it has exited
/// and waits only to be reaped.
#[derive(Clone, Copy)]
struct Info {
    start: u64,
    parent: i32,
    exited: bool,
}
#[cfg(target_os = "macos")]
fn info(pid: i32) -> Option<Info> {
    let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
    let size = std::mem::size_of::<libc::proc_bsdinfo>() as libc::c_int;
    let read =
        unsafe { libc::proc_pidinfo(pid, libc::PROC_PIDTBSDINFO, 0, (&raw mut info).cast(), size) };
    (read == size && info.pbi_pid == pid as u32).then(|| Info {
        start: info.pbi_start_tvsec * 1_000_000 + info.pbi_start_tvusec,
        parent: info.pbi_ppid as i32,
        exited: info.pbi_status == libc::SZOMB,
    })
}
#[cfg(target_os = "macos")]
fn children(pid: i32) -> Vec<i32> {
    const MOST: usize = 4096;
    // A zeroed buffer: whatever the count's unit, only real pids (never 0) are taken from it.
    let mut pids = vec![0 as libc::pid_t; MOST];
    let count = unsafe {
        libc::proc_listchildpids(
            pid,
            pids.as_mut_ptr().cast(),
            (MOST * std::mem::size_of::<libc::pid_t>()) as libc::c_int,
        )
    };
    if count <= 0 {
        return Vec::new();
    }
    pids.truncate((count as usize).min(MOST));
    pids.retain(|p| *p > 0);
    pids
}
#[cfg(target_os = "linux")]
fn info(pid: i32) -> Option<Info> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    // The command name may hold spaces and parentheses; the fields follow its last `)`.
    let fields: Vec<&str> = stat
        .get(stat.rfind(')')? + 1..)?
        .split_whitespace()
        .collect();
    Some(Info {
        start: fields.get(19)?.parse().ok()?,
        parent: fields.get(1)?.parse().ok()?,
        exited: matches!(fields.first(), Some(&"Z" | &"X")),
    })
}
#[cfg(target_os = "linux")]
fn children(pid: i32) -> Vec<i32> {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return Vec::new();
    };
    entries
        .filter_map(|e| e.ok()?.file_name().to_str()?.parse::<i32>().ok())
        .filter(|child| info(*child).is_some_and(|i| i.parent == pid))
        .collect()
}

/// The client and every descendant seen of it, by pid and start time.
pub struct Tree {
    root: i32,
    seen: BTreeMap<i32, u64>,
}
impl Tree {
    pub fn new(root: i32) -> Self {
        let mut seen = BTreeMap::new();
        if let Some(i) = info(root) {
            seen.insert(root, i.start);
        }
        Self { root, seen }
    }
    /// Whether `pid` is still the process seen under it, and has not exited.
    fn running(&self, pid: i32) -> bool {
        self.seen
            .get(&pid)
            .is_some_and(|start| info(pid).is_some_and(|i| i.start == *start && !i.exited))
    }
    /// Adds every child of each seen process that still runs, and theirs in turn. A process
    /// whose parent exited before any look is not seen: the looks are as frequent as the
    /// probe's reader, every few milliseconds while the client runs.
    pub fn sweep(&mut self) {
        let mut queue: Vec<i32> = self.seen.keys().copied().collect();
        while let Some(pid) = queue.pop() {
            if !self.running(pid) {
                continue;
            }
            for child in children(pid) {
                if self.seen.contains_key(&child) {
                    continue;
                }
                if let Some(i) = info(child).filter(|i| i.parent == pid) {
                    self.seen.insert(child, i.start);
                    queue.push(child);
                }
            }
        }
    }
    /// The descendants seen that still run, the client itself excepted.
    pub fn running_descendants(&self) -> Vec<i32> {
        self.seen
            .keys()
            .copied()
            .filter(|pid| *pid != self.root && self.running(*pid))
            .collect()
    }
}
