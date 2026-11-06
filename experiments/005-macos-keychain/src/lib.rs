//! Experiment 005: code shared by the probe and its two readers.
//!
//! Nothing here touches a keychain other than the throwaway file whose path it is given. Every
//! function that reads from a keychain is called only after the calling process has disabled
//! keychain user interaction, so a read that would show a dialog fails instead.

use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::{CFType, TCFType};
use core_foundation::data::{CFData, CFDataRef};
use core_foundation::string::{CFString, CFStringRef};
use security_framework::os::macos::keychain::SecKeychain;
use security_framework::os::macos::passwords::find_generic_password;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::ffi::c_void;
use std::io::Read;
use std::os::raw::c_char;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// The service name of every item the probe creates.
pub const SERVICE: &str = "conduit-e10-probe";

/// Declarations of Security framework functions that `security-framework` does not wrap.
pub mod ffi {
    use std::ffi::c_void;
    use std::os::raw::c_char;

    pub type OSStatus = i32;

    #[link(name = "Security", kind = "framework")]
    unsafe extern "C" {
        pub fn SecKeychainLock(keychain: *mut c_void) -> OSStatus;
        pub fn SecKeychainGetPath(
            keychain: *mut c_void,
            io_path_length: *mut u32,
            path_name: *mut c_char,
        ) -> OSStatus;
        pub fn SecKeychainGetStatus(keychain: *mut c_void, status: *mut u32) -> OSStatus;
        pub fn SecKeychainFindGenericPassword(
            keychain_or_array: *const c_void,
            service_length: u32,
            service: *const c_char,
            account_length: u32,
            account: *const c_char,
            password_length: *mut u32,
            password_data: *mut *mut c_void,
            item: *mut *mut c_void,
        ) -> OSStatus;
        pub fn SecKeychainItemCopyAccess(item: *mut c_void, access: *mut *mut c_void) -> OSStatus;
        pub fn SecAccessCopyACLList(access: *mut c_void, acl_list: *mut *const c_void) -> OSStatus;
        pub fn SecACLCopyContents(
            acl: *const c_void,
            application_list: *mut *const c_void,
            description: *mut *const c_void,
            prompt_selector: *mut u16,
        ) -> OSStatus;
        pub fn SecACLCopyAuthorizations(acl: *const c_void) -> *const c_void;
        pub fn SecTrustedApplicationCopyData(app: *const c_void, data: *mut *const c_void)
        -> OSStatus;
        pub fn SessionGetInfo(session: u32, session_id: *mut u32, attributes: *mut u32) -> OSStatus;
    }

    /// `callerSecuritySession` in AuthSession.h.
    pub const CALLER_SECURITY_SESSION: u32 = u32::MAX;
    pub const SESSION_IS_ROOT: u32 = 0x0001;
    pub const SESSION_HAS_GRAPHIC_ACCESS: u32 = 0x0010;
    pub const SESSION_HAS_TTY: u32 = 0x0020;
    pub const SESSION_IS_REMOTE: u32 = 0x1000;
    /// `kSecUnlockStateStatus` in SecKeychain.h.
    pub const UNLOCK_STATE_STATUS: u32 = 1;
}

/// The name of a Security framework result code, from SecBase.h.
pub fn status_name(code: i32) -> &'static str {
    match code {
        0 => "errSecSuccess",
        -61 => "errSecWrPerm",
        -128 => "errSecUserCanceled",
        -25291 => "errSecNotAvailable",
        -25292 => "errSecReadOnly",
        -25293 => "errSecAuthFailed",
        -25294 => "errSecNoSuchKeychain",
        -25295 => "errSecInvalidKeychain",
        -25299 => "errSecDuplicateItem",
        -25300 => "errSecItemNotFound",
        -25307 => "errSecNoDefaultKeychain",
        -25308 => "errSecInteractionNotAllowed",
        -25315 => "errSecInteractionRequired",
        -25320 => "errSecInDarkWake",
        -34018 => "errSecMissingEntitlement",
        _ => "unnamed",
    }
}

/// Lowercase hex of `n` random bytes from the kernel.
pub fn random_hex(n: usize) -> String {
    let mut buf = vec![0u8; n];
    std::fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut buf))
        .expect("read /dev/urandom");
    hex(&buf)
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// Decodes a string of hex digits, or returns `None` if it is not one.
pub fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.is_empty() || !s.len().is_multiple_of(2) || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

/// The text of every `<string>` element in an XML property list, in order.
pub fn plist_strings(xml: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find("<string>") {
        let after = &rest[start + "<string>".len()..];
        match after.find("</string>") {
            Some(end) => {
                out.push(after[..end].to_string());
                rest = &after[end + "</string>".len()..];
            }
            None => break,
        }
    }
    out
}

/// Replaces `exp_dir` with `.` and `home` with `~`, in that order.
pub fn sanitize(text: &str, exp_dir: &str, home: &str) -> String {
    let mut s = text.to_string();
    if !exp_dir.is_empty() {
        s = s.replace(exp_dir, ".");
    }
    if !home.is_empty() {
        s = s.replace(home, "~");
    }
    s
}

/// Seconds since the epoch as an ISO 8601 UTC timestamp.
pub fn iso_utc(secs: u64) -> String {
    let days = (secs / 86_400) as i64;
    let rem = secs % 86_400;
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock after 1970")
        .as_secs()
}

/// The result of running a process under a timeout.
pub struct RunOutput {
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub elapsed: Duration,
    pub timed_out: bool,
}

impl RunOutput {
    pub fn ok(&self) -> bool {
        !self.timed_out && self.exit_code == Some(0)
    }
}

/// Waits for `child` for at most `timeout`, killing it if it is still running then.
pub fn wait_with_timeout(child: &mut Child, timeout: Duration) -> (Option<i32>, bool) {
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return (status.code(), false),
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return (None, true);
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(5)),
            Err(_) => return (None, false),
        }
    }
}

/// Runs a command with stdin closed and its output captured, under a timeout.
pub fn run_with_timeout(cmd: &mut Command, timeout: Duration) -> RunOutput {
    let start = Instant::now();
    let mut child = match cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return RunOutput {
                exit_code: None,
                stdout: Vec::new(),
                stderr: format!("spawn failed: {e}").into_bytes(),
                elapsed: start.elapsed(),
                timed_out: false,
            };
        }
    };
    let mut out_pipe = child.stdout.take().expect("piped stdout");
    let mut err_pipe = child.stderr.take().expect("piped stderr");
    let out_thread = std::thread::spawn(move || {
        let mut v = Vec::new();
        let _ = out_pipe.read_to_end(&mut v);
        v
    });
    let err_thread = std::thread::spawn(move || {
        let mut v = Vec::new();
        let _ = err_pipe.read_to_end(&mut v);
        v
    });
    let (exit_code, timed_out) = wait_with_timeout(&mut child, timeout);
    let elapsed = start.elapsed();
    RunOutput {
        exit_code,
        stdout: out_thread.join().unwrap_or_default(),
        stderr: err_thread.join().unwrap_or_default(),
        elapsed,
        timed_out,
    }
}

/// The path the Security framework reports for an open keychain.
pub fn keychain_path(kc: &SecKeychain) -> Result<String, i32> {
    let mut buf = vec![0 as c_char; 2048];
    let mut len = buf.len() as u32;
    let status = unsafe {
        ffi::SecKeychainGetPath(kc.as_concrete_TypeRef().cast(), &mut len, buf.as_mut_ptr())
    };
    if status != 0 {
        return Err(status);
    }
    let bytes: Vec<u8> = buf[..len as usize].iter().map(|&c| c as u8).collect();
    Ok(String::from_utf8_lossy(&bytes).trim_end_matches('\0').to_string())
}

/// Whether the keychain is unlocked, as `SecKeychainGetStatus` reports it.
pub fn keychain_unlocked(kc: &SecKeychain) -> Value {
    let mut status = 0u32;
    let code = unsafe { ffi::SecKeychainGetStatus(kc.as_concrete_TypeRef().cast(), &mut status) };
    if code != 0 {
        return json!({"status": code, "status_name": status_name(code)});
    }
    json!({"unlocked": status & ffi::UNLOCK_STATE_STATUS != 0, "status_bits": status})
}

/// Locks the keychain with `SecKeychainLock`.
pub fn lock_keychain(kc: &SecKeychain) -> i32 {
    unsafe { ffi::SecKeychainLock(kc.as_concrete_TypeRef().cast()) }
}

/// The security session and terminal state of the calling process.
pub fn process_context() -> Value {
    let pid = std::process::id() as i32;
    let sid = unsafe { libc::getsid(0) };
    let tty = match std::fs::OpenOptions::new().read(true).open("/dev/tty") {
        Ok(_) => "present".to_string(),
        Err(e) => format!("none ({e})"),
    };
    let mut session_id = 0u32;
    let mut attrs = 0u32;
    let code = unsafe {
        ffi::SessionGetInfo(ffi::CALLER_SECURITY_SESSION, &mut session_id, &mut attrs)
    };
    json!({
        "session_leader": sid == pid,
        "stdin_is_tty": unsafe { libc::isatty(0) } == 1,
        "stdout_is_tty": unsafe { libc::isatty(1) } == 1,
        "controlling_terminal": tty,
        "security_session": {
            "status": code,
            "attributes_hex": format!("0x{attrs:04x}"),
            "is_root": attrs & ffi::SESSION_IS_ROOT != 0,
            "has_graphic_access": attrs & ffi::SESSION_HAS_GRAPHIC_ACCESS != 0,
            "has_tty": attrs & ffi::SESSION_HAS_TTY != 0,
            "is_remote": attrs & ffi::SESSION_IS_REMOTE != 0,
        },
    })
}

/// One read of a generic password from the given keychain only, timed. The value itself is
/// never returned: only its length and SHA-256.
pub fn timed_read(kc: &SecKeychain, account: &str) -> Value {
    let start = Instant::now();
    let result = find_generic_password(Some(std::slice::from_ref(kc)), SERVICE, account);
    let us = start.elapsed().as_micros() as u64;
    match result {
        Ok((password, _item)) => json!({
            "ok": true,
            "status": 0,
            "status_name": "errSecSuccess",
            "elapsed_us": us,
            "value_len": password.len(),
            "value_sha256": sha256_hex(&password),
        }),
        Err(e) => json!({
            "ok": false,
            "status": e.code(),
            "status_name": status_name(e.code()),
            "message": e.message(),
            "elapsed_us": us,
        }),
    }
}

fn cfstring_owned(ptr: *const c_void) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CFString::wrap_under_create_rule(ptr as CFStringRef) }.to_string())
}

/// The access control list of a generic password, read without its value: for each entry its
/// authorizations, its trusted applications (`null` means any application) and its
/// description, with a partition list decoded from the description where it is one.
pub fn acl_dump(kc: &SecKeychain, account: &str) -> Value {
    unsafe {
        let mut item: *mut c_void = std::ptr::null_mut();
        let code = ffi::SecKeychainFindGenericPassword(
            kc.as_concrete_TypeRef().cast(),
            SERVICE.len() as u32,
            SERVICE.as_ptr().cast(),
            account.len() as u32,
            account.as_ptr().cast(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut item,
        );
        if code != 0 {
            return json!({"error": "find", "status": code, "status_name": status_name(code)});
        }
        let item = CFType::wrap_under_create_rule(item as *const c_void);
        let mut access: *mut c_void = std::ptr::null_mut();
        let code = ffi::SecKeychainItemCopyAccess(item.as_CFTypeRef() as *mut c_void, &mut access);
        if code != 0 {
            return json!({"error": "copy access", "status": code, "status_name": status_name(code)});
        }
        let access = CFType::wrap_under_create_rule(access as *const c_void);
        let mut list: *const c_void = std::ptr::null();
        let code = ffi::SecAccessCopyACLList(access.as_CFTypeRef() as *mut c_void, &mut list);
        if code != 0 {
            return json!({"error": "copy acl list", "status": code, "status_name": status_name(code)});
        }
        let list: CFArray<CFType> = CFArray::wrap_under_create_rule(list as CFArrayRef);
        let mut entries = Vec::new();
        for acl in list.iter() {
            let acl_ref = acl.as_CFTypeRef();
            let auths_ptr = ffi::SecACLCopyAuthorizations(acl_ref);
            let authorizations: Vec<String> = if auths_ptr.is_null() {
                Vec::new()
            } else {
                let a: CFArray<CFType> = CFArray::wrap_under_create_rule(auths_ptr as CFArrayRef);
                a.iter()
                    .map(|s| s.downcast::<CFString>().map(|c| c.to_string()).unwrap_or_default())
                    .collect()
            };
            let mut apps: *const c_void = std::ptr::null();
            let mut desc: *const c_void = std::ptr::null();
            let mut selector = 0u16;
            let code = ffi::SecACLCopyContents(acl_ref, &mut apps, &mut desc, &mut selector);
            let applications = if code != 0 {
                json!({"status": code, "status_name": status_name(code)})
            } else if apps.is_null() {
                Value::Null
            } else {
                let a: CFArray<CFType> = CFArray::wrap_under_create_rule(apps as CFArrayRef);
                Value::Array(
                    a.iter()
                        .map(|app| {
                            let mut data: *const c_void = std::ptr::null();
                            let c = ffi::SecTrustedApplicationCopyData(app.as_CFTypeRef(), &mut data);
                            if c != 0 || data.is_null() {
                                return json!({"status": c, "status_name": status_name(c)});
                            }
                            let d = CFData::wrap_under_create_rule(data as CFDataRef);
                            let path = String::from_utf8_lossy(d.bytes())
                                .trim_end_matches('\0')
                                .to_string();
                            Value::String(path)
                        })
                        .collect(),
                )
            };
            let description = cfstring_owned(desc);
            let is_partition = authorizations.iter().any(|a| a == "ACLAuthorizationPartitionID");
            let partitions = description
                .as_deref()
                .filter(|_| is_partition)
                .and_then(unhex)
                .map(|bytes| plist_strings(&String::from_utf8_lossy(&bytes)));
            let mut entry = json!({
                "authorizations": authorizations,
                "applications": applications,
                "prompt_selector": selector,
            });
            match partitions {
                Some(p) => entry["partition_ids"] = json!(p),
                None => entry["description"] = json!(description),
            }
            entries.push(entry);
        }
        json!({"entries": entries})
    }
}

/// The partition IDs of an access control list dumped by [`acl_dump`].
pub fn partition_ids(dump: &Value) -> Vec<String> {
    dump["entries"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|e| e["partition_ids"].as_array())
        .flatten()
        .filter_map(|s| s.as_str().map(str::to_string))
        .collect()
}

/// The entry point of both readers. Usage:
///
/// ```text
/// reader read   <keychain> <account> <out.json>
/// reader bench  <keychain> <account> <count> <out.json>
/// reader create <keychain> <account> <out.json>
/// ```
///
/// The reader disables keychain user interaction before it opens the keychain, and writes its
/// result to `<out.json>`, not to its standard output, which may be closed.
pub fn reader_main(binary: &str, build_tag: &str) -> ! {
    let args: Vec<String> = std::env::args().collect();
    let (mode, keychain, account, count, out) = match args.get(1).map(String::as_str) {
        Some("read") | Some("create") if args.len() == 5 => {
            (args[1].clone(), args[2].clone(), args[3].clone(), 1usize, args[4].clone())
        }
        Some("bench") if args.len() == 6 => (
            args[1].clone(),
            args[2].clone(),
            args[3].clone(),
            args[4].parse().unwrap_or(1),
            args[5].clone(),
        ),
        _ => {
            eprintln!("usage: {binary} read|create <keychain> <account> <out.json>");
            eprintln!("       {binary} bench <keychain> <account> <count> <out.json>");
            std::process::exit(2);
        }
    };

    let mut result = json!({
        "binary": binary,
        "build_tag": build_tag,
        "mode": mode,
        "account": account,
        "context": process_context(),
    });

    let _ui = match SecKeychain::disable_user_interaction() {
        Ok(lock) => lock,
        Err(e) => {
            result["error"] = json!(format!("could not disable user interaction: {}", e.code()));
            write_out(&out, &result);
            std::process::exit(1);
        }
    };
    result["user_interaction_allowed"] = json!(SecKeychain::user_interaction_allowed().ok());
    if result["user_interaction_allowed"] != json!(false) {
        result["error"] = json!("user interaction still allowed; refusing to read");
        write_out(&out, &result);
        std::process::exit(1);
    }

    let kc = match SecKeychain::open(Path::new(&keychain)) {
        Ok(kc) => kc,
        Err(e) => {
            result["error"] = json!(format!("open failed: {}", e.code()));
            write_out(&out, &result);
            std::process::exit(1);
        }
    };
    result["keychain_before"] = keychain_unlocked(&kc);

    match mode.as_str() {
        "read" => result["read"] = timed_read(&kc, &account),
        "bench" => {
            let reads: Vec<Value> = (0..count).map(|_| timed_read(&kc, &account)).collect();
            result["reads"] = json!(reads);
        }
        "create" => {
            let value = random_hex(24);
            let start = Instant::now();
            let r = kc.add_generic_password(SERVICE, &account, value.as_bytes());
            let us = start.elapsed().as_micros() as u64;
            result["create"] = match r {
                Ok(()) => json!({
                    "ok": true,
                    "elapsed_us": us,
                    "value_len": value.len(),
                    "value_sha256": sha256_hex(value.as_bytes()),
                }),
                Err(e) => json!({
                    "ok": false,
                    "status": e.code(),
                    "status_name": status_name(e.code()),
                    "message": e.message(),
                }),
            };
        }
        _ => unreachable!(),
    }
    write_out(&out, &result);
    std::process::exit(0);
}

fn write_out(path: &str, value: &Value) {
    let text = serde_json::to_string_pretty(value).expect("serialize");
    if let Err(e) = std::fs::write(path, text) {
        eprintln!("could not write {path}: {e}");
        std::process::exit(3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_known_codes() {
        assert_eq!(status_name(-25308), "errSecInteractionNotAllowed");
        assert_eq!(status_name(-25293), "errSecAuthFailed");
        assert_eq!(status_name(1), "unnamed");
    }

    #[test]
    fn decodes_a_partition_list() {
        let xml = "<?xml version=\"1.0\"?><plist><dict><key>Partitions</key><array>\
                   <string>apple-tool:</string><string>cdhash:00ff</string></array></dict></plist>";
        let encoded = hex(xml.as_bytes());
        let decoded = unhex(&encoded).unwrap();
        assert_eq!(
            plist_strings(&String::from_utf8(decoded).unwrap()),
            vec!["apple-tool:", "cdhash:00ff"]
        );
        assert!(unhex("not hex").is_none());
        assert!(unhex("abc").is_none());
    }

    #[test]
    fn sanitizes_the_experiment_directory_before_home() {
        let s = sanitize(
            "/Users/u/x/exp/target/a and /Users/u/Library/k",
            "/Users/u/x/exp",
            "/Users/u",
        );
        assert_eq!(s, "./target/a and ~/Library/k");
    }

    #[test]
    fn formats_utc_dates() {
        assert_eq!(iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(1_793_923_200), "2026-11-06T00:00:00Z");
    }

    #[test]
    fn digests_are_sha256() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn times_out_a_process() {
        let out = run_with_timeout(
            Command::new("/bin/sleep").arg("5"),
            Duration::from_millis(200),
        );
        assert!(out.timed_out);
    }
}
