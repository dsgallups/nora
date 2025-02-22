pub struct NumJunction {
    value: i32,
}

impl NumJunction {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
