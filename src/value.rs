pub type Value = f64;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValueArray {
    pub capacity: usize,
    pub count: usize,
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, value: Value) {
        if self.capacity <= self.count {
            self.grow_capacity();
            self.values.resize(self.capacity, 0.);
        }
        self.values[self.count] = value;
        self.count += 1;
    }

    pub fn free(&mut self) {
        *self = Self::default();
    }

    fn grow_capacity(&mut self) {
        self.capacity = {
            if self.capacity < 8 {8}
            else {self.capacity * 2}
        }
    }
}

