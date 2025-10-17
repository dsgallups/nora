use crate::prelude::NumJunction;

pub struct Brain {}

impl Brain {
    // To "bring in"
    pub fn affer(&self, input: &NumJunction) {}

    pub fn effer(&self, output: &NumJunction) {}
}

impl Default for Brain {
    fn default() -> Self {
        Self {}
    }
}
