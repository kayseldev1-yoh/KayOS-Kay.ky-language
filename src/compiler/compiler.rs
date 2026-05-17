use crate::ast::{Expr, Stmt, Literal, BinaryOp};
use crate::vm::chunk::Chunk;
use crate::vm::opcode::OpCode;
use crate::runtime::Value;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self { chunk: Chunk::new() }
    }

    pub fn compile(mut self, statements: Vec<Stmt>) -> Result<Chunk, String> {
        for stmt in statements {
            self.compile_statement(stmt)?;
        }
        self.chunk.write(OpCode::OpReturn as u8);
        Ok(self.chunk)
    }

    fn compile_statement(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expression(expr)?;
            }
            _ => return Err("Unsupported statement in compiler yet".to_string()),
        }
        Ok(())
    }

    fn compile_expression(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::Literal(Literal::Number(n)) => {
                let constant = self.chunk.add_constant(Value::Number(n));
                self.chunk.write(OpCode::OpConstant as u8);
                self.chunk.write(constant as u8);
            }
            Expr::Binary(left, op, right) => {
                self.compile_expression(*left)?;
                self.compile_expression(*right)?;
                match op {
                    BinaryOp::Add => self.chunk.write(OpCode::OpAdd as u8),
                    BinaryOp::Sub => self.chunk.write(OpCode::OpSubtract as u8),
                    BinaryOp::Mul => self.chunk.write(OpCode::OpMultiply as u8),
                    BinaryOp::Div => self.chunk.write(OpCode::OpDivide as u8),
                    _ => return Err("Unsupported binary operator".to_string()),
                }
            }
            _ => return Err("Unsupported expression".to_string()),
        }
        Ok(())
    }
}
