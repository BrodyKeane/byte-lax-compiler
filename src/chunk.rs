#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    None,
    OpReturn,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chunk {
    count: usize,
    capacity: usize,
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    pub fn write(&mut self, byte: OpCode) {
        if self.capacity <= self.count {
            self.grow_capacity();
            self.grow_code();
        }
        self.code[self.count] = byte;
        self.count += 1;
    }

    pub fn free(&mut self) {
        *self = Chunk::default();
    }

    fn grow_capacity(&mut self) {
        self.capacity = {
            if self.capacity < 8 {8}
            else {self.capacity * 2}
        }
    }

    fn grow_code(&mut self) {
        self.code.resize(self.capacity, OpCode::None)
    }

    pub fn disassemble(&self, name: String) {
        println!("== {} ==", name);
        for (i, op) in self.code.iter().enumerate() {
            if let OpCode::None = op {continue}
            println!("{:<04}: {:?}", i, op);
        }
    }
}
