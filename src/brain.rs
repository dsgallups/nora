use crate::neuron::Neuron;

pub struct Brain {
    neurons: Vec<Neuron>,
}

impl Brain {
    pub fn sandbox() -> Self {
        let mut root_neuron = Neuron::default();
        let mut second_neuron = Neuron::default();
        let dendrite = second_neuron.grow_dendrite();

        root_neuron.attach_to(dendrite);
        Self {
            neurons: Vec::new(),
        }
    }
}
