use nora::prelude::*;
fn main() {
    subscribe();
    let mut neuron_1 = Neuron::new("N1");
    let mut neuron_2 = Neuron::new("N2");
    let mut neuron_3 = Neuron::new("N3");
    //let junction = ActionPotential::default();
    neuron_3.tx_to(&mut neuron_2);
    neuron_2.tx_to(&mut neuron_1);
    neuron_1.tx_to(&mut neuron_3);
    _ = neuron_2.fire(1);

    let mut brain = Brain::new("Brain");

    brain.add(neuron_1);
    brain.add(neuron_2);
    brain.add(neuron_3);

    for _ in 0..5 {
        brain.update();
    }
}
