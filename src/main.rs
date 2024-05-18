#[macro_use] extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

type Value = f64;

enum_from_primitive! {
#[derive(Debug, PartialEq)]
#[repr(u8)]
/// Operation code.
enum OpCode {
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
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn write(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    fn add_constant(&mut self, value: Value) -> usize{
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
            OpCode::Constant => constant_instruction("OP_CONSTANT", &self, offset),
            OpCode::Add => simple_instruction("OP_ADD", offset),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => simple_instruction("OP_MULTPLY", offset),
            OpCode::Divide => simple_instruction("OP_DIVIDE", offset),
            OpCode::Negate => simple_instruction("OP_NEGATE", offset),
            OpCode::Return => simple_instruction("OP_RETURN", offset),
        }
    }
}

const STACK_MAX: usize = 256;

struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: [Value; 256],
    stack_top: usize,
}

impl<'a> VM<'a> {
    fn new(chunk: &'a Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: [0.0; STACK_MAX],
            stack_top: 0,
        }
    }

    fn read_byte(&mut self) -> u8 {
        let code = self.chunk.code[self.ip];
        self.ip += 1;
        code
    }

    fn read_constant(&mut self) -> Value {
        let code = self.read_byte() as usize;
        self.chunk.constants[code]
    }

    fn push_value(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    fn pop_value(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn binary_op(&mut self, op: impl Fn(f64, f64) -> f64) {
        let a = self.pop_value();
        let b = self.pop_value();
        self.push_value(op(a, b));
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    print!("{}\n", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1] as usize;
    print!("{:<16} {:>4} '", name, constant);
    print_value(&chunk.constants[constant]);
    print!("'\n");
    offset + 2
}

fn print_value(value: &Value) {
    print!("{:.6}", value);
}

#[derive(Debug)]
enum InterpretError {
    CompileError,
    RuntimeError(String),
}

type InterpretResult<'a> = Result<(), InterpretError>;

fn interpret(chunk: &Chunk) -> InterpretResult {
    let mut vm = VM::new(chunk);
    run(&mut vm)
}

fn run(vm: &mut VM) -> InterpretResult<'static> {
    loop {
        #[cfg(feature = "debug_trace_execution")]
        {
            print!("          ");
            for i in 0..vm.stack_top {
                let slot = vm.stack[i];
                print!("[ ");
                print_value(&slot);
                print!(" ]");
            }
            print!("\n");
            vm.chunk.disassemble_instruction(vm.ip);
        }
        let instruction = vm.read_byte();
        let op_code = OpCode::from_u8(instruction);
        match op_code {
            Some(OpCode::Constant) => {
                let constant = vm.read_constant();
                vm.push_value(constant);
            }
            Some(OpCode::Add) => vm.binary_op(|a: Value, b: Value| { a + b }),
            Some(OpCode::Subtract) => vm.binary_op(|a: Value, b: Value| { a - b }),
            Some(OpCode::Multiply) => vm.binary_op(|a: Value, b: Value| { a * b }),
            Some(OpCode::Divide) => vm.binary_op(|a: Value, b: Value| { a / b }),
            Some(OpCode::Negate) => {
                let value = vm.pop_value();
                vm.push_value(-1.0 * value);
            }
            Some(OpCode::Return) => {
                print_value(&vm.pop_value());
                print!("\n");
                return Ok(())
            }
            _ => return Err(InterpretError::RuntimeError(format!("Instruction {instruction} not recognized!")))
        }

    }
}

fn main() -> InterpretResult<'static> {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Add as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Divide as u8, 123);

    chunk.write(OpCode::Negate as u8, 123);
    chunk.write(OpCode::Return as u8, 123);
    chunk.disassemble("debug");
    interpret(&chunk)
}
