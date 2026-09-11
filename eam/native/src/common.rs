use serde_json::{json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt, PermissionsExt},
        io::{AsRawFd, RawFd},
    },
    path::{Path, PathBuf},
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    io::Error::other(s.into()).into()
}
pub fn string<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| error(format!("Missing string: {k}")))
}
pub fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}
pub fn unique() -> String {
    let mut bytes = [0u8; 16];
    File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(&mut bytes))
        .expect("Operating-system random source unavailable");
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub fn private_dir(p: &Path) -> Result<()> {
    let m = fs::symlink_metadata(p)?;
    if !m.is_dir() || m.uid() != unsafe { libc::getuid() } || m.mode() & 0o077 != 0 {
        return Err(error("Private owned directory required"));
    }
    Ok(())
}
pub fn bounded(p: &Path, n: usize) -> Result<Vec<u8>> {
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    if !f.metadata()?.is_file() {
        return Err(error("Regular file required"));
    }
    let mut b = Vec::new();
    f.take(n as u64 + 1).read_to_end(&mut b)?;
    if b.len() > n {
        return Err(error("File exceeds size limit"));
    }
    Ok(b)
}
pub fn read_json(p: &Path, n: usize) -> Result<Value> {
    Ok(serde_json::from_slice(&bounded(p, n)?)?)
}
pub fn save(p: &Path, v: &Value) -> Result<()> {
    let temp = p.with_file_name(format!(".state-{}", unique()));
    let result = (|| {
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&temp)?;
        serde_json::to_writer(&mut f, v)?;
        f.write_all(b"\n")?;
        fs::rename(&temp, p)?;
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result
}
pub fn mkdir(p: &Path) -> Result<()> {
    use std::os::unix::fs::DirBuilderExt;
    // Never chmod an existing parent supplied by the caller.
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(p)?;
    Ok(())
}
pub fn runtime_dir() -> Result<PathBuf> {
    let p = PathBuf::from(format!("/tmp/eam-{}", unique()));
    fs::create_dir(&p)?;
    fs::set_permissions(&p, fs::Permissions::from_mode(0o700))?;
    Ok(p)
}
pub fn session(p: &Path) -> Result<Value> {
    private_dir(p)?;
    let v = read_json(&p.join("session.json"), 16384)?;
    let id = string(&v, "id")?;
    if v["version"] != 1 || id.len() != 32 || !id.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(error("Invalid session metadata"));
    }
    if v["stopped"] == true {
        return Ok(v);
    }
    let r = Path::new(string(&v, "runtime")?);
    private_dir(r)?;
    if bounded(&r.join("owner"), 32)? != id.as_bytes() {
        return Err(error("Owner mismatch"));
    }
    Ok(v)
}
pub fn lock(p: &Path, nonblock: bool) -> Result<File> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    let op = libc::LOCK_EX | if nonblock { libc::LOCK_NB } else { 0 };
    if unsafe { libc::flock(f.as_raw_fd(), op) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(f)
}
pub fn nonblock(fd: RawFd) -> Result<()> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags < 0 || unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(())
}
pub fn shquote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
pub fn stdin_limit(n: usize) -> Result<Vec<u8>> {
    let mut b = Vec::new();
    io::stdin().take(n as u64 + 1).read_to_end(&mut b)?;
    if b.len() > n {
        return Err(error("Request exceeds limit"));
    }
    Ok(b)
}
pub fn request() -> Result<Value> {
    Ok(serde_json::from_slice(&stdin_limit(65536)?)?)
}
pub fn emit(v: &Value) -> Result<()> {
    serde_json::to_writer(io::stdout(), v)?;
    println!();
    Ok(())
}
pub fn stopped(v: &Value) -> Value {
    json!({"state":"stopped","exit_code":v["exit_code"],"attached_clients":0})
}
