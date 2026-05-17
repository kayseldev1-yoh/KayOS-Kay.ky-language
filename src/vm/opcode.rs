#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    OpFFICall,
}
