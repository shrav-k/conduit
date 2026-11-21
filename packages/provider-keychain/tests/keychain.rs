#![cfg(target_os = "macos")]

mod common;

use common::{Client, failed, request};
use core_foundation::{
    array::CFArray,
    base::{CFType, TCFType},
    string::CFString,
};
use security_framework::os::macos::{
    keychain::{CreateOptions, KeychainSettings, SecKeychain},
    keychain_item::SecKeychainItem,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    ffi::{c_char, c_void},
    fs::{self, File},
    io::Read,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::Stdio,
    ptr,
    sync::{
        Once,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{
    process::Command,
    sync::{Mutex, MutexGuard},
    time::timeout,
};

const SECURITY: &str = "/usr/bin/security";
const BINARY: &str = env!("CARGO_BIN_EXE_provider-keychain");
const SERVICE: &str = "conduit-test-service";
const ACCOUNT: &str = "conduit-test-account";
const REFUSED: &str = "keychain store could not be read: access to the item was refused";
const DEDICATED: &str = "keychain store could not be read: only a dedicated keychain is allowed";
static SERIAL: Mutex<()> = Mutex::const_new(());
static INTERACTION: Once = Once::new();
static STOP: AtomicBool = AtomicBool::new(false);

#[link(name = "Security", kind = "framework")]
unsafe extern "C" {
    fn SecKeychainLock(keychain: *mut c_void) -> i32;
    fn SecKeychainGetStatus(keychain: *mut c_void, status: *mut u32) -> i32;
    fn SecKeychainGetPath(keychain: *mut c_void, len: *mut u32, path: *mut u8) -> i32;
    fn SecTrustedApplicationCreateFromPath(path: *const c_char, app: *mut *const c_void) -> i32;
    fn SecAccessCreate(
        description: *const c_void,
        trusted: *const c_void,
        access: *mut *const c_void,
    ) -> i32;
    fn SecKeychainItemCreateFromContent(
        class: u32,
        attributes: *mut AttributeList,
        length: u32,
        data: *const c_void,
        keychain: *mut c_void,
        access: *const c_void,
        item: *mut *mut c_void,
    ) -> i32;
}

#[repr(C)]
struct Attribute {
    tag: u32,
    length: u32,
    data: *mut c_void,
}

impl Attribute {
    fn new(tag: [u8; 4], value: &str) -> Self {
        Self {
            tag: u32::from_be_bytes(tag),
            length: value.len().try_into().expect("test attribute length"),
            data: value.as_ptr().cast_mut().cast(),
        }
    }
}

#[repr(C)]
struct AttributeList {
    count: u32,
    attributes: *mut Attribute,
}

fn require_no_interaction() {
    assert!(
        !status(SecKeychain::user_interaction_allowed()),
        "keychain interaction must be off"
    );
}

fn trusted_application(path: *const c_char) -> CFType {
    let mut app = ptr::null();
    let code = unsafe { SecTrustedApplicationCreateFromPath(path, &mut app) };
    assert_eq!(code, 0, "create trusted application reference");
    assert!(!app.is_null());
    unsafe { CFType::wrap_under_create_rule(app) }
}

fn stop(message: &'static str) -> ! {
    STOP.store(true, Ordering::SeqCst);
    panic!("{message}");
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut encoded = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(encoded, "{byte:02x}").expect("encode synthetic bytes");
    }
    encoded
}

fn marker() -> String {
    let mut random = [0u8; 32];
    File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut random))
        .expect("generate synthetic material");
    hex(&random)
}

fn status<T>(result: security_framework::base::Result<T>) -> T {
    result.unwrap_or_else(|error| panic!("Security framework status {}", error.code()))
}

struct Throwaway {
    chain: SecKeychain,
    path: PathBuf,
    password: String,
}

impl Throwaway {
    fn create(directory: &Path) -> Self {
        require_no_interaction();
        fs::create_dir_all(directory).expect("make throwaway parent");
        let requested = directory.join("throwaway.keychain");
        assert!(
            !requested.to_string_lossy().contains("login"),
            "invalid test path"
        );
        assert!(!requested.exists() && !directory.join("throwaway.keychain-db").exists());
        let password = marker();
        let chain = status(
            CreateOptions::new()
                .password(&password)
                .prompt_user(false)
                .create(&requested),
        );
        let mut buffer = [0u8; 4096];
        let mut length = buffer.len() as u32;
        let code = unsafe {
            SecKeychainGetPath(
                chain.as_concrete_TypeRef().cast(),
                &mut length,
                buffer.as_mut_ptr(),
            )
        };
        assert_eq!(code, 0, "get throwaway path");
        let path = std::str::from_utf8(&buffer[..length as usize]).expect("decode throwaway path");
        let path = PathBuf::from(path.trim_end_matches('\0'));
        assert!(
            path == requested || path == directory.join("throwaway.keychain-db"),
            "unexpected throwaway path"
        );
        assert!(fs::symlink_metadata(&path).is_ok_and(|info| info.is_file()));
        let mut result = Self {
            chain,
            path,
            password,
        };
        require_no_interaction();
        status(result.chain.set_settings(&KeychainSettings::new()));
        result.require_unlocked();
        result
    }

    fn require_unlocked(&self) {
        let mut bits = 0;
        let code =
            unsafe { SecKeychainGetStatus(self.chain.as_concrete_TypeRef().cast(), &mut bits) };
        assert_eq!(code, 0, "check throwaway lock state");
        assert_ne!(bits & 1, 0, "throwaway must be unlocked for this command");
    }

    fn add(&self, account: &str, value: &[u8], trust_provider: bool) -> SecKeychainItem {
        require_no_interaction();
        self.require_unlocked();
        // A null application path names the running test binary. The keychain
        // destination below is always the explicit handle returned by create.
        let mut apps = vec![trusted_application(ptr::null())];
        if trust_provider {
            let provider = std::ffi::CString::new(BINARY).expect("provider binary path");
            apps.push(trusted_application(provider.as_ptr()));
        }
        let trusted = CFArray::from_CFTypes(&apps);
        let description = CFString::new(SERVICE);
        let mut access = ptr::null();
        let code = unsafe {
            SecAccessCreate(
                description.as_CFTypeRef(),
                trusted.as_CFTypeRef(),
                &mut access,
            )
        };
        assert_eq!(code, 0, "create test item access list");
        assert!(!access.is_null());
        let access = unsafe { CFType::wrap_under_create_rule(access) };
        let mut attributes = [
            Attribute::new(*b"svce", SERVICE),
            Attribute::new(*b"acct", account),
        ];
        let mut attributes = AttributeList {
            count: attributes.len() as u32,
            attributes: attributes.as_mut_ptr(),
        };
        let mut item = ptr::null_mut();
        let code = unsafe {
            SecKeychainItemCreateFromContent(
                u32::from_be_bytes(*b"genp"),
                &mut attributes,
                value.len().try_into().expect("test value length"),
                value.as_ptr().cast(),
                self.chain.as_concrete_TypeRef().cast(),
                access.as_CFTypeRef(),
                &mut item,
            )
        };
        assert_eq!(code, 0, "add item to the throwaway keychain");
        assert!(!item.is_null());
        unsafe { SecKeychainItem::wrap_under_create_rule(item.cast()) }
    }

    fn lock(&self) -> i32 {
        require_no_interaction();
        unsafe { SecKeychainLock(self.chain.as_concrete_TypeRef().cast()) }
    }

    fn unlock(&mut self) {
        require_no_interaction();
        status(self.chain.unlock(Some(&self.password)));
        self.require_unlocked();
    }

    fn locator(&self, account: &str) -> Value {
        locator(&self.path, account)
    }
}

impl Drop for Throwaway {
    fn drop(&mut self) {
        let code = self.lock();
        let removed = fs::remove_file(&self.path).is_ok();
        if !std::thread::panicking() {
            assert_eq!(code, 0, "lock throwaway before cleanup");
            assert!(removed, "remove throwaway file");
        }
    }
}

#[derive(Clone, Copy)]
enum Setting {
    SearchList,
    Default,
}

async fn query(setting: Setting) -> Vec<u8> {
    assert!(!STOP.load(Ordering::SeqCst), "keychain safety stop");
    let arguments = match setting {
        Setting::SearchList => ["list-keychains", "-d", "user"],
        Setting::Default => ["default-keychain", "-d", "user"],
    };
    let output = match timeout(
        Duration::from_secs(30),
        Command::new(SECURITY)
            .args(arguments)
            .env_clear()
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .output(),
    )
    .await
    {
        Ok(Ok(output)) => output,
        Ok(Err(_)) => panic!("could not run the read-only settings query"),
        Err(_) => stop("settings query exceeded 30 seconds, treated as a dialog"),
    };
    assert!(
        output.status.success(),
        "settings query exited with status {:?}",
        output.status.code()
    );
    output.stdout
}

async fn settings() -> (Vec<u8>, Vec<u8>) {
    (
        query(Setting::SearchList).await,
        query(Setting::Default).await,
    )
}

struct Scenario {
    _serial: MutexGuard<'static, ()>,
    directory: tempfile::TempDir,
    before: (Vec<u8>, Vec<u8>),
}

impl Scenario {
    async fn start() -> Self {
        let serial = SERIAL.lock().await;
        assert!(!STOP.load(Ordering::SeqCst), "keychain safety stop");
        INTERACTION.call_once(|| {
            let guard = status(SecKeychain::disable_user_interaction());
            std::mem::forget(guard);
            require_no_interaction();
        });
        let before = settings().await;
        let directory = tempfile::tempdir().expect("make temporary test directory");
        Self {
            _serial: serial,
            directory,
            before,
        }
    }

    async fn chain(&self, name: &str) -> Throwaway {
        let root = self
            .directory
            .path()
            .canonicalize()
            .expect("resolve test directory");
        let chain = Throwaway::create(&root.join(name));
        self.verify_settings().await;
        chain
    }

    async fn verify_settings(&self) {
        if settings().await != self.before {
            stop("keychain search list or default did not match its initial setting");
        }
    }
}

impl Drop for Scenario {
    fn drop(&mut self) {
        if std::thread::panicking() {
            STOP.store(true, Ordering::SeqCst);
        }
    }
}

fn locator(path: &Path, account: &str) -> Value {
    json!({"keychain":path,"service":SERVICE,"account":account})
}

fn succeeded(wire: &Value, expected: &str) {
    assert!(
        wire["isError"] == false && wire["_meta"][provider_keychain::OUTCOME] == "succeeded",
        "throwaway read did not succeed"
    );
    let value = wire["structuredContent"]["value"]
        .as_str()
        .expect("read returned no value");
    assert!(
        Sha256::digest(value) == Sha256::digest(expected),
        "read returned unexpected bytes"
    );
}

fn refuses(wire: &Value, message: &str, markers: &[&str]) {
    failed(wire, message);
    let reply = wire.to_string();
    assert!(
        markers.iter().all(|value| !reply.contains(value)),
        "refusal included a marker"
    );
}

#[tokio::test]
async fn reads_the_named_item_and_reads_again_each_time() {
    let scenario = Scenario::start().await;
    let chain = scenario.chain("old").await;
    let first = format!("{}\n", marker());
    let updated = marker();
    let mut item = chain.add(ACCOUNT, first.as_bytes(), true);
    let client = Client::start().await;
    let read = request(chain.locator(ACCOUNT));
    succeeded(&client.call(read.clone()).await, &first);
    require_no_interaction();
    chain.require_unlocked();
    status(item.set_password(updated.as_bytes()));
    drop(item);
    succeeded(&client.call(read).await, &updated);
    client.stop().await;
    drop(chain);
    scenario.verify_settings().await;
}

#[tokio::test]
async fn a_locked_keychain_fails_at_once_and_tries_no_other_store() {
    let scenario = Scenario::start().await;
    let mut chain = scenario.chain("selected").await;
    let other = scenario.chain("other").await;
    let first = marker();
    let second = marker();
    chain.add(ACCOUNT, first.as_bytes(), true);
    other.add(ACCOUNT, second.as_bytes(), true);
    assert_eq!(chain.lock(), 0, "lock the named throwaway");
    let client = Client::start().await;
    let read = request(chain.locator(ACCOUNT));
    refuses(
        &client.call(read.clone()).await,
        "keychain store could not be read: keychain is locked",
        &[&first, &second],
    );
    chain.unlock();
    succeeded(&client.call(read).await, &first);
    client.stop().await;
    drop((chain, other));
    scenario.verify_settings().await;
}

#[tokio::test]
async fn missing_untrusted_and_unreadable_reads_fail_with_no_value() {
    let scenario = Scenario::start().await;
    let chain = scenario.chain("selected").await;
    let other = scenario.chain("other").await;
    let first = marker();
    let second = marker();
    chain.add("untrusted", first.as_bytes(), false);
    other.add(ACCOUNT, second.as_bytes(), true);
    let mut invalid = first.as_bytes().to_vec();
    invalid.push(0xff);
    let mut oversized = first.as_bytes().to_vec();
    oversized.resize(provider_keychain::MAX_VALUE_BYTES + 1, b'x');
    chain.add("invalid", &invalid, true);
    chain.add("oversized", &oversized, true);
    let client = Client::start().await;
    for (input, message) in [
        (
            locator(&scenario.directory.path().join("missing"), ACCOUNT),
            "keychain store could not be read: keychain file is missing",
        ),
        (
            locator(scenario.directory.path(), ACCOUNT),
            "keychain store could not be read: path is not a regular file",
        ),
        (
            chain.locator(ACCOUNT),
            "keychain store could not be read: generic password item is missing",
        ),
        (chain.locator("untrusted"), REFUSED),
        (
            chain.locator("invalid"),
            "keychain store could not be read: value is not valid UTF-8",
        ),
        (
            chain.locator("oversized"),
            "keychain store could not be read: value exceeds 65536 bytes",
        ),
    ] {
        refuses(
            &client.call(request(input)).await,
            message,
            &[&first, &second],
        );
    }
    client.stop().await;
    drop((chain, other));
    scenario.verify_settings().await;
}

#[tokio::test]
async fn refuses_a_throwaway_named_like_the_login_keychain() {
    let scenario = Scenario::start().await;
    let chain = scenario.chain("selected").await;
    let directory = chain.path.parent().expect("throwaway parent").to_owned();
    let login_name = directory.join("login.keychain-db");
    // Create the fixture under a safe name. Never create or access a keychain
    // through the Security framework with the reserved login basename.
    fs::hard_link(&chain.path, &login_name).expect("name the throwaway fixture");
    drop(chain);
    let alias = directory.join("alias.keychain");
    symlink(&login_name, &alias).expect("make a locator alias in the temporary directory");
    let client = Client::start().await;
    for path in [&login_name, &alias] {
        failed(
            &client.call(request(locator(path, ACCOUNT))).await,
            DEDICATED,
        );
    }
    client.stop().await;
    scenario.verify_settings().await;
}
