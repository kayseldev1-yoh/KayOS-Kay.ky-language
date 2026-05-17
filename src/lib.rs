pub mod lexer;
pub mod parser;
pub mod ast;
pub mod runtime;
pub mod vm; // Tambahkan ini
pub mod compiler;
pub mod os;
pub mod std;

use lexer::Lexer;
use parser::Parser;
use runtime::Interpreter;

pub fn run(source: String) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    
    // Gunakan Compiler dan VM baru
    let compiler = crate::compiler::compiler::Compiler::new();
    let chunk = compiler.compile(statements)?;
    
    let mut vm = crate::vm::vm::VM::new(chunk);
    vm.run()?;
    
    Ok(())
}
