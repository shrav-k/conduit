use crate::{
    Output,
    directory::{self, Directory, Identity},
    failure::Step,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq)]
struct Stat {
    kind: &'static str,
    size: i128,
    inode: u128,
    device: u128,
    modified: (i128, i128),
    changed: (i128, i128),
}
fn stat(m: &libc::stat) -> Stat {
    let mode = m.st_mode & libc::S_IFMT;
    let (device, inode) = directory::identity(m);
    Stat {
        kind: if mode == libc::S_IFDIR {
            "directory"
        } else if mode == libc::S_IFLNK {
            "link"
        } else if mode == libc::S_IFREG {
            "file"
        } else {
            "other"
        },
        size: i128::from(m.st_size),
        inode,
        device,
        modified: (i128::from(m.st_mtime), i128::from(m.st_mtime_nsec)),
        changed: (i128::from(m.st_ctime), i128::from(m.st_ctime_nsec)),
    }
}
type TargetStat = (i128, (i128, i128), (i128, i128));
#[derive(Debug, PartialEq, Eq)]
struct Entry {
    own: Stat,
    target: Option<TargetStat>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Listing(BTreeMap<PathBuf, Entry>);
impl Listing {
    pub fn read(home: &Path) -> io::Result<Self> {
        Self::read_observed(home, |_, _| {})
    }
    /// Deterministic test observation points; production supplies a no-op observer.
    pub fn read_observed(home: &Path, mut observe: impl FnMut(&Path, &str)) -> io::Result<Self> {
        fn visit(
            dir: &Directory,
            relative: &Path,
            map: &mut BTreeMap<PathBuf, Entry>,
            observe: &mut impl FnMut(&Path, &str),
        ) -> io::Result<()> {
            for name in dir.names()? {
                let path = relative.join(&name);
                observe(&path, "before_stat");
                let meta = match dir.metadata(&name, false) {
                    Ok(meta) => meta,
                    Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                    Err(e) => return Err(e),
                };
                observe(&path, "after_stat");
                let own = stat(&meta);
                let target = if own.kind == "link" {
                    match dir.metadata(&name, true) {
                        Ok(m) => {
                            let target = stat(&m);
                            if target.kind == "directory" {
                                return Err(io::Error::other("directory_link_refused"));
                            }
                            Some((target.size, target.modified, target.changed))
                        }
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            if dir
                                .metadata(&name, false)
                                .is_err_and(|e| e.kind() == io::ErrorKind::NotFound)
                            {
                                continue;
                            }
                            // A dangling link could resolve to the next scratch directory.
                            return Err(io::Error::other("unresolved_link_refused"));
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    None
                };
                if own.kind == "directory" {
                    let child = match dir.child(&name) {
                        Ok(dir) => dir,
                        Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
                        Err(e) => return Err(e),
                    };
                    let opened = stat(&child.own_metadata()?);
                    if (own.device, own.inode) != (opened.device, opened.inode) {
                        return Err(io::Error::other("directory_replaced"));
                    }
                    visit(&child, &path, map, observe)?;
                    // Detect a rename while walking; never resolve descendants by pathname.
                    let current = match dir.metadata(&name, false) {
                        Ok(meta) => stat(&meta),
                        Err(e) if e.kind() == io::ErrorKind::NotFound => {
                            map.retain(|entry, _| !entry.starts_with(&path));
                            continue;
                        }
                        Err(e) => return Err(e),
                    };
                    if current.kind != "directory"
                        || (current.device, current.inode) != (own.device, own.inode)
                    {
                        return Err(io::Error::other("directory_replaced"));
                    }
                }
                map.insert(path, Entry { own, target });
            }
            Ok(())
        }
        let result = (|| {
            let dir = Directory::open(home)?;
            let mut map = BTreeMap::new();
            visit(&dir, Path::new(""), &mut map, &mut observe)?;
            Ok(Self(map))
        })();
        result.map_err(|e| Step::Listing.error(e))
    }
    /// Whether a directory under the home is one of these directories, however it got there:
    /// a hard link or a mount of it would put what lies under it inside the home.
    pub fn holds_any_directory(&self, identities: &[Identity]) -> bool {
        self.0.values().any(|entry| {
            entry.own.kind == "directory"
                && identities.contains(&(entry.own.device, entry.own.inode))
        })
    }
    /// The identity of every entry under the home.
    pub fn identities(&self) -> impl Iterator<Item = Identity> + '_ {
        self.0
            .values()
            .map(|entry| (entry.own.device, entry.own.inode))
    }
    /// Whether an entry anywhere under the home has this name.
    pub fn holds_name(&self, name: &std::ffi::OsStr) -> bool {
        self.0.keys().any(|path| path.file_name() == Some(name))
    }
    /// Whether every entry under the home is on this device: none is, or lies under, a mount.
    pub fn on_device(&self, device: u128) -> bool {
        self.0.values().all(|entry| entry.own.device == device)
    }
    pub fn compare(&self, after: &Self, codex: bool, out: &mut Output) -> bool {
        let auth_names = |listing: &Self| {
            listing
                .0
                .keys()
                .filter(|p| p.components().count() == 1)
                .filter(|p| {
                    let n = p.as_os_str().to_string_lossy();
                    n.starts_with("auth") || n.starts_with(".auth")
                })
                .cloned()
                .collect::<BTreeSet<_>>()
        };
        let unchanged = self.0.get(Path::new("auth.json")) == after.0.get(Path::new("auth.json"))
            && auth_names(self) == auth_names(after);
        if codex {
            out.value("home.login_file_unchanged", unchanged);
        }
        out.value("home.unchanged", self == after);
        let mut tops = BTreeSet::new();
        for path in self.0.keys().chain(after.0.keys()) {
            if self.0.get(path) != after.0.get(path) {
                tops.insert(PathBuf::from(path.components().next().unwrap().as_os_str()));
            }
        }
        for top in tops {
            let name = top.to_string_lossy();
            let safe = matches!(
                name.as_ref(),
                "auth.json"
                    | "auth.lock"
                    | "auth.json.lock"
                    | ".auth.lock"
                    | ".claude.json"
                    | "config.toml"
                    | "settings.json"
                    | "state"
                    | "logs"
                    | "sessions"
                    | "projects"
                    | "history.jsonl"
                    | "cache"
            );
            let name = if safe { &name } else { "?" };
            let change = match (self.0.contains_key(&top), after.0.contains_key(&top)) {
                (false, true) => "added",
                (true, false) => "removed",
                _ => "changed",
            };
            out.value(&format!("home.changed {name}"), change);
        }
        unchanged
    }
}
