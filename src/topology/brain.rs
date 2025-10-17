use crate::prelude::*;

#[derive(Debug)]
pub struct Brain {
    name: String,
    neurons: Vec<Neuron>,
}

impl Brain {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            neurons: Vec::new(),
        }
    }
}
