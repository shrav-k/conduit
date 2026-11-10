//! Store-private ES256 authority and per-run files. No credential is a core input.
use crate::{
    Result,
    store::{Store, atomic_write, private_directory, private_read},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD as B64};
use conduit_core::{Context, State, access::Identity};
use p256::{
    ecdsa::{
        Signature, SigningKey,
        signature::{Signer, Verifier},
    },
    pkcs8::DecodePrivateKey,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub(crate) fn random_text(length: usize) -> Result<String> {
    let mut result = String::new();
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    while result.len() < length {
        let mut bytes = [0u8; 32];
        getrandom::fill(&mut bytes).map_err(|_| "OS random generator unavailable")?;
        for byte in bytes {
            if byte < 252 && result.len() < length {
                result.push(ALPHABET[(byte % 36) as usize] as char);
            }
        }
    }
    Ok(result)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityFile {
    engine_id: String,
    key: String,
    certificate: String,
}

pub(crate) struct Authority {
    data: AuthorityFile,
    key: SigningKey,
}

/// Kept only at the executor boundary. Deliberately has no Debug/Serialize implementation.
pub struct RunIdentity {
    pub environment: BTreeMap<String, String>,
    pub directory: PathBuf,
}

impl Authority {
    pub fn open(store: &Store) -> Result<Self> {
        let path = store.directory.join("authority.json");
        let data: AuthorityFile = if path.try_exists().map_err(|e| e.to_string())? {
            serde_json::from_slice(&private_read(&path)?)
                .map_err(|_| "invalid signing authority")?
        } else {
            // Losing the authority of an established store must not silently rotate its identity.
            if store.read()?.state.started {
                return Err("established store is missing its signing authority".into());
            }
            let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)
                .map_err(|_| "key generation failed")?;
            let engine_id = format!("{}.conduit.local", random_text(26)?);
            let mut params = rcgen::CertificateParams::new(vec![engine_id.clone()])
                .map_err(|_| "invalid root parameters")?;
            params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
            params.key_usages = vec![
                rcgen::KeyUsagePurpose::KeyCertSign,
                rcgen::KeyUsagePurpose::DigitalSignature,
            ];
            let certificate = params
                .self_signed(&key)
                .map_err(|_| "root signing failed")?
                .pem();
            let data = AuthorityFile {
                engine_id,
                key: key.serialize_pem(),
                certificate,
            };
            atomic_write(
                &store.directory,
                "authority.json",
                &serde_json::to_vec(&data).map_err(|_| "authority encoding failed")?,
            )?;
            data
        };
        let key = SigningKey::from_pkcs8_pem(&data.key).map_err(|_| "invalid signing key")?;
        Ok(Self { data, key })
    }
    fn jwks(&self) -> Result<Value> {
        let point = self.key.verifying_key().to_encoded_point(false);
        Ok(
            json!({"keys":[{"kty":"EC","crv":"P-256","use":"sig","alg":"ES256","kid":"store",
            "x":B64.encode(point.x().ok_or("missing public x")?),"y":B64.encode(point.y().ok_or("missing public y")?)}]}),
        )
    }
    pub(crate) fn token(&self, instance: &str, address: &str) -> Result<String> {
        let header = B64.encode(br#"{"alg":"ES256","typ":"JWT","kid":"store"}"#);
        let issued = chrono::Utc::now();
        let expires = issued
            .checked_add_months(chrono::Months::new(100 * 12))
            .ok_or("identity expiration out of range")?;
        let claims = json!({"sub":format!("spiffe://{}/instance/{instance}",self.data.engine_id),"aud":[address],"iat":issued.timestamp(),"exp":expires.timestamp(),"jti":Uuid::new_v4().to_string()});
        let payload =
            B64.encode(serde_json::to_vec(&claims).map_err(|_| "claims encoding failed")?);
        let signed = format!("{header}.{payload}");
        let signature: Signature = self.key.sign(signed.as_bytes());
        Ok(format!("{signed}.{}", B64.encode(signature.to_bytes())))
    }
    pub fn materialize(
        &self,
        store: &Path,
        descriptor: &Value,
        address: &str,
        state: &State,
        context: &Context,
    ) -> Result<RunIdentity> {
        let instance = descriptor["materializeFor"]
            .as_str()
            .ok_or("missing identity subject")?;
        if Uuid::parse_str(instance).is_err() || !exists(state, instance) {
            return Err("identity subject is not an established instance".into());
        }
        let schema = context
            .core_environment
            .scopes
            .values()
            .flat_map(|s| s.contracts.values())
            .find(|v| v["kind"] == "Operation" && v["metadata"]["name"] == "run.v1")
            .ok_or("missing run contract")?["spec"]["inputSchema"]["properties"]["identity"]
            .clone();
        let env = &schema["properties"]["environment"]["required"];
        let files = &schema["properties"]["files"]["required"];
        if descriptor["environmentNames"] != *env || descriptor["fileNames"] != *files {
            return Err("identity descriptor differs from held run contract".into());
        }
        let env = env
            .as_array()
            .filter(|v| v.len() == 2)
            .ok_or("unsupported identity environment contract")?;
        let files = files
            .as_array()
            .filter(|v| v.len() == 4)
            .ok_or("unsupported identity files contract")?;
        let runs = store.join("runs");
        private_directory(&runs)?;
        let directory = runs.join(Uuid::new_v4().to_string());
        private_directory(&directory)?;
        let contents = [
            self.data.certificate.clone(),
            self.jwks()?.to_string(),
            self.data.engine_id.clone(),
            self.token(instance, address)?,
        ];
        for (name, content) in files.iter().zip(contents) {
            let name = name.as_str().ok_or("invalid identity file name")?;
            if Path::new(name).components().count() != 1 || name == "." || name == ".." {
                return Err("invalid identity file path".into());
            }
            atomic_write(&directory, name, content.as_bytes())?;
        }
        Ok(RunIdentity {
            environment: BTreeMap::from([
                (
                    env[0].as_str().ok_or("invalid identity variable")?.into(),
                    address.into(),
                ),
                (
                    env[1].as_str().ok_or("invalid identity variable")?.into(),
                    directory.to_string_lossy().into_owned(),
                ),
            ]),
            directory,
        })
    }
    pub fn verify(&self, token: Option<&str>, address: &str, state: &State) -> Identity {
        let Some(token) = token else {
            return Identity::None;
        };
        self.verified_instance(token, address, state)
            .map(|id| Identity::Instance { id })
            .unwrap_or(Identity::Unreadable)
    }
    fn verified_instance(&self, token: &str, address: &str, state: &State) -> Option<String> {
        let mut parts = token.split('.');
        let (header, payload, signature) = (parts.next()?, parts.next()?, parts.next()?);
        if parts.next().is_some() {
            return None;
        }
        let header_value: Value = serde_json::from_slice(&B64.decode(header).ok()?).ok()?;
        if header_value["alg"] != "ES256" || header_value.get("crit").is_some() {
            return None;
        }
        let signature = Signature::from_slice(&B64.decode(signature).ok()?).ok()?;
        self.key
            .verifying_key()
            .verify(format!("{header}.{payload}").as_bytes(), &signature)
            .ok()?;
        let claims: Value = serde_json::from_slice(&B64.decode(payload).ok()?).ok()?;
        if !claims["aud"].as_array()?.iter().any(|aud| aud == address)
            || claims["iat"].as_i64().is_none()
            || claims["exp"].as_i64()? <= chrono::Utc::now().timestamp()
            || Uuid::parse_str(claims["jti"].as_str()?).is_err()
        {
            return None;
        }
        let prefix = format!("spiffe://{}/instance/", self.data.engine_id);
        let instance = claims["sub"].as_str()?.strip_prefix(&prefix)?;
        let id = Uuid::parse_str(instance).ok()?;
        if id.to_string() != instance || !exists(state, instance) {
            return None;
        }
        Some(instance.into())
    }
}

fn exists(state: &State, id: &str) -> bool {
    state
        .objects
        .iter()
        .any(|v| v["id"] == id && v.get("spec").is_none())
}
