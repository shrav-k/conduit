//! Directory descriptors keep traversal relative to an already-open parent.
use std::ffi::{CStr, CString, OsStr, OsString};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Component, Path};

pub struct Directory(OwnedFd);
/// A directory's identity, its device and inode: the same however its path is spelled.
pub type Identity = (u128, u128);
pub fn identity(stat: &libc::stat) -> Identity {
    (stat.st_dev as u128, u128::from(stat.st_ino))
}
fn name(value: &OsStr) -> io::Result<CString> {
    CString::new(value.as_bytes()).map_err(|_| io::Error::other("invalid_name"))
}
impl Directory {
    pub fn open(path: &Path) -> io::Result<Self> {
        if !path.is_absolute() {
            return Err(io::Error::other("absolute_required"));
        }
        let fd = unsafe {
            libc::open(
                c"/".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        let mut dir = Self::from_fd(fd)?;
        for part in path.components() {
            match part {
                Component::RootDir => {}
                Component::Normal(part) => dir = dir.child(part)?,
                _ => return Err(io::Error::other("normal_path_required")),
            }
        }
        Ok(dir)
    }
    fn from_fd(fd: i32) -> io::Result<Self> {
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Self(unsafe { OwnedFd::from_raw_fd(fd) }))
    }
    pub fn child(&self, child: &OsStr) -> io::Result<Self> {
        let child = name(child)?;
        Self::from_fd(unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                child.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        })
    }
    pub fn metadata(&self, child: &OsStr, follow: bool) -> io::Result<libc::stat> {
        let child = name(child)?;
        let mut stat = std::mem::MaybeUninit::uninit();
        let result = unsafe {
            libc::fstatat(
                self.0.as_raw_fd(),
                child.as_ptr(),
                stat.as_mut_ptr(),
                if follow { 0 } else { libc::AT_SYMLINK_NOFOLLOW },
            )
        };
        if result < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { stat.assume_init() })
    }
    pub fn own_metadata(&self) -> io::Result<libc::stat> {
        let mut stat = std::mem::MaybeUninit::uninit();
        if unsafe { libc::fstat(self.0.as_raw_fd(), stat.as_mut_ptr()) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { stat.assume_init() })
    }
    /// This directory's identity and that of each directory above it, found through `..` from
    /// the open descriptor alone, up to the root (whose `..` is itself). No path spelling, case,
    /// symlink or firmlink enters it: each step is the directory the kernel holds.
    pub fn ancestry(&self) -> io::Result<Vec<Identity>> {
        let mut current = self.child(OsStr::new("."))?;
        let mut ancestry = vec![identity(&current.own_metadata()?)];
        loop {
            let parent = current.child(OsStr::new(".."))?;
            let id = identity(&parent.own_metadata()?);
            if Some(&id) == ancestry.last() {
                return Ok(ancestry);
            }
            if ancestry.len() >= 4096 {
                return Err(io::Error::other("ancestry_unbounded"));
            }
            ancestry.push(id);
            current = parent;
        }
    }
    pub fn names(&self) -> io::Result<Vec<OsString>> {
        // openat(".") creates an independent directory offset, unlike dup.
        let fd = unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                c".".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let stream = unsafe { libc::fdopendir(fd) };
        if stream.is_null() {
            unsafe {
                libc::close(fd);
            };
            return Err(io::Error::last_os_error());
        }
        struct Stream(*mut libc::DIR);
        impl Drop for Stream {
            fn drop(&mut self) {
                unsafe {
                    libc::closedir(self.0);
                }
            }
        }
        let stream = Stream(stream);
        let mut names = Vec::new();
        loop {
            // readdir distinguishes EOF from error through errno.
            unsafe {
                *errno() = 0;
            }
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                if unsafe { *errno() } != 0 {
                    return Err(io::Error::last_os_error());
                }
                break;
            }
            let bytes = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
            if bytes != b"." && bytes != b".." {
                names.push(OsString::from_vec(bytes.to_vec()));
            }
        }
        Ok(names)
    }
    pub fn mkdir(&self, child: &OsStr) -> io::Result<Self> {
        let c = name(child)?;
        if unsafe { libc::mkdirat(self.0.as_raw_fd(), c.as_ptr(), 0o700) } < 0 {
            return Err(io::Error::last_os_error());
        }
        self.child(child)
    }
    pub fn write_new(&self, child: &OsStr, bytes: &[u8]) -> io::Result<()> {
        use std::io::Write;
        let child = name(child)?;
        let fd = unsafe {
            libc::openat(
                self.0.as_raw_fd(),
                child.as_ptr(),
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                0o600,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
        file.write_all(bytes)
    }
    /// Removes one entry by name, relative to this directory, never following a link: a file
    /// or link, or with `directory` an empty directory. Nothing is ever removed recursively here.
    pub fn unlink(&self, child: &OsStr, directory: bool) -> io::Result<()> {
        let child = name(child)?;
        if unsafe {
            libc::unlinkat(
                self.0.as_raw_fd(),
                child.as_ptr(),
                if directory { libc::AT_REMOVEDIR } else { 0 },
            )
        } < 0
        {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
pub fn is_directory(stat: &libc::stat) -> bool {
    stat.st_mode & libc::S_IFMT == libc::S_IFDIR
}
#[cfg(target_os = "macos")]
unsafe fn errno() -> *mut libc::c_int {
    unsafe { libc::__error() }
}
#[cfg(not(target_os = "macos"))]
unsafe fn errno() -> *mut libc::c_int {
    unsafe { libc::__errno_location() }
}

impl AsRawFd for Directory {
    fn as_raw_fd(&self) -> i32 {
        self.0.as_raw_fd()
    }
}
