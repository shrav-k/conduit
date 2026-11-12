//! Identity is parsed in memory. All failures deliberately lose their source text.
use super::*;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use ring::signature::{ECDSA_P256_SHA256_FIXED, UnparsedPublicKey};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct Proof {
    engine: String,
    instance: String,
    keys: HashMap<String, Vec<u8>>,
}
impl Proof {
    pub(super) fn load(identity: &Identity) -> Result<(Self, rustls::ServerConfig)> {
        let file = |name: &str| identity.files.get(name).ok_or(Unavailable);
        let certs = CertificateDer::pem_slice_iter(file("tls.crt")?.as_bytes())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Unavailable)?;
        let key =
            PrivateKeyDer::from_pem_slice(file("tls.key")?.as_bytes()).map_err(|_| Unavailable)?;
        let claims = jwt_parts(file("token")?).ok_or(Unavailable)?.1;
        let instance = claims["sub"].as_str().ok_or(Unavailable)?.to_owned();
        if !instance.starts_with("spiffe://")
            || certificate_uri(certs.first().ok_or(Unavailable)?.as_ref())
                != Some(instance.as_str())
        {
            return Err(Unavailable);
        }
        let jwks: Value = serde_json::from_str(file("jwks.json")?).map_err(|_| Unavailable)?;
        let mut keys = HashMap::new();
        for jwk in jwks["keys"].as_array().ok_or(Unavailable)? {
            if jwk["kty"] != "EC"
                || jwk["crv"] != "P-256"
                || jwk.get("alg").is_some_and(|v| v != "ES256")
                || jwk.get("use").is_some_and(|v| v != "sig")
            {
                continue;
            }
            let kid = jwk["kid"].as_str().ok_or(Unavailable)?;
            let x = B64
                .decode(jwk["x"].as_str().ok_or(Unavailable)?)
                .map_err(|_| Unavailable)?;
            let y = B64
                .decode(jwk["y"].as_str().ok_or(Unavailable)?)
                .map_err(|_| Unavailable)?;
            if x.len() != 32 || y.len() != 32 {
                return Err(Unavailable);
            }
            let mut point = vec![4];
            point.extend(x);
            point.extend(y);
            if keys.insert(kid.to_owned(), point).is_some() {
                return Err(Unavailable);
            }
        }
        let engine = file("engine-id")?.trim().to_owned();
        if keys.is_empty() || engine.is_empty() {
            return Err(Unavailable);
        }
        let provider = Arc::new(rustls::crypto::ring::default_provider());
        let mut tls = rustls::ServerConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|_| Unavailable)?
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|_| Unavailable)?;
        tls.alpn_protocols = vec![b"http/1.1".to_vec()];
        Ok((
            Self {
                engine,
                instance,
                keys,
            },
            tls,
        ))
    }
    pub(super) fn admits(&self, headers: &hyper::HeaderMap) -> bool {
        // Duplicate authorization fields are ambiguous and are never admitted.
        if headers.get_all(hyper::header::AUTHORIZATION).iter().count() != 1 {
            return false;
        }
        let Some(token) = headers
            .get(hyper::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
        else {
            return false;
        };
        self.verify(token).is_some()
    }
    fn verify(&self, token: &str) -> Option<()> {
        let (header, claims, signed, signature) = jwt_parts(token)?;
        if header["alg"] != "ES256" || header.get("crit").is_some() {
            return None;
        }
        let key = self.keys.get(header["kid"].as_str()?)?;
        UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, key)
            .verify(signed.as_bytes(), &signature)
            .ok()?;
        if claims["sub"].as_str()? != self.engine {
            return None;
        }
        let audience = match &claims["aud"] {
            Value::String(s) => s,
            Value::Array(a) if a.len() == 1 => a[0].as_str()?,
            _ => return None,
        };
        if audience != self.instance
            || claims["exp"].as_u64()?
                <= SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs()
        {
            return None;
        }
        Some(())
    }
}
fn jwt_parts(token: &str) -> Option<(Value, Value, &str, Vec<u8>)> {
    let mut parts = token.split('.');
    let (header, claims, signature) = (parts.next()?, parts.next()?, parts.next()?);
    if parts.next().is_some() {
        return None;
    }
    let signed = token.get(..header.len() + 1 + claims.len())?;
    Some((
        serde_json::from_slice(&B64.decode(header).ok()?).ok()?,
        serde_json::from_slice(&B64.decode(claims).ok()?).ok()?,
        signed,
        B64.decode(signature).ok()?,
    ))
}

// Only the certificate's SAN extension is needed here; rustls parses the certificate and
// checks its private key. A small definite-length DER reader avoids a production X.509
// dependency. Reject noncanonical lengths, duplicate SANs and any SAN other than one URI.
fn der<'a>(input: &mut &'a [u8], tag: u8) -> Option<&'a [u8]> {
    let (actual, rest) = input.split_first()?;
    if *actual != tag {
        return None;
    }
    let (&first, rest) = rest.split_first()?;
    let (len, rest) = if first < 128 {
        (usize::from(first), rest)
    } else {
        let count = usize::from(first & 127);
        if count == 0 || count > std::mem::size_of::<usize>() {
            return None;
        }
        let bytes = rest.get(..count)?;
        if bytes[0] == 0 {
            return None;
        }
        let mut len = 0usize;
        for b in bytes {
            len = len.checked_mul(256)?.checked_add(usize::from(*b))?;
        }
        if len < 128 {
            return None;
        }
        (len, rest.get(count..)?)
    };
    let value = rest.get(..len)?;
    *input = rest.get(len..)?;
    Some(value)
}
fn certificate_uri(mut cert: &[u8]) -> Option<&str> {
    let mut certificate = der(&mut cert, 0x30)?;
    if !cert.is_empty() {
        return None;
    }
    let mut tbs = der(&mut certificate, 0x30)?;
    if tbs.first() == Some(&0xa0) {
        der(&mut tbs, 0xa0)?;
    }
    for tag in [0x02, 0x30, 0x30, 0x30, 0x30, 0x30] {
        der(&mut tbs, tag)?;
    }
    // Optional issuerUniqueID and subjectUniqueID precede extensions.
    for tag in [0x81, 0x82] {
        if tbs.first() == Some(&tag) {
            der(&mut tbs, tag)?;
        }
    }
    let mut explicit = der(&mut tbs, 0xa3)?;
    if !tbs.is_empty() {
        return None;
    }
    let mut extensions = der(&mut explicit, 0x30)?;
    if !explicit.is_empty() {
        return None;
    }
    let mut uri = None;
    while !extensions.is_empty() {
        let mut extension = der(&mut extensions, 0x30)?;
        let oid = der(&mut extension, 0x06)?;
        if extension.first() == Some(&0x01) {
            der(&mut extension, 0x01)?;
        }
        let mut value = der(&mut extension, 0x04)?;
        if !extension.is_empty() {
            return None;
        }
        if oid == [0x55, 0x1d, 0x11] {
            if uri.is_some() {
                return None;
            }
            let mut names = der(&mut value, 0x30)?;
            let name = der(&mut names, 0x86)?;
            if !value.is_empty() || !names.is_empty() {
                return None;
            }
            uri = Some(std::str::from_utf8(name).ok()?);
        }
    }
    uri
}
