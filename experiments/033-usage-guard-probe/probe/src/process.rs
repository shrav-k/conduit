use crate::{
    directory::{self, Directory, Identity},
    failure::Step,
    listing::Listing,
    strays::Tree,
};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};
use std::io::{self, BufRead, BufReader};
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub const BLOCK: &str = "https://refresh.invalid/oauth/token";
pub const BASE: [&str; 6] = ["HOME", "LOGNAME", "PATH", "SHELL", "TERM", "USER"];
pub const STOP_GRACE: Duration = Duration::from_secs(2);
/// How long, after a stop, the probe still reads what the client printed before it.
pub const DRAIN: Duration = Duration::from_millis(500);
static INTERRUPTED: AtomicBool = AtomicBool::new(false);
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
extern "C" fn signal(_: libc::c_int) {
    INTERRUPTED.store(true, Ordering::SeqCst);
}
pub fn interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}
fn withheld() -> io::Result<()> {
    if interrupted() {
        return Err(Step::Interrupted.error(io::Error::other("interrupted")));
    }
    Ok(())
}
pub fn install_signals() {
    // The handler only sets an atomic flag; cleanup happens in normal control flow.
    unsafe {
        for sig in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
            libc::signal(sig, signal as *const () as libc::sighandler_t);
        }
        #[cfg(target_os = "linux")]
        libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0);
    }
}
pub fn wait(duration: Duration) {
    let end = Instant::now() + duration;
    while Instant::now() < end && !interrupted() {
        thread::sleep(Duration::from_millis(10).min(end.saturating_duration_since(Instant::now())));
    }
}
/// Which identity validation read: the home (0) or a directory above it (1 its parent), the
/// scratch root (0) or a directory above it, or the run's fresh directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seen {
    Home(usize),
    Scratch(usize),
    Fresh,
}
/// Deterministic test observation point over each identity validation reads; production
/// supplies a no-op observer.
pub type Observer = Arc<dyn Fn(Seen, &mut Identity) + Send + Sync>;
/// Set when cleanup leaves anything behind: `scratch_left: true` prints and the command exits 1.
static LEFT: AtomicBool = AtomicBool::new(false);
pub fn scratch_left() -> bool {
    LEFT.load(Ordering::SeqCst)
}
fn refused(reason: &'static str) -> io::Error {
    Step::Scratch.refuse(reason)
}
/// The identity of every entry the home held at a listing, and of the home itself: cleanup never
/// removes one, so a home directory or file renamed or linked into a scratch directory stays.
type Known = Arc<Mutex<BTreeSet<Identity>>>;
/// The run's one fresh directory, made inside the temporary root under a new random name, as
/// `mkdtemp` makes one, but relative to the root's descriptor, and kept open: every scratch
/// directory is made inside it, and the probe writes nowhere else.
struct Fresh {
    parent: Arc<Directory>,
    name: OsString,
    path: PathBuf,
    dir: Directory,
    id: Identity,
    known: Known,
}
impl Drop for Fresh {
    /// Each scratch directory has emptied and removed itself by now. What else lies here goes
    /// only if it is not a directory (a directory here is not one the probe made, and stays);
    /// the fresh directory goes only while its name in the root is still this directory.
    fn drop(&mut self) {
        let known = self.known.lock().map(|k| k.clone()).unwrap_or_default();
        let mut clean = true;
        match self.dir.names() {
            Ok(names) => {
                for name in names {
                    clean &= match self.dir.metadata(&name, false) {
                        Ok(stat) if directory::is_directory(&stat) => false,
                        Ok(_) => remove_entry(&self.dir, &name, self.id.0, &known, 0),
                        Err(e) => e.kind() == io::ErrorKind::NotFound,
                    };
                }
            }
            Err(_) => clean = false,
        }
        let ours = self
            .parent
            .metadata(&self.name, false)
            .is_ok_and(|stat| directory::identity(&stat) == self.id);
        if !(clean && ours && self.parent.unlink(&self.name, true).is_ok()) {
            LEFT.store(true, Ordering::SeqCst);
        }
    }
}
/// Deep enough for any client's working tree; anything deeper stays, and is reported.
const DEPTH: usize = 128;
/// Removes one entry of `dir` by name, through `dir`'s descriptor: never following a link, never
/// across a device boundary (each entry's device against the fresh directory's), never an entry
/// the home holds or held, and a directory only once it is opened without following a link, is
/// the directory just seen, and has been emptied the same way. Whether it went.
fn remove_entry(
    dir: &Directory,
    name: &OsStr,
    device: u128,
    known: &BTreeSet<Identity>,
    depth: usize,
) -> bool {
    let stat = match dir.metadata(name, false) {
        Ok(stat) => stat,
        Err(e) => return e.kind() == io::ErrorKind::NotFound,
    };
    let id = directory::identity(&stat);
    if id.0 != device || known.contains(&id) {
        return false;
    }
    if !directory::is_directory(&stat) {
        return dir.unlink(name, false).is_ok();
    }
    if depth >= DEPTH {
        return false;
    }
    let Ok(child) = dir.child(name) else {
        return false;
    };
    if child.own_metadata().map(|s| directory::identity(&s)).ok() != Some(id) {
        return false;
    }
    clear(&child, device, known, depth + 1) && dir.unlink(name, true).is_ok()
}
/// Empties `dir` through its descriptor, entry by entry as `remove_entry` allows. Whether all of
/// it went.
fn clear(dir: &Directory, device: u128, known: &BTreeSet<Identity>, depth: usize) -> bool {
    let Ok(names) = dir.names() else {
        return false;
    };
    let mut clean = true;
    for name in names {
        clean &= remove_entry(dir, &name, device, known, depth);
    }
    clean
}
fn random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u64(SEQUENCE.fetch_add(1, Ordering::Relaxed));
    hasher.finish()
}
pub struct ScratchRoot {
    path: PathBuf,
    home: PathBuf,
    /// The home's identity and those of the directories above it, as `..` finds them.
    home_ancestry: Vec<Identity>,
    dir: Arc<Directory>,
    observe: Observer,
    known: Known,
    fresh: Mutex<Option<Arc<Fresh>>>,
}
impl ScratchRoot {
    pub fn validate(home: &Path) -> io::Result<Self> {
        Self::validate_observed(home, |_, _| {})
    }
    /// Validates the home and the temporary root before anything is written anywhere. The home
    /// must lie on its parent's filesystem (a home that is itself a mount point may be a remount
    /// of a directory elsewhere), and nothing under it may be on another. The temporary root must
    /// lie on the home's filesystem: on one filesystem, identity alone rules out an alias, while
    /// a remount of the home elsewhere (nullfs, bindfs or another FUSE mount, an SMB or NFS
    /// loopback) reports a device of its own. And it may not be the home, lie under it, or lie
    /// above it, by identity: see `refuse_overlap`.
    pub fn validate_observed(
        home: &Path,
        observe: impl Fn(Seen, &mut Identity) + Send + Sync + 'static,
    ) -> io::Result<Self> {
        let observe: Observer = Arc::new(observe);
        let home = home.canonicalize().map_err(|e| Step::Listing.error(e))?;
        // Reject directory links before any scratch creation or client start. This also
        // prevents a link in the login home from reaching an otherwise separate temp root.
        let listing = Listing::read(&home)?;
        let mut home_ancestry = Directory::open(&home)
            .and_then(|dir| dir.ancestry())
            .map_err(|e| Step::Listing.error(e))?;
        let known: BTreeSet<Identity> = listing.identities().chain([home_ancestry[0]]).collect();
        for (i, id) in home_ancestry.iter_mut().enumerate() {
            observe(Seen::Home(i), id);
        }
        let home_device = home_ancestry[0].0;
        if home_ancestry
            .get(1)
            .is_none_or(|parent| parent.0 != home_device)
        {
            return Err(refused("home_mount_point"));
        }
        if !listing.on_device(home_device) {
            return Err(refused("home_holds_mount"));
        }
        let path = std::env::var_os("TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        let path = path.canonicalize().map_err(|e| Step::Scratch.error(e))?;
        let dir = Directory::open(&path).map_err(|e| Step::Scratch.error(e))?;
        let root = Self {
            path,
            home,
            home_ancestry,
            dir: Arc::new(dir),
            observe,
            known: Arc::new(Mutex::new(known)),
            fresh: Mutex::new(None),
        };
        root.refuse_overlap(&listing)?;
        Ok(root)
    }
    /// Refuses a temporary root on another filesystem than the home's, or one that is the home,
    /// lies under it or lies above it, by directory identity alone: the home's device and inode
    /// against those of the temporary root and each directory above it, as `..` finds them from
    /// the open root, and the root's against the home's and each directory's above it. The
    /// listing adds the other way in: a directory under the home that is the temporary root or one
    /// above it (a hard link). No spelling, case, symlink or firmlink of either path, nor a hard
    /// link inside the home, can pass.
    fn refuse_overlap(&self, listing: &Listing) -> io::Result<()> {
        let mut ancestry = self.dir.ancestry().map_err(|e| Step::Scratch.error(e))?;
        for (i, id) in ancestry.iter_mut().enumerate() {
            (self.observe)(Seen::Scratch(i), id);
        }
        let home = self.home_ancestry[0];
        if ancestry[0].0 != home.0 {
            return Err(refused("scratch_other_device"));
        }
        if ancestry.contains(&home)
            || self.home_ancestry.contains(&ancestry[0])
            || listing.holds_any_directory(&ancestry)
        {
            return Err(refused("scratch_overlap"));
        }
        Ok(())
    }
    /// Refuses the fresh directory where the home's listing holds its name or its identity: the
    /// temporary root was then an alias of a directory in the home that neither device nor
    /// identity showed, and the fresh directory, empty, is removed at once.
    fn refuse_alias(&self, fresh: &Fresh, listing: &Listing) -> io::Result<()> {
        let mut id = fresh.id;
        (self.observe)(Seen::Fresh, &mut id);
        if listing.holds_name(&fresh.name) || listing.holds_any_directory(&[id]) {
            return Err(refused("scratch_alias"));
        }
        Ok(())
    }
    /// The run's fresh directory, made at the first scratch directory, so nothing is written
    /// before a client is about to start.
    fn fresh(&self) -> io::Result<Arc<Fresh>> {
        let mut slot = self.fresh.lock().map_err(|_| refused("scratch_failed"))?;
        if let Some(fresh) = &*slot {
            return Ok(Arc::clone(fresh));
        }
        for _ in 0..100 {
            let name = OsString::from(format!(
                "usage-probe-{}-{:016x}",
                std::process::id(),
                random()
            ));
            match self.dir.mkdir(&name) {
                Ok(dir) => {
                    let id = dir
                        .own_metadata()
                        .map(|s| directory::identity(&s))
                        .map_err(|e| Step::Scratch.error(e))?;
                    let fresh = Fresh {
                        parent: Arc::clone(&self.dir),
                        path: self.path.join(&name),
                        name,
                        dir,
                        id,
                        known: Arc::clone(&self.known),
                    };
                    self.refuse_alias(&fresh, &Listing::read(&self.home)?)?;
                    let fresh = Arc::new(fresh);
                    *slot = Some(Arc::clone(&fresh));
                    return Ok(fresh);
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(Step::Scratch.error(e)),
            }
        }
        Err(refused("scratch_failed"))
    }
}
/// A scratch directory inside the run's fresh directory: its path, given to a client, and what
/// the probe kept of it.
pub struct Scratch(pub PathBuf, Kept);
pub struct Kept {
    dir: Directory,
    name: OsString,
    id: Identity,
    fresh: Arc<Fresh>,
}
impl Scratch {
    pub fn write_settings(&self) -> io::Result<()> {
        self.1
            .dir
            .write_new(OsStr::new("settings.json"), b"{\"disableAllHooks\":true}")
            .map_err(|e| Step::Settings.error(e))
    }
    pub fn new(root: &ScratchRoot) -> io::Result<Self> {
        let listing = Listing::read(&root.home)?;
        if let Ok(mut known) = root.known.lock() {
            known.extend(listing.identities());
        }
        // Checked again before each scratch directory: the root may have moved since.
        root.refuse_overlap(&listing)?;
        let fresh = root.fresh()?;
        root.refuse_alias(&fresh, &listing)?;
        for _ in 0..100 {
            let name = OsString::from(format!(
                "scratch-{}",
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            match fresh.dir.mkdir(&name) {
                Ok(dir) => {
                    let id = dir
                        .own_metadata()
                        .map(|s| directory::identity(&s))
                        .map_err(|e| Step::Scratch.error(e))?;
                    let scratch = Self(
                        fresh.path.join(&name),
                        Kept {
                            dir,
                            name,
                            id,
                            fresh: Arc::clone(&fresh),
                        },
                    );
                    // The path a client is given must still open this very directory.
                    let opened = Directory::open(&scratch.0)
                        .and_then(|d| d.own_metadata())
                        .map(|s| directory::identity(&s));
                    if opened.ok() != Some(id) {
                        return Err(refused("scratch_path_moved"));
                    }
                    return Ok(scratch);
                }
                Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(Step::Scratch.error(e)),
            }
        }
        Err(refused("scratch_failed"))
    }
}
impl Drop for Scratch {
    /// Empties the directory through the descriptor kept of it, never by a path, then removes its
    /// entry in the fresh directory only while that entry is still this directory.
    fn drop(&mut self) {
        let kept = &self.1;
        let known = kept
            .fresh
            .known
            .lock()
            .map(|k| k.clone())
            .unwrap_or_default();
        let cleared = clear(&kept.dir, kept.fresh.id.0, &known, 0);
        let removed = cleared
            && kept
                .fresh
                .dir
                .metadata(&kept.name, false)
                .is_ok_and(|stat| directory::identity(&stat) == kept.id)
            && kept.fresh.dir.unlink(&kept.name, true).is_ok();
        if !removed {
            LEFT.store(true, Ordering::SeqCst);
        }
    }
}
pub struct Launch {
    pub codex: bool,
    pub deadline: Duration,
    pub version: bool,
    pub settings: bool,
}
/// One line the client printed. A line over 1 MiB or not UTF-8 arrives once with an empty
/// `text`, which parses as nothing, and ends the reader: it is never silently lost.
pub struct Line {
    pub at: Instant,
    pub text: String,
}
pub struct Client {
    child: Child,
    pub input: Option<ChildStdin>,
    lines: Receiver<Line>,
    pub start: Instant,
    pub deadline: Instant,
    group: i32,
    stopped: Option<bool>,
    /// Lines `line()` took after the deadline: no answer, but kept for `drain`.
    late: RefCell<Vec<Line>>,
    /// The client and each descendant seen of it, swept by `watcher` every few milliseconds.
    tree: Arc<Mutex<Tree>>,
    watcher: Option<(Arc<AtomicBool>, thread::JoinHandle<()>)>,
    /// Descendants found running once the group had stopped, each killed.
    strays: usize,
    pub scratch: Scratch,
}
/// How often the watcher looks for the client's descendants.
const SWEEP: Duration = Duration::from_millis(5);
/// Every stray found in the run, and whether any client started: `stray_processes` prints the
/// count once a client has started, and any stray makes the command exit 1.
static STRAYS: AtomicUsize = AtomicUsize::new(0);
static SPAWNED: AtomicBool = AtomicBool::new(false);
pub fn strays() -> usize {
    STRAYS.load(Ordering::SeqCst)
}
pub fn spawned() -> bool {
    SPAWNED.load(Ordering::SeqCst)
}
impl Client {
    pub fn spawn(
        root: &ScratchRoot,
        path: &Path,
        home: &Path,
        args: &[String],
        launch: Launch,
    ) -> io::Result<Self> {
        let Launch {
            codex,
            deadline,
            version,
            settings,
        } = launch;
        // No client starts after an interrupt, however long the listings before it took.
        withheld()?;
        let scratch = Scratch::new(root)?;
        let mut cmd = Command::new(path);
        cmd.env_clear();
        for key in BASE {
            if let Some(value) = std::env::var_os(key) {
                cmd.env(key, value);
            }
        }
        if version {
            cmd.env("HOME", home);
        }
        cmd.env(
            if codex {
                "CODEX_HOME"
            } else {
                "CLAUDE_CONFIG_DIR"
            },
            home,
        );
        if codex {
            cmd.env("CODEX_REFRESH_TOKEN_URL_OVERRIDE", BLOCK);
        }
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .process_group(0);
        if settings {
            scratch.write_settings()?;
            cmd.arg("--settings").arg("settings.json");
        }
        let cwd = scratch.1.dir.as_raw_fd();
        // fchdir is async-signal-safe and cannot follow a replaced pathname.
        unsafe {
            cmd.pre_exec(move || {
                if libc::fchdir(cwd) < 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            });
        }
        withheld()?;
        let start = Instant::now();
        let mut child = cmd.spawn().map_err(|e| Step::Spawn.error(e))?;
        SPAWNED.store(true, Ordering::SeqCst);
        let group = child.id() as i32;
        let tree = Arc::new(Mutex::new(Tree::new(group)));
        let input = child.stdin.take();
        let stdout = child.stdout.take().unwrap();
        let (tx, lines) = mpsc::sync_channel(32);
        let unreadable = move |tx: &mpsc::SyncSender<Line>| {
            let _ = tx.send(Line {
                at: Instant::now(),
                text: String::new(),
            });
        };
        let reader = thread::Builder::new().spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                // Bound a malformed client's line without ever printing its bytes.
                let mut bytes = Vec::new();
                loop {
                    let buffer = match reader.fill_buf() {
                        Ok(buffer) => buffer,
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                        Err(_) => return unreadable(&tx),
                    };
                    if buffer.is_empty() {
                        if bytes.is_empty() {
                            return;
                        }
                        break;
                    }
                    let n = buffer
                        .iter()
                        .position(|b| *b == b'\n')
                        .map_or(buffer.len(), |n| n + 1);
                    if bytes.len() + n > 1_048_576 {
                        return unreadable(&tx);
                    }
                    bytes.extend_from_slice(&buffer[..n]);
                    reader.consume(n);
                    if bytes.last() == Some(&b'\n') {
                        break;
                    }
                }
                let at = Instant::now();
                let Ok(text) = String::from_utf8(bytes) else {
                    return unreadable(&tx);
                };
                if tx.send(Line { at, text }).is_err() {
                    return;
                }
            }
        });
        let done = Arc::new(AtomicBool::new(false));
        let watcher = reader.and_then(|_| {
            let (tree, done) = (Arc::clone(&tree), Arc::clone(&done));
            thread::Builder::new().spawn(move || {
                while !done.load(Ordering::SeqCst) {
                    if let Ok(mut tree) = tree.lock() {
                        tree.sweep();
                    }
                    thread::sleep(SWEEP);
                }
            })
        });
        let watcher = match watcher {
            Ok(watcher) => watcher,
            Err(error) => {
                // A std Child is not killed on drop: stop the group before its directory goes.
                drop(input);
                unsafe {
                    libc::killpg(group, libc::SIGKILL);
                }
                let _ = child.wait();
                return Err(Step::Spawn.error(error));
            }
        };
        Ok(Self {
            child,
            input,
            lines,
            start,
            deadline: start + deadline,
            group,
            stopped: None,
            late: RefCell::new(Vec::new()),
            tree,
            watcher: Some((done, watcher)),
            strays: 0,
            scratch,
        })
    }
    /// The next line the client printed, if one is waiting: no deadline applies, as in a hold.
    pub fn poll(&self) -> Option<Line> {
        self.lines.try_recv().ok()
    }
    pub fn line(&self) -> Option<Line> {
        loop {
            if interrupted() {
                return None;
            }
            match self.lines.try_recv() {
                Ok(line) => return self.timely(line),
                Err(mpsc::TryRecvError::Disconnected) => return None,
                Err(mpsc::TryRecvError::Empty) => {}
            }
            if Instant::now() >= self.deadline {
                return None;
            }
            match self.lines.recv_timeout(
                Duration::from_millis(10)
                    .min(self.deadline.saturating_duration_since(Instant::now())),
            ) {
                Ok(line) => return self.timely(line),
                Err(mpsc::RecvTimeoutError::Disconnected) => return None,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
    fn timely(&self, line: Line) -> Option<Line> {
        if line.at <= self.deadline {
            return Some(line);
        }
        self.late.borrow_mut().push(line);
        None
    }
    /// Waits up to `hold` for the client to exit after its answer, still reading: every line
    /// it prints meanwhile, with no spawn deadline, goes to `check`. Returns true at once when
    /// `check` does, for the caller to stop the group; false at the hold's end, at an
    /// interrupt, or once the client has exited and its output has closed.
    pub fn watch_exit(&mut self, hold: Duration, mut check: impl FnMut(&Line) -> bool) -> bool {
        let end = Instant::now() + hold;
        let mut open = true;
        loop {
            if interrupted() {
                return false;
            }
            if open {
                match self.lines.try_recv() {
                    Ok(line) => {
                        if check(&line) {
                            return true;
                        }
                    }
                    Err(mpsc::TryRecvError::Disconnected) => open = false,
                    Err(mpsc::TryRecvError::Empty) => {}
                }
            }
            let exited = self.child.try_wait().ok().flatten().is_some();
            let now = Instant::now();
            if (exited && !open) || now >= end {
                return false;
            }
            let pause = Duration::from_millis(10).min(end - now);
            if !open {
                thread::sleep(pause);
                continue;
            }
            match self.lines.recv_timeout(pause) {
                Ok(line) => {
                    if check(&line) {
                        return true;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => open = false,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
    }
    /// After a stop, hands `check` every line the client printed that no reader took: those
    /// `line()` took after the deadline, those queued and those still in the pipe, until its
    /// output closes or `DRAIN` has passed. A sign printed just before a stop, or after a
    /// deadline, is so still seen.
    pub fn drain(&self, mut check: impl FnMut(&Line) -> bool) {
        let late = std::mem::take(&mut *self.late.borrow_mut());
        for line in &late {
            check(line);
        }
        let end = Instant::now() + DRAIN;
        loop {
            let now = Instant::now();
            if now >= end {
                return;
            }
            match self.lines.recv_timeout(end - now) {
                Ok(line) => {
                    check(&line);
                }
                Err(_) => return,
            }
        }
    }
    pub fn stop(&mut self) -> bool {
        if let Some(done) = self.stopped {
            return done;
        }
        self.input.take();
        self.sweep();
        // Negative PID targets only the process group created at spawn.
        unsafe {
            libc::killpg(self.group, libc::SIGTERM);
        }
        let end = Instant::now() + STOP_GRACE;
        while Instant::now() < end {
            let _ = self.child.try_wait();
            self.reap_descendants();
            if !self.group_exists() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        if self.group_exists() {
            unsafe {
                libc::killpg(self.group, libc::SIGKILL);
            }
        }
        let _ = self.child.wait();
        let end = Instant::now() + Duration::from_millis(500);
        while self.group_exists() && Instant::now() < end {
            self.reap_descendants();
            thread::sleep(Duration::from_millis(10));
        }
        let stopped = !self.group_exists();
        if let Some((done, watcher)) = self.watcher.take() {
            done.store(true, Ordering::SeqCst);
            let _ = watcher.join();
        }
        self.strays = self.stop_strays();
        STRAYS.fetch_add(self.strays, Ordering::SeqCst);
        self.stopped = Some(stopped);
        stopped
    }
    /// Descendants found running once the group had stopped, each killed: a count only.
    pub fn strays(&self) -> usize {
        self.strays
    }
    fn sweep(&self) {
        if let Ok(mut tree) = self.tree.lock() {
            tree.sweep();
        }
    }
    /// Kills every descendant seen that still runs once the group has stopped: one that left the
    /// group, which no signal to the group reaches. Each is checked by its start time before
    /// each kill, and those a stray started meanwhile are found and killed too. On Linux, where
    /// the probe is the subreaper, each is reaped; on macOS the init process reaps it.
    fn stop_strays(&mut self) -> usize {
        let Ok(mut tree) = self.tree.lock() else {
            return 0;
        };
        let mut found = BTreeSet::new();
        let end = Instant::now() + STOP_GRACE;
        loop {
            tree.sweep();
            let running = tree.running_descendants();
            if running.is_empty() || Instant::now() >= end {
                found.extend(running);
                break;
            }
            for pid in running {
                found.insert(pid);
                unsafe {
                    libc::kill(pid, libc::SIGKILL);
                }
                #[cfg(target_os = "linux")]
                unsafe {
                    libc::waitpid(pid, std::ptr::null_mut(), libc::WNOHANG);
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
        found.len()
    }
    fn group_exists(&self) -> bool {
        let result = unsafe { libc::killpg(self.group, 0) };
        result == 0 || io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
    }
    fn reap_descendants(&self) {
        #[cfg(target_os = "linux")]
        unsafe {
            while libc::waitpid(-self.group, std::ptr::null_mut(), libc::WNOHANG) > 0 {}
        }
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        self.stop();
    }
}
