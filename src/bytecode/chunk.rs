use std::fmt;

use super::value::{Value, ValueArray};

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

    pub fn write<T: Into<u8>>(&mut self, byte: T, line: usize) {
        if self.capacity <= self.count {
            self.grow_capacity();
            self.code.resize(self.capacity, OpCode::None.into());
            self.lines.resize(self.capacity, 0)
        }
        self.code[self.count] = byte.into();
        self.lines[self.count] = line;
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

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> u8 {
        value as u8
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> OpCode {
        match value {
            x if x == OpCode::Constant.into() => OpCode::Constant,
            x if x == OpCode::Return.into() => OpCode::Return,
            x if x == OpCode::EOF.into() => OpCode::EOF,
            _ => OpCode::None
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
