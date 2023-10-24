use std::io::Write;

use agar_core::{Data, OpCode, Operands, Program};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StepResult {
    Ok,
    Error(RuntimeError),
    Panic(&'static str),
    Exit,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuntimeError {
    IncompatibleType,
    NotEnoughArgs,
    InvalidValue,
    Other,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExitStatus {
    Ok,
    Error(RuntimeError),
    Panic,
}

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    pub stack: Vec<Data>,
    pub program: Program,
    pub ip: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            program: Program::new(),
            ip: 0,
        }
    }

    pub fn load_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn step<T: Write>(&mut self, output: &mut T) -> StepResult {
        match self.program.get(self.ip) {
            Some(instr) => {
                match instr.op_code {
                    OpCode::PushInt => {
                        if let Operands::One(data) = instr.operands {
                            self.stack.push(data);
                        }
                    }
                    OpCode::PushFloat => {
                        if let Operands::One(data) = instr.operands {
                            self.stack.push(data);
                        }
                    }
                    OpCode::Print => {
                        let data = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        if write!(output, "{}", data).is_err() {
                            return StepResult::Error(RuntimeError::Other);
                        }
                    }
                    OpCode::PrintChar => {
                        let data = if let Some(a) = self.stack.pop() {
                            if let Data::Int(b) = a {
                                b
                            } else {
                                return StepResult::Error(RuntimeError::IncompatibleType);
                            }
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let ch = if let Some(a) = char::from_u32(data as u32) {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::InvalidValue);
                        };
                        if write!(output, "{}", ch).is_err() {
                            return StepResult::Error(RuntimeError::Other);
                        }
                    }
                    OpCode::Add => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if let Some(x) = a + b {
                            self.stack.push(x);
                        } else {
                            return StepResult::Error(RuntimeError::IncompatibleType);
                        }
                    }
                    OpCode::Sub => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if let Some(x) = b - a {
                            self.stack.push(x);
                        } else {
                            return StepResult::Error(RuntimeError::IncompatibleType);
                        }
                    }
                    OpCode::Mul => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if let Some(x) = a * b {
                            self.stack.push(x);
                        } else {
                            return StepResult::Error(RuntimeError::IncompatibleType);
                        }
                    }
                    OpCode::Eq => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if a == b {
                            self.stack.push(Data::Int(1));
                        } else {
                            self.stack.push(Data::Int(0));
                        }
                    }
                    OpCode::Gr => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if b > a {
                            self.stack.push(Data::Int(1));
                        } else {
                            self.stack.push(Data::Int(0));
                        }
                    }
                    OpCode::Less => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };

                        if b < a {
                            self.stack.push(Data::Int(1));
                        } else {
                            self.stack.push(Data::Int(0));
                        }
                    }
                    OpCode::Not => {
                        let a = if let Some(a) = self.stack.last() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        match a {
                            Data::Int(a) => {
                                if *a == 0 {
                                    self.stack.push(Data::Int(1));
                                } else {
                                    self.stack.push(Data::Int(0));
                                }
                            }
                            Data::Float(a) => {
                                if a.is_zero() {
                                    self.stack.push(Data::Int(1));
                                } else {
                                    self.stack.push(Data::Int(0));
                                }
                            }
                        }
                    }
                    OpCode::Dup => {
                        let a = if let Some(a) = self.stack.last() {
                            a
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                        self.stack.push(*a);
                    }
                    OpCode::Jump => {
                        if let Operands::One(Data::Int(new_ip)) = instr.operands {
                            self.ip = new_ip as usize;
                            return StepResult::Ok;
                        } else {
                            return StepResult::Error(RuntimeError::InvalidValue);
                        }
                    }
                    OpCode::CJump => {
                        if let Some(a) = self.stack.last() {
                            if let Data::Int(cond) = a {
                                if *cond > 0 {
                                    if let Operands::One(Data::Int(new_ip)) = instr.operands {
                                        self.ip = new_ip as usize;
                                        return StepResult::Ok;
                                    } else {
                                        return StepResult::Error(RuntimeError::InvalidValue);
                                    }
                                }
                            } else {
                                return StepResult::Error(RuntimeError::InvalidValue);
                            }
                        } else {
                            return StepResult::Error(RuntimeError::NotEnoughArgs);
                        };
                    }
                    OpCode::Panic => {
                        return StepResult::Panic("Panic from code");
                    }
                    OpCode::Exit => return StepResult::Exit,
                    OpCode::Nop => {}
                }
                self.ip += 1;
                StepResult::Ok
            }
            None => StepResult::Exit,
        }
    }

    pub fn run<T: Write>(&mut self, output: &mut T) -> ExitStatus {
        loop {
            let a = self.step(output);
            match a {
                StepResult::Error(e) => return ExitStatus::Error(e),
                StepResult::Panic(e) => {
                    self.panic(output, e);
                    return ExitStatus::Panic;
                }
                StepResult::Ok => {}
                StepResult::Exit => return ExitStatus::Ok,
            }
        }
    }

    pub fn goto(&mut self, ip: usize) {
        self.ip = ip;
    }

    pub fn stack(&self) -> &Vec<Data> {
        &self.stack
    }

    pub fn panic<T: Write>(&mut self, output: &mut T, error: &str) {
        if writeln!(output, "Oops...").is_err() {}
        if writeln!(output, "Error occurs: {}", error).is_err() {}
    }
}
