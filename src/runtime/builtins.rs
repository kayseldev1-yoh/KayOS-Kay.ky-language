// Built-in functions for Kay.ky
use crate::runtime::{Value, Environment};

pub fn register_globals(env: &mut Environment) {
    env.define("say".into(), Value::NativeFunction(|args| {
        println!("{}", args[0]);
        Value::Null
    }));

    env.define("len".into(), Value::NativeFunction(|args| {
        match &args[0] {
            Value::String(s) => Value::Number(s.len() as f64),
            _ => Value::Number(0.0),
        }
    }));

    env.define("type".into(), Value::NativeFunction(|args| {
        match &args[0] {
            Value::Number(_) => Value::String("number".into()),
            Value::String(_) => Value::String("string".into()),
            Value::Boolean(_) => Value::String("bool".into()),
            Value::Null => Value::String("null".into()),
            _ => Value::String("unknown".into()),
        }
    }));

    env.define("exit".into(), Value::NativeFunction(|args| {
        let code = match args.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => 0,
        };
        std::process::exit(code);
    }));
}
