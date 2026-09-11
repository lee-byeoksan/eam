use crate::common::*;
use std::{
    fs::OpenOptions,
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawFd,
    },
    path::Path,
    process::{Command, Stdio},
};
pub fn run(args: &[String]) -> Result<i32> {
    let (client, socket, files) = if args.first().is_some_and(|x| x == "--direct") {
        if args.len() < 4 {
            return Err(error("editor --direct CLIENT SOCKET FILE..."));
        }
        (args[1].clone(), args[2].clone(), &args[3..])
    } else {
        if args.len() < 2 {
            return Err(error("editor ROUTE FILE..."));
        }
        let p = Path::new(&args[0]);
        let m = std::fs::symlink_metadata(p)?;
        if m.uid() != unsafe { libc::getuid() } || m.mode() & 0o077 != 0 {
            return Err(error("Editor route must be private"));
        }
        let v = read_json(p, 4096)?;
        if v["version"] != 1 {
            return Err(error("Invalid editor route"));
        }
        if let Some(lease) = v["lease"].as_str() {
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(libc::O_NOFOLLOW)
                .open(lease)?;
            if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                return Err(error("No active Emacs attachment"));
            }
        }
        (
            string(&v, "client")?.to_owned(),
            string(&v, "socket")?.to_owned(),
            &args[1..],
        )
    };
    if !Path::new(&client).is_absolute() || !Path::new(&socket).is_absolute() {
        return Err(error("Absolute editor endpoint required"));
    }
    let paths: Vec<String> = files
        .iter()
        .map(|f| {
            if Path::new(f).is_absolute() {
                f.clone()
            } else {
                std::env::current_dir()
                    .unwrap()
                    .join(f)
                    .to_string_lossy()
                    .into_owned()
            }
        })
        .collect();
    let quoted: Vec<String> = paths
        .iter()
        .map(serde_json::to_string)
        .collect::<std::result::Result<_, _>>()?;
    let expr = format!(
        "(progn (eam-terminal--register-editor-files '({})) nil)",
        quoted.join(" ")
    );
    let base = ["--socket-name", &socket, "-a", "false"];
    let r = Command::new(&client)
        .args(base)
        .args(["--eval", &expr])
        .stdout(Stdio::null())
        .status()?;
    if !r.success() {
        return Ok(r.code().unwrap_or(1));
    }
    Ok(Command::new(&client)
        .args(base)
        .arg("--")
        .args(paths)
        .status()?
        .code()
        .unwrap_or(1))
}
