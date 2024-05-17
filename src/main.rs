use std::convert::TryFrom;

type Value = f64;

/// Operation code.
#[repr(u8)]
enum OpCode {
    Constant = 0,
    Return = 1,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Constant),
            1 => Ok(OpCode::Return),
            _ => Err("Invalid OpCode value"),
        }
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
        let op_code = OpCode::try_from(instruction).unwrap();
        match op_code {
            OpCode::Constant => {
                constant_instruction("OP_CONSTANT", &self, offset)
            }
            OpCode::Return => {
                simple_instruction("OP_RETURN", offset)
            }
        }
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

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2f64);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Return as u8, 123);
    chunk.disassemble("test chunk");
}
