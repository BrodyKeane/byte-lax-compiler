use std::fmt;

use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    None,
    Constant,
    Return,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chunk {
    pub count: usize,
    pub capacity: usize,
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: ValueArray
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, byte: u8) {
        if self.capacity <= self.count {
            self.grow_capacity();
            self.code.resize(self.capacity, OpCode::None as u8);
        }
        self.code[self.count] = byte;
        self.count += 1;
    }

    pub fn free(&mut self) {
        *self = Self::default();
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        (self.constants.count - 1) as u8
    }

    fn grow_capacity(&mut self) {
        self.capacity = {
            if self.capacity < 8 {8}
            else {self.capacity * 2}
        }
    }

}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
