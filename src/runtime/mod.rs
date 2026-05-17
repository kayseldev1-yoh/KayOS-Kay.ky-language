//! Kay.ky Runtime - Tree-walk Interpreter

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::{Expr, Stmt, Literal, BinaryOp, UnaryOp};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(String, Vec<String>, Vec<crate::ast::Stmt>, Rc<RefCell<Environment>>),
    NativeFunction(fn(Vec<Value>) -> Value),
    Object(Rc<RefCell<HashMap<String, Value>>>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Function(name, ..) => write!(f, "<fn {}>", name),
            Value::NativeFunction(_) => write!(f, "<native fn>"),
            Value::Object(_) => write!(f, "<object>"),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new(), enclosing: None }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self { values: HashMap::new(), enclosing: Some(enclosing) }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        None
    }

    pub fn assign(&mut self, name: String, value: Value) -> bool {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return true;
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        false
    }
}

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        
        // Native functions
        globals.borrow_mut().define("time".into(), Value::NativeFunction(|_| {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
            Value::Number(now.as_secs_f64())
        }));

        Self {
            globals: globals.clone(),
            environment: globals,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => { self.evaluate(expr)?; }
            Stmt::Say(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
            }
            Stmt::Assign(name, expr) => {
                let value = self.evaluate(expr)?;
                if !self.environment.borrow_mut().assign(name.clone(), value.clone()) {
                    self.environment.borrow_mut().define(name, value);
                }
            }
            Stmt::Define(name, params, body) => {
                let function = Value::Function(name.clone(), params, body, self.environment.clone());
                self.environment.borrow_mut().define(name, function);
            }
            Stmt::If(condition, then_branch, else_branch) => {
                let cond_value = self.evaluate(condition)?;
                if self.is_truthy(cond_value) {
                    self.execute_block(then_branch, Rc::new(RefCell::new(Environment::with_enclosing(self.environment.clone()))))?;
                } else if let Some(else_branch) = else_branch {
                    self.execute_block(else_branch, Rc::new(RefCell::new(Environment::with_enclosing(self.environment.clone()))))?;
                }
            }
            Stmt::While(condition, body) => {
                loop {
                    let cond_value = self.evaluate(condition.clone())?;
                    if !self.is_truthy(cond_value) { break; }
                    self.execute_block(body.clone(), Rc::new(RefCell::new(Environment::with_enclosing(self.environment.clone()))))?;
                }
            }
            Stmt::Return(value) => {
                let val = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Value::Null
                };
                return Err(format!("RETURN_VALUE: {:?}", val)); // Hacky return
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts, Rc::new(RefCell::new(Environment::with_enclosing(self.environment.clone()))))?;
            }
            Stmt::Import(_) => {}
        }
        Ok(())
    }

    fn execute_block(&mut self, stmts: Vec<Stmt>, env: Rc<RefCell<Environment>>) -> Result<(), String> {
        let previous = self.environment.clone();
        self.environment = env;
        
        let result = (|| {
            for stmt in stmts {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        self.environment = previous;
        result
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(match lit {
                Literal::Number(n) => Value::Number(n),
                Literal::String(s) => Value::String(s),
                Literal::Boolean(b) => Value::Boolean(b),
                Literal::Null => Value::Null,
            }),
            Expr::Variable(name) => {
                self.environment.borrow().get(&name)
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }
            Expr::Binary(left, op, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;
                match (left, op, right) {
                    (Value::Number(l), BinaryOp::Add, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), BinaryOp::Sub, Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), BinaryOp::Mul, Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::String(s), BinaryOp::Mul, Value::Number(n)) => {
                        Ok(Value::String(s.repeat(n as usize)))
                    },
                    (Value::Number(n), BinaryOp::Mul, Value::String(s)) => {
                        Ok(Value::String(s.repeat(n as usize)))
                    },
                    (Value::Number(l), BinaryOp::Div, Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Runtime Error: Division by zero".into())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    },
                    (Value::Number(l), BinaryOp::FloorDiv, Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Runtime Error: Division by zero".into())
                        } else {
                            Ok(Value::Number((l / r).floor()))
                        }
                    },
                    (Value::Number(l), BinaryOp::Modulo, Value::Number(r)) => {
                        if r == 0.0 {
                            Err("Runtime Error: Division by zero".into())
                        } else {
                            Ok(Value::Number(l % r))
                        }
                    },
                    (Value::Number(l), BinaryOp::Equal, Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Number(l), BinaryOp::Less, Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    (Value::Number(l), BinaryOp::Greater, Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    (Value::String(l), BinaryOp::Add, Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                    (l, BinaryOp::Equal, r) => Ok(Value::Boolean(format!("{:?}", l) == format!("{:?}", r))),
                    (l, op, r) => Err(format!("Invalid binary operation: {:?} {:?} {:?}", l, op, r)),
                }
            }
            Expr::Unary(op, right) => {
                let right = self.evaluate(*right)?;
                match (op, right) {
                    (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
                    (UnaryOp::Not, v) => Ok(Value::Boolean(!self.is_truthy(v))),
                    _ => Err("Invalid unary operation".into()),
                }
            }
            Expr::Call(callee, args) => {
                let callee = self.evaluate(*callee)?;
                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.evaluate(arg)?);
                }

                match callee {
                    Value::Function(_name, params, body, closure) => {
                        let env = Rc::new(RefCell::new(Environment::with_enclosing(closure)));
                        for (i, param) in params.iter().enumerate() {
                            env.borrow_mut().define(param.clone(), arguments.get(i).cloned().unwrap_or(Value::Null));
                        }
                        
                        let previous = self.environment.clone();
                        self.environment = env;
                        let result = self.execute_block(body, self.environment.clone());
                        self.environment = previous;

                        match result {
                            Err(e) if e.starts_with("RETURN_VALUE: ") => {
                                // Extract value from hacky return
                                Ok(Value::Null) // Should parse the stringified value or use a proper Error type
                            }
                            r => { r?; Ok(Value::Null) }
                        }
                    }
                    Value::NativeFunction(f) => Ok(f(arguments)),
                    _ => Err("Can only call functions".into()),
                }
            }
            Expr::Get(object, name) => {
                let _object = self.evaluate(*object)?;
                Err(format!("Property access not implemented: {}", name))
            }
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Null => false,
            Value::Boolean(b) => b,
            Value::Number(n) => n != 0.0,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
}
