//! Per-user CLI files, independent of the project and engine store.

use crate::{Config, Result, author, local_engine_url, store};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, DirBuilder},
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
};

const CONTEXT_FILE: &str = "context.json";

pub fn escape_line_breaks(text: &str) -> String {
    text.replace('\n', "\\n").replace('\r', "\\r")
}

pub fn directory() -> Result<PathBuf> {
    if let Some(value) = std::env::var_os("CONDUIT_CONFIG_DIR").filter(|value| !value.is_empty()) {
        let directory = PathBuf::from(value);
        return if directory.is_absolute() {
            Ok(directory)
        } else {
            Err("CONDUIT_CONFIG_DIR must be absolute".into())
        };
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_default();
    if !home.is_absolute() {
        return Err(
            "HOME must be set to an absolute path when CONDUIT_CONFIG_DIR is absent or empty"
                .into(),
        );
    }
    Ok(home.join(".config/conduit"))
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineContext {
    pub store: PathBuf,
    pub url: String,
    pub version: String,
}

impl EngineContext {
    pub fn record(&self) -> Result<()> {
        let directory =
            directory().map_err(|error| format!("cannot locate {CONTEXT_FILE}: {error}"))?;
        let file = directory.join(CONTEXT_FILE);
        let write = || {
            let bytes = serde_json::to_vec(self).map_err(|error| error.to_string())?;
            write_private(&directory, CONTEXT_FILE, &bytes)
        };
        write().map_err(|error| format!("cannot write {file:?}: {error}"))
    }
}

/// Locate and verify the current serving store without opening it for recovery.
pub async fn current() -> Result<(PathBuf, EngineContext)> {
    let file = directory()
        .map_err(|error| format!("cannot locate {CONTEXT_FILE}: {error}"))?
        .join(CONTEXT_FILE);
    let bytes = fs::read(&file).map_err(|error| format!("cannot read {file:?}: {error}"))?;
    let invalid = || {
        format!(
            "{file:?} must contain exactly three strings named store, url and version, with an absolute store path"
        )
    };
    if bytes.iter().find(|byte| !byte.is_ascii_whitespace()) != Some(&b'{') {
        return Err(invalid());
    }
    let context: EngineContext = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
    if !context.store.is_absolute() {
        return Err(invalid());
    }
    let store_error = |cause: &str| format!("{file:?}: store {:?} {cause}", context.store);
    if !context.store.join("state.json").is_file() {
        return Err(store_error("is not initialized"));
    }
    let fixed = Config::for_context(&context.store)
        .map_err(|error| store_error(&format!("cannot be read: {error}")))?;
    if fixed.engine_url != context.url {
        return Err(store_error(
            "has a fixed URL that differs from the context URL",
        ));
    }
    let endpoint = local_engine_url(&context.url)
        .map_err(|error| store_error(&format!("has an invalid fixed URL: {error}")))?;
    author::require_serving(&context.store, &endpoint, true)
        .await
        .map_err(|error| {
            store_error(&format!(
                "has no serving engine at {} as its root: {error}",
                escape_line_breaks(&context.url)
            ))
        })?;
    Ok((file, context))
}

pub(crate) fn cache_archive(hex: &str, bytes: &[u8]) {
    let write = || {
        let packages = directory()
            .map_err(|error| format!("cannot locate packages/{hex}: {error}"))?
            .join("packages");
        write_private(&packages, hex, bytes)
            .map_err(|error| format!("cannot write {:?}: {error}", packages.join(hex)))
    };
    if let Err(error) = write() {
        eprintln!("conduit install: cache was not written: {error}");
    }
}

fn write_private(directory: &Path, name: &str, bytes: &[u8]) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(directory)
        .map_err(|error| error.to_string())?;
    store::atomic_write(directory, name, bytes)
}
