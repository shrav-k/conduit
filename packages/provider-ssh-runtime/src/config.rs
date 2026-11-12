use crate::codec::option_path;
use base64::{Engine, engine::general_purpose::STANDARD};
use runtime_provider_kit::{Options, gate::GateConfig};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf, time::Duration};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MachineConfig {
    pub hostname: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    /// The public key only, as `ssh-ed25519 <base64>`, without a comment.
    pub host_key: String,
}
#[derive(Clone)]
pub struct Config {
    pub machines: Vec<MachineConfig>,
    pub private_key: PathBuf,
    pub durable_dir: PathBuf,
    pub base: String,
    pub run_path: String,
    pub max_lifetime: Duration,
    pub run_limit: usize,
    pub stop_timeout: Duration,
    pub drain_interval: Duration,
    pub systemd_run: String,
    pub systemctl: String,
    pub loginctl: String,
    pub prefix: String,
    pub gate: GateConfig,
}
pub fn safe_path(s: &str) -> bool {
    s.starts_with('/')
        && s.len() > 1
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"/._-".contains(&c))
        && !s.split('/').any(|p| matches!(p, "." | ".."))
}
fn host(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b".:-_".contains(&c))
}
impl Config {
    /// Only read at startup. Missing required places name their environment binding.
    /// Optional tool paths and prefix have the work order's defaults.
    pub fn from_lookup(mut get: impl FnMut(&str) -> Option<String>) -> Result<Self, &'static str> {
        fn required(
            get: &mut impl FnMut(&str) -> Option<String>,
            name: &'static str,
        ) -> Result<String, &'static str> {
            get(name).filter(|s| !s.is_empty()).ok_or(name)
        }
        fn seconds(
            get: &mut impl FnMut(&str) -> Option<String>,
            name: &'static str,
        ) -> Result<Duration, &'static str> {
            let n: u64 = required(get, name)?.parse().map_err(|_| name)?;
            if n == 0 || n > i32::MAX as u64 {
                return Err(name);
            }
            Ok(Duration::from_secs(n))
        }
        let machines = serde_json::from_str(&required(&mut get, "SSH_RUNTIME_MACHINES")?)
            .map_err(|_| "SSH_RUNTIME_MACHINES")?;
        let mut config = Self {
            machines,
            durable_dir: required(&mut get, "SSH_RUNTIME_DURABLE_DIR")?.into(),
            private_key: required(&mut get, "SSH_RUNTIME_PRIVATE_KEY")?.into(),
            base: required(&mut get, "SSH_RUNTIME_BASE")?,
            run_path: required(&mut get, "SSH_RUNTIME_PATH")?,
            run_limit: required(&mut get, "SSH_RUNTIME_RUN_LIMIT")?
                .parse()
                .map_err(|_| "SSH_RUNTIME_RUN_LIMIT")?,
            max_lifetime: seconds(&mut get, "SSH_RUNTIME_MAX_LIFETIME")?,
            stop_timeout: seconds(&mut get, "SSH_RUNTIME_STOP_TIMEOUT")?,
            drain_interval: seconds(&mut get, "SSH_RUNTIME_DRAIN_INTERVAL")?,
            systemd_run: get("SSH_RUNTIME_SYSTEMD_RUN")
                .unwrap_or_else(|| "/usr/bin/systemd-run".into()),
            systemctl: get("SSH_RUNTIME_SYSTEMCTL").unwrap_or_else(|| "/usr/bin/systemctl".into()),
            loginctl: get("SSH_RUNTIME_LOGINCTL").unwrap_or_else(|| "/usr/bin/loginctl".into()),
            prefix: get("SSH_RUNTIME_UNIT_PREFIX").unwrap_or_else(|| "conduit-run-".into()),
            gate: GateConfig {
                bind_address: required(&mut get, "SSH_RUNTIME_BIND_ADDRESS")?
                    .parse()
                    .map_err(|_| "SSH_RUNTIME_BIND_ADDRESS")?,
                advertised_host: required(&mut get, "SSH_RUNTIME_ADVERTISED_HOST")?,
                keepalive_idle: seconds(&mut get, "SSH_RUNTIME_KEEPALIVE_IDLE")?,
                keepalive_interval: seconds(&mut get, "SSH_RUNTIME_KEEPALIVE_INTERVAL")?,
                keepalive_count: required(&mut get, "SSH_RUNTIME_KEEPALIVE_COUNT")?
                    .parse()
                    .map_err(|_| "SSH_RUNTIME_KEEPALIVE_COUNT")?,
                ..GateConfig::default()
            },
        };
        config.base = config.base.trim_end_matches('/').into();
        config.validate()?;
        Ok(config)
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.run_limit == 0 || self.run_limit > 62 {
            return Err("SSH_RUNTIME_RUN_LIMIT");
        }
        let mut names = BTreeSet::new();
        if self.machines.is_empty()
            || self.machines.iter().any(|m| {
                let key = m
                    .host_key
                    .strip_prefix("ssh-ed25519 ")
                    .and_then(|s| STANDARD.decode(s).ok());
                !host(&m.hostname)
                    || !host(&m.host)
                    || m.port == 0
                    || m.user != "conduit-run"
                    || !names.insert(&m.hostname)
                    || !key.is_some_and(|b| {
                        b.len() == 51 && b[..19] == *b"\0\0\0\x0bssh-ed25519\0\0\0\x20"
                    })
            })
        {
            return Err("SSH_RUNTIME_MACHINES");
        }
        if option_path(self.durable_dir.to_str().ok_or("SSH_RUNTIME_DURABLE_DIR")?).is_err() {
            return Err("SSH_RUNTIME_DURABLE_DIR");
        }
        if option_path(self.private_key.to_str().ok_or("SSH_RUNTIME_PRIVATE_KEY")?).is_err() {
            return Err("SSH_RUNTIME_PRIVATE_KEY");
        }
        if !safe_path(&self.base) {
            return Err("SSH_RUNTIME_BASE");
        }
        if self.run_path.is_empty()
            || crate::codec::environment(&std::collections::BTreeMap::from([(
                "PATH".into(),
                self.run_path.clone(),
            )]))
            .is_err()
        {
            return Err("SSH_RUNTIME_PATH");
        }
        for (path, name) in [
            (&self.systemd_run, "SSH_RUNTIME_SYSTEMD_RUN"),
            (&self.systemctl, "SSH_RUNTIME_SYSTEMCTL"),
            (&self.loginctl, "SSH_RUNTIME_LOGINCTL"),
        ] {
            if !path.starts_with('/') || path.contains('\0') {
                return Err(name);
            }
        }
        if self.prefix.is_empty()
            || self.prefix.len() > 80
            || !self
                .prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err("SSH_RUNTIME_UNIT_PREFIX");
        }
        for (n, name) in [
            (self.max_lifetime, "SSH_RUNTIME_MAX_LIFETIME"),
            (self.stop_timeout, "SSH_RUNTIME_STOP_TIMEOUT"),
            (self.drain_interval, "SSH_RUNTIME_DRAIN_INTERVAL"),
        ] {
            if n.is_zero() || n.subsec_nanos() != 0 || n.as_secs() > i32::MAX as u64 {
                return Err(name);
            }
        }
        if self.gate.bind_address.is_unspecified() {
            return Err("SSH_RUNTIME_BIND_ADDRESS");
        }
        if !host(&self.gate.advertised_host) {
            return Err("SSH_RUNTIME_ADVERTISED_HOST");
        }
        if self.gate.keepalive_count == 0 || self.gate.keepalive_count > i32::MAX as u32 {
            return Err("SSH_RUNTIME_KEEPALIVE_COUNT");
        }
        for (n, name) in [
            (self.gate.keepalive_idle, "SSH_RUNTIME_KEEPALIVE_IDLE"),
            (
                self.gate.keepalive_interval,
                "SSH_RUNTIME_KEEPALIVE_INTERVAL",
            ),
        ] {
            if n.is_zero() || n.subsec_nanos() != 0 || n.as_secs() > i32::MAX as u64 {
                return Err(name);
            }
        }
        if self.gate.silent_cut_bound().as_millis() > i32::MAX as u128 {
            return Err("SSH_RUNTIME_KEEPALIVE_COUNT");
        }
        Ok(())
    }
    pub fn options(&self) -> Options {
        Options {
            max_lifetime: self.max_lifetime,
            drain_interval: self.drain_interval,
        }
    }
}

/// The file holds precisely the schema's configuration places, never process inputs.
const PLACES: &[(&str, &str)] = &[
    ("advertisedHost", "SSH_RUNTIME_ADVERTISED_HOST"),
    ("base", "SSH_RUNTIME_BASE"),
    ("bindAddress", "SSH_RUNTIME_BIND_ADDRESS"),
    ("drainInterval", "SSH_RUNTIME_DRAIN_INTERVAL"),
    ("durableDir", "SSH_RUNTIME_DURABLE_DIR"),
    ("keepaliveCount", "SSH_RUNTIME_KEEPALIVE_COUNT"),
    ("keepaliveIdle", "SSH_RUNTIME_KEEPALIVE_IDLE"),
    ("keepaliveInterval", "SSH_RUNTIME_KEEPALIVE_INTERVAL"),
    ("loginctl", "SSH_RUNTIME_LOGINCTL"),
    ("machines", "SSH_RUNTIME_MACHINES"),
    ("maxLifetime", "SSH_RUNTIME_MAX_LIFETIME"),
    ("path", "SSH_RUNTIME_PATH"),
    ("privateKey", "SSH_RUNTIME_PRIVATE_KEY"),
    ("runLimit", "SSH_RUNTIME_RUN_LIMIT"),
    ("stopTimeout", "SSH_RUNTIME_STOP_TIMEOUT"),
    ("systemctl", "SSH_RUNTIME_SYSTEMCTL"),
    ("systemdRun", "SSH_RUNTIME_SYSTEMD_RUN"),
    ("unitPrefix", "SSH_RUNTIME_UNIT_PREFIX"),
];

impl Config {
    pub fn write_drainer(&self, root: &std::path::Path) -> std::io::Result<()> {
        self.validate().map_err(std::io::Error::other)?;
        let places = serde_json::json!({
            "advertisedHost": self.gate.advertised_host,
            "base": self.base,
            "bindAddress": self.gate.bind_address.to_string(),
            "drainInterval": self.drain_interval.as_secs(),
            "durableDir": self.durable_dir,
            "keepaliveCount": self.gate.keepalive_count,
            "keepaliveIdle": self.gate.keepalive_idle.as_secs(),
            "keepaliveInterval": self.gate.keepalive_interval.as_secs(),
            "loginctl": self.loginctl,
            "machines": serde_json::to_string(&self.machines)?,
            "maxLifetime": self.max_lifetime.as_secs(),
            "path": self.run_path,
            "privateKey": self.private_key,
            "runLimit": self.run_limit,
            "stopTimeout": self.stop_timeout.as_secs(),
            "systemctl": self.systemctl,
            "systemdRun": self.systemd_run,
            "unitPrefix": self.prefix,
        });
        crate::storage::atomic_write(root, "drainer-config", &serde_json::to_vec(&places)?)
    }

    pub fn read_drainer(root: &std::path::Path) -> std::io::Result<Self> {
        use serde_json::Value;
        let invalid = || std::io::Error::other("invalid drainer-config");
        let places: serde_json::Map<String, Value> =
            serde_json::from_slice(&std::fs::read(root.join("drainer-config"))?)
                .map_err(|_| invalid())?;
        if places.len() != PLACES.len() {
            return Err(invalid());
        }
        let mut bindings = std::collections::BTreeMap::new();
        for &(place, binding) in PLACES {
            let value = places.get(place).ok_or_else(invalid)?;
            let integer = matches!(
                place,
                "drainInterval"
                    | "keepaliveCount"
                    | "keepaliveIdle"
                    | "keepaliveInterval"
                    | "maxLifetime"
                    | "runLimit"
                    | "stopTimeout"
            );
            let text = if integer {
                value.as_u64().ok_or_else(invalid)?.to_string()
            } else {
                value.as_str().ok_or_else(invalid)?.to_owned()
            };
            bindings.insert(binding, text);
        }
        Self::from_lookup(|name| bindings.remove(name)).map_err(|_| invalid())
    }
}
