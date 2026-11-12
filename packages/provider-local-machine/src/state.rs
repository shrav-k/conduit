use crate::KEY_FILE;
use anyhow::{Context, Result, bail, ensure};
use ring::{
    rand::{SecureRandom, SystemRandom},
    signature::{Ed25519KeyPair, KeyPair},
};
use rustix::fs::{CWD, Mode, OFlags, openat};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirBuilder, File},
    io::{Read, Write},
    os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};

pub struct State {
    directory: File,
    path: PathBuf,
}
impl State {
    pub fn open(path: &Path) -> Result<Self> {
        match DirBuilder::new().mode(0o700).create(path) {
            Ok(()) => (),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(_) => bail!("cannot create provider state directory"),
        }
        let directory = File::from(
            openat(
                CWD,
                path,
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .context("cannot open provider state directory")?,
        );
        ensure!(
            directory.metadata()?.uid() == rustix::process::geteuid().as_raw(),
            "state directory must belong to the provider's OS user"
        );
        directory.set_permissions(fs::Permissions::from_mode(0o700))?;
        directory
            .sync_all()
            .context("cannot fsync provider state directory")?;
        // Persist the directory's own entry too, including when a concurrent initializer
        // created it. No key/ID is exposed before both directory levels are durable.
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        File::open(parent)
            .context("cannot open state directory parent")?
            .sync_all()
            .context("cannot fsync state directory parent")?;
        Ok(Self {
            directory,
            path: path.into(),
        })
    }
    fn read_key(&self) -> Result<Option<[u8; 32]>> {
        let descriptor = match openat(
            &self.directory,
            KEY_FILE,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
            Mode::empty(),
        ) {
            Ok(file) => file,
            Err(rustix::io::Errno::NOENT) => return Ok(None),
            Err(_) => bail!("cannot safely open existing machine key; refusing to replace it"),
        };
        let mut file = File::from(descriptor);
        let metadata = file.metadata()?;
        ensure!(
            metadata.is_file()
                && metadata.uid() == rustix::process::geteuid().as_raw()
                && metadata.permissions().mode() & 0o7777 == 0o600
                && metadata.len() == 32,
            "machine key must be a 32-byte regular file owned by the provider's user at mode 0600; refusing to replace it"
        );
        let mut seed = [0u8; 32];
        file.read_exact(&mut seed)
            .context("cannot read machine key")?;
        // Also covers a concurrent first writer: do not serve an ID until its directory entry
        // is durable, even if that writer has not reached its own fsync yet.
        self.directory
            .sync_all()
            .context("cannot fsync machine key directory")?;
        Ok(Some(seed))
    }
    pub fn id(&self) -> Result<String> {
        if self.read_key()?.is_none() {
            let mut seed = [0u8; 32];
            SystemRandom::new()
                .fill(&mut seed)
                .map_err(|_| anyhow::anyhow!("OS random generation failed"))?;
            let mut temporary = tempfile::Builder::new()
                .prefix(".machine-key-")
                .tempfile_in(&self.path)
                .context("cannot create temporary machine key")?;
            temporary
                .as_file()
                .set_permissions(fs::Permissions::from_mode(0o600))?;
            temporary
                .write_all(&seed)
                .context("cannot write machine key")?;
            temporary
                .as_file()
                .sync_all()
                .context("cannot fsync machine key")?;
            let name = temporary.path().file_name().unwrap();
            #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
            match rustix::fs::renameat_with(
                &self.directory,
                name,
                &self.directory,
                KEY_FILE,
                rustix::fs::RenameFlags::NOREPLACE,
            ) {
                Ok(()) | Err(rustix::io::Errno::EXIST) => (),
                Err(_) => {
                    bail!("atomic non-replacing machine-key rename failed; existing key preserved")
                }
            }
            #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "ios")))]
            {
                let _ = name;
                bail!("this Unix platform lacks the supported non-replacing rename operation");
            }
            self.directory
                .sync_all()
                .context("cannot fsync machine key directory")?;
            // A concurrent initializer may have won; always use the key actually installed.
        }
        let seed = self
            .read_key()?
            .context("machine key disappeared; refusing to regenerate it")?;
        let key = Ed25519KeyPair::from_seed_unchecked(&seed)
            .map_err(|_| anyhow::anyhow!("invalid Ed25519 seed"))?;
        Ok(format!(
            "sha256:{:x}",
            Sha256::digest(key.public_key().as_ref())
        ))
    }
}
