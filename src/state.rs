//! Secure, durable storage for small application state files.
//!
//! State names are single, validated leaf names. All file operations after the
//! state root is opened are relative to its capability handle.

#![allow(dead_code)] // The complete reusable API is intentionally broader than history.

use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

use cap_std::fs::Dir;

const LOCK_RETRY: Duration = Duration::from_millis(10);
const LOCK_TIMEOUT: Duration = Duration::from_secs(8);
const MAX_LEAF_LEN: usize = 64;
static UNIQUE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug)]
pub(crate) struct StateDir {
    root: PathBuf,
}

impl StateDir {
    pub(crate) fn platform() -> Self {
        Self { root: state_root() }
    }

    #[cfg(test)]
    fn at(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn read_text(&self, leaf: &str, max_bytes: u64) -> Result<Option<String>, String> {
        validate_leaf(leaf)?;
        let dir = prepare_root(&self.root)?;
        read_text_from(&dir, leaf, max_bytes)
    }

    pub(crate) fn exists(&self, leaf: &str) -> Result<bool, String> {
        validate_leaf(leaf)?;
        let dir = prepare_root(&self.root)?;
        regular_file_exists(&dir, leaf)
    }

    pub(crate) fn write_text_atomic(&self, leaf: &str, text: &str) -> Result<(), String> {
        self.with_lock(leaf, |file| file.write_text_atomic(text))
    }

    pub(crate) fn remove(&self, leaf: &str) -> Result<(), String> {
        self.with_lock(leaf, |file| file.remove())
    }

    pub(crate) fn with_lock<T>(
        &self,
        leaf: &str,
        action: impl FnOnce(&LockedStateFile<'_>) -> Result<T, String>,
    ) -> Result<T, String> {
        validate_leaf(leaf)?;
        let dir = prepare_root(&self.root)?;
        let _lock = StateLock::acquire(&dir, leaf)?;
        action(&LockedStateFile { dir: &dir, leaf })
    }
}

pub(crate) struct LockedStateFile<'a> {
    dir: &'a Dir,
    leaf: &'a str,
}

impl LockedStateFile<'_> {
    pub(crate) fn read_text(&self, max_bytes: u64) -> Result<Option<String>, String> {
        read_text_from(self.dir, self.leaf, max_bytes)
    }

    pub(crate) fn exists(&self) -> Result<bool, String> {
        regular_file_exists(self.dir, self.leaf)
    }

    pub(crate) fn write_text_atomic(&self, text: &str) -> Result<(), String> {
        write_atomic_to(self.dir, self.leaf, text.as_bytes())
    }

    pub(crate) fn remove(&self) -> Result<(), String> {
        match self.dir.remove_file(self.leaf) {
            Ok(()) => sync_dir(self.dir).map_err(|error| error.to_string()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}

fn validate_leaf(leaf: &str) -> Result<(), String> {
    let mut bytes = leaf.bytes();
    let valid_start = bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric());
    let valid_rest =
        bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));
    if !valid_start || !valid_rest || leaf.len() > MAX_LEAF_LEN {
        return Err("state name must be a simple ASCII leaf name".into());
    }
    Ok(())
}

fn read_text_from(dir: &Dir, leaf: &str, max_bytes: u64) -> Result<Option<String>, String> {
    let metadata = match dir.symlink_metadata(leaf) {
        Ok(metadata) if metadata.is_file() => metadata,
        Ok(_) => return Err("state file is a symlink or non-file".into()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    if metadata.len() > max_bytes {
        return Err("state file exceeds its read limit".into());
    }

    // cap-std resolves this open beneath `dir`, even if the leaf changes after
    // the metadata check. The second bound also covers a concurrent growth.
    let mut file = dir.open(leaf).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    (&mut file)
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > max_bytes {
        return Err("state file exceeds its read limit".into());
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| "state file is not UTF-8".into())
}

fn regular_file_exists(dir: &Dir, leaf: &str) -> Result<bool, String> {
    match dir.symlink_metadata(leaf) {
        Ok(metadata) if metadata.is_file() => Ok(true),
        Ok(_) => Err("state file is a symlink or non-file".into()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error.to_string()),
    }
}

fn write_atomic_to(dir: &Dir, leaf: &str, bytes: &[u8]) -> Result<(), String> {
    let sequence = UNIQUE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = format!(".{leaf}.tmp.{}.{}", std::process::id(), sequence);
    let result = (|| {
        let mut options = cap_std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use cap_std::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = dir
            .open_with(&temp, &options)
            .map_err(|error| error.to_string())?;
        file.write_all(bytes)
            .and_then(|()| file.sync_all())
            .map_err(|error| error.to_string())?;
        dir.rename(&temp, dir, leaf)
            .map_err(|error| error.to_string())?;
        sync_dir(dir).map_err(|error| error.to_string())
    })();
    if result.is_err() {
        let _ = dir.remove_file(&temp);
    }
    result
}

fn prepare_root(root: &Path) -> Result<Dir, String> {
    match fs::symlink_metadata(root) {
        Ok(metadata) if metadata.file_type().is_dir() => {}
        Ok(_) => return Err("state root is a symlink or non-directory".into()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if let Some(parent) = root.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                let mut builder = fs::DirBuilder::new();
                builder.mode(0o700);
                match builder.create(root) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error.to_string()),
                }
            }
            #[cfg(not(unix))]
            match fs::create_dir(root) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error.to_string()),
            }
        }
        Err(error) => return Err(error.to_string()),
    }

    let dir = open_private_root(root).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use cap_std::fs::PermissionsExt;
        let metadata = dir.dir_metadata().map_err(|error| error.to_string())?;
        if metadata.permissions().mode() & 0o077 != 0 {
            dir.set_permissions(".", cap_std::fs::Permissions::from_mode(0o700))
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(dir)
}

fn open_private_root(root: &Path) -> io::Result<Dir> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
            .open(root)?;
        if file.metadata()?.uid() != unsafe { libc::geteuid() } {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "state root is owned by another user",
            ));
        }
        Ok(Dir::from_std_file(file))
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(root)?;
        let metadata = file.metadata()?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "state root is a symlink or non-directory",
            ));
        }
        Ok(Dir::from_std_file(file))
    }
    #[cfg(not(any(unix, windows)))]
    {
        Dir::open_ambient_dir(root, cap_std::ambient_authority())
    }
}

fn sync_dir(dir: &Dir) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::fd::{AsRawFd, FromRawFd};
        let fd = unsafe {
            libc::openat(
                dir.as_raw_fd(),
                c".".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe { fs::File::from_raw_fd(fd) }.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
        Ok(())
    }
}

struct StateLock<'a> {
    dir: &'a Dir,
    name: String,
    identity: Vec<u8>,
}

impl<'a> StateLock<'a> {
    fn acquire(dir: &'a Dir, leaf: &str) -> Result<Self, String> {
        Self::acquire_with_timeout(dir, leaf, LOCK_TIMEOUT)
    }

    fn acquire_with_timeout(dir: &'a Dir, leaf: &str, timeout: Duration) -> Result<Self, String> {
        let name = format!(".{leaf}.lock");
        let started = std::time::Instant::now();
        loop {
            match create_lock(dir, &name) {
                Ok(Some(identity)) => {
                    return Ok(Self {
                        dir,
                        name,
                        identity,
                    });
                }
                Ok(None) => {
                    if reclaim_stale_lock(dir, &name) {
                        continue;
                    }
                    if started.elapsed() >= timeout {
                        return Err(format!("timed out waiting for state lock {leaf}"));
                    }
                    thread::sleep(LOCK_RETRY);
                }
                Err(error) => return Err(error.to_string()),
            }
        }
    }
}

impl Drop for StateLock<'_> {
    fn drop(&mut self) {
        let _ = remove_if_unchanged(self.dir, &self.name, &self.identity);
    }
}

fn create_lock(dir: &Dir, name: &str) -> io::Result<Option<Vec<u8>>> {
    let sequence = UNIQUE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let identity = format!("{} {}\n", std::process::id(), sequence).into_bytes();
    let mut options = cap_std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = match dir.open_with(name, &options) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => return Ok(None),
        Err(error) => return Err(error),
    };
    if let Err(error) = file.write_all(&identity).and_then(|()| file.sync_all()) {
        let _ = remove_if_unchanged(dir, name, &identity);
        return Err(error);
    }
    Ok(Some(identity))
}

fn owner_is_proven_dead(owner_is_alive: Option<bool>) -> bool {
    matches!(owner_is_alive, Some(false))
}

fn invalid_or_stale_lock(dir: &Dir, name: &str) -> Option<Vec<u8>> {
    #[cfg(target_os = "linux")]
    {
        invalid_or_stale_lock_with_liveness(dir, name, linux_owner_liveness)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (dir, name);
        // There is no portable, trustworthy owner-liveness check. Failing
        // closed can require manual cleanup after a crash, but it cannot steal
        // a lock from a slow active process merely because the lock is old.
        owner_is_proven_dead(None).then(Vec::new)
    }
}

#[cfg(target_os = "linux")]
fn invalid_or_stale_lock_with_liveness(
    dir: &Dir,
    name: &str,
    owner_liveness: impl FnOnce(u32) -> Option<bool>,
) -> Option<Vec<u8>> {
    let identity = read_small_file(dir, name, 128)?;
    let pid = parse_lock_pid(&identity)?;
    owner_is_proven_dead(owner_liveness(pid)).then_some(identity)
}

fn parse_lock_pid(identity: &[u8]) -> Option<u32> {
    let text = std::str::from_utf8(identity).ok()?;
    let mut parts = text.split_whitespace();
    let pid = parts.next()?.parse::<u32>().ok()?;
    parts.next()?.parse::<u64>().ok()?;
    parts.next().is_none().then_some(pid)
}

#[cfg(target_os = "linux")]
fn linux_owner_liveness(pid: u32) -> Option<bool> {
    let proc = Path::new("/proc");
    fs::metadata(proc).ok()?.is_dir().then_some(())?;
    match fs::metadata(proc.join(pid.to_string())) {
        Ok(_) => Some(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Some(false),
        Err(_) => None,
    }
}

fn read_small_file(dir: &Dir, name: &str, limit: u64) -> Option<Vec<u8>> {
    let mut options = cap_std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use cap_std::fs::OpenOptionsExt;
        options.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let mut file = dir.open_with(name, &options).ok()?;
    let metadata = file.metadata().ok()?;
    if !metadata.is_file() || metadata.len() > limit {
        return None;
    }
    let mut bytes = Vec::new();
    (&mut file)
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .ok()?;
    (bytes.len() as u64 <= limit).then_some(bytes)
}

fn remove_if_unchanged(dir: &Dir, name: &str, expected: &[u8]) -> bool {
    !expected.is_empty()
        && read_small_file(dir, name, 128).is_some_and(|identity| identity == expected)
        && dir.remove_file(name).is_ok()
}

#[cfg(target_os = "linux")]
struct RecoveryGuard<'a> {
    dir: &'a Dir,
}

#[cfg(target_os = "linux")]
impl<'a> RecoveryGuard<'a> {
    fn acquire(dir: &'a Dir) -> Option<Self> {
        use std::os::fd::AsRawFd;

        (unsafe { libc::flock(dir.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0)
            .then_some(Self { dir })
    }
}

#[cfg(target_os = "linux")]
impl Drop for RecoveryGuard<'_> {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd;

        let _ = unsafe { libc::flock(self.dir.as_raw_fd(), libc::LOCK_UN) };
    }
}

fn reclaim_stale_lock(dir: &Dir, name: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        // The directory lock serializes the fresh liveness check and unlink.
        // Normal lock creation stays atomic and does not need this slow path.
        let Some(_guard) = RecoveryGuard::acquire(dir) else {
            return false;
        };
        invalid_or_stale_lock(dir, name)
            .is_some_and(|identity| remove_if_unchanged(dir, name, &identity))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (dir, name);
        false
    }
}

fn state_root() -> PathBuf {
    if let Some(dir) = std::env::var_os("XDG_STATE_HOME") {
        return PathBuf::from(dir).join("quicksearch");
    }
    if cfg!(windows) {
        if let Some(dir) = std::env::var_os("LOCALAPPDATA") {
            return PathBuf::from(dir).join("quicksearch/state");
        }
    }
    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        let home = PathBuf::from(home);
        if cfg!(target_os = "macos") {
            return home.join("Library/Application Support/quicksearch");
        }
        return home.join(".local/state/quicksearch");
    }
    std::env::temp_dir().join(format!("quicksearch-state-{}", std::process::id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let sequence = UNIQUE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "quicksearch-state-test-{}-{sequence}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&path);
            Self(path)
        }

        fn store(&self) -> StateDir {
            StateDir::at(self.0.clone())
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn lock_recovery_requires_proof_that_the_owner_is_dead() {
        assert!(owner_is_proven_dead(Some(false)));
        assert!(!owner_is_proven_dead(Some(true)));
        assert!(!owner_is_proven_dead(None));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn hostile_state_lock_entries_time_out_and_remain_untouched() {
        use std::os::unix::fs::FileTypeExt;
        use std::os::unix::net::UnixListener;

        let timeout = Duration::from_millis(25);

        let malformed = TestDir::new();
        fs::create_dir(&malformed.0).unwrap();
        let malformed_dir = prepare_root(&malformed.0).unwrap();
        let malformed_path = malformed.0.join(".history.lock");
        fs::write(&malformed_path, b"malformed").unwrap();
        assert!(StateLock::acquire_with_timeout(&malformed_dir, "history", timeout).is_err());
        assert_eq!(fs::read(&malformed_path).unwrap(), b"malformed");

        let oversized = TestDir::new();
        fs::create_dir(&oversized.0).unwrap();
        let oversized_dir = prepare_root(&oversized.0).unwrap();
        let oversized_path = oversized.0.join(".history.lock");
        fs::write(&oversized_path, vec![b'x'; 129]).unwrap();
        assert!(StateLock::acquire_with_timeout(&oversized_dir, "history", timeout).is_err());
        assert_eq!(fs::metadata(&oversized_path).unwrap().len(), 129);

        let symlinked = TestDir::new();
        fs::create_dir(&symlinked.0).unwrap();
        let symlinked_dir = prepare_root(&symlinked.0).unwrap();
        let symlink_path = symlinked.0.join(".history.lock");
        let target = symlinked.0.join("outside");
        fs::write(&target, b"outside").unwrap();
        std::os::unix::fs::symlink(&target, &symlink_path).unwrap();
        assert!(StateLock::acquire_with_timeout(&symlinked_dir, "history", timeout).is_err());
        assert!(fs::symlink_metadata(&symlink_path)
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(fs::read(&target).unwrap(), b"outside");

        let special = TestDir::new();
        fs::create_dir(&special.0).unwrap();
        let special_dir = prepare_root(&special.0).unwrap();
        let special_path = special.0.join(".history.lock");
        let _listener = UnixListener::bind(&special_path).unwrap();
        assert!(StateLock::acquire_with_timeout(&special_dir, "history", timeout).is_err());
        assert!(fs::symlink_metadata(&special_path)
            .unwrap()
            .file_type()
            .is_socket());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn unknown_state_lock_owner_is_left_untouched() {
        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        let dir = prepare_root(&test.0).unwrap();
        let name = ".history.lock";
        let identity = b"42 7
";
        fs::write(test.0.join(name), identity).unwrap();
        assert_eq!(
            invalid_or_stale_lock_with_liveness(&dir, name, |_| None),
            None
        );
        assert_eq!(fs::read(test.0.join(name)).unwrap(), identity);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn valid_dead_state_lock_identity_is_recovered() {
        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        let dir = prepare_root(&test.0).unwrap();
        let path = test.0.join(".history.lock");
        fs::write(
            &path,
            b"4294967295 7
",
        )
        .unwrap();
        let lock = StateLock::acquire_with_timeout(&dir, "history", Duration::from_millis(100))
            .expect("valid dead-owner lock should be recovered");
        assert_ne!(
            fs::read(&path).unwrap(),
            b"4294967295 7
"
        );
        drop(lock);
        assert!(!path.exists());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn held_recovery_guard_obeys_the_acquisition_deadline() {
        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        fs::write(test.0.join(".history.lock"), b"4294967295 16\n").unwrap();
        let held_dir = prepare_root(&test.0).unwrap();
        let waiting_dir = prepare_root(&test.0).unwrap();
        let _held = RecoveryGuard::acquire(&held_dir).unwrap();

        let started = std::time::Instant::now();
        assert!(StateLock::acquire_with_timeout(
            &waiting_dir,
            "history",
            Duration::from_millis(25)
        )
        .is_err());
        assert!(started.elapsed() < Duration::from_millis(500));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn two_stale_observers_cannot_remove_a_new_live_generation() {
        use std::sync::{mpsc, Arc, Barrier};

        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        let path = test.0.join(".history.lock");
        let stale = b"4294967295 17\n".to_vec();
        fs::write(&path, &stale).unwrap();

        let first_dir = prepare_root(&test.0).unwrap();
        let second_dir = prepare_root(&test.0).unwrap();
        let live_dir = prepare_root(&test.0).unwrap();
        let observed = Arc::new(Barrier::new(3));
        let (first_done_tx, first_done_rx) = mpsc::channel();
        let (second_go_tx, second_go_rx) = mpsc::channel();

        let first_observed = Arc::clone(&observed);
        let first_stale = stale.clone();
        let first = thread::spawn(move || {
            assert_eq!(
                invalid_or_stale_lock_with_liveness(&first_dir, ".history.lock", |_| Some(false)),
                Some(first_stale)
            );
            first_observed.wait();
            let removed = reclaim_stale_lock(&first_dir, ".history.lock");
            first_done_tx.send(removed).unwrap();
        });

        let second_observed = Arc::clone(&observed);
        let second = thread::spawn(move || {
            assert_eq!(
                invalid_or_stale_lock_with_liveness(&second_dir, ".history.lock", |_| Some(false)),
                Some(stale)
            );
            second_observed.wait();
            second_go_rx.recv().unwrap();
            reclaim_stale_lock(&second_dir, ".history.lock")
        });

        observed.wait();
        assert!(first_done_rx.recv().unwrap());
        let live =
            StateLock::acquire_with_timeout(&live_dir, "history", Duration::from_millis(100))
                .unwrap();
        let live_identity = live.identity.clone();
        second_go_tx.send(()).unwrap();

        first.join().unwrap();
        assert!(!second.join().unwrap());
        assert_eq!(fs::read(&path).unwrap(), live_identity);
        drop(live);
        assert!(!path.exists());
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn lock_files_with_unknown_owners_are_left_untouched() {
        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        let dir = prepare_root(&test.0).unwrap();
        let name = ".history.lock";

        for identity in [b"4294967295 0\n".as_slice(), b"malformed".as_slice()] {
            fs::write(test.0.join(name), identity).unwrap();
            assert_eq!(invalid_or_stale_lock(&dir, name), None);
            assert_eq!(fs::read(test.0.join(name)).unwrap(), identity);
        }
    }

    #[test]
    fn rejects_malformed_leaf_names() {
        let test = TestDir::new();
        let store = test.store();
        for leaf in ["", ".hidden", "../history", "a/b", "two words", "é"] {
            assert!(store.read_text(leaf, 10).is_err(), "accepted {leaf:?}");
            assert!(store.write_text_atomic(leaf, "no").is_err());
            assert!(store.remove(leaf).is_err());
        }
    }

    #[test]
    fn bounded_reads_allow_the_limit_and_reject_larger_files() {
        let test = TestDir::new();
        let store = test.store();
        store.write_text_atomic("history", "12345").unwrap();
        assert_eq!(
            store.read_text("history", 5).unwrap().as_deref(),
            Some("12345")
        );
        assert!(store.read_text("history", 4).is_err());
    }

    #[test]
    fn atomic_write_replaces_the_complete_file() {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let test = TestDir::new();
        let store = test.store();
        store.write_text_atomic("history", "old").unwrap();
        #[cfg(unix)]
        let old_inode = { fs::metadata(test.0.join("history")).unwrap().ino() };
        store.write_text_atomic("history", "new contents").unwrap();
        assert_eq!(
            store.read_text("history", 64).unwrap().as_deref(),
            Some("new contents")
        );
        #[cfg(unix)]
        assert_ne!(
            old_inode,
            fs::metadata(test.0.join("history")).unwrap().ino()
        );
        assert_eq!(fs::read_dir(&test.0).unwrap().count(), 1);
    }

    #[test]
    fn remove_is_idempotent() {
        let test = TestDir::new();
        let store = test.store();
        store.write_text_atomic("history", "entry").unwrap();
        store.remove("history").unwrap();
        store.remove("history").unwrap();
        assert_eq!(store.read_text("history", 64).unwrap(), None);
    }

    #[cfg(unix)]
    #[test]
    fn root_and_files_are_owner_private() {
        use std::os::unix::fs::PermissionsExt;

        let test = TestDir::new();
        fs::create_dir(&test.0).unwrap();
        fs::set_permissions(&test.0, fs::Permissions::from_mode(0o777)).unwrap();
        let store = test.store();
        store.write_text_atomic("history", "private").unwrap();
        assert_eq!(
            fs::metadata(&test.0).unwrap().permissions().mode() & 0o777,
            0o700
        );
        assert_eq!(
            fs::metadata(test.0.join("history"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlink_leaf_is_never_followed() {
        use std::os::unix::fs::symlink;

        let test = TestDir::new();
        let outside = test.0.with_extension("outside");
        fs::write(&outside, "outside").unwrap();
        prepare_root(&test.0).unwrap();
        symlink(&outside, test.0.join("history")).unwrap();
        let store = test.store();
        assert!(store.read_text("history", 64).is_err());
        assert_eq!(fs::read_to_string(&outside).unwrap(), "outside");
        store.write_text_atomic("history", "inside").unwrap();
        assert_eq!(fs::read_to_string(&outside).unwrap(), "outside");
        assert_eq!(
            store.read_text("history", 64).unwrap().as_deref(),
            Some("inside")
        );
        let _ = fs::remove_file(outside);
    }
}
