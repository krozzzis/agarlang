mod a {
    use crate::*;
    use agar_core::*;

    #[test]
    fn add_int() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(34)),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(35)),
            },
            Instruction {
                op_code: OpCode::Add,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"69")
    }

    #[test]
    fn add_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(345, 1))),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(346, 1))),
            },
            Instruction {
                op_code: OpCode::Add,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"69.1")
    }

    #[test]
    fn sub_int() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(34)),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(35)),
            },
            Instruction {
                op_code: OpCode::Sub,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"-1")
    }

    #[test]
    fn sub_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(345, 1))),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(356, 1))),
            },
            Instruction {
                op_code: OpCode::Sub,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"-1.1")
    }

    #[test]
    fn mul_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(15, 1))),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(25, 1))),
            },
            Instruction {
                op_code: OpCode::Mul,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"3.75")
    }

    #[test]
    fn mul_int() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(20)),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(21)),
            },
            Instruction {
                op_code: OpCode::Mul,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"420")
    }

    #[test]
    fn user_exit() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(345, 1))),
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Float(Float::new(346, 1))),
            },
            Instruction {
                op_code: OpCode::Exit,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn add_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(34)),
            },
            Instruction {
                op_code: OpCode::PushFloat,
                operands: Operands::One(Data::Float(Float::new(355, 1))),
            },
            Instruction {
                op_code: OpCode::Add,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleTypes));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn sub_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(34)),
            },
            Instruction {
                op_code: OpCode::PushFloat,
                operands: Operands::One(Data::Float(Float::new(355, 1))),
            },
            Instruction {
                op_code: OpCode::Sub,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleTypes));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn mul_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(34)),
            },
            Instruction {
                op_code: OpCode::PushFloat,
                operands: Operands::One(Data::Float(Float::new(355, 1))),
            },
            Instruction {
                op_code: OpCode::Mul,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleTypes));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn nop() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn nop_with_add() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(20)),
            },
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int(21)),
            },
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Mul,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Print,
                operands: Operands::Zero,
            },
            Instruction {
                op_code: OpCode::Nop,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"420")
    }

    #[test]
    fn user_panic() {
        let mut vm = Interpreter::new();
        let ops = vec![
            Instruction {
                op_code: OpCode::Panic,
                operands: Operands::Zero,
            },
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        assert_eq!(status, ExitStatus::Panic);
    }
}
