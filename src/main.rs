use std::env;

use virtual_machine::vm::Vm;
use bytecode::{
    chunk::{OpCode, Chunk},
    disassemble_chunk::disassemble_chunk,
};

pub mod bytecode;
pub mod virtual_machine;

fn main() { 
    let _args: Vec<String> = env::args().collect();

    let mut vm = Vm::new();
    let mut chunk = Chunk::new();
    
    let constant = chunk.add_constant(1.2);

    chunk.write(OpCode::Constant, 123);
    chunk.write(constant, 123);
    chunk.write(OpCode::Return, 123);

    disassemble_chunk(&chunk, "test chunk".into());
    vm.interpret(&chunk);

    chunk.free();
    vm.free();
}
