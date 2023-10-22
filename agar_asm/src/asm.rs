use std::str::FromStr;

use agar_core::{Data, Float, Instruction, OpCode, Operands, Program};

pub struct Assembler {
    pub source: String,
    pub line: usize,
}

impl Assembler {
    pub fn new(source: String) -> Self {
        Self { source, line: 0 }
    }

    pub fn parse_line(&self, line: &str) -> Result<Instruction, &'static str> {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words.first() {
            Some(&"pushi") => {
                if words.len() > 2 {
                    return Err("Too many arguments for PushI instruction");
                }
                if let Some(num) = words.get(1) {
                    if let Ok(n) = num.parse::<i64>() {
                        return Ok(Instruction {
                            op_code: OpCode::PushInt,
                            operands: Operands::One(Data::Int(n)),
                        });
                    } else {
                        return Err("Can't read Int const");
                    }
                } else {
                    return Err("Not enough arguments for PushI instruction");
                }
            }
            Some(&"pushf") => {
                if words.len() > 2 {
                    return Err("Too many arguments for PushF instruction");
                }
                if let Some(num) = words.get(1) {
                    if let Ok(n) = Float::from_str(num) {
                        return Ok(Instruction {
                            op_code: OpCode::PushFloat,
                            operands: Operands::One(Data::Float(n)),
                        });
                    } else {
                        return Err("Can't read Float const");
                    }
                } else {
                    return Err("Not enough arguments for PushF instruction");
                }
            }
            Some(&"add") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Add instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Add,
                    operands: Operands::Zero,
                });
            }
            Some(&"sub") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Sub instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Sub,
                    operands: Operands::Zero,
                });
            }
            Some(&"mul") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Mul instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Mul,
                    operands: Operands::Zero,
                });
            }
            Some(&"dup") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Dup instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Dup,
                    operands: Operands::Zero,
                });
            }
            Some(&"print") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Print instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Print,
                    operands: Operands::Zero,
                });
            }
            Some(&"panic") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Panic instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Panic,
                    operands: Operands::Zero,
                });
            }
            Some(&"exit") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Exit instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Exit,
                    operands: Operands::Zero,
                });
            }
            Some(&"nop") => {
                if words.len() > 1 {
                    return Err("Too many arguments for NOP instruction");
                }
                return Ok(Instruction {
                    op_code: OpCode::Nop,
                    operands: Operands::Zero,
                });
            }
            _ => {}
        }
        Err("")
    }

    pub fn parse_source(&mut self) -> Option<Program> {
        let mut ops = Vec::new();
        for line in self.source.lines() {
            match self.parse_line(line) {
                Ok(instr) => ops.push(instr),
                Err(e) => {
                    println!("Error: {e}");
                    return None;
                }
            }
            self.line += 1;
        }
        Some(Program { ops })
    }
}
