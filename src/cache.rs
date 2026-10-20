//! Typed, versioned on-disk caching.
//!
//! This module deliberately knows nothing about HTTP or cache directory
//! policy. A caller validates and converts fetched bytes to `T` in its fetch
//! callback before this module can persist them.

#![allow(dead_code)] // Stage 1 API; provider integration follows in a later stage.

use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cap_std::fs::Dir;
use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

const LOCK_RETRY: Duration = Duration::from_millis(10);
const LOCK_TIMEOUT: Duration = Duration::from_secs(8);
pub(crate) const CACHE_LIMIT_BYTES: u64 = 4 * 1024 * 1024;
// Leave control space for lock files and atomic-write bookkeeping. Cache data
// is kept below this lower ceiling, so transient control files cannot push a
// normal concurrent workload over the public 4 MiB limit.
const CACHE_CONTROL_RESERVE_BYTES: u64 = 64 * 1024;
const CACHE_DATA_BUDGET_BYTES: u64 = CACHE_LIMIT_BYTES - CACHE_CONTROL_RESERVE_BYTES;
const MAX_ENTRY_BYTES: u64 = 2 * 1024 * 1024;
const MAX_CACHE_FILE_BYTES: u64 = CACHE_LIMIT_BYTES;
const MAX_DECOMPRESSED_CACHE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_CACHE_FILES: usize = 65_536;
const MAX_CACHE_DEPTH: usize = 32;
const COMPRESSED_MAGIC: &[u8; 4] = b"QSC1";
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// The stable, typed envelope stored in every cache file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CacheRecord<T> {
    pub schema: u32,
    pub fetched_at: u64,
    pub key: String,
    pub value: T,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CacheStatus {
    Fresh,
    Refreshed,
    StaleOnError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Cached<T> {
    pub value: T,
    pub status: CacheStatus,
}

#[derive(Debug)]
pub(crate) enum CacheError<E> {
    Io(io::Error),
    Json(serde_json::Error),
    Fetch(E),
    LockTimeout(PathBuf),
    QuotaExceeded { needed: u64, available: u64 },
    ClockBeforeUnixEpoch,
}

impl<E: fmt::Display> fmt::Display for CacheError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "cache I/O failed: {error}"),
            Self::Json(error) => write!(f, "cache serialization failed: {error}"),
            Self::Fetch(error) => write!(f, "cache refresh failed: {error}"),
            Self::LockTimeout(path) => {
                write!(f, "timed out waiting for cache lock {}", path.display())
            }
            Self::QuotaExceeded { needed, available } => write!(
                f,
                "cache entry needs {needed} bytes but only {available} bytes are available"
            ),
            Self::ClockBeforeUnixEpoch => f.write_str("system clock is before the Unix epoch"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for CacheError<E> {}

impl<E> From<io::Error> for CacheError<E> {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl<E> From<serde_json::Error> for CacheError<E> {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

/// Read a fresh typed value without attempting a refresh.
pub(crate) fn read_fresh<T>(path: &Path, ttl: Duration, schema: u32, key: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let record = read_matching::<T, std::convert::Infallible>(path, schema, key)
        .ok()
        .flatten()?;
    (age(now, record.fetched_at) <= ttl).then_some(record.value)
}

/// Return a fresh typed value, or fetch, validate, and persist a replacement.
///
/// `stale_max_age` is the maximum total age of a fallback record, measured
/// from its fetch time. Corrupt records and records with another schema or key
/// are misses and are never supplied as stale data. The fetch callback must
/// parse and validate its input before returning `T`.
pub(crate) fn get_or_fetch<T, E, F>(
    path: &Path,
    ttl: Duration,
    stale_max_age: Duration,
    schema: u32,
    key: &str,
    fetch: F,
) -> Result<Cached<T>, CacheError<E>>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> Result<T, E>,
{
    get_or_fetch_with_clock(path, ttl, stale_max_age, schema, key, system_time, fetch)
}

fn system_time<E>() -> Result<u64, CacheError<E>> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CacheError::ClockBeforeUnixEpoch)
        .map(|duration| duration.as_secs())
}

fn get_or_fetch_at<T, E, F>(
    path: &Path,
    ttl: Duration,
    stale_max_age: Duration,
    schema: u32,
    key: &str,
    now: u64,
    fetch: F,
) -> Result<Cached<T>, CacheError<E>>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> Result<T, E>,
{
    get_or_fetch_with_clock(path, ttl, stale_max_age, schema, key, || Ok(now), fetch)
}

fn get_or_fetch_with_clock<T, E, F, C>(
    path: &Path,
    ttl: Duration,
    stale_max_age: Duration,
    schema: u32,
    key: &str,
    mut clock: C,
    fetch: F,
) -> Result<Cached<T>, CacheError<E>>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> Result<T, E>,
    C: FnMut() -> Result<u64, CacheError<E>>,
{
    let mut now = clock()?;
    let initial = read_matching(path, schema, key)?;
    if let Some(record) = initial.as_ref() {
        if age(now, record.fetched_at) <= ttl {
            return Ok(Cached {
                value: initial.expect("record was checked").value,
                status: CacheStatus::Fresh,
            });
        }
    }

    let _lock = match CacheLock::acquire(path) {
        Ok(lock) => lock,
        Err(CacheError::LockTimeout(lock_path)) => {
            // The holder may have refreshed the entry just before our timeout.
            // Re-read instead of falling back to the pre-wait snapshot.
            now = clock()?;
            if let Some(record) = read_matching(path, schema, key)? {
                let record_age = age(now, record.fetched_at);
                if record_age <= ttl {
                    return Ok(Cached {
                        value: record.value,
                        status: CacheStatus::Fresh,
                    });
                }
                if record_age <= stale_max_age {
                    return Ok(Cached {
                        value: record.value,
                        status: CacheStatus::StaleOnError,
                    });
                }
            }
            return Err(CacheError::LockTimeout(lock_path));
        }
        Err(error) => return Err(error),
    };

    // A competing process may have completed its refresh while we waited.
    now = clock()?;
    let old = read_matching(path, schema, key)?;
    if let Some(record) = old.as_ref() {
        if age(now, record.fetched_at) <= ttl {
            return Ok(Cached {
                value: old.expect("record was checked").value,
                status: CacheStatus::Fresh,
            });
        }
    }

    match fetch() {
        Ok(value) => {
            // A slow fetch must not receive the timestamp from before lock wait.
            now = clock()?;
            let record = CacheRecord {
                schema,
                fetched_at: now,
                key: key.to_owned(),
                value,
            };
            // A validated live result remains useful when the cache is full or
            // read-only. Persistence is best effort and never replaces the
            // provider result with a cache error.
            let _ = write_atomic::<CacheRecord<T>, E>(path, &record);
            Ok(Cached {
                value: record.value,
                status: CacheStatus::Refreshed,
            })
        }
        Err(error) => {
            now = clock()?;
            if let Some(record) = old {
                if age(now, record.fetched_at) <= stale_max_age {
                    return Ok(Cached {
                        value: record.value,
                        status: CacheStatus::StaleOnError,
                    });
                }
            }
            Err(CacheError::Fetch(error))
        }
    }
}

fn age(now: u64, fetched_at: u64) -> Duration {
    now.checked_sub(fetched_at)
        .map(Duration::from_secs)
        .unwrap_or(Duration::MAX)
}

fn managed_parent_is_safe(root: &Path, path: &Path) -> bool {
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(relative) = parent.strip_prefix(root) else {
        return false;
    };
    if !fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_dir()) {
        return false;
    }
    let mut current = root.to_path_buf();
    for part in relative.components() {
        use std::path::Component;
        let Component::Normal(name) = part else {
            return false;
        };
        current.push(name);
        if !fs::symlink_metadata(&current).is_ok_and(|metadata| metadata.file_type().is_dir()) {
            return false;
        }
    }
    true
}

fn read_matching<T: DeserializeOwned, E>(
    path: &Path,
    schema: u32,
    key: &str,
) -> Result<Option<CacheRecord<T>>, CacheError<E>> {
    let (file, file_len, previous_access) = if let Some(root) = cache_root_for(path) {
        let relative = path.strip_prefix(&root).map_err(|_| {
            CacheError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path is outside the managed cache",
            ))
        })?;
        let root_handle = match open_private_dir(&root) {
            Ok(handle) => handle,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(CacheError::Io(error)),
        };
        let metadata = match root_handle.symlink_metadata(relative) {
            Ok(metadata) if metadata.is_file() => metadata,
            Ok(_) => return Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(CacheError::Io(error)),
        };
        let file = match root_handle.open(relative) {
            Ok(file) => file.into_std(),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(CacheError::Io(error)),
        };
        (
            file,
            metadata.len(),
            metadata.accessed().ok().map(|time| time.into_std()),
        )
    } else if cfg!(test) {
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_file() => metadata,
            Ok(_) => return Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(CacheError::Io(error)),
        };
        let file = fs::File::open(path)?;
        (file, metadata.len(), metadata.accessed().ok())
    } else {
        return Ok(None);
    };
    if file_len > MAX_CACHE_FILE_BYTES {
        return Ok(None);
    }
    let mut bytes = Vec::new();
    (&file)
        .take(MAX_CACHE_FILE_BYTES + 1)
        .read_to_end(&mut bytes)?;
    let record = (|| -> Option<CacheRecord<T>> {
        if bytes.len() as u64 > MAX_CACHE_FILE_BYTES {
            return None;
        }
        let decoded = if let Some(compressed) = bytes.strip_prefix(COMPRESSED_MAGIC) {
            let mut decoded = Vec::new();
            MultiGzDecoder::new(compressed)
                .take(MAX_DECOMPRESSED_CACHE_BYTES + 1)
                .read_to_end(&mut decoded)
                .ok()?;
            if decoded.len() as u64 > MAX_DECOMPRESSED_CACHE_BYTES {
                return None;
            }
            decoded
        } else {
            bytes
        };
        let record: CacheRecord<T> = serde_json::from_slice(&decoded).ok()?;
        (record.schema == schema && record.key == key).then_some(record)
    })();
    match record {
        Some(record) => {
            let _ = file.set_times(std::fs::FileTimes::new().set_accessed(SystemTime::now()));
            Ok(Some(record))
        }
        None => {
            if let Some(accessed) = previous_access {
                let _ = file.set_times(std::fs::FileTimes::new().set_accessed(accessed));
            }
            Ok(None)
        }
    }
}

pub(crate) fn with_exclusive_lock<T>(
    path: &Path,
    action: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    let _lock = CacheLock::acquire::<String>(path).map_err(|error| error.to_string())?;
    action()
}

pub(crate) fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    write_atomic::<T, String>(path, value).map_err(|error| error.to_string())
}

/// Summary of all regular files below the cache root.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct CacheStats {
    pub bytes: u64,
    pub files: u64,
    pub limit_bytes: u64,
}

fn fallback_cache_dir() -> PathBuf {
    #[cfg(unix)]
    let scope = unsafe { libc::geteuid() }.to_string();
    #[cfg(not(unix))]
    let scope = std::env::var_os("USERNAME")
        .unwrap_or_default()
        .to_string_lossy()
        .bytes()
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
        .to_string();
    std::env::temp_dir().join(format!("quicksearch-{scope}"))
}

pub(crate) fn root_dir() -> PathBuf {
    let xdg = std::env::var_os("XDG_CACHE_HOME");
    let local = std::env::var_os("LOCALAPPDATA");
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from);
    let candidate = if let Some(dir) = xdg {
        PathBuf::from(dir).join("quicksearch")
    } else if cfg!(windows) {
        local
            .map(PathBuf::from)
            .map(|dir| dir.join("quicksearch"))
            .or_else(|| home.as_ref().map(|home| home.join(".cache/quicksearch")))
            .unwrap_or_else(fallback_cache_dir)
    } else if let Some(home) = home {
        if cfg!(target_os = "macos") {
            home.join("Library/Caches/quicksearch")
        } else {
            home.join(".cache/quicksearch")
        }
    } else {
        fallback_cache_dir()
    };
    canonicalize_cache_root(&candidate)
}

fn canonicalize_cache_root(path: &Path) -> PathBuf {
    let absolute = lexical_normalize(&if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    });
    let Some(name) = absolute.file_name() else {
        return absolute;
    };
    let Some(parent) = absolute.parent() else {
        return absolute;
    };
    // Canonicalize only the parent. Following the cache-root leaf here would
    // let an attacker swap that leaf to a symlink between inspection and
    // canonicalization and redefine the cache root to an outside directory.
    canonicalize_future_path(parent).join(name)
}

fn lexical_normalize(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(name) => normalized.push(name),
        }
    }
    normalized
}

fn canonicalize_future_path(path: &Path) -> PathBuf {
    let absolute = lexical_normalize(&if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    });
    if let Ok(canonical) = fs::canonicalize(&absolute) {
        return canonical;
    }
    let mut missing = Vec::new();
    let mut existing = absolute.as_path();
    while !existing.exists() {
        let Some(name) = existing.file_name() else {
            return absolute;
        };
        missing.push(name.to_os_string());
        let Some(parent) = existing.parent() else {
            return absolute;
        };
        existing = parent;
    }
    let mut canonical = fs::canonicalize(existing).unwrap_or_else(|_| existing.to_path_buf());
    for name in missing.into_iter().rev() {
        canonical.push(name);
    }
    canonical
}

fn prepare_cache_root(root: &Path) -> io::Result<()> {
    match fs::symlink_metadata(root) {
        Ok(metadata) if metadata.file_type().is_dir() => {}
        Ok(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache root is a symlink or non-directory",
            ));
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if let Some(parent) = root.parent() {
                fs::create_dir_all(parent)?;
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                let mut builder = fs::DirBuilder::new();
                builder.mode(0o700);
                match builder.create(root) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error),
                }
            }
            #[cfg(not(unix))]
            match fs::create_dir(root) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
            if !fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_dir()) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cache root is not a real directory",
                ));
            }
        }
        Err(error) => return Err(error),
    }
    let handle = open_private_dir(root)?;
    #[cfg(unix)]
    {
        use cap_std::fs::PermissionsExt;
        if handle.dir_metadata()?.permissions().mode() & 0o077 != 0 {
            handle.set_permissions(".", cap_std::fs::Permissions::from_mode(0o700))?;
        }
    }
    #[cfg(not(unix))]
    drop(handle);
    Ok(())
}

pub(crate) fn cache_stats(root: &Path) -> io::Result<CacheStats> {
    let files = regular_files(root)?;
    Ok(CacheStats {
        bytes: files.iter().map(|entry| entry.bytes).sum(),
        files: files.len() as u64,
        limit_bytes: CACHE_LIMIT_BYTES,
    })
}

/// Remove cache data while leaving active lock files alone.
pub(crate) fn clear_cache(root: &Path) -> Result<CacheStats, String> {
    clear_cache_report(root).map(|(_, after)| after)
}

pub(crate) fn clear_cache_report(root: &Path) -> Result<(CacheStats, CacheStats), String> {
    prepare_cache_root(root).map_err(|error| error.to_string())?;
    let control = root.join(".quota-control");
    let _lock = CacheLock::acquire::<String>(&control).map_err(|error| error.to_string())?;
    let before = cache_stats(root).map_err(|error| error.to_string())?;
    enforce_budget_locked::<String>(root, None, 0).map_err(|error| error.to_string())?;
    let root_handle = open_private_dir(root).map_err(|error| error.to_string())?;
    for entry in regular_files(root).map_err(|error| error.to_string())? {
        let name = entry
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if !(name.ends_with(".lock") || name.ends_with(".reap")) {
            match root_handle.remove_file(&entry.relative) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(format!(
                        "could not remove {}: {error}",
                        entry.path.display()
                    ));
                }
            }
        }
    }
    let after = cache_stats(root).map_err(|error| error.to_string())?;
    Ok((before, after))
}

/// Clean legacy artifacts and bring an existing cache below the data ceiling.
pub(crate) fn enforce_global_budget(root: &Path) -> Result<CacheStats, String> {
    prepare_cache_root(root).map_err(|error| error.to_string())?;
    let control = root.join(".quota-control");
    let _lock = CacheLock::acquire::<String>(&control).map_err(|error| error.to_string())?;
    enforce_budget_locked::<String>(root, None, 0).map_err(|error| error.to_string())?;
    cache_stats(root).map_err(|error| error.to_string())
}

#[derive(Debug)]
struct FileEntry {
    path: PathBuf,
    relative: PathBuf,
    bytes: u64,
    accessed: SystemTime,
}

pub(crate) fn open_private_dir(root: &Path) -> io::Result<Dir> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
            .open(root)?;
        use std::os::unix::fs::MetadataExt;
        let metadata = file.metadata()?;
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "private data directory is owned by another user",
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
        if !file.metadata()?.file_type().is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache root is a symlink or non-directory",
            ));
        }
        Ok(Dir::from_std_file(file))
    }
    #[cfg(not(any(unix, windows)))]
    {
        Dir::open_ambient_dir(root, cap_std::ambient_authority())
    }
}

pub(crate) fn sync_private_dir(dir: &Dir) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::fd::{AsRawFd, FromRawFd};
        let name = b".\0";
        let fd = unsafe {
            libc::openat(
                dir.as_raw_fd(),
                name.as_ptr().cast(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let file = unsafe { fs::File::from_raw_fd(fd) };
        file.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
        Ok(())
    }
}

fn regular_files(root: &Path) -> io::Result<Vec<FileEntry>> {
    let handle = open_private_dir(root)?;
    let mut files = Vec::new();
    collect_regular_files(&handle, Path::new(""), root, 0, &mut files)?;
    Ok(files)
}

fn collect_regular_files(
    dir: &Dir,
    relative_dir: &Path,
    root: &Path,
    depth: usize,
    files: &mut Vec<FileEntry>,
) -> io::Result<()> {
    if depth > MAX_CACHE_DEPTH || files.len() > MAX_CACHE_FILES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "cache tree exceeds its structural limits",
        ));
    }
    let entries = match dir.entries() {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        let name = entry.file_name();
        let relative = relative_dir.join(&name);
        let kind = match entry.file_type() {
            Ok(kind) => kind,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        if kind.is_dir() {
            match entry.open_dir() {
                Ok(child) => collect_regular_files(&child, &relative, root, depth + 1, files)?,
                Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error),
            }
        } else if kind.is_file() {
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error),
            };
            if !metadata.is_file() {
                continue;
            }
            if files.len() >= MAX_CACHE_FILES {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cache tree has too many files",
                ));
            }
            files.push(FileEntry {
                path: root.join(&relative),
                relative,
                bytes: metadata.len(),
                accessed: metadata
                    .accessed()
                    .or_else(|_| metadata.modified())
                    .map(|time| time.into_std())
                    .unwrap_or(UNIX_EPOCH),
            });
        }
    }
    Ok(())
}

fn remove_cache_symlinks(root: &Path) -> io::Result<()> {
    let handle = open_private_dir(root)?;
    remove_symlinks_beneath(&handle, 0)
}

fn remove_symlinks_beneath(dir: &Dir, depth: usize) -> io::Result<()> {
    if depth > MAX_CACHE_DEPTH {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "cache tree exceeds its depth limit",
        ));
    }
    let entries = match dir.entries() {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        let kind = match entry.file_type() {
            Ok(kind) => kind,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(error),
        };
        if kind.is_symlink() {
            match entry.remove_file() {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        } else if kind.is_dir() {
            match entry.open_dir() {
                Ok(child) => remove_symlinks_beneath(&child, depth + 1)?,
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(error),
            }
        }
    }
    Ok(())
}

fn is_control_file(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    name.ends_with(".lock") || name.ends_with(".reap") || name.contains(".tmp.")
}

fn is_forbidden_persistent_data(root: &Path, path: &Path) -> bool {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let components = relative
        .components()
        .filter_map(|part| part.as_os_str().to_str())
        .collect::<Vec<_>>();
    components.iter().any(|part| {
        matches!(
            *part,
            "fulltext" | "page-notes" | "examples" | "generated-examples"
        )
    }) || components
        .windows(2)
        .any(|parts| matches!(parts, ["python-docs-v1", "pages"]))
}

fn enforce_budget_locked<E>(
    root: &Path,
    protected: Option<&Path>,
    incoming_bytes: u64,
) -> Result<(), CacheError<E>> {
    remove_cache_symlinks(root)?;
    let root_handle = open_private_dir(root)?;
    let mut files = regular_files(root)?;
    for entry in &files {
        if is_control_file(&entry.path) {
            // Current-process locks live in the private runtime directory.
            // Any lock-shaped file inside the persistent cache is legacy or
            // hostile and cannot be allowed to bypass the global quota.
            let _ = root_handle.remove_file(&entry.relative);
        } else if is_forbidden_persistent_data(root, &entry.path)
            || (!protected.is_some_and(|path| entry.path == path)
                && (entry
                    .path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.contains(".tmp."))))
        {
            let _ = root_handle.remove_file(&entry.relative);
        }
    }
    files = regular_files(root)?;
    let mut total: u64 = files.iter().map(|entry| entry.bytes).sum();
    let mut candidates = files
        .into_iter()
        .filter(|entry| {
            !protected.is_some_and(|path| entry.path == path) && !is_control_file(&entry.path)
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        left.accessed
            .cmp(&right.accessed)
            .then_with(|| left.path.cmp(&right.path))
    });
    for entry in candidates {
        if total.saturating_add(incoming_bytes) <= CACHE_DATA_BUDGET_BYTES {
            break;
        }
        if root_handle.remove_file(&entry.relative).is_ok() {
            total = total.saturating_sub(entry.bytes);
        }
    }
    if total.saturating_add(incoming_bytes) > CACHE_DATA_BUDGET_BYTES {
        return Err(CacheError::QuotaExceeded {
            needed: incoming_bytes,
            available: CACHE_DATA_BUDGET_BYTES.saturating_sub(total),
        });
    }
    Ok(())
}

fn cache_root_for(path: &Path) -> Option<PathBuf> {
    let root = root_dir();
    if path.starts_with(&root) {
        return Some(root);
    }
    if cfg!(test) {
        return path.parent()?.ancestors().find_map(|ancestor| {
            ancestor
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("quicksearch-"))
                .then(|| ancestor.to_path_buf())
        });
    }
    None
}

pub(crate) fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<(), String> {
    write_bytes_atomic_inner::<String>(path, bytes).map_err(|error| error.to_string())
}

pub(crate) fn read_cache_text(path: &Path, limit: u64) -> Result<Option<String>, String> {
    let root =
        cache_root_for(path).ok_or_else(|| "path is outside the managed cache".to_string())?;
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| "path is outside the managed cache".to_string())?;
    if relative
        .components()
        .any(|part| !matches!(part, std::path::Component::Normal(_)))
    {
        return Err("cache path has an unsafe component".into());
    }
    match fs::symlink_metadata(&root) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
        Ok(_) => prepare_cache_root(&root).map_err(|error| error.to_string())?,
    }
    let root_handle = open_private_dir(&root).map_err(|error| error.to_string())?;
    let metadata = match root_handle.symlink_metadata(relative) {
        Ok(metadata) if metadata.is_file() => metadata,
        Ok(_) => return Ok(None),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
    };
    if metadata.len() > limit {
        return Err("legacy cache file is too large".into());
    }
    let mut file = root_handle
        .open(relative)
        .map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    (&mut file)
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > limit {
        return Err("legacy cache file is too large".into());
    }
    String::from_utf8(bytes)
        .map(Some)
        .map_err(|_| "legacy cache file is not UTF-8".into())
}

pub(crate) fn remove_cache_file(path: &Path) -> Result<(), String> {
    let root =
        cache_root_for(path).ok_or_else(|| "path is outside the managed cache".to_string())?;
    prepare_cache_root(&root).map_err(|error| error.to_string())?;
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| "path is outside the managed cache".to_string())?;
    if relative
        .components()
        .any(|part| !matches!(part, std::path::Component::Normal(_)))
    {
        return Err("cache path has an unsafe component".into());
    }
    let control = root.join(".quota-control");
    let _budget_lock = CacheLock::acquire::<String>(&control).map_err(|error| error.to_string())?;
    let root_handle = open_private_dir(&root).map_err(|error| error.to_string())?;
    match root_handle.remove_file(relative) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

struct BoundedBuffer {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for BoundedBuffer {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if self.bytes.len().saturating_add(input.len()) > self.limit {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "serialized cache entry exceeds the logical size limit",
            ));
        }
        self.bytes.extend_from_slice(input);
        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Write a complete side file and atomically replace the destination. A
/// root-wide lock serializes accounting, eviction, temp creation, and replace.
fn write_atomic<T: Serialize, E>(path: &Path, value: &T) -> Result<(), CacheError<E>> {
    let mut buffer = BoundedBuffer {
        bytes: Vec::new(),
        limit: MAX_DECOMPRESSED_CACHE_BYTES as usize,
    };
    serde_json::to_writer(&mut buffer, value)?;
    if buffer.bytes.len() == buffer.limit {
        return Err(CacheError::QuotaExceeded {
            needed: buffer.bytes.len() as u64 + 1,
            available: MAX_DECOMPRESSED_CACHE_BYTES,
        });
    }
    buffer.bytes.push(b'\n');
    let plain = buffer.bytes;
    let bytes = if plain.len() >= 64 * 1024 {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&plain)?;
        let compressed = encoder.finish()?;
        if compressed.len().saturating_add(COMPRESSED_MAGIC.len()) < plain.len() {
            let mut encoded = Vec::with_capacity(COMPRESSED_MAGIC.len() + compressed.len());
            encoded.extend_from_slice(COMPRESSED_MAGIC);
            encoded.extend_from_slice(&compressed);
            encoded
        } else {
            plain
        }
    } else {
        plain
    };
    write_bytes_atomic_inner(path, &bytes)
}

fn ensure_managed_parent(root: &Path, path: &Path) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "cache path has no parent"))?;
    let relative = parent.strip_prefix(root).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "cache path escapes the cache root",
        )
    })?;
    prepare_cache_root(root)?;
    let mut current = open_private_dir(root)?;
    for part in relative.components() {
        use std::path::Component;
        let Component::Normal(name) = part else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache path has an unsafe component",
            ));
        };
        match current.symlink_metadata(name) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cache path crosses a symlink or non-directory",
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                match current.create_dir(name) {
                    Ok(()) => {}
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error),
                }
            }
            Err(error) => return Err(error),
        }
        current = current.open_dir(name)?;
    }
    Ok(())
}

fn write_bytes_atomic_inner<E>(path: &Path, bytes: &[u8]) -> Result<(), CacheError<E>> {
    let incoming = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if incoming > MAX_ENTRY_BYTES {
        return Err(CacheError::QuotaExceeded {
            needed: incoming,
            available: MAX_ENTRY_BYTES,
        });
    }
    let root = cache_root_for(path);
    let _budget_lock = if let Some(root) = &root {
        if is_forbidden_persistent_data(root, path) {
            return Err(CacheError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "this cache payload class is not allowed to persist",
            )));
        }
        ensure_managed_parent(root, path)?;
        let control = root.join(".quota-control");
        let lock = CacheLock::acquire::<E>(&control)?;
        enforce_budget_locked(root, Some(path), incoming)?;
        Some(lock)
    } else if cfg!(test) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        None
    } else {
        return Err(CacheError::Io(io::Error::new(
            io::ErrorKind::InvalidInput,
            "cache write is outside the canonical quicksearch root",
        )));
    };

    if let Some(root) = &root {
        let relative = path.strip_prefix(root).map_err(|_| {
            CacheError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache path escapes the cache root",
            ))
        })?;
        let parent = relative.parent().unwrap_or_else(|| Path::new(""));
        let file_name = relative.file_name().ok_or_else(|| {
            CacheError::Io(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cache path has no file name",
            ))
        })?;
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let temp_name = format!(
            ".{}.tmp.{}.{}",
            file_name.to_string_lossy(),
            std::process::id(),
            sequence
        );
        let temp_relative = parent.join(temp_name);
        let root_handle = open_private_dir(root)?;
        let result = (|| -> Result<(), CacheError<E>> {
            let mut options = cap_std::fs::OpenOptions::new();
            options.write(true).create_new(true);
            let mut file = root_handle.open_with(&temp_relative, &options)?;
            file.write_all(bytes)?;
            file.flush()?;
            file.sync_all()?;
            #[cfg(test)]
            assert!(cache_stats(root)?.bytes <= CACHE_LIMIT_BYTES);
            root_handle.rename(&temp_relative, &root_handle, relative)?;
            if parent.as_os_str().is_empty() {
                sync_private_dir(&root_handle)?;
            } else {
                let parent_handle = root_handle.open_dir(parent)?;
                sync_private_dir(&parent_handle)?;
            }
            Ok(())
        })();
        if result.is_err() {
            let _ = root_handle.remove_file(&temp_relative);
        }
        return result;
    }

    let temp = unique_sidecar(path, "tmp");
    let result = (|| -> Result<(), CacheError<E>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        file.write_all(bytes)?;
        file.flush()?;
        file.sync_all()?;
        replace(&temp, path)?;
        sync_parent(path)?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

#[cfg(not(windows))]
fn replace(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}

#[cfg(not(windows))]
fn sync_parent(path: &Path) -> io::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    fs::File::open(parent)?.sync_all()
}

// `std::fs::rename` cannot replace an existing Windows file. MoveFileExW with
// both flags provides the platform's replace operation and asks Windows not to
// return until the move has reached disk. It also avoids the old remove/rename
// gap in which readers observed no cache file.
#[cfg(windows)]
fn replace(from: &Path, to: &Path) -> io::Result<()> {
    use std::os::windows::ffi::OsStrExt;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x1;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x8;

    #[link(name = "kernel32")]
    extern "system" {
        fn MoveFileExW(existing: *const u16, replacement: *const u16, flags: u32) -> i32;
    }

    let existing: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
    let replacement: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: Both pointers refer to live, NUL-terminated UTF-16 buffers for
    // the duration of the call. The flag values are defined by WinBase.h.
    let succeeded = unsafe {
        MoveFileExW(
            existing.as_ptr(),
            replacement.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if succeeded == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn sync_parent(_path: &Path) -> io::Result<()> {
    // MOVEFILE_WRITE_THROUGH above supplies the Windows durability request.
    Ok(())
}

fn lock_path(cache_path: &Path) -> io::Result<PathBuf> {
    #[cfg(unix)]
    let user_scope = u64::from(unsafe { libc::geteuid() });
    #[cfg(not(unix))]
    let user_scope = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .unwrap_or_default()
        .to_string_lossy()
        .bytes()
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        });
    let fallback_base =
        || std::env::temp_dir().join(format!("quicksearch-cache-locks-{user_scope:016x}"));
    let runtime_base = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .map(|dir| dir.join("quicksearch-cache-locks"))
        .unwrap_or_else(&fallback_base);
    let cache_root = root_dir();
    let runtime_lexical = lexical_normalize(&runtime_base);
    let root_lexical = lexical_normalize(&cache_root);
    let runtime_canonical = canonicalize_future_path(&runtime_base);
    let root_canonical = canonicalize_future_path(&cache_root);
    let requested_base = if runtime_lexical.starts_with(&root_lexical)
        || runtime_canonical.starts_with(&root_canonical)
    {
        fallback_base()
    } else {
        runtime_base
    };
    if !requested_base.exists() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(&requested_base)?;
        }
        #[cfg(not(unix))]
        fs::create_dir_all(&requested_base)?;
    }
    let metadata = fs::symlink_metadata(&requested_base)?;
    if !metadata.file_type().is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "cache lock directory is not a real directory",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
        let dir = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
            .open(&requested_base)?;
        let metadata = dir.metadata()?;
        if metadata.uid() != unsafe { libc::geteuid() } {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cache lock directory is owned by another user",
            ));
        }
        dir.set_permissions(fs::Permissions::from_mode(0o700))?;
        if dir.metadata()?.permissions().mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "cache lock directory is accessible by other users",
            ));
        }
    }
    let base = fs::canonicalize(&requested_base)?;
    // A lock names the caller's logical cache path. Never follow the mutable
    // cache leaf or any mutable descendant while deriving that identity.
    let key = lexical_normalize(&if cache_path.is_absolute() {
        cache_path.to_path_buf()
    } else {
        std::env::current_dir()?.join(cache_path)
    });
    let text = key.as_os_str().to_string_lossy();
    let first = text.bytes().fold(0xcbf29ce484222325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
    });
    let second = text
        .bytes()
        .rev()
        .fold(0x84222325cbf29ce4_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        });
    Ok(base.join(format!("{first:016x}{second:016x}.lock")))
}

fn try_create_identity_file(path: &Path) -> io::Result<Option<Vec<u8>>> {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let identity = format!("{} {sequence}\n", std::process::id()).into_bytes();
    let candidate = unique_sidecar(path, "claim");
    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&candidate)?;
        file.write_all(&identity)?;
        file.flush()?;
        file.sync_all()?;
        match fs::hard_link(&candidate, path) {
            Ok(()) => Ok(Some(identity)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(None),
            Err(error) => Err(error),
        }
    })();
    let _ = fs::remove_file(candidate);
    result
}

struct CacheLock {
    path: PathBuf,
    identity: Vec<u8>,
}

impl CacheLock {
    fn acquire<E>(cache_path: &Path) -> Result<Self, CacheError<E>> {
        Self::acquire_with_timeout(cache_path, LOCK_TIMEOUT)
    }

    fn acquire_with_timeout<E>(
        cache_path: &Path,
        timeout: Duration,
    ) -> Result<Self, CacheError<E>> {
        let path = lock_path(cache_path)?;
        let started = std::time::Instant::now();
        loop {
            match try_create_identity_file(&path) {
                Ok(Some(identity)) => return Ok(Self { path, identity }),
                Ok(None) => {
                    if let Some(identity) = stale_lock_identity(&path) {
                        if reclaim_lock_if_unchanged(&path, &identity) {
                            continue;
                        }
                    }
                    if started.elapsed() >= timeout {
                        return Err(CacheError::LockTimeout(path));
                    }
                    thread::sleep(LOCK_RETRY);
                }
                Err(error) => return Err(CacheError::Io(error)),
            }
        }
    }
}

fn owner_is_proven_dead(owner_is_alive: Option<bool>) -> bool {
    matches!(owner_is_alive, Some(false))
}

fn stale_lock_identity(path: &Path) -> Option<Vec<u8>> {
    #[cfg(target_os = "linux")]
    {
        stale_lock_identity_with_liveness(path, linux_owner_liveness)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = path;
        // There is no portable, trustworthy owner-liveness check. Failing
        // closed can require manual cleanup after a crash, but it cannot steal
        // a lock from a slow active process merely because the lock is old.
        owner_is_proven_dead(None).then(Vec::new)
    }
}

#[cfg(target_os = "linux")]
fn stale_lock_identity_with_liveness(
    path: &Path,
    owner_liveness: impl FnOnce(u32) -> Option<bool>,
) -> Option<Vec<u8>> {
    let identity = read_lock_identity(path, 128)?;
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

fn read_lock_identity(path: &Path, limit: u64) -> Option<Vec<u8>> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let mut file = options.open(path).ok()?;
    let metadata = file.metadata().ok()?;
    if !metadata.file_type().is_file() || metadata.len() > limit {
        return None;
    }
    let mut identity = Vec::new();
    (&mut file)
        .take(limit.saturating_add(1))
        .read_to_end(&mut identity)
        .ok()?;
    (identity.len() as u64 <= limit).then_some(identity)
}

#[cfg(target_os = "linux")]
struct RecoveryGuard {
    dir: fs::File,
}

#[cfg(target_os = "linux")]
impl RecoveryGuard {
    fn acquire(path: &Path) -> Option<Self> {
        use std::os::fd::AsRawFd;
        use std::os::unix::fs::OpenOptionsExt;

        let parent = path.parent()?;
        let dir = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC | libc::O_DIRECTORY | libc::O_NOFOLLOW)
            .open(parent)
            .ok()?;
        if unsafe { libc::flock(dir.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return None;
        }
        Some(Self { dir })
    }
}

#[cfg(target_os = "linux")]
impl Drop for RecoveryGuard {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd;

        let _ = unsafe { libc::flock(self.dir.as_raw_fd(), libc::LOCK_UN) };
    }
}

struct ReapClaim {
    path: PathBuf,
    identity: Vec<u8>,
}

impl ReapClaim {
    fn acquire(path: &Path) -> Option<Self> {
        for _ in 0..2 {
            match try_create_identity_file(path) {
                Ok(Some(identity)) => {
                    return Some(Self {
                        path: path.to_path_buf(),
                        identity,
                    });
                }
                Ok(None) => {
                    let stale = stale_lock_identity(path)?;
                    remove_lock_if_unchanged(path, &stale);
                }
                Err(_) => return None,
            }
        }
        None
    }
}

impl Drop for ReapClaim {
    fn drop(&mut self) {
        let _ = remove_lock_if_unchanged(&self.path, &self.identity);
    }
}

fn reclaim_lock_if_unchanged(path: &Path, expected_identity: &[u8]) -> bool {
    #[cfg(target_os = "linux")]
    {
        // Serialize every automatic stale-generation unlink in this private
        // directory. Recheck both identity and owner liveness while guarded.
        let Some(_guard) = RecoveryGuard::acquire(path) else {
            return false;
        };
        let Some(current_identity) = stale_lock_identity(path) else {
            return false;
        };
        if current_identity != expected_identity {
            return false;
        }
        let claim_path = sidecar(path, "reap");
        let Some(_claim) = ReapClaim::acquire(&claim_path) else {
            return false;
        };
        // The claim drops before the directory guard. Abandoned claims are
        // recovered only while this same directory guard is held.
        remove_lock_if_unchanged(path, &current_identity)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (path, expected_identity);
        false
    }
}

fn remove_lock_if_unchanged(path: &Path, expected_identity: &[u8]) -> bool {
    // The identity prevents an old guard or a stale-lock observer from deleting
    // a replacement lock in the common release/reacquire race.
    !expected_identity.is_empty()
        && read_lock_identity(path, 128).is_some_and(|identity| identity == expected_identity)
        && fs::remove_file(path).is_ok()
}

impl Drop for CacheLock {
    fn drop(&mut self) {
        let _ = remove_lock_if_unchanged(&self.path, &self.identity);
    }
}

fn sidecar(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(format!(".{suffix}"));
    PathBuf::from(name)
}

fn unique_sidecar(path: &Path, suffix: &str) -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut name = path.as_os_str().to_owned();
    name.push(format!(".{suffix}.{}.{}", std::process::id(), sequence));
    PathBuf::from(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    const TTL: Duration = Duration::from_secs(10);
    const STALE_MAX_AGE: Duration = Duration::from_secs(30);

    fn test_dir(name: &str) -> PathBuf {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "quicksearch-cache-test-{name}-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn get<F>(path: &Path, now: u64, fetch: F) -> Result<Cached<String>, CacheError<&'static str>>
    where
        F: FnOnce() -> Result<String, &'static str>,
    {
        get_or_fetch_at(path, TTL, STALE_MAX_AGE, 3, "docs", now, fetch)
    }

    #[test]
    fn lock_recovery_requires_proof_that_the_owner_is_dead() {
        assert!(owner_is_proven_dead(Some(false)));
        assert!(!owner_is_proven_dead(Some(true)));
        assert!(!owner_is_proven_dead(None));
    }

    #[cfg(target_os = "linux")]
    fn cache_lock_times_out(cache_path: &Path) {
        assert!(matches!(
            CacheLock::acquire_with_timeout::<String>(cache_path, Duration::from_millis(25)),
            Err(CacheError::LockTimeout(_))
        ));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn hostile_cache_lock_entries_time_out_and_remain_untouched() {
        use std::os::unix::fs::FileTypeExt;
        use std::os::unix::net::UnixListener;

        let malformed_dir = test_dir("malformed-lock");
        let malformed_cache = malformed_dir.join("entry.json");
        let malformed_lock = lock_path(&malformed_cache).unwrap();
        fs::write(&malformed_lock, b"malformed").unwrap();
        cache_lock_times_out(&malformed_cache);
        assert_eq!(fs::read(&malformed_lock).unwrap(), b"malformed");
        fs::remove_file(&malformed_lock).unwrap();
        fs::remove_dir_all(malformed_dir).unwrap();

        let oversized_dir = test_dir("oversized-lock");
        let oversized_cache = oversized_dir.join("entry.json");
        let oversized_lock = lock_path(&oversized_cache).unwrap();
        fs::write(&oversized_lock, vec![b'x'; 129]).unwrap();
        cache_lock_times_out(&oversized_cache);
        assert_eq!(fs::metadata(&oversized_lock).unwrap().len(), 129);
        fs::remove_file(&oversized_lock).unwrap();
        fs::remove_dir_all(oversized_dir).unwrap();

        let symlink_dir = test_dir("symlink-lock");
        let symlink_cache = symlink_dir.join("entry.json");
        let symlink_lock = lock_path(&symlink_cache).unwrap();
        let target = symlink_dir.join("outside");
        fs::write(&target, b"outside").unwrap();
        std::os::unix::fs::symlink(&target, &symlink_lock).unwrap();
        cache_lock_times_out(&symlink_cache);
        assert!(fs::symlink_metadata(&symlink_lock)
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(fs::read(&target).unwrap(), b"outside");
        fs::remove_file(&symlink_lock).unwrap();
        fs::remove_dir_all(symlink_dir).unwrap();

        let special_dir = test_dir("special-lock");
        let special_cache = special_dir.join("entry.json");
        let special_lock = lock_path(&special_cache).unwrap();
        let listener = UnixListener::bind(&special_lock).unwrap();
        cache_lock_times_out(&special_cache);
        assert!(fs::symlink_metadata(&special_lock)
            .unwrap()
            .file_type()
            .is_socket());
        drop(listener);
        fs::remove_file(&special_lock).unwrap();
        fs::remove_dir_all(special_dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn hostile_reaper_entries_time_out_and_remain_untouched() {
        use std::os::unix::fs::FileTypeExt;
        use std::os::unix::net::UnixListener;

        let malformed_dir = test_dir("malformed-reaper");
        let malformed_cache = malformed_dir.join("entry.json");
        let malformed_lock = lock_path(&malformed_cache).unwrap();
        let malformed_reaper = sidecar(&malformed_lock, "reap");
        fs::write(
            &malformed_lock,
            b"4294967295 1
",
        )
        .unwrap();
        fs::write(&malformed_reaper, b"malformed").unwrap();
        cache_lock_times_out(&malformed_cache);
        assert_eq!(
            fs::read(&malformed_lock).unwrap(),
            b"4294967295 1
"
        );
        assert_eq!(fs::read(&malformed_reaper).unwrap(), b"malformed");
        fs::remove_file(&malformed_lock).unwrap();
        fs::remove_file(&malformed_reaper).unwrap();
        fs::remove_dir_all(malformed_dir).unwrap();

        let oversized_dir = test_dir("oversized-reaper");
        let oversized_cache = oversized_dir.join("entry.json");
        let oversized_lock = lock_path(&oversized_cache).unwrap();
        let oversized_reaper = sidecar(&oversized_lock, "reap");
        fs::write(&oversized_lock, b"4294967295 2\n").unwrap();
        fs::write(&oversized_reaper, vec![b'x'; 129]).unwrap();
        cache_lock_times_out(&oversized_cache);
        assert_eq!(fs::read(&oversized_lock).unwrap(), b"4294967295 2\n");
        assert_eq!(fs::metadata(&oversized_reaper).unwrap().len(), 129);
        fs::remove_file(&oversized_lock).unwrap();
        fs::remove_file(&oversized_reaper).unwrap();
        fs::remove_dir_all(oversized_dir).unwrap();

        let symlink_dir = test_dir("symlink-reaper");
        let symlink_cache = symlink_dir.join("entry.json");
        let symlink_lock = lock_path(&symlink_cache).unwrap();
        let symlink_reaper = sidecar(&symlink_lock, "reap");
        let target = symlink_dir.join("outside");
        fs::write(&symlink_lock, b"4294967295 3\n").unwrap();
        fs::write(&target, b"outside").unwrap();
        std::os::unix::fs::symlink(&target, &symlink_reaper).unwrap();
        cache_lock_times_out(&symlink_cache);
        assert_eq!(fs::read(&symlink_lock).unwrap(), b"4294967295 3\n");
        assert!(fs::symlink_metadata(&symlink_reaper)
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(fs::read(&target).unwrap(), b"outside");
        fs::remove_file(&symlink_lock).unwrap();
        fs::remove_file(&symlink_reaper).unwrap();
        fs::remove_dir_all(symlink_dir).unwrap();

        let special_dir = test_dir("special-reaper");
        let special_cache = special_dir.join("entry.json");
        let special_lock = lock_path(&special_cache).unwrap();
        let special_reaper = sidecar(&special_lock, "reap");
        fs::write(
            &special_lock,
            b"4294967295 2
",
        )
        .unwrap();
        let listener = UnixListener::bind(&special_reaper).unwrap();
        cache_lock_times_out(&special_cache);
        assert_eq!(
            fs::read(&special_lock).unwrap(),
            b"4294967295 2
"
        );
        assert!(fs::symlink_metadata(&special_reaper)
            .unwrap()
            .file_type()
            .is_socket());
        drop(listener);
        fs::remove_file(&special_lock).unwrap();
        fs::remove_file(&special_reaper).unwrap();
        fs::remove_dir_all(special_dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn unknown_cache_lock_owner_is_left_untouched() {
        let dir = test_dir("unknown-linux-owner");
        let path = dir.join("entry.json.lock");
        let identity = b"42 7
";
        fs::write(&path, identity).unwrap();
        assert_eq!(stale_lock_identity_with_liveness(&path, |_| None), None);
        assert_eq!(fs::read(&path).unwrap(), identity);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(not(target_os = "linux"))]
    #[test]
    fn lock_files_with_unknown_owners_are_left_untouched() {
        let dir = test_dir("unknown-lock-owner");
        let path = dir.join("entry.json.lock");

        for identity in [b"4294967295 0\n".as_slice(), b"malformed".as_slice()] {
            fs::write(&path, identity).unwrap();
            assert_eq!(stale_lock_identity(&path), None);
            assert_eq!(fs::read(&path).unwrap(), identity);
        }
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn fresh_hit_avoids_fetch() {
        let dir = test_dir("fresh");
        let path = dir.join("entry.json");
        get(&path, 100, || Ok("one".into())).unwrap();
        let result = get(&path, 105, || panic!("fresh cache must not fetch")).unwrap();
        assert_eq!(
            result,
            Cached {
                value: "one".into(),
                status: CacheStatus::Fresh
            }
        );
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn expired_record_is_refreshed() {
        let dir = test_dir("refresh");
        let path = dir.join("entry.json");
        get(&path, 100, || Ok("one".into())).unwrap();
        let result = get(&path, 111, || Ok("two".into())).unwrap();
        assert_eq!(
            result,
            Cached {
                value: "two".into(),
                status: CacheStatus::Refreshed
            }
        );
        let record: CacheRecord<String> =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!((record.value.as_str(), record.fetched_at), ("two", 111));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn expired_fetch_error_returns_bounded_stale_value() {
        let dir = test_dir("stale");
        let path = dir.join("entry.json");
        get(&path, 100, || Ok("one".into())).unwrap();
        let stale = get(&path, 130, || Err("invalid response")).unwrap();
        assert_eq!(
            stale,
            Cached {
                value: "one".into(),
                status: CacheStatus::StaleOnError
            }
        );
        let too_old = get(&path, 131, || Err("offline"));
        assert!(matches!(too_old, Err(CacheError::Fetch("offline"))));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn invalid_fetched_value_cannot_overwrite_stale_record() {
        let dir = test_dir("invalid-fetch");
        let path = dir.join("entry.json");
        get(&path, 100, || Ok("valid".into())).unwrap();
        let result = get(&path, 120, || Err("validation failed")).unwrap();
        assert_eq!(result.status, CacheStatus::StaleOnError);
        let record: CacheRecord<String> =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!((record.value.as_str(), record.fetched_at), ("valid", 100));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn corrupt_wrong_schema_and_wrong_key_are_ignored() {
        let cases = [
            ("corrupt", "not json"),
            (
                "schema",
                r#"{"schema":2,"fetched_at":100,"key":"docs","value":"old"}"#,
            ),
            (
                "key",
                r#"{"schema":3,"fetched_at":100,"key":"other","value":"old"}"#,
            ),
        ];
        for (name, contents) in cases {
            let dir = test_dir(name);
            let path = dir.join("entry.json");
            fs::write(&path, contents).unwrap();
            let result = get(&path, 101, || Ok("new".into())).unwrap();
            assert_eq!(result.value, "new");
            let record: CacheRecord<String> =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert_eq!(record.value, "new");
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn unusable_record_does_not_become_stale_fallback() {
        let dir = test_dir("bad-fallback");
        let path = dir.join("entry.json");
        fs::write(&path, "not json").unwrap();
        assert!(matches!(
            get(&path, 101, || Err("offline")),
            Err(CacheError::Fetch("offline"))
        ));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn future_timestamp_is_not_treated_as_fresh() {
        assert_eq!(age(100, 101), Duration::MAX);
    }

    #[test]
    fn future_record_is_neither_fresh_nor_a_stale_fallback() {
        let dir = test_dir("future-record");
        let path = dir.join("entry.json");
        let record = CacheRecord {
            schema: 3,
            fetched_at: 101,
            key: "docs".to_owned(),
            value: "future".to_owned(),
        };
        fs::write(&path, serde_json::to_vec(&record).unwrap()).unwrap();
        assert!(matches!(
            get(&path, 100, || Err("offline")),
            Err(CacheError::Fetch("offline"))
        ));
        let refreshed = get(&path, 100, || Ok("present".into())).unwrap();
        assert_eq!(refreshed.status, CacheStatus::Refreshed);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn refresh_timestamp_is_taken_after_wait_and_fetch() {
        let dir = test_dir("late-timestamp");
        let path = dir.join("entry.json");
        let mut times = [100, 101, 150].into_iter();
        let result = get_or_fetch_with_clock(
            &path,
            TTL,
            STALE_MAX_AGE,
            3,
            "docs",
            || Ok(times.next().unwrap()),
            || Ok::<_, &'static str>("new".to_owned()),
        )
        .unwrap();
        assert_eq!(result.status, CacheStatus::Refreshed);
        let record: CacheRecord<String> =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(record.fetched_at, 150);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn old_guard_does_not_remove_replacement_lock() {
        let dir = test_dir("lock-identity");
        let path = dir.join("entry.json");
        let lock = CacheLock::acquire::<&'static str>(&path).unwrap();
        let lock_path = lock_path(&path).unwrap();
        fs::remove_file(&lock_path).unwrap();
        fs::write(&lock_path, b"replacement\n").unwrap();
        drop(lock);
        assert_eq!(fs::read(&lock_path).unwrap(), b"replacement\n");
        fs::remove_file(lock_path).unwrap();
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[cfg(target_os = "linux")]
    #[test]
    fn large_metadata_records_are_compressed_and_validated() {
        let dir = test_dir("compressed-record");
        let path = dir.join("metadata.json");
        let value = vec!["repeated metadata".to_string(); 50_000];
        get_or_fetch_at(&path, TTL, STALE_MAX_AGE, 7, "large-index", 100, || {
            Ok::<_, &'static str>(value.clone())
        })
        .unwrap();
        let bytes = fs::read(&path).unwrap();
        assert!(bytes.starts_with(COMPRESSED_MAGIC));
        assert!(bytes.len() < 100_000);
        let loaded = read_fresh::<Vec<String>>(&path, Duration::MAX, 7, "large-index").unwrap();
        assert_eq!(loaded.len(), value.len());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn highly_compressible_logical_bomb_is_not_persisted() {
        let dir = test_dir("logical-bomb");
        let path = dir.join("bomb.json");
        let value = "x".repeat(MAX_DECOMPRESSED_CACHE_BYTES as usize + 1);
        let result = get_or_fetch_at(&path, TTL, STALE_MAX_AGE, 8, "logical-bomb", 100, || {
            Ok::<_, &'static str>(value)
        })
        .unwrap();
        assert_eq!(
            result.value.len(),
            MAX_DECOMPRESSED_CACHE_BYTES as usize + 1
        );
        assert!(!path.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn oversized_entry_is_not_persisted() {
        let dir = test_dir("oversized");
        let path = dir.join("too-large.bin");
        let bytes = vec![b'x'; MAX_ENTRY_BYTES as usize + 1];
        assert!(write_bytes_atomic(&path, &bytes).is_err());
        assert!(!path.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn invalid_reads_do_not_promote_an_entry_in_lru_order() {
        let dir = test_dir("invalid-lru");
        let corrupt = dir.join("corrupt.json");
        let valid = dir.join("valid.bin");
        let filler = dir.join("filler.bin");
        let incoming = dir.join("incoming.bin");
        fs::write(&corrupt, vec![b'x'; 1024 * 1024]).unwrap();
        write_bytes_atomic(&valid, &vec![b'v'; 1024 * 1024]).unwrap();
        write_bytes_atomic(&filler, &vec![b'f'; 1024 * 1024]).unwrap();
        for (path, seconds) in [(&corrupt, 1), (&valid, 2), (&filler, 3)] {
            let time = UNIX_EPOCH + Duration::from_secs(seconds);
            fs::File::open(path)
                .unwrap()
                .set_times(fs::FileTimes::new().set_accessed(time).set_modified(time))
                .unwrap();
        }
        assert!(read_fresh::<String>(&corrupt, TTL, 1, "wrong").is_none());
        write_bytes_atomic(&incoming, &vec![b'i'; 1024 * 1024]).unwrap();
        assert!(!corrupt.exists());
        assert!(valid.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn least_recently_used_entry_is_evicted_before_commit() {
        let dir = test_dir("lru");
        let paths = (0..4)
            .map(|index| dir.join(format!("entry-{index}.bin")))
            .collect::<Vec<_>>();
        let bytes = vec![b'x'; 1024 * 1024];
        for (index, path) in paths.iter().take(3).enumerate() {
            write_bytes_atomic(path, &bytes).unwrap();
            let time = UNIX_EPOCH + Duration::from_secs(index as u64 + 1);
            fs::File::open(path)
                .unwrap()
                .set_times(fs::FileTimes::new().set_accessed(time).set_modified(time))
                .unwrap();
        }
        write_bytes_atomic(&paths[3], &bytes).unwrap();
        assert!(!paths[0].exists());
        assert!(paths[1..].iter().all(|path| path.exists()));
        assert!(cache_stats(&dir).unwrap().bytes <= CACHE_LIMIT_BYTES);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn concurrent_provider_writes_never_cross_the_global_limit() {
        use std::sync::atomic::{AtomicBool, AtomicU64};

        let dir = test_dir("global-concurrency");
        let stop = Arc::new(AtomicBool::new(false));
        let maximum = Arc::new(AtomicU64::new(0));
        let monitor_dir = dir.clone();
        let monitor_stop = Arc::clone(&stop);
        let monitor_maximum = Arc::clone(&maximum);
        let monitor = std::thread::spawn(move || {
            while !monitor_stop.load(Ordering::Relaxed) {
                let control = monitor_dir.join(".quota-control");
                if let Ok(_lock) = CacheLock::acquire::<String>(&control) {
                    if let Ok(stats) = cache_stats(&monitor_dir) {
                        monitor_maximum.fetch_max(stats.bytes, Ordering::Relaxed);
                    }
                }
                std::thread::yield_now();
            }
        });
        let mut writers = Vec::new();
        for provider in 0..24 {
            let writer_dir = dir.clone();
            writers.push(std::thread::spawn(move || {
                let path = writer_dir
                    .join(format!("provider-{provider}"))
                    .join("index.json");
                let bytes = vec![b'a' + (provider % 20) as u8; 320 * 1024];
                write_bytes_atomic(&path, &bytes).unwrap();
            }));
        }
        for writer in writers {
            writer.join().unwrap();
        }
        stop.store(true, Ordering::Relaxed);
        monitor.join().unwrap();
        let stats = cache_stats(&dir).unwrap();
        assert!(stats.bytes <= CACHE_LIMIT_BYTES, "{stats:?}");
        assert!(maximum.load(Ordering::Relaxed) <= CACHE_LIMIT_BYTES);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn clear_tolerates_concurrent_file_disappearance() {
        for round in 0..8 {
            let dir = test_dir(&format!("clear-enoent-{round}"));
            let mut paths = Vec::new();
            for index in 0..128 {
                let path = dir.join(format!("provider/{index}.bin"));
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(&path, b"x").unwrap();
                paths.push(path);
            }
            let remover = std::thread::spawn(move || {
                for path in paths {
                    let _ = fs::remove_file(path);
                    std::thread::yield_now();
                }
            });
            assert!(clear_cache(&dir).is_ok());
            remover.join().unwrap();
            fs::remove_dir_all(dir).unwrap();
        }
    }

    #[test]
    fn enforcement_removes_forbidden_corpora_and_clear_is_idempotent() {
        let dir = test_dir("forbidden-and-clear");
        let corpus = dir.join("devdocs-v1/fulltext/javascript.json");
        fs::create_dir_all(corpus.parent().unwrap()).unwrap();
        fs::write(&corpus, vec![0_u8; 1024]).unwrap();
        let notes = dir.join("devdocs-v1/page-notes/a.json");
        fs::create_dir_all(notes.parent().unwrap()).unwrap();
        fs::write(&notes, b"note").unwrap();
        let generated = dir.join("generated-examples/rust.json");
        fs::create_dir_all(generated.parent().unwrap()).unwrap();
        fs::write(&generated, b"generated code").unwrap();
        let hostile_lock = dir.join("adversarial.lock");
        fs::write(&hostile_lock, vec![0_u8; 5 * 1024 * 1024]).unwrap();
        let stats = enforce_global_budget(&dir).unwrap();
        assert!(stats.bytes <= CACHE_LIMIT_BYTES);
        assert!(!corpus.exists());
        assert!(!notes.exists());
        assert!(!generated.exists());
        assert!(!hostile_lock.exists());
        assert!(write_bytes_atomic(&corpus, b"forbidden").is_err());
        assert!(!corpus.exists());
        assert_eq!(clear_cache(&dir).unwrap().bytes, 0);
        assert_eq!(clear_cache(&dir).unwrap().bytes, 0);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn root_identity_never_follows_the_cache_leaf() {
        use std::os::unix::fs::symlink;

        let parent = test_dir("root-identity-parent");
        let outside = test_dir("root-identity-outside");
        let link = parent.join("quicksearch");
        symlink(&outside, &link).unwrap();
        assert_eq!(canonicalize_cache_root(&link), link);
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn cache_admin_does_not_follow_a_root_symlink() {
        use std::os::unix::fs::symlink;

        let parent = test_dir("symlink-root-parent");
        let outside = test_dir("symlink-root-victim");
        let victim = outside.join("victim");
        fs::write(&victim, b"keep").unwrap();
        let linked_root = parent.join("linked-root");
        symlink(&outside, &linked_root).unwrap();
        assert!(clear_cache(&linked_root).is_err());
        assert_eq!(fs::read(&victim).unwrap(), b"keep");
        fs::remove_dir_all(parent).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn cache_removal_does_not_follow_subdirectory_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("symlink-remove-parent");
        let outside = test_dir("symlink-remove-outside");
        let victim = outside.join("victim.json");
        fs::write(&victim, b"keep").unwrap();
        symlink(&outside, dir.join("provider")).unwrap();
        assert!(remove_cache_file(&dir.join("provider/victim.json")).is_err());
        assert_eq!(fs::read(&victim).unwrap(), b"keep");
        fs::remove_dir_all(dir).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

    #[test]
    fn future_lexical_aliases_keep_one_lock_identity() {
        let dir = test_dir("future-alias-lock");
        let alias = dir.join("missing/../real/entry.json");
        let before = lock_path(&alias).unwrap();
        fs::create_dir_all(dir.join("real")).unwrap();
        let after = lock_path(&alias).unwrap();
        assert_eq!(before, after);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn mutable_quota_control_leaf_keeps_one_lock_identity() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("quota-control-lock-identity");
        let control = dir.join(".quota-control");
        let target = dir.join("target");
        fs::write(&target, b"target").unwrap();

        let missing = lock_path(&control).unwrap();
        symlink(&target, &control).unwrap();
        let symlinked = lock_path(&control).unwrap();
        fs::remove_file(&control).unwrap();
        let deleted = lock_path(&control).unwrap();
        fs::write(&control, b"replacement").unwrap();
        let replaced = lock_path(&control).unwrap();

        assert_eq!(missing, symlinked);
        assert_eq!(missing, deleted);
        assert_eq!(missing, replaced);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn mutable_cache_ancestor_keeps_one_lock_identity() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("ancestor-lock-identity");
        let logical_root = dir.join("logical");
        let outside = dir.join("outside");
        fs::create_dir_all(&logical_root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        let path = logical_root.join("provider/entry.json");

        let missing = lock_path(&path).unwrap();
        symlink(&outside, logical_root.join("provider")).unwrap();
        let symlinked = lock_path(&path).unwrap();
        fs::remove_file(logical_root.join("provider")).unwrap();
        fs::create_dir(logical_root.join("provider")).unwrap();
        let replaced = lock_path(&path).unwrap();

        assert_eq!(missing, symlinked);
        assert_eq!(missing, replaced);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn cache_writes_do_not_follow_subdirectory_symlinks() {
        use std::os::unix::fs::symlink;

        let dir = test_dir("symlink-parent");
        let outside = test_dir("symlink-outside");
        let linked = dir.join("provider");
        symlink(&outside, &linked).unwrap();
        let target = linked.join("entry.json");
        assert!(write_bytes_atomic(&target, b"hostile").is_err());
        assert!(!outside.join("entry.json").exists());
        enforce_global_budget(&dir).unwrap();
        assert!(!linked.exists());
        fs::remove_dir_all(dir).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn held_recovery_guard_obeys_the_acquisition_deadline() {
        let dir = test_dir("held-recovery-guard");
        if std::env::var_os("QUICKSEARCH_PRIVATE_LOCK_TEST").is_none() {
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "cache::tests::held_recovery_guard_obeys_the_acquisition_deadline",
                ])
                .env("QUICKSEARCH_PRIVATE_LOCK_TEST", "1")
                .env("XDG_RUNTIME_DIR", &dir)
                .status()
                .unwrap();
            assert!(status.success());
            fs::remove_dir_all(dir).unwrap();
            return;
        }

        let cache_path = dir.join("entry.json");
        let path = lock_path(&cache_path).unwrap();
        fs::write(&path, b"4294967295 40\n").unwrap();
        let _held = RecoveryGuard::acquire(&path).unwrap();

        let started = std::time::Instant::now();
        assert!(matches!(
            CacheLock::acquire_with_timeout::<String>(&cache_path, Duration::from_millis(25)),
            Err(CacheError::LockTimeout(_))
        ));
        assert!(started.elapsed() < Duration::from_millis(500));
        drop(_held);
        fs::remove_file(path).unwrap();
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn two_stale_observers_cannot_remove_a_new_live_generation() {
        use std::sync::mpsc;

        let dir = test_dir("two-stale-observers");
        let path = dir.join("entry.lock");
        let stale = b"4294967295 41\n".to_vec();
        fs::write(&path, &stale).unwrap();

        let observed = Arc::new(Barrier::new(3));
        let (first_done_tx, first_done_rx) = mpsc::channel();
        let (second_go_tx, second_go_rx) = mpsc::channel();

        let first_path = path.clone();
        let first_stale = stale.clone();
        let first_observed = Arc::clone(&observed);
        let first = thread::spawn(move || {
            assert_eq!(
                stale_lock_identity_with_liveness(&first_path, |_| Some(false)),
                Some(first_stale.clone())
            );
            first_observed.wait();
            first_done_tx
                .send(reclaim_lock_if_unchanged(&first_path, &first_stale))
                .unwrap();
        });

        let second_path = path.clone();
        let second_stale = stale.clone();
        let second_observed = Arc::clone(&observed);
        let second = thread::spawn(move || {
            assert_eq!(
                stale_lock_identity_with_liveness(&second_path, |_| Some(false)),
                Some(second_stale.clone())
            );
            second_observed.wait();
            second_go_rx.recv().unwrap();
            reclaim_lock_if_unchanged(&second_path, &second_stale)
        });

        observed.wait();
        assert!(first_done_rx.recv().unwrap());
        let live_identity = try_create_identity_file(&path).unwrap().unwrap();
        let live = CacheLock {
            path: path.clone(),
            identity: live_identity.clone(),
        };
        second_go_tx.send(()).unwrap();

        first.join().unwrap();
        assert!(!second.join().unwrap());
        assert_eq!(fs::read(&path).unwrap(), live_identity);
        drop(live);
        assert!(!path.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn unreclaimable_stale_lock_does_not_spin_as_if_removed() {
        let dir = test_dir("unreclaimable-stale-lock");
        let lock = dir.join("record.lock");
        let identity = b"4294967295 0\n";
        fs::write(&lock, identity).unwrap();
        fs::create_dir(sidecar(&lock, "reap")).unwrap();

        assert!(!reclaim_lock_if_unchanged(&lock, identity));
        assert_eq!(fs::read(&lock).unwrap(), identity);
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn abandoned_reaper_claim_is_reclaimed() {
        let dir = test_dir("dead-reaper");
        let path = dir.join("value.json.lock");
        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, b"99999999 1\n").unwrap();
        let claim = sidecar(&path, "reap");
        fs::write(&claim, b"99999998 2\n").unwrap();
        let identity = fs::read(&path).unwrap();
        reclaim_lock_if_unchanged(&path, &identity);
        assert!(!path.exists());
        assert!(!claim.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn dead_process_lock_is_reclaimed() {
        let dir = test_dir("dead-lock");
        let path = dir.join("entry.json");
        let lock_path = lock_path(&path).unwrap();
        fs::write(&lock_path, "4294967295 0\n").unwrap();
        let result = get(&path, 100, || Ok("recovered".into())).unwrap();
        assert_eq!(result.value, "recovered");
        assert!(!lock_path.exists());
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn concurrent_writers_leave_one_complete_record_and_no_sidecars() {
        const THREADS: usize = 12;
        let dir = test_dir("concurrent");
        let path = Arc::new(dir.join("entry.json"));
        let barrier = Arc::new(Barrier::new(THREADS));
        let fetches = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();
        for _ in 0..THREADS {
            let path = Arc::clone(&path);
            let barrier = Arc::clone(&barrier);
            let fetches = Arc::clone(&fetches);
            handles.push(thread::spawn(move || {
                barrier.wait();
                get(&path, 100, || {
                    fetches.fetch_add(1, Ordering::SeqCst);
                    Ok("shared".into())
                })
                .unwrap()
            }));
        }
        for handle in handles {
            assert_eq!(handle.join().unwrap().value, "shared");
        }
        assert_eq!(fetches.load(Ordering::SeqCst), 1);
        let record: CacheRecord<String> =
            serde_json::from_slice(&fs::read(&*path).unwrap()).unwrap();
        assert_eq!(record.value, "shared");
        assert!(!lock_path(&path).unwrap().exists());
        let prefix = path.file_name().unwrap().to_string_lossy().into_owned();
        assert!(!fs::read_dir(&dir).unwrap().flatten().any(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with(&format!("{prefix}.tmp."))
        }));
        fs::remove_dir_all(dir).unwrap();
    }
}
