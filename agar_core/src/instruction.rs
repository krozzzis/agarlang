use crate::data::Data;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Nop = 0,
    PushInt,
    PushFloat,
    Add,
    Sub,
    Mul,
    Dup,
    Print,
    Panic,
    Exit,
}

impl OpCode {
    pub fn as_byte(&self) -> u8 {
        *self as u8
    }

    pub fn from_byte(num: u8) -> Option<Self> {
        Some(match num {
            _x if num == OpCode::Nop.as_byte() => OpCode::Nop,
            _x if num == OpCode::PushInt.as_byte() => OpCode::PushInt,
            _x if num == OpCode::PushFloat.as_byte() => OpCode::PushFloat,
            _x if num == OpCode::Add.as_byte() => OpCode::Add,
            _x if num == OpCode::Sub.as_byte() => OpCode::Sub,
            _x if num == OpCode::Mul.as_byte() => OpCode::Mul,
            _x if num == OpCode::Dup.as_byte() => OpCode::Dup,
            _x if num == OpCode::Print.as_byte() => OpCode::Print,
            _x if num == OpCode::Exit.as_byte() => OpCode::Exit,
            _x if num == OpCode::Panic.as_byte() => OpCode::Panic,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op_code: OpCode,
    pub operands: Operands,
}

#[derive(Debug, Clone, Copy)]
pub enum Operands {
    Zero,
    One(Data),
}
