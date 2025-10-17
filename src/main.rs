use nora::prelude::*;
fn main() {
    subscribe();
    let mut neuron_1 = Neuron::new("N1");
    let neuron_2 = Neuron::new("N2");
    //let junction = ActionPotential::default();

    neuron_2.tx_to(&mut neuron_1);
    neuron_2.fire(1);

    let mut brain = Brain::new("Brain");

    brain.add(neuron_1);
    brain.add(neuron_2);

    brain.update();
}
