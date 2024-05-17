/// Operation code.
enum OpCode {
    Return,
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
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return as u8);
}
