use std::fmt;

use tracing::info;
use uuid::Uuid;

use crate::prelude::{Neuron, NeuronRx};

pub struct Dendrite {
    id: Uuid,
    name: String,
    connected_to: Uuid,
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
    pub fn new(name: impl Into<String>, neuron: &Neuron) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            connected_to: neuron.id(),
            rx: neuron.spawn_rx(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn connected_to(&self) -> Uuid {
        self.connected_to
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
