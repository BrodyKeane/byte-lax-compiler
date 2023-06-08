use std::env;

use bytecode::{
    chunk::{OpCode, Chunk},
    disassemble_chunk::disassemble_chunk,
};

pub mod bytecode;

fn main() { 
    let _args: Vec<String> = env::args().collect();
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::Constant, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Return, 123);

    disassemble_chunk(&chunk, "test chunk".into());
    chunk.free();
}
