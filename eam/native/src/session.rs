use crate::{common::*, transport};
use serde_json::{json, Value};
use std::{
    fs, io,
    io::{Read, Seek, SeekFrom},
    os::unix::{net::UnixDatagram, process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};
fn control(v: &Value, action: &str) -> Result<Value> {
    let temp = runtime_dir()?;
    let result = (|| {
        let sock = UnixDatagram::bind(temp.join("c"))?;
        sock.set_read_timeout(Some(Duration::from_secs(1)))?;
        sock.connect(Path::new(string(v, "runtime")?).join("control"))?;
        sock.send(&serde_json::to_vec(&json!({"id":v["id"],"action":action}))?)?;
        let mut b = [0; 2048];
        let n = sock.recv(&mut b)?;
        let r: Value = serde_json::from_slice(&b[..n])?;
        if r["id"] != v["id"] {
            return Err(error("Owner mismatch"));
        }
        Ok(r)
    })();
    let _ = fs::remove_dir_all(temp);
    result
}
pub fn status(path: &Path) -> Result<Value> {
    let v = session(path)?;
    if v["stopped"] == true {
        return Ok(stopped(&v));
    }
    if v["backend"] != "pty" {
        return Err(error("Unsupported stored session backend"));
    }
    match control(&v, "status") {
        Ok(r) => Ok(r),
        Err(e) => {
            let latest = session(path)?;
            if latest["stopped"] == true {
                Ok(stopped(&latest))
            } else {
                Ok(json!({"state":"unavailable","detail":e.to_string()}))
            }
        }
    }
}
fn name(q: &Value) -> Result<String> {
    let value = string(q, "name")?.trim();
    if value.chars().count() > 128
        || value
            .chars()
            .any(|c| c.is_control() || c == '\u{2028}' || c == '\u{2029}')
    {
        return Err(error(
            "Session name must be at most 128 characters without controls",
        ));
    }
    Ok(value.into())
}
// Inspect only a bounded tail; detached sessions do not need a display reader.
fn notices(path: &Path) -> Result<Value> {
    let mut f = fs::File::open(path.join("events.jsonl"))?;
    let size = f.metadata()?.len();
    let start = size.saturating_sub(65536);
    f.seek(SeekFrom::Start(start))?;
    let mut bytes = Vec::new();
    f.take(65536).read_to_end(&mut bytes)?;
    let mut seq = 0;
    let mut lines = bytes.split_inclusive(|b| *b == b'\n');
    if start > 0 {
        lines.next();
    }
    for line in lines {
        if line.last() == Some(&b'\n') {
            let event: Value = serde_json::from_slice(line)?;
            seq = seq.max(
                event["seq"]
                    .as_u64()
                    .ok_or_else(|| error("Invalid event sequence"))?,
            );
        }
    }
    let receipt = path.join("notification-read.json");
    let read = if receipt.try_exists()? {
        read_json(&receipt, 4096)?["seq"]
            .as_u64()
            .ok_or_else(|| error("Invalid read receipt"))?
    } else {
        0
    };
    Ok(json!({"seq":seq,"read_seq":read,"unread":seq > read}))
}
fn acknowledge(q: &Value) -> Result<Value> {
    let path = Path::new(string(q, "session")?);
    session(path)?;
    let state = notices(path)?;
    let seq = q["seq"]
        .as_u64()
        .ok_or_else(|| error("Invalid read sequence"))?;
    if seq > state["seq"].as_u64().unwrap() {
        return Err(error("Read sequence is in the future"));
    }
    let seq = if q["unread"] == true {
        seq.saturating_sub(1)
    } else {
        seq.max(state["read_seq"].as_u64().unwrap())
    };
    save(&path.join("notification-read.json"), &json!({"seq":seq}))?;
    notices(path)
}
fn info(path: &Path) -> Result<Value> {
    let mut v = session(path)?;
    v["notifications"] = notices(path)?;
    let label = path.join("display.json");
    v["name"] = if label.try_exists()? {
        json!(name(&read_json(&label, 4096)?)?)
    } else {
        json!("")
    };
    if v["created_at"].is_null() {
        let m = fs::metadata(path.join("session.json"))?;
        v["created_at"] = json!(m
            .created()
            .or_else(|_| m.modified())?
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64());
    }
    Ok(v)
}
fn rename(q: &Value) -> Result<Value> {
    let path = Path::new(string(q, "session")?);
    session(path)?;
    let label = name(q)?;
    // Display metadata has its own file: a daemon writing its exit state
    // cannot overwrite a concurrent rename. It never changes the CLI ID.
    save(&path.join("display.json"), &json!({"name":label}))?;
    info(path)
}
pub fn start(q: &Value) -> Result<Value> {
    let label = if q.get("name").is_some() {
        name(q)?
    } else {
        String::new()
    };
    let path = Path::new(string(q, "session")?);
    let dir = fs::canonicalize(string(q, "directory")?)?;
    let exe = Path::new(string(q, "executable")?);
    if !exe.is_absolute() || !exe.is_file() || !dir.is_dir() {
        return Err(error(
            "Absolute CLI executable and existing directory required",
        ));
    }
    let backend = q["backend"].as_str().unwrap_or("pty");
    if backend != "pty" {
        return Err(error("Unknown backend"));
    }
    if let Some(parent) = path.parent() {
        mkdir(parent)?;
    }
    fs::create_dir(path)?;
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
    let path = fs::canonicalize(path)?;
    let runtime = runtime_dir()?;
    let id = unique();
    fs::write(runtime.join("owner"), &id)?;
    let raw = q["raw_recording"] == true;
    let v = json!({"version":1,"id":id,"provider":q["provider"],"backend":backend,"engine":"native","created_at":now(),"directory":dir,"runtime":runtime,"archive":if raw{json!(path.join("output.ansi"))}else{Value::Null},"events":path.join("events.jsonl")});
    save(&path.join("session.json"), &v)?;
    if !label.is_empty() {
        save(&path.join("display.json"), &json!({"name":label}))?;
    }
    save(
        &runtime.join("launch.json"),
        &json!({"executable":exe,"args":q["args"],"editor":path.join("editor.json")}),
    )?;
    let binary = std::env::current_exe()?;
    let log = fs::File::create(runtime.join("daemon.log"))?;
    let mut c = Command::new(&binary);
    c.arg("daemon")
        .arg(&path)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log));
    unsafe {
        c.pre_exec(|| {
            if libc::setsid() < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    let mut child = c.spawn()?;
    let end = Instant::now() + Duration::from_secs(4);
    loop {
        if child.try_wait()?.is_some() {
            return Err(error("PTY daemon failed; inspect daemon.log"));
        }
        if control(&v, "status").is_ok() {
            break;
        }
        if Instant::now() > end {
            child.kill()?;
            child.wait()?;
            return Err(error("PTY startup timed out"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Ok(v)
}
pub fn stop(path: &Path) -> Result<Value> {
    let _lock = lock(&path.join("stop.lock"), false)?;
    let v = session(path)?;
    let state = status(path)?;
    if state["state"] == "stopped" || state["state"] == "unavailable" {
        return Ok(state);
    }
    control(&v, "stop")?;
    let end = Instant::now() + Duration::from_secs(4);
    while Instant::now() < end {
        let latest = session(path)?;
        if latest["stopped"] == true {
            return Ok(stopped(&latest));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Err(error("PTY stop not verified"))
}
pub fn attach(args: &[String]) -> Result<()> {
    if args.len() != 4 {
        return Err(error("attach SESSION EMACSCLIENT SOCKET TOKEN"));
    }
    let path = Path::new(&args[0]);
    let v = session(path)?;
    let _lease = lock(&path.join("attach.lock"), true)?;
    let s = status(path)?;
    if s["state"] != "running" || s["attached_clients"].as_u64() != Some(0) {
        return Err(error("Session not available for attachment"));
    }
    if !Path::new(&args[1]).is_absolute()
        || !Path::new(&args[2]).is_absolute()
        || args[3].is_empty()
        || args[3].len() > 128
    {
        return Err(error("Invalid editor endpoint"));
    }
    let mut endpoint = json!({"version":1,"client":args[1],"socket":args[2],"lease":path.join("attach.lock"),"attachment_token":args[3],"attachment_pid":std::process::id(),"connected":false});
    save(&path.join("editor.json"), &endpoint)?;
    let result = transport::attach(Path::new(string(&v, "runtime")?), |complete| {
        endpoint["connected"] = json!(true);
        endpoint["replay_complete"] = json!(complete);
        save(&path.join("editor.json"), &endpoint)
    });
    let _ = fs::remove_file(path.join("editor.json"));
    result
}
pub fn dispatch(action: &str, q: &Value) -> Result<Value> {
    match action {
        "start" => start(q),
        "rename" => rename(q),
        "acknowledge" => acknowledge(q),
        "inspect" => {
            let p = Path::new(string(q, "session")?);
            Ok(json!({"metadata":info(p)?,"status":status(p)?}))
        }
        "stop" => stop(Path::new(string(q, "session")?)),
        "list-live" => {
            let root = Path::new(string(q, "root")?);
            let mut paths = std::collections::BTreeSet::new();
            if root.exists() {
                for entry in fs::read_dir(root)? {
                    let p = entry?.path();
                    if p.is_dir() {
                        paths.insert(p);
                    }
                    if paths.len() > 256 {
                        return Err(error("Too many sessions"));
                    }
                }
            }
            if let Some(extra) = q["extra"].as_array() {
                for p in extra {
                    paths.insert(PathBuf::from(
                        p.as_str().ok_or_else(|| error("Invalid path"))?,
                    ));
                }
            }
            if paths.len() > 256 {
                return Err(error("Too many sessions"));
            }
            let paths: Vec<PathBuf> = paths.into_iter().collect();
            let entries = std::thread::scope(|scope| {
                let jobs:Vec<_>=paths.chunks(paths.len().div_ceil(8).max(1)).map(|chunk|scope.spawn(move || {
                    chunk.iter().map(|path| match (info(path),status(path)) {
                        (Ok(v),Ok(s)) if s["state"]=="running" =>
                            Some(json!({"session":path,"provider":v["provider"],"directory":v["directory"],"attached_clients":s["attached_clients"],"name":v["name"],"created_at":v["created_at"],"last_input_at":s["last_input_at"],"last_output_at":s["last_output_at"],"notifications":v["notifications"]})),
                        (_,Ok(s)) if s["state"]=="stopped" => None,
                        _=>Some(json!({"unverified":path}))
                    }).collect::<Vec<_>>()
                })).collect();
                jobs.into_iter()
                    .flat_map(|job| job.join().expect("Session query worker failed"))
                    .flatten()
                    .collect::<Vec<_>>()
            });
            let sessions: Vec<_> = entries
                .iter()
                .filter(|e| e.get("session").is_some())
                .cloned()
                .collect();
            let unverified: Vec<_> = entries
                .iter()
                .filter_map(|e| e.get("unverified"))
                .cloned()
                .collect();
            Ok(json!({"sessions":sessions,"unverified":unverified}))
        }
        "guard-worktree" => {
            let root = Path::new(string(q, "root")?);
            let dir = fs::canonicalize(string(q, "directory")?)?;
            if root.exists() {
                for e in fs::read_dir(root)? {
                    let p = e?.path();
                    if !p.is_dir() {
                        continue;
                    }
                    let v = session(&p)?;
                    if Path::new(string(&v, "directory")?).starts_with(&dir) && v["stopped"] != true
                    {
                        return Err(error("Stop persistent session before worktree removal"));
                    }
                }
            }
            Ok(json!({"allowed":true}))
        }
        _ => Err(error("Unknown management operation")),
    }
}
