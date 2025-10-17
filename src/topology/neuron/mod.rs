mod axon;

pub use axon::*;

mod dendrite;
pub use dendrite::*;

mod soma;
pub use soma::*;
use tracing::info;

use crate::prelude::*;

pub type NeuronChannel = Channel<u8>;
pub type NeuronRx = Receiver<u8>;

#[derive(Debug)]
pub struct Neuron {
    name: String,
    axon: Axon,
    dendrites: Vec<Dendrite>,
}

impl Neuron {
    pub fn new(name: impl Into<String>) -> Self {
        let name: String = name.into();
        Self {
            axon: Axon::new(format!("{name} Axon")),
            name,
            dendrites: Vec::new(),
        }
    }
    /// "To bring into"
    pub fn tx_to(&self, receiver: &mut Neuron) {
        receiver.rx_from(&self.axon)
    }
    pub fn rx_from(&mut self, axon: &Axon) {
        let recv = axon.spawn_rx();
        self.dendrites
            .push(Dendrite::new(format!("{} Dendrite", &self.name), recv))
    }

    pub fn fire(&self, value: u8) {
        self.axon.fire(value)
    }

    pub fn update(&mut self) {
        let mut ap = 0;
        for dendrite in &mut self.dendrites {
            ap += dendrite.read_potential();
        }
        info!("{} AP: {ap}", self.name);
    }
}
