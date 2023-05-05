pub mod byte_block;
pub mod compiler;
pub mod constant;
pub mod disassembler;
pub mod lexer;
pub mod stack;
pub mod virtual_machine;

use compiler::Compiler;
use virtual_machine::{VirtualMachine, InterpretResult};

use std::io::{stdin, stdout, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn repl() {
    print_welcome_msg();

    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut line = String::new();

        stdin().read_line(&mut line).unwrap();
        run(&line);
    }
}

pub fn run(source: &str) -> InterpretResult {
    let mut compiler = Compiler::new(source);

    if !compiler.compile() {
        return InterpretResult::CompileError;
    }

    let mut vm = VirtualMachine::new();
    vm.interpret(compiler.bytes())
}

pub fn run_file(path: &str) {
    if let Ok(source) = std::fs::read_to_string(path) {
        run(&source);
    } else {
        println!("Failed to open file from path: /{path}");
    }
}

pub fn print_usage() {
    println!("Usage: dynamix <script>");
    println!("Args:");
    println!("\tscript: source filepath");
    println!();
    println!("(Hint: run dynamix with no args to start the interactive REPL)");
}

fn print_welcome_msg() {
    println!(
        "Welcome to Dynamix {VERSION}, running {} on platform {}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );
}
