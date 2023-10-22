# Agar programming language

AgarASM is an assembler that produce agar bytecode from agar assembly language.
AgarASM also can disassembly agar bytecode.

AgarVM is a stack based virtual machine that executes agar bytecode.

## Usage

Agar assembler:
```bash
$ cargo run --bin agar_asm <source.aa>
```

Agar virtual machine:
```bash
$ cargo run --bin agar_vm <source.ab>
```
