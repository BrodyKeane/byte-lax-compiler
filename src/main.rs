use std::env;

use chunk::{OpCode, Chunk};

pub mod chunk;

fn main() { 
    let args: Vec<String> = env::args().collect();
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test chunk".into());
    chunk.free();
}
