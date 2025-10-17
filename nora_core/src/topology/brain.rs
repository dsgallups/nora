use tracing::info;

use crate::prelude::*;

#[derive(Debug)]
pub struct Brain {
    name: String,
    neurons: Vec<Neuron>,
}

impl Brain {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            neurons: Vec::new(),
        }
    }

    pub fn add(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }

    pub fn update(&mut self) {
        info!("{} - RUNNING UPDATE SCHEDULE", self.name);
        for neuron in &mut self.neurons {
            neuron.update()
        }
        info!("{} - SCHEDULE COMPLETE", self.name);
    }
    pub fn neurons(&self) -> &[Neuron] {
        &self.neurons
    }
}

#[test]
fn simple_brain() {
    let mut neuron_1 = Neuron::new("N1");
    let neuron_2 = Neuron::new("N2");
    //let junction = ActionPotential::default();

    neuron_2.tx_to(&mut neuron_1);
    neuron_2.fire(1).unwrap();

    let mut brain = Brain::new("Brain");

    brain.add(neuron_1);
    brain.add(neuron_2);

    brain.update();
}
