//! Read and preflight the exact archive closure before sending installation requests.
mod archive;
mod pin;

use crate::{author, boundary, server, user_config};
use conduit_core::{
    Context, Input, State,
    archives::{ArchiveInput, FetchCompletion},
    packages::bytes_digest,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use url::Url;

const INSTALL: &str = "management.install.v1";
const LIST: &str = "management.list.v1";

pub(crate) enum Failure {
    Stop(String),
    Request(String),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pin {
    address: String,
    version: String,
    digest: String,
}

impl Pin {
    fn read(address: &str, value: &Value) -> Option<Self> {
        let version = value["version"].as_str()?;
        let digest = value["digest"].as_str()?;
        let hex = digest.strip_prefix("sha256:")?;
        if hex.len() != 64
            || !hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return None;
        }
        Some(Self {
            address: address.into(),
            version: version.into(),
            digest: digest.into(),
        })
    }

    fn hex(&self) -> &str {
        self.digest
            .strip_prefix("sha256:")
            .expect("validated digest")
    }

    fn description(&self) -> String {
        format!("{}@{}", self.address, self.version)
    }

    fn fields(&self) -> String {
        format!(
            "Package/{} version={} digest={}",
            self.address, self.version, self.digest
        )
    }

    fn version_key(&self) -> (&str, &str) {
        (&self.address, precedence(&self.version))
    }

    fn release_key(&self) -> Self {
        Self {
            address: self.address.clone(),
            version: precedence(&self.version).into(),
            digest: self.digest.clone(),
        }
    }
}

#[derive(Clone)]
struct Requirement {
    pin: Pin,
    fetch: Option<String>,
}

impl Requirement {
    fn read(address: &str, value: &Value) -> Option<Self> {
        Some(Self {
            pin: Pin::read(address, value)?,
            fetch: value["fetch"].as_str().map(str::to_owned),
        })
    }
}

fn precedence(version: &str) -> &str {
    version
        .split_once('+')
        .map_or(version, |(version, _)| version)
}

struct Candidate {
    pin: Pin,
    bytes: Vec<u8>,
    fetch: Url,
    source: String,
    authored: String,
    dependencies: Vec<Requirement>,
}

impl Candidate {
    fn open(
        pin: Pin,
        bytes: Vec<u8>,
        source: String,
        authored: String,
        fetch: Url,
        manifest: &Value,
    ) -> crate::Result<Self> {
        let dependencies = archive::dependencies(&bytes, manifest)?;
        Ok(Self {
            pin,
            bytes,
            source,
            authored,
            fetch,
            dependencies,
        })
    }

    fn arguments(&self) -> Value {
        json!({"address":self.pin.address,"version":self.pin.version,"digest":self.pin.digest,"fetch":self.fetch.as_str()})
    }
}

struct Sources {
    cache: Option<PathBuf>,
    mirrors: Vec<PathBuf>,
    dry_run: bool,
}

enum SearchError {
    Missing(Vec<String>),
    Invalid(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    mirrors: Vec<PathBuf>,
}

impl Sources {
    fn read(cli_mirrors: &[PathBuf], dry_run: bool) -> crate::Result<Self> {
        let config_dir = user_config::directory().ok();
        let mut mirrors = cli_mirrors.to_vec();
        if let Some(directory) = &config_dir {
            let config = directory.join("config.json");
            match fs::read(&config) {
                Ok(bytes) => {
                    let decoded = serde_json::from_slice::<Config>(&bytes)
                        .ok()
                        .filter(|_| bytes.iter().find(|byte| !byte.is_ascii_whitespace()) == Some(&b'{'))
                        .filter(|value| value.mirrors.iter().all(|mirror| mirror.is_absolute()))
                        .ok_or_else(|| format!("{config:?} must contain only mirrors, a list of absolute directories"))?;
                    mirrors.extend(decoded.mirrors);
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                    ) => {}
                Err(error) => return Err(format!("cannot read {config:?}: {error}")),
            }
        }
        for mirror in &mirrors {
            if !mirror.is_dir() {
                return Err(format!("mirror {mirror:?} is not a directory"));
            }
        }
        Ok(Self {
            cache: config_dir.map(|directory| directory.join("packages")),
            mirrors,
            dry_run,
        })
    }

    fn keep(&self, digest: &str, bytes: &[u8]) {
        if !self.dry_run {
            user_config::cache_archive(digest, bytes);
        }
    }

    fn named(&self, argument: &str) -> crate::Result<Candidate> {
        let verified = boundary::verified_install_archive(argument, |hex, bytes| {
            self.keep(hex, bytes);
        })?;
        let manifest = boundary::install_manifest(&verified.bytes)?;
        let pin = Pin {
            address: manifest["address"]
                .as_str()
                .ok_or("named archive has no address")?
                .into(),
            version: manifest["version"]
                .as_str()
                .ok_or("named archive has no version")?
                .into(),
            digest: verified.digest,
        };
        Candidate::open(
            pin,
            verified.bytes,
            "argument".into(),
            verified.authored_path.into(),
            verified.fetch,
            &manifest,
        )
    }

    fn find(&self, pin: &Pin, fetch: Option<&str>) -> Result<Candidate, SearchError> {
        let mut tried = Vec::new();
        let mut places = vec![(
            "cache".into(),
            self.cache
                .as_ref()
                .map(|directory| directory.join(pin.hex())),
        )];
        if let Some(fetch) = fetch {
            let parsed = Url::parse(fetch).ok();
            let sensitive = fetch.contains(['?', '#'])
                || fetch.starts_with("//") && fetch.contains('@')
                || parsed
                    .as_ref()
                    .is_some_and(|url| !url.username().is_empty() || url.password().is_some());
            let file = parsed
                .filter(|url| {
                    url.scheme() == "file" && url.username().is_empty() && url.password().is_none()
                })
                .and_then(|url| url.to_file_path().ok())
                .filter(|file| file.is_absolute());
            let source = if sensitive || (file.is_none() && fetch.contains('@')) {
                "<withheld>".into()
            } else {
                user_config::escape_line_breaks(fetch)
            };
            places.push((format!("fetch:{source}"), file));
        }
        places.extend(self.mirrors.iter().map(|directory| {
            (
                format!(
                    "mirror:{}",
                    user_config::escape_line_breaks(&directory.to_string_lossy())
                ),
                Some(directory.join(pin.hex())),
            )
        }));
        for (source, path) in places {
            let Some(path) = path else {
                tried.push(if source == "cache" {
                    source
                } else {
                    format!("{source}(unbuilt)")
                });
                continue;
            };
            let Ok(path) = path.canonicalize() else {
                tried.push(source);
                continue;
            };
            if !path.is_file() {
                tried.push(source);
                continue;
            }
            let Ok(bytes) = fs::read(&path) else {
                tried.push(source);
                continue;
            };
            if bytes_digest(&bytes) != pin.digest {
                tried.push(format!("{source}(mismatch)"));
                continue;
            }
            self.keep(pin.hex(), &bytes);
            let manifest = boundary::install_manifest(&bytes).map_err(|_| {
                SearchError::Invalid(format!("manifest=unreadable from={source} file={path:?}"))
            })?;
            let address = manifest["address"].as_str().unwrap_or("-");
            let version = manifest["version"].as_str().unwrap_or("-");
            if address != pin.address || precedence(version) != precedence(&pin.version) {
                return Err(SearchError::Invalid(format!(
                    "manifest={address}@{version}"
                )));
            }
            let fetch = Url::from_file_path(&path)
                .map_err(|()| SearchError::Invalid("archive=invalid-file-url".into()))?;
            let authored = path.to_string_lossy().into_owned();
            return Candidate::open(pin.clone(), bytes, source, authored, fetch, &manifest)
                .map_err(|error| {
                    SearchError::Invalid(format!(
                        "archive=invalid diagnostic={:?}",
                        user_config::escape_line_breaks(&error)
                    ))
                });
        }
        Err(SearchError::Missing(tried))
    }
}

struct Release {
    pin: Pin,
    source: ReleaseSource,
}

enum ReleaseSource {
    Missing {
        fetches: BTreeSet<Option<String>>,
        tried: Vec<String>,
    },
    Present,
    Found(Box<Candidate>),
    Rejected,
}

struct Walk<'a> {
    core: &'a str,
    sources: &'a Sources,
    installed: &'a [Value],
    needs: BTreeMap<Pin, BTreeSet<String>>,
    versions: BTreeMap<(String, String), String>,
    releases: BTreeMap<Pin, Release>,
    present: Vec<Pin>,
    ordered: Vec<Candidate>,
    stops: Vec<(Pin, String, &'static str)>,
}

impl Walk<'_> {
    fn stop(&mut self, pin: &Pin, cause: String, promise: &'static str) {
        self.stops.push((pin.clone(), cause, promise));
    }

    fn conflict(&mut self, pin: &Pin) -> Option<String> {
        let (address, version) = pin.version_key();
        let key = (address.to_owned(), version.to_owned());
        let prior = self
            .versions
            .entry(key)
            .or_insert_with(|| pin.digest.clone());
        if *prior != pin.digest {
            return Some(format!("listed={prior}"));
        }
        self.installed
            .iter()
            .find(|package| {
                package["address"] == address
                    && package["version"]
                        .as_str()
                        .is_some_and(|actual| precedence(actual) == version)
                    && package["digest"] != pin.digest
            })
            .and_then(|package| package["digest"].as_str())
            .map(|other| format!("installed={other}"))
    }

    fn found(&mut self, key: Pin, candidate: Candidate) {
        let needer = candidate.pin.description();
        let dependencies = candidate.dependencies.clone();
        let release = Release {
            pin: candidate.pin.clone(),
            source: ReleaseSource::Found(Box::new(candidate)),
        };
        self.releases.insert(key, release);
        for dependency in dependencies {
            self.visit(&dependency, &needer);
        }
    }

    fn visit(&mut self, requirement: &Requirement, needer: &str) {
        let pin = &requirement.pin;
        if pin.address == self.core {
            return;
        }
        let key = pin.release_key();
        self.needs
            .entry(key.clone())
            .or_default()
            .insert(needer.into());
        let conflict = self.conflict(pin);
        let release = self.releases.entry(key.clone()).or_insert_with(|| Release {
            pin: pin.clone(),
            source: ReleaseSource::Missing {
                fetches: BTreeSet::new(),
                tried: Vec::new(),
            },
        });
        if let Some(cause) = conflict {
            if !matches!(release.source, ReleaseSource::Rejected) {
                release.source = ReleaseSource::Rejected;
                self.stop(pin, cause, "PKG-025");
            }
            return;
        }
        let ReleaseSource::Missing { fetches, tried } = &mut release.source else {
            return;
        };
        if !fetches.insert(requirement.fetch.clone()) {
            return;
        }
        if self.installed.iter().any(|package| {
            package["digest"] == pin.digest
                && package["address"] == pin.address
                && package["version"]
                    .as_str()
                    .is_some_and(|version| precedence(version) == precedence(&pin.version))
        }) {
            release.source = ReleaseSource::Present;
            return;
        }
        match self
            .sources
            .find(&release.pin, requirement.fetch.as_deref())
        {
            Ok(candidate) => self.found(key, candidate),
            Err(SearchError::Missing(locations)) => {
                for location in locations {
                    if !tried.contains(&location) {
                        tried.push(location);
                    }
                }
            }
            Err(SearchError::Invalid(cause)) => {
                let first = release.pin.clone();
                release.source = ReleaseSource::Rejected;
                self.stop(&first, cause, "PKG-023");
            }
        }
    }

    fn named(&mut self, candidate: Candidate) {
        if let Some(cause) = self.conflict(&candidate.pin) {
            self.stop(&candidate.pin, cause, "PKG-025");
        }
        self.found(candidate.pin.release_key(), candidate);
    }

    // A shared release can acquire its source while reading a later branch.
    // Schedule after discovery to keep its first depth-first position.
    fn schedule(&mut self, pin: &Pin) {
        let key = pin.release_key();
        let Some(release) = self.releases.remove(&key) else {
            return;
        };
        match release.source {
            ReleaseSource::Found(candidate) => {
                for dependency in &candidate.dependencies {
                    self.schedule(&dependency.pin);
                }
                self.ordered.push(*candidate);
            }
            ReleaseSource::Present => self.present.push(release.pin),
            ReleaseSource::Missing { tried, .. } => {
                let rule = if self.needs[&key].contains("Manifest/-") {
                    "PKG-006"
                } else {
                    "PKG-028"
                };
                self.stop(
                    &release.pin,
                    format!("archive=absent tried=[{}]", tried.join(",")),
                    rule,
                );
            }
            ReleaseSource::Rejected => {}
        }
    }

    fn needed_by(&self, pin: &Pin) -> String {
        let names = self
            .needs
            .get(&pin.release_key())
            .into_iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<_>>();
        format!("needed-by=[{}]", names.join(","))
    }

    fn preflight(&mut self, directory: &Path, context: &Context) -> crate::Result<()> {
        if self.ordered.is_empty() {
            return Ok(());
        }
        let mut snapshot: State = serde_json::from_value(author::state(directory)?)
            .map_err(|_| "cannot read the engine's package snapshot")?;
        let builtin = context.builtin.as_ref().ok_or("missing built-in package")?;
        for candidate in &self.ordered {
            let input = Input::InstallArchive {
                archive: ArchiveInput {
                    name: candidate.pin.description(),
                    address: candidate.pin.address.clone(),
                    version: candidate.pin.version.clone(),
                    digest: candidate.pin.digest.clone(),
                    directory: None,
                    completion: FetchCompletion::Complete {
                        bytes: candidate.bytes.clone(),
                    },
                },
                snapshots: snapshot.preinstalled_snapshots.clone(),
                builtin: builtin.clone(),
            };
            // CTR-013's cross-address client-ID check belongs to the engine when built.
            let transition = conduit_core::step_owned(snapshot, input, context)?;
            for diagnostic in transition.reply["diagnostics"]
                .as_array()
                .into_iter()
                .flatten()
            {
                let rule = match diagnostic["ruleId"].as_str() {
                    Some("NAM-024") => "NAM-024",
                    Some("PKG-029") => "PKG-029",
                    Some("CTR-013") => "CTR-013",
                    _ => continue,
                };
                let cause = diagnostic["message"]["text"]
                    .as_str()
                    .unwrap_or("archive cannot be installed");
                self.stops.push((
                    candidate.pin.clone(),
                    format!("diagnostic={:?}", user_config::escape_line_breaks(cause)),
                    rule,
                ));
            }
            snapshot = transition.state;
        }
        Ok(())
    }

    fn print_stops(&self) {
        for (pin, cause, rule) in &self.stops {
            println!(
                "stop {} {cause} {} {rule}",
                pin.fields(),
                self.needed_by(pin)
            );
            eprintln!(
                "conduit install: no install.v1 sent for {}: {cause} ({rule})",
                pin.description()
            );
        }
    }

    fn print_present(&self) {
        for pin in &self.present {
            println!(
                "present {} {} with its dependencies (PKG-028)",
                pin.fields(),
                self.needed_by(pin)
            );
        }
    }

    async fn send(&self, directory: &Path) -> std::result::Result<(), Failure> {
        let mut installed = Vec::new();
        for (index, candidate) in self.ordered.iter().enumerate() {
            let pin = &candidate.pin;
            println!(
                "install {} from={} {}",
                pin.fields(),
                candidate.source,
                self.needed_by(pin)
            );
            if self.sources.dry_run {
                continue;
            }
            let cause =
                match server::client(directory, None, Some(INSTALL), candidate.arguments()).await {
                    Ok(mut reply) => {
                        author::author_diagnostics(&mut reply, None, &candidate.authored, true);
                        println!("{reply}");
                        if author::succeeded(&reply) {
                            if !self
                                .installed
                                .iter()
                                .any(|other| other["digest"] == pin.digest)
                            {
                                installed.push(pin.description());
                            }
                            continue;
                        }
                        "engine refused the request; see its printed result".into()
                    }
                    Err(error) => user_config::escape_line_breaks(&error),
                };
            let remaining = self.ordered[index + 1..]
                .iter()
                .map(|next| next.pin.description())
                .collect::<Vec<_>>();
            println!("not sent [{}]", remaining.join(","));
            return Err(Failure::Request(format!(
                "{INSTALL} for {} failed: {cause}; installed [{}], failed [{}], not sent [{}]; nothing was rolled back",
                pin.description(),
                installed.join(","),
                pin.description(),
                remaining.join(",")
            )));
        }
        Ok(())
    }
}

pub(crate) async fn run(
    directory: &Path,
    repository: &Path,
    args: &[String],
    options: &author::Options<'_>,
) -> std::result::Result<(), Failure> {
    let argument = match args {
        [] => None,
        [argument] => Some(argument.as_str()),
        _ => {
            return Err(Failure::Stop(
                "install takes at most one ARCHIVE#SHA256".into(),
            ));
        }
    };
    let sources =
        Sources::read(options.install_mirrors, options.install_dry_run).map_err(|error| {
            Failure::Stop(format!(
                "no install.v1 sent: {}",
                user_config::escape_line_breaks(&error)
            ))
        })?;
    let root = argument
        .map(|argument| sources.named(argument))
        .transpose()
        .map_err(|error| {
            Failure::Stop(format!(
                "no install.v1 sent: {}",
                user_config::escape_line_breaks(&error)
            ))
        })?;
    let manifest = pin::Manifest::read(repository, root.as_ref().map(|archive| &archive.pin))?;
    let context = boundary::context(repository).map_err(Failure::Stop)?;
    println!("list Package/* as=root");
    let response = server::client(directory, None, Some(LIST), json!({}))
        .await
        .map_err(Failure::Request)?;
    author::ensure_success(&response, LIST).map_err(Failure::Request)?;
    let installed = response["structuredContent"]["packages"]
        .as_array()
        .ok_or_else(|| Failure::Request(format!("{LIST} did not return installed packages")))?;
    let mut walk = Walk {
        core: &context.core_package,
        sources: &sources,
        installed,
        needs: BTreeMap::new(),
        versions: BTreeMap::new(),
        releases: BTreeMap::new(),
        present: Vec::new(),
        ordered: Vec::new(),
        stops: Vec::new(),
    };
    if let Some(candidate) = root {
        let pin = candidate.pin.clone();
        walk.named(candidate);
        walk.schedule(&pin);
    } else {
        for requirement in &manifest.requirements {
            walk.visit(requirement, "Manifest/-");
        }
        for requirement in &manifest.requirements {
            walk.schedule(&requirement.pin);
        }
    }
    walk.preflight(directory, &context)
        .map_err(|error| Failure::Stop(format!("no install.v1 sent: {error}")))?;
    walk.print_present();
    if !walk.stops.is_empty() {
        walk.print_stops();
        return Err(Failure::Stop(String::new()));
    }
    walk.send(directory).await?;
    manifest.finish(options.install_dry_run)?;
    if options.install_dry_run {
        println!("dry-run: no install.v1 sent, nothing written");
    }
    Ok(())
}
