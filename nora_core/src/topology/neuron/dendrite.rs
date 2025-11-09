use std::fmt;

use tracing::info;
use trotcast::error::TryRecvError;
use uuid::Uuid;

use crate::prelude::{Neuron, NeuronRx};

#[derive(Debug, Default)]
pub struct Dendrites(Vec<Dendrite>);

impl Dendrites {
    pub fn push(&mut self, dendrite: Dendrite) {
        self.0.push(dendrite)
    }
    pub fn swap_remove(&mut self, index: usize) -> Dendrite {
        self.0.swap_remove(index)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Dendrite> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Dendrite> {
        self.0.iter_mut()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn prune_the_dead(&mut self) {
        self.0.retain(|dendrite| !dendrite.is_dead())
    }
    pub fn find(&self, id: Uuid) -> Option<&Dendrite> {
        self.0.iter().find(|d| d.id == id)
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct Disconnected;

impl Dendrite {
    pub fn new(name: impl Into<String>, neuron: &Neuron) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            connected_to: neuron.id(),
            rx: neuron.spawn_rx(),
        }
    }
    pub fn is_dead(&self) -> bool {
        self.rx.closed()
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

    pub fn read_potential(&mut self) -> Result<u8, Disconnected> {
        match self.rx.try_recv() {
            Ok(val) => {
                info!("\t{} - Received {val}", self.name);
                Ok(val)
            }
            Err(err) => match err {
                TryRecvError::Disconnected => Err(Disconnected),
                _ => Ok(0),
            },
        }
    }
}
