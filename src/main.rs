#[macro_use] extern crate enum_primitive;
extern crate num;
use clap::Parser;
use num::FromPrimitive;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

mod scanner;
use scanner::Scanner;

mod token;
use token::{Token, TokenType};

mod vm;
use vm::{interpret, InterpretResult, InterpretError};

type Value = f64;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
#[repr(u8)]
/// Operation code.
pub enum OpCode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}
}


struct Chunk {
    code: Vec<u8>,
    lines: Vec<u32>,
    constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize{
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);


        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        let op_code = OpCode::from_u8(instruction).unwrap();
        match op_code {
            OpCode::Constant => constant_instruction("OP_CONSTANT", self, offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} '", name, constant);
    print_value(&chunk.constants[constant]);
    println!("'");
    offset + 2
}

fn print_value(value: &Value) {
    print!("{:.6}", value);
}

pub fn to_ascii_chars(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii()).collect()
}

fn compile(source: &str, chunk: &mut Chunk) -> InterpretResult<'static> {
    let mut scanner = Scanner::new(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:?} '{}'", token.token_type, token.content);

        if token.token_type == TokenType::Error {
            return InterpretResult::Err(InterpretError::CompileError(token.content));
        }
        if token.token_type == TokenType::Eof {
            return Ok(())
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional file name
    input_file: Option<PathBuf>,
}

fn main() -> InterpretResult<'static> {
    let args = Args::parse();
    if let Some(file_path) = args.input_file {
        run_file(&file_path)
    } else {
        repl()
    }
}

fn run_file(file_path: &PathBuf) -> InterpretResult<'static> {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            interpret(&source)
        }
        Err(_) => {
            InterpretResult::Err(InterpretError::CompileError("Could not read file.".to_string()))
        }
    }
}

fn repl() -> InterpretResult<'static> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut stdout = io::stdout();

    println!("Lox REPL (press Ctrl+D or Ctrl+Z to finish):");
    loop {
        print!("> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut source = String::new();
        match reader.read_line(&mut source) {
            Ok(0) => break, // Exit via Ctrl+D / Ctrl+Z
            Ok(_) => {
                let result = interpret(source.trim_end());
                if let Err(interpret_error) = result {
                    match interpret_error {
                        InterpretError::CompileError(output) => eprintln!("Compilation error: {}", output),
                        InterpretError::RuntimeError(output) => eprintln!("Runtime error: {}", output)
                    }
                }
            }
            Err(err) => {
                eprint!("Error reading line: {}", err);
                break;
            }
        }
    }

    Ok(())
}
