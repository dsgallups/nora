use crate::{axon::Axon, prelude::*};

#[derive(Default)]
pub struct Neuron {
    axon: Axon,
}

impl Neuron {
    /// This sends an electrical pulse down its axon for dendritic receptors to receive
    /// This may reset an interior an action potential
    pub fn pulse(&mut self) {}

    /// When called, this will do something with the pulses it has received from attached axons
    pub fn handle_pulses(&mut self) {}

    /// axons search/follow queues
    pub fn attach_to(&mut self, dendritic_receptor: Dendrite) {}

    /// axons
    pub fn repulse_from(&mut self, dendritic_receptor: Dendrite) {}
}
