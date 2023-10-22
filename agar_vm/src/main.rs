use std::{env, path::Path, io::stdout};

use agar_core::Program;
use agar_vm::Interpreter;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if let Some(raw_path) = args.get(1) {
        let path = Path::new(raw_path);
        let bytecode = std::fs::read(path).expect("Can't read bytecode");
        let program = Program::from_bytes(&bytecode).expect("Can't parse bytecode");

        let mut vm = Interpreter::new();
        vm.load_program(program);
        match vm.run(&mut stdout()) {
            agar_vm::ExitStatus::Error(e) => println!("RuntimeError: {e:?}"),
            _ => {}
        }
    }

    Ok(())
}
