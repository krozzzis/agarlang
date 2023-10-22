use crate::*;

#[test]
fn program_eq() {
    let ops = vec![Instruction {
        op_code: OpCode::PushInt,
        operands: Operands::One(Data::Int(15)),
    }];
    let program = Program { ops };
    let bytecode = program.to_bytes();
    let deser_program = Program::from_bytes(&bytecode);
    assert_eq!(Ok(program), deser_program);
}
