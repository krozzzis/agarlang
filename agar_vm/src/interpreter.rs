use agar_core::{Data, OpCode, Operands, Program};

pub enum StepResult {
    Ok,
    Error,
    Exit,
}

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

    pub fn step(&mut self) -> StepResult {
        match self.program.get(self.ip) {
            Some(instr) => {
                match instr.op_code {
                    OpCode::PushInt => {
                        if let Operands::One(data) = instr.operands {
                            self.stack.push(data.clone());
                        }
                    }
                    OpCode::PushFloat => {
                        if let Operands::One(data) = instr.operands {
                            self.stack.push(data.clone());
                        }
                    }
                    OpCode::Print => {
                        let data = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for print");
                            return StepResult::Error;
                        };
                        println!("{}", data);
                    }
                    OpCode::Add => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for add");
                            return StepResult::Error;
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for add");
                            return StepResult::Error;
                        };

                        if let Some(x) = a + b {
                            self.stack.push(x);
                        } else {
                            self.panic("Incompatible arguments for sum")
                        }
                    }
                    OpCode::Sub => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for sub");
                            return StepResult::Error;
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for sub");
                            return StepResult::Error;
                        };

                        if let Some(x) = b - a {
                            self.stack.push(x);
                        } else {
                            self.panic("Incompatible arguments for sub")
                        }
                    }
                    OpCode::Mul => {
                        let a = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for multiplication");
                            return StepResult::Error;
                        };
                        let b = if let Some(a) = self.stack.pop() {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for multiplication");
                            return StepResult::Error;
                        };

                        if let Some(x) = a * b {
                            self.stack.push(x);
                        } else {
                            self.panic("Incompatible arguments for multiplication")
                        }
                    }
                    OpCode::Dup => {
                        let a = if let Some(a) = self.stack.get(self.stack.len() - 1) {
                            a
                        } else {
                            self.panic("Not enough arguments on stack for duplication");
                            return StepResult::Error;
                        };
                        self.stack.push(a.clone());
                    }
                    OpCode::Nop => {}
                    OpCode::Panic => {
                        self.panic("Panic from the code");
                        return StepResult::Error;
                    }
                    OpCode::Exit => return StepResult::Exit,
                }
                self.ip += 1;
                StepResult::Ok
            }
            None => StepResult::Exit,
        }
    }

    pub fn run(&mut self) {
        while let StepResult::Ok = self.step() {}
    }

    pub fn goto(&mut self, ip: usize) {
        self.ip = ip;
    }

    pub fn stack(&self) -> &Vec<Data> {
        &self.stack
    }

    pub fn panic(&mut self, error: &str) {
        println!("Oops...");
        println!("Error occurs: {}", error);
    }
}
