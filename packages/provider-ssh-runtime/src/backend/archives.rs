use super::{SshBackend, codec, quote, ssh::Output, valid_hash};
use crate::{instance_key, storage::atomic_write};
use runtime_provider_kit::{ArchiveDescriptor, Result, RunInput, check_packaged_program};
use std::{
    ffi::OsString,
    fs,
    io::ErrorKind,
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::sync::CancellationToken;

const TOOLS: [&str; 5] = ["wc", "sha256sum", "gzip", "tar", "base64"];
const READ_ERROR: &str = "the engine's copy of the archive cannot be read";

pub(super) struct RunArchive<'a> {
    program: &'a str,
    descriptor: &'a ArchiveDescriptor,
    hex: &'a str,
    source: PathBuf,
}

impl<'a> RunArchive<'a> {
    pub(super) fn from_input(
        input: &'a RunInput,
    ) -> std::result::Result<Option<Self>, &'static str> {
        let (program, descriptor) = match (input.program.as_deref(), input.archive.as_ref()) {
            (None, None) => return Ok(None),
            (Some(program), Some(descriptor)) => (program, descriptor),
            _ => return Err("run.v2 requires both program and archive"),
        };
        check_packaged_program(&input.item, program)?;
        let hex = descriptor
            .digest
            .strip_prefix("sha256:")
            .filter(|hex| valid_hash(hex))
            .ok_or("run.v2 archive digest is invalid")?;
        let source = descriptor
            .urls
            .first()
            .and_then(|url| engine_file_path(url))
            .ok_or("archive URL is not a file URL on the provider's machine")?;
        Ok(Some(Self {
            program,
            descriptor,
            hex,
            source,
        }))
    }
}

fn engine_file_path(url: &str) -> Option<PathBuf> {
    let (scheme, rest) = url.split_once("://")?;
    let (host, path) = rest.split_once('/')?;
    if !scheme.eq_ignore_ascii_case("file")
        || !(host.is_empty() || host.eq_ignore_ascii_case("localhost"))
    {
        return None;
    }
    let mut decoded = vec![b'/'];
    let mut bytes = path.bytes();
    while let Some(byte) = bytes.next() {
        match byte {
            b'%' => {
                let high = char::from(bytes.next()?).to_digit(16)? as u8;
                let low = char::from(bytes.next()?).to_digit(16)? as u8;
                decoded.push(high * 16 + low);
            }
            b'?' | b'#' | b'\\' => return None,
            byte if byte.is_ascii_graphic() => decoded.push(byte),
            _ => return None,
        }
    }
    if decoded.starts_with(b"//") || decoded.contains(&0) {
        return None;
    }
    Some(PathBuf::from(OsString::from_vec(decoded)))
}

impl SshBackend {
    async fn archive_shell(
        &self,
        hostname: &str,
        args: &[String],
        stdin: Vec<u8>,
        cancel: &CancellationToken,
    ) -> Result<Output> {
        let call = self.invocation(hostname, Some(args))?;
        tokio::select! {
            biased;
            _ = cancel.cancelled() => self.refusal("stage cancelled"),
            result = self.0.executor.execute(call, stdin) => {
                result.map_err(|_| self.error(&format!("ssh archive operation on machine {hostname} unavailable")))
            }
        }
    }

    pub(super) async fn stage_archive(
        &self,
        hostname: &str,
        archive: &RunArchive<'_>,
        directory: &str,
        path: &str,
        cancel: &CancellationToken,
    ) -> Result<String> {
        // Serialize cache lookup, publication and invalidation on the node without
        // holding the SSH connection lock that unrelated stops use.
        let _cache = tokio::select! {
            biased;
            _ = cancel.cancelled() => return self.refusal("stage cancelled"),
            cache = self.0.archive_locks[hostname].lock() => cache,
        };
        let archives = format!("{}/{}/archives", self.0.config.base, self.0.part);
        let kept = format!("{archives}/{}", archive.hex);
        let digest = &archive.descriptor.digest;
        let check_name = format!(
            "archive-check-{}",
            instance_key(&format!("{hostname}\0{digest}"))
        );
        let pending_check = self.0.root.join(&check_name);
        match fs::symlink_metadata(&pending_check) {
            Ok(_) => {
                self.remove_kept_archive(hostname, &kept, cancel).await?;
                self.clear_archive_check(&pending_check, hostname, digest)?;
            }
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(_) => return self.refusal(&format!(
                "archive {digest} on machine {hostname}: could not read the provider's verification marker"
            )),
        }
        if !tokio::fs::metadata(&archive.source)
            .await
            .is_ok_and(|file| file.is_file())
        {
            return self.refusal(READ_ERROR);
        }
        let source = File::open(&archive.source)
            .await
            .map_err(|_| self.error(READ_ERROR))?;
        let shell = ["/bin/sh".into(), "-s".into()];
        let parent = directory.rsplit_once('/').unwrap().0;
        let preflight = format!(
            "{}kept={}\nparent={}\ndirectory={}\ntools={}\n{PREFLIGHT}",
            codec::run_path(path),
            quote(&kept),
            quote(parent),
            quote(directory),
            quote(&TOOLS.join(" "))
        );
        let output = self
            .archive_shell(hostname, &shell, preflight.into_bytes(), cancel)
            .await?;
        match (output.success, output.stdout.as_slice()) {
            (true, b"kept\n") => {}
            (true, b"bad-directory\n") => {
                return self.refusal(&format!(
                    "archive staging directory {directory} on machine {hostname} could not be prepared; check that its parent can be cleared and the directory created and entered"
                ));
            }
            (true, b"bad-kept\n") => {
                return self.refusal(&format!(
                    "kept archive {kept} on machine {hostname} is not a regular file; remove it"
                ));
            }
            (true, b"missing\n") => {
                self.carry_archive(hostname, source, &archives, &kept, cancel)
                    .await?
            }
            (_, reply) => {
                if let Some(tool) = reply
                    .strip_prefix(b"tool ")
                    .and_then(|reply| reply.strip_suffix(b"\n"))
                    && output.success
                    && let Some(tool) = TOOLS.into_iter().find(|name| name.as_bytes() == tool)
                {
                    return self
                        .refusal(&format!("machine {hostname} lacks {tool} on the run PATH"));
                }
                return self.refusal(&format!(
                    "archive pre-check for {kept} on machine {hostname} returned no recognizable result; check the node's shell and run PATH"
                ));
            }
        }
        // Persist intent before the remote check. A mismatch whose removal fails,
        // or a check interrupted before its result arrives, must not be reused.
        self.local_io(
            atomic_write(&self.0.root, &check_name, b""),
            &format!("archive {digest} on machine {hostname}: could not save the provider's verification marker"),
        )?;
        let program = format!("{directory}/package/{}", archive.program);
        let check = format!(
            "{}kept={}\ndirectory={}\nprogram={}\nexpected_size={}\nexpected_digest={}\n{CHECK}",
            codec::run_path(path),
            quote(&kept),
            quote(directory),
            quote(&program),
            archive.descriptor.size,
            archive.hex,
        );
        let output = self
            .archive_shell(hostname, &shell, check.into_bytes(), cancel)
            .await?;
        if output.success && output.stdout == b"ready\n" {
            self.clear_archive_check(&pending_check, hostname, digest)?;
            return Ok(program);
        }
        let reply = std::str::from_utf8(&output.stdout).unwrap_or("");
        let reply = reply.strip_suffix('\n').unwrap_or("");
        let mismatch = match reply.split_once(' ') {
            Some(("size", found)) if found.bytes().all(|byte| byte.is_ascii_digit()) => {
                found.parse::<u64>().ok().map(|found| {
                    format!(
                        "archive on machine {hostname} has {found} bytes; the call gave {}",
                        archive.descriptor.size
                    )
                })
            }
            Some(("digest", found)) if valid_hash(found) => Some(format!(
                "archive on machine {hostname} has digest sha256:{found}; the call gave {digest}"
            )),
            _ => None,
        };
        if let Some(reason) = mismatch {
            if self
                .remove_kept_archive(hostname, &kept, cancel)
                .await
                .is_ok()
            {
                let _ = self.clear_archive_check(&pending_check, hostname, digest);
            }
            return self.refusal(&reason);
        }
        let reason = match reply {
            "unpack" => format!(
                "archive {digest} on machine {hostname} could not be unpacked into {directory}/package (gzip or tar failed); the kept copy is unchanged"
            ),
            "copy" => format!(
                "archive {digest} on machine {hostname} could not be read from {kept} into the snapshot for {directory}"
            ),
            "kept-gone" => format!(
                "archive {digest} on machine {hostname} is no longer a regular file at {kept} for run directory {directory}"
            ),
            "run-dir" => format!(
                "archive {digest} on machine {hostname} could not enter run directory {directory}"
            ),
            "package-dir" => format!(
                "archive {digest} on machine {hostname} could not create the empty directory {directory}/package"
            ),
            "bad-length" => format!("archive check on machine {hostname} gave no readable length"),
            "bad-digest" => format!("archive check on machine {hostname} gave no readable digest"),
            "no-program" => format!(
                "archive on machine {hostname} holds no executable {}",
                archive.program.escape_default()
            ),
            _ if reply.starts_with("size ") => format!("archive check on machine {hostname} gave no readable length"),
            _ => return self.refusal(&format!(
                "archive {digest} check on machine {hostname} for {directory} returned no recognizable result"
            )),
        };
        self.clear_archive_check(&pending_check, hostname, digest)?;
        self.refusal(&reason)
    }

    fn clear_archive_check(&self, marker: &Path, hostname: &str, digest: &str) -> Result<()> {
        self.local_io(
            fs::remove_file(marker),
            &format!("archive {digest} on machine {hostname}: could not clear the provider's verification marker"),
        )
    }

    async fn remove_kept_archive(
        &self,
        hostname: &str,
        kept: &str,
        cancel: &CancellationToken,
    ) -> Result<()> {
        let shell = ["/bin/sh".into(), "-s".into()];
        let script = format!("kept={}\n{INVALIDATE}", quote(kept));
        let output = self
            .archive_shell(hostname, &shell, script.into_bytes(), cancel)
            .await?;
        if output.success && output.stdout == b"removed\n" {
            return Ok(());
        }
        self.refusal(&format!(
            "kept archive {kept} on machine {hostname} could not be removed; it is ineligible until removal succeeds"
        ))
    }

    async fn carry_archive(
        &self,
        hostname: &str,
        mut source: File,
        archives: &str,
        kept: &str,
        cancel: &CancellationToken,
    ) -> Result<()> {
        let mut bytes = Vec::new();
        tokio::select! {
            biased;
            _ = cancel.cancelled() => return self.refusal("stage cancelled"),
            result = source.read_to_end(&mut bytes) => result.map_err(|_| self.error(READ_ERROR))?,
        };
        let script = format!(
            "archives={}\nkept={}\nbytes_to_carry={}\n{TRANSFER}",
            quote(archives),
            quote(kept),
            bytes.len(),
        );
        let output = self
            .archive_shell(
                hostname,
                &["/bin/sh".into(), "-c".into(), script],
                bytes,
                cancel,
            )
            .await?;
        if output.success && output.stdout == b"carried\n" {
            return Ok(());
        }
        let reason = match output.stdout.as_slice() {
            b"archives-dir\n" => "could not create the archive directory",
            b"mktemp\n" => "could not create the temporary file",
            b"stdin\n" => "could not write the received bytes",
            b"length-tool\n" => "could not count the received bytes",
            b"length\n" => "the received byte count differs from the sent count",
            b"existing\n" => "an entry already exists at the destination",
            b"rename\n" => "could not rename the temporary file into place",
            _ => "the remote shell returned no recognizable result",
        };
        self.refusal(&format!(
            "archive transfer to {kept} on machine {hostname} failed: {reason}"
        ))
    }
}

const PREFLIGHT: &str = r#"umask 077
if ! /bin/rm -rf -- "$parent" || ! /bin/mkdir -p -- "$directory" || ! cd -- "$directory"; then
    printf 'bad-directory\n'
    exit 0
fi
for tool in $tools; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        printf 'tool %s\n' "$tool"
        exit 0
    fi
done
if [ -f "$kept" ] && [ ! -L "$kept" ]; then
    printf 'kept\n'
elif [ -e "$kept" ] || [ -L "$kept" ]; then
    printf 'bad-kept\n'
else
    printf 'missing\n'
fi
"#;

// Plain EOF can also mean a cut SSH transfer. Verify the actual sent stream length
// before publication, independently of the descriptor length verified before unpacking.
const TRANSFER: &str = r#"umask 077
fail() { printf '%s\n' "$1"; exit 1; }
/bin/mkdir -p -- "$archives" || fail archives-dir
temporary=$(/usr/bin/mktemp "$archives/.incoming.XXXXXXXXXX") || fail mktemp
trap '/bin/rm -f -- "$temporary"' 0
trap 'exit 1' HUP INT TERM
/bin/cat > "$temporary" || fail stdin
count=$(LC_ALL=C /usr/bin/wc -c < "$temporary") || fail length-tool
read -r received extra <<EOF
$count
EOF
[ -z "$extra" ] && [ "$received" = "$bytes_to_carry" ] || fail length
[ ! -e "$kept" ] && [ ! -L "$kept" ] || fail existing
/bin/mv -- "$temporary" "$kept" || fail rename
printf 'carried\n'
"#;

const INVALIDATE: &str = r#"/bin/rm -f -- "$kept" &&
    [ ! -e "$kept" ] && [ ! -L "$kept" ] || exit 1
printf 'removed\n'
"#;

// Capture the kept bytes once into shell memory, with no generation pathname or
// writable file descriptor for the snapshot. Each consumer decodes those same bytes.
// FD 3 carries producer failures into the result even without POSIX pipefail.
const CHECK: &str = r#"umask 077
fail() { printf '%s\n' "$1"; exit 1; }
cd -- "$directory" || fail run-dir
[ -f "$kept" ] && [ ! -L "$kept" ] || fail kept-gone
snapshot=$(base64 < "$kept") || fail copy
readonly snapshot
stream_archive() {
    printf '%s' "$snapshot" | base64 -d || printf 'decode failed\n' >&3
}
count=$({ stream_archive | LC_ALL=C wc -c; } 3>&1) || fail bad-length
case "$count" in *'
'*) fail bad-length ;; esac
read -r length extra <<EOF
$count
EOF
[ -z "$extra" ] || fail bad-length
case "$length" in ''|*[!0-9]*) fail bad-length ;; esac
while [ "$length" != 0 ] && [ "${length#0}" != "$length" ]; do length=${length#0}; done
if [ "$length" != "$expected_size" ]; then
    printf 'size %s\n' "$length"
    exit 1
fi
checksum=$({ stream_archive | LC_ALL=C sha256sum; } 3>&1) || fail bad-digest
case "$checksum" in *'
'*) fail bad-digest ;; esac
read -r digest remainder <<EOF
$checksum
EOF
case "$digest" in ''|*[!0-9a-f]*) fail bad-digest ;; esac
[ "${#digest}" -eq 64 ] || fail bad-digest
if [ "$digest" != "$expected_digest" ]; then
    printf 'digest %s\n' "$digest"
    exit 1
fi
/bin/mkdir -- "$directory/package" || fail package-dir
unpack=$({
    stream_archive | { gzip -dc || printf 'gzip failed\n' >&3; } |
        {
            tar -xf - -C "$directory/package" >/dev/null || printf 'tar failed\n' >&3
            /bin/cat >/dev/null
        }
} 3>&1) || fail unpack
[ -z "$unpack" ] || fail unpack
[ -f "$program" ] && [ ! -L "$program" ] || fail no-program
mode=$(LC_ALL=C /bin/ls -ld -- "$program") || fail no-program
case "$mode" in -??[xs]*) ;; *) fail no-program ;; esac
printf 'ready\n'
"#;
