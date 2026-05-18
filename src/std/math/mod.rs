// src/std/math/mod.rs
use crate::runtime::{Value, Environment};

pub fn register(env: &mut Environment) {
    env.define("sqrt".into(), Value::NativeFunction(|args| {
        match args[0] {
            Value::Number(n) => Value::Number(n.sqrt()),
            _ => Value::Null,
        }
    }));
}
