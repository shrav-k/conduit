use crate::MAX_BYTES;
use rustix::{
    fs::{
        AtFlags, Dir, FileType, Mode, OFlags, Stat, fchmod, fstat, linkat, mkdirat, open, openat,
        readlinkat, renameat, statat, unlinkat,
    },
    io::Errno,
};
use serde_json::{Value, json};
use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{self, Read, Write},
    os::{
        fd::OwnedFd,
        unix::{
            ffi::OsStrExt,
            fs::{MetadataExt, PermissionsExt, symlink},
        },
    },
    path::{Component, Path},
    sync::atomic::{AtomicU64, Ordering},
};

const DIRECTORY: OFlags = OFlags::RDONLY
    .union(OFlags::DIRECTORY)
    .union(OFlags::NOFOLLOW)
    .union(OFlags::CLOEXEC);

pub fn git_entry(name: &OsStr) -> bool {
    name.as_bytes().eq_ignore_ascii_case(b".git")
}

pub fn relative(path: &str) -> Result<&Path, String> {
    if path != "."
        && path.split('/').any(|part| {
            part.is_empty()
                || part == "."
                || part == ".."
                || git_entry(OsStr::new(part))
                || part.contains('\0')
        })
    {
        return Err(format!("invalid working-copy path {path:?}"));
    }
    Ok(Path::new(path))
}

pub fn dir_at(parent: &OwnedFd, name: &OsStr) -> rustix::io::Result<OwnedFd> {
    openat(parent, name, DIRECTORY, Mode::empty())
}

pub fn absolute(path: &Path) -> rustix::io::Result<OwnedFd> {
    let mut dir = open("/", DIRECTORY, Mode::empty())?;
    for step in path.components() {
        if step != Component::RootDir {
            dir = dir_at(&dir, step.as_os_str())?;
        }
    }
    Ok(dir)
}

pub fn walk(root: &OwnedFd, path: &Path, create: bool) -> rustix::io::Result<OwnedFd> {
    let mut dir = dir_at(root, OsStr::new("."))?;
    for step in path.components() {
        let name = step.as_os_str();
        dir = match dir_at(&dir, name) {
            Err(Errno::NOENT) if create => {
                match mkdirat(&dir, name, Mode::from_bits_truncate(0o777)) {
                    Ok(()) | Err(Errno::EXIST) => {}
                    Err(error) => return Err(error),
                }
                dir_at(&dir, name)?
            }
            result => result?,
        };
    }
    Ok(dir)
}

fn error(path: &Path, message: &str) -> String {
    format!("working-copy path {path:?}: {message}")
}

fn regular(stat: &Stat) -> bool {
    FileType::from_raw_mode(stat.st_mode) == FileType::RegularFile
}

pub fn regular_at(parent: &OwnedFd, name: &OsStr) -> io::Result<File> {
    let fd = openat(
        parent,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )?;
    if !regular(&fstat(&fd)?) {
        return Err(Errno::INVAL.into());
    }
    Ok(File::from(fd))
}

pub fn capture(parent: &OwnedFd, name: &OsStr, kind: FileType, target: &Path) -> io::Result<()> {
    match kind {
        FileType::RegularFile => {
            let mut source = regular_at(parent, name)?;
            let before = source.metadata()?;
            let mut output = File::create_new(target)?;
            let copied = io::copy(&mut source, &mut output)?;
            let after = source.metadata()?;
            let stamp = |meta: &fs::Metadata| {
                (
                    meta.mtime(),
                    meta.mtime_nsec(),
                    meta.ctime(),
                    meta.ctime_nsec(),
                )
            };
            if copied != before.len() || copied != after.len() || stamp(&before) != stamp(&after) {
                return Err(Errno::BUSY.into());
            }
            let mode = before.permissions().mode();
            output.set_permissions(fs::Permissions::from_mode(if mode & 0o100 != 0 {
                0o755
            } else {
                0o644
            }))
        }
        FileType::Symlink => {
            let before = statat(parent, name, AtFlags::SYMLINK_NOFOLLOW)?;
            let link = readlinkat(parent, name, Vec::new())?;
            let after = statat(parent, name, AtFlags::SYMLINK_NOFOLLOW)?;
            if FileType::from_raw_mode(before.st_mode) != FileType::Symlink
                || (before.st_dev, before.st_ino) != (after.st_dev, after.st_ino)
            {
                return Err(Errno::BUSY.into());
            }
            symlink(OsStr::from_bytes(link.to_bytes()), target)
        }
        _ => Err(Errno::INVAL.into()),
    }
}

pub fn remove_dir_at(parent: &OwnedFd, name: &OsStr, dir: &OwnedFd) -> rustix::io::Result<()> {
    let held = fstat(dir)?;
    let check_name = || -> rustix::io::Result<()> {
        let named = statat(parent, name, AtFlags::SYMLINK_NOFOLLOW)?;
        if (held.st_dev, held.st_ino) != (named.st_dev, named.st_ino) {
            return Err(Errno::BUSY);
        }
        Ok(())
    };
    check_name()?;
    for (child, kind) in entries(dir)? {
        if kind == FileType::Directory {
            let subdir = dir_at(dir, &child)?;
            remove_dir_at(dir, &child, &subdir)?;
        } else {
            unlinkat(dir, &child, AtFlags::empty())?;
        }
    }
    check_name()?;
    unlinkat(parent, name, AtFlags::REMOVEDIR)
}

pub fn read(root: &OwnedFd, path: &Path) -> Result<String, String> {
    let name = path
        .file_name()
        .ok_or_else(|| error(path, "not a regular file"))?;
    let parent = walk(root, path.parent().unwrap(), false)
        .map_err(|_| error(path, "parent is missing, inaccessible, or a symbolic link"))?;
    let file = openat(
        &parent,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .map_err(|_| error(path, "file is missing, inaccessible, or a symbolic link"))?;
    let stat = rustix::fs::fstat(&file).map_err(|_| error(path, "cannot inspect file"))?;
    if !regular(&stat) {
        return Err(error(path, "not a regular file"));
    }
    let mut bytes = Vec::new();
    File::from(file)
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| error(path, "cannot read file"))?;
    if bytes.len() > MAX_BYTES {
        return Err(error(path, "file exceeds 8 MiB"));
    }
    String::from_utf8(bytes).map_err(|_| error(path, "file is not valid UTF-8"))
}

pub fn entries(dir: &OwnedFd) -> rustix::io::Result<Vec<(OsString, FileType)>> {
    let mut entries = Vec::new();
    for item in Dir::read_from(dir)? {
        let item = item?;
        let name = OsStr::from_bytes(item.file_name().to_bytes());
        if name == "." || name == ".." {
            continue;
        }
        let stat = statat(dir, name, AtFlags::SYMLINK_NOFOLLOW)?;
        entries.push((name.to_owned(), FileType::from_raw_mode(stat.st_mode)));
    }
    Ok(entries)
}

pub fn list(root: &OwnedFd, path: &Path) -> Result<Vec<Value>, String> {
    let dir = walk(root, path, false).map_err(|_| {
        error(
            path,
            "not an accessible directory or contains a symbolic link",
        )
    })?;
    let mut entries = entries(&dir).map_err(|_| error(path, "cannot list directory"))?;
    entries.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    entries
        .into_iter()
        .filter(|(name, _)| !git_entry(name))
        .map(|(name, kind)| {
            let name = name
                .to_str()
                .ok_or_else(|| error(path, "entry name is not valid UTF-8"))?;
            let kind = match kind {
                FileType::RegularFile => "file",
                FileType::Directory => "directory",
                FileType::Symlink => "symlink",
                _ => "other",
            };
            Ok(json!({"name":name,"kind":kind}))
        })
        .collect()
}

fn temp_at(dir: &OwnedFd) -> rustix::io::Result<(OsString, File)> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    loop {
        let name = OsString::from(format!(
            ".conduit-write-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        match openat(
            dir,
            &name,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_bits_truncate(0o666),
        ) {
            Ok(fd) => return Ok((name, File::from(fd))),
            Err(Errno::EXIST) => continue,
            Err(error) => return Err(error),
        }
    }
}

pub fn write(root: &OwnedFd, path: &Path, content: &str) -> Result<(), String> {
    if content.len() > MAX_BYTES {
        return Err(error(path, "content exceeds 8 MiB"));
    }
    let name = path
        .file_name()
        .ok_or_else(|| error(path, "not a regular file"))?;
    let dir = walk(root, path.parent().unwrap(), true).map_err(|_| {
        error(
            path,
            "parent is inaccessible, is not a directory, or is a symbolic link",
        )
    })?;
    let previous = match statat(&dir, name, AtFlags::SYMLINK_NOFOLLOW) {
        Ok(stat) if regular(&stat) => Some(stat),
        Err(Errno::NOENT) => None,
        _ => {
            return Err(error(
                path,
                "cannot replace a non-regular file or symbolic link",
            ));
        }
    };
    let (temp, mut file) =
        temp_at(&dir).map_err(|_| error(path, "cannot create temporary file"))?;
    let result = (|| -> std::io::Result<()> {
        if let Some(stat) = &previous {
            fchmod(&file, Mode::from_bits_truncate(stat.st_mode & 0o777))?;
        }
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
        if let Some(previous) = previous {
            let current = statat(&dir, name, AtFlags::SYMLINK_NOFOLLOW)?;
            if !regular(&current)
                || (current.st_dev, current.st_ino) != (previous.st_dev, previous.st_ino)
            {
                return Err(Errno::BUSY.into());
            }
            renameat(&dir, &temp, &dir, name)?;
        } else {
            // A hard link publishes the complete new file only if the name is still unused.
            linkat(&dir, &temp, &dir, name, AtFlags::empty())?;
        }
        Ok(())
    })();
    let _ = unlinkat(&dir, &temp, AtFlags::empty());
    result.map_err(|_| error(path, "cannot publish complete file"))
}
