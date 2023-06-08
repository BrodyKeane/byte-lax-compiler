use super::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("\n======= {} =======", name);
    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
    println!();
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:<04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("  | ");
    } else {
        print!("{:<4}", chunk.lines[offset]);
    }

    let instruction = chunk.code[offset];
    match instruction.into() {
        OpCode::Return =>
            simple_instruction(OpCode::Return, offset),

        OpCode::Constant =>
            constant_instruction(OpCode::Constant, chunk, offset),

        OpCode::None => {
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
    println!("{}", instruction);
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
