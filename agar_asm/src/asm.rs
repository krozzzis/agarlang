use std::str::FromStr;

use agar_core::{Data, Instruction, OpCode, Operands, Program, Float};

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
        match words.get(0) {
            Some(&"pushi") => {
                if words.len() > 2 {
                    return Err("Too many arguments for PushI instruction")
                }
                if let Some(num) = words.get(1) {
                    if let Ok(n) = num.parse::<i64>() {
                        return Ok(Instruction {
                            op_code: OpCode::PushInt,
                            operands: Operands::One(Data::Int(n)),
                        });
                    } else {
                        return Err("Can't read Int const")
                    }
                } else {
                    return Err("Not enough arguments for PushI instruction")
                }
            }
            Some(&"pushf") => {
                if words.len() > 2 {
                    return Err("Too many arguments for PushF instruction")
                }
                if let Some(num) = words.get(1) {
                    if let Ok(n) = Float::from_str(num) {
                        return Ok(Instruction {
                            op_code: OpCode::PushFloat,
                            operands: Operands::One(Data::Float(n)),
                        });
                    } else {
                        return Err("Can't read Float const")
                    }
                } else {
                    return Err("Not enough arguments for PushF instruction")
                }
            }
            Some(&"add") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Add instruction")
                }
                return Ok(Instruction { op_code: OpCode::Add, operands: Operands::Zero })
            }
            Some(&"print") => {
                if words.len() > 1 {
                    return Err("Too many arguments for Print instruction")
                }
                return Ok(Instruction { op_code: OpCode::Print, operands: Operands::Zero })
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
