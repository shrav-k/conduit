//! Read only the local object database. Git itself sees only the imported data.
use crate::files;
use rustix::{fs::FileType, io::Errno};
use std::{
    collections::BTreeSet,
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read},
    os::{
        fd::OwnedFd,
        unix::{ffi::OsStrExt, fs::MetadataExt},
    },
    path::{Path, PathBuf},
};

pub fn import(source: &Path, destination: &Path) -> io::Result<()> {
    let (repository_path, repository) = repository_directory(source)?;
    let objects = match files::dir_at(&repository, OsStr::new("objects")) {
        Ok(objects) => (repository_path.join("objects"), objects),
        Err(Errno::NOENT) => {
            let common = repository_path.join(read_pointer(&repository, "commondir", b"")?);
            let directory = files::absolute(&common)?;
            (
                common.join("objects"),
                files::dir_at(&directory, OsStr::new("objects"))?,
            )
        }
        Err(error) => return Err(error.into()),
    };
    let mut pending = vec![objects];
    let mut visited = BTreeSet::new();
    while let Some((path, objects)) = pending.pop() {
        let metadata = File::from(objects.try_clone()?).metadata()?;
        if !visited.insert((metadata.dev(), metadata.ino())) {
            continue;
        }
        copy_objects(&objects, destination)?;
        for alternate in alternates(&objects)? {
            let alternate = path.join(alternate).canonicalize()?;
            pending.push((alternate.clone(), files::absolute(&alternate)?));
        }
    }
    Ok(())
}

fn alternates(objects: &OwnedFd) -> io::Result<Vec<PathBuf>> {
    let info = match files::dir_at(objects, OsStr::new("info")) {
        Ok(info) => info,
        Err(Errno::NOENT) => return Ok(Vec::new()),
        Err(error) => return Err(error.into()),
    };
    let alternates = match files::regular_at(&info, OsStr::new("alternates")) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut paths = Vec::new();
    for line in BufReader::new(alternates).split(b'\n') {
        let line = line?;
        if !line.is_empty() && !line.starts_with(b"#") {
            if line.contains(&0) {
                return Err(Errno::INVAL.into());
            }
            paths.push(PathBuf::from(OsStr::from_bytes(&line)));
        }
    }
    Ok(paths)
}

fn copy_objects(objects: &OwnedFd, destination: &Path) -> io::Result<()> {
    for (name, _) in files::entries(objects)? {
        let bytes = name.as_bytes();
        let packed = bytes == b"pack";
        if !packed && !hex_id(bytes, 2) {
            continue;
        }
        let directory = files::dir_at(objects, &name)?;
        let target = destination.join(&name);
        fs::create_dir_all(&target)?;
        for (entry, kind) in files::entries(&directory)? {
            let bytes = entry.as_bytes();
            let object = if packed {
                [b".pack".as_slice(), b".idx"].iter().any(|suffix| {
                    bytes
                        .strip_prefix(b"pack-")
                        .and_then(|b| b.strip_suffix(*suffix))
                        .is_some_and(|b| hex_id(b, 40))
                })
            } else {
                hex_id(bytes, 38)
            };
            if object {
                if kind != FileType::RegularFile {
                    return Err(Errno::INVAL.into());
                }
                let mut input = files::regular_at(&directory, &entry)?;
                match File::create_new(target.join(entry)) {
                    Ok(mut output) => {
                        io::copy(&mut input, &mut output)?;
                    }
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error),
                }
            }
        }
    }
    Ok(())
}

fn repository_directory(source: &Path) -> io::Result<(PathBuf, OwnedFd)> {
    let directory = files::absolute(source)?;
    match files::dir_at(&directory, OsStr::new(".git")) {
        Ok(git) => Ok((source.join(".git"), git)),
        Err(Errno::NOENT) => Ok((source.to_owned(), directory)),
        Err(Errno::NOTDIR) => {
            let path = source.join(read_pointer(&directory, ".git", b"gitdir: ")?);
            let git = files::absolute(&path)?;
            Ok((path, git))
        }
        Err(error) => Err(error.into()),
    }
}

fn read_pointer(directory: &OwnedFd, name: &str, prefix: &[u8]) -> io::Result<PathBuf> {
    let mut bytes = Vec::new();
    files::regular_at(directory, OsStr::new(name))?
        .take(4097)
        .read_to_end(&mut bytes)?;
    if bytes.len() > 4096 {
        return Err(Errno::INVAL.into());
    }
    let path = bytes.strip_prefix(prefix).ok_or(Errno::INVAL)?;
    let path = path.strip_suffix(b"\n").unwrap_or(path);
    let path = path.strip_suffix(b"\r").unwrap_or(path);
    if path.is_empty() || path.contains(&0) {
        return Err(Errno::INVAL.into());
    }
    Ok(PathBuf::from(OsStr::from_bytes(path)))
}

fn hex_id(bytes: &[u8], length: usize) -> bool {
    bytes.len() == length
        && bytes
            .iter()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(b))
}
