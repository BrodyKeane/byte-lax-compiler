use crate::bytecode::chunk::{Chunk, OpCode};

type ByteIterator = Box<dyn Iterator<Item = u8>>;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[derive(Default)]
pub struct Vm<'a> {
    chunk: Option<&'a Chunk>,
    ip: Option<ByteIterator>,
}

impl<'a> Vm<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn free(&mut self) {
        *self = Self::default()
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = Some(Box::new(chunk.code.into_iter()));
        self.run()
    }

    fn run(&self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction.into() {
                OpCode::Return => return InterpretResult::Ok,
                _ => return InterpretResult::CompileError
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        if let Some(mut ip) = self.ip {
            ip.next().unwrap_or(OpCode::None.into())
        } 
        else {OpCode::None.into()}
    }
}
