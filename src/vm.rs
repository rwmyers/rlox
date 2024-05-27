use crate::{compile, Chunk, OpCode, print_value, Value};
use num::FromPrimitive;

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
            println!();
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
                println!();
                return Ok(())
            }
            _ => return Err(InterpretError::RuntimeError(format!("Instruction {instruction} not recognized!")))
        }

    }
}

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
}

pub type InterpretResult<'a> = Result<(), InterpretError>;

pub fn interpret(source: &str) -> InterpretResult {
    let mut chunk = Chunk::new();
    compile(source, &mut chunk)?;
    Ok(())
}