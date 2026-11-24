#[cfg(unix)]
pub mod fields;
#[cfg(unix)]
pub mod listing;
#[cfg(unix)]
pub mod process;
#[cfg(unix)]
pub mod run;
#[cfg(unix)]
pub mod send;
#[cfg(unix)]
pub mod strays;
// Shared by the two test clients; never called by usage-probe.
#[cfg(unix)]
pub mod fake;

pub fn valid_line(line: &str) -> bool {
    !line.is_empty()
        && line.len() <= 200
        && line
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.?[] :+-".contains(&b))
}

use std::io::{self, Write};

/// One of the probe's two streams, written with the system's own `write`. Std's `Stdout` and
/// `Stderr` report a bad descriptor as success, which would lose every line without a sign.
#[cfg(unix)]
struct Stream(libc::c_int);
#[cfg(unix)]
impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = unsafe { libc::write(self.0, buf.as_ptr().cast(), buf.len()) };
        if written < 0 {
            // `write_all` retries an interrupted write and returns any other error.
            return Err(io::Error::last_os_error());
        }
        Ok(written as usize)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// The panic hook's one fixed line: a single system write whose result is ignored, so it can
/// never panic, and a panic with standard error gone stays a panic, never an abort.
pub fn panicked() {
    const LINE: &[u8] = b"panicked: true\n";
    #[cfg(unix)]
    unsafe {
        libc::write(libc::STDERR_FILENO, LINE.as_ptr().cast(), LINE.len());
    }
    #[cfg(not(unix))]
    {
        let _ = io::stderr().write_all(LINE);
    }
}

/// Every line the probe prints. Each write is fallible: a failed write is recorded in `failed`,
/// never a panic, and no write is tried after it, so the run starts no more clients, cleans up
/// and exits 1 (`run::main_entry`).
pub struct Output {
    pub dropped: bool,
    pub failed: bool,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}
impl Default for Output {
    fn default() -> Self {
        #[cfg(unix)]
        let output = Self::to(Stream(libc::STDOUT_FILENO), Stream(libc::STDERR_FILENO));
        #[cfg(not(unix))]
        let output = Self::to(io::stdout(), io::stderr());
        output
    }
}
impl Output {
    /// Output to the given sinks; the probe's own are its standard output and standard error.
    pub fn to(stdout: impl Write + 'static, stderr: impl Write + 'static) -> Self {
        Self {
            dropped: false,
            failed: false,
            stdout: Box::new(stdout),
            stderr: Box::new(stderr),
        }
    }
    pub fn line(&mut self, line: &str) {
        self.write(line, false);
    }
    pub fn write(&mut self, line: &str, stderr: bool) {
        if !valid_line(line) {
            self.dropped = true;
            return;
        }
        if self.failed {
            return;
        }
        let sink = if stderr {
            &mut self.stderr
        } else {
            &mut self.stdout
        };
        let bytes = format!("{line}\n");
        if sink
            .write_all(bytes.as_bytes())
            .and_then(|()| sink.flush())
            .is_err()
        {
            self.failed = true;
        }
    }
    pub fn value(&mut self, key: &str, value: impl std::fmt::Display) {
        self.line(&format!("{key}: {value}"));
    }
    /// Ends the output; false where a line was dropped or a write failed. After a failed write
    /// it tries one fixed line on standard error, the run's clients and scratch directories
    /// already gone, and ignores that write's own failure.
    pub fn finish(&mut self) -> bool {
        if self.dropped {
            self.line("unprintable_lines: true");
        }
        if self.failed {
            let _ = self
                .stderr
                .write_all(b"probe_failed.output: true\n")
                .and_then(|()| self.stderr.flush());
        }
        !self.dropped && !self.failed
    }
}

#[cfg(unix)]
pub mod directory;
#[cfg(unix)]
pub mod failure;
