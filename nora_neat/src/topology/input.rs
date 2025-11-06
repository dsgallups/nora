use std::sync::{Arc, RwLock, Weak};

use crate::{prelude::*, topology::neuron_type::Topology};

//pub type PolyInputTopology = PolyInput<Weak<RwLock<NeuronTopology>>>;

impl NeuronInput<Topology> {
    pub fn neuron(&self) -> Option<Arc<RwLock<NeuronTopology>>> {
        Weak::upgrade(self.input().handle())
    }

    pub fn downgrade(input: &Arc<RwLock<NeuronTopology>>, weight: f32, exp: i32) -> Self {
        Self::new(Topology::new(input), weight, exp)
    }
}
