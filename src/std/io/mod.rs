// src/std/io/mod.rs
use crate::runtime::{Value, Environment};
use std::fs;

pub fn register(env: &mut Environment) {
    env.define("read_file".into(), Value::NativeFunction(|args| {
        let path = match &args[0] { Value::String(s) => s, _ => return Value::Null };
        Value::String(fs::read_to_string(path).unwrap_or_default())
    }));
    env.define("write_file".into(), Value::NativeFunction(|args| {
        let path = match &args[0] { Value::String(s) => s, _ => return Value::Null };
        let content = match &args[1] { Value::String(s) => s, _ => return Value::Null };
        fs::write(path, content).is_ok();
        Value::Null
    }));
}
