//! Unix local-process mechanisms for the runtime-provider kit.
#![cfg(unix)]
mod backend;
mod guard;
mod storage;
pub use backend::{LocalBackend, Signaller, Staged, StopPipe, hostname};
pub use guard::guard_main;
pub use storage::{FileStore, instance_key, private_directory};

use runtime_provider_kit::{Options, Unavailable, gate::GateConfig};
use std::time::Duration;

pub const STOP_GRACE: Duration = Duration::from_millis(200);
pub const DRAIN_INTERVAL: Duration = Duration::from_secs(1);
pub const BINARY: &str = "provider-local-process";

/// Configuration is captured at provider startup, never refreshed from a call.
pub fn configuration() -> Result<(GateConfig, Options), Unavailable> {
    fn value(name: &str, default: &str) -> Result<String, Unavailable> {
        match std::env::var(name) {
            Ok(value) => Ok(value),
            Err(std::env::VarError::NotPresent) => Ok(default.into()),
            Err(_) => Err(Unavailable),
        }
    }
    fn seconds(name: &str, default: &str) -> Result<Duration, Unavailable> {
        let n: u64 = value(name, default)?.parse().map_err(|_| Unavailable)?;
        if n == 0 || n > i32::MAX as u64 {
            return Err(Unavailable);
        }
        Ok(Duration::from_secs(n))
    }
    let gate = GateConfig {
        bind_address: value("LOCAL_PROCESS_BIND_ADDRESS", "127.0.0.1")?
            .parse()
            .map_err(|_| Unavailable)?,
        advertised_host: value("LOCAL_PROCESS_ADVERTISED_HOST", "127.0.0.1")?,
        keepalive_idle: seconds("LOCAL_PROCESS_KEEPALIVE_IDLE", "30")?,
        keepalive_interval: seconds("LOCAL_PROCESS_KEEPALIVE_INTERVAL", "5")?,
        keepalive_count: value("LOCAL_PROCESS_KEEPALIVE_COUNT", "3")?
            .parse()
            .map_err(|_| Unavailable)?,
        ..GateConfig::default()
    };
    if gate.bind_address.is_unspecified()
        || gate.keepalive_count == 0
        || gate.keepalive_count > i32::MAX as u32
        || gate.silent_cut_bound().as_millis() > i32::MAX as u128
        || gate.advertised_host.is_empty()
        || (gate.advertised_host.parse::<std::net::IpAddr>().is_err()
            && (!gate.advertised_host.is_ascii()
                || gate.advertised_host.split('.').any(|part| {
                    part.is_empty()
                        || part.starts_with('-')
                        || part.ends_with('-')
                        || !part.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
                })))
    {
        return Err(Unavailable);
    }
    Ok((
        gate,
        Options {
            max_lifetime: seconds("LOCAL_PROCESS_MAX_LIFETIME", "3600")?,
            drain_interval: DRAIN_INTERVAL,
        },
    ))
}
