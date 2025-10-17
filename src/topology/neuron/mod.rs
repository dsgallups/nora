mod axon;

pub use axon::*;

mod dendrite;
pub use dendrite::*;

mod soma;
pub use soma::*;

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
}

#[test]
fn simple_brain() {
    use crate::prelude::*;

    let mut neuron_1 = Neuron::new("N1");
    let mut neuron_2 = Neuron::new("N2");
    let junction = ActionPotential::default();

    neuron_2.tx_to(&mut neuron_1);

    let mut brain = Brain::new("Brain");

    brain.add(&neuron_1);
    brain.add(&neuron_2);
    //brain.add(&junction);

    std::thread::spawn(move || {
        neuron_2.fire();
    });

    brain.run();
}
