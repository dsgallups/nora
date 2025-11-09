use crate::prelude::Neuron;

#[derive(Debug)]
pub struct NeuronProcess {}

#[derive(Default, Debug)]
pub struct BrainState {
    queued: Vec<NeuronProcess>,
}

impl BrainState {
    pub fn set_entry_point(&mut self, neuron: &Neuron) {
        let enqueue = neuron.enqueue();
        todo!()
    }
}
