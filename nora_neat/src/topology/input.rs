use std::sync::{Arc, RwLock, Weak};

use crate::prelude::*;

pub type PolyInputTopology = PolyInput<Weak<RwLock<NeuronTopology>>>;

impl PolyInputTopology {
    pub fn neuron(&self) -> Option<Arc<RwLock<NeuronTopology>>> {
        Weak::upgrade(self.input())
    }

    pub fn downgrade(input: &Arc<RwLock<NeuronTopology>>, weight: f32, exp: i32) -> Self {
        Self::new(Arc::downgrade(input), weight, exp)
    }
}
