mod a {
    use crate::*;
    use agar_core::*;
    use agar_macro::panic as pnic;
    use agar_macro::print as prnt;
    use agar_macro::*;

    #[test]
    fn add_int() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), pushi!(35), add!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"69")
    }

    #[test]
    fn add_float() {
        let mut vm = Interpreter::new();
        let ops = vec![pushf!("34.5"), pushf!("34.6"), add!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"69.1")
    }

    #[test]
    fn sub_int() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), pushi!(35), sub!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"-1")
    }

    #[test]
    fn sub_float() {
        let mut vm = Interpreter::new();
        let ops = vec![pushf!("34.5"), pushf!("35.6"), sub!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"-1.1")
    }

    #[test]
    fn mul_float() {
        let mut vm = Interpreter::new();
        let ops = vec![pushf!("1.5"), pushf!("2.5"), mul!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"3.75")
    }

    #[test]
    fn mul_int() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(20), pushi!(21), mul!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"420")
    }

    #[test]
    fn user_exit() {
        let mut vm = Interpreter::new();
        let ops = vec![pushf!("34.5"), pushf!("34.6"), exit!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![float!("34.5"), float!("34.6"),]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn add_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), pushf!("35.5"), add!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleType));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn sub_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), pushf!("35.5"), sub!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleType));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn mul_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), pushf!("35.5"), mul!(), prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleType));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn nop() {
        let mut vm = Interpreter::new();
        let ops = vec![nop!(), nop!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn nop_with_add() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(20),
            nop!(),
            pushi!(21),
            nop!(),
            mul!(),
            nop!(),
            prnt!(),
            nop!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"420")
    }

    #[test]
    fn user_panic() {
        let mut vm = Interpreter::new();
        let ops = vec![pnic!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Panic);
    }

    #[test]
    fn dup() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(34), dup!(), pushf!("35.5"), dup!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"");
        assert_eq!(
            *stack,
            vec![int!(34), int!(34), float!("35.5"), float!("35.5"),]
        );
    }

    #[test]
    fn print_char() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(66), pushi!(65), printch!(), printch!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"AB")
    }

    #[test]
    fn print_char_type_error() {
        let mut vm = Interpreter::new();
        let ops = vec![pushf!("35.5"), printch!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::IncompatibleType));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn add_not_enough_args1() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(100), add!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn add_not_enough_args2() {
        let mut vm = Interpreter::new();
        let ops = vec![add!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn sub_not_enough_args1() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(100), sub!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn sub_not_enough_args2() {
        let mut vm = Interpreter::new();
        let ops = vec![sub!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }
    #[test]
    fn mul_not_enough_args1() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(100), mul!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn mul_not_enough_args2() {
        let mut vm = Interpreter::new();
        let ops = vec![mul!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn print_not_enough_args() {
        let mut vm = Interpreter::new();
        let ops = vec![prnt!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn print_char_not_enough_args() {
        let mut vm = Interpreter::new();
        let ops = vec![printch!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![]);
        assert_eq!(status, ExitStatus::Error(RuntimeError::NotEnoughArgs));
        assert_eq!(buffer, b"")
    }

    #[test]
    fn eq_int_int() {
        let mut vm = Interpreter::new();
        let ops = vec![pushi!(60), pushi!(50), eq!(), pushi!(40), pushi!(40), eq!()];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(0), int!(1)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn eq_int_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(60),
            pushf!("50.0"),
            eq!(),
            pushf!("40.0"),
            pushi!(40),
            eq!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(0), int!(1)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }
    #[test]
    fn eq_float_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushf!("50.0"),
            pushf!("5.1"),
            eq!(),
            pushf!("40.0"),
            pushf!("40.0"),
            eq!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(0), int!(1)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn gr_int_int() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(60),
            pushi!(50),
            gr!(),
            pushi!(40),
            pushi!(40),
            gr!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(1), int!(0)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn gr_int_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(50),
            pushf!("5.1"),
            gr!(),
            pushf!("40.0"),
            pushi!(40),
            gr!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(1), int!(0)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }
    #[test]
    fn gr_float_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushf!("50.0"),
            pushf!("51.0"),
            gr!(),
            pushf!("40.0"),
            pushf!("41.0"),
            gr!(),
            pushf!("4.11"),
            pushf!("4.1"),
            gr!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(0), int!(0), int!(1)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }
    #[test]
    fn less_int_int() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(50),
            pushi!(60),
            less!(),
            pushi!(40),
            pushi!(40),
            less!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(1), int!(0)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }

    #[test]
    fn less_int_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushi!(40),
            pushf!("41.0"),
            less!(),
            pushf!("40.0"),
            pushi!(40),
            less!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![int!(1), int!(0)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"")
    }
    #[test]
    fn less_float_float() {
        let mut vm = Interpreter::new();
        let ops = vec![
            pushf!("4.89"),
            pushf!("4.9"),
            less!(),
            pushf!("40.0"),
            pushf!("40.0"),
            less!(),
        ];
        let mut buffer = Vec::new();
        let program = Program { ops };
        vm.load_program(program);
        let status = vm.run(&mut buffer);
        let stack = vm.stack();
        assert_eq!(*stack, vec![Data::Int(1), Data::Int(0)]);
        assert_eq!(status, ExitStatus::Ok);
        assert_eq!(buffer, b"");
    }
}
