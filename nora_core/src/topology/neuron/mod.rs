mod axon;

use std::fmt;

pub use axon::*;

mod dendrite;
pub use dendrite::*;

mod soma;
pub use soma::*;
use tracing::info;
use uuid::Uuid;

use crate::prelude::*;

pub type NeuronChannel = Channel<u8>;
pub type NeuronRx = Receiver<u8>;

pub struct Neuron {
    id: Uuid,
    name: String,
    axon: NeuronChannel,
    sensitization: i32,
    dendrites: Vec<Dendrite>,
}
impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Neuron")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("sensitization", &self.sensitization)
            .field("dendrites", &self.dendrites)
            .finish()
    }
}

impl Neuron {
    pub fn new(name: impl Into<String>) -> Self {
        let channel = Channel::new(20);
        let name: String = name.into();
        Self {
            id: Uuid::new_v4(),
            axon: channel,
            name,
            sensitization: 0,
            dendrites: Vec::new(),
        }
    }
    /// "To bring into"
    pub fn tx_to(&self, receiver: &mut Neuron) {
        receiver.rx_from(self)
    }
    pub fn rx_from(&mut self, neuron: &Neuron) {
        self.dendrites.push(Dendrite::new(
            format!("{} -> {} Dendrite", &neuron.name(), &self.name),
            neuron,
        ))
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn dendrites(&self) -> &[Dendrite] {
        &self.dendrites
    }

    pub fn spawn_rx(&self) -> NeuronRx {
        self.axon.spawn_rx()
    }

    pub fn fire(&self, value: u8) -> Result<(), SendError<u8>> {
        self.axon.send(value)
    }

    pub fn update(&mut self) {
        let mut ap = 0;

        info!("\t{} - Update", self.name);
        for dendrite in &mut self.dendrites {
            ap += dendrite.read_potential();
        }
        if ap as i32 > self.sensitization {
            info!("\t{} - Firing", self.name);
            _ = self.fire(1);
        }

        info!("\t{} - End", self.name);
    }
}
