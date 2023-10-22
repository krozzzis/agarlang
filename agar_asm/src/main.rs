use std::{env, fs, path::Path};

use agar_asm::Assembler;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if let Some(raw_path) = args.get(1) {
        let path = Path::new(raw_path);
        let source: String = std::fs::read_to_string(path).expect("Can't read source file");
        let mut asm = Assembler::new(source);
        if let Some(program) = asm.parse_source() {
            let bytecode = program.to_bytes();
            fs::write(path.with_extension("ab"), bytecode).expect("Can't save bytecode in file");
        } else {
            println!("Can't parse source file")
        }
    }

    Ok(())
}
