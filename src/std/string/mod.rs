// src/std/string/mod.rs
use crate::runtime::{Value, Environment};

pub fn register(env: &mut Environment) {
    env.define("upper".into(), Value::NativeFunction(|args| {
        match &args[0] {
            Value::String(s) => Value::String(s.to_uppercase()),
            _ => Value::Null,
        }
    }));
}
