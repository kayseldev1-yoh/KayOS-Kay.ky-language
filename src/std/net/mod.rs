// src/std/net/mod.rs
use crate::runtime::{Value, Environment};
use std::net::TcpStream;
use std::time::Duration;

pub fn register(env: &mut Environment) {
    env.define("is_open".into(), Value::NativeFunction(|args| {
        let host = match &args[0] { Value::String(s) => s, _ => return Value::Boolean(false) };
        let port = match &args[1] { Value::Number(n) => *n as u16, _ => return Value::Boolean(false) };
        Value::Boolean(TcpStream::connect_timeout(&format!("{}:{}", host, port).parse().unwrap(), Duration::from_millis(500)).is_ok())
    }));
}
