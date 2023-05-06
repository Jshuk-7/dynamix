pub mod byte_block;
pub mod compiler;
pub mod constant;
pub mod disassembler;
pub mod lexer;
pub mod stack;
pub mod virtual_machine;

use compiler::Compiler;
use virtual_machine::{InterpretResult, VirtualMachine};

use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type RuntimeResult = (InterpretResult, String);

pub fn repl() {
    println!(
        "Welcome to Dynamix {VERSION}, running {} on platform {}",
        std::env::consts::ARCH,
        std::env::consts::OS
    );

    let mut vm = VirtualMachine::new();

    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut line = String::new();

        stdin().read_line(&mut line).unwrap();
        let mut compiler = Compiler::new(&line);

        if !compiler.compile() {
            continue;
        }

        let byte_code = compiler.byte_code();
        let result = vm.interpret(byte_code);
        if let InterpretResult::RuntimeError = result {
            let error = vm.last_runtime_error();
            print_result(result, "<stdin>", error);
        }
    }
}

pub fn run(source: &str) -> RuntimeResult {
    let mut compiler = Compiler::new(source);

    if !compiler.compile() {
        return (InterpretResult::CompileError, "".to_string());
    }

    let mut vm = VirtualMachine::new();
    let byte_code = compiler.byte_code();
    let result = vm.interpret(byte_code);
    let error = vm.last_runtime_error();
    (result, error)
}

pub fn run_file(path: &str) {
    if let Ok(source) = std::fs::read_to_string(path) {
        let (result, error) = run(&source);
        let filename = Path::new(path).file_stem().unwrap().to_str().unwrap();
        print_result(result, filename, error);
    } else {
        println!("Failed to open file from path: /{path}");
    }
}

fn print_result(result: InterpretResult, name: &str, error: String) {
    match result {
        InterpretResult::Ok => println!("program exited successfully..."),
        InterpretResult::CompileError => {
            println!("could not compile '{name}' due to previous error")
        }
        InterpretResult::RuntimeError => println!("thread 'main' panicked at: {error}"),
    }
}

pub fn print_usage() {
    println!("Usage: dynamix <script>");
    println!("Args:");
    println!("\tscript: source filepath");
    println!();
    println!("(Hint: run dynamix with no args to start the interactive REPL)");
}
