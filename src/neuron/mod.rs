pub mod axon;
pub mod dendrite;
pub mod soma;

use rand::{rng, seq::IndexedRandom};

use crate::prelude::*;

#[doc = r#"

## Notes
an axon of a neuron can be connected to its own dendrite. This is called an autapses.


Neurons regulate their own excitability, but are told where to grow their axons to

"#]
pub struct Neuron {
    axon: Axon,
    soma: Soma,
    dendrites: Vec<Dendrite>,
}

impl Default for Neuron {
    fn default() -> Self {
        Self {
            axon: Axon::default(),
            soma: Soma::default(),
            dendrites: Vec::new(),
        }
    }
}

impl Neuron {
    /// This sends an electrical pulse down its axon for dendritic receptors to receive
    /// This may reset an interior an action potential
    pub fn pulse(&mut self) {}

    /// When called, this will do something with the pulses it has received from attached axons
    pub fn handle_pulses(&mut self) {}

    /// axons search/follow queues
    pub fn attach_to(&mut self, dendritic_receptor: &Dendrite) {
        self.axon.attach_to(dendritic_receptor);
    }

    pub fn random_dendrite(&self) -> Option<&Dendrite> {
        self.dendrites.choose(&mut rng())
    }

    pub fn grow_dendrite(&mut self) {
        self.dendrites.push(Dendrite::default())
    }

    /// axons
    pub fn repulse_from(&mut self, dendritic_receptor: Dendrite) {}
}

#[test]
fn make_default_neuron() {
    Neuron::default();
}

#[test]
fn attach_two_neurons() {
    let mut sender = Neuron::default();
    let mut receiver = Neuron::default();
    receiver.grow_dendrite();

    sender.attach_to(receiver.random_dendrite().unwrap())
}
