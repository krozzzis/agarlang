use crate::{Data, Float, Instruction, Int, OpCode, Operands};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Program {
    pub ops: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn get(&self, index: usize) -> Option<&Instruction> {
        self.ops.get(index)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut ops = Vec::new();

        let mut i = 0;
        while i < bytes.len() {
            if let Some(opcode) = OpCode::from_byte(bytes[i]) {
                match opcode {
                    OpCode::PushInt => {
                        if let Ok(slice) = bytes[i + 1..i + 9].try_into() {
                            let int = Int::from_le_bytes(slice);
                            ops.push(Instruction {
                                op_code: opcode,
                                operands: Operands::One(Data::Int(int)),
                            });
                            i += 9;
                        } else {
                            return Err("Can't read Int constant from bytecode");
                        }
                    }
                    OpCode::PushFloat => {
                        if let Ok(slice) = bytes[i + 1..i + 17].try_into() {
                            let float = Float::deserialize(slice);
                            ops.push(Instruction {
                                op_code: opcode,
                                operands: Operands::One(Data::Float(float)),
                            });
                            i += 17;
                        } else {
                            return Err("Can't read Int constant from bytecode");
                        }
                    }
                    _ => {
                        ops.push(Instruction {
                            op_code: opcode,
                            operands: Operands::Zero,
                        });
                        i += 1;
                    }
                }
            }
        }

        Ok(Self { ops })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for instr in &self.ops {
            bytes.push(instr.op_code.as_byte());
            match instr.op_code {
                OpCode::PushInt => {
                    if let Operands::One(Data::Int(num)) = instr.operands {
                        for byte in num.to_le_bytes() {
                            bytes.push(byte);
                        }
                    }
                }
                OpCode::PushFloat => {
                    if let Operands::One(Data::Float(num)) = instr.operands {
                        for byte in num.serialize() {
                            bytes.push(byte);
                        }
                    }
                }
                _ => {}
            }
        }

        bytes
    }
}
