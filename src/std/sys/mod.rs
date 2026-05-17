use std::env;
pub fn platform() -> String { std::env::consts::OS.to_string() }
pub fn env(key: &str) -> String { env::var(key).unwrap_or_default() }
pub fn exec(cmd: &str) { std::process::Command::new("sh").arg("-c").arg(cmd).spawn().unwrap(); }
