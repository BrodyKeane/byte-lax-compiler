use std::env;

use chunk::{OpCode, Chunk};
use disassemble_chunk::disassemble_chunk;

pub mod memory;
pub mod chunk;
pub mod disassemble_chunk;
pub mod value;

fn main() { 
    let _args: Vec<String> = env::args().collect();
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::Constant as u8);
    chunk.write(constant);
    chunk.write(OpCode::Return as u8);

    disassemble_chunk(&chunk, "test chunk".into());
    chunk.free();
}
