//! SSH/systemd backend for the shared runtime-provider kit. Unix provider host required.
#![cfg(unix)]
mod backend;
pub mod codec;
pub mod config;
pub mod ssh;
mod storage;
pub use backend::{SshBackend, Staged, parse_state};
pub use config::Config;
pub use storage::{FileStore, instance_key};
pub const BINARY: &str = "provider-ssh-runtime";
