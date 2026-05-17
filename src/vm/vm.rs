use crate::vm::chunk::Chunk;
use crate::vm::opcode::OpCode;
use crate::runtime::Value;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let instruction = self.read_byte();
            match instruction {
                x if x == OpCode::OpReturn as u8 => {
                    println!("Return: {:?}", self.stack.pop());
                    return Ok(());
                }
                x if x == OpCode::OpConstant as u8 => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                x if x == OpCode::OpAdd as u8 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a + b)),
                        _ => return Err("Operands must be numbers".to_string()),
                    }
                }
                x if x == OpCode::OpFFICall as u8 => {
                    let arg_count = self.read_byte() as usize;
                    let mut args = Vec::new();
                    for _ in 0..arg_count {
                        args.push(self.stack.pop().unwrap());
                    }
                    let func_name = match self.stack.pop().unwrap() {
                        Value::String(s) => s,
                        _ => return Err("Function name must be a string".to_string()),
                    };
                    let lib_name = match self.stack.pop().unwrap() {
                        Value::String(s) => s,
                        _ => return Err("Library name must be a string".to_string()),
                    };

                    unsafe {
                        let lib = libloading::Library::new(lib_name).map_err(|e| e.to_string())?;
                        let func: libloading::Symbol<unsafe extern "C" fn()> = lib.get(func_name.as_bytes()).map_err(|e| e.to_string())?;
                        func();
                    }
                    self.stack.push(Value::Null);
                }
                _ => return Err(format!("Unknown opcode: {}", instruction)),
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants[index].clone()
    }
}
