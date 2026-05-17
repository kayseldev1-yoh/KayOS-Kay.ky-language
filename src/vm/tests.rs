#[cfg(test)]
pub mod tests {
    use crate::vm::chunk::Chunk;
    use crate::vm::opcode::OpCode;
    use crate::vm::vm::VM;
    use crate::runtime::Value;

    #[test]
    pub fn test_addition() {
        let mut chunk = Chunk::new();
        let c1 = chunk.add_constant(Value::Number(1.2));
        let c2 = chunk.add_constant(Value::Number(3.4));
        
        chunk.write(OpCode::OpConstant as u8);
        chunk.write(c1 as u8);
        chunk.write(OpCode::OpConstant as u8);
        chunk.write(c2 as u8);
        chunk.write(OpCode::OpAdd as u8);
        chunk.write(OpCode::OpReturn as u8);
        
        let mut vm = VM::new(chunk);
        assert!(vm.run().is_ok());
    }
}
