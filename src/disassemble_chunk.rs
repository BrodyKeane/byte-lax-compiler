use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("\n===== {} =====", name);
    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
    println!();
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:<04} ", offset);
    let instruction = chunk.code[offset];
    match instruction {
        x if x == OpCode::Return as u8 =>
            simple_instruction(OpCode::Return, offset),

        x if x == OpCode::Constant as u8 =>
            constant_instruction(OpCode::Constant, chunk, offset),

        x if x == OpCode::None as u8 => {
            println!("Undefined opcode {}", OpCode::None);
            offset + 1
        }
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

fn simple_instruction(instruction: OpCode, offset: usize) -> usize {
    println!("{:?}", instruction);
    offset + 1
}

fn constant_instruction(instruction: OpCode, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = chunk.constants.values.get(constant as usize);
    match value {
        Some(value) => 
            println!("{:<16} {:<4} {}", instruction, constant, value),
        None => 
            println!("{:<16} {:<4} value not found", instruction, constant),
    }
    offset + 2
}
