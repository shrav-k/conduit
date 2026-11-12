//! DL13: fixed HTTPS audience and loopback HTTP listener configuration; no TLS here.
use crate::{Config, Result, store};
use base64::{Engine, engine::general_purpose::STANDARD};
use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
};

fn address(value: &str) -> Result<url::Url> {
    // IFC-042: the audience is the canonical HTTPS URI, never a loopback receiver.
    let url = url::Url::parse(value).map_err(|_| "--https-url must be a valid URL")?;
    // IFC-042: this receiver's audience uses HTTPS.
    if url.scheme() != "https" {
        return Err("--https-url must use scheme https".into());
    }
    // IFC-042: the canonical receiver URI carries no user information.
    if !url.username().is_empty() || url.password().is_some() {
        return Err("--https-url must have no user name or password".into());
    }
    // IFC-042: the fixed receiver has no query or fragment.
    if url.query().is_some() || url.fragment().is_some() {
        return Err("--https-url must have no query or fragment".into());
    }
    // DL13: the second address serves the MCP endpoint only.
    if url.path() != "/mcp" {
        return Err("--https-url must have path exactly /mcp".into());
    }
    // DL13: this address is distinct from the own-runner loopback address.
    let loopback = match url.host() {
        Some(url::Host::Domain(host)) => host == "localhost",
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => {
            ip.is_loopback() || ip.to_ipv4_mapped().is_some_and(|ip| ip.is_loopback())
        }
        None => return Err("--https-url must have a host".into()),
    };
    if loopback {
        return Err("--https-url must not name localhost or a loopback IP address".into());
    }
    // DL13: the receiver's address has a usable, fixed port.
    if url.port() == Some(0) {
        return Err("--https-url must have a nonzero port".into());
    }
    // IFC-042: use exactly the url crate's canonical serialization as audience.
    if url.as_str() != value {
        return Err(format!(
            "--https-url must be canonical; canonical form is {url}"
        ));
    }
    Ok(url)
}

/// PRV-022/DL13: check PEM labels and base64 only, without parsing certificates.
pub(crate) fn roots(content: &str) -> Result<String> {
    let mut remaining = content.trim();
    let mut normalized = String::new();
    while !remaining.is_empty() {
        let body = remaining
            .strip_prefix("-----BEGIN ")
            .ok_or("--https-roots must contain only CERTIFICATE PEM blocks and whitespace")?;
        let (label, body) = body
            .split_once("-----")
            .ok_or("--https-roots has an incomplete PEM label")?;
        // PRV-022/DL13: an incomplete header must not turn a key's body into a label diagnostic.
        if label.contains(['\n', '\r']) {
            return Err("--https-roots has an incomplete PEM label".into());
        }
        // PRV-022/DL13: accept public certificate roots only, never keys.
        if label != "CERTIFICATE" {
            return Err(format!(
                "--https-roots refuses PEM label {label}; expected CERTIFICATE"
            ));
        }
        let (encoded, tail) = body
            .split_once("-----END CERTIFICATE-----")
            .ok_or("--https-roots has an incomplete CERTIFICATE PEM block")?;
        let encoded: String = encoded
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        // PRV-022: every root block must contain decodable certificate bytes.
        if encoded.is_empty() || STANDARD.decode(&encoded).is_err() {
            return Err("--https-roots CERTIFICATE block must contain decodable base64".into());
        }
        normalized.push_str("-----BEGIN CERTIFICATE-----\n");
        for line in encoded.as_bytes().chunks(64) {
            normalized.push_str(std::str::from_utf8(line).expect("validated base64 is ASCII"));
            normalized.push('\n');
        }
        normalized.push_str("-----END CERTIFICATE-----\n");
        remaining = tail.trim();
    }
    // PRV-022: an explicitly supplied bundle cannot contain zero roots.
    if normalized.is_empty() {
        return Err("--https-roots must contain at least one CERTIFICATE PEM block".into());
    }
    Ok(normalized)
}

impl Config {
    /// DL13: read public fixed configuration without opening or recovering the store.
    pub(crate) fn read_https(&mut self, directory: &Path) -> Result<()> {
        let path = directory.join("engine-https-url.json");
        if path.try_exists().map_err(|e| e.to_string())? {
            self.engine_https_url = Some(
                serde_json::from_slice(&store::private_read(&path)?)
                    .map_err(|_| "invalid stored engine https URL")?,
            );
        }
        // PRV-022/DL13: the stored copy is a checked public root bundle.
        let path = directory.join("engine-https-roots.pem");
        if path.try_exists().map_err(|e| e.to_string())? {
            self.https_roots = Some(
                String::from_utf8(store::private_read(&path)?)
                    .map_err(|_| "invalid stored engine https roots")?,
            );
        }
        self.check_https()
    }

    fn check_https(&self) -> Result<()> {
        if let Some(value) = &self.engine_https_url {
            address(value)?;
        }
        if let Some(content) = &self.https_roots {
            roots(content)?;
        }
        // DL13: extra roots and the extra listener require the HTTPS audience.
        if self.engine_https_url.is_none() {
            if self.https_listen.is_some() {
                return Err("--https-listen requires --https-url (no fixed https address)".into());
            }
            if self.https_roots.is_some() {
                return Err("--https-roots requires --https-url (no fixed https address)".into());
            }
        }
        if let Some(listen) = self.https_listen {
            // DL13: bind exactly one of the two loopback addresses, with a fixed port.
            if !matches!(listen.ip(), IpAddr::V4(ip) if ip == Ipv4Addr::LOCALHOST)
                && listen.ip() != IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)
            {
                return Err("--https-listen must use 127.0.0.1 or ::1".into());
            }
            // DL13: the listener port is fixed for this serve invocation.
            if listen.port() == 0 {
                return Err("--https-listen must have a nonzero port".into());
            }
            let url = url::Url::parse(&self.engine_url).map_err(|_| "invalid engine URL")?;
            let ip = match url.host() {
                Some(url::Host::Domain("localhost")) => Some(IpAddr::V4(Ipv4Addr::LOCALHOST)),
                Some(url::Host::Ipv4(ip)) => Some(IpAddr::V4(ip)),
                Some(url::Host::Ipv6(ip)) => Some(IpAddr::V6(ip)),
                _ => None,
            };
            // DL13/IFC-042: separate listeners keep the two audiences apart.
            if ip == Some(listen.ip()) && url.port_or_known_default() == Some(listen.port()) {
                return Err("--https-listen must differ from the first listener's address".into());
            }
        }
        Ok(())
    }

    /// DL13: preflight serve options before opening (and writing) the store.
    pub fn validate_serve(&self) -> Result<()> {
        self.check_https()?;
        if let Some(address) = &self.engine_https_url
            && self.https_listen.is_none()
        {
            return Err(format!(
                "--https-listen is required for https address {address} (given or fixed in the store)"
            ));
        }
        Ok(())
    }

    pub(crate) fn fix_https(&mut self, directory: &Path) -> Result<()> {
        let supplied_roots = self.https_roots.is_some();
        // IFC-042/PRV-022: reject malformed supplied values before comparing fixed ones.
        if let Some(value) = &self.engine_https_url {
            address(value)?;
        }
        if let Some(content) = &self.https_roots {
            roots(content)?;
        }
        let mut fixed = Self::default();
        fixed.read_https(directory)?;
        // IFC-042: a store fixes its HTTPS token audience for its lifetime.
        if let (Some(old), Some(new)) = (&fixed.engine_https_url, &self.engine_https_url)
            && old != new
        {
            return Err(format!(
                "https address {new} differs from fixed address {old}; the store fixes the https token audience"
            ));
        }
        if self.engine_https_url.is_none() {
            self.engine_https_url = fixed.engine_https_url.clone();
        }
        if self.https_roots.is_none() {
            self.https_roots = fixed.https_roots.clone();
        }
        self.check_https()?;
        if self.engine_https_url != fixed.engine_https_url {
            store::atomic_write(
                directory,
                "engine-https-url.json",
                &serde_json::to_vec(&self.engine_https_url).map_err(|e| e.to_string())?,
            )?;
        }
        // PRV-022/DL13: an explicitly supplied bundle replaces the stored copy.
        if supplied_roots && let Some(content) = &self.https_roots {
            store::atomic_write(directory, "engine-https-roots.pem", content.as_bytes())?;
        }
        Ok(())
    }
}
