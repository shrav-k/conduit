use std::io::Write;
use std::os::unix::ffi::OsStrExt;
fn main() {
    if let Some(path) = std::env::var_os("A5_SHIM_LOG")
        && let Ok(mut log) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
    {
        for arg in std::env::args_os().skip(1) {
            let _ = log.write_all(arg.as_bytes());
            let _ = log.write_all(b"\n");
        }
    }
}
