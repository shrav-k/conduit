use core_foundation::base::TCFType;
use security_framework::os::macos::keychain::SecKeychain;
use std::{
    ffi::c_void,
    fs::{self, Metadata},
    io::ErrorKind,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Mutex,
    time::Duration,
};

#[link(name = "Security", kind = "framework")]
unsafe extern "C" {
    fn SecKeychainGetStatus(keychain: *mut c_void, status: *mut u32) -> i32;
}

const UNLOCK_STATE: u32 = 1;
const ITEM_NOT_FOUND: i32 = -25300;
static INTERACTION_STATE: Mutex<()> = Mutex::new(());
const DEDICATED: &str = "keychain store could not be read: only a dedicated keychain is allowed";
const DEFAULT_CHECK: &str =
    "keychain store could not be read: the default keychain path could not be verified";

type FileIdentity = (u64, u64);

fn file_identity(metadata: &Metadata) -> FileIdentity {
    (metadata.dev(), metadata.ino())
}

fn is_protected(
    candidate: &Path,
    default: &Path,
    identities: Option<(FileIdentity, FileIdentity)>,
) -> bool {
    candidate == default || identities.is_some_and(|(candidate, default)| candidate == default)
}

fn is_login_name(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        let bytes = name.as_encoded_bytes();
        bytes.eq_ignore_ascii_case(b"login.keychain")
            || bytes.eq_ignore_ascii_case(b"login.keychain-db")
    })
}

async fn default_path() -> Result<PathBuf, &'static str> {
    let output = tokio::time::timeout(
        Duration::from_secs(30),
        tokio::process::Command::new("/usr/bin/security")
            .args(["default-keychain", "-d", "user"])
            .env_clear()
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .output(),
    )
    .await
    .map_err(|_| DEFAULT_CHECK)?
    .map_err(|_| DEFAULT_CHECK)?;
    if !output.status.success() {
        return Err(DEFAULT_CHECK);
    }
    // Refuse an ambiguous or malformed path instead of guessing at CLI escaping.
    let text = std::str::from_utf8(&output.stdout).map_err(|_| DEFAULT_CHECK)?;
    let path = text
        .trim()
        .strip_prefix('"')
        .and_then(|text| text.strip_suffix('"'))
        .filter(|text| !text.contains(['"', '\\', '\n', '\r', '\0']))
        .map(Path::new)
        .filter(|path| path.is_absolute())
        .ok_or(DEFAULT_CHECK)?;
    Ok(path.to_owned())
}

async fn dedicated_path(path: &Path) -> Result<(PathBuf, FileIdentity), &'static str> {
    if is_login_name(path) {
        return Err(DEDICATED);
    }
    let default = default_path().await?;
    if is_protected(path, &default, None) {
        return Err(DEDICATED);
    }
    let path = match path.canonicalize() {
        Ok(path) => path,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            return Err("keychain store could not be read: keychain file is missing");
        }
        Err(_) => {
            return Err("keychain store could not be read: keychain file could not be inspected");
        }
    };
    // Resolve the parent first so the path check also works with a missing database.
    let default_in_canonical_parent = default
        .parent()
        .ok_or(DEFAULT_CHECK)?
        .canonicalize()
        .map_err(|_| DEFAULT_CHECK)?
        .join(default.file_name().ok_or(DEFAULT_CHECK)?);
    if is_login_name(&path) || is_protected(&path, &default_in_canonical_parent, None) {
        return Err(DEDICATED);
    }
    let metadata = path
        .metadata()
        .map_err(|_| "keychain store could not be read: keychain file could not be inspected")?;
    if !metadata.is_file() {
        return Err("keychain store could not be read: path is not a regular file");
    }
    // A hard link would hide the file's other name from canonicalize.
    if metadata.nlink() != 1 {
        return Err(DEDICATED);
    }
    let identity = file_identity(&metadata);
    // Metadata follows a symlink at the reported name without opening the keychain.
    let identities = match fs::metadata(&default) {
        Ok(metadata) => Some((identity, file_identity(&metadata))),
        Err(error) if error.kind() == ErrorKind::NotFound => None,
        Err(_) => return Err(DEFAULT_CHECK),
    };
    if is_protected(&path, &default, identities) {
        return Err(DEDICATED);
    }
    Ok((path, identity))
}

pub(super) async fn read(
    path: &Path,
    service: &str,
    account: &str,
) -> Result<String, &'static str> {
    let (path, identity) = dedicated_path(path).await?;
    const INTERACTION: &str =
        "keychain store could not be read: user interaction could not be disabled";
    // Dropping the framework guard enables interaction for the whole process.
    let _exclusive = INTERACTION_STATE.lock().map_err(|_| INTERACTION)?;
    let _interaction = SecKeychain::disable_user_interaction().map_err(|_| INTERACTION)?;
    if !matches!(SecKeychain::user_interaction_allowed(), Ok(false)) {
        return Err(INTERACTION);
    }
    let keychain = SecKeychain::open(&path)
        .map_err(|_| "keychain store could not be read: keychain file could not be opened")?;
    let (password, _) = keychain
        .find_generic_password(service, account)
        .map_err(|error| {
            let mut status = 0;
            let result =
                unsafe { SecKeychainGetStatus(keychain.as_concrete_TypeRef().cast(), &mut status) };
            if result == 0 && status & UNLOCK_STATE == 0 {
                "keychain store could not be read: keychain is locked"
            } else if error.code() == ITEM_NOT_FOUND {
                "keychain store could not be read: generic password item is missing"
            } else {
                "keychain store could not be read: access to the item was refused"
            }
        })?;
    if !fs::metadata(&path).is_ok_and(|metadata| file_identity(&metadata) == identity) {
        return Err(DEDICATED);
    }
    let bytes = password.as_ref();
    if bytes.len() > crate::MAX_VALUE_BYTES {
        return Err("keychain store could not be read: value exceeds 65536 bytes");
    }
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| "keychain store could not be read: value is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn protects_the_default_path_and_its_file_identity() {
        let directory = tempfile::tempdir().unwrap();
        let default = directory.path().join("default.keychain-db");
        fs::write(&default, []).unwrap();
        assert!(is_protected(&default, &default, None));

        let symlink_path = directory.path().join("reported.keychain-db");
        symlink(&default, &symlink_path).unwrap();
        let hard_link = directory.path().join("hard-link.keychain-db");
        fs::hard_link(&default, &hard_link).unwrap();
        let case_variant = directory.path().join("DEFAULT.KEYCHAIN-DB");
        if !case_variant.exists() {
            // On case-sensitive volumes, give the spelling the same identity explicitly.
            symlink(&default, &case_variant).unwrap();
        }

        let identity = |path: &Path| file_identity(&fs::metadata(path).unwrap());
        for alias in [&symlink_path, &hard_link, &case_variant] {
            assert_ne!(alias, &default);
            let identities = Some((identity(alias), identity(&default)));
            assert!(is_protected(alias, &default, identities));
            assert!(is_protected(&default, alias, identities));
        }

        let other = directory.path().join("dedicated.keychain-db");
        fs::write(&other, []).unwrap();
        assert!(!is_protected(
            &other,
            &default,
            Some((identity(&other), identity(&default)))
        ));
        assert!(!is_protected(&other, &default, None));
        let (device, inode) = identity(&default);
        assert!(!is_protected(
            &other,
            &default,
            Some(((device.wrapping_add(1), inode), (device, inode)))
        ));
    }
}
