use std::collections::HashSet;

use rand::{Rng, SeedableRng, rngs::StdRng};
use tracing::info;
use uuid::Uuid;

use crate::prelude::*;

#[derive(Debug)]
pub struct Brain {
    name: String,
    rng: StdRng,
    neurons: Vec<Neuron>,
}

impl Brain {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rng: StdRng::seed_from_u64(12829231),
            neurons: Vec::new(),
        }
    }

    pub fn add(&mut self, neuron: Neuron) {
        self.neurons.push(neuron);
    }
    /// dendrites awaiting messages
    pub fn update_dendrites(&mut self) -> impl Iterator<Item = DendriteMessage> {
        self.neurons.iter_mut().flat_map(|neuron| {
            let neuron_id = neuron.id();
            neuron
                .update_dendrites()
                .into_iter()
                .map(move |(dendrite_id, value)| DendriteMessage {
                    neuron_id,
                    dendrite_id,
                    current_potential: value,
                })
        })
    }
    pub fn update_axons(&mut self) -> impl Iterator<Item = AxonMessage> {
        self.neurons.iter_mut().map(|neuron| AxonMessage {
            id: neuron.id(),
            discharge: neuron.update_axon(),
        })
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
    pub fn get_neuron(&self, id: Uuid) -> &Neuron {
        self.neurons.iter().find(|n| n.id() == id).unwrap()
    }
    pub fn get_neuron_mut(&mut self, id: Uuid) -> &mut Neuron {
        self.neurons.iter_mut().find(|n| n.id() == id).unwrap()
    }

    pub fn add_neuron(&mut self) {
        let num_neurons = self.neurons.len();

        let mut sends_to = HashSet::new();
        let mut recvs_from = HashSet::new();

        for _ in 0..self.rng.random_range(1..num_neurons) {
            let rand = self.rng.random_range(0..num_neurons);
            sends_to.insert(self.neurons[rand].id());
        }
        for _ in 0..self.rng.random_range(1..num_neurons) {
            let rand = self.rng.random_range(0..num_neurons);
            recvs_from.insert(self.neurons[rand].id());
        }

        let name = format!("N{}", self.neurons.len());
        let mut new_neuron = Neuron::new(name);

        for receiver in sends_to {
            let receiver = self.get_neuron_mut(receiver);
            receiver.rx_from(&new_neuron);
        }
        for sender in recvs_from {
            let sender = self.get_neuron(sender);
            new_neuron.rx_from(sender);
        }

        self.neurons.push(new_neuron);
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

#[derive(Clone, Copy)]
pub struct DendriteMessage {
    pub neuron_id: Uuid,
    pub dendrite_id: Uuid,
    pub current_potential: i32,
}

#[derive(Clone, Copy)]
pub struct AxonMessage {
    pub id: Uuid,
    pub discharge: i32,
}
