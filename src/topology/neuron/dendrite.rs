use std::fmt;

use tracing::info;

use crate::prelude::NeuronRx;

pub struct Dendrite {
    name: String,
    rx: NeuronRx,
}
impl fmt::Debug for Dendrite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dendrite")
            .field("name", &self.name)
            .finish()
    }
}

impl Dendrite {
    pub fn new(name: impl Into<String>, rx: NeuronRx) -> Self {
        Self {
            name: name.into(),
            rx,
        }
    }
    pub fn read_potential(&mut self) -> u8 {
        match self.rx.try_recv() {
            Ok(val) => {
                info!("\t{} - Received {val}", self.name);
                val
            }
            Err(_) => {
                //todo
                0
            }
        }
    }
}
