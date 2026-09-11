use serde_json::{json, Value};
use std::{
    fs,
    io::{Read, Write},
    os::unix::{fs::PermissionsExt, net::UnixStream},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
const BIN: &str = env!("CARGO_BIN_EXE_eam-runtime");
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let p = PathBuf::from(format!(
            "/tmp/eam-rtest-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&p).unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o700)).unwrap();
        Self(p)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn run(args: &[&str], input: &[u8]) -> std::process::Output {
    let mut p = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(input).unwrap();
    p.wait_with_output().unwrap()
}
fn request(action: &str, v: Value) -> Value {
    let o = run(&["manager", action], &serde_json::to_vec(&v).unwrap());
    assert!(
        o.status.success(),
        "{} {}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    serde_json::from_slice(&o.stdout).unwrap()
}
fn history(v: Value) -> Value {
    let o = run(&["history"], &serde_json::to_vec(&v).unwrap());
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stdout));
    serde_json::from_slice(&o.stdout).unwrap()
}
fn fixture(name: &str) -> PathBuf {
    Path::new(BIN).parent().unwrap().join("examples").join(name)
}
fn wait(mut test: impl FnMut() -> bool) {
    let end = Instant::now() + Duration::from_secs(5);
    while Instant::now() < end {
        if test() {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(test(), "Timed out");
}
struct Session {
    path: PathBuf,
    runtime: PathBuf,
    _temp: Temp,
}
impl Session {
    fn new(backend: &str, args: Value, record: bool) -> Self {
        let tmp = Temp::new();
        let path = tmp.0.join("session");
        let v = request(
            "start",
            json!({"session":path,"provider":"Fake","backend":backend,"executable":fixture("fixture"),"args":args,"directory":tmp.0,"raw_recording":record}),
        );
        Self {
            path,
            runtime: v["runtime"].as_str().unwrap().into(),
            _temp: tmp,
        }
    }
    fn status(&self) -> Value {
        request("inspect", json!({"session":self.path}))["status"].clone()
    }
    fn connect(&self) -> UnixStream {
        let c = UnixStream::connect(self.runtime.join("socket")).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        c
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        let _ = run(
            &["manager", "stop"],
            &serde_json::to_vec(&json!({"session":self.path})).unwrap(),
        );
        let _ = fs::remove_dir_all(&self.runtime);
    }
}
fn packet(k: u8, b: &[u8]) -> Vec<u8> {
    let mut p = vec![k];
    p.extend(&(b.len() as u32).to_be_bytes());
    p.extend(b);
    p
}
fn next(c: &mut UnixStream) -> (u8, Vec<u8>) {
    let mut h = [0; 5];
    c.read_exact(&mut h).unwrap();
    let n = u32::from_be_bytes(h[1..5].try_into().unwrap()) as usize;
    assert!(n <= 256 * 1024);
    let mut b = vec![0; n];
    c.read_exact(&mut b).unwrap();
    (h[0], b)
}
fn until(c: &mut UnixStream, end: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    while !output.windows(end.len()).any(|x| x == end) {
        let (k, b) = next(c);
        if k == b'O' {
            output.extend(b)
        }
    }
    output
}
#[test]
fn pty_lifetime_bytes_resize_and_exit() {
    let s = Session::new("pty", json!(["echo"]), false);
    let mut c = s.connect();
    until(&mut c, b"READY");
    let pid = s.status()["recorder_pid"].clone();
    let input = "한글🙂\x1b[200~code\nlog\x1b[201~".as_bytes();
    for byte in packet(b'I', input) {
        c.write_all(&[byte]).unwrap();
    }
    assert_eq!(until(&mut c, input), input);
    drop(c);
    wait(|| s.status()["attached_clients"] == 0);
    assert_eq!(s.status()["recorder_pid"], pid);
    let mut c = s.connect();
    until(&mut c, input);
    let mut size = Vec::new();
    for n in [31u16, 97, 0, 0] {
        size.extend(n.to_ne_bytes());
    }
    c.write_all(&packet(b'R', &size)).unwrap();
    c.write_all(&packet(b'I', b"q")).unwrap();
    wait(|| s.status()["state"] == "stopped");
    assert_eq!(s.status()["exit_code"], 0);
    assert!(!s.path.join("output.ansi").exists());
}
#[test]
fn five_mib_replay_boundary() {
    let s = Session::new(
        "pty",
        json!(["replay", (5 * 1024 * 1024).to_string()]),
        false,
    );
    wait(|| s._temp.0.join("ready").exists());
    wait(|| s.status()["replay_bytes"] == 5 * 1024 * 1024);
    let mut c = s.connect();
    let b = until(&mut c, b"END");
    assert_eq!(b.len(), 5 * 1024 * 1024);
    assert_eq!(&b[..b.len() - 3], vec![b'a'; 5 * 1024 * 1024 - 3]);
    c.write_all(&packet(b'I', b"x")).unwrap();
    until(&mut c, b"x");
    drop(c);
    wait(|| s.status()["attached_clients"] == 0);
    let mut c = s.connect();
    let (k, b) = next(&mut c);
    assert_eq!(k, b'H');
    assert_eq!(
        serde_json::from_slice::<Value>(&b).unwrap()["replay_complete"],
        false
    );
}
#[test]
fn detached_natural_exit() {
    for backend in ["pty"] {
        let s = Session::new(backend, json!(["exit"]), true);
        wait(|| s.status()["state"] == "stopped");
        assert!(
            String::from_utf8(fs::read(s.path.join("output.ansi")).unwrap())
                .unwrap()
                .contains("한글 종료")
        );
    }
}
#[test]
fn notify_and_cross_chunk_search() {
    let o = run(&["notify-claude"], br#"{"hook_event_name":"Stop"}"#);
    assert!(o.status.success());
    let v: Value = serde_json::from_slice(&o.stdout).unwrap();
    assert!(v["terminalSequence"]
        .as_str()
        .unwrap()
        .contains("Claude Code;Stop"));
    let o = run(&["notify-claude"], b"bad");
    assert!(o.status.success() && o.stdout.is_empty());
    let t = Temp::new();
    let file = t.0.join("raw");
    let mut b = vec![b'a'; 65535];
    b.extend("한글needle".as_bytes());
    fs::write(&file, &b).unwrap();
    let o = run(
        &["search-archive", file.to_str().unwrap(), "0"],
        "한글".as_bytes(),
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&o.stdout).unwrap()["offset"],
        65535
    );
    assert_eq!(fs::read(file).unwrap(), b);
}
#[test]
fn native_history_filter_pages_and_readonly_sqlite() {
    let t = Temp::new();
    let projects = t.0.join("projects/project");
    fs::create_dir_all(&projects).unwrap();
    let path = projects.join("conversation.jsonl");
    let text = "한글 코드\n".repeat(2000);
    let records = [
        json!({"type":"user","sessionId":"conversation","cwd":t.0,"message":{"role":"user","content":text}}),
        json!({"type":"assistant","message":{"role":"assistant","content":[{"type":"thinking","thinking":"hidden"},{"type":"text","text":"끝"}]}}),
        json!({"type":"user","isMeta":true,"message":{"role":"user","content":"hidden"}}),
    ];
    let raw = records.iter().map(|v| format!("{v}\n")).collect::<String>() + "{\"partial\":";
    fs::write(&path, &raw).unwrap();
    let entry = json!({"provider":"Claude","id":"conversation","path":path});
    let expected = format!("\n## 사용자\n\n{text}\n\n## 응답\n\n끝\n");
    let mut rebuilt = String::new();
    for offset in (0..expected.chars().count()).step_by(1024) {
        let p = history(json!({"action":"page","entry":entry,"offset":offset,"limit":1024}));
        assert!(p["text"].as_str().unwrap().chars().count() <= 1024);
        rebuilt.push_str(p["text"].as_str().unwrap());
    }
    assert_eq!(rebuilt, expected);
    assert_eq!(fs::read_to_string(&path).unwrap(), raw);
    let list = history(json!({"action":"list","provider":"Claude","root":t.0,"directory":t.0}));
    assert_eq!(list["entries"][0]["id"], "conversation");
    let db = t.0.join("thread_history_1.sqlite");
    let c = rusqlite::Connection::open(&db).unwrap();
    c.execute_batch("CREATE TABLE thread_items(thread_id,rollout_ordinal,item_type,item_json)")
        .unwrap();
    c.execute(
        "INSERT INTO thread_items VALUES('one',1,'agentMessage',?)",
        [json!({"text":"SQLite 응답"}).to_string()],
    )
    .unwrap();
    drop(c);
    let before = fs::read(&db).unwrap();
    let entry = json!({"provider":"Codex","id":"one","mode":"paginated","history_db":db});
    let p = history(json!({"action":"page","entry":entry}));
    assert!(p["text"].as_str().unwrap().contains("SQLite 응답"));
    assert_eq!(fs::read(db).unwrap(), before);
}
#[test]
fn both_readonly_provider_adapters_use_native_fixture() {
    let t = Temp::new();
    for provider in ["claude", "codex"] {
        let state = t.0.join(format!("{provider}.json"));
        for prompt in ["입력 그대로\n$(echo unsafe) `literal`", "두 번째"] {
            let mut p = Command::new(BIN)
                .args([
                    "provider",
                    "--provider",
                    provider,
                    "--executable",
                    fixture("provider_fixture").to_str().unwrap(),
                    "--cwd",
                    t.0.to_str().unwrap(),
                    "--state",
                    state.to_str().unwrap(),
                    "--prefix",
                    t.0.join(provider).to_str().unwrap(),
                ])
                .env("CODEX_HOME", &t.0)
                .env("EMACS_AI_FIXTURE_REQUESTS", t.0.join("requests"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            p.stdin
                .take()
                .unwrap()
                .write_all(prompt.as_bytes())
                .unwrap();
            let o = p.wait_with_output().unwrap();
            assert!(
                o.status.success(),
                "{} {}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            let text = String::from_utf8(o.stdout).unwrap();
            assert_eq!(text.matches("한글🙂").count(), 1);
            let v: Value = serde_json::from_slice(&fs::read(&state).unwrap()).unwrap();
            assert_eq!(v["backend_id"], format!("fixture-{provider}"));
            assert_eq!(v["status"], "completed");
        }
    }
    let requests = fs::read_to_string(t.0.join("requests")).unwrap();
    assert!(
        requests.contains("--resume")
            && requests.contains("thread/resume")
            && requests.contains("$(echo unsafe)")
    );
}

fn adapter(temp: &Temp, provider: &str, mode: &str) -> std::process::Child {
    let mut c = Command::new(BIN)
        .args([
            "provider",
            "--provider",
            provider,
            "--executable",
            fixture("provider_fixture").to_str().unwrap(),
            "--cwd",
            temp.0.to_str().unwrap(),
            "--state",
            temp.0.join("state").to_str().unwrap(),
            "--prefix",
            temp.0.join("output").to_str().unwrap(),
        ])
        .env("CODEX_HOME", &temp.0)
        .env("EMACS_AI_FIXTURE_MODE", mode)
        .env("EMACS_AI_FIXTURE_AUTH", "apiKey")
        .env("EMACS_AI_FIXTURE_CHILD", temp.0.join("child"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin
        .take()
        .unwrap()
        .write_all("실패 시험".as_bytes())
        .unwrap();
    c
}
#[test]
fn adapter_failure_and_cancellation_do_not_reuse_partial_session() {
    for (provider, mode) in [("claude", "eof"), ("claude", "oversize"), ("codex", "auth")] {
        let t = Temp::new();
        let mut c = adapter(&t, provider, mode);
        wait(|| c.try_wait().unwrap().is_some());
        let o = c.wait_with_output().unwrap();
        assert!(!o.status.success());
        let v: Value = serde_json::from_slice(&fs::read(t.0.join("state")).unwrap()).unwrap();
        assert_eq!(v["status"], "failed");
        assert!(v["backend_id"].is_null());
    }
    let t = Temp::new();
    let mut c = adapter(&t, "claude", "wait");
    wait(|| t.0.join("child").exists());
    unsafe {
        libc::kill(c.id() as i32, libc::SIGTERM);
    }
    wait(|| c.try_wait().unwrap().is_some());
    assert!(!c.wait_with_output().unwrap().status.success());
    let v: Value = serde_json::from_slice(&fs::read(t.0.join("state")).unwrap()).unwrap();
    assert_eq!(v["status"], "cancelled");
    assert!(v["backend_id"].is_null());
    let pid: i32 = fs::read_to_string(t.0.join("child"))
        .unwrap()
        .parse()
        .unwrap();
    wait(|| unsafe { libc::kill(pid, 0) } < 0);
}
#[test]
fn detached_large_output_and_slow_consumer_keep_daemon_responsive() {
    // Output far beyond both replay and live budgets must not block the CLI.
    let s = Session::new("pty", json!(["burst"]), false);
    let mut slow = s.connect();
    let initial = until(&mut slow, b"READY").len();
    slow.write_all(&packet(b'I', b"b")).unwrap();
    wait(|| s._temp.0.join("ready").exists());
    wait(|| s.status()["output_bytes"] == initial + 32 * 1024 * 1024);
    let v = s.status();
    assert_eq!(v["replay_bytes"], 0);
    assert_eq!(v["attached_clients"], 0);
    assert_eq!(v["state"], "running");
    assert!(!s.path.join("output.ansi").exists());
    let mut c = s.connect();
    c.write_all(&packet(b'I', "한글".as_bytes())).unwrap();
    until(&mut c, "한글".as_bytes());
}

#[test]
fn cli_exit_with_slave_holder_finishes_attached_and_detached() {
    for attached in [true, false] {
        let s = Session::new("pty", json!(["inherited-pty"]), false);
        let mut c = s.connect();
        until(&mut c, b"READY");
        let began = Instant::now();
        c.write_all(&packet(b'I', b"q")).unwrap();
        wait(|| s._temp.0.join("child").exists()); // CLI consumed quit before detach.
        let mut attached_client = if attached {
            Some(c)
        } else {
            drop(c);
            None
        };
        // The fixture helper ignores HUP and holds the slave for three seconds.
        // The CLI session must close sooner without signalling that helper.
        wait(|| s.status()["state"] == "stopped");
        assert!(began.elapsed() < Duration::from_secs(2));
        assert_eq!(s.status()["exit_code"], 0);
        if let Some(ref mut c) = attached_client {
            let mut b = Vec::new();
            c.read_to_end(&mut b).unwrap();
        }
        let pid: i32 = fs::read_to_string(s._temp.0.join("child"))
            .unwrap()
            .parse()
            .unwrap();
        wait(|| unsafe { libc::kill(pid, 0) } < 0);
        assert!(!s.runtime.join("socket").exists());
    }
}

#[test]
fn unsupported_backend_and_retired_operations_are_rejected() {
    let t = Temp::new();
    let path = t.0.join("session");
    let o = run(
        &["manager", "start"],
        &serde_json::to_vec(&json!({
            "session":path,"provider":"Fake","backend":"retired",
            "executable":fixture("fixture"),"args":["echo"],"directory":t.0
        }))
        .unwrap(),
    );
    assert!(!o.status.success());
    assert!(!path.exists());
    for command in ["record", "reap", "reap-worker"] {
        let o = run(&[command], b"");
        assert!(!o.status.success());
        assert!(String::from_utf8_lossy(&o.stdout).contains("Unknown native operation"));
    }
}

#[test]
fn session_names_and_activity_survive_detach_and_exit() {
    let s = Session::new("pty", json!(["echo"]), false);
    let mut c = s.connect();
    until(&mut c, b"READY\r\n");
    let initial = s.status()["last_output_at"].as_f64().unwrap();
    assert!(initial > 0.0);
    let original = fs::read(s.path.join("session.json")).unwrap();
    let renamed = request("rename", json!({"session":s.path,"name":"  한글 작업  "}));
    assert_eq!(renamed["name"], "한글 작업");
    assert!(renamed["created_at"].as_f64().unwrap() > 0.0);
    assert_eq!(original, fs::read(s.path.join("session.json")).unwrap());
    assert_eq!(
        fs::metadata(s.path.join("display.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    for name in ["x".repeat(129), "invalid\nname".into()] {
        let o = run(
            &["manager", "rename"],
            &serde_json::to_vec(&json!({"session":s.path,"name":name})).unwrap(),
        );
        assert!(!o.status.success());
    }
    std::thread::sleep(Duration::from_millis(15));
    c.write_all(&packet(b'I', "한글".as_bytes())).unwrap();
    until(&mut c, "한글".as_bytes());
    wait(|| s.status()["last_input_at"].as_f64().unwrap() > initial);
    let output = s.status()["last_output_at"].as_f64().unwrap();
    assert!(output > initial);
    drop(c);
    wait(|| s.status()["attached_clients"] == 0);
    let live = request("list-live", json!({"root":s._temp.0}));
    assert_eq!(live["sessions"][0]["name"], "한글 작업");
    assert_eq!(live["sessions"][0]["last_output_at"], output);
    let mut c = s.connect();
    until(&mut c, "한글".as_bytes());
    // Replaying output does not manufacture a new activity timestamp.
    assert_eq!(s.status()["last_output_at"], output);
    c.write_all(&packet(b'I', b"q")).unwrap();
    wait(|| s.status()["state"] == "stopped");
    let info = request("inspect", json!({"session":s.path}));
    assert_eq!(info["metadata"]["name"], "한글 작업");
    assert!(info["metadata"]["last_input_at"].as_f64().unwrap() > initial);
    assert_eq!(
        request("rename", json!({"session":s.path,"name":""}))["name"],
        ""
    );
}

#[test]
fn detached_notification_receipts_are_persistent_and_snapshot_bounded() {
    let s = Session::new("pty", json!(["echo"]), false);
    let mut c = s.connect();
    until(&mut c, b"READY\r\n");
    c.write_all(&packet(b'I', b"\x1b]9;notice-one\x07"))
        .unwrap();
    until(&mut c, b"notice-one\x07");
    drop(c);
    wait(|| s.status()["attached_clients"] == 0);
    wait(|| request("inspect", json!({"session":s.path}))["metadata"]["notifications"]["seq"] == 1);
    let live = request("list-live", json!({"root":s._temp.0}));
    assert_eq!(live["sessions"][0]["notifications"]["unread"], true);
    let mut c = s.connect();
    until(&mut c, b"notice-one\x07");
    c.write_all(&packet(b'I', b"\x1b]9;notice-two\x07"))
        .unwrap();
    until(&mut c, b"notice-two\x07");
    wait(|| request("inspect", json!({"session":s.path}))["metadata"]["notifications"]["seq"] == 2);
    let ack = request("acknowledge", json!({"session":s.path,"seq":1}));
    assert_eq!(ack["unread"], true);
    assert_eq!(ack["read_seq"], 1);
    let ack = request("acknowledge", json!({"session":s.path,"seq":2}));
    assert_eq!(ack["unread"], false);
    // An older display cannot move the read cursor backwards by acknowledging.
    assert_eq!(
        request("acknowledge", json!({"session":s.path,"seq":1}))["read_seq"],
        2
    );
    assert_eq!(
        request(
            "acknowledge",
            json!({"session":s.path,"seq":2,"unread":true})
        )["unread"],
        true
    );
    request("acknowledge", json!({"session":s.path,"seq":2}));
    let o = run(
        &["manager", "acknowledge"],
        &serde_json::to_vec(&json!({"session":s.path,"seq":3})).unwrap(),
    );
    assert!(!o.status.success());
    c.write_all(&packet(b'I', b"q")).unwrap();
    wait(|| s.status()["state"] == "stopped");
    assert_eq!(
        request("inspect", json!({"session":s.path}))["metadata"]["notifications"]["unread"],
        false
    );
}
