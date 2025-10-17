mod axon;
pub use axon::*;

mod dendrite;
pub use dendrite::*;

mod soma;
pub use soma::*;

pub struct Neuron {
    axon: Axon,
    soma: Soma,
    dendrites: Vec<Dendrite>,
}

impl Default for Neuron {
    fn default() -> Self {
        todo!()
    }
}

// impl Neuron {
//     fn
// }

#[test]
fn simple_brain() {
    use crate::prelude::*;

    let mut neuron_1 = Neuron::default();
    let mut neuron_2 = Neuron::default();
    let junction = ActionPotential::default();

    neuron_1.affer(&neuron_2);
    neuron_2.affer(&junction);

    let mut brain = Brain::default();

    brain.add(&neuron_1);
    brain.add(&neuron_2);
    brain.add(&junction);

    std::thread::spawn(move || {
        junction.fire();
    });

    brain.run();
}
