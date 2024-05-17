use std::convert::TryFrom;

/// Operation code.
#[repr(u8)]
enum OpCode {
    Return = 0,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Return),
            _ => Err("Invalid OpCode value"),
        }
    }
}


struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    fn new() -> Self {
        Chunk {
            code: Vec::new(),
        }
    }

    fn write(&mut self, byte: u8) {
        self.code.push(byte);
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

        let instruction = self.code[offset];
        let op_code = OpCode::try_from(instruction).unwrap();
        match op_code {
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

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return as u8);
    chunk.disassemble("test chunk");
}
