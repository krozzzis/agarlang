use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn pushi(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!(
            "Instruction {{
                op_code: OpCode::PushInt,
                operands: Operands::One(Data::Int({a})),
            }}"
        )
        .parse()
        .unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn pushf(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!(
            "Instruction {{
                op_code: OpCode::PushFloat,
                operands: Operands::One(Data::Float(Float::from_str_exact({a}).unwrap())),
            }}"
        )
        .parse()
        .unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn jump(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!(
            "Instruction {{
                op_code: OpCode::Jump,
                operands: Operands::One(Data::Int({a}))),
            }}"
        )
        .parse()
        .unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn cjump(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!(
            "Instruction {{
                op_code: OpCode::CJump,
                operands: Operands::One(Data::Int({a}))),
            }}"
        )
        .parse()
        .unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn int(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!("agar_core::Data::Int({a})").parse().unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn float(item: TokenStream) -> TokenStream {
    match item.into_iter().next() {
        Some(TokenTree::Literal(a)) => format!("agar_core::Data::Float(agar_core::Float::from_str_exact({a}).unwrap())")
            .parse()
            .unwrap(),
        _ => TokenStream::new(),
    }
}

#[proc_macro]
pub fn less(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Less,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn gr(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Gr,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn eq(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Eq,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn not(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Not,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn printch(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::PrintChar,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn dup(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Dup,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn add(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Add,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn sub(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Sub,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn mul(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Mul,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn print(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Print,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn exit(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Exit,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn nop(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Nop,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}

#[proc_macro]
pub fn panic(_item: TokenStream) -> TokenStream {
    "Instruction {
        op_code: OpCode::Panic,
        operands: Operands::Zero,
    }"
    .parse()
    .unwrap()
}
